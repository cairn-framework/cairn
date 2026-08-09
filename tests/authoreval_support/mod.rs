//! Shared support for the authorability eval integration tests.
//!
//! Scenarios are driven through the real scorer against a scratch copy of the
//! bootstrap fixture, using the offline replay backend, so the whole path runs
//! with no network, no API key, and no installed harness.

// Reason: each integration binary compiles this whole module but uses only the
// helpers its own scenarios need, so unused-item warnings here are an artefact
// of sharing, not dead code.
#![allow(dead_code)]

use std::collections::BTreeMap;

use cairn::authoreval::{BackendSpec, Record, RunConfig, run_prompt_file};
use camino::{Utf8Path, Utf8PathBuf};
use serde_json::json;
use sha2::{Digest as _, Sha256};

pub const MODEL: &str = "offline-replay/v1";

/// The path every scenario prompt asks for.
pub const TARGET: &str = "meta/decisions/cli-json-default.md";

pub const VALID_DECISION: &str = "---\n\
id: dec.cli-json-default\n\
nodes: [cairn.kernel.cli]\n\
status: accepted\n\
date: 2026-08-09\n\
---\n\
\n\
# CLI output is human-readable by default\n\
\n\
## Decision\n\
\n\
Every command prints text by default and JSON only when `--json` is passed.\n";

/// Missing the whole frontmatter block, so the artefact reconciler reports
/// `CAIRN_ARTEFACT_MISSING_FIELD` at Error.
pub const BROKEN_DECISION: &str = "# CLI output is human-readable by default\n\
\n\
Every command prints text by default.\n";

/// Well formed frontmatter naming a node that does not exist, so the reconciler
/// reports `CAIRN_DECISION_ORPHANED` at Error. A second, distinct code is what
/// lets a test tell "the same finding survived a repair" apart from "some
/// finding survived a repair".
pub const ORPHANED_DECISION: &str = "---\n\
id: dec.cli-json-default\n\
nodes: [cairn.kernel.nonexistent]\n\
status: accepted\n\
date: 2026-08-09\n\
---\n\
\n\
# CLI output is human-readable by default\n\
\n\
## Decision\n\
\n\
Text by default.\n";

pub fn manifest_dir() -> Utf8PathBuf {
    Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn fixture() -> Utf8PathBuf {
    manifest_dir().join("tests/fixtures/cairn-bootstrap")
}

pub fn config(backend_max_repairs: u32) -> RunConfig {
    RunConfig {
        fixture: fixture(),
        cairn_bin: Utf8PathBuf::from(env!("CARGO_BIN_EXE_cairn")),
        backend: BackendSpec::Replay,
        max_repairs: backend_max_repairs,
        timeout_ms: 30_000,
    }
}

pub fn write_turn(contents: &str) -> serde_json::Value {
    json!({
        "kind": "response",
        "files": [{"path": TARGET, "contents": contents}],
        "tokens": {"prompt": 100, "completion": 20},
    })
}

pub fn write_prompt(dir: &Utf8Path, id: &str, turns: &[serde_json::Value]) -> Utf8PathBuf {
    let path = dir.join(format!("{id}.json"));
    let prompt = json!({
        "schema_version": 1,
        "id": id,
        "instruction": "Author meta/decisions/cli-json-default.md for cairn.kernel.cli.",
        "expects": [TARGET],
        "replay": {"model": MODEL, "turns": turns},
    });
    std::fs::write(
        &path,
        serde_json::to_string(&prompt).expect("serialise prompt"),
    )
    .expect("write prompt");
    path
}

pub fn scratch() -> (tempfile::TempDir, Utf8PathBuf) {
    let dir = tempfile::tempdir().expect("temp dir");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8 temp dir");
    (dir, root)
}

pub fn run(max_repairs: u32, id: &str, turns: &[serde_json::Value]) -> Record {
    let (_guard, dir) = scratch();
    let prompt = write_prompt(&dir, id, turns);
    run_prompt_file(&config(max_repairs), &prompt).expect("the instrument must produce a record")
}

/// Content hash of every file under `root`, keyed by relative path.
pub fn tree_digest(root: &Utf8Path) -> BTreeMap<String, String> {
    fn walk(root: &Utf8Path, current: &Utf8Path, out: &mut BTreeMap<String, String>) {
        for entry in std::fs::read_dir(current).expect("read fixture dir") {
            let entry = entry.expect("fixture entry");
            let path = Utf8PathBuf::from_path_buf(entry.path()).expect("utf-8 fixture path");
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                let bytes = std::fs::read(&path).expect("read fixture file");
                let relative = path
                    .strip_prefix(root)
                    .expect("fixture-relative path")
                    .to_string();
                out.insert(relative, format!("{:x}", Sha256::digest(&bytes)));
            }
        }
    }

    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}
