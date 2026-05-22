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
