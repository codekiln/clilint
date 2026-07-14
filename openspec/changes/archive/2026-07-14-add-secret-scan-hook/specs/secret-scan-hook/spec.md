## ADDED Requirements

### Requirement: Staged files are scanned before commit
The repository SHALL define a lefthook pre-commit command named `identity-guard` that runs `mise run secrets:scan {staged_files}` and SHALL block the commit when the scan exits non-zero.

#### Scenario: Staged file contains protected material
- **WHEN** a staged file contains material rejected by the global secret scanner
- **THEN** the pre-commit hook exits non-zero and the commit is not created

#### Scenario: Staged files pass the scan
- **WHEN** every staged file passes the global secret scanner
- **THEN** the identity guard exits zero and does not prevent the commit

### Requirement: lefthook is managed and installed by mise
The project mise configuration SHALL manage lefthook and SHALL install the repository hooks when mise enters the trusted project.

#### Scenario: Enter a trusted checkout
- **WHEN** a user with mise enters the trusted clilint checkout
- **THEN** lefthook installs the repository's configured Git hooks without a separate installation command

### Requirement: Private scan data stays outside the repository
The repository SHALL NOT contain the user's private identifier list, secretlint rules, or secretlint configuration; the hook SHALL call the global `secrets:scan` task supplied by the user's dotfiles.

#### Scenario: Inspect the repository hook configuration
- **WHEN** `lefthook.yml` and `mise.toml` are inspected
- **THEN** they contain only the name-free task invocation and tool wiring, with no protected identifier values
