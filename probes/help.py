"""Probes for the `help` category: discoverability through --help / -h."""

from clilint.model import Finding, Status

HANDLES = ["CLI-HELP-001", "CLI-HELP-002", "CLI-HELP-003", "CLI-HELP-004"]


def run(target):
    findings = []

    long_help = target.run(["--help"])
    ok = (not long_help.timed_out) and long_help.exit_code == 0 and long_help.stdout.strip() != ""
    findings.append(
        Finding(
            "CLI-HELP-001",
            Status.PASS if ok else Status.FAIL,
            "`--help` prints help to stdout and exits 0"
            if ok
            else "`--help` did not print to stdout with exit code 0",
            long_help.as_evidence(),
        )
    )

    short_help = target.run(["-h"])
    ok_short = (
        (not short_help.timed_out) and short_help.exit_code == 0 and short_help.stdout.strip() != ""
    )
    findings.append(
        Finding(
            "CLI-HELP-002",
            Status.PASS if ok_short else Status.WARN,
            "`-h` behaves like `--help`" if ok_short else "`-h` is not a help alias",
            short_help.as_evidence(),
        )
    )

    text = long_help.stdout.lower()
    has_usage = "usage" in text or "synopsis" in text
    findings.append(
        Finding(
            "CLI-HELP-003",
            Status.PASS if has_usage else Status.FAIL,
            "help includes a usage synopsis"
            if has_usage
            else "help output has no usage/synopsis line",
            {"has_usage": has_usage},
        )
    )

    has_example = "example" in text
    findings.append(
        Finding(
            "CLI-HELP-004",
            Status.PASS if has_example else Status.WARN,
            "help includes examples" if has_example else "help output shows no examples",
            {"has_example": has_example},
        )
    )

    return findings
