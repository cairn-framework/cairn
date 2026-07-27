---
node: cairn.root
status: done
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

## Outcome

Released 2026-07-27 from `66af1e3` (PR #487), tagged `v0.9.0`. Every
acceptance line was verified against the live artefacts, not inferred from the
pipeline exiting green:

- `cairn --version` reports `cairn 0.9.0`; `cairn scan` holds at its one
  deliberate deferred finding and `cairn hook all` exits 0 on merged `main`.
- `CHANGELOG.md` names all six `cairn pack` verbs, distinguishes OMP's
  retained live-host record from Claude's dogfood evidence, and scopes the
  containment fix to its audited surfaces.
- The FIFO and unreadable-file regressions are covered by
  `tests/pack_path_containment.rs` plus the direct `wire_agent_guide` unit
  test in `src/cli/commands/wire.rs`.
- GitHub Release `v0.9.0` is published, not a draft, carrying 18 assets.
- crates.io reports `max_version` and `newest_version` of `0.9.0`.
- The Homebrew tap formula reads `version "0.9.0"`.
- `cargo package` produced `cairn-framework-0.9.0.crate` with every embedded
  `src/ui_assets` asset present. The extensionless `src/ui_assets/api/*`
  fixtures are correctly excluded: nothing embeds them.

Review expanded a notes-and-version-bump unit into two code fixes: the
`init --wire` FIFO hang, which was reproduced against a binary built from the
`v0.8.0` tag and so was a second shipped defect, and the unreadable owned pack
file, which was reachable but unreleased.

Three reviewer passes produced 21 findings: 19 were a claim written wider than
its evidence, and 2 were regression-test quality. Fourteen of the 21 were
anchored to the draft release notes, including "both validated against a live
host" when only OMP has a retained record
(`res.pack-omp-adapter-validation`). All were resolved before tagging. The
notes are derived from artefacts rather than commit subjects for exactly this
reason: the merged `#484` subject line says "every project write", which is
wider than the surfaces `todo.pack-path-containment` actually audited.
