---
node: cairn.root
status: done
created: 2026-07-30
---

# Over-harness Research Capture

## Scope

Capture the maintainer conversation of 2026-07-30 and its Huntley citation
as source artefacts (`src.maintainer-design-threads-2026-07-30`,
`src.huntley-software-factory`), synthesise them in
`res.overharness-design-threads` anchored on `cairn.root`, and file the
trust-verified review gap as a new open todo,
`todo.review-gate-machine-check`, after confirming
`todo.local-gate-attestation` owns a different evidence class.

## Depends on

Nothing.

## Acceptance

- `cairn research cairn.root` lists `res.overharness-design-threads` with
  both sources attached.
- `cairn todos cairn.root` shows this todo `done` and
  `todo.review-gate-machine-check` `open`.
- `cairn scan --strict` exits 0; the two new `CAIRN_SOURCE_UNVERIFIED` Info
  findings are the designed markers for conversation-backed and unpinned
  sources, matching `src.mission-ratification-2026-07-30`.

## Origin

Maintainer mission, 2026-07-30: same-day capture alongside the mission
ratification artefacts.
