---
name: "Cairn Dev Loop"
description: Adapter-native invocation for one iteration of cairn development. Resolves to cairn-dev loop mode, which is the normative procedure.
category: Workflow
tags: [workflow, cairn, dogfood]
---

This command is transport. It carries no procedure of its own.

The normative procedure for one cairn development iteration is `cairn-dev` loop
mode, at `skills/cairn-dev/references/loop-mode.md` inside the installed pack
root (`.claude/` for the Claude adapter, `.omp/` for the OMP adapter)
(`dec.loop-command-harness-model` clause 8, relocated by
`dec.unified-cairn-dev-entry`).

Do this:

1. Run `cairn pack campaign verify` and read its message, not only its exit code.
   - `HALT: the installed pack no longer matches this campaign.`: the pack moved
     under the campaign. Touch nothing, report the output, and output
     `LOOP HALTED` as the final line. Any other failure you cannot classify is
     the same row.
   - `No campaign is active here.`: nothing is pinned, and this is not a
     failure even though the exit code is non-zero. Load assets from the
     installed pack at their ordinary paths and continue.
   - Exit 0: a campaign is pinned. It reports a `pinned:` directory holding
     immutable copies of loop mode and its whole closure. Load every asset from
     that directory, mirroring the pack path underneath it, and read no other
     copy for the rest of this session.
2. Load loop mode: `references/loop-mode.md` under the pinned directory when one
   is active, otherwise `skills/cairn-dev/references/loop-mode.md` under the
   installed pack root. If it cannot be loaded, touch nothing, report that, and
   output `LOOP HALTED` as the final line.
3. Bind MISSION to any text in this message beyond the command itself, and hand it
   to loop mode as its MISSION input.
4. Follow loop mode verbatim, including its required asset closure and its
   fail-closed rows, resolving every asset through the same source as step 2.
5. Pass its terminal token through unchanged as your final line, alone and
   verbatim. Append nothing after it.

Invoking this command IS the explicit selection that loop mode requires. Nothing
else in this file overrides, extends, or reinterprets loop mode; where this file
and loop mode appear to differ, loop mode governs and this file is wrong.
