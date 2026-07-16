## Why

Issue #5 exposes a repository-wide communication problem, not just a weak README draft: the project has no shared writing standard or repeatable way for agents to evaluate whether documentation follows it. The repository needs a project-owned communication style, dedicated README assessment skills, and a README rewritten against both.

## What Changes

- Add a project communication style under RuleSync source, adapted from the public writing preferences and AI communication rules in `logseq-encode-garden`.
- Make the style apply to repository documentation and agent-authored communication, with additional requirements for the root README.
- Define progressive disclosure for both human and agent readers: present the main point and information needed for the current decision or task first, then provide a clear route to deeper detail when it becomes relevant.
- Add a RuleSync skill that evaluates README wording against the project communication style.
- Add a separate RuleSync skill that evaluates whether the README answers a first-time visitor's questions about purpose, value, trust, status, installation, first use, help, maintenance, contribution, and license.
- Require agents changing the README to run both assessments and resolve each reported problem or record why it was accepted.
- Generate the skills and communication rule for the configured RuleSync targets and check the generated files with the existing repository checks.
- Rewrite the README so its opening explains the problem Clilint solves, why its approach is useful, and who should care.
- Give the README one short, copyable installation example—preferably using mise—and a path to a first result in under five minutes.
- Add `docs/installation.md` for platform-specific installation, checksum verification, PATH setup, and troubleshooting.
- Move package formats, AI-assessment procedures, development commands, and historical implementation details to focused documentation when they are not needed for first use.

## Capabilities

### New Capabilities

- `repository-communication-style`: Defines the repository's writing standard, its RuleSync distribution, and the two skills that assess README style and purpose.
- `repository-readme`: Defines the information, order, examples, project-status signals, and first-use path required in the human-facing README.

### Modified Capabilities

None. These are new documentation and agent-review capabilities.

## Impact

- Adds RuleSync source under `.rulesync/rules/` and `.rulesync/skills/`, then updates generated agent configuration with `rulesync generate`.
- Rewrites the root `README.md` and adds focused files under `docs/`, including `docs/installation.md`, plus `CONTRIBUTING.md` for material removed from it.
- Adds a required agent review for future README changes without changing Clilint's command behavior, package formats, reports, or release automation.
- Uses the existing RuleSync consistency check to reject stale generated files; the semantic assessments run through the generated agent skills.

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
