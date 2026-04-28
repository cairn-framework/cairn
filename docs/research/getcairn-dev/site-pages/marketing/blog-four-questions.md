# Four Questions Your Model Can't Answer

**Source:** https://www.getcairn.dev/blog/four-questions
**Captured:** 2026-04-28

Every systems engineer instinctively asks structural questions about their system's readiness, completeness, coherence, and reasoning history, yet no existing tool surfaces these inquiries effectively.

## The Four Analytical Lenses

### Question One: Causality Lens

**"What must exist before this can be realized?"**

This lens inverts the traditional decomposition tree to reveal prerequisite technologies and foundational dependencies. Each layer is color-coded by maturity level (green for mature, amber for growth phase, red for early development). 

In the Autonomous Delivery Rover example, the Nav Computer appears mature in the architecture tree, but the Causality Lens exposes that ML-based path planning exists only at TRL 5 to 6, revealing a software maturity risk invisible in standard hierarchical views.

### Question Two: Completeness Lens

**"What's missing from this model?"**

Model fidelity spans three independent dimensions:
- Entity coverage (sufficient child nodes)
- Process coverage (defined behaviors)
- Relationship coverage (interfaces and traced requirements)

The tool generates three-axis radar charts per node and heatmaps across the entire tree. A node might score 95% on entities but 0% on behaviors, indicating decomposition without functional definition.

### Question Three: Narrative Lens

**"What does this system actually do?"**

This lens identifies the primary transformation chain and renders it as a systemigram, a directed graph expressing the system's story as readable sentences rather than engineering diagrams.

### Question Four: Dendritic Lens

**"How did we arrive at this design?"**

This lens surfaces pruned alternatives as first-class model elements, each carrying its elimination rationale. The model preserves its intellectual history by making engineering judgment visible alongside surviving design paths.

## Convergence and Integration

These four lenses operate on the same underlying data structure. A single problematic node might trigger alerts across all four dimensions simultaneously (incompleteness, missing causality, narrative disconnection, and absent pruning history) converging on a unified diagnosis that no single view could achieve alone.
