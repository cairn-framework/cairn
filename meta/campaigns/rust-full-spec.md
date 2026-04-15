# Campaign: Cairn Full Product Specification (Rust)

## Objective

Create complete OpenSpec changes for all phases of the Cairn project, targeting Rust as the implementation language. Each phase is a separate OpenSpec change with proposal, specs, design, and tasks — written to a standard that a headless AI agent (Codex) can implement autonomously via cflx.

This is a full product specification, not a prototype. Every phase produces production-grade, fully tested Rust code. No shortcuts, no placeholders, no "good enough for now."

## Language Target: Rust (Strictest Mode)

### Crate-level attributes (required in every lib.rs / main.rs)

```rust
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![forbid(unsafe_code)] // unless explicitly justified per-function with a safety comment
```

### Git pre-commit hooks (Phase 0 sets these up)

```bash
#!/bin/sh
# .git/hooks/pre-commit
set -e
cargo fmt --check
cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery
cargo test
```

### Every phase's tasks.md must include verification steps

```markdown
- [ ] N.X `cargo build` passes with zero warnings
- [ ] N.X `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` passes
- [ ] N.X `cargo fmt --check` passes
- [ ] N.X `cargo test` — all tests pass
```

These are not optional. They are acceptance criteria. The cflx acceptance gate will fail if these are unchecked without evidence.

## cflx Configuration

Agent: `codex exec` (headless, non-interactive). Codex uses git natively — git commit hooks enforce quality gates on every commit during apply.

```jsonc
{
  "apply_command": "codex exec '/openspec:apply {change_id} {prompt}'",
  "acceptance_command": "codex exec '/cflx-accept {change_id} {prompt}'",
  "archive_command": "codex exec '/openspec:archive {change_id} {prompt}'"
}
```

## Phase Map

Derived from docs/spec.md section 14. Each phase is one OpenSpec change.

| Change ID | Phase | Spec Sections | Depends On |
|-----------|-------|---------------|------------|
| `phase-0-foundation` | 0: Rust project setup | N/A | None |
| `phase-1-kernel` | 1: Kernel | 6, 7, 10 (partial), 12 (partial) | phase-0 |
| `phase-2-artefacts` | 2: Full artefact types | 8 | phase-1 |
| `phase-3-changes` | 3: Change system | 9 | phase-2 |
| `phase-4-hooks` | 4: Hooks | 11 | phase-3 |
| `phase-5-edges-docstrings` | 5: Edge validation & docstrings | 12 (full) | phase-4 |
| `phase-6-multi-target` | 6: Multi-target & languages | 7 (list paths) | phase-5 |
| `phase-7-mcp` | 7: MCP wrapper | 12 (MCP) | phase-6 |
| `phase-8-summariser` | 8: Summariser | 13 | phase-7 |
| `phase-9-brownfield` | 9: Brownfield extraction | 15 | phase-8 |
| `phase-10-distribution` | 10: Distribution | 14 (10+) | phase-9 |

### Phase 0: Foundation (not in the original spec — infrastructure)

This phase exists to establish the Rust project skeleton and quality tooling before any domain code. It produces:

- `Cargo.toml` with workspace structure
- `src/lib.rs` and `src/main.rs` with strict lint attributes
- Git pre-commit hook enforcing fmt + clippy + test
- CI-equivalent verification tasks
- Empty test harness confirming the pipeline works
- `.gitignore` for Rust targets
- The `test/fixtures/` DSL files wired as test resources

No domain logic. Just the foundation that every subsequent phase builds on.

### Phase 1: Kernel

DSL parser (hand-written recursive descent), in-memory graph (ontology), code reconciler interface (trait-based), basic scanner (path existence checks only), CLI with all query commands. Tree-sitter integration for the code reconciler's initial language (Rust or TypeScript — spec says either). Generated outputs: `index.md` and `.cairn/log.md`.

This is the largest phase. Consider splitting into sub-changes:
- `phase-1a-parser` — lexer, parser, AST types
- `phase-1b-graph` — ontology builder, integrity checks
- `phase-1c-cli` — CLI commands, output formatters
- `phase-1d-scanner` — reconciler trait, code reconciler, scanner loop

### Phase 2: Full Artefact Types

Todos, decisions (ADR format), reviews (with agent subtypes), research, sources — all with integrity rules, link validation, and CLI commands.

### Phase 3: Change System

Change directories, delta semantics (ADDED/MODIFIED/REMOVED), archive command, rename command with atomic propagation, change-aware queries.

### Phase 4: Hooks

Structural hooks, interface hooks, tension hooks. Pre-commit gate integration. Conflict detection between concurrent active changes.

### Phase 5: Edge Validation & Docstrings

Deeper Tree-sitter analysis or LSP integration. Verify declared edges match real imports. `cairn docstring <node>` command. Drift detection.

### Phase 6: Multi-target

Modules with path lists reconciled across targets. Per-target interface hashes. Additional language reconcilers.

### Phase 7: MCP Wrapper

Wrap CLI queries as an MCP server. Compose project context and rules into query responses. Direct agent integration.

### Phase 8: Summariser

Optional LLM-powered component. Pluggable backends. Three-action resolution (accept/edit/discard). Drives brownfield and docstring generation.

### Phase 9: Brownfield Extraction

`cairn init --from-code` and `cairn refine`. Reconciler extracts structural candidates, summariser names and describes, human refines.

### Phase 10: Distribution

LSP server for editors. Claude Code plugin packaging. Additional reconcilers for non-code domains.

## Quality Protocol

Every phase goes through this pipeline before being committed:

```
WRITE → REFORGE → DEBATE → (ITERATE if needed) → FINALIZE
```

### Step 1: Write

Create the OpenSpec change directory with all artifacts:
- `proposal.md` — why this phase exists, what it changes
- `specs/<capability>/spec.md` — WHEN/THEN scenarios for each capability
- `design.md` — Rust-specific technical decisions, crate structure, trait design
- `tasks.md` — implementation checklist including verification steps

Guidelines for writing specs that produce good Codex output:
- Be explicit about Rust idioms: `Result<T, E>` for errors, `impl Trait` for interfaces, `#[derive]` for data types
- Name the exact modules/files to create
- Include the exact CLI commands and their expected output format
- Every task should be independently verifiable
- Include negative test cases (error paths, malformed input)
- Reference docs/spec.md section numbers for authority

### Step 2: Reforge (/reforge)

Run /reforge on the change directory. The reforger checks for:
- Redundancy between proposal and design
- Vague or un-testable requirements in specs
- Tasks that are too large or too abstract
- Missing verification steps
- Unnecessary complexity

The reforger writes directly — it edits the files.

### Step 3: Debate (/debate — file-based)

Adversarial review with written output to keep orchestrator context lean.

**Protocol:**

1. **Spawn attacker agent** — reads the phase's OpenSpec change, writes critique to:
   `meta/debates/phase-N/round-1-attack.md`

2. **Spawn defender agent** — reads the change + attack, writes defense to:
   `meta/debates/phase-N/round-1-defense.md`

3. **Spawn adjudicator agent** — reads attack + defense, determines:
   - `CONVERGED` — disagreements are resolved or cosmetic
   - `DIVERGENT` — substantive issues remain, another round needed

   Writes verdict to: `meta/debates/phase-N/round-1-verdict.md`

4. **If DIVERGENT**, repeat with round-2-attack.md, round-2-defense.md, round-2-verdict.md.
   Maximum 3 rounds.

5. **Final summary** — adjudicator writes `meta/debates/phase-N/final-summary.md` with:
   - Accepted critiques (changes to make)
   - Rejected critiques (with reasoning)
   - Overall quality assessment

The orchestrator reads ONLY `final-summary.md` and applies findings.

**Attacker focus areas:**
- Will Codex actually be able to implement this from the spec alone?
- Are there ambiguities that will cause the agent to guess wrong?
- Are Rust-specific concerns addressed (ownership, lifetimes, error handling)?
- Are the tasks ordered by dependency correctly?
- Do the specs have testable scenarios with concrete inputs/outputs?
- Is anything missing from the spec that the phase claims to deliver?

**Defender focus areas:**
- Is the critique valid or nitpicking?
- Would the suggested change actually improve implementation quality?
- Does the attacker understand the Rust ecosystem and idioms correctly?
- Are there practical constraints the attacker is ignoring?

### Step 4: Iterate

Apply accepted findings from the debate. If changes are substantial, run /reforge again. If minor, apply directly and move to finalize.

### Step 5: Finalize

Commit the change. Verify with `openspec validate`. Move to next phase.

## Execution Sequence

Process phases strictly in order. Each phase's spec may reference code/structures from prior phases.

```
Phase 0 → write → reforge → debate → finalize → commit
Phase 1 → write → reforge → debate → finalize → commit
  (or Phase 1a → 1b → 1c → 1d if splitting)
Phase 2 → write → reforge → debate → finalize → commit
...
Phase 10 → write → reforge → debate → finalize → commit
```

After all phases are committed, the full set of changes lives in `openspec/changes/` ready for `cflx tui` to execute.

## Agent Dispatch Template

For each phase, the orchestrator dispatches these agents:

### Writer (foreground — need result to proceed)
```
Agent(subagent_type="general-purpose", description="Write phase N spec")
Prompt: "Read docs/spec.md sections [X, Y, Z] and create the OpenSpec change
for phase-N-{name}. Target language: Rust. Follow the campaign protocol at
meta/campaigns/rust-full-spec.md for structure and strictness requirements.
Create: proposal.md, specs/, design.md, tasks.md under openspec/changes/phase-N-{name}/"
```

### Reforger (foreground)
```
/reforge on openspec/changes/phase-N-{name}/
```

### Debate — Attacker (background)
```
Agent(description="Phase N debate: attack", run_in_background=true)
Prompt: "Read the OpenSpec change at openspec/changes/phase-N-{name}/ and
the reference spec at docs/spec.md. Write an adversarial critique focusing on
implementability by a headless AI agent (Codex) targeting Rust. Write your
critique to meta/debates/phase-N/round-1-attack.md"
```

### Debate — Defender (after attacker completes)
```
Agent(description="Phase N debate: defend", run_in_background=true)
Prompt: "Read the OpenSpec change at openspec/changes/phase-N-{name}/,
the reference spec at docs/spec.md, and the attack at
meta/debates/phase-N/round-1-attack.md. Write a defense.
Output to meta/debates/phase-N/round-1-defense.md"
```

### Debate — Adjudicator (after defender completes)
```
Agent(description="Phase N debate: adjudicate", run_in_background=true)
Prompt: "Read round-1-attack.md and round-1-defense.md in meta/debates/phase-N/.
Determine CONVERGED or DIVERGENT. Write verdict to round-1-verdict.md.
If CONVERGED, also write final-summary.md with accepted/rejected critiques."
```

## Language Rules

- Never use the word "MVP" in any OpenSpec artifact. This causes sloppy implementation behavior in LLMs.
- Use "production", "complete", "fully specified" instead.
- Every phase is a complete unit of work, not a partial or minimal implementation.
- Specs should use definitive language: "SHALL", "MUST", not "should", "could", "might".

## Reference Materials

- `docs/spec.md` — Cairn v0.6 specification (authoritative)
- `docs/dsl.md` — DSL grammar reference
- `test/fixtures/cairn.dsl` — example DSL file
- `test/fixtures/cairn-bootstrap/` — Cairn described as a Cairn project
- `openspec/changes/archive/cairn-kernel-mvp/` — historical reference (TS implementation, archived)

## Post-Campaign

After all phases are written, reviewed, and committed:
1. Ensure git working tree is clean
2. Run `cflx tui`
3. Select phases in order
4. F5 to execute
5. Monitor progress — each phase goes through apply → accept → archive

The phases are designed to be executed sequentially by cflx. Phase 0 creates the foundation, each subsequent phase builds on the prior one's output.
