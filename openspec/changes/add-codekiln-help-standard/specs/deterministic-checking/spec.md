## MODIFIED Requirements

### Requirement: Target invocation
Clilint SHALL run a tested CLI tool with check-bundle-declared arguments,
environment changes, standard input behavior, and a bounded timeout.

#### Scenario: Timed invocation
- **WHEN** a declared invocation exceeds its timeout
- **THEN** Clilint stops the invocation and records the timeout as evidence

### Requirement: Captured observations
Each tested CLI tool invocation SHALL capture its arguments, exit status,
timeout state, duration, standard output, and standard error.

#### Scenario: Invalid argument observation
- **WHEN** a checker invokes the tested CLI tool with an invalid argument
- **THEN** its evidence distinguishes the exit status, standard output, and standard error

### Requirement: Typed deterministic checks
Each deterministic check SHALL use a supported checker whose accepted inputs
and possible results are validated before the tested CLI tool runs.

#### Scenario: Valid output assertion
- **WHEN** a check bundle declares a supported assertion about captured output
- **THEN** Clilint runs the checker and returns a pass, warning, or failure with supporting evidence

### Requirement: Closed standard input
Clilint SHALL close the tested CLI tool's standard input unless a checker
explicitly supplies input.

#### Scenario: Tested CLI tool attempts to read input
- **WHEN** a tested CLI tool reads standard input during a check whose checker supplies no input
- **THEN** the tested CLI tool receives end-of-file instead of inheriting the user's terminal
