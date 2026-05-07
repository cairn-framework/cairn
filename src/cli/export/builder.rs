//! Builder for the export envelope: scans the project and assembles the
//! full payload.

use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    artefacts::registry::{ArtefactSet, ArtefactType},
    changes::{self, Change},
    map::graph::{Graph, NodeRecord},
    scanner,
};

use super::{ArtefactEntry, ChangeEntry, EdgeEntry, ExportEnvelope, SCHEMA_VERSION};

/// Builds a full export envelope by scanning the project and reading active changes.
///
/// # Errors
///
/// Returns the scanner error string when the project cannot be loaded.
pub fn build_export(file: &Path, changes_dir: &Path) -> Result<ExportEnvelope, String> {
    let root = super::blueprint_root(file);
    let scan_result = scanner::load_project(root, file)?;
    let now = current_timestamp_rfc3339();
    let edges = flatten_edges(&scan_result.graph);
    let artefacts = flatten_artefacts(&scan_result.artefacts);
    let change_records = changes::discover(changes_dir).unwrap_or_default();
    let changes_out = flatten_changes(&change_records);
    let mut nodes: Vec<NodeRecord> = scan_result.graph.nodes.values().cloned().collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(ExportEnvelope {
        schema_version: SCHEMA_VERSION,
        generated_at: now,
        blueprint_path: file.to_path_buf(),
        nodes,
        edges,
        artefacts,
        changes: changes_out,
    })
}

fn flatten_edges(graph: &Graph) -> Vec<EdgeEntry> {
    let mut out = Vec::new();
    for (from, refs) in &graph.outbound {
        for edge in refs {
            out.push(EdgeEntry {
                from: from.clone(),
                to: edge.to.clone(),
                verb: edge.description.clone(),
            });
        }
    }
    out.sort_by(|a, b| a.from.cmp(&b.from).then_with(|| a.to.cmp(&b.to)));
    out
}

fn flatten_artefacts(set: &ArtefactSet) -> Vec<ArtefactEntry> {
    let mut out = Vec::new();
    for contract in set.contracts.contracts.values() {
        out.push(ArtefactEntry {
            artefact_type: ArtefactType::Contract,
            id: contract.path.clone(),
            path: contract.path.clone(),
            node: Some(contract.node.clone()),
        });
    }
    for todo in &set.todos {
        out.push(ArtefactEntry {
            artefact_type: ArtefactType::Todo,
            id: todo.path.clone(),
            path: todo.path.clone(),
            node: Some(todo.node.clone()),
        });
    }
    for decision in &set.decisions {
        out.push(ArtefactEntry {
            artefact_type: ArtefactType::Decision,
            id: decision.id.clone(),
            path: decision.path.clone(),
            node: decision.nodes.first().cloned(),
        });
    }
    for review in &set.reviews {
        out.push(ArtefactEntry {
            artefact_type: ArtefactType::Review,
            id: review.path.clone(),
            path: review.path.clone(),
            node: Some(review.node.clone()),
        });
    }
    for research in &set.research {
        out.push(ArtefactEntry {
            artefact_type: ArtefactType::Research,
            id: research.id.clone(),
            path: research.path.clone(),
            node: research.nodes.first().cloned(),
        });
    }
    for source in &set.sources {
        out.push(ArtefactEntry {
            artefact_type: ArtefactType::Source,
            id: source.id.clone(),
            path: source.path.clone(),
            node: None,
        });
    }
    out.sort_by(|a, b| {
        artefact_type_order(a.artefact_type)
            .cmp(&artefact_type_order(b.artefact_type))
            .then_with(|| a.id.cmp(&b.id))
    });
    out
}

pub(super) fn artefact_type_order(t: ArtefactType) -> u8 {
    match t {
        ArtefactType::Contract => 0,
        ArtefactType::Decision => 1,
        ArtefactType::Todo => 2,
        ArtefactType::Research => 3,
        ArtefactType::Review => 4,
        ArtefactType::Source => 5,
    }
}

pub(super) fn artefact_type_label(t: ArtefactType) -> &'static str {
    match t {
        ArtefactType::Contract => "contract",
        ArtefactType::Decision => "decision",
        ArtefactType::Todo => "todo",
        ArtefactType::Research => "research",
        ArtefactType::Review => "review",
        ArtefactType::Source => "source",
    }
}

fn flatten_changes(changes_in: &[Change]) -> Vec<ChangeEntry> {
    let mut out: Vec<ChangeEntry> = changes_in
        .iter()
        .map(|c| ChangeEntry {
            id: c.id.clone(),
            state: "active".to_owned(),
            title: c.title.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

fn current_timestamp_rfc3339() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_unix_timestamp(secs)
}

fn format_unix_timestamp(secs: u64) -> String {
    let (y, mo, d, h, mi, s) = unix_to_ymdhms(secs);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

fn unix_to_ymdhms(secs: u64) -> (u32, u32, u32, u32, u32, u32) {
    let day = secs / 86_400;
    let secs_of_day = u32::try_from(secs % 86_400).unwrap_or(0);
    let h = secs_of_day / 3600;
    let mi = (secs_of_day % 3600) / 60;
    let s = secs_of_day % 60;
    let (y, mo, d) = days_to_ymd(day + 719_468);
    (y, mo, d, h, mi, s)
}

fn days_to_ymd(days: u64) -> (u32, u32, u32) {
    let era = days / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (
        u32::try_from(year).unwrap_or(0),
        u32::try_from(m).unwrap_or(0),
        u32::try_from(d).unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_epoch_formats_to_zero_date() {
        let (y, mo, d, h, mi, s) = unix_to_ymdhms(0);
        assert_eq!((y, mo, d, h, mi, s), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn format_timestamp_for_known_value() {
        // 2026-05-07T12:00:00Z = 1778155200 seconds since unix epoch.
        let stamp = format_unix_timestamp(1_778_155_200);
        assert_eq!(stamp, "2026-05-07T12:00:00Z");
    }

    #[test]
    fn build_export_propagates_scanner_errors() {
        let result = build_export(
            Path::new("/nonexistent/cairn.blueprint"),
            Path::new("/nonexistent/changes"),
        );
        assert!(result.is_err());
    }
}
