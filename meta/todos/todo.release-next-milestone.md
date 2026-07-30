---
node: cairn.root
status: open
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
