---
name: "Cairn Dev Loop"
description: Adapter-native invocation for one iteration of cairn development. Resolves to cairn-dev loop mode, which is the normative procedure.
category: Workflow
tags: [workflow, cairn, dogfood]
---

This command is transport. It carries no procedure of its own.

The normative procedure for one cairn development iteration is `cairn-dev` loop
mode, at `.claude/skills/cairn-dev/references/loop-mode.md`
(`dec.loop-command-harness-model` clause 8, relocated by
`dec.unified-cairn-dev-entry`).

Do this:

1. Load `.claude/skills/cairn-dev/references/loop-mode.md`. If it cannot be
   loaded, touch nothing, report that, and output `LOOP HALTED` as the final line.
2. Bind MISSION to any text in this message beyond the command itself, and hand it
   to loop mode as its MISSION input.
3. Follow loop mode verbatim, including its required asset closure and its
   fail-closed rows.
4. Pass its terminal token through unchanged as your final line, alone and
   verbatim. Append nothing after it.

Invoking this command IS the explicit selection that loop mode requires. Nothing
else in this file overrides, extends, or reinterprets loop mode; where this file
and loop mode appear to differ, loop mode governs and this file is wrong.
