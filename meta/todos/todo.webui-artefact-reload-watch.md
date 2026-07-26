---
node: cairn.ui
status: done
created: 2026-07-17
related: [todo.webui-load-error-surfacing]
---

# Webui Artefact Reload Watch

## Problem

`Server::watched_files_mtime` (src/ui/server.rs) watches the blueprint,
target reports, and contract files, but not artefact inputs
(meta/todos, meta/decisions, meta/research, meta/sources). Editing or
adding an artefact mid-session leaves the UI serving stale data until
the blueprint or a source file changes.

## Direction

Include artefact file mtimes (and their directories, so newly added
files are noticed) in the reload decision, with a regression test that
edits an artefact and observes the refreshed API payload. Surfaced
during review of todo.webui-load-error-surfacing (gh:#414).
