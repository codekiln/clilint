# Project direction

Clilint is intended to be a behavioral test harness for command-line programs. It runs a command in defined ways, captures what happens, and evaluates rules: testable expectations about how the command should behave. A Clilint package is a reusable collection of those rules.

Some rules use mechanical checks, called deterministic checks in the current reports. They compute a result without asking an AI to make a judgment. For example, a rule can require `--help` to finish, exit successfully, and write help text to standard output.

Other rules are judgment-based. For example, a rule can ask whether the help teaches a likely task with a useful example. Each judgment-based rule should have a rubric that describes the evidence to consider and what result levels such as pass, warning, failure, or unable to judge mean. An AI agent can apply that rubric to evidence captured by Clilint.

This scope is broader than static analysis or conventional linting. Clilint executes the target, observes its behavior, applies mechanical checks, and can ask an AI to evaluate qualities that need judgment.

The project is not limited to one fixed checklist. A person or team should be able to choose a package of expectations when designing, generating, reviewing, or testing a command-line program.

## Core guidelines and additional opinions

The built-in package is intended to grow into an opinionated superset of the [Command Line Interface Guidelines](https://clig.dev/). Clilint should automate as many of those guidelines as can be evaluated responsibly. It can then add defaults for needs that the guidelines do not fully cover.

The current release implements only part of that direction. It has one built-in package and accepts a local package that adds rules or makes an inherited rule more severe. The package format will need to develop before it can represent every kind of preference or combine several independently maintained packages.

## Help that people and agents can explore

One possible package could define richer documentation discovery for people and AI agents. Expectations might include:

- `--help` gives a concise introduction and the most common actions;
- every command provides a `help` subcommand;
- the help subcommand provides an overview or table of contents before deeper detail;
- detailed information remains available from the command line; and
- a documentation website complements the command-line help when one exists.

The exact interface is not settled. Open questions include the options accepted by a help subcommand, how help is organized across nested commands, and how command-line and website documentation stay consistent.

## Reusable expectations

Packages should make preferences portable between projects. A package could become part of a request to build a command-line tool: given these expectations, create a tool for a particular job and show how it performs against them.

A future package-authoring workflow could also learn from existing command-line tools. A person could point an AI at one well-designed tool or a related family of tools. The AI could exercise their interfaces, identify repeated expectations, and propose mechanical and judgment-based rules. After review, that package could test a new tool so that its help, errors, output, and interaction patterns feel like they belong to the same family.

The goal is to reproduce chosen expectations, not every observed detail. A generated package should distinguish behavior it observed from preferences it inferred, and a person should be able to reject accidental quirks before treating them as reusable rules.

That model requires clear package composition, stable rule identities, evidence that another tool can inspect, and honest separation between mechanical checks and judgments made by AI. These are goals for the project rather than promises of the current release.

The operational design for AI assessments and even the project name remain unsettled. [Design explorations](design-explorations.md) record candidate approaches and open questions without treating them as requirements.
