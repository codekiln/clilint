## ADDED Requirements

### Requirement: Judgment-based Checker CLI
A local check bundle SHALL be able to define a complete judgment-based Check
with a Checker CLI. The CLI SHALL be able to use an Agent Skill stored with the
Checker, gather evidence, and own the Assessment handoff without requiring
Clilint to execute the Skill.

#### Scenario: Installed Skill gathers its own evidence
- **WHEN** an installed judgment-based Checker CLI exposes a Skill and evidence to an external agent
- **THEN** the external agent can create an Assessment without check-specific evidence gathering in Clilint

## MODIFIED Requirements

### Requirement: Help-quality assessment
The core check bundle SHALL include a judgment-based check that determines
whether help text teaches a new user how to perform a likely task with a useful
example. Its Assessment SHALL record how the selected rubric was applied to
the gathered evidence.

#### Scenario: Useful help example
- **WHEN** help explains the tool's purpose and gives a concrete example of a likely task
- **THEN** the Assessment can support a high Score and explain the evidence

#### Scenario: Example heading without useful guidance
- **WHEN** help contains the word `example` but does not teach a likely task
- **THEN** the Assessment does not assign a high Score solely because the word is present

### Requirement: Agent evidence
A judgment-based Checker CLI SHALL gather the evidence needed by its Check. It
MAY return an Awaiting Assessment outcome containing the versioned Check
Request, required Skill, rubric, and evidence. Clilint SHALL record that
pending work in the report. After a later invocation supplies an Assessment
JSON file, Clilint SHALL pass the Assessment to the same Checker CLI, which
SHALL return a Check Result bound to the request.

#### Scenario: Check without an AI assessment
- **WHEN** a tested CLI tool is checked without a supplied assessment for the help-quality check
- **THEN** the report contains the check request, required Skill, rubric, evidence, and an Awaiting Assessment outcome

#### Scenario: External agent uses another harness
- **WHEN** an external agent follows the recorded Skill and writes a valid Assessment JSON file
- **THEN** a later Clilint invocation can supply the file without Clilint identifying the model or agent harness

### Requirement: Assessment attachment
A judgment-based Checker CLI SHALL validate an Assessment's request binding,
Check identifier, expected Skill, and format version before returning a Check
Result. Clilint SHALL validate the returned Check Result's request binding,
Score, and Check Messages before attaching it to a report.

#### Scenario: Assessment matches current request
- **WHEN** an assessment uses the expected Skill and references the current check request
- **THEN** the Checker CLI returns a Check Result and Clilint includes the result and Assessment explanation in the report

#### Scenario: Assessment references another request
- **WHEN** an assessment's request binding does not match the pending check request
- **THEN** the Checker CLI rejects the assessment with a Check Error and no Score
