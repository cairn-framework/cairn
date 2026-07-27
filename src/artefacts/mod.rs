//! Artefact loaders.

pub mod contract;
pub mod frontmatter;
pub mod registry;
pub mod tasks;

pub use registry::types::{Claims, ClaimsMode};
pub use tasks::{TaskProgress, count_tasks};
