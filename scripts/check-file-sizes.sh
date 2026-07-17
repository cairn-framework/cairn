#!/bin/sh
set -eu

root=${CAIRN_FILE_SIZE_ROOT:-$(git rev-parse --show-toplevel)}
limit=500
failure_file=${TMPDIR:-/tmp}/cairn-file-size-failure.$$
: > "$failure_file"
trap 'rm -f "$failure_file"' 0 HUP INT TERM

check_file() {
    target=$1
    prefix=$2
    suffix=$3

    lines=$(wc -l < "$target" | tr -d ' ')
    [ "$lines" -le "$limit" ] && return 0

    first_nonblank=$(awk 'NF { print; exit }' "$target")
    case "$first_nonblank" in
        "$prefix"*"$suffix")
            rest=${first_nonblank#"$prefix"}
            [ -n "$suffix" ] && rest=${rest%"$suffix"}
            trimmed=$(printf '%s' "$rest" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')
            if [ -n "$trimmed" ]; then
                return 0
            fi
            printf '%s: %s lines. missing non-empty allow-list reason\n' "$target" "$lines" >&2
            printf '%s\n' failed > "$failure_file"
            ;;
        *)
            printf '%s: %s lines exceeds %s without allow-list\n' "$target" "$lines" "$limit" >&2
            printf '%s\n' failed > "$failure_file"
            ;;
    esac
}

find "$root/src" -type f -name '*.rs' -print | LC_ALL=C sort |
while IFS= read -r file; do
    check_file "$file" "// cairn:allow-large-module reason:" ""
done

ui_assets="$root/src/ui_assets"
if [ -d "$ui_assets" ]; then
    find "$ui_assets" -type f -name '*.js' ! -path '*/vendor/*' -print | LC_ALL=C sort |
    while IFS= read -r file; do
        check_file "$file" "// cairn:allow-large-module reason:" ""
    done

    find "$ui_assets" -type f -name '*.css' ! -path '*/vendor/*' -print | LC_ALL=C sort |
    while IFS= read -r file; do
        check_file "$file" "/* cairn:allow-large-module reason:" "*/"
    done
fi

[ ! -s "$failure_file" ]
