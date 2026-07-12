# probes/

Behavioral tests that exercise a target CLI and observe how it actually behaves — help output on
`-h`/`--help`, exit codes, the stdout/stderr split, structured-output modes, error formats, TTY vs
non-TTY behavior, and so on. Each probe supplies the evidence a rule in [`rules/`](../rules/) uses to
decide pass / warn / fail.
