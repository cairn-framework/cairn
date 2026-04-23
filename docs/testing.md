# Testing Workflow

This repository uses `insta` to pin public wire formats and other reviewed output.

## Snapshot Review

Run tests normally while authoring:

```sh
cargo test
```

When a snapshot changes, `insta` writes a `*.snap.new` candidate beside the committed snapshot. Review and accept those updates with:

```sh
cargo insta review
```

Accepted snapshots become committed `*.snap` files under `tests/snapshots/`.

## Snapshot Layout

- Integration-test snapshots live under `tests/snapshots/`.
- `*.snap` files are committed.
- `*.snap.new` files are ignored and only exist during review.

## Inline vs File-Based Snapshots

Use file-based snapshots for public JSON wire formats, CLI payloads, and any response that acts as a stable contract across phases.

Use inline snapshots for small unit-test values where keeping the expected shape next to the assertion makes the test easier to read and review.
