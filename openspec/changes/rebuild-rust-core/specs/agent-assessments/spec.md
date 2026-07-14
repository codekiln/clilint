## ADDED Requirements

### Requirement: Installable assessment skill
The repository SHALL provide an `assess-cli-help` Agent Skill at `skills/assess-cli-help/SKILL.md` that the Vercel Skills CLI can discover and install from the repository.

#### Scenario: List repository skills
- **WHEN** a user lists the skills available from the clilint repository with the Vercel Skills CLI
- **THEN** `assess-cli-help` appears as an installable skill

### Requirement: Help-quality assessment
The global package SHALL include an AI-agent rule that assesses whether help text teaches a new user how to perform a likely task with a useful example.

#### Scenario: Useful help example
- **WHEN** help explains the tool's purpose and gives a concrete example of a likely task
- **THEN** the assessment skill can classify the rule as passing and explain the evidence

#### Scenario: Example heading without useful guidance
- **WHEN** help contains the word `example` but does not teach a likely task
- **THEN** the assessment skill does not classify the rule as passing solely because the word is present

### Requirement: Agent evidence
Clilint SHALL include the captured help needed by an AI-agent rule in the machine-readable check result and SHALL mark the rule as unassessed until a valid assessment is supplied.

#### Scenario: Check without an AI assessment
- **WHEN** a target is checked without a supplied assessment for the help-quality rule
- **THEN** the report contains the rule, its evidence, its required skill, and an unassessed result

### Requirement: Assessment attachment
Clilint SHALL accept an assessment document for an AI-agent rule and SHALL validate its rule identifier, result, skill identity, format version, and evidence digest before attaching it to a report.

#### Scenario: Assessment matches current evidence
- **WHEN** an assessment uses the expected skill and references the current evidence digest
- **THEN** clilint includes the assessment result and explanation in the report

#### Scenario: Assessment references stale evidence
- **WHEN** an assessment's evidence digest does not match the current target observations
- **THEN** clilint rejects the assessment and leaves the rule unassessed

### Requirement: Safe help assessment
The help-quality skill SHALL inspect help output without executing example commands found in that output.

#### Scenario: Help contains a destructive example
- **WHEN** captured help includes an example that could change local or remote state
- **THEN** the skill judges the text without executing the example
