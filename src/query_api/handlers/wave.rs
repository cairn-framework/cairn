//! The wave preview and its stats projection on the wire.
//!
//! `wave` renders the composed wave: units, write-sets, completeness
//! stamps, the hotspot permission holder, held units, and the plan digest.
//! No `active`, `expired`, `stale`, or `status` field appears anywhere;
//! staleness is reader-derived and `observed_at` is echoed verbatim.
//! `wave stats` renders the false-overlap projection over
//! `outcome.touched_files` facts, reader-side with no mutable counter; the
//! promotion threshold is deliberately unset until the first twenty
//! exclusions carry merge evidence.

use std::path::Path;

use serde_json::{Value, json};

use crate::artefacts::registry::dates::date_to_days;
use crate::coord::read::{StoreRead, read_facts};
use crate::map::paths::is_component_prefix;
use crate::query_api::wave::compose::compose_wave;
use crate::scanner;

use super::super::{QueryError, QueryRequest, QuerySince, SCHEMA_VERSION};

/// An RFC 3339 instant normalized to UTC for temporal comparisons.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Rfc3339Instant {
    seconds: i64,
    nanos: u32,
}

fn parse_digits(bytes: &[u8], start: usize, length: usize) -> Option<u32> {
    bytes
        .get(start..start.checked_add(length)?)?
        .iter()
        .try_fold(0_u32, |value, byte| {
            byte.is_ascii_digit()
                .then_some(value * 10 + u32::from(byte - b'0'))
        })
}

fn parse_rfc3339(value: &str) -> Option<Rfc3339Instant> {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes.get(10) != Some(&b'T')
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
    {
        return None;
    }
    let days = date_to_days(value.get(..10)?)?;
    let hour = parse_digits(bytes, 11, 2)?;
    let minute = parse_digits(bytes, 14, 2)?;
    let second = parse_digits(bytes, 17, 2)?;
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }

    let mut cursor = 19;
    let nanos = if bytes.get(cursor) == Some(&b'.') {
        cursor += 1;
        let start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        let length = cursor - start;
        if !(1..=9).contains(&length) {
            return None;
        }
        let fraction = parse_digits(bytes, start, length)?;
        fraction * 10_u32.pow(u32::try_from(9 - length).ok()?)
    } else {
        0
    };

    let offset_seconds = match bytes.get(cursor) {
        Some(b'Z') if cursor + 1 == bytes.len() => 0_i64,
        Some(sign @ (b'+' | b'-'))
            if cursor + 6 == bytes.len() && bytes.get(cursor + 3) == Some(&b':') =>
        {
            let hours = parse_digits(bytes, cursor + 1, 2)?;
            let minutes = parse_digits(bytes, cursor + 4, 2)?;
            if hours > 23 || minutes > 59 {
                return None;
            }
            let seconds = i64::from(hours * 3_600 + minutes * 60);
            if *sign == b'-' { -seconds } else { seconds }
        }
        _ => return None,
    };
    let local_seconds = days
        .checked_mul(86_400)?
        .checked_add(i64::from(hour * 3_600 + minute * 60 + second))?;
    Some(Rfc3339Instant {
        seconds: local_seconds.checked_sub(offset_seconds)?,
        nanos,
    })
}

// The stats window must compare instants, not their spellings. In particular,
// a fractional second sorts before `Z` lexically even though it is later.
fn compare_recorded_at(
    left: (Option<Rfc3339Instant>, &str),
    right: (Option<Rfc3339Instant>, &str),
) -> std::cmp::Ordering {
    match (left.0, right.0) {
        (Some(left_instant), Some(right_instant)) => left_instant
            .cmp(&right_instant)
            .then_with(|| left.1.cmp(right.1)),
        _ => left.1.cmp(right.1),
    }
}

/// The rolling window over exclusions with merge evidence.
const STATS_WINDOW: usize = 20;

fn coord_error(message: String) -> QueryError {
    QueryError {
        code: "CAIRN_COORD_READ_FAILED".to_owned(),
        message,
        source_span: None,
        remediation: None,
    }
}

fn wave_stats_since(request: &QueryRequest) -> Option<&str> {
    match request.since.as_ref() {
        Some(QuerySince::WaveStatsTimestamp(since)) => Some(since.as_str()),
        _ => None,
    }
}

/// `wave`: the dispatch preview and its plan digest.
///
/// # Errors
///
/// Fails closed when the coordination store is partially resolvable or a
/// unit's content hash cannot be computed.
pub(in crate::query_api) fn wave_json(
    root: &Path,
    scan_result: &scanner::ScanResult,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let wave = compose_wave(
        root,
        &scan_result.graph,
        &scan_result.artefacts.todos,
        request.at.as_deref(),
    )
    .map_err(coord_error)?;
    let units: Vec<Value> = wave
        .units
        .iter()
        .map(|unit| {
            json!({
                "id": unit.id,
                "content_hash": unit.content_hash,
                "hotspot_permission": unit.hotspot_permission,
                "write_set": {
                    "includes": unit.write_set.includes,
                    "excludes": unit.write_set.excludes,
                    "resolution": unit.write_set.resolution,
                    "unresolved_reason": unit.write_set.unresolved_reason,
                    "completeness": unit.write_set.completeness,
                    "completeness_reason": unit.write_set.completeness_reason,
                },
            })
        })
        .collect();
    let held: Vec<Value> = wave
        .held
        .iter()
        .map(|entry| {
            json!({
                "id": entry.id,
                "behind": entry.behind,
                "reason": entry.reason,
                "blocking_fact_id": entry.blocking_fact_id,
            })
        })
        .collect();
    Ok(json!({
        "schema_version": SCHEMA_VERSION,
        "store_state": if wave.store_ready { "ready" } else { "uninitialised" },
        "observed_at": request.at,
        "plan": wave.digest,
        "rule": wave.rule,
        "preimage": wave.preimage,
        "hotspot_prefixes": wave.hotspot_prefixes,
        "units": units,
        "held": held,
        "conflicts": [],
    }))
}

/// `wave stats`: the false-overlap projection.
///
/// An exclusion with merge evidence is an `outcome.touched_files` fact
/// whose payload carries `excluded_by_prefixes`; it is proven false when
/// none of its `files` fall under any of those prefixes.
///
/// # Errors
///
/// Fails closed when the coordination store is partially resolvable.
pub(in crate::query_api) fn wave_stats_json(
    root: &Path,
    request: &QueryRequest,
) -> Result<Value, QueryError> {
    let since = wave_stats_since(request).and_then(parse_rfc3339);
    let facts = match read_facts(root).map_err(coord_error)? {
        StoreRead::Uninitialised => {
            return Ok(json!({
                "schema_version": SCHEMA_VERSION,
                "store_state": "uninitialised",
                "observed_at": request.at,
                "window": { "size": 0, "cap": STATS_WINDOW },
                "exclusions_with_evidence": 0,
                "proven_false": 0,
                "false_overlap_rate": Value::Null,
                "threshold": Value::Null,
            }));
        }
        StoreRead::Ready(facts) => facts,
    };
    let mut evidenced: Vec<(Option<Rfc3339Instant>, &str, bool)> = facts
        .iter()
        .filter(|named| named.fact.kind == "outcome.touched_files")
        .filter_map(|named| {
            let recorded_at = named.fact.recorded_at.as_str();
            let instant = parse_rfc3339(recorded_at);
            if since.is_some_and(|since| instant.is_none_or(|recorded| recorded < since)) {
                return None;
            }
            let prefixes = named.fact.payload.get("excluded_by_prefixes")?.as_array()?;
            let files = named.fact.payload.get("files")?.as_array()?;
            let overlapped = files.iter().filter_map(Value::as_str).any(|file| {
                prefixes
                    .iter()
                    .filter_map(Value::as_str)
                    .any(|prefix| is_component_prefix(prefix, file))
            });
            Some((instant, recorded_at, !overlapped))
        })
        .collect();
    evidenced.sort_by(|left, right| compare_recorded_at((left.0, left.1), (right.0, right.1)));
    let window: Vec<bool> = evidenced
        .iter()
        .rev()
        .take(STATS_WINDOW)
        .map(|(_, _, proven_false)| *proven_false)
        .collect();
    let proven_false = window.iter().filter(|proven| **proven).count();
    let rate = if window.is_empty() {
        Value::Null
    } else {
        // The window is capped at twenty entries, so u32 always holds.
        let numerator = f64::from(u32::try_from(proven_false).unwrap_or(u32::MAX));
        let denominator = f64::from(u32::try_from(window.len()).unwrap_or(u32::MAX));
        json!(numerator / denominator)
    };
    Ok(json!({
        "schema_version": SCHEMA_VERSION,
        "store_state": "ready",
        "observed_at": request.at,
        "window": { "size": window.len(), "cap": STATS_WINDOW },
        "exclusions_with_evidence": window.len(),
        "proven_false": proven_false,
        "false_overlap_rate": rate,
        "threshold": Value::Null,
    }))
}

#[cfg(test)]
mod tests {
    use crate::coord::append::{NewFact, append_fact};
    use crate::coord::envelope::Actor;
    use std::path::Path;

    use super::*;

    fn repo() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["init", "-q"])
            .output()
            .expect("git runs");
        assert!(output.status.success());
        dir
    }

    fn record(root: &Path, recorded_at: &str, payload: serde_json::Value) {
        append_fact(
            root,
            NewFact {
                kind: "outcome.touched_files".to_owned(),
                recorded_at: recorded_at.to_owned(),
                recorded_by: Actor {
                    kind: "driver".to_owned(),
                    id: "t".to_owned(),
                },
                commit: "a".repeat(40),
                supersedes: None,
                payload,
            },
        )
        .expect("appends");
    }

    #[test]
    fn wave_stats_uses_its_timestamp_since_variant() {
        let timestamp = "2026-08-07T03:45:12Z";
        let request = QueryRequest {
            since: Some(QuerySince::WaveStatsTimestamp(timestamp.to_owned())),
            ..QueryRequest::default()
        };
        assert_eq!(wave_stats_since(&request), Some(timestamp));

        let request = QueryRequest {
            since: Some(QuerySince::CoordinationCursor("fact.json".to_owned())),
            ..QueryRequest::default()
        };
        assert_eq!(wave_stats_since(&request), None);
    }

    #[test]
    fn parse_rfc3339_normalizes_fraction_and_offset() {
        assert_eq!(
            parse_rfc3339("2026-08-07T03:45:12.500Z"),
            parse_rfc3339("2026-08-07T05:45:12.5+02:00")
        );
    }

    #[test]
    fn wave_stats_since_filters_out_older_touched_files_facts() {
        let dir = repo();
        record(
            dir.path(),
            "2026-08-07T03:00:00Z",
            serde_json::json!({
                "files": ["src/old.rs"],
                "excluded_by_prefixes": ["src/"],
            }),
        );
        record(
            dir.path(),
            "2026-08-07T05:00:00Z",
            serde_json::json!({
                "files": ["src/new.rs"],
                "excluded_by_prefixes": ["docs/"],
            }),
        );
        let request = QueryRequest {
            since: Some(QuerySince::WaveStatsTimestamp(
                "2026-08-07T04:00:00Z".to_owned(),
            )),
            ..QueryRequest::default()
        };
        let data = wave_stats_json(dir.path(), &request).expect("stats");
        assert_eq!(data["window"]["size"], 1);
        assert_eq!(data["proven_false"], 1);
        assert_eq!(data["false_overlap_rate"], 1.0);
    }

    #[test]
    fn wave_stats_since_includes_fractional_second_after_exact_boundary() {
        let dir = repo();
        record(
            dir.path(),
            "2026-08-07T03:45:12Z",
            serde_json::json!({
                "files": ["src/exact.rs"],
                "excluded_by_prefixes": ["src"],
            }),
        );
        record(
            dir.path(),
            "2026-08-07T03:45:12.500Z",
            serde_json::json!({
                "files": ["src/fractional.rs"],
                "excluded_by_prefixes": ["docs"],
            }),
        );
        let request = QueryRequest {
            since: Some(QuerySince::WaveStatsTimestamp(
                "2026-08-07T03:45:12Z".to_owned(),
            )),
            ..QueryRequest::default()
        };
        let data = wave_stats_json(dir.path(), &request).expect("stats");
        assert_eq!(data["window"]["size"], 2);
        assert_eq!(data["proven_false"], 1);
        assert_eq!(data["false_overlap_rate"], 0.5);
    }
}
