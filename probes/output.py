"""Probes for the `output` category: color is disabled when output is not a TTY."""

from clilint.runner import has_ansi
from clilint.model import Finding, Status

HANDLES = ["CLI-OUTPUT-001"]


def run(target):
    # Subprocess pipes are never a TTY, so a well-behaved CLI must emit no ANSI
    # color codes here. We scan the output of several common invocations.
    invocations = {
        "--help": target.run(["--help"]),
        "(no args)": target.run([]),
        "--version": target.run(["--version"]),
    }
    offenders = [
        label
        for label, inv in invocations.items()
        if has_ansi(inv.stdout) or has_ansi(inv.stderr)
    ]
    ok = not offenders
    return [
        Finding(
            "CLI-OUTPUT-001",
            Status.PASS if ok else Status.FAIL,
            "no ANSI color codes in non-TTY output"
            if ok
            else f"ANSI color codes emitted to a pipe by: {', '.join(offenders)}",
            {"offenders": offenders},
        )
    ]
