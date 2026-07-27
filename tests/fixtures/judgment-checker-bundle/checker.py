#!/usr/bin/env python3
import hashlib
import json
from pathlib import Path
import sys


request = json.load(sys.stdin)
checker_directory = Path(__file__).resolve().parent
skill_path = checker_directory / "SKILL.md"
evidence = {
    "readme": (Path(request["project_directory"]) / "README.md").read_text()
}
evidence_digest = "sha256:" + hashlib.sha256(
    json.dumps(evidence, sort_keys=True, separators=(",", ":")).encode()
).hexdigest()
skill = {"name": "fixture-judgment", "version": "1.0.0"}
base = {
    "format_version": 1,
    "request_id": request["request_id"],
    "check": request["check"],
    "method": "judgment-based",
}
assessment = request.get("assessment")
if not skill_path.is_file():
    base.update(
        {
            "outcome": "error",
            "error": {
                "message": "The Checker could not find its Agent Skill.",
            },
        }
    )
elif assessment is None:
    base.update(
        {
            "outcome": "awaiting-assessment",
            "assessment_request": {
                "request_id": request["request_id"],
                "evidence_digest": evidence_digest,
                "skill": skill,
                "rubric": (checker_directory / "rubric.md").read_text(),
                "evidence": {
                    **evidence,
                    "skill_path": str(skill_path),
                },
            },
        }
    )
elif (
    assessment.get("request_id") != request["request_id"]
    or assessment.get("check") != request["check"]
    or assessment.get("evidence_digest") != evidence_digest
    or assessment.get("skill") != skill
):
    base.update(
        {
            "outcome": "error",
            "error": {
                "message": "Assessment does not match the pending request.",
            },
        }
    )
else:
    base.update(
        {
            "outcome": "result",
            "result": {
                "score": assessment["score"],
                "messages": assessment["messages"],
                "assessment": {
                    "skill": assessment["skill"],
                    "explanation": assessment["explanation"],
                    "assessor": assessment.get("assessor"),
                },
            },
        }
    )
print(json.dumps(base))
