---
node: cairn.root
status: done
created: 2026-08-10
parent: todo.context-engineering-pass
related: [res.skill-absorption]
---

# Context engineering pass over AGENTS.md

Child of todo.context-engineering-pass (item 1 plus its share of item 4).
Source: `src.context-engineering-claude5`. AGENTS.md front-loads conventions
the tree or `cairn context` already answers; prune it to gotchas and pointers.

## Task

1. Prune AGENTS.md: keep the test-locked strings
   (`tests/command_reference_consistency.rs` asserts them), keep guardrails
   that encode real repo-specific traps, drop restatements of things the tree
   or `cairn context` answers.
2. Apply the writing-for-agents framework in `res.skill-absorption`: audit
   every always-loaded line as a context pointer; push reference content down
   the hierarchy behind pointers; end steps on checkable completion criteria;
   prefer positive-form rules, pairing any line that earns hard-guardrail
   status; delete no-op sentences whole; collapse restatements into leading
   words. The Terminology section stays (ubiquitous-language cache).
3. Measure: token count of AGENTS.md before and after, recorded in this todo
   on completion.

## Acceptance

- `cargo test --test command_reference_consistency` (the AGENTS.md
  consistency assertions) green.
- AGENTS.md token count reduced and recorded here on completion.
- No convention lost: each deleted rule is either enforced by a gate already
  or demonstrably answerable from the tree.
- `scripts/check-voice-markers.sh` runs clean (zero FAIL) over AGENTS.md.

## Completion measurement (2026-08-11)

Token count measured with tiktoken `o200k_base`
(`len(enc.encode(text))` over the file before and after):
before 3,687 tokens, after 2,427 tokens, a 34% reduction.
Supporting sizes from `wc -c -w AGENTS.md`:
before 15,263 bytes / 2,056 words, after 10,047 bytes / 1,381 words.
Deletions and where each pruned rule still lives:

- Duplicated "Check if relevant" bullets: folded into the Where things live
  table (single source).
- The 15-line command block: answerable from `cairn --help` and the router's
  command reference; the query-cairn-directly rule and gate commands stay.
- Change-directory contents: the directory answers itself; kept as one line.
- Design-system font and commit detail: answerable from
  `docs/design-system/README.md`, `fonts.css`, and
  `dec.marketing-visual-world`; hardcoded hex/rem is gate-enforced by
  `scripts/check-design-tokens.sh`.
- Em-dash bullet in the UI section: Guardrails keeps the ban; the pre-commit
  hook enforces it.

Gates at completion: `cargo test --test command_reference_consistency`
16 passed, `scripts/check-voice-markers.sh AGENTS.md` 0 fail 0 warn.
