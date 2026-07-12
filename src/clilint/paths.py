"""Filesystem locations of clilint's own data directories.

clilint is designed to run from a checkout of its repository (``./clilint`` or
``python -m clilint``). The rule catalog, probes, and schemas live in top-level
directories alongside ``src/``; these paths locate them relative to the package.
"""

from __future__ import annotations

from pathlib import Path

PACKAGE_DIR = Path(__file__).resolve().parent          # src/clilint
SRC_DIR = PACKAGE_DIR.parent                            # src
REPO_ROOT = SRC_DIR.parent                              # repository root

RULES_DIR = REPO_ROOT / "rules"
PROBES_DIR = REPO_ROOT / "probes"
SCHEMAS_DIR = REPO_ROOT / "schemas"
