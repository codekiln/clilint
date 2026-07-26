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

- [x] 2.1 Rename package, rule, and check terms consistently across CLI options, bundle files, Rust types, reports, and documentation without compatibility aliases
- [x] 2.2 Extend check-bundle installation and loading to preserve the order of inherited bundles and checks
- [x] 2.3 Add named-bundle installation and accept a local bundle path as an installation source
- [x] 2.4 Allow local check bundles to extend `codekiln-help` without weakening inherited checks
- [x] 2.5 Apply the same format and validation to `codekiln-help` and check bundles supplied by users
- [x] 2.6 Add tests for the built-in core bundle, installed `codekiln-help`, and a local extension

## 3. Add Hierarchical Help Checking

- [x] 3.1 Define the hierarchical help checker, supported checks, size limits, and check-bundle validation
- [x] 3.2 Parse and validate JSON help overviews, child commands, outlines, search results, and section results
- [x] 3.3 Run the required help commands at every discovered command path and stop at configured limits
- [x] 3.4 Reuse command-hierarchy and help-document observations during one Clilint run while producing a separate finding for each check
- [x] 3.5 Add report evidence for command paths, commands run, captured output, requested sections, and validation failures
- [x] 3.6 Add unit tests for malformed output, invalid relationships, exceeded limits, timeouts, and reused results

## 4. Build the `codekiln-help` Check Bundle

- [x] 4.1 Add one `codekiln-help` check for each help behavior defined by the specifications
- [x] 4.2 Add tested CLI fixtures covering child-command discovery, shared help, optional programmatic guidance, outlines, sections, search, and viewers
- [x] 4.3 Add check-bundle examples that connect check definitions to passing and failing fixture behavior
- [x] 4.4 Add integration tests that identify failures at root, group, and deeply nested command paths
- [x] 4.5 Add non-interactive tests for closed input, plain output, offline execution, and viewer behavior
- [x] 4.6 Add tests that default help serves people and agents and that `--programmatic` retains it while adding programmatic guidance

## 5. Document and Verify

- [x] 5.1 Write a check-bundle authoring guide that explains `codekiln-help` from its bundle file through its checks, fixtures, and report results
- [x] 5.2 Document the help commands, JSON fields, check-bundle installation, checker limits, and examples for CLI authors
- [x] 5.3 Add links between the guide, `codekiln-help` source, checks, and tests
- [x] 5.4 Run the experiment checks and record any result that changes the design
- [x] 5.5 Run formatting, linting, tests, and OpenSpec validation through the project mise tasks
- [ ] 5.6 Verify the implementation against the proposal, design, and all delta specs before archive

## 6. Reduce Scope to What Is Settled

A review found behavior that does not match the design, and answers 15 and 17
removed work from this change. These items need no further decision.

- [x] 6.1 Open issue #8 for Git sourcing, ref resolution, the lockfile, and the data and cache directories, carrying over resolved questions 10 through 14
- [x] 6.2 Open issue #9 for rich local viewing and web viewing, recording the `gh` precedent and the clig.dev constraints
- [ ] 6.3 Reduce installation to a local path, per answer 15. Remove the Git source type, ref resolution, `.clilint/lock.toml`, `bundle lock`, `bundle update`, `--locked`, `--offline`, and the data and cache directories from `src/project_config.rs`, `src/cli.rs`, `docs/check-bundles.md`, and the `conformance-packages` delta spec
- [ ] 6.4 Remove the `view` operation, per answer 17. Drop the `Local help viewer` and `Web help viewer` requirements from `hierarchical-help-standard`, the `Web check rating` requirement from `hierarchical-help-checking`, the `local-view` and `web-view` checks and the `--version` handling that supports them, and the viewer text in `proposal.md` and `docs/codekiln-help.md`
- [ ] 6.5 Record a local bundle source as a path relative to the project. `clilint bundle install ../mybundle` writes an absolute, unnormalized path into the committed `.clilint/config.toml` (`src/project_config.rs:266`)
- [ ] 6.6 Remove `check_bundle::load_resolved`, superseded by `project_config::load_for_check` and now unused (`src/check_bundle.rs:134`)
- [ ] 6.7 Rename the `conformance-packages` capability to match the check-bundle terminology, or record in `proposal.md` why it keeps the earlier identifier
- [x] 6.8 Give each prototype its own named subfolder under `experiments/`, with a README describing what it tests and how to run it
- [ ] 6.9 Add `references/` notes for mise, uv, Dev Container Features, Claude Code plugin marketplaces, and RuleSync, each of which shaped an installation decision
- [ ] 6.10 Run the mise format, lint, test, and OpenSpec validation tasks after the reductions

## 7. Rework How a Check Bundle Expresses Behavior

Open question 16 asks how a bundle expresses new behavior without a change to
the Clilint binary. It governs whether `src/help_checker.rs` and the
`hierarchical-help` checker type survive at all, so the remaining findings are
grouped here rather than patched in place.

- [ ] 7.1 Hold a dedicated OpenSpec explore session on question 16 and record the answer in `design.md`
  - a first session ran on 2026-07-25 and restated question 16 rather than
    answering it; the record is in
    `references/brainstorm-reframing-plugin-architecture-for-judgment-based-checks-influenced-by-jig-idea.md`
  - the answer is still open, so this task stays incomplete
- [ ] 7.2 Revise the `hierarchical-help-checking` delta spec, which currently requires Clilint itself to supply the checker that discovers command paths
- [ ] 7.3 Re-express the `codekiln-help` checks under the chosen model, then confirm an author can write an equivalent bundle without changing Clilint
- [ ] 7.4 Carry these findings into the rework or fix them where the code survives:
  - limits are accepted on each checker and then required to be identical by two separate validations (`src/check_bundle.rs:64`, `src/engine.rs:26`)
  - `flatten` disables `deny_unknown_fields`, so `timeout_mss = 1` validates, runs, and silently uses the default (`src/check_bundle.rs:88`)
  - the passing fixture runs about 41 commands per command path, so `command_count = 64` cannot be reached within `total_commands = 1024` (`src/check_bundle.rs:97`)
  - an exceeded command budget fails whichever behavior was running instead of reporting itself (`src/help_checker.rs:776`)
  - the three-command fixture produces a 235 KB report, 67 KB of it in the non-interactive finding, which receives every observation (`src/help_checker.rs:146`)
  - the programmatic check requires byte-exact containment of the default help plus the literal words `json` and `section`, rather than the four instructions the spec requires (`src/help_checker.rs:305`)
  - the search check passes on a tool whose search returns nothing (`src/help_checker.rs:644`)
- [ ] 7.5 Run the mise format, lint, test, and OpenSpec validation tasks, then repeat task 5.6 against the revised artifacts
