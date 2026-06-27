---
node: cairn.suggested-edges
---

# Contract: cairn.suggested-edges

## Purpose
Owns the per-change suggested-edges queue (`suggested-edges.json`): the mutable triage workflow for AI-suggested graph edges. Where the `provenance` module holds immutable trace records, this module owns the queue lifecycle: read, write, validate, and count. It supplies the `--strict` validation gate that blocks archive while suggestions remain untriaged.

## Public interface
- `mod.rs`:
  - `SUGGESTED_EDGES_QUEUE_VERSION: u32` (currently `1`), the wire schema version.
  - `read_queue(path) -> Result<Option<SuggestedEdgesQueue>, QueueError>`: reads a queue, returning `Ok(None)` when the file is absent.
  - `queue_path_for_change(change_dir) -> PathBuf` and `read_from_change(change_dir)`: resolve and read the queue for a change directory.
  - `write_to_change(change_dir, queue)`: atomic write via a pid+nanos+counter temp path.
  - `count_pending(queue) -> usize`: counts `Pending` entries.
  - `validate_strict(change_id, change_dir)`: the CC002/CC003 gate.
- `types.rs`: `TriageState` (`Pending` default, `Accepted`, `Rejected`, `Deferred`), `EdgeProvenance` (`trace_phase`, `stage`), `SuggestedEdgeEntry` (`source`, `target`, `relation`, `triage_state`, optional `confidence`, `provenance`, `triage_note`), `SuggestedEdgesQueue` (`version`, `entries`), and `QueueError`.

## Invariants
- New entries default to `TriageState::Pending`; only human triage moves them to accepted, rejected, or deferred.
- `read_queue` rejects a queue whose `version` exceeds `SUGGESTED_EDGES_QUEUE_VERSION` with `UnsupportedVersion`; an absent file is `Ok(None)`, never an error.
- Writes are atomic: a unique temp file (pid, nanos, counter suffix) prevents concurrent writers racing on one `.json.tmp`.
- `validate_strict` returns `CairnError::UntriagedSuggestedEdges` (CC002) when any entry is pending and `CairnError::ChangeDiscovery` (CC003) on queue I/O, parse, or version errors.
- `confidence`, when present, is a producer value in [0.0, 1.0]; `provenance` optionally points back into a trace sidecar.

## Dependencies
Leaf node: no outgoing blueprint edges. Uses `serde`/`serde_json`, `std`, and `crate::error::CairnError` for the strict-gate error codes.

## Tests
Unit tests in `mod.rs` under `#[cfg(test)]` (lines ~146 onward) cover read/write round-trips, absent-file handling, version rejection, pending counts, atomic write behaviour, and the strict-validation gate producing CC002 for pending entries and CC003 for malformed queues.
