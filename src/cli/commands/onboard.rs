//! CLI onboard command implementation.
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn run_onboard_command(parsed: &ParsedArgs) -> CliResult {
    let root = parsed
        .file
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let (blueprint_path, _temp_dir) = if parsed.file.exists() {
        (parsed.file.clone(), None)
    } else {
        let dir = std::env::temp_dir().join(format!(
            "cairn-onboard-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        let _ = fs::create_dir_all(&dir);
        let stub = dir.join("cairn.blueprint");
        let _ = fs::write(&stub, "System Stub \"onboard stub\" id \"stub\" {\n}\n");
        (stub, Some(dir))
    };

    match crate::scanner::load_project(root, &blueprint_path) {
        Ok(result) => {
            let report = crate::brownfield::onboard::analyze(&result.graph.findings);
            let output = if parsed.json {
                let inner = crate::brownfield::onboard::render_json(&report);
                let inner = inner.trim();
                format!("{{\"command\":\"onboard\",\"status\":\"ok\",\"data\":{inner}}}\n")
            } else {
                crate::brownfield::onboard::render_human(&report)
            };
            CliResult {
                code: 0,
                stdout: output,
                stderr: String::new(),
            }
        }
        Err(error) => {
            if parsed.json {
                CliResult {
                    code: 1,
                    stdout: format!(
                        "{{\"command\":\"onboard\",\"status\":\"error\",\"data\":{{\"message\":\"{}\"}}}}\n",
                        format::esc(&error)
                    ),
                    stderr: String::new(),
                }
            } else {
                err(1, &error)
            }
        }
    }
}
