# Entity Types Documentation

**Source:** https://www.getcairn.dev/docs/entity-types
**Captured:** 2026-04-28

## Entity Types

The data model primitives that make up every Cairn project.

### Node

Core building block. System, subsystem, assembly, part, or external actor. Properties bag supports engineering params, evaluation status, narrative cache, and brief data.

**Fields:** `id`, `name`, `type`, `description`, `parentId`, `properties`, `position`, `sortOrder`, `createdBy`

### Requirement

Functional, performance, interface, safety, environmental, or constraint requirement scoped to a node.

**Fields:** `id`, `nodeId`, `title`, `description`, `type`, `priority`, `rationale`, `acceptanceCriteria`, `sortOrder`

### Interface

Connection between two nodes carrying typed signals with protocol, rate, and simulation metadata.

**Fields:** `id`, `name`, `sourceNodeId`, `targetNodeId`, `signals`, `protocol`, `description`

### State

Behavioral state in a node's state machine. Supports timing annotations for simulation.

**Fields:** `id`, `name`, `type`, `description`, `nodeId`, `position`, `typicalDuration`, `durationUnit`

### Transition

State transition with guard condition, action, and optional timing parameters.

**Fields:** `id`, `nodeId`, `sourceStateId`, `targetStateId`, `trigger`, `guard`, `action`, `typicalDuration`

### Verification

Test, analysis, demonstration, or inspection record linked to a requirement.

**Fields:** `id`, `requirementId`, `method`, `title`, `description`, `status`, `results`

### TraceLink

Traceability relationship: satisfies, implements, verifies, derives, depends_on, or custom.

**Fields:** `id`, `type`, `sourceId`, `targetId`, `sourceKind`, `targetKind`, `rationale`, `confidence`, `status`

### UseCase

Operational scenario with actors, preconditions, postconditions, and ordered steps.

**Fields:** `id`, `nodeId`, `title`, `description`, `actors`, `preconditions`, `postconditions`, `steps`

### Property

Key-value engineering parameter with units, category, and source. Stored in node.properties.params[].

**Fields:** `key`, `label`, `value`, `unitId`, `category`, `source`, `description`

### Evaluation

Optional dendritic evaluation metadata on nodes for pruned alternatives tracking.

**Fields:** `evaluationStatus`, `decisionType`, `pruneReason`, `firstPrinciple`, `evaluationPhase`, `crossDependencies`
