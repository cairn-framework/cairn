# AI Governance: Propose, Review, Apply

**Source:** https://www.getcairn.dev/docs/ai-governance
**Captured:** 2026-04-28

When you request that Cairn's AI decompose a subsystem or generate requirements, a distinctive process unfolds: *nothing changes immediately*.

The AI generates a ChangeSet, a collection of proposed operations, rather than directly modifying your model. You review these proposals, and the model only updates when you choose to apply the ChangeSet.

## Why AI Shouldn't Directly Mutate Your Model

AI systems produce errors. They generate requirements that lack coherence, decompose systems while overlooking constraints, use poor naming conventions, and occasionally propose interfaces that contradict physical principles.

If AI modified your model directly, you might not discover these problems until much later, when they've become embedded within dependent components. Resolving such issues requires significant investigative work to understand what occurred, identify downstream effects, and safely reverse the changes.

The ChangeSet pattern circumvents this risk. You observe precisely what the AI intends before implementation occurs. You can embrace valuable components while rejecting problematic ones, maintaining full control throughout.

## The ChangeSet Contract

A ChangeSet comprises a sequence of operations:

- **create**. Introduce a new node, requirement, interface, state, or similar entity
- **update**. Alter properties of an existing entity
- **delete**. Eliminate an entity from the model

Each operation remains independent and transparent. When reviewing a ChangeSet, you encounter:

```
create node "Battery Pack" as child of "Power Subsystem"
create requirement "REQ-PWR-001: Capacity" on "Battery Pack"
update node "Power Subsystem", description changed
```

This represents explicit, evaluable actions rather than ambiguous AI activity.

## Operation-by-Operation Review

The review interface displays each operation including:

- The affected entity
- The nature of creation, modification, or removal
- A diff comparison for updates
- Granular controls for acceptance, rejection, or modification

You might accept node additions while rejecting an illogical requirement, or revise a name before proceeding. Full ChangeSet rejection remains available if the AI misinterpreted your request.

Selective acceptance represents the standard approach. AI typically succeeds 80% of the time; your responsibility involves identifying the remaining 20%.

## History and Rollback

Every applied ChangeSet gets logged through the History tool, capturing:

- Time of application
- Origin (specialist identifier or manual edit attribution)
- Operation summary
- Comprehensive operation details

Should you apply a ChangeSet and later regret the decision, you can locate it in history and review exact modifications. Comprehensive undo functionality remains under development; the history feature currently provides an audit trail for manual reversions.

## Trust Calibration

Begin with skepticism. Examine each operation within initial ChangeSets. Decline liberally. Standardize naming conventions before accepting.

Through exposure to AI outputs, patterns emerge regarding accuracy and weaknesses. The Architect specialist typically demonstrates strength in decomposition. The Requirements specialist occasionally generates excessive requirements. The Behavior specialist requires clearer guidance on state division.

Customize your confidence level by specialist and circumstance. The ChangeSet pattern supplies the transparency necessary for this calibration.

## The V and V Literature Agrees

This methodology draws from established verification and validation research. Model modifications require traceability, reviewability, and clear attribution. Simulation V and V frameworks emphasize that "you can't validate what you can't inspect."

Cairn extends this principle to AI-supported engineering contexts. All AI recommendations function as proposals. Applied modifications remain traceable. Your model's origins stay transparent.
