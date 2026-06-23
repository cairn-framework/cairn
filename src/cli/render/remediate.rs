//! Remediation and next-action query renderers.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::query_api;

pub(crate) fn render_remediate(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
    if parsed.json {
        format!("{remediate}\n")
    } else {
        format_remediate_human(&remediate)
    }
}

fn format_remediate_human(remediate: &serde_json::Value) -> String {
    let empty: Vec<serde_json::Value> = Vec::new();
    let actions = remediate
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty);
    if actions.is_empty() {
        return "No actions required.\n".to_owned();
    }
    let mut lines = Vec::new();
    lines.push(format!("Actions ({}):", actions.len()));
    for action in actions {
        let priority = action
            .get("priority")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(99);
        let name = action
            .get("action")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown");
        let command = action
            .get("command")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let description = action
            .get("description")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let nodes = action
            .get("nodes")
            .and_then(serde_json::Value::as_array)
            .map(|arr: &Vec<serde_json::Value>| {
                arr.iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        lines.push(format!("  [{priority}] {name}"));
        if !description.is_empty() {
            lines.push(format!("      {description}"));
        }
        if !command.is_empty() {
            lines.push(format!("      run: {command}"));
        }
        if !nodes.is_empty() {
            lines.push(format!("      nodes: {}", nodes.join(", ")));
        }
    }
    lines.join("\n") + "\n"
}

pub(crate) fn render_next(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> String {
    let changes_dir = root.join(&parsed.changes_dir);
    let health = query_api::health_json(root, &changes_dir, scan_result);
    let clean = health
        .get("clean")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    if parsed.json {
        if clean {
            let items = crate::state::backlog::read(root);
            let ready = crate::state::backlog::ready(&items);
            return ready.first().map_or_else(
                || "{\"next\":null,\"clean\":true,\"ready\":0}\n".to_owned(),
                |top| {
                    format!(
                        "{{\"next\":{{\"bead\":\"{}\",\"title\":\"{}\",\"priority\":{},\"source\":\"beads-backlog\"}},\"clean\":true,\"ready\":{}}}\n",
                        esc(&top.id),
                        esc(&top.title),
                        top.priority,
                        ready.len()
                    )
                },
            );
        }
        let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
        let empty: Vec<serde_json::Value> = Vec::new();
        let actions = remediate
            .get("actions")
            .and_then(serde_json::Value::as_array)
            .unwrap_or(&empty);
        let first = actions.first().unwrap_or(&serde_json::Value::Null);
        return format!("{{\"next\":{first},\"clean\":false}}\n");
    }
    if clean {
        let items = crate::state::backlog::read(root);
        let ready = crate::state::backlog::ready(&items);
        return ready.first().map_or_else(
            || "Next: nothing to do. Project is clean.\n".to_owned(),
            |top| {
                let mut out = vec![
                    format!("Next: {} [P{}] {}", top.id, top.priority, top.title),
                    format!("  source: beads backlog ({} ready)", ready.len()),
                    format!("  run: bd show {}", top.id),
                ];
                if let Some(node) = top.linked_node() {
                    out.push(format!("  node: {node}"));
                }
                out.join("\n") + "\n"
            },
        );
    }
    let remediate = query_api::remediate_json(root, &changes_dir, scan_result);
    let empty: Vec<serde_json::Value> = Vec::new();
    let actions = remediate
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .unwrap_or(&empty);
    actions.first().map_or_else(
        || "Next: nothing to do.\n".to_owned(),
        |first| {
            let name = first
                .get("action")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let command = first
                .get("command")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            let description = first
                .get("description")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("");
            let nodes = first
                .get("nodes")
                .and_then(serde_json::Value::as_array)
                .map(|arr: &Vec<serde_json::Value>| {
                    arr.iter()
                        .filter_map(serde_json::Value::as_str)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let mut lines = Vec::new();
            lines.push(format!("Next action: {name}"));
            if !description.is_empty() {
                lines.push(format!("  {description}"));
            }
            if !command.is_empty() {
                lines.push(format!("  run: {command}"));
            }
            if !nodes.is_empty() {
                lines.push(format!("  nodes: {}", nodes.join(", ")));
            }
            lines.join("\n") + "\n"
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_remediate_human_empty_actions() {
        let json = serde_json::json!({"actions": []});
        assert_eq!(format_remediate_human(&json), "No actions required.\n");
    }

    #[test]
    fn format_remediate_human_lists_actions() {
        let json = serde_json::json!({
            "actions": [{
                "priority": 1,
                "action": "fix-lint",
                "description": "Run cargo fmt",
                "command": "cargo fmt",
                "nodes": ["app"]
            }]
        });
        let rendered = format_remediate_human(&json);
        assert!(rendered.contains("Actions (1):"));
        assert!(rendered.contains("[1] fix-lint"));
        assert!(rendered.contains("Run cargo fmt"));
        assert!(rendered.contains("run: cargo fmt"));
        assert!(rendered.contains("nodes: app"));
    }

    #[test]
    fn format_remediate_human_omits_optional_fields() {
        let json = serde_json::json!({
            "actions": [{"action": "noop"}]
        });
        let rendered = format_remediate_human(&json);
        assert!(rendered.contains("[99] noop"));
        assert!(!rendered.contains("run:"));
        assert!(!rendered.contains("nodes:"));
    }
}
