## ADDED Requirements

### Requirement: Maintained release pull request
After a releasable change reaches the primary branch, automation SHALL create or update one release pull request containing the calculated version, lockfile, and changelog changes.

#### Scenario: Feature reaches primary branch
- **WHEN** a releasable feature commit reaches the primary branch
- **THEN** the release pull request shows the calculated next version and its changelog entry

### Requirement: Automatic release merge
Automation SHALL enable squash auto-merge on the release pull request and SHALL allow it to merge only after required checks pass.

#### Scenario: Release checks pass
- **WHEN** all required checks on the release pull request pass
- **THEN** GitHub merges the release pull request without maintainer action

#### Scenario: Release check fails
- **WHEN** a required check on the release pull request fails
- **THEN** the pull request remains open and no tag is created

### Requirement: Workflow-triggering credentials
Release automation SHALL use credentials limited to the repository permissions needed to create and merge release pull requests, push release tags, and trigger required workflows.

#### Scenario: Release tag is pushed
- **WHEN** the release pull request merges successfully
- **THEN** the created tag triggers the distribution workflow

### Requirement: mise-managed release tools
The repository SHALL manage release-plz and dist through mise, and release CI SHALL install the project tool configuration before invoking those tools.

#### Scenario: Reproduce release preparation locally
- **WHEN** a release preparation check fails in CI
- **THEN** a maintainer can run the named mise task locally with the same managed release tool version
