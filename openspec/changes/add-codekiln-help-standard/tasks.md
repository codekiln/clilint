## 1. Resolve Open Design Questions

- [x] 1.1 Choose the section-retrieval contract and update the design and hierarchical-help-standard spec with the decision
- [x] 1.2 Choose whether JSON programmatic help identifies the source of inherited guidance
- [x] 1.3 Assign web viewing to the `Good` and `Excellent` `codekiln-help` rating levels
- [x] 1.4 Decide whether discrete rating calculation belongs in this change or a separate OpenSpec change
- [x] 1.5 Decide whether to rename the current public package, rule, and check interfaces or add compatibility aliases
- [x] 1.6 Choose the flag for optional programmatic-use guidance
- [x] 1.7 Decide where Clilint obtains named check bundles during installation
- [x] 1.8 Decide the scope or location of a check-bundle installation

## 2. Add Check-Bundle Installation

Groups 2 through 5 record the first implementation. Answers 15, 16, and 17
superseded parts of that work. The checked boxes preserve what was completed;
groups 6 onward describe the replacement and removal work still required.

- [x] 2.1 Rename package, rule, and check terms consistently across CLI options, bundle files, Rust types, reports, and documentation without compatibility aliases
- [x] 2.2 Extend check-bundle installation and loading to preserve the declared order of combined bundles and Checks
- [x] 2.3 Add named-bundle installation and accept a local bundle path as an installation source
- [x] 2.4 Allow local check bundles to include `codekiln-help` through `extends` while preserving its Checks unchanged
- [x] 2.5 Apply the same format and validation to `codekiln-help` and check bundles supplied by users
- [x] 2.6 Add tests for the built-in core bundle, installed `codekiln-help`, and an additional local bundle

## 3. Add Hierarchical Help Checking

- [x] 3.1 Define the hierarchical help checker, supported checks, size limits, and check-bundle validation
- [x] 3.2 Parse and validate JSON help overviews, child commands, outlines, search results, and section results
- [x] 3.3 Run the required help commands at every discovered command path and stop at configured limits
- [x] 3.4 Reuse command-hierarchy and help-document observations during one Clilint run while producing a separate legacy `Finding` for each check
- [x] 3.5 Add report evidence for command paths, commands run, captured output, requested sections, and validation failures
- [x] 3.6 Add unit tests for malformed output, invalid relationships, exceeded limits, timeouts, and reused results

## 4. Build the `codekiln-help` Check Bundle

- [x] 4.1 Add one `codekiln-help` check for each help behavior defined by the specifications
- [x] 4.2 Add tested CLI fixtures covering child-command discovery, shared help, optional programmatic guidance, outlines, sections, search, and viewers
- [x] 4.3 Add check-bundle examples that connect check definitions to passing and failing fixture behavior
- [x] 4.4 Add integration tests that identify failures at root, group, and deeply nested command paths
- [x] 4.5 Add non-interactive tests for closed input, plain output, and viewer behavior
- [x] 4.6 Add tests that default help serves people and agents and that `--programmatic` retains it while adding programmatic guidance

## 5. Document and Verify

- [x] 5.1 Write a check-bundle authoring guide that explains `codekiln-help` from its bundle file through its checks, fixtures, and report results
- [x] 5.2 Document the help commands, JSON fields, check-bundle installation, checker limits, and examples for CLI authors
- [x] 5.3 Add links between the guide, `codekiln-help` source, checks, and tests
- [x] 5.4 Run the experiment checks and record any result that changes the design
- [x] 5.5 Run formatting, linting, tests, and OpenSpec validation through the project mise tasks

The final implementation review formerly listed as task 5.6 is now task 13.4,
after the replacement work.

## 6. Reduce Scope to What Is Settled

A review found behavior that does not match answers 15 and 17. Complete this
reduction before introducing the Checker CLI protocol so the replacement
starts from the smaller supported surface.

- [x] 6.1 Open issue #8 for Git sourcing, ref resolution, the lockfile, and the data and cache directories, carrying over resolved questions 10 through 14
- [x] 6.2 Open issue #9 for rich local viewing and web viewing, recording the `gh` precedent and the clig.dev constraints
- [x] 6.3 Reduce `BundleSource` and `.clilint/config.toml` loading to local bundle paths
- [x] 6.4 Store installed local paths relative to the project directory and test project subdirectories and sibling bundle paths containing `..`
- [x] 6.5 Remove `bundle lock`, `bundle update`, `--locked`, and `--offline` from the CLI and update CLI parsing tests
- [x] 6.6 Remove Git fetching, ref resolution, the lockfile, data and cache directories, missing-bundle restoration, and their dependencies from `src/project_config.rs`
- [x] 6.7 Replace Git, lockfile, cache, and offline-restoration integration tests with local-path loading, listing, removal, and failure tests
- [x] 6.8 Remove local and web viewer checks, fixture behavior, viewer-specific version validation, and their tests
- [x] 6.9 Remove viewer and remote-installation text from the focused guides and other product documentation
- [x] 6.10 Remove `check_bundle::load_resolved`, which `project_config::load_for_check` superseded
- [x] 6.11 Give each prototype its own named subfolder under `experiments/`, with a README describing what it tests and how to run it
- [x] 6.12 Run format, lint, tests, and strict OpenSpec validation after the reductions

## 7. Resolve the First Protocol Details

Answer 16 gives each Check one bundle-owned Checker for its complete run.
The [Q16 follow-up task review](q16-follow-up-task-review.md) maps the stale
implementation and the work that follows. Questions 18 and 22 settle the
shared result model and terms. Answer 23 requires a Checker CLI for every
bundle-owned Check and keeps the project as its working directory. Questions
19 through 21 record the protocol choices used by the application work.

- [x] 7.1 Hold a dedicated OpenSpec explore session on question 16 and record the answer in `design.md`
  - a first session ran on 2026-07-25 and restated question 16 rather than
    answering it; the record is in
    `references/brainstorm-reframing-plugin-architecture-for-judgment-based-checks-influenced-by-jig-idea.md`
  - `experiments/check-extension-model-comparison/` compares a whole-check
    entry point with host-managed lifecycle phases
- [x] 7.2 Have `codekiln` answer questions 18 and 22 in `design.md` and define Check Outcome, Check Result, Score, Check Message, Assessment, and mechanistic versus judgment-based checks
- [x] 7.3 Compare mechanistic and judgment-based Checker CLI prototypes and record `codekiln`'s answer 23 in `design.md`
- [x] 7.4 Settle questions 19 through 21 from the established project-owner preferences and record the drafting-agent decisions in `design.md`
- [x] 7.5 Replace the placeholders and reconcile the decisions, delta specs, and remaining tasks

## 8. Add the Check Outcome and Check Result Model

- [x] 8.1 Add versioned Check Request, Check Outcome, Check Result, Check Error, Score, Check Message, and judgment-based Assessment types with strict unknown-field validation
- [x] 8.2 Make Check Result, Check Error, Awaiting Assessment, and Skipped mutually exclusive Check Outcomes
- [x] 8.3 Accept finite fractional Scores from `0.0` through `4.0` and enforce the settled Check Message rules for imperfect and perfect Scores
- [x] 8.4 Bind each Check Outcome to its request, check identifier, and check-bundle identity and reject mismatches
- [x] 8.5 Count Check Results, Check Errors, Awaiting Assessment outcomes, and Check Message levels separately and derive process exit status from unfinished work or errors rather than Score
- [x] 8.6 Adapt built-in checker outcomes to the new model according to answer 21
- [x] 8.7 Update JSON and human reports to present Scores, Check Messages, Check Errors, and mechanistic or judgment-based methods separately
- [x] 8.8 Add serialization, range, message-rule, mismatch, summary, and exit-status tests for the new model

## 9. Add the Checker CLI Protocol

- [x] 9.1 Preserve each loaded check bundle's source directory long enough to resolve its Checker CLI
- [x] 9.2 Add strict manifest validation for one nonempty Checker CLI command array per bundle-owned Check and expand the literal `{bundle}` placeholder in its arguments
- [x] 9.3 Construct the versioned request with the tested CLI tool, project directory, protocol version, and Check identity
- [x] 9.4 Invoke the Checker CLI directly without a shell from the directory in which the user invoked Clilint, inherit the environment, and send one JSON request on standard input
- [x] 9.5 Keep one Check Outcome JSON document on standard output, apply fixed process and output limits, retain only bounded Checker logs from standard error, and mark truncated logs
- [x] 9.6 Convert nonzero exit, timeout, excessive output, and malformed protocol output into a Check Error without a Score
- [x] 9.7 Add a small local proof bundle whose Checker CLI finds its installed resources, gathers evidence, and returns one Score with several Check Messages without check-specific Rust code
- [x] 9.8 Add unit and integration tests for project-directory execution, Checker resource resolution, language-independent request exchange, bounded memory, protocol errors, Checker logs, and the proof bundle

## 10. Add the Judgment-Based Checker CLI Handoff

- [x] 10.1 Define the Awaiting Assessment data that a judgment-based Checker CLI may return, including the request binding and any Skill, rubric, and evidence needed by an external agent
- [x] 10.2 Require the Checker CLI to resolve its own Agent Skill, rubric, and other installed resources
- [x] 10.3 Record pending Assessment work in the report, accept one Assessment JSON file on a later Clilint invocation, and document the external agent's expected input and output
- [x] 10.4 Require the Checker CLI to validate the returned Assessment before producing a request-bound Check Result
- [x] 10.5 Replace the stale `AssessmentDocument` and evidence-digest attachment interfaces according to answer 20
- [x] 10.6 Update `assess-cli-help` or add a focused judgment-based Checker CLI whose bundled Skill records an Assessment and whose CLI returns the shared Check Result
- [x] 10.7 Keep agent-harness operational logs outside Check Results and avoid requiring a model-specific or harness-specific log format
- [x] 10.8 Add tests for pending requests, valid attachment, wrong request, wrong check, wrong Skill, repeated attachment, and invalid Scores or Check Messages

## 11. Replace the Fixed Hierarchical-Help Checker

- [x] 11.1 Replace the ten `codekiln-help` manifest checks with one Checker CLI for the complete hierarchical-help Check
- [x] 11.2 Port command discovery, JSON parsing, outline, section, shared-help, programmatic-guidance, search, and non-interactive behavior into the bundle-owned Checker CLI
- [x] 11.3 Return one Check Result with a Score, focused Check Messages, and shared evidence instead of copying every observation into each message
- [x] 11.4 Make command-count, command-depth, document-size, search-result, total-command, and per-command timeout limits independently reachable and report the exceeded limit directly
- [x] 11.5 Reject unknown checker configuration fields and add a regression test for a misspelled timeout field
- [x] 11.6 Add black-box regressions for incomplete programmatic guidance, empty search results, exhausted command budgets, malformed relationships, and deeply nested failures
- [x] 11.7 Confirm an independently authored local bundle can implement an equivalent complete Check through the same Checker CLI protocol
- [x] 11.8 Remove `HierarchicalHelp`, `HierarchicalHelpBehavior`, `HelpLimits`, the special `HelpContext` engine path, and `src/help_checker.rs` after the replacement tests pass
- [x] 11.9 Remove tests and report fixtures that require one Clilint result per help behavior

## 12. Update Documentation

- [x] 12.1 Rewrite the check-bundle guide around local sources, one Checker CLI per bundle-owned Check, project-directory execution, Check Outcomes, Check Results, and Check Messages
- [x] 12.2 Rewrite the `codekiln-help` guide around its one complete check and link its bundle, checker, fixtures, and black-box tests
- [x] 12.3 Document the Checker CLI protocol, bounded Checker logs, judgment-based handoff, trust boundary, failure behavior, and report format
- [x] 12.4 Update the README and run both README assessment skills, resolving each finding or recording an accepted exception
- [x] 12.5 Update source and test links so a bundle author can follow each worked example without searching

## 13. Verify and Prepare for Archive

- [x] 13.1 Run formatting, Clippy, tests, and strict OpenSpec validation through the project mise tasks
- [x] 13.2 Run every experiment and record any result that changes the design
- [x] 13.3 Run the lefthook pre-commit secret scan
- [x] 13.4 Verify the implementation against the proposal, design, and every delta specification
- [x] 13.5 Replace fixed Checker output and log limits with temporary-file outcome capture and standard-error propagation, then update the guides and protocol tests
- [x] 13.6 Remove help search from the first standard, Checker CLI, fixtures, tests, and guides after review
- [x] 13.7 Remove unused manifest controls, keep built-in failure message levels with the built-in Checker definitions, and keep failed binary Scores at `0.0`
- [x] 13.8 Preserve declared Check order, validate proposed project configuration before writing it, validate every Checker outcome, and make Awaiting Assessment exit with status 1
- [x] 13.9 Align the Checker, design, specifications, and guides with the execution behavior Clilint can verify
- [ ] 13.10 Run the spec-sync workflow before archiving the change
