---
node: cairn.brownfield
status: done
created: 2026-07-26
---

# Emit a contract pointer for every brownfield-discovered node

## Problem

`cairn init --from-code` followed by `cairn change apply brownfield-init`
produces a map that immediately fails `cairn scan` with one
`CAIRN_CONTRACT_LEAF_UNCOVERED` finding per discovered node. The contract
artefacts are not missing: `write_change` emits
`meta/changes/<id>/contracts/<id>.md` in the same apply step, and
`parse_artefact_operations` promotes it to `meta/contracts/<id>.md`. The node
block in the emitted blueprint delta simply carries no `contract` pointer, so
the coverage gate reports the artefact absent.

## Direction

- Single-source the contract filename in `src/brownfield/mod.rs`
  (`contract_file_name`, `contract_pointer`); the change-directory writer and
  the delta emitter currently derive it independently
  (`src/brownfield/mod.rs:90`, `src/brownfield/refine.rs:151`).
- Emit `contract "./meta/contracts/<id>.md"` from `blueprint_delta` alongside
  the existing `path` line. `refine` inherits it through
  `blueprint_delta_with_renames`.
- Not the `@no-contract` tag: the contract file genuinely exists, the pointer
  is the missing half.

## Success criterion

After `cairn init --from-code` and `cairn change apply brownfield-init` on a
project with no prior cairn state, `cairn scan` reports zero
`CAIRN_CONTRACT_LEAF_UNCOVERED` findings.

## Out of scope

The remaining `CAIRN_ORDER_CYCLE` finding on brownfield output is tracked
separately in `todo.brownfield-parent-package-cycle`.
