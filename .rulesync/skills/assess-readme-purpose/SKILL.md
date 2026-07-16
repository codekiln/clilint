---
name: assess-readme-purpose
description: Evaluate whether README.md gives a first-time visitor the information needed to understand, trust, install, and try Clilint, with links to focused details. Use after changing the README or when asked whether it serves its purpose.
targets: ["agentsskills", "claudecode", "codexcli"]
license: MIT
---

# Assess README purpose

Evaluate the root `README.md` as a first-time visitor's path from discovery to first use. Report findings; do not edit the file unless the user separately asks for changes.

## Procedure

1. Read `README.md` with line numbers.
2. Check each question against repository evidence rather than expectation:
   - What does Clilint check?
   - Who is it for?
   - Why do these checks matter?
   - Why would someone choose Clilint's approach?
   - What is the project's maturity?
   - Is there a current release, and which operating systems and processors does it support?
   - Where can a reader verify continuous integration, maintenance, help, and the license?
   - Does the README show one concise, copyable installation example?
   - Does it link directly and descriptively to `docs/installation.md` for every other supported path and verification detail?
   - Can a reader run `clilint check <command>` without a repository checkout, understand the shown result, and substitute their own command?
   - Is the quick path designed to finish in under five minutes? If the text alone cannot establish timing, record uncertainty and request execution evidence.
   - Can a reader find focused guidance for packages, AI assessments, and contribution?
3. Check the order. Purpose, audience, value, status, installation, and first use must precede reference, development, and historical detail.
4. Apply progressive disclosure. A concise installation example plus a descriptive installation-guide link is complete. System-by-system procedures in the README are premature detail. Deferred information with no findable path is missing information.
5. Verify commands, claims, and links against repository files and current release evidence when available. State uncertainty when verification is unavailable.

## Output

Use this structure:

```text
Verdict: PASS | FAIL

Problems
- README.md:<line or heading> — <visitor question>: <missing, misplaced, or unsupported information>. <effect on the visitor>. <required correction direction>.

Suggestions
- README.md:<line or heading> — <optional improvement>.

Uncertainty
- <claim or timing that available evidence cannot establish>.
```

Write `None.` under an empty section. `PASS` means there are no unresolved problems; suggestions do not cause failure. Do not use a numeric score.

## Examples

Incomplete entry point:

> Clilint is written in Rust and emits a typed report document.

Expected findings: purpose, audience, value, and reason to choose Clilint remain unanswered. Implementation information appears before visitor questions.

Excessive installation section:

> Download these files and follow these separate extraction procedures for macOS, Linux, and Windows.

Expected finding: organization and progressive disclosure. Keep one supported example in the README and link to `docs/installation.md` for complete system-specific instructions.

Complete first-use path:

> Install with `mise use -g github:codekiln/clilint`, then run `clilint check clilint`. For other systems and verification, see [Installation](docs/installation.md).

Expected result: the installation structure passes if the command works, the linked guide covers every supported download, and nearby text explains the first result and how to check another command.
