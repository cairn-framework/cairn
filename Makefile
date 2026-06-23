.PHONY: check status status-phases status-worktrees status-untracked install-hooks biome-check biome-fix tokens-check a11y-check

check:
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
	biome check --error-on-warnings src/ui_assets/app.js src/ui_assets/style.css
	sh scripts/check-design-tokens.sh
	sh scripts/check-a11y.sh

# One-screen project status. Sub-targets are independently runnable so a failure
# in one (e.g. corrupt worktree state) doesn't suppress the others.
status: status-phases status-worktrees status-untracked

status-phases:
	@echo "── Active phases (openspec/changes/) ──────────────────────────"
	@for f in openspec/changes/phase-*/tasks.md; do \
		[ -f "$$f" ] || continue; \
		total=$$(grep -c '^[[:space:]]*[-*][[:space:]]*\[' "$$f"); \
		done_count=$$(grep -c '^[[:space:]]*[-*][[:space:]]*\[[xX]\]' "$$f"); \
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

install-hooks:
	prek install --install-hooks --hook-type pre-commit --hook-type pre-push

biome-check:
	biome check --error-on-warnings src/ui_assets/app.js src/ui_assets/style.css

tokens-check:
	sh scripts/check-design-tokens.sh

a11y-check:
	sh scripts/check-a11y.sh

biome-fix:
	biome check --write --unsafe src/ui_assets/app.js
	biome format --write src/ui_assets/style.css
