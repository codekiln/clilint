## ADDED Requirements

### Requirement: Machine-readable report
Clilint SHALL emit a versioned JSON report containing the tool version, package identities, target, findings, evidence, and summary counts.

#### Scenario: JSON output requested
- **WHEN** a user checks a target with JSON output selected
- **THEN** standard output contains one valid report document and no human commentary

### Requirement: Distinct evaluation methods
Each finding SHALL identify whether it was produced by a deterministic check or an AI-agent assessment.

#### Scenario: Mixed report
- **WHEN** a report contains both deterministic and AI-agent findings
- **THEN** a consumer can distinguish the two methods without interpreting finding text

### Requirement: Unassessed AI rules
The report SHALL distinguish an AI-agent rule that has not been assessed from a rule that passed, warned, failed, or was skipped.

#### Scenario: Missing assessment
- **WHEN** no assessment is supplied for an applicable AI-agent rule
- **THEN** summary data records it as unassessed rather than passed or skipped

### Requirement: Separate measurements
The report SHALL present repeatable deterministic results separately from AI-agent assessment results and SHALL NOT imply that the AI-agent result is repeatable.

#### Scenario: Completed AI assessment
- **WHEN** a valid AI assessment is attached
- **THEN** the report records its result and provenance separately from the deterministic score
