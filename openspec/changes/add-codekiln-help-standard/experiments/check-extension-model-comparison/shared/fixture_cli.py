#!/usr/bin/env python3
"""Fixture CLI for the check extension model experiment."""

from __future__ import annotations

import sys


DEFAULT_HELP = """kiln-demo - work with hosted repositories

Usage: kiln-demo <command>

Example:
  kiln-demo repo list
"""

FULL_HELP = """kiln-demo - work with hosted repositories

Usage: kiln-demo <command>

Commands:
  repo list    List repositories visible to the current credentials

Options:
  -h, --help   Show full help

Example:
  kiln-demo repo list
"""


def main(arguments: list[str]) -> int:
    if arguments in (["-h"], ["--help"]):
        print(FULL_HELP, end="")
        return 0
    if not arguments:
        print(DEFAULT_HELP, end="")
        return 0
    print(f"unknown command: {' '.join(arguments)}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
