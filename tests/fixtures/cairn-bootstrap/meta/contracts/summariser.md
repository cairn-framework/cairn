---
node: cairn.summariser
informed_by:
  - type: decision
    id: dec.contradiction-classes
---

# Contract: cairn.summariser

The Summariser is the optional LLM callout: when the scanner surfaces an interface contradiction, it drafts a proposed contract update for a human to accept or reject. The module is tagged `@optional` and the system is complete without it.

## Interface

- **Input.** An interface contradiction finding plus the contract text it contradicts.
- **Output.** A proposed contract amendment, clearly marked as a proposal; never an applied edit.
- **Errors.** An unavailable or failing model degrades to silence: the finding stands unchanged and nothing is proposed.

## Invariants

- Proposals are advisory. Acceptance is a human act; the summariser never writes contract files itself.
- Absence of the module changes no scan verdict: findings and severities are identical with and without it.
- Every proposal names the finding that motivated it.

## Out of scope

- Detecting contradictions. The scanner does; the summariser reacts.
- Deciding severity. dec.contradiction-classes fixes the classes; the summariser operates downstream of them.
