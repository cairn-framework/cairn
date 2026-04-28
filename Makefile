.PHONY: check status status-phases status-worktrees status-untracked

check:
	cargo fmt --check
	RUSTFLAGS="-D warnings" cargo clippy --all-targets --all-features
	cargo test
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# One-screen project status. Sub-targets are independently runnable so a failure
# in one (e.g. corrupt worktree state) doesn't suppress the others.
status: status-phases status-worktrees status-untracked

status-phases:
	@echo "── Active phases (openspec/changes/) ──────────────────────────"
	@for f in openspec/changes/phase-*/tasks.md; do \
		[ -f "$$f" ] || continue; \
		total=$$(grep -c '^[[:space:]]*[-*][[:space:]]*\[' "$$f"); \
		done_count=$$(grep -c '^[[:space:]]*[-*][[:space:]]*\[x\]' "$$f"); \
		dir=$$(dirname "$$f"); \
		printf "  %-45s  %2d / %2d done\n" "$$dir" "$$done_count" "$$total"; \
	done

status-worktrees:
	@echo
	@echo "── Worktrees ──────────────────────────────────────────────────"
	@git worktree list

status-untracked:
	@echo
	@echo "── Untracked roots ────────────────────────────────────────────"
	@git status --short | awk '/^\?\?/ {print "  " $$2}' | head -20
	@untracked=$$(git status --short | awk '/^\?\?/' | wc -l | tr -d ' '); \
	if [ "$$untracked" -gt 20 ]; then \
		echo "  ... and $$(($$untracked - 20)) more"; \
	fi
