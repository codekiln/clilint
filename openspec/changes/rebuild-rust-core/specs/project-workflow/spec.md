## ADDED Requirements

### Requirement: mise-managed project environment
The repository SHALL use a root `mise.toml` to manage the Rust toolchain and project development tools, and SHALL commit a `mise.lock` file for reproducible tool resolution.

#### Scenario: Set up a fresh checkout
- **WHEN** a contributor runs `mise install` in a fresh checkout
- **THEN** mise installs the Rust toolchain and project development tools at the versions resolved by the committed configuration and lockfile

### Requirement: Executable mise file tasks
Project task automation SHALL use executable files under `.mise/tasks/` instead of inline TOML task bodies, except for a task that is genuinely one line. Each file task SHALL have a `#MISE description`, and each task with arguments or flags SHALL define them with `#USAGE` metadata.

#### Scenario: List project tasks
- **WHEN** a contributor runs `mise tasks`
- **THEN** each project file task appears with a useful description and any accepted arguments are available to mise completion

### Requirement: Shared local and CI commands
Continuous integration SHALL install project tools through mise and SHALL invoke the same mise tasks used for local formatting, linting, testing, OpenSpec validation, and RuleSync checks.

#### Scenario: Reproduce a CI check locally
- **WHEN** a CI check fails for a mise task
- **THEN** a contributor can run the named mise task locally without translating the workflow into a different command sequence

### Requirement: OpenSpec skills are loaded before artifact work
The repository SHALL store OpenSpec skills under `.rulesync/skills/`, generate supported agent copies with RuleSync, and instruct an agent to load the matching OpenSpec skill and follow the current CLI artifact instructions before creating or editing an OpenSpec artifact.

#### Scenario: Agent edits a proposal
- **WHEN** an agent is asked to create or revise an OpenSpec proposal
- **THEN** the agent loads the project `openspec-propose` skill and obtains the proposal instructions from the OpenSpec CLI before writing the proposal

### Requirement: OpenSpec skill version matches the managed CLI
Each OpenSpec skill in `.rulesync/skills/` SHALL identify the same `generatedBy` version as the OpenSpec CLI resolved by the project mise configuration, and CI SHALL reject a version mismatch.

#### Scenario: Managed OpenSpec version changes
- **WHEN** the OpenSpec version resolved by mise changes without refreshing the RuleSync skill source
- **THEN** the project workflow check fails and reports the mismatched CLI and skill versions
