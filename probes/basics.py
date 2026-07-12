"""Probes for the `basics` category: exit codes and the stdout/stderr split."""

from clilint.model import Finding, Status

HANDLES = ["CLI-BASICS-001", "CLI-BASICS-002", "CLI-BASICS-003"]

# A flag no real tool defines, used to provoke an argument error.
BOGUS = "--clilint-nonexistent-flag-zzz"


def run(target):
    findings = []

    # BASICS-001: a successful invocation exits 0. --help or --version counts.
    help_inv = target.run(["--help"])
    ver_inv = target.run(["--version"])
    success = (not help_inv.timed_out and help_inv.exit_code == 0) or (
        not ver_inv.timed_out and ver_inv.exit_code == 0
    )
    findings.append(
        Finding(
            "CLI-BASICS-001",
            Status.PASS if success else Status.FAIL,
            "a successful invocation exits 0"
            if success
            else "neither `--help` nor `--version` exited 0",
            {"help_exit": help_inv.exit_code, "version_exit": ver_inv.exit_code},
        )
    )

    # BASICS-002: an invalid flag is an error → non-zero exit.
    bad = target.run([BOGUS])
    errored = (not bad.timed_out) and bad.exit_code not in (0, None)
    findings.append(
        Finding(
            "CLI-BASICS-002",
            Status.PASS if errored else Status.FAIL,
            "an unknown flag exits non-zero"
            if errored
            else "an unknown flag was accepted with a zero exit code",
            bad.as_evidence(),
        )
    )

    # BASICS-003: help is primary output → it goes to stdout, not stderr.
    on_stdout = help_inv.stdout.strip() != "" and len(help_inv.stdout) >= len(help_inv.stderr)
    findings.append(
        Finding(
            "CLI-BASICS-003",
            Status.PASS if on_stdout else Status.FAIL,
            "help text is written to stdout"
            if on_stdout
            else "help text is not on stdout (empty or dominated by stderr)",
            {"stdout_len": len(help_inv.stdout), "stderr_len": len(help_inv.stderr)},
        )
    )

    return findings
