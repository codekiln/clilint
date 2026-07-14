## 1. Preserve the prototype

- [x] 1.1 Align the Python prototype's displayed tool and standard versions with 0.0.1 and verify its existing end-to-end tests.
- [x] 1.2 Create the reviewed 0.0.1 snapshot commit and annotated `v0.0.1` tag before removing prototype files.

## 2. Establish the project workflow

- [x] 2.1 Complete `mise.toml` with the Rust and development tools, generate and commit `mise.lock`, and verify a fresh `mise install`.
- [ ] 2.2 Add executable, described `.mise/tasks/` file tasks for formatting, linting, tests, OpenSpec validation, RuleSync checks, and aggregate CI; add `#USAGE` metadata wherever a task accepts input.
- [ ] 2.3 Add CI that installs tools through the pinned mise GitHub Action and runs the same aggregate mise task used locally.
- [ ] 2.4 Add a workflow check that compares each OpenSpec source skill's `generatedBy` metadata with the mise-managed OpenSpec CLI version.
- [ ] 2.5 Verify `rulesync generate --check` reproduces the project-local OpenSpec skills and agent rules from `.rulesync/` source.

## 3. Establish the Rust core

- [ ] 3.1 Create a Rust 0.0.2 binary crate with the agreed source, package, skill, test, and OpenSpec layout.
- [ ] 3.2 Implement command parsing, target execution with closed standard input and timeouts, and focused unit tests.
- [ ] 3.3 Define typed observations, deterministic checks, findings, and report data with serialization tests.
- [ ] 3.4 Implement JSON and human report output and verify stdout, stderr, and exit behavior end to end.

## 4. Implement conformance packages

- [ ] 4.1 Define and test the versioned TOML package manifest and package-scoped rule identifiers.
- [ ] 4.2 Create `packages/clilint/clilint.toml` and bundle it into the binary with a parse-consistency test.
- [ ] 4.3 Implement local extension loading, additive inheritance, and errors for weakening or conflicting rules.
- [ ] 4.4 Re-express the useful Python rules as supported deterministic checks and verify them against Rust fixtures.
- [ ] 4.5 Verify deterministic checking succeeds with network access unavailable.

## 5. Implement AI-agent assessments

- [ ] 5.1 Add the help-quality rule, captured evidence, unassessed result, and required-skill metadata to the global package and report.
- [ ] 5.2 Define the versioned assessment document and implement validation of rule, result, skill, and evidence digest.
- [ ] 5.3 Add repeatable `--assessment` input to `clilint check` and verify valid, malformed, and stale assessments.
- [ ] 5.4 Create `skills/assess-cli-help/SKILL.md` with the clilint evidence-and-assessment workflow.
- [ ] 5.5 Add contrasting useful-help and superficial-example fixtures and forward-test the skill against both.
- [ ] 5.6 Verify the Vercel Skills CLI discovers and can install `assess-cli-help` from a local checkout.

## 6. Complete the rebuild

- [ ] 6.1 Compare the Rust behavior and retained standard content with the Python prototype and record intentional differences.
- [ ] 6.2 Remove superseded Python, probe, prompt, schema, fixture, integration, and root spec files after their retained content has a destination.
- [ ] 6.3 Run the aggregate mise CI task covering formatting, Clippy with warnings denied, unit tests, integration tests, fixture tests, OpenSpec strict validation, and RuleSync checks.
- [ ] 6.4 Update the project overview for the Rust package, mise task, and skill workflow and verify every documented command.
