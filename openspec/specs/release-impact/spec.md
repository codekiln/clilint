## Purpose

Define how pull request titles and squash-merged Conventional Emoji Commits determine automatic semantic release impact.

## Requirements

### Requirement: Conventional Emoji pull request titles
CI SHALL require each pull request title to use the Conventional Emoji Commit form `<emoji> <type>[optional scope]: <description>` and SHALL reject a mismatch between the emoji and type.

#### Scenario: Valid feature title
- **WHEN** a pull request is titled `✨ feat(package): add local extensions`
- **THEN** the title check passes and identifies feature release impact

#### Scenario: Mismatched emoji and type
- **WHEN** a pull request uses the feature emoji with the `fix` type
- **THEN** the title check fails with the accepted pairing

### Requirement: Automatic release impact
The release system SHALL derive release impact from the squash-merged Conventional Emoji Commit without requiring a maintainer to choose a version or label.

#### Scenario: Compatible fix merges
- **WHEN** a valid `🩹 fix` pull request is squash-merged
- **THEN** the next release uses the applicable patch increment

#### Scenario: Breaking change merges
- **WHEN** a merged commit contains the configured breaking-change marker
- **THEN** the next release uses the applicable breaking increment for the current development stage

### Requirement: Non-release changes
Documentation, test-only, CI-only, style, and housekeeping commits SHALL NOT create a release unless they contain a breaking-change marker.

#### Scenario: Documentation pull request merges
- **WHEN** a valid documentation-only pull request is squash-merged without a breaking-change marker
- **THEN** release automation does not prepare a new version for that commit
