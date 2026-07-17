# Project direction

Clilint aims to make expectations about command-line programs reusable and testable. A package should describe how a command is expected to behave, and Clilint should run the command and report where its behavior matches or departs from those expectations.

This direction is broader than one fixed checklist. A person or team should be able to choose a package of expectations when designing, generating, reviewing, or testing a command-line program.

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

Packages should make preferences portable between projects. A package could become part of a request to build a command-line tool: given these expectations, create a tool for a particular job and show that it conforms to them.

That model requires clear package composition, stable rule identities, evidence that another tool can inspect, and honest separation between repeatable checks and judgments that need an AI assessment. These are goals for the project rather than promises of the current release.
