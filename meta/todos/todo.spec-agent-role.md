---
node: cairn.root
status: open
created: 2026-07-16
---

# Spec Agent Role

Two related fixes to how docs/spec.md serves agents, from the deferral
language audit in `res.a2ui-analysis` (follow-on findings section).

1. Actuator wording alignment. The spec's own section 2 framing is
correct ("the actuator is deliberately external (a human or an agent,
never cairn)", spec:49), but later sections shorten "a human or an
agent" to "the human" for deterministic or agent-capable actions:
spec:612-613 (change flow ends "the human runs cairn archive"; the
cairn-archive skill exists for agents), spec:777 ("the human remains the
ultimate authority over contract content", contradicting spec:771 four
lines above), spec:63 (provenance correctness "human judgment calls";
the point is cairn never makes them, not that agents cannot), spec:252
(init ignore confirmation), spec:817-824 (brownfield "human refines").
Align these with section 2's framing ("the operator, human or agent"),
keeping "human" where genuinely meant (review subtype `human`, the
two-human review example at spec:234, `cairn ui` as a browser surface).
Also reword remediate.rs:224 "must be fixed manually" to "by editing the
blueprint; no command repairs parse errors". This wording lean matters
because agents do read the spec and may infer deterministic actions are
reserved for humans.

2. Entry-point routing. CLAUDE.md says "Read this first for any
architecture question" about the spec, while the same file's cairn
section names `cairn context` as the agent entry point. Reposition: the
graph (context, get, rationale, bundle) is the first stop for
current-truth questions; the spec is design rationale, unbuilt phases
(Declared), open questions, and the changelog, read when the graph lacks
the answer. The blueprint does not replace the spec (different layers:
reconciled structure vs design narrative), and freezing or splitting the
spec was assessed and refuted. Keep the existing extraction discipline:
new normative rules land as spec-rules registry rows with enforcing
finding codes (CK004 machinery), so the enforced surface keeps migrating
out of prose.

Part 1 is a prose pass, no proposal needed. Part 2 touches CLAUDE.md and
the emitted agent guide wording; small, but run it past the maintainer
since it changes the documented read-first rule.
