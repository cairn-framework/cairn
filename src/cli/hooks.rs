//! Hook engine for Phase 4 hooks.

use std::path::PathBuf;

use crate::map::graph::{Finding, FindingSeverity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HookKind {
    Structural,
    Interface,
    Tension,
    All,
}

impl HookKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "structural" => Some(Self::Structural),
            "interface" => Some(Self::Interface),
            "tension" => Some(Self::Tension),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::Interface => "interface",
            Self::Tension => "tension",
            Self::All => "all",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExitDecision {
    Pass,
    Fail,
}

impl From<ExitDecision> for u8 {
    fn from(decision: ExitDecision) -> Self {
        match decision {
            ExitDecision::Pass => 0,
            ExitDecision::Fail => 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HookReport {
    pub kind: HookKind,
    pub findings: Vec<Finding>,
    pub conflicts: Vec<Finding>,
    pub exit_decision: ExitDecision,
    pub elapsed_ms: u64,
    pub output_paths: Vec<PathBuf>,
}

impl HookReport {
    pub fn new(kind: HookKind) -> Self {
        Self {
            kind,
            findings: Vec::new(),
            conflicts: Vec::new(),
            exit_decision: ExitDecision::Pass,
            elapsed_ms: 0,
            output_paths: Vec::new(),
        }
    }

    pub fn with_findings(mut self, findings: Vec<Finding>) -> Self {
        self.findings = findings;
        self
    }

    pub fn with_conflicts(mut self, conflicts: Vec<Finding>) -> Self {
        self.conflicts = conflicts;
        self
    }

    pub fn with_elapsed_ms(mut self, ms: u64) -> Self {
        self.elapsed_ms = ms;
        self
    }

    pub fn with_output_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.output_paths = paths;
        self
    }

    pub fn compute_exit_decision(mut self) -> Self {
        self.exit_decision = determine_exit_decision(&self.kind, &self.findings, &self.conflicts);
        self
    }

    pub fn render_json(&self) -> String {
        let findings_json = self
            .findings
            .iter()
            .map(finding_json)
            .collect::<Vec<_>>()
            .join(",");
        let conflicts_json = self
            .conflicts
            .iter()
            .map(finding_json)
            .collect::<Vec<_>>()
            .join(",");
        let output_paths_json = self
            .output_paths
            .iter()
            .map(|p| format!("\"{}\"", esc(p.to_string_lossy().as_ref())))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"kind\":\"{}\",\"findings\":[{}],\"conflicts\":[{}],\"exit_decision\":\"{:?}\",\"elapsed_ms\":{},\"output_paths\":[{}]}}\n",
            self.kind.as_str(),
            findings_json,
            conflicts_json,
            self.exit_decision,
            self.elapsed_ms,
            output_paths_json
        )
    }

    pub fn render_human(&self) -> String {
        let blocks = matches!(self.exit_decision, ExitDecision::Fail);
        let findings_count = self.findings.len();
        let conflicts_count = self.conflicts.len();
        let mut output = format!(
            "Hook: {}\nBlocks: {}\nFindings: {}\n",
            self.kind.as_str(),
            blocks,
            findings_count
        );
        if conflicts_count > 0 {
            output.push_str(&format!("Conflicts: {}\n", conflicts_count));
        }
        output.push_str(&format!("Elapsed: {}ms\n", self.elapsed_ms));
        if !self.findings.is_empty() {
            output.push_str("\n");
            for finding in &self.findings {
                output.push_str(&format!(
                    "{:?}: {} {}\n",
                    finding.severity, finding.code, finding.message
                ));
            }
        }
        if !self.conflicts.is_empty() {
            output.push_str("\nConflicts:\n");
            for conflict in &self.conflicts {
                output.push_str(&format!(
                    "{:?}: {} {}\n",
                    conflict.severity, conflict.code, conflict.message
                ));
            }
        }
        output
    }
}

pub fn determine_exit_decision(
    kind: &HookKind,
    findings: &[Finding],
    conflicts: &[Finding],
) -> ExitDecision {
    let has_errors = findings
        .iter()
        .any(|f| f.severity == FindingSeverity::Error);
    let has_conflicts = !conflicts.is_empty();

    match kind {
        HookKind::Structural => {
            if has_errors || has_conflicts {
                ExitDecision::Fail
            } else {
                ExitDecision::Pass
            }
        }
        HookKind::Interface => {
            if has_errors || has_conflicts {
                ExitDecision::Fail
            } else {
                ExitDecision::Pass
            }
        }
        HookKind::Tension => ExitDecision::Pass,
        HookKind::All => {
            if has_errors || has_conflicts {
                ExitDecision::Fail
            } else {
                ExitDecision::Pass
            }
        }
    }
}

fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{:?}\",\"message\":\"{}\"}}",
        esc(&finding.code),
        finding.severity,
        esc(&finding.message)
    )
}

pub fn errors_from_findings(findings: &[Finding]) -> Vec<Finding> {
    findings
        .iter()
        .filter(|f| f.severity == FindingSeverity::Error)
        .cloned()
        .collect()
}

pub fn warnings_from_findings(findings: &[Finding]) -> Vec<Finding> {
    findings
        .iter()
        .filter(|f| f.severity == FindingSeverity::Warning)
        .cloned()
        .collect()
}
