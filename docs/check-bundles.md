# Check bundles

Clilint's built-in `clilint` check bundle covers behavior that applies to many command-line tools. A project can install more bundles from local paths or Git sources. An extension adds checks to its parent; it cannot remove or weaken inherited checks.

## Install bundles for a project

Run these commands from the directory containing the tested CLI tool:

```sh
clilint bundle install ./path/to/codekiln-help
clilint bundle list
clilint check ./my-cli
```

Installation writes `.clilint/config.toml` in the current directory. Clilint does not search for a Git root or parent configuration in this version. Git declarations use explicit fields:

```toml
[check_bundles.codekiln-help]
source = "git"
url = "https://github.com/example/check-bundles.git"
ref = "v1.0.0"
path = "check-bundles/codekiln-help"
```

A local declaration uses `source = "local"` and a `path`. An ordinary `clilint check` installs a missing declared Git bundle before checking. `--offline` prohibits network access. `clilint bundle lock` writes exact Git commits to `.clilint/lock.toml`; `--locked` requires that file to match the declarations.

```text
clilint bundle install [<source>]
clilint bundle lock [<name>]
clilint bundle list [--json]
clilint bundle update [<name>]
clilint bundle remove <name>
```

Installed Git contents use `CLILINT_DATA_DIR`, then `XDG_DATA_HOME/clilint`, then the platform XDG fallback. Disposable clones use `CLILINT_CACHE_DIR` or the corresponding XDG cache location.

## Author a bundle

Create a TOML file such as `team.toml`:

```toml
format_version = 1
extends = "clilint"

[check_bundle]
name = "team"
version = "1.0.0"

[[checks]]
id = "team/help/team-option"
title = "Help describes the team option"
severity = "warn"
evaluation_method = "deterministic"

[checks.checker]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["--team"] }]
```

Test the check bundle directly while authoring it:

```sh
clilint check my-cli --check-bundle ./team.toml
```

Each check identifier belongs to its bundle. An extension can add checks or make an inherited check more severe. Clilint rejects bundles that try to exclude, replace, or weaken inherited checks before it runs the tested CLI tool.

The built-in bundle at [`check-bundles/clilint/clilint.toml`](../check-bundles/clilint/clilint.toml) shows the supported invocation checkers and assertions. [`codekiln-help`](../check-bundles/codekiln-help/clilint.toml) uses the same format and demonstrates a checker that gathers shared evidence for several checks. Its [implementation guide](codekiln-help.md) follows one check from the bundle file through the checker and fixtures.
