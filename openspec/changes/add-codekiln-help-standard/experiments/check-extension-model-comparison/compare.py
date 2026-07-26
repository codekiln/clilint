#!/usr/bin/env python3
"""Run and validate the check extension model comparison."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import tomllib
from typing import Any


ROOT = Path(__file__).resolve().parent
TARGET = ROOT / "shared" / "fixture_cli.py"
CHECK_ID = "cli-guidelines/help/concise-default"
RATINGS = {"poor", "minimal", "acceptable", "good", "excellent"}


def load_manifest(path: Path) -> dict[str, Any]:
    with path.open("rb") as stream:
        manifest = tomllib.load(stream)
    assert manifest["format_version"] == 1
    assert manifest["check"] == CHECK_ID
    assert (path.parent / manifest["rubric"]).is_file()
    assert (path.parent / manifest["output_contract"]).is_file()
    return manifest


def command(command: list[str], payload: dict[str, Any], cwd: Path) -> dict[str, Any]:
    resolved = [sys.executable if command[0] == "python3" else command[0], *command[1:]]
    completed = subprocess.run(
        resolved,
        cwd=cwd,
        input=json.dumps(payload),
        capture_output=True,
        check=False,
        text=True,
        timeout=5,
    )
    if completed.returncode != 0:
        raise AssertionError(completed.stderr)
    return json.loads(completed.stdout)


def evidence_value(evidence: dict[str, Any], reference: str) -> Any:
    value: Any = evidence
    for part in reference.split("."):
        value = value[part]
    return value


def validate_assessment(assessment: dict[str, Any]) -> None:
    assert assessment["format_version"] == 1
    assert assessment["check"] == CHECK_ID
    assert assessment["rating"] in RATINGS
    evidence = assessment["evidence"]
    assert evidence["format_version"] == 1
    assert set(evidence["observations"]) == {"no-arguments", "full-help"}
    for finding in assessment["findings"]:
        assert finding["severity"] in {"warning", "error"}
        assert finding["expectation"]
        assert finding["observed"]
        assert finding["evaluation_method"] in {"mechanistic", "llm"}
        assert finding["evidence_refs"]
        for reference in finding["evidence_refs"]:
            assert evidence_value(evidence, reference) is not None


def whole_check_prototype(request: dict[str, Any]) -> dict[str, Any]:
    directory = ROOT / "whole-check-entrypoint"
    mechanistic = load_manifest(directory / "mechanistic.toml")
    agent = load_manifest(directory / "agent.toml")
    assert mechanistic["protocol"] == agent["protocol"] == "clilint-check-v1"
    assert mechanistic["entrypoint"]["runner"] == "command"
    assert agent["entrypoint"]["runner"] == "agent-skill"
    assert (directory / agent["entrypoint"]["path"]).is_file()
    assessment = command(mechanistic["entrypoint"]["command"], request, directory)
    validate_assessment(assessment)
    return assessment


def phase_oriented_prototype(request: dict[str, Any]) -> dict[str, Any]:
    directory = ROOT / "phase-oriented"
    mechanistic = load_manifest(directory / "mechanistic.toml")
    agent = load_manifest(directory / "agent.toml")
    assert mechanistic["protocol"] == agent["protocol"] == "clilint-check-phases-v1"
    mechanistic_phases = mechanistic["phases"]
    agent_phases = agent["phases"]
    assert [phase["name"] for phase in mechanistic_phases] == [
        "gather-evidence",
        "assess",
    ]
    assert mechanistic_phases[0] == agent_phases[0]
    assert mechanistic_phases[1]["runner"] == "command"
    assert agent_phases[1]["runner"] == "agent-skill"
    assert (directory / agent_phases[1]["path"]).is_file()

    evidence = command(mechanistic_phases[0]["command"], request, directory)
    assessment = command(mechanistic_phases[1]["command"], evidence, directory)
    validate_assessment(assessment)
    return assessment


def main() -> int:
    request = {
        "format_version": 1,
        "check": CHECK_ID,
        "target": [sys.executable, str(TARGET)],
    }
    whole = whole_check_prototype(request)
    phases = phase_oriented_prototype(request)
    assert whole == phases

    with (ROOT / "shared" / "agent-assessment.example.json").open() as stream:
        agent_assessment = json.load(stream)
    validate_assessment(agent_assessment)

    print("whole-check mechanistic: command -> assessment-v1")
    print("whole-check agent:       agent-skill -> assessment-v1")
    print("phase mechanistic:       command -> evidence-v1 -> command -> assessment-v1")
    print("phase agent:             command -> evidence-v1 -> agent-skill -> assessment-v1")
    print("all prototype manifests and example assessments are valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
