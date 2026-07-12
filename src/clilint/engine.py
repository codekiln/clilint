"""Discover probes, run them against a target, and assemble a scored report."""

from __future__ import annotations

import importlib.util
from pathlib import Path

from . import STANDARD, __version__
from .catalog import rule_weight
from .model import Finding, Status


def discover_probes(probes_dir: Path) -> list:
    """Load every ``probes/*.py`` module by file path.

    Probes are loaded from the filesystem rather than imported as a package so the
    probe directory can be overridden. Each module imports ``clilint`` normally.
    """
    modules = []
    for f in sorted(Path(probes_dir).glob("*.py")):
        if f.name.startswith("_"):
            continue
        spec = importlib.util.spec_from_file_location(f"clilint_probe_{f.stem}", f)
        module = importlib.util.module_from_spec(spec)
        assert spec and spec.loader
        spec.loader.exec_module(module)
        modules.append(module)
    return modules


def run_probes(target, applicable: set, probes_dir: Path) -> dict:
    """Run the probes covering ``applicable`` rules; return ``{rule_id: Finding}``."""
    findings: dict = {}
    for module in discover_probes(probes_dir):
        handles = set(getattr(module, "HANDLES", []))
        covered = handles & applicable
        if not covered:
            continue
        try:
            results = module.run(target)
        except Exception as exc:  # a broken probe should not sink the whole run
            for rid in covered:
                findings[rid] = Finding(rid, Status.SKIP, f"probe raised {type(exc).__name__}: {exc}")
            continue
        for finding in results:
            if finding.rule in applicable:
                findings[finding.rule] = finding

    # Any applicable rule no probe spoke to is recorded as skipped, not silently dropped.
    for rid in applicable:
        findings.setdefault(rid, Finding(rid, Status.SKIP, "no probe produced a result"))
    return findings


def score(findings: dict, catalog: dict) -> int:
    """Weighted conformance score in ``0..100``. pass=full, warn=half, fail=0, skip=excluded."""
    total = 0.0
    earned = 0.0
    for rid, finding in findings.items():
        if finding.status is Status.SKIP:
            continue
        weight = rule_weight(catalog[rid])
        total += weight
        if finding.status is Status.PASS:
            earned += weight
        elif finding.status is Status.WARN:
            earned += weight * 0.5
    if total == 0:
        return 100
    return round(100 * earned / total)


def build_report(target_label: str, profile: str, findings: dict, catalog: dict) -> dict:
    """Assemble the machine-readable CLI Lint Report."""
    counts = {s.value: 0 for s in Status}
    rows = []
    for rid in sorted(findings):
        finding = findings[rid]
        rule = catalog[rid]
        counts[finding.status.value] += 1
        rows.append(
            {
                "rule": rid,
                "category": rule.get("category", ""),
                "title": rule.get("title", ""),
                "severity": rule.get("severity", "warn"),
                "status": finding.status.value,
                "weight": rule_weight(rule),
                "detail": finding.detail,
                "remediation": rule.get("remediation", ""),
                "evidence": finding.evidence,
            }
        )
    counts["total"] = len(rows)
    return {
        "clilint_version": __version__,
        "standard": STANDARD,
        "profile": profile,
        "target": target_label,
        "score": score(findings, catalog),
        "summary": counts,
        "findings": rows,
    }
