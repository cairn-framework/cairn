// cairn:allow-large-module reason: security-focused path validation tests require cohesion with the guarded functions
//! `cairn init --wire` implementation: appends an idempotent cairn
//! orientation reference to the project's agent instructions file.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Sentinel comment marking the start of the auto-wired cairn orientation
/// block. Used for idempotency detection: if already present in the target
/// file, `--wire` is a no-op.
const WIRE_SENTINEL: &str = "<!-- cairn:agent-guide-begin -->";

/// The reference block appended to the project's agent instructions file by
/// `cairn init --wire`. Points the agent at `.cairn/AGENTS.md` rather than
/// duplicating its content, so the guide stays in sync when `cairn init`
/// backfills it. This is a file-generation asset (like `AGENT_GUIDE`), not a
/// CLI display string, so it lives here rather than in `copy.toml`.
const WIRE_BLOCK: &str = "\
<!-- cairn:agent-guide-begin -->\n\
## Cairn orientation\n\
\n\
This project uses cairn to keep its architecture map in sync with code. Read\n\
`.cairn/AGENTS.md` for full orientation, then follow\n\
`.claude/skills/cairn-dev/SKILL.md` for the development loop.\n\
<!-- cairn:agent-guide-end -->\n";

/// Write `content` to a collision-safe temp file in `dir`, then atomically
/// rename it to `target`. Cleans up the temp file on any failure.
pub(crate) fn atomic_write(dir: &Path, target: &Path, content: &str) -> Result<(), String> {
    use std::io::Write;
    let pid = std::process::id();
    let mut tmp_path = dir.join(format!(".cairn-wire-{pid}"));
    let mut handle = None;
    for i in 0u32..100 {
        if i > 0 {
            tmp_path = dir.join(format!(".cairn-wire-{pid}-{i}"));
        }
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp_path)
        {
            Ok(h) => {
                handle = Some(h);
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.to_string()),
        }
    }
    let Some(mut f) = handle else {
        return Err(copy::lookup("init.wire.err-temp-collision").to_owned());
    };
    // On Unix, restrict the temp file to the target's mode before writing
    // any content, so private data is never in a permissive temp file.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(target) {
            let mode = metadata.permissions().mode();
            if let Err(e) = fs::set_permissions(&tmp_path, fs::Permissions::from_mode(mode)) {
                let _ = fs::remove_file(&tmp_path);
                return Err(e.to_string());
            }
        }
    }
    if let Err(e) = f.write_all(content.as_bytes()) {
        let _ = fs::remove_file(&tmp_path);
        return Err(e.to_string());
    }
    drop(f);
    if let Err(e) = fs::rename(&tmp_path, target) {
        let _ = fs::remove_file(&tmp_path);
        return Err(e.to_string());
    }
    Ok(())
}

/// Walk from root toward `file`, returning an error if any component is a
/// symlink that could redirect a write outside the project root. Read-only.
pub(crate) fn check_symlink_containment(root: &Path, file: &Path) -> CliResult {
    let relative = file.strip_prefix(root).unwrap_or(file);
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        if let Ok(metadata) = fs::symlink_metadata(&current)
            && metadata.file_type().is_symlink()
        {
            return err(
                1,
                &copy::lookup("init.wire.err-symlink")
                    .replace("{file}", &current.display().to_string()),
            );
        }
    }
    ok(String::new())
}

/// Check if `file` is or is under a cairn scaffold path. Compares
/// case-insensitively so the guard matches filesystem identity on macOS and
/// Windows. `.claude` itself is rejected (init creates it for skills) but
/// files directly under `.claude/` (e.g. `.claude/CLAUDE.md`) are allowed.
fn is_scaffold_path(root: &Path, file: &Path) -> bool {
    let Some(relative) = file.strip_prefix(root).ok() else {
        return false;
    };
    let rel = relative
        .to_string_lossy()
        .to_ascii_lowercase()
        .replace('\\', "/");
    for scaffold in [
        ".cairn",
        ".claude/skills",
        "cairn.blueprint",
        "cairn.config.yaml",
    ] {
        if rel == scaffold || rel.starts_with(&format!("{scaffold}/")) {
            return true;
        }
    }
    // The .claude directory itself is scaffold-created, but files directly
    // under it (like .claude/CLAUDE.md) are legitimate targets.
    rel == ".claude"
}

/// Validate that an explicit wire target is project-relative. Accepts only
/// `Normal` and `CurDir` path components, rejecting absolute paths, `..`,
/// Windows drive prefixes, and root-relative paths. Callable before
/// `init_project` so an invalid target does not scaffold the project.
pub(crate) fn validate_wire_target(target: Option<&str>) -> CliResult {
    if let Some(path) = target {
        let p = std::path::Path::new(path);
        if p.components().any(|c| {
            !matches!(
                c,
                std::path::Component::Normal(_) | std::path::Component::CurDir
            )
        }) {
            return err(2, copy::lookup("init.wire.err-absolute"));
        }
    }
    ok(String::new())
}

/// Preflight check: validate the wire target lexically AND check for symlink
/// escapes, before `init_project` creates scaffold files. This ensures an
/// invalid or symlinked target does not scaffold the project.
pub(crate) fn preflight_wire_check(root: &Path, target: Option<&str>) -> CliResult {
    let validation = validate_wire_target(target);
    if validation.code != 0 {
        return validation;
    }
    let file = match target {
        Some(path) => root.join(path),
        None => detect_instructions_file(root),
    };
    if is_scaffold_path(root, &file) {
        return err(2, copy::lookup("init.wire.err-scaffold"));
    }
    let containment = check_symlink_containment(root, &file);
    if containment.code != 0 {
        return containment;
    }
    if file.is_dir() {
        return err(
            1,
            &copy::lookup("init.wire.err-not-file").replace("{file}", &file.display().to_string()),
        );
    }
    ok(String::new())
}

/// Wire `.cairn/AGENTS.md` into the project's agent instructions file by
/// appending an idempotent reference block.
///
/// `target` is an explicit path relative to `root`, or `None` to auto-detect
/// (CLAUDE.md if it exists, else AGENTS.md, creating the latter if neither
/// exists). If the sentinel is already present the call is a byte-exact no-op.
pub(crate) fn wire_agent_guide(root: &Path, target: Option<&str>) -> CliResult {
    let validation = validate_wire_target(target);
    if validation.code != 0 {
        return validation;
    }
    let guide = root.join(".cairn/AGENTS.md");
    if !guide.is_file() {
        if guide.exists() {
            return err(1, copy::lookup("init.wire.err-guide-not-file"));
        }
        return err(1, copy::lookup("init.wire.err-no-guide"));
    }
    let file = match target {
        Some(path) => root.join(path),
        None => detect_instructions_file(root),
    };
    if is_scaffold_path(root, &file) {
        return err(2, copy::lookup("init.wire.err-scaffold"));
    }
    let containment = check_symlink_containment(root, &file);
    if containment.code != 0 {
        return containment;
    }
    if file.is_dir() {
        return err(
            1,
            &copy::lookup("init.wire.err-not-file").replace("{file}", &file.display().to_string()),
        );
    }
    let existing = match fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            return err(
                1,
                &copy::lookup("init.wire.err-read")
                    .replace("{file}", &file.display().to_string())
                    .replace("{detail}", &e.to_string()),
            );
        }
    };
    if existing.contains(WIRE_SENTINEL) {
        return ok(copy::lookup("init.wire.already-wired").to_owned());
    }
    let mut content = existing;
    if !content.is_empty() {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push('\n');
    }
    content.push_str(WIRE_BLOCK);
    if let Some(parent) = file.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        return err(
            1,
            &copy::lookup("init.wire.err-create-dir")
                .replace("{dir}", &parent.display().to_string())
                .replace("{detail}", &e.to_string()),
        );
    }
    // Write atomically so a failed write cannot truncate the user's existing
    // instructions file.
    let dir = file.parent().unwrap_or(root);
    if let Err(detail) = atomic_write(dir, &file, &content) {
        return err(
            1,
            &copy::lookup("init.wire.err-write")
                .replace("{file}", &file.display().to_string())
                .replace("{detail}", &detail),
        );
    }
    ok(copy::lookup("init.wire.done").replace("{file}", &file.display().to_string()))
}

/// Pick the agent instructions file to wire into: CLAUDE.md if it exists,
/// then AGENTS.md, defaulting to creating AGENTS.md.
fn detect_instructions_file(root: &Path) -> PathBuf {
    for candidate in ["CLAUDE.md", "AGENTS.md"] {
        let path = root.join(candidate);
        if path.symlink_metadata().is_ok() {
            return path;
        }
    }
    root.join("AGENTS.md")
}

#[cfg(test)]
mod tests {
    use super::super::project::init_project;
    use super::*;

    #[test]
    fn test_wire_auto_detect_creates_agents_md_when_none_exists() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);
        // init does not create a root AGENTS.md, so wire must create one.
        assert!(!dir.path().join("AGENTS.md").exists());

        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(result.code, 0, "wire should succeed: {}", result.stderr);

        let file = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert!(
            file.contains("cairn:agent-guide-begin"),
            "wired file must contain the cairn orientation block"
        );
        assert!(
            file.contains(".cairn/AGENTS.md"),
            "wired file must reference the agent guide"
        );
    }

    #[test]
    fn test_wire_auto_detect_prefers_existing_claude_md() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("CLAUDE.md"),
            "# My project\n\nSome rules.\n",
        )
        .unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(result.code, 0, "wire should succeed: {}", result.stderr);

        let claude = std::fs::read_to_string(dir.path().join("CLAUDE.md")).unwrap();
        assert!(
            claude.contains("cairn:agent-guide-begin"),
            "wire must target the existing CLAUDE.md"
        );
        assert!(
            claude.starts_with("# My project"),
            "wire must preserve existing content, not overwrite"
        );
        // Root AGENTS.md must not be created when CLAUDE.md exists.
        assert!(
            !dir.path().join("AGENTS.md").exists(),
            "wire must not create a root AGENTS.md when CLAUDE.md exists"
        );
    }

    #[test]
    fn test_wire_explicit_target_resolved_relative_to_root() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), Some(".cursor/rules/cairn.md"));
        assert_eq!(result.code, 0, "wire should succeed: {}", result.stderr);

        let file = std::fs::read_to_string(dir.path().join(".cursor/rules/cairn.md")).unwrap();
        assert!(
            file.contains("cairn:agent-guide-begin"),
            "explicit target must receive the orientation block"
        );
    }

    #[test]
    fn test_wire_is_idempotent_byte_exact() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        let first = wire_agent_guide(dir.path(), None);
        assert_eq!(first.code, 0);
        let after_first = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();

        let second = wire_agent_guide(dir.path(), None);
        assert_eq!(second.code, 0, "second wire must succeed");

        let after_second = std::fs::read_to_string(dir.path().join("AGENTS.md")).unwrap();
        assert_eq!(
            after_first, after_second,
            "re-running wire must leave the file byte-exact unchanged"
        );
        let count = after_second.matches("cairn:agent-guide-begin").count();
        assert_eq!(count, 1, "block must not be duplicated");
    }

    #[test]
    fn test_wire_preserves_file_without_trailing_newline() {
        let dir = tempfile::tempdir().unwrap();
        // File with no trailing newline: wire must not mangle the last line.
        std::fs::write(dir.path().join("CLAUDE.md"), "# Rules").unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(result.code, 0, "wire should succeed: {}", result.stderr);

        let claude = std::fs::read_to_string(dir.path().join("CLAUDE.md")).unwrap();
        assert!(
            claude.starts_with("# Rules\n"),
            "wire must add a newline after content lacking one"
        );
    }

    #[test]
    fn test_wire_fails_without_guide() {
        let dir = tempfile::tempdir().unwrap();
        // No init, so .cairn/AGENTS.md does not exist.
        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(
            result.code, 1,
            "wire without a prior init must fail, not create a dangling reference"
        );
    }

    #[test]
    fn test_wire_rejects_absolute_path() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), Some("/etc/CLAUDE.md"));
        assert_eq!(
            result.code, 2,
            "absolute path must be rejected, not used to escape project root"
        );
    }

    #[test]
    fn test_wire_rejects_parent_dir_traversal() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), Some("../CLAUDE.md"));
        assert_eq!(
            result.code, 2,
            "path with '..' must be rejected, not used to escape project root"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_wire_rejects_symlink_target() {
        use std::os::unix::fs::symlink;
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        // Create a symlink at the auto-detect target pointing outside.
        let outside = tempfile::tempdir().unwrap();
        let link = dir.path().join("CLAUDE.md");
        symlink(outside.path().join("CLAUDE.md"), &link).unwrap();

        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(
            result.code, 1,
            "wire must reject a symlink target, not follow it"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_wire_rejects_symlinked_parent_directory() {
        use std::os::unix::fs::symlink;
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        // Create a symlinked directory inside the project pointing outside.
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), dir.path().join(".cursor")).unwrap();

        let result = wire_agent_guide(dir.path(), Some(".cursor/CLAUDE.md"));
        assert_eq!(
            result.code, 1,
            "wire must reject a symlinked parent directory, not follow it outside the project"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_wire_preserves_existing_file_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        // Create a CLAUDE.md with restrictive permissions.
        let path = dir.path().join("CLAUDE.md");
        std::fs::write(&path, "# Rules\n").unwrap();
        std::fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();

        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(result.code, 0, "wire should succeed: {}", result.stderr);

        let metadata = std::fs::metadata(&path).unwrap();
        assert_eq!(
            metadata.permissions().mode() & 0o777,
            0o600,
            "wire must preserve the existing file's permissions, not widen them"
        );
    }

    #[test]
    fn test_wire_rejects_scaffold_blueprint() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), Some("cairn.blueprint"));
        assert_eq!(result.code, 2, "must reject wiring into cairn.blueprint");
    }

    #[test]
    fn test_wire_rejects_scaffold_cairn_dir() {
        let dir = tempfile::tempdir().unwrap();
        init_project(dir.path(), false);

        let result = wire_agent_guide(dir.path(), Some(".cairn/AGENTS.md"));
        assert_eq!(
            result.code, 2,
            "must reject wiring into cairn's own .cairn/ directory"
        );
    }

    #[test]
    fn test_wire_rejects_guide_directory() {
        let dir = tempfile::tempdir().unwrap();
        // Create .cairn/AGENTS.md as a directory, not a file.
        std::fs::create_dir_all(dir.path().join(".cairn/AGENTS.md")).unwrap();

        let result = wire_agent_guide(dir.path(), None);
        assert_eq!(
            result.code, 1,
            "must reject when .cairn/AGENTS.md is a directory, not a file"
        );
    }
}
