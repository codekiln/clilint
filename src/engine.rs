use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use sha2::{Digest, Sha256};

use crate::{
    VERSION, assessment,
    check_bundle::{
        Assertion, CheckBundleManifest, CheckDefinition, CheckerDefinition, InvocationCheck,
    },
    model::{
        Assessment, AssessmentRequest, CheckBundleIdentity, CheckError, CheckMessage, CheckOutcome,
        CheckRecord, CheckRequest, CheckResult, CheckerResponse, EvaluationMethod, Report, Score,
    },
    runner::Runner,
};

const CHECKER_TIMEOUT_MS: u64 = 60_000;

pub fn check(
    target: &str,
    project_directory: &Path,
    bundle: &CheckBundleManifest,
    runner: &mut Runner,
    assessments: &[Assessment],
) -> Result<Report, String> {
    let mut supplied = HashMap::new();
    for assessment in assessments {
        if supplied
            .insert(assessment.check.clone(), assessment)
            .is_some()
        {
            return Err(format!(
                "more than one Assessment was supplied for Check {}",
                assessment.check
            ));
        }
    }

    let mut used_assessments = HashSet::new();
    let mut checks = Vec::with_capacity(bundle.checks.len());
    for check in &bundle.checks {
        let owner = owner_identity(bundle, &check.id)?;
        let supplied_assessment = supplied.get(&check.id).copied();
        if supplied_assessment.is_some() {
            used_assessments.insert(check.id.clone());
        }
        checks.push(match &check.checker {
            Some(CheckerDefinition::Cli { command }) => run_checker_cli(
                target,
                project_directory,
                owner,
                check,
                command,
                supplied_assessment,
            )?,
            _ => match check.evaluation_method {
                EvaluationMethod::Mechanistic => built_in_mechanistic(check, runner)?,
                EvaluationMethod::JudgmentBased => {
                    built_in_judgment(check, runner, supplied_assessment)?
                }
            },
        });
    }
    if let Some(unknown) = supplied
        .keys()
        .find(|check| !used_assessments.contains(*check))
    {
        return Err(format!("Assessment references unknown Check {unknown}"));
    }
    checks.sort_by(|left, right| left.check.cmp(&right.check));

    let mut report = Report {
        format_version: 3,
        tool_version: VERSION.into(),
        check_bundles: if bundle.resolved_check_bundles.is_empty() {
            vec![bundle.check_bundle.clone()]
        } else {
            bundle.resolved_check_bundles.clone()
        },
        target: target.into(),
        checks,
        summary: Default::default(),
    };
    report.recalculate();
    Ok(report)
}

fn owner_identity<'a>(
    bundle: &'a CheckBundleManifest,
    check: &str,
) -> Result<&'a CheckBundleIdentity, String> {
    let name = check
        .split_once('/')
        .map(|(name, _)| name)
        .ok_or_else(|| format!("Check {check} has no bundle prefix"))?;
    if bundle.check_bundle.name == name {
        return Ok(&bundle.check_bundle);
    }
    bundle
        .resolved_check_bundles
        .iter()
        .find(|identity| identity.name == name)
        .ok_or_else(|| format!("Check {check} belongs to unknown check bundle {name}"))
}

fn built_in_mechanistic(
    check: &CheckDefinition,
    runner: &mut Runner,
) -> Result<CheckRecord, String> {
    let checker = check
        .checker
        .as_ref()
        .ok_or_else(|| format!("mechanistic Check {} has no checker", check.id))?;
    let (passed, evidence, failures) = match checker {
        CheckerDefinition::SingleInvocation { invocation } => {
            let (observation, failures) = evaluate(invocation, runner)?;
            (
                failures.is_empty(),
                serde_json::json!({"observations": [observation]}),
                failures,
            )
        }
        CheckerDefinition::AnyInvocation { invocations } => {
            let mut observations = Vec::new();
            let mut all_failures = Vec::new();
            let mut passed = false;
            for invocation in invocations {
                let (observation, failures) = evaluate(invocation, runner)?;
                passed |= failures.is_empty();
                observations.push(observation);
                all_failures.extend(failures);
            }
            (
                passed,
                serde_json::json!({"observations": observations}),
                if passed { Vec::new() } else { all_failures },
            )
        }
        CheckerDefinition::AllInvocations { invocations } => {
            let mut observations = Vec::new();
            let mut failures = Vec::new();
            for invocation in invocations {
                let (observation, invocation_failures) = evaluate(invocation, runner)?;
                observations.push(observation);
                failures.extend(invocation_failures);
            }
            (
                failures.is_empty(),
                serde_json::json!({"observations": observations}),
                failures,
            )
        }
        CheckerDefinition::Cli { .. } => {
            return Err(format!(
                "Checker CLI {} was sent to the built-in checker path",
                check.id
            ));
        }
    };
    let result = if passed {
        CheckResult {
            score: Score::new(4.0).expect("valid perfect Score"),
            messages: Vec::new(),
            assessment: None,
        }
    } else {
        CheckResult {
            score: check.severity.failed_score(),
            messages: vec![CheckMessage {
                level: check.severity.message_level(),
                message: failures.join("; "),
                evidence,
            }],
            assessment: None,
        }
    };
    result.validate()?;
    Ok(CheckRecord {
        check: check.id.clone(),
        title: check.title.clone(),
        method: EvaluationMethod::Mechanistic,
        outcome: CheckOutcome::Result { result },
    })
}

fn built_in_judgment(
    check: &CheckDefinition,
    runner: &mut Runner,
    supplied_assessment: Option<&Assessment>,
) -> Result<CheckRecord, String> {
    let evidence_spec = check.evidence.as_ref().ok_or_else(|| {
        format!(
            "judgment-based Check {} has no evidence invocation",
            check.id
        )
    })?;
    let skill = check
        .skill
        .as_ref()
        .ok_or_else(|| format!("judgment-based Check {} has no Skill", check.id))?;
    let observation = runner.run(evidence_spec)?;
    let evidence = serde_json::json!({"observations": [observation]});
    let evidence_digest = evidence_digest(&evidence);
    let request_id = digest_value(&serde_json::json!({
        "check": check.id,
        "evidence_digest": evidence_digest,
    }));
    let outcome = if let Some(document) = supplied_assessment {
        CheckOutcome::Result {
            result: assessment::validate(
                document,
                &request_id,
                &check.id,
                skill,
                &evidence_digest,
            )?,
        }
    } else {
        CheckOutcome::AwaitingAssessment {
            assessment_request: AssessmentRequest {
                request_id,
                evidence_digest,
                skill: skill.clone(),
                rubric: "Apply the bundled Skill to the captured help evidence.".into(),
                evidence,
            },
        }
    };
    Ok(CheckRecord {
        check: check.id.clone(),
        title: check.title.clone(),
        method: EvaluationMethod::JudgmentBased,
        outcome,
    })
}

fn run_checker_cli(
    target: &str,
    project_directory: &Path,
    owner: &CheckBundleIdentity,
    check: &CheckDefinition,
    command: &[String],
    supplied_assessment: Option<&Assessment>,
) -> Result<CheckRecord, String> {
    let bundle_root = check
        .bundle_root
        .as_ref()
        .ok_or_else(|| format!("Checker CLI {} has no bundle directory", check.id))?;
    let command = command
        .iter()
        .map(|part| part.replace("{bundle}", &bundle_root.to_string_lossy()))
        .collect::<Vec<_>>();
    let request_id = digest_value(&serde_json::json!({
        "format_version": 1,
        "check_bundle": owner,
        "check": check.id,
        "target": target,
        "project_directory": project_directory,
    }));
    let request = CheckRequest {
        format_version: 1,
        request_id: request_id.clone(),
        check_bundle: owner.clone(),
        check: check.id.clone(),
        target: vec![target.into()],
        project_directory: project_directory.to_string_lossy().into_owned(),
        assessment: supplied_assessment.cloned(),
    };
    let request_json = serde_json::to_vec(&request).map_err(|error| error.to_string())?;
    let process = run_process(&command, project_directory, &request_json);
    let outcome = match process {
        Ok(mut process) => {
            if let Some(message) = process_failure_message(&process) {
                CheckOutcome::Error {
                    error: process_error(&message),
                }
            } else {
                let response = process
                    .stdout
                    .rewind()
                    .map_err(|error| format!("could not read Checker CLI standard output: {error}"))
                    .and_then(|_| parse_checker_response(&mut process.stdout));
                match response {
                    Ok(response) => match validate_response(response, &request_id, check) {
                        Ok(outcome) => outcome,
                        Err(message) => CheckOutcome::Error {
                            error: process_error(&message),
                        },
                    },
                    Err(error) => CheckOutcome::Error {
                        error: process_error(&error),
                    },
                }
            }
        }
        Err(message) => CheckOutcome::Error {
            error: CheckError { message },
        },
    };
    Ok(CheckRecord {
        check: check.id.clone(),
        title: check.title.clone(),
        method: check.evaluation_method,
        outcome,
    })
}

fn process_failure_message(process: &ProcessOutput) -> Option<String> {
    if process.timed_out {
        Some("Checker CLI timed out".into())
    } else if process.exit_status != Some(0) {
        Some(format!(
            "Checker CLI exited with status {}",
            process
                .exit_status
                .map_or_else(|| "unknown".into(), |status| status.to_string())
        ))
    } else {
        None
    }
}

fn parse_checker_response(reader: impl Read) -> Result<CheckerResponse, String> {
    let value: serde_json::Value = serde_json::from_reader(reader)
        .map_err(|error| format!("Checker CLI returned invalid JSON: {error}"))?;
    let object = value
        .as_object()
        .ok_or_else(|| "Checker CLI response must be one JSON object".to_owned())?;
    let outcome = object
        .get("outcome")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "Checker CLI response must name one outcome".to_owned())?;
    let payload = match outcome {
        "result" => "result",
        "error" => "error",
        "awaiting-assessment" => "assessment_request",
        "skipped" => "reason",
        _ => {
            return Err(format!(
                "Checker CLI response uses unknown outcome {outcome:?}"
            ));
        }
    };
    let allowed = [
        "format_version",
        "request_id",
        "check",
        "method",
        "outcome",
        payload,
    ];
    if let Some(field) = object
        .keys()
        .find(|field| !allowed.contains(&field.as_str()))
    {
        return Err(format!(
            "Checker CLI response contains unknown field {field:?}"
        ));
    }
    serde_json::from_value(value)
        .map_err(|error| format!("Checker CLI returned an invalid Check Outcome: {error}"))
}

fn validate_response(
    response: CheckerResponse,
    request_id: &str,
    check: &CheckDefinition,
) -> Result<CheckOutcome, String> {
    if response.format_version != 1 {
        return Err(format!(
            "Checker CLI returned unsupported format version {}",
            response.format_version
        ));
    }
    if response.request_id != request_id {
        return Err(format!(
            "Checker CLI response belongs to request {}, expected {request_id}",
            response.request_id
        ));
    }
    if response.check != check.id {
        return Err(format!(
            "Checker CLI response names Check {}, expected {}",
            response.check, check.id
        ));
    }
    if response.method != check.evaluation_method {
        return Err(format!(
            "Checker CLI response uses method {:?}, expected {:?}",
            response.method, check.evaluation_method
        ));
    }
    if let CheckOutcome::Result { result } = &response.outcome {
        result.validate()?;
    }
    if let CheckOutcome::AwaitingAssessment { assessment_request } = &response.outcome {
        if check.evaluation_method != EvaluationMethod::JudgmentBased {
            return Err("a mechanistic Checker CLI cannot return Awaiting Assessment".into());
        }
        if assessment_request.request_id != request_id {
            return Err("Awaiting Assessment uses the wrong request binding".into());
        }
    }
    Ok(response.outcome)
}

struct ProcessOutput {
    exit_status: Option<i32>,
    timed_out: bool,
    stdout: File,
}

fn run_process(
    command: &[String],
    project_directory: &Path,
    input: &[u8],
) -> Result<ProcessOutput, String> {
    let (program, arguments) = command
        .split_first()
        .ok_or_else(|| "Checker CLI command is empty".to_owned())?;
    let stdout = tempfile::tempfile()
        .map_err(|error| format!("could not create Checker CLI output file: {error}"))?;
    let child_stdout = stdout
        .try_clone()
        .map_err(|error| format!("could not prepare Checker CLI output file: {error}"))?;
    let mut child = Command::new(program)
        .args(arguments)
        .current_dir(project_directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::from(child_stdout))
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| format!("could not start Checker CLI {program}: {error}"))?;
    child
        .stdin
        .take()
        .ok_or_else(|| "Checker CLI standard input was unavailable".to_owned())?
        .write_all(input)
        .map_err(|error| format!("could not write Checker CLI request: {error}"))?;

    let deadline = Instant::now() + Duration::from_millis(CHECKER_TIMEOUT_MS);
    let (status, timed_out) = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("could not wait for Checker CLI: {error}"))?
        {
            break (Some(status), false);
        }
        if Instant::now() >= deadline {
            child
                .kill()
                .map_err(|error| format!("could not stop timed-out Checker CLI: {error}"))?;
            let status = child
                .wait()
                .map_err(|error| format!("could not reap timed-out Checker CLI: {error}"))?;
            break (Some(status), true);
        }
        thread::sleep(Duration::from_millis(5));
    };
    Ok(ProcessOutput {
        exit_status: if timed_out {
            None
        } else {
            status.and_then(|status| status.code())
        },
        timed_out,
        stdout,
    })
}

fn process_error(message: &str) -> CheckError {
    CheckError {
        message: message.into(),
    }
}

fn evaluate(
    check: &InvocationCheck,
    runner: &mut Runner,
) -> Result<(crate::model::Observation, Vec<String>), String> {
    let observation = runner.run(&check.invocation)?;
    let failures = check
        .assertions
        .iter()
        .filter_map(|assertion| assertion_failure(assertion, &observation))
        .collect();
    Ok((observation, failures))
}

fn assertion_failure(
    assertion: &Assertion,
    observation: &crate::model::Observation,
) -> Option<String> {
    let combined = format!("{}\n{}", observation.stdout, observation.stderr).to_lowercase();
    let stdout = observation.stdout.to_lowercase();
    let failed = match assertion {
        Assertion::ExitCode { value } => observation.exit_status != Some(*value),
        Assertion::ExitNonZero => observation.exit_status.is_none_or(|status| status == 0),
        Assertion::NotTimedOut => observation.timed_out,
        Assertion::StdoutNotEmpty => observation.stdout.trim().is_empty(),
        Assertion::StderrNotEmpty => observation.stderr.trim().is_empty(),
        Assertion::StdoutAtLeastStderr => {
            observation.stdout.trim().is_empty()
                || observation.stdout.len() < observation.stderr.len()
        }
        Assertion::OutputContainsAny { values } => !values
            .iter()
            .any(|value| combined.contains(&value.to_lowercase())),
        Assertion::StdoutContainsAny { values } => !values
            .iter()
            .any(|value| stdout.contains(&value.to_lowercase())),
        Assertion::NoAnsi => observation.has_ansi,
        Assertion::DurationAtMost { milliseconds } => {
            observation.timed_out || observation.duration_ms > *milliseconds
        }
        Assertion::VersionNumber => !contains_version_number(&combined),
    };
    failed.then(|| {
        format!(
            "assertion {assertion:?} failed for arguments {:?}",
            observation.args
        )
    })
}

fn contains_version_number(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.windows(3).enumerate().any(|(index, window)| {
        window[0].is_ascii_digit()
            && window[1] == b'.'
            && window[2].is_ascii_digit()
            && (index == 0 || !bytes[index - 1].is_ascii_digit())
    })
}

pub fn evidence_digest(evidence: &serde_json::Value) -> String {
    let mut stable_evidence = evidence.clone();
    remove_unstable_measurements(&mut stable_evidence);
    digest_value(&stable_evidence)
}

fn digest_value(value: &serde_json::Value) -> String {
    let encoded = serde_json::to_vec(value).expect("JSON values always serialize");
    let digest = Sha256::digest(encoded);
    format!("sha256:{digest:x}")
}

fn remove_unstable_measurements(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(object) => {
            object.remove("duration_ms");
            for child in object.values_mut() {
                remove_unstable_measurements(child);
            }
        }
        serde_json::Value::Array(array) => {
            for child in array {
                remove_unstable_measurements(child);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_number_requires_dotted_digits() {
        assert!(contains_version_number("tool 1.2.3"));
        assert!(!contains_version_number("tool version one"));
    }

    #[test]
    fn evidence_digest_is_stable() {
        let first = serde_json::json!({"stdout": "hello", "duration_ms": 1.0});
        let second = serde_json::json!({"stdout": "hello", "duration_ms": 9.0});
        assert_eq!(evidence_digest(&first), evidence_digest(&second));
        let changed = serde_json::json!({"stdout": "changed", "duration_ms": 1.0});
        assert_ne!(evidence_digest(&first), evidence_digest(&changed));
    }

    #[test]
    fn checker_response_rejects_unknown_fields() {
        let response = serde_json::json!({
            "format_version": 1,
            "request_id": "request-1",
            "check": "bundle/check",
            "method": "mechanistic",
            "outcome": "skipped",
            "reason": "not applicable",
            "unexpected": true
        });
        assert!(
            parse_checker_response(serde_json::to_vec(&response).unwrap().as_slice())
                .unwrap_err()
                .contains("unknown field")
        );
    }

    #[test]
    fn checker_process_failures_are_distinct() {
        let output = |exit_status, timed_out| ProcessOutput {
            exit_status,
            timed_out,
            stdout: tempfile::tempfile().unwrap(),
        };
        assert_eq!(
            process_failure_message(&output(None, true)).unwrap(),
            "Checker CLI timed out"
        );
        assert!(
            process_failure_message(&output(Some(7), false))
                .unwrap()
                .contains("status 7")
        );
    }
}
