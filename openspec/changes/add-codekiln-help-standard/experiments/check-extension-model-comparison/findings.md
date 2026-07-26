# Findings from the check extension model comparison

## Requirements exercised

Both prototypes let a bundle provide:

- imperative evidence gathering;
- either mechanistic or agent judgment;
- one shared assessment and finding structure; and
- a new check without a check-specific change to the Clilint binary.

Both can support optional setup. This check does not need setup, so neither
prototype declares it.

## One entry point

One entry point is the smaller host interface. Clilint sends a check request and
receives an assessment. A command or Agent Skill can perform setup, gather
evidence, and assess the check internally.

This model has two costs:

1. Clilint cannot retain or validate the evidence before judgment unless the
   entry point includes it in the final response.
2. The lifecycle phases are conventions inside each entry point rather than
   boundaries shared by all check bundles.

The model is still sufficient for a plugin system. A bundle can add arbitrary
mechanistic or agent-driven behavior as long as the entry point follows the
request and assessment protocol.

## Phase-oriented entry points

The phase-oriented prototype adds a manifest-level boundary after evidence
gathering. Clilint receives a versioned evidence document and passes it to
either a command or an agent skill for assessment.

This model:

- matches codekiln's setup, evidence, and assessment lifecycle;
- lets mechanistic and agent assessment consume the same evidence shape;
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
   and one assessment phase.
3. Each phase selects a runner supported by Clilint. The first runners are an
   out-of-process command and an external agent skill.
4. Command phases exchange versioned JSON documents with Clilint. Agent phases
   receive the same input document, a rubric, and the required output contract.
5. Every assessment returns the shared rating and finding structure.
6. A new check adds bundle files. The Clilint binary changes only when it gains
   a runner or changes a shared protocol.

This is the drafting agent's recommendation. `codekiln` has not accepted it.

## Hooks

The prototype uses lifecycle phases directly. It does not add `pre-*`,
`post-*`, or installation hooks. A later experiment can add a hook when a
concrete check needs behavior that the three phases cannot express clearly.
