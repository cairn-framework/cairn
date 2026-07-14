//! CLI hook command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use std::{fs, process::Command};

// Reason: setting executable bit is Unix-only; Windows ignores Unix permission bits.
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const CAIRN_MARKER: &str = "# Managed by Cairn. Do not edit.";

pub(crate) fn run_hook_command(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
    legacy_warning: String,
) -> CliResult {
    if matches!(
        parsed.command_args.get(1).map(String::as_str),
        Some("install" | "status" | "uninstall")
    ) {
        return run_hook_lifecycle(parsed, root);
    }
    let Some(kind) = parsed
        .command_args
        .get(1)
        .and_then(|value| parse_hook_kind(value))
    else {
        return err(2, copy::lookup("hooks.usage"));
    };
    let changes_dir = root.join(&parsed.changes_dir);
    let report = hooks::run(kind, root, &changes_dir, scan_result);
    CliResult {
        code: report.exit_code(),
        stdout: if parsed.json {
            hooks::render_json(&report)
        } else {
            hooks::render_human_verbose(&report, parsed.verbose)
        },
        stderr: legacy_warning,
    }
}

pub(crate) fn run_hook_lifecycle_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    run_hook_lifecycle(parsed, root)
}

fn run_hook_lifecycle(parsed: &ParsedArgs, root: &Path) -> CliResult {
    let operation = parsed.command_args[1].as_str();
    let hook_name = if parsed.command_args.iter().any(|arg| arg == "--pre-push") {
        "pre-push"
    } else {
        "pre-commit"
    };
    let result = hook_lifecycle(operation, hook_name, root);
    let command = format!("hook {operation}");
    match result {
        Ok((state, path)) => {
            if parsed.json {
                ok(format!(
                    "{{\"command\":\"{command}\",\"status\":\"ok\",\"data\":{{\"hook\":\"{hook_name}\",\"state\":\"{state}\",\"path\":{}}}}}\n",
                    serde_json::to_string(&path.to_string_lossy().to_string()).unwrap()
                ))
            } else {
                ok(format!("{state}: {hook_name} ({})\n", path.display()))
            }
        }
        Err(message) if parsed.json => CliResult {
            code: 1,
            stdout: format!(
                "{{\"command\":\"{command}\",\"status\":\"error\",\"error\":{}}}\n",
                serde_json::to_string(&message).unwrap()
            ),
            stderr: String::new(),
        },
        Err(message) => err(1, &message),
    }
}

fn hook_lifecycle(
    operation: &str,
    hook_name: &str,
    root: &Path,
) -> Result<(&'static str, PathBuf), String> {
    let repo_root = git_root(root)?;
    let hook_dir = git_path(&repo_root, "hooks")?;
    let path = hook_dir.join(hook_name);
    match operation {
        "status" => Ok((
            if is_cairn_hook(&path) {
                "installed"
            } else {
                "absent"
            },
            path,
        )),
        "install" => {
            if repo_root.join(".pre-commit-config.yaml").exists() {
                return Err(copy::lookup("hooks.pre-commit-conflict").to_owned());
            }
            if fs::symlink_metadata(&path).is_ok() {
                if is_cairn_hook(&path) {
                    return Ok(("installed", path));
                }
                return Err(format!(
                    "{}: {}",
                    copy::lookup("hooks.existing"),
                    path.display()
                ));
            }
            fs::create_dir_all(&hook_dir).map_err(|error| error.to_string())?;
            fs::write(&path, hook_script()).map_err(|error| error.to_string())?;
            #[cfg(unix)]
            {
                let mut permissions = fs::metadata(&path)
                    .map_err(|error| error.to_string())?
                    .permissions();
                permissions.set_mode(0o755);
                fs::set_permissions(&path, permissions).map_err(|error| error.to_string())?;
            }
            Ok(("installed", path))
        }
        "uninstall" => {
            if is_cairn_hook(&path) {
                fs::remove_file(&path).map_err(|error| error.to_string())?;
                return Ok(("removed", path));
            }
            if fs::symlink_metadata(&path).is_ok() {
                return Err(format!(
                    "{}: {}",
                    copy::lookup("hooks.existing"),
                    path.display()
                ));
            }
            Ok(("absent", path))
        }
        _ => Err(copy::lookup("hooks.usage").to_owned()),
    }
}

fn git_root(root: &Path) -> Result<PathBuf, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|error| format!("{}: {error}", copy::lookup("hooks.git-error")))?;
    if !output.status.success() {
        return Err(copy::lookup("hooks.git-error").to_owned());
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    ))
}

fn git_path(root: &Path, argument: &str) -> Result<PathBuf, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--git-path", argument])
        .output()
        .map_err(|error| format!("{}: {error}", copy::lookup("hooks.git-error")))?;
    if !output.status.success() {
        return Err(copy::lookup("hooks.git-error").to_owned());
    }
    let path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_owned());
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

fn hook_script() -> &'static str {
    "#!/bin/sh\n# Managed by Cairn. Do not edit.\nexec cairn hook all \"$@\"\n"
}
fn is_cairn_hook(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
        && fs::read_to_string(path).is_ok_and(|body| body.contains(CAIRN_MARKER))
}

pub(crate) fn parse_hook_kind(value: &str) -> Option<HookKind> {
    match value {
        "structural" => Some(HookKind::Structural),
        "interface" => Some(HookKind::Interface),
        "tension" => Some(HookKind::Tension),
        "architecture-decision" => Some(HookKind::ArchitectureDecision),
        "all" => Some(HookKind::All),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hook_kind_all_valid_strings() {
        assert_eq!(parse_hook_kind("structural"), Some(HookKind::Structural));
        assert_eq!(parse_hook_kind("interface"), Some(HookKind::Interface));
        assert_eq!(parse_hook_kind("tension"), Some(HookKind::Tension));
        assert_eq!(
            parse_hook_kind("architecture-decision"),
            Some(HookKind::ArchitectureDecision)
        );
        assert_eq!(parse_hook_kind("all"), Some(HookKind::All));
    }

    #[test]
    fn test_parse_hook_kind_unknown_returns_none() {
        assert!(parse_hook_kind("unknown").is_none());
        assert!(parse_hook_kind("Structural").is_none());
        assert!(parse_hook_kind("").is_none());
    }

    #[test]
    fn lifecycle_installs_is_idempotent_and_uninstalls_owned_hook() {
        let root = temp_git_root("lifecycle");
        Command::new("git")
            .args(["config", "--local", "core.hooksPath", ".git/hooks"])
            .current_dir(&root)
            .status()
            .unwrap();
        let install = make_parsed(&root, false, ["hook", "install"]);
        let first = run_hook_lifecycle_command(&install, &root);
        assert_eq!(
            first.code, 0,
            "stdout={} stderr={}",
            first.stdout, first.stderr
        );
        let path = root.join(".git/hooks/pre-commit");
        assert!(is_cairn_hook(&path));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&path).unwrap().permissions().mode() & 0o111,
                0o111
            );
        }
        let body = fs::read(&path).unwrap();
        assert_eq!(run_hook_lifecycle_command(&install, &root).code, 0);
        assert_eq!(fs::read(&path).unwrap(), body);
        assert!(
            run_hook_lifecycle_command(&make_parsed(&root, false, ["hook", "status"]), &root)
                .stdout
                .contains("installed")
        );
        assert_eq!(
            run_hook_lifecycle_command(&make_parsed(&root, false, ["hook", "uninstall"]), &root)
                .code,
            0
        );
        assert!(!path.exists());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn lifecycle_refuses_existing_hook_and_honours_pre_push_and_hooks_path() {
        let root = temp_git_root("safety");
        let custom = root.join("custom-hooks");
        Command::new("git")
            .args(["config", "core.hooksPath", "custom-hooks"])
            .current_dir(&root)
            .status()
            .unwrap();
        fs::create_dir_all(&custom).unwrap();
        fs::write(custom.join("pre-push"), "#!/bin/sh\n").unwrap();
        let install = make_parsed(&root, true, ["hook", "install", "--pre-push"]);
        assert_eq!(run_hook_lifecycle_command(&install, &root).code, 1);
        fs::remove_file(custom.join("pre-push")).unwrap();
        assert_eq!(run_hook_lifecycle_command(&install, &root).code, 0);
        assert!(custom.join("pre-push").exists());
        fs::write(root.join(".pre-commit-config.yaml"), "repos: []\n").unwrap();
        assert_eq!(
            run_hook_lifecycle_command(&make_parsed(&root, false, ["hook", "install"]), &root).code,
            1
        );
        let args = vec![
            "--json",
            "--file",
            root.join("missing.blueprint").to_str().unwrap(),
            "hook",
            "status",
            "--pre-push",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
        let result = crate::cli::run(&args);
        assert_eq!(result.code, 0);
        assert!(result.stdout.contains("\"command\":\"hook status\""));
        let _ = fs::remove_dir_all(root);
    }

    fn make_parsed<const N: usize>(root: &Path, json: bool, args: [&str; N]) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            file: root.join("missing.blueprint"),
            changes_dir: PathBuf::from("meta/changes"),
            command: "hook".to_owned(),
            command_args: args.into_iter().map(str::to_owned).collect(),
            verbose: false,
            brief: false,
        }
    }

    fn temp_git_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("cairn-hook-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        assert!(
            Command::new("git")
                .args(["init", "--quiet"])
                .current_dir(&root)
                .status()
                .unwrap()
                .success()
        );
        root
    }
}
