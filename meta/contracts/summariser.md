---
node: cairn.summariser
---

# Contract: cairn.summariser

## Purpose

Phase-8 LLM-assisted summarisation backend for `cairn summarise`. Owns the
pluggable backend trait, the typed local-command request/response protocol, the
prompt builder that grounds requests in live project state, the generation
orchestrator, and the filesystem draft store with its accept workflow. The
framework ships here; concrete hosted provider adapters land in later phases, so
the default mode is disabled and refuses every invocation.

## Public interface

- `backend`: `SummariserBackend` trait (per-call `timeout` obligation),
  `SummariserMode`, `SummariserBackendError`, and implementations
  `DisabledBackend`, `FakeBackend`, `LocalCommandBackend`, `HostedBackend`
  (with `HostedConfig`).
- `request`: `SummariserRequest`, `SummariserResponse`, `CodeSample`, and
  `SUMMARISER_SCHEMA_VERSION`; all wire structs use `deny_unknown_fields`.
- `prompt`: `build_request` plus `PromptError`, assembling map facts, contract
  excerpts, findings, rules, and bounded code samples under byte limits.
- `generate`: `generate` plus `GenerateError`, invoking a backend and persisting
  the response as a `PendingDraft`.
- `store`: `DraftStore`, `Draft` (status-tagged enum of `PendingDraft`,
  `EditableDraft`, `AcceptedDraft`, `DiscardedDraft`), `DraftStatus`,
  `DraftHeader`, `TransitionRecord`, `validate_transition`, `read_draft`,
  `DRAFT_SCHEMA_VERSION`, and errors `DraftStoreError`, `DraftTransitionError`,
  `EmptyInterfaceHash`.
- `accept`: `accept` plus `AcceptError`, applying a draft to its contract.
- `config`: `SummariserSettings` loaded from `cairn.config.yaml`.
- `assert_draft_tools_registered`, `assert_draft_tool_safety_classes`: startup
  safety assertions tying draft tools to the query-api registry.

## Invariants

- Disabled is the default mode and refuses every call; absent config yields it.
- The draft lifecycle is illegal-state-unrepresentable: each `Draft` variant
  carries only its valid fields, `Accepted` and `Discarded` are terminal, and
  `validate_transition` rejects outgoing transitions from terminal states.
- `AcceptedDraft` always carries a non-empty interface hash, enforced through a
  private field plus a `TryFrom` shim so corrupt payloads are rejected on load.
- `DraftStore::write` never clobbers an existing draft (returns
  `DraftStoreError::Conflict`); replacement requires the explicit `overwrite`.
- Backends must honour the per-call `timeout` and return
  `SummariserBackendError::Timeout` when the deadline elapses.
- Wire drift fails loudly: `deny_unknown_fields` on every request/response struct.

## Dependencies

Leaf node with no outgoing blueprint edges. Internally `prompt` reads graph,
config, contract, and metadata state to build requests, and `accept` re-parses
the blueprint AST and runs a post-write scan, but the module exposes no edges to
sibling modules in `cairn.blueprint`.

## Tests

Unit tests are colocated per submodule: `backend/tests.rs`, `prompt/tests.rs`,
inline `#[cfg(test)] mod tests` in `request.rs`, `accept.rs`, `config.rs`,
`generate.rs`, `store.rs`, and `mod.rs`. They cover backend timeout and disabled
refusal behaviour, request/response serde with unknown-field rejection, prompt
grounding and byte-limit truncation, generate persistence, draft lifecycle
transitions, the non-empty-hash invariant and conflict handling, accept-path
contract replacement, and config defaulting.
