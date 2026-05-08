# Hooks

Cairn hooks enforce structural and interface integrity at task boundaries (pre-commit, CI, agent-task-end). Three kinds exist, each with distinct semantics.

## Hook kinds

### `cairn hook structural`

Blocks on structural errors: duplicate node IDs, path ties, broken artefact pointers, reference orphans, source checksum mismatches, orphaned files.

**Exit codes:**
- `0`: no structural errors found
- `1`: one or more structural errors found (blocks commit)

### `cairn hook interface`

Blocks on unresolved interface contradictions: a module's current interface hash differs from what is recorded in `.cairn/state/interface-hashes.json`.

**Exit codes:**
- `0`: no interface contradictions found
- `1`: one or more interface contradictions found (blocks commit)

### `cairn hook tension`

Reports rationale tensions (advisory findings) without blocking. Always exits `0`.

Reports: orphan sources, research not linked from decisions, decision cite chains pointing to deleted artefacts, revisit triggers that appear relevant.

**Exit codes:**
- `0`: always (tensions are advisory, not blocking)

### `cairn hook all`

Combined semantics. Runs structural and interface checks. Reports tensions.

**Exit codes:**
- `0`: no structural errors and no interface contradictions
- `1`: structural errors or interface contradictions found

Tensions do not independently fail the hook; they appear in the report alongside blocking findings.

## CLI flags

| Flag | Description |
|---|---|
| `--json` | Emit JSON output |
| `--output <path>` | Write output to file instead of stdout |
| `--changes-dir <path>` | Use custom changes directory (default: `meta/changes`) |
| `--file <path>` | Use custom blueprint path (default: `cairn.blueprint`) |

## Integration

### Pre-commit hook

Install the pre-commit hook once:

```sh
./scripts/install-pre-commit-hook.sh
```

This installs a Git pre-commit hook that runs `cargo fmt --check` and `cairn hook all` before every commit. Both must pass for the commit to succeed.

### CI integration

In CI pipelines, invoke `cairn hook all` after `cargo fmt --check`:

```sh
cargo fmt --check
cargo run --quiet -- hook all
```

For JSON output in CI:

```sh
cargo run --quiet -- hook all --json > hook-report.json
```

### Agent task end

For agent-task-end integration, invoke `cairn hook all` as the final step before task completion:

```sh
cairn hook all
# or, from a Cargo project:
cargo run --quiet -- hook all
```

The script `scripts/cairn-hook.sh` is provided for environments where `cairn` may not be in `$PATH`:

```sh
./scripts/cairn-hook.sh
```

## Exit code reference

| Command | 0 | 1 |
|---|---|---|
| `cairn hook structural` | No structural errors | Structural errors found |
| `cairn hook interface` | No interface contradictions | Interface contradictions found |
| `cairn hook tension` | Always (advisory only) | Never |
| `cairn hook all` | No errors or contradictions | Errors or contradictions found |

## Output format

Human output:
```
Hook: <kind>
Blocks: <true|false>
Findings: <count>
Elapsed: <ms>

<severity>: <code> <message>
...

Conflicts:
<severity>: <code> <message>
...
```

JSON output:
```json
{"kind":"<kind>","findings":[{"code":"...","severity":"Error","message":"..."}],"conflicts":[],"exit_decision":"Pass","elapsed_ms":0,"output_paths":[]}
```