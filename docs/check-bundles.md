# Check bundles

A check bundle is a named, versioned collection of checks. Clilint includes
the core `clilint` bundle. A project can install local bundles that add checks
without changing the Clilint binary.

## Install a local bundle

Run the commands from the directory containing the tested project:

```sh
clilint bundle install ../my-checks
clilint bundle list
clilint check ./my-cli
```

Clilint writes `.clilint/config.toml` in the current directory and stores the
bundle path relative to that directory. It reads that file during later
checks. This version reads one project directory and does not search parent
directories.

```text
clilint bundle install <local-path>
clilint bundle list [--json]
clilint bundle remove <name>
```

## Define a bundle-owned check

Create `clilint.toml` in the bundle directory:

```toml
format_version = 1
extends = "clilint"

[check_bundle]
name = "team"
version = "1.0.0"

[[checks]]
id = "team/help/team-option"
title = "Help describes the team option"
severity = "warn"
evaluation_method = "mechanistic"

[checks.checker]
type = "cli"
command = ["python3", "{bundle}/checker.py"]
```

Every bundle-owned check declares one Checker CLI command. Clilint replaces
the literal `{bundle}` placeholder with the bundle directory and starts the
command directly, without a shell. The Checker runs from the directory where
the user invoked Clilint and finds its own bundled scripts, rubrics, Skills,
and other resources.

The command may name a native executable or an interpreter-backed CLI. The
bundle author is responsible for making its runtime available on supported
systems.

## Exchange a request and outcome

Clilint writes one JSON Check Request to the Checker's standard input:

```json
{
  "format_version": 1,
  "request_id": "sha256:...",
  "check_bundle": {"name": "team", "version": "1.0.0"},
  "check": "team/help/team-option",
  "target": ["./my-cli"],
  "project_directory": "/work/project"
}
```

The Checker writes one JSON Check Outcome to standard output. Operational logs
go to standard error.

```json
{
  "format_version": 1,
  "request_id": "sha256:...",
  "check": "team/help/team-option",
  "method": "mechanistic",
  "outcome": "result",
  "result": {
    "score": 2.75,
    "messages": [
      {
        "level": "warning",
        "message": "Describe --team in the help output.",
        "evidence": {"stdout": "Usage: my-cli"}
      }
    ]
  }
}
```

A Score is a finite number from `0.0` through `4.0`. A Score below `4.0`
requires at least one useful Check Message. A Score of `4.0` can contain Info
messages and cannot contain Warning or Error messages.

A Checker that cannot produce a result returns a Check Error without a Score.
Clilint also creates a Check Error when a Checker times out, exits
unsuccessfully, exceeds the protocol-output or retained-log limit, or returns
invalid JSON. Clilint keeps only a bounded amount of Checker logs and marks
the Check Error when it truncated them.

## Use judgment

A judgment-based Checker uses `"method": "judgment-based"`. It may gather
evidence and return Awaiting Assessment. An external agent writes an
Assessment JSON file, and a later Clilint invocation supplies that file to the
same Checker:

```sh
clilint check ./my-cli --assessment ./assessment.json
```

The Checker validates the request, evidence, Skill, Score, and messages before
returning a Check Result. See [Judgment-based Assessments](ai-assessments.md)
for the complete file workflow.

## Trust and composition

An installed local bundle contains executable code and receives the same
environment and operating-system permissions as Clilint. Review a bundle
before installing it.

An extension adds checks to its parent. It can make an inherited check more
severe and cannot remove, replace, or weaken inherited checks. Clilint
validates the complete bundle before running the tested CLI tool.

The built-in bundle at
[`check-bundles/clilint/clilint.toml`](../check-bundles/clilint/clilint.toml)
shows the built-in Rust checkers. The
[`codekiln-help` bundle](../check-bundles/codekiln-help/clilint.toml) and its
[implementation guide](codekiln-help.md) show a complete bundle-owned Checker
CLI.
