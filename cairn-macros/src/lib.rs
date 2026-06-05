//! Proc macros for the Cairn architecture graph tool.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Lit, parse_macro_input, spanned::Spanned};

/// Marks a test as planned for a future phase.
///
/// Emits `#[ignore = "cairn_planned: phase-<N>"]` and registers the test in `target/cairn/planned.json`.
#[proc_macro_attribute]
pub fn cairn_planned(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Parse phase argument
    let mut phase: Option<u32> = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("phase") {
            let value = meta.value()?;
            let lit: Lit = value.parse()?;
            if let Lit::Int(val) = lit {
                let val = val
                    .base10_parse::<i64>()
                    .map_err(|e| syn::Error::new(val.span(), e))?;
                let phase_val = u32::try_from(val).map_err(|_| {
                    syn::Error::new(val.span(), "phase must be a positive integer (>= 1)")
                })?;
                if phase_val == 0 {
                    return Err(syn::Error::new(
                        val.span(),
                        "phase must be a positive integer (>= 1)",
                    ));
                }
                phase = Some(phase_val);
                Ok(())
            } else {
                Err(syn::Error::new(lit.span(), "phase must be an integer"))
            }
        } else {
            Err(meta.error("unsupported argument; expected `phase = <positive_integer>`"))
        }
    });
    if let Err(e) = syn::parse::Parser::parse2(parser, args.into()) {
        return e.to_compile_error().into();
    }

    let Some(phase) = phase else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "cairn_planned requires a `phase = <positive_integer>` argument",
        )
        .to_compile_error()
        .into();
    };

    // Check for existing #[ignore] attribute
    for attr in &input_fn.attrs {
        if attr.path().is_ident("ignore") {
            return syn::Error::new(
                attr.span(),
                "cannot combine #[cairn_planned] with #[ignore]; use one mechanism only",
            )
            .to_compile_error()
            .into();
        }
    }

    let fn_name = input_fn.sig.ident.to_string();
    let file = String::from("<unknown>");
    let line = 0;

    // Write registration entry to sidecar
    let sidecar_path = std::path::PathBuf::from("target/cairn/planned.json");
    let _ = update_sidecar(&sidecar_path, &fn_name, phase, &file, line);

    let ignore_msg = format!("cairn_planned: phase-{phase}");
    let expanded = quote! {
        #[ignore = #ignore_msg]
        #input_fn
    };

    expanded.into()
}

fn update_sidecar(
    path: &std::path::Path,
    test_path: &str,
    phase: u32,
    file: &str,
    line: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path.parent().ok_or("invalid path")?)?;

    let mut entries = Vec::new();
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        // Simple parsing: extract entries array
        if let Some(start) = content.find("\"entries\"")
            && let Some(arr_start) = content[start..].find('[')
        {
            let arr_start = start + arr_start;
            if let Some(arr_end) = content[arr_start..].rfind(']') {
                let arr_end = arr_start + arr_end + 1;
                let arr_text = &content[arr_start..arr_end];
                // Parse individual entries
                for entry_text in split_json_array(arr_text) {
                    entries.push(entry_text.to_string());
                }
            }
        }
    }

    // Add new entry
    let new_entry = format!(
        "{{ \"test_path\": \"{}\", \"phase\": {}, \"file\": \"{}\", \"line\": {} }}",
        escape_json(test_path),
        phase,
        escape_json(file),
        line
    );
    entries.push(new_entry);

    // Build output
    let entries_text = entries.join(",\n    ");
    let output = format!("{{\n  \"version\": 1,\n  \"entries\": [\n    {entries_text}\n  ]\n}}");

    std::fs::write(path, output)?;
    Ok(())
}

fn split_json_array(arr_text: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let mut in_string = false;
    let mut escape = false;

    for (i, ch) in arr_text.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            '{' if !in_string => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    entries.push(&arr_text[start..=i]);
                }
            }
            _ => {}
        }
    }

    entries
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
