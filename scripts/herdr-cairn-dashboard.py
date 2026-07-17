#!/usr/bin/env python3
"""Cairn ground-truth dashboard for a herdr pane.

Renders cairn state deterministically from the cairn CLI itself, never from a
cache. Every rendered snapshot re-runs `cairn lint --json` and `cairn
status --json`, shows the collection time, and writes the derived counts to a
sidecar JSON file so the rendered numbers can be verified against fresh
command output.

Triggers that decide WHEN to re-render (never WHAT to display):
  - `cairn watch --interval N` emits a finding delta (ground-truth change to
    findings). Each delta forces a fresh full snapshot.
  - A status-poll DELTA detector: every CADENCE seconds the pane fetches
    `cairn status --json`, hashes the backlog-relevant fields, and only when
    that hash differs from the prior poll does it force a fresh full snapshot.
    `cairn watch` emits finding events only, so this is the only trigger that
    notices backlog movement. The polled status is a change detector only; it
    is never the data that gets displayed.
  - An on-demand refresh handshake: touching `.cairn/dashboard-refresh` forces
    an immediate re-render. This is the mechanism that guarantees counts match
    at read time: a verifier touches the file, waits for a sidecar whose
    `refresh_marker` reaches that touch's mtime (the snapshot counter alone
    can be advanced by an in-flight unrelated trigger), then compares the
    sidecar against fresh `cairn lint/status --json`.

Whatever fires, `do_render()` always re-derives BOTH `cairn lint --json` and
`cairn status --json` from scratch; no count is ever shown that was not just
derived for that snapshot.

Dependency-free (python3 standard library only). Works outside herdr too: the
`herdr pane report-metadata` call is skipped silently unless HERDR_ENV=1.

Usage:
    CAIRN_BIN=/path/to/cairn python3 scripts/herdr-cairn-dashboard.py
"""

import json
import os
import select
import shutil
import subprocess
import sys
import time

CAIRN_BIN = os.environ.get("CAIRN_BIN") or shutil.which("cairn") or "cairn"
SIDECAR = os.path.join(".cairn", "dashboard-snapshot.json")
REFRESH_FILE = os.path.join(".cairn", "dashboard-refresh")
OVERLAY_FILE = os.environ.get("CAIRN_DASH_OVERLAY", "")
STATUS_POLL_INTERVAL = float(os.environ.get("CAIRN_DASH_POLL", "5"))
WATCH_INTERVAL = os.environ.get("CAIRN_DASH_WATCH_INTERVAL", "3")
WATCH_RESTART_INITIAL = 1.0
WATCH_RESTART_MAX = 10.0
WATCH_STABILITY_WINDOW = 5.0
CAIRN_TIMEOUT = 30
METADATA_TIMEOUT = 10

snapshot_counter = 0






def run_cairn_json(args, required_key):
    """Run a cairn command and return its parsed JSON payload.

    Exit 1 is accepted only when the payload carries `required_key` (lint
    exits 1 on error-severity findings while still emitting the full result);
    an `{"error": ...}` envelope or a payload missing the command's key is a
    collection failure, never counted as zeros.
    """
    try:
        proc = subprocess.run(
            [CAIRN_BIN] + args,
            capture_output=True,
            text=True,
            timeout=CAIRN_TIMEOUT,
        )
    except Exception as exc:
        return {"_error": str(exc)}
    if proc.returncode not in (0, 1):
        return {"_error": "exit %d: %s" % (proc.returncode, proc.stderr.strip())}
    try:
        payload = json.loads(proc.stdout)
    except Exception as exc:
        if proc.returncode != 0:
            return {"_error": "exit %d: %s" % (proc.returncode, proc.stderr.strip())}
        return {"_error": "json: %s" % exc}
    if not isinstance(payload, dict) or required_key not in payload:
        detail = payload.get("error") if isinstance(payload, dict) else payload
        return {"_error": "exit %d: %s" % (proc.returncode, detail)}
    return payload


def collect():
    """Authoritative snapshot: always fetch BOTH commands fresh.

    Returns (ok, data). ok is False if either command failed; then data holds
    only an "error" message and no counts, so the caller shows the failure
    without publishing false zeros to the sidecar or herdr metadata.
    """
    lint = run_cairn_json(["lint", "--json"], "findings")
    status = run_cairn_json(["status", "--json"], "open_todos")
    if "_error" in lint:
        return False, {"error": "lint: %s" % lint["_error"]}
    if "_error" in status:
        return False, {"error": "status: %s" % status["_error"]}
    sev = {}
    for finding in lint.get("findings", []):
        sev_value = finding.get("severity", "info")
        sev[sev_value] = sev.get(sev_value, 0) + 1
    for bucket in ("error", "warning", "info"):
        sev.setdefault(bucket, 0)
    data = {
        "findings": {bucket: sev[bucket] for bucket in ("error", "warning", "info")},
        "extra_severities": {
            k: v for k, v in sev.items() if k not in ("error", "warning", "info")
        },
        "open_todos": len(status.get("open_todos", [])),
        "active_changes": len(status.get("active_changes", [])),
        "next_recommended": status.get("next_recommended"),
    }
    return True, data


def status_signature(status):
    """Signature of the backlog fields the dashboard renders.

    Only the two collection lengths and next_recommended are displayed, so
    only those participate; volatile fields (recent_log_entries, per-item
    bodies) never force a re-render.
    """
    if "_error" in status:
        return None
    return json.dumps(
        {
            "open_todos": len(status.get("open_todos", [])),
            "active_changes": len(status.get("active_changes", [])),
            "next": status.get("next_recommended") or {},
        },
        sort_keys=True,
    )


def poll_status_delta(last_signature):
    """Fetch status once and report whether the backlog signature changed.

    Returns (new_signature, changed). On fetch error returns the prior
    signature and changed=False so a transient failure does not force churn.
    """
    status = run_cairn_json(["status", "--json"], "open_todos")
    signature = status_signature(status)
    if signature is None:
        return last_signature, False
    return signature, signature != last_signature


def now_iso():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def fmt_duration(seconds):
    seconds = int(seconds)
    if seconds < 60:
        return "%ds" % seconds
    if seconds < 3600:
        return "%dm%02ds" % (seconds // 60, seconds % 60)
    return "%dh%02dm" % (seconds // 3600, (seconds % 3600) // 60)


def load_overlay():
    """Optional orchestrator-claims overlay (CAIRN_DASH_OVERLAY).

    The file is a session orchestrator's own task/phase log, in the wave
    dashboard shape: {"tasks": [{"name", "phase", "agent"?, "note"?,
    "history": [[phase, start_epoch], ...], "end_ts"?}]}. It is CLAIMS, not
    cairn ground truth: it is rendered under an explicit label and never
    written to the sidecar or herdr metadata.
    """
    if not OVERLAY_FILE:
        return None
    try:
        with open(OVERLAY_FILE) as handle:
            return json.load(handle)
    except (OSError, ValueError) as exc:
        return {"_error": str(exc)}


def overlay_lines():
    """Build the overlay section defensively.

    The overlay is unverified claims; a malformed file must degrade to an
    error line, never raise into the render path (which would kill the
    ground-truth pane after the sidecar acknowledged the snapshot).
    """
    overlay = load_overlay()
    if overlay is None:
        return []
    lines = ["", "--- ORCHESTRATOR CLAIMS (overlay, unverified) ---"]
    if isinstance(overlay, dict) and "_error" in overlay:
        lines.append("overlay unreadable: %s" % overlay["_error"])
        return lines
    try:
        now = time.time()
        for task in overlay.get("tasks", []):
            history = task.get("history") or []
            phase = task.get("phase", "?")
            end = task.get("end_ts", now)
            total = fmt_duration(end - history[0][1]) if history else "-"
            in_phase = fmt_duration(end - history[-1][1]) if history else "-"
            agent = task.get("agent") or ""
            lines.append(
                "%s  phase=%s%s"
                % (task.get("name", "?"), phase, "  agent=" + agent if agent else "")
            )
            parts = []
            for index, entry in enumerate(history):
                phase_end = history[index + 1][1] if index + 1 < len(history) else end
                parts.append("%s:%s" % (entry[0], fmt_duration(phase_end - entry[1])))
            lines.append(
                "  total=%s  in-phase=%s%s"
                % (total, in_phase, "  " + " > ".join(parts) if parts else "")
            )
            if task.get("note"):
                lines.append("  note: %s" % task["note"])
    except Exception as exc:
        lines.append("overlay invalid: %s" % exc)
    return lines


def render(snapshot, data, collected_at, overlay):
    out = ["\033[2J\033[H", "=== cairn dashboard ==="]
    out.append("Snapshot #%d  collected %s" % (snapshot, collected_at))
    findings = data["findings"]
    out.append(
        "Findings:  error=%d  warning=%d  info=%d"
        % (findings["error"], findings["warning"], findings["info"])
    )
    if data["extra_severities"]:
        extra = " ".join(
            "%s=%d" % (k, v) for k, v in sorted(data["extra_severities"].items())
        )
        out.append("           other: %s" % extra)
    out.append(
        "Backlog:   open_todos=%d  active_changes=%d"
        % (data["open_todos"], data["active_changes"])
    )
    nr = data["next_recommended"]
    if nr:
        out.append("Next:      %s" % nr.get("command", ""))
        if nr.get("title"):
            out.append("           %s" % nr["title"])
    else:
        out.append("Next:      (none)")
    out.extend(overlay)
    out.append("")
    out.append("refresh: touch .cairn/dashboard-refresh")
    out.append("sidecar:  .cairn/dashboard-snapshot.json")
    sys.stdout.write("\n".join(out) + "\n")
    sys.stdout.flush()


def write_sidecar(snapshot, data, collected_at, refresh_marker):
    os.makedirs(os.path.dirname(SIDECAR), exist_ok=True)
    payload = {
        "snapshot": snapshot,
        "collected_at": collected_at,
        "refresh_marker": refresh_marker,
        "findings": data["findings"],
        "open_todos": data["open_todos"],
        "active_changes": data["active_changes"],
        "next_recommended": data["next_recommended"],
    }
    tmp_path = SIDECAR + ".tmp"
    with open(tmp_path, "w") as handle:
        json.dump(payload, handle, indent=2)
        handle.write("\n")
    os.replace(tmp_path, SIDECAR)


def report_metadata(data):
    if os.environ.get("HERDR_ENV") != "1" or not os.environ.get("HERDR_PANE_ID"):
        return
    pane = os.environ["HERDR_PANE_ID"]
    findings = data["findings"]
    cmd = [
        "herdr",
        "pane",
        "report-metadata",
        pane,
        "--source",
        "cairn-dashboard",
        "--token",
        "errors=%d" % findings["error"],
        "--token",
        "warnings=%d" % findings["warning"],
        "--token",
        "info=%d" % findings["info"],
        "--token",
        "todos=%d" % data["open_todos"],
    ]
    try:
        subprocess.run(cmd, capture_output=True, timeout=METADATA_TIMEOUT)
    except Exception:
        pass


def render_error(message):
    out = ["\033[2J\033[H", "=== cairn dashboard ==="]
    out.append("Snapshot #%d  (last good; collection FAILED)" % snapshot_counter)
    out.append("ERROR:     %s" % message)
    out.append("")
    out.append("Retaining last sidecar and metadata. Next trigger retries.")
    out.append("refresh: touch .cairn/dashboard-refresh")
    sys.stdout.write("\n".join(out) + "\n")
    sys.stdout.flush()


def load_persisted_snapshot_counter():
    """Resume the counter from a prior session's sidecar.

    The sidecar survives across panes; restarting at zero would move the
    counter backwards and stall verifiers waiting for a larger value.
    """
    try:
        with open(SIDECAR) as handle:
            persisted = json.load(handle).get("snapshot")
        return persisted if isinstance(persisted, int) and persisted > 0 else 0
    except (OSError, ValueError):
        return 0


def do_render(refresh_marker):
    """Collect, publish, and draw one snapshot.

    Returns True on a successful collection. `refresh_marker` is the refresh
    file mtime this snapshot was rendered against; the verifier waits for a
    sidecar whose marker is at or past its own touch.
    """
    global snapshot_counter
    overlay = overlay_lines()
    ok, data = collect()
    if not ok:
        render_error(data.get("error"))
        return False
    snapshot_counter += 1
    collected_at = now_iso()
    write_sidecar(snapshot_counter, data, collected_at, refresh_marker)
    render(snapshot_counter, data, collected_at, overlay)
    report_metadata(data)
    return True


def refresh_file_mtime():
    try:
        return os.stat(REFRESH_FILE).st_mtime
    except OSError:
        return 0.0

def overlay_file_mtime():
    if not OVERLAY_FILE:
        return 0.0
    try:
        return os.stat(OVERLAY_FILE).st_mtime
    except OSError:
        return 0.0



def start_watch():
    try:
        return subprocess.Popen(
            [CAIRN_BIN, "watch", "--interval", WATCH_INTERVAL],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
            bufsize=1,
        )
    except Exception:
        return None


def close_watch(watch):
    if watch is None:
        return
    try:
        watch.terminate()
    except OSError:
        pass
    try:
        watch.wait(timeout=3)
    except Exception:
        try:
            watch.kill()
        except OSError:
            pass
        try:
            watch.wait(timeout=3)
        except Exception:
            pass
    if watch.stdout is not None:
        watch.stdout.close()


def main():
    global snapshot_counter
    snapshot_counter = load_persisted_snapshot_counter()
    last_refresh_mtime = refresh_file_mtime()
    last_overlay_mtime = overlay_file_mtime()
    # Baseline the status signature BEFORE the first render so a backlog
    # change racing startup is either rendered or still detected as a delta.
    last_status_signature, _ = poll_status_delta(None)
    last_status_poll = time.monotonic()
    do_render(last_refresh_mtime)
    pending_status_signature = None
    watch = None
    next_watch_restart = 0.0
    watch_backoff = WATCH_RESTART_INITIAL
    watch_started_at = 0.0
    try:
        while True:
            now = time.monotonic()
            if watch is None and now >= next_watch_restart:
                watch = start_watch()
                if watch is None:
                    next_watch_restart = now + watch_backoff
                    watch_backoff = min(watch_backoff * 2, WATCH_RESTART_MAX)
                else:
                    watch_started_at = now
            time_to_poll = max(
                0.0, STATUS_POLL_INTERVAL - (time.monotonic() - last_status_poll)
            )
            timeout = min(0.5, time_to_poll)
            streams = [watch.stdout] if watch is not None else []
            ready, _, _ = select.select(streams, [], [], timeout)
            dirty = False
            if ready and watch is not None:
                closed = False
                while True:
                    line = watch.stdout.readline()
                    if not line:
                        closed = True
                        break
                    dirty = True
                    if not select.select([watch.stdout], [], [], 0.0)[0]:
                        break
                if closed:
                    lifetime = time.monotonic() - watch_started_at
                    close_watch(watch)
                    watch = None
                    if lifetime >= WATCH_STABILITY_WINDOW:
                        watch_backoff = WATCH_RESTART_INITIAL
                    else:
                        watch_backoff = min(watch_backoff * 2, WATCH_RESTART_MAX)
                    next_watch_restart = time.monotonic() + watch_backoff
            current_refresh_mtime = refresh_file_mtime()
            if current_refresh_mtime != last_refresh_mtime:
                last_refresh_mtime = current_refresh_mtime
                dirty = True
            current_overlay_mtime = overlay_file_mtime()
            if current_overlay_mtime != last_overlay_mtime:
                last_overlay_mtime = current_overlay_mtime
                dirty = True
            if time.monotonic() - last_status_poll >= STATUS_POLL_INTERVAL:
                last_status_poll = time.monotonic()
                new_signature, changed = poll_status_delta(last_status_signature)
                if changed:
                    pending_status_signature = new_signature
            if pending_status_signature is not None:
                # An observed backlog delta stays dirty until a successful
                # render publishes it.
                dirty = True
            if dirty:
                if do_render(last_refresh_mtime):
                    # Render succeeded: it re-derived status, so the delta is
                    # published and the new signature can be committed.
                    if pending_status_signature is not None:
                        last_status_signature = pending_status_signature
                        pending_status_signature = None
    finally:
        close_watch(watch)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(0)
