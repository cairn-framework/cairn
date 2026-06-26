# AI Pipeline

**Source:** https://www.getcairn.dev/docs/ai-pipeline
**Captured:** 2026-04-28

Every Command+K command flows through a 5-stage pipeline before reaching your review screen.

1. Router (Haiku)
2. Context Assembly
3. Specialist (Sonnet)
4. Validator
5. User Review

## Specialists

The router selects the right specialist based on your request:

| Specialist | Triggers | Description |
|---|---|---|
| **Architect** | Decompose, add subsystems, restructure | Functional and physical decomposition. Creates 3 to 6 children with property seeding and interface suggestions. Max 16K tokens. |
| **Requirements** | Generate requirements, add constraints | Requirements elicitation from node descriptions. Generates functional, performance, and safety requirements with acceptance criteria. |
| **Interfaces** | Define interfaces, add signals | Interface and signal definition between nodes. Identifies data flows, power buses, command channels, and protocol selection. |
| **Behavior** | Define states, create state machine | State machine design with guards, actions, and transition conditions. Generates operating mode state machines. Max 12K tokens. |
| **Verification** | Add tests, verification methods | Verification planning. Test, analysis, demonstration, inspection methods linked to requirements. |
| **Brief** | Generate brief, document system | Narrative documentation generation with 8 sections: purpose, capabilities, modes, architecture, interfaces, constraints, verification, risks. |
| **Causality** | Analyze technology readiness | AI-assisted maturity analysis based on Harney's Technology Evaluation framework. Returns narrative assessment, not model changes. |
| **Narrative** | Generate systemigram narrative | Identifies mainstay transformation chain, generates verb phrases for connections, classifies connection roles, and produces prose narrative cards. |
| **Dendritic** | Generate alternatives, trade-off analysis | Generates 1 to 3 plausible pruned alternatives with engineering prune reasons, first principles, and cross-branch dependencies. Standard ChangeSet governance. |
| **General** | Catch-all for unclassified requests | Fallback specialist for requests that don't match a specific domain. Routes to the most appropriate action. |
