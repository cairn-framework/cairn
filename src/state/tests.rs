//! Tests for state backend persistence and queries.

use super::{FilesystemStateBackend, StateRecord, beads, storage_backend};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TestRecord {
    id: String,
    count: u32,
}

fn temp_backend() -> FilesystemStateBackend {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cairn-state-test-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    FilesystemStateBackend::new(dir)
}

#[test]
fn round_trip_save_and_load() {
    let backend = temp_backend();
    let record = TestRecord {
        id: "test-1".to_owned(),
        count: 42,
    };
    backend.save("rec1", &record).unwrap();
    let loaded: Option<TestRecord> = backend.load("rec1").unwrap();
    assert_eq!(loaded, Some(record));
}

#[test]
fn load_missing_returns_none() {
    let backend = temp_backend();
    let loaded: Option<TestRecord> = backend.load("missing").unwrap();
    assert!(loaded.is_none());
}

#[test]
fn list_returns_saved_keys() {
    let backend = temp_backend();
    backend
        .save(
            "a",
            &TestRecord {
                id: "a".to_owned(),
                count: 1,
            },
        )
        .unwrap();
    backend
        .save(
            "b",
            &TestRecord {
                id: "b".to_owned(),
                count: 2,
            },
        )
        .unwrap();
    let mut keys = backend.list().unwrap();
    keys.sort();
    assert_eq!(keys, vec!["a", "b"]);
}

#[test]
fn list_empty_when_root_missing() {
    let backend = temp_backend();
    let keys = backend.list().unwrap();
    assert!(keys.is_empty());
}

#[test]
fn remove_deletes_record() {
    let backend = temp_backend();
    backend
        .save(
            "del",
            &TestRecord {
                id: "x".to_owned(),
                count: 0,
            },
        )
        .unwrap();
    assert!(backend.load::<TestRecord>("del").unwrap().is_some());
    backend.remove("del").unwrap();
    assert!(backend.load::<TestRecord>("del").unwrap().is_none());
}

#[test]
fn remove_missing_is_idempotent() {
    let backend = temp_backend();
    backend.remove("never-existed").unwrap();
}

#[test]
fn save_overwrites_existing() {
    let backend = temp_backend();
    backend
        .save(
            "key",
            &TestRecord {
                id: "first".to_owned(),
                count: 1,
            },
        )
        .unwrap();
    backend
        .save(
            "key",
            &TestRecord {
                id: "second".to_owned(),
                count: 2,
            },
        )
        .unwrap();
    let loaded: TestRecord = backend.load("key").unwrap().unwrap();
    assert_eq!(loaded.id, "second");
    assert_eq!(loaded.count, 2);
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct QueryableRecord {
    id: String,
    record_type: String,
    labels: Vec<String>,
    dependencies: Vec<String>,
}

impl StateRecord for QueryableRecord {
    fn record_type(&self) -> &str {
        &self.record_type
    }

    fn labels(&self) -> &[String] {
        &self.labels
    }

    fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
}

#[test]
fn query_by_type_filters_records() {
    let backend = temp_backend();
    backend
        .save(
            "draft1",
            &QueryableRecord {
                id: "d1".to_owned(),
                record_type: "draft".to_owned(),
                labels: vec![],
                dependencies: vec![],
            },
        )
        .unwrap();
    backend
        .save(
            "snapshot1",
            &QueryableRecord {
                id: "s1".to_owned(),
                record_type: "snapshot".to_owned(),
                labels: vec![],
                dependencies: vec![],
            },
        )
        .unwrap();
    let results: Vec<(String, QueryableRecord)> = backend.query_by_type("draft").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "draft1");
    assert_eq!(results[0].1.id, "d1");
}

#[test]
fn query_by_label_filters_records() {
    let backend = temp_backend();
    backend
        .save(
            "rec1",
            &QueryableRecord {
                id: "r1".to_owned(),
                record_type: "x".to_owned(),
                labels: vec!["urgent".to_owned()],
                dependencies: vec![],
            },
        )
        .unwrap();
    backend
        .save(
            "rec2",
            &QueryableRecord {
                id: "r2".to_owned(),
                record_type: "x".to_owned(),
                labels: vec!["low".to_owned()],
                dependencies: vec![],
            },
        )
        .unwrap();
    let results: Vec<(String, QueryableRecord)> = backend.query_by_label("urgent").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "rec1");
}

#[test]
fn query_by_dependency_filters_records() {
    let backend = temp_backend();
    backend
        .save(
            "rec1",
            &QueryableRecord {
                id: "r1".to_owned(),
                record_type: "x".to_owned(),
                labels: vec![],
                dependencies: vec!["db".to_owned()],
            },
        )
        .unwrap();
    backend
        .save(
            "rec2",
            &QueryableRecord {
                id: "r2".to_owned(),
                record_type: "x".to_owned(),
                labels: vec![],
                dependencies: vec!["api".to_owned()],
            },
        )
        .unwrap();
    let results: Vec<(String, QueryableRecord)> = backend.query_by_dependency("db").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "rec1");
}

#[test]
fn storage_backend_filesystem_returns_working_backend() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cairn-state-be-test-{}-{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let backend = storage_backend("filesystem", dir.clone()).unwrap();
    backend
        .save(
            "key",
            &TestRecord {
                id: "x".to_owned(),
                count: 1,
            },
        )
        .unwrap();
    let loaded: Option<TestRecord> = backend.load("key").unwrap();
    assert_eq!(loaded.unwrap().id, "x");
}

#[test]
fn storage_backend_unknown_returns_error() {
    let dir = std::env::temp_dir().join("cairn-state-be-unknown");
    let result = storage_backend("s3", dir);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("unknown state backend"));
    assert!(err.contains("s3"));
}
#[test]
fn storage_backend_beads_returns_working_backend() {
    if std::process::Command::new("bd")
        .arg("--version")
        .output()
        .is_err()
    {
        return;
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let backend = storage_backend("beads", root).unwrap();
    let key = format!("test-{}", std::process::id());
    backend
        .save(
            &key,
            &TestRecord {
                id: "x".to_owned(),
                count: 1,
            },
        )
        .unwrap();
    let loaded: Option<TestRecord> = backend.load(&key).unwrap();
    assert_eq!(loaded.unwrap().id, "x");
    backend.remove(&key).unwrap();
}
/// Initialise a throw-away beads database in a temporary directory.
///
/// Returns `None` when the `bd` binary is not on `PATH` so that tests
/// can skip gracefully in CI environments where beads is not installed.
///
/// The returned `TempDir` keeps the directory alive for the duration of
/// the test; it is deleted automatically on drop, so no manual `bd delete`
/// cleanup is needed.
fn temp_beads_root() -> Option<(beads::BeadsStateBackend, tempfile::TempDir)> {
    let dir = tempfile::TempDir::new().expect("create temp dir");
    let status = std::process::Command::new("bd")
        .current_dir(dir.path())
        .arg("init")
        .arg("--non-interactive")
        .arg("--prefix")
        .arg("cairn")
        .arg("--skip-agents")
        .arg("--skip-hooks")
        .status()
        .ok()?;
    if !status.success() {
        return None;
    }
    let backend = beads::BeadsStateBackend::new(dir.path().to_path_buf());
    Some((backend, dir))
}

#[test]
fn beads_create_change_epic_returns_bead_id() {
    let Some((backend, _dir)) = temp_beads_root() else {
        return;
    };
    let bead_id = backend.create_change_epic("my-change").unwrap();
    assert!(!bead_id.is_empty(), "bead ID must not be empty");
    assert!(
        bead_id.starts_with("cairn-"),
        "bead ID should start with cairn- prefix, got: {bead_id}"
    );
}

#[test]
fn beads_create_task_beads_returns_bead_ids() {
    let Some((backend, _dir)) = temp_beads_root() else {
        return;
    };
    let epic_id = backend.create_change_epic("my-change").unwrap();
    let task_ids = backend
        .create_task_beads(&epic_id, &["First task", "Second task"])
        .unwrap();
    assert_eq!(task_ids.len(), 2, "expected 2 task beads");
    for id in &task_ids {
        assert!(
            id.starts_with("cairn-"),
            "task bead ID should start with cairn-"
        );
    }
}

#[test]
fn beads_list_child_tasks_returns_task_titles() {
    let Some((backend, _dir)) = temp_beads_root() else {
        return;
    };
    let epic_id = backend.create_change_epic("my-change").unwrap();
    backend
        .create_task_beads(&epic_id, &["Alpha", "Beta"])
        .unwrap();
    let tasks = backend.list_child_tasks(&epic_id).unwrap();
    assert_eq!(tasks.len(), 2, "expected 2 child tasks");
}

#[test]
fn beads_claim_change_succeeds() {
    let Some((backend, _dir)) = temp_beads_root() else {
        return;
    };
    let epic_id = backend.create_change_epic("my-change").unwrap();
    backend
        .create_task_beads(&epic_id, &["One", "Two"])
        .unwrap();
    backend.claim_change(&epic_id).unwrap();
}
