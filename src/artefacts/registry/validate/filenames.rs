//! Artefact filename convention checks.

use std::path::Path;

use crate::map::graph::Finding;

use super::super::io::warning;
use super::super::types::ArtefactSet;

const CODE: &str = "CAIRN_ARTEFACT_FILENAME_DRIFT";

/// Checks every artefact filename against `dec.artefact-layout-authority`:
/// a decision, research, or source filename is its id with the typed prefix
/// stripped, while a todo keeps `todo.<slug>.md` because `cairn todo new` and
/// `cairn todo set` resolve slugs through that exact path.
pub(super) fn validate_filenames(set: &mut ArtefactSet) {
    for item in &set.decisions {
        check_typed(&mut set.findings, &item.path, &item.id, "dec.");
    }
    for item in &set.research {
        check_typed(&mut set.findings, &item.path, &item.id, "res.");
    }
    for item in &set.sources {
        check_typed(&mut set.findings, &item.path, &item.id, "src.");
    }
    for todo in &set.todos {
        if !has_todo_slug(&todo.path) {
            set.findings.push(warning(
                CODE,
                format!("todo `{}` should be named `todo.<slug>.md`", todo.path),
                Some(todo.node.clone()),
                Some(todo.path.clone()),
            ));
        }
    }
}

/// Compares one id-bearing artefact's filename stem against its id.
///
/// A rename is only suggested once `id` is known to be `prefix` plus a
/// non-empty slug. Nothing else validates that an id carries the prefix its
/// kind requires, so deriving a filename from a malformed one would hand out
/// remediation that cannot be followed (`id: dec.` yields `.md`, which is not
/// a Markdown file the loader would ever read) or that is actively wrong (a
/// decision declaring `id: res.foo` would be told to become `res.foo.md`,
/// putting a typed prefix back into a filename).
fn check_typed(findings: &mut Vec<Finding>, path: &str, id: &str, prefix: &str) {
    let message = match id.strip_prefix(prefix) {
        Some(slug) if !slug.is_empty() => {
            if stem(path) == Some(slug) {
                return;
            }
            format!("artefact `{path}` should be named `{slug}.md` to match id `{id}`")
        }
        _ => format!("artefact `{path}` declares id `{id}`, which is not `{prefix}` plus a slug"),
    };
    findings.push(warning(CODE, message, None, Some(path.to_owned())));
}

/// True when the filename is `todo.` followed by a non-empty slug.
fn has_todo_slug(path: &str) -> bool {
    stem(path).is_some_and(|stem| {
        stem.strip_prefix("todo.")
            .is_some_and(|slug| !slug.is_empty())
    })
}

/// Filename without its final extension: `gas-city.analysis.md` yields
/// `gas-city.analysis`, so slug namespacing survives the comparison.
fn stem(path: &str) -> Option<&str> {
    Path::new(path).file_stem()?.to_str()
}
