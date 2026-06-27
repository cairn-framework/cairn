# Completeness and causality

## What this is

Two analytical lenses on every node: a **three-axis fidelity radar** that scores how completely a node has been modelled (Entities, Processes, Relationships), and a **causal pyramid** that shows what the node is a prerequisite for and what depends on it. Both surfaces are quantitative, both pair the chart with a per-node detail breakdown in the side rail, and both include contextual book quotes that teach the user how to interpret the numbers.

For the chrome that frames these views, see [03-information-architecture.md](./03-information-architecture.md). For how scoring informs project-level chrome (the `Quality 73` chip), see also that file. For the scoring philosophy, see [09-design-influence.md](./09-design-influence.md#contextual-book-quotes-as-pedagogy).

## Three-axis fidelity radar

### Layout

The Completeness tab shows a triangular radar chart per node (**from screenshots 13, 14, 15, 20**). Three vertices (**from screenshot 13**):

- **Entities** (top). Coloured ochre/orange. Children defined.
- **Processes** (bottom-right). Coloured green. Behaviors defined.
- **Relationships** (bottom-left). Coloured purple. Interfaces and requirements defined.

Concentric guide rings at 25%, 50%, 75%, 100%. A purple data-point dot inside the triangle marks the node's current position.

Below the radar, three colour-coded summary cards repeat the per-axis percentages with a one-line metric:

```
ENTITIES 0%             PROCESSES 0%           RELATIONSHIPS 0%
0 children defined      0 behaviors defined    0 interfaces · 0 reqs
```

(**from screenshot 13**, USV Platform empty.)

For Power Generation & Distribution (**from screenshot 15**) the values shift to `Entities 0% / Processes 0% / Relationships 33%`, with the relationship-card metric showing `1 interfaces · 0 reqs`.

### Score formula (inferred)

The radar implies that "completeness" is multidimensional, not a single percentage. Each axis is its own ratio (children defined out of expected, behaviors defined out of expected, interfaces and requirements defined out of expected). The exact formulas for the expected denominators are **unknown**.

In the side rail (**from screenshots 15 and 16**), a fourth aggregated number `Score 11%` appears as `OVERALL COMPLETENESS`. This is **inferred** to be a weighted blend of the three axis percentages plus possibly children counts. The closed-source product gives no public formula.

What we can say with confidence: the per-axis scores are visible, the breakdown is exposed in the side rail, and the user is shown the contributing counts (Children, Depth, Behaviors, Interfaces, Requirements). The score is auditable in the sense that you can see where the points come from, even if the weighting is hidden.

### Side-rail breakdown panel

When a node is selected on the Completeness tab, a side-rail panel shows its full coverage breakdown (**from screenshots 15 and 16**, SUB-PWR):

```
SUB-PWR · Node                                                              X

  Generate Requirements

OVERALL COMPLETENESS  ──────────────────────
  ▰▱▱▱▱▱▱▱▱▱  Score 11%

ENTITY COVERAGE  ──────────────────────
  ▱▱▱▱▱▱▱▱▱▱
  Children   0
  Depth      1

PROCESS COVERAGE  ──────────────────────
  ▱▱▱▱▱▱▱▱▱▱
  Behaviors  0

RELATIONSHIP COVERAGE  ──────────────────────
  ▰▰▰▱▱▱▱▱▱▱
  Interfaces    1
  Requirements  0

CHILDREN  ──────────────────────  0
  No children, leaf node

  "Fidelity is multidimensional. A single metric is not very meaningful.
   A set of descriptions, entity coverage, process coverage,
   relationship coverage, are much more useful in determining
   what actions to take."
                                       Pace, Ch.3 (Loper 2015)

  Delete
```

Notable design choices:

- **Coverage is shown as a partial-fill bar plus a contributing-count list.** Both visual and numeric. (**from screenshot 16**.)
- **The Pace quote teaches the user to interpret the numbers.** It is rendered as a left-bordered italic block in a tinted card. The quote argues that a single metric obscures rather than informs. This is a UX choice: the chart is taught alongside the data. See [09-design-influence.md](./09-design-influence.md#contextual-book-quotes-as-pedagogy).

### Inline nudge banner

Below the radar and summary cards, a yellow-tinted banner appears with an `⚠` icon and a plain-language interpretation of the current state, plus a `Fix with AI →` button (**from screenshots 14 and 15**):

```
⚠  USV Platform is structurally and behaviourally defined but underconnected.
   0% relationship coverage means interfaces and requirements are thin.
                                                              [ Fix with AI → ]
```

```
⚠  Power Generation & Distribution has structural definition but lacks behavioural
   specification. Only 0% process coverage. The model knows what it is, but
   not what it does.
                                                              [ Fix with AI → ]
```

The banner copy is auto-generated and reads as if templated against the per-axis values: "structurally defined but X" where X is whichever axis is lowest. The exact templating is **unknown** but the pattern is consistent across the two captured cases.

This is one of the strongest UX touches in the product. The banner does three things at once:

1. Translates a numeric finding into prose.
2. Names the deficiency in domain language ("the model knows what it is, but not what it does").
3. Offers an AI-driven action to close the gap, in-place.

See [09-design-influence.md](./09-design-influence.md#fix-with-ai-inline-nudge-banners) for why this pattern is worth studying. See [08-borrow-list.md](./08-borrow-list.md#6-completeness-roll-up-with-prose-nudges-on-deficits) for the borrow rationale.

## Causal pyramid

### Layout

The Causality tab shows a per-node "causality pyramid" (**from screenshot 12**, USV Platform). For a node with no children, the pyramid is empty:

```
Causality (4)        ← chip with count 4 in the tab itself

    SUBSYSTEM
    USV Platform
    SUB-USV

    Provides the sea-surface vessel hull, propulsion, and station-keeping
    capability for the system. The USV is a hybrid diesel-electric vessel
    designed to transit in sea states up to SS6 and maintain station during
    ROV operations. It interfaces with the Power...   Show more

                              ◇

                  No prerequisite data to visualise
            Decompose USV Platform to reveal its causality pyramid

                       [ Decompose with AI ]
```

The "pyramid" shape is **inferred** to be a tiered diagram with the current node at one tier, prerequisites flowing up, dependencies flowing down (or vice versa). The captured screenshot shows the empty state; the populated form is **unknown**.

### Side-rail causal position panel

When a node is selected on the Causality tab, the side rail shows (**from screenshot 12**):

```
SUB-USV · Node                                                              X

  + Generate Requirements

CAUSAL POSITION  ──────────────────────
  Prerequisite for: Offshore Survey USV-ROV
  ⚠ No children, pyramid layer incomplete

DEPENDENCIES  ──────────────────────
  0                    0
  REQUIREMENTS         REQUIREMENTS    ← (sic, possible UI bug or different col labels)

  0                    0
  INTERFACES           GAPS BELOW

  "A technology domain performs no other purpose than to be a
   collection of parts, techniques, tools, and heuristics."
                                       Harney, Technology Evaluation, Ch. 1

  ▲ Refocus pyramid on this node
  + Decompose with AI
  + Analyze Causality

  Delete
```

Notable structural features:

- **`Prerequisite for: <parent-node>`.** Each node is positioned in the causal graph by what it is a prerequisite for. The relation is upward (a child is a prerequisite for its parent's existence/function). This is unusual: most decomposition tools show "parent of" rather than "prerequisite for". The framing is causal, not structural.
- **`No children, pyramid layer incomplete`** is a domain-language warning. The pyramid is incomplete because the layer below this node is empty. Different from saying "no children": framing it as "layer incomplete" implies the pyramid expects a layer at this depth.
- **`GAPS BELOW`** counter. **inferred** to be the count of expected-but-missing nodes one layer down. The mechanism for "expected" is **unknown** but likely templated against the type or against AI inference.
- **Three action buttons** at panel bottom: `Refocus pyramid on this node`, `+ Decompose with AI`, `+ Analyze Causality`. Refocus changes the pyramid centre. Analyze Causality is **inferred** to be an AI action that infers prerequisites from descriptions and properties.
- **Harney quote.** A different book than the Pace quote on Completeness. Suggests the pedagogical quotes are tab-specific or section-specific.

## What this implies for our model

(Term-mapping in [07-ontology-comparison.md](./07-ontology-comparison.md); architectural implications here.)

- Their **three-axis fidelity** maps loosely onto our two-dimensional maturity model (sync state of `ghost` / `synced` / `orphaned` plus evidence state of `verified` / `external` / `unverified`). Theirs is per-node and quantitative; ours is per-artefact and categorical. The axes are not the same axes, but the spirit (multidimensional rather than single-metric) is shared. We should not adopt their three axes; our two have stronger semantics.
- Their **per-node quality score and project-level rollup** maps onto our reconciler's drift summary. We do not surface a numeric quality at the project level. Whether to do so is a UX question, not a correctness one.
- Their **causality pyramid** is genuinely novel as a primary view. It is an ordering of the system not by structural decomposition but by causal precedence. Whether this is useful for our domain (developer repos, not engineered systems) is **unknown** but worth a thought experiment: a build artefact's causal predecessors are its inputs (declarations, fixtures, contracts) and its causal successors are what consume it (tests, downstream artefacts, users).
- Their **inline `Fix with AI` banner on a deficit** is high-leverage UX. We do not currently surface "the reconciler found a gap, here is one click to address it" inside our outputs. A version of this for our scan output (or webui) would be high-impact.

## Open questions

1. What is the formula for `OVERALL COMPLETENESS Score`? Public docs do not explain it.
2. Does the "Quality 73" header chip aggregate per-node scores, or is it a separate computation?
3. What is the populated form of the causal pyramid? Empty state captured; populated state not.
4. Is the `Analyze Causality` action distinct from `Decompose with AI`, or a synonym in a different framing?
5. What types of nodes have causal positions, and what does "prerequisite for" mean for a non-hierarchical relation (e.g. an interface)?
