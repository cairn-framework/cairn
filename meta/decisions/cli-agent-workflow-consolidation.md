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
refines:
  - dec.native-todos-first
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

The CLI node carried twenty accepted decisions. Eleven were narrow agent
guidance or point-contract rulings that had already shipped. Reading them as
independent binding authorities obscured their current shared contract and the
query, brownfield, MCP, and root boundaries they also govern.

This decision consolidates those obligations without changing behaviour. The
more detailed authorities for the original loop contract, unified loop entry,
reconciliation step, agent-pack packaging, and adapter roots remain accepted.

## Decision

### Retained loop authority

This consolidation does not restate or alter the development-loop contract.
`dec.adopt-cairn-dev-loop`, `dec.loop-command-harness-model`,
`dec.unified-cairn-dev-entry`, `dec.loop-reconcile-step`, and
`dec.loop-resolves-knowable-gaps` remain accepted and govern loop activation,
procedure, gates, reconciliation, gap handling, and harness ownership.

### Agent guidance and pack

The canonical, harness-neutral agent pack and deterministic adapters remain
governed by `dec.agent-pack-packaging` and `dec.pack-adapter-roots`. Init wiring
delegates to that manifest-owned installer instead of maintaining a second
direct-emission authority.

The shipped `cairn-explore` guidance includes the provenance query path through
`rationale`, `decisions`, `research`, and `sources`, while making clear that the
graph is not a source-symbol index. The retired `karpathy-guidelines` skill is
not restored. Its durable discipline stays absorbed in `cairn-dev` and the loop
implementation procedure: state material assumptions, write a checkable
criterion, prefer the smallest sufficient change, and ask only when readings
have materially different consequences.
Future pack promotions are judged on marginal lift over the current pack and
merge non-overlapping value into the owning skill before adding a new skill.

### CLI point contracts

The following shipped contracts remain binding:

1. `cairn brief` fuses the selected unit, its accepted decisions, node contract,
   task body, and gates. Proposed or superseded decisions are not binding. A
   unit without a node still receives universal gates and a node-binding hint.
   `brief` and `next` remain registered in help and command documentation.
   Selection reads the committed Beads export without a live Dolt dependency
   and prints a staleness note naming `bd ready` as authoritative.
2. `cairn context --json` retains `edge_count` and an `edges` array of full-id
   `{source,target,label}` objects. Human `Structure:` output lists every node
   once, shows labelled outbound edges with the system-root prefix stripped,
   omits redundant paths and default synced state, and retains anomalous state.
3. `cairn feedback` appends timestamped, versioned friction to
   `.cairn/feedback.md` and produces a prefilled upstream issue URL without
   network access. Init writes appendable `.cairn/AGENTS.md` guidance covering
   orientation commands, the scan-before-commit loop, and feedback before a
   workaround, then prints actionable next steps.
4. `cairn hook install`, `status`, and `uninstall` own only hooks marked
   `# Managed by Cairn. Do not edit.` and refuse unmarked files or dangling
   symlinks. They resolve the repository root and hook path through Git before
   scanner startup, work without a blueprint, preserve platform-correct
   permissions, install a script that invokes `cairn hook all`, and remain
   explicit opt-in.
5. `cairn init --from-code --apply` remains explicit opt-in, preserves the
   reviewable default, and delegates to the normal change-apply path with its
   conflict checks, validation, rollback, and archive logging. JSON mode emits
   the archive command envelope unchanged; the prose prefix is text-only.
6. Strict-subset commands remain folded without aliases. `get --symbols`
   conditionally adds symbols to `NodeResponse`; MCP retains `cairn_get` and
   does not restore `cairn_symbols`. `lint --node` preserves its non-blocking
   exit, missing-blueprint preflight, legacy-file rename guidance, and empty
   state copy; MCP retains `cairn_lint`. `deps <id> --direction in|out` defaults
   outbound; MCP retains `cairn_depends` and `cairn_dependents`, and the web
   routes remain `/api/depends` and `/api/dependents`.
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
   This verb is the single backend seam for a future `StateBackend` or GitHub
   projector; file-only writes remain the default.

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
