## Why

Clilint is public and can receive pasted personal, employer, or credential material in source and planning files. The dotfiles repository already provides a global, non-interactive `secrets:scan` task, so clilint can use the same check before each commit without copying private identifiers into this repository.

## What Changes

- Add a lefthook pre-commit command that scans staged files through the global `mise run secrets:scan` task.
- Add lefthook to the project mise tools and install the repository hooks when mise enters the project.
- Keep private identifiers, secretlint configuration, and the scan task outside this public repository.

## Capabilities

### New Capabilities

- `secret-scan-hook`: Blocks commits when the existing global secret scanner finds protected material in a staged file.

### Modified Capabilities

None.

## Impact

- Adds `lefthook.yml` and extends `mise.toml`.
- Installs a local Git pre-commit hook for contributors who use the project mise configuration.
- Depends on the global `secrets:scan` task managed by the user's dotfiles.

## Citations

- [My/Principle/Make the Right Thing Easy and the Wrong Thing Hard](logseq://graph/logseq-encode-garden?page=My/Principle/Make%20the%20Right%20Thing%20Easy%20and%20the%20Wrong%20Thing%20Hard)
- [My/Principle/Simplify/Fewer and Deeper](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Fewer%20and%20Deeper)
- [My/Pref/Dev/mise](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise)
