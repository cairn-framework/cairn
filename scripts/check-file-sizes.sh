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

[ -d "$root/src" ] &&
find "$root/src" -type f -name '*.rs' -print | LC_ALL=C sort |
while IFS= read -r file; do
    check_file "$file" "// cairn:allow-large-module reason:" ""
done

# Non-Rust sources are gated wherever the blueprint claims them: derive the
# claimed directories from cairn.blueprint path declarations and walk the
# live filesystem underneath, so a freshly added file is caught before any
# snapshot regeneration. Vendored third-party assets are excluded.
blueprint="$root/cairn.blueprint"
if [ -f "$blueprint" ]; then
    # Claimed paths come from cairn.blueprint path declarations: single-line
    # scalars plus single-line and multi-line list forms. Any other layout
    # that starts a path declaration (bare keyword, unterminated list) fails
    # closed so a claim can never silently bypass the gate. Values are taken
    # only from quoted strings, so bracket characters inside path values are
    # never mistaken for list syntax.
    claims_file=${TMPDIR:-/tmp}/cairn-file-size-claims.$$
    if ! awk '
        # Strip a # comment that begins outside any quoted string, and fail
        # closed on backslash escapes inside quoted values: the blueprint
        # lexer decodes escapes, and mis-decoding a claim here would silently
        # ungate real files.
        function prepare(line,    i, c, out, instr) {
            out = ""; instr = 0
            for (i = 1; i <= length(line); i++) {
                c = substr(line, i, 1)
                if (c == "\\" && instr) { bad_escape = 1 }
                if (c == "\"") instr = !instr
                if (c == "#" && !instr) break
                out = out c
            }
            return out
        }
        function emit_strings(line) {
            while (match(line, /"[^"]*"/)) {
                print substr(line, RSTART + 1, RLENGTH - 2)
                line = substr(line, RSTART + RLENGTH)
            }
        }
        function closes_list(line) {
            gsub(/"[^"]*"/, "", line)
            return index(line, "]") > 0
        }
        {
            bad_escape = 0
            stripped = prepare($0)
            if (bad_escape && (inlist || stripped ~ /^[[:space:]]*path([[:space:]]|$)/)) {
                printf "check-file-sizes: escaped characters in path value are unsupported: %s\n", $0 > "/dev/stderr"
                bad = 1
                next
            }
        }
        inlist {
            emit_strings(stripped)
            if (closes_list(stripped)) inlist = 0
            next
        }
        stripped ~ /^[[:space:]]*path([[:space:]]|$)/ {
            if (stripped ~ /^[[:space:]]*path[[:space:]]+"[^"]*"[[:space:]]*$/) {
                emit_strings(stripped)
            } else if (stripped ~ /^[[:space:]]*path[[:space:]]+\[/) {
                emit_strings(stripped)
                if (!closes_list(stripped)) inlist = 1
            } else {
                printf "check-file-sizes: unparsable path declaration: %s\n", $0 > "/dev/stderr"
                bad = 1
            }
        }
        END {
            if (inlist) {
                print "check-file-sizes: unterminated path list in cairn.blueprint" > "/dev/stderr"
                bad = 1
            }
            exit bad
        }
    ' "$blueprint" > "$claims_file"; then
        rm -f "$claims_file"
        exit 1
    fi

    LC_ALL=C sort -u "$claims_file" |
    while IFS= read -r claim; do
        target="$root/${claim#./}"
        if [ -f "$target" ]; then
            case "$target" in
                */vendor/*) ;;
                *.js | *.css) printf '%s\n' "$target" ;;
            esac
        elif [ -d "$target" ]; then
            find "$target" -type f \( -name '*.js' -o -name '*.css' \) ! -path '*/vendor/*' -print
        fi
    done | LC_ALL=C sort -u |
    while IFS= read -r file; do
        # Skip gitignored artefacts (local build leftovers are not gated).
        if git -C "$root" check-ignore -q "$file" 2>/dev/null; then
            continue
        fi
        case "$file" in
            *.js) check_file "$file" "// cairn:allow-large-module reason:" "" ;;
            *.css) check_file "$file" "/* cairn:allow-large-module reason:" "*/" ;;
        esac
    done
    rm -f "$claims_file"
fi

[ ! -s "$failure_file" ]
