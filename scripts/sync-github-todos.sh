#!/usr/bin/env bash
# One-way projector: mirror meta/todos/*.md to GitHub issues.
#
# Files in git are canonical (dec.bead-github-sync, dec.native-todos-first);
# GitHub issues are a derived, read-only projection. This script upserts one
# issue per native todo (keyed by a stable marker line), closes issues whose
# todo is done or deleted, and flags externally filed issues for triage. It
# never creates a todo from an issue and never deletes an issue.
#
# Usage: scripts/sync-github-todos.sh [--dry-run]
# Requires: gh (authenticated). Idempotent: two consecutive runs are a no-op.
set -euo pipefail

TODO_DIR="meta/todos"
LABEL="cairn-todo"
UNMAPPED_LABEL="cairn-todo-unmapped"
MARKER_PREFIX="cairn-todo: todo."
DRY_RUN=0
[ "${1:-}" = "--dry-run" ] && DRY_RUN=1
# Scope guard: never run against an unintended repository.
TARGET_REPO="${GH_REPO:-$(gh repo view --json nameWithOwner --jq .nameWithOwner 2>/dev/null || true)}"
if [ -z "$TARGET_REPO" ]; then
  echo "refusing to sync: no repository resolved (set GH_REPO or run inside the repo)" >&2
  exit 1
fi
echo "syncing todos to: $TARGET_REPO"

run() {
  if [ "$DRY_RUN" -eq 1 ]; then
    echo "DRY: $*"
  else
    "$@" >/dev/null
  fi
}

# Ensure labels exist (idempotent; create fails silently when present).
if [ "$DRY_RUN" -eq 0 ]; then
  gh label create "$LABEL" --description "Mirrored from meta/todos (read-only projection)" --color 5319e7 2>/dev/null || true
  gh label create "$UNMAPPED_LABEL" --description "Filed on GitHub without a native todo; needs triage" --color d93f0b 2>/dev/null || true
fi

# Current projection state, keyed by slug. The inventory is materialised
# and validated BEFORE any mutation: a failed or partial list must abort
# the run, never masquerade as an empty projection (which would recreate
# every issue).
PROJECTION="$(gh issue list --label "$LABEL" --state all --limit 500 \
  --json number,state,title,body \
  --jq '.[] | [(.number|tostring), .state, .title,
        ((.body | capture("cairn-todo: todo\\.(?<s>[A-Za-z0-9._-]+)").s) // ""),
        ((.body | capture("\nstatus: (?<st>[a-z_]+)").st) // ""),
        ((.body | capture("\nnode: (?<n>[A-Za-z0-9._-]+)").n) // "")] | @tsv')"
declare -A ISSUE_NUM ISSUE_STATE ISSUE_TITLE ISSUE_STATUS ISSUE_NODE
while IFS=$'\t' read -r number state title slug status node_field; do
  [ -n "$slug" ] || continue
  ISSUE_NUM["$slug"]="$number"
  ISSUE_STATE["$slug"]="$state"
  ISSUE_TITLE["$slug"]="$title"
  ISSUE_STATUS["$slug"]="$status"
  ISSUE_NODE["$slug"]="$node_field"
done <<<"$PROJECTION"

# Desired state from canonical todo files.
declare -A SEEN
for file in "$TODO_DIR"/todo.*.md; do
  [ -e "$file" ] || continue
  slug="$(basename "$file" .md)"; slug="${slug#todo.}"
  SEEN["$slug"]=1
  status="$(sed -n 's/^status:[[:space:]]*//p' "$file" | head -1)"
  node="$(sed -n 's/^node:[[:space:]]*//p' "$file" | head -1)"
  title="$(sed -n 's/^#[[:space:]]\{1,\}//p' "$file" | head -1)"
  [ -n "$title" ] || title="$slug"
  case "$status" in
    done) want_state="CLOSED" ;;
    open | in_progress | blocked) want_state="OPEN" ;;
    *) echo "skip $file: unknown status '$status'" >&2; continue ;;
  esac
  want_title="[todo] $title"
  body="${MARKER_PREFIX}${slug}
node: ${node}
status: ${status}
artefact: ${file}

Mirrored from the canonical todo artefact. Files in git are the source of
truth; edits here are never read back (dec.bead-github-sync)."

  if [ -z "${ISSUE_NUM[$slug]:-}" ]; then
    # New todo. A done todo with no issue needs no projection.
    if [ "$want_state" = "OPEN" ]; then
      echo "create: $slug ($status)"
      if [ "$DRY_RUN" -eq 1 ]; then
        echo "DRY: gh issue create --title '$want_title' --label $LABEL"
      else
        gh issue create --title "$want_title" --label "$LABEL" --body "$body" >/dev/null
      fi
    fi
    continue
  fi

  number="${ISSUE_NUM[$slug]}"
  if [ "${ISSUE_TITLE[$slug]}" != "$want_title" ]; then
    echo "retitle #$number: $slug"
    run gh issue edit "$number" --title "$want_title"
  fi
  if [ "${ISSUE_STATUS[$slug]:-}" != "$status" ] || [ "${ISSUE_NODE[$slug]:-}" != "$node" ]; then
    # Canonical status or node changed (or the derived body was edited on
    # GitHub): rewrite the projected body so files win on the next sync.
    echo "rebody #$number: $slug ($status, $node)"
    run gh issue edit "$number" --body "$body"
  fi
  if [ "${ISSUE_STATE[$slug]}" = "OPEN" ] && [ "$want_state" = "CLOSED" ]; then
    echo "close #$number: $slug is done"
    run gh issue close "$number" --comment "Todo flipped to done in ${file}; closing the mirror."
  elif [ "${ISSUE_STATE[$slug]}" = "CLOSED" ] && [ "$want_state" = "OPEN" ]; then
    echo "reopen #$number: $slug is $status"
    run gh issue reopen "$number"
  fi
done

# Deleted todos: close their still-open mirror issues.
for slug in "${!ISSUE_NUM[@]}"; do
  if [ -z "${SEEN[$slug]:-}" ] && [ "${ISSUE_STATE[$slug]}" = "OPEN" ]; then
    number="${ISSUE_NUM[$slug]}"
    echo "close #$number: todo.$slug removed from $TODO_DIR"
    run gh issue close "$number" --comment "The canonical todo artefact was removed from ${TODO_DIR}; closing the mirror. Issues are never deleted."
  fi
done

# Inward: flag externally filed issues (no marker, no flag yet). Never
# import. The scan is materialised and validated like the projection. The
# triage comment posts BEFORE the exclusion label so a failed comment is
# retried on the next run rather than skipped forever.
UNMAPPED="$(gh issue list --state open --limit 500 --json number,labels,body \
  --jq '.[] | select((.labels | map(.name) | index("'"$LABEL"'") or index("'"$UNMAPPED_LABEL"'") | not))
             | select((.body // "" | test("cairn-todo: todo\\.")) | not)
             | (.number|tostring)')"
while IFS=$'\t' read -r number; do
  [ -n "$number" ] || continue
  echo "flag #$number: unmapped external issue"
  run gh issue comment "$number" --body "Triage: this issue has no matching native todo in ${TODO_DIR} (the canonical tracker, see AGENTS.md). If the work is accepted, create one with \`cairn todo new <slug> --node <id>\` carrying a \`gh:#${number}\` reference; this mirror never auto-imports issues."
  run gh issue edit "$number" --add-label "$UNMAPPED_LABEL"
done <<<"$UNMAPPED"

echo "sync complete"
