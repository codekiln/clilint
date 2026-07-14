# clilint

Clilint checks observable command-line behavior against a versioned conformance package. The 0.0.2 implementation is a Rust binary that runs package-declared invocations, captures evidence, evaluates deterministic rules, and exposes pending AI-agent assessments without combining the two methods into one measurement.

## Quick start

Install the project tools and build the binary:

```sh
mise install
cargo build
```

Check one of the included fixtures:

```sh
cargo run -- check ./tests/fixtures/useful-help-cli
cargo run -- check ./tests/fixtures/useful-help-cli --format json
```

Human output shows each finding and separate deterministic and AI-agent summaries. JSON output is one versioned report document on stdout. A deterministic or attached AI-agent failure exits 1; invalid input or package data exits 2 with a diagnostic on stderr.

## Conformance packages

The embedded global package is defined in [`packages/clilint/clilint.toml`](packages/clilint/clilint.toml). It contains package-scoped rule identifiers and typed invocations and assertions. Checks capture:

- arguments and environment changes;
- exit status and timeout state;
- duration, stdout, and stderr;
- ANSI escape presence.

Target stdin receives end-of-file unless an invocation explicitly supplies input. Deterministic checking does not fetch packages or use network services.

A local TOML package can extend `clilint`:

```toml
format_version = 1
extends = "clilint"

[package]
name = "team"
version = "1.0.0"

[[rules]]
id = "team/help/team-option"
title = "Help describes the team option"
severity = "warn"
evaluation_method = "deterministic"

[rules.check]
type = "invocation"
args = ["--help"]
assertions = [{ type = "stdout-contains-any", values = ["--team"] }]
```

Pass it to the check command with `--package ./team.toml`. Extensions add rules or strengthen inherited severity; attempts to exclude, replace, or weaken inherited rules are rejected before the target runs.

## Help assessment skill

The global package includes `clilint/help/useful-example`, an AI-agent rule for whether help teaches a likely task with a useful example. Without an assessment, its report result is `unassessed` and includes captured help, a required skill identity, and an evidence digest.

List and install the repository skill with the mise-managed Vercel Skills CLI:

```sh
skills add . --list
skills add . --skill assess-cli-help --agent codex -y --copy
```

The installed `assess-cli-help` skill runs clilint, judges only the captured help, writes a versioned assessment document, and asks clilint to validate and attach it. It does not execute examples found in help output.

Assessment documents can also be supplied directly and repeatedly:

```sh
cargo run -- check ./tests/fixtures/useful-help-cli \
  --assessment ./tests/fixtures/useful-help-assessment.toml \
  --format json
```

Clilint validates the document format, rule identifier, result, skill name and version, and evidence digest. A digest from changed evidence is rejected.

## Development

Mise pins Rust and the project development tools in `mise.lock`. Project automation uses executable files under `.mise/tasks/`:

```sh
mise tasks
mise run format
mise run lint
mise run test
mise run openspec:validate
mise run openspec:check-skill-versions
mise run rulesync:check
mise run ci
```

`mise run ci` is the same aggregate entry point used by GitHub Actions. It checks Rust formatting, runs Clippy with warnings denied, runs all tests, validates OpenSpec strictly, compares OpenSpec skill metadata with the managed CLI version, and checks generated RuleSync files.

## Repository layout

```text
src/                         Rust library and binary
packages/clilint/            Embedded global package
skills/assess-cli-help/      Installable help assessment skill
tests/fixtures/              Behavioral and skill fixtures
openspec/                    Project specifications and changes
.rulesync/                   Source for generated agent configuration
.mise/tasks/                 Local and CI task entry points
```

The Python 0.0.1 prototype remains available at tag `v0.0.1`. See [`docs/prototype-comparison.md`](docs/prototype-comparison.md) for the retained rule mapping and public behavior changes.

## License

[MIT](LICENSE)
