---
name: cairn-explore
description: Explore the Cairn graph and query project state. Use when the user wants to understand the architecture, find nodes, check status, or inspect findings.
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
- `cairn check [<node>]` - Check findings (inspection mode, always exits 0)
- `cairn ui` - Open the visual graph explorer in a browser

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
