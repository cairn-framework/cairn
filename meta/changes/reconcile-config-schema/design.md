# Design: Reconcile config spec with parser

## Approach

Two independent edits.

1. Spec correction. Rewrite the `cairn.config.yaml` block in
   `docs/spec.md` (around line 199) to show the real schema, using the
   parser's actual top-level YAML keys: `context:`, `rules:`,
   `artefact_types:`, `targets:`, `multi_target:`, `ignore:`. Lead with
   `targets:` as the language-override mechanism, since that is what
   `build_targets` consumes. Note that struct fields `ignores` and
   `intentional_asymmetries` are populated by the `ignore:` and
   `multi_target:` keys respectively. Keep the `reconcilers:` /
   `tree_sitter_languages` shape in a clearly labelled "future" subsection
   that points at `generic-language-reconciler`, so the aspiration is
   recorded but no longer masquerades as working config.

2. Unknown-key warning. The config parser is a line-oriented state
   machine (`src/scanner/config/mod.rs`, `parse_config`). Add a known
   top-level-key set using the real YAML keys (`context`, `rules`,
   `artefact_types`, `targets`, `multi_target`, `ignore`) and, when a
   top-level `key:` is seen that is not in the set, record a warning
   naming the key. Non-aborting: the scan continues with the recognised
   keys.

### Warning carrier (findings have no central registry)

`parse_config` only mutates `Config`; `load_project` merges contract,
artefact, and reconcile findings into the graph, not config warnings. To
make the warning reach `cairn scan` / `cairn lint` output, add a carrier:
either a `Config.findings: Vec<Finding>` field (or a typed
`ConfigWarning`) that `load_project` converts into `Finding`s before
`build_graph`. The code is emitted as an inline `Finding` string
(`CAIRN_CONFIG_UNKNOWN_KEY`), with `heading`/`body`/`cta` text under
`[findings.codes]` in `docs/design-system/copy.toml` via the `src/copy.rs`
lookup pattern.

### Reporter-block scope

In the documented block, `tree_sitter_languages` is nested under
`reconcilers` -> `config`, so a top-level-key warning only sees
`reconcilers`. The warning message should name `reconcilers` as the
unknown key and explain that the nested `tree_sitter_languages` is
unsupported, pointing at the `targets:` override. Do not attempt nested
detection for that future block (it is not a real schema yet).

## Changes

MODIFIED:
- `docs/spec.md`: config example rewritten to match the parser (real YAML
  keys); future `reconcilers:` block labelled and cross-referenced.
- `src/scanner/config/mod.rs`: known-key set (YAML keys) +
  `CAIRN_CONFIG_UNKNOWN_KEY` warning recording.
- `src/scanner/mod.rs` (`load_project`): convert config warnings into
  `Finding`s before `build_graph`.

ADDED:
- `Config.findings` (or `ConfigWarning`) carrier field.
- Finding code `CAIRN_CONFIG_UNKNOWN_KEY`: inline `Finding` emission +
  `[findings.codes]` entry in `copy.toml`.

## Guards

- Config parser tests: unknown top-level key -> warning, recognised keys
  still parsed; the warning reaches `cairn scan` / `cairn lint` output.
- The reporter's exact block -> one warning naming `reconcilers`, scan
  otherwise unaffected.
- Self-host `cairn scan` unaffected (no unknown keys in this repo's
  config).
