"""Probes for the `robustness` category: responsiveness and not hanging."""

from clilint.model import Finding, Status

HANDLES = ["CLI-ROBUST-001", "CLI-ROBUST-002"]

# Help should feel instant. Startup time is part of a CLI's UX.
FAST_MS = 2000.0


def run(target):
    findings = []

    help_inv = target.run(["--help"])
    fast = (not help_inv.timed_out) and help_inv.duration_ms <= FAST_MS
    findings.append(
        Finding(
            "CLI-ROBUST-001",
            Status.PASS if fast else Status.WARN,
            f"`--help` responded in {help_inv.duration_ms:.0f}ms"
            if fast
            else f"`--help` was slow ({help_inv.duration_ms:.0f}ms > {FAST_MS:.0f}ms) or timed out",
            {"duration_ms": round(help_inv.duration_ms, 1), "timed_out": help_inv.timed_out},
        )
    )

    # Run with no args and a closed stdin: it must not block waiting for input.
    no_args = target.run([])
    ok = not no_args.timed_out
    findings.append(
        Finding(
            "CLI-ROBUST-002",
            Status.PASS if ok else Status.FAIL,
            "does not hang when run with no arguments"
            if ok
            else "hung waiting on input when run with no arguments",
            {"timed_out": no_args.timed_out, "exit_code": no_args.exit_code},
        )
    )

    return findings
