//! Loop-level unit tests: the repair-feedback contract, the scoring envelope,
//! and the command backend's subprocess lifecycle, driven against shell stubs
//! so they need no built binary and no model.
//!
//! The whole module is unix-only; see the `cfg` on its declaration.

use camino::{Utf8Path, Utf8PathBuf};

use super::backend::{
    AuthorRequest, AuthorResponse, AuthorevalBackend, BackendError, BackendIdentity, FileEdit,
    TokenUsage,
};
use super::prompt::Prompt;
use super::runner::{BackendSpec, RunConfig};
use super::scorer::Finding;

/// A backend that records every request it is handed and serves a fixed script.
///
/// The shipped `ReplayBackend` ignores the request entirely, so no test built
/// on it can defend the repair-feedback contract: the loop could send an empty
/// finding list, or an unsorted one, and every replay assertion would still
/// pass.
struct RecordingBackend {
    requests: std::cell::RefCell<Vec<Vec<Finding>>>,
    responses: Vec<AuthorResponse>,
}

impl RecordingBackend {
    fn new(responses: Vec<AuthorResponse>) -> Self {
        Self {
            requests: std::cell::RefCell::new(Vec::new()),
            responses,
        }
    }
}

impl AuthorevalBackend for RecordingBackend {
    fn identity(&self) -> BackendIdentity {
        BackendIdentity {
            kind: "recording".to_owned(),
            model: "recording/v1".to_owned(),
        }
    }

    fn invoke(
        &self,
        request: &AuthorRequest<'_>,
        _timeout: std::time::Duration,
    ) -> Result<AuthorResponse, BackendError> {
        self.requests.borrow_mut().push(request.findings.to_vec());
        let index = self.requests.borrow().len() - 1;
        self.responses
            .get(index)
            .cloned()
            .ok_or(BackendError::ScriptExhausted {
                attempt: request.attempt,
            })
    }
}

/// A stand-in `cairn` that answers both scoring surfaces from a script.
///
/// `scan --strict` exits 1 while the marker file is absent and 0 once the
/// second attempt has been reached; `lint --json` prints a deliberately
/// unsorted wire so the loop's ordering guarantee is observable.
fn stub_cairn(dir: &Utf8Path) -> Utf8PathBuf {
    use std::os::unix::fs::PermissionsExt as _;

    let marker = dir.join("scan-count");
    let script = dir.join("cairn-stub.sh");
    let body = format!(
        r#"#!/bin/sh
case "$1" in
  scan)
    printf 'x' >> "{marker}"
    if [ "$(wc -c < "{marker}")" -le 1 ]; then exit 1; fi
    exit 0
    ;;
  lint)
    if [ "$(wc -c < "{marker}")" -gt 1 ]; then
      printf '%s' '{{"schema_version":1,"strict_green":true,"findings":[]}}'
    else
      printf '%s' '{{"schema_version":1,"strict_green":false,"findings":[{{"severity":"info","code":"CAIRN_ZEBRA","message":"z","node":null,"path":"z.md","deferred_by":null,"parked_by":"todo.parks-it"}},{{"severity":"error","code":"CAIRN_ALPHA","message":"a","node":"cairn.root","path":"a.md","deferred_by":"dec.defers-it","parked_by":null}}]}}'
    fi
    exit 0
    ;;
esac
exit 2
"#
    );
    std::fs::write(&script, body).expect("write stub");
    let mut perms = std::fs::metadata(&script).expect("stat stub").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script, perms).expect("chmod stub");
    script
}

#[test]
fn test_repair_feedback_is_the_previous_scan_verbatim_and_sorted() {
    let dir = tempfile::tempdir().expect("temp");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8");
    let fixture = root.join("fixture");
    std::fs::create_dir_all(&fixture).expect("fixture dir");
    std::fs::write(fixture.join("seed.txt"), "seed").expect("seed");

    let prompt = Prompt {
        schema_version: 1,
        id: "feedback".to_owned(),
        instruction: "author it".to_owned(),
        expects: vec!["out.md".to_owned()],
        replay: None,
    };
    let response = AuthorResponse {
        files: vec![FileEdit {
            path: "out.md".to_owned(),
            contents: "x".to_owned(),
        }],
        tokens: TokenUsage::default(),
    };
    let backend = RecordingBackend::new(vec![response.clone(), response]);

    let config = RunConfig {
        fixture,
        cairn_bin: stub_cairn(&root),
        backend: BackendSpec::Replay,
        max_repairs: 3,
        timeout_ms: 30_000,
    };

    let record = super::runner::run(&config, &prompt, &backend).expect("record");
    assert_eq!(record.outcome, super::record::Outcome::CleanAfterRepair);

    let requests = backend.requests.borrow();
    assert_eq!(requests.len(), 2, "one first shot and one repair");
    assert!(
        requests[0].is_empty(),
        "the first attempt has no preceding scan to feed back"
    );

    assert_repair_is_the_wire_verbatim(&requests[1]);
}

/// The repair feedback must be the previous scan, sorted, field for field.
fn assert_repair_is_the_wire_verbatim(repair: &[Finding]) {
    assert_eq!(repair.len(), 2, "every finding is fed back, none dropped");
    assert_eq!(
        repair[0].code, "CAIRN_ALPHA",
        "errors sort ahead of info, whatever order the wire used"
    );
    assert_eq!(repair[1].code, "CAIRN_ZEBRA");
    assert_eq!(
        repair[0].deferred_by.as_deref(),
        Some("dec.defers-it"),
        "`deferred_by` tells a model the finding is not its to fix; dropping it is not verbatim"
    );
    assert_eq!(repair[1].parked_by.as_deref(), Some("todo.parks-it"));
    assert_eq!(repair[0].node.as_deref(), Some("cairn.root"));
    assert_eq!(repair[0].path.as_deref(), Some("a.md"));
    assert_eq!(repair[0].message, "a", "the message is the finding");
    assert_eq!(repair[1].message, "z");
    assert!(
        repair[1].node.is_none() && repair[1].deferred_by.is_none(),
        "absent wire fields stay absent rather than being invented"
    );
}

#[test]
fn test_a_lint_envelope_without_findings_fails_closed() {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = tempfile::tempdir().expect("temp");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8");
    let fixture = root.join("fixture");
    std::fs::create_dir_all(&fixture).expect("fixture dir");

    let script = root.join("cairn-stub.sh");
    std::fs::write(
        &script,
        "#!/bin/sh\ncase \"$1\" in scan) exit 1 ;; lint) printf '%s' '{}' ; exit 0 ;; esac\nexit 2\n",
    )
    .expect("write stub");
    let mut perms = std::fs::metadata(&script).expect("stat").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script, perms).expect("chmod");

    let error = super::scorer::score(&script, &fixture)
        .expect_err("a wire with no findings key must fail closed");
    assert!(
        error.to_string().contains("no `findings` key"),
        "unexpected error: {error}"
    );
}

/// Builds an executable shell script and returns its path.
fn shell_script(dir: &Utf8Path, name: &str, body: &str) -> Utf8PathBuf {
    use std::os::unix::fs::PermissionsExt as _;

    let path = dir.join(name);
    std::fs::write(&path, body).expect("write script");
    let mut perms = std::fs::metadata(&path).expect("stat").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&path, perms).expect("chmod");
    path
}

fn command_request(findings: &[Finding]) -> AuthorRequest<'_> {
    AuthorRequest {
        schema_version: 1,
        prompt_id: "p",
        attempt: 1,
        instruction: "author it",
        findings,
    }
}

#[test]
fn test_command_backend_reads_a_response_from_a_backend_that_never_reads_stdin() {
    let dir = tempfile::tempdir().expect("temp");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8");
    // Emits its answer and exits without ever draining stdin. The broken pipe
    // that causes is normal, and the answer must still be read.
    let script = shell_script(
        &root,
        "backend.sh",
        "#!/bin/sh\nprintf '%s' '{\"files\":[{\"path\":\"out.md\",\"contents\":\"x\"}],\"tokens\":{\"prompt\":1,\"completion\":2}}'\nexit 0\n",
    );

    let backend = super::backend::CommandBackend::new(script.into_string(), Vec::new(), "m".into());
    let response = backend
        .invoke(&command_request(&[]), std::time::Duration::from_secs(30))
        .expect("a backend that ignores stdin still answers");

    assert_eq!(response.files.len(), 1);
    assert_eq!(response.files[0].path, "out.md");
    assert_eq!(response.tokens.completion, 2);
}

#[test]
fn test_command_backend_times_out_a_backend_that_never_answers() {
    let dir = tempfile::tempdir().expect("temp");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8");
    // `exec` so the sleeper IS the direct child: a forked descendant is out of
    // contract, and leaving one behind would outlive the test.
    let script = shell_script(&root, "hang.sh", "#!/bin/sh\nexec sleep 30\n");

    let backend = super::backend::CommandBackend::new(script.into_string(), Vec::new(), "m".into());
    let started = std::time::Instant::now();
    let error = backend
        .invoke(&command_request(&[]), std::time::Duration::from_millis(200))
        .expect_err("a backend that never answers must time out");

    assert_eq!(error.class(), super::backend::BackendErrorClass::Timeout);
    assert!(
        started.elapsed() < std::time::Duration::from_secs(10),
        "the deadline must actually cut the call short, took {:?}",
        started.elapsed()
    );
}

#[test]
fn test_command_backend_reports_a_non_zero_exit_with_its_stderr() {
    let dir = tempfile::tempdir().expect("temp");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8");
    let script = shell_script(
        &root,
        "fail.sh",
        "#!/bin/sh\necho 'went wrong' >&2\nexit 3\n",
    );

    let backend = super::backend::CommandBackend::new(script.into_string(), Vec::new(), "m".into());
    let error = backend
        .invoke(&command_request(&[]), std::time::Duration::from_secs(30))
        .expect_err("a failing backend is an invocation failure");

    assert_eq!(error.class(), super::backend::BackendErrorClass::Invocation);
    assert!(error.to_string().contains("went wrong"), "{error}");
}

#[test]
fn test_command_backend_classifies_an_unparseable_answer_as_protocol() {
    let dir = tempfile::tempdir().expect("temp");
    let root = Utf8PathBuf::from_path_buf(dir.path().to_path_buf()).expect("utf-8");
    let script = shell_script(&root, "junk.sh", "#!/bin/sh\nprintf 'not json'\nexit 0\n");

    let backend = super::backend::CommandBackend::new(script.into_string(), Vec::new(), "m".into());
    let error = backend
        .invoke(&command_request(&[]), std::time::Duration::from_secs(30))
        .expect_err("a non-JSON answer is a protocol failure");

    assert_eq!(error.class(), super::backend::BackendErrorClass::Protocol);
}
