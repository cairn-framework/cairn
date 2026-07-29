---
node: cairn.kernel.artefacts
status: open
created: 2026-07-28
---

# Decision Ratification Tiers

## Problem

The loop may never self-ratify a decision, so every ruling costs a maintainer
round trip regardless of how small its blast radius is. That is correct for
rulings every adopting repository inherits and wrong for the long tail of local
ones. The gate is all-or-nothing, and it is the maintainer-only end of it that
stalls automation.

Opening the gate globally is not the answer: `dec.source-tracked-verification`
changes the source verification enum every repository shares, and
`dec.contract-node-shape-drift-deferred` parked a spec rule (since fulfilled
and deprecated). Both must stay maintainer-only forever.

## Scope

Add a `ratification:` field to the decision schema with two values, and let the
loop self-accept exactly one of them.

- `local`: every id in the decision's `nodes:` lives inside one container, it
  supersedes nothing, and it changes no artefact schema, no spec invariant, no
  registry rule (`docs/registries/`), and no shipped pack content
  (`tools/agent-pack/content/`). The loop may set `status: accepted` on this tier on
  its own, under the review-convergence condition below, and only with the
  For/Against/Verdict record and a `ratified_by: machine` marker in the artefact, so
  every machine acceptance is queryable rather than indistinguishable from a human
  one.
- `binding`: anything else, including any decision that supersedes an accepted one.
  Maintainer-only, permanently.

**Review convergence, not a single clean pass.** Today's evidence is that a first
paired round rarely comes back clean: across 26 usable paired rounds on 2026-07-28
and the day before, 19 surfaced an explicit blocker and 0 surfaced only nits, and
PR #520 needed three rounds to reach a clean pair. A single clean round is therefore
not evidence of correctness; it is evidence that this round found nothing.

A round is defined so it can be audited rather than asserted, and the receipt is a
typed artefact rather than an id in prose. Cairn already has the shape: the Review
artefact (`docs/artefacts.md:58-72`) carries `node`, `review_type`, `date`,
`reviewer`, and `related_change`. Each lens produces
`meta/reviews/rev.<slug>-<lens>.md` with `review_type: agent_cross_model`, a Verdict
section, and a `subject_hash`. `agent://` handles and harness transcripts are
ephemeral and must never be the evidence of record.

**Lens identity, so independence is checked rather than claimed.** `reviewer` is not
free text. It is `<model-id>/<lens-id>`, where the model id is the provider's exact
model string and the lens id names the committed prompt that produced the review. The
receipt also carries `lens_prompt_hash`, the hash of that prompt file. Two receipts
are independent when their `reviewer` values differ in the model id, the lens id, or
both, and when neither review saw the other's output, which the harness asserts by
running them concurrently from one dispatch. Two receipts with identical `reviewer`
and `lens_prompt_hash` are one round however many times they were run. The prompts
live in the repository, so a lens can be audited later rather than being an
unreproducible name in a file. Today's evidence says the pair is asymmetric, with the
correctness lens carrying most semantic findings, so a same-model rerun of one prompt
must never count as two votes.

**The binding identity is a content hash over the whole reviewed change, not a commit
SHA.** Two constraints rule out commits. A receipt cannot cite the commit that
contains it, and the loop lands one squash commit per iteration
(`.claude/skills/cairn-dev/references/loop-mode.md`), which rewrites any SHA a
receipt might have named.

So `subject_hash` is a canonical hash of a manifest covering everything the review
judged. The manifest is a sorted list of `path` plus `hash` pairs, built by one rule
per path so nothing is hashed twice or cycles:

- The decision's own path, which `affects:` must list because the hook checks the
  diff against it, is hashed from its **governed content**: body plus frontmatter with
  the ratification fields excluded (`status`, `ratification`, `ratified_by`, and the
  receipt references). Never from its raw bytes, which change at the status flip and
  would make the subject depend on its own acceptance.
- The receipt artefacts are excluded entirely. They are the evidence, not the subject.
- Every other path in `affects:` is hashed from its raw bytes.

Hashing the decision alone would let the implementation drift after review: the ruling
stays byte-identical while a file it governs changes, and a receipt bound only to the
decision text would still verify. Binding the manifest closes that, and it is also why
`affects:` must be complete, which the hook enforces against the diff.

That makes the protocol squash-safe and cycle-free:

1. **Candidate.** The decision and the change it rules on reach their final state at
   `status: proposed`. The manifest hashes to H.
2. **Review.** Each lens reads that state and writes a receipt carrying
   `subject_hash: H`. `related_change` may record the working commit for audit, but
   nothing depends on that SHA surviving.
3. **Acceptance.** The status flip plus the receipts may land in the same squash commit
   as the candidate, because identity is content. The hook recomputes the manifest from
   the tree and refuses acceptance unless it equals the `subject_hash` both receipts
   carry. Editing one word of the ruling, or one line of any governed file, changes H
   and invalidates both receipts.

Two receipts naming the same reviewer are one round. Receipts carrying a stale
`subject_hash` are kept for audit and never counted: re-wording after a blocker
produces H', which needs two fresh receipts. The `affects:` list must cover the
receipt paths, or the tier hook would reject the evidence it requires. This is
deliberately the stricter reading: PR #520 needed three rounds, so the last word has
to be about the wording that actually lands.

The tier must be validated, not self-asserted, and the two halves of it validate
differently.

What the graph proves on its own: container span, from the decision's `nodes:`
(`cairn neighbourhood`, `cairn deps --transitive`), and supersession, from its
`supersedes` field. A decision claiming `local` while its nodes span containers or
while it supersedes anything is an Error with no further evidence needed.

What the graph cannot prove: `nodes:` says nothing about which files a decision
changes, so no lint rule can infer schema, spec, registry, or pack impact from it.
That half needs a declared `affects:` list of repository paths on the decision,
checked two ways. The hook confirms the commit's changed paths are a subset of
`affects:`, so understating it fails closed rather than passing on the agent's word.
And every declared path must sit outside the binding-surface allowlist.

**The allowlist is data, matched deterministically.** It lives in one committed file
as an ordered list of rules, each an exact repository-relative path or a path ending
in `/` meaning that directory and everything under it. No globs, no regex, no
categories: "the artefact schema modules" is not a rule, `src/artefacts/registry/` is.
Matching is: normalise both sides to repository-relative form with no `.` or `..`
segment, then a path is inside the allowlist when it equals an exact rule or is
prefixed by a directory rule. Symlinks resolve before matching, and a path escaping
the repository is an Error rather than a miss. The starting list is `docs/spec.md`,
`docs/registries/`, `tools/agent-pack/content/`, `src/artefacts/registry/`, and the
blueprint. Extending it is a binding decision by construction, since the file is
itself inside `docs/registries/`.

An agent that wants a cheaper tier must therefore make the change smaller, not the
claim bolder.

Surfaces to touch:

- `src/artefacts/registry/types.rs` and `parse.rs`: the `ratification:`, `affects:`,
  and `ratified_by:` decision fields, the `subject_hash`, `reviewer`, and
  `lens_prompt_hash` review fields, their parse, and an invalid-value finding per
  field.
- `src/artefacts/registry/validate/mod.rs`: the receipt-to-decision link check, so a
  decision naming a receipt that does not exist, or a receipt whose `subject_hash`
  matches nothing, is a finding rather than silence.
- The canonical manifest hasher, one function with its own unit tests, since both the
  scanner and the hook must compute byte-identical results from it.
- `src/scanner/checks.rs`: the graph half of the tier check (container span,
  supersession), the `affects:` allowlist check, and the convergence check (two
  receipts, distinct reviewers, matching `subject_hash`), with codes registered in
  `docs/registries/error-codes.md` and `[findings.codes]` entries in
  `docs/design-system/copy.toml`.
- `src/hooks/`: the commit-time checks, that the diff's changed paths are a subset of
  `affects:`, and that the recomputed manifest equals the `subject_hash` the receipts
  carry, for any decision the commit accepts at tier `local`.
- `src/query_api/serialise.rs`: the new fields are wire values, so the exhaustive
  matches there fail to build without them, and the wire snapshots rebase.
- The lens prompt files the `reviewer` ids name, committed so a review is
  reproducible.
- Tests: the acceptance status flip alone must not change `subject_hash`, which is the
  regression that would break every receipt at once.
- `docs/registries/spec-rules.md`: a row per rule this makes enforceable.
- The loop assets that state the never-self-ratify rule
  (`.claude/skills/cairn-loop-reconcile/SKILL.md` clause 4,
  `.claude/skills/cairn-loop-scope/SKILL.md` section 2) and their canonical copies
  under `tools/agent-pack/content/`, which ship to adopting repositories.
- `docs/conventions.md` section 10 and `docs/artefacts.md`: the decision schema
  is taught in both.

## Depends on

Maintainer acceptance of the tiering rule itself. It changes the artefact schema
every adopting repository shares, so by its own definition it is `binding` and the
loop may not ratify it. The rule is promoted and its codes allocated by the
implementing commit, per `docs/conventions.md` rule 2.

Tier design ratified 2026-07-29 (PR #528 sheet W8); implementation may proceed.
The implementation is substantial, so it goes through a change proposal
(`cairn-propose`) rather than a bare loop unit.

## Acceptance

- A `local` decision authored by the loop, carrying a For/Against/Verdict body, a
  converged review record, `ratified_by: machine`, and an `affects:` list covering
  its diff, passes scan and the hook with `status: accepted` and no finding.
- A decision claiming `local` raises an Error when its `nodes:` span containers,
  when it supersedes anything, or when `affects:` names a path inside the
  binding-surface allowlist.
- A commit that accepts a `local` decision while touching a path absent from its
  `affects:` list is blocked by the hook. This is the case that makes the tier fail
  closed, so it gets a test of its own.
- A `local` decision is refused unless it references two committed Review artefacts
  with clean verdicts, `reviewer` values differing in model id or lens id, and the
  same `subject_hash`. One receipt, two receipts from the same `reviewer` and
  `lens_prompt_hash`, and a receipt path missing from `affects:` each fail, with a
  test apiece.
- Acceptance is refused when the manifest recomputed from the tree differs from the
  `subject_hash` the receipts carry, even by a byte, so slipping a rule or an
  implementation change in alongside the status flip is impossible. This is the
  keystone test of the tier.
- Flipping `status` to `accepted` and adding the receipt references does not itself
  change `subject_hash`, or every receipt would invalidate at the moment of use.
- A decision with no `ratification:` field is treated as `binding`, so existing
  artefacts keep their current protection without a migration.
- `ratified_by: machine` is queryable, so `cairn decisions <node>` can list every
  machine acceptance for audit in one command.
- The shipped pack references and the loop assets agree with the code, verified by
  the pack-conformance tests.

## Widening the blast radius, with evidence

The tier starts deliberately narrow and widens only on measured audit results. The
`ratified_by: machine` marker is what makes that measurable: after every batch of
ten machine-accepted `local` decisions, the maintainer audits them and records how
many were wrong in a durable way (a rule that had to be superseded, not a wording
nit). Widening is proposed only against a clean or near-clean audit, and each
widening is its own binding decision naming the audited batch. A batch with a
durable error narrows the tier instead.

The one candidate widening step, not pre-approved: decisions whose `nodes:` span two
containers with no shared dependant. Nothing else. Superseding an accepted decision
stays maintainer-only even when the target was itself machine-accepted, because an
accepted decision is accepted whoever signed it, and nothing touching spec
invariants, artefact schema, `docs/registries/`, or `tools/agent-pack/content/` is
ever a candidate however small the edit looks. Those are what every adopting
repository inherits, and the maintainer set them as permanently maintainer-only.

## Evidence

Paired-review reliability was audited over 26 usable rounds spanning 17 changes on
2026-07-27 and 2026-07-28. It supports the narrow tier and contradicts a wider one:

- Review catches durable rule errors, not only code defects. PR #506's reviewer
  drove a materially different SCC reporting rule, and the author recorded that the
  review improved the ruling. Similar wrong-rule catches on #494, #496, #503, #519,
  and #520.
- Two clean lenses are not equivalent to a maintainer signature. After two passes on
  #506 the author found a larger masking hole neither pass caught; on #511 CodeRabbit
  later required atomic baseline semantics the pair missed; on #521 CodeRabbit caught
  that the unit's todo had been marked done against its own unmet criterion, which
  the pair had not.
- Reviewers can be confidently wrong. On #506 a correctness lens blocked on an
  acceptance sentence that belonged to a different todo file, an affirmatively
  disproven finding.
- Agreement is asymmetric: the correctness and adversarial lenses carry most semantic
  findings while the simplicity lens often reports only duplication or nothing, so
  two approvals are not two independent votes on the same question.

The audit trail behind the review-reliability numbers, including its window,
sources, and limits, is recorded at https://github.com/cairn-framework/cairn/pull/523#issuecomment-5105144953, with the per-iteration session ids and pinned counts at https://github.com/cairn-framework/cairn/pull/523#issuecomment-5105158502. The transcripts
themselves are machine-local and were not committable, so every number those todos
rely on is restated here as a PR number or a measured count.

## Origin

Maintainer conversation, 2026-07-28, on why three consecutive iterations (#518,
#519, #520) spent themselves on provenance bookkeeping instead of the backlog, and
on starting the blast radius small and widening it safely over time. The tiering is
the maintainer's ruling, recorded here in substance: the two currently parked
decisions are both `binding`, and this tiering deliberately does not unblock them.
