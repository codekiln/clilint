"""The ``clilint`` command: check / score / explain.

This CLI aims to conform to the same standard it enforces — help and primary
output go to stdout, diagnostics to stderr, it offers ``--json`` and ``--plain``
machine-readable modes, honors ``NO_COLOR`` and non-TTY output, and returns
meaningful exit codes.
"""

from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path

from . import STANDARD, __version__, catalog as cat, paths, report as rep
from .engine import build_report, run_probes


def _epilog() -> str:
    return (
        "Examples:\n"
        "  clilint check ./my-cli               Check an executable and print findings\n"
        "  clilint check ./my-cli --format json Emit a machine-readable report\n"
        "  clilint score ./my-cli               Print just the conformance score\n"
        "  clilint explain CLI-OUTPUT-001       Explain a rule and how to satisfy it\n"
    )


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="clilint",
        description=f"A CLI conformance linter — the reference implementation of {STANDARD}.",
        epilog=_epilog(),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--version", "-V", action="version", version=f"clilint {__version__}"
    )
    sub = parser.add_subparsers(dest="command", metavar="<command>")

    def add_target_opts(p: argparse.ArgumentParser) -> None:
        p.add_argument("target", help="path to the CLI executable to lint")
        p.add_argument(
            "--profile", default="generated-cli", help="conformance profile (default: generated-cli)"
        )
        p.add_argument(
            "--timeout", type=float, default=10.0, help="per-invocation timeout in seconds"
        )
        p.add_argument("--rules-dir", type=Path, default=paths.RULES_DIR, help=argparse.SUPPRESS)
        p.add_argument("--probes-dir", type=Path, default=paths.PROBES_DIR, help=argparse.SUPPRESS)

    p_check = sub.add_parser("check", help="run the rules and report findings")
    add_target_opts(p_check)
    p_check.add_argument(
        "--format", choices=["human", "plain", "json"], default="human", help="output format"
    )
    p_check.add_argument("--output", "-o", type=Path, help="write the report to a file")
    p_check.add_argument("--no-color", action="store_true", help="disable colored output")
    p_check.add_argument(
        "--min-score", type=int, default=None, help="fail (exit 1) if the score is below this"
    )

    p_score = sub.add_parser("score", help="print only the conformance score")
    add_target_opts(p_score)
    p_score.add_argument(
        "--min-score", type=int, default=None, help="fail (exit 1) if the score is below this"
    )

    p_explain = sub.add_parser("explain", help="explain a rule by its CLI- id")
    p_explain.add_argument("rule", help="rule id, e.g. CLI-OUTPUT-001")
    p_explain.add_argument("--rules-dir", type=Path, default=paths.RULES_DIR, help=argparse.SUPPRESS)
    p_explain.add_argument("--json", action="store_true", help="emit the rule as JSON")

    return parser


def _use_color(no_color_flag: bool) -> bool:
    if no_color_flag or os.environ.get("NO_COLOR") is not None:
        return False
    return sys.stdout.isatty()


def _load(rules_dir: Path):
    catalog = cat.load_catalog(rules_dir)
    profiles = cat.load_profiles(rules_dir)
    return catalog, profiles


def _lint(args):
    from .runner import Target

    catalog, profiles = _load(args.rules_dir)
    applicable = cat.resolve_profile(profiles, catalog, args.profile)
    target_path = args.target
    if not Path(target_path).exists():
        # Allow commands on PATH; only warn-guard obvious local-path mistakes.
        if ("/" in target_path or "\\" in target_path):
            raise FileNotFoundError(target_path)
    target = Target([target_path], timeout=args.timeout)
    findings = run_probes(target, applicable, args.probes_dir)
    return build_report(target_path, args.profile, findings, catalog)


def _cmd_check(args) -> int:
    report = _lint(args)
    if args.format == "json":
        text = rep.format_json(report)
    elif args.format == "plain":
        text = rep.format_plain(report)
    else:
        text = rep.format_human(report, color=_use_color(args.no_color))

    if args.output:
        args.output.write_text(text, encoding="utf-8")
        print(f"wrote report to {args.output}", file=sys.stderr)
    else:
        sys.stdout.write(text)

    return _gate(report, args.min_score)


def _cmd_score(args) -> int:
    report = _lint(args)
    print(report["score"])
    return _gate(report, args.min_score)


def _cmd_explain(args) -> int:
    catalog = cat.load_catalog(args.rules_dir)
    rule = catalog.get(args.rule)
    if rule is None:
        print(f"error: unknown rule {args.rule!r}. Run 'clilint check' to see rule ids.", file=sys.stderr)
        return 2
    if args.json:
        import json

        print(json.dumps(rule, indent=2, ensure_ascii=False))
        return 0
    print(f"{rule['id']}  [{rule.get('category', '')}/{rule.get('severity', '')}]")
    print(f"  {rule.get('title', '')}")
    if rule.get("rationale"):
        print(f"\nWhy:\n  {rule['rationale']}")
    if rule.get("remediation"):
        print(f"\nHow to satisfy:\n  {rule['remediation']}")
    if rule.get("source"):
        print("\nSource:\n  " + ", ".join(rule["source"]))
    return 0


def _gate(report: dict, min_score) -> int:
    if report["summary"]["fail"] > 0:
        return 1
    if min_score is not None and report["score"] < min_score:
        print(
            f"score {report['score']} is below --min-score {min_score}",
            file=sys.stderr,
        )
        return 1
    return 0


def main(argv=None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    if not args.command:
        parser.print_help()
        return 0
    try:
        handler = {"check": _cmd_check, "score": _cmd_score, "explain": _cmd_explain}[args.command]
        return handler(args)
    except FileNotFoundError as exc:
        print(f"error: target not found: {exc}", file=sys.stderr)
        return 2
    except cat.CatalogError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
