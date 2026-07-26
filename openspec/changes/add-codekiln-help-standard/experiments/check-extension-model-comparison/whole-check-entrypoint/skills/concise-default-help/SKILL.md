---
name: concise-default-help
description: Gather a CLI program's default and full help, assess the default help against the supplied rubric, and return the required Clilint assessment.
---

# Check concise default help

The check request supplies the target command, rubric, and assessment contract.

## Lifecycle

1. Perform any setup required by the check. This check requires none.
2. Run the bundled mechanistic entry point with the check request:

   ```sh
   python3 ../../check.py < check-request.json > mechanistic-assessment.json
   ```

3. Read its evidence and mechanistic findings.
4. Use the supplied rubric to judge whether the description and example are
   useful and concise.
5. Return one assessment that follows the supplied contract. Include every
   mechanistic and LLM finding and cite the gathered evidence.

The skill owns the complete lifecycle. Clilint sees only the initial request and
the final assessment.
