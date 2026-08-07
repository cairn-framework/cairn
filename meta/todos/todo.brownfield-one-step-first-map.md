---
node: cairn.brownfield
status: done
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

Terminology rulings (owner sign-off, 2026-07-10): `cairn change archive` meaning "apply
the proposal" reads as shelving, not activating; an `apply` alias is ratified (tracked in
todo.change-apply-alias). "Brownfield" stays in agent-facing guides, technical reference
docs (docs/brownfield.md), change ids, and code, where it is immediately recognisable.
First-touch marketing surfaces (README, landing, quickstart) say "existing project" /
"existing codebase" instead: the word fails a plain-language pass for AI-first devs who
never did long-cycle software work.

Resolved 2026-07-12: `cairn init --from-code --apply` applies the discovered
proposal in one command via the shared archive path (dec.init-from-code-apply-flag).
Default remains a reviewable proposal. First-run copy updated in README,
quickstart, brownfield doc, agent-setup, and the init next-steps hint.

2026-08-07 (todo.brownfield-init-review-handoff): the 2026-07-12 note above
covered the greenfield `init.next-steps` keys only; the `--from-code` non-apply
branch printed one sentence and no ladder. That branch now renders its own
ordered `init.from-code.next-actions` ladder, review before apply, so the claim
holds for both entry points.
