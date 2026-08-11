# Cairn Artefacts

Cairn loads six v1 artefact types from Markdown frontmatter: contracts, todos, decisions, reviews, research, and sources. A blueprint node attaches directories or files with pointer fields such as `contract`, `todos`, `decisions`, `reviews`, `research`, and `sources`.

## Contract

```markdown
---
node: app.auth
---

# Auth Contract
```

`node` must match the node that declared the `contract` pointer. A contract may
bind a numeral to code with an inline code span of the form `NAME = N`
(a `SCREAMING_SNAKE` constant name and an unsigned integer); the scanner
verifies it against the `const NAME` integer literal in the node's claimed
Rust files and reports `CAIRN_CONTRACT_NUMERAL_DRIFT` when they disagree.

## Todo

```markdown
---
node: app.auth
status: open
created: 2026-04-17
satisfies: login
---

# Add login error handling
```

`status` is one of `open`, `in_progress`, `done`, or `blocked`. Unknown nodes are reported as orphan warnings.

## Decision

```markdown
---
id: dec.auth-storage
nodes: [app.auth]
status: accepted
date: 2026-04-17
revisited: 2026-04-17
revisit_triggers:
  - "Auth persistence changes"
informed_by:
  - type: research
    id: res.auth-storage
  - type: source
    id: src.auth-notes
supersedes: []
refines: []
ratification: local
affects:
  - meta/decisions/auth-storage.md
  - meta/reviews/rev.auth-storage-correctness.md
  - meta/reviews/rev.auth-storage-simplicity.md
ratified_by: machine
receipts: [rev.auth-storage-correctness, rev.auth-storage-simplicity]
related: []
---

# Store auth sessions centrally
```

`status` is one of `proposed`, `accepted`, `deprecated`, or `superseded`. A decision must reference at least one node unless it is explicitly marked `orphaned: true` with a non-empty `orphan_reason`.

`ratification` is `local` or `binding`; when absent, it means `binding`. `affects` lists normalised repository-relative paths (exact files, or directory rules ending in `/`). `ratified_by: machine` marks a loop signature at any status; an absent marker means maintainer-signed only once the decision is `accepted`, and a proposed decision has no signer yet. `receipts` lists review file stems. A machine-signed accepted local decision's body must additionally carry `## For`, `## Against`, and `## Verdict` headings in that order, each section non-empty: the debate record is a condition of loop acceptance, not optional prose.

## Review

```markdown
---
node: app.auth
review_type: agent_cross_model
date: 2026-04-17
reviewer: <model-id>/<lens-id>
related_change: commit:abc123
subject_hash: sha256:<64 lowercase hex characters>
lens_prompt_hash: sha256:<64 lowercase hex characters>
---

# Review notes

## Verdict

PASS
```


`review_type` defaults to `human` and may also be `agent_introspective` or `agent_cross_model`.
A receipt-grade review has `subject_hash`, `review_type: agent_cross_model`, `reviewer: <model-id>/<lens-id>`, and `lens_prompt_hash`, the sha256 of the committed prompt file at `docs/agent/lenses/<lens-id>.md`. The model id is the provider's exact string and may itself contain `/`; the lens id is everything after the final slash. The first body line matching exactly `## Verdict` opens the verdict section. Its first non-blank line must start at column zero with `PASS` or `BLOCKING`; `PASS` is clean. A missing heading, empty section, or any other first token is not clean.

## Research

```markdown
---
id: res.auth-storage
nodes: [app.auth]
date: 2026-04-17
sources:
  - src.auth-notes
tags: [auth]
---

# Auth storage options
```

Research must reference at least one node and at least one source.

## Source

```markdown
---
id: src.auth-notes
file: ./meta/sources/auth-notes.txt
sha256: bdcf4c994585af6dd6cb1cfbff78bcc73ab27dc30a299db5bb83766ca05b5de4
verification: verified
type: document
date: 2026-04-17
tags: [auth]
description: Notes used for auth design.
---

# Source notes
```

`verification` is one of `verified`, `external`, `unverified`, or `tracked`. Verified sources require a local file and matching SHA-256. External sources require an HTTP or HTTPS URL. Unverified sources are allowed but reported as rationale tensions. Tracked sources cite a live in-repo file or directory that must resolve inside the repository root, with no SHA-256.

## Finding Classes

Structural errors block map queries. Examples include missing required frontmatter, invalid review subtypes, unknown review nodes, missing research sources, invalid external URLs, and verified source checksum mismatches.

Rationale tensions are warnings. Examples include orphan todos, source records that are not cited, unverified sources, unknown decision provenance, and decision cross-reference status mismatches.

Edge divergence and docstring drift are also rationale tensions. A declared blueprint edge can drift from observed Rust imports or module declarations, and authored docstring facts can drift from node names, dependencies, tags, or contract pointers in the map. These findings are advisory by default and are rendered anywhere findings are rendered, including `lint`, `scan`, JSON responses, generated `map.md`, and hook reports that consume map findings.

## Neighbourhood Defaults

`cairn neighbourhood <node>` includes contracts and accepted decisions by default. Todos, research, reviews, deprecated decisions, and active changes are opt-in with `--include-todos`, `--include-research`, `--include-reviews`, `--include-deprecated-decisions`, and `--include-changes`. With `--include-changes`, active changes are scoped to the node and its direct neighbours: only operations from `meta/changes/` proposals that touch a node in the neighbourhood are listed.
