# Design explorations

> **Status:** Brainstorming, not a specification or commitment. These notes preserve open questions and candidate designs so that future work does not mistake an early idea for a settled decision. Any part may change or be discarded.

## AI-judged checks without required agent skills

A judgment-based check is a check whose result depends on interpreting captured evidence against written criteria. Unlike a repeatable check for an exit code or exact text, it may need an AI agent to decide whether the evidence passes.

The current prototype assigns one Agent Skill to each judgment-based rule. For example, the [`assess-cli-help` skill](../skills/assess-cli-help/SKILL.md) tells an agent to run Clilint, judge the captured help, write an assessment document, and run Clilint again to validate and attach the result. This proves that evidence can be bound to a judgment, but it is not necessarily the intended long-term design.

### Design principle under consideration

A well-designed command-line tool should explain itself well enough that a capable agent can use it without installing a tool-specific Agent Skill. `--help`, structured output, and errors should expose the complete workflow. A skill may offer convenience, but it should not contain essential instructions that the command-line tool withholds.

An assessment job would give an agent the question, criteria, captured evidence, and required response format together. This suggests a division of responsibility:

- a package author states the question, the evidence to collect, and the criteria for each result in a reusable collection of rules;
- Clilint runs the target only in the ways declared by the package, captures evidence, and emits a self-contained assessment job;
- an agent runs Clilint, judges that evidence, and returns a structured result; and
- Clilint validates the rule identity, criteria version, evidence digest, result, explanation, and assessor information before including the result in its report.

One possible flow is:

```text
agent or person
      |
      |  clilint check <target>
      v
Clilint runs declared commands and reports repeatable results
      |
      |  self-contained assessment job:
      |  question + criteria + evidence + response schema
      v
agent judges captured evidence without inventing target commands
      |
      |  structured assessment response
      v
Clilint validates and attaches the response
```

The exact commands and file formats are unsettled. Clilint might print the next command to run, expose a dedicated `assessment` subcommand, or support a machine-oriented mode that guides an agent through the exchange. Whatever interface emerges should be discoverable from the CLI itself.

### How a judgment-based rule might be expressed

A package may eventually need to carry more than a skill name. A self-contained rule could include:

- the question to answer;
- which Clilint-captured observations are relevant;
- plain-language criteria for `pass`, `warn`, `fail`, and `skip`;
- safety limits on what may be executed or followed;
- a version or digest for the criteria; and
- the required response schema.

This is a conceptual list, not a proposed package format. An open question is how much freedom package authors need without turning each rule into an opaque prompt that is difficult to compare, secure, or reproduce.

### Candidate operating models

Several models remain plausible:

1. **An external agent follows a self-describing CLI exchange.** Clilint does not need model credentials or provider integrations, and no Agent Skill is required. This most directly supports the design principle above, but the steps and recovery behavior need careful design.
2. **Clilint invokes an AI provider.** One command could run the complete check in CI, but Clilint would take on credentials, cost controls, provider differences, model selection, and network failure handling.
3. **Each rule has its own Agent Skill.** Rules can have specialized workflows, but users must install and trust many skills, and essential behavior can drift away from the CLI.
4. **One generic Clilint skill drives all judgment-based checks.** This reduces duplication, but the skill still becomes required knowledge unless every step it performs is independently discoverable through Clilint.

A possible direction is to treat the first model as the baseline, allow the second through optional runners, and permit skills only as convenience wrappers. That direction has not been chosen.

### Open questions

- Should one Clilint invocation complete the whole exchange, or should evidence collection and assessment attachment remain separate, inspectable steps?
- Should assessment jobs be transient output, files that can be reviewed, or both?
- How does an agent discover all pending assessments and recover after an interrupted run?
- Can one set of criteria evaluate several related rules while recording which judgment produced each result?
- Which target commands may a judgment-based rule request, and who approves commands with side effects?
- How should Clilint identify the model, agent, rubric, and package versions that contributed to a result?
- What should happen when two agents disagree about the same evidence?
- How can the exchange remain useful in local terminals, automated CI, and agent harnesses without favoring one provider?

## A name that survives speech and search

The project name is also unsettled. In voice dictation, “Clilint” has already been transcribed as “Clevent.” That is evidence that the current name may be hard to infer from speech without prior context.

A candidate name should be evaluated for whether it:

- is transcribed reliably when spoken to an AI that has no project-specific training;
- is easy to pronounce, hear, and spell back;
- is distinctive enough to find in a web or package search;
- suggests what the tool does without requiring an explanation; and
- works as a command name, package namespace, repository name, and conversational reference.

No rename is proposed here. A useful naming investigation would test candidates through several transcription systems and realistic sentences, then check search and package-name collisions. The result should be evidence for a later decision, not a reason to defend the current name.
