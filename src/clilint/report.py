"""Render a CLI Lint Report as human text, machine-parseable plain text, or JSON."""

from __future__ import annotations

import json

_COLORS = {
    "pass": "\x1b[32m",   # green
    "warn": "\x1b[33m",   # yellow
    "fail": "\x1b[31m",   # red
    "skip": "\x1b[90m",   # grey
}
_RESET = "\x1b[0m"


def _paint(status: str, color: bool) -> str:
    if not color:
        return status
    return f"{_COLORS.get(status, '')}{status}{_RESET}"


def format_human(report: dict, color: bool = False) -> str:
    """A readable summary, in the style of the CLI Lint report example."""
    lines = [
        report["standard"],
        f"Profile: {report['profile']}",
        f"Target:  {report['target']}",
        "",
        f"Score: {report['score']}/100",
        "",
    ]
    width = max((len(f["rule"]) for f in report["findings"]), default=0)
    for f in report["findings"]:
        row = f"{f['rule']:<{width}}   {_paint(f['status'], color):<5}"
        if f["status"] in ("warn", "fail") and f["detail"]:
            row += f"   {f['detail']}"
        lines.append(row.rstrip())

    s = report["summary"]
    lines += [
        "",
        f"{s['pass']} pass, {s['warn']} warn, {s['fail']} fail, {s['skip']} skip "
        f"({s['total']} rules)",
    ]
    return "\n".join(lines) + "\n"


def format_plain(report: dict) -> str:
    """Tab-separated, one finding per line — stable for scripts and pipes.

    Columns: rule, status, severity, category, detail.
    """
    lines = []
    for f in report["findings"]:
        detail = f["detail"].replace("\t", " ").replace("\n", " ")
        lines.append("\t".join([f["rule"], f["status"], f["severity"], f["category"], detail]))
    return "\n".join(lines) + ("\n" if lines else "")


def format_json(report: dict) -> str:
    return json.dumps(report, indent=2, ensure_ascii=False) + "\n"
