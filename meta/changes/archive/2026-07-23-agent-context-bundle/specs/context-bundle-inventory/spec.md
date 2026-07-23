# Context bundle inventory specification

## ADDED Requirements

### Requirement: Complete current-surface inventory

The research artefact MUST cover `context`, `get`, `neighbourhood`, `bundle`,
`deps`, `rationale`, `locate`, `todos`, and `sources`. For each verb it MUST
state the default payload, opt-in or lean behaviour, overlap, missing facts, and
the task decision served.

#### Scenario: Inventory can distinguish defaults from unavailable modes

- **GIVEN** the query surface at the pinned inventory revision
- **WHEN** a reader selects a candidate composition
- **THEN** the artefact identifies which fields are default, opt-in,
  transport-specific, internally represented but unreachable, or absent

### Requirement: Reproducible deferred sample

The research artefact MUST define a deterministic selector over the baseline's
frozen strata without opening or publishing the sealed confirmation prompts and
ground truth.

#### Scenario: Two evaluators select the same tasks

- **GIVEN** identical frozen baseline corpus inputs
- **WHEN** two evaluators build the evaluation manifest and apply the selector
- **THEN** they select the same tasks and required nodes in the same order

### Requirement: Fixed output accounting

The research artefact MUST fix exact run-record fields, byte and character
counts, pinned tokenization, required-fact recall, evidence-unit precision, and
disjoint relevant-first, duplicate, and irrelevant token classes before
candidate output is measured.

#### Scenario: Failed invocation remains in analysis

- **GIVEN** a candidate invocation that errors or returns empty output
- **WHEN** the later evaluation computes intention-to-treat results
- **THEN** the invocation's request, timing, exit state, and raw streams remain
  in the run record

### Requirement: Unscored candidate handoff

The research artefact MUST list exact compositions of existing verbs and MAY
list one paper-only hypothetical projection. It MUST NOT publish measurements,
thresholds, recommendations, runtime implementation, or guidance authority.

#### Scenario: Evaluation starts from the registered candidates

- **GIVEN** the baseline corpus is preregistered
- **WHEN** `todo.agent-context-bundle-evaluation` begins
- **THEN** it uses the registered candidates, sample rule, and accounting
  protocol without treating the hypothetical projection as implemented
