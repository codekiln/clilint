#!/usr/bin/env python3
"""Evidence phase for the phase-oriented extension model experiment."""

from __future__ import annotations

import json
import subprocess
import sys
from typing import Any


def observe(target: list[str], arguments: list[str]) -> dict[str, Any]:
    completed = subprocess.run(
        [*target, *arguments],
        capture_output=True,
        check=False,
        text=True,
        timeout=5,
    )
    return {
        "args": arguments,
        "exit_status": completed.returncode,
        "stdout": completed.stdout,
        "stderr": completed.stderr,
    }


def main() -> int:
    request = json.load(sys.stdin)
    target = request["target"]
    json.dump(
        {
            "format_version": 1,
            "observations": {
                "no-arguments": observe(target, []),
                "full-help": observe(target, ["--help"]),
            },
        },
        sys.stdout,
        indent=2,
    )
    print()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
