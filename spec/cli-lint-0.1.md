# CLI Lint 0.1

CLI Lint is a versioned, testable standard for command-line tools. Conformance is measured by the `clilint` reference linter, which runs behavioral probes against an executable and produces a 0-100 score plus a set of findings. The standard distills best practice from [clig.dev](https://clig.dev/) (Command Line Interface Guidelines) and ["12 Factor CLI Apps" by Jeff Dickey](https://jdxcode.medium.com/12-factor-cli-apps-dd3c227a0e46) into explicit, checkable criteria.

## How conformance is measured

Each rule has a severity that sets its score weight:

- `error` = 3
- `warn` = 1
- `info` = 0.5

When the linter evaluates a rule, the rule resolves to one of four outcomes:

- **pass** — earns full weight.
- **warn** — earns half weight.
- **fail** — earns zero weight.
- **skip** — excluded from the total; it contributes to neither earned nor applicable weight.

The overall score is:

```
score = round(100 * earned / total applicable weight)
```

`clilint check` exits non-zero if any rule fails or if the score is below the `--min-score` threshold.

## Profiles

- **`generated-cli`** (default) — enforces all 17 rules, including the agent-oriented ones. Intended for CLIs that are newly written or modified, often by an agent.
- **`modern-cli`** — enforces all rules except the two agent rules, `CLI-AGENT-001` and `CLI-AGENT-002`. Intended for general human-facing CLIs.

## Rules

### Basics

#### CLI-BASICS-001 — A successful invocation exits 0
**Severity:** error
**Check:** A successful run such as `--help` or `--version` exits with code 0.
**Remediation:** Return exit code 0 from successful runs; reserve non-zero for failures.

#### CLI-BASICS-002 — Errors exit non-zero
**Severity:** error
**Check:** Passing an unknown flag results in a non-zero exit code.
**Remediation:** Use an argument parser that rejects unknown flags and exits non-zero.

#### CLI-BASICS-003 — Help and primary output go to stdout
**Severity:** error
**Check:** `--help` prints to stdout (not stderr).
**Remediation:** Print help and normal results to stdout; send logs/errors to stderr.

### Help

#### CLI-HELP-001 — Responds to --help with help on stdout
**Severity:** error
**Check:** `--help` exits 0 with non-empty stdout.
**Remediation:** Implement `--help` to print full usage to stdout and exit 0.

#### CLI-HELP-002 — Responds to -h as a help alias
**Severity:** warn
**Check:** `-h` exits 0 with non-empty stdout.
**Remediation:** Map `-h` to the same behavior as `--help`.

#### CLI-HELP-003 — Help includes a usage synopsis
**Severity:** warn
**Check:** Help output contains a "Usage:" or "Synopsis:" line.
**Remediation:** Include a usage/synopsis line in help output.

#### CLI-HELP-004 — Help shows examples
**Severity:** warn
**Check:** Help output contains an "Examples" section.
**Remediation:** Add an Examples section with a few real invocations.

### Version

#### CLI-VERSION-001 — Responds to --version with a version number
**Severity:** warn
**Check:** `--version` exits 0 and output matches a version pattern like `1.2`.
**Remediation:** Implement `--version` to print a version like `1.2.0` and exit 0.

#### CLI-VERSION-002 — Responds to -V as a version alias
**Severity:** info
**Check:** `-V` exits 0 and prints a version number.
**Remediation:** Map `-V` to the same behavior as `--version`.

### Output

#### CLI-OUTPUT-001 — Color is disabled when output is not a TTY
**Severity:** error
**Check:** No ANSI color escape sequences appear in piped (non-TTY) output of `--help`, no-args, or `--version`.
**Remediation:** Emit color only when stdout is a TTY; honor NO_COLOR and a --no-color flag.

### Error

#### CLI-ERROR-001 — Error messages go to stderr
**Severity:** error
**Check:** An invalid flag produces non-empty output on stderr.
**Remediation:** Write all error/diagnostic messages to stderr.

#### CLI-ERROR-002 — Errors name the problem
**Severity:** warn
**Check:** The error output names the problem (e.g. mentions the offending flag, or words like unknown/unrecognized/invalid/error/usage).
**Remediation:** Include the offending input and a human-readable reason.

#### CLI-ERROR-003 — Errors point the user toward help
**Severity:** warn
**Check:** The error output references help or usage (e.g. --help, -h, "usage", "try").
**Remediation:** On usage errors, print a usage line or suggest running --help.

### Robustness

#### CLI-ROBUST-001 — Responds quickly to --help
**Severity:** warn
**Check:** `--help` completes in under 2000ms.
**Remediation:** Avoid loading heavy dependencies before parsing args; keep --help fast.

#### CLI-ROBUST-002 — Does not hang when run with no arguments
**Severity:** error
**Check:** Run with no arguments and a closed stdin, the tool exits rather than blocking/timing out.
**Remediation:** When required input is missing, print help or an error and exit; do not block on a TTY read.

### Agent

#### CLI-AGENT-001 — Advertises a machine-readable output mode
**Severity:** warn
**Check:** `--help` mentions a structured-output flag such as --json, --format, --plain, or --output/-o.
**Remediation:** Offer --json (or --format json) and keep it stable across releases.

#### CLI-AGENT-002 — Advertises a non-interactive mode
**Severity:** warn
**Check:** `--help` mentions an unattended flag such as --no-input, --non-interactive, --yes, --force, --batch, or -y.
**Remediation:** Provide a way to skip prompts and never require interactive input.

## Sources

- Command Line Interface Guidelines — https://clig.dev/
- 12 Factor CLI Apps (Jeff Dickey) — https://jdxcode.medium.com/12-factor-cli-apps-dd3c227a0e46
