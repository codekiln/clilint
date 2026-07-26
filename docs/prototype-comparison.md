# Python 0.0.1 to Rust 0.0.2

The Rust rebuild retains the prototype's behavioral observations: arguments, exit status, timeout state, duration, stdout, stderr, and ANSI escape detection. Tested CLI tool stdin is closed unless a check-bundle invocation supplies input, and every invocation has a timeout.

## Check mapping

| Python 0.0.1 check | Rust 0.0.2 check |
| --- | --- |
| `CLI-BASICS-001` | `clilint/basics/success-exit` |
| `CLI-BASICS-002` | `clilint/basics/error-exit` |
| `CLI-BASICS-003` | `clilint/basics/help-on-stdout` |
| `CLI-ERROR-001` | `clilint/error/on-stderr` |
| `CLI-ERROR-002` | `clilint/error/names-problem` |
| `CLI-ERROR-003` | `clilint/error/points-to-help` |
| `CLI-HELP-001` | `clilint/help/long-option` |
| `CLI-HELP-002` | `clilint/help/short-option` |
| `CLI-HELP-003` | `clilint/help/usage` |
| `CLI-HELP-004` | `clilint/help/useful-example` |
| `CLI-OUTPUT-001` | `clilint/output/no-ansi-when-piped` |
| `CLI-ROBUST-001` | `clilint/robustness/fast-help` |
| `CLI-ROBUST-002` | `clilint/robustness/no-args-do-not-hang` |
| `CLI-VERSION-001` | `clilint/version/long-option` |
| `CLI-VERSION-002` | `clilint/version/short-option` |
| `CLI-AGENT-001` | `clilint/agent/structured-output` |
| `CLI-AGENT-002` | `clilint/agent/non-interactive` |

`CLI-HELP-004` previously passed whenever help contained the word `example`.
Its replacement is a judgment-based Check that distinguishes a useful task
example from a heading or placeholder. The other mapped Checks are
mechanistic.

## Public behavior changes

- `clilint check` is the 0.0.2 workflow. The prototype's `score` and `explain` subcommands, profiles, plain output, output-file option, and minimum-score gate are absent.
- Human and JSON reports give every completed Check a Score from `0.0` through
  `4.0` and focused Check Messages.
- Check identifiers are scoped by check bundle. The core bundle is embedded in the binary, and local extensions can add checks or strengthen inherited severity.
- JSON reports represent Check Result, Check Error, Awaiting Assessment, and
  Skipped as distinct outcomes.
- `--assessment` accepts repeatable JSON Assessment files. Clilint checks that
  each Assessment matches the Check, current evidence, and Skill, then
  validates its Score, messages, and format version.
- Built-in Checkers use Rust types. Each Check in a local bundle declares a
  Checker CLI, which can use any programming language or tools through the
  shared JSON exchange.
