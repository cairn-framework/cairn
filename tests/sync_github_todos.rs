//! Behavioural tests for scripts/sync-github-todos.sh.
//!
//! The script is a one-way projector from meta/todos/*.md to GitHub issues.
//! These tests run it against a stub `gh` binary that serves canned issue
//! state and records every mutating invocation, asserting the reconciliation
//! rules from res.github-todo-sync: idempotent upsert, done/deleted closes,
//! inward flagging without import, and a mutation-free dry run.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Sandbox {
    dir: tempfile::TempDir,
}

impl Sandbox {
    /// `projection` is the TSV the stub returns for the marker-labelled
    /// issue list (number, state, title, slug per line). `unmapped` is the
    /// newline-separated issue numbers returned for the external scan.
    fn new(projection: &str, unmapped: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::create_dir_all(root.join("meta/todos")).unwrap();
        fs::create_dir_all(root.join("bin")).unwrap();
        fs::write(root.join("projection.tsv"), projection).unwrap();
        fs::write(root.join("unmapped.txt"), unmapped).unwrap();
        let stub = format!(
            r#"#!/usr/bin/env bash
# Stub gh: serve canned lists, record every mutating call, and expose body payloads.
root="{root}"
args="$*"
if [[ "$1 $2" == "issue list" ]]; then
    [[ -e "$root/list-fails" ]] && exit 1
    if [[ "$args" == *"--label"* ]]; then
        while IFS=$'\t' read -r number state title slug status node_field; do
            [ -n "$number" ] || continue
            body_b64=""
            if [[ -e "$root/body.$slug" ]]; then
                body_b64="$(base64 < "$root/body.$slug" | tr -d '\n')"
            fi
            printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
                "$number" "$state" "$title" "$slug" "$status" "$node_field" "$body_b64"
        done < "$root/projection.tsv"
    else
        cat "$root/unmapped.txt"
    fi
    exit 0
fi
if [[ "$1 $2" == "issue create" || "$1 $2" == "issue edit" ]]; then
    i=1
    while (( i <= $# )); do
        if [[ "${{!i}}" == "--body" ]]; then
            body_arg=$((i + 1))
            printf '%s' "${{!body_arg}}" > "$root/last-body"
            break
        fi
        i=$((i + 1))
    done
fi
echo "$args" >> "$root/mutations.log"
"#,
            root = root.display()
        );
        let stub_path = root.join("bin/gh");
        fs::write(&stub_path, stub).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&stub_path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        Self { dir }
    }

    fn add_todo(&self, slug: &str, status: &str, title: &str) {
        fs::write(
            self.dir.path().join(format!("meta/todos/todo.{slug}.md")),
            format!(
                "---\nnode: cairn.root\nstatus: {status}\ncreated: 2026-07-12\n---\n\n\
                 # {title}\n\n## Problem\n\nBody.\n\n## Acceptance\n\nAccept.\n"
            ),
        )
        .unwrap();
    }

    fn run(&self, dry_run: bool) -> String {
        let script = script_path();
        let mut cmd = Command::new("bash");
        cmd.arg(&script);
        if dry_run {
            cmd.arg("--dry-run");
        }
        let path = format!(
            "{}:{}",
            self.dir.path().join("bin").display(),
            std::env::var("PATH").unwrap_or_default()
        );
        let out = cmd
            .current_dir(self.dir.path())
            .env("PATH", path)
            .env("GH_REPO", "stub/repo")
            .output()
            .expect("script runs");
        assert!(
            out.status.success(),
            "script failed: {}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn mutations(&self) -> Vec<String> {
        fs::read_to_string(self.dir.path().join("mutations.log"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect()
    }
    fn rendered_body(&self, slug: &str, status: &str, node: &str) -> String {
        let raw =
            fs::read_to_string(self.dir.path().join(format!("meta/todos/todo.{slug}.md"))).unwrap();
        let markdown = raw.splitn(3, "---\n").nth(2).unwrap();
        format!(
            "cairn-todo: todo.{slug}\nnode: {node}\nstatus: {status}\n\
             artefact: meta/todos/todo.{slug}.md\n\
             one-way mirror of a cairn todo; edits here are not read back, \
             dec.task-tracking-authority\n{markdown}"
        )
    }

    fn set_projection_body(&self, slug: &str, body: &str) {
        fs::write(self.dir.path().join(format!("body.{slug}")), body).unwrap();
    }

    fn last_body(&self) -> String {
        fs::read_to_string(self.dir.path().join("last-body")).unwrap_or_default()
    }
}

fn script_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/sync-github-todos.sh")
}

#[test]
fn open_todo_without_issue_is_created() {
    let sb = Sandbox::new("", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    sb.run(false);
    let muts = sb.mutations();
    assert!(
        muts.iter()
            .any(|m| m.starts_with("issue create") && m.contains("[todo] Alpha Work")),
        "expected a create, got {muts:?}"
    );
}
#[test]
fn create_projects_full_todo_body() {
    let sb = Sandbox::new("", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    sb.run(false);
    let body = sb.last_body();
    assert_eq!(
        body,
        sb.rendered_body("alpha", "open", "cairn.root"),
        "create must carry the complete canonical projection"
    );
    assert!(
        body.contains("# Alpha Work") && body.contains("Body."),
        "expected the full todo body in create payload, got {body}"
    );
    assert!(
        body.contains("one-way mirror of a cairn todo; edits here are not read back"),
        "expected a one-line one-way note, got {body}"
    );
    assert!(
        !body.contains("Files in git are the source of truth"),
        "prior multi-line disclaimer must be dropped, got {body}"
    );
}

#[test]
fn matching_state_is_a_noop() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.root\n", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    sb.set_projection_body("alpha", &sb.rendered_body("alpha", "open", "cairn.root"));
    sb.run(false);
    let muts: Vec<String> = sb
        .mutations()
        .into_iter()
        .filter(|m| !m.starts_with("label create"))
        .collect();
    assert!(muts.is_empty(), "expected a no-op, got {muts:?}");
}

#[test]
fn done_todo_closes_its_open_issue() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.root\n", "");
    sb.add_todo("alpha", "done", "Alpha Work");
    sb.run(false);
    assert!(
        sb.mutations()
            .iter()
            .any(|m| m.starts_with("issue close 7")),
        "expected close of #7, got {:?}",
        sb.mutations()
    );
}

#[test]
fn reopened_todo_reopens_closed_issue() {
    let sb = Sandbox::new(
        "7\tCLOSED\t[todo] Alpha Work\talpha\tblocked\tcairn.root\n",
        "",
    );
    sb.add_todo("alpha", "blocked", "Alpha Work");
    sb.run(false);
    assert!(
        sb.mutations()
            .iter()
            .any(|m| m.starts_with("issue reopen 7")),
        "expected reopen of #7, got {:?}",
        sb.mutations()
    );
}

#[test]
fn deleted_todo_closes_only_marker_owned_issue() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Gone Work\tgone\topen\tcairn.root\n", "");
    sb.run(false);
    let muts = sb.mutations();
    assert!(
        muts.iter().any(|m| m.starts_with("issue close 7")),
        "expected close of orphaned mirror #7, got {muts:?}"
    );
}

#[test]
fn external_issue_is_flagged_never_imported() {
    let sb = Sandbox::new("", "42\n");
    sb.run(false);
    let muts = sb.mutations();
    assert!(
        muts.iter()
            .any(|m| m.starts_with("issue edit 42") && m.contains("cairn-todo-unmapped")),
        "expected unmapped label on #42, got {muts:?}"
    );
    assert!(
        muts.iter().any(|m| m.starts_with("issue comment 42")),
        "expected triage comment on #42, got {muts:?}"
    );
    assert!(
        !sb.dir.path().join("meta/todos/todo.42.md").exists(),
        "no todo may be auto-created from an issue"
    );
}

#[test]
fn dry_run_performs_no_mutations() {
    let sb = Sandbox::new(
        "7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.root\n",
        "42\n",
    );
    sb.add_todo("alpha", "done", "Alpha Work");
    sb.add_todo("beta", "open", "Beta Work");
    let out = sb.run(true);
    assert!(sb.mutations().is_empty(), "dry run must not mutate");
    assert!(out.contains("close #7"), "dry run still reports the plan");
}

#[test]
fn done_todo_with_no_issue_is_not_projected() {
    let sb = Sandbox::new("", "");
    sb.add_todo("alpha", "done", "Alpha Work");
    sb.run(false);
    let muts: Vec<String> = sb
        .mutations()
        .into_iter()
        .filter(|m| !m.starts_with("label create"))
        .collect();
    assert!(
        muts.is_empty(),
        "done todos need no new issue, got {muts:?}"
    );
}
#[test]
fn status_change_rewrites_projected_body() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.root\n", "");
    sb.add_todo("alpha", "blocked", "Alpha Work");
    sb.run(false);
    let log = sb.mutations().join("\n");
    assert!(
        log.contains("issue edit 7 --body") && log.contains("status: blocked"),
        "expected body rewrite for status change, got {log}"
    );
}

#[test]
fn node_change_rewrites_projected_body() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.ui\n", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    sb.run(false);
    let log = sb.mutations().join("\n");
    assert!(
        log.contains("issue edit 7 --body") && log.contains("node: cairn.root"),
        "expected body rewrite for node change, got {log}"
    );
}
#[test]
fn body_only_change_rewrites_projected_body() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.root\n", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    sb.set_projection_body("alpha", "stale body\n");
    sb.run(false);
    let log = sb.mutations().join("\n");
    assert!(
        log.contains("issue edit 7 --body"),
        "expected body-only change to rebody, got {log}"
    );
    assert_eq!(
        sb.last_body(),
        sb.rendered_body("alpha", "open", "cairn.root"),
        "rebody must carry the complete canonical projection"
    );
}

#[test]
fn unchanged_body_is_not_rewritten_on_second_run() {
    let sb = Sandbox::new("7\tOPEN\t[todo] Alpha Work\talpha\topen\tcairn.root\n", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    let body = sb.rendered_body("alpha", "open", "cairn.root");
    sb.set_projection_body("alpha", &body);
    sb.run(false);
    sb.run(false);
    let edits: Vec<String> = sb
        .mutations()
        .into_iter()
        .filter(|m| m.starts_with("issue edit") && m.contains("--body"))
        .collect();
    assert!(
        edits.is_empty(),
        "unchanged projected body must not be rewritten, got {edits:?}"
    );
}

#[test]
fn failed_inventory_aborts_without_mutation() {
    let sb = Sandbox::new("", "");
    sb.add_todo("alpha", "open", "Alpha Work");
    fs::write(sb.dir.path().join("list-fails"), "").unwrap();
    let script = script_path();
    let path = format!(
        "{}:{}",
        sb.dir.path().join("bin").display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let out = Command::new("bash")
        .arg(&script)
        .current_dir(sb.dir.path())
        .env("PATH", path)
        .env("GH_REPO", "stub/repo")
        .output()
        .expect("script spawns");
    assert!(
        !out.status.success(),
        "a failed issue inventory must abort the run"
    );
    assert!(
        sb.mutations().iter().all(|m| m.starts_with("label create")),
        "no reconciliation mutation may follow a failed inventory, got {:?}",
        sb.mutations()
    );
}

#[test]
fn triage_comment_precedes_exclusion_label() {
    let sb = Sandbox::new("", "42\n");
    sb.run(false);
    let muts = sb.mutations();
    let comment = muts.iter().position(|m| m.starts_with("issue comment 42"));
    let label = muts
        .iter()
        .position(|m| m.starts_with("issue edit 42") && m.contains("cairn-todo-unmapped"));
    assert!(
        comment.is_some() && label.is_some() && comment < label,
        "comment must post before the exclusion label so failures retry, got {muts:?}"
    );
}
