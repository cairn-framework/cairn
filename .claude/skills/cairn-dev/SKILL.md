---
name: cairn-dev
description: "Entry point for working in a repository that contains cairn.blueprint. Use when navigating architecture, adding or moving files, interpreting cairn findings, authoring decisions or todos, or when the user says 'check cairn', 'update the blueprint', or 'cairn lint fails'."
---

# cairn-dev

This repository declares its architecture in `cairn.blueprint`. Cairn reconciles
that declaration against the code and reports the gap.

This file is an index and a router. It is deliberately short. Read the one
reference your task needs; do not read them all.

## Authority

The target repository outranks this guide. Its `AGENTS.md`, `CLAUDE.md`, and
`CONTRIBUTING.md` decide conventions, gates, and workflow. Accepted decisions in
the graph (`cairn rationale <node>`) bind the nodes they name. Where this guide
and either of those disagree, they win and this guide is wrong. Nothing here
restates or overrides an accepted decision.

## First move

```bash
cairn context
```

One command. It returns the nodes, edges, artefacts, and current findings. Read
it before opening files: it tells you which node owns the area you were asked
about, which is the input to everything below.

`map.json` and `map.md` are generated review snapshots for humans reading a diff.
They are not agent context. Never read them to orient; use the queries.

## The gate

```bash
cairn scan        # zero findings is the target
cairn hook all    # exit 0 means the commit is safe
```

Run both before you hand work back. An Error finding blocks; fix its cause. Never
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
| Write or edit `cairn.blueprint` | `references/blueprint-syntax.md` |
| Interpret a finding code | `references/finding-codes.md` |
| Write a decision, research, source, or todo | `references/artefact-schemas.md` |
| Look up a command or flag | `references/command-reference.md` |
| Navigate the graph, or a node id will not resolve | `references/graph-navigation.md` |
| Run one full cairn development iteration | `references/loop-mode.md`, if installed, and only on explicit request (see below) |

If no row fits, stay here and use `cairn context`, `cairn get`, and
`cairn neighbourhood`. Loading a reference you do not need costs the session
tokens and buys nothing.

## Loop mode

`references/loop-mode.md` is the canonical procedure for one full development
iteration: select one unit, land it as one squash commit, emit one terminal
token. It is fail-closed and it is not ordinary development guidance.

**It is optional and often absent.** Loop mode is a dev-loop procedure that a
repository opts into by installing it alongside its required assets
(`cairn-loop-scope`, `cairn-loop-implement`, `cairn-loop-recovery`,
`cairn-loop-landing`). `cairn init` does not install it. If
`references/loop-mode.md` is not present here, loop mode is not available in this
repository: say so and use the routes above instead. Do not reconstruct the
procedure from memory, and do not treat its absence as an error.

When it is installed, enter it only when the user or the harness explicitly asks
for a cairn development iteration, by name or through an adapter-native
invocation such as `/cairn-loop`. Never enter it because a session merely
resembles cairn work, and never enter it to "be helpful". Ordinary requests,
including large ones, are served by the routes above. You may tell the user the
invocation exists; do not invoke it for them.

## Working discipline

- State your assumptions. When a request has materially different readings,
  present them rather than silently picking one.
- Turn the task into a success criterion you can check before you start. For a
  bug, that criterion is a test that fails now and passes after.
- Prefer the smallest change that meets the criterion. If a senior engineer would
  call the result overcomplicated, simplify it.
- New files must fall under some node's `path` in `cairn.blueprint`, tests
  included. If none fits, extend a node's paths or declare a new Module, then run
  `cairn scan`.
- New cross-module calls need a blueprint edge.
- Record friction with `cairn feedback "<what you expected, what happened>"`.
