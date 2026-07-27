# Draft assessment contract

Every mechanistic command and agent skill in this experiment returns the same
JSON object.

Required top-level fields:

- `format_version`
- `check`
- `rating`
- `evidence`
- `findings`

Each finding contains:

- `severity`: `warning` or `error`;
- `expectation`: the part of the check that is not met;
- `observed`: what the evidence shows;
- `evidence_refs`: references into the evidence document; and
- `evaluation_method`: `mechanistic` or `llm`.

The exact production contract remains a design question. These fields are the
smallest concrete draft used to compare plugin models.
