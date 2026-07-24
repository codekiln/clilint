use sha2::{Digest, Sha256};

use crate::{
    VERSION,
    check_bundle::{
        Assertion, CheckBundleManifest, CheckDefinition, CheckerDefinition, InvocationCheck,
    },
    help_checker::HelpContext,
    model::{EvaluationMethod, Finding, Report, ResultStatus},
    runner::Runner,
};

pub fn check(
    target: &str,
    bundle: &CheckBundleManifest,
    runner: &mut Runner,
) -> Result<Report, String> {
    let mut findings = Vec::with_capacity(bundle.checks.len());
    let help_limits = bundle.checks.iter().find_map(|check| {
        if let Some(CheckerDefinition::HierarchicalHelp { limits, .. }) = &check.checker {
            Some(limits)
        } else {
            None
        }
    });
    if let Some(expected) = help_limits
        && bundle.checks.iter().any(|check| {
            matches!(
                &check.checker,
                Some(CheckerDefinition::HierarchicalHelp { limits, .. }) if limits != expected
            )
        })
    {
        return Err(
            "all hierarchical-help checkers in one run must use the same limits".to_owned(),
        );
    }
    let help_context = help_limits.map(|limits| HelpContext::collect(runner, limits));
    for check in &bundle.checks {
        findings.push(match check.evaluation_method {
            EvaluationMethod::Deterministic => deterministic(check, runner, help_context.as_ref())?,
            EvaluationMethod::AiAgent => agent(check, runner)?,
        });
    }
    findings.sort_by(|left, right| left.check.cmp(&right.check));

    let mut report = Report {
        format_version: 2,
        tool_version: VERSION.into(),
        check_bundles: if bundle.resolved_check_bundles.is_empty() {
            vec![bundle.check_bundle.clone()]
        } else {
            bundle.resolved_check_bundles.clone()
        },
        target: target.into(),
        deterministic: Default::default(),
        ai_agent: Default::default(),
        findings,
    };
    report.recalculate();
    Ok(report)
}

fn deterministic(
    check: &CheckDefinition,
    runner: &mut Runner,
    help_context: Option<&HelpContext>,
) -> Result<Finding, String> {
    let checker = check
        .checker
        .as_ref()
        .ok_or_else(|| format!("deterministic check {} has no checker", check.id))?;
    let (passed, evidence, failures) = match checker {
        CheckerDefinition::Invocation { invocation } => {
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
        CheckerDefinition::HierarchicalHelp { behavior, .. } => {
            let context = help_context.ok_or_else(|| {
                format!(
                    "hierarchical help context was not collected for {}",
                    check.id
                )
            })?;
            let (passed, detail, evidence) = context.result(*behavior);
            let result = if passed {
                ResultStatus::Pass
            } else {
                check.severity.failed_result()
            };
            return Ok(finding(check, result, detail, evidence, None));
        }
    };
    let result = if passed {
        ResultStatus::Pass
    } else {
        check.severity.failed_result()
    };
    let detail = if passed {
        "declared behavioral check passed".into()
    } else {
        failures.join("; ")
    };
    Ok(finding(check, result, detail, evidence, None))
}

fn agent(check: &CheckDefinition, runner: &mut Runner) -> Result<Finding, String> {
    let evidence_spec = check
        .evidence
        .as_ref()
        .ok_or_else(|| format!("AI-agent check {} has no evidence invocation", check.id))?;
    let observation = runner.run(evidence_spec)?;
    let evidence = serde_json::json!({"observations": [observation]});
    Ok(finding(
        check,
        ResultStatus::Unassessed,
        "run the required skill to assess the captured evidence".into(),
        evidence,
        check.skill.clone(),
    ))
}

fn finding(
    check: &CheckDefinition,
    result: ResultStatus,
    detail: String,
    evidence: serde_json::Value,
    required_skill: Option<crate::model::SkillRef>,
) -> Finding {
    Finding {
        check: check.id.clone(),
        title: check.title.clone(),
        severity: check.severity,
        evaluation_method: check.evaluation_method,
        result,
        required_for_ratings: check.required_for_ratings.clone(),
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
