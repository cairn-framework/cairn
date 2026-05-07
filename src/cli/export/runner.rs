//! CLI dispatch for `cairn export`: parses local flags, builds the
//! envelope, renders, and writes to disk.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    cli::{
        CliResult,
        format::{err, error_output, ok},
    },
    error::CairnError,
};

use super::{ExportEnvelope, build_export, render_json, render_markdown};

/// Entrypoint for the `cairn export` command.
#[must_use]
pub fn run(args: &[String], file: &Path, changes_dir: &Path) -> CliResult {
    let parsed = match parse_export_args(args) {
        Ok(p) => p,
        Err(result) => return result,
    };
    let envelope = match build_export(file, changes_dir) {
        Ok(e) => e,
        Err(error) => return error_output(false, error.code(), &error.to_string()),
    };
    let body = render(&envelope, parsed.format);
    if let Err(error) = write_output(&parsed.output, &body) {
        return error_output(false, error.code(), &error.to_string());
    }
    ok(format!(
        "wrote {} bytes to {}\n",
        body.len(),
        parsed.output.to_string_lossy()
    ))
}

fn render(envelope: &ExportEnvelope, format: ExportFormat) -> String {
    match format {
        ExportFormat::Json => render_json(envelope),
        ExportFormat::Markdown => render_markdown(envelope),
    }
}

fn write_output(path: &Path, body: &str) -> Result<(), CairnError> {
    fs::write(path, body).map_err(|e| CairnError::WriteOutput {
        path: path.to_string_lossy().into_owned(),
        detail: e.to_string(),
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExportFormat {
    Json,
    Markdown,
}

struct ExportArgs {
    format: ExportFormat,
    output: PathBuf,
}

fn parse_export_args(args: &[String]) -> Result<ExportArgs, CliResult> {
    let mut format = ExportFormat::Json;
    let mut output: Option<PathBuf> = None;
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--format" => {
                let Some(value) = iter.next() else {
                    return Err(err(1, "--format requires a value (json or md)"));
                };
                format = parse_format(value)?;
            }
            "--output" => {
                let Some(value) = iter.next() else {
                    return Err(err(1, "--output requires a path"));
                };
                output = Some(PathBuf::from(value));
            }
            other => {
                return Err(err(1, &format!("unknown export flag: {other}")));
            }
        }
    }
    let Some(output) = output else {
        return Err(err(1, "--output is required for cairn export"));
    };
    Ok(ExportArgs { format, output })
}

fn parse_format(value: &str) -> Result<ExportFormat, CliResult> {
    match value {
        "json" => Ok(ExportFormat::Json),
        "md" | "markdown" => Ok(ExportFormat::Markdown),
        other => Err(err(
            1,
            &format!("--format value '{other}' is not 'json' or 'md'"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_export_args_requires_output() {
        let args = vec![
            "export".to_owned(),
            "--format".to_owned(),
            "json".to_owned(),
        ];
        let result = parse_export_args(&args);
        let Err(r) = result else {
            panic!("expected error")
        };
        assert_eq!(r.code, 1);
        assert!(r.stderr.contains("--output"));
    }

    #[test]
    fn parse_export_args_rejects_invalid_format() {
        let args = vec![
            "export".to_owned(),
            "--format".to_owned(),
            "csv".to_owned(),
            "--output".to_owned(),
            "out.csv".to_owned(),
        ];
        let result = parse_export_args(&args);
        let Err(r) = result else {
            panic!("expected error")
        };
        assert_eq!(r.code, 1);
        assert!(r.stderr.contains("csv"));
    }

    #[test]
    fn parse_export_args_accepts_md_format() {
        let args = vec![
            "export".to_owned(),
            "--format".to_owned(),
            "md".to_owned(),
            "--output".to_owned(),
            "out.md".to_owned(),
        ];
        let Ok(parsed) = parse_export_args(&args) else {
            panic!("should parse")
        };
        assert_eq!(parsed.format, ExportFormat::Markdown);
        assert_eq!(parsed.output, PathBuf::from("out.md"));
    }
}
