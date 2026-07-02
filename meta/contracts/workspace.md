---
node: cairn.workspace
---

# Contract: cairn.workspace

## Purpose

Multi-project workspace aggregation (`dec.workspace-aggregation`). A
`cairn.workspace` TOML file declares a set of independent member projects,
each with its own `cairn.blueprint`. `cairn workspace <status|lint|frontier>`
loads every member via the existing `scanner::load_project` and aggregates
the results into one view: `status` sums node/edge/finding counts per
member, `lint` prefixes each member's findings with `<project>: `, and
`frontier` qualifies buildable-now / blocked node IDs `<project>:<node>`.
This is a thin enumeration and result-aggregation layer, not a new graph
model: each member's blueprint, graph, and reconciliation stay fully
independent (no shared graph, no cross-project edges; that is a v1 scope
boundary, not a placeholder).

## Public interface

- `Workspace::load(path) -> Result<Workspace, String>`: parses a
  `cairn.workspace` TOML file (`[[project]] name / root`); member `root`
  paths resolve relative to the workspace file's own directory.
- `status(&Workspace) -> WorkspaceStatus`: one `MemberStatus` row per member
  (nodes, edges, error/warning/info counts, or `missing` when the member
  failed to load) plus a summed `totals` row.
- `lint(&Workspace) -> Vec<Finding>`: every member's findings with the
  message prefixed `<project>: `; a member that fails to load contributes
  one `CAIRN_WORKSPACE_MEMBER_MISSING` finding instead.
- `frontier(&Workspace) -> WorkspaceFrontier`: every member's
  `query::frontier` result, with `FrontierEntry.node` and `.blocking`
  qualified `<project>:<node>`; a member that fails to load or is cyclic
  contributes to `WorkspaceFrontier.findings` and is skipped.

## Invariants

- A member whose root or blueprint fails to load never aborts the
  aggregate: it contributes a `CAIRN_WORKSPACE_MEMBER_MISSING` (Error)
  finding for that member, and the loop continues with the remaining
  members.
- `status` totals are the arithmetic sum of every member's counts,
  including members that failed to load (which contribute 0 nodes/edges,
  1 error).
- No cross-project blueprint edges exist or are inferred; each member's
  scan is fully independent.

## Dependencies

Leaf with no outgoing blueprint edges. Depends on `cairn.kernel.scanner`
(`scanner::load_project`, per member) and `cairn.kernel.map`
(`query::frontier`, per member, for the `frontier` subcommand only); no
blueprint `->` edge is declared since it is invoked from the CLI layer
(`src/cli/commands/workspace.rs`), mirroring `cairn.watch`.

## Tests

Unit tests are colocated in a `#[cfg(test)]` module in `src/workspace/mod.rs`
(TOML parsing, two-member `status` summation, missing-member error handling,
`frontier` node qualification) and in `src/cli/commands/workspace.rs`
(CLI dispatch: usage error, missing `cairn.workspace` file, single-member
`status` human output, missing-member JSON output, `frontier` qualification).
