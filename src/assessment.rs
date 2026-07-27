use std::{fs, path::Path};

use crate::model::{Assessment, AssessmentProvenance, CheckResult, SkillRef};

pub fn load(path: &Path) -> Result<Assessment, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("could not read Assessment {}: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("invalid Assessment {}: {error}", path.display()))
}

pub fn validate(
    assessment: &Assessment,
    request_id: &str,
    check: &str,
    skill: &SkillRef,
    evidence_digest: &str,
) -> Result<CheckResult, String> {
    if assessment.format_version != 1 {
        return Err(format!(
            "Assessment for {} uses unsupported format version {}",
            assessment.check, assessment.format_version
        ));
    }
    if assessment.request_id != request_id {
        return Err(format!(
            "Assessment for {} belongs to request {}, expected {}",
            assessment.check, assessment.request_id, request_id
        ));
    }
    if assessment.check != check {
        return Err(format!(
            "Assessment references Check {}, expected {check}",
            assessment.check
        ));
    }
    if &assessment.skill != skill {
        return Err(format!(
            "Assessment for {} uses Skill {} {}, expected {} {}",
            assessment.check,
            assessment.skill.name,
            assessment.skill.version,
            skill.name,
            skill.version
        ));
    }
    if assessment.evidence_digest != evidence_digest {
        return Err(format!(
            "Assessment for {} has stale evidence digest {}, expected {}",
            assessment.check, assessment.evidence_digest, evidence_digest
        ));
    }
    let result = CheckResult {
        score: assessment.score,
        messages: assessment.messages.clone(),
        assessment: Some(AssessmentProvenance {
            skill: assessment.skill.clone(),
            explanation: assessment.explanation.clone(),
            assessor: assessment.assessor.clone(),
        }),
    };
    result.validate()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CheckMessage, CheckMessageLevel, Score};

    fn skill() -> SkillRef {
        SkillRef {
            name: "assess-cli-help".into(),
            version: "1.0.0".into(),
        }
    }

    fn assessment() -> Assessment {
        Assessment {
            format_version: 1,
            request_id: "request-1".into(),
            check: "clilint/help/useful-example".into(),
            evidence_digest: "sha256:abc".into(),
            skill: skill(),
            score: Score::new(3.5).unwrap(),
            messages: vec![CheckMessage {
                level: CheckMessageLevel::Warning,
                message: "Add an example of a likely task.".into(),
                evidence: serde_json::Value::Null,
            }],
            explanation: "The help provides a useful but incomplete example.".into(),
            assessor: Some("test".into()),
        }
    }

    #[test]
    fn validates_matching_assessment() {
        let result = validate(
            &assessment(),
            "request-1",
            "clilint/help/useful-example",
            &skill(),
            "sha256:abc",
        )
        .unwrap();
        assert_eq!(result.score.value(), 3.5);
    }

    #[test]
    fn rejects_stale_assessment() {
        let mut document = assessment();
        document.evidence_digest = "sha256:stale".into();
        assert!(
            validate(
                &document,
                "request-1",
                "clilint/help/useful-example",
                &skill(),
                "sha256:abc",
            )
            .unwrap_err()
            .contains("stale")
        );
    }

    #[test]
    fn rejects_wrong_request() {
        let document = assessment();
        assert!(
            validate(
                &document,
                "request-2",
                "clilint/help/useful-example",
                &skill(),
                "sha256:abc",
            )
            .unwrap_err()
            .contains("expected request-2")
        );
    }

    #[test]
    fn rejects_wrong_check() {
        let document = assessment();
        assert!(
            validate(
                &document,
                "request-1",
                "clilint/help/another-check",
                &skill(),
                "sha256:abc",
            )
            .unwrap_err()
            .contains("expected clilint/help/another-check")
        );
    }

    #[test]
    fn rejects_wrong_skill() {
        let mut document = assessment();
        document.skill.name = "other".into();
        assert!(
            validate(
                &document,
                "request-1",
                "clilint/help/useful-example",
                &skill(),
                "sha256:abc",
            )
            .unwrap_err()
            .contains("expected")
        );
    }
}
