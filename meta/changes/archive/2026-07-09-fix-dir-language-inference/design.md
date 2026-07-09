# Design: Fix directory language inference

## Approach

Today a target's language is decided once, at collection time
(`collect_targets`, `src/scanner/mod.rs:91-104`), from the path extension
alone. Directories have no extension, so the decision collapses to the
Rust fallback.

The fix splits the decision into two stages:

1. At collection time, keep the extension-based fast path for file targets.
   For directory targets, defer the language to a new
   `Language::infer_from_directory(root, path, ignores)` that walks the
   directory once and returns the dominant supported language by file
   count (ties broken by the `SUPPORTED_LANGUAGES` order).
2. If inference returns `None`, the target is marked `Language::Unknown`
   (new variant) rather than `Rust`. `reconcile_targets`
   (`src/scanner/mod.rs:126-171`) skips reconciliation for `Unknown`
   targets and pushes a warning finding (`CAIRN_RECONCILE_LANGUAGE_UNKNOWN`)
   naming the node and path, with a CTA pointing at the `targets:` override.

The `Unknown` variant makes "I could not tell" a first-class visible
state instead of a silent default. It is intentionally not reconciled: no
files are claimed, no symbols collected, no hash emitted.

### Design choice to ratify (before code)

"Infer from contents, else fail loud" is chosen over "require explicit
declaration" because the latter would break every existing directory
target on first scan (including the cairn self-host). Inference preserves
backward compatibility while removing the silent fiction. If the
maintainer prefers requiring declaration, that is a valid alternative and
should be recorded as a decision artefact (`dec.language-inference-policy`)
before implementation.

### Empty-files hash suppression

Independently of language, a target whose reconciler ran but claimed zero
files (e.g. a Rust directory that is actually empty) currently still
emits `InterfaceFingerprint::from_symbols(&[])`. Change
`reconcile_targets` to emit a warning finding
(`CAIRN_RECONCILE_EMPTY_TARGET`) and omit the hash when `claimed_files`
is empty, so a zero-file reconcile never looks successful.

### Findings emission (no central registry)

This repo has no findings registry. Codes are inline `Finding` strings at
the emission site, and user-facing text is resolved through
`docs/design-system/copy.toml` under `[findings.codes]` via the
`src/copy.rs` lookup pattern (e.g. `findings.codes.CAIRN_RECONCILE_ORPHANED_FILE.heading`).
The two new codes follow the same pattern: emit the inline `Finding` in
`reconcile_targets`, and add `heading`/`body`/`cta` entries under
`[findings.codes]` in `copy.toml`. Name them to align with the existing
`CAIRN_RECONCILE_*_LANGUAGE` family routed in
`src/query_api/handlers/remediate.rs:173-179`.

### Remediation routing

`src/query_api/handlers/remediate.rs` matches finding codes to classify
remediation paths (e.g. `CAIRN_RECONCILE_ORPHANED_FILE` -> `has_orphans`
at line 121; the `CAIRN_RECONCILE_*_LANGUAGE` / `CAIRN_RECONCILE_PARSE_*`
family at lines 173-179). Add `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` and
`CAIRN_RECONCILE_EMPTY_TARGET` to that match (or document explicitly why
they are excluded), with a test covering the remediation output.

## Changes

ADDED:
- `Language::Unknown` variant (`src/reconcile/target.rs`) plus its
  `as_str` ("unknown") and `reconciler_id` (returns a sentinel; never
  dispatched).
- `Language::infer_from_directory(root, path, ignores) -> Option<Language>`
  in `src/reconcile/target.rs`: walks the directory reusing the existing
  ignore filter (`scanner::config::is_ignored`), counts files per
  supported extension, returns the dominant language.
- Finding codes `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` and
  `CAIRN_RECONCILE_EMPTY_TARGET`: inline `Finding` emission in
  `reconcile_targets` + `[findings.codes]` entries in `copy.toml`.

MODIFIED:
- `src/scanner/mod.rs:96`: replace
  `Language::from_extension(&path).unwrap_or(Language::Rust)` with
  extension fast-path, else `infer_from_directory`, else `Unknown`.
- `src/scanner/mod.rs:126-171` (`reconcile_targets`): skip `Unknown`;
  emit warnings; suppress hash on empty `claimed_files`.
- `src/scanner/mod.rs:33-48` (`TargetReport`): `hash` becomes
  `Option<String>` for honesty. This ripples through every reader of the
  hash, not just the renderer:
  - `cairn files` renderer (`src/cli/render/node.rs`).
  - query API / MCP files endpoint (`src/query_api/handlers/node.rs:84`,
    `files_json` serialises `"hash": report.hash`). Decide the `None`
    representation: omit the `hash` key (preferred, avoids a misleading
    `"hash": null`) and record it as a wire-schema change for both the
    CLI `--json` and the query API files payloads.
  - `src/scanner/cache.rs` (`build_reports_from_cache` reconstructs
    `TargetReport`s from cached `ReconcileReport`s; must propagate `None`).
  - `.cairn/state/interface-hashes.json` persistence
    (`state::TargetHashes`) and the aggregate `interface_hash` in
    `src/scanner/mod.rs`.
  - `detect_divergence` (`src/scanner/mod.rs:176`), which compares hashes.
- `src/query_api/handlers/remediate.rs:173-179`: route the two new codes.

REMOVED:
- The implicit "directory means Rust" assumption. No public API removed.

## Guards

- `tests/` (scanner): add cases for (a) `.ts` directory -> TypeScript, (b)
  mixed-extension directory -> dominant, (c) empty/unsupported dir ->
  `Unknown` + warning + no hash, (d) `targets:` override still wins over
  inference. Cover both a fresh scan and a cached scan (cache round-trip
  preserves `None`).
- Wire-format snapshot for `cairn files --json` updated as a recorded
  change (hash absent for unknown/empty targets), not a silent drift.
- Self-host: `cairn scan` on this repo must keep all Rust node hashes
  identical (inference must not perturb file-target language detection).
