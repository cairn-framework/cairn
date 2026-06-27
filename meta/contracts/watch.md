---
node: cairn.watch
---

# Contract: cairn.watch

## Purpose

Watch mode: periodic re-scanning that emits finding-change events. Drives the
`cairn watch` CLI command, which loops on a fixed interval, re-runs the scanner,
and reports findings that were added or resolved between two consecutive scans
as newline-delimited JSON. This is the streaming, change-oriented view over the
scanner's otherwise snapshot output.

## Public interface

- `WatchOpts`: loop configuration (`interval_secs`, `once`), with `Default`
  (5 second interval, looping) and `from_args(&[String])` which parses the
  `--interval` and `--once` flags and returns a descriptive `String` error on a
  missing value, a non-numeric interval, a zero interval, or an unknown flag.
- `WatchEvent`: a serde-tagged enum (`tag = "event"`) with two variants,
  `FindingAdded` and `FindingResolved`, each carrying an ISO-8601 `timestamp`
  and the `Finding`. Serialised as `finding_added` / `finding_resolved`.
- `diff_findings(old, new) -> Vec<WatchEvent>`: computes the delta between two
  finding sets, matching findings by the `(code, node, target, path)` key.

## Invariants

- Findings are keyed by `(code, node, target, path)`; a same-key finding whose
  severity or message changed is reported as a resolution of the old plus an
  addition of the new, preserving the simple added/resolved contract.
- Interval must be at least 1 second; `from_args` rejects a zero or missing
  interval value.
- Timestamps are emitted as ISO-8601 with no third-party datetime dependency
  (a hand-rolled Unix-to-UTC conversion and an inlined leap-year predicate).

## Dependencies

Leaf with no outgoing blueprint edges. Consumes the `Finding` type from
`cairn.kernel.map` (`crate::map::graph::Finding`) but declares no `->` edge in
the blueprint; it is invoked from the CLI layer.

## Tests

Unit tests are colocated in a `#[cfg(test)]` module in `src/watch.rs`,
covering `WatchOpts::from_args` flag parsing and error cases, the
added/resolved/changed branches of `diff_findings`, and the ISO-8601
timestamp formatting helpers.
