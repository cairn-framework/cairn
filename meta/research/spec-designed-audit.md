---
id: res.spec-designed-audit
nodes:
  - cairn.root
date: 2026-06-27
method: primary
---
# Audit: spec.md Designed items vs implementation

Evidence for bead cairn-mqe. spec.md:22-26 defines the maturity ladder
Declared -> Designed -> Implemented and states implementation status is "tracked
in the project's own Cairn state, not in the spec itself" (spec:24). That tracking
is not enforced, so a Designed-but-unbuilt rule can rot in prose and pass scan
(proven by cairn-481: the leaf-contract rule Designed at spec:318, unimplemented,
silently green). This is a one-time exhaustive pass over every Designed integrity
rule, freshness rule, and rationale tension in spec.md, cross-referenced against
the emitted finding codes in `src/` and the error-code registry.

## Method

For each rule: read its spec section, note its maturity marker (unmarked =
Designed; "(Declared, see section 17.)" = Declared), then grep `src/` (excluding
tests) for an emitted `CAIRN_*` finding code that enforces it. A registry entry in
`docs/registries/error-codes.md` is NOT evidence of emission; only a constant
emitted in non-test source counts as implemented.

## Gap list

### Structural errors (spec:619-625, Designed, block commits)

| Rule | spec | Emitted code | Implemented? |
|---|---|---|---|
| Duplicate node IDs | 620 | CAIRN_INTEGRITY_DUPLICATE_ID | yes |
| Path ties between leaf nodes | 621 | CAIRN_INTEGRITY_PATH_TIE | yes |
| Broken artefact pointer | 622 | CAIRN_ARTEFACT_POINTER_MISSING / CAIRN_CONTRACT_MISSING | yes |
| Artefact references non-existent node | 623 | CAIRN_ARTEFACT_UNKNOWN_NODE | yes |
| Source SHA mismatch | 624 | CAIRN_SOURCE_SHA | yes |
| Orphan file under claimed container | 625 | CAIRN_RECONCILE_ORPHANED_FILE | yes |

### Interface contradiction (spec:627-628, Designed)

| Rule | spec | Emitted code | Implemented? |
|---|---|---|---|
| Module interface hash drift | 628 | CAIRN_INTERFACE_HASH_CHANGED | yes |
| Multi-target interface divergence | 637 | CT001/CT002 (CAIRN target checks) | yes |

### Artefact integrity rules (Designed)

| Rule | spec | Emitted code | Implemented? |
|---|---|---|---|
| Research must cite >=1 source (unless primary) | 61 | CAIRN_RESEARCH_MISSING_SOURCES | yes |
| Decision must cite >=1 research/source | 61 | CAIRN_DECISION_UNKNOWN_PROVENANCE | yes |
| Leaf node should have a contract (warn; error ghost->synced) | 318 | CK003 CAIRN_CONTRACT_LEAF_UNCOVERED | yes (cairn-481) |
| Todo references exactly one valid node | 339 | CAIRN_TODO_ORPHAN_NODE | yes |
| Source referenced by >=1 research/decision | 474 | CAIRN_SOURCE_ORPHAN | yes |
| External source `file` must be a URL | 474 | CAIRN_SOURCE_EXTERNAL_URL | yes |
| Unverified source persists as tension | 474 | CAIRN_SOURCE_UNVERIFIED | yes |
| Decision `supersedes` target must be `superseded` | 867 | CAIRN_DECISION_SUPERSEDES_STATUS | yes |

### Freshness rules (Designed)

| Rule | spec | Emitted code | Implemented? |
|---|---|---|---|
| Verified source hash change -> structural error | 476 | CAIRN_SOURCE_SHA | yes |

### Rationale tensions (spec:630-637, Designed, advisory)

| Tension | spec | Emitted code | Implemented? |
|---|---|---|---|
| Decision cites deleted research/source | 631 | CAIRN_DECISION_UNKNOWN_PROVENANCE / *_REFERENCE_UNKNOWN | yes |
| Research not linked from any decision | 632 | (none) | **NO -> cairn-mqe gap 1** |
| Source not cited by any research/decision | 633 | CAIRN_SOURCE_ORPHAN | yes |
| ADR `revisit_triggers` appear relevant to recent changes | 634 | (none) | **NO -> cairn-mqe gap 2** |
| Edge divergence (declared edge vs observed import) | 635 | (none) | Declared, sec 17 (not a gap) |
| Docstring drift (authored docstring vs map) | 636 | (none) | Declared, sec 17 (not a gap) |
| Multi-target interface divergence | 637 | CT001/CT002 | yes |

## Confirmed Designed-but-unimplemented gaps

**Gap 1 - research-orphan tension (spec:632).** `validate_sources`
(src/artefacts/registry/validate/mod.rs:212) flags a source no research/decision
cites (CAIRN_SOURCE_ORPHAN), but there is no parallel check that a research
artefact is cited by some decision. The data is already available
(`research_ids` vs each `decision.informed_by`). Clear asymmetry; advisory
(warning) per spec. Filed as a tracking bead.

**Gap 2 - revisit-trigger relevance tension (spec:634).** `revisit_triggers` is
parsed (src/artefacts/registry/mod.rs:96), stored
(src/artefacts/registry/types.rs:139), and rendered in the webui
(src/ui_assets/app.js:1195), but no finding ever evaluates whether a trigger is
relevant to recent changes. Unlike gap 1 this needs a design decision for what
"relevant based on recent changes" means (it depends on change-correlation
semantics), so it is a natural input to cairn-iy2 (the ghost-rule primitive).
Filed as a tracking bead, related to cairn-iy2.

## Non-gaps (intentionally not filed)

- **Edge validation against observed imports (spec:603, 635; registry CE001-CE003)**
  and **docstring drift (spec:604, 636; registry CE004-CE009)** are both marked
  "(Declared, see section 17.)". Section 17 deliberately does not commit to a
  Tree-sitter/LSP semantic-analysis strategy, on which both depend. They are
  correctly at Declared maturity, tracked by the phased build order (phase 5),
  not Designed-but-unimplemented. The registry pre-allocates their codes
  anticipatorily; absence of emission is expected.
- **Decision-violation gating (spec:71 area)** is explicitly v2-deferred, not a
  miss.
- **Todo coverage** is "informational in v1" (spec:339), not an enforced rule.

## Conclusion

Every Designed integrity/freshness rule and all but two rationale tensions are
implemented. Two confirmed gaps filed as tracking beads (research-orphan,
revisit-trigger relevance). The two large unimplemented capabilities people
notice first (edge validation, docstring drift) are Declared, not Designed, so
they are out of this audit's scope by the spec's own maturity rules.
