# Project direction

Clilint helps people state what a command-line tool should do and gives coding
agents useful feedback while they build it.

A Check states one expectation about a project. A Checker gathers evidence
and returns a Score with messages explaining what could improve. A check
bundle groups related Checks so projects can reuse them.

Checks can ask whether `--help` exits successfully, whether a release pipeline
runs in CI, whether help teaches a likely task, or whether a changelog explains
important changes. Built-in checkers cover common mechanical behavior. Checks
added through local bundles use Checker CLIs. They can use scripts, other
tools, AI agents, or any programming language while Clilint keeps one request
and outcome format.

## Core and optional standards

The built-in `clilint` bundle is intended to grow into an opinionated superset
of the [Command Line Interface Guidelines](https://clig.dev/). Clilint should
automate the guidelines it can evaluate reliably.

Optional bundles add focused standards. A project installs the bundles it
wants, and a project bundle can add local expectations. The
[`codekiln-help` bundle](codekiln-help.md) checks navigable offline help and
serves as an example for bundle authors.

## Mechanistic and judgment-based Checks

A Check is mechanistic when its Score and Check Messages require no human or
model interpretation. Exit codes, output fields, and response times are common
mechanistic evidence.

A Check is judgment-based when human or model interpretation affects its Score
or messages. The Checker can still run scripts and other tools while gathering
evidence. An external agent applies a rubric and returns an Assessment, which
the Checker validates before producing the shared Check Result.

The report identifies the method used for each result. Scores express the
extent to which an expectation is met; Check Messages provide the specific
feedback needed to improve it.

## Reusable expectations

Check bundles make project preferences reusable. A person can give a coding
agent a bundle, ask it to build a CLI, and use the resulting Scores and
messages to guide improvement.

A future bundle-authoring workflow could study an existing CLI, propose
candidate expectations, and let a person keep the preferences that matter.
[Design explorations](design-explorations.md) records ideas that have not
become product requirements.
