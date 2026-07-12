# schemas/

Schemas for the linter's inputs and outputs — the **CLI Lint Report** (findings with stable rule
ids, evidence, severity, score impact, and remediation) and any manifest a target project can
supply to declare its profile or expected behavior. A stable report schema is what lets an agent
consume `--format json` output and iterate without parsing prose.
