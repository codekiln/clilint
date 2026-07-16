---
root: true
targets: ["*"]
description: "codekiln/clilint project instructions"
---

## OpenSpec

Use OpenSpec for planned changes. Before creating or editing OpenSpec artifacts, load the
matching OpenSpec skill from `.rulesync/skills/` and follow the artifact instructions
returned by the OpenSpec CLI. Do not invent the document format from memory.

When archiving an OpenSpec change, always run the spec-sync workflow before moving the
change into the archive. Do not offer archive without syncing as a routine option.

Project-level AI configuration is generated from `.rulesync/`. Edit the RuleSync source,
then run `rulesync generate`; do not hand-edit generated tool files.

## Communication

Before writing or revising repository documentation or other project prose, read and
apply `.rulesync/rules/communication-style.md`. It defines plain language,
reader-centered information order, progressive disclosure for human and agent readers,
and the language to remove.

After changing `README.md`, run both `assess-readme-style` and
`assess-readme-purpose`. Resolve every reported problem or record the accepted
exception and its reason before considering the README change complete.

## mise

Use mise to manage project tools, environment settings, and tasks. Prefer executable file
tasks under `.mise/tasks/` to inline TOML tasks. Give each file task a `#MISE description`
and use `#USAGE` metadata for arguments and flags.
