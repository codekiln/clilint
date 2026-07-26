# Judgment-based Assessments

A judgment-based Check uses human or model interpretation to determine its
Score or Check Messages.

## Complete an Assessment

First, save a JSON report:

```sh
clilint check my-cli --format json > clilint-report.json
```

An unfinished judgment-based Check has `"outcome": "awaiting-assessment"`.
Its `assessment_request` contains the Check, Skill, rubric, and evidence needed
by an external agent.

The agent writes one Assessment JSON file:

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

Copy `request_id` and `evidence_digest` from `assessment_request`. The Checker
uses them to reject an Assessment made for a different Check request or older
evidence.

Supply the file on a later run:

```sh
clilint check my-cli \
  --assessment ./clilint-help-assessment.json \
  --format json
```

The Checker verifies that the Assessment uses the current Check, evidence, and
Skill. Clilint validates the Score and Check Messages before adding the Check
Result to the report. If the Checker gathers different evidence on the later
run, it rejects the older Assessment.

Any agent that can read `assessment_request` and write the JSON Assessment can
complete the Check.

## Install the help-assessment Skill

From a Clilint repository checkout:

```sh
skills add . --list
skills add . --skill assess-cli-help --agent codex -y --copy
```

The Skill reads only the evidence captured by Clilint. It does not execute
example commands found in help text.

See the [Skill source](../skills/assess-cli-help/SKILL.md) and the
[working Assessment fixture](../tests/fixtures/useful-help-assessment.json).
