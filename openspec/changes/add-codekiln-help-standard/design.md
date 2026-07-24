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

For this change, installation applies to one project. The project records its
installed check bundles in `.clilint/config.toml` in the current working
directory. Clilint reads that file and includes every installed bundle in later
checks without a separate activation step. It does not use the Git root or
search parent directories in this version. This lets a monorepo configure each
CLI tool from its own directory.

Installation is declarative. The project records where a bundle comes from
instead of requiring the bundle contents to be copied into the project. A
source can refer to a Git repository and ref. Clilint also accepts a local
bundle as an installation source so an author can develop and test a bundle
before publishing it.

The committed configuration uses explicit fields instead of packing the
repository, ref, and path into one string:

```toml
[check_bundles.example]
source = "git"
url = "https://example.com/check-bundles.git"
ref = "v1.0.0"
path = "bundles/example"
```

`source` distinguishes a Git source from a local path. A Git source names its
repository URL and may name a branch, tag, or commit in `ref`. It may also name
the directory containing one bundle in `path`. The table name must match the
name declared by the bundle after Clilint loads it.

An installation command may accept a compact source spelling for convenience,
but it writes the explicit fields to `.clilint/config.toml`. Clilint does not
map `codekiln-help` or another short name to a hidden source in this version.
Such mappings would require a catalog similar to a Claude Code plugin
marketplace. A later change can add catalogs without changing the explicit
source format.

#### Offer an optional project lockfile

A project can commit `.clilint/lock.toml`. `clilint bundle lock` resolves each
requested Git ref to a full commit SHA and records enough source information to
verify the downloaded bundle. Once the lockfile exists, ordinary installation
and checking preserve its resolved commits. They do not advance a branch or tag
merely because its remote value changed.

`clilint bundle update [<name>]` is the explicit way to re-resolve a requested
ref. It updates every bundle when no name is given and only the named bundle
otherwise. A `--locked` option makes commands fail when the lockfile is absent,
does not match the declarations, or would need to change.

This follows the useful behavior shared by mise and uv: the declaration can
remain readable and flexible while the optional lockfile records the exact
result. Unlike Dev Container Features, Clilint does not create a lockfile by
default in this version.

#### Separate installed data from disposable cache

Resolved check bundles are installed data because later checks depend on them.
They live under the Clilint data directory rather than inside the project or
the disposable cache. `CLILINT_DATA_DIR` overrides the location. Otherwise
Clilint uses `XDG_DATA_HOME/clilint`, with the platform-appropriate XDG fallback
when `XDG_DATA_HOME` is unset.

Temporary Git clones, downloads, and source metadata use the Clilint cache
directory. `CLILINT_CACHE_DIR` overrides the location. Otherwise Clilint uses
`XDG_CACHE_HOME/clilint`, with the platform cache directory as the fallback.
Deleting this cache does not remove installed bundles.

This distinction follows mise: files that ordinary use requires belong in its
data directory, while files that may be deleted and regenerated belong in its
cache directory.

#### Install a missing declared bundle before checking

An ordinary `clilint check` makes the declared configuration true before it
checks the tested CLI tool. If an installed bundle is missing, Clilint
downloads, validates, and installs it before running any check. It uses the
locked commit when a lockfile exists. Without a lockfile, it reuses the locally
installed resolution until the user requests an update.

Clilint never silently skips a declared bundle. If installation fails, the
whole check fails before it runs the tested CLI tool. `--offline` prohibits
network requests; when required content is unavailable locally, Clilint names
the missing bundles and prints the command that can install them later.

This makes the complete check set the easy path while making a partial,
misleading report impossible. It resembles the default auto-install behavior
of `mise run` and the automatic lock and sync behavior of `uv run`, while still
providing strict offline and locked modes.

#### Manage declarations under one `bundle` command

The first command surface is:

```text
clilint bundle install [<source>]
clilint bundle lock [<name>]
clilint bundle list [--json]
clilint bundle update [<name>]
clilint bundle remove <name>
```

`install <source>` adds or updates one declaration and installs it. `install`
without a source installs anything already declared but missing, which is the
normal command after cloning a project. `lock` creates or completes the
optional lockfile without advancing entries that are already locked. `list`
shows each declaration, requested ref, resolved commit, source, and installation
status. `update` explicitly advances requested refs. `remove` removes the
declaration and its active project installation.

This keeps Claude Code's familiar install, list, update, and remove operations,
but groups them under Clilint's singular `bundle` command. A separate `add`
command would duplicate the declaration-writing behavior of `install`.

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
- **A moving Git ref can produce different checks on different machines** →
  Offer the committed lockfile, preserve existing resolutions, and advance refs
  only through `bundle update`.
- **A project declaration can cause a network request during `check`** → Show
  which missing bundles are being installed and provide `--offline` and
  `--locked` modes for callers that prohibit resolution or downloads.
- **A remote bundle is untrusted input** → Accept only declarative checks using
  supported checkers, validate the complete bundle before activation, and
  replace installed contents atomically.
- **An unavailable bundle could produce a report with fewer checks than the
  project declared** → Fail the entire run before checking the tested CLI tool.

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

## Open Questions

None for the first `codekiln-help` proposal.

## Citations

- [My/Principle/CLI/Centricity/Offline Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___CLI___Centricity___Offline%20Tools.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Principle/Simplify/Minimize Surface Area](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Simplify___Minimize%20Surface%20Area.md)
- [My/Pref/Dev/Tool/Prefer XDG-Compliant CLI Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Dev___Tool___Prefer%20XDG-Compliant%20CLI%20Tools.md)
- [My/Principle/Make the Right Thing Easy and the Wrong Thing Hard](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20the%20Right%20Thing%20Easy%20and%20the%20Wrong%20Thing%20Hard.md)
