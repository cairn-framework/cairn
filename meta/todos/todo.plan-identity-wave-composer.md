---
node: cairn.kernel.query
status: done
created: 2026-08-07
blocked_by: [todo.coord-read-surface, todo.write-set-derivation]
related: [dec.rung-three-coordination-substrate, res.parallel-dispatch-rung-3]
---

# Plan identity and the shared wave composer

Implements `res.parallel-dispatch-rung-3` Part 1 and
`dec.rung-three-coordination-substrate` clause 1. Blocked on that signature:
substituting recompute-equality for `dec.webui-write-authority` clause 4's
base-commit condition is a maintainer ruling.

## Task

1. The canonical preimage: `cairn-plan-v1` magic line, `rule=`, then `unit=` and
   sorted `ws=` lines, units sorted by todo id, paths carrying no trailing slash,
   leading `./` stripped, one trailing LF. The base commit and
   `query_api::SCHEMA_VERSION` are both excluded from the hashed bytes; the base
   commit is recorded on the ruling fact as provenance only.
2. `plan-<16 hex>` of SHA-256 over that preimage, reusing
   `src/artefacts/registry/sha256.rs`.
3. One wave composer, consumed by the console preview and by the driver through
   the same passive query (`cairn wave`), so the two cannot drift into two
   compositions.
4. `cairn ruling run <plan>` records one fact whose `payload.target` is the
   digest. Nothing is recorded when a preview is rendered.
5. The decline path: `outcome.run_declined` with the closed reason enum
   (`readiness-moved`, `write-sets-overlap`, `unit-set-moved`, `parked`,
   `lease-held`, `ruleset-changed`, `consent-expired`, `already-consumed`,
   `superseded-by-concurrent-ruling`), structured per-unit `causes`, the preimage
   diff spilled to a sidecar above 4 KiB, and `dispatched: []` enforced as
   all-or-nothing.
6. Register every reason code in `docs/registries/error-codes.md` and
   `docs/design-system/copy.toml` under `[findings.codes]`.

## Acceptance

- A test proves an unrelated commit does not change the digest, and that a
  changed unit set, write-set, or composer rule does.
- A test proves a second run ruling on a consumed digest declines with
  `already-consumed`, and that two concurrent rulings resolve by
  `(recorded_at, fact_id)` over the fully listed fact set.
- A test proves no partial wave is ever dispatched.
