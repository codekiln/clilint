## Why

The Python prototype demonstrated that behavioral CLI checks can produce useful evidence, but its dynamically loaded probes, separate rule files, and many top-level folders make the relationship between the standard and its implementation difficult to understand. The next version should establish the smaller Rust and package-based foundation that future deterministic and AI-agent checks can extend.

## What Changes

- **BREAKING**: Replace the Python command and runtime with a Rust CLI.
- Replace the separate built-in `probes/` modules with typed Rust checks.
- Introduce a package format used by both the bundled global standard and user-authored extensions.
- Let extension packages add rules or strengthen the global standard without silently weakening it.
- Keep package execution offline after packages have been obtained.
- Move test fixtures under the Rust test structure and remove superseded top-level folders during implementation.
- Add a top-level `skills/` directory whose skills can be installed with the Vercel Skills CLI.
- Ship `assess-cli-help`, an Agent Skill that judges whether help teaches a likely task with useful examples.
- Represent deterministic results and AI-agent assessments separately in reports, while allowing a validated AI assessment to be attached to a report.
- Use mise to manage the Rust toolchain, development tools, environment settings, and executable file tasks shared by local development and CI.
- Keep OpenSpec skills as RuleSync source and require agents to load the matching skill before editing change artifacts.

## Capabilities

### New Capabilities

- `conformance-packages`: Defines the bundled standard package, additive user extensions, rule identity, validation, and local package loading.
- `deterministic-checking`: Exercises a target executable and evaluates typed, repeatable checks with captured evidence.
- `agent-assessments`: Exposes evidence for AI-agent rules and accepts validated assessments produced by an installed Agent Skill.
- `conformance-reporting`: Reports deterministic findings, pending AI-agent work, completed AI-agent assessments, and their evidence without presenting probabilistic results as repeatable measurements.
- `project-workflow`: Defines mise as the project tool and task interface and RuleSync as the source for OpenSpec agent skills.

### Modified Capabilities

None. The OpenSpec source of truth is new; the Python prototype is historical input rather than an existing OpenSpec capability.

## Impact

- Replaces the Python packaging and source tree with a Rust crate.
- Reorganizes `rules/`, `probes/`, `schemas/`, `prompts/`, `fixtures/`, `integrations/`, and `spec/` after their useful content is accounted for.
- Adds a public package manifest and report format that will require compatibility care after release.
- Adds an Agent Skill that agents can install directly from this repository.
- Adds `mise.toml`, executable `.mise/tasks/`, RuleSync source, and generated project-local agent configuration.

## Citations

- [My/Principle/Simplify/Fewer and Deeper](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Fewer%20and%20Deeper)
- [My/Principle/Simplify/Minimize Surface Area](logseq://graph/logseq-encode-garden?page=My/Principle/Simplify/Minimize%20Surface%20Area)
- [My/Principle/Make Illegal States Unrepresentable](logseq://graph/logseq-encode-garden?page=My/Principle/Make%20Illegal%20States%20Unrepresentable)
- [My/Pref/Dev/mise](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise)
- [My/Pref/Dev/mise/Tasks](logseq://graph/logseq-encode-garden?page=My/Pref/Dev/mise/Tasks)
