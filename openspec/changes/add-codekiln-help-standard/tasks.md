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

- [ ] 2.1 Rename package, rule, and check terms consistently across CLI options, bundle files, Rust types, reports, and documentation without compatibility aliases
- [ ] 2.2 Extend check-bundle installation and loading to preserve the order of inherited bundles and checks
- [ ] 2.3 Add named-bundle installation and accept a local bundle path as an installation source
- [ ] 2.4 Allow local check bundles to extend `codekiln-help` without weakening inherited checks
- [ ] 2.5 Apply the same format and validation to `codekiln-help` and check bundles supplied by users
- [ ] 2.6 Add tests for the built-in core bundle, installed `codekiln-help`, and a local extension

## 3. Add Hierarchical Help Checking

- [ ] 3.1 Define the hierarchical help checker, supported checks, size limits, and check-bundle validation
- [ ] 3.2 Parse and validate JSON help overviews, child commands, outlines, search results, and section results
- [ ] 3.3 Run the required help commands at every discovered command path and stop at configured limits
- [ ] 3.4 Reuse command-hierarchy and help-document observations during one Clilint run while producing a separate finding for each check
- [ ] 3.5 Add report evidence for command paths, commands run, captured output, requested sections, and validation failures
- [ ] 3.6 Add unit tests for malformed output, invalid relationships, exceeded limits, timeouts, and reused results

## 4. Build the `codekiln-help` Check Bundle

- [ ] 4.1 Add one `codekiln-help` check for each help behavior defined by the specifications
- [ ] 4.2 Add tested CLI fixtures covering child-command discovery, shared help, optional programmatic guidance, outlines, sections, search, and viewers
- [ ] 4.3 Add check-bundle examples that connect check definitions to passing and failing fixture behavior
- [ ] 4.4 Add integration tests that identify failures at root, group, and deeply nested command paths
- [ ] 4.5 Add non-interactive tests for closed input, plain output, offline execution, and viewer behavior
- [ ] 4.6 Add tests that default help serves people and agents and that `--programmatic` retains it while adding programmatic guidance

## 5. Document and Verify

- [ ] 5.1 Write a check-bundle authoring guide that explains `codekiln-help` from its bundle file through its checks, fixtures, and report results
- [ ] 5.2 Document the help commands, JSON fields, check-bundle installation, checker limits, and examples for CLI authors
- [ ] 5.3 Add links between the guide, `codekiln-help` source, checks, and tests
- [ ] 5.4 Run the experiment checks and record any result that changes the design
- [ ] 5.5 Run formatting, linting, tests, and OpenSpec validation through the project mise tasks
- [ ] 5.6 Verify the implementation against the proposal, design, and all delta specs before archive
