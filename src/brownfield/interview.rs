//! Interview runner for brownfield onboarding sessions.
//!
//! Manages multi-round elicitation by persisting session state inside the
//! change directory and writing the final transcript to `genesis.md`.
//! The session file is transient: it is removed when the session completes
//! or is abandoned.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::CairnError;

/// Wire schema version for the interview session file.
const SESSION_VERSION: u32 = 1;

/// Session file name, relative to the change directory's `research/` folder.
const SESSION_FILE: &str = "research/interview-session.json";

/// One Q/A turn in the interview.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Turn {
    /// Question text.
    pub question: String,
    /// Answer text, empty until recorded.
    #[serde(default)]
    pub answer: String,
}

/// In-progress interview session persisted to disk.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InterviewSession {
    /// Schema version.
    pub version: u32,
    /// Change ID this session belongs to.
    pub change_id: String,
    /// Ordered list of Q/A turns.
    pub turns: Vec<Turn>,
    /// Index of the next unanswered question.
    pub cursor: usize,
    /// True when all questions have been answered.
    #[serde(default)]
    pub complete: bool,
}

/// Start a new interview session and persist it to disk.
///
/// Creates the `research/` directory inside `change_dir` if needed.
/// Returns the initial session with `cursor` at `0`.
///
/// # Errors
///
/// Returns `CairnError::WriteOutput` when the session file cannot be written.
pub fn start_session(
    change_dir: &Path,
    change_id: &str,
    questions: &[String],
) -> Result<InterviewSession, CairnError> {
    let research_dir = change_dir.join("research");
    std::fs::create_dir_all(&research_dir).map_err(|e| CairnError::WriteOutput {
        path: research_dir.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })?;

    let turns = questions
        .iter()
        .map(|q| Turn {
            question: q.clone(),
            answer: String::new(),
        })
        .collect();

    let session = InterviewSession {
        version: SESSION_VERSION,
        change_id: change_id.to_owned(),
        turns,
        cursor: 0,
        complete: false,
    };

    write_session(change_dir, &session)?;
    Ok(session)
}

/// Resume an existing interview session from disk.
///
/// Returns `Ok(None)` when no session file exists.
///
/// # Errors
///
/// Returns `CairnError::ChangeDiscovery` when the file exists but cannot be
/// read or parsed.
pub fn resume_session(change_dir: &Path) -> Result<Option<InterviewSession>, CairnError> {
    let path = change_dir.join(SESSION_FILE);
    if !path.exists() {
        return Ok(None);
    }

    let text = std::fs::read_to_string(&path).map_err(|e| CairnError::ChangeDiscovery {
        path: path.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })?;

    let session: InterviewSession =
        serde_json::from_str(&text).map_err(|e| CairnError::ChangeDiscovery {
            path: path.to_string_lossy().into_owned(),
            detail: e.to_string(),
        })?;

    Ok(Some(session))
}

/// Record an answer for the current question and advance the cursor.
///
/// Writes the updated session back to disk.  If the cursor reaches the end
/// of the turn list, `complete` is set to `true`.
///
/// # Errors
///
/// Returns `CairnError::WriteOutput` when the session file cannot be written.
/// Returns `CairnError::ChangeDiscovery` when the session is already complete.
pub fn record_answer(
    change_dir: &Path,
    session: &InterviewSession,
    answer: &str,
) -> Result<InterviewSession, CairnError> {
    if session.complete {
        return Err(CairnError::ChangeDiscovery {
            path: change_dir.join(SESSION_FILE).to_string_lossy().into_owned(),
            detail: "session is already complete".to_owned(),
        });
    }

    let mut updated = session.clone();
    if updated.cursor < updated.turns.len() {
        answer.clone_into(&mut updated.turns[updated.cursor].answer);
        updated.cursor += 1;
    }
    if updated.cursor >= updated.turns.len() {
        updated.complete = true;
    }

    write_session(change_dir, &updated)?;
    Ok(updated)
}

/// Complete the session by writing the genesis transcript and removing the
/// session file.
///
/// # Errors
///
/// Returns `CairnError::WriteOutput` when `genesis.md` cannot be written.
pub fn complete_session(
    change_dir: &Path,
    session: &InterviewSession,
    _change_id: &str,
) -> Result<(), CairnError> {
    let research_dir = change_dir.join("research");
    std::fs::create_dir_all(&research_dir).map_err(|e| CairnError::WriteOutput {
        path: research_dir.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })?;

    let genesis_path = research_dir.join("genesis.md");
    let content = render_genesis(session);
    std::fs::write(&genesis_path, content).map_err(|e| CairnError::WriteOutput {
        path: genesis_path.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })?;

    let session_path = change_dir.join(SESSION_FILE);
    if session_path.exists() {
        let _ = std::fs::remove_file(&session_path);
    }

    Ok(())
}

/// Abandon an in-progress session by deleting the session file.
///
/// Does nothing when the session file does not exist.
pub fn abandon_session(change_dir: &Path) {
    let session_path = change_dir.join(SESSION_FILE);
    if session_path.exists() {
        let _ = std::fs::remove_file(&session_path);
    }
}

/// Render a genesis transcript from a completed session.
fn render_genesis(session: &InterviewSession) -> String {
    let mut lines = vec![
        "# Genesis Transcript\n".to_owned(),
        format!("change_id: {}\n", session.change_id),
    ];

    for (i, turn) in session.turns.iter().enumerate() {
        lines.push(format!("\n## Turn {}\n", i + 1));
        lines.push(format!("**Q:** {}\n", turn.question));
        if turn.answer.is_empty() {
            lines.push("**A:** (unanswered)\n".to_owned());
        } else {
            lines.push(format!("**A:** {}\n", turn.answer));
        }
    }

    if session.complete {
        lines.push("\n---\n\nSession complete.\n".to_owned());
    }

    lines.concat()
}

/// Write the session file atomically.
fn write_session(change_dir: &Path, session: &InterviewSession) -> Result<(), CairnError> {
    let path = change_dir.join(SESSION_FILE);
    let json = serde_json::to_string_pretty(session).map_err(|e| CairnError::WriteOutput {
        path: path.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })?;

    std::fs::write(&path, json).map_err(|e| CairnError::WriteOutput {
        path: path.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_change_dir(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(name);
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(change_dir.join("research")).unwrap();
        change_dir
    }

    #[test]
    fn test_start_session_creates_file() {
        let dir = temp_change_dir("bf-int-start");
        let session =
            start_session(&dir, "test-change", &["Q1".to_owned()]).expect("start should succeed");

        assert_eq!(session.version, 1);
        assert_eq!(session.cursor, 0);
        assert!(!session.complete);
        assert!(dir.join("research/interview-session.json").exists());
    }

    #[test]
    fn test_record_answer_advances_cursor() {
        let dir = temp_change_dir("bf-int-answer");
        let session = start_session(&dir, "test-change", &["Q1".to_owned(), "Q2".to_owned()])
            .expect("start should succeed");

        let updated = record_answer(&dir, &session, "A1").expect("record should succeed");

        assert_eq!(updated.cursor, 1);
        assert_eq!(updated.turns[0].answer, "A1");
        assert!(!updated.complete);
    }

    #[test]
    fn test_record_answer_completes_at_end() {
        let dir = temp_change_dir("bf-int-complete");
        let session =
            start_session(&dir, "test-change", &["Q1".to_owned()]).expect("start should succeed");

        let updated = record_answer(&dir, &session, "A1").expect("record should succeed");

        assert!(updated.complete);
    }

    #[test]
    fn test_complete_session_writes_genesis() {
        let dir = temp_change_dir("bf-int-genesis");
        let session = start_session(&dir, "test-change", &["What?".to_owned()])
            .expect("start should succeed");

        let answered = record_answer(&dir, &session, "Yes.").expect("record should succeed");

        complete_session(&dir, &answered, "test-change").expect("complete should succeed");

        assert!(dir.join("research/genesis.md").exists());
        assert!(!dir.join("research/interview-session.json").exists());
    }

    #[test]
    fn test_abandon_session_removes_file() {
        let dir = temp_change_dir("bf-int-abandon");
        start_session(&dir, "test-change", &["Q1".to_owned()]).expect("start should succeed");

        assert!(dir.join("research/interview-session.json").exists());
        abandon_session(&dir);
        assert!(!dir.join("research/interview-session.json").exists());
    }

    #[test]
    fn test_resume_session_reads_existing() {
        let dir = temp_change_dir("bf-int-resume");
        let session = start_session(&dir, "test-change", &["Q1".to_owned(), "Q2".to_owned()])
            .expect("start should succeed");

        let resumed = resume_session(&dir)
            .expect("resume should succeed")
            .expect("session should exist");

        assert_eq!(resumed.change_id, session.change_id);
        assert_eq!(resumed.turns.len(), 2);
    }

    #[test]
    fn test_resume_session_returns_none_when_missing() {
        let dir = temp_change_dir("bf-int-none");
        let result = resume_session(&dir).expect("resume should succeed");
        assert!(result.is_none());
    }
}
