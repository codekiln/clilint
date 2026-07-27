# `codekiln-help`

`codekiln-help` checks whether people and programs can navigate a CLI tool's
complete local documentation. The bundle also shows how to implement a Checker
CLI.

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
```

`<command path> help` and `<command path> --help` describe the same command.
Each help operation writes to standard output and completes without input.
Callers can pipe the output to tools such as `less`, `rg`, `grep`, or `jq`.
This follows [clig.dev's output guidance](https://clig.dev/#output), which
recommends output that composes with other programs and limits pagers to
interactive streams.

The default help describes the command's purpose, behavior, usage, side
effects, and required permissions when they apply. `--programmatic` retains
that help and adds instructions for piping, JSON output, section retrieval,
and avoiding interactive displays.

## JSON responses

`help --format json` identifies the current command and its immediate child
commands:

```json
{
  "format_version": 1,
  "command_path": ["repo"],
  "programmatic": false,
  "child_commands": [{"name": "clone"}]
}
```

Callers discover the complete command hierarchy by requesting JSON help for
each returned child. The first JSON format supports command discovery; it does
not repeat every argument or option. The text returned by `help` and `--help`
describes the command's usage, arguments, options, and child commands.

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
Every `section` returned by an outline must work with the corresponding
section command.

## Checker behavior

The Checker:

- recursively discovers command paths;
- compares `help` with `--help`;
- validates JSON command paths, outline headings, `--level` and `--max-level`
  results, and section retrieval;
- verifies the added programmatic guidance;
- closes input and captures output without a terminal;
- bounds command count, depth, document bytes, total commands, and time per
  command.

The first implementation uses fixed limits:

| Limit | Value |
| --- | ---: |
| Discovered command paths | 64 |
| Command depth | 8 |
| One captured stream | 1 MiB |
| Total help commands | 1,024 |
| One help command | 2 seconds |

The Checker returns `4.0` when every expectation is met. Each unmet
expectation lowers the Score and adds an Error Check Message with the relevant
command path, invocation, output, and validation detail.

The [passing fixture](../tests/fixtures/hierarchical-help-cli) and
[integration tests](../tests/cli.rs) cover passing help, nested failures,
malformed JSON, invalid command paths and headings, and incomplete
programmatic guidance.
