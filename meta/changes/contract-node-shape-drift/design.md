# Design: Contract node-shape drift

## Approach

Compare the node shape a contract was accepted against with the node shape the
blueprint declares now. The current shape is already available as
`NodeFingerprint` (`src/scanner/state.rs`); the missing piece is the accepted
shape, recorded when a contract becomes canonical, in a reduced record the
scanner can diff against (question 3).

The four questions the originating todo raised are settled below. Each answer is
restated as a testable criterion in `specs/contract-node-shape-drift.md`.

## Question 1: tier

**Ruling: Warning.**

`cairn scan --strict` exits 1 on Warning (`src/cli/mod.rs`, `--strict` help
text), so this is the load-bearing choice. Three properties justify Warning
here.

What the signal proves, precisely: this contract has not been reviewed against
the node's current shape. It does not prove the prose is wrong. A baseline
records the shape a human last checked the contract against, so a difference is
exact review-freshness evidence, the same class of claim as the interface hash
rule, and it names the fields that moved. There is no scoring, no threshold, and
no bag of words. `dec.revisit-trigger-correlator-deferred` rejected the
revisit-trigger correlator because cairn has a single advisory channel and a
proximity proxy would mostly fire where a re-read concludes "not fired",
training inattention on that one channel. That argument condemns advisories
whose input does not establish their claim. This finding claims only what its
input establishes, which is also why the wording says the baseline is old rather
than that the prose is false.

The check is opt-in by construction, which is what makes Warning safe. Under
question 3 a node is compared only once a baseline has been explicitly recorded
for it, by `accept()` or by the re-record surface. A repository that has
recorded none has an empty baseline set and can never go red. This is exactly
the shape of the adjacent Warning-tier contract rule:
`CAIRN_CONTRACT_INTERFACE_DRIFT` (CK008) is Warning, and it is safe at that tier
because contracts with no `interface:` block are never checked
(`src/scanner/checks.rs`).

Remediation is available, but only once the re-record surface exists. Clearing
the finding means re-reading the prose against the new shape and re-recording
the baseline, and re-accepting is not that path:
`res.contract-baseline-rerecord-reachability` shows why. That is why the
non-generative re-record surface is an unconditional prerequisite of the
enforcer rather than a footnote, and why Warning is the right tier once both
have landed.

Rejected: **Info**. It understates a finding saying a human-authored document
has never been checked against the declaration it describes, and it parks an
exact signal on the channel `dec.revisit-trigger-correlator-deferred` protects.

## Question 2: finding code and wording

**Code:** `CAIRN_CONTRACT_NODE_SHAPE_DRIFT`. Its registry number is **not**
allocated here. `docs/conventions.md` rule 2 requires a new code to be appended
to `docs/registries/error-codes.md` in the same commit that introduces it in
Rust source, and this unit writes no Rust. The enforcer's commit allocates the
next free number in the CK block, expected CK033, and fills the rule row's
`Code` cell in the same commit. Reserving it early was considered and rejected:
the argument for visibility does not outrank a binding convention, and rule 5's
sequential allocation means an intervening change could take CK033 anyway.

**Registry row.** A `pending` rule row lands with this change in
`docs/registries/spec-rules.md`, naming the rule with `Code` still `-`, which
the registry defines as "no enforcer is named yet". Its `Spec` cell is `-`
because this registry owns the rule outright, with no `docs/spec.md` anchor, and
its `Deferred-by` cell names `dec.contract-node-shape-drift-deferred`. That is
the registry's deliberately-deferred case: the build is parked behind a
prerequisite capability, the non-generative re-record surface below, with the
rationale in a decision artefact and its evidence in
`res.contract-baseline-rerecord-reachability`. The Info message then names the
parking inline, so a reader of `cairn scan` learns why the rule is unbuilt
without opening a todo. The row and the decision are canonical where they live;
this document does not restate their text.

That row emits one CK004 Info until the enforcer ships, which is the mechanism's
designed behaviour and is accounted for in `proposal.md`. When the enforcer
lands, the row gains its `Code` and is promoted to `enforced`, and the Info
clears.

**User-facing text** lives in `docs/design-system/copy.toml` under
`[findings.codes]`, in the heading/body/cta shape its neighbours use. Two
constraints on it belong here rather than in that file:

- The body carries a `{node}` slot and a `{target}` slot. `{target}` is the
  comma-separated list of changed fields, in the order `kind`, `parent`,
  `edges`, so the reader learns what moved without opening the blueprint. The
  `Finding.target` field already exists for this (`src/map/graph.rs`).
- Nothing interpolates those slots automatically. The CLI prints
  `finding.message` verbatim (`render_finding_lines`,
  `src/cli/format/render.rs`) and the web UI prints `item.message`
  (`src/ui_assets/channel-bar.js`), so the emitter resolves the leaf key
  `findings.codes.CAIRN_CONTRACT_NODE_SHAPE_DRIFT.body` with
  `crate::copy::lookup` and substitutes both slots itself, the way
  `findings.deferred-collapsed` is resolved at its emitting site. `lookup`
  returns the key text for a non-string value, so naming the table rather than
  the leaf would print the key. A message that reaches a renderer still holding
  `{node}` or `{target}` is a bug.

## Question 3: baseline schema and migration

**Schema.** `.cairn/state/contract-baselines.json` reuses the top-level
`version`/`nodes` envelope of `blueprint-snapshot.json`, `version` serialised
first per conventions section 3, at version `1`:

```json
{
  "version": 1,
  "nodes": {
    "cairn.kernel.scanner": {
      "kind": "Module",
      "parent": "cairn.kernel",
      "edges": ["cairn.kernel.blueprint", "cairn.reconcile"]
    }
  }
}
```

The entry type is **not** `NodeFingerprint`. That struct serialises a mandatory
`paths` field (`src/scanner/state.rs`), so writing it would emit `paths`, and
reading the shape above back into it would fail on the missing field. The
enforcer declares a separate reduced record holding `kind`, `parent`, and
`edges`, constructed from a `NodeFingerprint` by dropping `paths`.

**Compared fields: `kind`, `parent`, `edges`.** `paths` is deliberately excluded.
`check_blueprint_change_decisions` already leaves path-only edits ungated
("path-only change must not be gated", `src/scanner/tests.rs`), and gating them
here would contradict that with no new information: moving a file does not
change what a contract asserts. Storing `paths` in the baseline and then not
comparing it would invite a later reader to "fix" the omission, so the field is
absent from the schema rather than present and ignored.

**Migration: explicit recording only. No automatic backfill.** An entry appears
only when a writer records it: `accept()` at accept time, or the re-record
surface when a user asks. A repository with contracts but no baseline file scans
exactly as it does today: no baselines, no comparisons, zero new findings, on
upgrade day and every day after, until something records one. Nothing in this
rule can flag on upgrade day, and the specs must not claim otherwise.

Rejected: **backfilling current fingerprints automatically at first scan.** It
buys universal coverage by asserting something nobody verified. The baseline's
claim is "a human checked this contract against this shape"; a backfilled entry
claims that for prose no human ever checked, and every later blueprint edit then
flags on the strength of a fabricated review. It also converts a safe opt-in
Warning into a universal one, which is the outcome the tier question exists to
avoid. A user explicitly recording a baseline for a hand-authored contract is
not backfill: it is the review this rule tracks.

## Question 4: recording point

**Confirmed: `src/summariser/accept.rs` is the accept-time writer, and the
scanner is never a writer.** `accept()` is the one place a contract's canonical
text is installed and a hash is recorded, and it already performs a post-write
scan with rollback, so the fingerprint it records is one the graph accepted. The
baseline write belongs immediately after that scan succeeds, on the same path
that constructs `AcceptedDraft`. The only other sanctioned writer is the
non-generative re-record surface under "Prerequisite" below, which exists so a
baseline can be refreshed deliberately, by a user, without generating a draft.

The scanner reads the file and never writes it. `scan` owns persistence of
`interface-hashes.json` and `blueprint-snapshot.json` (`src/scanner/mod.rs`);
neither it nor `load_project` may write `contract-baselines.json`, because a
baseline the scanner refreshed would silently re-accept the drift it exists to
report.

**Correction to the todo's premise.** The todo describes `accept.rs` as
re-recording into the interface-hash state. It does not. Its
`accepted_interface_hash` is a hash of the contract *text*
(`compute_hash(&contract_text)`) stored in the draft store under
`.cairn/state/summariser/`, while `.cairn/state/interface-hashes.json` holds
code-target hashes written by the scanner. The two are unrelated. The baseline
is therefore a third recorded value: contract text hash (draft store), code
target hashes (scanner), node shape baseline (this change). It reinterprets
neither.

## Changes

ADDED:
- `meta/changes/contract-node-shape-drift/` (this change and its spec).
- `docs/registries/spec-rules.md`: pending rule row for
  `CAIRN_CONTRACT_NODE_SHAPE_DRIFT`, `Code` cell left empty until the enforcer's
  commit allocates the number, `Deferred-by` naming the decision below.
- `docs/design-system/copy.toml`: `[findings.codes]` entry for the same code.
- `meta/decisions/contract-node-shape-drift-deferred.md` (`proposed`): the
  deferral ruling the registry row cites.
- `meta/research/contract-baseline-rerecord-reachability.md`: the code evidence
  that ruling rests on.
- `meta/sources/summariser-accept-path.md` and
  `meta/sources/query-api-draft-generation.md`: the inspected code, registered
  `unverified` so live source files are not hash-pinned.
- `meta/todos/todo.contract-baseline-rerecord-surface.md`: the prerequisite this
  change's evidence proved necessary (see below).

MODIFIED:
- None under `src/`. This unit writes no source code.
- `meta/todos/todo.contract-blueprint-staleness.md`: body reduced to a pointer at
  `specs/contract-node-shape-drift.md`, which is now its binding contract, with
  the two premises this change disproved removed. It stays `blocked`: the
  dependency it was waiting on is delivered here, and the re-record prerequisite
  below takes its place.
- `meta/todos/todo.contract-node-shape-drift-proposal.md`: closed `done`, with an
  Outcome recording the two Scope premises this unit narrowed.
- `map.json`: regenerated scan snapshot, carrying the two unverified-source Infos
  and the two deferred-rule findings.

REMOVED:
- None.

RENAMED:
- None.

## Prerequisite: a non-generative re-record surface

`res.contract-baseline-rerecord-reachability` settles the summariser-disabled
case: an accepted draft is terminal and a fresh one needs a live summariser, so a
repository that disabled the summariser after accepting holds baselines it cannot
re-record. A shape change there would produce a Warning whose only remediation is
re-enabling an LLM backend.

Shipping a Warning-tier finding into that state is the defect, not a footnote, so
a non-generative surface for recording, re-recording, and dropping a baseline is
an unconditional prerequisite of the enforcer unit. `dec.contract-node-shape-drift-deferred`
is the ruling; `meta/todos/todo.contract-baseline-rerecord-surface.md` (node
`cairn.summariser`) is the work, and it is listed in the enforcer todo's
`Depends on`.

## Residual risk

Coverage is zero until a repository records baselines, through either sanctioned
writer. In this repository, whose contracts are hand-authored, the baseline set
stays empty and the check is inert until that changes. If recording stays at
zero the enforcer is dead code, and the right response is to widen the recording
surface, not to backfill.
