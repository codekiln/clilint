---
root: true
targets: ["*"]
description: "codekiln/clilint project instructions"
---

## OpenSpec

Use OpenSpec for planned changes. Before creating or editing OpenSpec artifacts, load the
matching OpenSpec skill from `.rulesync/skills/` and follow the artifact instructions
returned by the OpenSpec CLI. Do not invent the document format from memory.

Project-level AI configuration is generated from `.rulesync/`. Edit the RuleSync source,
then run `rulesync generate`; do not hand-edit generated tool files.

## mise

Use mise to manage project tools, environment settings, and tasks. Prefer executable file
tasks under `.mise/tasks/` to inline TOML tasks. Give each file task a `#MISE description`
and use `#USAGE` metadata for arguments and flags.
