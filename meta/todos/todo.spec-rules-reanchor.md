---
node: cairn.kernel.map
status: open
created: 2026-07-30
---

# Re-derive the stale Spec anchors in the spec-rule registry

`res.spec-rules-anchor-drift` (2026-07-30, at `022faef`) audited every
`Spec` anchor in `docs/registries/spec-rules.md` against the `docs/spec.md`
text at that line and found the anchors systematically stale: `spec:620-636`
point into the section 9.6 rename prose and the section 10 heading,
`spec:865-867` into the open-questions list, `spec:474`/`spec:476` into the
section 8.5 research template, and `spec:61`/`spec:318`/`spec:327`/`spec:341`
at a blank line, the wrong rule, a section heading, and template frontmatter
respectively. Only `spec:24` and `spec:515` anchor text that states their
rules. The finding-emission half of every row is correct; only the anchors
drifted.

## Scope

- For each Enforced, Pending, and Declared row, re-derive the `Spec` cell
  from the rule text: find the normative sentence in `docs/spec.md` that
  states the rule and anchor to its current line. Where the rule no longer
  originates in the spec (the registry owns it outright,
  `dec.spec-authority-retirement`), set the cell to `-` instead of inventing
  an anchor.
- Treat no anchor as good without re-reading its line, including `spec:24`
  and `spec:515`.
- Do not restate or renumber rules in `docs/spec.md` itself; this unit edits
  the registry only.
- Consider adding a drift guard if one is cheap: the spec-rule coverage check
  already parses the registry, so asserting each anchored line mentions a
  keyword from its Rule cell may be a small extension of
  `src/map/spec_rule_coverage/`. If it is not small, note why and leave the
  guard to its own unit.

## Depends on

- Nothing. The registry and spec are both on main.

## Acceptance

- Every `Spec` cell either anchors a line whose text states the row's rule or
  is `-`, verified by reading each anchored line.
- `cargo test --test finding_code_coverage` and the spec-rule coverage tests
  still pass; no row's Code or Status cell changes.
- `cairn scan --strict` exits 0.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves investigable. It keeps specification rules anchored to the surfaces that implement them.
