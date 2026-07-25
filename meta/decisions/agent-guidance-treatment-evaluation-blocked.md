---
id: dec.agent-guidance-treatment-evaluation-blocked
nodes:
  - cairn.kernel.cli
status: proposed
date: 2026-07-25
informed_by:
  - res.agent-guidance-treatment-evaluation-blocker
---
# Agent Guidance Treatment Evaluation Blocked

## Context
The treatment evaluation requires six randomly ordered arms, a fixed worker
epoch, blind grading, preserved raw runs and failures, and a sealed
confirmation comparison. The repository contains the baseline development
archive but not the assigned treatment runs, sealed prompts, ground truth, or
an authenticated replay environment.

## Decision

Keep `todo.agent-guidance-treatment-evaluation` blocked until the study owner
provides a runnable authenticated worker epoch and the sealed confirmation
inputs. Do not infer a retain, revise, or remove verdict from the existing
development archive, and do not publish the OMP adapter before a valid terminal
verdict.

## Rationale

The existing archive explicitly leaves the sealed confirmation material
unopened and records a different baseline comparison. Reusing it would violate
the todo's intention-to-treat and holdout rules. The blocker is therefore an
evidence and environment prerequisite, not a treatment result.

## Consequences

The evaluation remains eligible for a later supervised loop iteration once the
owner supplies the missing runtime and sealed materials. This decision records
the evidence boundary without accepting a pack composition or opening OMP
publication.
