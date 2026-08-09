//! Archive regressions for provenance-aware edge operations.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use super::*;

#[test]
fn archive_rename_then_edge_operations_preserve_provenance()
-> Result<(), Box<dyn std::error::Error>> {
    for inferred in [false, true] {
        let removed = archive_rename_edge_operation("remove", inferred)?;
        assert!(!removed.contains("app.http -> app.db"));
        let modified = archive_rename_edge_operation("modify", inferred)?;
        let marker = if inferred { " @inferred" } else { "" };
        let expected = format!("app.http -> app.db \"updated\"{marker}");
        assert_eq!(modified.matches(&expected).count(), 1, "{modified}");
    }
    Ok(())
}

fn archive_rename_edge_operation(
    operation: &str,
    inferred: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let root = temp_root("archive-rename-edge")?;
    write_project_with_edge(&root, inferred)?;
    let change_path = root.join("meta/changes/rename-edge");
    fs::create_dir_all(&change_path)?;
    fs::write(change_path.join("proposal.md"), "# Rename edge\n")?;
    let marker = if inferred { " @inferred" } else { "" };
    let edge_section = match operation {
        "remove" => format!("## REMOVED Edges\napp.http -> app.db \"calls\"{marker}\n"),
        "modify" => format!("## MODIFIED Edges\napp.http -> app.db \"updated\"{marker}\n"),
        _ => return Err(format!("unknown edge operation `{operation}`").into()),
    };
    let delta = format!("## RENAMED Nodes\n- app.api -> app.http\n\n{edge_section}");
    fs::write(change_path.join("blueprint.delta"), delta)?;
    archive(
        &root,
        &root.join("cairn.blueprint"),
        &root.join("meta/changes"),
        "rename-edge",
    )?;
    Ok(fs::read_to_string(root.join("cairn.blueprint"))?)
}

fn write_project_with_edge(root: &Path, inferred: bool) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(root.join("meta/changes"))?;
    let marker = if inferred { " @inferred" } else { "" };
    fs::write(
        root.join("cairn.blueprint"),
        format!(
            "System App \"desc\" id \"app\" {{\n\
Module Api \"desc\" id \"app.api\" {{}}\n\
Module Db \"desc\" id \"app.db\" {{}}\n\
}}\n\
app.api -> app.db \"calls\"{marker}\n"
        ),
    )?;
    fs::write(
        root.join("cairn.config.yaml"),
        "ignore:\n  - target\ncontext: \"\"\nrules: {}\n",
    )?;
    Ok(())
}

fn temp_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-edge-provenance-{name}-{suffix}"));
    fs::create_dir_all(&root)?;
    Ok(root)
}
