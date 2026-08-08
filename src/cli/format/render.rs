//! Human-readable rendering and status display helpers for CLI output.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::json::node_json;
use super::util::esc;
use crate::query_api::{relative_path, todo_status};

pub(crate) fn render_node(node: &NodeRecord, json: bool) -> String {
    if json {
        format!("{}\n", node_json(node))
    } else {
        format!(
            "ID: {}\nName: {}\nDescription: {}\nState: {:?}\n",
            node.id, node.name, node.description, node.state
        )
    }
}

pub(crate) fn render_findings(findings: &[Finding], json: bool, verbose: bool) -> String {
    if json {
        let mut out = String::from("{\"findings\":[");
        for (i, finding) in findings.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str("{\"code\":\"");
            out.push_str(&esc(&finding.code));
            out.push_str("\",\"severity\":\"");
            out.push_str(finding.severity.name());
            out.push_str("\",\"message\":\"");
            out.push_str(&esc(&finding.message));
            out.push('"');
            if let Some(deferred_by) = &finding.deferred_by {
                out.push_str(",\"deferred_by\":\"");
                out.push_str(&esc(deferred_by));
                out.push('"');
            }
            if let Some(parked_by) = &finding.parked_by {
                out.push_str(",\"parked_by\":\"");
                out.push_str(&esc(parked_by));
                out.push('"');
            }
            out.push('}');
        }
        out.push_str("],\"strict_green\":");
        out.push_str(if crate::map::graph::strict_green(findings) {
            "true"
        } else {
            "false"
        });
        out.push_str("}\n");
        out
    } else if findings.is_empty() {
        format!(
            "Findings:\n{}\n",
            crate::copy::lookup("empty-states.cli-clean-map.body")
        )
    } else {
        let mut out = String::from("Findings:\n");
        out.push_str(&render_finding_lines(findings, verbose));
        out
    }
}

/// Renders finding lines (without the `Findings:` header). When `verbose` is
/// false, findings deferred by a decision collapse into a single summary line
/// per decision; non-deferred findings always render in full. Verbose mode
/// renders every finding in full regardless of deferral. A parked finding
/// never collapses: it renders in full with a suffix naming its parking todo,
/// so the count a human sees does not change
/// (`todo.lint-selection-folding` item 1a).
pub(crate) fn render_finding_lines(findings: &[Finding], verbose: bool) -> String {
    if verbose {
        let mut out = String::new();
        for finding in findings {
            let _ = writeln!(out, "{}", full_finding_line(finding));
        }
        return out;
    }
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for finding in findings {
        if let Some(dec) = &finding.deferred_by {
            *counts.entry(dec.clone()).or_insert(0) += 1;
        }
    }
    let mut out = String::new();
    let mut emitted: std::collections::HashSet<String> = std::collections::HashSet::new();
    for finding in findings {
        match &finding.deferred_by {
            Some(dec) => {
                if emitted.insert(dec.clone()) {
                    let count = *counts.get(dec).unwrap_or(&0);
                    let noun = if count == 1 { "finding" } else { "findings" };
                    let line = crate::copy::lookup("findings.deferred-collapsed")
                        .replace("{count}", &count.to_string())
                        .replace("{noun}", noun)
                        .replace("{decision}", dec);
                    let _ = writeln!(out, "{line}");
                }
            }
            None => {
                let _ = writeln!(out, "{}", full_finding_line(finding));
            }
        }
    }
    out
}

/// One full finding line, annotated from `parked_by` when the finding is
/// parked. The annotation is render-level: the message itself stays as the
/// emitting check wrote it.
fn full_finding_line(finding: &Finding) -> String {
    let mut line = format!(
        "{:?}: {} {}",
        finding.severity, finding.code, finding.message
    );
    if let Some(todo) = &finding.parked_by {
        line.push_str(&crate::copy::lookup("findings.parked-suffix").replace("{todo}", todo));
    }
    line
}

pub(crate) fn todo_line(todo: &Todo, root: &Path) -> String {
    format!(
        "{} [{}] {}",
        todo.node,
        todo_status(todo.status),
        relative_path(&todo.path, root)
    )
}

pub(crate) fn research_line(research: &Research) -> String {
    format!("{} sources: {}", research.id, research.sources.join(", "))
}

pub(crate) fn review_line(review: &Review) -> String {
    format!(
        "{} [{}] {}",
        review.node,
        review_type(review.review_type),
        review.path
    )
}

pub(crate) const fn review_type(review_type: ReviewType) -> &'static str {
    match review_type {
        ReviewType::Human => "human",
        ReviewType::AgentIntrospective => "agent_introspective",
        ReviewType::AgentCrossModel => "agent_cross_model",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        artefacts::registry::{ResearchMethod, TodoStatus},
        blueprint::{NodeKind, Span},
        map::{FindingSeverity, NodeRecord, NodeState},
    };

    #[test]
    fn test_review_type_display_strings() {
        assert_eq!(review_type(ReviewType::Human), "human");
        assert_eq!(
            review_type(ReviewType::AgentIntrospective),
            "agent_introspective"
        );
        assert_eq!(
            review_type(ReviewType::AgentCrossModel),
            "agent_cross_model"
        );
    }

    // ── line-formatting helpers ──────────────────────────────────────────────

    fn todo(status: TodoStatus) -> Todo {
        Todo {
            path: "./todo.md".to_owned(),
            node: "app".to_owned(),
            status,
            created: "2026-01-01".to_owned(),
            satisfies: None,
            blocked_by: Vec::new(),
            parent: None,
            related: Vec::new(),
            defers: Vec::new(),
            body: String::new(),
        }
    }

    #[test]
    fn test_todo_line_format() {
        assert_eq!(
            todo_line(&todo(TodoStatus::Open), Path::new(".")),
            "app [open] todo.md"
        );
        assert_eq!(
            todo_line(&todo(TodoStatus::InProgress), Path::new(".")),
            "app [in_progress] todo.md"
        );
    }

    #[test]
    fn test_research_line_format() {
        let research = Research {
            path: "./research.md".to_owned(),
            id: "r-1".to_owned(),
            nodes: Vec::new(),
            date: String::new(),
            sources: vec!["src-1".to_owned(), "src-2".to_owned()],
            method: ResearchMethod::Secondary,
            tags: Vec::new(),
            body: String::new(),
        };
        assert_eq!(research_line(&research), "r-1 sources: src-1, src-2");
    }

    #[test]
    fn test_review_line_format() {
        let review = Review {
            path: "./review.md".to_owned(),
            node: "app".to_owned(),
            review_type: ReviewType::Human,
            date: String::new(),
            reviewer: String::new(),
            subject_hash: None,
            lens_prompt_hash: None,
            related_change: None,
            body: String::new(),
        };
        assert_eq!(review_line(&review), "app [human] ./review.md");
    }

    // ── render_node / render_findings ─────────────────────────────────────────

    fn sample_node() -> NodeRecord {
        NodeRecord {
            kind: NodeKind::Module,
            id: "app".to_owned(),
            name: "app".to_owned(),
            description: "The app".to_owned(),
            tags: Vec::new(),
            parent: None,
            children: Vec::new(),
            paths: Vec::new(),
            owns_files: false,
            symbols: Vec::new(),
            contracts: Vec::new(),
            state: NodeState::Synced,
            files: Vec::new(),
            span: Span::point("test", 1, 1),
        }
    }

    #[test]
    fn test_render_node_human_format() {
        let rendered = render_node(&sample_node(), false);
        assert!(rendered.contains("ID: app"));
        assert!(rendered.contains("Name: app"));
        assert!(rendered.contains("Description: The app"));
        assert!(rendered.contains("State: Synced"));
    }

    #[test]
    fn test_render_node_json_format() {
        let rendered = render_node(&sample_node(), true);
        assert!(rendered.contains("\"id\":\"app\""));
        assert!(rendered.contains("\"name\":\"app\""));
    }

    #[test]
    fn test_render_findings_empty_json() {
        assert_eq!(
            render_findings(&[], true, false),
            "{\"findings\":[],\"strict_green\":true}\n"
        );
    }

    #[test]
    fn test_render_findings_non_empty_json() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Error,
            message: "bad".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        };
        let rendered = render_findings(&[finding], true, false);
        assert!(rendered.contains("\"code\":\"CAIRN_TEST\""));
        assert!(rendered.contains("\"severity\":\"error\""));
        assert!(rendered.contains("\"message\":\"bad\""));
    }

    #[test]
    fn test_render_findings_json_deferred_by_present_only_when_set() {
        let deferred = Finding {
            code: "CAIRN_SPEC_RULE_UNIMPLEMENTED".to_owned(),
            severity: FindingSeverity::Info,
            message: "pending (deferred by dec.x)".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: Some("dec.x".to_owned()),
            parked_by: None,
        };
        let live = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Warning,
            message: "live".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        };
        let rendered = render_findings(&[deferred, live], true, false);
        assert!(rendered.contains("\"deferred_by\":\"dec.x\""));
        assert!(
            !rendered.contains("\"message\":\"live\",\"deferred_by\""),
            "a finding without a deferral must omit the field: {rendered}"
        );
    }

    #[test]
    fn test_render_findings_json_strict_green_true_on_info_only() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Info,
            message: "advisory".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        };
        let rendered = render_findings(&[finding], true, false);
        assert!(
            rendered.contains("\"strict_green\":true"),
            "info-only sets are strict-green: {rendered}"
        );
    }

    #[test]
    fn test_render_findings_json_strict_green_false_on_warning() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Warning,
            message: "look out".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        };
        let rendered = render_findings(&[finding], true, false);
        assert!(
            rendered.contains("\"strict_green\":false"),
            "a warning must publish strict_green false: {rendered}"
        );
    }

    #[test]
    fn test_render_findings_non_empty_human() {
        let finding = Finding {
            code: "CAIRN_TEST".to_owned(),
            severity: FindingSeverity::Warning,
            message: "look out".to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: None,
        };
        let rendered = render_findings(&[finding], false, false);
        assert!(rendered.starts_with("Findings:\n"));
        assert!(rendered.contains("Warning: CAIRN_TEST look out"));
    }

    fn parked_info(message: &str, todo: Option<&str>) -> Finding {
        Finding {
            code: "CAIRN_SOURCE_UNVERIFIED".to_owned(),
            severity: FindingSeverity::Info,
            message: message.to_owned(),
            node: None,
            target: None,
            path: None,
            deferred_by: None,
            parked_by: todo.map(str::to_owned),
        }
    }

    #[test]
    fn test_render_findings_json_parked_by_present_only_when_set() {
        let parked = parked_info("parked one", Some("todo.park-sources"));
        let live = parked_info("live one", None);
        let rendered = render_findings(&[parked, live], true, false);
        assert!(rendered.contains("\"parked_by\":\"todo.park-sources\""));
        assert!(
            !rendered.contains("\"message\":\"live one\",\"parked_by\""),
            "a finding without a parking todo must omit the field: {rendered}"
        );
    }

    #[test]
    fn test_render_finding_lines_parked_full_line_never_collapses() {
        let findings = [
            parked_info("first", Some("todo.park-sources")),
            parked_info("second", Some("todo.park-sources")),
        ];
        let rendered = render_finding_lines(&findings, false);
        let lines: Vec<&str> = rendered.lines().collect();
        assert_eq!(
            lines.len(),
            2,
            "the count a human sees must not change: {rendered}"
        );
        for (line, message) in lines.iter().zip(["first", "second"]) {
            assert!(
                line.contains(message) && line.ends_with("(parked by todo.park-sources)"),
                "each parked finding renders in full naming its todo: {line}"
            );
        }
    }

    #[test]
    fn test_render_finding_lines_verbose_annotates_parked() {
        let rendered =
            render_finding_lines(&[parked_info("first", Some("todo.park-sources"))], true);
        assert!(
            rendered
                .trim_end()
                .ends_with("(parked by todo.park-sources)"),
            "verbose mode keeps the parked annotation: {rendered}"
        );
    }
}
