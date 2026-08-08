---
node: cairn.root
status: done
created: 2026-07-15
---

# Error Codes Registry Completeness

## Problem

`docs/registries/error-codes.md` states "Every error code that appears in Rust source MUST have an entry here", but some emitted codes are missing from it.

## Evidence

Discovered during PR #345 doc-sync: `CAIRN_RECONCILE_ORPHANED_FILE` (emitted at `src/reconcile/generic.rs:206`, severity Info) and `CAIRN_ORDER_CYCLE` (`src/map/integrity.rs:189`, Error) have no registry entry. There may be others.

## Approach (backlog only)

Audit `src/` for all emitted `CAIRN_*`/`CXNNN` finding codes and reconcile the registry; consider a scan or test guard that fails when a source-emitted code has no registry entry. Do NOT implement here.

## Resolution (2026-07-16)

Reconciled `docs/registries/error-codes.md`: added 90 rows (CP001-CP004,
CK005-CK030, CA006-CA037, CC005-CC013, CH003, CM001-CM009, CS001-CS004,
CO002-CO006) for every emitted `CAIRN_*` code the registry was missing,
including both codes named in the evidence above. Descriptions were
written from each code's actual construction site, not guessed from the
name; category assignment follows the emitting subsystem's directory
(`src/reconcile/*` and `src/scanner/*` -> Kernel/Map, `src/artefacts/*`
-> Artefacts, `src/query_api/*` non-summariser/non-draft handlers ->
MCP, etc). Phase tag is `pre-registry provenance unknown -- audited
2026-07-16`: these codes predate the registry and their true
introduction phase isn't tracked, so the tag says that honestly rather
than claiming this audit introduced them.

Added `tests/finding_code_coverage.rs` as the scan/test guard the
"Approach" section asked to consider: `test_every_emitted_code_has_a_registry_entry`
fails on any emitted code with no allocated `CXNNN` row (matched against
real allocation rows only, not any prose mention of the code elsewhere
in the file). It shares its emitted-code scan with
`todo.finding-code-test-coverage`'s coverage test, per that todo's
overlap note.

Deliberately out of scope, unchanged: the registry's own `CXNNN`
short-code convention (`CH001`, `CT001`, `CC001`, ...) is a separate,
fully-allocated scheme this audit did not touch. `CE001`-`CE010` (Phase
5 docstring-fact codes) remain unemitted in current source; nothing to
reconcile there.
