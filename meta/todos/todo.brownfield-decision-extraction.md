---
node: cairn.brownfield
status: open
created: 2026-07-31
related: [src.reddit-gregerw-first-user-test]
---

# Brownfield decision extraction: mine existing invariants into the graph

First external user test (src.reddit-gregerw-first-user-test): after
installing on a real codebase, the user looked for, and could not find, a
way to extract the decisions already embedded in the code into the graph
as invariants that would be gated later. They expected the install prompt
to point at this as the next step. It does not exist, and the gap reads
as "a lot of machinery, no value".

## Task

Design and land the extraction path for an existing codebase:

1. A guided flow (prompt, skill, or `cairn onboard` extension) that walks
   the codebase and existing ADR-like material (docs/adr, comments,
   README sections) and drafts decision artefacts with `nodes:` bindings
   for the maintainer to accept, reject, or edit. Drafts are proposals;
   nothing self-ratifies (`dec.decision-ratification-tiers` binds).
2. Post-install pointers: init/onboard output names this as the next
   step, alongside the existing scaffolding, so the post-install dead
   end the user hit ("What now?") has an answer that produces visible
   value on their own code.
3. Scope the flow to brownfield repos; the dogfood repo is not the test
   case. Validate on at least one external repository.

## Acceptance

- On a brownfield fixture (or a real external repo), the flow produces
  at least one accepted decision bound to real nodes, starting from code
  the user did not annotate for cairn.
- `cairn init`/`onboard` output points at the flow.
- The value story is demonstrable in the first session after install.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It gives brownfield users a concrete path from code to accepted decisions.

2026-08-07 audit (todo.roadmap-assumption-audit): keep as written; adopter queue after todo.init-ignore-scaffolding.
