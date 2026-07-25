//! First-turn and advertised-metadata budget for the guidance pack.
//!
//! `todo.agent-guidance-router-playbooks` requires the router and its
//! just-in-time references to fit a ceiling derived from the navigation
//! baseline. This test enforces that ceiling so the router cannot silently grow
//! back into the manual it replaced.
//!
//! **Derivation.** The baseline pack arm (`res.agent-experiment-linklint`,
//! 2026-07-23) shipped nine guidance entries and cost roughly 9,800 more input
//! tokens per run than the no-pack arm for no quality gain reaching the
//! preregistered one-point threshold. Measured on that fixture with `o200k_base`,
//! the unavoidable part of that cost, what every session pays before routing, was
//! 5,031 tokens: 265 tokens of advertised metadata (every skill's `name` plus
//! `description`, which a catalog shows unconditionally) and 4,766 tokens of
//! first-turn body (the 3,789-token `cairn-dev` manual plus the 977-token emitted
//! guide). That measured 5,031 is the ceiling.
//!
//! **Byte proxy.** A Rust test cannot run the tokenizer, so the budget is
//! expressed in bytes. Measured on this corpus, `o200k_base` averages 4.31 bytes
//! per token, so the 5,031-token ceiling is about 21,600 bytes. The constant below
//! is deliberately tighter than that conversion: the router shipped at 8,874 bytes
//! (2,060 tokens, 59 percent under ceiling), and a budget set just under the
//! ceiling would not notice the router doubling. 12,000 bytes leaves real
//! authoring headroom while still failing if the manual comes back.

use std::path::{Path, PathBuf};

/// Bytes an agent pays before it has routed anywhere: advertised metadata for
/// every skill, plus the bodies loaded unconditionally.
const FIRST_TURN_BUDGET_BYTES: usize = 12_000;

/// No single just-in-time reference may itself become a manual. Loop mode is the
/// largest by design (it is a full fail-closed procedure) and is excluded; it is
/// loaded only on explicit invocation, never by routing.
const JIT_REFERENCE_BUDGET_BYTES: usize = 6_000;

fn pack_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn repo_root() -> PathBuf {
    pack_dir().join("../..")
}

/// Byte length of a pack asset. The budget is a byte count, so the `u64` the
/// filesystem reports is converted rather than cast: a size that does not fit
/// `usize` is a broken checkout, not a budget breach.
fn file_len(path: &Path) -> usize {
    usize::try_from(std::fs::metadata(path).unwrap().len())
        .unwrap_or_else(|_| panic!("{} is larger than this platform's usize", path.display()))
}

fn frontmatter_field(body: &str, field: &str) -> Option<String> {
    let mut lines = body.lines();
    if lines.next()? != "---" {
        return None;
    }
    lines
        .take_while(|line| *line != "---")
        .find_map(|line| line.strip_prefix(&format!("{field}: ")))
        .map(str::to_owned)
}

#[test]
fn first_turn_and_advertised_metadata_fit_the_baseline_ceiling() {
    let pack = pack_dir();
    let mut metadata = 0usize;
    let skills = std::fs::read_dir(pack.join("content/skills")).unwrap();
    for entry in skills {
        let skill = entry.unwrap().path().join("SKILL.md");
        if !skill.exists() {
            continue;
        }
        let body = std::fs::read_to_string(&skill).unwrap();
        for field in ["name", "description"] {
            metadata += frontmatter_field(&body, field).map_or(0, |value| value.len());
        }
    }
    // A frontmatter parser that silently returns None for every file would zero
    // this term and quietly stop enforcing half the budget. Pin it.
    assert!(
        metadata > 1_000,
        "advertised metadata measured {metadata} bytes across the pack's skills, \
         which is implausibly low: the frontmatter parser is broken, not the pack"
    );

    let router = file_len(&pack.join("content/skills/cairn-dev/SKILL.md"));
    let guide = file_len(&repo_root().join("src/cli/agent_guide.md"));

    let total = metadata + router + guide;
    assert!(
        total <= FIRST_TURN_BUDGET_BYTES,
        "first turn plus advertised metadata is {total} bytes, over the \
         {FIRST_TURN_BUDGET_BYTES} budget (metadata {metadata}, router {router}, \
         emitted guide {guide}). The router is an index: move detail into a \
         just-in-time reference rather than growing this."
    );
}

#[test]
fn no_routed_reference_grows_into_a_manual() {
    let references = pack_dir().join("content/skills/cairn-dev/references");
    for entry in std::fs::read_dir(&references).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        if path.file_name().and_then(|n| n.to_str()) == Some("loop-mode.md") {
            continue;
        }
        let size = file_len(&path);
        assert!(
            size <= JIT_REFERENCE_BUDGET_BYTES,
            "{} is {size} bytes, over the {JIT_REFERENCE_BUDGET_BYTES} per-reference \
             budget; split it or cut it",
            path.display()
        );
    }
}

#[test]
fn the_router_stays_an_index_not_a_manual() {
    let router =
        std::fs::read_to_string(pack_dir().join("content/skills/cairn-dev/SKILL.md")).unwrap();
    // The manual this replaced carried the full command table inline. Its
    // absence is the structural property worth pinning: a router that starts
    // listing flags has stopped routing.
    let inline_command_rows = router
        .lines()
        .filter(|line| line.starts_with('|') && line.contains("--json"))
        .count();
    assert_eq!(
        inline_command_rows, 0,
        "the router has inlined command-table rows; that content belongs in \
         references/command-reference.md"
    );
    assert!(
        router.contains("references/command-reference.md"),
        "the router must route to the command reference instead of inlining it"
    );
}
