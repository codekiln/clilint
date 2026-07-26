# Judgment-based Assessments

A judgment-based Check uses human or model interpretation to determine its
Score or Check Messages. The Checker still owns evidence gathering and result
validation.

## Complete an Assessment

First, save a JSON report:

```sh
clilint check my-cli --format json > clilint-report.json
```

An unfinished judgment-based Check has `"outcome": "awaiting-assessment"`.
Its `assessment_request` contains the request identifier, evidence digest,
Skill, rubric, and evidence needed by an external agent.

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

Supply the file on a later run:

```sh
clilint check my-cli \
  --assessment ./clilint-help-assessment.json \
  --format json
```

Clilint or the bundle-owned Checker validates the request identifier, Check,
Skill, evidence digest, Score, and Check Messages. A change to the captured
evidence makes the older Assessment invalid.

Clilint does not choose or start an AI model. Any agent harness that can read
the request and write the JSON Assessment can complete the Check.

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
