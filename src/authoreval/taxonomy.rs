//! Failure taxonomy for scored hotspots.
//!
//! A hotspot is attributable to one of exactly three classes: authoring
//! syntax, cairn's generated guidance, or a missing repair affordance. The
//! subclass keeps the finer blueprint-versus-artefact attribution without
//! inventing a fourth peer class.

use serde::{Deserialize, Serialize};

/// What a hotspot is attributable to.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    /// The authored text is malformed or incomplete.
    Syntax,
    /// The text is well formed; cairn's generated guidance did not prevent an
    /// inconsistent graph.
    GeneratedGuidance,
    /// The finding survived a repair attempt, so the feedback offered nothing
    /// that cleared it.
    MissingRepairAffordance,
}

/// Where the failure originates, independent of whether a repair survived it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureSubclass {
    /// A defect in the `.blueprint` text.
    Blueprint,
    /// A defect in artefact frontmatter or its declared fields.
    Artefact,
    /// A mismatch between the declared graph and the tree.
    Graph,
    /// The code is outside the classification table.
    Unknown,
}

/// Prefix table, evaluated in declaration order. A code matching nothing lands
/// in [`FailureSubclass::Unknown`] so the table's own coverage gap stays
/// visible in the data rather than mimicking an unrelated class.
const SUBCLASS_TABLE: &[(&str, FailureSubclass)] = &[
    ("CAIRN_PARSE_", FailureSubclass::Blueprint),
    ("CAIRN_BLUEPRINT_", FailureSubclass::Blueprint),
    ("CAIRN_INTEGRITY_", FailureSubclass::Blueprint),
    ("CAIRN_TAG_UNREGISTERED", FailureSubclass::Blueprint),
    ("CAIRN_ORDER_CYCLE", FailureSubclass::Blueprint),
    ("CAIRN_NO_BLUEPRINT", FailureSubclass::Blueprint),
    ("CAIRN_IO_READ_BLUEPRINT", FailureSubclass::Blueprint),
    ("CAIRN_ARTEFACT_", FailureSubclass::Artefact),
    ("CAIRN_DECISION_", FailureSubclass::Artefact),
    ("CAIRN_TODO_", FailureSubclass::Artefact),
    ("CAIRN_RESEARCH_", FailureSubclass::Artefact),
    ("CAIRN_SOURCE_", FailureSubclass::Artefact),
    ("CAIRN_REVIEW_", FailureSubclass::Artefact),
    ("CAIRN_CONTRACT_", FailureSubclass::Artefact),
    ("CAIRN_CHANGE_", FailureSubclass::Artefact),
    ("CAIRN_RECONCILE_", FailureSubclass::Graph),
    ("CAIRN_PROVENANCE_", FailureSubclass::Graph),
    ("CAIRN_TEST_COVERAGE_", FailureSubclass::Graph),
    ("CAIRN_MODULE_OVERSIZED", FailureSubclass::Graph),
    ("CAIRN_INTERFACE_HASH_CHANGED", FailureSubclass::Graph),
    ("CAIRN_PATH_GITIGNORED", FailureSubclass::Graph),
    ("CAIRN_SPEC_RULE_", FailureSubclass::Graph),
    ("CAIRN_HOOK_", FailureSubclass::Graph),
];

/// Where a finding code originates.
pub(crate) fn subclass_for(code: &str) -> FailureSubclass {
    SUBCLASS_TABLE
        .iter()
        .find_map(|(prefix, subclass)| code.starts_with(prefix).then_some(*subclass))
        .unwrap_or(FailureSubclass::Unknown)
}

/// Classifies one finding code.
///
/// `persisted` is true only when the same code was present in the immediately
/// preceding failed scan, which cannot happen before attempt 2. Precedence is
/// fixed: persistence first, then the code table.
pub(crate) fn classify(code: &str, persisted: bool) -> (FailureClass, FailureSubclass) {
    let subclass = subclass_for(code);
    let class = if persisted {
        FailureClass::MissingRepairAffordance
    } else {
        match subclass {
            FailureSubclass::Blueprint | FailureSubclass::Artefact => FailureClass::Syntax,
            FailureSubclass::Graph | FailureSubclass::Unknown => FailureClass::GeneratedGuidance,
        }
    };
    (class, subclass)
}
