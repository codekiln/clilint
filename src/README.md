# src/

The reference linter — the `clilint` executable, as the `clilint` Python package. Loads the
definitions in [`rules/`](../rules/), runs the [`probes/`](../probes/) against a target executable,
and emits findings and a conformance score (human-readable, plain, or JSON per
[`schemas/`](../schemas/)). Written in Python 3.10+ using only the standard library.

Modules: `cli.py` (the command), `engine.py` (probe discovery, scoring, report assembly),
`runner.py` (spawns the target), `catalog.py` (loads rules and profiles), `report.py` (formatting),
`model.py` (shared types), `paths.py` (data-directory locations).
