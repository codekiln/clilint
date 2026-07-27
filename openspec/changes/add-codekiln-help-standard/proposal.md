## Why

People and agents need a predictable way to find nested commands and read the
relevant part of a CLI tool's offline documentation. Programmatic callers
sometimes need extra instructions for using the same interface without
interactive output. Clilint also needs a complete custom check bundle that
shows bundle authors how to turn their own CLI preferences into reusable
checks.

## What Changes

- Add a scoped `codekiln-help` check bundle for a `help` subcommand at every
  command path.
- Give each Check in a local bundle one Checker CLI that owns setup, evidence
  gathering, and production of the Check Outcome without adding
  check-specific code to Clilint.
- Invoke each Checker CLI as an ordinary child process from the project
  directory, with a JSON request and outcome exchange.
- Define one Check Outcome, Check Result, Score, and Check Message format for
  mechanistic and judgment-based Checker CLIs. Reserve Assessment for
  judgment-based Checks.
- Keep the built-in checkers in place for this change while making their
  results use the shared output model.
- Use a file-based, two-pass Assessment handoff without choosing an AI model or
  agent harness.
- Build `codekiln-help` with the same bundle format and Checker CLI protocol
  available to other bundle authors.
- Document `codekiln-help` as a worked example of creating, testing, installing,
  and combining custom Clilint check bundles.
- Define offline command discovery, Markdown outlines, section retrieval,
  shared help for people and agents, and optional programmatic-use guidance.
- Add JSON output that lets clients copy command paths and sections
  between help operations.
- Add one `codekiln-help` check that discovers the tested CLI tool's command
  hierarchy, evaluates help at each command path, and returns one Score with
  Check Messages for the behaviors that do not meet the standard.
- Make `codekiln-help` independently installable from other future codekiln
  check bundles.
- **BREAKING** Rename the current public package, rule, and check terms to
  `check bundle`, `check`, and `checker`. The `0.0.x` project will not retain
  compatibility aliases for the old names.
- Preserve the speculative help implementations and their recorded results as
  research material for the change.

## Capabilities

### New Capabilities

- `check-entry-points`: Defines how an installed check bundle supplies a
  complete Checker CLI and returns a structured Check Outcome.
- `hierarchical-help-standard`: Defines the help interface and offline
  documentation behavior expected from a tested CLI tool that follows the
  standard.
- `hierarchical-help-checking`: Defines how Clilint discovers command paths,
  retrieves sections named by earlier help results, and reports check results
  for the hierarchical help interface.

### Modified Capabilities

- `conformance-packages`: Adds installation and composition of the named
  `codekiln-help` check bundle separately from the built-in core check bundle.
  The capability keeps its existing OpenSpec identifier because it modifies
  the existing specification.
- `deterministic-checking`: Renames the built-in mechanism that performs a
  check to a checker and distinguishes it from an installed bundle's Checker
  CLI.
- `conformance-reporting`: Renames package and rule fields, adds one Check
  Outcome per check, and allows a Check Result to contain several Check
  Messages.
- `agent-assessments`: Associates judgment-based Assessments with checks and
  lets a judgment-based Checker CLI use a bundled Agent Skill while owning
  evidence gathering, Assessment validation, and its Check Outcome.

## Impact

- Adds the scoped `codekiln-help` check bundle and the ability to install it
  separately from a local path.
- Changes check-bundle loading so Clilint can resolve a Checker CLI while the
  CLI resolves its own installed scripts, Skills, rubrics, and other resources.
- Changes checker execution and reports so one check can reuse evidence and
  return one Score with several Check Messages.
- Adds tested CLI fixtures for nested command hierarchies, shared help, optional
  programmatic guidance, JSON outlines, and offline browsing.
- Adds a check-bundle authoring guide based on the complete `codekiln-help`
  check bundle.
- Adds documentation for CLI authors whose tools adopt the help standard.
- Defines offline help as part of the tested CLI standard.

## Citations

- [My/Principle/CLI/Centricity/Offline Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___CLI___Centricity___Offline%20Tools.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Principle/Simplify/Minimize Surface Area](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Simplify___Minimize%20Surface%20Area.md)
- [My/Principle/Make Illegal States Unrepresentable](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20Illegal%20States%20Unrepresentable.md)
- [My/Principle/Make it Obvious](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20it%20Obvious.md)
