---
id: dec.herdr-dashboard-integration
nodes:
  - cairn.root
status: accepted
date: 2026-07-25
informed_by:
  - res.herdr-plugin-feasibility
---
# Herdr dashboard integration: a read-only pane over cairn ground truth, not a cairn feature

## Context

`res.herdr-plugin-feasibility` (2026-07-17) ran live probes against a real herdr
session to answer whether cairn should grow a herdr plugin. Its recommended
increment path was a hybrid standalone pane process: consume `cairn watch` for
finding deltas, poll `cairn status --json` on a timer for backlog and next-step
state, and surface both through `herdr pane report-metadata`. The reasoning was
that `cairn watch` emits finding events only (probe `wM:pB` produced
`finding_added` and `finding_resolved` and never a backlog event), so a
watch-only design cannot satisfy the requirement that findings AND backlog
counts match `cairn lint --json` and `cairn status --json` at read time.

That path shipped. `scripts/herdr-cairn-dashboard.py` is the dependency-free
pane process and `docs/herdr-dashboard.md` documents it. The commitment was
therefore already made and running, but no decision artefact recorded it, which
left the research orphaned from the authority chain
(`CAIRN_RESEARCH_ORPHAN`).

## Decision

Herdr integration stays outside cairn: a checked-in script that reads the cairn
CLI, never a cairn subsystem, module, or shipped plugin surface. Three rulings
follow.

1. **The pane is a consumer, not a feature.** It lives in `scripts/`, depends
   only on the python3 standard library, and calls the public JSON surfaces
   (`cairn lint --json`, `cairn status --json`, `cairn watch`). No blueprint
   node, no Rust code, and no cairn release artefact is added for it. This keeps
   `dec.no-orchestrator` intact: cairn is driven, it does not drive.
2. **Ground truth is re-derived, never cached.** Every rendered snapshot re-runs
   both commands and stamps the collection time and a monotonic counter.
   Triggers (`cairn watch` deltas, the status poll, a timer) decide WHEN to
   render, never WHAT is displayed.
3. **Orchestrator claims are a labelled overlay.** The optional
   `CAIRN_DASH_OVERLAY` layer renders under an explicit
   "ORCHESTRATOR CLAIMS (overlay, unverified)" heading and never enters the
   sidecar or herdr metadata. Cairn state and orchestrator assertions stay
   visibly separate.

## Rationale

The alternative, a first-class cairn herdr plugin, was rejected by the research
on evidence: herdr exposes no stable plugin contract the probes could bind to,
and the pane needs two different cairn surfaces with different update shapes.
A consumer script keeps the coupling one-directional and disposable. If herdr
changes, one script changes and cairn is untouched.

Recording this as `accepted` documents a commitment already shipped rather than
opening a new one.

## Consequences

1. `res.herdr-plugin-feasibility` is now linked into the authority chain, so the
   orphan finding clears without fabricating provenance.
2. Cairn's public JSON surfaces (`lint --json`, `status --json`, `watch`) are
   load-bearing for this consumer. Changing their shape is a breaking change for
   the dashboard, though not for cairn itself.
3. `todo.herdr-cairn-tool-attribution` stays `blocked` and is unaffected: tool
   attribution is a separate question from the dashboard's read path.
4. No orchestration capability is implied. The overlay renders claims supplied
   by an external runner and verifies none of them.
