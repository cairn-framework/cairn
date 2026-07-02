//! Human-readable renderer for `cairn bundle`.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use std::fmt::Write as _;

use super::super::format::node_arg;
use super::super::*;

pub(crate) fn render_bundle(
    parsed: &ParsedArgs,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    node_arg(&parsed.command_args).and_then(|node| {
        let node = scan_result.graph.resolve(node)?;

        let contract = node
            .contracts
            .iter()
            .filter_map(|path| scan_result.contracts.contracts.get(path))
            .find(|contract| contract.node == node.id)
            .map(|contract| contract.body.trim());

        let decisions = scan_result
            .artefacts
            .decisions
            .iter()
            .filter(|decision| {
                decision.status == crate::artefacts::registry::DecisionStatus::Accepted
                    && decision.nodes.contains(&node.id)
            })
            .collect::<Vec<_>>();

        let dependencies = scan_result
            .graph
            .outbound
            .get(&node.id)
            .into_iter()
            .flatten()
            .filter_map(|edge| scan_result.graph.nodes.get(&edge.to))
            .collect::<Vec<_>>();

        let mut out = format!("Bundle for {}:\n\n", node.id);
        out.push_str("Contract:\n");
        match contract {
            Some(body) if !body.is_empty() => {
                let _ = writeln!(out, "{body}");
            }
            _ => out.push_str("  (missing)\n"),
        }
        out.push_str("\nDecisions:\n");
        if decisions.is_empty() {
            out.push_str("  (none)\n");
        } else {
            for decision in &decisions {
                let _ = writeln!(out, "  {}", decision.id);
            }
        }
        out.push_str("\nDependency interfaces:\n");
        if dependencies.is_empty() {
            out.push_str("  (none)\n");
        } else {
            for target in &dependencies {
                let _ = writeln!(out, "  {}:", target.id);
                if target.symbols.is_empty() {
                    out.push_str("    (no extracted symbols)\n");
                } else {
                    for symbol in &target.symbols {
                        let _ = writeln!(out, "    {}", symbol.signature);
                    }
                }
            }
        }
        out.push_str("\nGates:\n");
        out.push_str(crate::copy::lookup("brief.gates"));
        out.push('\n');
        Ok(out)
    })
}
