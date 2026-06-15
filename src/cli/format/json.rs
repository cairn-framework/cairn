//! JSON serialization helpers for CLI output.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::render::{decision_status, review_type, source_verification, todo_status};
use super::util::esc;

pub(crate) fn node_json(node: &NodeRecord) -> String {
    format!(
        "{{\"id\":\"{}\",\"name\":\"{}\",\"description\":\"{}\",\"state\":\"{:?}\",\"children\":{},\"files\":{}}}",
        esc(&node.id),
        esc(&node.name),
        esc(&node.description),
        node.state,
        string_array_json(&node.children),
        string_array_json(&node.files)
    )
}

pub(crate) fn finding_json(finding: &Finding) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{}\",\"message\":\"{}\"}}",
        esc(&finding.code),
        finding.severity.name(),
        esc(&finding.message)
    )
}

pub(crate) fn todos_json(todos: &[Todo]) -> String {
    format!(
        "[{}]",
        todos
            .iter()
            .map(|todo| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"status\":\"{}\",\"created\":\"{}\",\"satisfies\":\"{}\"}}",
                    esc(&todo.path),
                    esc(&todo.node),
                    todo_status(todo.status),
                    esc(&todo.created),
                    esc(todo.satisfies.as_deref().unwrap_or(""))
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn decisions_json(decisions: &[Decision]) -> String {
    format!(
        "[{}]",
        decisions
            .iter()
            .map(|decision| {
                format!(
                    "{{\"id\":\"{}\",\"status\":\"{}\",\"nodes\":{},\"informed_by\":{},\"supersedes\":{},\"refines\":{},\"related\":{}}}",
                    esc(&decision.id),
                    decision_status(decision.status),
                    string_array_json(&decision.nodes),
                    string_array_json(&decision.informed_by),
                    string_array_json(&decision.supersedes),
                    string_array_json(&decision.refines),
                    string_array_json(&decision.related)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn research_json(research: &[Research]) -> String {
    format!(
        "[{}]",
        research
            .iter()
            .map(|item| {
                format!(
                    "{{\"id\":\"{}\",\"nodes\":{},\"sources\":{},\"date\":\"{}\"}}",
                    esc(&item.id),
                    string_array_json(&item.nodes),
                    string_array_json(&item.sources),
                    esc(&item.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn reviews_json(reviews: &[Review]) -> String {
    format!(
        "[{}]",
        reviews
            .iter()
            .map(|review| {
                format!(
                    "{{\"path\":\"{}\",\"node\":\"{}\",\"review_type\":\"{}\",\"date\":\"{}\",\"reviewer\":\"{}\"}}",
                    esc(&review.path),
                    esc(&review.node),
                    review_type(review.review_type),
                    esc(&review.date),
                    esc(&review.reviewer)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn sources_json(sources: &[Source]) -> String {
    format!(
        "[{}]",
        sources
            .iter()
            .map(|source| {
                format!(
                    "{{\"id\":\"{}\",\"file\":\"{}\",\"verification\":\"{}\",\"type\":\"{}\",\"date\":\"{}\"}}",
                    esc(&source.id),
                    esc(&source.file),
                    source_verification(source.verification),
                    esc(&source.source_type),
                    esc(&source.date)
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    )
}

pub(crate) fn string_array_json(values: &[String]) -> String {
    let mut out = String::from('[');
    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&esc(value));
        out.push('"');
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── string_array_json ────────────────────────────────────────────────────

    #[test]
    fn test_string_array_json_empty() {
        assert_eq!(string_array_json(&[]), "[]");
    }

    #[test]
    fn test_string_array_json_single() {
        assert_eq!(string_array_json(&["a".to_owned()]), "[\"a\"]");
    }

    #[test]
    fn test_string_array_json_multiple() {
        assert_eq!(
            string_array_json(&["a".to_owned(), "b".to_owned()]),
            "[\"a\",\"b\"]"
        );
    }

    #[test]
    fn test_string_array_json_value_with_quote_is_escaped() {
        assert_eq!(string_array_json(&["a\"b".to_owned()]), "[\"a\\\"b\"]");
    }
}
