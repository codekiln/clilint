# Extension packages

Clilint's built-in package checks behavior that applies to many command-line programs. A local package can add rules for a project or team. Local packages extend the built-in package; they cannot remove a built-in rule or reduce its severity.

Create a TOML file such as `team.toml`:

```toml
format_version = 1
extends = "clilint"

[package]
name = "team"
version = "1.0.0"

[[rules]]
id = "team/help/team-option"
title = "Help describes the team option"
severity = "warn"
evaluation_method = "deterministic"

[rules.check]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["--team"] }]
```

Run Clilint with the package:

```sh
clilint check my-cli --package ./team.toml
```

Each rule identifier belongs to its package. An extension can add rules or make an inherited rule more severe. Clilint rejects packages that try to exclude, replace, or weaken inherited rules before it runs the target command.

The built-in package at [`packages/clilint/clilint.toml`](../packages/clilint/clilint.toml) shows the supported invocation checks and assertions. Clilint reports an error if a local package does not match the expected format.
