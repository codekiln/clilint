## Context

The current implementation stores policy in JSON rules and executable behavior in dynamically loaded Python modules. A hidden path option can replace the probe directory, but neither the rule data nor Python type checking ensures that a named probe exists or returns the expected finding. The repository also gives policy, implementation, prompts, schemas, fixtures, and integrations separate top-level folders.

The rebuild must preserve the useful behavioral-checking idea while establishing a Rust CLI, a package format shared by the global standard and user extensions, and one working Agent Skill assessment.

## Goals / Non-Goals

**Goals:**

- Build a single Rust binary with a small, understandable module structure.
- Make built-in checks typed Rust behavior rather than filesystem plug-ins.
- Use one package model for the global standard and additive user preferences.
- Provide repeatable deterministic evidence and traceable AI-agent assessments.
- Ship one complete skill-based assessment in 0.0.2.
- Give contributors and CI the same mise-managed tools and task names.
- Make the OpenSpec skill source and generation process explicit in the repository.

**Non-Goals:**

- A network package registry.
- Arbitrary native or interpreted code in user packages.
- Direct calls from clilint to an AI provider.
- A single score that hides the difference between deterministic and AI-agent results.
- Reproducing every Python prototype feature before validating the new package model.

## Decisions

### Use a single Rust crate

The first Rust release will be one binary crate. Likely direct dependencies are `clap` for argument parsing, `serde` plus `toml` and `serde_json` for package and report data, and a focused error crate if standard errors become difficult to read. Target execution can use the Rust standard library.

The intended high-level shape is:

```text
Cargo.toml
src/
packages/clilint/
skills/assess-cli-help/
tests/fixtures/
openspec/
```

Alternative considered: a Cargo workspace with separate engine, format, and CLI crates. That would create boundaries before independent reuse has been demonstrated.

### Use mise as the project command interface

The root `mise.toml` will manage Rust and project development tools. A committed `mise.lock` will pin resolved versions. Repeated jobs will be executable files under `.mise/tasks/`, grouped by task name. Tasks with inputs will use `#USAGE` metadata, and CI will install tools with the mise GitHub Action before invoking the same task names used locally.

The initial task set will include formatting, linting, tests, OpenSpec validation, RuleSync generation checks, and an aggregate CI task. Release-plz and dist will join the same project tool configuration in the release change.

Alternative considered: call Cargo, OpenSpec, RuleSync, release-plz, and dist directly from each workflow. That would duplicate tool setup and command sequences between local development and CI.

### Keep OpenSpec skills in RuleSync source

OpenSpec workflow skills will be committed under `.rulesync/skills/`. RuleSync will generate `.agents/skills/` and `.claude/skills/`; generated copies will not be edited by hand. A short root rule will require agents to load the matching skill and query the current OpenSpec artifact instructions before writing. A mise workflow check will compare the source skills' `generatedBy` metadata with the project-managed OpenSpec CLI version so a tool update cannot silently leave older instructions in place.

Alternative considered: rely on a globally installed skill or on an agent's memory of OpenSpec. That would make valid planning depend on machine state or stale instructions rather than the project checkout.

### Treat the global standard as a package

`packages/clilint/clilint.toml` will use the same manifest and rule representation accepted from a local extension path. The binary will bundle this package so the default check does not depend on repository-relative files.

Extension resolution is additive. Package-scoped rule identifiers prevent accidental collisions. An extension can add rules and make a requirement stricter through an explicit supported operation, but it cannot exclude or redefine a global rule.

Alternative considered: separate formats for built-in rules and user preferences. Two formats would make the global standard a special case and increase the amount users and maintainers must learn.

### Use a closed set of deterministic check types

Package data will select from a tagged Rust enum of supported checks and assertions. Initial observations include exit status, timeout, duration, stdout, stderr, and ANSI escape presence. Package loading validates the complete check before any target process starts.

Alternative considered: load external probe programs from a package. Executable extensions would make packages less portable and would run untrusted code with the user's permissions. A process protocol or WebAssembly boundary can be considered after declarative checks prove insufficient.

### Keep one main check workflow

`clilint check` will run deterministic checks and collect evidence required by applicable AI-agent rules. JSON output will mark those rules as unassessed and name the skill that can assess them. A repeatable `--assessment <path>` option will load assessment documents and produce the combined report.

Each assessment document will contain a format version, rule identifier, result, explanation, skill name and version, evidence digest, and optional assessor metadata. Clilint will recompute the evidence digest and reject stale assessments.

Alternative considered: add separate `evidence`, `assess`, and `report` subcommands. That makes internal phases part of the public CLI before the workflow is understood.

### Ship `assess-cli-help` as the first vertical slice

The skill will be stored at `skills/assess-cli-help/SKILL.md`, which is a standard discovery location for the Vercel Skills CLI. It will run clilint, read the pending help-quality task, judge whether the examples teach a likely task, write an assessment document, and ask clilint to validate and attach it.

The skill will not execute commands copied from help output. Its first fixtures will include help with a useful task example and help that contains an example heading without useful instruction.

Alternative considered: begin with error-message quality. Helpful errors are also suitable for AI assessment, but generating representative failures safely requires more target-specific exploration than reading help.

### Separate result methods and summaries

Every finding will carry an evaluation method. Deterministic findings contribute to a deterministic score. AI-agent findings have their own result counts and provenance. The report will not calculate a combined score in 0.0.2.

Alternative considered: preserve the prototype's one-number score. A combined score would give probabilistic classifications the appearance of repeatability and make changes in an agent or model look like changes in CLI behavior.

## Risks / Trade-offs

- [The initial check enum may be too limited] → Port the current probes as data-driven cases first, then add a check type only when a real rule requires it.
- [Package rules may expose internal implementation details] → Specify checks in terms of observable target behavior and version the package format independently.
- [AI agents may classify the same evidence differently] → Record provenance and explanation, keep AI results separate, and test the skill against contrasting fixtures.
- [Assessment files could be attached to changed evidence] → Bind each assessment to a digest of the evidence clilint produced.
- [Bundled and source copies of the global package could disagree] → Generate the bundled representation from the checked-in package during the Rust build and test that it parses.
- [Generated agent files become stale] → Add a mise task that runs RuleSync's generation check and run it in CI.
- [RuleSync source skills become older than the OpenSpec CLI] → Check every source skill's `generatedBy` version against the mise-managed CLI version.
- [A broad tool version range resolves differently over time] → Commit `mise.lock` and review tool updates as source changes.

## Migration Plan

1. Record and tag the Python prototype as 0.0.1 after aligning its displayed version.
2. Set up the mise-managed Rust and development environment, file tasks, and shared CI task interface.
3. Establish the Rust crate and package/report types on a rebuild branch.
4. Re-express the useful prototype rules as the bundled package and typed checks.
5. Add Rust unit, integration, and fixture tests before removing Python files.
6. Implement the pending-assessment and attachment path.
7. Create and forward-test `assess-cli-help` against the two contrasting fixtures.
8. Account for useful text in the existing `spec/` and other top-level folders, then remove superseded files.
9. Release the accepted rebuild as 0.0.2 through the automated release path.

The 0.0.1 tag preserves the previous implementation, so rollback means checking out that tag rather than keeping two runtimes in the primary branch.

## Open Questions

- Which package fields need independent version numbers for the package format, global standard, and clilint binary?
- Should a local extension package be selected only by command-line option in 0.0.2, or may a project configuration file select it?
- Which optional assessor metadata can agents report reliably across supported coding tools?

## References

- [Command Line Interface Guidelines](https://github.com/cli-guidelines/cli-guidelines/blob/main/content/_index.md)
- [Vercel Skills CLI](https://github.com/vercel-labs/skills)

## Citations

- [My/Principle/Simplify/Fewer and Deeper](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Fewer%20and%20Deeper)
- [My/Principle/Simplify/Minimize Surface Area](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Minimize%20Surface%20Area)
- [My/Principle/Make Illegal States Unrepresentable](logseq://graph/logseq-encode-garden?page=My/Principle/Make%20Illegal%20States%20Unrepresentable)
- [My/Pref/Dev/mise](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise)
- [My/Pref/Dev/mise/Tasks](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise/Tasks)
