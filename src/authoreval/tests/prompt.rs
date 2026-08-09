//! Prompt loading and validation.

use camino::Utf8PathBuf;

use super::super::prompt::Prompt;

#[test]
fn test_prompt_load_rejects_a_wrong_schema_version() {
    let dir = tempfile::tempdir().expect("temp");
    let path = Utf8PathBuf::from_path_buf(dir.path().join("p.json")).expect("utf-8");
    std::fs::write(
        &path,
        r#"{"schema_version": 2, "id": "x", "instruction": "y", "expects": ["a.md"]}"#,
    )
    .expect("write");

    let error = Prompt::load(&path).expect_err("wrong version must be rejected");
    assert!(error.to_string().contains("schema_version 2"));
}

#[test]
fn test_prompt_load_rejects_empty_identity_and_instruction() {
    let dir = tempfile::tempdir().expect("temp");

    let empty_id = Utf8PathBuf::from_path_buf(dir.path().join("id.json")).expect("utf-8");
    std::fs::write(
        &empty_id,
        r#"{"schema_version": 1, "id": "", "instruction": "y", "expects": ["a.md"]}"#,
    )
    .expect("write");
    assert!(
        Prompt::load(&empty_id)
            .expect_err("empty id")
            .to_string()
            .contains("empty id")
    );

    let empty_instruction =
        Utf8PathBuf::from_path_buf(dir.path().join("instruction.json")).expect("utf-8");
    std::fs::write(
        &empty_instruction,
        r#"{"schema_version": 1, "id": "x", "instruction": "", "expects": ["a.md"]}"#,
    )
    .expect("write");
    assert!(
        Prompt::load(&empty_instruction)
            .expect_err("empty instruction")
            .to_string()
            .contains("empty instruction")
    );
}

#[test]
fn test_prompt_load_rejects_a_replay_script_with_no_model() {
    let dir = tempfile::tempdir().expect("temp");
    let path = Utf8PathBuf::from_path_buf(dir.path().join("p.json")).expect("utf-8");
    std::fs::write(
        &path,
        r#"{"schema_version": 1, "id": "x", "instruction": "y", "expects": ["a.md"],
            "replay": {"model": "", "turns": []}}"#,
    )
    .expect("write");

    assert!(
        Prompt::load(&path)
            .expect_err("an unattributable replay must be refused")
            .to_string()
            .contains("empty model")
    );
}
