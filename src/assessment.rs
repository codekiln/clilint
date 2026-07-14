use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::{AssessmentProvenance, EvaluationMethod, Report, ResultStatus, SkillRef};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssessmentDocument {
    pub format_version: u32,
    pub rule: String,
    pub result: ResultStatus,
    pub explanation: String,
    pub skill: SkillRef,
    pub evidence_digest: String,
    #[serde(default)]
    pub assessor: Option<String>,
}

pub fn load(path: &Path) -> Result<AssessmentDocument, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("could not read assessment {}: {error}", path.display()))?;
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("json") => serde_json::from_str(&text)
            .map_err(|error| format!("invalid assessment {}: {error}", path.display())),
        _ => toml::from_str(&text)
            .map_err(|error| format!("invalid assessment {}: {error}", path.display())),
    }
}

pub fn attach(report: &mut Report, document: AssessmentDocument) -> Result<(), String> {
    if document.format_version != 1 {
        return Err(format!(
            "assessment for {} uses unsupported format version {}",
            document.rule, document.format_version
        ));
    }
    if document.result == ResultStatus::Unassessed {
        return Err(format!(
            "assessment for {} cannot use result unassessed",
            document.rule
        ));
    }
    let finding = report
        .findings
        .iter_mut()
        .find(|finding| finding.rule == document.rule)
        .ok_or_else(|| format!("assessment references unknown rule {}", document.rule))?;
    if finding.evaluation_method != EvaluationMethod::AiAgent {
        return Err(format!(
            "assessment rule {} is not an AI-agent rule",
            document.rule
        ));
    }
    if finding.assessment.is_some() {
        return Err(format!(
            "more than one assessment was supplied for rule {}",
            document.rule
        ));
    }
    let expected_skill = finding
        .required_skill
        .as_ref()
        .ok_or_else(|| format!("rule {} has no required skill", document.rule))?;
    if &document.skill != expected_skill {
        return Err(format!(
            "assessment for {} uses skill {} {}, expected {} {}",
            document.rule,
            document.skill.name,
            document.skill.version,
            expected_skill.name,
            expected_skill.version
        ));
    }
    if document.evidence_digest != finding.evidence_digest {
        return Err(format!(
            "assessment for {} has stale evidence digest {}, expected {}",
            document.rule, document.evidence_digest, finding.evidence_digest
        ));
    }

    finding.result = document.result;
    finding.detail = document.explanation;
    finding.assessment = Some(AssessmentProvenance {
        skill: document.skill,
        assessor: document.assessor,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DeterministicSummary, Finding, PackageIdentity, ResultCounts, Severity};

    fn report() -> Report {
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
            findings: vec![Finding {
                rule: "clilint/help/useful-example".into(),
                title: "Useful help".into(),
                severity: Severity::Warn,
                evaluation_method: EvaluationMethod::AiAgent,
                result: ResultStatus::Unassessed,
                detail: String::new(),
                evidence: serde_json::json!({"help": "text"}),
                evidence_digest: "sha256:abc".into(),
                required_skill: Some(SkillRef {
                    name: "assess-cli-help".into(),
                    version: "1.0.0".into(),
                }),
                assessment: None,
            }],
        }
    }

    fn document() -> AssessmentDocument {
        AssessmentDocument {
            format_version: 1,
            rule: "clilint/help/useful-example".into(),
            result: ResultStatus::Pass,
            explanation: "The example teaches a likely task.".into(),
            skill: SkillRef {
                name: "assess-cli-help".into(),
                version: "1.0.0".into(),
            },
            evidence_digest: "sha256:abc".into(),
            assessor: Some("test".into()),
        }
    }

    #[test]
    fn attaches_matching_assessment() {
        let mut report = report();
        attach(&mut report, document()).unwrap();
        assert_eq!(report.findings[0].result, ResultStatus::Pass);
    }

    #[test]
    fn rejects_stale_assessment() {
        let mut report = report();
        let mut document = document();
        document.evidence_digest = "sha256:stale".into();
        assert!(attach(&mut report, document).unwrap_err().contains("stale"));
        assert_eq!(report.findings[0].result, ResultStatus::Unassessed);
    }

    #[test]
    fn rejects_wrong_skill() {
        let mut report = report();
        let mut document = document();
        document.skill.name = "other".into();
        assert!(
            attach(&mut report, document)
                .unwrap_err()
                .contains("expected")
        );
    }
}
