//! Prompt files.
//!
//! A prompt is the authoring instruction, the paths the answer must author,
//! and optionally the replay script that lets the offline backend serve it
//! deterministically. A prompt with no replay script is still valid: it simply
//! cannot be run offline.

use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use super::backend::ReplayScript;
use super::workspace::canonical_relative;
use crate::error::CairnError;

/// Wire schema version for prompt files.
pub(crate) const PROMPT_SCHEMA_VERSION: u32 = 1;

/// One authoring prompt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Prompt {
    /// Wire schema version.
    pub(crate) schema_version: u32,
    /// Stable prompt identifier, carried into the record.
    pub(crate) id: String,
    /// The authoring instruction sent to the model.
    pub(crate) instruction: String,
    /// Workspace-relative paths the answer must author.
    ///
    /// Required and non-empty. The fixture already scans clean, so without a
    /// declared target an answer that writes nothing would score
    /// `clean_first_shot` and the run would report perfect authorability for
    /// no authoring at all.
    pub(crate) expects: Vec<String>,
    /// Turns the offline backend serves for this prompt.
    #[serde(default)]
    pub(crate) replay: Option<ReplayScript>,
}

impl Prompt {
    /// Loads and validates one prompt file.
    pub(crate) fn load(path: &Utf8Path) -> Result<Self, CairnError> {
        let raw = std::fs::read_to_string(path).map_err(|e| CairnError::AuthorEval {
            message: format!("failed to read prompt `{path}`: {e}"),
        })?;

        let mut prompt: Self = serde_json::from_str(&raw).map_err(|e| CairnError::AuthorEval {
            message: format!("prompt `{path}` is not a valid prompt file: {e}"),
        })?;

        prompt
            .check_fields()
            .map_err(|reason| CairnError::AuthorEval {
                message: format!("prompt `{path}` {reason}"),
            })?;
        prompt.expects = prompt
            .canonical_expects()
            .map_err(|reason| CairnError::AuthorEval {
                message: format!("prompt `{path}` {reason}"),
            })?;
        prompt
            .check_replay()
            .map_err(|reason| CairnError::AuthorEval {
                message: format!("prompt `{path}` {reason}"),
            })?;
        Ok(prompt)
    }

    /// Checks the fields every prompt must carry.
    fn check_fields(&self) -> Result<(), String> {
        if self.schema_version != PROMPT_SCHEMA_VERSION {
            return Err(format!(
                "declares schema_version {}, expected {PROMPT_SCHEMA_VERSION}",
                self.schema_version
            ));
        }
        if self.id.is_empty() {
            return Err("declares an empty id".to_owned());
        }
        if self.instruction.is_empty() {
            return Err("declares an empty instruction".to_owned());
        }
        if self.expects.is_empty() {
            return Err(
                "declares no `expects` paths; a prompt whose answer cannot be located cannot be scored"
                    .to_owned(),
            );
        }
        Ok(())
    }

    /// Checks the replay script can attribute the records it produces.
    ///
    /// Runs after the expected paths are canonicalised, so a prompt that is
    /// malformed in both ways still reports the path problem first.
    fn check_replay(&self) -> Result<(), String> {
        // A record must never be readable without knowing what produced it.
        if self
            .replay
            .as_ref()
            .is_some_and(|replay| replay.model.is_empty())
        {
            return Err("declares a replay script with an empty model".to_owned());
        }
        Ok(())
    }

    /// Canonicalises the expected paths through the workspace's own authority.
    ///
    /// A prompt asking for a path the workspace would refuse can never be
    /// satisfied, and that is our bug, not a backend's, so it fails at load.
    fn canonical_expects(&self) -> Result<Vec<String>, String> {
        let mut canonical = Vec::with_capacity(self.expects.len());
        for expected in &self.expects {
            let relative = canonical_relative(expected)
                .map_err(|reason| format!("expects `{expected}`, which {reason}"))?;
            if canonical.contains(&relative) {
                return Err(format!("expects `{expected}` more than once"));
            }
            canonical.push(relative);
        }
        Ok(canonical)
    }

    /// The declared paths a response failed to author, in declaration order.
    ///
    /// Both sides are already canonical: `expects` was canonicalised at load,
    /// and the runner canonicalises the response's paths through the same
    /// authority before calling this. Comparison is therefore exact.
    pub(crate) fn unmet(&self, written: &[String]) -> Vec<&str> {
        self.expects
            .iter()
            .filter(|expected| !written.contains(expected))
            .map(String::as_str)
            .collect()
    }
}
