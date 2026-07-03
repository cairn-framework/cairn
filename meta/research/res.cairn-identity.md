---
id: res.cairn-identity
nodes:
  - cairn.root
date: 2026-07-03
method: primary
---

# Cairn's identity: map versus reconciliation controller

A brainstorm prompted by a maintainer observation ("cairn feels more like
something else than a map") evaluated seven candidate identities for what
cairn *actually is*, mechanically, against the spec's own description of its
parts (`docs/spec.md` §2 blueprint, §3 scan/reconcile, §9 gates, §10-11
provenance/changes) and the standing no-orchestrator architecture
(`dec.no-orchestrator`).

## Candidates evaluated

- **Map.** Descriptive: a static rendering of structure. Under-describes
  cairn because the blueprint is *prescriptive* (a declared intent the
  scanner is checked against) and *gated* (drift blocks the commit/task
  boundary); a map has no such enforcement relationship to the territory.
- **Ledger.** Closer on provenance (append-only decision/research/source
  chains via `informed_by`), but a ledger has no notion of a live sensor
  reading compared against a setpoint, and doesn't explain why gates block.
- **Version control for intent.** Explains the change system (`meta/changes/`,
  `cairn accept`) but not the reconcile/scan/finding loop, which has no
  analogue in git.
- **Control system.** See "truest identity" below.
- **Phase space.** The frame the "more like something else" instinct
  initially reached for: cairn's graph as a space of possible
  architecture-states. Kinematic only: describes where the system *could*
  be, not the dynamics that push it toward or away from a declared target,
  and has no natural home for provenance (why a state was chosen).
- **Contract law.** Explains leaf contracts and the gate/violation
  vocabulary, but not the sensor (scan) or the actuator boundary
  (deliberately external).
- **Immune system.** Explains drift *detection* (scan-as-surveillance) but
  wrongly implies cairn *acts* on what it detects; `dec.no-orchestrator`
  makes acting-on-findings deliberately external.

## Truest identity: a declarative reconciliation controller

Cairn is a control loop whose setpoint is itself a versioned,
provenance-bearing artefact:

- **Setpoint** = the blueprint (`cairn.blueprint`): the declared,
  authored intent for what the architecture should be.
- **Sensor** = the scanner (`cairn scan`): reads the actual code on disk.
- **Error signal** = findings: the diff between setpoint and sensor
  reading (drift, gaps, orphans, integrity violations).
- **Boundary** = the commit/task gate (`cairn hook`, `cairn accept`):
  where the error signal is allowed to block progress.
- **Actuator** = deliberately external (a human or an agent, never cairn
  itself), the one component a conventional control system would
  automate, and the one `dec.no-orchestrator` explicitly refuses to own.

This frame subsumes every other candidate rather than competing with them:
the map is the sensor's readout: presentation over the reconciliation
result, not the mechanism itself; the ledger is the reconciliation step's
audit trail (why a setpoint changed); the "version control for intent"
observation is setpoint-versioning; contract law is the boundary's
vocabulary. It also *predicts* design decisions the other frames only
describe after the fact: why gates block (a controller enforces a
boundary, it doesn't merely observe), why there is no orchestrator (a
controller's actuator is out of scope by definition, `dec.no-orchestrator`
Layer 3), and why the summariser never auto-applies a draft (the actuator
boundary is inviolable regardless of which component is proposing the
action).

## The physicists' instinct was right, the prescription was wrong

The "more like something else than a map" intuition is correct: a map is
descriptive, and cairn's blueprint is prescriptive and gated, so "map"
under-describes the mechanism. But the phase-space frame that instinct
reached for is the wrong destination: it is kinematic (a space of possible
states) with no dynamics (nothing that models cairn *pushing* the system
toward the setpoint) and no home for provenance (a phase space has no
concept of "why this point was chosen over that one"). The
declarative-reconciliation-controller frame keeps the physicists'
correct diagnosis and supplies the missing mechanism.

## Verdict: keep the map public, adopt the controller internally

The public "map" framing does not change. A map names the user's pain
(illegible architecture) and relief (a navigable rendering of it) in one
breath; a colder, mechanically-truer phrase would tax comprehension at
the most expensive moment, the first-run pitch. The controller identity
lives internally: one integration sentence in `docs/spec.md`'s intro and
one in `README.md`'s "How it works" section (both landed alongside this
research artefact). Hero copy, tagline, and the landing page are
unchanged.
