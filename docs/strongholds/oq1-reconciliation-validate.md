# OQ1: reconciliation validate failure root cause

## Method

Investigated by running the actual validator binary at the repo root, inspecting its source code in the cargo registry, listing OpenSpec changes and specs, and tracing whether the validator ever touches `openspec/specs/`. Specific commands: `cflx openspec validate reconciliation --strict` (per-spec attempt), `cflx openspec validate --strict` (full battery), `cflx openspec list --specs`, `cflx openspec list`. Source read at `/Users/george/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cflx-0.6.20/src/openspec_cmd.rs` to confirm what the validator iterates and what fields it enforces. Compared against the structural read in cross-check E and the report from image #65.

Note on tooling: there is no `cflx.py` in the repo. `cflx` is a Rust binary at `~/.cargo/bin/cflx` (version `0.6.20 (20260428131134)`); the Python `cflx.py` referenced in the original investigation prompt is stale terminology from an earlier era. The current invocation is `cflx openspec validate ...`, not `python cflx.py validate ...`.

## Findings

### Validator output (verbatim)

Per-spec attempt:

```
$ cflx openspec validate reconciliation --strict
✗ Validation failed:
  Change 'reconciliation' not found
EXIT: 1
```

Full validate (strict):

```
$ cflx openspec validate --strict
✗ Validation failed:
  phase-9-brownfield: proposal.md missing 'Change Type' field (must be one of: hybrid, implementation, spec-only)
  phase-0-foundation: Missing proposal.md
  phase-0-foundation: Missing tasks.md
  phase-0-foundation: No spec deltas found (required in strict mode)
  phase-10.0-tests: proposal.md missing 'Change Type' field (must be one of: hybrid, implementation, spec-only)
  phase-8-summariser: proposal.md missing 'Change Type' field (must be one of: hybrid, implementation, spec-only)
  phase-8.0-tests: proposal.md missing 'Change Type' field (must be one of: hybrid, implementation, spec-only)
  phase-10-distribution: proposal.md missing 'Change Type' field (must be one of: hybrid, implementation, spec-only)
  phase-9.0-tests: proposal.md missing 'Change Type' field (must be one of: hybrid, implementation, spec-only)
EXIT: 1
```

Full validate without `--strict`:

```
$ cflx openspec validate
✗ Validation failed:
  phase-0-foundation: Missing proposal.md
  phase-0-foundation: Missing tasks.md
EXIT: 1
```

Total: **9 strict-mode errors across 7 active change directories**, plus a separate `Warning: Ignoring invalid change directory 'phase-0-foundation' (missing proposal.md)` emitted by `cflx openspec list`.

### Full validate comparison vs image #65

Image #65 was reported as showing 20 items with 11 failing, including a "spec/reconciliation" entry. The current validator emits **9 errors across 7 changes** (plus zero entries for any spec). This does NOT match the 11/20 split. The image was almost certainly produced by an instrument that listed both spec entries (13 specs from `cflx openspec list --specs`) and change entries (7 from `cflx openspec list`) in a combined view, then cross-applied the change errors against spec names. There is no execution path in `cflx openspec validate` that ever produces a "spec/reconciliation: FAIL" line, because the validator never iterates `openspec/specs/` at all.

The 11/20 reading in the image was the human reader (or whatever rendered the screenshot) conflating two distinct outputs: the spec list and the change-validate output. The actual ground truth on `dev` HEAD `c98d506` is: **`reconciliation` does not fail validation, because it is not the kind of artifact the validator considers.**

### Root cause

There is no failure on `openspec/specs/reconciliation/spec.md`. The screenshot was misinterpreted. The concrete reasons:

1. **`cflx openspec validate` only walks `openspec/changes/`**, never `openspec/specs/`.
   Source confirmation, `openspec_cmd.rs:466-484`: the "validate all" branch reads `self.changes_dir` (which is `openspec/changes/`) and skips `archive` and dot-prefixed dirs. There is no parallel iteration of the published specs directory.

2. **Per-spec invocation is rejected outright.** The validator's `change_id` argument is matched against active change directories via `find_change_dir(id)`, which only searches `openspec/changes/<id>/`. Calling `cflx openspec validate reconciliation --strict` returns `Change 'reconciliation' not found` because no such change directory exists. There is no mechanism to validate a published spec at all from the CLI.

3. **The closest thing to spec validation is `validate_specs_dir`** (`openspec_cmd.rs:629`), but it operates on the `specs/` *subdirectory inside a change* (i.e., the spec deltas for that change), not on `openspec/specs/`. The active change `phase-9-brownfield` has a `specs/brownfield/` delta, and no active change carries a `specs/reconciliation/` delta. The only change that ever did was `phase-5-edges-docstrings`, which is archived (commit `dc058e4`). Archived changes are explicitly skipped during validation.

4. **`openspec/specs/reconciliation/spec.md` itself is structurally well-formed.** The file (71 lines) declares 3 top-level requirements with `### Requirement:` headers and 7 scenarios with `#### Scenario:` headers. It is the standard published spec format consumed by archive promotion, not by validation. There is no schema gate for these files because they are "compiled output" of the archive step, not authoring artifacts.

5. **The 9 errors that DO fire are in change proposals, not specs.** Six changes (`phase-9-brownfield`, `phase-10.0-tests`, `phase-8-summariser`, `phase-8.0-tests`, `phase-10-distribution`, `phase-9.0-tests`) lack the `Change Type:` frontmatter field that strict mode requires (added in cflx 0.6.x, source: `openspec_cmd.rs:525-545`). One change (`phase-0-foundation`) is an empty stub directory containing only `.DS_Store`, triggering 3 errors (missing proposal, missing tasks, no deltas).

### Adversarial pressure-test results

- **Could the failure be in a transitive dep that reconciliation references?** No. The validator does not chase cross-references between specs. It only scans for `### Requirement:` / `#### Scenario:` headers and `## ADDED|MODIFIED|REMOVED Requirements` delta markers within a change's `specs/` subdirectory. There is no link-traversal path that could surface a "spec/reconciliation FAIL" line through transitivity.

- **Could the validator be skipping reconciliation silently while showing a stale pass indicator?** The validator never tries to validate published specs in the first place, so there is nothing to "skip silently." `cmd_validate` at `openspec_cmd.rs:1441` prints either `✓ Validation passed` or `✗ Validation failed` followed by the explicit error list. The 9 errors I see above are the entire output. No spec entries are emitted at all.

- **Could the test pass on this machine but fail in CI / on another branch?** I ran on `dev` at HEAD `c98d506`, which is the current default branch. The validator binary is `cflx 0.6.20 (20260428131134)`, not a build-from-source variant. The validator behavior is purely a function of file-system state (it does not call external services), so `dev` and CI must agree. Other feature branches with broken proposal.md files would emit additional errors, but cannot emit spec-level errors because the spec-validation code path does not exist.

- **Could image #65 have been from an older cflx that did validate published specs?** I checked cflx 0.5.12 (also installed in cargo registry). It did not have an `openspec validate` subcommand at all (the openspec utilities were added in 0.6.x). So no historic version of the binary validated published specs either. The image cannot have come from an older cflx that lost this feature.

- **Could it have been from a different tool entirely (a Python helper, hooks, a graphify check)?** I searched the repo for any Python validator (`find ... -name "*.py" -exec grep -l validate {} \;`) and for `cflx.py`. No such helper exists. The Makefile targets (`check`, `status*`) do not invoke any spec validator. The graphify output at `graphify-out/` is a knowledge-graph artifact, not a validator.

- **Could the user have manually tagged some specs "FAIL" in a different artifact (cross-check log, audit doc) and then reported it as validator output?** Plausible. Cross-check E itself flagged that it could not reproduce the per-spec failures, which is the strongest signal that the original screenshot was a misread of a different rendering surface (e.g., a TUI tab that mixed spec and change entries, or a custom dashboard line wrap that grouped "FAIL: phase-9-brownfield" under a "spec/reconciliation" subheading because reconciliation was the topic of an adjacent column).

The conclusion survives all pressure tests: there is no validator code path that would mark `openspec/specs/reconciliation/spec.md` as failing, the file itself is structurally correct, and no failure exists today.

## Decision

- **Verdict**: **already-fixed** (more precisely: never broken: image #65 was a misread).
- **Reasoning**: The `cflx openspec validate` binary does not validate published specs; the spec file is well-formed; the only real validate failures are 6 missing `Change Type:` fields in active phase proposals plus the empty `phase-0-foundation/` stub directory. None of these touch `reconciliation`.
- **Confidence**: **high**. The evidence is direct (source read of the validator + actual run on current HEAD + pressure tests across alternative tooling, alternative versions, transitivity, and CI divergence). The only residual uncertainty is what the screenshot actually showed, which is unanswerable without re-rendering the same view from the same git state, but irrelevant, because no current path produces that line.
- **Recommended fix**:
  1. **No change to `openspec/specs/reconciliation/spec.md` is needed.** Bundle B should not absorb a "fix reconciliation spec" task because there is nothing to fix.
  2. **Sweep phase scope changes**. The "spec-validate-cleanup sweep phase" should drop the per-spec validation framing entirely and instead target the actual 9 errors:
     - Add `**Change Type**: <hybrid|implementation|spec-only>` to the proposal.md frontmatter of: `phase-8-summariser`, `phase-8.0-tests`, `phase-9-brownfield`, `phase-9.0-tests`, `phase-10-distribution`, `phase-10.0-tests`. Single-line edit each, no logic change.
     - Decide what to do with `openspec/changes/phase-0-foundation/` (currently an empty `.DS_Store`-only stub). Either delete the directory or populate it with the foundational proposal/tasks/specs. Most likely it should be deleted, since `phase-0-foundation` already exists in `openspec/changes/archive/`.
  3. **Cross-check E was correct.** The "manual structural read showed nothing wrong" finding was right; the contradiction was on the screenshot side, not the spec side.

## What this changes for the integrated plan

The wave-3 sweep phase no longer needs to re-validate published specs and shrinks substantially: it becomes a 6-line frontmatter edit across six proposals plus a directory-cleanup decision on `phase-0-foundation`, total surface area under ~30 minutes. Bundle B does not need to absorb any reconciliation-spec fix into its first commit, because there is no fix to absorb; B can remain focused on its UX-foundation scope without the spec-validate-cleanup surcharge that was tentatively allocated to it.
