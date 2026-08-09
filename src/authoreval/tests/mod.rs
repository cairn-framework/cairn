//! Unit tests for the parts of the instrument that need no model and no scan.

use camino::Utf8PathBuf;

use super::backend::FileEdit;
use super::workspace::Workspace;

fn edit(path: &str, contents: &str) -> FileEdit {
    FileEdit {
        path: path.to_owned(),
        contents: contents.to_owned(),
    }
}

/// Validate-then-write, the pairing the runner performs.
fn apply(workspace: &Workspace, files: &[FileEdit]) -> Result<(), crate::error::CairnError> {
    let targets = workspace
        .validate(files)
        .map_err(|message| crate::error::CairnError::AuthorEval { message })?;
    Workspace::write(files, &targets)
}

fn args(raw: &[&str]) -> Vec<String> {
    raw.iter().map(|value| (*value).to_owned()).collect()
}

fn scratch_fixture() -> (tempfile::TempDir, Utf8PathBuf) {
    let dir = tempfile::tempdir().expect("temp fixture");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8 temp dir");
    std::fs::write(root.join("keep.txt"), "original").expect("seed fixture");
    (dir, root)
}

mod cli;
mod prompt;
mod taxonomy;
mod workspace;
