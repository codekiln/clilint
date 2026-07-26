# Findings from the check extension model comparison

## Requirements exercised

Both prototypes let a bundle provide:

- imperative evidence gathering;
- either mechanistic or agent judgment;
- one shared Check Outcome, Check Result, and Check Message structure; and
- a new check without a check-specific change to the Clilint binary.

Both can support optional setup. This check does not need setup, so neither
prototype declares it.

## One entry point

One entry point is the smaller host interface. Clilint sends a check request and
receives a Check Outcome. A command or Agent Skill can perform setup, gather
evidence, and produce the outcome internally.

This model has two costs:

1. Clilint cannot retain or validate the evidence before judgment unless the
   entry point includes it in the final response.
2. The lifecycle phases are conventions inside each entry point rather than
   boundaries shared by all check bundles.

The model is still sufficient for a plugin system. A bundle can add arbitrary
mechanistic or agent-driven behavior as long as the entry point follows the
request and outcome protocol.

## Phase-oriented entry points

The phase-oriented prototype adds a manifest-level boundary after evidence
gathering. Clilint receives a versioned evidence document and passes it to
either a command for mechanistic scoring or an Agent Skill for judgment-based
assessment.

This model:

- represents the complete setup, evidence, and scoring lifecycle as explicit
  phases;
- lets mechanistic scoring and judgment-based assessment consume the same
  evidence shape;
- lets Clilint preserve evidence and verify evidence references;
- lets a future check add setup without changing the other phases; and
- keeps each extension outside the Clilint process.

Its cost is a larger protocol. Clilint must define phase names, runner types,
and the documents passed between phases.

## Drafting agent recommendation

Use the phase-oriented model as the first extension system:

1. An installed bundle contains one directory per check, including a manifest
   and any scripts, skills, rubric, references, or assets the check needs.
2. The manifest lists an optional `setup` phase, one evidence-gathering phase,
   and one scoring phase.
3. Each phase selects a runner supported by Clilint. The first runners are an
   out-of-process command and an external agent skill.
4. Command phases exchange versioned JSON documents with Clilint. Agent phases
   receive the same input document, a rubric, and the required output contract.
5. Every scoring phase returns the shared Check Outcome structure.
6. A new check adds bundle files. The Clilint binary changes only when it gains
   a runner or changes a shared protocol.

This is the drafting agent's recommendation. `codekiln` chose the whole-check
entry point for the first extension protocol.

## Project-owner decision

Begin with one bundle-owned entry point for the complete check. The entry point
owns any setup, evidence gathering, and production of the Check Outcome.
Clilint standardizes the request and outcome boundaries.

This choice keeps phase state within one check run and leaves the first
extension protocol smaller. A later change can expose phases when a concrete
check needs Clilint to retain intermediate evidence, repeat scoring, or
manage one phase independently.

`codekiln` later chose Check Result for the shared completed output and Check
Message for its structured feedback. Assessment is reserved for a
judgment-based check's record of applying a rubric to evidence. The drafting
agent's earlier use of Assessment and Finding as shared terms is superseded.

`codekiln` then narrowed the first bundle-owned entry point to a Checker CLI.
Agent Skills and other tools sit behind that CLI instead of becoming peer
Clilint checker implementations. The
[Checker CLI contract comparison](../checker-cli-contract-comparison/README.md)
tests that later decision.

## Hooks

The prototype uses lifecycle phases directly. It does not add `pre-*`,
`post-*`, or installation hooks. A later experiment can add a hook when a
concrete check needs behavior that the three phases cannot express clearly.
