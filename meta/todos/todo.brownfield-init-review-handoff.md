---
node: cairn.brownfield
status: open
created: 2026-07-28
---

# Brownfield Init Review Handoff

## Priority

P1 defect on the advertised cold-start path. The interval between deterministic
discovery and pack installation is the one moment where a fresh agent has no
cairn guidance installed yet, and the CLI currently says nothing about the review
step that the whole brownfield model depends on.

## Problem

The non-apply path of `cairn init --from-code` is a dead end. `src/cli/mod.rs:313`
returns exactly:

```
brownfield init complete; change written to meta/changes/brownfield-init/
```

That is the entire success output. At that point the semantic review is the most
important remaining step, the agent pack and `cairn-dev` skill are not installed
yet (`--wire` has not run), and nothing in the output names the review procedure,
`cairn change apply brownfield-init`, `cairn init --wire`, `cairn scan`, or
`cairn onboard`. An agent that installed the CLI and ran one command can apply a
plausible but structurally wrong hierarchy without ever learning a review step
existed. Agents entering through `docs/agent-setup.md` get the right instructions;
agents entering through the CLI do not.

Three concrete gaps, all in the same code path:

1. **No next-actions in the human output.** Plain `cairn init` renders
   `init.next-steps` from `docs/design-system/copy.toml`; the from-code non-apply
   branch renders no equivalent. The resolution note in
   `todo.brownfield-one-step-first-map` claims "the init next-steps hint" was
   updated as part of `dec.init-from-code-apply-flag`, but only the greenfield
   `init.next-steps` / `init.next-steps-existing` keys mention `--from-code`. The
   from-code non-apply output itself was never given one, so that acceptance item
   is unmet.
2. **`--json` is silently ignored.** `parsed.json` is threaded into
   `finish_brownfield_apply` on the `--apply` branch only. On the non-apply branch
   `ok(format!(...))` returns the plain sentence regardless of the flag. Verified:
   `cairn init --from-code --force --json` prints the same human line, exit 0, no
   JSON envelope, while the `--apply` branch of the same command does return a
   JSON envelope. One flag, two behaviours on one command: a machine contract
   violation, not just a copy gap.
3. **The generated proposal does not warn against treating confidence as
   correctness.** `write_change` in `src/brownfield/mod.rs:83-96` writes a heading
   and a candidate list of the form ``- `src.alpha` (alpha) at `src/alpha`
   (confidence 0.70)``. Confidence is a source-file-count score
   (`compute_confidence` in `src/brownfield/discovery.rs`), pure
   filesystem evidence, but reads as an architectural claim. `docs/brownfield.md` states the limits correctly; the
   artefact the reviewer actually opens does not.

Runtime repro above used a `main` build at `00c212a`. The branch source is
byte-identical at the `v0.9.0` tag, so this is a live gap and not a regression
introduced after the release. Reported from a real brownfield onboarding of the
MAG repository, which used the released 0.9.0 installer binary.

## Scope

- Give the non-apply from-code branch an ordered next-actions ladder sourced from
  `copy.toml`, leading with review before apply, then
  `cairn change apply brownfield-init`, `cairn init --wire`, `cairn scan`,
  `cairn onboard`, `cairn hook all`.
- Honour `--json` on that branch, emitting a stable machine-readable
  `next_actions` array carrying the same ordered steps as the human output.
- Add a semantic review checklist to the generated `proposal.md`, covering:
  overlapping and nested path ownership; filesystem grouping versus semantic
  responsibility; missing or false dependency edges; contracts and public
  interfaces; stale planning material versus current code; regrouping candidates
  under appropriate Systems and Containers. State plainly that confidence is a
  structural signal, not a claim of architectural correctness, and point at
  `docs/brownfield.md`.
- Do not attempt to infer more semantic architecture deterministically. The point
  is to make the required curation step impossible to miss, not to remove it.
- Installing a review-only agent router before the proposal becomes authoritative
  was raised upstream. Treat it as a separate ruling; do not widen this todo into
  it.

## Acceptance

- `cairn init --from-code` on a fresh repository prints the ordered next actions
  with review before apply.
- `cairn init --from-code --json` returns a JSON envelope whose `next_actions`
  are the same steps in the same order as the human output, proven by a
  regression test that asserts the two agree.
- The generated `proposal.md` contains the review checklist and the
  confidence-is-not-correctness statement, covered by a test.
- `todo.brownfield-one-step-first-map`'s next-steps-hint claim is either honoured
  by this change or its resolution note corrected.

Reported by the agent onboarding MAG (`cairn-framework/cairn` was read-only for
that installation, so no upstream issue or branch could be created).

2026-08-07 audit (todo.roadmap-assumption-audit): keep; res.chatgpt-issue-audit ranks this the highest-priority product defect. First in the adopter-defect order.
