#!/bin/sh
# Git custom merge driver for map.json (dec.persistent-map-snapshot,
# dec.map-snapshot-merge-driver). map.json is a derived, rebuildable
# measurement record, not hand-edited content, so a conflict on it is
# resolved by regenerating the snapshot in a temporary worktree that
# reconstructs the merged tree, rather than attempting a textual/JSON merge.
#
# Registered via .gitattributes: `map.json merge=cairn-map`
# One-time setup (per clone), see docs/conventions.md (Git Hooks) or run
# `make install-hooks`:
#   git config merge.cairn-map.driver 'scripts/merge-map-json.sh %O %A %B %P'
#   git config merge.cairn-map.recursive binary
# The second line stops Git from reusing this driver to compute a virtual
# ancestor in a criss-cross merge (an internal merge step, unrelated to the
# real one); `binary` is Git's built-in driver that never guesses and always
# reports a conflict for that internal step.
#
# This driver handles plain `git merge` only: it requires the GITHEAD_<sha>
# environment variable, which Git sets before invoking merge drivers during a
# real merge. Rebase and cherry-pick conflicts on map.json (rare; not
# concurrent-PR contention) fall back to a normal Git conflict, resolved
# manually by re-running `cairn scan`.
#
# Git invokes this with the standard merge-driver placeholders:
#   $1 = %O  ancestor's version (temp file, unused: we regenerate wholesale)
#   $2 = %A  current branch's version (temp file); Git reads the merge
#            result back from this path, so we overwrite it on success
#   $3 = %B  other branch's version (temp file, unused)
#   $4 = %P  pathname of the file in the repository (e.g. "map.json")
set -eu

current=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
pathname="${4:-map.json}"
repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$repo_root"

temporary_tree=$(mktemp -d)
cleanup() {
    git worktree remove --force "$temporary_tree" >/dev/null 2>&1 || rm -rf "$temporary_tree"
}
trap cleanup 0
trap 'exit 1' HUP INT TERM

# Parse GITHEAD_<sha> environment variables to identify the other merge tip.
# Git sets these during a plain merge.
other_commit=""
candidates=""
for var in $(env | grep '^GITHEAD_[0-9a-f]*=' | cut -d= -f1); do
    sha=${var#GITHEAD_}
    if git rev-parse --verify "${sha}^{commit}" >/dev/null 2>&1; then
        candidates="${candidates}${sha}
"
    fi
done

unique_candidates=$(printf '%s' "$candidates" | sort -u | sed '/^$/d')
candidate_count=$(printf '%s\n' "$unique_candidates" | sed '/^$/d' | wc -l | tr -d ' ')

if [ "$candidate_count" = "1" ]; then
    other_commit="$unique_candidates"
elif [ "$candidate_count" -gt 1 ]; then
    echo "merge-map-json.sh: expected exactly one other merge tip, found $candidate_count (octopus merge not supported); leaving conflict" >&2
    exit 1
else
    echo "merge-map-json.sh: no GITHEAD_<sha> found (not a plain merge, e.g. rebase or cherry-pick); leaving conflict for manual resolution" >&2
    exit 1
fi

# Build a temporary worktree and perform the same merge there, so all other
# merged paths are present when the scanner regenerates the snapshot.
if ! git worktree add --quiet --detach "$temporary_tree" HEAD; then
    echo "merge-map-json.sh: could not create temporary merged worktree" >&2
    exit 1
fi
if ! git -C "$temporary_tree" -c merge.cairn-map.driver=true merge --no-commit --no-edit "$other_commit" >/dev/null 2>&1; then
    echo "merge-map-json.sh: could not reconstruct merged tree for $pathname" >&2
    exit 1
fi

# Prefer a freshly built local binary over a possibly stale installed one;
# fall back to an installed `cairn` if the local build is unavailable or
# fails (e.g. no Cargo toolchain, or a build error unrelated to the scan).
run_scan() {
    cd "$temporary_tree"
    if [ -f Cargo.toml ] && command -v cargo >/dev/null 2>&1; then
        if cargo run --release --bin cairn --quiet -- scan --strict; then
            return 0
        fi
    fi
    if command -v cairn >/dev/null 2>&1; then
        cairn scan --strict
        return $?
    fi
    return 127
}

if run_scan >&2 && [ -f "$temporary_tree/$pathname" ]; then
    cp "$temporary_tree/$pathname" "$current"
    exit 0
fi

echo "merge-map-json.sh: could not regenerate $pathname via 'cargo run --release --bin cairn -- scan --strict' or an installed 'cairn scan --strict'; leaving conflict for manual resolution" >&2
exit 1
