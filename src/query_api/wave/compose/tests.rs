//! Composer acceptance tests: digest stability, wave admission,
//! parked subtraction, canonical preimage shape, and the concurrent
//! tie-break.

use super::*;
use crate::blueprint::NodeKind;
use crate::map::graph::{NodeRecord, NodeState};
use std::collections::BTreeMap;

fn node(id: &str, paths: &[&str]) -> NodeRecord {
    NodeRecord {
        kind: NodeKind::Module,
        id: id.to_owned(),
        name: id.to_owned(),
        description: String::new(),
        tags: Vec::new(),
        parent: None,
        children: Vec::new(),
        paths: paths.iter().map(ToString::to_string).collect(),
        owns_files: false,
        contracts: Vec::new(),
        state: NodeState::Synced,
        files: Vec::new(),
        symbols: Vec::new(),
        span: crate::blueprint::Span::point(String::new(), 1, 1),
    }
}

fn graph(nodes: Vec<NodeRecord>) -> Graph {
    Graph {
        nodes: nodes.into_iter().map(|n| (n.id.clone(), n)).collect(),
        names: BTreeMap::new(),
        outbound: BTreeMap::new(),
        inbound: BTreeMap::new(),
        findings: Vec::new(),
    }
}

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
    run(&[
        "-c",
        "user.name=t",
        "-c",
        "user.email=t@t",
        "commit",
        "--allow-empty",
        "-q",
        "-m",
        "base",
    ]);
    dir
}

fn todo(root: &Path, slug: &str, node_id: &str, body: &str) -> Todo {
    let dir = root.join("meta/todos");
    std::fs::create_dir_all(&dir).expect("todos dir");
    let path = dir.join(format!("todo.{slug}.md"));
    std::fs::write(&path, body).expect("todo file");
    Todo {
        path: path.to_string_lossy().to_string(),
        node: node_id.to_owned(),
        status: TodoStatus::Open,
        created: "2026-08-07".to_owned(),
        satisfies: None,
        blocked_by: Vec::new(),
        parent: None,
        related: Vec::new(),
        defers: Vec::new(),
        body: body.to_owned(),
    }
}

#[test]
fn digest_is_stable_across_unrelated_commits_and_moves_with_content() {
    let dir = repo();
    let g = graph(vec![
        node("app.a", &["./src/a"]),
        node("app.b", &["./src/b"]),
    ]);
    let todos = vec![
        todo(dir.path(), "alpha", "app.a", "# Alpha\n"),
        todo(dir.path(), "beta", "app.b", "# Beta\n"),
    ];
    let first = compose_wave(dir.path(), &g, &todos, None).expect("composes");
    assert_eq!(first.units.len(), 2, "disjoint units share the wave");
    assert!(first.units[0].hotspot_permission);
    assert!(!first.units[1].hotspot_permission);
    assert!(first.digest.starts_with("plan-"), "{}", first.digest);
    assert_eq!(first.digest.len(), 21);

    // An unrelated commit does not change the digest: the base commit
    // is absent from the preimage by design.
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "--allow-empty",
            "-q",
            "-m",
            "unrelated",
        ])
        .output()
        .expect("git runs");
    assert!(output.status.success());
    let second = compose_wave(dir.path(), &g, &todos, None).expect("recomposes");
    assert_eq!(first.digest, second.digest);

    // A changed task definition changes the digest: the content hash
    // is the panel's correctness graft.
    let rewritten = todo(dir.path(), "alpha", "app.a", "# Alpha rewritten\n");
    let third =
        compose_wave(dir.path(), &g, &[rewritten, todos[1].clone()], None).expect("recomposes");
    assert_ne!(first.digest, third.digest);

    // A changed unit set changes the digest.
    let fourth = compose_wave(dir.path(), &g, &todos[..1], None).expect("recomposes");
    assert_ne!(first.digest, fourth.digest);

    // A changed rule changes the digest.
    assert_ne!(
        digest(&preimage("wf.default:1", &first.units)),
        digest(&preimage("wf.other:2", &first.units)),
    );
}

#[test]
fn overlapping_units_are_held_behind_the_admitted_unit() {
    let dir = repo();
    let g = graph(vec![node("app.a", &["./src/a"])]);
    let todos = vec![
        todo(dir.path(), "alpha", "app.a", "# Alpha\n"),
        todo(dir.path(), "gamma", "app.a", "# Gamma\n"),
    ];
    let wave = compose_wave(dir.path(), &g, &todos, None).expect("composes");
    assert_eq!(wave.units.len(), 1);
    assert_eq!(wave.units[0].id, "todo.alpha");
    assert_eq!(wave.held.len(), 1);
    assert_eq!(wave.held[0].id, "todo.gamma");
    assert_eq!(wave.held[0].reason, "write-sets-overlap");
    assert_eq!(wave.held[0].behind.as_deref(), Some("todo.alpha"));
}

#[test]
fn parked_units_are_subtracted_and_preimage_shape_is_canonical() {
    let dir = repo();
    let g = graph(vec![
        node("app.a", &["./src/a"]),
        node("app.b", &["./src/b"]),
    ]);
    let todos = vec![
        todo(dir.path(), "alpha", "app.a", "# Alpha\n"),
        todo(dir.path(), "beta", "app.b", "# Beta\n"),
    ];
    crate::coord::append::append_fact(
        dir.path(),
        crate::coord::append::NewFact {
            kind: "ruling.park".to_owned(),
            recorded_at: "2026-08-07T03:00:00Z".to_owned(),
            recorded_by: crate::coord::envelope::Actor {
                kind: "maintainer".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            supersedes: None,
            payload: serde_json::json!({ "target": "todo.alpha" }),
        },
    )
    .expect("park appends");
    let wave = compose_wave(dir.path(), &g, &todos, None).expect("composes");
    assert_eq!(wave.units.len(), 1, "the parked unit is subtracted");
    assert_eq!(wave.held[0].reason, "parked");
    assert!(wave.held[0].blocking_fact_id.is_some());
    let hash = &wave.units[0].content_hash;
    assert_eq!(
        wave.preimage,
        format!("cairn-plan-v1\nrule=wf.default:1\nunit=todo.beta@{hash}\nws=src/b\n"),
        "line-oriented, sorted, one trailing LF, no base commit"
    );
}

#[test]
fn concurrent_rulings_resolve_deterministically() {
    use crate::coord::envelope::{Actor, Envelope};
    use crate::coord::read::NamedFact;
    let fact = |recorded_at: &str, fact_id: &str| NamedFact {
        name: format!("{recorded_at}-ruling.run-{fact_id}.json"),
        fact: Envelope {
            format: 1,
            fact_id: fact_id.to_owned(),
            kind: "ruling.run".to_owned(),
            recorded_at: recorded_at.to_owned(),
            recorded_by: Actor {
                kind: "maintainer".to_owned(),
                id: "t".to_owned(),
            },
            commit: "a".repeat(40),
            evidence_class: "attested".to_owned(),
            supersedes: None,
            payload: serde_json::json!({ "target": "plan-0123456789abcdef" }),
        },
    };
    let earlier = fact("2026-08-07T03:00:00Z", "bbbbbbbbbbbb");
    let same_second = fact("2026-08-07T03:00:00Z", "aaaaaaaaaaaa");
    let later = fact("2026-08-07T03:00:01Z", "000000000000");
    let winner = concurrent_winner(&[&earlier, &same_second, &later]).expect("winner");
    assert_eq!(
        winner.fact.fact_id, "aaaaaaaaaaaa",
        "the (recorded_at, fact_id) pair is a total order over the full listing"
    );
    // Order of listing does not change the winner.
    let winner = concurrent_winner(&[&later, &earlier, &same_second]).expect("winner");
    assert_eq!(winner.fact.fact_id, "aaaaaaaaaaaa");
}
