//! User-facing rendering for [`BaselineError`].
//!
//! The error variants carry data, never prose. This is the single place that
//! turns one into text, shared by the `baseline` command and by draft
//! acceptance, so neither hardcodes a sentence: every string resolves from
//! `docs/design-system/copy.toml`.

use super::baseline::BaselineError;
use crate::copy;

/// Renders a [`BaselineError`] from `docs/design-system/copy.toml`.
#[must_use]
pub(crate) fn describe(error: &BaselineError) -> String {
    match error {
        BaselineError::BlueprintUnreadable { path, error } => {
            copy::lookup("baseline.blueprint-unreadable")
                .replace("{path}", path)
                .replace("{error}", error)
        }
        BaselineError::StateReadFailed(error) => {
            copy::lookup("baseline.state-read-failed").replace("{error}", error)
        }
        BaselineError::StateWriteFailed(error) => {
            copy::lookup("baseline.state-write-failed").replace("{error}", error)
        }
        BaselineError::NodeNotDeclared(node) => {
            copy::lookup("baseline.node-not-declared").replace("{node}", node)
        }
        BaselineError::NoContract(node) => {
            copy::lookup("baseline.no-contract").replace("{node}", node)
        }
        BaselineError::ContractUnreadable { node, path, error } => {
            copy::lookup("baseline.contract-unreadable")
                .replace("{node}", node)
                .replace("{path}", path)
                .replace("{error}", error)
        }
        BaselineError::NotRecorded(node) => {
            copy::lookup("baseline.not-recorded").replace("{node}", node)
        }
        BaselineError::StillLive(node) => {
            copy::lookup("baseline.still-live").replace("{node}", node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BaselineError, describe};

    #[test]
    fn every_variant_resolves_to_copy_with_no_unsubstituted_slots() {
        let rendered = [
            describe(&BaselineError::BlueprintUnreadable {
                path: "cairn.blueprint".to_owned(),
                error: "boom".to_owned(),
            }),
            describe(&BaselineError::StateReadFailed("boom".to_owned())),
            describe(&BaselineError::StateWriteFailed("boom".to_owned())),
            describe(&BaselineError::NodeNotDeclared("app.api".to_owned())),
            describe(&BaselineError::NoContract("app.api".to_owned())),
            describe(&BaselineError::ContractUnreadable {
                node: "app.api".to_owned(),
                path: "meta/contracts/api.md".to_owned(),
                error: "boom".to_owned(),
            }),
            describe(&BaselineError::NotRecorded("app.api".to_owned())),
            describe(&BaselineError::StillLive("app.api".to_owned())),
        ];
        for message in rendered {
            assert!(!message.is_empty());
            assert!(!message.contains('{'), "unsubstituted slot: {message}");
            assert!(
                !message.starts_with("baseline."),
                "unresolved copy key: {message}"
            );
        }
    }
}
