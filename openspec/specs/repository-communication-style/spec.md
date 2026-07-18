## Purpose

Define the repository's reader-centered communication standard and the RuleSync-managed skills used to assess README style and purpose.

## Requirements

### Requirement: Project communication standard
The repository SHALL define a project-owned communication standard at `.rulesync/rules/communication-style.md` that applies to documentation and agent-authored prose and that can be understood without access to a private knowledge source.

#### Scenario: Agent writes project documentation
- **WHEN** an agent prepares or revises documentation in the repository
- **THEN** generated project instructions provide the communication standard that applies to the work

### Requirement: Reader-centered style
The communication standard SHALL require authors to write for an intelligent reader who may not share their background, put the main point first, order information by reader need, prefer common and specific words, define necessary technical terms, and use direct sentences with one clear meaning.

#### Scenario: Reader lacks project context
- **WHEN** a reader encounters project documentation for the first time
- **THEN** the writing does not require unstated project knowledge to understand its main point

### Requirement: Progressive disclosure
The communication standard SHALL define progressive disclosure as presenting the main point and the information needed for the reader's current decision or task first, then providing a clear path to deeper detail when it becomes relevant. The standard SHALL state that progressive disclosure applies to both human and agent readers and does not permit necessary information to become undiscoverable.

#### Scenario: Human reader begins a common task
- **WHEN** a human reader follows an entry-point document
- **THEN** the document presents the common decision or action before special cases and provides descriptive links to complete details

#### Scenario: Agent begins a scoped task
- **WHEN** an agent receives project instructions for a scoped task
- **THEN** the instructions provide enough context to begin and direct the agent to load the relevant skill, rule, namespace, specification, or reference before it is needed

#### Scenario: Deeper detail remains discoverable
- **WHEN** information is deferred from an entry point
- **THEN** the entry point names and links or references the deeper source closely enough that the reader can find it when needed

### Requirement: Distracting language review
The communication standard SHALL direct authors to remove unexplained jargon, coined phrases, word salad, double negatives, needless figurative language, vague claims, unnecessary contrasts, irrelevant implementation details, exaggerated certainty, dramatic framing, and exclusionary language.

#### Scenario: Sentence sounds fluent but adds no information
- **WHEN** a sentence does not answer a likely reader question or reduce uncertainty
- **THEN** the standard directs the author to remove or rewrite it

### Requirement: Authoring instructions remain outside human documents
The communication standard SHALL keep instructions about how agents write and review documents in RuleSync source rather than presenting those instructions as goals or content of the human-facing document.

#### Scenario: Agent applies the communication standard
- **WHEN** an agent writes a README section
- **THEN** the resulting section contains the information readers need without describing the agent's writing process or internal rules

### Requirement: RuleSync-managed README style skill
The repository SHALL define `assess-readme-style` under `.rulesync/skills/` and generate it for every configured target that supports RuleSync skills. The skill SHALL evaluate `README.md` against the project communication standard, including progressive disclosure at document, section, and sentence level, and report problems with exact locations, named criteria, evidence, and reader impact.

#### Scenario: README contains jargon and an unnecessary contrast
- **WHEN** `assess-readme-style` evaluates the affected text
- **THEN** it reports each problem at its location and identifies the communication criterion it violates

#### Scenario: README reveals detail before it is useful
- **WHEN** a README section presents special cases or reference material before the common action and its purpose
- **THEN** `assess-readme-style` reports the premature detail and identifies the deeper document or later section where it belongs

### Requirement: RuleSync-managed README purpose skill
The repository SHALL define `assess-readme-purpose` under `.rulesync/skills/` and generate it for every configured target that supports RuleSync skills. The skill SHALL evaluate whether `README.md` explains purpose, value, why a reader would choose Clilint, trust, maturity, installation, first use, help, maintenance, contribution, and license in the order a first-time visitor needs them.

#### Scenario: README explains internals before user value
- **WHEN** `assess-readme-purpose` evaluates the README
- **THEN** it reports the misplaced information and the unanswered visitor question

### Requirement: Evidence-based assessment results
Each README assessment skill SHALL report problems separately from optional suggestions, cite README lines or headings, state uncertainty when evidence is insufficient, and report a passing result only when it has no unresolved problem.

#### Scenario: Assessment finds no problem to fix
- **WHEN** a skill completes its evaluation with only optional suggestions
- **THEN** it reports a passing result and keeps the suggestions separate from the verdict

### Requirement: Required dual README assessment
Generated project instructions SHALL require both README assessment skills after a change to `README.md`. Reported problems SHALL be resolved or recorded as accepted exceptions with a reason before the documentation change is considered complete.

#### Scenario: Agent changes the README
- **WHEN** an agent finishes a README edit
- **THEN** it runs both assessments and reports how each problem was handled

### Requirement: Assessment examples and generation checks
Each README assessment skill SHALL include clear and unclear examples with expected findings, and the repository SHALL verify that RuleSync source generates current copies for every configured target.

#### Scenario: RuleSync source changes
- **WHEN** the communication rule or either README skill changes
- **THEN** the RuleSync consistency check fails until the configured outputs are regenerated
