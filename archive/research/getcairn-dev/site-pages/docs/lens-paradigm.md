# The Lens Paradigm

**Source:** https://www.getcairn.dev/docs/lens-paradigm
**Captured:** 2026-04-28

Most engineering tools create separate documents for separate concerns. You have a requirements document, an architecture diagram, a state machine drawing, and a verification matrix. Each in its own file, each with its own copy of the system structure, each drifting out of sync with the others.

Cairn takes a different approach: **one model, multiple lenses**.

## One Model

Your entire system (nodes, requirements, interfaces, states, verifications, trace links) lives in a single connected data structure. There's one source of truth, not five documents that each contain a partial copy.

When you rename a subsystem (click its title in the Inspector, or right-click it in the tree and choose Rename) the change propagates everywhere. When you add an interface between two nodes, it appears in the Architecture lens and the Brief's interface summary simultaneously. When you delete a requirement, every trace link pointing to it knows it's gone, and a cascade preview shows you those links before you confirm.

This isn't just convenience. It's what makes questions like "what depends on this?" and "what traces to that?" answerable at all. You can't compute dependencies across disconnected documents.

## Multiple Lenses

A lens is a perspective on that single model. Select a node, switch tabs, and see the same data rendered for different purposes:

- **Overview** shows the node's health, children, and summary metrics. Your dashboard.
- **Requirements** shows functional and performance requirements scoped to that node
- **Architecture** shows the node's children as a visual graph with interfaces
- **Behavior** shows the node's state machine
- **Verification** shows test coverage and results

The model doesn't change when you switch lenses. Only the rendering changes. You're not navigating to a different document, you're looking at the same thing from a different angle.

## Node-Scoped Everything

This is the key insight: _the tree IS your filter_.

When you select a node, every lens automatically scopes to that node and its subtree. You're not looking at "all requirements" and then filtering, you're looking at "requirements for this node." You're not looking at "the whole architecture" and then zooming, you're looking at "this node's children."

This makes large models navigable. A system with 200 nodes doesn't overwhelm you because you're only ever looking at one node's local context. Drill down by selecting children. Drill up by selecting parents. The lens shows what's relevant.

## Standard vs. Analytical Lenses

The standard lenses (Overview, Requirements, Architecture, Behavior, Verification) show data you explicitly created, the requirements you wrote, the states you defined.

The four analytical lenses compute insight from that same data:

- **Causality** computes what prerequisite technologies your system depends on
- **Completeness** computes what's missing from your model
- **Narrative** computes the main transformation chain through your system
- **Dendritic** surfaces the decision history and pruned alternatives

These aren't views you populate, they're questions the model answers.
