//! Exact symbol-to-location query handler.
use crate::scanner;
use serde_json::{Value, json};

pub(crate) fn locate_json(scan_result: &scanner::ScanResult, symbol: &str) -> Value {
    let matches = locate_matches(
        scan_result
            .graph
            .nodes
            .values()
            .map(|node| (node.id.as_str(), node.symbols.as_slice())),
        symbol,
    );
    json!({ "symbol": symbol, "matches": matches })
}

fn locate_matches<'a, I>(nodes: I, symbol: &str) -> Vec<Value>
where
    I: IntoIterator<Item = (&'a str, &'a [crate::reconcile::SymbolRecord])>,
{
    matching_records(nodes, symbol)
        .iter()
        .map(|(node_id, record)| {
            json!({
                "node_id": node_id,
                "file": record.file,
                "line": record.line,
                "end_line": record.end_line,
                "kind": record.kind,
                "signature": record.signature,
            })
        })
        .collect()
}

fn matching_records<'a, I>(
    nodes: I,
    symbol: &str,
) -> Vec<(&'a str, &'a crate::reconcile::SymbolRecord)>
where
    I: IntoIterator<Item = (&'a str, &'a [crate::reconcile::SymbolRecord])>,
{
    let mut matches = Vec::new();
    for (node_id, symbols) in nodes {
        for record in symbols {
            if record.name == symbol {
                matches.push((node_id, record));
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::{locate_matches, matching_records};
    use crate::query_api::serialise::requires_valid_map;
    use crate::query_api::{QueryRequest, execute};
    use crate::reconcile::{SymbolKind, SymbolRecord};

    fn record(name: &str, file: &str) -> SymbolRecord {
        SymbolRecord {
            name: name.to_owned(),
            kind: SymbolKind::Function,
            signature: format!("fn {name}()"),
            file: file.to_owned(),
            line: 4,
            end_line: 6,
        }
    }

    #[test]
    fn exact_lookup_returns_all_collisions() {
        let one = vec![record("connect", "one.rs")];
        let two = vec![record("connect", "two.rs")];
        let matches = matching_records(
            [("node.one", one.as_slice()), ("node.two", two.as_slice())],
            "connect",
        );
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].0, "node.one");
        assert_eq!(matches[1].0, "node.two");
        let emitted = locate_matches(
            [("node.one", one.as_slice()), ("node.two", two.as_slice())],
            "connect",
        );
        assert_eq!(emitted[0]["node_id"], "node.one");
        assert_eq!(emitted[0]["file"], "one.rs");
        assert_eq!(emitted[0]["line"], 4);
        assert_eq!(emitted[0]["end_line"], 6);
    }

    #[test]
    fn unknown_name_returns_empty() {
        let symbols = vec![record("connect", "one.rs")];
        assert!(matching_records([("node.one", symbols.as_slice())], "missing").is_empty());
    }

    #[test]
    fn test_requires_valid_map_includes_locate() {
        assert!(
            requires_valid_map("locate"),
            "locate must require a valid map, like the other node-lookup tools"
        );
    }

    #[test]
    fn test_execute_locate_blocked_by_invalid_graph() {
        let tmp = std::env::temp_dir().join(format!("cairn-locate-invalid-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);
        let _ = std::fs::write(
            tmp.join("cairn.blueprint"),
            "System Test \"T\" id \"t\" {\n    Module A \"A\" id \"t.dup\" {\n    }\n    Module B \"B\" id \"t.dup\" {\n    }\n}\n",
        );
        let request = QueryRequest {
            tool: "locate".to_owned(),
            symbol: Some("anything".to_owned()),
            ..QueryRequest::default()
        };
        let result = execute(
            &tmp,
            &tmp.join("cairn.blueprint"),
            &tmp.join("meta/changes"),
            &request,
        );
        assert!(
            result.is_err(),
            "locate must be blocked when the graph has integrity errors: {result:?}"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
