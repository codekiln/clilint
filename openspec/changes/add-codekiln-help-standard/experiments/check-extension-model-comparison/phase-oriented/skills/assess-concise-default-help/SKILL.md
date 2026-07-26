---
name: assess-concise-default-help
description: Assess already-gathered default CLI help against the supplied concise-help rubric and return the required Clilint assessment.
---

# Assess concise default help

Clilint supplies a versioned evidence document, the rubric, and the assessment
contract. Evidence gathering is complete before this skill starts.

## Assessment

1. Verify that the evidence contains observations for the target with no
   arguments and with `--help`.
2. Run the bundled mechanistic assessment with the evidence:

   ```sh
   python3 ../../assess.py < evidence.json > mechanistic-assessment.json
   ```

3. Preserve its mechanistic findings.
4. Judge whether the default description and examples are useful and concise.
5. Add a warning or error finding for every meaningful gap.
6. Return one assessment that follows the supplied contract and cites only the
   supplied evidence.

This skill performs the assessment phase only. Clilint retains the evidence and
can verify every evidence reference in the returned findings.
