# Contributing to Clilint

Clilint welcomes bug reports, documentation improvements, and focused code changes. Open an [issue](https://github.com/codekiln/clilint/issues) before starting a change that would alter command behavior, package formats, reports, or releases.

## Set up the repository

Install [mise](https://mise.jdx.dev/getting-started.html) and clone the repository. From the checkout, run:

```sh
mise trust
mise install
```

Mise installs the pinned Rust toolchain and project tools from `mise.toml` and `mise.lock`.

## Run the checks

List the available project tasks:

```sh
mise tasks
```

Run the same checks used by GitHub Actions:

```sh
mise run ci
```

For a shorter feedback loop, run individual tasks:

```sh
mise run format
mise run lint
mise run test
```

Integration fixtures live under [`tests/fixtures/`](tests/fixtures/). Add or update a fixture when a behavior change needs a stable target command.

## Planned changes

This repository uses [OpenSpec](https://github.com/Fission-AI/OpenSpec) for planned changes. Keep implementation consistent with the active change under `openspec/changes/`, and run:

```sh
mise run openspec:validate
```

Project agent configuration is generated from `.rulesync/`. Edit only the source under `.rulesync/`, run `rulesync generate`, and verify it with:

```sh
mise run rulesync:check
```

## Pull requests

Keep a pull request focused on one change. Explain the behavior a reviewer can observe and include the commands used to test it. GitHub Actions runs `mise run ci` on every pull request.

Release maintainers can find the release process in [`docs/releases.md`](docs/releases.md).
