# Borrow list: what to take from Blitzy

Ranked by leverage given our actual roadmap (finish phases 7.6 → 10, then revisit orchestration).

## Borrow now (cflx era)

### B1. Upgrade cflx to 0.6.66 + add escalation/diagnose commands
**Leverage**: high. **Effort**: minutes. **Risk**: near zero.

Not a Blitzy borrow per se, but the upgrade contains the bug fixes that bite during autonomous loops (rejected-apply handoff, log spam, manual-resolve scheduler restart). The escalation/diagnose config maps loosely to Blitzy's "run the strong model when the cheap one stalls" pattern.

```jsonc
{
  "apply_command":                "opencode run '{prompt}'",
  "apply_escalation_command":     "claude -p '{prompt}'",
  "apply_stall_diagnose_command": "claude -p 'diagnose stall for {change_id}'",
  "stall_detection": {
    "enabled": true, "threshold": 5,
    "apply_escalation_after_empty_wip": 3,
    "apply_escalation_max_uses_per_stall": 2
  }
}
```

### B2. Make the navigation-not-truth principle explicit in phase 8 design
**Leverage**: high. **Effort**: paragraph in design.md. **Risk**: zero.

When phase 8 (summariser) opens, write into `design.md`: "summarisers are navigation aids, not substitutes for artefact retrieval. The reconciler always pulls live content. Generated summaries are advisory metadata, not authority." This is the lesson from Blitzy's "RAG as navigation layer" approach, and it pre-empts the failure mode where summary text is treated as ground truth.

### B3. Continue manual proposal review (Layer 1 / AAP review)
**Leverage**: high. **Effort**: zero (already happening). **Risk**: zero.

Recent commits show this pattern already works for us. Three of the last five commits before research were AAP-review iterations. The `.0-tests` pre-phase mechanism is structurally an AAP-correctness gate. Don't formalize it into scripts yet; the manual cadence is producing the right outcomes.

## Defer until post phase 10

### D1. Multi-vendor reviewer ensemble at acceptance
Vendor diversity is structural, not cosmetic. When we eventually formalize, wire in three vendor families (Anthropic + OpenAI + Google), not three prompts against one. Until then, single claude reviewer at acceptance plus the verification gate is enough.

### D2. Risk-tier system on proposals
Blitzy's Deep / Standard / Light tiering is useful at scale but premature for our current change volume. Add `review_tier` to proposal frontmatter only when we have 50+ changes/quarter flowing through cflx.

### D3. Report-card artifact for distributable changes
Worth considering during phase 10 distribution: a per-change human-readable evidence summary (what was tested, what blockers were resolved, what was deferred to Future Work, gates passed). This would ship alongside the code, not replace it. Note it in phase 10 design.md when we get there.

### D4. Dynamic agent generation
Out of scope for cflx. This is the thing CAIRN-native orchestration would do that cflx structurally cannot. Do not try to retrofit cflx for it; treat it as a phase 12+ horizon.

## Reject

### R1. Replacing acceptance with multi-reviewer ensemble for all changes
Blitzy's own guidance is that Layer 3 (code review) effort should be reserved for Deep tier. Piling reviewers onto every change is the wrong end of the curve.

### R2. Runtime telemetry capture
Blitzy runs customer applications in parallel cloud sandboxes to capture true execution dependencies. This is a major engineering investment with cloud cost implications. Not in scope for any current phase.

### R3. Vector embeddings as primary context substrate
CAIRN's typed-artefact kernel is structurally better than vector substrate for the use cases we care about (declared-vs-actual reconciliation). Don't add a vector layer just because Blitzy has one; their workload (100M LOC enterprise codebases) is not our workload.

## Borrow priority for finishing initial phases

In order:
1. **Now**: B1 (cflx upgrade + escalation config).
2. **When phase 7.6 starts**: keep B3 (manual review) cadence; the `.0-tests` pattern already operationalizes Blitzy's "review intent before code" principle.
3. **When phase 8 opens**: B2 (navigation-not-truth constraint in summariser design).
4. **When phase 10 opens**: revisit D3 (report-card artifact) for distributability.

Everything else stays parked.
