//! Map graph construction, integrity checks, and query services.

pub mod build;
mod contract_coverage;
pub mod graph;
pub mod integrity;
mod module_size;
pub(crate) mod paths;
pub mod query;
mod spec_rule_coverage;
mod test_coverage;

pub use build::build_graph;
pub use graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};
pub(crate) use spec_rule_coverage::validate_deferred_decision_targets;
