# Agent guidance baseline specification

## ADDED Requirements

### Requirement: Reproducible three-arm comparison

The research record MUST identify the pinned repository revisions, model and runner settings, development tasks, arm treatments, paired order, grading rubric, and sample-size rule. It MUST preserve the failed original cohort separately from corrected engaged runs.

#### Scenario: Independent reader audits the comparison

- **GIVEN** the archived public evidence bundle
- **WHEN** a reader inspects its manifests, run records, blinded packets, grades, and adjudications
- **THEN** the reader can reproduce cohort counts and final arm statistics without access to the sealed confirmation split

### Requirement: Claims are answered without overreach

The research record MUST answer whether the current Cairn query surface beats target-only navigation and whether the current guidance pack improves use of those capabilities. It MUST report the primary failure, treatment-fidelity failures, effect sizes, dispersion, and the one-point smallest effect of interest.

#### Scenario: Observed effect is smaller than the threshold

- **GIVEN** a paired arm difference below one quality point
- **WHEN** the result is interpreted
- **THEN** it is reported as descriptive and not as evidence of a meaningful improvement

### Requirement: Follow-up remains implementation-neutral

The research record MAY state reversible follow-up hypotheses. It MUST NOT implement, recommend, or grant authority to a new query, router, pack, or runtime mechanism.
