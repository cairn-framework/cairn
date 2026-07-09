# Proposal: Fix directory language inference defaulting to Rust

## Motivation

Issue #215 (cairn-framework/cairn): a module `path "./lib"` pointing at a
directory of TypeScript files is reconciled as Rust, discovers zero files,
and reports a stable interface hash as if the reconcile succeeded.

Root cause is `src/scanner/mod.rs:96`:

    let language = Language::from_extension(&path).unwrap_or(Language::Rust);

A directory has no extension, so `Language::from_extension`
(`src/reconcile/target.rs:49`) returns `None` and every directory target
silently becomes Rust. The Rust reconciler then walks `.rs` files only
(`src/reconcile/code.rs:282`), so `lib/main.ts` is invisible. The
reporter's `language: rust` / empty `lib:` / non-empty `hash` output is the
expected result of this path, not a transient failure.

Two defects share this root:

1. Silent Rust default on undeterminable language (the inference bug).
2. A target that discovers zero files still emits a fingerprint
   (`InterfaceFingerprint::from_symbols(&[])` yields a stable hash,
   `bd60acb658c79e45` for empty input), so a failed reconcile looks
   successful.

This is a design flaw, not just a one-liner: defaulting to any single
language when inference fails converts "I don't know" into a
plausible-looking fiction. The fix makes an undeterminable language a
visible state instead of a silent Rust pretense.

## Scope

- Replace the `unwrap_or(Language::Rust)` default with content-based
  inference: sample the target directory's files and pick the dominant
  supported language.
- When language is still undeterminable (empty dir, only unsupported
  extensions), emit a warning finding instead of silently reconciling as
  Rust.
- Suppress the interface hash for targets with zero discovered files; emit
  a finding so the failure is visible.
- A user-declared `targets:` language override (already supported via
  `build_targets`, `src/scanner/mod.rs:74-86`) continues to take
  precedence over inference.

## Out of scope

- Implementing the `reconcilers:` / `tree_sitter_languages` config block
  documented in `docs/spec.md` (separate change: `reconcile-config-schema`).
- Extracting a generic parameterized reconciler to make language extension
  cheap (separate change: `generic-language-reconciler`).
- Adding new languages (e.g. Dart).

## Acceptance

- A module pointing at a directory of `.ts` files reconciles as TypeScript
  and discovers those files, with no `targets:` override required.
- A directory whose contents are all unsupported extensions (or empty)
  produces a warning finding naming the target, and emits no interface
  hash.
- The cairn self-host scan (Rust) is unchanged: existing `target_reports`
  and interface hashes for Rust nodes are byte-identical.
- `cargo test` + `cairn scan` + `cairn hook all` pass.
