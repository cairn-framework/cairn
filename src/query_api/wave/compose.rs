//! The wave composer and the plan identity
//! (`res.parallel-dispatch-rung-3` Part 1, clause 1).
//!
//! One composer, two callers: the console preview and the driver's re-read
//! consume this through the same passive query, so recompute-equality is
//! trustworthy. Until workflow artefacts land, the composer rule id is the
//! constant [`DEFAULT_RULE_ID`]: the single built-in rule (ready units,
//! rank-ordered, disjoint write-sets, one hotspot permission per wave in
//! topological-rank-then-id order). Workflow artefacts
//! (`todo.driver-in-repo` task 2) replace it later; the digest changes when
//! they do, and `ruleset-changed` is a decline reason by design.
//!
//! Composition reads the working tree. The base commit is deliberately
//! absent from the hashed preimage: recompute-equality replaces
//! commit-pinning, and the commit is recorded on the ruling fact envelope
//! as provenance, never compared.

use std::path::Path;

use crate::artefacts::registry::sha256::sha256_hex;
use crate::artefacts::registry::types::{Todo, TodoStatus};
use crate::coord::read::{NamedFact, StoreRead, held, read_facts};
use crate::map::graph::Graph;

use super::{WriteSet, derive_write_set, hotspot_prefixes, write_sets_disjoint};

/// The pre-decided stand-in rule id until workflow artefacts exist.
pub(crate) const DEFAULT_RULE_ID: &str = "wf.default:1";

/// One unit admitted to the wave.
#[derive(Clone, Debug)]
pub(crate) struct WaveUnit {
    /// Todo stem, e.g. `todo.driver-in-repo`.
    pub id: String,
    /// First 12 hex of SHA-256 over the todo file bytes at composition.
    pub content_hash: String,
    /// The derived write surface.
    pub write_set: WriteSet,
    /// True for the one unit per wave holding the hotspot permission.
    pub hotspot_permission: bool,
}

/// A ready unit held out of this wave, with its reason.
#[derive(Clone, Debug)]
pub(crate) struct HeldUnit {
    /// Todo stem.
    pub id: String,
    /// The wave unit it queues behind, when the cause is another unit.
    pub behind: Option<String>,
    /// `write-sets-overlap`, `runs-alone`, `parked`, or `lease-held`.
    pub reason: &'static str,
    /// The blocking fact id for `parked` and `lease-held`.
    pub blocking_fact_id: Option<String>,
}

/// A composed wave and its content-addressed identity.
#[derive(Clone, Debug)]
pub(crate) struct Wave {
    /// The composer rule id hashed into the preimage.
    pub rule: &'static str,
    /// Units dispatched together.
    pub units: Vec<WaveUnit>,
    /// Ready units held out, with reasons.
    pub held: Vec<HeldUnit>,
    /// The canonical preimage text.
    pub preimage: String,
    /// `plan-` plus the first 16 hex of SHA-256 over the preimage.
    pub digest: String,
    /// The hotspot prefixes phase 0 cannot attribute.
    pub hotspot_prefixes: Vec<String>,
    /// Whether the coordination store had been initialised at composition.
    pub store_ready: bool,
}

/// Todo stem (`todo.<slug>`) from an artefact path.
fn todo_stem(todo: &Todo) -> Option<String> {
    let name = Path::new(&todo.path).file_name()?.to_str()?;
    name.strip_suffix(".md").map(ToOwned::to_owned)
}

/// True when `unit` is subtracted from the ready set by an unmatched
/// `ruling.park`.
fn parked<'a>(facts: &'a [NamedFact], unit: &str) -> Option<&'a NamedFact> {
    let park = facts
        .iter()
        .filter(|named| {
            named.fact.kind == "ruling.park"
                && named
                    .fact
                    .payload
                    .get("target")
                    .and_then(serde_json::Value::as_str)
                    == Some(unit)
        })
        .max_by(|a, b| {
            (a.fact.recorded_at.as_str(), a.fact.fact_id.as_str())
                .cmp(&(b.fact.recorded_at.as_str(), b.fact.fact_id.as_str()))
        })?;
    let unparked = facts.iter().any(|named| {
        named.fact.kind == "ruling.unpark"
            && named
                .fact
                .payload
                .get("target")
                .and_then(serde_json::Value::as_str)
                == Some(unit)
            && named.fact.recorded_at >= park.fact.recorded_at
    });
    (!unparked).then_some(park)
}

/// Composes the wave from committed graph state, todo artefacts, and the
/// coordination facts.
///
/// `at` is the caller's observation instant: leases are evaluated only when
/// it is supplied, because the core consults no clock.
///
/// # Errors
///
/// Fails closed when the coordination store is partially resolvable or a
/// ready todo's file bytes cannot be read for its content hash.
pub(crate) fn compose_wave(
    root: &Path,
    graph: &Graph,
    todos: &[Todo],
    at: Option<&str>,
) -> Result<Wave, String> {
    let (facts, store_ready) = match read_facts(root)? {
        StoreRead::Uninitialised => (Vec::new(), false),
        StoreRead::Ready(facts) => (facts, true),
    };

    // Ready: open, with every declared blocker done or absent.
    let by_stem: std::collections::BTreeMap<String, &Todo> = todos
        .iter()
        .filter_map(|todo| todo_stem(todo).map(|stem| (stem, todo)))
        .collect();
    let mut ready: Vec<&str> = by_stem
        .iter()
        .filter(|(_, todo)| todo.status == TodoStatus::Open)
        .filter(|(_, todo)| {
            todo.blocked_by.iter().all(|blocker| {
                by_stem
                    .get(blocker)
                    .is_none_or(|other| other.status == TodoStatus::Done)
            })
        })
        .map(|(stem, _)| stem.as_str())
        .collect();
    // Every ready unit has no open blocker, so topological rank is uniform
    // and the rank-then-id order is id ascending.
    ready.sort_unstable();

    let mut units: Vec<WaveUnit> = Vec::new();
    let mut held: Vec<HeldUnit> = Vec::new();
    for stem in ready {
        if let Some(park) = parked(&facts, stem) {
            held.push(HeldUnit {
                id: stem.to_owned(),
                behind: None,
                reason: "parked",
                blocking_fact_id: Some(park.fact.fact_id.clone()),
            });
            continue;
        }
        if let Some(instant) = at
            && held_lease(&facts, stem, instant, &mut held)
        {
            continue;
        }
        let write_set = derive_write_set(graph, &by_stem[stem].node);
        if let Some(opener) = units.first()
            && opener.write_set.resolution == "unresolved"
        {
            held.push(HeldUnit {
                id: stem.to_owned(),
                behind: Some(opener.id.clone()),
                reason: "runs-alone",
                blocking_fact_id: None,
            });
            continue;
        }
        if write_set.resolution == "unresolved" && !units.is_empty() {
            held.push(HeldUnit {
                id: stem.to_owned(),
                behind: None,
                reason: "runs-alone",
                blocking_fact_id: None,
            });
            continue;
        }
        if let Some(clash) = units
            .iter()
            .find(|unit| !write_sets_disjoint(&unit.write_set, &write_set))
        {
            held.push(HeldUnit {
                id: stem.to_owned(),
                behind: Some(clash.id.clone()),
                reason: "write-sets-overlap",
                blocking_fact_id: None,
            });
            continue;
        }
        let bytes = std::fs::read(root.join(&by_stem[stem].path))
            .or_else(|_| std::fs::read(&by_stem[stem].path))
            .map_err(|error| format!("cannot read todo `{stem}` for its content hash: {error}"))?;
        let mut content_hash = sha256_hex(&bytes);
        content_hash.truncate(12);
        units.push(WaveUnit {
            id: stem.to_owned(),
            content_hash,
            write_set,
            hotspot_permission: units.is_empty(),
        });
    }

    let preimage = preimage(DEFAULT_RULE_ID, &units);
    let digest = digest(&preimage);
    Ok(Wave {
        rule: DEFAULT_RULE_ID,
        units,
        held,
        preimage,
        digest,
        hotspot_prefixes: hotspot_prefixes(graph),
        store_ready,
    })
}

/// Pushes a `lease-held` row when `unit` is leased at `instant`.
fn held_lease(
    facts: &[NamedFact],
    unit: &str,
    instant: &str,
    held_units: &mut Vec<HeldUnit>,
) -> bool {
    if !held(facts, unit, instant) {
        return false;
    }
    let head = crate::coord::read::lease_chain_head(facts, unit);
    held_units.push(HeldUnit {
        id: unit.to_owned(),
        behind: None,
        reason: "lease-held",
        blocking_fact_id: head.map(|named| named.fact.fact_id.clone()),
    });
    true
}

/// The canonical preimage: line-oriented, LF-terminated, no escaping.
/// Units sorted ascending by todo id, `ws=` prefixes sorted within a unit,
/// exactly one trailing LF. The base commit and `SCHEMA_VERSION` are
/// deliberately absent.
pub(crate) fn preimage(rule: &str, units: &[WaveUnit]) -> String {
    use std::fmt::Write as _;
    let mut text = String::from("cairn-plan-v1\n");
    let _ = writeln!(text, "rule={rule}");
    let mut sorted: Vec<&WaveUnit> = units.iter().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for unit in sorted {
        let _ = writeln!(text, "unit={}@{}", unit.id, unit.content_hash);
        let mut prefixes = unit.write_set.includes.clone();
        prefixes.sort();
        for prefix in prefixes {
            let _ = writeln!(text, "ws={prefix}");
        }
    }
    text
}

/// `plan-` plus the first sixteen hex of SHA-256 over the preimage.
pub(crate) fn digest(preimage: &str) -> String {
    let mut hex = sha256_hex(preimage.as_bytes());
    hex.truncate(16);
    format!("plan-{hex}")
}

/// Deterministic reader-side tie-break over concurrent rulings: the total
/// order `(recorded_at, fact_id)` over the fully listed fact set. Returns
/// the winner; every other ruling on the same digest or sharing a unit
/// declines `superseded-by-concurrent-ruling`.
#[allow(dead_code)]
// Reason: the driver layer (todo.driver-in-repo) consumes the tie-break; its
// determinism is pinned by the tests below until that caller lands.
pub(crate) fn concurrent_winner<'a>(rulings: &[&'a NamedFact]) -> Option<&'a NamedFact> {
    rulings
        .iter()
        .min_by(|a, b| {
            (a.fact.recorded_at.as_str(), a.fact.fact_id.as_str())
                .cmp(&(b.fact.recorded_at.as_str(), b.fact.fact_id.as_str()))
        })
        .copied()
}

#[cfg(test)]
mod tests;
