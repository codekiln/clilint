use std::fmt::Write;

use crate::model::{EvaluationMethod, Report};

pub fn json(report: &Report) -> Result<String, String> {
    serde_json::to_string_pretty(report).map_err(|error| error.to_string())
}

pub fn human(report: &Report) -> String {
    let mut output = String::new();
    let packages = report
        .packages
        .iter()
        .map(|package| format!("{} {}", package.name, package.version))
        .collect::<Vec<_>>()
        .join(", ");
    let _ = writeln!(output, "CLI Lint {packages}");
    let _ = writeln!(output, "Target: {}", report.target);
    let _ = writeln!(
        output,
        "Deterministic score: {}/100\n",
        report.deterministic.score
    );
    for finding in &report.findings {
        let method = match finding.evaluation_method {
            EvaluationMethod::Deterministic => "deterministic",
            EvaluationMethod::AiAgent => "ai-agent",
        };
        let _ = writeln!(
            output,
            "{:<42} {:<10} {:<13} {}",
            finding.rule,
            format!("{:?}", finding.result).to_lowercase(),
            method,
            finding.detail
        );
    }
    let deterministic = &report.deterministic.counts;
    let agent = &report.ai_agent;
    let _ = writeln!(
        output,
        "\nDeterministic: {} pass, {} warn, {} fail, {} skip",
        deterministic.pass, deterministic.warn, deterministic.fail, deterministic.skip
    );
    let _ = writeln!(
        output,
        "AI agent: {} pass, {} warn, {} fail, {} skip, {} unassessed",
        agent.pass, agent.warn, agent.fail, agent.skip, agent.unassessed
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DeterministicSummary, PackageIdentity, Report, ResultCounts};

    fn empty_report() -> Report {
        Report {
            format_version: 1,
            tool_version: "0.0.2".into(),
            packages: vec![PackageIdentity {
                name: "clilint".into(),
                version: "0.0.2".into(),
            }],
            target: "fixture".into(),
            deterministic: DeterministicSummary::default(),
            ai_agent: ResultCounts::default(),
            findings: Vec::new(),
        }
    }

    #[test]
    fn json_is_one_document() {
        let encoded = json(&empty_report()).unwrap();
        let decoded: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded["format_version"], 1);
    }

    #[test]
    fn human_separates_measurements() {
        let output = human(&empty_report());
        assert!(output.contains("Deterministic:"));
        assert!(output.contains("AI agent:"));
    }
}
