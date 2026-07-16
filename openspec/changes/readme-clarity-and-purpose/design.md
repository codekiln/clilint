## Context

The root README is the first project document most GitHub visitors see. Its current opening describes internal implementation and report concepts before it explains the practical reason to use Clilint. The quick start builds the repository and checks a test fixture, so it serves a contributor better than someone evaluating the released tool.

Clilint currently publishes checksummed archives for macOS, Linux, and Windows. Its main user action is `clilint check <command>`. The repository also contains detailed material about extension packages, AI-agent assessments, development tasks, release automation, and the earlier Python prototype. Those details remain useful after a reader understands the project and decides to use or contribute to it.

The guidance collected in issue #5 points to the same order of reader needs: explain the project and its value, show its status, provide a short path to first use, and link to deeper documentation. It also distinguishes the human audience of `README.md` from the agent instructions in `AGENTS.md`.

## Goals / Non-Goals

**Goals:**

- Help a first-time visitor understand Clilint's purpose and value from the opening.
- State the project's maturity and available releases without tying the description to one release number.
- Give macOS, Linux, and Windows users a supported path from a release archive to their first check.
- Show a representative result and explain what the main result categories mean.
- Keep advanced and contributor information available through focused links.
- Make claims, commands, examples, and links checkable during implementation.

**Non-Goals:**

- Change Clilint's command behavior, report formats, checks, package formats, or release process.
- Turn the README into a full command, package, or assessment reference.
- Copy agent-specific project instructions into the README.
- Add a general-purpose documentation linter or require a subjective score for all future README changes.
- Add promotional claims that the repository cannot support with current behavior or project evidence.

## Decisions

### Put visitor questions before implementation details

The README will use this order:

1. Project name, a one-sentence description, and a small set of useful status links or badges.
2. A short explanation of who Clilint helps and what problem it finds.
3. A plain statement of current project maturity and supported release platforms.
4. Installation and a first check.
5. A short example of the result and the main checks Clilint performs.
6. Links for extension packages, AI assessments, help, contributing, maintenance, and the license.

This order answers the questions a new visitor is likely to have before presenting details they need only after adoption. Keeping the existing implementation-first order would preserve more technical context near the top, but it would leave the issue's central problem in place. A long README containing every detail was also considered; focused documents make that information easier to find without delaying first use.

### Use existing release archives as the supported installation path

The installation section will link to the latest release and identify the existing archives for Apple silicon macOS, Intel macOS, x86-64 Linux, and x86-64 Windows. It will give short extraction and PATH instructions for Unix-like systems and Windows. It will also point to checksums for readers who want to verify a download.

The first copyable check will run Clilint against `clilint` itself, followed by the form for checking the reader's own command. This avoids requiring a repository checkout or a test fixture. Building from source will remain contributor documentation because the project already publishes release binaries. Adding generated installers would simplify installation further, but that changes release behavior and belongs in a separate change.

### Show evidence instead of describing the implementation

The README will show a short terminal example of a real check and summarize the user-visible checks in a list. It will explain deterministic results and pending AI assessment only to the extent needed to read that example. Rust modules, typed data structures, package resolution, digest validation, and release history will stay in focused documents.

A terminal transcript is more useful for this command-line project than a decorative image. It shows the command, the result, and the tool's vocabulary in text that remains accessible, searchable, and easy to update.

### Separate first-use, reference, and contributor material

Package authoring and AI-assessment details will move into focused files under `docs/`, with short links from the README. Contributor setup and the full list of development checks will move to `CONTRIBUTING.md`. Existing release and prototype documents will remain linked only where they answer a reader's question.

Keeping all current sections in the README would reduce the number of files but would continue to mix adoption, reference, history, and maintenance information. Removing the details without preserving links would make the project harder to use after the first check.

### Use focused checks for this change

Implementation will run every documented command that is available on the development platform, inspect each link, compare claims with the current CLI and release configuration, and review each section against the issue's reader questions. LintMe may be used as an advisory review, with human judgment deciding which findings apply to this project.

A permanent generic prose score or AI review gate is out of scope. The research cited by the issue emphasizes context-specific checks and author control, while the repository does not yet have an agreed set of stable automated README rules. A later change can add repeatable checks after this rewrite identifies rules that are specific enough to test.

## Risks / Trade-offs

- [Release instructions become stale] → Link to the latest release, avoid release numbers in general prose, and name only platforms produced by the current release configuration.
- [Platform-specific installation steps become long] → Keep each path short and put checksum or troubleshooting details behind links when possible.
- [Moving details makes features harder to discover] → Keep a short feature summary in the README and use descriptive links to the focused documents.
- [A concise status statement understates project limits] → State the `0.0.x` maturity and interface stability plainly near the first-use instructions.
- [Manual language review permits later regressions] → Record concrete acceptance checks in the spec and tasks; consider automated enforcement separately after useful rules are known.

## Migration Plan

1. Create the focused reference and contribution documents from useful material in the current README.
2. Rewrite the README in the selected order and link to those documents.
3. Run the documented first-use commands, check links and current release facts, and review the rendered Markdown.
4. Revert the documentation changes if the new path cannot be completed with the current release artifacts; no data or software migration is required.

## Citations

- [My/Principle/Favor Readers Over Writers](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Favor%20Readers%20Over%20Writers.md)
- [My/Principle/Dispel Ambiguity](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Principle___Dispel%20Ambiguity.md)
- [My/Pref/Writing/Use Plain language](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20Plain%20language.md)
- [My/Pref/Writing/Avoid Jargon, Especially Tech-speek](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Avoid%20Jargon%2C%20Especially%20Tech-speek.md)
- [My/Pref/Writing/Use the simpler word](https://github.com/codekiln/logseq-encode-garden/blob/main/pages/My___Pref___Writing___Use%20the%20simpler%20word.md)
