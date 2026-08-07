---
node: cairn.brownfield
status: open
created: 2026-07-11
---

# Brownfield stress test: Cairn on the AutoDocs repository (Arm A)

One-sided Arm A experiment, rewritten 2026-07-29 under the accepted
`dec.autodocs-head-to-head-arm-b` (maintainer ratification, PR #528 sheet W6).
The head-to-head as filed on 2026-07-11 also mandated Arm B, AutoDocs run over
its own repository. That arm is dropped: AutoDocs supports neither polyglot nor
nested-layout repositories and its own repo is both, so lifting either limit
alone is not enough, and the comparison was unavailable at any price
(`res.autodocs-head-to-head-feasibility`, source `src.autodocs`).

## Status note

Open. The blocking prerequisite was satisfied 2026-07-29 by acceptance of
`dec.autodocs-head-to-head-arm-b` (maintainer ratification, sheet of record
PR #528, row W6).

## Scope

- Clone TrySita/AutoDocs locally and run Cairn's brownfield onboarding on it:
  `cairn init --from-code`, the delta apply, and a first scan.
- Stress-test Cairn brownfield on a real polyglot repo (Python ingestion
  service plus TypeScript Next.js app, Docker Compose, MCP surface).
- Use Cairn's map as the navigation aid for our own competitor study of their
  codebase, so the study itself validates the tool.
- Record setup friction, runtime, node counts, findings, and false positives.
- This run also unblocks the large-brownfield measurement deferred in
  `res.codeatlas-analysis` item 7.

## Boundaries

- Local and private; publish nothing project-specific without maintainer
  clearance (see business adoption plan in the george repo). No unsolicited
  external contact.
- The webui/dashboard comparison sub-goal stays dropped, per the 2026-07-11
  review disposition: it was already delivered by
  `todo.webui-simplicity-review`.
- No quality-axis substitution: reporting a local model's output as AutoDocs'
  quality would launder a model swap into a product claim, per the decision's
  rationale.

## Acceptance

- The Arm A run is completed and documented as a research artefact
  (meta/research) carrying the recorded measurements and any Cairn defects or
  borrow-candidates it exposes, with follow-up todos filed per defect.
- At least three concrete Cairn improvements or confirmations recorded.
- No unsolicited external contact.

## Revisit

Arm B as originally specified returns when upstream lifts both limits, the
decision's revisit trigger: AutoDocs supports polyglot repositories AND drops
the repository-root layout requirement. At that point running Arm B needs only
a spend ruling.

## History

- 2026-07-11: filed as a two-arm head-to-head, Cairn on AutoDocs versus
  AutoDocs on itself.
- 2026-07-11 review disposition: narrow and defer; the webui/dashboard
  sub-goal dropped as already delivered by `todo.webui-simplicity-review`.
- 2026-07-27: blocked on `todo.autodocs-arm-b-ruling` after
  `res.autodocs-head-to-head-feasibility` showed Arm B doubly blocked, an
  upstream capability gap plus metered spend on the documented
  default-provider configuration. Arm A deliberately not landed alone, because
  half a binding Acceptance would read as progress.
- 2026-07-29: `dec.autodocs-head-to-head-arm-b` accepted; rewritten one-sided
  and reopened.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. Accepted dec.autodocs-head-to-head-arm-b reopened this as the one-sided Arm A stress test that still delivers its highest-value goal and unblocks the deferred large-brownfield measurement.

2026-08-07 audit (todo.roadmap-assumption-audit): keep as written; deferred behind the adopter-defect queue (res.chatgpt-issue-audit keeps #280).
