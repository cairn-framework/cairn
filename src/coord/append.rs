//! The single write path for coordination facts.
//!
//! Every sanctioned verb appends one file and does nothing else. The console
//! barrier lives here: `dec.orchestration-placement` clause 4 says the
//! console never acquires or renews a lease, so the one shared write path
//! refuses `lease.*` and `driver.singleton.*` from a console actor as a
//! checked invariant rather than a convention.

use std::path::{Path, PathBuf};

use crate::persist;

use super::envelope::{Actor, Envelope, STORE_FORMAT, evidence_class_for, fact_id_for};
use super::store;
use super::time::compact_rfc3339;

/// A fact as a caller states it; the appender derives everything else.
pub struct NewFact {
    /// Fact kind, e.g. `ruling.run`.
    pub kind: String,
    /// RFC 3339 UTC recording instant.
    pub recorded_at: String,
    /// The recording actor.
    pub recorded_by: Actor,
    /// 40-hex commit at recording.
    pub commit: String,
    /// Fact id this record supersedes.
    pub supersedes: Option<String>,
    /// Family-specific payload.
    pub payload: serde_json::Value,
}

/// Appends one fact to the family store for `root`, initialising the store
/// on first use. Returns the written path.
///
/// # Errors
///
/// Refuses a console-actor `lease.*` or `driver.singleton.*` fact, an
/// unknown fact kind or actor kind, a malformed commit, and any store or
/// write failure.
pub fn append_fact(root: &Path, fact: NewFact) -> Result<PathBuf, String> {
    if !matches!(
        fact.recorded_by.kind.as_str(),
        "maintainer" | "driver" | "console"
    ) {
        return Err(format!(
            "unknown actor kind `{}`; expected maintainer, driver, or console",
            fact.recorded_by.kind
        ));
    }
    if fact.recorded_by.kind == "console"
        && (fact.kind.starts_with("lease.") || fact.kind.starts_with("driver.singleton."))
    {
        return Err(format!(
            "the console never writes `{}` facts (dec.orchestration-placement clause 4)",
            fact.kind
        ));
    }
    let Some(evidence_class) = evidence_class_for(&fact.kind) else {
        return Err(format!(
            "fact kind `{}` is outside the sanctioned families",
            fact.kind
        ));
    };
    if fact.commit.len() != 40 || !fact.commit.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!(
            "commit `{}` is not a 40-hex object id",
            fact.commit
        ));
    }

    let mut envelope = Envelope {
        format: STORE_FORMAT,
        fact_id: String::new(),
        kind: fact.kind,
        recorded_at: fact.recorded_at,
        recorded_by: fact.recorded_by,
        commit: fact.commit,
        evidence_class: evidence_class.to_owned(),
        supersedes: fact.supersedes,
        payload: fact.payload,
    };
    envelope.fact_id = fact_id_for(&envelope)?;

    let store = store::store_root(root)?;
    store::ensure_initialised(&store)?;
    let name = format!(
        "{}-{}-{}.json",
        compact_rfc3339(&envelope.recorded_at),
        envelope.kind,
        envelope.fact_id
    );
    let path = store.join("facts").join(name);
    let body = serde_json::to_string_pretty(&envelope)
        .map_err(|error| format!("fact does not serialise: {error}"))?;
    persist::atomic_write(&path, &format!("{body}\n"))
        .map_err(|error| format!("cannot write fact: {error}"))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let run = |args: &[&str]| {
            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(dir.path())
                .args(args)
                .output()
                .expect("git runs");
            assert!(output.status.success());
        };
        run(&["init", "-q"]);
        dir
    }

    fn fact(kind: &str, actor: &str) -> NewFact {
        NewFact {
            kind: kind.to_owned(),
            recorded_at: "2026-08-07T03:45:12Z".to_owned(),
            recorded_by: Actor {
                kind: actor.to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({ "target": "todo.example" }),
        }
    }

    #[test]
    fn console_lease_grant_is_refused() {
        let dir = repo();
        let error = append_fact(dir.path(), fact("lease.grant", "console"))
            .expect_err("console lease refused");
        assert!(error.contains("console never writes"), "{error}");
        let error = append_fact(dir.path(), fact("driver.singleton.grant", "console"))
            .expect_err("console singleton refused");
        assert!(error.contains("console never writes"), "{error}");
        assert!(
            !dir.path().join(".git/cairn/coord").exists(),
            "a refused append initialises nothing"
        );
    }

    #[test]
    fn append_initialises_the_store_and_names_the_file_by_instant_kind_id() {
        let dir = repo();
        let path = append_fact(dir.path(), fact("ruling.run", "maintainer")).expect("appends");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        assert!(name.starts_with("20260807T034512Z-ruling.run-"), "{name}");
        assert!(
            std::path::Path::new(&name)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        );
        let store = dir.path().join(".git/cairn/coord");
        assert_eq!(
            std::fs::read_to_string(store.join("format")).expect("format"),
            "1\n"
        );
        let written: Envelope =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("fact bytes"))
                .expect("fact parses");
        assert_eq!(written.evidence_class, "attested");
        assert_eq!(written.fact_id.len(), 12);
    }

    #[test]
    fn evidence_class_is_derived_never_chosen() {
        let dir = repo();
        let path =
            append_fact(dir.path(), fact("outcome.touched_files", "driver")).expect("appends");
        let written: Envelope =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("fact bytes"))
                .expect("fact parses");
        assert_eq!(written.evidence_class, "observed");
    }

    #[test]
    fn unsanctioned_kind_is_refused() {
        let dir = repo();
        let error = append_fact(dir.path(), fact("gossip.rumour", "driver")).expect_err("refused");
        assert!(error.contains("outside the sanctioned families"), "{error}");
    }
}
