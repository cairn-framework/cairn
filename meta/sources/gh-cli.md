---
id: src.gh-cli
file: https://cli.github.com/manual/gh_issue
verification: external
type: tool
date: 2026-07-10
---

# GitHub CLI (`gh`) issue commands

Official `gh` manual for issue management: `gh issue create`, `gh issue edit`,
`gh issue close`, and `gh issue list` (with `--label` / `--search`). The
proposed mirror script drives GitHub entirely through these commands plus a
`GH_TOKEN`, so cairn's own code stays out of the GitHub write path (per
`dec.bead-github-sync`). The stable per-todo identity marker
(`cairn-todo: todo.<slug>` in the issue body) is queried via `gh issue list
--label cairn-todo` to build the slug-to-number map for idempotent upsert.
