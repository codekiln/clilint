## ADDED Requirements

### Requirement: One complete codekiln-help check
The `codekiln-help` check bundle SHALL define one hierarchical-help check whose
Checker CLI gathers the command hierarchy and help documents, verifies every
help behavior in this specification, and returns one Check Result with one
Score and zero or more Check Messages.

#### Scenario: Check a CLI tool that follows the standard
- **WHEN** a user checks a nested CLI tool that follows the standard with the `codekiln-help` check bundle
- **THEN** the hierarchical-help Check Result has a Score of `4.0` and no Warning or Error Check Message

#### Scenario: Several help behaviors fail
- **WHEN** one tested CLI tool has invalid command discovery and incomplete programmatic guidance
- **THEN** one hierarchical-help Check Result contains focused Check Messages for both unmet expectations

### Requirement: Hierarchical-help Checker CLI
The `codekiln-help` bundle SHALL supply its hierarchical-help checker through
the same Checker CLI protocol available to another installed check
bundle. Clilint SHALL NOT contain a hierarchical-help checker type or
check-specific command traversal.

#### Scenario: Bundle author implements an equivalent check
- **WHEN** a bundle author installs another local bundle with an equivalent Checker CLI
- **THEN** Clilint can run its check without adding check-specific code to Clilint

### Requirement: Reused help evidence
The hierarchical-help checker SHALL gather each needed help response once
during its check and reuse the response while verifying related expectations.

#### Scenario: Several expectations use one command hierarchy
- **WHEN** command discovery, outlines, and section retrieval need the same command hierarchy
- **THEN** the checker reuses the recorded hierarchy within one check run

### Requirement: Recursive command-path checking
The hierarchical-help checker SHALL verify applicable help behavior at every
valid command path returned by recursively following immediate child commands
in JSON help.

#### Scenario: Failure at a deeply nested command
- **WHEN** `tool repo clone` is advertised but lacks its required help outline
- **THEN** a Check Message identifies the `repo clone` command path and the failed invocation

### Requirement: JSON help validation
The hierarchical-help checker SHALL validate JSON format versions, command
paths, heading levels, `section` values, search results, and relationships
between responses before using returned values in later commands.

#### Scenario: Follow a discovered section
- **WHEN** a JSON outline returns a valid `section` value
- **THEN** the checker passes that exact value to a later section invocation

#### Scenario: Outline contains an invalid heading level
- **WHEN** a heading record contains a level outside the Markdown heading range
- **THEN** a Check Message identifies the malformed heading and the checker does not request its section

### Requirement: Hierarchical-help limits
The hierarchical-help checker SHALL bound command count, command depth,
captured document bytes, search results, total help commands, and time per
command. Its configured limits SHALL permit every bound to be reached and
reported independently.

#### Scenario: Tested CLI tool advertises excessive commands
- **WHEN** recursive child-command discovery exceeds the command-count limit
- **THEN** the checker stops following child commands and returns a Check Message that identifies that limit

#### Scenario: Total command budget is exhausted
- **WHEN** the checker reaches the total help-command limit
- **THEN** a Check Message identifies the exhausted budget instead of assigning the failure to whichever help behavior happened to run last

### Requirement: Shared and programmatic help checking
The hierarchical-help checker SHALL verify whether default help contains the
guidance required for every caller and whether `--programmatic` retains that
help while adding instructions for piping, section retrieval,
machine-readable output, and avoiding interactive output.

#### Scenario: Programmatic guidance omits an instruction
- **WHEN** programmatic help mentions JSON and sections but does not explain piping or avoiding interactive output
- **THEN** the Check Result contains a Check Message for the omitted required guidance

### Requirement: Search checking
The hierarchical-help checker SHALL verify that a search for a term present in
the local help returns at least one valid command path and `section` value that
can be used for section retrieval.

#### Scenario: Search returns no results
- **WHEN** the checker searches for a term present in the fixture help and receives an empty result
- **THEN** the Check Result contains an Error-level Check Message for search

### Requirement: Non-interactive offline checking
The hierarchical-help checker SHALL close standard input for tested CLI tool
invocations, capture output without a terminal, omit network access, and
complete without starting a pager or another interactive program.

#### Scenario: Check captured help safely
- **WHEN** Clilint runs the hierarchical-help check with closed input and captured output
- **THEN** the checker completes without waiting for input, opening another program, or using the network

### Requirement: Focused hierarchical-help evidence
Each hierarchical-help Check Message SHALL identify the relevant command path,
invocation, observed output or validation failure, and the evidence supporting
the message. The Check Result SHALL bound repeated observations instead of
copying all gathered evidence into every Check Message.

#### Scenario: Inspect a section mismatch
- **WHEN** a retrieved section does not match the section advertised by an outline
- **THEN** the Check Message identifies the command path, requested section, outline observation, and section observation
