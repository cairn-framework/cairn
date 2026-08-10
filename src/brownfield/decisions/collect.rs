//! Evidence collection for the decision-evidence index.
//!
//! Each collector reads one class of the closed evidence set
//! (`dec.brownfield-extraction-mechanism` clause 1) and appends what it observed.
//! Nothing here binds evidence to a node; `super::index` does that.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use super::{
    DOCUMENT_ROOTS, Evidence, EvidenceKind, INVARIANT_MARKERS, MAX_DOCUMENT_DEPTH, README_FILE,
    SECTION_HEADINGS, discovery, walk,
};

pub(super) fn collect_documents(root: &Path, out: &mut Vec<Evidence>) {
    for dir in DOCUMENT_ROOTS {
        collect_documents_in(root, &root.join(dir), 0, out);
    }
}

fn collect_documents_in(root: &Path, dir: &Path, depth: usize, out: &mut Vec<Evidence>) {
    if depth > MAX_DOCUMENT_DEPTH {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        if is_symlink(&path) {
            continue;
        }
        if path.is_dir() {
            collect_documents_in(root, &path, depth + 1, out);
        } else if path.is_file() {
            out.push(Evidence {
                kind: EvidenceKind::Document,
                path: relative(root, &path),
                line: None,
                detail: first_heading(&path),
            });
        }
    }
}

/// Scans the root README plus a README beside any source file the survey
/// observed.
///
/// Survey-scoped for the same reason as the invariant scan: the closed evidence
/// set names README sections, not candidate directories, so a README beside two
/// source files still carries evidence.
pub(super) fn collect_readme_sections(root: &Path, survey: &walk::Survey, out: &mut Vec<Evidence>) {
    let mut dirs: BTreeSet<PathBuf> = BTreeSet::new();
    dirs.insert(root.to_path_buf());
    for file in survey.source_files() {
        if let Some(parent) = file.parent() {
            dirs.insert(parent.to_path_buf());
        }
    }
    for dir in dirs {
        let path = dir.join(README_FILE);
        if is_symlink(&path) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let rel = relative(root, &path);
        for (offset, heading) in evidence_headings(&text) {
            out.push(Evidence {
                kind: EvidenceKind::ReadmeSection,
                path: rel.clone(),
                line: Some(offset + 1),
                detail: heading,
            });
        }
    }
}

/// Evidence headings in `text`, as `(zero-based line, heading text)`.
///
/// Handles ATX (`## Decision`) and setext (`Decision` over `===`), and skips
/// fenced code so an example heading is not evidence. Follows the `CommonMark`
/// rules the closed evidence set depends on: at most three leading spaces (four
/// is indented code), and a fence closes only on a delimiter run alone on its
/// line.
pub(super) fn evidence_headings(text: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut fence: Option<&str> = None;
    for (offset, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(open) = fence {
            if closes_fence(trimmed, open) {
                fence = None;
            }
            continue;
        }
        if is_indented_code(line) {
            continue;
        }
        if let Some(open) = fence_marker(trimmed) {
            fence = Some(open);
            continue;
        }
        if let Some(heading) = heading_text(line) {
            if is_evidence_heading(heading) {
                found.push((offset, heading.to_owned()));
            }
            continue;
        }
        // Setext: the underline decides, so the heading is the line above it.
        if is_setext_underline(trimmed)
            && offset > 0
            && let Some(above) = lines.get(offset - 1)
            && !is_indented_code(above)
            && is_evidence_heading(above.trim())
        {
            found.push((offset - 1, above.trim().to_owned()));
        }
    }
    found
}

/// True when four or more leading spaces make `line` indented code.
fn is_indented_code(line: &str) -> bool {
    line.bytes().take_while(|b| *b == b' ').count() >= 4
}

/// True when `trimmed` is a closing fence: the delimiter run and nothing else.
///
/// An info-string line such as ```` ```rust ```` opens a fence but must not
/// close one, or the block's contents would be parsed as Markdown.
fn closes_fence(trimmed: &str, open: &str) -> bool {
    let delimiter = open.as_bytes()[0];
    let run = trimmed.bytes().take_while(|b| *b == delimiter).count();
    run >= open.len() && trimmed[run..].trim().is_empty()
}

/// The fence delimiter opening at `trimmed`, if any.
fn fence_marker(trimmed: &str) -> Option<&'static str> {
    if trimmed.starts_with("```") {
        Some("```")
    } else if trimmed.starts_with("~~~") {
        Some("~~~")
    } else {
        None
    }
}

/// True for a setext underline: a run of `=` or `-` and nothing else.
fn is_setext_underline(trimmed: &str) -> bool {
    (trimmed.starts_with('=') && trimmed.bytes().all(|b| b == b'='))
        || (trimmed.starts_with('-') && trimmed.bytes().all(|b| b == b'-'))
}

/// Scans every source file the bounded survey observed, not just the files
/// behind a discovery candidate: a directory below the candidate threshold
/// still carries invariant comments, and the closed evidence set names the
/// comment marker, not the candidate.
///
/// Symlinked files are skipped, matching `collect_documents` and the
/// reconciler's `file_type().is_file()`: a link out of the project root would
/// otherwise inject evidence from a path the blueprint cannot own.
pub(super) fn collect_invariant_comments(
    root: &Path,
    survey: &walk::Survey,
    out: &mut Vec<Evidence>,
) {
    let mut files: BTreeSet<String> = BTreeSet::new();
    for file in survey.source_files() {
        if is_symlink(file) {
            continue;
        }
        files.insert(relative(root, file));
    }
    for file in files {
        let Ok(text) = std::fs::read_to_string(root.join(&file)) else {
            continue;
        };
        // Rust has no single-quoted strings, and `'a` lifetimes would make an
        // apostrophe count meaningless there; every other surveyed language
        // quotes with apostrophes.
        let apostrophe_quotes = !Path::new(&file)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"));
        for (offset, line) in text.lines().enumerate() {
            if let Some(detail) = invariant_detail(line, apostrophe_quotes) {
                out.push(Evidence {
                    kind: EvidenceKind::InvariantComment,
                    path: file.clone(),
                    line: Some(offset + 1),
                    detail: detail.to_owned(),
                });
            }
        }
    }
}

/// Records each discovery candidate directory as bounded code evidence. The
/// detail carries the path-derived candidate id, which is evidence only and is
/// never equated with a blueprint node id.
///
/// The candidate path is separator-normalised here rather than trusted: it comes
/// from `Path::to_string_lossy`, so on Windows it would otherwise carry
/// backslashes and match no declared path.
pub(super) fn collect_code_targets(extraction: &discovery::Extraction, out: &mut Vec<Evidence>) {
    for candidate in &extraction.candidates {
        out.push(Evidence {
            kind: EvidenceKind::CodeTarget,
            path: forward_slashed(&candidate.path),
            line: None,
            detail: candidate.id.clone(),
        });
    }
}

/// True when `path` is itself a symlink.
///
/// Both collectors skip symlinks: a link out of the project root would inject
/// evidence from a path no declared blueprint path can own, and the target can
/// change between runs.
fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|meta| meta.file_type().is_symlink())
}

fn relative(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    forward_slashed(&rel.to_string_lossy())
}

/// Normalises separators so every emitted path is comparable with the
/// blueprint's declared paths, which always use forward slashes.
fn forward_slashed(path: &str) -> String {
    if path.contains('\\') {
        path.replace('\\', "/")
    } else {
        path.to_owned()
    }
}

/// The first ATX heading in `path`, or an empty string when there is none.
fn first_heading(path: &Path) -> String {
    let Ok(text) = std::fs::read_to_string(path) else {
        return String::new();
    };
    text.lines()
        .find_map(heading_text)
        .unwrap_or_default()
        .to_owned()
}

/// The text of an ATX markdown heading, or `None` for any other line.
///
/// `CommonMark` ATX rules: one to six `#`, then whitespace before the text. A
/// bare `#Decision` or a seventh `#` is not a heading, so it is not evidence.
pub(super) fn heading_text(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let hashes = trimmed.bytes().take_while(|b| *b == b'#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let rest = trimmed.get(hashes..)?;
    if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
        return None;
    }
    // A closing run counts only when whitespace separates it, so `Decision###`
    // keeps its hashes and does not match the closed set.
    let text = rest.trim();
    let closing = text.len() - text.trim_end_matches('#').len();
    if closing > 0
        && let Some(before) = text[..text.len() - closing].chars().next_back()
        && before.is_whitespace()
    {
        return Some(text[..text.len() - closing].trim());
    }
    Some(text)
}

pub(super) fn is_evidence_heading(heading: &str) -> bool {
    let normalised = heading.trim_end_matches(':').trim();
    SECTION_HEADINGS
        .iter()
        .any(|candidate| normalised.eq_ignore_ascii_case(candidate))
}

/// The text after an invariant marker, or `None` when the line carries none.
///
/// The marker counts wherever a real comment carries it, including after code on
/// the same line. Three exclusions keep prose *about* the marker from becoming
/// evidence, which would otherwise make this module and its own tests index
/// themselves:
///
/// - a doc comment (`///`, `//!`), which documents code rather than asserting
///   an invariant;
/// - a marker that does not open a comment, meaning it follows something other
///   than whitespace or a statement terminator, as a URL's scheme colon does;
/// - a marker inside a string literal, detected by an odd number of preceding
///   quotes. `apostrophe_quotes` is false for Rust, which has no single-quoted
///   strings and whose `'a` lifetimes would make that count meaningless.
///
/// This is lexical, not a parser. A marker inside a multi-line string whose own
/// line looks like a comment is still reported; the index is advisory evidence
/// an agent reviews, so a rare extra path costs less than a missed invariant.
pub(super) fn invariant_detail(line: &str, apostrophe_quotes: bool) -> Option<&str> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("///") || trimmed.starts_with("//!") {
        return None;
    }
    INVARIANT_MARKERS.iter().find_map(|marker| {
        let at = line.find(marker)?;
        let before = &line[..at];
        let opens_comment = before
            .chars()
            .next_back()
            .is_none_or(|c| c.is_whitespace() || matches!(c, ';' | ')' | '}' | ']' | ','));
        if !opens_comment {
            return None;
        }
        if before.matches('"').count() % 2 == 1 || before.matches('`').count() % 2 == 1 {
            return None;
        }
        if apostrophe_quotes && before.matches('\'').count() % 2 == 1 {
            return None;
        }
        Some(line[at + marker.len()..].trim())
    })
}
