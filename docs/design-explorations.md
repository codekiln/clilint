# Design explorations

> **Status:** Brainstorming, not a specification or commitment. These notes preserve open questions and candidate designs.

## Future judgment-based Check automation

A judgment-based Checker can return Awaiting Assessment with its Skill,
rubric, and evidence. An external agent writes a JSON Assessment, and a later
Clilint invocation supplies the file to the same Checker. The Checker validates
the Assessment before returning a Check Result.

The [`assess-cli-help` Skill](../skills/assess-cli-help/SKILL.md) demonstrates
this two-pass workflow. Clilint leaves model and agent-harness selection to the
caller.

The current flow is:

```text
person or agent runs clilint check
             |
             v
Checker CLI gathers evidence and returns Awaiting Assessment
             |
             v
external agent applies the Skill and rubric
             |
             v
Checker CLI validates the Assessment and returns a Check Result
```

Future work could let an agent harness automate the file exchange or let a
Checker talk to a provider directly. Any such integration should preserve the
same Check Request, Assessment, and Check Outcome meanings.

## Learning check bundles from reference tools

A future authoring workflow could run safe commands against well-designed reference tools, capture help and behavior, and propose checks. A person would distinguish:

- **observed behavior:** what a reference tool did;
- **inferred expectation:** the pattern an AI believes it expresses; and
- **chosen preference:** an expectation a person accepted into a check bundle.

The workflow should not silently turn every observation into a check. Open questions include how many examples support a shared expectation, how to handle tools that disagree, which commands are safe to exercise, and how a generated bundle preserves the evidence behind a proposed check.

## A name that survives speech and search

The project name is also unsettled. “Clilint” has been transcribed as “Clevent,” so any future name should be tested for speech recognition, pronunciation, spelling, and search collisions. No rename is proposed here.
