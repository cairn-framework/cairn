---
node: cairn
status: done
created: 2026-07-30
---

# Cairn Mission

Record the ratified mission as first-class provenance on the root System node
and surface it in default output.

## Origin

Maintainer chat approval, 2026-07-30, during the supervised B-queue session:
the mission statement was ratified verbatim for landing as an accepted
decision on the root System node.

## Scope

- `meta/sources/mission-ratification-2026-07-30.md`
  (`src.mission-ratification-2026-07-30`) records the ratification event. A
  chat session is not addressable from the repository, so the record carries
  `file: null`, `verification: unverified`, `type: conversation`; the one
  standing `CAIRN_SOURCE_UNVERIFIED` Info finding this produces is expected
  and tolerated by `cairn scan --strict`.
- `meta/decisions/cairn-mission.md` (`dec.cairn-mission`): `nodes: [cairn]`,
  `status: accepted`, `informed_by: [src.mission-ratification-2026-07-30]`,
  `related: [dec.north-star-continuous-loop]`. The body carries the verbatim
  mission statement, the four properties as formal project terms, the
  mechanism, the limit, the reframing of `dec.north-star-continuous-loop` as
  an operational strategy subordinate to the mission, and the authoring
  rubric that decision requires.
- `cairn.blueprint`: the System node description string becomes the condensed
  mission line. No new blueprint schema fields.
- `cairn context`: default output prints the System description (the mission
  headline) plus a `Pending signatures: <count>` line counting decisions at
  `status: proposed`, with the string sourced from
  `docs/design-system/copy.toml`.
- `docs/index.html`: the two user-facing mirrors (the blueprint snippet and
  the `cairn context` example block) are updated to match.

Non-goals: the bootstrap fixture, harness fixtures, and studio mocks stay as
recorded; `cairn pending` is unchanged; no new frontmatter or blueprint
fields anywhere.

## Depends on

Nothing. `dec.north-star-continuous-loop` stays accepted and is linked via
`related:`, not superseded.

## Acceptance

- `cairn decisions cairn` lists `dec.cairn-mission` as accepted.
- `cairn get cairn` prints the condensed mission description.
- `cairn context` prints the mission headline and a `Pending signatures:`
  line whose count equals the number of decisions at `status: proposed`,
  covered by a unit test exercising both the zero and the counted case.
- `cairn scan --strict` exits 0; the only new finding is the expected
  `CAIRN_SOURCE_UNVERIFIED` Info on the ratification source.
