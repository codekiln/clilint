## 1. Define pull request release input

- [x] 1.1 Add and test a validator for the agreed Conventional Emoji Commit title format and emoji/type pairings.
- [x] 1.2 Add pull request CI for title validation, formatting, Clippy with warnings denied, tests, and a release build.
- [x] 1.3 Configure and verify squash merging, required checks, and auto-merge repository settings.

## 2. Configure version preparation

- [x] 2.1 Add release-plz and dist to the project mise configuration, update `mise.lock`, and add described file tasks for release checks, preparation, and dist regeneration.
- [x] 2.2 Configure Git-only release-plz behavior, emoji-aware version increments, non-release commit filters, changelog updates, tags, and disabled GitHub Release creation.
- [x] 2.3 Add tests or dry checks showing feature, fix, breaking, and non-release commits produce the expected release impact before and after 1.0.0.
- [x] 2.4 Add the primary-branch workflow that creates or updates one release pull request and enables squash auto-merge from the release-plz output.

## 3. Configure workflow credentials

- [x] 3.1 Create a repository-scoped GitHub App with only the contents and pull-request permissions required by the workflows.
- [x] 3.2 Store the App ID and private key as repository secrets and generate short-lived tokens in the release workflow.
- [x] 3.3 Verify a bot-created release pull request triggers required CI and a bot-created test tag triggers its workflow.

## 4. Configure binary distribution

- [x] 4.1 Choose the required 0.0.2 release targets and document the target decision in the design.
- [x] 4.2 Generate and commit the dist release workflow without hand-editing its build matrix.
- [x] 4.3 Run dist's supported pull request or dry-run checks and verify archives and SHA-256 checksums for every target.
- [x] 4.4 Verify release-plz owns versioning and tags while dist alone creates and completes the GitHub Release.

## 5. Exercise the complete release path

- [x] 5.1 Test that a failed required check leaves the release pull request open and publishes no tag or completed release.
- [x] 5.2 Test that a successful releasable pull request leads to an automatically merged release pull request, matching tag, and checksummed GitHub Release without manual dispatch.
- [x] 5.3 Run the aggregate mise CI task, strict OpenSpec validation, and record the one-time repository setup needed to keep later releases automatic.
