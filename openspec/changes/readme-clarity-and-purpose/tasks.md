## 1. Preserve Focused Guidance

- [ ] 1.1 Create `docs/packages.md` from the current package-extension guidance, then validate its manifest example with Clilint's package parser.
- [ ] 1.2 Create `docs/ai-assessments.md` from the current assessment-skill guidance, then run the documented skill discovery and assessment commands against the repository fixtures.
- [ ] 1.3 Create `CONTRIBUTING.md` with repository setup and development checks, then run the documented mise setup or task-list commands in a fresh or equivalent checkout.

## 2. Rewrite the Human Entry Point

- [ ] 2.1 Rewrite the README opening to state who Clilint helps, what problem it finds, and why that matters; verify that package, assessment, report, and implementation details do not precede those answers.
- [ ] 2.2 Add a plain project-status section and current links for releases, continuous integration, maintenance, help, and the MIT license; verify each claim against repository and GitHub state.
- [ ] 2.3 Add installation instructions for the macOS, Linux, and Windows release archives and their checksums; test the local-platform path and compare every named archive with the current release configuration and published assets.
- [ ] 2.4 Add a first-use example based on `clilint check clilint`, capture representative output from the built release behavior, and explain how to check another command and read the main result categories.
- [ ] 2.5 Replace the long package, AI-assessment, and development sections with short summaries and descriptive links to the focused documents; check every relative link from a local README render.
- [ ] 2.6 Add concise help, maintainer, contribution, and license sections; verify that a visitor can find an issue-reporting path and the contributor setup without reading agent instructions.

## 3. Check the Finished Documentation

- [ ] 3.1 Run every README command available on the development platform and check external links, release links, badges, headings, code blocks, and relative paths.
- [ ] 3.2 Review each README sentence for a likely visitor question, replacing unexplained jargon, vague claims, unnecessary contrasts, hard-coded release prose, and implementation details that belong in another document.
- [ ] 3.3 Evaluate the README with LintMe as an advisory check, inspect each finding in context, and resolve findings that improve this README without treating a generic score as acceptance criteria.
- [ ] 3.4 Compare the rendered README with every `repository-readme` requirement and with issue #5's questions: purpose, value, project status, trust, first use, help, maintenance, contribution, and license.
- [ ] 3.5 Run `mise run ci` and strict OpenSpec validation, then review the final diff to ensure the change affects documentation only.
