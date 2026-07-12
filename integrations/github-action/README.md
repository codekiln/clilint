# clilint GitHub Action

A composite GitHub Action that runs [clilint](https://github.com/codekiln/clilint)
against a CLI executable and fails the build when a rule fails or the score
falls below a configured minimum.

## Inputs

| Input       | Required | Default         | Description                                    |
| ----------- | -------- | --------------- | ---------------------------------------------- |
| `target`    | yes      | —               | Path to the CLI executable to lint.            |
| `profile`   | no       | `generated-cli` | Conformance profile to check against.          |
| `min-score` | no       | `0`             | Fail (exit 1) if the score is below this value.|
| `format`    | no       | `human`         | Output format: `human`, `plain`, or `json`.    |

## How it works

The action is a `composite` action. It sets up Python 3.x
(`actions/setup-python@v5`) and then invokes clilint from disk at
`$GITHUB_ACTION_PATH/../../clilint`. This means the action expects clilint to
live two directories up from `action.yml` — i.e. the action is used from within
a checkout of `codekiln/clilint`, or from a repo that vendors clilint at its
root.

clilint is Python 3.10+ and has zero third-party dependencies, so no
`pip install` step is required.

## Usage

If your workflow has checked out `codekiln/clilint` (or vendors it), reference
the action by its path:

```yaml
- uses: actions/checkout@v4
  with:
    repository: codekiln/clilint
    path: .clilint

- uses: ./.clilint/integrations/github-action
  with:
    target: ./path/to/your-cli
    min-score: "90"
```

## Not vendoring clilint?

If you would rather not depend on the action's on-disk layout, check out
`codekiln/clilint` into a subdirectory and call it directly. See
[`example-workflow.yml`](./example-workflow.yml) for a complete, copy-pasteable
workflow you can drop into `.github/workflows/clilint.yml`.
