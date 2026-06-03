# Brownfield Extraction

Brownfield extraction turns an existing codebase into a Cairn blueprint without starting from scratch. It discovers structural candidates from the filesystem, groups them into modules, and writes a change proposal that a human reviews before archive.

## Commands

### `cairn init --from-code`

Scans the current repository, discovers module-like directories, and writes a brownfield-init change under `openspec/changes/brownfield-init/`.

```bash
cairn init --from-code
```

If the change directory already exists, the command fails. Pass `--force` to overwrite.

### `cairn refine`

Compares the current codebase against the existing blueprint and writes a timestamped change containing only proposed additions, removals, or modifications.

```bash
cairn refine
```

Each refine run creates a new change directory (`brownfield-refine-<timestamp>`) so prior proposals are never overwritten.

## Discovery heuristics

The discovery engine walks the filesystem and identifies directories that look like modules.

### Thresholds

| Threshold | Value | Description |
|---|---|---|
| Minimum files | 3 | A directory must contain at least 3 source files to become a candidate. |
| Maximum depth | 4 | Directories deeper than 4 levels below the repo root are skipped. |
| Supported languages | Rust, TypeScript, JavaScript, Python, Go | File extensions that count toward the minimum. |

### Confidence scoring

Each candidate receives a confidence score based on observed imports:

```
coupling_score = (internal_imports + 1) / (external_imports + 1)
```

- **High** (`score >= 2.0`): strong internal cohesion, few external imports.
- **Medium** (`score >= 1.0`): balanced internal and external imports.
- **Low** (`score < 1.0`): mostly external imports, weak cohesion.

Low-confidence candidates are still emitted but marked for review.

### Candidate naming

When the summariser is disabled (the default), IDs and names are derived mechanically from directory paths:

- `src/auth` becomes node ID `src.auth` and name `auth`.
- `a/b/c` becomes node ID `a.b.c` and name `c`.

If a summariser backend is configured, it may override these with inferred names and descriptions.

## Human review workflow

1. **Run extraction**: `cairn init --from-code` or `cairn refine`.
2. **Inspect the change directory**: Review `proposal.md`, `blueprint.delta`, and stub contracts in `contracts/`.
3. **Edit if needed**: Modify the blueprint delta or contracts directly, or discard the change and re-run.
4. **Archive when satisfied**: `cairn archive brownfield-init` (or the refine change ID) to apply the proposal.

The workflow is identical to any other Cairn change: extraction produces a proposal, a human reviews it, and only then is it archived into the blueprint.

## Output format

A brownfield change directory contains:

```
openspec/changes/brownfield-init/
  proposal.md           # human-readable summary
  blueprint.delta       # machine-readable delta operations
  contracts/
    src_auth.md         # stub contract per candidate
```

### `blueprint.delta`

Delta operations use `+` to add nodes and edges. Example:

```delta
# Blueprint delta
+ Module "auth" id "src.auth" {
    path "src/auth"
    edge -> src.store "persists sessions"
}
```

## Limitations

Brownfield extraction is a starting point, not a finished architecture. It cannot:

- Infer semantic relationships that are not visible in the filesystem or imports.
- Distinguish a well-designed module from a grab-bag of unrelated files in the same directory.
- Detect architectural layers (e.g., hexagonal ports and adapters) without code-level evidence.
- Guarantee that inferred edges are correct; they are structural siblings, not verified dependencies.

Always review generated proposals before archiving. The confidence score is a heuristic, not a correctness proof.

## MCP exposure

Both brownfield commands are registered as mutating MCP tools:

- `cairn_init_from_code`
- `cairn_refine`

They are hidden unless the MCP server starts with `--allow-mutating-tools` and the caller passes `"mutating": true`.

## Suggest engine

Both `cairn init --from-code` and `cairn refine` run the suggest engine after deterministic extraction. The engine looks for candidate pairs that share at least one tag but do not already have a deterministic edge. Each match produces bidirectional `related_to` suggestions.

### Queue file contract

Suggested edges are written to `openspec/changes/<change>/suggested-edges.json` using the phase-7.6 queue schema:

- `version`: always `1`.
- `entries`: array of `SuggestedEdgeEntry` values.

Every entry carries:
- `source` and `target`: node IDs.
- `relation`: the suggested relation verb (currently always `related_to`).
- `triage_state`: always `"pending"` at emission time. The engine never auto-accepts.
- `confidence`: the lower of the two candidate confidence scores.
- `provenance.trace_phase`: `"phase-9-brownfield"`.
- `provenance.stage`: `"propose"`.

### Archive gate

A brownfield change with pending entries blocks `cflx openspec validate <change> --strict` with error code `CC002`. Archive is not allowed until every entry is triaged to `accepted`, `rejected`, or `deferred`.

## Interview runner

The interview runner supports multi-round elicitation for brownfield onboarding sessions.

### Session file

A session is persisted at `openspec/changes/<change>/research/interview-session.json` while in progress. It contains:

- `version`: schema version (currently `1`).
- `change_id`: the change this session belongs to.
- `turns`: ordered Q/A pairs.
- `cursor`: index of the next unanswered question.
- `complete`: `true` when all questions have been answered.

### Resume semantics

If `interview-session.json` exists and `complete` is `false`, the runner resumes at the cursor. It does not restart. This lets a human suspend and resume an interview across multiple invocations without losing progress.

### Genesis transcript

When the session completes, the runner writes `openspec/changes/<change>/research/genesis.md` and removes the transient session file. The genesis transcript carries the user-visible Q/A turns plus the final premise. System prompts and intermediate session state stay out.

## Templated authoring

Organisations can declare contract templates that the brownfield generator consumes when drafting stubs.

### Template declaration

A `ContractTemplate` has three fields:

- `name`: a unique identifier for diagnostics.
- `match_rules`: a list of `MatchRule` values evaluated in order. A template matches when ANY rule succeeds.
  - `Path(substring)`: matches when the candidate's path contains the substring.
  - `HasTag(tag)`: matches when the candidate carries the tag.
- `body`: a template string with `{id}`, `{name}`, and `{description}` placeholders.

### Resolution order

The generator resolves templates in declared order. The first matching template wins. If no template matches, the generator falls back to the built-in stub.

### Precedence rule

Template body provides structure (headers, required sections). Summariser output provides content (names, descriptions, suggested tags). Where both supply text for a section, summariser content takes precedence and template text becomes a guidance comment in the draft.

### Error handling

A failed-to-parse template is skipped with a warning. It never blocks authoring. An empty template list is valid and simply means every candidate uses the built-in stub.
