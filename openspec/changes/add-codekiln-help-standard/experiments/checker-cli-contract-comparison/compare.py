#!/usr/bin/env python3
"""Run the Checker CLI contract comparison."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


ROOT = Path(__file__).resolve().parent
PROJECT = ROOT / "fixture-project"
TARGET = PROJECT / "target_cli.py"
MECHANISTIC_CHECKER = ROOT / "mechanistic-check" / "check.py"
JUDGMENT_CHECKER = ROOT / "judgment-check" / "check.py"
ASSESSMENT = ROOT / "judgment-check" / "assessment.example.json"
REQUEST_ID = "checker-cli-contract-comparison"


def run_cli(
    checker: Path, request: dict[str, Any], *arguments: str
) -> tuple[dict[str, Any], str]:
    completed = subprocess.run(
        [sys.executable, str(checker), *arguments],
        cwd=PROJECT,
        input=json.dumps(request),
        capture_output=True,
        check=False,
        text=True,
        timeout=5,
    )
    if completed.returncode != 0:
        raise AssertionError(completed.stderr)
    return json.loads(completed.stdout), completed.stderr


def assert_cli_surface(checker: Path) -> None:
    for argument in ("--help", "--version"):
        completed = subprocess.run(
            [sys.executable, str(checker), argument],
            cwd=PROJECT,
            capture_output=True,
            check=False,
            text=True,
            timeout=5,
        )
        assert completed.returncode == 0, completed.stderr
        assert completed.stdout


def validate_result(outcome: dict[str, Any], method: str) -> None:
    assert outcome["format_version"] == 1
    assert outcome["request_id"] == REQUEST_ID
    assert outcome["method"] == method
    assert outcome["outcome"] == "result"
    result = outcome["result"]
    assert 0.0 <= result["score"] <= 4.0
    if result["score"] < 4.0:
        assert result["messages"]
    if result["score"] == 4.0:
        assert all(
            message["level"] == "info" for message in result["messages"]
        )


def main() -> int:
    request = {
        "format_version": 1,
        "request_id": REQUEST_ID,
        "project_directory": str(PROJECT.resolve()),
        "target": [sys.executable, str(TARGET.resolve())],
    }

    assert_cli_surface(MECHANISTIC_CHECKER)
    assert_cli_surface(JUDGMENT_CHECKER)

    mechanistic, mechanistic_logs = run_cli(MECHANISTIC_CHECKER, request)
    validate_result(mechanistic, "mechanistic")
    assert mechanistic["working_directory"] == str(PROJECT.resolve())
    assert mechanistic["checker_resource"].startswith(
        str(MECHANISTIC_CHECKER.parent.resolve())
    )
    assert mechanistic_logs

    awaiting, awaiting_logs = run_cli(JUDGMENT_CHECKER, request)
    assert awaiting["outcome"] == "awaiting_assessment"
    assert awaiting["working_directory"] == str(PROJECT.resolve())
    assert awaiting["assessment_request"]["skill"].startswith(
        str(JUDGMENT_CHECKER.parent.resolve())
    )
    assert awaiting["assessment_request"]["rubric"].startswith(
        str(JUDGMENT_CHECKER.parent.resolve())
    )
    assert awaiting_logs

    judgment, judgment_logs = run_cli(
        JUDGMENT_CHECKER, request, "--assessment", str(ASSESSMENT)
    )
    validate_result(judgment, "judgment-based")
    assert judgment["working_directory"] == str(PROJECT.resolve())
    assert judgment_logs

    print("mechanistic Checker CLI: project cwd -> Check Result")
    print("judgment Checker CLI:    project cwd -> Awaiting Assessment")
    print("judgment Checker CLI:    project cwd + Assessment -> Check Result")
    print("both Checker CLIs found package resources outside the project cwd")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
