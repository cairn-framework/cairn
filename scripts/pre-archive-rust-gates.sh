#!/bin/sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

echo "pre-archive: running cargo fmt --check"
cargo fmt --check

echo "pre-archive: running strict cargo clippy"
RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features

echo "pre-archive: running cargo test"
cargo test

echo "pre-archive: checking Rust file sizes"
scripts/check-file-sizes.sh
