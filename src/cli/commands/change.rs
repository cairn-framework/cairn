//! CLI change-directory command implementations.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn run_change_new(root: &Path, change_id: &str) -> CliResult {
    if change_id.is_empty()
        || !change_id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return err(
            2,
            "change ID must be kebab-case (lowercase letters, digits, hyphens only)",
        );
    }
    let change_dir = root.join("meta/changes").join(change_id);
    if change_dir.exists() {
        return err(
            1,
            &format!("change directory already exists: {}", change_dir.display()),
        );
    }
    if let Err(error) = fs::create_dir_all(&change_dir) {
        return err(1, &format!("failed to create change directory: {error}"));
    }

    let proposal = format!(
        "# Proposal: {change_id}\n\n## Motivation\n\nDescribe the problem this change solves.\n\n## Scope\n\n- What this change covers\n\n## Out of scope\n\n- What this change does not cover\n",
    );
    if let Err(error) = fs::write(change_dir.join("proposal.md"), proposal) {
        return err(1, &format!("failed to write proposal.md: {error}"));
    }

    let design = format!(
        "# Design: {change_id}\n\n## Approach\n\nHigh-level approach to the solution.\n\n## Changes\n\nADDED:\n- New components\n\nMODIFIED:\n- Existing components\n\nREMOVED:\n- Obsolete components\n\nRENAMED:\n- Components with new names\n",
    );
    if let Err(error) = fs::write(change_dir.join("design.md"), design) {
        return err(1, &format!("failed to write design.md: {error}"));
    }

    let tasks =
        format!("# Tasks: {change_id}\n\n- [ ] Task one\n- [ ] Task two\n- [ ] Task three\n");
    if let Err(error) = fs::write(change_dir.join("tasks.md"), tasks) {
        return err(1, &format!("failed to write tasks.md: {error}"));
    }

    if let Err(error) = fs::create_dir_all(change_dir.join("specs")) {
        return err(1, &format!("failed to create specs directory: {error}"));
    }

    if let Ok(config) = crate::scanner::config::load(root)
        && config.state_backend == "beads"
    {
        let beads = crate::state::BeadsStateBackend::new(root.to_path_buf());
        match beads.create_change_epic(change_id) {
            Ok(bead_id) => {
                let _ = fs::write(change_dir.join(".bead-id"), &bead_id);
                let tasks_content =
                    fs::read_to_string(change_dir.join("tasks.md")).unwrap_or_default();
                let task_lines: Vec<&str> = tasks_content
                    .lines()
                    .filter_map(|line| line.strip_prefix("- [ ] "))
                    .collect();
                if !task_lines.is_empty() {
                    match beads.create_task_beads(&bead_id, &task_lines) {
                        Ok(task_ids) => {
                            let _ =
                                fs::write(change_dir.join(".task-bead-ids"), task_ids.join("\n"));
                        }
                        Err(error) => {
                            eprintln!("warning: failed to create task beads: {error}");
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("warning: failed to create beads epic: {error}");
            }
        }
    }

    ok(format!(
        "created change directory at meta/changes/{change_id}/\n"
    ))
}

/// List tasks for a change backed by beads.
pub(crate) fn run_change_tasks(root: &Path, change_id: &str) -> CliResult {
    let change_dir = root.join("meta/changes").join(change_id);
    if !change_dir.exists() {
        return err(
            1,
            &format!("change directory not found: {}", change_dir.display()),
        );
    }
    let bead_id_path = change_dir.join(".bead-id");
    if !bead_id_path.exists() {
        return err(1, "change has no beads backing; tasks are in tasks.md only");
    }
    let bead_id = match fs::read_to_string(&bead_id_path) {
        Ok(id) => id.trim().to_owned(),
        Err(error) => return err(1, &format!("failed to read .bead-id: {error}")),
    };
    let beads = crate::state::BeadsStateBackend::new(root.to_path_buf());
    match beads.list_child_tasks(&bead_id) {
        Ok(tasks) => {
            if tasks.is_empty() {
                return ok("no tasks found\n".to_owned());
            }
            let mut out = String::new();
            for (id, title) in tasks {
                let _ = std::fmt::Write::write_fmt(&mut out, format_args!("{id}: {title}\n"));
            }
            ok(out)
        }
        Err(error) => err(1, &format!("failed to list tasks: {error}")),
    }
}

/// Claim a change and all its open tasks.
pub(crate) fn run_change_apply(root: &Path, change_id: &str) -> CliResult {
    let change_dir = root.join("meta/changes").join(change_id);
    if !change_dir.exists() {
        return err(
            1,
            &format!("change directory not found: {}", change_dir.display()),
        );
    }
    let bead_id_path = change_dir.join(".bead-id");
    if !bead_id_path.exists() {
        return err(1, "change has no beads backing; apply is not supported");
    }
    let bead_id = match fs::read_to_string(&bead_id_path) {
        Ok(id) => id.trim().to_owned(),
        Err(error) => return err(1, &format!("failed to read .bead-id: {error}")),
    };
    let beads = crate::state::BeadsStateBackend::new(root.to_path_buf());
    match beads.claim_change(&bead_id) {
        Ok(()) => ok(format!("claimed change {change_id} and its tasks\n")),
        Err(error) => err(1, &format!("failed to claim change: {error}")),
    }
}
