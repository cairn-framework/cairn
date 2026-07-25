# Working with cairn in this repository

This project declares its architecture in `cairn.blueprint`. Cairn reconciles that
declaration against the code and reports the gap.

## This repository's own instructions come first

Whatever this repository says in its `AGENTS.md`, `CLAUDE.md`, or
`CONTRIBUTING.md` outranks cairn's guidance, and accepted decisions in the graph
bind the nodes they name. Cairn tells you what exists and what is out of sync; the
repository tells you how to work.

## Orient before you read files

```bash
cairn context
```

That one command returns the nodes, edges, artefacts, and current findings. It
tells you which node owns the area you were asked about. Query cairn for project
state rather than inferring it from notes, scratch files, or memory: the graph is
the source of truth. Every command accepts `--json`.

## The gate

```bash
cairn scan        # zero findings is the target
cairn hook all    # exit 0 means the commit is safe
```

Run both before committing. New source files, tests included, must fall under some
module's `path` in `cairn.blueprint`, or `cairn scan` will report them as
orphaned.

## Everything else

`cairn init` installed the `cairn-dev` skill at
`.claude/skills/cairn-dev/SKILL.md`. It is the entry point: a short router that
loads the one reference your task needs, whether that is navigating the graph,
investigating a bug, writing a decision, or running a full development iteration.
Start there rather than reading the references directly.

If cairn misbehaves or gets in your way, record it before moving on:

```bash
cairn feedback "what you expected, and what happened instead"
```
