---
id: dec.workspace-aggregation
nodes:
  - cairn.kernel
  - cairn.workspace
status: accepted
date: 2026-07-02
informed_by: [res.vision-refactor-audit]
revisit_triggers:
  - "A concrete cross-project blueprint edge use case appears (e.g. a monorepo module depending on a sibling repo's node); revisit whether workspace v2 needs edge semantics"
---

# Workspace aggregation: enumeration and aggregate queries, no cross-project edges

## Context

Cairn's structural authority is single-project (one `cairn.blueprint`, one
graph). Projects that manage several related cairn-tracked repos (or several
independently-scanned subprojects) currently have no way to ask "what does
this whole workspace look like" without shelling out per project and
concatenating output by hand.

## Decision

`cairn.workspace` (TOML, `[[project]] name / root`) declares a set of member
projects, each with its own independent `cairn.blueprint`. `cairn workspace
<status|lint|frontier>` loads each member via the existing
`scanner::load_project` and aggregates: `status` sums nodes/edges/findings per
member, `lint` prefixes each member's findings with `<project>:` and applies
the existing `--strict` exit-code logic across the union, `frontier` renders
each member's `cairn frontier` (`dec.frontier-query`) with node ids qualified
`<project>:<node>`. A missing member root or blueprint contributes one
`CAIRN_WORKSPACE_MEMBER_MISSING` error finding and the loop continues with the
remaining members.

**Workspace v1 is aggregation-only.** No cross-project blueprint edges, no
shared graph, no cross-project dependency resolution. This is a scope
boundary, not a placeholder: no semantics have been established for what a
cross-project edge would mean (which project owns drift detection? which
scan's cache invalidates the other's?), and inventing them speculatively is
exactly the kind of underspecified guess `dec.generative-bundles-and-gaps`'s
gap protocol exists to avoid. If cross-project edges become a real need, that
gap gets logged and resolved deliberately, not designed in advance.

## Rationale

Each member's blueprint, graph, and reconciliation stay fully independent:
`cairn.workspace` is a thin enumeration and result-aggregation layer over
`scanner::load_project`, not a new graph model. This keeps the single-project
invariant (one blueprint, one source of truth per repo) intact while giving a
multi-repo operator one command instead of N.

## Consequences

- `src/workspace/mod.rs` has no dependency on `src/map/graph.rs` beyond what
  each member's own scan already produces; workspace commands are a fold over
  N independent scans.
- Depends on `dec.frontier-query` for the `frontier` subcommand only;
  `status`/`lint` do not depend on any other phase 1–5 decision.
