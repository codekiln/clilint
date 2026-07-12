"""Probes for the `error` category: errors go to stderr, are informative, and guide."""

from clilint.model import Finding, Status

HANDLES = ["CLI-ERROR-001", "CLI-ERROR-002", "CLI-ERROR-003"]

BOGUS = "--clilint-nonexistent-flag-zzz"
_INFORMATIVE = ("unknown", "unrecognized", "invalid", "no such", "error", "usage", BOGUS)
_GUIDANCE = ("--help", "-h", "help", "usage", "try")


def run(target):
    bad = target.run([BOGUS])
    combined = f"{bad.stdout}\n{bad.stderr}".lower()
    findings = []

    on_stderr = bad.stderr.strip() != ""
    findings.append(
        Finding(
            "CLI-ERROR-001",
            Status.PASS if on_stderr else Status.FAIL,
            "error messages are written to stderr"
            if on_stderr
            else "no error text on stderr for an invalid flag",
            bad.as_evidence(),
        )
    )

    informative = any(token in combined for token in _INFORMATIVE)
    findings.append(
        Finding(
            "CLI-ERROR-002",
            Status.PASS if informative else Status.FAIL,
            "the error names the problem"
            if informative
            else "the error message does not explain what went wrong",
            {"informative": informative},
        )
    )

    guides = any(token in combined for token in _GUIDANCE)
    findings.append(
        Finding(
            "CLI-ERROR-003",
            Status.PASS if guides else Status.WARN,
            "the error points to help/usage"
            if guides
            else "the error does not suggest how to get help",
            {"guides": guides},
        )
    )

    return findings
