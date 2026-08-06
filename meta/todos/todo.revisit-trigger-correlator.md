---
node: cairn.root
status: blocked
created: 2026-07-27
---

# Revisit Trigger Correlator

## Why this exists

`cairn lint` reports one standing finding:

```
CAIRN_SPEC_RULE_UNIMPLEMENTED [info] docs/registries/spec-rules.md
spec rule `ADR revisit_triggers appear relevant to recent changes` (spec:634)
is pending but names no enforcer
(deferred by dec.revisit-trigger-correlator-deferred)
```

`dec.revisit-trigger-correlator-deferred` (accepted, 2026-06-29) rules that
`spec:634` stays `pending` and that the Info "remains the honest living tracker of
a Designed-but-unbuilt rule". It said to "re-file an implementation bead when
such a capability lands", so no tracker existed.
`dec.loop-selection-deferred-findings` (accepted 2026-07-29) recommends that
deferred work be represented by a todo, so this record is filed `blocked` as
the tracker item for the parked enforcer. It does not supersede the earlier
ruling and does not wait on ratification to be useful: nothing about the
correlator may be built, and a blocked todo is a record, not a work item.

## Scope

Build the `spec:634` enforcer: flag decisions whose `revisit_triggers` have
actually fired against recent changes, and promote `spec:634` from `pending` to
`enforced` in `docs/registries/spec-rules.md`.

Foreclosed by the accepted decision, do not reopen without superseding it:

- deterministic term-coverage correlation (measures topical proximity, and its
  one live hit is a structural tautology: `res.revisit-trigger-correlator-probe`);
- verbatim trigger matching (0 recall) and node-id matching (0 recall);
- git-log correlation (foreclosed by that decision as breaking scan purity).

## Depends on

A capability that can judge relevance rather than proximity. The deferring
decision names two candidates, and only one is still outstanding: the
`cairn-iy2` ghost-rule primitive already shipped (`dec.ghost-rule-tracking`,
commit `7e49981`) and is what emits this very finding, so it tracks a
Designed-but-unbuilt rule without judging whether a trigger fired. The
remaining candidate is a maintainer-sanctioned semantic or vision gate, which
would also supersede the vision-loop foreclosure now carried by
`dec.webui-write-authority`.

Blocked on that gate. It does not exist, no todo proposes it, and the accepted
decision forbids shipping a proximity proxy in its place. Unblock by setting
this todo `open` if a maintainer sanctions one.

## Acceptance

- The enforcer distinguishes "this trigger condition fired" from "a change is
  working in this decision's area", demonstrated against the live corpus
  including the `webui-design-quality` pair the probe falsified.
- `spec:634` moves to `enforced` and `CAIRN_SPEC_RULE_UNIMPLEMENTED` stops firing
  for it.
- A superseding decision records the capability that made the ruling change.

## Mission disposition

2026-08-02: blocked against dec.cairn-mission. Serves investigable. It remains deferred under the accepted revisit-trigger decision.
