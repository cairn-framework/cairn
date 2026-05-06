# cairn-macros

Procedural macros for the [Cairn](https://github.com/cairn-framework/cairn) architecture graph tool.

This crate exposes a single attribute macro, `#[cflx_planned(phase = <N>)]`, used to mark integration tests that assert against acceptance criteria for a feature phase that has not yet shipped. The macro expands to `#[ignore = "cflx_planned: phase-<N>"]` so `cargo test` continues to pass while the planned tests are queued.

See `openspec/conventions.md` (section: "Test-First Pre-Phase") in the parent repository for the full convention.

## Usage

```rust
use cairn_macros::cflx_planned;

#[cflx_planned(phase = 8)]
#[test]
fn test_summariser_drafts_contract_update() {
    // Assertion against phase-8 acceptance criteria.
    // Test runs (and fails meaningfully) once the `#[cflx_planned]` attribute
    // is removed by the phase-8 implementation.
}
```

The macro rejects combination with a manual `#[ignore]` attribute. If a planned test also needs an unrelated ignore, drop the planned attribute once the prerequisite phase lands and add a plain `#[ignore]` with a reason.

## License

MIT OR Apache-2.0.
