---
node: cairn.root
status: open
created: 2026-07-27
---

# Release v0.9.0

## Priority

P0. The shipped v0.8.0 binary has two defects, both reproduced by building
the `v0.8.0` tag and running it: `cairn init` scaffolds `AGENTS.md` and
`state/` outside the project through a symlinked `.cairn`, and
`cairn init --wire` hangs forever on a FIFO at `AGENTS.md`. The first was
already fixed on main; the second is fixed in this unit.

## Scope

1. Bump `Cargo.toml` to `0.9.0`. Minor under the 0.x policy: new features and
   behaviour change, no breakage.
2. Add a `CHANGELOG.md` `## v0.9.0` section in the existing house style.
   Derive it from the artefacts, not from commit subjects: the `#484` subject
   line says "every project write", which is wider than the three surfaces
   `todo.pack-path-containment` actually audited.
3. Refresh README `## Status`, which never mentions `cairn pack`.
4. Add a dated README `## Roadmap` that links to `cairn frontier` for live
   detail, so it cannot silently become the source of truth.
5. Fix what release review surfaced rather than shipping notes around it:
   `cairn init --wire` refuses a non-regular instructions file, and an owned
   pack file that is present but unreadable is reported as modified rather
   than classified as missing and overwritten.
6. Land as one PR, then tag `v0.9.0` to trigger cargo-dist, Homebrew, and the
   crates.io publish.

## Acceptance

- `cairn --version` reports 0.9.0.
- The changelog names the six new `cairn pack` verbs, both harness adapters
  with an honest statement of which one has retained live-host evidence, and
  the containment fix scoped to its audited surfaces.
- `init --wire` against a FIFO exits non-zero instead of hanging, and an
  unreadable owned file survives `pack update`. Both have regression tests.
- The GitHub Release, the crates.io version, and the Homebrew formula all
  show 0.9.0.
- `cargo package` contains every embedded `src/ui_assets` file.
