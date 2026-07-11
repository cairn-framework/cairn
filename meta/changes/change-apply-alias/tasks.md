# Tasks: change-apply-alias

- [x] Add `Some("apply")` match arm aliasing `Some("archive")` in `run_change_command`.
- [x] Update usage messages to use the invoked verb name.
- [x] Update `change` command description to include `apply`.
- [x] Add `test_cli_apply_aliases_archive` test (TDD: wrote test first, verified red, then green).
- [x] Update `test_every_registered_command_has_description` expected description.
- [x] Switch all user-facing docs and agent-facing guides to `apply` as primary verb (8 files).
- [x] Verify: cargo build, clippy, fmt, test, cairn scan, cairn hook all, cairn change accept.
