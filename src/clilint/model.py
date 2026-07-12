"""Core data types shared between the engine, probes, and reporters."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum


class Status(str, Enum):
    """The outcome of evaluating one rule against a target."""

    PASS = "pass"
    WARN = "warn"
    FAIL = "fail"
    SKIP = "skip"


# Default score weight per severity, used when a rule does not set an explicit weight.
DEFAULT_WEIGHT = {"error": 3.0, "warn": 1.0, "info": 0.5}


@dataclass
class Finding:
    """A probe's verdict on a single rule.

    ``rule`` is a ``CLI-`` identifier that must exist in the catalog. ``evidence``
    holds the raw observations (exit codes, captured output snippets, timings) that
    justify the status, so a report is auditable and an agent can act on it.
    """

    rule: str
    status: Status
    detail: str = ""
    evidence: dict = field(default_factory=dict)
