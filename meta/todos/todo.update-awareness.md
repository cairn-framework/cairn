---
node: cairn.kernel.cli
status: open
created: 2026-07-16
---

# Simple Cairn updates and update awareness

Give users one memorable Cairn command for checking and applying a stable
update. Normal Cairn commands should also expose a lightweight, non-blocking
update signal so humans and AI agents can notice an outdated binary without
turning routine project queries into a network dependency.

## Required outcomes

- Add an explicit update surface with human-readable and `--json` output. It
  reports the installed version, latest stable version, whether an update is
  available, and the action taken.
- Make updating simple across the supported install paths. Prefer one Cairn
  command; where an installation channel must own replacement, detect it and
  provide or invoke the exact safe channel-native action instead of silently
  mixing installers.
- Surface cached update availability through an agent-facing command already
  used for orientation, such as `cairn status` or `cairn context`, so an agent
  can tell the user once when their installed version is behind.
- Rate-limit notices and remember acknowledgement per released version. Do not
  print the same notice on every command.
- Keep routine commands fast and offline-safe. Network failure, rate limiting,
  malformed release data, and an unavailable cache must never fail a project
  command or alter its exit code.
- Do not mutate the installation during an implicit check. Applying an update
  remains an explicit user action.
- Define opt-out and CI behaviour, cache location and expiry, prerelease
  handling, and the trust source used to discover the latest release.
- Centralise all user-facing copy in `docs/design-system/copy.toml`.

## Acceptance

An outdated-version fixture produces a visible human notice and a stable JSON
field that an AI agent can act on. The same released version is not announced
again after acknowledgement. Fresh, offline, failed-request, prerelease, and
install-channel cases are covered without contacting the real network in
tests. A smoke scenario proves the explicit update surface from check through
the selected update action.

This is separate from the pack lifecycle in `todo.agent-guidance-program`, whose `pack update`
refreshes Cairn's installed agent guidance and skills rather than the Cairn
binary itself.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It lets users know when a new Cairn binary release exists and guides the selected update action.
