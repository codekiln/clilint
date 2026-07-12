# clilint

**A CLI conformance linter.** Point `clilint` at a command-line tool and get a reproducible
conformance score plus machine-readable, actionable findings — so both humans and LLM coding agents
can iterate a CLI toward a shared standard.

> This CLI scores 84 under CLI Lint 0.1.

`clilint` is early-stage. This repository holds both the **CLI Lint** standard and its reference
linter.

## What it is

Most CLIs are snowflakes: each invents its own flags, help layout, error format, and output modes.
That makes every CLI harder to learn — for people and for agents. **CLI Lint** turns established
command-line best practice into explicit, versioned, testable criteria, and `clilint` measures a
real executable against them.

The prose standard supports the tool rather than being the only deliverable: the operative promise
is *generate or modify a CLI, run this tool, and receive a reproducible conformance score with
actionable findings.*

## Vocabulary

- **CLI Lint** — the standard (the full standard-and-tooling project).
- **CLI Lint rules** — the individual, versioned criteria, identified by a `CLI-` prefix.
- **CLI Lint score** — a 0–100 conformance score for a given tool under a profile.
- **CLI Lint conformance** — meeting the rules for a chosen profile at a chosen version.
- **`clilint`** — the reference command.

## Commands

```text
clilint check ./my-cli      # run the rules against an executable, print findings
clilint score ./my-cli      # print just the conformance score
clilint explain CLI-OUTPUT-003   # explain a rule and how to satisfy it
```

## Example

```text
$ clilint check ./target/debug/mytool

CLI Lint 0.1
Profile: generated-cli

Score: 84/100

CLI-HELP-002       pass
CLI-OUTPUT-004     fail    No stable structured-output mode
CLI-ERROR-003      fail    Error output lacks a machine-readable code
CLI-SAFETY-006     pass
CLI-AGENT-002      warn    Help output does not expose side effects
```

## The workflow

`clilint` is built for a repair loop that a human or an agent can drive:

```text
generate or modify a CLI
        ↓
clilint exercises the executable
        ↓
machine-readable findings and score
        ↓
repair the violations
        ↓
clilint verifies conformance
```

```bash
clilint check ./mytool \
  --profile generated-cli \
  --format json \
  --output clilint-report.json
```

The JSON report contains stable rule identifiers, evidence, severity, score impact, and remediation
instructions, so an agent can iterate without having to interpret prose diagnostics.

## Conformance profiles

The initial profile is `generated-cli` (a CLI newly written or modified, often by an agent). The
resulting CLI must remain good for humans — the agent is the implementer and evaluator, not the sole
user. Additional profiles (for example `modern-cli`) can raise or relax rules for different contexts.

## Repository layout

```text
clilint/
├── spec/            # Normative CLI conformance standard (prose)
├── rules/           # Machine-readable rule definitions
├── src/             # Reference linter
├── probes/          # Behavioral CLI tests
├── schemas/         # Result and manifest schemas
├── prompts/         # Instructions for LLM coding agents
├── fixtures/        # Conformant and nonconformant example CLIs
└── integrations/    # GitHub Actions, pre-commit, agent skills
```

## Sources

CLI Lint distills and makes testable the best practices described by:

- [Command Line Interface Guidelines (clig.dev)](https://clig.dev/) — the primary source.
- [12 Factor CLI Apps](https://jdxcode.medium.com/12-factor-cli-apps-dd3c227a0e46) by Jeff Dickey.

Related standards and conventions it draws on include the
[POSIX Utility Conventions](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html),
the [GNU Coding Standards](https://www.gnu.org/prep/standards/html_node/Program-Behavior.html), and
the [Heroku CLI Style Guide](https://devcenter.heroku.com/articles/cli-style-guide).

## License

[MIT](LICENSE).
