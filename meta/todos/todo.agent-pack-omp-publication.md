---
node: cairn.kernel.cli
status: done
created: 2026-07-22
---

# Agent Pack OMP Publication

## Priority

P3. The final publication step for the OMP adapter, deliberately separated
from implementation so it can be gated on evidence without blocking a
completable adapter unit.

## Scope

- Publish the validated OMP adapter only after the treatment evaluation issues
  a retain verdict on the revised pack.
- Run the final revised-content smoke against the published adapter.
- Mark the OMP adapter row supported only once live validation and the retain
  verdict both hold.

## Outcome

OMP is documented as a supported adapter in `docs/commands.md` and
`docs/agent-setup.md` on 2026-07-27, which is what publication means here: the
runtime already shipped, bound to the ownership ledger, and covered by
`tests/pack_omp_adapter.rs`.

Publication rests on the live OMP 17.1.3 validation recorded in
`res.pack-omp-adapter-validation`, the adapter structure ruled by
`dec.pack-adapter-roots`, and `dec.pack-publication-on-activation-evidence`,
which retired the treatment verdict as a precondition. The retain-verdict
condition named in Scope above is therefore void; live-harness validation is
the whole gate, per `dec.agent-pack-packaging` clause 2.
