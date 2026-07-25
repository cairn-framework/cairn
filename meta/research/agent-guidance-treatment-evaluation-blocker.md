---
id: res.agent-guidance-treatment-evaluation-blocker
nodes:
  - cairn.kernel.cli
sources:
  - src.agent-guidance-baseline-archive
  - src.harness-engineering
date: 2026-07-26
---

# Treatment evaluation cannot run from the repository alone

## Question

Can `todo.agent-guidance-treatment-evaluation` produce its required treatment
verdict in this loop environment without opening or fabricating evidence?

## Evidence inspected

The completed baseline is recorded in `res.agent-experiment-linklint`. Its
three-arm development study preserved the sealed confirmation prompts and
ground truth outside the repository. The development evidence reports a runner
availability failure in the primary intention-to-treat cohort, followed by a
secondary engaged-run cohort. It does not contain the new treatment arms,
assigned raw runs, blind grades, or a sealed confirmation comparison required
by this todo.

The selected todo requires six incrementally different guidance arms, the same
worker epoch and Claude lifecycle, randomised order, blind grading, preserved
raw failures and variance, and a terminal verdict from a confirmation set
opened once after revisions. None of those new run records or the sealed
confirmation material is available in this worktree.

The environment can inspect the existing pack and archived development
baseline, but cannot provision the external worker epoch or obtain the sealed
confirmation inputs. Reusing the baseline's development results as a terminal
treatment result would violate the todo's validity gate and contaminate the
holdout.

## Recommendation

Keep the treatment-evaluation todo blocked until the study owner provides the
sealed confirmation material and a runnable, authenticated worker epoch for
all assigned arms. At that point, execute the frozen protocol, preserve every
raw outcome including failures, obtain blind grading, and author the required
retain, revise, or remove decision from the resulting evidence. Do not publish
the OMP adapter while this evidence gap remains.

## Boundary

This record is not an outcome evaluation and makes no retain, revise, or remove
verdict. It records only the observable prerequisite gap so a later loop does
not mistake archive inspection for empirical evidence.
