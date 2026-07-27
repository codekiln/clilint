## ADDED Requirements

### Requirement: Complete checks through Checker CLIs
Each Check in a local check bundle SHALL select one Checker CLI. The Checker
CLI SHALL own any one-time setup, evidence gathering, and production of a Check
Outcome needed for the complete Check.

#### Scenario: Add a mechanistic check from a local bundle
- **WHEN** a user installs a local check bundle whose Check names a valid Checker CLI
- **THEN** Clilint can run the check without adding check-specific code to Clilint

#### Scenario: One check verifies several related matters
- **WHEN** one check gathers shared evidence and finds several unmet expectations
- **THEN** its one Check Result contains one Score and focused Check Messages for the unmet expectations

### Requirement: Bundle resource root
Clilint SHALL retain the directory from which it loads each installed check
bundle and SHALL resolve its Checker CLI within that directory. A Checker CLI
SHALL resolve its own installed Skills, rubrics, scripts, and other resources
without using the bundle directory as its process working directory.

#### Scenario: Checker reads a sibling rubric
- **WHEN** a Checker CLI uses a rubric installed beside its executable or language package
- **THEN** the Checker can read the rubric while its process working directory remains the tested project

#### Scenario: Empty Checker command
- **WHEN** a check declares an empty Checker CLI command
- **THEN** Clilint rejects the bundle before running the check

### Requirement: Versioned check exchange
Clilint SHALL give a Checker CLI a versioned request that identifies the check
bundle, Check, tested CLI tool, project directory, and protocol version. The
Checker CLI SHALL return a versioned Check Outcome bound to that request.

#### Scenario: Check Outcome matches its request
- **WHEN** a checker returns a supported Check Outcome whose request binding and check identity match the current request
- **THEN** Clilint attaches the outcome to that check

#### Scenario: Check Outcome belongs to another request
- **WHEN** a returned or supplied Check Outcome does not match the current request
- **THEN** Clilint rejects the outcome and identifies the mismatch

### Requirement: Tool-independent protocol
The versioned request and Check Outcome protocol SHALL NOT require a specific
model, agent harness, checker programming language, or operating system.

#### Scenario: Command uses another programming language
- **WHEN** a Checker CLI is written in a language that Clilint does not embed
- **THEN** Clilint can exchange the versioned request and Check Outcome without interpreting that language

#### Scenario: Agent uses another harness
- **WHEN** a judgment-based Checker CLI accepts an Assessment produced by another agent harness
- **THEN** Clilint can validate the Checker's outcome without identifying the harness or model

### Requirement: Check Outcome alternatives
A Check Outcome SHALL contain exactly one of a Check Result, Check Error,
Awaiting Assessment state, or Skipped state. A Check Error SHALL NOT contain a
Score. A checker timeout, unsuccessful exit, or invalid protocol response
SHALL produce a Check Error rather than a scored Check Result.

#### Scenario: Checker fails before scoring
- **WHEN** a Checker CLI exits unsuccessfully before returning a valid Check Result
- **THEN** its Check Outcome contains a Check Error and no Score

#### Scenario: Checker still needs judgment
- **WHEN** a judgment-based Checker CLI has gathered evidence but has no returned Assessment
- **THEN** its Check Outcome is Awaiting Assessment rather than a Check Result or Check Error

### Requirement: Shared Check Result
A completed mechanistic or judgment-based check SHALL return the same Check
Result structure. A Check Result SHALL contain one finite floating-point Score
from `0.0` through `4.0`, inclusive, and zero or more Check Messages. A binary
Score SHALL be `0.0` or `4.0`.

#### Scenario: Fractional Score
- **WHEN** a checker measures partial improvement between two whole-number scores
- **THEN** its Check Result preserves the fractional Score within the allowed range

#### Scenario: Check meets every expectation
- **WHEN** a completed check returns a Score of `4.0`
- **THEN** its Check Result contains no Warning or Error Check Message

### Requirement: Check Messages
Each Check Message SHALL have an Info, Warning, or Error level, explain an
observation that affected or helps interpret the Score, and include supporting
evidence when available. A Score below `4.0` SHALL include at least one Check
Message that explains what could improve. Clilint SHALL reject a Check Result
that breaks either message rule.

#### Scenario: Imperfect Score has improvement feedback
- **WHEN** a checker returns a Score below `4.0` with no message explaining what could improve
- **THEN** Clilint rejects the Check Result as invalid

#### Scenario: Error message within a valid result
- **WHEN** a checker completes and reports a serious problem in the tested project
- **THEN** the Check Result can contain an Error-level Check Message without becoming a Check Error

### Requirement: Check method
A check SHALL be judgment-based when human or model interpretation affects its
Score or Check Messages. Otherwise, it SHALL be mechanistic. The Check Result
SHALL identify the check's method.

#### Scenario: Judgment-based check invokes a script
- **WHEN** a judgment-based Checker CLI uses a script to gather evidence before a model applies a rubric
- **THEN** the check is judgment-based

#### Scenario: Mechanistic check uses variable input
- **WHEN** a Checker CLI calculates its Score without human or model interpretation
- **THEN** the check is mechanistic

### Requirement: Checker CLI invocation
Clilint SHALL run a Checker CLI outside the Clilint process from the directory
in which the user invoked Clilint. A bundle SHALL name the CLI with a nonempty
command argument array. Clilint SHALL replace the literal `{bundle}`
placeholder in any argument with the installed bundle directory. Clilint SHALL
use the first item as the program and the remaining items as its arguments.
The Checker SHALL inherit Clilint's environment and operating-system
permissions.

#### Scenario: Interpreter-backed Checker CLI
- **WHEN** a bundle declares `["python3", "{bundle}/checker.py"]`
- **THEN** Clilint expands the script path, starts `python3`, and passes the script path as its first argument

Clilint SHALL write one JSON Check Request to standard input and close it.
Standard output SHALL contain only one versioned JSON Check Outcome. Standard
error SHALL contain Checker logs. Clilint SHALL direct standard output to a
temporary file and parse it after the Checker exits. Clilint SHALL leave the
Checker standard error connected to its own standard error. Clilint SHALL stop
a Checker that exceeds its fixed run-time limit.

#### Scenario: Checker CLI inherits the project directory
- **WHEN** a user invokes Clilint from a project directory
- **THEN** the Checker CLI runs from that same directory

#### Scenario: Checker CLI returns a valid Check Result
- **WHEN** a Checker CLI completes within its run-time limit and returns one valid Check Result
- **THEN** Clilint validates and records that result

#### Scenario: Checker CLI violates the protocol
- **WHEN** a Checker CLI times out, exits unsuccessfully, or returns malformed output
- **THEN** Clilint records a Check Error without a Score

#### Scenario: Checker writes logs
- **WHEN** a Checker CLI writes logs to standard error
- **THEN** the logs continue to Clilint's standard error and remain outside the Check Outcome

#### Scenario: Bundle attempts to configure the run-time limit
- **WHEN** a bundle declares a run-time limit for its Checker CLI
- **THEN** Clilint rejects the unsupported field instead of changing its fixed run-time limit

### Requirement: Judgment-based Checker CLI
A judgment-based Checker CLI SHALL be able to use an Agent Skill, rubric,
model, agent harness, or other tools without requiring another Clilint checker
implementation. The CLI MAY return Awaiting Assessment and later accept an
Assessment bound to the same Check Request through Clilint's file-based
Assessment handoff before returning a Check Result.

#### Scenario: Agent completes a pending check
- **WHEN** an external agent follows the Skill exposed by a Checker CLI and supplies an Assessment bound to the pending request
- **THEN** the Checker CLI validates the Assessment and returns a Check Result for Clilint to validate

#### Scenario: Skill file is unavailable
- **WHEN** a judgment-based Checker CLI cannot resolve a Skill it requires
- **THEN** the Checker CLI returns a Check Error without a Score

#### Scenario: Agent harness has operational logs
- **WHEN** an agent harness records operational details while following the Skill
- **THEN** those logs remain outside the Check Result and the shared protocol does not require a harness-specific log format
