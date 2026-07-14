## Purpose

Define clilint conformance packages, package extension rules, validation, and offline operation.

## Requirements

### Requirement: Bundled global standard
The clilint binary SHALL include a global conformance package and SHALL use it when the user does not select another package.

#### Scenario: Check without a package option
- **WHEN** a user checks a target without selecting a package
- **THEN** clilint evaluates the target against the bundled global package

### Requirement: Local extension packages
Clilint SHALL load a user-authored package from a local path and SHALL evaluate its rules together with the global rules it extends.

#### Scenario: Check with an extension package
- **WHEN** a user checks a target with a valid local extension package
- **THEN** the report contains findings for the global rules and the extension rules

### Requirement: Additive conformance
An extension package MUST NOT remove, replace, or weaken a rule inherited from the global package.

#### Scenario: Package attempts to weaken a global rule
- **WHEN** an extension package excludes a global rule or lowers its required result
- **THEN** clilint rejects the package and identifies the conflicting rule

### Requirement: Package validation
Clilint SHALL reject package data containing an unknown check type, a duplicate rule identifier, an invalid skill reference, or an unsupported package format version.

#### Scenario: Unknown deterministic check
- **WHEN** a package declares a deterministic check type that this clilint version does not support
- **THEN** clilint exits non-zero and identifies the unsupported check type

### Requirement: Offline checking
Clilint SHALL NOT require network access to check a target after the selected packages and skills are available locally.

#### Scenario: Check without network access
- **WHEN** a user checks a target with locally available package data while network access is unavailable
- **THEN** deterministic checking and evidence collection still complete
