# `codekiln-help`

`codekiln-help` is an optional check bundle for command-line documentation that people and programmatic callers can browse offline. It is also a worked example for authors making their own Clilint check bundles.

Install the bundle in the directory containing the tested CLI tool, then run Clilint normally:

```sh
clilint bundle install ./check-bundles/codekiln-help
clilint check ./my-cli
```

The [bundle file](../check-bundles/codekiln-help/clilint.toml) extends the built-in `clilint` bundle. Each `[[checks]]` entry names one behavior and uses the `hierarchical-help` checker. The checker implementation in [`src/help_checker.rs`](../src/help_checker.rs) gathers the command hierarchy and documents once, then gives each check its own finding and relevant evidence. The [passing fixture](../tests/fixtures/hierarchical-help-cli) and [integration tests](../tests/cli.rs) show the required behavior and failures at root, group, and nested command paths.

## Interface for tested CLI tools

Every advertised command path provides:

```text
<tool> [<command> ...] help
<tool> [<command> ...] help outline
<tool> [<command> ...] help section <section> [--recursive]
<tool> [<command> ...] help search <query>
<tool> [<command> ...] help view [--web]
```

`<command path> help` and `<command path> --help` describe the same command. `help`, `outline`, `section`, and `search` write to stdout without a pager. `help view` may use a terminal viewer only when attached to a terminal. When captured, local view writes the document and web view writes its URL.

The default help is the primary documentation for people and agents. It explains purpose, behavior, usage, side effects, and required permissions when they apply. `--programmatic` retains that help and adds instructions for piping, JSON output, section retrieval, and avoiding interactive displays. It is an optional route through one shared interface, not separate command documentation.

## JSON responses

Every JSON response contains:

```json
{
  "format_version": 1,
  "command_path": ["repo", "clone"],
  "programmatic": false
}
```

Ordinary help adds immediate children:

```json
{
  "child_commands": [{"name": "clone"}]
}
```

Callers discover the full command hierarchy by requesting JSON help for each returned child. `outline` adds ordered headings:

```json
{
  "headings": [
    {"level": 2, "title": "Permissions", "section": "permissions"}
  ]
}
```

`--level N` returns only level `N`; `--max-level N` returns levels one through `N`. A caller copies `section` into `help section <section>`. A section response uses:

```json
{
  "sections": [
    {
      "level": 2,
      "title": "Permissions",
      "section": "permissions",
      "content": "This command needs repository write access."
    }
  ]
}
```

The default section response contains each matching heading and its direct body. `--recursive` includes descendants. Repeated section names return every match in document order.

Search covers the current command and its descendants:

```json
{
  "results": [
    {
      "command_path": ["repo", "clone"],
      "section": "permissions",
      "title": "Permissions"
    }
  ]
}
```

Every `section` from outline or search must work with the corresponding section command.

## Checker limits and evidence

The checker closes stdin, captures stdout and stderr without a terminal, sets `CLILINT_OFFLINE=1`, applies a per-command timeout, and rejects terminal control sequences. Its default limits are:

| Limit | Default |
| --- | ---: |
| Discovered command paths | 64 |
| Command depth | 8 |
| One captured document | 1 MiB |
| Search results | 256 |
| Total help commands | 1,024 |
| One command | 2 seconds |

Bundle authors can set `command_count`, `command_depth`, `document_bytes`, `search_results`, `total_commands`, and `timeout_ms` in a hierarchical checker. A zero value is invalid.

Each JSON finding records relevant command paths, commands run, captured output, requested sections, and validation failures. The web-view check declares `required_for_ratings = ["good", "excellent"]`; calculating those ratings belongs to a later change.

## Make another bundle

Copy the shape of the [`codekiln-help` bundle](../check-bundles/codekiln-help/clilint.toml), choose a new bundle name and check IDs, and use only supported checker types. A local bundle can extend `codekiln-help`:

```toml
format_version = 1
extends = "codekiln-help"

[check_bundle]
name = "team-help"
version = "1.0.0"

[[checks]]
id = "team-help/help/team-option"
title = "Help describes the team option"
severity = "warn"
evaluation_method = "deterministic"

[checks.checker]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["--team"] }]
```

Install `codekiln-help` and the local extension. Clilint reports `clilint`, `codekiln-help`, then `team-help`, and it rejects any extension that excludes or weakens inherited checks.
