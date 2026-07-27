## MODIFIED Requirements

### Requirement: Target invocation
For a check that uses a built-in checker, Clilint SHALL run the tested CLI tool
with check-bundle-declared arguments, environment changes, standard input
behavior, and a bounded timeout. A Checker CLI SHALL own its tested CLI tool
invocations.

#### Scenario: Timed invocation
- **WHEN** a declared invocation exceeds its timeout
- **THEN** Clilint stops the invocation and records the timeout as evidence

### Requirement: Captured observations
Each tested CLI tool invocation performed by a built-in checker SHALL capture
its arguments, exit status, timeout state, duration, standard output, and
standard error. A Checker CLI SHALL return the evidence required by the shared
Check Result contract.

#### Scenario: Invalid argument observation
- **WHEN** a checker invokes the tested CLI tool with an invalid argument
- **THEN** its evidence distinguishes the exit status, standard output, and standard error

### Requirement: Typed deterministic checks
Each built-in mechanistic check SHALL use a supported checker whose accepted
inputs and possible results are validated before the tested CLI tool runs.
Each mechanistic check in a local bundle SHALL use a Checker CLI that returns
the shared Check Outcome structure.

Built-in checkers SHALL remain compiled into Clilint in this change and SHALL
return the same Check Result, Score, and Check Message structure used by
Checker CLIs.

#### Scenario: Valid output assertion
- **WHEN** a check bundle declares a supported assertion about captured output
- **THEN** Clilint runs the checker and returns a Score and Check Messages with supporting evidence

#### Scenario: Installed mechanistic check
- **WHEN** an installed check bundle declares a valid Checker CLI for a mechanistic check
- **THEN** Clilint invokes the CLI and validates its returned Check Outcome without adding a built-in checker type

### Requirement: Closed standard input
Clilint SHALL close the tested CLI tool's standard input during a built-in
check unless its checker explicitly supplies input. A Checker CLI SHALL control
standard input for the tested CLI tool invocations it performs.

#### Scenario: Tested CLI tool attempts to read input
- **WHEN** a tested CLI tool reads standard input during a check whose checker supplies no input
- **THEN** the tested CLI tool receives end-of-file instead of inheriting the user's terminal
