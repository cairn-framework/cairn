---
node: cairn.root
status: blocked
created: 2026-07-17
---

# Herdr Cairn Tool Attribution


## Problem

The baseline dashboard (todo.herdr-live-dashboard) renders cairn ground truth
via `cairn watch`, but it cannot show recent cairn tool activity (which agent
or tool recently called cairn). Note the limit: an MCP proxy sees only cairn
`tools/call` messages and a shell shim sees only cairn CLI invocations;
neither observes direct source-file mutations, so this is recent-activity
surfacing, not causal trigger attribution. `res.herdr-plugin-feasibility` Q4
verified two plumbing points: an MCP wrapper around the public
`cairn::mcp::handle_line` entry point (`src/mcp/mod.rs:72-109`; the internal
name/result evidence lives in the private `call_tool`, `src/mcp/mod.rs:147`
onward) and a shell shim around the `cairn` CLI. Both are harness-agnostic
and need zero cairn source changes.

## Task

Decide whether recent-activity surfacing is worth building, and if so, ship
the cheapest option: an external MCP proxy that, per `tools/call`, spawns
`herdr pane report-metadata <pane_id> --source cairn-activity --token
tool=<name>` targeting the dashboard pane (resolve `<pane_id>` at proxy start,
e.g. from a `herdr pane list` lookup by the dashboard's label, and keep the
`--source` ID stable). The spawn must keep the MCP response path non-blocking
(a `herdr pane` shell-out is 70-110ms; see Q4) AND reap the child: hand it to
a background waiter thread or hold a persistent Herdr connection; dropping
`std::process::Child` without waiting leaks zombies in a long-running proxy.

## Unblock condition

Blocked until todo.herdr-live-dashboard is done AND a session has recorded a
concrete instance where not knowing which agent/tool triggered a finding
change cost time (note it in this todo's body). Until both hold, this task
is not evaluable.

## Acceptance

Either a documented decision not to build it, or a wrapper/shim that surfaces
the most recent cairn-calling tool name on the dashboard pane without adding
material latency to cairn MCP/CLI calls and without leaking child processes.