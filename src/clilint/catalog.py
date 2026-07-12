"""Load and query the rule catalog and conformance profiles from ``rules/``.

The catalog is the registry of every ``CLI-`` rule: its category, severity,
score weight, rationale, remediation, and the profiles it belongs to. Probes
produce findings that reference these ids; the engine cross-checks that every
finding maps to a known rule.
"""

from __future__ import annotations

import json
from pathlib import Path

from .model import DEFAULT_WEIGHT


class CatalogError(Exception):
    """Raised when the rule catalog or profiles are malformed."""


def load_catalog(rules_dir: Path) -> dict:
    """Return ``{rule_id: rule_dict}`` from every ``rules/*.json`` (except profiles)."""
    catalog: dict = {}
    files = sorted(Path(rules_dir).glob("*.json"))
    for f in files:
        if f.name == "profiles.json":
            continue
        data = json.loads(f.read_text(encoding="utf-8"))
        if not isinstance(data, list):
            raise CatalogError(f"{f}: expected a JSON array of rules")
        for rule in data:
            rid = rule.get("id")
            if not rid:
                raise CatalogError(f"{f}: a rule is missing its 'id'")
            if rid in catalog:
                raise CatalogError(f"duplicate rule id {rid!r} in {f}")
            catalog[rid] = rule
    if not catalog:
        raise CatalogError(f"no rules found in {rules_dir}")
    return catalog


def load_profiles(rules_dir: Path) -> dict:
    p = Path(rules_dir) / "profiles.json"
    if not p.exists():
        raise CatalogError(f"missing {p}")
    return json.loads(p.read_text(encoding="utf-8"))


def resolve_profile(profiles: dict, catalog: dict, name: str) -> set:
    """Return the set of rule ids active for ``name``.

    A profile may set ``rules`` to ``"all"`` or an explicit list, plus an optional
    ``exclude`` list. A rule that names ``profiles`` restricts itself to those.
    """
    if name not in profiles:
        raise CatalogError(f"unknown profile {name!r}; known: {', '.join(sorted(profiles))}")
    spec = profiles[name]
    include = spec.get("rules", "all")
    ids = set(catalog) if include == "all" else set(include)
    ids -= set(spec.get("exclude", []))

    active = set()
    for rid in ids:
        rule = catalog.get(rid)
        if rule is None:
            raise CatalogError(f"profile {name!r} references unknown rule {rid!r}")
        allowed = rule.get("profiles")
        if allowed and name not in allowed:
            continue
        active.add(rid)
    return active


def rule_weight(rule: dict) -> float:
    """A rule's score weight: explicit ``weight`` or the severity default."""
    if rule.get("weight") is not None:
        return float(rule["weight"])
    return DEFAULT_WEIGHT.get(rule.get("severity", "warn"), 1.0)
