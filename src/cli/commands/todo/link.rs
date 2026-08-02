//! `cairn todo link`/`unlink`: relationship-edge edits on todo frontmatter.
//!
//! Implements the author surface for `dec.todo-relationship-model` ruling 1
//! via surgical frontmatter edits through the sanctioned todo write path:
//! only the field lines named by the invocation change, the body never
//! does. `link` unions `--blocked-by`/`--related` entries and replaces
//! `--parent`; `unlink` removes the named entries and errors on ones that
//! are not present. Reference shapes are checked only when adding: unlink
//! must stay able to remove a malformed entry the scanner warns about.

// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::*;

#[cfg(test)]
mod tests;

/// Whether the invocation adds or removes entries.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum Mode {
    Link,
    Unlink,
}

/// Parsed relationship flags.
pub(super) struct Args {
    blocked_by: Vec<String>,
    parents: Vec<String>,
    related: Vec<String>,
    json: bool,
}

impl Args {
    /// Parses the three relationship flags with arity checking: a flag
    /// occurrence whose value is missing (end of argv or another flag) is
    /// an error, never a silent no-op.
    pub(super) fn from_cli(parsed: &ParsedArgs) -> Result<Self, CliResult> {
        Ok(Self {
            blocked_by: strict_flag_values(&parsed.command_args, "--blocked-by")?,
            parents: strict_flag_values(&parsed.command_args, "--parent")?,
            related: strict_flag_values(&parsed.command_args, "--related")?,
            json: parsed.json,
        })
    }
}

/// Collects every value of `flag`, erroring on a valueless occurrence.
fn strict_flag_values(args: &[String], flag: &str) -> Result<Vec<String>, CliResult> {
    let mut values = Vec::new();
    let mut iter = args.iter().peekable();
    while let Some(token) = iter.next() {
        if token != flag {
            continue;
        }
        match iter.peek() {
            Some(next) if !next.starts_with("--") => {
                values.push((*next).clone());
                iter.next();
            }
            _ => {
                return Err(err(
                    1,
                    &copy::lookup("todo.flag-needs-value").replace("{flag}", flag),
                ));
            }
        }
    }
    Ok(values)
}

/// Post-transition values for the three fields; `write_*` marks which
/// fields the invocation actually named, and only those lines are written.
struct NextFields {
    blocked_by: Vec<String>,
    parent: Option<String>,
    related: Vec<String>,
    write_blocked_by: bool,
    write_parent: bool,
    write_related: bool,
}

pub(super) fn run(root: &Path, slug: &str, args: &Args, mode: Mode) -> CliResult {
    match transition(root, slug, args, mode) {
        Ok(result) | Err(result) => result,
    }
}

/// Full verb pipeline; every failure short-circuits with its CLI error.
fn transition(root: &Path, slug: &str, args: &Args, mode: Mode) -> Result<CliResult, CliResult> {
    validate(slug, args, mode)?;
    let path = root.join("meta/todos").join(format!("todo.{slug}.md"));
    if !path.exists() {
        return Err(err(
            1,
            &copy::lookup("todo.not-found").replace("{slug}", slug),
        ));
    }
    let source =
        fs::read_to_string(&path).map_err(|e| io_error("todo.io-read-error", &path, &e))?;
    let next = next_fields(&source, args, mode, slug)?;
    let updated = write_fields(source, &next, slug)?;
    persist::atomic_write(&path, &updated)
        .map_err(|e| io_error("todo.io-write-error", &path, &e))?;
    Ok(render(slug, &path, &next, args.json, mode))
}

fn validate(slug: &str, args: &Args, mode: Mode) -> Result<(), CliResult> {
    if !is_kebab_slug(slug) {
        return Err(err(1, copy::lookup("todo.invalid-slug")));
    }
    if args.blocked_by.is_empty() && args.parents.is_empty() && args.related.is_empty() {
        let usage = match mode {
            Mode::Link => "todo.link-usage",
            Mode::Unlink => "todo.unlink-usage",
        };
        return Err(err(1, copy::lookup(usage)));
    }
    if args.parents.len() > 1 {
        return Err(err(1, copy::lookup("todo.parent-single")));
    }
    // Shape checks apply only when adding; unlink must stay able to remove
    // an entry the scanner already warns about.
    if mode == Mode::Link {
        for value in args.blocked_by.iter().chain(args.parents.iter()) {
            if !valid_relation_ref(value, false) {
                return Err(err(
                    1,
                    &copy::lookup("todo.invalid-ref").replace("{value}", value),
                ));
            }
        }
        for value in &args.related {
            if !valid_relation_ref(value, true) {
                return Err(err(
                    1,
                    &copy::lookup("todo.invalid-ref").replace("{value}", value),
                ));
            }
        }
    }
    Ok(())
}

/// Computes the post-transition field values from the parsed frontmatter.
fn next_fields(source: &str, args: &Args, mode: Mode, slug: &str) -> Result<NextFields, CliResult> {
    let current = parse_frontmatter(source);
    let current_list = |key: &str| current.lists.get(key).cloned().unwrap_or_default();
    // The raw scalar is preserved even when malformed (an inline `[..]`
    // form): unlink must be able to remove exactly what the scanner warns
    // about, and rendering must not claim the field is absent.
    let current_parent = current.values.get("parent").cloned();
    let (write_blocked_by, write_parent, write_related) = (
        !args.blocked_by.is_empty(),
        !args.parents.is_empty(),
        !args.related.is_empty(),
    );
    let (blocked_by, parent, related) = match mode {
        Mode::Link => (
            union(current_list("blocked_by"), &args.blocked_by),
            args.parents.first().cloned().or(current_parent),
            union(current_list("related"), &args.related),
        ),
        Mode::Unlink => (
            difference(current_list("blocked_by"), &args.blocked_by)
                .map_err(|missing| unlink_missing(slug, "blocked_by", &missing))?,
            match args.parents.first() {
                Some(given) if current_parent.as_deref() == Some(given.as_str()) => None,
                Some(given) => return Err(unlink_missing(slug, "parent", given)),
                None => current_parent,
            },
            difference(current_list("related"), &args.related)
                .map_err(|missing| unlink_missing(slug, "related", &missing))?,
        ),
    };
    Ok(NextFields {
        blocked_by,
        parent,
        related,
        write_blocked_by,
        write_parent,
        write_related,
    })
}

/// Applies only the invocation-named field values, dropping emptied lines.
fn write_fields(source: String, next: &NextFields, slug: &str) -> Result<String, CliResult> {
    let malformed = || {
        err(
            1,
            &copy::lookup("todo.malformed-frontmatter").replace("{slug}", slug),
        )
    };
    let mut updated = source;
    if next.write_blocked_by {
        updated =
            write_list_field(&updated, "blocked_by", &next.blocked_by).map_err(|()| malformed())?;
    }
    if next.write_related {
        updated = write_list_field(&updated, "related", &next.related).map_err(|()| malformed())?;
    }
    if next.write_parent {
        updated = write_scalar_field(&updated, "parent", next.parent.as_deref())
            .map_err(|()| malformed())?;
    }
    Ok(updated)
}

fn render(slug: &str, path: &Path, next: &NextFields, json: bool, mode: Mode) -> CliResult {
    if json {
        return ok(serde_json::json!({
            "slug": slug,
            "blocked_by": next.blocked_by,
            "parent": next.parent,
            "related": next.related,
            "path": path.to_string_lossy(),
        })
        .to_string());
    }
    let key = match mode {
        Mode::Link => "todo.link-success",
        Mode::Unlink => "todo.unlink-success",
    };
    ok(copy::lookup(key).replace("{slug}", slug))
}

fn io_error(key: &str, path: &Path, error: &std::io::Error) -> CliResult {
    err(
        1,
        &copy::lookup(key)
            .replace("{path}", &path.display().to_string())
            .replace("{error}", &error.to_string()),
    )
}

/// `todo.<slug>` stems for `--blocked-by`/`--parent`; `--related` also
/// accepts `dec.`/`res.`/`src.` ids, whose slugs may nest with dots
/// (`res.gas-city.analysis`).
fn valid_relation_ref(value: &str, related: bool) -> bool {
    if let Some(rest) = value.strip_prefix("todo.") {
        return is_kebab_slug(rest);
    }
    related
        && ["dec.", "res.", "src."].iter().any(|prefix| {
            value
                .strip_prefix(prefix)
                .is_some_and(|rest| !rest.is_empty() && rest.split('.').all(is_kebab_slug))
        })
}

/// Appends each addition absent from `current`, preserving authored order
/// and de-duplicating repeats within one invocation.
fn union(mut current: Vec<String>, additions: &[String]) -> Vec<String> {
    for value in additions {
        if !current.contains(value) {
            current.push(value.clone());
        }
    }
    current
}

/// Removes each named entry, or reports the first one that is not present.
fn difference(mut current: Vec<String>, removals: &[String]) -> Result<Vec<String>, String> {
    for value in removals {
        match current.iter().position(|entry| entry == value) {
            Some(index) => {
                current.remove(index);
            }
            None => return Err(value.clone()),
        }
    }
    Ok(current)
}

fn unlink_missing(slug: &str, field: &str, value: &str) -> CliResult {
    err(
        1,
        &copy::lookup("todo.unlink-missing")
            .replace("{slug}", slug)
            .replace("{field}", field)
            .replace("{value}", value),
    )
}

/// Writes a list field as an inline `[a, b]` frontmatter line, removing the
/// line entirely when the list empties. Absent key plus empty list is a
/// no-op.
fn write_list_field(source: &str, key: &str, values: &[String]) -> Result<String, ()> {
    if values.is_empty() {
        return match remove_field(source, key) {
            Ok(next) => Ok(next),
            Err(SetFieldError::KeyNotFound) => Ok(source.to_owned()),
            Err(SetFieldError::NoFrontmatter) => Err(()),
        };
    }
    upsert_field(source, key, &format!("[{}]", values.join(", "))).map_err(|_| ())
}

/// Writes a scalar field, removing the line when the value clears.
fn write_scalar_field(source: &str, key: &str, value: Option<&str>) -> Result<String, ()> {
    match value {
        Some(value) => upsert_field(source, key, value).map_err(|_| ()),
        None => match remove_field(source, key) {
            Ok(next) => Ok(next),
            Err(SetFieldError::KeyNotFound) => Ok(source.to_owned()),
            Err(SetFieldError::NoFrontmatter) => Err(()),
        },
    }
}
