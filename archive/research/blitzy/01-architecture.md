# Blitzy architecture: technical findings

Five load-bearing observations from the Cognitive Revolution interview with Brian Elliott (CEO) and Sid Pardeshi (CTO), cross-checked against Blitzy's product docs and the TWIML episode. Direct quotes are attributed; everything else is paraphrase from the same sources.

## 1. The "graph" is a multi-source evidence layer, not a code graph

Blitzy's context substrate is a hybrid that fuses:

- **Structural analysis**: language-agnostic, AST-shaped relationships among globals, classes, variables, and functions. Brian Elliott describes it as *"a so not an AST but resembles the characteristics of an AST from accuracy that is programming language agnostic designed for AI agent traversal"*.
- **Runtime telemetry**: from running the customer's application in a parallel cloud environment to capture actual execution relationships rather than just declared imports.
- **Production signals**: logs, load-time behaviours, request paths.

This spans the compile-time → runtime → production spectrum. Ingestion of a 100M-line codebase is a multi-day compute job that produces what they call "deep relational understanding" with line-level dependency precision.

**Mapping to CAIRN**: this is structurally identical to CAIRN's provenance chain (source → research → decision). Blitzy bolts evidence on at three altitudes; CAIRN's typed artefacts (`source`, `research`, `decision`) carry the same evidence-flowing-in role. The phase 7.6 ai-provenance-foundation work is on the same ladder.

## 2. RAG is a navigation layer, not the source of truth

Quote: *"Relational graphs guide traversal; actual code pulled just-in-time."*

The graph answers "what's relevant"; the agent fetches the actual content fresh at execution time. They explicitly avoid storing summaries as truth, because summaries drift and hallucinate when used as substitutes for real artefacts.

**Mapping to CAIRN**: this is the reconciler-against-actual-code pattern. Declarations describe; reconciliation pulls live truth. **This should anchor the phase 8 (summariser) design**: summarisers are navigation aids, never substitutes for artefact retrieval. Worth recording as an explicit constraint in the phase 8 design.md.

## 3. Orchestration is dynamic, not centralized

Quote (Brian Elliott): *"Everything that we do in Blitzy is dynamic design... agents are generated dynamically just in time."*

Specifically:

- Agents spawn other agents.
- Prompts for child agents are written by parent agents at runtime.
- Tool selection is dynamic, assessed via context injection rather than hardcoded.
- Each agent references the latest vendor prompting guidelines fresh, rather than a frozen harness.

The stated motivation is "harness depreciation": as foundation models improve, a static orchestrator harness becomes a bottleneck. Dynamic generation lets the system absorb new model capabilities without rewriting the harness.

**Mapping to CAIRN**: cflx is statically configured, so we will hit this ceiling eventually. A CAIRN-native orchestrator (post phase 10) needs graph-dispatched agents, not a fixed pipeline. The CAIRN substrate already supports this in principle: typed artefacts can carry routing metadata, and the kernel's reconciler-vs-declared discipline is the right primitive for graph-dispatched coordination.

## 4. Validation is layered and adversarial

Blitzy's QA stack:

1. Unit tests run before and after each file modification.
2. Integration tests between service clusters.
3. End-to-end tests with screenshots and agent UI interactions.
4. Conflict detection when tests oscillate (fail → pass → fail).
5. Independent evaluator system, structurally separate from the executor system.
6. "Report card" artifact summarising what was completed and what needs human input.

Item 5 is the architecturally significant one: the evaluator is not the same agent or the same harness as the executor. This is the multi-vendor reviewer pattern made formal.

**Mapping to CAIRN**: items 1-3 already match the verification gate battery (`cargo build`, `clippy -D warnings`, `fmt`, `test`, `test --locked`, `cflx.py validate --strict`). Item 4 (oscillation detection) is partly covered by cflx 0.6.46+'s error_circuit_breaker. Item 5 is the multi-reviewer acceptance pattern, currently a single opencode call. Item 6 (report card) is worth considering for phase 10 distribution: a per-change human-readable evidence summary that ships alongside the code.

## 5. Multi-vendor reviewer ensemble is structural, not cosmetic

Quote: *"Multiple models reviewing each other's work across vendor families (OpenAI, Google, Anthropic)."*

Vendor diversity is not a stylistic choice. Different vendor families have different blind spots; the ensemble does work because the blind spots do not overlap.

**Mapping to CAIRN**: when we eventually formalize the acceptance ensemble (deferred to post phase 10), vendor diversity needs to be wired in by default, not added later. Multi-prompt against one model produces correlated reviews and misses the point.

## 6. Memory lives at the application layer, not in model weights

Both founders emphasised that long-term memory belongs at the application/system layer, storing relational and semantic preferences. They explicitly avoid fine-tuning, on the grounds that custom weights become stranded as foundation models improve.

**Mapping to CAIRN**: this is already CAIRN's thesis. Artefacts in the framework, models stay disposable. This bit is reassurance, not redirection.

## What Blitzy has that CAIRN does not yet

- Runtime telemetry capture (running the application in a parallel sandbox to map true execution dependencies). Closest CAIRN equivalent would be a future scanner type that captures runtime evidence; not in scope for phases 7.6-10.
- Dynamic agent generation harness (cflx is static).
- Evaluator-separate-from-executor as a first-class architectural separation.

## What CAIRN has that Blitzy seems not to have

- A typed artefact kernel where AAP-equivalents are mechanically verifiable, not informally markdown. Blitzy's AAP is reviewable but not type-checked against a kernel schema.
- The two-chain topology with a hinge. Blitzy's evidence layer is rich, but the authority chain (decision → blueprint → contract → code) is not separately named in their architecture.
- A change-isolation primitive where AI proposals are reviewable before execution against a deterministic reality fingerprint.

## Source confidence

Items 1-2 (graph + RAG-as-navigation) and items 3 (dynamic orchestration), 5 (multi-vendor), 6 (memory) are direct quotes from founders. Item 4 (validation stack) is partly direct, partly extracted from the product docs `code-review` page that was pasted into chat 2026-05-04. The "report card" framing is paraphrase; exact format not specified.

## Related CAIRN reading

- `docs/spec.md` §3.4 (framework's role)
- `CLAUDE.md` "Architecture: two chains meeting at a hinge"
- `openspec/changes/phase-7.6-ai-provenance-foundation/`
- `openspec/changes/phase-8-summariser/` (apply the navigation-not-truth constraint here)
- `openspec/changes/phase-10-distribution/` (consider the report-card artifact here)
