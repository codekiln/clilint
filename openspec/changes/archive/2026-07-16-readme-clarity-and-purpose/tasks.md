## 1. Define the Communication Standard

- [x] 1.1 Add `.rulesync/rules/communication-style.md` by adapting the relevant public garden rules and preferences into a self-contained project standard with source links and concrete examples.
- [x] 1.2 Cover audience, information order, plain and specific language, uncertainty, sentence value, jargon, coined phrases, double negatives, figurative language, unnecessary contrasts, irrelevant implementation detail, tone, inclusion, progressive disclosure for human and agent readers, and the separation between agent instructions and human documents.
- [x] 1.3 Define progressive disclosure as giving the main point and currently needed information first, followed by a descriptive path to deeper detail; include human and agent examples and make clear that deferral must not make required information undiscoverable.
- [x] 1.4 Update `.rulesync/rules/overview.md` so agents must run both README assessment skills after changing `README.md` and resolve each reported problem or record why it was accepted.

## 2. Add the README Assessment Skills

- [x] 2.1 Add `.rulesync/skills/assess-readme-style/SKILL.md` with a sentence-level procedure, progressive-disclosure checks at document and section level, exact-line evidence, named criteria, separate findings and suggestions, an explicit verdict, and clear and unclear examples.
- [x] 2.2 Add `.rulesync/skills/assess-readme-purpose/SKILL.md` with checks for purpose, audience, the reason to choose Clilint, trust, maturity, one concise installation example, a direct link to complete installation guidance, five-minute first use, help, maintenance, contribution, license, organization, and appropriate links to deeper documentation.
- [x] 2.3 Run `rulesync generate`, inspect every configured target, and run the existing RuleSync consistency check to verify that generated rules and both skills match their source.
- [x] 2.4 Run both skills against the historical README from issue #5 and verify that their findings identify its wording and first-time-visitor failures without editing the file.
- [x] 2.5 Limit the OpenSpec skill-version check to OpenSpec-managed skills so custom RuleSync assessment skills do not need false `generatedBy` metadata.

## 3. Preserve Focused Guidance

- [x] 3.1 Create `docs/installation.md` that identifies the correct download for each supported operating system and processor type and explains checksums, extraction, PATH setup, and common problems; compare every named file with the current GitHub Release and test the local installation path.
- [x] 3.2 Create `docs/packages.md` from the current package-extension guidance, then validate its manifest example with Clilint's package parser.
- [x] 3.3 Create `docs/ai-assessments.md` from the current assessment-skill guidance, then run the documented skill discovery and assessment commands against the repository fixtures.
- [x] 3.4 Create `CONTRIBUTING.md` with repository setup and development checks, then run the documented mise task-list commands in a fresh or equivalent checkout.

## 4. Rewrite the Human Entry Point

- [x] 4.1 Rewrite the README opening to state who Clilint helps, what problem it finds, why that matters, and what makes its approach useful before any package, assessment, report, or implementation details.
- [x] 4.2 Add a plain project-status section and current links for releases, continuous integration, maintenance, help, and the MIT license; verify each claim against repository and GitHub state.
- [x] 4.3 Verify `mise use -g github:codekiln/clilint` against the current release assets. Put it in the README as the single installation example if it works reliably; otherwise use the shortest supported alternative, record the reason, and link directly to `docs/installation.md` for every platform-specific path.
- [x] 4.4 Show `clilint check clilint` as the first command to run after installation, include enough real output to explain what Clilint found, tell readers how to replace the final argument with their command, and time the installation-to-result path to ensure it finishes in under five minutes.
- [x] 4.5 Replace long package, AI-assessment, and development sections with short summaries and descriptive links to the focused documents; check every relative link from a local README render.
- [x] 4.6 Add concise help, maintainer, contribution, and license sections; verify that a visitor can find support and contributor setup without reading agent instructions.

## 5. Evaluate and Validate the Result

- [x] 5.1 Run `assess-readme-style` on the rewritten README and resolve each reported problem or record the accepted exception and reason.
- [x] 5.2 Run `assess-readme-purpose` on the rewritten README and resolve each reported problem or record the accepted exception and reason.
- [x] 5.3 Verify that both assessments treat a concise installation example plus a descriptive `docs/installation.md` link as progressive disclosure, report platform-by-platform README instructions as premature detail, and report deferred information that has no findable path.
- [x] 5.4 Evaluate the README with LintMe as an independent advisory check and inspect each finding in the context of the project standard.
- [x] 5.5 Run every README command available on the development platform and check external links, release links, badges, headings, code blocks, and relative paths.
- [x] 5.6 Run `mise run ci`, strict OpenSpec validation, and the RuleSync consistency check, then review the final diff to ensure generated files came from `.rulesync/` source and application behavior did not change.
