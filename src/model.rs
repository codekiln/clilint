use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackageIdentity {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warn,
    Info,
}

impl Severity {
    pub fn weight(self) -> f64 {
        match self {
            Self::Error => 3.0,
            Self::Warn => 1.0,
            Self::Info => 0.5,
        }
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warn => 1,
            Self::Error => 2,
        }
    }

    pub fn failed_result(self) -> ResultStatus {
        match self {
            Self::Error => ResultStatus::Fail,
            Self::Warn | Self::Info => ResultStatus::Warn,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvaluationMethod {
    Deterministic,
    AiAgent,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResultStatus {
    Pass,
    Warn,
    Fail,
    Skip,
    Unassessed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillRef {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct InvocationSpec {
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub stdin: Option<String>,
    pub timeout_ms: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Observation {
    pub args: Vec<String>,
    pub exit_status: Option<i32>,
    pub timed_out: bool,
    pub duration_ms: f64,
    pub stdout: String,
    pub stderr: String,
    pub has_ansi: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssessmentProvenance {
    pub skill: SkillRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Finding {
    pub rule: String,
    pub title: String,
    pub severity: Severity,
    pub evaluation_method: EvaluationMethod,
    pub result: ResultStatus,
    pub detail: String,
    pub evidence: serde_json::Value,
    pub evidence_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_skill: Option<SkillRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<AssessmentProvenance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ResultCounts {
    pub pass: u32,
    pub warn: u32,
    pub fail: u32,
    pub skip: u32,
    pub unassessed: u32,
    pub total: u32,
}

impl ResultCounts {
    pub fn add(&mut self, result: ResultStatus) {
        match result {
            ResultStatus::Pass => self.pass += 1,
            ResultStatus::Warn => self.warn += 1,
            ResultStatus::Fail => self.fail += 1,
            ResultStatus::Skip => self.skip += 1,
            ResultStatus::Unassessed => self.unassessed += 1,
        }
        self.total += 1;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeterministicSummary {
    pub score: u8,
    #[serde(flatten)]
    pub counts: ResultCounts,
}

impl Default for DeterministicSummary {
    fn default() -> Self {
        Self {
            score: 100,
            counts: ResultCounts::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Report {
    pub format_version: u32,
    pub tool_version: String,
    pub packages: Vec<PackageIdentity>,
    pub target: String,
    pub deterministic: DeterministicSummary,
    pub ai_agent: ResultCounts,
    pub findings: Vec<Finding>,
}

impl Report {
    pub fn recalculate(&mut self) {
        let mut deterministic = ResultCounts::default();
        let mut agent = ResultCounts::default();
        let mut possible = 0.0;
        let mut earned = 0.0;

        for finding in &self.findings {
            match finding.evaluation_method {
                EvaluationMethod::Deterministic => {
                    deterministic.add(finding.result);
                    if finding.result != ResultStatus::Skip {
                        let weight = finding.severity.weight();
                        possible += weight;
                        earned += match finding.result {
                            ResultStatus::Pass => weight,
                            ResultStatus::Warn => weight / 2.0,
                            _ => 0.0,
                        };
                    }
                }
                EvaluationMethod::AiAgent => agent.add(finding.result),
            }
        }
        let score = if possible == 0.0 {
            100
        } else {
            (earned * 100.0 / possible).round() as u8
        };
        self.deterministic = DeterministicSummary {
            score,
            counts: deterministic,
        };
        self.ai_agent = agent;
    }

    pub fn has_failures(&self) -> bool {
        self.deterministic.counts.fail > 0 || self.ai_agent.fail > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observation_and_report_are_serializable() {
        let observation = Observation {
            args: vec!["--help".into()],
            exit_status: Some(0),
            timed_out: false,
            duration_ms: 4.2,
            stdout: "Usage: tool".into(),
            stderr: String::new(),
            has_ansi: false,
        };
        let evidence = serde_json::to_value(&observation).unwrap();
        assert_eq!(evidence["exit_status"], 0);

        let counts = ResultCounts::default();
        let report = Report {
            format_version: 1,
            tool_version: "0.0.2".into(),
            packages: vec![PackageIdentity {
                name: "clilint".into(),
                version: "0.0.2".into(),
            }],
            target: "tool".into(),
            deterministic: DeterministicSummary::default(),
            ai_agent: counts,
            findings: Vec::new(),
        };
        let encoded = serde_json::to_string(&report).unwrap();
        let decoded: Report = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.format_version, 1);
    }
}
