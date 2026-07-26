#!/usr/bin/env python3
"""Fixture CLI for the Checker CLI contract comparison."""

from __future__ import annotations

import argparse


def main() -> int:
    parser = argparse.ArgumentParser(description="Serve an example project.")
    parser.add_argument("command", nargs="?")
    arguments = parser.parse_args()
    if arguments.command is None:
        print("Example CLI serves an example project.")
        print("Example: target_cli.py serve")
        return 0
    if arguments.command == "serve":
        print("serving")
        return 0
    parser.error(f"unknown command: {arguments.command}")


if __name__ == "__main__":
    raise SystemExit(main())
