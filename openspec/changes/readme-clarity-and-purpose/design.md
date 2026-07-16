## Context

The current README demonstrates recurring failures in AI-authored documentation: it begins with internal terminology, uses long sentences that join unrelated ideas, states implementation details before user value, and explains what the system does not do when the reader has not asked. Issue #5 describes the intended reader questions and links research and guidance about README structure, style, linting, and the separation between human documentation and agent instructions.

The relevant communication preferences already exist publicly in `logseq-encode-garden`, but this repository does not load them as project instructions. A future agent can therefore repeat the same writing failures. The project already uses RuleSync as the source for agent rules and skills, with `agentsskills`, `claudecode`, and `codexcli` as configured targets.

## Goals / Non-Goals

**Goals:**

- Establish a repository-owned communication style adapted from the relevant garden rules and preferences.
- Apply the style to repository documentation and agent-authored prose, with specific guidance for README files.
- Provide two focused RuleSync skills that evaluate README style and README purpose independently.
- Make each assessment produce evidence tied to README lines and named criteria.
- Require both assessments after README changes and keep their generated copies current.
- Rewrite the README so a supported user can understand the project and reach a first result in under five minutes without making the README an installation manual.

**Non-Goals:**

- Copy every personal garden rule into this repository regardless of relevance.
- Turn subjective prose quality into a numeric score or claim that an agent review removes the need for human judgment.
- Run an AI model inside continuous integration as part of this change.
- Apply prose rules to Rust formatting, identifiers, or other source-code conventions.
- Put agent authoring instructions into the human-facing README.
- Change Clilint's command behavior, package formats, reports, or release process.

## Decisions

### Keep the communication style in RuleSync source

The project-owned standard will live at `.rulesync/rules/communication-style.md`. It will adapt the relevant garden material into instructions that stand on their own in this repository while linking to the public source pages for context.

The standard will cover:

- writing for an intelligent reader who may not share the author's background;
- putting the main point first and ordering information by reader need;
- using common, specific words and defining necessary technical terms;
- using active voice and sentences that carry one clear idea;
- removing jargon, coined phrases, word salad, double negatives, needless metaphors, vague claims, unnecessary contrasts, and irrelevant implementation details;
- stating uncertainty without exaggeration;
- using inclusive language and keeping the tone calm;
- requiring every sentence to answer a likely reader question or reduce uncertainty;
- using progressive disclosure for human and agent readers;
- keeping authoring instructions in agent rules rather than inserting them into the document being written; and
- applying only rules that relate to the document at hand.

Copying and adapting the useful material makes this repository self-contained. Referencing the garden without a local standard would leave agents dependent on external context. Copying the full namespace without editing would import personal or unrelated material that does not belong in this project.

### Define progressive disclosure for human and agent readers

The communication standard will define progressive disclosure as presenting the main point and the information needed for the reader's current decision or task first, then providing a clear path to deeper detail when it becomes relevant. It is not an excuse to omit necessary information: later detail must remain findable through descriptive links, references, or the next layer of instructions.

For a human reader, this means a README can show one useful installation path and link to a complete installation guide instead of placing every platform procedure in the entry point. Within a guide, common steps precede special cases and troubleshooting.

For an agent reader, this means initial project instructions provide the rules needed to select and begin the task, then direct the agent to load the relevant skill, namespace, specification, or reference before acting. Agents should not copy all loaded instructions into the artifact or burden the user with background that does not affect the current decision.

This principle applies at document, section, and sentence level. The style assessment will identify detail revealed before it is useful or deeper information that has no findable path. The purpose assessment will evaluate the overall visitor journey. This division preserves the distinction between local communication quality and missing or misplaced README content.

### Use two README assessment skills

RuleSync source will define these skills:

- `.rulesync/skills/assess-readme-style/SKILL.md` checks the README sentence by sentence against the communication style.
- `.rulesync/skills/assess-readme-purpose/SKILL.md` checks the README as a first-time visitor journey: what the tool is, why it is useful, what distinguishes its approach, whether it is ready for attention, whether it offers a concise installation example and a clear route to complete installation guidance, how to try it in under five minutes, and where to find help, maintenance, contribution, and license information.

Each skill will read the current `README.md`, cite exact lines or headings, name the applicable criterion, explain the effect on the reader, and report problems separately from optional suggestions. A passing result means the skill found no unresolved problem. The skills evaluate and report; they edit only when a user separately asks for changes.

Two skills keep wording problems separate from missing or misplaced information. One combined skill would make it harder to tell whether a failure belongs to a sentence or to the document's structure. More narrowly divided skills would add coordination cost without creating a clearer review boundary.

### Make the assessment process part of generated project instructions

The RuleSync overview will instruct agents to run both README skills after changing `README.md`. Reported problems must be fixed or recorded as accepted exceptions with a reason. `rulesync generate` will update the configured agent targets, and the existing RuleSync check will reject stale generated copies.

This provides a repeatable agent review. Continuous integration will verify the RuleSync sources and generated files, but it will not call an AI agent. Adding unattended AI review to CI would require model selection, credentials, cost controls, and a stable machine-readable result contract; that can be considered separately.

Each skill will contain a small clear and unclear example with expected findings. During implementation, both skills will be exercised against the historical README identified by issue #5 and against the rewritten README. LintMe remains a useful independent review, but it does not replace the project-specific skills.

### Put visitor questions before implementation details

The README will use this order:

1. Project name, a one-sentence description, and useful status links or badges.
2. Who Clilint helps, what problem it finds, and what makes its approach useful.
3. Current project maturity and supported release platforms.
4. One short, copyable installation example and first-check path designed to finish in under five minutes, followed by a link to complete installation documentation.
5. A short terminal example and a list of the main user-visible checks.
6. Links for extension packages, AI assessments, help, contributing, maintenance, and the license.

This order answers the questions a visitor needs before deciding to adopt the project. Package authoring and AI-assessment details will move under `docs/`; contributor setup and development checks will move to `CONTRIBUTING.md`.

### Keep installation details out of the README

The README will show one copyable installation example using mise, preferably `mise use -g github:codekiln/clilint`, after the command has been verified against the published release assets. It will then link to `docs/installation.md`. If the mise GitHub backend cannot install the published assets reliably, the README will use the shortest supported alternative and the implementation will record why.

`docs/installation.md` will provide the details for every currently supported download: Apple silicon macOS, Intel macOS, x86-64 Linux, and x86-64 Windows. For each system, it will name the file to download and explain checksum verification, extraction, PATH setup, and common installation problems. Keeping this detail in a focused guide lets the README help a new visitor try the tool without forcing every reader through instructions for other systems.

The first check will run Clilint against `clilint` itself, then show how to substitute another command. A short terminal transcript is the useful visual for this CLI because it shows the command and result in accessible, searchable text. A decorative image would increase maintenance without helping the reader complete a task.

## Risks / Trade-offs

- [The project copy diverges from the garden] → Treat the repository copy as its own standard, retain source links, and update it deliberately when a garden change is relevant here.
- [Agent assessments vary between runs] → Use named criteria, line evidence, clear severity, examples, and human review instead of a numeric score.
- [Generated RuleSync files add review noise] → Edit only `.rulesync/` source, run `rulesync generate`, and verify generated files with the existing check.
- [The two skills report overlapping findings] → Assign wording to the style skill and missing, misplaced, or incomplete information to the purpose skill.
- [Progressive disclosure becomes an excuse to omit information] → Require a descriptive path to the deeper layer and test both premature detail and unreachable detail.
- [The mise example does not work with a release asset or platform] → Verify it against the current releases, link the complete platform guide beside it, and use the shortest supported alternative if necessary.
- [Manual invocation can be skipped] → Put the dual-assessment requirement in generated project instructions and verify it during OpenSpec review.

## Migration Plan

1. Add the communication style and README assessment skills under `.rulesync/`.
2. Generate and inspect the configured RuleSync targets.
3. Run both skills against the historical README to establish that they detect the issue's failures.
4. Move detailed installation, reference, and contributor material out of the README.
5. Rewrite the README and test its concise installation example and five-minute first-use path.
6. Run both project skills and LintMe against the rewritten README, resolve findings, and run repository validation.

## Citations

- [My/Principle/Favor Readers Over Writers](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Favor%20Readers%20Over%20Writers.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Pref/Writing/Use Plain language](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20Plain%20language.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
- [My/Pref/Writing/Be specific and explicit](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Be%20specific%20and%20explicit.md)
- [My/Pref/Writing/Avoid Jargon, Especially Tech-speek](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Avoid%20Jargon%2C%20Especially%20Tech-speek.md)
- [My/Pref/Writing/Avoid double negatives](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Avoid%20double%20negatives.md)
- [My/Pref/Writing/Avoid Distractors such as Awkward or Superfluous Metaphors](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Avoid%20Distractors%20such%20as%20Awkward%20or%20Superfluous%20Metaphors.md)
- [My/Pref/Writing/Don't be an Attention Vampire; Lower the Drama](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Don%27t%20be%20an%20Attention%20Vampire%3B%20Lower%20the%20Drama.md)
- [My/Pref/Writing/Seek Inclusive Language](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Seek%20Inclusive%20Language.md)
