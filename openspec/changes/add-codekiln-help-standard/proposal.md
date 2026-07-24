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
- Build `codekiln-help` with the same bundle format, supported checkers, and
  inheritance available to other bundle authors.
- Document `codekiln-help` as a worked example of creating, testing, installing,
  and extending a custom Clilint check bundle.
- Define offline command discovery, Markdown outlines, section retrieval,
  search, human viewing, web-page mapping, shared help for people and agents,
  and optional programmatic-use guidance.
- Add JSON output that lets clients copy command paths and sections
  between help operations.
- Add a Clilint check that discovers the tested CLI tool's command hierarchy
  and evaluates help at each command path.
- Make `codekiln-help` independently installable from other future codekiln
  check bundles.
- **BREAKING** Rename the current public package, rule, and check terms to
  `check bundle`, `check`, and `checker`. The `0.0.x` project will not retain
  compatibility aliases for the old names.
- Preserve the speculative help implementations and their findings as research
  material for the change.

## Capabilities

### New Capabilities

- `hierarchical-help-standard`: Defines the help interface and offline
  documentation behavior expected from a tested CLI tool that follows the
  standard.
- `hierarchical-help-checking`: Defines how Clilint discovers command paths,
  retrieves sections named by earlier help results, and reports check results for the
  hierarchical help interface.

### Modified Capabilities

- `conformance-packages`: Adds installation and composition of the named
  `codekiln-help` check bundle separately from the built-in core check bundle.
  The capability keeps its existing identifier while this change is open.
- `deterministic-checking`: Renames the mechanism that performs checks from a
  check type to a checker.
- `conformance-reporting`: Renames package and rule fields in JSON reports to
  check-bundle and check fields.
- `agent-assessments`: Associates AI assessments with checks instead of rules.

## Impact

- Adds the scoped `codekiln-help` check bundle and the ability to install it by
  name.
- Changes check-bundle loading, checker execution, and report evidence so one help
  command can use output from an earlier help command.
- Adds tested CLI fixtures for nested command hierarchies, shared help, optional
  programmatic guidance, JSON outlines, offline browsing, and interactive
  versus non-interactive viewing.
- Adds a check-bundle authoring guide based on the complete `codekiln-help`
  check bundle.
- Adds documentation for CLI authors whose tools adopt the help standard.
- Requires no network access while Clilint runs the checks.

## Citations

- [My/Principle/CLI/Centricity/Offline Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___CLI___Centricity___Offline%20Tools.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Principle/Simplify/Minimize Surface Area](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Simplify___Minimize%20Surface%20Area.md)
