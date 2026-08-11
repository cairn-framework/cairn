---
name: cairn-dev
description: "Entry point for working in a repository that contains cairn.blueprint. Use when navigating architecture, adding or moving files, interpreting cairn findings, authoring decisions or todos, or when the user says 'check cairn', 'update the blueprint', or 'cairn lint fails'."
---

# cairn-dev

This repository declares its architecture in `cairn.blueprint`; cairn reconciles
that declaration against the code and reports the gap. This file is a router:
load the one reference your task needs, and only that one.

## Authority

The target repository outranks this guide: its `AGENTS.md`, `CLAUDE.md`, and
`CONTRIBUTING.md` decide conventions, gates, and workflow, and accepted
decisions in the graph (`cairn rationale <node>`) bind the nodes they name.
Where this guide disagrees with either, this guide is wrong.

## First move

```bash
cairn context
```

It returns the nodes, edges, artefacts, and current findings, and names the
node that owns the area you were asked about. Run it before opening files.
Orient from the queries, never from `map.json` or `map.md`: those are generated
review snapshots for humans reading a diff, not agent context.

## The gate

```bash
cairn scan        # zero findings is the target
cairn hook all    # exit 0 means the commit is safe
```

Run both before you hand work back. Fix the cause of an Error finding. Never
bypass a hook.

## Routes

Match the session to one row and load only that reference. Paths are relative to
this file.

| The task is | Load |
|---|---|
| Find why something misbehaves, then fix it | `references/task-bug-investigation.md` |
| Restructure code without changing behaviour | `references/task-refactoring.md` |
| Understand how a system fits together | `references/task-architecture-discovery.md` |
| Build something new, or extend a module | `references/task-feature-implementation.md` |
| Mine an existing codebase into proposed decisions | `references/task-brownfield-decision-extraction.md` |
| Write or edit `cairn.blueprint` | `references/blueprint-syntax.md` |
| Interpret a finding code | `references/finding-codes.md` |
| Write a decision, research, source, or todo | `references/artefact-schemas.md` |
| Look up a command or flag | `references/command-reference.md` |
| Navigate the graph, read source through it, or resolve a node id | `references/graph-navigation.md` |
| Run one full cairn development iteration | `references/loop-mode.md`, if installed, and only on explicit request (see below) |

If no row fits, stay here and work from `cairn context`, `cairn get`, and
`cairn neighbourhood`.

## Loop mode

`references/loop-mode.md` is the fail-closed procedure for one full development
iteration: one unit, one squash commit, one terminal token. It is not ordinary
development guidance. A repository opts in by installing it alongside its
required assets; `cairn init` does not. If the file is absent here, loop mode is
unavailable in this repository: say so and use the routes above rather than
reconstructing the procedure from memory. Enter it only when the user or the
harness explicitly requests a cairn development iteration, by name or through an
adapter-native invocation such as `/cairn-loop`, never because a session merely
resembles cairn work.

## Working discipline

- State your assumptions; when a request has materially different readings,
  present them rather than silently picking one.
- Before you start, turn the task into a success criterion you can check by
  running something. For a bug, that is a test that fails now and passes after.
- Prefer the smallest change that meets the criterion.
- Keep the blueprint honest: every new file falls under a node `path`, tests
  included, and every new cross-module call gets an edge. The task references
  carry the specifics.
- Record friction with `cairn feedback "<what you expected, what happened>"`.
