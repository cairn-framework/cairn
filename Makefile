.PHONY: check

check:
	cargo fmt --check
	RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features
	cargo test
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
