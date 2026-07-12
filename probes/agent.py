"""Probes for the `agent` category: features that make a CLI usable unattended.

These check what an agent or script needs: a stable machine-readable output mode
and a way to run without interactive prompts. Both are detected from help text,
since that is where a CLI advertises its contract.
"""

from clilint.model import Finding, Status

HANDLES = ["CLI-AGENT-001", "CLI-AGENT-002"]

_STRUCTURED = ("--json", "--format", "--plain", "--output", "-o ")
_UNATTENDED = ("--no-input", "--non-interactive", "--yes", "--force", "--batch", "-y")


def run(target):
    help_text = target.run(["--help"]).stdout.lower()
    findings = []

    structured = any(token in help_text for token in _STRUCTURED)
    findings.append(
        Finding(
            "CLI-AGENT-001",
            Status.PASS if structured else Status.WARN,
            "advertises a machine-readable output mode"
            if structured
            else "no machine-readable output mode (e.g. --json) advertised in help",
            {"structured": structured},
        )
    )

    unattended = any(token in help_text for token in _UNATTENDED)
    findings.append(
        Finding(
            "CLI-AGENT-002",
            Status.PASS if unattended else Status.WARN,
            "advertises a non-interactive/unattended flag"
            if unattended
            else "no unattended flag (e.g. --no-input/--yes) advertised in help",
            {"unattended": unattended},
        )
    )

    return findings
