use std::fmt::Write;

use crate::model::{CheckOutcome, EvaluationMethod, Report};

pub fn json(report: &Report) -> Result<String, String> {
    serde_json::to_string_pretty(report).map_err(|error| error.to_string())
}

pub fn human(report: &Report) -> String {
    let mut output = String::new();
    let check_bundles = report
        .check_bundles
        .iter()
        .map(|bundle| format!("{} {}", bundle.name, bundle.version))
        .collect::<Vec<_>>()
        .join(", ");
    let _ = writeln!(output, "Clilint {check_bundles}");
    let _ = writeln!(output, "Target: {}\n", report.target);
    for check in &report.checks {
        let method = match check.method {
            EvaluationMethod::Mechanistic => "mechanistic",
            EvaluationMethod::JudgmentBased => "judgment-based",
        };
        match &check.outcome {
            CheckOutcome::Result { result } => {
                let _ = writeln!(
                    output,
                    "{:<46} {:<16} Score {}",
                    check.check, method, result.score
                );
                for message in &result.messages {
                    let _ = writeln!(
                        output,
                        "  {:<7} {}",
                        format!("{:?}", message.level).to_lowercase(),
                        message.message
                    );
                }
            }
            CheckOutcome::Error { error } => {
                let _ = writeln!(
                    output,
                    "{:<46} {:<16} Check Error: {}",
                    check.check, method, error.message
                );
            }
            CheckOutcome::AwaitingAssessment { .. } => {
                let _ = writeln!(
                    output,
                    "{:<46} {:<16} Awaiting Assessment",
                    check.check, method
                );
            }
            CheckOutcome::Skipped { reason } => {
                let _ = writeln!(
                    output,
                    "{:<46} {:<16} Skipped: {}",
                    check.check, method, reason
                );
            }
        }
    }
    let summary = &report.summary;
    let _ = writeln!(
        output,
        "\n{} results, {} Check Errors, {} awaiting Assessment, {} skipped",
        summary.check_results, summary.check_errors, summary.awaiting_assessment, summary.skipped
    );
    let _ = writeln!(
        output,
        "{} Info, {} Warning, {} Error Check Messages",
        summary.info_messages, summary.warning_messages, summary.error_messages
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CheckBundleIdentity, ReportSummary};

    fn empty_report() -> Report {
        Report {
            format_version: 3,
            tool_version: "0.0.2".into(),
            check_bundles: vec![CheckBundleIdentity {
                name: "clilint".into(),
                version: "0.0.2".into(),
            }],
            target: "fixture".into(),
            checks: Vec::new(),
            summary: ReportSummary::default(),
        }
    }

    #[test]
    fn json_is_one_document() {
        let encoded = json(&empty_report()).unwrap();
        let decoded: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded["format_version"], 3);
    }

    #[test]
    fn human_names_results_and_messages() {
        let output = human(&empty_report());
        assert!(output.contains("results"));
        assert!(output.contains("Check Messages"));
    }
}
