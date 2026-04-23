#!/bin/sh
set -eu

root=${CAIRN_FILE_SIZE_ROOT:-$(git rev-parse --show-toplevel)}
limit=500
failed=0

for file in $(find "$root/src" -type f -name '*.rs' | sort); do
    lines=$(wc -l < "$file" | tr -d ' ')
    [ "$lines" -le "$limit" ] && continue

    first_nonblank=$(awk 'NF { print; exit }' "$file")
    case "$first_nonblank" in
        "// cairn:allow-large-module reason:"*)
            rest=${first_nonblank#"// cairn:allow-large-module reason:"}
            trimmed=$(printf '%s' "$rest" | sed 's/^[[:space:]]*//')
            if [ -n "$trimmed" ]; then
                continue
            fi
            printf '%s: %s lines. missing non-empty allow-list reason\n' "$file" "$lines" >&2
            failed=1
            ;;
        *)
            printf '%s: %s lines exceeds %s without allow-list\n' "$file" "$lines" "$limit" >&2
            failed=1
            ;;
    esac
done

exit "$failed"
