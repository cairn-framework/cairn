---
node: cairn.kernel.cli
status: blocked
created: 2026-07-22
---

# Agent Pack OMP Publication

## Priority

P3. The final publication step for the OMP adapter, deliberately separated
from implementation so it can be gated on evidence without blocking a
completable adapter unit.

## Depends on

`todo.agent-pack-omp-adapter` (validated, unpublished adapter) and an accepted
(`status: accepted`) decision recording a `retain` verdict from the
treatment-evaluation chain. Reconciliation opens this child explicitly, only
after verifying that accepted retain decision; it is not opened by the generic
all-`Depends on`-done rule, because a treatment round lands `done` on `revise`
too and a retain record may still be `proposed`. A `revise` round spawns a
successor round while this child stays `blocked`; an accepted `remove` decision
drops this child per the umbrella completion condition.

## Scope

- Publish the validated OMP adapter only after the treatment evaluation issues
  a retain verdict on the revised pack.
- Run the final revised-content smoke against the published adapter.
- Mark the OMP adapter row supported only once live validation and the retain
  verdict both hold.

## Acceptance

- The OMP adapter is public only after a recorded retain verdict.
- The revised-content smoke passes against the published adapter.
- No unverified adapter row ships as fact.
