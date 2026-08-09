//! Scoring one authoring attempt with cairn's production surfaces.
//!
//! `cairn scan --strict` supplies the verdict (its exit status is the gate)
//! and `cairn lint --json` supplies the structured findings. No finding logic
//! is reimplemented here: the instrument measures the shipped surfaces, so
//! reproducing them would measure a copy instead.
//!
//! [`Finding`] mirrors the lint wire field for field. Repair feedback is the
//! previous scan verbatim, so a field dropped here is a field the model never
//! sees, and `deferred_by` and `parked_by` are exactly the context that tells a
//! model a finding is not its to fix.

use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::error::CairnError;

/// One finding as published by `cairn lint --json`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Finding {
    /// Severity string as published on the lint wire.
    pub(crate) severity: String,
    /// Finding code, e.g. `CAIRN_ARTEFACT_MISSING_FIELD`.
    pub(crate) code: String,
    /// Human-readable message.
    #[serde(default)]
    pub(crate) message: String,
    /// Node the finding names, when it names one.
    #[serde(default)]
    pub(crate) node: Option<String>,
    /// Path the finding names, when it names one.
    #[serde(default)]
    pub(crate) path: Option<String>,
    /// Decision that defers this finding, when one does.
    #[serde(default)]
    pub(crate) deferred_by: Option<String>,
    /// Todo that parks this finding, when one does.
    #[serde(default)]
    pub(crate) parked_by: Option<String>,
}

impl Finding {
    /// Sort rank for a severity string: errors first, then warnings, then the
    /// rest. Unknown severities sort last, so a wire change cannot silently
    /// reorder the feedback the repair loop sends back.
    fn severity_rank(&self) -> u8 {
        match self.severity.as_str() {
            "error" => 0,
            "warning" => 1,
            "info" => 2,
            _ => 3,
        }
    }
}

/// Verdict for one scored attempt.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ScanVerdict {
    /// True when `cairn scan --strict` exited 0.
    pub(crate) clean: bool,
    /// Findings from `cairn lint --json`, deterministically sorted.
    pub(crate) findings: Vec<Finding>,
}

/// Resolves the scoring binary to an absolute path.
///
/// Every scoring call sets the child's working directory to the scratch
/// workspace. A relative program *path* combined with `current_dir` resolves
/// platform-dependently, so it is absolutised once, against the process's own
/// working directory, before any spawn.
///
/// A bare program name carries no separator and is meant for a `PATH` lookup;
/// rewriting it to `cwd/name` would break that, so it is left alone.
pub(crate) fn absolute_bin(cairn_bin: &Utf8Path) -> Result<Utf8PathBuf, CairnError> {
    // Checked on the raw string, not on components: camino strips `.`, so
    // `./cairn` normalises to a single component and would be mistaken for a
    // bare name. A bare name is one carrying no separator at all.
    let bare_name = !cairn_bin.as_str().contains(['/', '\\']);
    if cairn_bin.is_absolute() || bare_name {
        return Ok(cairn_bin.to_path_buf());
    }

    let cwd = std::env::current_dir().map_err(|e| CairnError::AuthorEval {
        message: format!("could not resolve `{cairn_bin}` against the working directory: {e}"),
    })?;

    Utf8PathBuf::from_path_buf(cwd.join(cairn_bin)).map_err(|path| CairnError::AuthorEval {
        message: format!(
            "resolved cairn binary path `{}` is not utf-8",
            path.display()
        ),
    })
}

/// Scores the workspace by running both production surfaces inside it.
///
/// A non-zero exit from `lint` is not an error: blocking findings are the
/// measurement. A lint envelope without a `findings` key is an error, because
/// silently reading it as "no findings" would erase the repair feedback.
pub(crate) fn score(cairn_bin: &Utf8Path, workspace: &Utf8Path) -> Result<ScanVerdict, CairnError> {
    let scan = Command::new(cairn_bin)
        .args(["scan", "--strict"])
        .current_dir(workspace)
        .output()
        .map_err(|e| CairnError::AuthorEval {
            message: format!("`{cairn_bin} scan --strict` could not be run: {e}"),
        })?;

    let lint = Command::new(cairn_bin)
        .args(["lint", "--json"])
        .current_dir(workspace)
        .output()
        .map_err(|e| CairnError::AuthorEval {
            message: format!("`{cairn_bin} lint --json` could not be run: {e}"),
        })?;

    let wire: LintWire =
        serde_json::from_slice(&lint.stdout).map_err(|e| CairnError::AuthorEval {
            message: format!(
                "`{cairn_bin} lint --json` produced unreadable output: {e}; stderr: {}",
                String::from_utf8_lossy(&lint.stderr)
            ),
        })?;

    let Some(mut findings) = wire.findings else {
        return Err(CairnError::AuthorEval {
            message: format!(
                "`{cairn_bin} lint --json` published no `findings` key; refusing to read that as a clean scan"
            ),
        });
    };

    findings.sort_by(|a, b| {
        a.severity_rank()
            .cmp(&b.severity_rank())
            .then_with(|| a.code.cmp(&b.code))
            .then_with(|| a.path.cmp(&b.path))
            .then_with(|| a.node.cmp(&b.node))
            .then_with(|| a.message.cmp(&b.message))
    });

    Ok(ScanVerdict {
        clean: scan.status.success(),
        findings,
    })
}

#[derive(Deserialize)]
struct LintWire {
    findings: Option<Vec<Finding>>,
}
