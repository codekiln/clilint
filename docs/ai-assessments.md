# AI assessments

Most Clilint rules use repeatable checks such as exit codes, output text, and response time. A rule can instead request an AI agent when the question requires judgment.

Clilint keeps these results separate. A requested AI assessment appears as `unassessed` until the matching skill reviews the captured evidence and produces an assessment document.

## Install the help assessment skill

From a Clilint repository checkout, list the available skills:

```sh
skills add . --list
```

Install `assess-cli-help` for your agent. For example, for Codex:

```sh
skills add . --skill assess-cli-help --agent codex -y --copy
```

The skill runs Clilint, reviews only the captured `--help` output, writes an assessment document, and asks Clilint to validate and attach it. The skill does not run examples found in the target's help text.

## Attach an existing assessment

Pass one or more TOML or JSON assessment documents with `--assessment`:

```sh
clilint check my-cli \
  --assessment ./clilint-help-assessment.toml \
  --format json
```

Clilint checks the document format, rule identifier, result, skill name and version, and evidence digest. The digest binds the assessment to the captured evidence. If the target's evidence changes, Clilint rejects the older assessment instead of attaching it to the new report.

The skill source is [`skills/assess-cli-help/SKILL.md`](../skills/assess-cli-help/SKILL.md). A working assessment fixture is available at [`tests/fixtures/useful-help-assessment.toml`](../tests/fixtures/useful-help-assessment.toml).
