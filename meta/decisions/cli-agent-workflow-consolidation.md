---
id: dec.cli-agent-workflow-consolidation
nodes:
  - cairn.kernel.cli
  - cairn.kernel.query
  - cairn.brownfield
  - cairn.mcp
  - cairn.root
status: accepted
date: 2026-07-28
informed_by:
  - res.agent-pack-packaging-survey
  - res.agent-experiment-linklint
  - res.harness-engineering
supersedes:
  - dec.locate-result-semantics
  - dec.todo-write-surface
  - dec.explore-teaches-provenance
  - dec.retire-karpathy-guidelines-skill
  - dec.cairn-brief-orientation
  - dec.context-edges
  - dec.feedback-loop
  - dec.hook-git-install
  - dec.init-from-code-apply-flag
  - dec.simplify-cli-subset-folds
  - dec.pack-publication-on-activation-evidence
related:
  - dec.unified-cairn-dev-entry
  - dec.loop-reconcile-step
  - dec.agent-pack-packaging
  - dec.pack-adapter-roots
  - dec.loop-resolves-knowable-gaps
  - dec.init-emits-agent-skills
---
# Consolidate CLI workflow and point-contract authority

## Context

The CLI node carried twenty accepted decisions. Eleven described successive
stages of one agent-facing workflow or recorded narrow command implementations
that have already shipped. Reading them independently obscured the current
contract, especially where the original ten-phase self-loop became the
one-unit, harness-invoked loop and where direct skill emission became the
manifest-owned agent pack.

This decision consolidates those obligations without changing behaviour. The
more detailed authorities for the original loop contract, unified loop entry,
reconciliation step, agent-pack packaging, and adapter roots remain accepted.

## Decision

### Agent workflow

`cairn-dev` is the single logical entry for agent guidance. Its compact default
mode routes ordinary work. Its loop mode activates only when explicitly
selected and remains the sole normative one-iteration procedure together with
exactly its declared asset closure.

One loop invocation selects and lands exactly one unit, then ends. It preserves
the accepted fail-closed preflight, dedicated worktree and `loop/*` namespace,
exact node resolution, explicit-path staging, one-PR squash landing, and final
token contract in `dec.unified-cairn-dev-entry`. Reconciliation remains a
required step under `dec.loop-reconcile-step`. Cairn may compute eligibility
inside an invocation, but the user or harness owns invocation, repetition,
runtime, and concurrency.

Every implemented unit must satisfy its written acceptance criterion, run the
repository's language gates when relevant, reach zero blocking `cairn scan`
findings, pass `cairn hook all`, and land only after review and CI. A knowable
decision gap becomes a researched, adversarially tested recommendation plus a
blocked tracker item. A true external blocker is reported precisely. Neither
path permits self-ratification or selection of a second unit.

### Agent guidance and pack

The canonical, harness-neutral agent pack and deterministic adapters remain
governed by `dec.agent-pack-packaging` and `dec.pack-adapter-roots`. Init wiring
delegates to that manifest-owned installer instead of maintaining a second
direct-emission authority. Adapter publication requires successful validation
in the live harness and may claim activation only, not improved answer quality.
Any restored six-arm trial is a fresh unit, and any stronger quality claim
requires new evidence.

The shipped `cairn-explore` guidance includes the provenance query path through
`rationale`, `decisions`, `research`, and `sources`, while making clear that the
graph is not a source-symbol index. The retired `karpathy-guidelines` skill is
not restored. Its durable discipline stays absorbed in `cairn-dev` and the loop
implementation procedure: state material assumptions, write a checkable
criterion, prefer the smallest sufficient change, and ask only when readings
have materially different consequences.

### CLI point contracts

The following shipped contracts remain binding:

1. `cairn brief` fuses the selected unit, its accepted decisions, node contract,
   task body, and gates. Proposed or superseded decisions are not presented as
   binding. A unit without a node still receives universal gates and a visible
   node-binding hint.
2. `cairn context --json` exposes full dependency edges. Human output presents
   one structure tree with labelled outbound edges and retains anomalous state.
3. `cairn feedback` records versioned friction locally and produces a prefilled
   upstream issue URL without network access. Init guidance teaches the command
   and prints actionable next steps.
4. `cairn hook install`, `status`, and `uninstall` own only marker-managed hooks,
   resolve the repository root through Git, treat dangling symlinks as occupied,
   preserve platform-correct permissions, and remain explicit opt-in.
5. `cairn init --from-code --apply` remains explicit opt-in and delegates to the
   normal change-apply path, preserving conflict checks, validation, rollback,
   and archive logging.
6. Strict-subset commands remain folded without aliases: `get --symbols`
   conditionally includes symbols, `lint --node` preserves the non-blocking
   node-check exit contract, and `deps <id> --direction in|out` defaults to
   outbound. Existing MCP command names, schemas, and direct web routes remain
   stable.
7. Pack publication is gated by live-harness validation and the adapter
   conformance obligations in `dec.unified-cairn-dev-entry`. It carries only
   the measured activation claim. A quality claim requires new evidence, and
   the deferred six-arm trial may return only as a fresh unit with its
   environment restored.
8. `cairn locate <symbol>` remains an exact-name reverse lookup over persisted
   public symbol records. It returns every collision, each with owning node id,
   file, line, end line, kind, and signature. It performs no fuzzy match,
   ranking, first-wins selection, or separate extraction pass.
9. `cairn todo set <slug> <status>` accepts only
   `open|in_progress|done|blocked`, rewrites only the frontmatter status, and
   leaves the body untouched. Files remain truth and Git remains history.
   Validated mutation is state stewardship, while claiming, assigning,
   sequencing, and prioritising remain forbidden coordination. Direct edits
   remain legal but discouraged, with lint as the backstop.

The eleven decisions named in `supersedes` are historical detail after this
consolidation. This decision carries their live obligations; their frontmatter
is set to `superseded` so they remain available through history without
inflating the node's binding authority set.

## Rationale

This is the smallest consolidation that reaches the configured threshold. It
groups one coherent agent-workflow surface while retaining decisions whose
detailed contracts still govern active boundaries. Restating the live
obligations avoids treating shipped code as authority and avoids semantic loss
from status-only cleanup.

## Consequences

- `cairn.kernel.cli` has ten accepted decisions at the current threshold.
- Queries show this decision as the binding summary and retain the superseded
  decisions for provenance.
- Future changes amend or supersede this summary when they alter these
  obligations instead of adding another narrow accepted implementation record.
