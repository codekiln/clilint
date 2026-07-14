## Why

The project should be easy to release without asking a maintainer to calculate a version, dispatch a workflow, assemble binaries, or write release notes. Release behavior should follow the meaning already expressed by Conventional Emoji Commit messages and run after ordinary pull requests merge.

## What Changes

- Require squash-merge pull request titles to follow the Conventional Emoji Commit format.
- Validate the relationship between the emoji and conventional commit type in pull request CI.
- Use commit meaning to calculate Semantic Versioning changes without labels or manual version choices.
- Use release-plz to maintain the version, lockfile, changelog, release pull request, and release tag.
- Automatically enable merge for the release pull request after required checks pass.
- Use a narrowly scoped GitHub App token so release pull requests and tags can trigger their required workflows.
- Use `dist` to generate the cross-platform binary build and GitHub Release workflow from a release tag.
- Manage release-plz and dist with mise and invoke release checks through the same mise tasks used locally.
- Do not release documentation, test-only, CI-only, or housekeeping changes unless they contain a breaking-change declaration.
- Treat the planned 0.0.1 to 0.0.2 rebuild as the initial-development starting point for subsequent automation.

## Capabilities

### New Capabilities

- `release-impact`: Validates Conventional Emoji Commit titles and derives the next Semantic Versioning change.
- `release-preparation`: Maintains a release pull request and merges it automatically after required checks pass.
- `release-distribution`: Creates a tag-driven GitHub Release with tested binaries and checksums.

### Modified Capabilities

None. The repository does not currently have an automated project release capability.

## Impact

- Adds GitHub Actions workflows, release-plz configuration, and dist configuration generated for this Rust binary.
- Requires one-time repository settings and narrowly scoped GitHub App credentials.
- Establishes pull request title and squash-merge conventions as part of the release interface.
- Replaces the current integration-only GitHub Action material with a project release pipeline where appropriate.

## Citations

- [My/Principle/Make the Right Thing Easy and the Wrong Thing Hard](logseq://graph/logseq-encode-garden?page=My/Principle/Make%20the%20Right%20Thing%20Easy%20and%20the%20Wrong%20Thing%20Hard)
- [My/Principle/Simplify/Fewer and Deeper](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Fewer%20and%20Deeper)
- [My/Principle/Simplify/Prefer Standards and Defaults](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Prefer%20Standards%20and%20Defaults)
- [My/Pref/Dev/mise](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise)
