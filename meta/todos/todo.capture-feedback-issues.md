---
node: cairn.root
status: open
created: 2026-07-10
---

# Capture Feedback Issues

Triage and verify the 16 agent-filed issues #232 to #247, then record a verdict
for each. These were logged on 2026-07-10 while driving cairn 0.1.4 end to end
(propose, parallel implement, verify, accept) against a 16-node bun/TypeScript/
three.js library built from a ghost blueprint. The session recorded 37 structured
observations, filed as 15 self-contained issues (#232 to #246) plus the tracking
umbrella #247 (body quoted in local://open-issues.md).

## Problem
The friction concentrates in four areas: reconciler coverage outside Rust, query
completeness, non-Rust gate support, and documentation drift. As filed they block
confident agent-driven use of cairn on non-Rust projects.

## Evidence
- Tracking issue #247 enumerates every child issue with quoted evidence and a
  suggested sequencing (reconciler trust first, then acceptance, then query gaps,
  then docs).
- All 16 issues were filed against the released cairn 0.1.4.

## Important caveat
These issues target v0.1.4. The simplify-architecture programme (#223 to #231)
landed between that release and current main and rewrote the query spine
(todo.simplify-ui-query-api #229), the CLI registry table (#228), the reconciler
(todo.generic-language-reconciler #227), and the canonical JSON renderer
(todo.simplify-render-canonical-json #230). Several of #232 to #246 are very
likely fixed on main or materially changed. No issue in this set becomes a work
item until re-verified against current main.

## Proposed approach
Run one verification pass over #232 to #247, producing a per-issue verdict:

- `fixed-on-main`: the behaviour is correct on main now. Draft a one-line close
  rationale; close the issue (no native todo).
- `still-valid`: the friction remains. Mint a native cairn todo whose body carries
  a `gh:#NNN` reference line; close the original issue pointing at the new todo.
- `wont-fix`: out of scope or superseded by design. Draft a close rationale citing
  the superseding decision or PR; close.

The `gh:#NNN` reference line on every minted todo lets the future issue-sync
tooling (referenced in todo.github-issues-cleanup) reconcile later.

## Themes and draft verdicts
Verdicts below are proposed stances to confirm during the verification pass,
annotated with the simplify-architecture item most likely to have changed them.
Mark each `VERIFY` before recording the final verdict.

### Language-agnostic gates and reconciler activation
- #232 Reconciler activation and init scaffolding are nondeterministic and
  project-language-blind. Likely addressed by todo.generic-language-reconciler
  (#227) and dec.changes-in-artefact-set. `Proposed: fixed-on-main (VERIFY)`.
  If residual nondeterminism remains, mint todo.generic-language-reconciler-triage
  with `gh:#232`.
- #234 `cairn accept` hardcodes cargo gates, so non-Rust changes never pass.
  Partly addressed by generic-language-reconciler (#227) language inference.
  `Proposed: fixed-on-main or still-valid (VERIFY)`. If the accept gate still
  cargo-pins, mint todo.accept-language-aware-gates with `gh:#234`.
- #245 cairn-apply skill assumes one sequential agent and Rust gates; add parallel
  mode and language-aware gates. Feeds #102. `Proposed: still-valid` (skill change
  independent of core). Mint todo.cairn-apply-parallel-mode with `gh:#245`.

### Findings and exit-code coherence
- #233 CT001 fires for any node with multiple public_api path targets whose exports
  differ. `Proposed: still-valid (VERIFY against generic-language-reconciler CT001
  handling)`. If still firing, mint todo.ct001-multi-target with `gh:#233`.
- #235 `cairn hook` blocking decisions are inconsistent with `cairn scan` Error
  findings. `Proposed: still-valid (VERIFY; dec.loop-resolves-knowable-gaps touches
  hook/scan coherence)`. Mint todo.hook-scan-finding-coherence with `gh:#235`.
- #238 No signal distinguishes implemented from ghost (empty-dir) modules. Partly
  touched by dec.ghost-rule-tracking but no read surface yet. `Proposed: still-valid`.
  Mint todo.ghost-module-signal with `gh:#238`.

### Query and ergonomics
- #236 `cairn neighbourhood` reports no inbound/outbound edges despite blueprint
  edges. Likely addressed by todo.simplify-ui-query-api (#229) edge surface.
  `Proposed: fixed-on-main (VERIFY)`. If edges still missing, mint
  todo.neighbourhood-edges with `gh:#236`.
- #237 `cairn frontier` omits leaf modules and `cairn order` inverts container/child
  ordering. Likely addressed by the frontier-query decision and ui-query-api (#229).
  `Proposed: fixed-on-main (VERIFY)`. Residual: mint todo.frontier-order-fix with
  `gh:#237`.
- #239 Bundle of node-query ergonomics gaps in get/rationale/neighbourhood. Partly
  addressed by ui-query-api (#229). `Proposed: still-valid (partial; VERIFY which
  gaps remain)`. Mint todo.query-ergonomics with `gh:#239`.
- #244 Per-command --help falls back to the global command list; no per-command
  usage docs. `Proposed: still-valid (independent, small)`. Mint
  todo.per-command-help with `gh:#244`.

### JSON envelope
- #240 13 of 14 --json commands violate the documented {command, status, data}
  envelope. Directly targeted by todo.simplify-render-canonical-json (#230, six
  commands stop-ruled with measured numbers), dec.query-json-schema-version, and
  webui-json-schema-version. `Proposed: fixed-on-main (VERIFY remaining 8 commands)`.
  Residual envelope gaps: mint todo.json-envelope-completion with `gh:#240`.

### Change lifecycle read surface
- #241 Change lifecycle has no coherent read surface: active-change definitions
  conflict; progress and gate status are invisible. Partly addressed by
  todo.status-active-changes-bug (fixes active-change definitions) but progress and
  gate preview are still missing. `Proposed: still-valid (partial)`. Mint
  todo.change-read-surface with `gh:#241`.

### Todo UX
- #242 No project-wide todo listing; `cairn todo new` stamps the wrong created date.
  `Proposed: still-valid`. Mint todo.todo-listing-and-date with `gh:#242`.

### Docs drift
- #243 cairn-dev skill and finding-codes reference omit ~17 commands, several emitted
  finding codes, and correct exit-code semantics. The command surface changed during
  simplify-architecture (#223 to #231), so this drift is now worse. `Proposed:
  still-valid (urgent)`. Mint todo.cairn-dev-docs-sync with `gh:#243`.

### Feedback command
- #246 `cairn feedback`: add --area/--severity/--json fields and stop truncating
  generated issue titles. `Proposed: still-valid`. Mint
  todo.feedback-structured-fields with `gh:#246`.

### Tracking umbrella
- #247 Umbrella tracking issue for #232 to #246. Close only after every child issue
  has a recorded verdict and either a linked todo or a close rationale. `gh:#247`.

## Acceptance
Every issue in #232 to #247 has a recorded verdict (fixed-on-main, still-valid, or
wont-fix) and either a linked cairn todo carrying `gh:#NNN` or a drafted close
rationale.

## Dependencies and ordering
Run before todo.github-issues-cleanup, which performs the actual closures in one
sweep once these verdicts exist.
