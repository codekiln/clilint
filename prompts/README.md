# prompts/

Instructions for LLM coding agents that build or repair CLIs against the CLI Lint standard. These
guide an agent through the loop — generate or modify a CLI, run `clilint`, read the JSON report, fix
the violations, and re-check conformance — and describe how to interpret findings and remediation.
