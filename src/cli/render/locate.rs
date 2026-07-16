//! Human renderer for exact symbol locations.
use super::super::format::symbol_arg;
use super::super::*;
use crate::query_api::QueryRequest;
use std::fmt::Write;

pub(crate) fn render_locate(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> Result<String, Finding> {
    let symbol = symbol_arg(&parsed.command_args)?;
    let request = QueryRequest {
        tool: "locate".to_owned(),
        symbol: Some(symbol.to_owned()),
        ..QueryRequest::default()
    };
    let data = crate::query_api::execute_with_scan(
        root,
        &parsed.file,
        &root.join(&parsed.changes_dir),
        &request,
        scan_result,
    )
    .map_err(super::query_error_to_finding)?
    .data;
    let matches = data["matches"].as_array().cloned().unwrap_or_default();
    if matches.is_empty() {
        return Ok(format!(
            "{}\n",
            crate::copy::lookup("locate.no-matches").replace("{symbol}", symbol)
        ));
    }
    let mut out = format!("{symbol}:\n");
    for item in matches {
        let _ = writeln!(
            out,
            "  {}:{}-{} [{}] {} ({})",
            item["file"].as_str().unwrap_or_default(),
            item["line"],
            item["end_line"],
            item["kind"].as_str().unwrap_or_default(),
            item["signature"].as_str().unwrap_or_default(),
            item["node_id"].as_str().unwrap_or_default(),
        );
    }
    Ok(out)
}
