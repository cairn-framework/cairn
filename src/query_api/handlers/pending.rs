//! Maintainer pending queue: decisions awaiting ratification.
//!
//! Typed data only (`todo.maintainer-pending-queue` v1): every decision at
//! `status: proposed`, with its signed age in whole days, its nodes, and its
//! ratification tier. The `ratification:` frontmatter field does not exist in
//! the artefact schema yet (`todo.decision-ratification-tiers` owns it), so
//! every row renders that todo's documented default, `binding`.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Ratification tier rendered while the artefact schema has no
/// `ratification:` field: absent means `binding`.
const RATIFICATION_DEFAULT: &str = "binding";

/// Seconds per civil day.
const SECS_PER_DAY: i64 = 86_400;

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
    /// Ratification tier; `binding` when the artefact declares none.
    pub ratification: String,
}

/// Wire shape of the `pending` query response.
#[derive(Clone, Debug, serde::Serialize, schemars::JsonSchema)]
pub struct PendingResponse {
    /// Proposed decisions, oldest first (age descending, ties by id
    /// ascending).
    pub pending: Vec<PendingDecision>,
}

/// Dispatch shim: builds the pending queue against the wall clock.
pub(crate) fn pending_json(scan_result: &scanner::ScanResult) -> Result<Value, QueryError> {
    let response = pending_response(&scan_result.artefacts.decisions, today_days())?;
    Ok(serde_json::to_value(response).expect("PendingResponse serialises"))
}

/// Rows for the CLI human renderer; same computation the JSON wire carries.
pub(crate) fn pending_rows(
    scan_result: &scanner::ScanResult,
) -> Result<Vec<PendingDecision>, QueryError> {
    pending_response(&scan_result.artefacts.decisions, today_days()).map(|r| r.pending)
}

/// Builds the queue from the decision set alone, against an injected `today`
/// (days since the Unix epoch) so the arithmetic is testable.
fn pending_response(decisions: &[Decision], today: i64) -> Result<PendingResponse, QueryError> {
    let mut pending = decisions
        .iter()
        .filter(|decision| decision.status == DecisionStatus::Proposed)
        .map(|decision| {
            let days = date_to_days(&decision.date).ok_or_else(|| invalid_date(decision))?;
            Ok(PendingDecision {
                id: decision.id.clone(),
                age_days: today - days,
                nodes: decision.nodes.clone(),
                ratification: RATIFICATION_DEFAULT.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, QueryError>>()?;
    pending.sort_by(|a, b| b.age_days.cmp(&a.age_days).then_with(|| a.id.cmp(&b.id)));
    Ok(PendingResponse { pending })
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
mod tests {
    use super::*;

    fn decision(id: &str, status: DecisionStatus, date: &str, nodes: &[&str]) -> Decision {
        Decision {
            id: id.to_owned(),
            path: format!("meta/decisions/{id}.md"),
            nodes: nodes.iter().map(|n| (*n).to_owned()).collect(),
            status,
            date: date.to_owned(),
            revisited: None,
            revisit_triggers: Vec::new(),
            informed_by: Vec::new(),
            supersedes: Vec::new(),
            refines: Vec::new(),
            related: Vec::new(),
            orphaned: false,
            orphan_reason: None,
            gap: false,
            claims: None,
            body: String::new(),
        }
    }

    #[test]
    fn test_days_from_civil_known_values() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(1970, 1, 2), 1);
        assert_eq!(days_from_civil(2000, 3, 1), 11_017);
        assert_eq!(days_from_civil(2026, 7, 30), 20_664);
        assert_eq!(days_from_civil(1969, 12, 31), -1);
    }

    #[test]
    fn test_date_to_days_rejects_malformed_values() {
        for value in [
            "2026-7-30",
            "2026/07/30",
            "2026-13-01",
            "2026-00-10",
            "2026-02-30",
            "2025-02-29",
            "+026-01-01",
            "yesterday!!",
            "2026-07-30T00:00:00Z",
            "",
        ] {
            assert!(date_to_days(value).is_none(), "{value:?} must not parse");
        }
        assert_eq!(
            date_to_days("2024-02-29"),
            Some(days_from_civil(2024, 2, 29))
        );
    }

    #[test]
    fn test_pending_lists_only_proposed_oldest_first() {
        let today = days_from_civil(2026, 7, 30);
        let decisions = [
            decision("dec.newer", DecisionStatus::Proposed, "2026-07-20", &["a"]),
            decision(
                "dec.accepted",
                DecisionStatus::Accepted,
                "2026-01-01",
                &["a"],
            ),
            decision(
                "dec.older",
                DecisionStatus::Proposed,
                "2026-07-01",
                &["a", "b"],
            ),
            decision(
                "dec.superseded",
                DecisionStatus::Superseded,
                "2026-01-01",
                &["a"],
            ),
            decision(
                "dec.deprecated",
                DecisionStatus::Deprecated,
                "2026-01-01",
                &["a"],
            ),
        ];
        let response = pending_response(&decisions, today).unwrap();
        let ids: Vec<&str> = response.pending.iter().map(|row| row.id.as_str()).collect();
        assert_eq!(ids, ["dec.older", "dec.newer"]);
        assert_eq!(response.pending[0].age_days, 29);
        assert_eq!(response.pending[0].nodes, ["a", "b"]);
        assert_eq!(response.pending[1].age_days, 10);
        for row in &response.pending {
            assert_eq!(row.ratification, "binding");
        }
    }

    #[test]
    fn test_pending_age_ties_break_by_id_ascending() {
        let today = days_from_civil(2026, 7, 30);
        let decisions = [
            decision("dec.zeta", DecisionStatus::Proposed, "2026-07-10", &["a"]),
            decision("dec.alpha", DecisionStatus::Proposed, "2026-07-10", &["a"]),
        ];
        let response = pending_response(&decisions, today).unwrap();
        let ids: Vec<&str> = response.pending.iter().map(|row| row.id.as_str()).collect();
        assert_eq!(ids, ["dec.alpha", "dec.zeta"]);
    }

    #[test]
    fn test_pending_future_date_yields_negative_age_and_sorts_last() {
        let today = days_from_civil(2026, 7, 30);
        let decisions = [
            decision("dec.future", DecisionStatus::Proposed, "2026-08-04", &["a"]),
            decision("dec.past", DecisionStatus::Proposed, "2026-07-25", &["a"]),
        ];
        let response = pending_response(&decisions, today).unwrap();
        let ids: Vec<&str> = response.pending.iter().map(|row| row.id.as_str()).collect();
        assert_eq!(ids, ["dec.past", "dec.future"]);
        assert_eq!(response.pending[1].age_days, -5);
    }

    #[test]
    fn test_pending_invalid_date_is_a_deterministic_error() {
        let decisions = [decision(
            "dec.bad",
            DecisionStatus::Proposed,
            "not-a-date!",
            &["a"],
        )];
        let error = pending_response(&decisions, 0).unwrap_err();
        assert_eq!(error.code, "CAIRN_PENDING_INVALID_DATE");
        assert!(error.message.contains("dec.bad"), "{}", error.message);
        assert!(error.message.contains("not-a-date!"), "{}", error.message);
        assert!(error.remediation.is_some());
    }

    #[test]
    fn test_pending_ignores_invalid_dates_on_non_proposed_decisions() {
        let decisions = [
            decision("dec.done", DecisionStatus::Accepted, "garbage-date", &["a"]),
            decision("dec.live", DecisionStatus::Proposed, "2026-07-01", &["a"]),
        ];
        let response = pending_response(&decisions, days_from_civil(2026, 7, 30)).unwrap();
        assert_eq!(response.pending.len(), 1);
        assert_eq!(response.pending[0].id, "dec.live");
    }
}
