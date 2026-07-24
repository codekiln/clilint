# Project direction

Clilint is a behavioral test harness for command-line tools. It runs a tested CLI tool in defined ways, captures what happens, and evaluates checks. A check is one testable expectation. A check bundle is a reusable group of related checks.

Most checks use deterministic checkers. They compute a result from captured evidence without asking an AI to make a judgment. For example, a checker can require `--help` to finish, exit successfully, and write help to stdout.

Other checks require judgment. An AI agent can apply a written rubric to evidence captured by Clilint. Reports keep deterministic and AI-assessed results separate and mark a judgment-based check `unassessed` until a matching assessment is attached.

This scope is broader than static analysis or conventional linting. Clilint executes the tested CLI tool, observes its behavior, applies deterministic checkers, and can ask for judgments that mechanical code should not pretend to make.

## Core checks and additional opinions

The built-in `clilint` check bundle is intended to grow into an opinionated superset of the [Command Line Interface Guidelines](https://clig.dev/). Clilint should automate as many of those guidelines as it can evaluate responsibly.

Optional bundles add standards that do not belong in the core. A project installs the bundles it wants, and an extension can add stricter local preferences without removing or weakening inherited checks. The [`codekiln-help` bundle](codekiln-help.md) is both an opinionated help standard and an example that other bundle authors can copy.

## Help that people and agents can explore

Agents are another accessibility case for command-line documentation. Default help should be useful to everyone. An optional programmatic route can add the equivalent of a ramp: instructions for piping, machine-readable output, retrieving one section, and avoiding pagers or full-screen interfaces.

`codekiln-help` checks that every command path exposes shared documentation, local discovery, outlines, sections, search, and safe non-interactive viewing. People and agents can browse the complete documentation offline without adding a website to the permission boundary.

## Reusable expectations

Check bundles make preferences portable between projects. A bundle can become part of a request to build a command-line tool: given these checks, create a tool for a particular job and show how it performs.

A future bundle-authoring workflow could learn from command-line tools that already provide a good experience. A person could point an AI at one tool or a related family, review the proposed expectations, reject accidental quirks, and keep the chosen preferences as checks.

That model requires clear check-bundle composition, stable check identities, inspectable evidence, and honest separation between deterministic results and AI judgments. [Design explorations](design-explorations.md) record candidate approaches that are not yet requirements.
