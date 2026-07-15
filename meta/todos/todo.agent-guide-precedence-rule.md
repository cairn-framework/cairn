---
node: cairn.kernel.cli
status: open
created: 2026-07-13
related: [dec.init-emits-agent-skills]
---

# Ship the AGENTS.md-precedence rule in the emitted agent guide

Owner field report (2026-07-13): a downstream repo had to establish manually
that its own AGENTS.md overrides shipped cairn skill guidance when the two
conflict (the shipped cairn-apply skill is cargo-flavoured; the repo is
TypeScript). That precedence rule currently lives nowhere in the emitted
surface.

## Task

State the rule explicitly in the agent guide `cairn init` writes
(`agent_guide.md`, emitted via `src/cli/commands/project.rs`): the target
repo's AGENTS.md wins over shipped cairn skills wherever they conflict;
shipped skills are defaults, not authority. One sentence in the guide, not a
new mechanism.

## Relations

Pairs with the language-aware gate derivation owned by
todo.cairn-apply-parallel-mode (gh:#245). If dec.agent-pack-packaging is
accepted, the rule ships as part of the pack contract wording there instead;
do not state it in two places.
