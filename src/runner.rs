use std::{
    collections::HashMap,
    io::{Read, Write},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::model::{InvocationSpec, Observation};

pub struct Runner {
    target: String,
    default_timeout_ms: u64,
    cache: HashMap<String, Observation>,
    execution_count: usize,
}

impl Runner {
    pub fn new(target: String, default_timeout_ms: u64) -> Self {
        Self {
            target,
            default_timeout_ms,
            cache: HashMap::new(),
            execution_count: 0,
        }
    }

    pub fn run(&mut self, spec: &InvocationSpec) -> Result<Observation, String> {
        self.run_with_output_limit(spec, None)
    }

    pub fn run_bounded(
        &mut self,
        spec: &InvocationSpec,
        output_limit: usize,
    ) -> Result<Observation, String> {
        self.run_with_output_limit(spec, Some(output_limit))
    }

    fn run_with_output_limit(
        &mut self,
        spec: &InvocationSpec,
        output_limit: Option<usize>,
    ) -> Result<Observation, String> {
        let timeout_ms = spec.timeout_ms.unwrap_or(self.default_timeout_ms);
        let key = serde_json::to_string(&(spec, timeout_ms, output_limit))
            .map_err(|error| error.to_string())?;
        if let Some(observation) = self.cache.get(&key) {
            return Ok(observation.clone());
        }

        let mut command = Command::new(&self.target);
        command
            .args(&spec.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (name, value) in &spec.env {
            command.env(name, value);
        }
        if spec.stdin.is_some() {
            command.stdin(Stdio::piped());
        } else {
            command.stdin(Stdio::null());
        }

        let start = Instant::now();
        self.execution_count += 1;
        let mut child = command
            .spawn()
            .map_err(|error| format!("could not run target {}: {error}", self.target))?;

        if let Some(input) = &spec.stdin {
            let mut stdin = child
                .stdin
                .take()
                .ok_or_else(|| "target stdin was not available".to_owned())?;
            stdin
                .write_all(input.as_bytes())
                .map_err(|error| format!("could not write target stdin: {error}"))?;
        }

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "target stdout was not available".to_owned())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "target stderr was not available".to_owned())?;
        let stdout_reader = thread::spawn(move || read_all(stdout, output_limit));
        let stderr_reader = thread::spawn(move || read_all(stderr, output_limit));

        let deadline = start + Duration::from_millis(timeout_ms);
        let (status, timed_out) = loop {
            if let Some(status) = child
                .try_wait()
                .map_err(|error| format!("could not wait for target: {error}"))?
            {
                break (Some(status), false);
            }
            if Instant::now() >= deadline {
                child
                    .kill()
                    .map_err(|error| format!("could not stop timed-out target: {error}"))?;
                let status = child
                    .wait()
                    .map_err(|error| format!("could not reap timed-out target: {error}"))?;
                break (Some(status), true);
            }
            thread::sleep(Duration::from_millis(5));
        };

        let stdout = join_reader(stdout_reader, "stdout")?;
        let stderr = join_reader(stderr_reader, "stderr")?;
        let observation = Observation {
            args: spec.args.clone(),
            exit_status: if timed_out {
                None
            } else {
                status.and_then(|value| value.code())
            },
            timed_out,
            duration_ms: start.elapsed().as_secs_f64() * 1000.0,
            stdout: String::from_utf8_lossy(&stdout).into_owned(),
            stderr: String::from_utf8_lossy(&stderr).into_owned(),
            has_ansi: contains_ansi(&stdout) || contains_ansi(&stderr),
        };
        self.cache.insert(key, observation.clone());
        Ok(observation)
    }

    pub fn execution_count(&self) -> usize {
        self.execution_count
    }
}

fn read_all(mut reader: impl Read, limit: Option<usize>) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    if let Some(limit) = limit {
        reader
            .by_ref()
            .take(limit.saturating_add(1) as u64)
            .read_to_end(&mut output)?;
    } else {
        reader.read_to_end(&mut output)?;
    }
    Ok(output)
}

fn join_reader(
    reader: thread::JoinHandle<std::io::Result<Vec<u8>>>,
    stream: &str,
) -> Result<Vec<u8>, String> {
    reader
        .join()
        .map_err(|_| format!("target {stream} reader panicked"))?
        .map_err(|error| format!("could not read target {stream}: {error}"))
}

fn contains_ansi(bytes: &[u8]) -> bool {
    bytes.windows(2).any(|pair| pair == b"\x1b[")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn invocation(args: &[&str], timeout_ms: Option<u64>) -> InvocationSpec {
        InvocationSpec {
            args: args.iter().map(ToString::to_string).collect(),
            env: BTreeMap::new(),
            stdin: None,
            timeout_ms,
        }
    }

    #[test]
    fn closes_standard_input() {
        let mut runner = Runner::new("sh".into(), 1_000);
        let result = runner
            .run(&invocation(&["-c", "read value || echo eof"], None))
            .unwrap();
        assert_eq!(result.stdout.trim(), "eof");
        assert!(!result.timed_out);
    }

    #[test]
    fn stops_timed_out_process() {
        let mut runner = Runner::new("sh".into(), 1_000);
        let result = runner
            .run(&invocation(&["-c", "sleep 2"], Some(20)))
            .unwrap();
        assert!(result.timed_out);
        assert_eq!(result.exit_status, None);
    }

    #[test]
    fn supplies_explicit_input() {
        let mut spec = invocation(&["-c", "read value; echo $value"], None);
        spec.stdin = Some("hello\n".into());
        let result = Runner::new("sh".into(), 1_000).run(&spec).unwrap();
        assert_eq!(result.stdout.trim(), "hello");
    }

    #[test]
    fn reuses_an_identical_observation() {
        let mut runner = Runner::new("printf".into(), 1_000);
        let spec = invocation(&["hello"], None);
        let first = runner.run(&spec).unwrap();
        let second = runner.run(&spec).unwrap();
        assert_eq!(first.stdout, second.stdout);
        assert_eq!(runner.execution_count(), 1);
    }

    #[test]
    fn bounds_captured_output() {
        let mut runner = Runner::new("printf".into(), 1_000);
        let result = runner
            .run_bounded(&invocation(&["123456789"], None), 5)
            .unwrap();
        assert_eq!(result.stdout, "123456");
    }
}
