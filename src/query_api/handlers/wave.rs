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

use crate::coord::read::{StoreRead, read_facts};
use crate::map::paths::is_component_prefix;
use crate::query_api::wave::compose::compose_wave;
use crate::scanner;

use super::super::{QueryError, QueryRequest, SCHEMA_VERSION};

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
    let mut evidenced: Vec<(&str, bool)> = facts
        .iter()
        .filter(|named| named.fact.kind == "outcome.touched_files")
        .filter(|named| {
            request
                .since
                .as_deref()
                .is_none_or(|since| named.fact.recorded_at.as_str() >= since)
        })
        .filter_map(|named| {
            let prefixes = named.fact.payload.get("excluded_by_prefixes")?.as_array()?;
            let files = named.fact.payload.get("files")?.as_array()?;
            let overlapped = files.iter().filter_map(Value::as_str).any(|file| {
                prefixes
                    .iter()
                    .filter_map(Value::as_str)
                    .any(|prefix| is_component_prefix(prefix, file))
            });
            Some((named.fact.recorded_at.as_str(), !overlapped))
        })
        .collect();
    evidenced.sort_by_key(|(recorded_at, _)| *recorded_at);
    let window: Vec<bool> = evidenced
        .iter()
        .rev()
        .take(STATS_WINDOW)
        .map(|(_, proven_false)| *proven_false)
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
