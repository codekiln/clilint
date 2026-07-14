## ADDED Requirements

### Requirement: Tag-driven distribution
A release tag SHALL start one CI-managed distribution run for the version recorded in the Rust package manifest.

#### Scenario: Matching release tag
- **WHEN** automation pushes a release tag whose version matches the Rust package version
- **THEN** the distribution workflow plans builds for every configured release target

### Requirement: Tested release binaries
The distribution workflow SHALL run the required Rust checks before publishing release artifacts and SHALL stop publication when a required check fails.

#### Scenario: Required test fails
- **WHEN** a required Rust check fails for a release commit
- **THEN** the workflow does not publish a completed GitHub Release

### Requirement: Release artifacts
Each successful distribution SHALL create a GitHub Release containing versioned binary archives and SHA-256 checksums for every configured release target.

#### Scenario: All configured builds succeed
- **WHEN** every configured target builds and packages successfully
- **THEN** the GitHub Release contains each archive and its checksum

### Requirement: Generated distribution workflow
The repository SHALL use the workflow generated and maintained by `dist` for binary distribution instead of maintaining a custom build matrix by hand.

#### Scenario: Distribution configuration changes
- **WHEN** a maintainer changes supported release targets
- **THEN** regenerating the dist workflow updates the build plan without editing each build job manually
