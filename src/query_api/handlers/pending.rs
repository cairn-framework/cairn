//! Maintainer pending queue: decisions awaiting ratification.
//!
//! Every decision at `status: proposed`, oldest first, with its signed age,
//! nodes, and ratification tier, plus the parsed briefing surfaces: ruling
//! summary, rubric, local review evidence with tri-state hash comparison,
//! the changed-since-review marker, the ruling prompt, and the exact reopen
//! command. Local-tier rows expose the manifest hash a receipt must cover;
//! unavailable manifests render as `null` with an error message.
use super::pending_brief::{PendingRubric, parse_pending_brief};
use super::pending_evidence::PendingEvidence;
use crate::{
    artefacts::registry::{Decision, DecisionStatus},
    query_api::QueryError,
    scanner,
};
use serde_json::Value;

/// Seconds per civil day.
const SECS_PER_DAY: i64 = 86_400;

/// Wire form of the ratification tier, constrained in the committed schema.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum PendingTier {
    /// Machine-acceptable under the receipt protocol.
    Local,
    /// Maintainer-only, permanently.
    Binding,
}

impl PendingTier {
    /// Lowercase wire text, for the human renderer.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Binding => "binding",
        }
    }
}

impl From<crate::artefacts::registry::RatificationTier> for PendingTier {
    fn from(tier: crate::artefacts::registry::RatificationTier) -> Self {
        match tier {
            crate::artefacts::registry::RatificationTier::Local => Self::Local,
            crate::artefacts::registry::RatificationTier::Binding => Self::Binding,
        }
    }
}

/// One row of the maintainer pending queue: a decision at `status: proposed`.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PendingDecision {
    /// Decision id.
    pub id: String,
    /// Signed whole days between the decision's `date:` frontmatter and
    /// today; negative when the date lies in the future.
    pub age_days: i64,
    /// Node ids the decision references.
    pub nodes: Vec<String>,
    /// Parsed ratification tier; absent frontmatter defaults to `binding`.
    pub ratification: PendingTier,
    /// Current subject manifest for local-tier decisions, when it can be computed.
    pub subject_hash: Option<String>,
    /// Manifest construction failure for local-tier decisions, when one occurs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_hash_error: Option<String>,
    /// Plain summary of the ruling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruling_summary: Option<String>,
    /// Decision rubric, when the body carries a rubric heading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubric: Option<PendingRubric>,
    /// Review evidence for a local-tier decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<PendingEvidence>,
    /// True when any comparable review receipt hash differs from the current
    /// subject hash; receipts without a comparable hash never set it.
    pub changed_since_review: bool,
    /// Plain instruction for ruling on this decision in the session.
    pub ruling_prompt: String,
    /// Exact command that reproduces this briefing in full.
    pub reopen_command: String,
}

/// Wire shape of the `pending` query response.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PendingResponse {
    /// Proposed decisions, oldest first (age descending, ties by id
    /// ascending).
    pub pending: Vec<PendingDecision>,
}

/// Dispatch shim: builds the pending queue against the wall clock.
pub(crate) fn pending_json(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
    request: &super::super::QueryRequest,
) -> Result<Value, QueryError> {
    let mut response = pending_response(
        root,
        &scan_result.artefacts.decisions,
        &scan_result.artefacts.reviews,
        today_days(),
    )?;
    if let Some(id) = request.node.as_deref() {
        response.pending.retain(|row| row.id == id);
        if response.pending.is_empty() {
            return Err(QueryError {
                code: "CAIRN_COMMAND_FAILED".to_owned(),
                message: crate::copy::lookup("pending.not-found").replace("{id}", id),
                source_span: None,
                remediation: None,
            });
        }
    }
    Ok(serde_json::to_value(response).expect("PendingResponse serialises"))
}

/// Rows for the CLI human renderer; same computation the JSON wire carries.
pub(crate) fn pending_rows(
    root: &std::path::Path,
    scan_result: &scanner::ScanResult,
) -> Result<Vec<PendingDecision>, QueryError> {
    pending_response(
        root,
        &scan_result.artefacts.decisions,
        &scan_result.artefacts.reviews,
        today_days(),
    )
    .map(|r| r.pending)
}

/// Builds the queue from the decision and review sets, against an injected
/// `today` (days since the Unix epoch) so the arithmetic is testable.
fn pending_response(
    root: &std::path::Path,
    decisions: &[Decision],
    reviews: &[crate::artefacts::registry::Review],
    today: i64,
) -> Result<PendingResponse, QueryError> {
    let mut pending = decisions
        .iter()
        .filter(|decision| decision.status == DecisionStatus::Proposed)
        .map(|decision| {
            let days = date_to_days(&decision.date).ok_or_else(|| invalid_date(decision))?;
            let (subject_hash, subject_hash_error) = decision_subject_hash(root, decision);
            let local_tier =
                decision.ratification == crate::artefacts::registry::RatificationTier::Local;
            let brief =
                parse_pending_brief(root, decision, reviews, subject_hash.as_deref(), local_tier);
            Ok(PendingDecision {
                id: decision.id.clone(),
                age_days: today - days,
                nodes: decision.nodes.clone(),
                ratification: decision.ratification.into(),
                subject_hash,
                subject_hash_error,
                ruling_summary: brief.ruling_summary,
                rubric: brief.rubric,
                evidence: brief.evidence,
                changed_since_review: brief.changed_since_review,
                ruling_prompt: crate::copy::lookup("pending.ruling-prompt").to_owned(),
                reopen_command: reopen_command(decision),
            })
        })
        .collect::<Result<Vec<_>, QueryError>>()?;
    pending.sort_by(|a, b| b.age_days.cmp(&a.age_days).then_with(|| a.id.cmp(&b.id)));
    Ok(PendingResponse { pending })
}

fn reopen_command(decision: &Decision) -> String {
    // The briefing is the ruling surface: from any list this drills into the
    // full detail, while `cairn decisions <node>` (a bare id/status list)
    // recreates the search break the queue exists to remove.
    format!("cairn pending {}", decision.id)
}

fn decision_subject_hash(
    root: &std::path::Path,
    decision: &Decision,
) -> (Option<String>, Option<String>) {
    if decision.ratification != crate::artefacts::registry::RatificationTier::Local {
        return (None, None);
    }
    match crate::artefacts::registry::manifest::compute_decision_subject_hash(root, decision) {
        Ok(hash) => (Some(hash), None),
        Err(error) => (None, Some(error.message)),
    }
}
fn invalid_date(decision: &Decision) -> QueryError {
    QueryError {
        code: "CAIRN_PENDING_INVALID_DATE".to_owned(),
        message: crate::copy::lookup("pending.err-invalid-date")
            .replace("{id}", &decision.id)
            .replace("{value}", &decision.date),
        source_span: None,
        remediation: Some(crate::copy::lookup("pending.remediation-invalid-date").to_owned()),
    }
}

/// Whole days since the Unix epoch for the current UTC wall clock.
fn today_days() -> i64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Reason: dividing first keeps the value far below i64::MAX for any realistic clock.
    #[allow(clippy::cast_possible_wrap)]
    let days = (secs / SECS_PER_DAY as u64) as i64;
    days
}

/// Strict `YYYY-MM-DD` to whole days since the Unix epoch. Rejects malformed
/// shapes and out-of-range calendar components.
fn date_to_days(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    if !bytes
        .iter()
        .enumerate()
        .all(|(i, b)| matches!(i, 4 | 7) || b.is_ascii_digit())
    {
        return None;
    }
    let year: i64 = value[0..4].parse().ok()?;
    let month: i64 = value[5..7].parse().ok()?;
    let day: i64 = value[8..10].parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_len = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ][usize::try_from(month - 1).ok()?];
    if !(1..=month_len).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day))
}

/// Days from 1970-01-01 for a proleptic-Gregorian civil date (Howard
/// Hinnant's `days_from_civil`).
const fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
#[path = "pending_tests.rs"]
mod tests;
