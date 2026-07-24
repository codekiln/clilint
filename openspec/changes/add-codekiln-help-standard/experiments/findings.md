# Findings from the help interface experiments

## What worked in both prototypes

- A command document and its heading hierarchy can be represented independently.
- An outline can return `section` values that a later command copies without
  reconstructing them.
- `--level N` selects one heading level. `--max-level N` selects levels one
  through N.
- Direct section retrieval gives a small response. An explicit recursive option
  retrieves the complete section subtree.
- The prototypes included document digests, although the behavioral standard
  does not need to require them.

## Postfix help

The `<command path> help <operation>` form remained clear when the command hierarchy
contained a command named `view`:

```text
kiln-demo view help
kiln-demo repo clone help view
```

It also made `<command path> help` and `<command path> --help` clear ways to
request the same command description. The design uses this interface.

The prototype exposed command discovery and document headings as `tree` and
`outline`. Although the data differed, the neighboring names made callers
learn two hierarchy operations. The design removes `tree`: JSON help returns
immediate child commands, while `outline` refers only to document headings.

## Viewing one help section

The resource prototype combined a command path and `section` value:

```text
repo/clone#kiln-demo-repo-clone.examples.private-repository
```

The combined value worked, but it added a format that callers would need to
learn. The design keeps `command_path` and `section` as separate JSON fields
and passes the `section` value to `help section`. The prototype's `list`, `get`,
and `view` commands were also less clear as the main interface because they no
longer followed the tested CLI tool's command path.

The experiment also produced verbose `section` values that would change when a
heading or ancestor heading changes. The standard requires copyable values
while allowing repeated headings to share one value.

## Shared and programmatic documentation

The separate agent page in the postfix prototype was easy to understand but
made an agent choose between command instructions and programmatic guidance.

The resource prototype composed the shared page with inherited programmatic
guidance. That gave the caller one complete page and avoided repeating general
guidance at every command. It also lengthened outlines and raised questions
about ordering, overriding inherited guidance, and identifying the source of a
composed section.

The design now treats default help as the primary documentation for people and
agents. The optional `--programmatic` view keeps that help and adds
instructions for programmatic use, such as piping, JSON, section retrieval,
and avoiding interactive output. The check bundle specifies the commands and
returned content, not how the tested CLI tool produces them.

## Viewer behavior

Printing a web URL when another program captures the output is predictable and
safe to test. The tested CLI tool chooses its local viewer and browser-opening
program.

## Implications for Clilint

Clilint's current checks run commands with arguments known in advance. The help
check must also:

1. discover command paths from JSON output;
2. run checks for every discovered path;
3. parse JSON outlines;
4. copy discovered `section` values into later invocations; and
5. validate that section responses agree with the advertised hierarchy.

Clilint therefore needs one hierarchical help check that can share these
results across the separate `codekiln-help` checks. The finished check bundle
will also demonstrate this checker for authors of custom check bundles.

## Implementation verification

The eight experiment tests still pass after implementing the checker and
bundle. The implementation confirmed the existing decisions: ordinary JSON
help is sufficient for recursive command discovery, outline and search results
can feed exact `section` values into later commands, and direct and recursive
section retrieval need separate checks. No experiment result required a design
change.
