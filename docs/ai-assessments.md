# Judgment-based Assessments

A judgment-based Check uses human or model interpretation to determine its
Score or Check Messages.

## Complete a Check with an agent

First, save a report:

```sh
clilint check my-cli --format json > clilint-report.json
```

An unfinished judgment-based Check has `"outcome": "awaiting-assessment"`.
The report contains the evidence and rubric that an agent needs, and Clilint
exits with code 1 until the Check has a completed result.

Give the report to an agent that has the
[`assess-cli-help`](../skills/assess-cli-help/SKILL.md) Skill. For example:

> Follow the assess-cli-help Skill to complete the Awaiting Assessment in
> clilint-report.json. Write clilint-help-assessment.json.

The agent reads the captured evidence, applies the rubric, and writes an
Assessment file. Supply that file on a later run:

```sh
clilint check my-cli \
  --assessment ./clilint-help-assessment.json \
  --format json
```

The Checker gathers the evidence again and verifies that the Assessment still
matches it. Clilint then validates the Score and Check Messages and adds the
Check Result to the report. If the evidence changed, Clilint reports that a
new Assessment is needed.

## Give an agent the Assessment Skill

The bundled `assess-cli-help` Skill tells an agent how to score captured help
and write the Assessment file. From a Clilint repository checkout, this
command installs the Skill for Codex:

```sh
skills add . --skill assess-cli-help --agent codex --copy -y
```

Other tools that support Agent Skills can load the same
[Skill source](../skills/assess-cli-help/SKILL.md).

The Skill reads only the evidence captured by Clilint. It does not execute
example commands found in help text.

## Assessment file

An Assessment file has this shape:

```json
{
  "format_version": 1,
  "request_id": "sha256:...",
  "check": "clilint/help/useful-example",
  "evidence_digest": "sha256:...",
  "skill": {
    "name": "assess-cli-help",
    "version": "1.0.0"
  },
  "score": 3.5,
  "messages": [
    {
      "level": "warning",
      "message": "Explain what the example command changes.",
      "evidence": {"example": "contacts add Ada"}
    }
  ],
  "explanation": "The example is usable, but its result is unclear.",
  "assessor": "optional agent or model name"
}
```

The agent copies these fields from `assessment_request`:

- `request_id` links the Assessment to this Check request.
- `evidence_digest` changes when the captured evidence changes.
- `check` identifies the Check being scored.
- `skill` identifies the Skill and version used for the Assessment.

The [working Assessment fixture](../tests/fixtures/useful-help-assessment.json)
provides a complete example.
