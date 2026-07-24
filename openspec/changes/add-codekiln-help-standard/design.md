## Context

Clilint runs checks against a command-line program. This design calls that
program the **tested CLI tool**.

Clilint currently includes one built-in set of checks. A user can add checks
from one local file, but the repository does not contain a complete optional
set that demonstrates how to define and distribute a separate standard.

`codekiln-help` serves two purposes:

1. It checks an opinionated help interface for people and AI agents.
2. It gives check-bundle authors a complete example they can copy and adapt
   when making their own Clilint check bundles.

The experiments under `experiments/` tested two possible help interfaces. Their
results are summarized in `experiments/findings.md`.

## Goals / Non-Goals

**Goals:**

- Define an offline help interface for CLI tools with nested commands.
- Let any caller browse complete help or request one relevant section.
- Add programmatic-use guidance when a caller needs a non-interactive route
  through the same help interface.
- Make `codekiln-help` installable separately from other future codekiln check
  bundles.
- Use `codekiln-help` to teach bundle authors how custom check bundles are
  organized, tested, installed, and extended.
- Report a separate Clilint result for each help check.

**Non-Goals:**

- Standardize how tested CLI tools implement the required help behavior.
- Standardize the terminal viewer, Markdown renderer, and browser used by
  tested CLI tools.
- Compare local documentation with a website during a Clilint check.
- Use AI judgment for checks that Clilint can perform mechanically.

## Decisions

### Use `check` and `check bundle` as the reader-facing terms

A **check** is one named expectation that Clilint evaluates and reports
separately. The `codekiln-help` checks are all the expectations supplied by
`codekiln-help`.

A **check bundle** is a named, versioned set of related checks. A check bundle
can extend another check bundle. `codekiln-help` is one check bundle.

A **checker** is the code that gathers evidence and decides the result of a
deterministic check. One checker can support several checks. An **AI
assessment** supplies the result for a check that requires judgment.

This vocabulary gives each term one job. It also avoids explaining that a
"rule" contains a "check" inside a "package." The current command-line option,
manifest fields, Rust types, and JSON report still use those older terms.
This change will rename those public interfaces directly, without
compatibility aliases.

### Treat `codekiln-help` as a worked check-bundle example

`codekiln-help` will use the same check-bundle format and checkers available
to other bundle authors. Its bundle file will pass the same validation as a
bundle supplied by a user.

The check-bundle documentation will explain:

- the bundle file and its identity;
- how it includes the core `clilint` checks;
- how each check names one observable behavior;
- how a check names a supported checker;
- how to install the bundle and run its checks;
- how another bundle can extend it; and
- how its fixtures and tests demonstrate passing and failing behavior.

The source files will link to that guide. The guide will link back to the
relevant bundle entries and tests. A reader should be able to follow one check
from its bundle definition to the checker and the fixture that
demonstrates it.

### Put `help` after the command path

Every command path reserves a `help` child:

```text
<tool> [<command> ...] help
<tool> [<command> ...] help outline
<tool> [<command> ...] help section <section>
<tool> [<command> ...] help search <query>
<tool> [<command> ...] help view [--web]
```

For example, `tool repo clone help` describes `tool repo clone`.
`tool repo clone --help` provides the same command description.

This order keeps help operations distinct from command names. If the tested CLI
tool has a command named `view`, `tool view help` describes that command and
`tool help view` opens the root help document in a viewer.

### Discover child commands through ordinary help

The standard does not add a separate `help tree` operation. The JSON form of
ordinary help identifies the current command path and its immediate child
commands. A caller discovers the complete command hierarchy by requesting the
same JSON help for each child.

This keeps command discovery with the command description it belongs to and
removes one public operation. It also gives `outline` one meaning: the heading
structure inside the current command's help document.

### Use outlines to discover sections

`help outline` lists the headings in one help document.

- `--level N` returns headings at level N.
- `--max-level N` returns headings from level one through N.

Each result includes a `section` value that can be copied into `help section`.
The tested CLI tool can use author-written section names or generate the values
from headings.

The same `section` value may appear on more than one heading.
`help section <section>` returns all matching headings in document order. It
returns each heading and its direct text. `--recursive` also returns child
sections.

The default text interface accepts the section as a separate argument. JSON
results keep the command path and `section` in separate fields.

### Treat agent use as an accessibility concern

The preferred design gives every caller the same documentation and interface.
The `--programmatic` option is an optional route through that interface when
programmatic use needs additional instructions. It is not a separate source of
command truth.

The default help is the primary documentation for every caller. It should give
both people and agents the command's purpose, behavior, and usage, plus side
effects and required permissions when they apply. An agent should not need a
separate document to understand the command.

Every help operation also accepts `--programmatic`. The name describes the way
the interface will be used rather than who will use it. This view adds
instructions only when programmatic use benefits from a different way to use
the same interface. Useful additions include how to:

- pipe or redirect output;
- request JSON;
- retrieve only the relevant section;
- avoid pagers and full-screen terminal interfaces; and
- suppress animations, progress displays, and extra notifications.

The `--programmatic` view includes the default help rather than replacing it.
Clilint checks the commands and their output, not how the tested CLI tool
produces that output. JSON output does not need to identify which source
supplied a section.

### Keep ordinary help commands non-interactive

`help`, `outline`, `section`, and `search` write to standard output and
do not start a pager.

`help view` is for human reading. It can open a terminal viewer when standard
output is a terminal. When another program captures the output, it writes the
document and exits.

`help view --web` opens the matching web page when used interactively. When
another program captures the output, it prints the URL and does not open a
browser.

### Install `codekiln-help` independently

The core `clilint` check bundle remains built in. A user can install
`codekiln-help` without installing other codekiln check bundles. After
installation, Clilint runs the core checks followed by the help checks. A local
check bundle can extend `codekiln-help`, so the report lists:

```text
clilint
codekiln-help
local-check-bundle
```

Installing a check bundle makes its checks part of later Clilint runs. There is
no separate activation step for each run. Clilint also accepts a local
bundle as an installation source.

### Reuse one help checker across the checks

Several `codekiln-help` checks need the same command hierarchy and help
documents.
Clilint will gather that information once and reuse it for each check.

The hierarchical help checker will:

1. discover immediate child commands recursively from JSON help;
2. reject malformed output and stop at configured size limits;
3. run help commands at every discovered command path;
4. read the default and `--programmatic` outlines;
5. copy returned `section` values into section commands;
6. test search and non-interactive viewer behavior; and
7. record the commands, outputs, sections, and failures in the report.

Each check still receives its own pass, warning, or failure. Sharing the
collected information avoids running the same help command once per check.

### Assign web help to the higher rating levels

A versioned web page is required for `Good` and `Excellent` ratings from
`codekiln-help`. Clilint also reports the web help check separately so the user
can see why the tested CLI tool did not reach one of those ratings.

## Risks / Trade-offs

- **A tested CLI tool can advertise a very large command hierarchy** → Limit command
  count, command depth, document size, search results, total commands run, and
  time per command.
- **Generated section values can change when headings change** → Use the
  `section` value from the current outline or search result.
- **Added programmatic guidance can appear more than once** → Check the final
  document returned by `--programmatic` and report repeated or misplaced
  guidance through the relevant check.
- **Interactive viewers behave differently across systems** → Test the
  non-interactive behavior in the normal Clilint run and keep interactive tests
  separate.

## Migration Plan

1. Rename the public options, data fields, and Rust types without compatibility
   aliases.
2. Add support for installing named check bundles.
3. Add the hierarchical help checker and its report evidence.
4. Add `codekiln-help` using the same format as user-authored check bundles.
5. Add tested CLI fixtures that demonstrate passing and failing checks.
6. Publish the check-bundle authoring guide and the help-interface guide.
7. Keep the experiments with the change until verification is complete.

Existing checks continue to use only the core `clilint` check bundle until the
user installs another bundle.

## Resolved Questions

### 1 - Should the `section` value be written by the author, generated from the help document, or either?

Either. `codekiln-help` checks whether callers can retrieve named sections. It
does not require one way of producing the `section` value. If the same value
names several headings, the section command returns every match in document
order.

### 2 - When programmatic guidance combines shared and command-specific text, must its JSON output identify which source supplied each section?

No. `codekiln-help` checks the complete document returned by `--programmatic`.
Clilint reports a separate result for every tested check.

### 3 - At which `codekiln-help` rating levels is a versioned web page required?

A versioned web page is required for `Good` and `Excellent` ratings. The
web-page check also appears as its own result.

### 4 - Should calculation and reporting of the `Poor`, `Minimal`, `Acceptable`, `Good`, and `Excellent` ratings be part of this change or a separate OpenSpec change?

Use a separate OpenSpec change because this change is already large. A
prototype may remain in this change's `experiments/` directory if it helps
shape the later proposal.

### 5 - Should this change rename the public `--package` option, manifest fields such as `[package]` and `[[rules]]`, Rust types, and JSON report fields, or should it add the new terms through compatibility aliases first?

Rename the public terms directly. Clilint is in the `0.0.x` stage and does not
promise backward compatibility. Keeping aliases would preserve the ambiguous
terms without providing a needed compatibility benefit.

### 6 - Which flag should request the optional programmatic-use guidance?

Use `--programmatic`. It describes how the interface will be used and remains
available to people, scripts, and agents.

## Open Questions

### 7 - Where should Clilint obtain a named check bundle during installation?

<ANSWER_HERE>

### 8 - Should check-bundle installation apply to one project, one user, or an explicit location chosen by the user?

<ANSWER_HERE>

## Citations

- [My/Principle/CLI/Centricity/Offline Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___CLI___Centricity___Offline%20Tools.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Principle/Simplify/Minimize Surface Area](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Simplify___Minimize%20Surface%20Area.md)
