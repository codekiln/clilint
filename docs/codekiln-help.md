# `codekiln-help`

`codekiln-help` checks whether people and programs can navigate a CLI tool's
complete local documentation. The bundle is also a working example of a
bundle-owned Checker CLI.

Install it in the directory containing the tested project:

```sh
clilint bundle install ./check-bundles/codekiln-help
clilint check ./my-cli
```

The [bundle file](../check-bundles/codekiln-help/clilint.toml) defines one
hierarchical-help Check. Its
[Checker CLI](../check-bundles/codekiln-help/checker.py) gathers the command
hierarchy and help documents once, verifies the related expectations, and
returns one Score with focused Check Messages.

## Interface for tested CLI tools

Every advertised command path provides:

```text
<tool> [<command> ...] help
<tool> [<command> ...] help outline
<tool> [<command> ...] help section <section> [--recursive]
<tool> [<command> ...] help search <query>
```

`<command path> help` and `<command path> --help` describe the same command.
Each help operation writes to standard output without starting a pager or
requiring input.

The default help describes the command's purpose, behavior, usage, side
effects, and required permissions when they apply. `--programmatic` retains
that help and adds instructions for piping, JSON output, section retrieval,
and avoiding interactive displays.

## JSON responses

Every JSON response identifies its format, command path, and programmatic
mode:

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

Callers discover the complete command hierarchy by requesting JSON help for
each returned child.

An outline adds ordered headings:

```json
{
  "headings": [
    {"level": 2, "title": "Permissions", "section": "permissions"}
  ]
}
```

`--level N` returns only level `N`; `--max-level N` returns levels one through
`N`. A caller copies `section` into `help section <section>`.

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

The default section response contains each matching heading and its direct
body. `--recursive` includes descendants. Repeated section names return every
match in document order.

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

Every `section` returned by outline or search must work with the corresponding
section command.

## Checker behavior

The Checker:

- recursively discovers command paths;
- compares `help` with `--help`;
- validates outlines, filters, sections, and search results;
- verifies the added programmatic guidance;
- closes input and captures output without a terminal;
- sets offline behavior for tested help invocations; and
- bounds command count, depth, document bytes, search results, total commands,
  and time per command.

The first implementation uses fixed limits:

| Limit | Value |
| --- | ---: |
| Discovered command paths | 64 |
| Command depth | 8 |
| One captured stream | 1 MiB |
| Search results | 256 |
| Total help commands | 1,024 |
| One help command | 2 seconds |

The Checker returns `4.0` when every expectation is met. Each unmet
expectation lowers the Score and adds an Error Check Message with the relevant
command path, invocation, output, and validation detail.

The [passing fixture](../tests/fixtures/hierarchical-help-cli) and
[integration tests](../tests/cli.rs) cover passing help, nested failures,
malformed JSON, invalid command paths and headings, and empty search results.
