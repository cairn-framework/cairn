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
ones. The gate is currently all-or-nothing, and the all end is what stalls
automation.

Opening the gate globally is not the answer: `dec.source-tracked-verification`
changes the source verification enum every repository shares, and
`dec.contract-node-shape-drift-deferred` parks a spec rule. Both must stay
maintainer-only forever.

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

A round is defined so it can be audited rather than asserted: two lenses run
independently, neither seeing the other's output, over one commit SHA, each
producing a report stored immutably and referenced by id in the decision alongside
the SHA it read.

Self acceptance requires two independent clean lens reports on the FINAL SHA, the one
being accepted. Earlier rounds are recorded for audit, not counted toward
convergence, since fixing a blocker changes the tree and any clean report on a parent
SHA says nothing about what replaced it. Rerunning the same lens twice over one SHA is
one report, not two. This is deliberately the stricter reading: the whole point is
that PR #520 needed three rounds, so the last word must be about the artefact that
actually lands.

The tier must be validated, not self-asserted, and the two halves of it validate
differently.

What the graph proves on its own: container span, from the decision's `nodes:`
(`cairn neighbourhood`, `cairn deps --transitive`), and supersession, from its
`supersedes` field. A decision claiming `local` while its nodes span containers or
while it supersedes anything is an Error with no further evidence needed.

What the graph cannot prove: `nodes:` says nothing about which files a decision
changes, so no lint rule can infer schema, spec, registry, or pack impact from it.
That half needs a declared `affects:` list of repository paths on the decision,
checked two ways: every declared path must sit outside a committed
binding-surface allowlist (`docs/spec.md`, `docs/registries/`,
`tools/agent-pack/content/`, the artefact schema modules), and the hook that runs
at commit time must confirm the commit's changed paths are a subset of `affects:`.
An undeclared path in the diff is an Error, so understating `affects:` fails
closed at the gate rather than passing on the agent's word. An agent that wants a
cheaper tier must therefore make the change smaller, not the claim bolder.

Surfaces to touch:

- `src/artefacts/registry/types.rs` and `parse.rs`: the `ratification:` and
  `affects:` fields, their parse, and an invalid-value finding.
- `src/scanner/checks.rs`: the graph half of the tier check (container span,
  supersession) plus the `affects:` allowlist check, with codes registered in
  `docs/registries/error-codes.md` and `[findings.codes]` entries in
  `docs/design-system/copy.toml`.
- `src/hooks/`: the commit-time check that the diff's changed paths are a subset
  of `affects:` for any decision the commit accepts at tier `local`.
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
- A `local` decision is refused unless it references two independent clean lens
  reports whose recorded SHA equals the SHA being accepted. One report, two reports
  from the same lens, or a clean report on a parent SHA all fail, each with a test.
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
sources, and limits, is recorded as a comment on PR #523. The transcripts
themselves are machine-local and were not committable, so every number those todos
rely on is restated here as a PR number or a measured count.

## Origin

Maintainer conversation, 2026-07-28, on why three consecutive iterations (#518,
#519, #520) spent themselves on provenance bookkeeping instead of the backlog, and
on starting the blast radius small and widening it safely over time. The tiering is
the maintainer's ruling, recorded here in substance: the two currently parked
decisions are both `binding`, and this tiering deliberately does not unblock them.
