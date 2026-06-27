//! Map graph construction, integrity checks, and query services.

pub mod build;
mod contract_coverage;
pub mod graph;
pub mod integrity;
pub mod query;
mod test_coverage;

pub use build::build_graph;
pub use graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};
