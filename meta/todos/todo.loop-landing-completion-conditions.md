---
node: cairn.kernel.cli
status: open
created: 2026-08-10
---

# Make the loop landing completion rule handle unfinished and rerouted units

Two defects in the shipped landing procedure
(`.claude/skills/cairn-loop-landing/SKILL.md` and its canonical copy under
`tools/agent-pack/content/skills/`), both surfaced while decomposing
`todo.brownfield-extraction-external-validation` on 2026-08-10.

## Task

**1. Parent completion keys on open children, not on done children.** The
procedure says to set the selected todo `done` and, "when it was the last open
child, flip the blocked parent to `done`". A parent with one `blocked` child and
one `open` child therefore completes when the open child lands, even though the
blocked child's criteria are unmet. Maintainer-gated work must be `blocked` to
stay out of selection, so every parent holding such a child is exposed. The rule
should key on the parent's declared `blocked_by` targets all being done, which
is the ordering link `dec.todo-relationship-model` defines, rather than on no
`open` child remaining. `parent` is a grouping link in that model and
`CAIRN_TODO_STATUS_CONTRADICTION` is computed from `blocked_by` alone
(`src/artefacts/registry/validate/relations.rs`), so keying completion on
children would need the schema amendment the decision reserves. The completion
rule is procedural either way: the scanner will not catch the mistake for you.

The brownfield decomposition worked around this by hanging the maintainer-gated
todo one level up and encoding order in a single `blocked_by` edge. That is a
per-unit workaround, not a fix, and it costs a level of nesting each time.

Second occurrence, 2026-08-10: `todo.console-orchestration-ux-design` reached
Scope with only a maintainer act outstanding, so the iteration extracted that
criterion to `todo.console-round-three-closeout` rather than take the reroute
path Land would have closed as `done`. Same shape, same cost.

**2. There is no landing path for a unit that lands unfinished.** Loop mode
routes `REROUTED` straight to Land, and `cairn-loop-scope` section 4 tells a
rerouted unit to set its own todo `blocked` and land the tracker edits. Landing
then unconditionally sets the selected todo `done`, which closes an unmet
acceptance criterion. The sizing-rule decomposition path has the same shape: it
deliberately leaves the selected parent `blocked`, and the procedure has no
clause acknowledging it. Landing needs an explicit branch: when the unit lands
tracker edits instead of satisfying its acceptance, the selected todo keeps the
status the reroute set and no parent is flipped.

Both fixes are edits to the same procedure text plus its canonical pack copy,
which must stay byte-identical, and the pack manifest and determinism tests may
pin their sizes.

## Acceptance

- The landing procedure completes a parent only when every one of that parent's
  declared `blocked_by` targets is `done`, and says so in terms a reader can
  check against `cairn todos <node>` output.
- The landing procedure carries an explicit unfinished-unit branch covering both
  the `REROUTED` path and the sizing-rule decomposition path, naming what
  happens to the selected todo's status and to its parent.
- The canonical pack copy and the `.claude` copy stay byte-identical, and the
  agent-pack determinism and router tests pass.
- `cairn scan --strict` exits 0.
