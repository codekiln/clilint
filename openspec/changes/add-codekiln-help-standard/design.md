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

The experiments under `experiments/` compare help interfaces and check
extension models. Each named experiment records its result in its own
`findings.md`.

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
- Let an installed bundle add a complete check without adding check-specific
  code to Clilint.
- Return one outcome for each check. A completed result gives the check's
  score and focused messages that explain what affected the score.

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

### Give each check term one meaning

A **check** states an expectation about a quality or aspect of a project. It
includes documentation of what is checked and why, a way to gather relevant
evidence, and a way to score that evidence. Its author chooses its logical
scope. One check may gather evidence once and verify several related matters.

A **check bundle** is a named, versioned set of related checks. A check bundle
can extend another check bundle. `codekiln-help` is one check bundle.

A **checker** performs a check's complete lifecycle: any one-time setup,
evidence gathering, and production of a Check Outcome.

A check is **judgment-based** when human or model interpretation affects its
Score or Check Messages. Otherwise, it is **mechanistic**. A judgment-based
check can still run scripts and other tools while gathering evidence.

A mechanistic check uses one or more metrics to map evidence to a Score. A
judgment-based check uses one or more rubrics. Either method may combine
several weighted parts internally, but the first protocol returns one
aggregate Score and does not standardize component scores. An **Assessment**
is the structured record of a human or model applying rubrics to evidence. To
**assess** is the act that creates that record. Assessment is not the shared
name for every check's output.

Every run produces one **Check Outcome**. The outcome is exactly one of:

- a **Check Result**, which contains a Score and Check Messages;
- a **Check Error**, which means the checker did not produce a valid result;
- Awaiting Assessment, for a judgment-based Checker CLI that still needs
  judgment; or
- Skipped, when the check does not apply.

A **Score** is a finite floating-point number from `0.0` through `4.0`,
inclusive, where higher is better and fractional values are allowed. A binary
check uses `0.0` or `4.0`. The protocol preserves the score returned by the
checker; rounding is a presentation choice.

A **Check Message** is structured feedback attached to a Check Result. It
explains an observation that affected or helps interpret the Score and
includes supporting evidence when available. The first protocol has Info,
Warning, and Error message levels. Checker logs are separate from Check
Messages.

A Check Result with a Score below `4.0` must include at least one Check Message
that explains what could improve. A result with a Score of `4.0` cannot
contain a Warning or Error message. Clilint rejects a result that breaks these
rules.

A Check Error and a Check Result are mutually exclusive. An Error-level Check
Message is different: it describes a serious problem in the tested project
inside an otherwise valid Check Result. A checker crash, timeout, or invalid
protocol response produces a Check Error and no Score.

This vocabulary gives each term one job. It also avoids explaining that a
"rule" contains a "check" inside a "package." The current command-line option,
manifest fields, Rust types, and JSON report still use those older terms.
This change will rename those public interfaces directly, without
compatibility aliases.

### Treat `codekiln-help` as a worked check-bundle example

`codekiln-help` will use the same check-bundle format and Checker CLI protocol
available to other bundle authors. Its bundle file will pass the same
validation as a bundle supplied by a user.

The check-bundle documentation will explain:

- the bundle file and its identity;
- how it includes the core `clilint` checks;
- how one check can verify several related behaviors;
- how a Check names its Checker CLI;
- how to install the bundle and run its checks;
- how another bundle can extend it; and
- how its fixtures and tests demonstrate passing and failing behavior.

The source files will link to that guide. The guide will link back to the
Checker CLI, Check Result, Check Messages, and relevant tests. A reader should
be able to follow the Check from its bundle definition to the Checker and the
fixture that demonstrates it.

### Put `help` after the command path

Every command path reserves a `help` child:

```text
<tool> [<command> ...] help
<tool> [<command> ...] help outline
<tool> [<command> ...] help section <section>
```

For example, `tool repo clone help` describes `tool repo clone`.
`tool repo clone --help` provides the same command description.

This order keeps help operations distinct from command names.

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

`help`, `outline`, and `section` write to standard output and do not start a
pager. Nothing in the standard behaves differently on a terminal, so a person
and a program run the same commands and get the same text. A person who wants
paging or rendering composes one: `tool help | less`.

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
installs independently of Clilint, and installed bundles compose in a declared
order. Retrieving a bundle from a Git repository, resolving a ref to a commit,
committing a lockfile, and storing downloaded contents outside the project form
a separate capability, tracked outside this change.

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

### Require a Checker CLI for each Check in a local bundle

Each Check in a local bundle points to one Checker CLI. In the first protocol,
this is what it means for a Check to be a CLI. The Check remains the logical
unit of verification, and the Checker CLI is its executable interface.

The CLI boundary is recursive:

```text
Clilint CLI
    |
    v
Checker CLI
    |
    +-- invokes the tested CLI tool
    +-- runs scripts or other tools
    +-- applies metrics or judgment
    |
    v
Check Outcome
```

The Checker CLI owns any one-time setup, evidence gathering, and production of
the Check Outcome. The CLI may be written in any language and may use scripts,
Agent Skills, rubrics, model runtimes, remote services, WebAssembly, or other
tools internally. Clilint does not need a separate checker implementation for
each of those choices.

Clilint runs the Checker CLI from the directory in which the user invoked
Clilint, normally the project being checked. Clilint resolves the Checker CLI
from the installed bundle before starting it, but it does not change the
working directory to the bundle. A Checker CLI finds its installed Skills,
rubrics, scripts, and other resources through its own executable or language
package.

Clilint gives the CLI a versioned Check Request that identifies the check
bundle, Check, tested CLI tool, project directory, and protocol version. The
CLI returns a versioned Check Outcome bound to that request. Clilint validates
the outcome before adding it to the report.

The check bundle declares the Checker CLI as a nonempty command argument
array. Clilint replaces the literal `{bundle}` placeholder in any argument
with the installed bundle directory. The first item names the program and the
remaining items are its arguments. The Checker inherits Clilint's environment
and operating-system permissions. An explicitly installed local bundle is
trusted code; the first protocol does not add a sandbox.

Setup state and intermediate evidence remain private to the checker. A later
protocol can expose phases if a concrete check needs Clilint to retain or
repeat one phase.

Clilint writes one JSON Check Request to the Checker's standard input and
closes it. The Checker writes one versioned JSON Check Outcome to standard
output. Clilint directs that output to a temporary file and parses it after the
Checker exits. Checker logs continue directly to Clilint's standard error and
stay outside the Check Outcome. Clilint stops a Checker that exceeds the fixed
run-time limit.

A judgment-based Checker CLI may return Awaiting Assessment, expose a bundled
Agent Skill and rubric to an external agent, and later validate a returned
Assessment before producing a Check Result. The first handoff is file-based:
Clilint records the pending request, Skill, rubric, and evidence; an external
agent writes an Assessment JSON file; and a later Clilint invocation supplies
that Assessment to the same Checker CLI. Clilint does not invoke or prescribe
an agent harness.

The built-in core checks continue to use checkers compiled into Clilint in this
change. They adopt the shared Check Outcome and Check Result model. Checks in
local bundles use Checker CLIs. A later change can migrate built-in checkers
after the CLI protocol has been used by a complete bundle.

The [Checker CLI contract comparison](experiments/checker-cli-contract-comparison/README.md)
demonstrates mechanistic and judgment-based Checker CLIs running from a tested
project directory while finding resources installed with each Checker.

### Begin `codekiln-help` as one complete check

The first `codekiln-help` bundle contains one hierarchical-help check. Its
Checker CLI gathers the command hierarchy and help documents once, verifies all
related behaviors, and returns one Check Result with focused Check Messages.

The checker will:

1. discover immediate child commands recursively from JSON help;
2. reject malformed output and stop at configured size limits;
3. run help commands at every discovered command path;
4. read the default and `--programmatic` outlines;
5. copy returned `section` values into section commands;
6. test non-interactive output; and
7. return focused evidence with each Check Message.

The bundle can split this work into several checks later if separate execution,
configuration, or reporting proves useful.

## Risks / Trade-offs

- **A tested CLI tool can advertise a very large command hierarchy** → Limit command
  count, command depth, document size, total commands run, and time per
  command.
- **Generated section values can change when headings change** → Use the
  `section` value from the current outline.
- **Added programmatic guidance can appear more than once** → Check the final
  document returned by `--programmatic` and report repeated or misplaced
  guidance in a focused Check Message.
- **A local bundle's Checker CLI runs code chosen by the bundle author** →
  Execute Checker CLIs only from explicitly installed bundles, keep execution
  out of the Clilint process, and document the permissions the CLI
  receives.
- **An outcome can be attached to the wrong or an earlier request** → Bind
  each outcome to the versioned request and reject a mismatched binding.
- **A Checker CLI can hang or return malformed output** → Limit its run time,
  parse one Check Outcome from a temporary file, and report process or
  protocol failures as Check Errors.
- **An unloadable bundle could produce a report with fewer checks than the
  project declared** → Fail the entire run before checking the tested CLI tool.

## Migration Plan

1. Rename the public options, data fields, and Rust types without compatibility
   aliases.
2. Reduce installation to local bundle paths and remove the deferred Git,
   lockfile, data-directory, cache-directory, and offline-restoration paths.
3. Remove local and web viewer behavior from the help standard, bundle,
   fixtures, tests, and documentation.
4. Introduce the versioned Check Outcome, Check Result, Score, Check Message,
   Check Error, and judgment-based Assessment model.
5. Preserve each installed bundle's directory long enough to resolve its
   Checker CLI and add the CLI protocol with a fixed run-time limit and
   temporary-file outcome capture.
6. Add the external Agent Skill handoff and judgment-based Assessment path.
7. Port hierarchical-help behavior into one `codekiln-help` Checker CLI and
   verify it with black-box tests.
8. Remove the fixed hierarchical-help checker and its special engine path
   after the replacement passes those tests.
9. Publish the check-bundle authoring guide and the help-interface guide.
10. Keep the experiments with the change until verification is complete.

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

### 16 - How can a check bundle define setup, evidence gathering, and assessment for a new check without changing the Clilint binary?

`codekiln` chose one bundle-owned Checker CLI for each complete Check. The
bundle can keep the Checker, supporting scripts, metrics, rubrics, Agent
Skills, references, and other resources with the Check definition. The
Checker CLI may use those resources internally.

Clilint supplies a versioned Check Request and requires a versioned Check
Outcome in return. The checker owns an imperative lifecycle:

1. optionally set up its environment;
2. gather evidence with the tools available to its runner; and
3. produce a Check Result by applying mechanistic metrics or judgment-based
   rubrics, or return another valid Check Outcome.

The check author chooses the logical scope of the check. One check may gather
evidence once and verify several related matters. A completed Check Result has
one Score and zero or more Check Messages. The same result structure applies
to mechanistic and judgment-based checks. Assessment remains specific to the
judgment-based path.

The first extension protocol standardizes the request and outcome
boundaries. Setup, intermediate evidence, and phase state remain inside the
checker. This lets one check retain resources and state for its complete
run without requiring Clilint to define phase ordering and state transfer.

`codekiln-help` will begin as one check. Its checker can gather the command
hierarchy once, verify all of the related help expectations, and return one
Score with several Check Messages. A later change can split it when a concrete
need makes separate checks useful.

A later change may expose setup, evidence gathering, or scoring as separate
protocol phases when a concrete Check needs Clilint to retain intermediate
evidence, repeat a phase, or manage a phase independently. Adding a new Check
requires only bundle files when its Checker CLI can work within the shared
process and data protocol. The Clilint binary changes only when a Checker
needs a new shared execution capability or a change to that protocol.

The drafting agent had recommended separate host-managed phases. `codekiln`
chose the whole-check model because it provides the required extension point
with less protocol. The first version does not add lifecycle hooks without a
concrete check that needs them.

The [check extension model comparison](experiments/check-extension-model-comparison/README.md)
records the whole-check and phase-oriented alternatives that led to this
answer.

### 18 - What overall result should the first shared check protocol report?

`codekiln` chose a continuous Score so an agent can measure smaller
improvements while working toward conformance. A completed Check Result
contains one finite Score from `0.0` through `4.0` and zero or more Check
Messages. A binary check uses `0.0` or `4.0`. The protocol preserves the
returned value instead of reducing it to `Poor`, `Minimal`, `Acceptable`,
`Good`, or `Excellent`.

Score and Check Messages have separate jobs. The Score says how well the
project meets the check. Check Messages say what affected the Score and what
could improve. A Score below `4.0` requires at least one improvement message.
A Score of `4.0` cannot contain a Warning or Error message.

The drafting agent had recommended a separate pass, warning, or failure result
and a five-level rating. `codekiln` rejected that duplication. Check Errors,
Awaiting Assessment, and Skipped remain separate Check Outcomes rather than
special Score values.

### 22 - Are Assessment and Finding the right names for the result of the Check and the warning or error within the assessment describing an unmet expectation?

No. `codekiln` chose **Check Result** for the completed output shared by every
check and **Check Message** for structured feedback within that result.
“Assessment” felt subjective and “Finding” did not explain what the item was.

Assessment is reserved for the structured record of a judgment-based check
applying a rubric to evidence. A judgment-based Assessment can produce the
same Check Result as a mechanistic calculation. In the existing application,
`AssessmentDocument` names a JSON file used by the old AI-only path. The new
domain term is Assessment; file loading is a transport concern rather than
part of the term.

The drafting agent had proposed Assessment as the shared result and Finding as
an unmet expectation. `codekiln` rejected those shared names and chose the
terms above.

### 23 - Should every bundle-owned Check be exposed as a CLI, and which working directory should it use?

Yes, for the first protocol. This answer narrows answer 16: `codekiln` chose
one Checker CLI as the executable interface for each bundle-owned Check. The
CLI owns the complete lifecycle and may invoke other CLIs, scripts, tools,
Agent Skills, or model runtimes internally.

Clilint runs the Checker CLI from the directory in which the user invoked
Clilint, normally the project being checked. The Checker locates its own
installed resources through its executable or language package rather than by
changing the process working directory to the bundle.

The drafting agent initially proposed using the bundle directory as the
working directory to simplify resource lookup. `codekiln` rejected that
special case because a Checker CLI should manage its own packaged resources
and should inherit the same project context as Clilint.

The [Checker CLI contract comparison](experiments/checker-cli-contract-comparison/README.md)
supports this decision with mechanistic and judgment-based prototypes. The
judgment-based prototype also shows that an Agent Skill can sit behind the
Checker CLI instead of becoming a separate Clilint checker implementation.

### 19 - What process contract should Clilint use when it invokes a Checker CLI?

The drafting agent chose the smallest ordinary child-process contract that
supports the settled design:

- the bundle declares one nonempty command argument array;
- Clilint expands the literal `{bundle}` placeholder to the installed bundle
  directory and starts the command without a shell;
- the Checker inherits the directory, environment, and operating-system
  permissions of the Clilint process;
- Clilint sends one JSON Check Request on standard input;
- the Checker returns one JSON Check Outcome on standard output;
- standard error contains operational Checker logs; and
- Clilint applies fixed timeout, protocol-output, and retained-log limits.

An explicitly installed local bundle is trusted code. Clilint validates the
declaration and protocol documents but does not add a sandbox. The first
version does not let bundles raise the limits. That avoids unbounded memory and
configuration before a concrete Checker demonstrates a need for tuning.

### 20 - How should a judgment-based Checker CLI hand off an Assessment to an external agent?

The drafting agent chose a file exchange for the first protocol:

1. Clilint invokes the Checker CLI.
2. The Checker gathers evidence and returns Awaiting Assessment with the
   request binding, Skill, rubric, and evidence.
3. Clilint records that pending work in its report.
4. An external agent follows the Skill and writes one Assessment JSON file.
5. A later Clilint invocation supplies the Assessment to the same Checker CLI.
6. The Checker validates the Assessment and returns a Check Result.

This keeps the protocol model- and harness-independent and preserves the
existing two-pass user workflow. Clilint does not start an agent. “File” is
only the first transport for Assessment; it is not part of the Assessment
domain name.

### 21 - Should the built-in `clilint` checks use the new Checker CLI protocol in this change?

No. The drafting agent chose to keep the built-in checker variants in Rust for
this change. They adopt the shared Check Outcome, Check Result, Score, and
Check Message model. Bundle-owned checks use Checker CLIs.

This makes `codekiln-help` the proof that a bundle can add behavior without a
Clilint binary change. A later change can move built-in checks behind the CLI
boundary after the protocol has been exercised by a concrete bundle.

### 24 - Should Checks or Checker CLIs be called "bundle-owned"?

No. `codekiln` chose **Check** and **Checker CLI**. Every Check belongs to a
check bundle, so “bundle-owned” adds no distinction. When the implementation
matters, the design distinguishes a Checker CLI from a built-in Rust checker.

### 25 - How should Clilint handle Checker output and logs?

`codekiln` questioned the fixed standard-output and retained-log limits before
a concrete need for those limits exists. `codekiln` asked the drafting agent
to compare alternatives such as writing to disk or streaming.

The drafting agent compared three choices:

- collecting both streams in memory is simple, but keeps the raw JSON and the
  parsed Check Outcome in memory at the same time;
- fixed size limits constrain memory, but can reject a valid Check Outcome
  because of an arbitrary limit; and
- writing standard output to a temporary file avoids the extra in-memory copy,
  while letting standard error continue to the user preserves ordinary CLI
  logging.

The drafting agent chose the temporary-file option for the first protocol.
Clilint parses the Check Outcome after the Checker exits. Checker logs continue
directly to Clilint's standard error and are not copied into a Check Error.
Clilint keeps the fixed run-time limit because a process that does not exit
would otherwise stop the check indefinitely.

This answer replaces the output and log limits in answer 19. The protocol can
add streaming or a configurable limit after a concrete Checker demonstrates
the need.

### 26 - Should the first help standard include a search operation?

No. `codekiln` chose command output that callers can pipe to tools such as
`rg`, `grep`, or `jq`. `codekiln` noted that a dedicated search operation could
be useful if it searches the complete command hierarchy, but chose to defer it
until that need is concrete.

The drafting agent had included `help search` to return command paths and
section values. The current standard keeps `help`, `outline`, `section`, and
JSON command discovery.

## Open Questions

None.

## Citations

- [My/Principle/CLI/Centricity/Offline Tools](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___CLI___Centricity___Offline%20Tools.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Principle/Simplify/Minimize Surface Area](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Simplify___Minimize%20Surface%20Area.md)
- [My/Principle/Make the Right Thing Easy and the Wrong Thing Hard](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20the%20Right%20Thing%20Easy%20and%20the%20Wrong%20Thing%20Hard.md)
- [My/Principle/Declarative over Imperative](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Declarative%20over%20Imperative.md)
- [My/Principle/Make Illegal States Unrepresentable](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20Illegal%20States%20Unrepresentable.md)
- [My/Principle/Make it Obvious](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Make%20it%20Obvious.md)
