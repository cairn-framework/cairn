---
id: res.herdr-plugin-feasibility
nodes:
  - cairn.root
date: 2026-07-17
method: primary
---

# Herdr plugin feasibility: cheap probes over a cairn dashboard pane

Live probes against a real herdr session (`HERDR_ENV=1`, workspace `wM`)
answer the investigation questions in `todo.herdr-plugin-feasibility`. Every
factual claim below is either a command run in this session with its literal
output, or a source line read from this worktree's `src/mcp/mod.rs` (cairn
0.5.0, built at `/tmp/cairn-wt-herdr-plugin-feasibility`). Nothing is
implemented here; the only forward-looking content is the closing recommended
increment path and the plumbing-point MCP feasibility sketches in Q4 (where a
hook would attach, not a buildable design).

All probe panes (`wM:pA` status+lint poll, `wM:pB` watch delta, `wM:pC`
report-metadata fidelity) were created via `herdr pane split --current
--direction down --no-focus` and closed by this session; no other pane, in
particular the orchestrator's dashboard pane `wM:p7`, was touched.

## Q2: what does herdr actually expose today?

`herdr --help` and each subcommand's bare invocation enumerate the real
surface. The relevant groups, verified live:

- `herdr pane report-agent <pane_id> --source ID --agent LABEL --state idle|working|blocked|unknown [--message TEXT]`
- `herdr pane report-metadata <pane_id> --source ID [--title TEXT] [--state-label STATUS=TEXT] [--token NAME=VALUE] [--ttl-ms N]`
- `herdr pane release-agent`, `herdr pane run <pane_id> <command>`, `herdr pane get/list/read/split/close`
- `herdr integration install|uninstall|status <harness>` for pi, omp, claude, codex, copilot, devin, droid, kimi, opencode, kilo, hermes, qodercli, cursor, mastracode
- `herdr agent list|get|read|send|wait|attach|start|explain`, `herdr wait output|agent-status`, `herdr api snapshot|schema`
- `herdr notification show <title> [--body TEXT] [--position top-left|top-right|bottom-left|bottom-right] [--sound none|done|request]` (verified live via a standalone `herdr notification` invocation). This is a desktop-alert surface, not a pane-state extension: it cannot render cairn state in a pane, so it offers no dashboard-render surface, but a dashboard process could call it to alert on a severity change (for example fire `notification show` when `cairn watch` emits a new error-severity finding).

There is no "herdr plugin" artefact type or manifest format; the entire
extension surface is these socket-API CLI verbs plus the terminal itself. A
"herdr plugin" is therefore always one of: (a) a plain process running in a
pane, optionally narrating itself via `report-metadata`/`report-agent`, or
(b) a harness-side hook script installed by `herdr integration install
<harness>` that calls these same verbs on lifecycle events. There is no
third mechanism (no webhook registration, no persistent background plugin
process herdr loads itself). This directly resolves the "merely a pane
running a process" half of Q2: yes, that is the entire model, `report-agent`
and `report-metadata` are the only way to attach structured state to a pane
beyond raw text.

`herdr integration status` shows the OMP integration already installed at
`~/.omp/agent/extensions/herdr-omp-agent-state.ts` (v5, current). Reading it
confirms the mechanism for Q4's "harness hooks" option: it is an OMP
extension (`export default function (pi) { ... }`) that listens for OMP
session lifecycle/message events and calls `herdr pane report-agent`
(`working`/`blocked`/`idle`) and `report-metadata` via a queued, retrying
socket connection to `HERDR_SOCKET_PATH`. This is a real, working precedent
for harness-side event push, but it fires on agent turn/session lifecycle,
not on individual cairn tool calls; cairn is invisible to it.

## Q1 and Q3: standalone versus harness-reliant, and state-file fidelity

One pane-process polling probe was run, in a pane this session created via
`herdr pane split --current --direction down --no-focus` and closed
afterwards.

**Combined status + lint poll (pane `wM:pA`).** Ran a bounded script
(`/tmp/cairn-probe-poll.sh`, four cycles at `sleep 3`) emitting both the
severity breakdown from `cairn lint --json` and the navigational summary from
`cairn status --json`. Captured via `herdr pane read wM:pA --source recent
--lines 20`:

```
== 08:11:29 cycle=1 ==
findings {'info': 9}
next cairn todos cairn.brownfield | open_todos 36 | active_changes 0
== 08:11:32 cycle=2 ==
findings {'info': 9}
next cairn todos cairn.brownfield | open_todos 36 | active_changes 0
== 08:11:35 cycle=3 ==
findings {'info': 9}
next cairn todos cairn.brownfield | open_todos 36 | active_changes 0
== 08:11:38 cycle=4 ==
findings {'info': 9}
next cairn todos cairn.brownfield | open_todos 36 | active_changes 0
```

`cairn status --json` exposes `next_recommended` (a `command`/`node`/`title`
triple), `open_todos` (array of `{node, path, status, created}`), and
`active_changes` (array). The `info: 9` count includes an advisory
`CAIRN_RESEARCH_ORPHAN` on this very artefact (research not yet cited by a
decision; legitimate per spec section 8.5 and advisory-only, so it does not
fail the gate); moving the artefact aside drops the count to `info: 8`. The
poller is internally stable across all four cycles. Isolated via
`/usr/bin/time -l ./target/release/cairn lint --json`: 0.12s real / 0.17s user
/ 0.06s sys, 15.8MB peak RSS. That is roughly 0.23s of CPU per run, so a 2-3s
poll interval spends about 8-12% of one core on lint alone, before adding the
`cairn status --json` call the proposed poller also makes (unbenchmarked
here). Continuous polling is affordable but not free; the hybrid below exists
partly to avoid paying it every tick. This answers Q1's "what can a
plugin do on its own": full read-only visibility into current findings AND
backlog/next-step navigation via `cairn lint --json` / `cairn status --json`,
with zero harness cooperation.

For Q3 (state-file fidelity: pane state matching files rather than an agent's
claims), three options were probed:

- **fswatch**: `which fswatch` exits 1, no binary printed. fswatch is NOT
  installed on this workstation, so any fswatch-on-map.json re-render path is
  unavailable without a new dependency install, outside a cheap probe's scope.
- **`cairn watch` (real filesystem-triggered delta, not just a startup
  dump).** `./target/release/cairn --help` lists `watch: Watch for finding
  changes and emit events`. A first `cairn watch --once` confirmed the event
  shape (newline-delimited JSON, one `finding_added` per current finding as
  the baseline seeds). Then a continuous run was exercised against a
  controlled, fully-reverted change. Pane `wM:pB` ran `cairn watch --interval
  3`; after it seeded 9 baseline `finding_added` events, this session created
  `src/zz_watch_probe_trigger.rs` (an unclaimed Rust file) at 08:16:04 and
  deleted it at 08:16:11. `herdr pane read wM:pB` captured the live,
  filesystem-triggered delta:

  ```
  {"event":"finding_added","timestamp":"2026-07-17T04:16:04Z","finding":{"code":"CAIRN_RECONCILE_ORPHANED_FILE","severity":"info","message":"Rust file `src/zz_watch_probe_trigger.rs` is not owned by any eligible node","node":null,"path":"src/zz_watch_probe_trigger.rs"}}
  {"event":"finding_resolved","timestamp":"2026-07-17T04:16:13Z","finding":{"code":"CAIRN_RECONCILE_ORPHANED_FILE","severity":"info","message":"Rust file `src/zz_watch_probe_trigger.rs` is not owned by any eligible node","node":null,"path":"src/zz_watch_probe_trigger.rs"}}
  ```

  The `finding_added` event carries the same timestamp as the file creation
  (04:16:04Z), and `finding_resolved` fires two seconds after the deletion,
  within the 3s scan interval. This is deterministic change detection sourced
  from cairn's own reconcile (not from an agent's running commentary), so a
  pane rendering `cairn watch` has high source fidelity with bounded latency:
  it reflects the reconciled state as of its last internal scan, not
  instantaneously, and it emits deltas so no separate diffing logic is needed.
  It is an internal poll loop (default `--interval 5`), not event-driven, and
  needs no fswatch.
  The temp file was removed and `map.json` (which the
  reconcile regenerated) was restored via `git checkout`; `git status` showed
  only this research artefact afterwards.
- **map.json / `.cairn/state` re-render path (the literal Q3 phrasing).** The
  repo root carries a generated, gitignored `map.json` (the reconciled map)
  plus a `.cairn/state/` directory holding `reconciler-cache.json`,
  `blueprint-snapshot.json`, `interface-hashes.json`, and `head-blueprint.cache`.
  A "fswatch on map.json triggering re-render" path is therefore
  blocked twice over on this workstation: fswatch is absent, AND no existing
  renderer reads `map.json` in the first place. The baseline orchestrator
  dashboard (`/tmp/cairn-wave/dashboard.py`) renders `/tmp/cairn-wave/state.json`,
  a hand-maintained file of orchestrator phase/runtime claims, and has no
  `map.json` (or `.cairn/`) input at all. So the map.json re-render option is
  not merely unprobed here, it is not wired anywhere today; among the
  mechanisms probed in this session, `cairn watch` is the only live,
  dependency-free ground-truth event source, and it derives from the same
  reconcile that writes `map.json`, so it is the strict functional substitute
  for a (presently impossible) fswatch-on-map.json trigger. One unprobed
  standalone alternative exists: the shipped `cairn-lsp` binary
  (dec.lsp-diagnostics-server) republishes findings over LSP stdio from its
  own background watch loop; it targets editors, so driving a pane from it
  would need an LSP client shim, which is why it was not probed as the
  dashboard source here.

For metadata fidelity specifically: `herdr pane report-metadata wM:pC
--source cairn-probe --title "Cairn Probe Pane" --token findings=9 --token
severity=info --ttl-ms 60000` and `herdr pane report-agent wM:pC --source
cairn-probe --agent cairn-probe-agent --state working --message "polling
cairn lint"` both returned silently (exit 0). `herdr pane get wM:pC`
immediately reflected every field:

```
{"pane":{"agent":"cairn-probe-agent","agent_status":"working", ...,
"title":"Cairn Probe Pane","tokens":{"findings":"9","severity":"info"}, ...}}
```

The same fields also appeared filtering `herdr pane list --workspace wM` for
`pane_id == "wM:pC"`: identical `title`/`tokens`/`agent`/`agent_status`, the
same data source a workspace-wide sidebar would read. So a plugin that pipes
`cairn watch` (or a poller) into `report-metadata --token` calls gets both
per-pane and workspace-level surfaces for free, and the pane state is
whatever the last `report-metadata`/`report-agent` call said, not an
independent render: fidelity is exactly as good as whoever calls those
verbs, which is why deciding what calls them (cairn's own event stream
versus an agent's assertions) is the crux of the fidelity question, not the
transport.

## Q4: deterministic updates on AI tool use

Two portable mechanisms were found; neither was implemented, only located.

1. **Harness hook (already exists for cairn's own OMP session).** The
   installed OMP integration
   (`~/.omp/agent/extensions/herdr-omp-agent-state.ts`) proves harness-level
   push works today, but it hooks OMP's *session/message* lifecycle, not
   cairn's tool calls specifically; it would need cairn-specific logic added
   inside the OMP extension (fragile, one-harness-only, and not this
   session's to edit) to react to "an agent called cairn".
2. **MCP wrapper.** Read `src/mcp/mod.rs` (368 lines) and
   `src/bin/cairn-mcp.rs` (43 lines) in this worktree. `cairn-mcp` (bin) just
   calls `cairn::mcp::serve_stdio(&config)` (mod.rs:42-46), which locks
   stdin/stdout and runs `serve()` (mod.rs:53-68): a blocking `for line in
   reader.lines()` loop dispatching each line through `handle_line()`
   (mod.rs:72-109), which routes `"tools/call"` to `call_tool()`
   (mod.rs:147-173). `call_tool` extracts `params.name` at line 149 BEFORE
   calling `query_api::execute(...)` at line 162, and gets a
   success/error `Result` immediately after. `serve`, `handle_line`,
   `call_tool`'s call site, `config_from_args`, and `ServerConfig` are all
   `pub` (mod.rs is a library module, not main-only), so both are viable
   with zero cairn source changes:
   - an **external proxy** binary that intercepts stdin/stdout between the
     MCP client and a real `cairn-mcp` child process, greps `tools/call` +
     `name` the same way `call_tool` does, forwards the line unmodified,
     relays the reply, and fires `herdr pane report-metadata --token
     tool=<name>` as a non-blocking side effect; or
   - an **in-process wrapper** binary that reimplements the ~15-line `serve`
     loop directly against the public `cairn::mcp` functions and adds the
     `herdr` calls around the per-line dispatch, still non-blocking.

   Cost: a `herdr pane` CLI shell-out measured 70-110ms wall time in the
   `report-metadata`/`report-agent` probe above; a *blocking* shell-out per
   MCP tool call would add material latency to every round-trip, so any
   implementation MUST keep the MCP response path non-blocking. Note that
   `spawn()` without `.wait()` leaks zombies in a long-running proxy
   (`std::process::Child` does not reap on drop), so hand each spawned
   child to a background waiter thread that reaps it, or hold a persistent
   Herdr connection instead. Reporting the call's outcome stays possible: a
   second reaped async spawn after `query_api::execute` returns can report
   outcome, since `call_tool` sees both `name` and the `Result`.

   Portability: this only reports MCP-tool-mediated cairn usage. Any CLI
   invocation of `cairn` directly (shell, scripts, this very session's
   `./target/release/cairn lint`) is invisible to an MCP wrapper; only a
   shell shim (aliasing `cairn` to a script that runs the real binary then
   fires `herdr pane report-metadata`) would additionally catch those, at
   the cost of being installed per-shell-profile rather than per-harness.

Ranking Q4's three options on standalone-vs-harness-reliant / portability:
harness hooks are harness-specific and require editing a herdr-managed
integration file per harness (`~/.omp/...`, `~/.codex/...`, etc, 14 targets
today); an MCP wrapper is portable across ANY MCP client (harness-agnostic)
but blind to direct CLI use; a shell shim is the most portable (catches
every invocation regardless of harness or MCP) but is the most manual to
install and is defeated by `env -i` / login-shell edge cases. None of the
three is required for the baseline dashboard use case in Q5, since
`cairn watch` already gives ground-truth events without needing to know
which agent or tool triggered them.

## Q5: which views earn a pane, ranked against the hand-rolled dashboard

Read `/tmp/cairn-wave/dashboard.py` and `/tmp/cairn-wave/state.json` (the
orchestrator's hand-rolled JSON-state-file + 2s-poll Python renderer used
for the 2026-07-17 wave, explicitly labelled "orchestrator claims, not
cairn ground truth" in its own banner). It tracks per-task `phase` and
per-phase timing from a manually-mutated JSON file with no cairn
involvement at all: it answers "what is the swarm doing", not "what is
cairn's state".

Ranked by value against that baseline:

1. **Findings/lint stream** (`cairn watch` piped into a pane, optionally
   summarised via `report-metadata --token errors=N --token warnings=N`).
   Highest value: this is the one view the wave dashboard cannot provide at
   all (it has zero cairn awareness), it is high source fidelity with bounded
   latency (Q3), and the poll/probe above shows it is cheap enough to run
   continuously. This is the direct answer to the todo's stated problem.
2. **Backlog / `cairn status --json`** (`next_recommended`, ready count).
   Second highest: complements the findings stream with "what to do next",
   same cheap polling mechanism, already probed working (`{"command":"cairn
   todos ..."}` observed live in probe 1).
3. **Map/graph summary.** Lower value for a *live* pane: the graph changes
   far less often per session than findings/backlog, so a poll-driven pane
   would mostly show a static render; better served by an on-demand
   `cairn context`/`cairn get` call than a persistent pane.
4. **Wave/task dashboard (the orchestrator-claims overlay).** Lowest
   cairn-relevance by definition, since it is explicitly not cairn ground
   truth; `todo.herdr-live-dashboard`'s own acceptance criterion says to
   keep it as a "separate, clearly-labelled layer if kept at all", which
   this probe corroborates: it is worth keeping only as an optional add-on
   next to the cairn-grounded views above, never as the primary pane.

## Options compared

The four candidate mechanisms, consolidated across the three axes the
investigation asked for (each cell is grounded in a probe or source read
above):

| Option | Standalone vs harness-reliant | Fidelity to cairn ground truth | Portability |
|---|---|---|---|
| Pane-process polling (`cairn lint/status --json` on a timer) | Fully standalone; no harness or agent involvement (probed in `wM:pA`) | High source fidelity, bounded latency: reads reconciled state each tick, so the pane matches within one poll interval, not instantly | Universal: any shell/pane, any harness or none; zero dependencies |
| File-watch (fswatch, or `cairn watch`) | Standalone; no harness. fswatch is NOT installed here; `cairn watch` ships with cairn and was exercised live (`wM:pB`) | High source fidelity, bounded latency: `cairn watch` emits reconcile-derived `finding_added`/`finding_resolved` deltas (verified against a controlled create/delete), detected on its next internal scan; fswatch would be file-level and would still need a renderer | `cairn watch` universal (no extra deps); fswatch needs a separate install (not default on macOS) |
| MCP wrapper (proxy/in-process around `cairn::mcp::handle_line`) | Harness-agnostic but limited to MCP-mediated traffic; blind to direct CLI cairn use | Medium: attributes which tool was called and its outcome (name at `mod.rs:149`, result at `mod.rs:162`), but reports tool calls, not cairn state, and sees nothing invoked outside MCP | Any MCP client, but requires a wrapper binary and adds a 70-110ms shell-out unless fire-and-forget; does not catch CLI/script cairn use |
| Harness hooks (`herdr integration install <harness>`) | Harness-reliant: one managed extension file per harness; already installed for OMP | Low for cairn: the installed OMP hook fires on agent session/message lifecycle (working/blocked/idle), not on cairn state or tool calls; cairn is invisible to it today | Broad coverage across 14 harness targets, but most fragmented to maintain; each harness needs its own hook authored |

The table makes the mandate's comparison unambiguous: the two standalone
file-grounded options (polling, `cairn watch`) are the only ones with high
fidelity AND universal portability; the MCP wrapper trades fidelity (tool-call
attribution, not state) for a narrower but harness-agnostic surface; harness
hooks are the most harness-coupled and the least cairn-aware today.

## Recommended increment path

Recommendation: a hybrid standalone pane process that consumes `cairn watch`
for finding deltas and polls `cairn status --json` on a timer for backlog and
next-step state. The renderer calls `herdr pane report-metadata --token
errors=N --token warnings=N --token info=N --token todos=N` so both findings
and backlog counts surface at workspace level.

Why hybrid, not watch-only: `todo.herdr-live-dashboard`'s acceptance requires
findings AND backlog counts to always match `cairn lint --json` and `cairn
status --json` at read time. `cairn watch` emits finding events only (verified: the
`wM:pB` probe produced `finding_added`/`finding_resolved` and never a backlog
event); it carries no `next_recommended`/`open_todos` signal. A watch-only
pane would satisfy the findings half of the acceptance but silently miss every
backlog change.

Options weighed:
- Watch-only (cheapest): pro = emits finding deltas (the renderer updates only
  on change) and is ground truth; it still polls internally every `--interval`
  (default 5s), so it is not timerless. Con = emits finding events only, cannot
  keep backlog matched, so it fails half the acceptance.
- Poll-only (`status` + `lint` on a timer, as probed in `wM:pA`): pro = covers
  both findings and backlog from one mechanism. Con = re-derives the full
  finding set each tick (roughly 120ms) instead of reacting to deltas, and lags
  by up to one poll interval.
- Hybrid (recommended): `cairn watch` as the change trigger (it polls
  internally every `--interval` and emits only the delta) plus a periodic
  `cairn status --json` check for backlog movement. On every triggered
  re-render the pane re-derives BOTH `cairn lint --json` and `cairn status
  --json` and displays the collection time, so a count shown at read time was
  just derived from the commands themselves, satisfying the at-read-time
  acceptance; the watch/poll machinery only decides when to re-render, never
  what to display. Both halves are standalone; zero new dependencies, zero
  cairn source changes, no harness-specific work.

Short-sightedness flag: recommending watch-only would defer the backlog half
of the acceptance to a later unit, accruing debt against the todo's stated
invariant (backlog counts always match `cairn status --json` at read time).
The hybrid closes both halves now for the same standalone, dependency-free
cost.

Acceptance operationalisation: counts must always match the commands at read
time. The pane meets this by never caching displayed counts: each rendered
snapshot re-runs `cairn lint --json` and `cairn status --json` (about 0.23s
CPU per lint run, acceptable per render) and shows the collection time
alongside. Watch events and the status poll are triggers that decide when a
new snapshot is rendered; the displayed numbers always come from the commands
run for that snapshot.

MCP-wrapper and harness-hook event push (Q4) are real and plumbing-point
verified, but neither is needed to close the dashboard gap; they surface
recent cairn tool activity (not causal trigger attribution), a separate,
lower-priority follow-up (todo.herdr-cairn-tool-attribution), not a blocker.

## Gate note

`cairn scan --strict` exits 0 with this artefact present. The artefact
introduces one `CAIRN_RESEARCH_ORPHAN` finding at **Info** severity (this
research is not yet cited by any decision); `--strict` fails only on
Warning-or-above, and `nodes: [cairn.root]` passes node validation, so no
Error fires. The Info orphan is expected for new research and does not fail
the gate; it clears automatically once a future decision cites this research.
