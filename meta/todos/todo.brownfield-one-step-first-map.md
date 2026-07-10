---
node: cairn.brownfield
status: open
created: 2026-07-10
---

# Brownfield One Step First Map

`cairn init --from-code` writes a comment-only `cairn.blueprint` plus a proposal under
`meta/changes/brownfield-init/`; until someone runs `cairn change archive brownfield-init`,
`cairn scan` reconciles an empty plan and `map.md` is empty. A new user following the
quickstart literally hits an empty map on day one. Options: auto-apply the draft when the
user has not customised anything, have scan read the active brownfield delta, or add a
`--apply` flag. Surfaced by an adversarial offer review: the "two commands to your first
map" promise is currently false and copy had to be rewritten around the archive step.
Closing this gap is the highest-leverage effort fix in the first-run experience.

Related terminology finding: `cairn change archive` meaning "apply the proposal" reads as
shelving, not activating, to new users. Consider an `apply`/`accept` alias or rename as
part of this work.
