//! Pending-queue detail renderer: briefing, rubric, and review evidence.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::query_api::PendingDecision;
pub(crate) fn render_pending_detail(row: &PendingDecision) -> String {
    use std::fmt::Write as _;

    let mut out = format!(
        "{}\n",
        copy::lookup("pending.detail-header")
            .replace("{id}", &row.id)
            .replace("{age}", &row.age_days.to_string())
    );
    if let Some(summary) = row.ruling_summary.as_deref() {
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("pending.ruling").replace("{summary}", summary)
        );
    }
    if let Some(rubric) = row.rubric.as_ref() {
        out.push_str(copy::lookup("pending.briefing"));
        out.push('\n');
        if let Some(tier) = rubric.tier.as_deref() {
            let _ = writeln!(
                out,
                "{}",
                copy::lookup("pending.rubric-tier").replace("{value}", tier)
            );
        }
        render_rubric_section(
            &mut out,
            "unblocks",
            rubric.unblocks.as_deref(),
            "pending.rubric-section",
        );
        render_rubric_section(
            &mut out,
            "alignment",
            rubric.alignment.as_deref(),
            "pending.rubric-section",
        );
        render_rubric_section(
            &mut out,
            "options",
            rubric.options.as_deref(),
            "pending.rubric-section",
        );
    }
    render_pending_evidence(&mut out, row);
    if row.changed_since_review {
        out.push_str(copy::lookup("pending.changed"));
        out.push('\n');
    }
    let _ = writeln!(
        out,
        "{}",
        copy::lookup("pending.next-action").replace("{prompt}", &row.ruling_prompt)
    );
    let _ = writeln!(
        out,
        "{}",
        copy::lookup("pending.reopen").replace("{command}", &row.reopen_command)
    );
    out
}

fn render_rubric_section(out: &mut String, label: &str, values: Option<&[String]>, key: &str) {
    use std::fmt::Write as _;
    let Some(values) = values else {
        return;
    };
    let label = match label {
        "unblocks" => copy::lookup("webui.channel.pending-unblocks"),
        "alignment" => copy::lookup("webui.channel.pending-alignment"),
        _ => copy::lookup("webui.channel.pending-options"),
    };
    let _ = writeln!(out, "{}", copy::lookup(key).replace("{label}", label));
    for value in values {
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("pending.rubric-item").replace("{value}", value)
        );
    }
}

fn render_pending_evidence(out: &mut String, row: &PendingDecision) {
    use std::fmt::Write as _;
    let Some(evidence) = row.evidence.as_ref() else {
        return;
    };
    out.push_str(copy::lookup("pending.evidence"));
    out.push('\n');
    if evidence.receipts.is_empty() {
        out.push_str(copy::lookup("pending.evidence-none"));
        out.push('\n');
        return;
    }
    for receipt in &evidence.receipts {
        let reviewer = receipt
            .reviewer
            .as_deref()
            .unwrap_or_else(|| copy::lookup("pending.evidence-reviewer-unknown"));
        let verdict = receipt
            .verdict
            .as_deref()
            .unwrap_or_else(|| copy::lookup("pending.evidence-verdict-unknown"));
        let matched = match receipt.subject_hash_matches {
            Some(true) => copy::lookup("pending.evidence-match"),
            Some(false) => copy::lookup("pending.evidence-mismatch"),
            None => copy::lookup("pending.evidence-unverified"),
        };
        let _ = writeln!(
            out,
            "{}",
            copy::lookup("pending.evidence-receipt")
                .replace("{stem}", &receipt.stem)
                .replace("{reviewer}", reviewer)
                .replace("{verdict}", verdict)
                .replace("{match}", matched)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query_api::{PendingEvidence, PendingReceipt, PendingTier};

    #[test]
    fn unknown_hash_receipt_renders_the_unverified_label() {
        let row = PendingDecision {
            id: "dec.subject".to_owned(),
            age_days: 1,
            nodes: vec!["app".to_owned()],
            ratification: PendingTier::Local,
            subject_hash: Some("sha256:aaa".to_owned()),
            subject_hash_error: None,
            ruling_summary: None,
            rubric: None,
            evidence: Some(PendingEvidence {
                receipts: vec![PendingReceipt {
                    stem: "rev.mystery".to_owned(),
                    reviewer: None,
                    verdict: None,
                    subject_hash_matches: None,
                }],
            }),
            changed_since_review: false,
            ruling_prompt: "Say your ruling in this session.".to_owned(),
            reopen_command: "cairn pending dec.subject".to_owned(),
        };
        let rendered = render_pending_detail(&row);
        assert!(
            rendered.contains("could not be checked against the current version"),
            "unknown comparison renders the unverified label: {rendered}"
        );
        assert!(
            !rendered.contains("does not match current reviewed material"),
            "unknown must not claim a mismatch: {rendered}"
        );
    }
}
