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
- Define user configuration, XDG configuration, parent-project inheritance, or
  local configuration overrides for Clilint. A later change can add that
  hierarchy.

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

### Keep every help command non-interactive

`help`, `outline`, `section`, and `search` write to standard output and do not
start a pager. Nothing in the standard behaves differently on a terminal, so a
person and a program run the same commands and get the same text. A person who
wants paging or rendering composes one: `tool help | less`.

Rich local reading and web viewing are deferred to issue #9. They return only
once they can detect the environment a caller is in.

### Install a check bundle from a local path

The core `clilint` check bundle remains built in. A user can install
`codekiln-help` without installing other codekiln check bundles. After
installation, Clilint runs the core checks followed by the help checks. A local
check bundle can extend `codekiln-help`, so the report lists:

```text
clilint
codekiln-help
local-check-bundle
```

Installation applies to one project. The project records its installed check
bundles in `.clilint/config.toml` in the current working directory. Clilint
reads that file and includes every installed bundle in later checks without a
separate activation step. It does not use the Git root or search parent
directories. This lets a monorepo configure each CLI tool from its own
directory.

Installation is declarative. The project records where a bundle comes from
rather than copying the bundle contents into the project:

```toml
[check_bundles.example]
source = "local"
path = "check-bundles/example"
```

`source` names the kind of source, so a later change can add another kind
without changing the shape of the file. `path` is relative to the project
directory, so the committed file works for everyone who clones the project. The
table name must match the name declared by the bundle after Clilint loads it.

A local path is enough to show what this change needs to show: a check bundle
installs independently of the Clilint binary, and installed bundles compose in a
declared order. Retrieving a bundle from a Git repository, resolving a ref to a
commit, committing a lockfile, and storing downloaded contents outside the
project form a separate capability, tracked outside this change.

#### Manage declarations under one `bundle` command

```text
clilint bundle install <path>
clilint bundle list [--json]
clilint bundle remove <name>
```

`install <path>` records one declaration and confirms that the bundle loads and
validates. `list` shows each declaration and whether Clilint can load it.
`remove` removes the declaration. Recording the declaration is part of
installing a bundle, so there is no separate `add` command.

Clilint never silently skips a declared bundle. If a declared bundle fails to
load, the whole check fails before Clilint runs the tested CLI tool, rather than
producing a report with fewer checks than the project declared.

User-level defaults, parent-directory discovery and merging, and local
overrides remain deferred. The future hierarchy should draw on mise's
configuration model, but this change will not establish its precedence rules.

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
- **A check bundle chooses the arguments, environment, and input given to the
  tested CLI tool** → Accept only declarative checks using supported checkers
  and validate the complete bundle before running anything.
- **An unloadable bundle could produce a report with fewer checks than the
  project declared** → Fail the entire run before checking the tested CLI tool.

## Migration Plan

1. Rename the public options, data fields, and Rust types without compatibility
   aliases.
2. Add support for installing a check bundle from a local path.
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

### 7 - Must installation copy a check bundle into the project that develops the tested CLI tool?

No. Installation records a source declaration in project configuration. A
source can refer to a Git repository and ref, so a project can install a bundle
without committing a copy of its contents. A local bundle path is also
supported for bundle development.

### 8 - Should check-bundle installation apply to one project, one user, or an explicit location chosen by the user?

Use project-level installation for this change. Later changes can add user
configuration, XDG configuration, parent-project inheritance, and local
overrides.

### 9 - What project dot directory and files should hold installed check-bundle declarations, and how should Clilint find the project root?

Use `.clilint/config.toml` in the current working directory. That directory is
the project root for the current invocation. Clilint does not infer the project
root from `.git` and does not search parent directories in this version. This
allows separate CLI tools inside one monorepo to use separate configurations.

A later change may search parent directories and merge project, user, XDG, and
local configuration. That change must define precedence before enabling the
search.

### 10 - How should a source declaration identify a Git repository, ref, and optional bundle path, and should a short name such as `codekiln-help` map to a known source?

Use explicit TOML fields for the source type, repository URL, requested ref,
and optional path within the repository. A command may accept a compact
spelling inspired by RuleSync, but it writes the expanded fields to
`.clilint/config.toml`.

The configured name must match the name declared by the downloaded bundle.
Clilint does not map `codekiln-help` to a hidden source in this version. A
future catalog can provide trusted short-name mappings in the same way that a
Claude Code marketplace maps plugin names to source declarations.

### 11 - Should Clilint resolve a requested Git ref to an exact commit in a committed lockfile, and how should a user request an update?

Yes, as an optional affordance. `clilint bundle lock` creates
`.clilint/lock.toml`, which records each requested ref's full commit SHA. The
project should commit that file. Once it exists, install and check commands
preserve its resolved commits.

`clilint bundle update [<name>]` explicitly advances one named bundle or every
bundle when no name is given. `--locked` fails rather than creating or changing
the lockfile.

### 12 - Where should Clilint cache resolved bundle contents, and what should happen when the required contents are missing?

Installed bundle contents are data, not cache. Store them under
`CLILINT_DATA_DIR` when set or the platform's `XDG_DATA_HOME/clilint`
location otherwise. Store disposable Git clones, downloads, and source
metadata under `CLILINT_CACHE_DIR` or the platform's
`XDG_CACHE_HOME/clilint` location.

If installed contents are missing, Clilint restores them before checking.
Deleting the disposable cache does not remove installed bundles. If restoration
is impossible, Clilint fails before running any check and identifies the
missing bundles.

### 13 - May an ordinary `clilint check` download a missing bundle, or must a separate installation command perform every network request?

An ordinary `clilint check` may download a declared bundle when its installed
contents are missing. It does not refresh an installed branch or tag during an
ordinary check. Clilint either runs the complete declared check set or fails;
it never omits an unavailable bundle and produces a partial report.

`--offline` prohibits network access and fails with the missing bundle names
and the command needed to install them later. `--locked` also requires the
project lockfile to cover the declarations without changing it.

### 14 - Which commands should add, update, list, and remove project check-bundle declarations?

Use one `bundle` command group:

```text
clilint bundle install [<source>]
clilint bundle lock [<name>]
clilint bundle list [--json]
clilint bundle update [<name>]
clilint bundle remove <name>
```

`install <source>` records and installs one bundle. `install` without a source
installs any declared bundle that is missing. `lock` creates or completes the
optional lockfile without advancing existing entries. `list` reports declared
and resolved state. `update` advances requested refs. `remove` removes the
project declaration and active installation.

Do not add a separate `add` command. Recording the declaration is part of
installing a bundle.

### 15 - Should this change ship remote Git sourcing and the lockfile?

No. This change needs to show that the foundation for installing check bundles
independently of the Clilint binary is in place. Declaring a local path to a
check bundle shows that. Everything beyond it is a second capability that
should carry its own change.

This narrows the answers to questions 10 through 14. Keep the declarative
`.clilint/config.toml` file, the `source` field, the requirement that the table
name match the loaded bundle, the rule that a declared bundle never gets
silently skipped, and the single `bundle` command group with `install`, `list`,
and `remove`. Defer the Git source type, ref resolution, `.clilint/lock.toml`,
`bundle lock`, `bundle update`, `--locked`, `--offline`, the data and cache
directories, and short-name catalogs. The deferred work is issue #8.

### 17 - Should the standard keep the `view` operation?

No, not in this change. Removing it leaves `help`, `outline`, `section`, and
`search`, all of which write to standard output and none of which behave
differently on a terminal. That removes the terminal-versus-captured split, two
of the ten `codekiln-help` checks, the risk that viewers differ across systems,
and the requirement that a resolved URL contain the tool's version string.

Rich local reading and web viewing are still wanted. `gh` shows why: it renders
Markdown rather than printing it raw, and `--web` means the same thing
everywhere in the tool. Both should return once they can respect the
environment a caller is in, following clig.dev on pagers, color, and captured
output, so that an agent reading help through a pipe gets plain text without
asking. That work is issue #9, which also needs to carry the web-page
requirement that answer 3 attached to the `Good` and `Excellent` ratings.

## Open Questions

### 16 - How should a check bundle express new behavior without a change to the Clilint binary?

> Context from the drafting agent, for question 16.
>
> Clilint exists so that a person or team can write their own standard, install
> it, and have an AI agent verify a CLI tool against it while building that
> tool. A bundle that requires a change to Clilint cannot serve that purpose.
>
> `codekiln-help` requires one. Each of its checks names
> `type = "hierarchical-help"` and one `behavior` value from a fixed set, so the
> bundle file states which behaviors to evaluate while `src/help_checker.rs`
> decides what each behavior means.
>
> The declarative vocabulary is missing four things that `codekiln-help` needs:
> reading values out of a command's JSON output, iterating over the values it
> read, substituting them into later invocations, and repeating that on
> discovered child commands. Every part of `src/help_checker.rs` is those four
> capabilities applied to help.

<ANSWER_HERE>

## Citations

- [My/Principle/CLI/Centricity/Offline Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___CLI___Centricity___Offline%20Tools.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Principle/Simplify/Minimize Surface Area](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Simplify___Minimize%20Surface%20Area.md)
- [My/Principle/Make the Right Thing Easy and the Wrong Thing Hard](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20the%20Right%20Thing%20Easy%20and%20the%20Wrong%20Thing%20Hard.md)
- [My/Principle/Declarative over Imperative](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Declarative%20over%20Imperative.md)
