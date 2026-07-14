use sha2::{Digest, Sha256};

use crate::{
    VERSION,
    model::{EvaluationMethod, Finding, Report, ResultStatus},
    package::{Assertion, CheckDefinition, InvocationCheck, PackageManifest, RuleDefinition},
    runner::Runner,
};

pub fn check(
    target: &str,
    package: &PackageManifest,
    runner: &mut Runner,
) -> Result<Report, String> {
    let mut findings = Vec::with_capacity(package.rules.len());
    for rule in &package.rules {
        findings.push(match rule.evaluation_method {
            EvaluationMethod::Deterministic => deterministic(rule, runner)?,
            EvaluationMethod::AiAgent => agent(rule, runner)?,
        });
    }
    findings.sort_by(|left, right| left.rule.cmp(&right.rule));

    let mut report = Report {
        format_version: 1,
        tool_version: VERSION.into(),
        packages: if package.resolved_packages.is_empty() {
            vec![package.package.clone()]
        } else {
            package.resolved_packages.clone()
        },
        target: target.into(),
        deterministic: Default::default(),
        ai_agent: Default::default(),
        findings,
    };
    report.recalculate();
    Ok(report)
}

fn deterministic(rule: &RuleDefinition, runner: &mut Runner) -> Result<Finding, String> {
    let check = rule
        .check
        .as_ref()
        .ok_or_else(|| format!("deterministic rule {} has no check", rule.id))?;
    let (passed, evidence, failures) = match check {
        CheckDefinition::Invocation { invocation } => {
            let (observation, failures) = evaluate(invocation, runner)?;
            (
                failures.is_empty(),
                serde_json::json!({"observations": [observation]}),
                failures,
            )
        }
        CheckDefinition::AnyInvocation { invocations } => {
            let mut observations = Vec::new();
            let mut all_failures = Vec::new();
            let mut passed = false;
            for invocation in invocations {
                let (observation, failures) = evaluate(invocation, runner)?;
                if failures.is_empty() {
                    passed = true;
                }
                observations.push(observation);
                all_failures.extend(failures);
            }
            (
                passed,
                serde_json::json!({"observations": observations}),
                if passed { Vec::new() } else { all_failures },
            )
        }
        CheckDefinition::AllInvocations { invocations } => {
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
    };
    let result = if passed {
        ResultStatus::Pass
    } else {
        rule.severity.failed_result()
    };
    let detail = if passed {
        "declared behavioral check passed".into()
    } else {
        failures.join("; ")
    };
    Ok(finding(rule, result, detail, evidence, None))
}

fn agent(rule: &RuleDefinition, runner: &mut Runner) -> Result<Finding, String> {
    let evidence_spec = rule
        .evidence
        .as_ref()
        .ok_or_else(|| format!("AI-agent rule {} has no evidence invocation", rule.id))?;
    let observation = runner.run(evidence_spec)?;
    let evidence = serde_json::json!({"observations": [observation]});
    Ok(finding(
        rule,
        ResultStatus::Unassessed,
        "run the required skill to assess the captured evidence".into(),
        evidence,
        rule.skill.clone(),
    ))
}

fn finding(
    rule: &RuleDefinition,
    result: ResultStatus,
    detail: String,
    evidence: serde_json::Value,
    required_skill: Option<crate::model::SkillRef>,
) -> Finding {
    Finding {
        rule: rule.id.clone(),
        title: rule.title.clone(),
        severity: rule.severity,
        evaluation_method: rule.evaluation_method,
        result,
        detail,
        evidence_digest: evidence_digest(&evidence),
        evidence,
        required_skill,
        assessment: None,
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
            "assertion {assertion:?} failed for args {:?}",
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
    let encoded = serde_json::to_vec(&stable_evidence).expect("JSON values always serialize");
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
}
