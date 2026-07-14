## ADDED Requirements

### Requirement: Target invocation
Clilint SHALL run a target executable with package-declared arguments, environment changes, standard input behavior, and a bounded timeout.

#### Scenario: Timed invocation
- **WHEN** a declared target invocation exceeds its timeout
- **THEN** clilint stops the invocation and records the timeout as evidence

### Requirement: Captured observations
Each target invocation SHALL capture its arguments, exit status, timeout state, duration, standard output, and standard error.

#### Scenario: Invalid argument observation
- **WHEN** a check invokes the target with an invalid argument
- **THEN** its evidence distinguishes the exit status, standard output, and standard error

### Requirement: Typed deterministic checks
Each deterministic rule SHALL use a supported check type whose accepted inputs and possible results are validated before the target runs.

#### Scenario: Valid output assertion
- **WHEN** a package declares a supported assertion about captured output
- **THEN** clilint evaluates the assertion and returns a pass, warning, or failure with supporting evidence

### Requirement: Closed standard input
Clilint SHALL close target standard input unless a check explicitly supplies input.

#### Scenario: Target attempts to read input
- **WHEN** a target reads standard input during a check that supplies no input
- **THEN** the target receives end-of-file instead of inheriting the user's terminal
