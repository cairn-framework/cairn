---
node: cairn.kernel.scanner
status: done
created: 2026-07-16
---

# Decision Accumulation Finding

Accepted decisions accrete on nodes forever and nothing ever prompts use
of the supersession machinery cairn already has: 67 of 68 dogfood
decisions are still accepted (cairn.root carries 19, cairn.kernel.cli 14,
cairn.ui 11), and every default bundle drags them along.

Add a deterministic check in `src/scanner/checks.rs` counting accepted
decisions per node, emitting an Info finding when the count exceeds a
configurable threshold (default 10; it would fire on cairn.root,
cairn.kernel.cli, and cairn.ui today). Allocate a CA-series code in
`docs/registries/error-codes.md` and map it in
`src/query_api/handlers/remediate.rs` alongside the other decision codes.
Remediation text prescribes the consolidating-decision workflow: author
one decision that supersedes the stale set; superseded targets flip
status and default payloads shrink automatically
(`src/query_api/handlers/bundle.rs:34`, `node.rs:111` filter to Accepted).

Compaction itself stays judgment work for humans or agents, never
automatic, consistent with the summariser's propose-never-apply
principle. Provenance survives: superseded decisions still count as
provenance coverage (`src/scanner/tests.rs:362`).

Caveat to fold into guidance: `meta/decisions/revisit-trigger-relevance.md`
uses a `superseded_by` frontmatter field the parser does not read; the
canonical link direction is the superseding decision's `supersedes` list.

Motivation: `res.a2ui-analysis` finding 2. Small; no change proposal
needed for the Info-tier check itself.
