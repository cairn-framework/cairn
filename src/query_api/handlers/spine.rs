//! Spine (project-level) query handlers used by the HTTP server.
//!
//! These mirror the legacy `src/ui/api.rs` endpoints but centralise JSON
//! shape-building in `query_api`, so the webui becomes a thin router.

#![allow(clippy::wildcard_imports)] // Reason: spine handlers re-export the parent glob import surface, matching the api.rs/serialise.rs split.

use super::super::*;

/// Introspection endpoint mirroring the legacy `api::meta_json`: the cairn
/// version plus every registered query tool and its request/response schemas.
pub(crate) fn ui_meta_json() -> Value {
    let commands = crate::cli::registry()
        .iter()
        .map(|command| {
            json!({
                "name": command.cli_name,
                "request": command.request_schema,
                "response": command.response_schema,
                "safety": format!("{:?}", command.safety),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "version": crate::package_version(),
        "available_commands": commands,
    })
}

/// Returns the raw blueprint file contents (or an error object on read failure).
pub(crate) fn blueprint_json(blueprint_path: &Path) -> Value {
    let display_path = blueprint_path.to_string_lossy().to_string();
    match fs::read_to_string(blueprint_path) {
        Ok(source) => json!({
            "path": display_path,
            "source": source,
        }),
        Err(error) => json!({
            "path": display_path,
            "source": Value::Null,
            "error": error.to_string(),
        }),
    }
}

/// Lists the backlog beads linked to a node.
pub(crate) fn beads_json(root: &Path, node: &str) -> Value {
    let items = crate::state::backlog::read(root);
    let beads = crate::state::backlog::for_node(&items, node)
        .iter()
        .map(|item| item.to_json())
        .collect::<Vec<_>>();
    json!({
        "node": node,
        "beads": beads,
    })
}
