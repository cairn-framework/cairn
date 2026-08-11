---
name: cairn-explore
description: Explore the Cairn graph and query project state: architecture, nodes, status, findings, and the decisions and provenance behind a node. Use when the user wants to understand structure, check health, find nodes, inspect findings, or ask why a node is shaped the way it is.
license: MIT
compatibility: Requires Cairn CLI.
metadata:
  author: cairn
  version: "1.0"
  generatedBy: "1.0"
---

Explore the Cairn architecture graph.

**Commands**

- `cairn status` - project summary (nodes, edges, findings)
- `cairn get <node>` - detailed info for a node
- `cairn neighbourhood <node>` - dependencies and dependents
- `cairn files <node>` - files owned by a node
- `cairn islands` - disconnected components
- `cairn lint` - findings across the project (`--node <id>` scopes to one node); exits 1 when an Error finding is present
- `cairn ui` - visual graph explorer in a browser
- `cairn rationale <node>` - accepted decisions and provenance chain behind a node (why it exists)
- `cairn decisions <node>` - decision artefacts attached to a node
- `cairn research <node>` - research artefacts linked to a node
- `cairn sources <node>` - external sources a node cites
- `cairn change list --json` - registered change proposals (the `--json` flag is required)

**Steps**

1. **Pick the target**: ask when the request names none:
   > "What would you like to explore? A specific node, the overall health, or something else?"
2. **Run the matching command(s)** and show the output.
3. **Interpret the results** in plain English:
   - `synced` - node matches the filesystem
   - `ghost` - declared in blueprint but missing on disk
   - `orphaned` - exists on disk but not declared
   - `drift` - declared but contradicts observed state
   - Findings severity: Error > Warning > Info
4. **Suggest the next step**: `cairn scan` to refresh the graph, `cairn lint` for
   detailed findings, a blueprint edit for structural issues, or `cairn refine`
   to generate a change proposal from code changes.

**Provenance: why a node is shaped the way it is**

For "what decisions affect node X" or "why was X built this way", query
`cairn rationale <node>` first: it returns the node's accepted decisions with
their provenance links (`informed_by`, `related`, `refines`, `supersedes`), so
there is no need to grep `meta/decisions/` by hand. `cairn decisions <node>`
adds decisions in other states (proposed, deprecated, superseded); `--json`
gives the structured fields. The reconciled graph is the authority: when an
auto-generated guidance block contradicts an accepted decision, the decision
cairn returns wins.

**Where cairn cannot help: read the source**

The graph models blueprint structure plus artefacts (decisions, research,
contracts, todos), not source symbols. For enum variants, struct fields,
function definitions, or call sites, read the source files directly rather
than expecting a cairn query to answer.
