---
node: cairn.root
status: done
created: 2026-07-30
---

# Release at the next good milestone

## Priority

P2, milestone-gated: no shipped defect forces a cut (contrast
`todo.release-v0-9-0`, which was P0 on two reproduced v0.8.0 defects). The
trigger is the milestone below becoming true, not a date.

## Milestone gate

Cut when all of these hold on a clean checkout of `main`:

1. `cairn scan --strict` and `cairn hook all` both exit 0.
2. The signature queue is quiet: `cairn pending` lists zero proposed
   decisions, or every remaining row is deliberately parked and says so in
   its rubric.
3. No in-flight unit is mid-way through an artefact-schema or shipped-pack
   change: a half-landed schema teaches adopters a rule the binary does not
   enforce yet. As of authoring that means the ratification-tiers programme
   has landed or been explicitly re-scoped out of the cut by the maintainer.
4. The maintainer confirms the semver bump at cut time under the 0.x policy;
   the features and behaviour changes merged since v0.9.0 already mean at
   least a minor bump.

## Scope

1. Version bump in `Cargo.toml`, a `CHANGELOG.md` section in house style, and
   a README `## Status` plus dated `## Roadmap` refresh. Derive the changelog
   from merged artefacts (todos, decisions, change archives), never from
   commit subjects, with claims no wider than their artefact evidence. Landed
   since v0.9.0 as of authoring: the maintainer pending queue (#536), typed
   `defers:` parking (#533), the strict-green selection fold (#532),
   accepted-only `deferred_by` selection (#531), source `verification:
   tracked` mode (#537), the source self-reference finding (#539), the
   bootstrap fixture corpus split (#535), `dec.cairn-mission` plus the
   `cairn context` mission headline (#540), and the provenance and rubric
   captures (#528, #530, #541, #542). Add whatever lands before the cut.
2. Land as one PR through the full gates, then tag to trigger the
   `release.yml` and `cargo-publish.yml` workflows.
3. Verify after, against the live artefacts, per the `todo.release-v0-9-0`
   Outcome pattern: published GitHub Release with assets, crates.io version,
   Homebrew formula version, `cargo package` embedding every
   `src/ui_assets` file, and a refreshed local install.

## Acceptance

- `cairn --version` on the tagged build reports the new version; scan and
  hook are green on merged `main`.
- The changelog names every landed surface with claims scoped to artefact
  evidence (the v0.9.0 lesson: 14 of 21 review findings were claims written
  wider than their evidence).
- GitHub Release, crates.io, and the Homebrew tap agree on the version.
- The milestone gate above was checked and true at the cut commit, and the
  check results are recorded in this todo's Outcome section.

## Origin

Maintainer request, 2026-07-30: add a todo for performing a new release at a
good milestone.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It keeps releases tied to an explicit milestone.

## Gate check (2026-08-03): NOT MET, gates 1 and 2 fail

Checked against `main` at `b9d7c8c` with a freshly built binary. Rebuilding
matters: a stale `target/release/cairn` reports the same tree as clean, so a
stale binary can hide the blocker below; how it landed is that todo's open
question.

1. **Gate 1: FAIL.** `cairn scan --strict` exits 1 and `cairn hook all`
   exits 1 on one Error finding,
   `CAIRN_DECISION_CONVERGENCE_UNMET` against
   `dec.bootstrap-fixture-corpus-split`. `cargo test --locked --workspace`
   fails two tests in `tests/schema_validation.rs` for the same cause.
   Filed as `todo.convergence-receipt-hash-drift`, which is now the
   blocking prerequisite for this cut.
2. **Gate 2: FAIL.** `cairn pending` lists one proposed decision,
   `dec.orchestration-placement`. It is a binding record that supersedes
   `dec.product-perimeter`, so only the maintainer can sign it, and it is
   not parked: `todo.driver-in-repo` waits on it. Gate 2 clears when it is
   signed or deliberately parked with that stated in its rubric.
3. **Gate 3: PASS.** The ratification-tiers programme landed (PR #544,
   archived as `2026-07-31-decision-ratification-tiers`). The one active
   change, `driver-v2-selection`, touches no artefact schema and no
   shipped pack: its declared delta is a single research artefact and
   nothing under `src/`.
4. **Gate 4: OUTSTANDING.** Maintainer confirmation of the bump. The
   surface count since v0.9.0 is a minor bump under the 0.x policy.

Other gate work still owed at cut time, unchanged by this check: the
version bump, the changelog section derived from merged artefacts, the
README status and roadmap refresh, and the full post-tag verification of
GitHub Release, crates.io, and the Homebrew formula. The public asset
staleness in `todo.ui-asset-refresh` is deliberately still blocked: the
webui changes again under `todo.console-state-legibility` and
`todo.console-signed-widening`, so re-recording now means recording
twice.

Add to the changelog scope, landed since the list above was written:
`dec.control-plane-programme` and the read-only three-lane console
(#572), and `dec.reverse-provenance-wire` with computed `refined_by` and
`superseded_by` on schema version 11. `dec.orchestration-placement` joins
that list only if it is signed before the cut.

cairn.root anchor justified (2026-08-07): release cut is crate-wide process (Cargo.toml, CHANGELOG).

2026-08-07 audit (todo.roadmap-assumption-audit): keep; refresh the gate inventory at release time. Milestone gate 1 (convergence receipt drift) is resolved: main is strict-green as of 2026-08-07.

## Gate check (2026-08-19): MET, cutting v0.10.0

Checked against `main` at `23332d9c` (after merging #706) with a freshly built
binary. 158 commits landed since `v0.9.0`.

1. **Gate 1: PASS.** `cairn scan --strict` and `cairn hook all` both exit 0.
   `cargo fmt --check`, `RUSTFLAGS=-D warnings cargo clippy --workspace
   --all-targets --all-features`, `cargo test --workspace`, and
   `scripts/check-file-sizes.sh` all pass.
2. **Gate 2: MET.** Two decisions were proposed at cut time:
   `dec.autodocs-arm-a-item-7-correction` (binding, supersedes the
   maintainer-signed `dec.autodocs-head-to-head-arm-b`) was ratified by the
   maintainer in session and accepted; `dec.brownfield-package-root-discovery`
   (local) is deliberately parked with its rubric, its behaviour already shipped
   in #669 with tests, and its acceptance tracked by
   `todo.brownfield-package-root-discovery-ratification` (a session with a second
   distinct reviewer model is needed for the convergence panel).
3. **Gate 3: PASS.** The one active non-pilot change, `driver-v2-selection`,
   touches no artefact schema and no shipped pack. The decision-ratification
   tiers regime has landed.
4. **Gate 4: PASS.** Maintainer confirmed a 0.10.0 minor bump in session.

The changelog is derived from merged artefacts (decisions, todos, change
archives) with PR numbers as locators, and the four post-tag subsections that
had been misfiled under `v0.9.0` (parked findings, strict-green fold, deferral
publication, change lifecycle read surface) were relocated to `v0.10.0`.

## Outcome

Released 2026-08-19 from `88688b1b` (PR #708), tagged `v0.10.0`. The tag
triggered `release.yml`; every job succeeded (build matrix across five targets,
host, `custom-cargo-publish`, `publish-homebrew-formula`, and announce).
Verified against the live artefacts, not inferred from the pipeline exiting
green:

- `cairn --version` reports `cairn 0.10.0`; the local install was refreshed from
  the tagged source (`cargo install --path . --force`).
- GitHub Release `v0.10.0` is published, not a draft, carrying 18 assets.
- crates.io reports `cairn-framework` `0.10.0` (created 2026-08-19, not yanked).
- The Homebrew tap formula reads `version "0.10.0"`.
- `cargo package --list` embeds all 18 `src/ui_assets` files.
- On merged `main`, `cairn scan --strict` and `cairn hook all` exit 0.

This todo tracked releasing at the next good milestone rather than on a date, and
that milestone is now shipped. One follow-up remains tracked separately:
`dec.brownfield-package-root-discovery` was parked `proposed` at the cut (its
behaviour shipped in #669 with tests) and its convergent-receipt ratification is
carried by `todo.brownfield-package-root-discovery-ratification`.
