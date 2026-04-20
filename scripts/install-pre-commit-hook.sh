#!/bin/sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
hook_path=$(git rev-parse --git-path hooks/pre-commit)

mkdir -p "$(dirname "$hook_path")"

cat > "$hook_path" <<'HOOK'
#!/bin/sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

echo "pre-commit: running cargo fmt --check"
if ! cargo fmt --check; then
    echo "pre-commit: cargo fmt --check failed" >&2
    exit 1
fi

echo "pre-commit: running cairn hook all"
if ! scripts/cairn-hook-all.sh; then
    echo "pre-commit: cairn hook all failed" >&2
    exit 1
fi
HOOK

chmod +x "$hook_path"

echo "installed executable pre-commit hook at $hook_path"
