# Agent guide: using clilint

You just generated or modified a command-line tool. Before you call the task
done, run it through `clilint` — a conformance linter that checks a CLI against a
standard set of rules for help text, versioning, output, error handling, and
robustness. This guide tells you how to run it and how to act on what it reports.

## What a good score means

`clilint` grades a target executable from 0 to 100 based on weighted rules. A
high score means the CLI behaves the way users and other programs expect: it has
usable `--help`, reports its version, writes errors to stderr, exits with correct
codes, and does not hang or misbehave on odd input. Treat the score as a quality
gate, not a vanity metric — the point is a CLI that is genuinely pleasant to use.

`clilint` is Python 3.10+, stdlib only, and runs from a checkout as
`./clilint` (or `python -m clilint` with `src/` on `PYTHONPATH`).

## Commands to run

For machine consumption, always prefer JSON:

```
./clilint check <target-executable> --format json
```

Other useful invocations:

- `./clilint check <target>` — human-readable findings (add `--no-color` for logs).
- `./clilint check <target> --min-score 90` — exit non-zero unless the score clears 90.
- `./clilint score <target>` — print just the integer score.
- `./clilint explain <RULE-ID>` — title, rationale, and remediation for a rule
  (add `--json` for structured output).

Useful flags on `check`: `--output FILE`, `--profile <name>`, `--timeout SECONDS`.
Profiles: `generated-cli` (default, all 17 rules) and `modern-cli` (excludes the
two agent-specific rules). `check` exits non-zero if any rule fails or the score
is below `--min-score`.

## Reading the JSON report

The report has `clilint_version`, `standard`, `profile`, `target`, `score`, a
`summary` object (`pass`, `warn`, `fail`, `skip`, `total`), and a `findings`
array. Iterate `findings` and act on each by its `status`:

- `fail` — the rule was violated. **Must fix.**
- `warn` — a soft violation. **Should fix.**
- `pass` / `skip` — no action needed.

Each finding carries the fields you need to fix it:

- `rule` (e.g. `CLI-HELP-001`), `category` (basics, help, version, output, error,
  robustness, agent), `severity` (error, warn, info), and `weight`.
- `detail` — what was observed.
- `remediation` — **use this as your fix instruction.**
- `evidence` — raw observations (exit codes, output snippets) explaining *why*
  the rule tripped. Read it to understand the failure before editing.

If a finding is unclear, run `./clilint explain <rule>` for the full rationale.

## How to fix

1. Fix `fail` findings first, prioritizing error-severity rules. These are the
   real conformance gaps.
2. Then address `warn` findings.
3. Group related fixes — e.g. handle all `help` findings in one edit, all `error`
   findings in another.
4. Re-run `clilint check` after each round to confirm the findings clear and the
   score rises. Repeat until there are no fails and the score meets your gate.

## Keep it good for humans

`clilint` optimizes for a CLI that works well for programs *and* people. Do not
game rules with output that satisfies the linter but confuses a human reader —
help text should still be readable, error messages still actionable, and output
still sensible at a terminal. A conformant CLI is the floor, not the ceiling.
