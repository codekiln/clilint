#!/usr/bin/env python3
"""Mechanistic assessment phase for the extension model experiment."""

from __future__ import annotations

import json
import sys


CHECK_ID = "cli-guidelines/help/concise-default"


def main() -> int:
    evidence = json.load(sys.stdin)
    output = evidence["observations"]["no-arguments"]["stdout"]
    findings = []
    if "Example:" not in output:
        findings.append(
            {
                "severity": "error",
                "expectation": "Default help contains an example invocation.",
                "observed": "The default output contains no Example section.",
                "evidence_refs": ["observations.no-arguments.stdout"],
                "evaluation_method": "mechanistic",
            }
        )
    if "--help" not in output:
        findings.append(
            {
                "severity": "error",
                "expectation": "Default help tells the caller how to request full help.",
                "observed": "The default output does not mention --help.",
                "evidence_refs": ["observations.no-arguments.stdout"],
                "evaluation_method": "mechanistic",
            }
        )

    json.dump(
        {
            "format_version": 1,
            "check": CHECK_ID,
            "rating": "acceptable" if not findings else "minimal",
            "evidence": evidence,
            "findings": findings,
        },
        sys.stdout,
        indent=2,
    )
    print()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
