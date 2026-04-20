//! Map graph construction, integrity checks, and query services.

pub mod build;
pub mod graph;
pub mod integrity;
pub mod query;

pub use build::build_graph;
pub use graph::{EdgeRef, Finding, FindingSeverity, Graph, NodeRecord, NodeState};
