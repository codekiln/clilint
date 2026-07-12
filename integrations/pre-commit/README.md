# clilint pre-commit hook

Run [clilint](https://github.com/codekiln/clilint) as a
[pre-commit](https://pre-commit.com) hook to check a CLI executable for
conformance on every commit.

## Requirements

The hook uses `language: system`, so `clilint` must be resolvable on your
`PATH`. Either install clilint so that `clilint` is a command, or wrap the
invocation so it points at your checkout (for example a small shell script on
`PATH` that runs `python /path/to/clilint/clilint "$@"`).

## Usage

Add the hook to your repository's `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: https://github.com/codekiln/clilint
    rev: v1.0.0  # pin to a released tag or commit
    hooks:
      - id: clilint
        args: ["./my-cli", "--min-score", "90"]
```

## Notes

- clilint lints an executable target, not the files that changed in the commit,
  so the hook sets `pass_filenames: false`. You must pass the target as the
  first entry in `args`.
- Everything after the target is forwarded to `clilint check` — e.g.
  `--min-score`, `--profile`, or `--format`.
- Because the hook always lints the same target regardless of what changed, you
  may want to run it in a manual or push stage rather than on every commit:

  ```yaml
      - id: clilint
        args: ["./my-cli", "--min-score", "90"]
        stages: [pre-push]
  ```
