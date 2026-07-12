"""Probes for the `version` category: a discoverable, useful version string."""

import re

from clilint.model import Finding, Status

HANDLES = ["CLI-VERSION-001", "CLI-VERSION-002"]

_VERSION_RE = re.compile(r"\d+\.\d+")


def run(target):
    findings = []

    long_v = target.run(["--version"])
    combined = f"{long_v.stdout}\n{long_v.stderr}"
    ok = (not long_v.timed_out) and long_v.exit_code == 0 and bool(_VERSION_RE.search(combined))
    findings.append(
        Finding(
            "CLI-VERSION-001",
            Status.PASS if ok else Status.FAIL,
            "`--version` prints a version number and exits 0"
            if ok
            else "`--version` did not print a version like `1.2` and exit 0",
            long_v.as_evidence(),
        )
    )

    short_v = target.run(["-V"])
    ok_short = (
        (not short_v.timed_out)
        and short_v.exit_code == 0
        and bool(_VERSION_RE.search(f"{short_v.stdout}\n{short_v.stderr}"))
    )
    findings.append(
        Finding(
            "CLI-VERSION-002",
            Status.PASS if ok_short else Status.WARN,
            "`-V` is a version alias" if ok_short else "`-V` is not a version alias",
            short_v.as_evidence(),
        )
    )

    return findings
