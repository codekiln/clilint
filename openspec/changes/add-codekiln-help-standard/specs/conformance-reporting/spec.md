## ADDED Requirements

### Requirement: One Check Outcome per check
The report SHALL contain one Check Outcome for each check. A completed Check
Result SHALL contain one Score and zero or more Check Messages. Every Check
Message SHALL belong to that result.

#### Scenario: One check returns several Check Messages
- **WHEN** a check finds an error in command discovery and a warning in programmatic guidance
- **THEN** the report contains one Check Result for the check with both Check Messages

### Requirement: Result and message summaries
The report SHALL count Check Results and Check Errors separately from Info,
Warning, and Error Check Messages. Clilint's process exit status SHALL be
nonzero when a report contains a Check Error or an Error-level Check Message.
A Score by itself SHALL NOT determine the process exit status.

#### Scenario: Check Result with several messages
- **WHEN** one Check Result contains two Error messages and one Warning message
- **THEN** the summary counts one Check Result, two errors, and one warning and Clilint exits nonzero

#### Scenario: Score zero without an Error message
- **WHEN** one valid Check Result has a Score of `0.0` and only Warning messages
- **THEN** the summary preserves the Score without counting a Check Error or Error-level Check Message

## MODIFIED Requirements

### Requirement: Machine-readable report
Clilint SHALL emit a versioned JSON report containing the tool version, check
bundle identities, tested CLI tool, Check Outcomes, Check Messages, evidence,
and summary counts.

#### Scenario: JSON output requested
- **WHEN** a user checks a tested CLI tool with JSON output selected
- **THEN** standard output contains one valid report document and no human commentary

### Requirement: Distinct evaluation methods
Each check SHALL identify whether its Score or Check Messages were produced
mechanistically or through human or model judgment.

#### Scenario: Mixed report
- **WHEN** a report contains mechanistic and judgment-based Check Results
- **THEN** a consumer can distinguish the two methods without interpreting Check Message text

### Requirement: Unassessed AI rules
The report SHALL distinguish a judgment-based Checker CLI that is Awaiting
Assessment from a Check Result, Check Error, or Skipped outcome.

#### Scenario: Missing assessment
- **WHEN** no Assessment is supplied for an applicable judgment-based check
- **THEN** summary data records it as Awaiting Assessment rather than a Check Result, Check Error, or Skipped outcome

### Requirement: Separate measurements
The report SHALL preserve each Check Result's Score and method and SHALL NOT
imply that a judgment-based result is repeatable.

#### Scenario: Completed judgment-based Assessment
- **WHEN** a valid judgment-based Assessment and Check Result are attached
- **THEN** the report records the Score and judgment source separately from mechanistic Check Results

## RENAMED Requirements

- FROM: `Unassessed AI rules`
- TO: `Unassessed AI checks`
