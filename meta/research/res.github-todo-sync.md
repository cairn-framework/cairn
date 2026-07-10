---
id: res.github-todo-sync
nodes:
  - cairn.root
date: 2026-07-10
method: secondary
sources: [src.gh-cli, src.alstr-todo-to-issue]
informed_by: [res.native-task-state-gap]
---

# Mirroring native todos to GitHub issues without creating a second source of truth

## Question
How should cairn keep `meta/todos/*.md` and GitHub issues in lockstep so the
drift observed on 2026-07-10 (roughly 16 issues filed directly on GitHub while
repo work lives in native todos) cannot recur? The architecture invariant is
fixed and already ratified: files in git are the source of truth; GitHub issues
are a derived projection, never a second canonical store
(`dec.bead-github-sync`, accepted). This note evaluates the mechanism, not the
invariant.

## Options considered

### (a) One-way mirror script (todo -> issue)
A script run as a CI job (on push to `main`) or a pre-push hook, using the
`gh` CLI (src.gh-cli). Each todo is upserted to an issue carrying a stable
identity marker: a body line `cairn-todo: todo.<slug>` plus a `cairn-todo`
label. The script lists issues with that label, parses markers into a
slug-to-number map, then creates/updates/closes. A todo that flips to `done`
closes its issue. Re-running is idempotent because the marker is the key and the
script reads current state before writing.
- Pros: honours the ratified invariant exactly; deterministic; no cairn code
  touches GitHub; the marker survives edits and closures so re-sync is stable.
- Cons: a push-only mirror lags between runs (acceptable for a visibility
  projection, per `dec.bead-github-sync` risks); needs a `GH_TOKEN` in CI.

### (b) Inbox direction (externally filed issues -> triage flag)
The same script also scans issues that lack the `cairn-todo` marker (i.e. filed
directly on GitHub, including those born from `cairn feedback`). It flags them
with label `cairn-todo-unmapped` and a comment prompting a human to create a
native todo. It never auto-imports: no issue is silently turned into a todo.
- Pros: closes the inward gap that option (a) alone ignores; preserves the
  single-writer rule (a human, not the script, decides what becomes a todo);
  turns drift into a visible, actionable queue.
- Cons: requires a human to actually triage, so the unmapped pile can grow if
  ignored; this is a process dependency, not a code failure.

### (c) Existing tools vs bespoke
- `alstr/todo-to-issue-action` (src.alstr-todo-to-issue) watches for `- [ ]`
  checkboxes in markdown/code and opens issues from them. It is keyed to the
  checkbox-todo model, has no notion of our `meta/todos/*.md` frontmatter
  (`status: done` closes, single node ID), does no inward triage, and offers no
  stable per-todo identity marker for idempotent upsert. Adapting it would mean
  fighting its model.
- A bespoke script of roughly 100 to 120 lines (parsing our frontmatter, calling
  `gh issue create/edit/close`, labelling) maps cleanly to the ratified design
  and owns the marker convention. Given the custom frontmatter and the inward
  triage requirement, the bespoke script clearly wins.

## Recommendation
Adopt a single bespoke one-way mirror script that combines (a) and (b): outward
todo-to-issue upsert plus inward unmapped-issue flagging, with no auto-import in
either direction. This is exactly the opt-in, never-canonical, never-bidirectional
mirror `dec.bead-github-sync` sketches, transplanted from the obsolete beads/Dolt
mechanism onto the current native-todo canonical store. Reject any bidirectional
or off-the-shelf checkbox tool. The script lives under `scripts/` and is invoked
by CI on push to `main` (single writer, post-merge); it is also runnable locally
for ad-hoc sync. Cairn's own code stays out of the GitHub write path.

## Reconciliation rules

| Source event | Script action on GitHub |
|---|---|
| Todo created in `meta/todos/` | Upsert: create issue if no marker match; body gets `cairn-todo: todo.<slug>`, label `cairn-todo`. |
| Todo status changed (`open`/`in_progress`/`blocked`) | Upsert: update title/body/state to open; keep marker. |
| Todo flipped `done` | Close the issue (add a `done` note); marker line retained so re-sync is a no-op. |
| Todo deleted from `meta/todos/` | Close its open issue with a note; leave it closed. Never delete the GitHub issue. |
| Issue opened externally (no marker) | Flag: add label `cairn-todo-unmapped` + triage comment. Never auto-create a todo. |

Idempotency rule: on every run the script rebuilds the slug-to-number map from
issues carrying `cairn-todo`, compares desired state to actual, and writes only
diffs. Two consecutive runs change nothing.

## Interaction with `cairn feedback` (issue #246)
`cairn feedback` records to `.cairn/feedback.md` and prints a prefilled upstream
issue URL, so it is the channel that produces most externally filed issues today.
The inward flagger in this script is the detection half of that loop: it labels
those issues `cairn-todo-unmapped` and asks for a native todo. A possible native
home later is to extend `cairn feedback --json` (issue #246, request 2) to also
drop a `todo.<slug>` stub in `meta/todos/`, closing the loop without the script
ever importing an issue. That extension is out of scope for this pass; for now
the mirror consumes external issues as unmapped and leaves triage to a human.

## Source of truth restated
`meta/todos/*.md` in git is canonical. The script is a projector. If GitHub and
the todos disagree, the todos win and the next sync fixes GitHub; GitHub edits
are never read back.
