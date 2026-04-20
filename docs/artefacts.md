# Cairn Artefacts

Cairn loads six v1 artefact types from Markdown frontmatter: contracts, todos, decisions, reviews, research, and sources. A blueprint node attaches directories or files with pointer fields such as `contract`, `todos`, `decisions`, `reviews`, `research`, and `sources`.

## Contract

```markdown
---
node: app.auth
---

# Auth Contract
```

`node` must match the node that declared the `contract` pointer.

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
nodes: [app.auth, app.store]
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
supersedes: [dec.old-auth-storage]
refines: []
related: []
---

# Store auth sessions centrally
```

`status` is one of `proposed`, `accepted`, `deprecated`, or `superseded`. A decision must reference at least one node unless it is explicitly marked `orphaned: true` with a non-empty `orphan_reason`.

## Review

```markdown
---
node: app.auth
review_type: human
date: 2026-04-17
reviewer: george
related_change: commit:abc123
---

# Review notes
```

`review_type` defaults to `human` and may also be `agent_introspective` or `agent_cross_model`.

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

`verification` is one of `verified`, `external`, or `unverified`. Verified sources require a local file and matching SHA-256. External sources require an HTTP or HTTPS URL. Unverified sources are allowed but reported as rationale tensions.

## Finding Classes

Structural errors block map queries. Examples include missing required frontmatter, invalid review subtypes, unknown review nodes, missing research sources, invalid external URLs, and verified source checksum mismatches.

Rationale tensions are warnings. Examples include orphan todos, source records that are not cited, unverified sources, unknown decision provenance, and decision cross-reference status mismatches.

Edge divergence and docstring drift are also rationale tensions. A declared blueprint edge can drift from observed Rust imports or module declarations, and authored docstring facts can drift from node names, dependencies, tags, or contract pointers in the map. These findings are advisory by default and are rendered anywhere findings are rendered, including `lint`, `scan`, JSON responses, generated `map.md`, and hook reports that consume map findings.

## Neighbourhood Defaults

`cairn neighbourhood <node>` includes contracts and accepted decisions by default. Todos, research, reviews, deprecated decisions, and active changes are opt-in with `--include-todos`, `--include-research`, `--include-reviews`, `--include-deprecated-decisions`, and `--include-changes`. Active changes remain empty until the change directory phase lands.
