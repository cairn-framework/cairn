#!/bin/sh
# Dogfood gate: run cairn against itself.
# Called from pre-push hook and CI workflow.
#
# Build and run the working tree's own cairn via `cargo run`, never a
# PATH-installed binary. A stale ~/.cargo/bin/cairn can make this gate
# false-green by linting with an old binary that lacks the working tree's
# newer checks (cairn-9ey). `cargo run` builds first and invokes exactly the
# binary it produced, so it also respects CARGO_TARGET_DIR.
set -e

manifest="$(git rev-parse --show-toplevel)/Cargo.toml"
run_cairn() {
    cargo run --release --quiet --manifest-path "$manifest" --bin cairn -- "$@"
}

echo "== cairn lint =="
run_cairn lint

echo "== cairn hook all =="
run_cairn hook all

echo "== dogfood pass =="
