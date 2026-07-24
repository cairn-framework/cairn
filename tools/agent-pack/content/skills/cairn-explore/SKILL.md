---
name: cairn-explore
description: Explore the Cairn graph and query project state: architecture, nodes, status, findings, and the decisions and provenance behind a node. Use when the user wants to understand the structure, check health, find nodes, inspect findings, or ask what decisions affect a node or why it is shaped the way it is.
license: MIT
compatibility: Requires Cairn CLI.
metadata:
  author: cairn
  version: "1.0"
  generatedBy: "1.0"
---

Explore the Cairn architecture graph.

I'll help you understand your project's structure by querying the reconciled graph.

**Available commands**

- `cairn status` - Show project summary (nodes, edges, findings)
- `cairn get <node>` - Show detailed info for a node
- `cairn neighbourhood <node>` - Show dependencies and dependents
- `cairn files <node>` - List files owned by a node
- `cairn islands` - Show disconnected components
- `cairn lint` - Report findings across the project
- `cairn lint --node <id>` - Check findings (inspection mode, always exits 0)
- `cairn ui` - Open the visual graph explorer in a browser
- `cairn rationale <node>` - Show the accepted decisions and provenance chain behind a node (why it exists)
- `cairn decisions <node>` - List decision artefacts attached to a node
- `cairn research <node>` - List research artefacts linked to a node
- `cairn sources <node>` - List external sources a node cites
- `cairn change list --json` - List registered change proposals (the `--json` flag is required)

**Steps**

1. **Understand what the user wants to explore**

   Ask if unclear:
   > "What would you like to explore? A specific node, the overall health, or something else?"

2. **Run the appropriate Cairn command(s)**

   Execute the command and show the output.

3. **Interpret the results**

   Explain what the output means in plain English:
   - `synced` - node matches the filesystem
   - `ghost` - declared in blueprint but missing on disk
   - `orphaned` - exists on disk but not declared
   - `drift` - declared but contradicts observed state
   - Findings severity: Error > Warning > Info

4. **Suggest next steps**

   Based on the findings, suggest actions:
   - Run `cairn scan` to refresh the graph
   - Run `cairn lint` for detailed findings
   - Edit the blueprint to fix structural issues
   - Use `cairn refine` to update the blueprint from code changes

**Provenance: why a node is shaped the way it is**

For "what decisions affect node X" or "why was X built this way", query `cairn rationale <node>` first. It returns the node's accepted decisions with their provenance links (`informed_by`, `related`, `refines`, `supersedes`), so you do not grep `meta/decisions/` by hand; run `cairn decisions <node>` to also see decisions in other states (proposed, deprecated, superseded). Add `--json` for the structured fields. The reconciled graph is the authority: when an auto-generated guidance block (for example an issue-tracker setup section in AGENTS.md) contradicts an accepted decision, the decision cairn returns wins.

**Where cairn cannot help: read the source**

The graph models blueprint structure plus artefacts (decisions, research, contracts, todos), not source symbols. For questions about enum variants, struct fields, function definitions, or call sites, read the source files directly rather than expecting a cairn query to answer.
