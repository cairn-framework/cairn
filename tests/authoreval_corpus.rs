//! Guards the shipped corpus: shape coverage, fixture boundaries, and
//! reference-answer satisfiability.
//!
//! The corpus is the instrument's stimulus, so a defect in it is
//! indistinguishable from a model failure in the published metrics.

mod authoreval_support;

use authoreval_support::*;
use cairn::authoreval::{Outcome, run_prompt_file};
use camino::Utf8PathBuf;

/// The sentinel every corpus prompt carries, keeping the fixture's unloaded
/// evidence corpus out of reach of the answer.
const NO_EVIDENCE: &str = "Do not add a `research` or `sources` pointer: this project's evidence corpus is \
deliberately unloaded, and citing it would break the baseline.";

/// Directories a corpus answer may author into. Five are loaded authority in
/// the fixture: the artefact kinds its blueprint claims, and source a module
/// claims. `meta/changes/` is the deliberate exception: the parent's authoring
/// family requires a staged `blueprint.delta`, which lives in a change
/// directory the fixture's scanner does not load. It is an allowed authoring
/// surface, not loaded authority, and the baseline reports that prompt's record
/// as ungraded for the same reason.
const ALLOWED_DIRECTORIES: &[&str] = &[
    "meta/contracts/",
    "meta/decisions/",
    "meta/todos/",
    "meta/reviews/",
    "meta/changes/",
    "src/",
];

/// Whether `path` is an authoring surface this corpus is allowed to write.
///
/// Shape is checked before membership, because membership is lexical and
/// lexical matching alone is not a boundary: `meta/contracts/../research/x.md`
/// starts with an allowed directory and resolves outside it. Both separators
/// count, matching `workspace::canonical_relative`, or `src/..\meta/research`
/// would pass with the traversal hidden inside what this saw as one segment.
/// Then the blueprint file is matched exactly, because a prefix test there
/// would accept `cairn.blueprint.bak`, and directories are matched with their
/// separator, because `src` alone would accept a `srcfoo` sibling.
fn allowed_authoring_surface(path: &str) -> bool {
    let plain_relative = !path.contains('\0')
        && path
            .split(['/', '\\'])
            .all(|segment| !matches!(segment, "" | "." | ".."));

    plain_relative
        && (path == "cairn.blueprint"
            || ALLOWED_DIRECTORIES
                .iter()
                .any(|directory| path.starts_with(directory)))
}

fn corpus() -> Vec<(String, serde_json::Value)> {
    let dir = manifest_dir().join("harness/authoreval/prompts");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("read prompt directory") {
        let path = Utf8PathBuf::from_path_buf(entry.expect("prompt entry").path())
            .expect("utf-8 prompt path");
        let name = path.file_name().expect("prompt file name").to_owned();
        if !name.starts_with("corpus.") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("read prompt");
        out.push((
            name,
            serde_json::from_str(&text).expect("prompt parses as JSON"),
        ));
    }
    out.sort_by(|left, right| left.0.cmp(&right.0));
    assert!(!out.is_empty(), "the corpus must not be empty");
    out
}

/// Every workspace path a prompt declares, in declaration order.
fn declared(prompt: &serde_json::Value) -> Vec<&str> {
    prompt["expects"]
        .as_array()
        .expect("expects is an array")
        .iter()
        .map(|value| value.as_str().expect("expects entry is a string"))
        .collect()
}

/// Every workspace path the prompt's reference answer writes, across all turns.
fn replayed(prompt: &serde_json::Value) -> Vec<&str> {
    prompt["replay"]["turns"]
        .as_array()
        .expect("replay turns is an array")
        .iter()
        .filter_map(|turn| turn["files"].as_array())
        .flatten()
        .filter_map(|file| file["path"].as_str())
        .collect()
}

#[test]
fn test_the_corpus_covers_the_authoring_family_within_its_size_bound() {
    let corpus = corpus();
    assert!(
        (5..=10).contains(&corpus.len()),
        "the corpus carries {} prompts, outside the five-to-ten bound",
        corpus.len()
    );

    let ids: Vec<String> = corpus
        .iter()
        .map(|(_, prompt)| prompt["id"].as_str().expect("id is a string").to_owned())
        .collect();
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(
        unique.len(),
        ids.len(),
        "prompt ids must be unique: {ids:?}"
    );

    for (name, prompt) in &corpus {
        let id = prompt["id"].as_str().expect("id is a string");
        assert_eq!(
            format!("{id}.json"),
            *name,
            "a prompt's file name must be its id"
        );
    }

    for required in [
        "corpus.module-claiming-files",
        "corpus.blueprint-delta-rename",
        "corpus.decision-multi-node",
    ] {
        assert!(
            ids.iter().any(|id| id == required),
            "the authoring family requires {required}; corpus carries {ids:?}"
        );
    }
    assert!(
        ids.iter().any(|id| {
            id.starts_with("corpus.todo-")
                || id.starts_with("corpus.review-")
                || id.starts_with("corpus.contract-")
        }),
        "the corpus needs at least one further loaded artefact form; carries {ids:?}"
    );
}

#[test]
fn test_no_prompt_reaches_the_fixtures_unloaded_evidence_corpus() {
    for outside in [
        "cairn.blueprint.bak",
        "meta/research/notes.md",
        "meta/contracts/../research/notes.md",
        "meta/contracts/../../meta/sources/x.md",
        r"src/..\meta/research/x.md",
        "meta/decisions/./x.md",
        "/meta/decisions/x.md",
        "meta/decisions/",
        "",
    ] {
        assert!(
            !allowed_authoring_surface(outside),
            "{outside:?} must not pass the authoring-surface boundary"
        );
    }

    for (_, prompt) in corpus() {
        let id = prompt["id"].as_str().expect("id is a string");
        let instruction = prompt["instruction"]
            .as_str()
            .expect("instruction is a string");
        assert!(
            instruction.contains(NO_EVIDENCE),
            "{id} does not forbid a research or sources pointer"
        );

        for path in declared(&prompt) {
            assert!(
                allowed_authoring_surface(path),
                "{id} expects {path}, outside this corpus's allowed authoring surfaces"
            );
        }

        // The reference answer is scored by the same fixture, and the runner
        // tolerates a response that writes more than it was asked for, so a
        // replay authoring an extra file would still score clean and the
        // satisfiability test would vouch for a stimulus wider than the prompt.
        let mut authored = replayed(&prompt);
        let mut expected = declared(&prompt);
        authored.sort_unstable();
        expected.sort_unstable();
        assert_eq!(
            authored, expected,
            "{id}'s reference answer must author exactly the paths the prompt declares"
        );
    }
}

#[test]
fn test_every_corpus_prompt_is_satisfiable() {
    for (name, prompt) in corpus() {
        let id = prompt["id"].as_str().expect("id is a string").to_owned();
        assert!(
            prompt.get("replay").is_some(),
            "{id} carries no reference answer, so its satisfiability is unproven"
        );
        let path = manifest_dir()
            .join("harness/authoreval/prompts")
            .join(&name);
        let record = run_prompt_file(&config(0), &path).expect("the corpus must produce a record");
        assert_eq!(
            record.outcome,
            Outcome::CleanFirstShot,
            "{id} is not satisfiable: {record:?}"
        );
    }
}
