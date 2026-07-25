# Graph navigation

## Reading source through the graph

Every symbol the graph returns carries a span. `cairn get <node> --symbols --json`
and `cairn locate <symbol> --json` both give `file`, `line`, and `end_line`.

Read that window, not the file. A node can own thousands of lines; the span is
usually tens. Whole-file reads are the single largest avoidable cost in a cairn
session, and they bury the relevant lines in noise.

The human renderers print a narrower view than `--json`. When you need the span
fields, ask for `--json`.

## When a node id will not resolve

Node ids are dotted and case-sensitive (`cairn.kernel.scanner`). If a command
reports an unknown node, walk this ladder in order and stop at the first hit:

1. **Exact id.** Confirm you have the whole dotted path, not a fragment. Ids are
   never abbreviated.
2. **The error's own suggestions.** Cairn prints near-miss candidates when it can.
   A suffix alias often resolves: if `scanner` is unique across the graph,
   `cairn get cairn.kernel.scanner` is the id it meant. Suffix matching is an
   interactive convenience for forming a corrected command; it is never an
   accepted resolution form inside loop mode, which fails closed instead.
3. **Path lookup.** If you know a file the node owns, the owning node is
   authoritative: `cairn context --json` lists every node with its `paths` and
   `files`. The node whose `path` prefixes your file is the one you want.
4. **Filesystem search.** Only when the three above fail. If a file has no owning
   node, that is not a lookup failure, it is a
   `CAIRN_RECONCILE_ORPHANED_FILE` finding: the blueprint does not claim the file.
   Fix the blueprint rather than working around it.

`cairn onboard` groups orphaned files by directory and suggests an owning node or
an ignore entry, which is faster than reasoning about them one at a time.

## Generated snapshots are not context

`map.json` and `map.md` are generated review artefacts, refreshed so a human can
read structural change in a diff. They are large, they duplicate what the queries
return, and they go stale between refreshes.

Never read them to orient, and never cite them as the state of the graph. Use
`cairn context`, `cairn get`, `cairn neighbourhood`, and `cairn deps`. The only
legitimate reason to open `map.json` is to review a change to `map.json` itself.

## MCP

If the runtime exposes `cairn-mcp`, prefer its tools over shell invocations for
`get`, `neighbourhood`, `lint`, and `rationale`. The handlers are the same; the
transport avoids a subprocess.

Two differences matter. MCP returns an outer envelope with `project_context`,
`rules`, and `findings` around the payload, so its bytes are not interchangeable
with CLI `--json`. And MCP `get` has no `--symbols` flag, so root symbols are
CLI-only; when you need spans, use the CLI.
