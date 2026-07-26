# Check extension model comparison

This experiment compares two ways for an installed check bundle to add a check
without changing the Clilint binary:

1. one entry point owns the complete check lifecycle; and
2. the check manifest names each lifecycle phase separately.

Both prototypes use the same check and return the same assessment structure.
They are design probes, not proposed bundle formats.

## Concrete check

The check applies the CLI Guidelines requirement to display concise help when a
program that requires arguments runs without them. The source says that concise
help should contain:

- a description of the program;
- one or two example invocations;
- descriptions of flags when the list is manageable; and
- an instruction to pass `--help` for more information.

Source: `content/_index.md` in the `cli-guidelines/cli-guidelines` repository,
under `Display concise help text by default`.

The fixture CLI deliberately omits the instruction to pass `--help`. That
supports a mechanistic error finding. Judging whether its description and
example are useful requires the rubric and can be done by an agent.

The check has no setup phase. Neither prototype adds an unused setup hook. The
phase-oriented format can add an optional `setup` phase when a concrete check
needs one.

## Shared lifecycle

```text
optional setup
      |
      v
gather evidence
  - run the target without arguments
  - run the target with --help
      |
      v
assess
  - mechanistic code, or
  - an agent reading evidence and rubric
      |
      v
assessment
  - overall rating
  - warning and error findings
  - evidence references
```

The check author chooses the logical scope of the check. A check may verify
several related things against the evidence it gathered.

## Prototype A: one entry point

`whole-check-entrypoint/` gives Clilint one entry point for the complete
lifecycle.

- `mechanistic.toml` runs `check.py`.
- `agent.toml` delegates the complete lifecycle to `SKILL.md`.

The host contract is small: Clilint supplies a check request, and the entry
point returns an assessment. Setup and evidence gathering are private to the
entry point.

## Prototype B: phase-oriented

`phase-oriented/` declares evidence gathering and assessment as separate
phases.

- Both manifests run `gather.py` and receive an evidence document.
- `mechanistic.toml` runs `assess.py`.
- `agent.toml` delegates assessment to
  `skills/assess-concise-default-help/SKILL.md`.

Clilint owns the transition between phases. It can retain evidence before
mechanistic code or an agent judges it.

## Running the comparison

Run from this directory:

```sh
python3 compare.py
```

The script:

1. parses all four manifests;
2. runs the mechanistic form of each prototype against `shared/fixture_cli.py`;
3. validates the example agent assessment;
4. checks that every referenced skill exists; and
5. prints each execution plan.

The prototypes use only the Python standard library.

## Result

See [findings.md](findings.md) for the drafting agent's comparison and proposed
answer to question 16. `codekiln` has not accepted that answer.
