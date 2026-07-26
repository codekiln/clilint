# Clilint

[![GitHub Release](https://img.shields.io/github/v/release/codekiln/clilint)](https://github.com/codekiln/clilint/releases/latest)
[![CI](https://github.com/codekiln/clilint/actions/workflows/ci.yml/badge.svg)](https://github.com/codekiln/clilint/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Clilint checks the behavior of command-line programs. It runs a program,
scores each expectation from `0.0` through `4.0`, and gives focused messages
that a person or coding agent can use to improve the program.

The built-in checks cover common help, version, error, output, and
non-interactive behavior. Projects can install local check bundles for
additional standards. Each bundle-owned check is a CLI, so its author can use
any programming language, scripts, tools, or AI agent harness behind the same
request and result format.

Clilint is for people and teams building command-line tools, especially when a
coding agent needs specific feedback rather than a single pass or fail.

## Project status

Clilint is an early-stage `0.0.x` project. Command behavior, check-bundle
files, and report formats may change before version 1.0.

Current releases support Apple silicon macOS, Intel macOS, x86-64 Linux, and
x86-64 Windows. [GitHub Releases](https://github.com/codekiln/clilint/releases/latest)
provides downloads and SHA-256 checksums. The
[continuous integration workflow](https://github.com/codekiln/clilint/actions/workflows/ci.yml)
runs the repository checks. [@codekiln](https://github.com/codekiln)
maintains the project. Use [GitHub Issues](https://github.com/codekiln/clilint/issues)
for help and bug reports.

## Install

With [mise](https://mise.jdx.dev/):

```sh
mise use -g github:codekiln/clilint
```

See [Installation](docs/installation.md) for other supported systems,
checksum verification, PATH setup, and troubleshooting.

## Check a command

Start by checking Clilint:

```sh
clilint check clilint
```

A report contains one outcome for each check:

```text
Clilint clilint 0.0.2
Target: clilint

clilint/help/long-option                    mechanistic      Score 4.00
clilint/help/useful-example                 judgment-based   Awaiting Assessment

16 results, 0 Check Errors, 1 awaiting Assessment, 0 skipped
0 Info, 0 Warning, 0 Error Check Messages
```

A completed Check Result has a Score. A Score below `4.0` includes a Check
Message explaining what could improve. A Check Error means the checker could
not produce a valid result; it has no Score.

Replace the final argument with your command name or executable path:

```sh
clilint check my-cli
clilint check ./path/to/my-cli --format json
```

Clilint exits with code 1 for a Check Error or an Error Check Message. A lower
Score alone does not determine the exit code. Invalid commands, bundles, and
Assessment files exit with code 2 and write an error to standard error.

## Add project-specific checks

Install a local check bundle from the project directory:

```sh
clilint bundle install ../my-checks
clilint check ./my-cli
```

Clilint records the relative path in `.clilint/config.toml`. The
[`codekiln-help` bundle](docs/codekiln-help.md) is a complete example that
checks navigable help at every command path.

## Learn more

- [Install Clilint on each supported system](docs/installation.md)
- [Install and author check bundles](docs/check-bundles.md)
- [Implement the `codekiln-help` interface](docs/codekiln-help.md)
- [Run and attach judgment-based Assessments](docs/ai-assessments.md)
- [Read the project direction](docs/vision.md)
- [Contribute to Clilint](CONTRIBUTING.md)

## License

Clilint is available under the [MIT License](LICENSE).
