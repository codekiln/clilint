# Clilint

[![GitHub Release](https://img.shields.io/github/v/release/codekiln/clilint)](https://github.com/codekiln/clilint/releases/latest)
[![CI](https://github.com/codekiln/clilint/actions/workflows/ci.yml/badge.svg)](https://github.com/codekiln/clilint/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Clilint lets people express reusable packages of testable expectations for command-line programs, so builders can state a standard once and check tools against it. Its built-in package is intended to grow into an opinionated superset of the [Command Line Interface Guidelines](https://clig.dev/): automate as many of those guidelines as possible, then add sensible defaults for newer uses such as agent-readable help.

The current release runs common help, version, error, and no-argument commands. Its built-in checks cover help and version options, exit codes, error messages, output for scripts, and color codes in piped output. A local extension package can add expectations for a project or team. Optional AI assessments handle questions that need judgment, and Clilint reports those results separately from repeatable checks.

Clilint is for people who build command-line tools and teams that want those tools to follow shared expectations. The longer-term [project direction](docs/vision.md) includes reusable preference packages and command-line help that people and agents can explore without leaving the terminal.

## Project status

Clilint is an early-stage `0.0.x` project. Command behavior, package files, and report formats may change before version 1.0.

Current releases support Apple silicon macOS, Intel macOS, x86-64 Linux, and x86-64 Windows. [GitHub Releases](https://github.com/codekiln/clilint/releases/latest) publishes the downloads and their SHA-256 checksums.

The [continuous integration workflow](https://github.com/codekiln/clilint/actions/workflows/ci.yml) runs the repository checks. [@codekiln](https://github.com/codekiln) maintains the project. Use [GitHub Issues](https://github.com/codekiln/clilint/issues) for help and bug reports.

## Install

With [mise](https://mise.jdx.dev/):

```sh
mise use -g github:codekiln/clilint
```

See [Installation](docs/installation.md) for every supported download, checksum verification, PATH setup, and troubleshooting.

## Check a command

Start by asking Clilint to check itself:

```sh
clilint check clilint
```

A current release reports results like these:

```text
CLI Lint clilint 0.0.2
Deterministic score: 98/100

clilint/agent/non-interactive   warn       deterministic assertion failed
clilint/help/long-option        pass       deterministic declared behavioral check passed
clilint/help/useful-example     unassessed ai-agent run the required skill to assess the captured evidence

Deterministic: 15 pass, 1 warn, 0 fail, 0 skip
AI agent: 0 pass, 0 warn, 0 fail, 0 skip, 1 unassessed
```

Each line names a rule and its result. `pass`, `warn`, and `fail` come from repeatable checks. `unassessed` means an optional AI assessment has not been attached.

Replace the final `clilint` with a command name or executable path:

```sh
clilint check my-cli
clilint check ./path/to/my-cli --format json
```

Clilint exits with code 1 when a repeatable check or attached AI assessment fails. Invalid commands, packages, or assessment files exit with code 2 and write an error to stderr.

## What Clilint checks

The built-in rules check whether a command:

- provides working `--help`, `-h`, `--version`, and `-V` options;
- returns useful exit codes without hanging;
- sends normal output and errors to the expected streams;
- names errors and points the user toward help;
- avoids color codes when output is piped; and
- advertises structured and non-interactive modes for automation.

Clilint also captures help text for an optional AI assessment of whether the help teaches a likely task with a useful example.

## Learn more

- [Install Clilint on each supported system](docs/installation.md)
- [Add project-specific checks with an extension package](docs/packages.md)
- [Run and attach AI assessments](docs/ai-assessments.md)
- [Read the project direction](docs/vision.md)
- [Contribute to Clilint](CONTRIBUTING.md)
- [Compare the Rust implementation with the Python prototype](docs/prototype-comparison.md)

## License

Clilint is available under the [MIT License](LICENSE).
