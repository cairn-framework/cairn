---
node: cairn.brownfield
status: open
created: 2026-08-09
---

# Manifest-derived `node_modules` ignore entries are redundant

Filed from the Arm A brownfield stress test over TrySita/AutoDocs
(`res.autodocs-arm-a-brownfield-run`, source `src.autodocs`).

## Evidence

`collect_initial_ignore_candidates` (`src/cli/mod.rs:188-198`) emits
`<dir>/node_modules` for every directory containing a `package.json`, with no
check that the directory exists.

On a clean AutoDocs clone this writes 15 entries into the adopter's
`cairn.config.yaml`. All 15 name directories that do not exist: the clone has
zero `node_modules` on disk and zero tracked under one.

They are redundant regardless of existence. `node_modules` is already in the
scanner's built-in ignore list (`src/scanner/config/mod.rs:155-159`) and is
hard-coded in discovery's `is_ignored_dir`
(`src/brownfield/discovery.rs:188-194`). A path-specific entry adds nothing on
top of those, so existence is the wrong axis to fix on: existence-gating would
leave exactly the same noise on any repository where dependencies are
installed.

Net effect on first run: 15 lines of config the adopter cannot verify and did
not need.

Note on evidence quality: the Arm A run tried and failed to demonstrate the
redundancy behaviourally. Injecting a `node_modules` tree left the finding count
unchanged, but so did an identically placed control under a non-ignored name, so
that run cannot separate the ignore machinery from what the reconciler
enumerates. Treat the redundancy as established by the two source facts above,
and build a purpose-made fixture if a behavioural assertion is wanted.

## Scope

`node_modules` reaches `cairn.config.yaml` by two independent routes, and both
must be closed or the Acceptance below cannot hold:

- The manifest route: `collect_initial_ignore_candidates` emits
  `<dir>/node_modules` for every directory holding a `package.json`
  (`src/cli/mod.rs:188-198`). This fires whether or not the directory exists.
- The classifier route: for a directory that does exist,
  `ignore_suggestion_for_path` (`src/cli/mod.rs:210-213`) runs the onboard
  classifier, whose `IGNORE_PATTERNS` includes `node_modules`
  (`src/brownfield/onboard.rs:11-18`), and the suggestion is inserted.

Suppress `node_modules` from both. Keep the general suggestion mechanism
intact: classifier-derived suggestions for other patterns (`dist`, `build`,
`vendor`, `__pycache__`) are still worth writing, and this todo does not touch
them.

## Acceptance

- A test asserts `cairn init --from-code` over a tree containing a
  `package.json` emits no `node_modules` ignore entry, in both the case where
  the sibling `node_modules` directory is absent and the case where it exists.
- Re-running Arm A writes no `node_modules` entries into `cairn.config.yaml`.
