# Verification and Traceability

**Source:** https://www.getcairn.dev/docs/verification
**Captured:** 2026-04-28

Verification confirms that your system meets its requirements. Traceability connects everything. Requirements to nodes, verifications to requirements, decisions to rationale.

## Verification Methods

Four standard methods:

**Test**: Physical or functional testing that measures performance.

"Conduct range test: drive vehicle until battery depletes, measure total distance traveled."

**Analysis**: Calculation or simulation that demonstrates compliance.

"Power budget analysis shows 18% margin under worst-case assumptions."

**Demonstration**: Operational exercise that shows capability.

"Demonstrate successful delivery sequence with live operator monitoring."

**Inspection**: Visual or physical examination.

"Inspect connector pinout against ICD; verify thermal compound application."

## Verification Titles

Beyond the method and description, every verification carries a short title. The line that appears in the Inspector heading and in cascade previews when you delete the requirement it verifies. AI-generated verifications come with a title by default; rewrite it in the Inspector at any time.

## Coverage Metrics

The Verification lens shows: what percentage of requirements have verification records, what percentage have passing status, and which requirements have no verification plan. Coverage rolls up through the tree.

## Trace Links

Trace links connect entities across your model:

- **satisfies**: A design element satisfies a requirement
- **verifies**: A verification record verifies a requirement
- **derives**: A child requirement derives from a parent
- **depends_on**: One node depends on another

The Traceability tool shows a matrix view with gaps immediately visible. Requirements with no verifications, nodes with no requirements, orphaned verifications.

## Impact Analysis

Trace links enable impact analysis: "If I change this requirement, what's affected?" The Traceability tool highlights the impact chain when you select any entity.

This is a core advantage of connected models over disconnected documents. When everything links to everything else, you can actually answer "what does this affect?"
