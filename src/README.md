# src/

The reference linter — the `clilint` executable. Loads the definitions in [`rules/`](../rules/),
runs the [`probes/`](../probes/) against a target executable, and emits findings and a conformance
score (human-readable or JSON per [`schemas/`](../schemas/)). Implementation language is not yet
fixed.
