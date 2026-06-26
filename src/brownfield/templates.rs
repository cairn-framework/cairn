//! Templated authoring for brownfield stub contracts.
//!
//! Organisations declare contract templates that the brownfield generator
//! consumes when drafting stubs.  Templates are matched against candidates
//! by path or tag; the first matching template wins.  If no template
//! matches, the generator falls back to the built-in stub.

use std::path::Path;

use serde::Deserialize;

use super::discovery::DiscoveredCandidate;

/// One match rule for a contract template.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum MatchRule {
    /// Matches when the candidate's path contains the given substring.
    Path(String),
    /// Matches when the candidate carries the given tag.
    HasTag(String),
}

/// A contract template declaration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ContractTemplate {
    /// Unique template name (for diagnostics).
    pub name: String,
    /// Match rules evaluated in order; the template matches if ANY rule
    /// succeeds (OR semantics).
    pub match_rules: Vec<MatchRule>,
    /// Body template with `{id}`, `{name}`, and `{description}` placeholders.
    pub body: String,
}

/// Render a stub contract for a candidate, using the first matching template
/// or falling back to the built-in stub.
///
/// Placeholders in the template body are replaced with candidate values:
/// - `{id}`    → candidate.id
/// - `{name}`  → candidate.name
/// - `{description}` → candidate.description
///
/// # Panics
///
/// Never panics.
#[must_use]
pub fn render_stub(candidate: &DiscoveredCandidate, templates: &[ContractTemplate]) -> String {
    let rendered = find_matching_template(candidate, templates).map_or_else(
        || super::stub_contract(candidate),
        |template| apply_template(candidate, &template.body),
    );
    ensure_operation_added(rendered)
}

/// Guarantee the rendered contract carries `operation` frontmatter so it passes
/// change-archive validation (`parse_artefact_operations`). Idempotent: a body
/// that already declares an `operation` is returned unchanged; a body whose
/// frontmatter omits it gains `operation: added`; a body with no frontmatter
/// gains a minimal block.
fn ensure_operation_added(body: String) -> String {
    if let Some(opener) = ["---\r\n", "---\n"]
        .into_iter()
        .find(|prefix| body.starts_with(prefix))
    {
        let rest = &body[opener.len()..];
        if let Some(end) = rest.find("\n---") {
            let front = &rest[..end];
            if front
                .lines()
                .any(|line| line.trim_start().starts_with("operation:"))
            {
                return body;
            }
            let newline = &opener[3..];
            return format!("{opener}operation: added{newline}{rest}");
        }
    }
    format!("---\noperation: added\n---\n{body}")
}

/// Find the first template whose match rules succeed for the candidate.
fn find_matching_template<'a>(
    candidate: &DiscoveredCandidate,
    templates: &'a [ContractTemplate],
) -> Option<&'a ContractTemplate> {
    templates.iter().find(|t| template_matches(candidate, t))
}

/// Check whether a candidate matches a template's rules.
///
/// A template matches when ANY of its rules succeed (OR semantics).
fn template_matches(candidate: &DiscoveredCandidate, template: &ContractTemplate) -> bool {
    template.match_rules.iter().any(|rule| match rule {
        MatchRule::Path(sub) => candidate.path.contains(sub),
        MatchRule::HasTag(tag) => candidate.tags.contains(tag),
    })
}

/// Replace placeholders in a template body with candidate values.
fn apply_template(candidate: &DiscoveredCandidate, body: &str) -> String {
    body.replace("{id}", &candidate.id)
        .replace("{name}", &candidate.name)
        .replace("{description}", &candidate.description)
}

/// Load contract templates from `meta/templates.toml` in the project root.
///
/// Returns an empty vector when the file does not exist. Returns an error
/// only when the file is present but malformed.
#[must_use]
pub fn load_templates(root: &Path) -> Vec<ContractTemplate> {
    let path = root.join("meta/templates.toml");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(source) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    match toml::from_str::<TomlRoot>(&source) {
        Ok(root) => root.template,
        Err(_) => Vec::new(),
    }
}

#[derive(Deserialize)]
struct TomlRoot {
    template: Vec<ContractTemplate>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate_with_tags(tags: &[&str]) -> DiscoveredCandidate {
        DiscoveredCandidate {
            id: "src.test".to_owned(),
            name: "test".to_owned(),
            description: "Test module".to_owned(),
            path: "src/test".to_owned(),
            tags: tags.iter().map(std::string::ToString::to_string).collect(),
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        }
    }

    #[test]
    fn test_find_matching_template_by_tag() {
        let tmpl = ContractTemplate {
            name: "t1".to_owned(),
            match_rules: vec![MatchRule::HasTag("security".to_owned())],
            body: "# {name}".to_owned(),
        };
        let candidate = candidate_with_tags(&["security"]);
        let templates = [tmpl];
        let found = find_matching_template(&candidate, &templates);
        assert!(found.is_some());
    }

    #[test]
    fn test_find_matching_template_by_path() {
        let tmpl = ContractTemplate {
            name: "t1".to_owned(),
            match_rules: vec![MatchRule::Path("auth".to_owned())],
            body: "# {name}".to_owned(),
        };
        let mut candidate = candidate_with_tags(&[]);
        candidate.path = "src/auth".to_owned();
        let templates = [tmpl];
        let found = find_matching_template(&candidate, &templates);
        assert!(found.is_some());
    }

    #[test]
    fn test_no_match_returns_none() {
        let tmpl = ContractTemplate {
            name: "t1".to_owned(),
            match_rules: vec![MatchRule::HasTag("security".to_owned())],
            body: "# {name}".to_owned(),
        };
        let candidate = candidate_with_tags(&["util"]);
        let templates = [tmpl];
        let found = find_matching_template(&candidate, &templates);
        assert!(found.is_none());
    }

    #[test]
    fn test_apply_template_replaces_placeholders() {
        let candidate = DiscoveredCandidate {
            id: "app.core".to_owned(),
            name: "core".to_owned(),
            description: "Core module".to_owned(),
            path: "src/core".to_owned(),
            tags: vec![],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let body = "# {name}\n\n{id}: {description}";
        let rendered = apply_template(&candidate, body);
        assert_eq!(rendered, "# core\n\napp.core: Core module");
    }

    #[test]
    fn test_render_stub_fallback_when_no_templates() {
        let candidate = candidate_with_tags(&[]);
        let rendered = render_stub(&candidate, &[]);
        assert!(rendered.contains("src.test"));
    }

    #[test]
    fn test_render_stub_injects_operation_for_template_without_frontmatter() {
        // A matched template body with no frontmatter would otherwise fail
        // change-archive validation, which requires `operation` on every artefact.
        let tmpl = ContractTemplate {
            name: "bare".to_owned(),
            match_rules: vec![MatchRule::HasTag("api".to_owned())],
            body: "# {name}\n\nNo frontmatter here.".to_owned(),
        };
        let candidate = candidate_with_tags(&["api"]);
        let rendered = render_stub(&candidate, &[tmpl]);
        assert!(
            rendered.starts_with("---\noperation: added\n---\n"),
            "operation frontmatter must be prepended: {rendered:?}"
        );
    }

    #[test]
    fn test_render_stub_injects_operation_into_existing_frontmatter() {
        let tmpl = ContractTemplate {
            name: "fm".to_owned(),
            match_rules: vec![MatchRule::HasTag("api".to_owned())],
            body: "---\nnode: {id}\n---\n# {name}".to_owned(),
        };
        let candidate = candidate_with_tags(&["api"]);
        let rendered = render_stub(&candidate, &[tmpl]);
        assert!(
            rendered.contains("operation: added"),
            "operation must be injected into existing frontmatter: {rendered:?}"
        );
        assert!(
            rendered.contains("node: src.test"),
            "existing frontmatter keys must be preserved: {rendered:?}"
        );
    }

    #[test]
    fn test_render_stub_idempotent_when_operation_present() {
        // The built-in stub already declares `operation: added`; rendering must
        // not duplicate it.
        let candidate = candidate_with_tags(&[]);
        let rendered = render_stub(&candidate, &[]);
        assert_eq!(
            rendered.matches("operation: added").count(),
            1,
            "operation must not be duplicated: {rendered:?}"
        );
    }

    #[test]
    fn test_render_stub_injects_operation_into_crlf_frontmatter() {
        // A template saved with CRLF line endings must still get operation
        // injected into its existing frontmatter, not a duplicate block prepended.
        let tmpl = ContractTemplate {
            name: "crlf".to_owned(),
            match_rules: vec![MatchRule::HasTag("api".to_owned())],
            body: "---\r\nnode: {id}\r\n---\r\n# {name}".to_owned(),
        };
        let candidate = candidate_with_tags(&["api"]);
        let rendered = render_stub(&candidate, &[tmpl]);
        assert_eq!(
            rendered.matches("operation: added").count(),
            1,
            "operation must be injected exactly once: {rendered:?}"
        );
        assert!(
            rendered.contains("node: src.test"),
            "existing frontmatter key must be preserved: {rendered:?}"
        );
    }

    #[test]
    fn test_first_matching_template_wins() {
        let t1 = ContractTemplate {
            name: "first".to_owned(),
            match_rules: vec![MatchRule::HasTag("api".to_owned())],
            body: "# First".to_owned(),
        };
        let t2 = ContractTemplate {
            name: "second".to_owned(),
            match_rules: vec![MatchRule::HasTag("api".to_owned())],
            body: "# Second".to_owned(),
        };
        let candidate = candidate_with_tags(&["api"]);
        let rendered = render_stub(&candidate, &[t1, t2]);
        assert!(rendered.contains("# First"));
        assert!(!rendered.contains("# Second"));
    }
}
