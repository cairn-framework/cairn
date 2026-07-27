---
id: dec.cairn-identity
nodes:
  - cairn.root
status: accepted
date: 2026-07-03
informed_by:
  - res.cairn-identity
---
# Cairn's Identity: Map Stays Public, Controller Ships Internally

## Context

A maintainer observation ("cairn feels more like something else than a
map") prompted `res.cairn-identity`, a brainstorm evaluating seven
candidate identities for cairn against its actual mechanics (`docs/spec.md`
§2/§3/§9/§10/§11, `dec.no-orchestrator`). The brainstorm concluded cairn's
truest identity is a declarative reconciliation controller: blueprint as
setpoint, scan as sensor, findings as error signal, the commit/task gate as
boundary, and a deliberately external actuator. The map framing under-
describes cairn (a map is descriptive; cairn's blueprint is prescriptive
and gated) but a colder, more mechanically accurate identity risks taxing
comprehension at the first-run pitch, the most expensive moment to lose a
reader.

## Decision

1. **The public "map" framing is unchanged.** Hero copy, tagline, and
   `docs/landing/index.html` are not touched by this decision.
2. **Exactly two integration sentences ship**, naming the internal
   identity without replacing the public one:
   - `README.md` "How it works", after the pipeline diagram, before the
     numbered walkthrough.
   - `docs/spec.md` §2 ("Framing: map, not procedure"), as the closing
     sentence of that section.
   No other prose in either document changes.

## Rationale

A map names the user's pain (illegible architecture) and its relief (a
navigable rendering of it) in one breath; that framing does its job at
the point of first contact and should not be traded for mechanical
precision there. The controller identity is real and useful: it explains
*why* cairn is shaped the way it is (why gates block, why there is no
orchestrator, why a drafted summary never auto-applies), so it belongs
where a reader is already past the pitch and wants the mechanism: the
spec's own framing section and the README's explanation of how the tool
works.

## Consequences

- `res.cairn-identity` is no longer an orphaned research artefact; this
  decision is its citing decision.
- Any future prose describing cairn's mechanism (not its pitch) should be
  consistent with the controller framing (setpoint/sensor/error
  signal/boundary/actuator) rather than inventing a competing metaphor.
- If the map framing itself is ever revisited (a future decision), it
  supersedes this one; until then the split (public map, internal
  controller) stands.
