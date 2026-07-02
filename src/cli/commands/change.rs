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

    ok(format!(
        "created change directory at meta/changes/{change_id}/\n"
    ))
}
