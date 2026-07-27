# Brainstorm: reframing the extension architecture for judgment-based checks

Record of an explore session held 2026-07-25 on open question 16 of the
`add-codekiln-help-standard` change. The session did not answer question 16. It
concluded that the question, as written, presumes a mechanical answer and should
be restated before anyone answers it.

Question 16 remains open in `design.md`. This file exists so a later session can
resume without repeating the discussion.

## How to read the attribution

Two voices appear in this file, and they are labeled every time:

- **codekiln** — the project owner's positions, corrections, and pasted material.
  Treat these as the direction to follow.
- **Agent** — the assistant's analysis. codekiln accepted some of it, corrected
  some, and has not yet responded to the rest. Where a position was corrected,
  the correction appears directly beneath it.

Where codekiln pasted material from earlier ChatGPT conversations, the
instruction was explicit: do not assume it is a 100% valid representation of
those opinions. It is directional. Those passages are marked as pasted material
rather than as codekiln's settled position.

## 1. The question as it was handed over

`design.md` open question 16 reads:

> How should a check bundle express new behavior without a change to the Clilint
> binary?

A previous drafting agent attached this context to it:

> `codekiln-help` requires one. Each of its checks names
> `type = "hierarchical-help"` and one `behavior` value from a fixed set, so the
> bundle file states which behaviors to evaluate while `src/help_checker.rs`
> decides what each behavior means.
>
> The declarative vocabulary is missing four things that `codekiln-help` needs:
> reading values out of a command's JSON output, iterating over the values it
> read, substituting them into later invocations, and repeating that on
> discovered child commands.

That framing, including the four missing capabilities, came from a previous
agent. It is not codekiln's framing, and by the end of this session the agent
concluded it is the wrong framing. See section 5.

## 2. What the agent explored first, and how codekiln corrected it

### Agent's first pass

The agent tested the four-capability hypothesis against the `codekiln-help`
checks and against invented checks unrelated to help. It concluded the four
capabilities were stated too narrowly, proposed a more general set, and proposed
that a check should be expressed as a named sequence of steps, each combining an
invocation, value extraction, and assertions. It argued for data over code on
two grounds it labeled "the trust constraint" and "the Rust-interface
constraint."

### codekiln's corrections

**The writing was not readable.** codekiln could not follow the summary
paragraph at all, and named specific terms that carried no meaning: "budget,"
"curve-fitting," "bind a value into a later invocation's arguments," "reused
help results," and the function name `collect_unknown_section`. The request was
to say plainly what each meant, alongside this: "I don't have the active memory
to hold everything in the repo and don't assume I've read everything in the
repository at least once."

The agent's clarifications, recorded here so they do not have to be re-derived:

- "Budget" was the agent's invented word for the limits already in the code
  (`HelpLimits` in `src/check_bundle.rs`): at most 64 discovered commands, depth
  8, 1 MB captured per command, 256 search results, 1024 total commands, 2 second
  timeout per command.
- "Reused help results" is a requirement heading in the
  `hierarchical-help-checking` delta spec. Concretely: several `codekiln-help`
  checks need the output of the same command, such as
  `mytool help --format json`. Rather than run it once per check,
  `src/help_checker.rs` runs it once and shares the output.
- "Bind a value into a later invocation's arguments" means: run
  `mytool help outline --format json`, receive
  `{"headings": [{"section": "usage"}, ...]}`, then run
  `mytool help section usage`. The word `usage` came out of the first command's
  output. Today a check can only run commands whose arguments were written down
  in advance.
- "Curve-fitting" meant: the four capabilities may have been derived by looking
  only at `codekiln-help`, so they might fit those checks and nothing else.
- `collect_unknown_section` is a function in `src/help_checker.rs`. It runs
  `mytool help section clilint-unknown-section-7f3d` and checks that the error
  message mentions the section name it passed in.

**The agent argued from deferred material.** Answer 17 removed the `view`
operation from this change, and the agent used the `web-view` check as evidence
anyway. codekiln flagged it: the previous agent had said it factored `view` out
into a dedicated GitHub issue, and this discussion should not relitigate
deferred scope. The code still contains `view` only because task 6.4 has not run
yet.

**The two "constraints" were unattributed.** codekiln asked what they were. They
came from the message that opened the session, not from the agent's own reading:

- "Prefer typed Rust interfaces over dynamically loaded implementation files."
  The agent traced this to the archived `rebuild-rust-core` design, which records
  that the pre-Rust version kept rule data in JSON and behavior in Python files
  loaded at runtime, and that nothing verified a named Python function existed or
  returned the right thing until it ran.
- "A bundle installed from someone else's repository is input you did not write."
  codekiln guessed this referred to lockfiles and SHA pinning. It did not. The
  point was narrower: if an installed bundle is a data file, Clilint can read it
  and report what commands it will run before running any; if it is a program,
  you have to run it to find out.

**Backwards compatibility is not a consideration.** codekiln: "This repo is in
very early alpha. We may wipe away the code and rebuild at any point. Try to
keep in mind that it's more important that I land on the right architectural
model than that I maintain anything resembling backwards compatibility. That's
why we're in explore mode."

This also reopens the Rust-interface preference above. It is a decision recorded
in an archived change, not a fixed law, and question 16 may legitimately reopen
it.

**`codekiln-help` is a toy.** codekiln: "the help example is just a toy. It's
just the first thing I came up with in order to be an example of something where
my preferences for CLIs might vary from the average user, so I don't want the
core clilint to have that built-in." Designing the extension model to express
`codekiln-help` well is therefore not the goal. Its only job is to be one case
where a personal preference lives outside the core.

**The "plan of steps" naming was rejected.** codekiln found the sequencing
metaphor a distraction and named simplicity as one of the project's core design
principles, then restated the underlying need directly: "A person who is using
clilint
is trying to reproduce behavior that may be validated mechanistically or with an
LLM as a judge. That's a simple thing to describe."

**The whole approach was too mechanistic.** This was the most important
correction. codekiln: "All of those sound pretty mechanistic... it almost sounds
like you're thinking that judgment-based checks are an afterthought. I think
this is essentially you falling into the availability bias: as a code-RL'd AI
model, there are more examples in your training data of thinking about
mechanistic checkers than flexible, judgment based checkers, which is in my
opinion a nascent software sub-industry of its own."

## 3. Research: how other tools handle extension

codekiln asked for a survey of other tools' extension systems as architectural
inspiration, and said explicitly: do not let this become a programming-language
decision. The goal is to think structurally about how agents author and install
check bundles independent of the language checks are written in, and consider
languages only afterward. Requiring Rust for any check that uses code remains an
option on the table.

codekiln also supplied a context packet on the **pi** coding editor's extension
system, prompted by Peter Steinberger saying on a podcast that copying pi's
model worked out well. codekiln's caveat: do not read the packet's verbosity as
an endorsement, and do not overfit on pi's model, which is TypeScript.

Facts verified against current documentation during the session:

| Tool | You install | Behavior written by | The host must |
|---|---|---|---|
| Vale | YAML files naming one of 12 built-in check kinds | the host | ship every kind it will ever support |
| Hurl | a plain-text file of requests, captures, assertions | the host | ship an interpreter for that format |
| Semgrep | YAML rules in a code pattern language | the host | ship the matching engine |
| Conftest | Rego policies | a third party (Rego) | embed a policy engine |
| Powerpipe | a git repo of SQL queries | a third party (SQL) | embed a query engine |
| Checkov | Python classes, or a simpler YAML format | both, side by side | support two routes |
| Dylint | a compiled Rust library | the extension author | ship a compiler driver, pin a toolchain |
| pre-commit | a git repo containing any executable | the extension author | define invocation, install their toolchain |
| ESLint | an npm package of rule callbacks | the extension author | be written in JavaScript |
| Clippy | nothing — no plugin system, deliberately | the host only | — |
| pi | a TypeScript file loaded in-process | the extension author | be written in TypeScript |

Notes worth keeping:

**Vale is clilint's closest relative.** A prose linter whose entire premise is
that you write your own writing standard and install it. A rule is a YAML file
saying `extends: existence` (or `substitution`, `occurrence`, `capitalization`,
one of twelve) plus data, mostly regular expressions and word lists. Users write
no code. There is a package hub of published standards. Structurally this is
what clilint does today: the bundle names a built-in check kind and supplies
data. Vale's twelfth kind, `script`, runs an embedded scripting language as a
second route for what the other eleven cannot reach.

**Hurl solves clilint's chaining problem as a file format, in Rust.** A `.hurl`
file lists HTTP requests in plain text. After a response, a `[Captures]` block
pulls a value out and names it; a `[Asserts]` block checks things; a captured
name is reused later as `{{name}}`:

```
POST https://api.example.org/jobs
HTTP 201
[Captures]
job_id: jsonpath "$.id"

GET https://api.example.org/jobs/{{job_id}}
HTTP 200
[Asserts]
jsonpath "$.status" == "RUNNING"
```

Substitute command invocations for HTTP requests and that is three of the four
capabilities question 16 names. Hurl has no loops, so "do this for each item I
captured" is not expressible — which is precisely the fourth capability.

**Tavern does the same thing in YAML** with `save:` to capture and `{name}` to
reuse, plus `$ext: function: module:name` to call out to Python when YAML runs
out.

**Powerpipe has the cleanest contract of the eleven.** A control is a SQL query
that must return rows containing columns named `status`, `reason`, and
`resource`. Anything producing those rows is a valid control. `powerpipe mod
install github.com/owner/repo` clones into `.powerpipe/` and records the
dependency; `--dry-run` prints what would run without running it.

**cram and trycmd** store CLI tests as transcripts: `$ command` followed by the
expected output. No captures, no loops. trycmd can rewrite the expected output
for you with `TRYCMD=overwrite`.

**Dylint's real cost is toolchain coupling.** Out-of-tree Rust lints are
compiled `cdylib`s exporting `register_lints`, listed in `Cargo.toml` workspace
metadata. Each library is bound to a specific rustc toolchain, and Dylint builds
and caches a compiler driver per toolchain.

**Clippy's answer is instructive by omission.** It has no plugin system on
purpose; every lint is contributed upstream. Dylint exists because people wanted
out-of-tree lints and Clippy would not have them.

**The pattern across all of them:** the tools that let checks be written in a
language the host does not speak are exactly the tools that fixed what a check
must *return*. Powerpipe returns rows with named columns; pre-commit returns an
exit code plus stdout. ESLint and pi cannot, because a check is a callback
inside the host's own runtime.

## 4. codekiln's project vision

The agent offered a summary of the project vision to check alignment. codekiln
said it was "fairly directionally correct" but described the project as
primarily mechanical, with judgment-based checks sounding like an afterthought.
Material from earlier ChatGPT conversations followed, pasted in to realign, with
the caveat that it is directional rather than a settled statement of codekiln's
views.

### The jig metaphor (pasted material, endorsed in direction)

"Lint" implies finding faults after something is written. A **jig** is a
purpose-built woodworking aid that constrains a task so it can be done
accurately and repeatably with less dependence on individual skill. A jig does
not do the work; it creates an environment in which the correct operation
becomes easy and the incorrect one becomes difficult.

The proposed formulation: "A CLI jig is an executable guide that constrains,
exercises, and evaluates the construction of a command-line interface so that
conformant behavior becomes repeatable."

The shift is from checking to **guiding construction**. The rules participate
throughout generation as continual feedback inside an agentic loop, rather than
grading a finished artifact.

codekiln intends to rebrand the project as `jig` eventually, and placed that
outside the scope of this discussion.

### Inner loop, outer loop, and round-trip reconstruction (pasted material)

The inner loop is the coding loop: an implementation agent edits code and
receives feedback from checks. The outer loop improves the jig itself, asking
not "did the implementation satisfy the jig?" but "was the jig sufficient to
reproduce the behavior of the original artifact?"

The round trip: observe an existing artifact, infer a jig from it, use the jig
to recreate the artifact in another implementation, measure behavioral
similarity. If the recreation behaves similarly, the jig captured the design.
That becomes the benchmark — proving the rules faithfully encode an observable
design, rather than proving agents follow rules.

The original motivation was analyzing an existing CLI such as `gh`, extracting
its conventions into a reusable package, and using that package to guide an
agent building a different CLI with the same shape.

### Rule, evaluator, assessment (pasted material, and a correction inside it)

ChatGPT initially proposed two kinds of rules, deterministic and judgment-based.
codekiln pushed back on that distinction. The model they arrived at instead:

```
Rule → Evaluator → Assessment
```

The evaluator is free to run Python, invoke shell commands, inspect a
repository, call an LLM, have the LLM call tools, or combine these. How it works
is an execution concern, not a modeling concern.

The better organizing principle proposed: not "mechanistic versus AI" but **what
object is this check about?** Several different subjects can be checked — did
the command execute, did evidence get collected, did the evaluator return a
valid assessment object, does the evidence satisfy the rule.

A clarification recorded in that thread: "validation of the judge's response"
means output-contract validation only — does the JSON parse, are required fields
present, is `result` one of `pass|warn|fail`, does it cite evidence that exists.
It says nothing about whether the judgment is correct.

### Agent Skills as the unit (codekiln, directly)

codekiln: "The nice thing about agent skills is that any knowledge worker can
read and write one given 10 minutes of instruction; it's a very simple basic
format ... but it can grow to include scripts, reference material, etc."

The main mental model codekiln holds for LLM-as-judge is an Agent Skill
(`~/ghq/github.com/agentskills/agentskills/docs`), with the caveat of not being
convinced it should be the basis for checks that need an LLM.

The pasted material goes further and proposes that every individual rule be
expressed as an AI skill, containing instructions, resources, executable
scripts, examples, structured output contracts, and verification logic —
leveraging an ecosystem agents already understand rather than inventing another
configuration language. Influences named: Claude Code packages, Claude skills,
agentskills.io.

### Vendoring, after ShadCN (pasted material)

Rather than depending on centralized packages at runtime, ShadCN vendors
component source directly into the project. The same ownership model appeals for
AI guidance: projects vendor the actual guidance rather than installing opaque
rule packages, making it editable, inspectable, forkable, and project-specific.

### Structured findings as the shared language (pasted material)

Whatever happens inside an evaluator, every evaluation produces structured
findings, and those findings are the language spoken between the outer and inner
loops.

### BAML (pasted material)

BAML was considered as a compilation target for rules and then set aside. It
looks like one possible runtime for evaluating rules, not the definition of the
rules. The idea worth borrowing is treating an LLM call as a typed function:
`evaluate(rule, evidence) -> Assessment`, with a strongly typed assessment
object.

### Eventual scope (codekiln, directly)

codekiln: "I'm hoping that eventually the scope of this project (once it's
rebranded as `jig`) will be much broader: it will be a flexible check harness
for any aspect of a project, whether that's how the CI supports the release
pipeline, how dev dependencies are installed and managed, how secrets are
managed, how the branch protection rules are configured, how the docs website is
created, ... in other words, not just mechanistic checks of CLI tools, but
basically any aspect of a software project should be able to have flexible
checks and mechanistic checks delivered in a bundle that define the
expectations."

And the goal statement: "These days we are thinking about loop engineering and
ralph loops and verification criteria. I want to make a metaframework for
expressing verification criteria in a way that's equally comfortable in
mechanistic, script-driven checks as it is in 'describe how it should be and the
AI agent decides the extent to which it meets your description using a rubric.'"

The pasted material lists candidate subjects beyond CLIs: README files, GitHub
project organization, release processes, semantic versioning, changelog
conventions, documentation, API design, CI pipelines, software architecture. A
README jig could be induced from top GitHub repositories; a release jig could
capture semantic-versioning and GitHub Releases conventions.

## 5. Where the agent's analysis landed after the reframe

These are the agent's positions after codekiln's corrections. codekiln has not
yet accepted or rejected them.

**Question 16 is a mechanical question in general clothes.** Its four
capabilities — read a value out of JSON, iterate, substitute, recurse — are all
about *gathering evidence*. None concerns judgment. Answering it as written
would settle the architecture in favor of the mechanical half, which is the bias
codekiln identified.

**The distinction that survives is gather versus evaluate, not mechanical versus
judged.** This agrees with the `Rule → Evaluator → Assessment` model. Notably,
clilint's code already contains this split but expresses it as two incompatible
shapes: a deterministic check carries a `checker`, while an AI check carries a
`skill` plus an `evidence` invocation. Same idea, two grammars. Unified, the
mechanical/judged distinction stops being architectural and becomes only a
question of which evaluator reads the evidence:

```
subject           evidence           evaluator                 assessment
CLI, repo,   →    what was      →    a built-in assertion,  →  result +
CI, docs,         observed            a script, or              reason +
release                               an agent + rubric         cited evidence
```

**The return contract is what makes evaluators interchangeable across
languages.** Powerpipe and pre-commit are the two surveyed tools that accept
checks in a language the host does not speak, and both did it by fixing what a
check must return. clilint has the beginnings of this in `Finding` and
`src/assessment.rs`. Nailing that contract is what would let a shell script, a
built-in assertion, and an LLM rubric be interchangeable — and it is what the
outer loop would measure.

**The language question is decided by loop iteration speed, not security.** If a
jig is derived by observing an existing tool, and the outer loop keeps refining
it, the format must be fast to write and rewrite. Compiled Rust plugins lose on
that alone — compile step, pinned toolchain, slow iteration — before trust
enters the picture. Markdown plus data plus optional scripts wins because an
agent can author and revise it in one turn. This is the strongest argument for
the skill format, and it is a vision argument rather than a matter of taste.

**Judgment is safe to make extensible; evidence collection is where the hard
problems live.** A rubric is prose an agent reads. "Run this, read the JSON, do
it for each thing you found" is either data the host interprets or code the host
trusts, and it is also where repeatability is won or lost. So question 16
shrinks to one thing: **how does a bundle say what evidence to collect?**

**Three candidate answers to that smaller question.** Only the first arrow of
the diagram above is at issue — who runs the commands and collects the output
that gets judged:

1. The Clilint binary collects it, following data in the bundle. Repeatable.
   Requires a data format. This is what question 16 assumed.
2. The bundle ships a script that Clilint runs. Repeatable if the script is.
   Brings a language choice and a trust decision.
3. The AI agent collects it, following instructions in the bundle. No format
   needed, works across arbitrary subjects, not repeatable.

A mechanical check needs option 1 or 2. A rubric can live with option 3. So the
choice may be per check rather than global.

codekiln could not follow this three-way split as originally phrased and asked
what it meant. The restatement above is the clarification. codekiln's position
on it is not yet recorded.

**The agent recommended making extensibility its own OpenSpec change,** on the
grounds that the `hierarchical-help-checking` delta spec already contains a
requirement about clilint's extension model disguised as a requirement about
help:

> Clilint SHALL provide a deterministic checker that discovers command paths by
> following immediate child commands in JSON help output and uses JSON output
> from one help command as input to later help commands.

codekiln's response: discussing that split "feels too myopic to talk about yet,
because I don't have a good sense [whether we] are even 50% aligned with me on
the project vision (I need that to get to 90% before we talk details)."

## 6. Current code, for reference

So a later session does not have to re-derive it.

Check kinds in the bundle format (`CheckerDefinition` in `src/check_bundle.rs`):
`invocation`, `any-invocation`, `all-invocations`, `hierarchical-help`.

The pi context packet codekiln supplied lists `shell-program` as a fourth check
kind. That is wrong. `shell-program` appears in `src/check_bundle.rs` only
inside a test that substitutes it for a valid kind to confirm the parser rejects
unknown kinds.

Assertions (`Assertion`, same file), eleven in total: `exit-code`,
`exit-non-zero`, `not-timed-out`, `stdout-not-empty`, `stderr-not-empty`,
`stdout-at-least-stderr`, `output-contains-any`, `stdout-contains-any`,
`no-ansi`, `duration-at-most`, `version-number`. All operate on one
`Observation`, which holds args, exit status, timed-out flag, duration, stdout,
stderr, and an ANSI flag.

`EvaluationMethod` is `Deterministic` or `AiAgent`. A deterministic check
requires a `checker` and forbids `skill` and `evidence`; an AI check requires
`skill` and `evidence` and forbids `checker`. Validation enforces this in
`check_bundle::validate`.

`src/help_checker.rs` is 873 lines implementing the `hierarchical-help` kind. It
discovers command paths from JSON help, runs help commands at each path, pulls
section names out of outlines and passes them to later commands, and records
failures tagged with one of ten `HierarchicalHelpBehavior` values. The
`codekiln-help` bundle file is nothing but ten checks naming those ten values.

The core bundle is `check-bundles/clilint/clilint.toml`, compiled into the
binary with `include_str!`. It has 17 checks, 16 deterministic and one AI-agent
(`clilint/help/useful-example`, pointing at the `assess-cli-help` skill).

Task 7.4 in `tasks.md` lists implementation problems found by an earlier review,
grouped under question 16 because the question governs whether
`src/help_checker.rs` survives. They are not repeated here.

## 7. Still open

- **Question 16 itself.** Unanswered, and the agent's position is that it should
  be restated before it is answered.
- **Whether the extension model becomes its own OpenSpec change.** The agent
  recommends yes. codekiln considers the question premature until vision
  alignment is higher.
- **codekiln's position on the three candidate answers for evidence
  collection.** Not yet recorded; the restatement in section 5 was the first
  readable version.
- **Whether a rubric is vendored into the bundle or referenced by name and
  version.** The ShadCN influence argues for vendoring. The current code
  references a skill by name and version. Not discussed.
- **Whether Agent Skills become the unit of a check.** codekiln named the
  format's authorability as its main appeal, and separately expressed doubt
  about using it as the basis for LLM-needing checks.
- **How a check identifies its subject** once subjects include repositories, CI
  configuration, release processes, and documentation sites rather than only a
  CLI binary. Raised by the vision, not yet discussed.
- **Renaming to `jig`.** codekiln says out of scope here.

## Sources consulted

- [Vale styles and check types](https://docs.vale.sh/topics/styles)
- [Hurl captures](https://hurl.dev/docs/capturing-response.html) and
  [templates](https://hurl.dev/docs/templates.html)
- [Powerpipe controls](https://powerpipe.io/docs/powerpipe-hcl/control) and
  [mod dependencies](https://powerpipe.io/docs/build/mod-dependencies)
- [pre-commit hook authoring](https://pre-commit.com/#new-hooks)
- [How Dylint works](https://github.com/trailofbits/dylint/blob/master/docs/how_dylint_works.md)
- [trycmd file formats](https://docs.rs/trycmd/latest/trycmd/)
- [Tavern basics](https://github.com/taverntesting/tavern/blob/master/docs/source/basics.md)
- pi extension system: context packet supplied by codekiln, sourced from
  `~/ghq/github.com/earendil-works/pi` and codekiln's Logseq notes under
  `PiAI___Extension*`
