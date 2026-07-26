## ADDED Requirements

### Requirement: Installable check bundles
Clilint SHALL include the core `clilint` check bundle and SHALL allow a user to
install the named `codekiln-help` check bundle from a local path without
installing other named check bundles. Installed check bundles SHALL participate
in later Clilint runs without a separate activation step.

#### Scenario: Install the codekiln help check bundle
- **WHEN** a user installs the named `codekiln-help` check bundle
- **THEN** later Clilint runs include the core and help checks

#### Scenario: Run with only the core bundle
- **WHEN** a user runs Clilint without installing another check bundle
- **THEN** Clilint uses only the built-in `clilint` check bundle

### Requirement: Named check-bundle inheritance
The `codekiln-help` check bundle SHALL extend `clilint`. Clilint SHALL preserve
the order and checks of the core bundle, an installed named bundle, and any
local bundle that extends them.

#### Scenario: Resolve codekiln help inheritance
- **WHEN** Clilint resolves the named `codekiln-help` check bundle
- **THEN** the report identifies `clilint` followed by `codekiln-help` and contains checks from both bundles

### Requirement: Custom check-bundle example
The repository SHALL implement `codekiln-help` with the same bundle format and
Checker CLI protocol available to other bundle authors. The repository SHALL
provide a guide that follows the check from its definition to its checker,
Check Result, Check Messages, test fixtures, and report output.

#### Scenario: Bundle author studies codekiln-help
- **WHEN** a check-bundle author reads the `codekiln-help` guide
- **THEN** the guide points to the bundle file, explains its identity, inheritance, Check, and Checker CLI, and links each example to relevant tests

#### Scenario: Use the same check-bundle validation
- **WHEN** Clilint loads the installed `codekiln-help` check bundle
- **THEN** the bundle passes the same format and check validation applied to a bundle supplied by a user

### Requirement: Project-local bundle declarations
Clilint SHALL record installed check bundles in `.clilint/config.toml` in the
current project directory. A local source path SHALL be stored relative to that
directory and SHALL remain relative when it contains parent-directory
components.

#### Scenario: Install a sibling bundle
- **WHEN** a user runs `clilint bundle install ../codekiln-help`
- **THEN** `.clilint/config.toml` records a relative path that resolves to the sibling bundle from the project directory

#### Scenario: Declared bundle cannot load
- **WHEN** a project declares a local check bundle that Clilint cannot load or validate
- **THEN** Clilint stops before running any check and identifies the declared bundle

## MODIFIED Requirements

### Requirement: Bundled global standard
Clilint SHALL include the core `clilint` check bundle and SHALL use it when the
user has not installed another check bundle.

#### Scenario: Check without an installed bundle
- **WHEN** a user checks a tested CLI tool without installing another check bundle
- **THEN** Clilint evaluates the tested CLI tool against the built-in core checks

### Requirement: Local extension packages
Clilint SHALL install a user-authored check bundle from a local path and SHALL
evaluate its checks in later runs together with every check bundle it extends.

#### Scenario: Check with an extension of the core bundle
- **WHEN** an installed local check bundle extends `clilint`
- **THEN** the report contains Check Outcomes for the core checks and the local extension checks

#### Scenario: Check with an extension of the codekiln help bundle
- **WHEN** an installed local check bundle extends `codekiln-help`
- **THEN** the report contains Check Outcomes for `clilint`, `codekiln-help`, and the local extension in that order

### Requirement: Additive conformance
An extension check bundle MUST NOT remove, replace, or weaken a check inherited
from another check bundle.

#### Scenario: Check bundle attempts to weaken an inherited check
- **WHEN** an extension excludes an inherited check or lowers its required result
- **THEN** Clilint rejects the check bundle and identifies the conflicting check

### Requirement: Package validation
Clilint SHALL reject check-bundle data containing an invalid built-in checker,
an invalid Checker CLI, a duplicate check identifier, or an unsupported format
version.

#### Scenario: Invalid Checker CLI
- **WHEN** a check bundle declares an invalid Checker CLI
- **THEN** Clilint exits nonzero and identifies the invalid Checker CLI

### Requirement: Offline checking
Clilint SHALL load local check bundles and run built-in checks without network
access. A Checker CLI is responsible for any network access its Check requires.

#### Scenario: Run an offline local checker
- **WHEN** the selected local check bundles are available and their Checker CLIs do not request online evidence
- **THEN** Clilint loads and runs them without requiring network access

## RENAMED Requirements

- FROM: `Bundled global standard`
- TO: `Built-in core check bundle`
- FROM: `Local extension packages`
- TO: `Local extension check bundles`
- FROM: `Package validation`
- TO: `Check-bundle validation`
