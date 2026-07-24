## ADDED Requirements

### Requirement: Checks in codekiln-help
The `codekiln-help` check bundle SHALL define deterministic checks for help availability,
command discovery, document outlines, section retrieval, shared help,
programmatic guidance, offline search, non-interactive output, local viewing,
and web-page resolution.

#### Scenario: Check a CLI tool that follows the standard
- **WHEN** a user checks a nested CLI tool that follows the standard with the `codekiln-help` check bundle
- **THEN** the report contains passing deterministic findings for the hierarchical help checks

### Requirement: Hierarchical help checker
Clilint SHALL provide a deterministic checker that discovers command paths
by following immediate child commands in JSON help output and uses JSON output
from one help command as input to later help commands.
Check-bundle validation SHALL reject unsupported hierarchical help checks before
running the tested CLI tool.

#### Scenario: Follow a discovered section
- **WHEN** a JSON outline returns a valid `section` value
- **THEN** Clilint passes that exact value to a later section invocation

#### Scenario: Reject an unknown check
- **WHEN** a check bundle declares an unsupported hierarchical help check
- **THEN** Clilint rejects the check bundle before invoking the tested CLI tool

### Requirement: Reused help results
When several checks need the same help command, Clilint SHALL run that command
once during a check and reuse its result. Clilint SHALL retain a separate
finding for each check.

#### Scenario: Several checks use one command hierarchy
- **WHEN** command-discovery, outline, and section checks need the same command hierarchy
- **THEN** Clilint reuses the recorded JSON help observations and evaluates separate findings from them

### Requirement: Recursive command-path checking
Clilint SHALL evaluate applicable help checks at every valid command path
returned by recursively following child commands in JSON help.

#### Scenario: Failure at a deeply nested command
- **WHEN** `tool repo clone` is advertised but lacks its required help outline
- **THEN** the relevant finding identifies the `repo clone` command path and the failed invocation

### Requirement: JSON help validation
Clilint SHALL validate the JSON format version, command paths, heading levels,
`section` values, and search results before using them in later commands.

#### Scenario: Outline contains an invalid heading level
- **WHEN** a heading record contains a level outside the Markdown heading range
- **THEN** Clilint records the malformed heading as a deterministic failure and does not request its section

### Requirement: Help check limits
Clilint SHALL enforce configurable limits on discovered command count, command
depth, captured document bytes, search results, total help commands, and time
per command.

#### Scenario: Tested CLI tool advertises excessive commands
- **WHEN** recursive child-command discovery exceeds the configured command limit
- **THEN** Clilint stops following child commands and reports which bound was exceeded

### Requirement: Non-interactive offline checking
The hierarchical help checker SHALL close standard input, capture output from the
tested CLI tool without a terminal, and complete without network access, a
pager, a local viewer, or a browser.

#### Scenario: Check viewer behavior safely
- **WHEN** Clilint evaluates local and web viewer checks
- **THEN** it captures their non-interactive stdout behavior without opening another program

### Requirement: Hierarchical help evidence
Each hierarchical-help finding SHALL include the relevant command paths,
commands run, captured output, requested sections, and validation failures in
the JSON report. Clilint SHALL report a separate result for every tested
`codekiln-help` check.

#### Scenario: Inspect a section mismatch
- **WHEN** a retrieved section does not match the section advertised by an outline
- **THEN** the report evidence identifies the command path, requested section, outline observation, and section observation

### Requirement: Web check rating
The `codekiln-help` check bundle SHALL classify successful versioned web-page
resolution as required for `Good` and `Excellent` ratings.

#### Scenario: Web check fails at a high rating
- **WHEN** a tested CLI tool satisfies the lower-rated help checks but cannot resolve its versioned web page
- **THEN** the web check result prevents `Good` and `Excellent` `codekiln-help` ratings
