#!/usr/bin/env python3
"""Judgment-based Checker CLI prototype."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys
from typing import Any


CHECK_ID = "prototype/docs/first-task-clarity"
VERSION = "0.1.0"


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(
        description="Judge whether project documentation gives a clear first task."
    )
    result.add_argument("--version", action="version", version=VERSION)
    result.add_argument("--assessment", type=Path)
    return result


def validate_assessment(
    assessment: dict[str, Any], request: dict[str, Any]
) -> None:
    assert assessment["format_version"] == 1
    assert assessment["request_id"] == request["request_id"]
    assert assessment["check_id"] == CHECK_ID
    score = assessment["score"]
    assert isinstance(score, float)
    assert 0.0 <= score <= 4.0
    messages = assessment["messages"]
    if score < 4.0:
        assert messages
    if score == 4.0:
        assert all(message["level"] == "info" for message in messages)
    assert all(
        message["level"] in {"info", "warning", "error"}
        for message in messages
    )


def main() -> int:
    arguments = parser().parse_args()
    request = json.load(sys.stdin)
    project_directory = Path(request["project_directory"]).resolve()
    if Path.cwd().resolve() != project_directory:
        raise ValueError("Checker did not inherit the requested project directory")

    checker_directory = Path(__file__).resolve().parent
    skill = checker_directory / "SKILL.md"
    rubric = checker_directory / "rubric.md"
    evidence = {
        "readme": (project_directory / "README.md").read_text(),
    }

    print("judgment Checker gathered project documentation", file=sys.stderr)
    if arguments.assessment is None:
        json.dump(
            {
                "format_version": 1,
                "request_id": request["request_id"],
                "check_id": CHECK_ID,
                "method": "judgment-based",
                "outcome": "awaiting_assessment",
                "working_directory": str(Path.cwd().resolve()),
                "assessment_request": {
                    "skill": str(skill),
                    "rubric": str(rubric),
                    "evidence": evidence,
                },
            },
            sys.stdout,
        )
        print()
        return 0

    assessment = json.loads(arguments.assessment.read_text())
    validate_assessment(assessment, request)
    json.dump(
        {
            "format_version": 1,
            "request_id": request["request_id"],
            "check_id": CHECK_ID,
            "method": "judgment-based",
            "outcome": "result",
            "working_directory": str(Path.cwd().resolve()),
            "assessment": assessment,
            "result": {
                "score": assessment["score"],
                "messages": assessment["messages"],
            },
        },
        sys.stdout,
    )
    print()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
