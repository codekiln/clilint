#!/usr/bin/env python3
"""Mechanistic Checker CLI prototype."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys


CHECK_ID = "prototype/help/default-output"
VERSION = "0.1.0"


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(
        description="Check whether default output points to full help."
    )
    result.add_argument("--version", action="version", version=VERSION)
    return result


def main() -> int:
    parser().parse_args()
    request = json.load(sys.stdin)
    project_directory = Path(request["project_directory"]).resolve()
    if Path.cwd().resolve() != project_directory:
        raise ValueError("Checker did not inherit the requested project directory")

    expectation = Path(__file__).with_name("expectation.txt").resolve()
    expectation_text = expectation.read_text().strip()
    completed = subprocess.run(
        request["target"],
        cwd=project_directory,
        capture_output=True,
        check=False,
        text=True,
        timeout=5,
    )
    mentions_help = "--help" in completed.stdout
    messages = []
    if not mentions_help:
        messages.append(
            {
                "level": "error",
                "message": expectation_text,
                "evidence": {
                    "command": request["target"],
                    "stdout": completed.stdout,
                    "stderr": completed.stderr,
                    "exit_status": completed.returncode,
                },
            }
        )

    print("mechanistic Checker gathered target output", file=sys.stderr)
    json.dump(
        {
            "format_version": 1,
            "request_id": request["request_id"],
            "check_id": CHECK_ID,
            "method": "mechanistic",
            "outcome": "result",
            "working_directory": str(Path.cwd().resolve()),
            "checker_resource": str(expectation),
            "result": {
                "score": 4.0 if mentions_help else 2.0,
                "messages": messages,
            },
        },
        sys.stdout,
    )
    print()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
