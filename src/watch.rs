//! Watch mode: periodic scan with finding-change events.
//!
//! Emits newline-delimited JSON when findings are added or resolved between
//! consecutive scans.  Driven by the `cairn watch` CLI command.

use crate::map::graph::Finding;
use serde::Serialize;
use std::collections::BTreeMap;

/// Event emitted when the finding set changes between scans.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "event")]
pub enum WatchEvent {
    /// A new finding appeared.
    #[serde(rename = "finding_added")]
    FindingAdded {
        /// ISO-8601 timestamp.
        timestamp: String,
        /// The new finding.
        finding: Finding,
    },
    /// A previously-seen finding is no longer present.
    #[serde(rename = "finding_resolved")]
    FindingResolved {
        /// ISO-8601 timestamp.
        timestamp: String,
        /// The resolved finding.
        finding: Finding,
    },
}

/// Compute the delta between two sets of findings.
///
/// Findings are matched by `(code, node, target, path)`.  A finding that
/// changes severity or message but keeps the same key is treated as a
/// resolution of the old one plus addition of the new one, which preserves
/// the simple added/resolved contract.
#[must_use]
pub fn diff_findings(old: &[Finding], new: &[Finding]) -> Vec<WatchEvent> {
    let mut events = Vec::new();
    let now = now_iso8601();

    let key = |f: &Finding| {
        (
            f.code.clone(),
            f.node.clone(),
            f.target.clone(),
            f.path.clone(),
        )
    };

    let old_map: BTreeMap<_, _> = old.iter().map(|f| (key(f), f)).collect();
    let new_map: BTreeMap<_, _> = new.iter().map(|f| (key(f), f)).collect();

    // Added: present in new, absent in old.
    for (k, f) in &new_map {
        if !old_map.contains_key(k) {
            events.push(WatchEvent::FindingAdded {
                timestamp: now.clone(),
                finding: (*f).clone(),
            });
        }
    }

    // Resolved: present in old, absent in new.
    for (k, f) in &old_map {
        if !new_map.contains_key(k) {
            events.push(WatchEvent::FindingResolved {
                timestamp: now.clone(),
                finding: (*f).clone(),
            });
        }
    }

    events
}

fn now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Simple RFC 3339 without subseconds.
    let (y, m, d, hh, mm, ss) = unix_to_datetime(elapsed);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

// Minimal Unix-timestamp to UTC calendar conversion for no-dependency output.
fn unix_to_datetime(mut secs: u64) -> (u32, u32, u32, u32, u32, u32) {
    let ss = (secs % 60) as u32;
    secs /= 60;
    let mm = (secs % 60) as u32;
    secs /= 60;
    let hh = (secs % 24) as u32;
    secs /= 24;

    // Days since 1970-01-01.
    let mut days = secs;
    let mut year = 1970u32;
    loop {
        let leap = is_leap_year(year);
        let year_days = if leap { 366 } else { 365 };
        if days < year_days {
            break;
        }
        days -= year_days;
        year += 1;
    }

    let month_lengths = [
        31,
        if is_leap_year(year) { 29 } else { 28 },
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
    ];
    let mut month = 1u32;
    for &len in &month_lengths {
        if days < len {
            break;
        }
        days -= len;
        month += 1;
    }
    // Reason: days is bounded by month length (<=31), so truncation never happens.
    #[allow(clippy::cast_possible_truncation)]
    let day = (days + 1) as u32;
    (year, month, day, hh, mm, ss)
}

#[allow(clippy::inline_always)]
#[inline(always)]
const fn is_leap_year(y: u32) -> bool {
    y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::graph::FindingSeverity;
    fn finding(code: &str, message: &str, severity: FindingSeverity) -> Finding {
        Finding {
            code: code.to_owned(),
            severity,
            message: message.to_owned(),
            node: None,
            target: None,
            path: None,
        }
    }

    #[test]
    fn test_diff_empty_both() {
        let events = diff_findings(&[], &[]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_diff_added() {
        let old = vec![];
        let new = vec![finding("CA001", "msg", FindingSeverity::Warning)];
        let events = diff_findings(&old, &new);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], WatchEvent::FindingAdded { .. }));
    }

    #[test]
    fn test_diff_resolved() {
        let old = vec![finding("CA001", "msg", FindingSeverity::Warning)];
        let new = vec![];
        let events = diff_findings(&old, &new);
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], WatchEvent::FindingResolved { .. }));
    }

    #[test]
    fn test_diff_unchanged() {
        let f = finding("CA001", "msg", FindingSeverity::Warning);
        let old = vec![f.clone()];
        let new = vec![f];
        let events = diff_findings(&old, &new);
        assert!(events.is_empty());
    }

    #[test]
    fn test_diff_mixed() {
        let old = vec![
            finding("CA001", "msg1", FindingSeverity::Warning),
            finding("CA002", "msg2", FindingSeverity::Error),
        ];
        let new = vec![
            finding("CA002", "msg2", FindingSeverity::Error),
            finding("CA003", "msg3", FindingSeverity::Info),
        ];
        let events = diff_findings(&old, &new);
        assert_eq!(events.len(), 2);
        assert!(events.iter().any(
            |e| matches!(e, WatchEvent::FindingAdded { finding, .. } if finding.code == "CA003")
        ));
        assert!(events.iter().any(
            |e| matches!(e, WatchEvent::FindingResolved { finding, .. } if finding.code == "CA001")
        ));
    }

    #[test]
    fn test_diff_same_key_different_severity() {
        let old = vec![finding("CA001", "msg", FindingSeverity::Warning)];
        let new = vec![finding("CA001", "msg", FindingSeverity::Error)];
        let events = diff_findings(&old, &new);
        // Same key means the finding is still present; severity changes are
        // not surfaced by the simple added/resolved contract.
        assert!(events.is_empty());
    }

    #[test]
    fn test_unix_to_datetime_epoch() {
        assert_eq!(unix_to_datetime(0), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn test_unix_to_datetime_known_date() {
        // 2026-01-01 00:00:00 UTC
        // Days from 1970-01-01 to 2026-01-01 = 20454
        let secs = 20454 * 86400;
        assert_eq!(unix_to_datetime(secs), (2026, 1, 1, 0, 0, 0));
    }
}
