---
node: cairn.brownfield
status: blocked
created: 2026-08-19
---

# Ratify dec.brownfield-package-root-discovery on convergent receipts

Blocked: needs a session with a second distinct reviewer model.

`dec.brownfield-package-root-discovery` is a `local`-tier decision whose governed
behaviour already shipped in #669 with tests. It was parked `proposed` at the
v0.10.0 cut (2026-08-19) because a `local` ruling is machine-acceptable only on
two convergent agent-cross-model receipts (`dec.reviewer-panel-ratification`,
`dec.decision-ratification-tiers`), and the cutting session had a single distinct
reviewer model (Gemini) available. Fabricating the second receipt is forbidden
(CA054).

## Unblock when

A second genuinely distinct reviewer model is available (for example
`openai-codex/*` and `xai-grok-build/*`, as used for
`dec.conventions-error-types`), so two independent agent-cross-model reviews can
be run.

## Acceptance

- Run the convergence panel: two independent agent-cross-model lens reviews of
  the decision (read-only), each ending in a clean `## Verdict` / `PASS`.
- Author each receipt under `meta/reviews/` with a `subject_hash` equal to the
  decision's recomputed subject manifest and a `lens_prompt_hash` matching its
  `docs/agent/lenses/<lens>.md` file.
- Extend the decision's `affects:` to cover the two receipt paths (review paths
  are excluded from the subject manifest, so this does not move the hash), and
  add a `receipts:` list naming the two review stems.
- Set `dec.brownfield-package-root-discovery` `status: accepted`.
- `cairn scan --strict` exits 0 with no `CAIRN_DECISION_CONVERGENCE_UNMET`.
- Set this todo `done`.
