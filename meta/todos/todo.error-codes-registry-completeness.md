---
node: cairn.root
status: open
created: 2026-07-15
---

# Error Codes Registry Completeness

## Problem

`docs/registries/error-codes.md` states "Every error code that appears in Rust source MUST have an entry here", but some emitted codes are missing from it.

## Evidence

Discovered during PR #345 doc-sync: `CAIRN_RECONCILE_ORPHANED_FILE` (emitted at `src/reconcile/generic.rs:206`, severity Info) and `CAIRN_ORDER_CYCLE` (`src/map/integrity.rs:33`, Error) have no registry entry. There may be others.

## Approach (backlog only)

Audit `src/` for all emitted `CAIRN_*`/`CXNNN` finding codes and reconcile the registry; consider a scan or test guard that fails when a source-emitted code has no registry entry. Do NOT implement here.
