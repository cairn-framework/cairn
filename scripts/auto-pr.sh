#!/usr/bin/env bash
# Auto PR Review and Merge runner for the OMP coding harness.
# Usage: scripts/auto-pr.sh <pr-number> [--fix]
#
# Phases:
#   1. checkout  — fetch and check out the PR branch
#   2. gates     — run all quality gates (fmt, clippy, test, lint, hook all, dogfood)
#   3. report    — print findings; exit non-zero if any gate fails
#   4. merge     — if --fix was NOT passed and all gates pass, merge the PR
#
# When called without --fix, this is a read-only assessment.
# When called with --fix, the caller (agent) is expected to fix issues and re-run.

set -euo pipefail

PR_NUMBER="${1:-}"
FIX_MODE=false
if [[ "${2:-}" == "--fix" ]]; then
  FIX_MODE=true
fi

if [[ -z "$PR_NUMBER" ]]; then
  echo "Usage: scripts/auto-pr.sh <pr-number> [--fix]" >&2
  exit 1
fi

REPO_ROOT=$(git rev-parse --show-toplevel)
cd "$REPO_ROOT"

# ── Phase 1: checkout ─────────────────────────────────────────────────────────
echo "== Auto PR: #$PR_NUMBER =="
echo ""

# Fetch PR metadata
PR_JSON=$(gh pr view "$PR_NUMBER" --json number,title,author,headRefName,baseRefName,mergeStateStatus,mergeable,url)
HEAD_BRANCH=$(echo "$PR_JSON" | jq -r '.headRefName')
BASE_BRANCH=$(echo "$PR_JSON" | jq -r '.baseRefName')
PR_TITLE=$(echo "$PR_JSON" | jq -r '.title')
PR_AUTHOR=$(echo "$PR_JSON" | jq -r '.author.login')
PR_URL=$(echo "$PR_JSON" | jq -r '.url')

echo "PR:    #$PR_NUMBER — $PR_TITLE"
echo "Author: @$PR_AUTHOR"
echo "Branch: $HEAD_BRANCH → $BASE_BRANCH"
echo "URL:    $PR_URL"
echo ""

# Stash any local changes before switching branches
if ! git diff --quiet HEAD || ! git diff --cached --quiet HEAD; then
  echo "Stashing local changes..."
  git stash push -m "auto-pr-stash-$(date +%s)"
  STASHED=true
else
  STASHED=false
fi

# Fetch and check out the PR branch
gh pr checkout "$PR_NUMBER"

echo ""
echo "Checked out $HEAD_BRANCH at $(git rev-parse --short HEAD)"
echo ""

# ── Phase 2: gates ────────────────────────────────────────────────────────────
GATES_PASSED=true

run_gate() {
  local name="$1"
  shift
  echo "== Gate: $name =="
  if "$@"; then
    echo "✅ $name passed"
    echo ""
    return 0
  else
    echo "❌ $name FAILED"
    echo ""
    return 1
  fi
}

# Gate: cargo fmt
cargo fmt --check 2>&1 || { GATES_PASSED=false; echo "   → cargo fmt --check failed"; }

# Gate: cargo clippy
cargo clippy --lib --tests 2>&1 || { GATES_PASSED=false; echo "   → cargo clippy failed"; }

# Gate: cargo test
cargo test 2>&1 || { GATES_PASSED=false; echo "   → cargo test failed"; }

# Gate: cairn lint
cairn lint 2>&1 || { GATES_PASSED=false; echo "   → cairn lint failed"; }

# Gate: cairn hook all
cairn hook all 2>&1 || { GATES_PASSED=false; echo "   → cairn hook all failed"; }

# Gate: dogfood
echo "== Gate: dogfood =="
if bash scripts/dogfood.sh; then
  echo "✅ dogfood passed"
  echo ""
else
  echo "❌ dogfood FAILED"
  echo ""
  GATES_PASSED=false
fi

echo ""

# ── Phase 3: report ───────────────────────────────────────────────────────────
if $GATES_PASSED; then
  echo "========================================"
  echo "✅ ALL GATES PASSED"
  echo "========================================"
  echo ""
  echo "PR #$PR_NUMBER is ready for merge."
  echo ""
else
  echo "========================================"
  echo "❌ GATES FAILED"
  echo "========================================"
  echo ""
  echo "PR #$PR_NUMBER has issues that must be fixed before merge."
  echo "Run with --fix after making corrections."
  echo ""
fi

# ── Phase 4: merge (only if --fix was NOT passed and all gates pass) ─────────
if ! $FIX_MODE && $GATES_PASSED; then
  echo "== Merging PR #$PR_NUMBER =="
  gh pr merge "$PR_NUMBER" --squash --delete-branch
  echo "✅ Merged and deleted branch."
fi

# Restore stashed changes if we stashed them
if [[ "$STASHED" == "true" ]]; then
  echo ""
  echo "Restoring stashed changes..."
  git stash pop || echo "Warning: stash pop had conflicts"
fi

if $GATES_PASSED; then
  exit 0
else
  exit 2
fi
