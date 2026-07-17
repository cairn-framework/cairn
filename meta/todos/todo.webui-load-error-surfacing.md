---
node: cairn.ui
status: done
created: 2026-07-17
related: [todo.webui-artefact-reload-watch]
---

# Webui Load Error Surfacing

gh:#414

## Problem

When `scanner::load_project` failed (for example a blueprint parse
error), every `/api` route returned 500. The server body carried the
diagnosis (`CAIRN_UI_PROJECT_LOAD_FAILED` plus the parse error) but the
webui's `fetchJson` discarded it and showed only "request failed:
/api/graph (500)". Nothing was logged to the terminal, and a mid-session
parse error hard-failed even when a good cached scan existed.

## Fix

- `fetchJson` (src/ui_assets/utils.js) parses the structured error body
  and surfaces `CODE: message` in the boot banner.
- `Server::api` logs the load failure to the terminal.
- `Server::load_project` serves the cached scan (with a logged warning)
  when a reload fails mid-session; a first-load failure still returns
  500 with the error body.

Regression tests: `test_ui_project_load_failure_returns_diagnostic`,
`test_ui_project_load_failure_serves_cached_scan` (src/ui/mod.rs).
