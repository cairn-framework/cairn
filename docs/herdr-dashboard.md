# Herdr live dashboard

`scripts/herdr-cairn-dashboard.py` is a dependency-free pane process (python3
standard library only) that renders cairn ground truth and keeps it matched to
`cairn lint --json` and `cairn status --json` at read time. It is the durable
replacement for the session-local wave dashboard: cairn's own state is the
primary layer, an orchestrator's task/phase claims can be shown as a separate,
clearly labelled overlay, and it survives across sessions because it is a
checked-in script.

## What it renders

Every snapshot re-derives both commands from scratch (never a cache) and shows
the collection time, a monotonic snapshot counter, and the counts:

- Findings by severity (`error`, `warning`, `info`) from `cairn lint --json`.
- Backlog (`open_todos`, `active_changes`) and the next recommended step from
  `cairn status --json`.

### Orchestrator overlay (optional)

Set `CAIRN_DASH_OVERLAY=/path/to/state.json` to render a session
orchestrator's task/phase log beneath the cairn counts, under an explicit
"ORCHESTRATOR CLAIMS (overlay, unverified)" label. The file uses the wave
dashboard shape: `{"tasks": [{"name", "phase", "agent"?, "note"?, "history":
[[phase, start_epoch], ...], "end_ts"?}]}`. Each task renders its current
phase, total and in-phase runtimes, and a per-phase runtime breakdown. The
overlay re-renders when the file's mtime changes. It is claims, not ground
truth: it never enters the sidecar or herdr metadata, so the verification
protocol below is unaffected.

## When it re-renders

Three trigger classes decide when to draw a new frame. None of them is the data
shown; `do_render()` always fetches fresh lint and status for the snapshot it
draws.

- **`cairn watch --interval N`** emits a finding delta (ground-truth change to
  findings). Each delta forces a fresh full snapshot.
- **Status-poll delta.** Every `CAIRN_DASH_POLL` seconds (default 5) the pane
  fetches `cairn status --json`, hashes the backlog fields, and only when that
  hash differs from the prior poll does it force a fresh full snapshot. `cairn
  watch` emits finding events only, so this is the only trigger that notices
  backlog movement. The polled status is a change detector, never the displayed
  data.
- **On-demand refresh handshake.** `touch .cairn/dashboard-refresh` forces an
  immediate re-render. This is the mechanism that guarantees read-time matching
  (see the verification protocol).

Each snapshot is also written to `.cairn/dashboard-snapshot.json` (gitignored
under the `.cairn/` rule). If a `lint` or `status` command fails, the pane
shows the error and leaves the last valid sidecar and herdr metadata intact
rather than publishing false zeros; the snapshot counter only advances on a
successful collection.

## Launching in a herdr pane

The pane needs a `cairn` binary and the repo as its working directory. Set
`CAIRN_BIN` so the script uses a known-good binary (the dashboard re-derives
counts from it, so it must be the build you trust).

```
herdr pane split --current --direction down --no-focus \
  --cwd /path/to/cairn-repo \
  --env CAIRN_BIN=/path/to/cairn-repo/target/release/cairn
herdr pane run <new-pane-id> python3 scripts/herdr-cairn-dashboard.py
```

When `HERDR_ENV=1` and `HERDR_PANE_ID` are set (any pane herdr spawns), the
pane also pushes its counts to the workspace sidebar after each render:

```
herdr pane report-metadata $HERDR_PANE_ID --source cairn-dashboard \
  --token errors=N --token warnings=N --token info=N --token todos=N
```

Outside herdr those calls are skipped silently, so the script also runs as a
plain terminal process.

## Verification protocol

Because the terminal holds the last frame between triggers, verify counts at
read time by forcing a fresh frame and comparing the sidecar against fresh
commands:

1. Force a re-render: `touch .cairn/dashboard-refresh`, then stat its mtime.
2. Wait for the acknowledging snapshot. Read
   `.cairn/dashboard-snapshot.json` until its `refresh_marker` is at or past
   the mtime you statted (the counter alone is not enough: a watch, poll, or
   overlay trigger already in flight can advance `snapshot` with counts
   collected before your touch; `refresh_marker` records the refresh mtime
   the snapshot was actually rendered against).
3. Compare the sidecar against freshly derived commands. The sidecar always
   carries all three severity keys (zero when absent), so normalise the fresh
   lint output the same way. One assertion yields a pass/fail result for the
   complete rendered snapshot:

```
python3 - <<'PY'
import json, subprocess, collections, os, sys
binary = os.environ.get('CAIRN_BIN', 'cairn')
sidecar = json.load(open('.cairn/dashboard-snapshot.json'))
lint_proc = subprocess.run([binary, 'lint', '--json'], capture_output=True, text=True)
assert lint_proc.returncode in (0, 1), lint_proc.stderr
lint = json.loads(lint_proc.stdout)['findings']
status = json.loads(subprocess.check_output([binary, 'status', '--json']))
sev = collections.Counter(f['severity'] for f in lint)
fresh_findings = {k: sev.get(k, 0) for k in ('error', 'warning', 'info')}
fresh_todos = len(status['open_todos'])
fresh_changes = len(status['active_changes'])
fresh_next = status.get('next_recommended')
ok = (sidecar['findings'] == fresh_findings
      and sidecar['open_todos'] == fresh_todos
      and sidecar['active_changes'] == fresh_changes
      and sidecar['next_recommended'] == fresh_next)
print('sidecar :', sidecar['findings'], sidecar['open_todos'],
      sidecar['active_changes'], sidecar['next_recommended'])
print('fresh   :', fresh_findings, fresh_todos, fresh_changes, fresh_next)
print('RESULT  :', 'PASS' if ok else 'FAIL')
sys.exit(0 if ok else 1)
PY
```

Run the protocol at rest, then again after a state change (for example create
then revert an unclaimed source file) to confirm the finding delta path updates
the sidecar. RESULT must be PASS on both runs, including active changes and the
next recommended step.

## Environment overrides

| Variable | Default | Purpose |
|---|---|---|
| `CAIRN_BIN` | first `cairn` on `PATH` | The cairn binary used to derive every snapshot |
| `CAIRN_DASH_OVERLAY` | unset | Optional orchestrator-claims JSON rendered as a labelled overlay |
| `CAIRN_DASH_POLL` | `5` | Seconds between status-poll delta checks |
| `CAIRN_DASH_WATCH_INTERVAL` | `3` | `--interval` passed to `cairn watch` |
