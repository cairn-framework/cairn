---
node: cairn.brownfield
---

# Contract: cairn.brownfield

## Purpose

Phase 9 brownfield extraction: typed candidate, confidence, and coupling-score
helpers plus orphan grouping shared across `cairn init --from-code`,
`cairn refine`, and the suggest engine. It turns an existing, un-blueprinted
codebase into proposed graph nodes by traversing the filesystem, scoring
directory cohesion, and clustering orphaned files into actionable suggestions.

## Public interface

- `heuristics` (re-exported): `Candidate` (path-derived `id` via
  `Candidate::new`), `CandidateConfidence` bucket enum, `coupling_score` and
  `classify_score`, and the threshold constants `MIN_CANDIDATE_FILE_COUNT = 3`,
  `DIRECTORY_DEPTH_LIMIT = 4`, `EDGE_OBSERVATION_THRESHOLD = 2`,
  `CONFIDENCE_HIGH` (2.0), `CONFIDENCE_MEDIUM` (1.0).
- `onboard`: `analyze(&[Finding]) -> OnboardReport`, grouping orphan findings
  into `OrphanCluster`s with a `ClusterSuggestion`, plus `render_human` and
  `render_json`.
- `decisions`: `index(&Path, &Graph) -> Result<EvidenceIndex, CairnError>` plus
  `OwnerResolver`, `Evidence`, `BoundEvidence`, `EvidenceKind`, `SCHEMA_VERSION`,
  `render_human`, and `render_json`: the deterministic decision-evidence index
  behind `cairn onboard decisions`.
- `mod` top level: `stub_contract`, `write_change`, and `blueprint_delta` build
  a brownfield change directory (proposal, blueprint delta, stub contracts).
- Further submodules: `discovery`, `walk`, `init`, `refine`, `suggest`,
  `summarise`, `interview`, `templates`.

## Invariants

- Coupling score is `(internal_imports + 1) / (external_imports + 1)`; the +1
  offset avoids division by zero and rewards internal cohesion at small totals.
- `Candidate.id` is path-derived by construction (`path_derived_id`); a
  different id source requires the explicit `with_id`.
- Onboard groups only findings with code `CAIRN_RECONCILE_ORPHANED_FILE`;
  directories matching `IGNORE_PATTERNS` classify as ignore candidates.
- Blueprint node names are emitted as barewords (`bareword`), since the grammar's
  name slot is a bareword, not a string.
- Discovery anchors candidates on package roots: a directory holding a package
  manifest (`package.json`, `pyproject.toml`, `Cargo.toml`, `go.mod`) accounts
  for every source file below it that no nearer package root claims, the depth
  budget restarts at each package root under an absolute traversal ceiling, and
  where package roots nest the innermost wins. A directory no package root
  claims still qualifies on holding three source files directly. Nothing inside
  a package-root candidate is proposed separately, so a source file directly
  under a workspace root dropped for enclosing another package is claimed by
  nothing and reconciles as an orphan
  (`dec.brownfield-package-root-discovery`).
- The decision-evidence index reads a closed evidence set: files under
  `docs/adr/` and `docs/decisions/`, README sections headed Decision,
  Rationale, or Invariant, source comments carrying the literal
  `// invariant:` or `# invariant:` marker, and the code targets discovery
  reports. Binding is path-to-blueprint: eligible leaf or `owns-files` nodes
  contribute normalised declared paths, most-specific first, and a
  path-derived discovery candidate id is evidence, never a node id. Evidence
  no declared path claims is reported unbound and a binding is never invented
  (`dec.brownfield-extraction-mechanism` clause 1).
- The invariant and README scans cover every source file the bounded survey
  observed and every directory holding one, not only discovery candidates: a
  directory below the candidate threshold still carries invariant comments and
  a README. Every collector that reads file contents skips symlinks, so no
  evidence text is read through a link. Discovery's own survey still counts a
  symlinked source file toward its candidate threshold, so a code-target
  directory can be link-influenced; that is discovery's rule, not the index's.
- An invariant marker counts wherever a real comment carries it, including
  after code on the same line, but not in a doc comment (`///`, `//!`) and not
  when quoted: an odd number of preceding quotes, or a quote or backtick
  immediately before the marker, means the line is prose about the marker
  rather than an assertion. Without that rule the index reports its own source.
- README evidence sections are ATX (one to six `#` then whitespace) or setext
  headings, matched case-insensitively against the whole heading text, and
  fenced code is skipped so an example heading is not evidence.
- Two limits of the index are deliberate. The marker rule is lexical, not a
  parser: a marker inside a multi-line string whose own line reads like a
  comment is still reported, since an advisory index costs less on a spare path
  than on a missed invariant. And README scanning reaches the root plus any
  directory holding a surveyed source file, so a README in a documentation-only
  directory is not indexed; the bounded survey records source files, not a
  directory inventory.
- Two different nodes declaring an equally specific matching path is a
  blueprint defect, not a binding: the resolver reports the evidence unbound
  rather than resolve a tie the reconciler resolves in declaration order, which
  the graph does not preserve. The command also refuses to index at all when
  the loaded graph carries structural errors.

## Dependencies

`cairn.brownfield -> cairn.kernel.map`: reads orphan findings, consuming the
`Finding` type (`crate::map::graph::Finding`) in `onboard::analyze`, and reads
declared ownership from `Graph` plus `map::paths` containment in
`decisions::OwnerResolver`. It is dispatched by the CLI
(`cairn.kernel.cli -> cairn.brownfield`) for the onboard command. Also uses the
crate-wide `CairnError` for write paths.

## Tests

Unit tests live in `#[cfg(test)]` modules within `src/brownfield/mod.rs`,
`heuristics.rs` (score and bucket classification), and `onboard.rs` (orphan
grouping, classification, and human/JSON rendering), exercised alongside the
crate's integration tests under `tests/`. The decision-evidence index is
covered by `tests/kernel.rs` (binding, JSON wire, usage error, and the
blueprint requirement) and by `tests/onboard_owner_parity.rs`, which pins the
onboard owner resolver to the reconciler's fixture expectations.
