//! Data-driven artefact kind table and per-kind record constructors.
//!
//! One table entry owns the blueprint pointer name and the function that turns
//! a parsed frontmatter file into a typed record. Adding a kind is a row plus
//! a `load_one_*` function; the shared walk over pointers/files lives once.

// Reason: this split keeps the original parent-owned import surface to avoid semantic drift.
#![allow(clippy::wildcard_imports)]
use super::super::frontmatter::Frontmatter;
use super::io::{list, markdown_paths, optional, parse_file, path_string, pointers, required};
use super::parse::{
    parse_affects_path, parse_decision_status, parse_defers_reference, parse_lens_prompt_hash,
    parse_ratification_tier, parse_ratified_by, parse_receipt_reviewer, parse_research_method,
    parse_review_type, parse_source_verification, parse_subject_hash, parse_todo_status,
};
use super::*;
use crate::blueprint::Ast;
use std::path::Path;

/// Descriptor for one frontmatter-backed artefact kind loaded from blueprint pointers.
pub(super) struct ArtefactKind {
    /// Blueprint field name declaring directories of this kind (`todos`, `decisions`, …).
    pub pointer: &'static str,
    /// Construct one typed record from a parsed file, pushing findings on failure.
    pub load_one: fn(&Path, &Frontmatter, &mut ArtefactSet),
}

/// Kinds loaded through the shared pointer/directory walk.
///
/// Contract and change loaders are genuine special cases and are not listed here.
pub(super) const ARTEFACT_KINDS: &[ArtefactKind] = &[
    ArtefactKind {
        pointer: "todos",
        load_one: load_one_todo,
    },
    ArtefactKind {
        pointer: "decisions",
        load_one: load_one_decision,
    },
    ArtefactKind {
        pointer: "reviews",
        load_one: load_one_review,
    },
    ArtefactKind {
        pointer: "research",
        load_one: load_one_research,
    },
    ArtefactKind {
        pointer: "sources",
        load_one: load_one_source,
    },
];

/// The decisions kind entry (used by architecture hooks without a full load).
pub(super) fn decisions_kind() -> &'static ArtefactKind {
    // Index must match ARTEFACT_KINDS order: todos, decisions, ...
    debug_assert_eq!(ARTEFACT_KINDS[1].pointer, "decisions");
    &ARTEFACT_KINDS[1]
}

/// Walk every kind in [`ARTEFACT_KINDS`] and load matching markdown files.
pub(super) fn load_kinds(root: &Path, ast: &Ast, set: &mut ArtefactSet) {
    for kind in ARTEFACT_KINDS {
        load_kind(root, ast, kind, set);
    }
}

/// Load a single kind (used by hooks that only need decisions).
pub(super) fn load_kind(root: &Path, ast: &Ast, kind: &ArtefactKind, set: &mut ArtefactSet) {
    for pointer in pointers(ast, kind.pointer) {
        for path in markdown_paths(root, &pointer, set) {
            if let Some(parsed) = parse_file(&path, &pointer, set) {
                (kind.load_one)(&path, &parsed, set);
            }
        }
    }
}

fn load_one_todo(path: &Path, parsed: &Frontmatter, set: &mut ArtefactSet) {
    let Some(node) = required(&parsed.values, "node", path_string(path), set) else {
        return;
    };
    let Some(status) = required(&parsed.values, "status", path_string(path), set)
        .and_then(|value| parse_todo_status(&value, path, set))
    else {
        return;
    };
    let Some(created) = required(&parsed.values, "created", path_string(path), set) else {
        return;
    };
    // A scalar `defers: CODE path` reaches only `values`, never `lists`, and
    // would otherwise be a silent no-op; the field is a list or it is
    // malformed (an inline `[..]` form populates both maps and is fine).
    if optional(&parsed.values, "defers").is_some() && !parsed.lists.contains_key("defers") {
        parse_defers_reference("", path, set);
    }
    let defers = list(parsed, "defers")
        .iter()
        .filter_map(|value| parse_defers_reference(value, path, set))
        .collect();
    set.todos.push(Todo {
        path: path_string(path),
        node,
        status,
        created,
        satisfies: optional(&parsed.values, "satisfies"),
        blocked_by: list(parsed, "blocked_by"),
        parent: optional(&parsed.values, "parent"),
        related: list(parsed, "related"),
        defers,
        body: parsed.body.clone(),
    });
}

fn load_one_decision(path: &Path, parsed: &Frontmatter, set: &mut ArtefactSet) {
    let Some(id) = required(&parsed.values, "id", path_string(path), set) else {
        return;
    };
    let Some(status) = required(&parsed.values, "status", path_string(path), set)
        .and_then(|value| parse_decision_status(&value, path, set))
    else {
        return;
    };
    let Some(date) = required(&parsed.values, "date", path_string(path), set) else {
        return;
    };
    // A key parsed into the LIST map is present but scalar-shaped wrong; it
    // must raise its invalid-value finding rather than silently defaulting
    // (`ratification:` with block items would otherwise gate as `binding`).
    if parsed.lists.contains_key("ratification") {
        parse_ratification_tier("", path, set);
        return;
    }
    if parsed.lists.contains_key("ratified_by") {
        parse_ratified_by("", path, set);
        return;
    }
    let ratification = parsed.values.get("ratification").cloned().map_or(
        Some(crate::artefacts::registry::RatificationTier::Binding),
        |value| parse_ratification_tier(&value, path, set),
    );
    let Some(ratification) = ratification else {
        return;
    };
    let ratified_by_machine = parsed
        .values
        .get("ratified_by")
        .cloned()
        .map_or(Some(false), |value| parse_ratified_by(&value, path, set));
    let Some(ratified_by_machine) = ratified_by_machine else {
        return;
    };
    if parsed.values.contains_key("affects") && !parsed.lists.contains_key("affects") {
        parse_affects_path("", path, set);
    }
    let affects = list(parsed, "affects")
        .iter()
        .filter_map(|value| parse_affects_path(value, path, set))
        .collect();
    set.decisions.push(Decision {
        id,
        path: path_string(path),
        nodes: list(parsed, "nodes"),
        status,
        ratification,
        affects,
        ratified_by_machine,
        receipts: list(parsed, "receipts"),
        date,
        revisited: optional(&parsed.values, "revisited"),
        revisit_triggers: list(parsed, "revisit_triggers"),
        informed_by: list(parsed, "informed_by"),
        supersedes: list(parsed, "supersedes"),
        refines: list(parsed, "refines"),
        related: list(parsed, "related"),
        orphaned: optional(&parsed.values, "orphaned").is_some_and(|value| value == "true"),
        orphan_reason: optional(&parsed.values, "orphan_reason"),
        gap: optional(&parsed.values, "gap").is_some_and(|value| value == "true"),
        claims: parse_claims(&parsed.values, &parsed.lists, path),
        body: parsed.body.clone(),
    });
}

pub(super) fn parse_claims(
    values: &std::collections::BTreeMap<String, String>,
    lists: &std::collections::BTreeMap<String, Vec<String>>,
    _path: &Path,
) -> Option<crate::artefacts::Claims> {
    let folder = values.get("claims_folder")?;
    let mode = match values.get("claims_mode").map(String::as_str) {
        Some("exhaustive") => crate::artefacts::ClaimsMode::Exhaustive,
        Some("illustrative") => crate::artefacts::ClaimsMode::Illustrative,
        _ => return None,
    };
    let items = lists.get("claims_items").cloned().unwrap_or_default();
    Some(crate::artefacts::Claims {
        folder: folder.clone(),
        mode,
        items,
    })
}

fn load_one_review(path: &Path, parsed: &Frontmatter, set: &mut ArtefactSet) {
    let Some(node) = required(&parsed.values, "node", path_string(path), set) else {
        return;
    };
    let Some(date) = required(&parsed.values, "date", path_string(path), set) else {
        return;
    };
    let Some(reviewer) = required(&parsed.values, "reviewer", path_string(path), set) else {
        return;
    };
    let review_type = optional(&parsed.values, "review_type")
        .map_or(Some(ReviewType::Human), |value| {
            parse_review_type(&value, path, set)
        });
    let Some(review_type) = review_type else {
        return;
    };
    if parsed.lists.contains_key("subject_hash") {
        parse_subject_hash("", path, set);
        return;
    }
    if parsed.lists.contains_key("lens_prompt_hash") {
        parse_lens_prompt_hash(Some(String::new()), true, path, set);
        return;
    }
    let subject_hash_raw = parsed.values.get("subject_hash").cloned();
    let receipt_grade = subject_hash_raw.is_some();
    let subject_hash = subject_hash_raw
        .as_deref()
        .and_then(|value| parse_subject_hash(value, path, set));
    let reviewer_valid = parse_receipt_reviewer(&reviewer, receipt_grade, path, set);
    let lens_prompt_hash = parse_lens_prompt_hash(
        parsed.values.get("lens_prompt_hash").cloned(),
        receipt_grade,
        path,
        set,
    );
    if (receipt_grade && (subject_hash.is_none() || lens_prompt_hash.is_none())) || !reviewer_valid
    {
        return;
    }
    set.reviews.push(Review {
        path: path_string(path),
        node,
        review_type,
        date,
        reviewer,
        subject_hash,
        lens_prompt_hash,
        related_change: optional(&parsed.values, "related_change"),
        body: parsed.body.clone(),
    });
}

fn load_one_research(path: &Path, parsed: &Frontmatter, set: &mut ArtefactSet) {
    let Some(id) = required(&parsed.values, "id", path_string(path), set) else {
        return;
    };
    let Some(date) = required(&parsed.values, "date", path_string(path), set) else {
        return;
    };
    let method = optional(&parsed.values, "method")
        .and_then(|value| parse_research_method(&value, path, set))
        .unwrap_or_default();
    set.research.push(Research {
        id,
        path: path_string(path),
        nodes: list(parsed, "nodes"),
        date,
        sources: list(parsed, "sources"),
        method,
        tags: list(parsed, "tags"),
        body: parsed.body.clone(),
    });
}

fn load_one_source(path: &Path, parsed: &Frontmatter, set: &mut ArtefactSet) {
    let Some(id) = required(&parsed.values, "id", path_string(path), set) else {
        return;
    };
    let Some(file) = required(&parsed.values, "file", path_string(path), set) else {
        return;
    };
    let Some(verification) = required(&parsed.values, "verification", path_string(path), set)
        .and_then(|value| parse_source_verification(&value, path, set))
    else {
        return;
    };
    let Some(source_type) = required(&parsed.values, "type", path_string(path), set) else {
        return;
    };
    let Some(date) = required(&parsed.values, "date", path_string(path), set) else {
        return;
    };
    set.sources.push(Source {
        id,
        path: path_string(path),
        file,
        sha256: optional(&parsed.values, "sha256").filter(|value| value != "null"),
        verification,
        source_type,
        date,
        tags: list(parsed, "tags"),
        description: optional(&parsed.values, "description").unwrap_or_default(),
        body: parsed.body.clone(),
    });
}

#[cfg(test)]
mod tests;
