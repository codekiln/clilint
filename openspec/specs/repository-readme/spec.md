## Purpose

Define the information, order, examples, and focused links required for Clilint's root README to serve a first-time visitor.

## Requirements

### Requirement: Immediate project purpose and value
The root README SHALL explain in its opening what Clilint checks, who it is for, why those checks matter, and what makes the project's approach useful before introducing package formats, assessment methods, implementation language, or report internals.

#### Scenario: First-time visitor reads the opening
- **WHEN** a visitor reads the title and opening section
- **THEN** the visitor can identify the practical problem Clilint solves, whether it applies to their project, and why they would use Clilint for it

### Requirement: Project status and trust information
The root README SHALL state the project's current maturity, release availability, supported platforms, where to get help, who maintains it, and its license using text or links that a visitor can verify.

#### Scenario: Visitor evaluates the project
- **WHEN** a visitor looks for evidence that the project is active and usable
- **THEN** the README provides current release, continuous-integration, maintenance, and license information without requiring the visitor to infer status from implementation details

### Requirement: Five-minute installation and first use
The root README SHALL provide one short, copyable installation example and a first check that lets a user on an applicable supported platform reach and understand the main result within five minutes.

#### Scenario: New user follows the quick start
- **WHEN** a user starts with the README on a supported platform and follows the quick-start instructions
- **THEN** the user reaches an explained Clilint result within five minutes without reading platform-specific instructions that do not apply to that path

### Requirement: Concise README installation guidance
The root README SHALL show one supported installation example, preferably using mise, and SHALL link directly to `docs/installation.md` for complete installation options instead of reproducing platform-specific instructions.

#### Scenario: Visitor scans the installation section
- **WHEN** a visitor reaches the README installation section
- **THEN** the visitor sees one copyable example and a clear link to complete instructions without first reading steps for every platform

### Requirement: Installation guide covers every supported download
The repository SHALL provide `docs/installation.md` with instructions for each operating system and processor type supported by the current GitHub Release. For each one, the guide SHALL identify the file to download and explain how to verify the download, extract Clilint, and make the `clilint` command available on PATH.

#### Scenario: Reader installs Clilint without mise
- **WHEN** a reader opens `docs/installation.md` and finds their operating system and processor type
- **THEN** the reader can identify the correct download and install Clilint by following only the instructions for their system

### Requirement: README shows how to check a command
The root README SHALL show a copyable `clilint check <command>` example that works after installation without a repository checkout. It SHALL show enough of the expected output to explain what Clilint found and tell the reader to replace the example command with the name or path of a command they want to check.

#### Scenario: Reader runs Clilint for the first time
- **WHEN** a reader copies the example after installing Clilint
- **THEN** Clilint checks a command available on PATH and the README explains the result and how to check a different command

### Requirement: Reader-centered information order
The root README SHALL apply progressive disclosure by presenting the project's purpose, why a reader would choose Clilint, status, installation, and first use before detailed package, assessment, development, or historical information, while providing descriptive links to the deferred details.

#### Scenario: Visitor scans the README
- **WHEN** a visitor scans the headings from top to bottom
- **THEN** the headings follow the decisions needed to evaluate and try the project before linking to advanced or contributor topics

### Requirement: Focused supporting documentation
The repository SHALL preserve installation, package authoring, AI-assessment, and contributor guidance in focused documents and SHALL link to them from short, descriptive README sections.

#### Scenario: User needs advanced guidance
- **WHEN** a user needs another installation method, wants to extend checks, attach an AI assessment, or contribute to the repository
- **THEN** the README directs the user to a document focused on that task

### Requirement: Communication style conformance
The root README SHALL conform to the repository communication standard and SHALL not include agent instructions that belong in RuleSync source.

#### Scenario: README is ready for review
- **WHEN** the README rewrite is complete
- **THEN** `assess-readme-style` reports no unresolved problem

### Requirement: README purpose conformance
The root README SHALL satisfy the first-time visitor criteria defined by the repository README purpose assessment.

#### Scenario: README is ready for review
- **WHEN** the README rewrite is complete
- **THEN** `assess-readme-purpose` reports no unresolved problem

### Requirement: Verifiable README content
README commands, links, platform claims, and descriptions of Clilint behavior SHALL be checked against the implemented CLI, the current release configuration, and the repository files they describe.

#### Scenario: Documentation change is reviewed
- **WHEN** the rewritten README is ready for review
- **THEN** its executable examples have been run where the development platform permits, its links have been checked, and its factual claims match the repository state
