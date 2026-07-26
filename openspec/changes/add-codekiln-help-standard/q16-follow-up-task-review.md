# Q16 follow-up task review

## Implementation status

The replacement described by this review has now been applied. The sections
under “Stale implementation” record the state found before the replacement;
they are retained to explain why the replacement tasks exist. The current
implementation uses the shared Check Outcome model, one Checker CLI for the
`codekiln-help` Check, local bundle paths, and no viewer behavior.

## Main conclusion

The proposal, design, delta specifications, and task list now describe the
replacement required by answer 16. Questions 18 through 21 in `design.md`
originally recorded the remaining project-owner decisions. Questions 18 and 22
now settle the shared result terms and score. Answer 23 requires one Checker
CLI per bundle-owned Check and keeps the project as its working directory.
Questions 19 through 21 are settled. Application work can proceed without
further project-owner decisions.

The replacement should preserve useful behavior through black-box tests. Once
the bundle-owned check passes those tests, remove the fixed hierarchical-help
checker and its engine paths.

## Settled decisions

Six decisions govern the remaining work:

- Answer 15 limits check-bundle installation in this change to local paths.
- Answer 17 removes local and web help viewers from this change.
- Answer 16 gives each check one bundle-owned checker for its complete run.
  The checker owns any setup, evidence gathering, and production of a Check
  Outcome. Clilint standardizes the request and outcome boundaries.
- Answer 18 gives every completed Check Result a finite floating-point Score
  from `0.0` through `4.0` and Check Messages that explain the Score.
- Answer 22 reserves Assessment for judgment-based checks and replaces the
  shared terms Assessment and Finding with Check Result and Check Message.
- Answer 23 exposes each bundle-owned Check through a Checker CLI. Clilint
  runs the CLI from the directory in which the user invoked Clilint. Agent
  Skills and other tools remain behind the Checker CLI boundary.

Question 16 also leaves room for one check to gather evidence once, verify
several related matters, and return one Score with several Check Messages.

## Stale implementation found by the review

### Check-bundle schema

`src/check_bundle.rs` defines a fixed `CheckerDefinition` enum. Adding
hierarchical help required adding `HierarchicalHelp`, `HelpLimits`, and
`HierarchicalHelpBehavior` to the Clilint binary. That is the extension model
question 16 replaces.

The loader also reduces each loaded bundle to its parsed manifest. Clilint
needs the bundle directory long enough to resolve the Checker CLI. The Checker
CLI then resolves its own scripts, Skills, rubrics, references, and other
installed resources.

### Execution engine

`src/engine.rs` dispatches each deterministic check through Rust checker
variants. It contains special handling that collects one `HelpContext` for all
hierarchical-help checks. Its agent path also asks Clilint to gather evidence
before an external agent assesses it.

Under answers 16 and 23, the engine should invoke a complete Checker CLI from
the project directory and validate its returned Check Outcome. Check-specific
evidence gathering belongs to the Checker CLI.

### Hierarchical-help checker

`src/help_checker.rs` contains the check-specific command traversal, response
parsing, limits, validation, and result selection. The useful behavior should
move to the `codekiln-help` bundle. The Rust module and the corresponding
engine and manifest variants should leave the Clilint binary after replacement
tests pass.

The existing review findings should guide acceptance tests for the new
bundle-owned check:

- limits must form a reachable and coherent set;
- unknown manifest fields must fail validation;
- an exceeded command budget must be reported directly;
- reports must bound and focus their evidence;
- programmatic guidance must satisfy the specified instructions;
- empty search results must not pass a search check; and
- failures must retain the relevant command path and invocation evidence.

### Check Outcome and report model

`src/model.rs` and `src/report.rs` treat a finding as the result of a check.
The current `Finding` type is therefore closer to the newly settled Check
Result than to a Check Message. The new model needs a request-bound Check
Outcome that contains exactly one Check Result, Check Error, Awaiting
Assessment state, or Skipped state.

A valid Check Result has one Score and zero or more Info, Warning, or Error
Check Messages. A Score below `4.0` requires an improvement message. A Score of
`4.0` cannot contain a Warning or Error message. A Check Error has no Score.

### Agent-assessment workflow

The current `agent-assessments` specification, `src/assessment.rs`,
`skills/assess-cli-help/`, and `--assessment` workflow implement a
host-gathered-evidence model. A judgment-based Checker CLI instead owns
evidence gathering, exposes any Skill and rubric needed by an external agent,
validates the Assessment, and returns the Check Result.

`src/assessment.rs` calls its AI-only file type `AssessmentDocument`. The
settled domain term is Assessment: the structured record of applying a rubric
to evidence. “Document” describes the current file transport and should not
remain part of the domain type name.

An Agent Skill is loaded and followed by an agent client; the Skill file is not
an executable program. Answer 23 puts it behind the Checker CLI rather than
making it a second Clilint checker implementation. The design still needs a
concrete external-agent handoff before implementation begins.

### Installation

`src/project_config.rs`, `src/cli.rs`, integration tests, and
`docs/check-bundles.md` still implement Git sources, ref resolution, a
lockfile, installation and cache directories, missing-bundle restoration,
`--locked`, and `--offline`. Answer 15 defers that work.

The local installer currently writes an absolute path to
`.clilint/config.toml`. It should preserve a path relative to the project
directory, including a useful `..` path when the bundle is beside the project.

### Viewer behavior

The `codekiln-help` manifest, `src/help_checker.rs`, the passing fixture,
integration tests, README, and focused guides still describe local and web
viewers. The planning artifacts now reflect answer 17; the application and
product documentation do not.

### Tests and documentation

Several tests currently enforce stale behavior:

- ten fixed `codekiln-help` checks using the Rust hierarchical-help checker;
- Git installation, locking, cache directories, and offline restoration;
- local and web view behavior;
- one report `Finding` per check; and
- Clilint-gathered evidence for a judgment-based Check.

The README and focused guides repeat those assumptions. Passing tests and
strict OpenSpec validation therefore do not establish agreement with the
settled design.

## Settled result model

Define versioned types for:

- check and check-bundle identity;
- tested target and project context;
- Check Outcome, Check Result, Check Error, Awaiting Assessment, and Skipped;
- a fractional Score from `0.0` through `4.0`;
- Info, Warning, and Error Check Messages;
- evidence and evidence references;
- mechanistic and judgment-based methods; and
- the message rules for Scores below and equal to `4.0`.

A check is judgment-based when human or model interpretation affects its Score
or Check Messages. Otherwise, it is mechanistic. A judgment-based check may
still run scripts while gathering evidence.

## Settled protocol choices

### Checker CLI invocation

The first Checker CLI contract uses:

- Checker CLI manifest syntax;
- CLI resolution against the bundle directory;
- JSON input and output;
- environment handling;
- fixed whole-check timeout and protocol-output limits owned by Clilint;
- bounded retention and truncation reporting for standard-error checker logs;
- no per-bundle process-limit configuration;
- malformed output and nonzero-exit behavior;
- Checker runtime dependency responsibility; and
- the trust boundary created by executing an installed local bundle.

The working directory is settled: the Checker CLI inherits the directory in
which the user invoked Clilint, normally the tested project directory.

### Judgment-based Checker CLI handoff

The first handoff retains the external two-pass workflow:

```text
Clilint invokes the Checker CLI
    |
    v
the Checker CLI returns Awaiting Assessment with its Skill, rubric, and evidence
    |
    v
an external agent writes an Assessment
    |
    v
the Checker CLI validates the Assessment and returns a Check Result
```

The Assessment travels in a JSON file supplied to a later Clilint invocation.
The Checker CLI validates the request binding, Skill identity, Assessment, and
Check Result. Clilint does not start or identify the external agent harness.

### Built-in checks

The built-in `clilint` checks keep their current Rust checker types in this
change and adopt the shared Check Result model. `codekiln-help` is the proof
that an installed bundle can add behavior without a binary change.

### `codekiln-help` check boundary

`codekiln` chose one hierarchical-help check that returns one Score and several
Check Messages.
The checker gathers the command hierarchy once. A later change can split the
check if separate execution or reporting becomes useful.

## Task-list changes applied

- The final implementation review moved from task 5.6 to group 13.
- The installation reduction is split across configuration, CLI, storage code,
  tests, and documentation.
- The viewer removal is split across the bundle, fixture, tests, and
  documentation.
- Relative local paths are part of the local-source work, with project
  subdirectories and sibling bundle paths named in tests.
- The unused loader is part of the reduction work.
- `conformance-packages` keeps its existing OpenSpec capability identifier
  because this change modifies that specification. Public terminology still
  changes to `check bundle`.
- The research sources for remote installation belong with issue #8.
- The scope-reduction checkpoint remains before the Checker CLI replacement.
- The protocol work spans check bundles, mechanistic Checker CLIs,
  judgment-based Checker CLIs, reporting, and hierarchical-help checking.
- Each bundle-owned Check now has one Checker CLI. Agent Skills and other
  implementation choices remain behind that CLI boundary.
- The shared output model is Check Outcome, Check Result, Score, Check Error,
  and Check Message. Assessment remains specific to judgment-based checks.
- The Checker CLI protocol, proof bundle, judgment-based handoff, `codekiln-help`
  replacement, and old-code removal are separate task groups.
- The known hierarchical-help defects are black-box acceptance tests for the
  replacement.
- Final verification includes the README assessments and the pre-commit secret
  scan.

## Recommended work order

### 1. Apply the settled protocol decisions

The proposal, design, delta specifications, and tasks now describe the
whole-check Checker CLI model and stale-code replacement. No further
project-owner answer is required before application work begins.

### 2. Remove the independently stale scope

1. Reduce project configuration and commands to local bundle sources.
2. Store local source paths relative to the project directory.
3. Remove Git, lockfile, cache, offline-restoration, and related test and
   documentation paths.
4. Remove viewer requirements, checks, fixture behavior, tests, and
   documentation.
5. Run the repository checks as a smaller baseline.

### 3. Prove the Checker CLI protocol

1. Preserve each bundle's source directory long enough to resolve its Checker
   CLI.
2. Add the versioned Check Request and Check Outcome types.
3. Add Checker CLI manifest validation.
4. Run the CLI from the project directory with bounded protocol output,
   Checker logs, and time.
5. Validate its Check Outcome before adding it to the report.
6. Add a small installed bundle that proves a new check can gather evidence
   and return one Score with several Check Messages without a Clilint binary
   change.

### 4. Prove the judgment-based Checker CLI handoff

1. Prototype the external-agent exchange.
2. Bind an Assessment and Check Result to the pending check request.
3. Make the Checker CLI resolve its installed Skill and rubric.
4. Make the Checker CLI validate the Assessment and return the Check Result.
5. Update or replace the current Assessment attachment interface.
6. Rewrite the help-assessment Skill and its tests around whole-check
   ownership behind the Checker CLI.

### 5. Replace hierarchical help

1. Replace the ten manifest checks with the one settled hierarchical-help
   check.
2. Port applicable hierarchical-help behavior into its bundle-owned Checker
   CLI.
3. Encode the known behavior listed above as black-box tests.
4. Remove the `HierarchicalHelp` enum, `HelpLimits`, engine special cases, and
   `src/help_checker.rs` after replacement tests pass.

### 6. Update reports and documentation

1. Present one Check Result with a Score and several Check Messages.
2. Apply the settled summary and exit-status rules.
3. Rewrite the check-bundle, AI-assessment, vision, and `codekiln-help` guides.
4. Update README and run both README assessment skills.
5. Confirm that `codekiln-help` uses the same extension path available to
   another bundle author.

### 7. Verify the revised change

Run formatting, Clippy, tests, experiment checks, strict OpenSpec validation,
and the pre-commit secret scan. Verify the implementation against the revised
proposal, design, and delta specifications. Run spec sync before archive.
