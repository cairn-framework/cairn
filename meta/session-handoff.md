# Session Handoff: 2026-07-31 (cairn-loop)

Working tree clean, parked at `origin/main`. `cairn scan --strict` exit 0,
`cairn hook all` exit 0.

## What was done (this session)

Five PRs merged, in order:

| PR | What | Merged |
|---|---|---|
| #544 | `todo.decision-ratification-tiers` implemented: the ratification tier machinery | `d0d1296` |
| #545 | `dec.parked-deferral-composition` accepted on the maintainer's signature | `ee1550e` |
| #547 | Two false claims corrected in `dec.bootstrap-fixture-corpus-split` | `a004b11` |
| #548 | `todo.portfolio-hygiene` authored | `37f0b3a` |
| #549 | `driver-v2-selection` change proposed | `725f7fe` |
| #550 | `todo.release-next-milestone` authored | `cb62620` |

### The tiers unit (#544)

A decision declares `ratification: local` or `binding`; absent means
`binding`, so nothing existing changed protection. A `local` decision may be
accepted only under the receipt protocol: two committed Review receipts from
independent lenses, bound by `subject_hash` to a canonical manifest of
everything the decision governs, plus `ratified_by: machine` and a
For/Against/Verdict record when the loop signs.

Surfaces: schema fields and CA045-CA057; the manifest hasher (governed-content
stripping that mirrors `frontmatter::parse`, identity-based receipt exclusion,
directory expansion, symlink containment); scanner checks (span via structural
parent links, supersession, binding-surface allowlist with both sides
canonicalised, convergence with committed lens-prompt hash binding); the
range-based commit hook (merge-base to index in pre-commit, to HEAD under
`--head` in CI, NUL-delimited paths, rename-safe, untracked and unstaged
governed refusal, binding-surface classification against the MERGE-BASE
allowlist); wire `schema_version` 8 with tri-state `ratified_by` and the
candidate `subject_hash`; and the tier-aware never-self-ratify rule in both
skill copies with committed lens prompts at `docs/agent/lenses/`.

The two-lens review ran twice in sequence and found **13 blockers** across
both rounds, plus a third focused pass on the post-review delta that found the
trigger still trusting the worktree. Every one was fixed at its source with a
regression test: rename riders, untracked governed files, C-quoted paths, a
range rewriting its own allowlist, nested symlink aliases into a binding
surface, and a staged acceptance reverted in the worktree.

## The protocol's first live exercise, and what it caught (#547)

Accepting `dec.bootstrap-fixture-corpus-split` (the one `local`-tier decision)
was attempted under the real protocol. Two independent receipt-grade lens
reviews both returned **BLOCKING** on the same claim: the decision asserted
that every declared node in the fixture carries an anchoring decision, but
`check_provenance_coverage` requires one only for leaf nodes and the fixture's
System node `cairn` has none. A second finding corrected wording that
conflated who signs with which checks run.

Both were fixed and the decision **stays `proposed`**. The maintainer's
signature covered the previous wording; correcting governed content after a
signature means the corrected text needs its own word. Re-wording moved the
candidate hash from `sha256:4ab84fee` to `sha256:5552edec`, so no receipt
bound to the old text could bind, which is the protocol behaving as ratified.

## Waiting on the maintainer

1. **`dec.bootstrap-fixture-corpus-split`**: sign the CORRECTED text, or
   reject. It is the only entry in `cairn pending`. On signature, acceptance
   needs two fresh receipts against `sha256:5552edec` (the hash
   `cairn pending --json` prints) landing in one range whose whole diff sits
   inside its `affects:` list.
2. **Folding item 3 (accumulation threshold)**: the approved "kind-scaled
   thresholds" option rested on a false effect claim of mine. Blueprint-verified
   facts: `cairn.root` is the System node, `cairn.kernel` is the ONLY Container,
   and `cairn.kernel.cli` and `cairn.ui` are Modules. So 10/15/20 clears the
   `cairn.root` Info only; `cairn.kernel.cli` (12 > 10) stays firing. It is
   non-selecting anyway under the strict-green fold. Nothing was landed;
   `local://folding3-spec.md` holds the corrected spec.

## Standing findings (all Info, strict green)

Two `CAIRN_RESEARCH_ORPHAN`, three `CAIRN_SOURCE_UNVERIFIED`, the spec:634
deferred footnote, and two `CAIRN_DECISION_ACCUMULATION` (`cairn.kernel.cli`
and `cairn.root`, both at 12 against a flat 10), owned by
`todo.lint-selection-folding` item 3.

## Next work

1. Sign or reject the corrected corpus-split decision (above).
2. Decide folding item 3 under the corrected facts (above).
3. `todo.ratification-candidate-pointer`: candidate discovery in the
   ratification hook hardcodes `meta/decisions` rather than resolving the
   configured pointer, so an adopter with a non-default pointer gets a silent
   gate. Filed from the post-merge review of #544.
4. `todo.portfolio-hygiene`: the 32-todo mission sweep, driver-scheduled.
5. `driver-v2-selection`: the proposal awaits a word; task 1 is a read-surface
   audit that decides whether the mission line can be built from the JSON.
6. `todo.release-next-milestone`: cut when the gate in it is satisfied.
7. Cut 1b of `todo.lint-selection-folding` (decision-side `defers:`) remains
   unratified.
