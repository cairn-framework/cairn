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
//! model a finding is not its to fix. The one finding the wire did not publish
//! is the one [`ErrorEnvelope`] stands in for.

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
    /// True only for the finding synthesised from an `error` envelope.
    ///
    /// Never serialised and never read from the wire: `lint` publishes no such
    /// field, and the repair feedback must mirror the wire exactly. It exists
    /// so the taxonomy can tell a generic envelope wrapper apart from an
    /// untabled code the wire really published.
    #[serde(skip)]
    pub(crate) from_envelope: bool,
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

    /// The span of a synthesised envelope whose message begins
    /// `<span>:line:col:`, and nothing otherwise.
    ///
    /// The span alone is too weak, because `cairn lint --json` labels every
    /// project load failure with the blueprint path. Only a reported position
    /// distinguishes a parse failure from a corrupt state snapshot. Full rule
    /// in `meta/contracts/authoreval.md`.
    pub(crate) fn envelope_parse_span(&self) -> Option<&str> {
        let span = self
            .from_envelope
            .then_some(self.path.as_deref())
            .flatten()?;
        let rest = self.message.strip_prefix(span)?.strip_prefix(':')?;
        let (line, rest) = rest.split_once(':')?;
        let (column, _) = rest.split_once(':')?;
        (is_number(line) && is_number(column)).then_some(span)
    }
}

/// Whether a message segment is a decimal position, as a parse error reports.
fn is_number(segment: &str) -> bool {
    !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit())
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
/// measurement. An envelope publishing an `error` instead of `findings` is not
/// an error either: `lint` reports the response's own defect that way (an
/// unparseable blueprint is the common case), so it becomes a dirty verdict
/// carrying one synthesised finding built from cairn's own code and message.
/// An envelope publishing neither key is an instrument fault, because silently
/// reading it as "no findings" would erase the repair feedback.
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

    // A `null` under either name reads as absence, so a wire the future gives
    // an always-present `error: null` still scores normally. What matters is
    // which of the two is populated.
    let mut findings = match (wire.findings, wire.error) {
        // Cairn populates one or the other, never both. A wire carrying both is
        // one cairn cannot have produced, and guessing which half is
        // authoritative could score a failed workspace clean.
        (Some(_), Some(_)) => {
            return Err(CairnError::AuthorEval {
                message: format!(
                    "`{cairn_bin} lint --json` published both findings and an error; refusing to guess which one is the verdict"
                ),
            });
        }
        (Some(findings), None) => findings,
        // The workspace was unreadable, so `scan`'s exit status describes
        // nothing: the verdict is dirty regardless, and the envelope is the
        // only thing the model can repair from.
        (None, Some(envelope)) => {
            return Ok(ScanVerdict {
                clean: false,
                findings: vec![envelope.into_finding()],
            });
        }
        (None, None) => {
            return Err(CairnError::AuthorEval {
                message: format!(
                    "`{cairn_bin} lint --json` published neither findings nor an error; refusing to read that as a clean scan"
                ),
            });
        }
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
    error: Option<ErrorEnvelope>,
}

/// The error envelope `cairn lint --json` publishes when it cannot read the
/// workspace at all.
#[derive(Deserialize)]
struct ErrorEnvelope {
    code: String,
    #[serde(default)]
    message: String,
    #[serde(default)]
    source_span: Option<String>,
}

impl ErrorEnvelope {
    /// The envelope as the one finding it stands for.
    ///
    /// This is not the wire verbatim: the envelope has no severity, and its
    /// `remediation` has nowhere to go. Cairn's own code and message are what
    /// carry through unchanged, because they are what the model repairs from.
    /// `source_span` becomes the finding's path, which is what the taxonomy
    /// attributes by.
    fn into_finding(self) -> Finding {
        Finding {
            severity: "error".to_owned(),
            code: self.code,
            message: self.message,
            node: None,
            path: self.source_span,
            deferred_by: None,
            parked_by: None,
            from_envelope: true,
        }
    }
}
