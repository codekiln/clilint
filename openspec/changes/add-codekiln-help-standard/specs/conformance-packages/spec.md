## ADDED Requirements

### Requirement: Installable check bundles
Clilint SHALL include the core `clilint` check bundle and SHALL allow a user to
install the named `codekiln-help` check bundle without installing other named
check bundles. Installed check bundles SHALL participate in later Clilint runs
without a separate activation step.

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
checkers available to other bundle authors. The repository SHALL provide a
guide that follows checks from their definitions to their checkers, test
fixtures, and report results.

#### Scenario: Bundle author studies codekiln-help
- **WHEN** a check-bundle author reads the `codekiln-help` guide
- **THEN** the guide points to the bundle file, explains its identity, inheritance, checks, and checkers, and links each example to relevant tests

#### Scenario: Use the same check-bundle validation
- **WHEN** Clilint loads the installed `codekiln-help` check bundle
- **THEN** the bundle passes the same format and check validation applied to a bundle supplied by a user

## MODIFIED Requirements

### Requirement: Local extension check bundles
Clilint SHALL install a user-authored check bundle from a local path and SHALL
evaluate its checks in later runs together with every check bundle it extends.

#### Scenario: Check with an extension of the core bundle
- **WHEN** an installed local check bundle extends `clilint`
- **THEN** the report contains findings for the core checks and the local extension checks

#### Scenario: Check with an extension of the codekiln help bundle
- **WHEN** an installed local check bundle extends `codekiln-help`
- **THEN** the report contains findings for `clilint`, `codekiln-help`, and the local extension in that order
