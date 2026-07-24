# Design explorations

> **Status:** Brainstorming, not a specification or commitment. These notes preserve open questions and candidate designs.

## AI-assessed checks without required agent skills

A check is one testable expectation. A deterministic checker computes its result from captured evidence. A judgment-based check asks an AI agent to apply a written rubric to captured evidence.

The current prototype assigns one Agent Skill to each judgment-based check. The [`assess-cli-help` skill](../skills/assess-cli-help/SKILL.md) runs Clilint, judges captured help, writes an assessment document, and asks Clilint to validate and attach it. This proves that evidence can be bound to a judgment, but it is not necessarily the intended long-term design.

A well-designed command-line tool should explain itself well enough that a capable agent can use it without installing a tool-specific skill. Help, structured output, and errors should expose the complete workflow. A skill may offer convenience, but it should not contain essential instructions that the tool withholds.

One possible flow is:

```text
person or agent runs clilint check
             |
             v
Clilint captures evidence and deterministic results
             |
             v
agent receives question + criteria + evidence + response schema
             |
             v
Clilint validates and attaches the structured assessment
```

The exact commands and formats remain unsettled. An assessment job could contain the question, relevant observations, criteria for every result, safety limits, a criteria version, and the response schema.

Candidate operating models include an external agent following a self-describing CLI exchange, Clilint invoking an AI provider, one skill per check, or one generic Clilint assessment skill. The first avoids provider credentials; the second makes one-command CI easier; the skill models add convenience but risk hiding essential behavior outside the CLI.

Open questions include how an agent discovers pending assessments, whether jobs are files or transient output, which tested CLI tool commands a judgment check may request, how disagreement is represented, and how reports identify the model, rubric, and check-bundle versions.

## Learning check bundles from reference tools

A future authoring workflow could run safe commands against well-designed reference tools, capture help and behavior, and propose checks. A person would distinguish:

- **observed behavior:** what a reference tool did;
- **inferred expectation:** the pattern an AI believes it expresses; and
- **chosen preference:** an expectation a person accepted into a check bundle.

The workflow should not silently turn every observation into a check. Open questions include how many examples support a shared expectation, how to handle tools that disagree, which commands are safe to exercise, and how a generated bundle preserves the evidence behind a proposed check.

## A name that survives speech and search

The project name is also unsettled. “Clilint” has been transcribed as “Clevent,” so any future name should be tested for speech recognition, pronunciation, spelling, and search collisions. No rename is proposed here.
