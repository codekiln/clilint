## MODIFIED Requirements

### Requirement: Help-quality assessment
The core check bundle SHALL include an AI-assessed check that determines
whether help text teaches a new user how to perform a likely task with a useful
example.

#### Scenario: Useful help example
- **WHEN** help explains the tool's purpose and gives a concrete example of a likely task
- **THEN** the assessment skill can classify the check as passing and explain the evidence

#### Scenario: Example heading without useful guidance
- **WHEN** help contains the word `example` but does not teach a likely task
- **THEN** the assessment skill does not classify the check as passing solely because the word is present

### Requirement: Agent evidence
Clilint SHALL include the captured help needed by an AI-assessed check in the
machine-readable result and SHALL mark the check as unassessed until a valid
assessment is supplied.

#### Scenario: Check without an AI assessment
- **WHEN** a tested CLI tool is checked without a supplied assessment for the help-quality check
- **THEN** the report contains the check, its evidence, its required skill, and an unassessed result

### Requirement: Assessment attachment
Clilint SHALL accept an assessment document for an AI-assessed check and SHALL
validate its check identifier, result, skill identity, format version, and
evidence digest before attaching it to a report.

#### Scenario: Assessment matches current evidence
- **WHEN** an assessment uses the expected skill and references the current evidence digest
- **THEN** Clilint includes the assessment result and explanation in the report

#### Scenario: Assessment references stale evidence
- **WHEN** an assessment's evidence digest does not match the current observations
- **THEN** Clilint rejects the assessment and leaves the check unassessed
