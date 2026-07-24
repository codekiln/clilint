## ADDED Requirements

### Requirement: Help at every command path
A tested CLI tool that follows this standard SHALL reserve `help` beneath the
root command and every advertised command path. Invoking
`<command path> help` SHALL describe the same command, usage, options, and
immediate child commands as `<command path> --help`.

#### Scenario: Nested command help
- **WHEN** a user invokes `tool repo clone help`
- **THEN** the command writes the overview for `tool repo clone` to stdout and exits successfully

#### Scenario: Help option equivalence
- **WHEN** a user invokes `tool repo clone help` and `tool repo clone --help`
- **THEN** both invocations describe the same command, usage, options, and immediate child commands

### Requirement: Offline child-command discovery
`<command path> help --format json` SHALL identify the current command path and
its immediate child commands from locally available information. A caller SHALL
be able to discover the complete command hierarchy by repeating the operation
for each returned child command.

#### Scenario: Discover a nested hierarchy
- **WHEN** an agent invokes `tool help --format json` and follows each returned child command
- **THEN** the responses identify every advertised command path beneath the root

#### Scenario: Discover one subtree
- **WHEN** an agent invokes `tool repo help --format json`
- **THEN** the result identifies `repo` and its immediate child commands without unrelated command paths

### Requirement: Markdown document outline
`<command path> help outline` SHALL describe the Markdown heading hierarchy of the
Markdown document for that command. Each heading record SHALL include its level,
title, and `section` value.

#### Scenario: List all headings
- **WHEN** an agent invokes `tool repo clone help outline --format json`
- **THEN** the result contains an ordered record for every heading in the command document

#### Scenario: Select one heading level
- **WHEN** an agent invokes `tool repo clone help outline --level 3 --format json`
- **THEN** every returned heading has Markdown level three

#### Scenario: Select headings through a level
- **WHEN** an agent invokes `tool repo clone help outline --max-level 3 --format json`
- **THEN** every returned heading has a Markdown level from one through three

### Requirement: Section retrieval
`<command path> help section <section>` SHALL accept a `section` value returned
by the current document's outline or search result. The default response SHALL
contain every matching heading and each heading's direct body in document
order. `--recursive` SHALL also contain each matching heading's descendant
sections.

#### Scenario: Retrieve a direct section body
- **WHEN** an agent retrieves an H3 section that contains an H4 child
- **THEN** the response contains the H3 heading and direct body without the H4 child

#### Scenario: Retrieve a section subtree
- **WHEN** an agent repeats the section retrieval with `--recursive`
- **THEN** the response also contains the H4 child and its descendants

#### Scenario: Retrieve repeated headings
- **WHEN** two headings share the same `section` value in an outline
- **THEN** section retrieval returns both matching headings in document order

#### Scenario: Reject an unknown section
- **WHEN** a caller supplies a section absent from the current document
- **THEN** the command exits non-zero and identifies the invalid section argument

### Requirement: JSON help responses
JSON help overviews, outlines, searches, and section responses SHALL identify
their format version, command path, and whether they include programmatic
guidance. JSON results that refer to sections SHALL include a `section` value
accepted by the section operation.

#### Scenario: Follow a JSON outline
- **WHEN** an agent copies the `section` value from a JSON outline into a section invocation for the same command document
- **THEN** the tested CLI tool returns the advertised section

### Requirement: Shared help with optional programmatic guidance
The default help for a command SHALL describe its purpose, behavior, usage,
and any applicable side effects and required permissions in a form usable by
people and agents.
Every help operation SHALL accept `--programmatic`. The `--programmatic`
document SHALL include the default help and SHALL add relevant instructions for
programmatic use, including piping, section retrieval, machine-readable output,
and avoiding interactive output.

#### Scenario: Agent uses the default help
- **WHEN** an agent invokes `tool repo clone help` without `--programmatic`
- **THEN** the response contains the command guidance needed by any caller

#### Scenario: Any caller requests programmatic guidance
- **WHEN** a caller invokes `tool repo clone help --programmatic`
- **THEN** the response includes the default help and instructions for retrieving only the needed output without a pager, full-screen interface, animation, or extra notification

#### Scenario: Navigate programmatic sections
- **WHEN** a caller uses `--programmatic` with outline and section operations
- **THEN** the returned `section` values can retrieve sections from the default help and its added programmatic guidance

### Requirement: Offline documentation search
`<command path> help search <query>` SHALL search locally available help
documents for the current command and the commands below it. The result SHALL
contain matching command paths and `section` values. `--programmatic` SHALL
include the added programmatic guidance.

#### Scenario: Search all local documentation
- **WHEN** an agent invokes `tool help search permissions --programmatic --format json` without network access
- **THEN** the result identifies locally available matching sections that can be passed to the section operation

### Requirement: Non-interactive help retrieval
Help overview, outline, section, and search operations SHALL write to
stdout without starting a pager. When stdout is not a terminal, they SHALL omit
terminal control sequences.

#### Scenario: Agent retrieves help through a pipe
- **WHEN** an agent captures `tool repo clone help section <section>`
- **THEN** the invocation completes without input, a pager, or terminal control sequences

### Requirement: Local help viewer
`<command path> help view` SHALL present the complete local document. When
attached to an interactive terminal it SHALL use a human-readable terminal
presentation. When non-interactive it SHALL write the document to stdout
without starting a pager.

#### Scenario: Non-interactive local view
- **WHEN** stdout for `tool repo clone help view` is not a terminal
- **THEN** the command writes the local document to stdout and exits without waiting for input

### Requirement: Web help viewer
A tested CLI tool claiming a `Good` or `Excellent` `codekiln-help` rating
SHALL support `<command path> help view --web`. The command SHALL resolve the
web page corresponding to the tested CLI tool's current version and command
path. In an interactive terminal it SHALL open that page. When non-interactive
it SHALL write the resolved URL to stdout without opening a browser.

#### Scenario: Agent resolves a web page
- **WHEN** an agent captures `tool repo clone help view --web`
- **THEN** the command writes the corresponding URL without launching a browser

### Requirement: Offline help behavior
Child-command discovery, default and `--programmatic` overviews, outlines,
section retrieval, search, and local viewing SHALL complete without network
access.

#### Scenario: Navigate without network access
- **WHEN** network access is unavailable
- **THEN** a caller can discover a command path and retrieve any section in its human or agent document
