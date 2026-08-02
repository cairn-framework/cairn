---
node: cairn.root
status: open
created: 2026-07-11
---

# Positioning: outcome-first copy on README and landing page

Competitor study of AutoDocs/Sita (trysita.com, 2026-07-11) showed their copy leads with outcomes ("start shipping faster in 10 minutes", "cut AI spend by 15%") while Cairn's copy leads with mechanism (graph, reconciliation, provenance). Rework the README intro and docs/landing/index.html hero copy so the first screen answers "what do I get" and the mechanism becomes a numbered "how it really works" walkthrough.

Scope:

- Outcome-first headline and subhead: fewer wrong-file edits, agents that know blast radius, onboarding in minutes, zero install.
- An "agent-ready" compatibility line: Claude Code, Codex, Cursor, plus anything that supports MCP.
- "Star us on GitHub" as the primary free call to action.
- Counter-positioning sentence versus hosted/Docker-first tools: Cairn works in the repo as it is; artefacts are authored, versioned intent, not disposable generated docs.
- Note but defer: a commercial waitlist CTA is under consideration and not to be added yet.

Acceptance: README and landing hero copy lead with outcomes; em-dash ban and copy.toml conventions respected; check-design-tokens and a11y gates still pass.

## Sequencing (added 2026-07-11 after backlog review)

Land AFTER `todo.terminology-plain-language`: the outcome-first hero copy should
use the ratified plain wording, not re-introduce jargon this pass is removing.
Both edit README and docs/landing/index.html, so coordinate to avoid edit
collisions; distinct axes (positioning versus vocabulary), do not merge.

## Explainer reference (added 2026-07-25)

Owner observation (2026-07-25): the 2026-04 infographic, now preserved at
`studio/reference/infographic.html`, is a better explainer than the current
landing page. Its advantage is structural, not cosmetic: it walks the reader
through the map one layer at a time with a live detail panel, so the mechanism
lands as a guided tour rather than a wall of claims.

That is directly usable here. The "how it really works" walkthrough this todo
calls for already has a working precedent: reuse its progressive layer-stepping
shape instead of designing a new one. Sequencing is unchanged (land after
`todo.terminology-plain-language`), and the infographic's pre-rename vocabulary
must not be copied forward.

This todo owns the README-overhaul work and the outward README story.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It makes the outward story answer what users get first.
