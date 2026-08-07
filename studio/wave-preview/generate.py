#!/usr/bin/env python3
"""Rung 3 wave preview: composes today's dispatch wave from committed state.

Derivation follows dec.rung-three-coordination-substrate (accepted 2026-08-07):
containment closure of the todo's anchor (dotted-id descent), Node.paths
prefixes with more-specific outside owners subtracted, hotspot prefixes in no
unit's write-set (phase 0), component-boundary disjointness, one hotspot
permission holder per wave. Read-only: this script starts nothing and writes
only index.html beside itself. Python3 stdlib only (Herdr-dashboard precedent).

Usage: python3 studio/wave-preview/generate.py && open studio/wave-preview/index.html
"""
import collections
import datetime
import glob
import html
import json
import os
import re
import subprocess

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
os.chdir(ROOT)
M = json.load(open("map.json"))
NODES = {n["id"]: n for n in M["nodes"]}
HOTSPOTS = ["docs/registries", "cairn.blueprint", "docs/design-system/copy.toml"]
HEAD = subprocess.run(["git", "rev-parse", "--short", "HEAD"],
                      capture_output=True, text=True).stdout.strip()


def norm(p):
    return re.sub(r"^\./", "", p).rstrip("/")


def overlap(a, b):
    a, b = norm(a), norm(b)
    return a == b or b.startswith(a + "/") or a.startswith(b + "/")


def write_set(anchor):
    if anchor not in NODES:
        return {"inc": ["."], "exc": [], "resolution": "unresolved",
                "reason": "anchor " + anchor + " is not a graph node"}
    cl = {i for i in NODES if i == anchor or i.startswith(anchor + ".")}
    inc = sorted({norm(p) for i in cl for p in NODES[i]["paths"]})
    if not inc:
        return {"inc": ["."], "exc": [], "resolution": "unresolved",
                "reason": anchor + " closure declares no paths"}
    exc = sorted({norm(p) for i, n in NODES.items() if i not in cl
                  for p in n["paths"] if any(norm(p).startswith(q + "/") for q in inc)})
    return {"inc": inc, "exc": exc + HOTSPOTS, "resolution": "derived"}


def ws_overlap(A, B):
    hits = []
    for a in A["inc"]:
        for b in B["inc"]:
            if overlap(a, b):
                deeper = b if len(norm(b)) >= len(norm(a)) else a
                if any(overlap(deeper, e) and len(norm(e)) >= len(norm(deeper))
                       for e in A["exc"] + B["exc"]):
                    continue
                hits.append((norm(a), norm(b)))
    return hits


TODOS = {}
for f in glob.glob("meta/todos/*.md"):
    t = open(f).read().split("---")[1]

    def grab(key, text=t):
        m = re.search(r"^" + key + r":\s*(.+)$", text, re.M)
        return m.group(1).strip() if m else None

    bb = re.search(r"^blocked_by:\s*\[([^\]]*)\]", t, re.M)
    stem = "todo." + os.path.basename(f)[5:-3]
    TODOS[stem] = {
        "status": grab("status") or "?",
        "node": grab("node") or "?",
        "blocked_by": [x.strip() for x in bb.group(1).split(",") if x.strip()] if bb else [],
    }

OPEN = {k: v for k, v in TODOS.items() if v["status"] == "open"}


def open_blockers(u):
    return [b for b in OPEN[u]["blocked_by"] if TODOS.get(b, {}).get("status") == "open"]


READY = sorted(u for u in OPEN if not open_blockers(u))


def rank(u, seen=()):
    bs = [b for b in OPEN.get(u, {}).get("blocked_by", []) if b in OPEN and b not in seen]
    return 0 if not bs else 1 + max(rank(b, seen + (u,)) for b in bs)


WS = {u: write_set(OPEN[u]["node"]) for u in READY}
wave, held, hot_holder = [], [], None
for u in sorted(READY, key=lambda x: (rank(x), x)):
    w = WS[u]
    if w["resolution"] == "unresolved":
        if not wave:
            wave.append(u)
        else:
            held.append((u, None, w["reason"]))
        continue
    clash = None
    for v in wave:
        if WS[v]["resolution"] != "derived":
            continue
        hits = ws_overlap(WS[v], w)
        if hits:
            clash = (v, hits[0])
            break
    if clash:
        held.append((u, clash[0], clash[1]))
    else:
        wave.append(u)
        if hot_holder is None:
            hot_holder = u


def esc(s):
    return html.escape(str(s))


def mono(s):
    return '<span class="mono">' + esc(s) + "</span>"


rows = []
for u in wave:
    w = WS[u]
    note = ""
    if w["resolution"] != "derived":
        note = ('<div class="rule">runs alone: ' + esc(w["reason"])
                + ", so cairn treats it as touching every file</div>")
    hot = " &middot; holds the hotspot permission" if u == hot_holder else ""
    rows.append(
        '<div class="unit"><div class="top">' + mono(u)
        + '<span class="meta">anchor ' + mono(OPEN[u]["node"]) + hot + "</span></div>"
        + '<div class="rule">rule: ready &middot; write-sets disjoint &middot; '
        + "completeness: partial (hotspots uncovered)</div>"
        + '<div class="ws">writes ' + esc(", ".join(w["inc"])) + "</div>" + note + "</div>")

by = collections.defaultdict(list)
for u, v, hit in held:
    by[v].append((u, hit))
held_html = []
for v, us in sorted(by.items(), key=lambda kv: (kv[0] is None, -len(kv[1]))):
    if v is None:
        for u, reason in us:
            held_html.append("<p>" + mono(u) + " runs alone in a later wave: "
                             + esc(reason) + ".</p>")
        continue
    shared = ", ".join(WS[v]["inc"])
    names = ", ".join(mono(u) for u, _ in us[:4])
    if len(us) > 4:
        names += " and " + str(len(us) - 4) + " more"
    verb = "waits" if len(us) == 1 else "wait"
    queue = "It queues" if len(us) == 1 else "They queue"
    held_html.append(
        "<p>" + names + " " + verb + " for this wave: same files as " + mono(v)
        + " (" + esc(shared) + "), one at a time. " + queue
        + " behind that unit and join the next wave.</p>")

root_ready = [u for u in READY if OPEN[u]["node"] == "cairn.root"]
now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
page = ("<!doctype html><html lang=\"en-GB\"><head><meta charset=\"utf-8\">"
 + "<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">"
 + "<title>Wave preview: " + esc(HEAD) + "</title>"
 + "<link rel=\"stylesheet\" href=\"../../docs/design-system/fonts.css\">"
 + "<link rel=\"stylesheet\" href=\"../../docs/design-system/tokens.css\">"
 + "<style>"
 + "body{max-width:72ch;margin:var(--ci-s8) auto;padding:0 var(--ci-s5);"
 + "font-family:var(--font-serif);background:var(--stone-3);color:var(--ink-1);"
 + "font-size:var(--t-body);line-height:1.55}"
 + ".mono{font-family:var(--font-mono);font-size:.92em}"
 + ".bezel{font-family:var(--font-mono);font-size:var(--t-micro);"
 + "letter-spacing:.08em;text-transform:uppercase;color:var(--ink-2);"
 + "border-bottom:var(--line-1) solid var(--seam-faint);"
 + "padding-bottom:var(--ci-s2);margin-bottom:var(--ci-s6)}"
 + ".unit{border:var(--line-1) solid var(--seam-thin);"
 + "padding:var(--ci-s3) var(--ci-s4);margin:var(--ci-s2) 0}"
 + ".unit .top{display:flex;justify-content:space-between;gap:var(--ci-s4);"
 + "flex-wrap:wrap}"
 + ".unit .meta{font-family:var(--font-ui);font-size:var(--t-small);"
 + "color:var(--ink-2)}"
 + ".unit .rule,.unit .ws{font-family:var(--font-mono);font-size:var(--t-small);"
 + "color:var(--ink-2);margin-top:var(--ci-s1)}"
 + "h1{font-size:var(--t-h3)} h2{font-size:var(--t-lede);margin-top:var(--ci-s7)}"
 + ".callout{border-left:var(--ci-keel) solid var(--ink-2);"
 + "padding:var(--ci-s1) 0 var(--ci-s1) var(--ci-s4);margin:var(--ci-s5) 0}"
 + ".foot{margin-top:var(--ci-s9);font-size:var(--t-small);color:var(--ink-2)}"
 + "</style></head><body>"
 + "<div class=\"bezel\">derived preview &middot; committed state at " + esc(HEAD)
 + " &middot; generated " + esc(now)
 + " &middot; no driver attached &middot; nothing here is a button</div>"
 + "<h1>Next wave &middot; " + str(len(wave)) + " units</h1>"
 + "<p class=\"mono\" style=\"color:var(--ink-2)\">write-sets disjoint &middot; parallel worktrees"
 + " &middot; hotspot permission: one unit per wave</p>"
 + "".join(rows)
 + "<p>Only one unit at a time may change " + mono("docs/registries/") + ", "
 + mono("cairn.blueprint") + ", or " + mono("docs/design-system/copy.toml") + ", and "
 + mono(hot_holder or "nobody")
 + " holds that permission this wave. Every other unit is constrained not to touch them.</p>"
 + "<h2>Held &middot; " + str(len(held)) + " units</h2>"
 + "".join(held_html)
 + "<div class=\"callout\">" + str(len(root_ready)) + " of " + str(len(READY))
 + " ready units anchor to " + mono("cairn.root")
 + ", a module owning seven entry-point files, so at most one enters any wave. "
 + "Most touch none of those files: the anchor is a catch-all, not a fact. "
 + mono("todo.root-anchor-hygiene")
 + " re-anchors them; every re-anchored unit is a candidate lane in this preview.</div>"
 + "<p class=\"foot\">Derivation per " + mono("dec.rung-three-coordination-substrate")
 + " (accepted 2026-08-07, panel receipts). This page renders a projection; only a driver"
 + " dispatches, and none exists yet. Sentences follow clause 5: a unit queues behind a"
 + " <em>unit</em>, never a claim, because no lease facts exist. Regenerate: "
 + mono("python3 studio/wave-preview/generate.py") + "</p>"
 + "</body></html>")
out = os.path.join(ROOT, "studio/wave-preview/index.html")
open(out, "w").write(page)
print("wave=" + str(len(wave)) + " held=" + str(len(held))
      + " hotspot_holder=" + str(hot_holder) + " -> " + out)
