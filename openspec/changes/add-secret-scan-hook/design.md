## Context

The dotfiles repository manages secretlint, its rules, the private identifier source, and a global `secrets:scan` mise file task. Its documented per-repository opt-in is a name-free lefthook command plus a mise-managed lefthook installation. Clilint already uses mise but does not have a Git hook configuration.

## Goals / Non-Goals

**Goals:**

- Scan each staged clilint file with the existing global scanner before commit.
- Keep the hook installation and lefthook version under mise.
- Keep private identifiers and scanner configuration out of this public repository.

**Non-Goals:**

- Copying the global scan task or secretlint configuration into clilint.
- Defining a second identifier list.
- Adding a different secret scanner for contributors who do not use the user's dotfiles.

## Decisions

### Use the dotfiles opt-in command unchanged

`lefthook.yml` will define `identity-guard` as `mise run secrets:scan {staged_files}`. Lefthook supplies only staged paths, and mise resolves the global task installed by the dotfiles.

Alternative considered: copy the scan task and secretlint setup into clilint. That would create a second implementation and risk placing private configuration in a public repository.

### Let mise manage and install lefthook

The root `mise.toml` will add `lefthook` and an enter hook that runs `lefthook install`. This follows the dotfiles setup and makes hook installation automatic after the checkout is trusted.

Alternative considered: require each checkout to run `lefthook install` manually. That makes it easy to believe the guard is active when it is not.

## Risks / Trade-offs

- [The global `secrets:scan` task is unavailable] → The hook fails instead of allowing an unscanned commit; setup requires applying the user's dotfiles and running the global mise installation.
- [A staged path contains spaces] → Use lefthook's `{staged_files}` expansion exactly as exercised by the dotfiles repository.
- [The hook configuration accidentally contains an identifier] → Keep the repository wiring name-free and exercise it against the global scanner before completion.

## Migration Plan

1. Add lefthook and its enter installation hook to `mise.toml`.
2. Add the name-free pre-commit command to `lefthook.yml`.
3. Install the hook in the current checkout.
4. Verify a clean staged file passes and a temporary protected value is blocked.

Rollback removes `lefthook.yml`, the mise lefthook declaration and enter hook, then reinstalls or removes the local Git hook as appropriate.

## Open Questions

None.

## Citations

- [My/Principle/Make the Right Thing Easy and the Wrong Thing Hard](logseq://graph/logseq-encode-garden?page=My/Principle/Make%20the%20Right%20Thing%20Easy%20and%20the%20Wrong%20Thing%20Hard)
- [My/Principle/Simplify/Fewer and Deeper](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Fewer%20and%20Deeper)
- [My/Pref/Dev/mise](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise)
