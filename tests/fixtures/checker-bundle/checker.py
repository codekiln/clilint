#!/usr/bin/env python3
import json
from pathlib import Path
import subprocess
import sys


request = json.load(sys.stdin)
project_directory = Path(request["project_directory"]).resolve()
resource = Path(__file__).with_name("expectation.txt")
completed = subprocess.run(
    request["target"],
    cwd=project_directory,
    capture_output=True,
    check=False,
    stdin=subprocess.DEVNULL,
    text=True,
    timeout=5,
)
passed = "--help" in completed.stdout
messages = []
if not passed:
    messages.append(
        {
            "level": "warning",
            "message": resource.read_text().strip(),
            "evidence": {
                "cwd": str(Path.cwd().resolve()),
                "resource": str(resource.resolve()),
                "stdout": completed.stdout,
            },
        }
    )
print("fixture Checker ran", file=sys.stderr)
print(
    json.dumps(
        {
            "format_version": 1,
            "request_id": request["request_id"],
            "check": request["check"],
            "method": "mechanistic",
            "outcome": "result",
            "result": {
                "score": 4.0 if passed else 2.5,
                "messages": messages,
            },
        }
    )
)
