---
node: cairn.root
status: open
created: 2026-07-11
---

# Terminology: plain-language pass over Cairn's vocabulary

Owner feedback (2026-07-11): Cairn's terminology is hard to understand. The project owner admits not knowing what some terms mean, which means a new user has no chance. Revisit the core vocabulary and make every user-facing term understandable at roughly a 5th-grade reading level, without losing technical precision where it matters.

Terms to audit (at least): provenance, artefact, reconciliation, blueprint, contract, drift, frontier, ghost node, orphaned node, hinge, authority, chain balance, brownfield, lineage, decision, research, source.

Approach:

- For each term, decide: keep with a plain one-line definition everywhere it first appears; rename to an everyday word; or demote to internal-only vocabulary (never shown to users).
- Candidate plain framings to evaluate: provenance = "where this came from and why"; reconciliation = "checking the plan against the real code"; drift = "the code no longer matches the plan"; ghost = "planned but not built yet"; orphaned = "built but not in the plan"; frontier = "what you can build next".
- Every user-facing surface must use the chosen wording consistently: CLI output (copy.toml), webui labels (including the chain banner and inspector sections), README, landing page, docs/spec.md glossary, and the ProseNudge explanations.
- Add or update a single glossary (one source of truth) that all surfaces point at; do not scatter definitions.
- Follow the plain-language pass convention (skill: cairn-doc-plain-language-pass) and the em-dash ban; British spelling.
- Renames that touch node/finding identifiers or the wire format need a decision artefact first; wording-only changes do not.

Acceptance: a terminology table (term, verdict, plain wording) ratified by the owner; user-facing surfaces updated consistently; copy.toml remains the single home for CLI strings; wire-format snapshots unchanged unless a decision covers the change; all gates pass.

## Prior art: the 2026-04 infographic (added 2026-07-25)

Owner observation (2026-07-25): the retired root-level infographic explains
cairn better than the current landing page does, despite being months older.
It was moved out of repo root to `studio/reference/infographic.html` (plus
`infographic_v3.png`) rather than deleted, so this pass can mine it.

What it does that current surfaces do not:

- **"Two Chains, One Hinge"** renders the provenance and authority chains as one
  picture with the decision at the hinge. `AGENTS.md` calls this topology
  load-bearing, yet no user-facing surface draws it.
- **"Explore the Map"** steps through the graph one layer at a time
  (prev/next controls, a live detail panel) instead of presenting the whole
  graph at once. Progressive disclosure teaches the vocabulary in the order a
  newcomer can absorb it: each term arrives attached to the thing it names.
- **"Changes Over Time"** reduces the change lifecycle to four named steps
  (Propose, Isolate, Archive, Record), which is plainer than the current prose.

Use these as the shape test for the ratified wording: if a chosen plain term
cannot carry one of these three explanations, it is the wrong term. Treat the
infographic as reference only; it predates the terminology rename and still
says `cairn.dsl`, so do not copy its identifiers.
