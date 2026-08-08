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

decode_base64() {
  if base64 --decode </dev/null >/dev/null 2>&1; then
    base64 --decode
  else
    base64 -D
  fi
}

# Read one top-level scalar from a todo's frontmatter. The parser intentionally
# handles the same simple YAML subset as Cairn's frontmatter parser.
frontmatter_scalar() {
  local key="$1"
  local file="$2"
  awk -v key="$key" '
    NR == 1 {
      line = $0
      sub(/\r$/, "", line)
      if (line == "---") {
        frontmatter = 1
        next
      }
    }
    frontmatter {
      line = $0
      sub(/\r$/, "", line)
      if (line == "---") exit
      if (line ~ /^[[:space:]]/) next
      prefix = key ":"
      if (index(line, prefix) == 1) {
        value = substr(line, length(prefix) + 1)
        sub(/^[[:space:]]*/, "", value)
        sub(/[[:space:]]*#.*$/, "", value)
        print value
        exit
      }
    }
  ' "$file"
}

# Print one list member per line for an inline list or an indented block list.
frontmatter_list() {
  local key="$1"
  local file="$2"
  awk -v key="$key" '
    function trim(value) {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
      return value
    }
    NR == 1 {
      line = $0
      sub(/\r$/, "", line)
      if (line == "---") {
        frontmatter = 1
        next
      }
    }
    frontmatter {
      line = $0
      sub(/\r$/, "", line)
      if (line == "---") exit
      if (line ~ /^[^[:space:]-][^:]*:/) {
        active = 0
        field = line
        sub(/:.*/, "", field)
        if (field != key) next
        value = line
        sub(/^[^:]*:[[:space:]]*/, "", value)
        sub(/[[:space:]]*#.*$/, "", value)
        if (value ~ /^\[.*\]$/) {
          value = substr(value, 2, length(value) - 2)
          count = split(value, items, ",")
          for (i = 1; i <= count; i++) {
            item = trim(items[i])
            if (item != "") print item
          }
        } else if (value == "") {
          active = 1
        }
        next
      }
      if (active && line ~ /^[[:space:]]*-[[:space:]]*/) {
        item = line
        sub(/^[[:space:]]*-[[:space:]]*/, "", item)
        item = trim(item)
        if (item != "") print item
      }
    }
  ' "$file"
}

resolve_ref() {
  local ref="$1"
  if [[ "$ref" == todo.* ]]; then
    local slug="${ref#todo.}"
    local number="${ISSUE_NUM[$slug]:-}"
    if [ -n "$number" ]; then
      printf '#%s' "$number"
      return
    fi
  fi
  printf '%s' "$ref"
}

# Sort by canonical todo references before converting them to issue numbers.
# This keeps rendering stable even when authored list order changes.
resolve_list() {
  local refs="$1"
  printf '%s\n' "$refs" |
    sort -u |
    while IFS= read -r ref; do
      [ -n "$ref" ] || continue
      resolve_ref "$ref"
      printf '\n'
    done |
    awk 'BEGIN { separator = "" } { printf "%s%s", separator, $0; separator = ", " } END { if (NR > 0) printf "\n" }'
}

render_relationships() {
  local file="$1"
  local parent
  local blocked_by
  local related
  parent="$(frontmatter_scalar parent "$file")"
  blocked_by="$(frontmatter_list blocked_by "$file")"
  related="$(frontmatter_list related "$file")"
  if [ -z "$parent" ] && [ -z "$blocked_by" ] && [ -z "$related" ]; then
    return
  fi

  printf '\n## Relationships\n\n'
  if [ -n "$parent" ]; then
    printf -- '- Sub-issue of: %s\n' "$(resolve_ref "$parent")"
  fi
  local blocked_links
  blocked_links="$(resolve_list "$blocked_by")"
  if [ -n "$blocked_links" ]; then
    printf -- '- Blocked by: %s\n' "$blocked_links"
  fi
  local related_links
  related_links="$(resolve_list "$related")"
  if [ -n "$related_links" ]; then
    printf -- '- Related: %s\n' "$related_links"
  fi
}

render_body() {
  local file="$1"
  local slug="$2"
  local status="$3"
  local node="$4"
  local include_relationships="$5"
  local frontmatter_end
  local todo_markdown
  frontmatter_end="$(awk '
    NR == 1 {
      line = $0
      sub(/\r$/, "", line)
      if (line == "---") { in_frontmatter = 1; next }
    }
    in_frontmatter {
      line = $0
      sub(/\r$/, "", line)
      if (line == "---") { print NR; exit }
    }
  ' "$file")"
  if [ -n "$frontmatter_end" ]; then
    todo_markdown="$(tail -n "+$((frontmatter_end + 1))" "$file"; printf '\034')"
  else
    todo_markdown="$(cat "$file"; printf '\034')"
  fi
  todo_markdown="${todo_markdown%$'\034'}"

  printf 'cairn-todo: todo.%s\nnode: %s\nstatus: %s\nartefact: %s\none-way mirror of a cairn todo; edits here are not read back, dec.task-tracking-authority\n' \
    "$slug" "$node" "$status" "$file"
  printf '%s' "$todo_markdown"
  if [ "$include_relationships" -eq 1 ]; then
    render_relationships "$file"
  fi
  printf '\034'
}

if [ "$DRY_RUN" -eq 0 ]; then
  gh label create "$LABEL" --description "Mirrored from meta/todos (read-only projection)" --color 5319e7 2>/dev/null || true
  gh label create "$UNMAPPED_LABEL" --description "Filed on GitHub without a native todo; needs triage" --color d93f0b 2>/dev/null || true
fi

declare -A ISSUE_NUM ISSUE_STATE ISSUE_TITLE ISSUE_BODY_B64

load_projection() {
  local projection
  if ! projection="$(gh issue list --label "$LABEL" --state all --limit 500 \
    --json number,state,title,body \
    --jq '.[] | [(.number|tostring), .state, .title,
          ((.body | capture("cairn-todo: todo\\.(?<s>[A-Za-z0-9._-]+)").s) // ""),
          ((.body // "") | @base64)] | @tsv')"; then
    echo "refusing to sync: could not load the GitHub todo inventory" >&2
    exit 1
  fi
  ISSUE_NUM=()
  ISSUE_STATE=()
  ISSUE_TITLE=()
  ISSUE_BODY_B64=()
  while IFS=$'\t' read -r number state title slug body_b64; do
    [ -n "$slug" ] || continue
    ISSUE_NUM["$slug"]="$number"
    ISSUE_STATE["$slug"]="$state"
    ISSUE_TITLE["$slug"]="$title"
    ISSUE_BODY_B64["$slug"]="$body_b64"
  done <<<"$projection"
}

# Phase 1: ensure every non-done todo has an issue number. New issues carry
# identity fields and canonical markdown without links; phase 2 fills links
# after the refreshed inventory makes every sibling resolvable.
load_projection
declare -A SEEN
for file in "$TODO_DIR"/todo.*.md; do
  [ -e "$file" ] || continue
  slug="$(basename "$file" .md)"
  slug="${slug#todo.}"
  SEEN["$slug"]=1
  status="$(sed -n 's/^status:[[:space:]]*//p' "$file" | head -1 | sed 's/\r$//')"
  node="$(sed -n 's/^node:[[:space:]]*//p' "$file" | head -1 | sed 's/\r$//')"
  title="$(sed -n 's/^#[[:space:]]\{1,\}//p' "$file" | head -1 | sed 's/\r$//')"
  [ -n "$title" ] || title="$slug"
  case "$status" in
    done) want_state="CLOSED" ;;
    open | in_progress | blocked) want_state="OPEN" ;;
    *) echo "skip $file: unknown status '$status'" >&2; continue ;;
  esac
  want_title="[todo] $title"
  base_body="$(render_body "$file" "$slug" "$status" "$node" 0)"
  base_body="${base_body%$'\034'}"
  phase_one_body="$base_body"
  if [ "$want_state" = "OPEN" ]; then
    phase_one_body="$(render_body "$file" "$slug" "$status" "$node" 1)"
    phase_one_body="${phase_one_body%$'\034'}"
  fi

  if [ -z "${ISSUE_NUM[$slug]:-}" ]; then
    if [ "$want_state" = "OPEN" ]; then
      echo "create: $slug ($status)"
      if [ "$DRY_RUN" -eq 1 ]; then
        echo "DRY: gh issue create --title '$want_title' --label $LABEL"
      else
        gh issue create --title "$want_title" --label "$LABEL" --body "$base_body" >/dev/null
      fi
    fi
    continue
  fi

  number="${ISSUE_NUM[$slug]}"
  issue_body=""
  if [ -n "${ISSUE_BODY_B64[$slug]:-}" ]; then
    issue_body="$(printf '%s' "${ISSUE_BODY_B64[$slug]}" | decode_base64; printf '\034')"
    issue_body="${issue_body%$'\034'}"
  fi
  if [ "$issue_body" != "$phase_one_body" ]; then
    echo "rebody #$number: $slug ($status, $node)"
    run gh issue edit "$number" --body "$phase_one_body"
  fi
  if [ "${ISSUE_TITLE[$slug]}" != "$want_title" ]; then
    echo "retitle #$number: $slug"
    run gh issue edit "$number" --title "$want_title"
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

# Phase 2: refresh the inventory after phase 1 creates, then rebody every
# existing mirror with issue-number relationship links.
load_projection
for file in "$TODO_DIR"/todo.*.md; do
  [ -e "$file" ] || continue
  slug="$(basename "$file" .md)"
  slug="${slug#todo.}"
  [ -n "${ISSUE_NUM[$slug]:-}" ] || continue
  status="$(sed -n 's/^status:[[:space:]]*//p' "$file" | head -1 | sed 's/\r$//')"
  node="$(sed -n 's/^node:[[:space:]]*//p' "$file" | head -1 | sed 's/\r$//')"
  case "$status" in
    done) want_state="CLOSED"; include_relationships=0 ;;
    open | in_progress | blocked) want_state="OPEN"; include_relationships=1 ;;
    *) continue ;;
  esac
  body="$(render_body "$file" "$slug" "$status" "$node" "$include_relationships")"
  body="${body%$'\034'}"
  number="${ISSUE_NUM[$slug]}"
  issue_body=""
  if [ -n "${ISSUE_BODY_B64[$slug]:-}" ]; then
    issue_body="$(printf '%s' "${ISSUE_BODY_B64[$slug]}" | decode_base64; printf '\034')"
    issue_body="${issue_body%$'\034'}"
  fi
  if [ "$issue_body" != "$body" ]; then
    echo "rebody #$number: $slug ($status, $node)"
    run gh issue edit "$number" --body "$body"
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
