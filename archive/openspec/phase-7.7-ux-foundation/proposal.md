# Proposal: Phase 7.7 UX Foundation

**Change Type**: hybrid

## Dependencies

- `phase-7.7.0-tests` (required; ships the test contract this phase grades against by removing `#[cflx_planned]` group-by-group).
- `phase-7.5c-verification-states` (recommended ordering only; not a code dependency).

Execution: MAY run in parallel with the other active phases (`phase-8-summariser`, `phase-9-brownfield`, `phase-10-distribution`). Touches the consumer side of the existing `Finding` stream and the read-only `/api/lint` endpoint; no kernel artefact additions beyond a single `FindingSeverity` variant.

## Sequencing

This phase has no hard ordering against the other active phases. Internal commit sequencing is strict because each step consumes only artefacts produced by earlier steps.

1. Add `FindingSeverity::Info` to the kernel enum and emit it from orphaned and unverified producers. Touches every existing `match` over `FindingSeverity` (around fifteen sites) and three to five producer sites.
2. Create the centralised copy file at `docs/design-system/copy.toml` with two top-level sections, `[empty-states]` keyed by surface state and `[findings]` keyed by finding code.
3. Update the voice section of `docs/design-system/README.md` and add a voice review checklist as bullets in that file.
4. Add the `cairn check` CLI subcommand as a thin wrapper over the existing `query::lint` flow with human-readable output only.
5. Add the empty-state component to `docs/design-system/components.css`. Sweep `src/ui_assets/app.js` to replace the ten inline empty-state strings with the new component, reading copy from the centralised file. Add CLI parallel empty-state copy in `src/cli/format.rs` and `src/cli/mod.rs` no-args path.
6. Add the Findings rollup panel in `src/ui_assets/app.js` that reads from `/api/lint`, renders three severity buckets, a scope toggle (whole map vs single node), and a category filter on finding code prefix.
7. Add the prose-nudge banner at the top of the node-detail panel that reads from the `[findings]` section of the copy file, keyed by finding code, and renders a copy-pasteable CLI snippet as the call to action.

Steps 1 and 2 are foundation. Step 3 is documentation. Step 4 establishes the CLI single-source point. Steps 5 to 7 are the user-facing UI work.

## Problem/Context

The cairn web UI today renders empty states as bare declarative strings such as "No contracts attached." with no next move, scattered across roughly ten inline sites in `src/ui_assets/app.js`. The reconciler emits structured `Finding` values with `code` and `severity`, but only two severity buckets exist (`Error`, `Warning`), so a UI that needs to surface low-priority advisory signals (orphaned files, unverified contracts) has nowhere to put them. The `/api/lint` endpoint already serves the finding stream, and `cairn lint` already renders the same stream as text and JSON, but no shared rendering vocabulary turns those findings into plain-English diagnostics with copy-pasteable CLI snippets. No central copy file exists for UI strings; the only centralised string registries are `error-codes.md` and `declared-items.md`.

Three concrete user-facing gaps follow.

1. Empty states do not name the next move. The roadmap stronghold and Batch A flagged this as a critical broadening-of-audience problem: users new to cairn open the explorer, see "No X." and have no path forward. The CLI has the same gap on `cairn` with no arguments and on `cairn lint` against a clean map.
2. Reconciler findings render as raw codes (for example `CE001`) without prose translation. A user who sees `CE001` in the panel does not know whether to edit the blueprint, run a different command, or ignore the finding.
3. There is no rollup view of findings. The webui surfaces findings per-node in two scattered places (the empty-inspector recent-findings stub and the changes drawer), but no panel groups them by severity or filters them by category.

The roadmap stronghold also flagged a category error in the original Bundle B naming: it called the new inspection command `cflx check`. `cflx` is the workflow runner; `cairn` is the framework. Map queries belong on `cairn`. The cross-check confirms the rename to `cairn check`.

## Proposed Solution

Add seven things in the sequence above.

1. A new variant `FindingSeverity::Info` in the kernel `Finding` enum. Producers for orphaned-file states and unverified-contract states emit `Info` findings. The third bucket is producer-side, not render-time inferred.
2. A central copy file at `docs/design-system/copy.toml` with TOML structure. Two top-level sections: `[empty-states]` keyed by surface state and `[findings]` keyed by finding code.
3. A voice section update in `docs/design-system/README.md` plus a voice review checklist as bullets in that same file (single source for voice rules, alongside tokens).
4. A new `cairn check` subcommand that calls the same `query::lint` library function as `cairn lint` but renders inspection-flavoured output (always exits zero, no gate semantics). Accepts an optional node argument for scope-toggle parity with the webui panel: `cairn check` reports on the whole map, `cairn check <node>` reports on a single node.
5. An empty-state component in `docs/design-system/components.css` (icon plus heading plus body plus call-to-action pattern, consuming only existing tokens). The webui sweep replaces inline strings; the CLI sweep adds parallel CTAs to `src/cli/mod.rs` no-args path and `src/cli/format.rs` clean-map output.
6. A Findings rollup panel in `src/ui_assets/app.js` that consumes `/api/lint`, groups results into three severity buckets (`Error`, `Warning`, `Info`), provides a scope toggle (whole map vs single node), and filters by finding-code prefix.
7. A prose-nudge banner at the top of the node-detail panel that looks up `[findings.<code>]` from the copy file, renders heading plus body plus a copy-pasteable CLI snippet as the call to action.

The phase does NOT introduce write paths on the webui API. All call-to-action affordances render as copy-pasteable CLI commands. The webui-write-surface direction is a separate stronghold-level investigation; this phase ships the CLI-handoff fallback that the read-only `/api/` surface mandates.

## Acceptance Criteria

- `FindingSeverity::Info` is defined in `src/map/graph.rs` and exhaustively handled at every existing `match` site.
- Producers for orphaned-file states and unverified-contract states emit `Info` findings; previously silent advisory states are now visible in the finding stream.
- `docs/design-system/copy.toml` exists with two top-level sections (`[empty-states]`, `[findings]`); the file is parsed by the webui at compile time via `include_str!` and consumed by `src/ui_assets/app.js`.
- The empty-states section contains entries for the ten existing inline empty-state sites in `app.js` (no paths, no contracts, no decisions, no todos, no research, no sources, no outbound dependencies, no inbound dependents, clean map, no matches), each with `heading`, `body`, and `cta` fields.
- The findings section contains entries for every finding code currently allocated in `openspec/registries/error-codes.md` (`CE001` through `CE010`, `CT001`, `CT002`), each with `heading`, `body`, and `cta` fields.
- `docs/design-system/README.md` voice section names the em-dash ban, the plain-English bar, and the load-bearing taxonomy from `CLAUDE.md`. A voice review checklist appears as bullets immediately after the prose.
- `cairn check` runs as a new CLI subcommand. Without arguments, it reports findings for the whole map. With a node argument, it reports findings filtered to that node.
- `cairn check` exits with code zero regardless of finding severity (inspection semantics, not gate semantics).
- The empty-state component class lives in `docs/design-system/components.css` and consumes only existing tokens (no new hex values, no new size values).
- `src/ui_assets/app.js` reads empty-state copy from the centralised file by surface-state key; the previously inline strings are gone.
- `src/cli/mod.rs` no-args path renders an empty-state CTA naming the next move; `src/cli/format.rs` clean-map output renders a parallel empty-state CTA naming the next move.
- The Findings rollup panel renders in the webui with three severity buckets, a scope toggle, and a category filter on finding-code prefix; the panel reads exclusively from `/api/lint`.
- The prose-nudge banner appears at the top of the node-detail panel when the node has at least one finding; it reads `[findings.<code>]` from the copy file and renders a copy-pasteable CLI snippet.
- All strict Rust gates pass: `cargo build` (zero warnings), `RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features`, `cargo fmt --check`, `cargo test`, `cargo test --locked`, `cflx openspec validate phase-7.7-ux-foundation --strict`.

## Out of Scope

- Adding write paths on the webui `/api/` surface. The webui stays read-only; all calls-to-action render as copy-pasteable CLI commands (the CLI-handoff fallback).
- The deferred sub-components gated on a webui write surface: in-UI Fix buttons, in-UI per-row Fix actions, in-UI CTA actions that mutate state. These remain deferred until the webui-write-surface direction resolves at the stronghold level.
- The `--json` output mode of `cairn check`. The existing `cairn lint --json` already covers any CI consumer today; adding `--json` to `cairn check` is duplication without consumer demand.
- Custom empty-state illustrations beyond reusing the existing artefact glyphs in `docs/design-system/components.css`. Custom illustrations are a later marketing-and-onboarding push.
- Splitting the `Info` severity into multiple sub-codes for distinct advisory-state causes. One `Info` variant ships now; sub-codes wait on operational evidence.
- A re-run button or timestamp on the Findings panel. The deterministic reconciler does not need a manual re-run UI; file-watch refresh stays the cheap default.
- Empty-state treatment for error-path CLI messages (such as `cairn neighbourhood <unknown-node>`). The current closest-match-suggestion behaviour is already a reasonable empty-state-equivalent for unknown-node cases. Bundle B touches only zero-data empty states.
- A separate `voice.md` file. Voice rules consolidate into `docs/design-system/README.md` to avoid a third location for voice authority alongside `CLAUDE.md` and the design-system README.
- Renaming or deprecating `cairn lint`. Both commands ship; `cairn lint` retains gate semantics for hooks and pre-commit, `cairn check` carries inspection semantics for users and the panel.
