# Proposal: Reconcile the cairn.config.yaml spec with the parser

## Motivation

Issue #215: the reporter copied the `cairn.config.yaml` example from
`docs/spec.md:199-207`:

    reconcilers:
      - id: code
        version: 1
        config:
          tree_sitter_languages: [rust, typescript, python]
          ignore:
            - "**/node_modules/**"
            ...

and `cairn files` was unchanged. (The reporter's own variant used
`[typescript]`; the docs ship `[rust, typescript, python]`. The bug
reproduces either way.) The config parser
(`src/scanner/config/mod.rs`) never implements `reconcilers:` or
`tree_sitter_languages`. `Config` only accepts the top-level YAML keys
`context:`, `rules:`, `artefact_types:`, `targets:`, `multi_target:`, and
`ignore:` (struct fields `ignores` / `intentional_asymmetries` map to the
`ignore:` / `multi_target:` keys). The documented `reconcilers:` keys are
silently dropped with no warning.

This is spec/implementation drift: the docs are ahead of the code, and
the only working language override, the `targets:` list matched by node
in `build_targets` (`src/scanner/mod.rs:74-86`, YAML key `node:` not
`node_id:`), is undocumented. A user following the manual gets a silent
no-op.

## Scope

- Correct `docs/spec.md` to document the config schema the parser
  actually accepts, using the real YAML keys (`targets`, `ignore`,
  `multi_target`, `context`, `rules`, `artefact_types`) and the `targets:`
  entry shape (`node`, `path`, `language`, `contract_role`).
- Mark the `reconcilers:` / `tree_sitter_languages` block as a future
  capability (or remove it), cross-referencing
  `generic-language-reconciler` as the change that would make it real.
- Add an unknown-key warning to the config parser so silently-ignored
  top-level keys (like `reconcilers:` today) surface as a warning finding
  rather than disappearing.

## Out of scope

- Implementing `tree_sitter_languages` as a dynamic grammar selector
  (depends on `generic-language-reconciler`).
- Changing the `targets:` override semantics (already correct).

## Acceptance

- `docs/spec.md` config example matches what the parser accepts (real
  YAML keys, not struct field names).
- A config containing an unknown top-level key produces a warning naming
  the key, without aborting the scan, and the warning appears in
  `cairn scan` / `cairn lint` output.
- The reporter's exact block produces a warning on `reconcilers` (the
  unknown top-level key) whose message notes that `tree_sitter_languages`
  is unsupported and points at the `targets:` override.
