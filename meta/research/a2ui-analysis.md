---
id: res.a2ui-analysis
nodes:
  - cairn.kernel.scanner
  - cairn.kernel.query
  - cairn.kernel.hooks
  - cairn.tests
  - cairn.root
sources: [src.a2ui]
method: primary
date: 2026-07-16
---

# A2UI comparative analysis: verified lessons for cairn

Multi-agent analysis run 2026-07-16 against `src.a2ui` (pinned commit
`6cb4599`): six parallel deep-reads over a2ui subsystems plus one cairn
baseline read, one synthesis pass producing eleven candidate lessons, then
one adversarial verifier per lesson instructed to refute against cairn's
actual code and principles before accepting. Verdicts: nine adapted with
corrections, two refuted. Refuted lessons and rejected patterns are
recorded here so they are not re-proposed from scratch. A follow-on section
records cairn-internal findings surfaced while verifying (deferral
language, findings-as-tasks, the spec's role for agents).

A2UI and cairn share a core bet: a declarative format produced and
consumed by LLMs where a deterministic validator, not the model, owns
enforcement. A2UI's "safe like data, expressive like code" is a cousin of
cairn's fence-and-navigator framing, and a2ui independently evolved a
blueprint-driven development layer (module blueprints, per-codebase
compliance pinning, CI validation, lifecycle skills). The structural
difference limiting transfer: a2ui is a multi-party wire protocol with N
independent implementations, so much of its machinery keeps strangers
honest; cairn is a single Rust implementation whose surfaces share one
query registry. Lessons were refuted or reshaped wherever they
cargo-culted that burden.

## Adapted (verified against cairn source, with corrections)

### 1. Hook block output is a remediation dead end (adapt, small)

A2UI standardizes a machine-readable error format explicitly so the LLM
can self-correct. Verification refuted the proposed fix (a new per-code
hint table) as duplicate machinery: cairn already has the remediation
engine (`src/query_api/handlers/remediate.rs` mapping CAIRN_* codes to
prioritized actions with exact commands, exposed as `cairn remediate`,
`next`, `brief`, and the `cairn_remediate` MCP tool, plus
`QueryError.remediation` at `src/query_api/mod.rs:159`). The verified
residual gap: `src/hooks/render.rs::render_human_verbose` lists blocking
findings with no pointer to any of it, so a blocked agent must already
know the engine exists. Fix: on `ExitDecision::Block`, append a
copy-registry footer pointing at `cairn remediate --json`, or inline the
top actions for the blocking codes. Follow-up:
`todo.hook-block-remediate-pointer`. Adjacent: the action description
strings are separately tracked for copy centralisation
(`todo.remediate-copy-centralisation`).

### 2. Nothing ever prompts decision compaction (adapt, small)

A2UI merges feature blueprints into module blueprints and archives them,
with compaction as stated policy
(`specification/proposals/spec_driven_development.md:143`). Cairn has the
full supersession machinery (DecisionStatus::Superseded, `supersedes`
links at `src/artefacts/registry/types.rs:133`, accepted-only default
payloads in `src/query_api/handlers/bundle.rs:34` and `node.rs:111`,
superseded-still-counts-as-provenance guarded by
`src/scanner/tests.rs:362`) but no trigger: 67 of 68 dogfood decisions are
still accepted; cairn.root carries 19, cairn.kernel.cli 14, cairn.ui 11,
each dragged into every default bundle. Verifier corrections: the primary
remediation is a consolidating decision (one new decision superseding N
stale ones), not folding into contracts, which have a soft size limit
(spec section 13) and would just relocate the unbounded growth; and the
check is deterministic and small (count per node, Info tier). Caveat
found: `meta/decisions/revisit-trigger-relevance.md` uses a
`superseded_by` frontmatter field the parser does not read; the canonical
direction is the superseding decision's `supersedes` list. Follow-up:
`todo.decision-accumulation-finding`.

### 3. Contract-prose-vs-blueprint-shape staleness is uncovered (adapt, small)

A2UI pins codebase blueprints to a module-blueprint commit hash so owed
work is one `git diff`. Verification found the proposed mechanism (git
hash in contract frontmatter) violates cairn's spec twice: contracts are
purely human-authored with machine state in `.cairn/state/`
(docs/spec.md:338), and the scanner is deliberately git-free. Two of the
three staleness directions are already covered: code-vs-contract by the
interface hash freshness rule (docs/spec.md:342), blueprint-vs-decision by
CAIRN_BLUEPRINT_CHANGE_NO_DECISION backed by BlueprintSnapshot and
NodeFingerprint (`src/scanner/state.rs`, `checks.rs`). The uncovered
direction: contract prose authored against an old node shape stays
silently current. Fix with existing machinery: record the node's
NodeFingerprint as a baseline when the contract's interface hash is
recorded (`src/summariser/accept.rs`), compare on scan, emit Warning
naming the changed fields. Content-based, so formatting-only edits cannot
false-positive. Follow-up: `todo.contract-blueprint-staleness`.

### 4. About 20 finding codes ship with no triggering test (adapt, small)

A2UI's conformance suites pin what schemas must reject, with the vector
format itself schema-validated. The proposed declarative corpus was
refuted as redundant for a single-implementation project: negative cases
with typed expected codes already exist (`tests/blueprint_parser.rs`,
`src/artefacts/registry/validate/tests.rs` with 40 code-asserting tests),
and the spec-rules registry plus `src/map/spec_rule_coverage.rs` (CK004)
already enforce rule-to-emitter coverage as a scan finding. The verified
residue: of 103 distinct emitted CAIRN_* codes, about 20 (IO and plumbing
tiers) have no test that triggers them, and nothing fails when a new code
ships untested. Fix: a meta-test diffing emitted codes in `src/` against
asserted codes in tests, with a documented allowlist burned down over
time. Overlap: `todo.error-codes-registry-completeness` tracks the
sibling guard (emitted code with no registry entry); the two audits
share a scan of emitted codes and should land together or share a
helper. Follow-up: `todo.finding-code-test-coverage`.

### 5. The example corpus exists but has no anti-rot loop (adapt, small)

A2UI ports one numbered example corpus through every spec version as
docs, CI fixture, and prompt material at once. Roughly 70 percent already
exists in cairn: `examples/demo/` (full living project guarded by
`tests/phase_10_distribution.rs`), `tests/fixtures/reconcile_baseline/`
(committed expected baselines), insta wire-format snapshots
(`tests/wire_format_snapshots.rs`), and the dogfood CI gate
(`scripts/dogfood.sh`). A new fixture corpus would be a third parallel
surface with the same rot risk. The verified gaps: `examples/demo` is
asserted for existence but not against an expected finding set, and
`test/fixtures/cairn-bootstrap/` is only smoke-parsed (CLAUDE.md already
admits it "may lag behind root"). Fix: run `cairn scan --json` against
both in a test and assert committed expected-findings JSON, forcing the
lagging fixture to be fixed or deleted. Follow-up:
`todo.example-corpus-scan-assertions`.

### 6. Typo'd tags silently disable behavior (adapt, small)

A2UI's catalog is a closed declared vocabulary where typos fail fast.
Cairn tags are freeform strings the parser merely collects
(`src/blueprint/parser.rs:93-97`), yet three code paths key behavior off
exact strings: `no-contract` (`src/map/contract_coverage.rs:19`),
`no-test-coverage` (`src/map/test_coverage.rs:17`), and
MatchRule::HasTag (`src/brownfield/templates.rs:95`). A typo'd exemption
tag silently fails to exempt, and the cairn-dev skill (line 131) wrongly
documents tags as purely informational. Verifier corrections: keep the
vocabulary open (opt-in `tags:` section in cairn.config.yaml, following
the summariser/gates config precedent, not new blueprint grammar);
severity must be Info because `scan --strict` fails on Warning; allocate
a CK-series code in the registry rather than inventing a name. Seed with
the root repo's tags, flagging the two behavior-affecting ones.
Follow-up: `todo.tag-registry`.

### 7. Bundle does not surface the real gates (adapt, medium)

A2UI codebase blueprints carry per-codebase test/lint/format commands and
its AGENTS.md bans guessed build sequences. Verification found the
sharper gap upstream of the per-node proposal: `cairn bundle` and `brief`
do not surface even the global configured gates. All three call sites
(`src/query_api/handlers/bundle.rs:93`, `src/cli/render/bundle.rs:74`,
`src/cli/render/remediate.rs:370`) emit static text from
`docs/design-system/copy.toml` whose [brief] gates entry still claims
gates do not live in cairn, contradicting the shipped `gates:` config
feature (PR #340). Fix first: thread the actual gate recipe (config gates
or battery selection from `src/cli/accept/gates.rs`) into the
bundle/brief render paths and rewrite the stale copy. Per-target gate
overrides are a legitimate second step only when a polyglot consumer
materialises, with supplement-not-replace precedence decided explicitly.
Overlap: `todo.agent-context-bundle` (investigation of a composed
implementer bundle) is the natural home for the second step; the stale
copy fix should not wait for it. Follow-up: `todo.bundle-real-gates`.

### 8. Wire formats have version stamps but no shapes (adapt, medium)

A2UI treats machine-readable schemas as normative and re-proves
implementations against them in CI
(`renderers/web_core/src/v0_9/schema/verify-schema.test.ts`).
Verification found cairn's gap worse than proposed:
`docs/integration-contract.md` promises envelope stability with no
mechanism; SCHEMA_VERSION is stamped (`src/query_api/mod.rs:54`,
`src/scanner/snapshot.rs:17`) but shapeless; the registry's
response_schema fields are bare string labels and only 2 of about 30
named response types exist as structs, the rest ad-hoc `json!` literals a
field rename ships through silently. Corrections: schema generation from
serde derives is impossible for the `json!` responses, so hand-maintain
schema files or accept a large refactor; scope to what external consumers
parse (map.json first, which is proper serde structs; then the Finding
wire shape and envelope); make each registry response_schema label
resolve to a schema file via a registry test; jsonschema as
dev-dependency only. Follow-up: `todo.wire-format-schemas`.

### 9. No measurement of whether models can author cairn's formats (adapt, medium)

A2UI scores LLM-generated payloads with the exact production validator
(`eval/a2ui_eval/scorers.py`), and that measurement drove its v0.9
prompt-first schema rewrite. Cairn's blueprint syntax, blueprint.delta,
and artefact frontmatter are increasingly agent-authored (init
--from-code, draft family, gap, change authoring) with zero measurement
of valid production. Verifier corrections: the primary metric for cairn
is convergence cost (iterations and tokens to a clean scan under the
deterministic repair loop), not a2ui's one-shot validity, which fits a
streaming UI setting; and drop the heavyweight apparatus (daily CI, auto
issue filing, encrypted datasets) until the harness is scheduled rather
than on-demand. Session extension: the same harness supports a second
family, navigation evals: task prompts with ground truth extracted
deterministically from map.json ("which node owns file X", "what
decisions affect Y"), scoring steps and tokens for an agent using cairn
commands against a grep-only baseline. That family overlaps
`res.codeatlas-analysis` finding 2 and `todo.agent-guidance-baseline`, which
propose A/B navigation benchmarks with pinned SHAs; one shared runner serves
both families rather than two harnesses.
Follow-up: `todo.blueprint-authorability-eval`.

## Refuted (recorded so they are not re-proposed)

### Frozen side-by-side spec versions with evolution guides

The substance exists: docs/spec.md carries a per-version changelog (v0.1
to v0.8); state files mandate integer versions plus migration chains
(docs/conventions.md section 3); docs/integration-contract.md declares
stability tiers and a breaking-change policy for the surfaces external
consumers parse; map.json leads with schema_version. Frozen spec siblings
would plant live-looking pre-phase-2.6 terminology (DSL, ontology) in
docs/, the exact stale-context hazard CLAUDE.md guards against. Revisit
only at the first breaking change to an externally consumed format, and
that gate is already encoded in the integration contract.

### Layered agent instruction payloads (tight core plus on-demand packs)

Already implemented, and the lesson mischaracterized the repo:
`src/cli/agent_guide.md` is a 79-line always-on core (not a monolith; it
covers neither drafts nor brownfield), and `cairn init` emits
per-workflow recipe-card skills via include_str!
(`src/cli/commands/project.rs`: cairn-explore, propose, apply, archive,
plus cairn-dev with three on-demand references), pinned by
`test_init_project_emits_cairn_skills_and_loop_guide`. The one residue is
that brownfield onboarding has no recipe card; that is feature coverage,
not layering.

## Considered and deliberately not imported

- Flat adjacency-list blueprint syntax: blueprint.delta already gives
  ID-keyed order-independent mutation; lexical nesting keeps ownership
  visible to humans.
- Commit-hash pinning as the primary drift mechanism: strictly weaker
  than the content-addressable interface fingerprint (spec section 3.5),
  and misfit for a single whole-graph blueprint file.
- Ignore-by-default isolation of the blueprint layer: contradicts
  map-as-default-context.
- Multi-language SDK codegen, per-language YAML conformance harnesses,
  pixel-parity renderer testing, wire-level per-message version routing:
  multi-implementation machinery with no second implementation to keep
  honest.
- Catalog-agnostic $ref envelope swapping: no multi-vocabulary envelope
  exists; the Reconciler trait is the pluggability seam.
- LLM-as-judge second-stage scoring: premature before any programmatic
  authorability eval exists.
- Proposals directory with runnable prototype DSLs: the change system
  plus the Declared/Designed/Implemented ladder already provide
  incubation.
- Transcrypt dataset encryption and repo-hygiene bots: operations
  tooling, only meaningful downstream of an eval that runs unattended.

## Follow-on findings surfaced during verification (cairn-internal)

### Deferral language audit

Concern: repo language may steer agents toward human deferral on
deterministic actions. Audit result: the surfaces agents load are clean.
`src/cli/agent_guide.md` and AGENTS.md contain zero occurrences of
"human"; the skills only use the `review_type: human` enum;
docs/agent/cairn-dev-workflow.md is well calibrated (bans the "it is
blocked, you decide" reflex, splits knowable gaps from true external
blockers). The drift is in docs/spec.md, whose own section 2 framing is
correct ("the actuator is deliberately external (a human or an agent,
never cairn)", spec:49) but which later shortens "a human or an agent"
to "the human" for agent-capable or deterministic actions: spec:612-613
(change flow ends "the human runs cairn archive"; the cairn-archive
skill exists for agents), spec:777 ("the human remains the ultimate
authority over contract content", contradicting spec:771 four lines
above), spec:63 (provenance correctness "human judgment calls"; the
load-bearing point is cairn never makes them), spec:252 (init ignore
confirmation), spec:817-824 (brownfield "human refines"). Minor:
remediate.rs:224 "must be fixed manually" means "by editing, no command
exists". Fix is mechanical alignment with spec section 2's own actuator
framing. Follow-up: `todo.spec-authority-retirement`.

### Findings as tasks: unify the projection, not the artefact

Assessed whether findings should occupy the same shape as a task. Cairn
already has most of the right structure: findings are deliberately
ephemeral error signals (controller framing, spec section 2);
remediation actions are already task-shaped (priority, verb, command,
nodes); and `cairn next` is already the unified front door
(`src/cli/render/remediate.rs:100`: dirty project yields top remediation
action, clean yields top open native todo, then top ready bead, per
dec.native-todos-first). Materializing findings as durable todo
artefacts is rejected: a finding that persists after the drift is fixed
is a desync bug factory. Two verified gaps: (1) `cairn status` disagrees
with `cairn next`: `status_json` computes next_recommended from the
beads backlog only (`src/query_api/handlers/project.rs:31`), ignoring
findings and native todos, so the status answer contradicts next's
priority order (distinct from the fixed todo.status-active-changes-bug,
which covered active_changes). (2) Remediation actions, todos, and beads
each serialize with different field names; a shared work-item projection
(source, title, node, command, rank) in status/next/remediate --json
would give agents one queue vocabulary without touching the underlying
artefacts. Follow-up: `todo.next-recommended-unification`.

### The spec's role for agents

Question raised: should agents still read docs/spec.md; is the blueprint
the spec now? Assessment: the blueprint and the spec are different
layers, not competitors. The blueprint plus typed artefacts are the
machine-reconciled current-truth layer; the spec holds what the graph
cannot yet: unbuilt phases (Declared), open questions, design rationale
narrative, the changelog. The spec is actively maintained (touched
2026-07-16) but it is not reconciled prose: res.spec-designed-audit
proved a Designed-but-unbuilt rule can rot silently, which is why the
spec-rules registry plus CK004 coverage check exist. The problem is
CLAUDE.md's routing: "Read this first for any architecture question"
sends agents to a 961-line narrative when the same file's cairn section
already names `cairn context` as the agent entry point, and the deferral
language above makes the spec the one agent-visible surface with
human-lean wording. Suggestion (three parts, wording pass plus routing,
no structural change): (1) route architecture questions through the
graph first (cairn context, get, rationale) and position the spec as
design rationale and future intent, read when the graph lacks an answer;
(2) apply the actuator wording alignment above so the spec is safe agent
reading where it is reached; (3) continue the existing extraction
discipline: every normative rule the spec adds lands as a spec-rules
registry row with an enforcing finding code, per the CK004 machinery, so
the enforced surface keeps migrating out of prose. Not proposed: freezing
or splitting the spec (refuted above) or demoting it for humans.
Follow-up: `todo.spec-authority-retirement`.

## Overlap declarations

- `todo.error-codes-registry-completeness`: sibling guard to finding 4;
  land together or share the emitted-code scan.
- `todo.agent-context-bundle`: natural home for finding 7's second step
  (per-target gates in a composed bundle); the stale-copy fix proceeds
  independently.
- `todo.remediate-copy-centralisation`: adjacent to finding 1; the hook
  footer should use the copy registry from day one.
- `res.codeatlas-analysis` and `todo.agent-guidance-baseline`: the navigation
  eval family in finding 9 is the same idea; one shared runner serves both.
  Declared here to prevent parallel harnesses.
- `todo.status-active-changes-bug` (done): distinct from the
  next_recommended gap in the findings-as-tasks section.
