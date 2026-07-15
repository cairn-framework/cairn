//! Per-command help routing and rendering.
//!
//! Global `cairn --help` stays in `help_text()` (mod.rs). This module owns
//! `cairn <cmd> --help` so the dispatch hub does not grow past its allow-list.
//!
//! Flag labels and usage synopsis strings live in `docs/design-system/copy.toml`
//! under `help.*`. The Rust tables only select which copy keys apply to each
//! command, so prose cannot drift from the design-system file while the set of
//! recognised spellings is still enforced in one place.
//!
//! Compound spellings (`change accept`, `draft list`, …) are first-class help
//! targets. Lookup prefers the longest matching command path so
//! `cairn change accept --help` renders accept-specific usage, not the generic
//! `change` page.
//!
//! Optional per-command copy keys under `help.commands.<copy_key>`:
//! - `usage` (required)
//! - `args` (optional Arguments section body)
//! - `description` (optional one-line summary; else registry/CLI-only desc)
//! - `preferred` (optional preferred spelling for retired aliases)

use super::{command_description, copy, version_label};

/// Result of inspecting argv for a help request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HelpTarget<'a> {
    /// Bare `cairn --help` / `cairn -h` (no command token).
    Global,
    /// `cairn <cmd…> --help`. The string is the longest recognised command path
    /// (possibly compound, e.g. `"change accept"`), or the first unknown token
    /// when no path matches.
    Command(&'a str),
}

/// Metadata that selects which copy keys render for a command path.
#[derive(Clone, Copy, Debug)]
struct CommandHelpSpec {
    /// Command path users type after `cairn` (may contain a space for subcommands).
    name: &'static str,
    /// Copy key suffix under `help.commands.<suffix>.*`.
    copy_key: &'static str,
    /// Ordered `help.flags.<key>` entries to list under Options.
    flags: &'static [&'static str],
}

/// Shared flag sets referenced by multiple commands (keeps the table short).
const FLAGS_BASIC: &[&str] = &["json", "file", "help"];
const FLAGS_JSON_HELP: &[&str] = &["json", "help"];
const FLAGS_NODE: &[&str] = &["json", "file", "help"];
const FLAGS_LINT: &[&str] = &["node", "strict", "verbose", "json", "file", "help"];
const FLAGS_STATUS: &[&str] = &["brief", "json", "file", "help"];
const FLAGS_ACCEPT: &[&str] = &["json", "help"];

/// Every recognised top-level and compound spelling, including retired aliases.
/// Coverage of top-level names is enforced by `every_recognised_command_has_help`.
const COMMAND_HELP: &[CommandHelpSpec] = &[
    // --- top-level (alphabetical) ------------------------------------------
    spec("backlog", "backlog", FLAGS_NODE),
    spec("beads", "beads", FLAGS_NODE),
    spec("blueprint", "blueprint", FLAGS_BASIC),
    spec("brief", "brief", FLAGS_BASIC),
    spec("bundle", "bundle", FLAGS_NODE),
    spec("change", "change", FLAGS_JSON_HELP),
    spec(
        "context",
        "context",
        &["depth", "scope", "mermaid", "json", "file", "help"],
    ),
    spec("contract", "contract", FLAGS_NODE),
    spec(
        "decision",
        "decision",
        &["node-flag", "informed-by", "help"],
    ),
    spec(
        "decisions",
        "decisions",
        &["status", "grep", "json", "file", "help"],
    ),
    spec(
        "deps",
        "deps",
        &["direction", "transitive", "json", "file", "help"],
    ),
    spec(
        "docstring",
        "docstring",
        &["language", "json", "file", "help"],
    ),
    spec("draft", "draft", &["edited", "json", "help"]),
    spec(
        "export",
        "export",
        &["format", "output", "file", "changes-dir", "json", "help"],
    ),
    spec("feedback", "feedback", &["help"]),
    spec("files", "files", FLAGS_NODE),
    spec("frontier", "frontier", FLAGS_BASIC),
    spec("gap", "gap", &["question", "file", "help"]),
    spec("get", "get", &["symbols", "json", "file", "help"]),
    spec("graph", "graph", FLAGS_BASIC),
    spec("health", "health", FLAGS_BASIC),
    spec("hook", "hook", &["pre-push", "json", "file", "help"]),
    spec("import-openspec", "import-openspec", FLAGS_JSON_HELP),
    spec(
        "init",
        "init",
        &["from-code", "apply", "wire", "force", "help"],
    ),
    spec("islands", "islands", FLAGS_BASIC),
    spec("lint", "lint", FLAGS_LINT),
    spec(
        "neighbourhood",
        "neighbourhood",
        &[
            "include-orphans",
            "include-todos",
            "include-research",
            "include-reviews",
            "include-deprecated-decisions",
            "include-changes",
            "json",
            "file",
            "help",
        ],
    ),
    spec("next", "next", FLAGS_BASIC),
    spec("onboard", "onboard", FLAGS_BASIC),
    spec("order", "order", FLAGS_BASIC),
    spec("rationale", "rationale", FLAGS_NODE),
    spec("refine", "refine", FLAGS_JSON_HELP),
    spec("remediate", "remediate", FLAGS_BASIC),
    spec("rename", "rename", &["json", "file", "help"]),
    spec("research", "research", FLAGS_NODE),
    spec("scan", "scan", FLAGS_LINT),
    spec("sources", "sources", FLAGS_NODE),
    spec("status", "status", FLAGS_STATUS),
    spec("todo", "todo", &["node-flag", "help"]),
    spec("todos", "todos", &["status", "json", "file", "help"]),
    spec("ui", "ui", &["port", "no-open", "help"]),
    spec("ui_meta", "ui_meta", FLAGS_BASIC),
    spec("watch", "watch", &["interval", "once", "help"]),
    spec("workspace", "workspace", FLAGS_JSON_HELP),
    // --- compound change family --------------------------------------------
    spec("change new", "change-new", FLAGS_JSON_HELP),
    spec("change list", "change-list", FLAGS_JSON_HELP),
    spec("change show", "change-show", FLAGS_JSON_HELP),
    spec("change accept", "change-accept", FLAGS_ACCEPT),
    spec("change apply", "change-apply", FLAGS_JSON_HELP),
    spec("change archive", "change-archive", FLAGS_JSON_HELP),
    // --- compound draft family ---------------------------------------------
    spec("draft list", "draft-list", FLAGS_JSON_HELP),
    spec("draft show", "draft-show", FLAGS_JSON_HELP),
    spec("draft edit", "draft-edit", FLAGS_JSON_HELP),
    spec("draft discard", "draft-discard", FLAGS_JSON_HELP),
    spec("draft accept", "draft-accept", &["edited", "json", "help"]),
    spec("draft create", "draft-create", FLAGS_JSON_HELP),
    // --- other compounds ---------------------------------------------------
    spec(
        "decision new",
        "decision-new",
        &["node-flag", "informed-by", "help"],
    ),
    spec("todo new", "todo-new", &["node-flag", "help"]),
    spec("todo set", "todo-set", &["help"]),
    spec("workspace status", "workspace-status", FLAGS_JSON_HELP),
    spec("workspace lint", "workspace-lint", FLAGS_JSON_HELP),
    spec("workspace frontier", "workspace-frontier", FLAGS_JSON_HELP),
    // --- retired top-level aliases -----------------------------------------
    // preferred/description come from copy.toml under help.commands.<copy_key>
    spec("accept", "accept", FLAGS_ACCEPT),
    spec("archive", "archive", FLAGS_JSON_HELP),
    spec("changes", "changes", FLAGS_JSON_HELP),
    spec("show", "show", FLAGS_JSON_HELP),
];

const fn spec(
    name: &'static str,
    copy_key: &'static str,
    flags: &'static [&'static str],
) -> CommandHelpSpec {
    CommandHelpSpec {
        name,
        copy_key,
        flags,
    }
}

/// Inspects argv for `--help`/`-h` and returns the help target, if any.
///
/// Returns `None` when the user did not request help. Bare global help (no
/// command token) and per-command help are distinguished so the dispatch
/// short-circuit can keep both behaviours.
#[must_use]
pub(crate) fn help_request(args: &[String]) -> Option<HelpTarget<'_>> {
    if !args.iter().any(|a| a == "--help" || a == "-h") {
        return None;
    }
    let tokens = command_tokens(args);
    if tokens.is_empty() {
        return Some(HelpTarget::Global);
    }
    // Longest-prefix match against `COMMAND_HELP` names. When nothing matches,
    // fall back to the first token (unknown command).
    if let Some(matched) = resolve_spec(&tokens) {
        return Some(HelpTarget::Command(matched.name));
    }
    Some(HelpTarget::Command(tokens[0]))
}

/// Non-flag argv tokens, skipping known global options and their values.
fn command_tokens(args: &[String]) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" | "--json" | "--strict" | "--verbose" | "--brief" | "--version" => {}
            "--file" | "--changes-dir" => {
                let _ = iter.next();
            }
            s if s.starts_with('-') => {}
            s => tokens.push(s),
        }
    }
    tokens
}

/// Longest `COMMAND_HELP` name whose tokens are a prefix of `tokens`.
fn resolve_spec(tokens: &[&str]) -> Option<&'static CommandHelpSpec> {
    let mut best: Option<&CommandHelpSpec> = None;
    let mut best_len = 0usize;
    for entry in COMMAND_HELP {
        let parts: Vec<&str> = entry.name.split(' ').collect();
        if parts.len() > tokens.len() {
            continue;
        }
        if parts.iter().zip(tokens.iter()).all(|(a, b)| a == b) && parts.len() > best_len {
            best = Some(entry);
            best_len = parts.len();
        }
    }
    best
}

/// Renders the per-command usage page for `name`, or `None` if unknown.
///
/// `name` may be a compound path (`"change accept"`).
#[must_use]
pub(crate) fn command_help_text(name: &str) -> Option<String> {
    let entry = COMMAND_HELP.iter().find(|s| s.name == name)?;
    Some(render_spec(entry))
}

fn render_spec(entry: &CommandHelpSpec) -> String {
    let usage_key = format!("help.commands.{}.usage", entry.copy_key);
    let usage = copy::lookup(&usage_key);
    let description = description_for(entry);
    let mut out = format!("{}\n\n", version_label());
    out.push_str(copy::lookup("help.usage-label"));
    out.push(' ');
    out.push_str(usage);
    out.push('\n');
    if !description.is_empty() {
        out.push('\n');
        out.push_str(&description);
        out.push('\n');
    }
    if let Some(preferred) = preferred_for(entry) {
        out.push('\n');
        out.push_str(&copy::lookup("help.retired-note").replace("{preferred}", &preferred));
        out.push('\n');
    }
    let args_key = format!("help.commands.{}.args", entry.copy_key);
    let args = copy::lookup(&args_key);
    // lookup falls back to the key itself when missing; only emit the
    // Arguments section when a real string was authored.
    if args != args_key && !args.is_empty() {
        out.push('\n');
        out.push_str(copy::lookup("help.arguments-heading"));
        out.push('\n');
        out.push_str(args);
        out.push('\n');
    }
    if !entry.flags.is_empty() {
        out.push('\n');
        out.push_str(copy::lookup("help.options-heading"));
        out.push('\n');
        for key in entry.flags {
            let flag_key = format!("help.flags.{key}");
            out.push_str(copy::lookup(&flag_key));
            out.push('\n');
        }
    }
    out
}

fn description_for(entry: &CommandHelpSpec) -> String {
    let key = format!("help.commands.{}.description", entry.copy_key);
    let from_copy = copy::lookup(&key);
    if from_copy != key && !from_copy.is_empty() {
        return from_copy.to_owned();
    }
    // Prefer an exact registry match for compound paths (`draft list`, …),
    // then fall back to the top-level token's description.
    let exact = command_description(entry.name);
    if !exact.is_empty() {
        return exact.to_owned();
    }
    let top = entry.name.split(' ').next().unwrap_or(entry.name);
    command_description(top).to_owned()
}

fn preferred_for(entry: &CommandHelpSpec) -> Option<String> {
    let key = format!("help.commands.{}.preferred", entry.copy_key);
    let from_copy = copy::lookup(&key);
    if from_copy != key && !from_copy.is_empty() {
        Some(from_copy.to_owned())
    } else {
        None
    }
}

/// True when `name` is a known help spelling (top-level or compound).
#[must_use]
#[cfg(test)]
pub(crate) fn is_known_help_command(name: &str) -> bool {
    COMMAND_HELP.iter().any(|s| s.name == name)
}

#[cfg(test)]
mod tests;
