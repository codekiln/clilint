use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckBundleIdentity {
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
    pub fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warn => 1,
            Self::Error => 2,
        }
    }

    pub fn message_level(self) -> CheckMessageLevel {
        match self {
            Self::Error => CheckMessageLevel::Error,
            Self::Warn => CheckMessageLevel::Warning,
            Self::Info => CheckMessageLevel::Info,
        }
    }

    pub fn failed_score(self) -> Score {
        Score::new(match self {
            Self::Error => 0.0,
            Self::Warn => 2.0,
            Self::Info => 3.0,
        })
        .expect("built-in scores are valid")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvaluationMethod {
    Mechanistic,
    JudgmentBased,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RatingLevel {
    Poor,
    Minimal,
    Acceptable,
    Good,
    Excellent,
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
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub args: Vec<String>,
    pub exit_status: Option<i32>,
    pub timed_out: bool,
    pub duration_ms: f64,
    pub stdout: String,
    pub stderr: String,
    pub has_ansi: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Score(f64);

impl Score {
    pub fn new(value: f64) -> Result<Self, String> {
        if value.is_finite() && (0.0..=4.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(format!(
                "Score must be a finite number from 0.0 through 4.0, got {value}"
            ))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Serialize for Score {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

impl fmt::Display for Score {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:.2}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckMessageLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckMessage {
    pub level: CheckMessageLevel,
    pub message: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub evidence: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssessmentProvenance {
    pub skill: SkillRef,
    pub explanation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckResult {
    pub score: Score,
    #[serde(default)]
    pub messages: Vec<CheckMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<AssessmentProvenance>,
}

impl CheckResult {
    pub fn validate(&self) -> Result<(), String> {
        if self.score.value() < 4.0
            && !self
                .messages
                .iter()
                .any(|message| !message.message.trim().is_empty())
        {
            return Err(
                "a Score below 4.0 requires a Check Message explaining what could improve"
                    .to_owned(),
            );
        }
        if self.score.value() == 4.0
            && self
                .messages
                .iter()
                .any(|message| message.level != CheckMessageLevel::Info)
        {
            return Err(
                "a Score of 4.0 cannot contain a Warning or Error Check Message".to_owned(),
            );
        }
        if self
            .messages
            .iter()
            .any(|message| message.message.trim().is_empty())
        {
            return Err("a Check Message cannot be empty".to_owned());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckError {
    pub message: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub checker_logs: String,
    #[serde(default)]
    pub logs_truncated: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssessmentRequest {
    pub request_id: String,
    pub evidence_digest: String,
    pub skill: SkillRef,
    pub rubric: String,
    pub evidence: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "kebab-case")]
pub enum CheckOutcome {
    Result {
        result: CheckResult,
    },
    Error {
        error: CheckError,
    },
    AwaitingAssessment {
        assessment_request: AssessmentRequest,
    },
    Skipped {
        reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckRecord {
    pub check: String,
    pub title: String,
    pub method: EvaluationMethod,
    #[serde(flatten)]
    pub outcome: CheckOutcome,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReportSummary {
    pub check_results: u32,
    pub check_errors: u32,
    pub awaiting_assessment: u32,
    pub skipped: u32,
    pub info_messages: u32,
    pub warning_messages: u32,
    pub error_messages: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub format_version: u32,
    pub tool_version: String,
    pub check_bundles: Vec<CheckBundleIdentity>,
    pub target: String,
    pub checks: Vec<CheckRecord>,
    pub summary: ReportSummary,
}

impl Report {
    pub fn recalculate(&mut self) {
        let mut summary = ReportSummary::default();
        for check in &self.checks {
            match &check.outcome {
                CheckOutcome::Result { result } => {
                    summary.check_results += 1;
                    for message in &result.messages {
                        match message.level {
                            CheckMessageLevel::Info => summary.info_messages += 1,
                            CheckMessageLevel::Warning => summary.warning_messages += 1,
                            CheckMessageLevel::Error => summary.error_messages += 1,
                        }
                    }
                }
                CheckOutcome::Error { .. } => summary.check_errors += 1,
                CheckOutcome::AwaitingAssessment { .. } => summary.awaiting_assessment += 1,
                CheckOutcome::Skipped { .. } => summary.skipped += 1,
            }
        }
        self.summary = summary;
    }

    pub fn has_failures(&self) -> bool {
        self.summary.check_errors > 0 || self.summary.error_messages > 0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CheckRequest {
    pub format_version: u32,
    pub request_id: String,
    pub check_bundle: CheckBundleIdentity,
    pub check: String,
    pub target: Vec<String>,
    pub project_directory: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Assessment>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckerResponse {
    pub format_version: u32,
    pub request_id: String,
    pub check: String,
    pub method: EvaluationMethod,
    #[serde(flatten)]
    pub outcome: CheckOutcome,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Assessment {
    pub format_version: u32,
    pub request_id: String,
    pub check: String,
    pub evidence_digest: String,
    pub skill: SkillRef,
    pub score: Score,
    pub messages: Vec<CheckMessage>,
    pub explanation: String,
    #[serde(default)]
    pub assessor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_rejects_non_finite_and_out_of_range_values() {
        assert!(Score::new(f64::NAN).is_err());
        assert!(Score::new(-0.1).is_err());
        assert!(Score::new(4.1).is_err());
        assert_eq!(Score::new(2.75).unwrap().value(), 2.75);
    }

    #[test]
    fn imperfect_result_requires_improvement_feedback() {
        let result = CheckResult {
            score: Score::new(3.5).unwrap(),
            messages: Vec::new(),
            assessment: None,
        };
        assert!(result.validate().is_err());
    }

    #[test]
    fn perfect_result_rejects_warning_or_error_messages() {
        let result = CheckResult {
            score: Score::new(4.0).unwrap(),
            messages: vec![CheckMessage {
                level: CheckMessageLevel::Warning,
                message: "unexpected warning".into(),
                evidence: serde_json::Value::Null,
            }],
            assessment: None,
        };
        assert!(result.validate().is_err());
    }

    #[test]
    fn outcomes_are_distinct_serialized_states() {
        let outcome = CheckOutcome::Error {
            error: CheckError {
                message: "checker failed".into(),
                checker_logs: String::new(),
                logs_truncated: false,
            },
        };
        let value = serde_json::to_value(outcome).unwrap();
        assert_eq!(value["outcome"], "error");
        assert!(value.get("result").is_none());
    }

    #[test]
    fn summary_counts_messages_separately_from_results() {
        let mut report = Report {
            format_version: 3,
            tool_version: "test".into(),
            check_bundles: Vec::new(),
            target: "target".into(),
            checks: vec![CheckRecord {
                check: "bundle/check".into(),
                title: "Check".into(),
                method: EvaluationMethod::Mechanistic,
                outcome: CheckOutcome::Result {
                    result: CheckResult {
                        score: Score::new(1.5).unwrap(),
                        messages: vec![
                            CheckMessage {
                                level: CheckMessageLevel::Error,
                                message: "First error".into(),
                                evidence: serde_json::Value::Null,
                            },
                            CheckMessage {
                                level: CheckMessageLevel::Error,
                                message: "Second error".into(),
                                evidence: serde_json::Value::Null,
                            },
                            CheckMessage {
                                level: CheckMessageLevel::Warning,
                                message: "One warning".into(),
                                evidence: serde_json::Value::Null,
                            },
                        ],
                        assessment: None,
                    },
                },
            }],
            summary: ReportSummary::default(),
        };
        report.recalculate();
        assert_eq!(report.summary.check_results, 1);
        assert_eq!(report.summary.error_messages, 2);
        assert_eq!(report.summary.warning_messages, 1);
        assert!(report.has_failures());
    }

    #[test]
    fn zero_score_with_only_a_warning_is_not_a_process_failure() {
        let mut report = Report {
            format_version: 3,
            tool_version: "test".into(),
            check_bundles: Vec::new(),
            target: "target".into(),
            checks: vec![CheckRecord {
                check: "bundle/check".into(),
                title: "Check".into(),
                method: EvaluationMethod::Mechanistic,
                outcome: CheckOutcome::Result {
                    result: CheckResult {
                        score: Score::new(0.0).unwrap(),
                        messages: vec![CheckMessage {
                            level: CheckMessageLevel::Warning,
                            message: "Improve this behavior".into(),
                            evidence: serde_json::Value::Null,
                        }],
                        assessment: None,
                    },
                },
            }],
            summary: ReportSummary::default(),
        };
        report.recalculate();
        assert!(!report.has_failures());
    }
}
