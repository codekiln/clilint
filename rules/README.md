# rules/

Machine-readable rule definitions — one entry per `CLI-` rule. Each carries its identifier,
category, severity, score impact, the profiles it applies to, and a pointer to the probe that
checks it and the remediation text. The reference linter in [`src/`](../src/) loads these; the prose
rationale lives in [`spec/`](../spec/).
