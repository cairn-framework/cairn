---
id: dec.design-studio-exploration-method
nodes:
  - cairn.ui
status: accepted
date: 2026-07-10
informed_by: [res.design-studio-greenfield]
---

# Webui design direction: decide via two-track design-studio exploration

## Context

`todo.webui-design-quality` bet A (the aesthetic direction for the webui)
was blocked as a maintainer-gated taste call with no evidence to decide
on. `res.design-studio-greenfield` produced a concrete, repeatable recipe
for generating that evidence with the design-studio toolchain.

## Decision

The aesthetic call is not made from taste alone. It is made by running
two parallel design-studio tracks and comparing their outputs:

- Track A, greenfield-simulated: a stripped worktree (no `src/ui_assets`,
  no `docs/design-system`) plus a project brief and frozen real
  `map.json`/API fixtures; design agents run the full create loop with
  context denial so existing CSS cannot anchor the output.
- Track B, iterate-current: the design-studio review lane audits the live
  current webui and proposes a polish direction.

The comparison is ratified as a future `dec.webui-design-direction`,
which supersedes the blocked bet A framing in
`todo.webui-design-quality`. Wiring the winner into `src/ui_assets`
waits for the remaining `todo.simplify-ui-query-api` endpoint flips so
design work does not rebase over wire churn.

## Consequences

- The exploration itself can start any time: it runs on a throwaway fork
  and never touches main.
- Design output enters the repo only through the existing design-system
  token gates.
- If both tracks converge, the direction ships with evidence; if they
  diverge, the maintainer picks between two concrete rendered artefacts
  rather than abstractions.

revisit_triggers:
  - design-studio tooling becomes unavailable or its output quality is
    insufficient to compare tracks
  - the webui is replaced or retired before the exploration runs
