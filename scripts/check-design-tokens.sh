#!/bin/sh
# Design-token conformance gate for the webui stylesheet.
#
# src/ui_assets/style.css rides on the design-system tokens
# (docs/design-system/tokens.css) and must source every colour and rem-based
# size from a `var(--token)`, never a hardcoded literal. The stylesheet header,
# CLAUDE.md, and AGENTS.md all carry this rule; biome's recommended rules cannot
# express it, so this gate enforces it deterministically.
#
# Fails (exit 1) when the target stylesheet contains a hardcoded hex colour or a
# hardcoded rem value; passes (exit 0) otherwise. CSS comments are stripped first
# so hex/rem mentioned in prose does not trip the gate.
#
# Override the target for testing with CAIRN_DESIGN_TOKENS_TARGET.
set -eu

target=${CAIRN_DESIGN_TOKENS_TARGET:-src/ui_assets/style.css}

if [ ! -f "$target" ]; then
    printf '%s: design-token target not found\n' "$target" >&2
    exit 1
fi

# Strip /* ... */ comments (including multi-line) while preserving line numbers,
# so reported line numbers match the original file.
stripped=$(awk '
{
    line = $0
    out = ""
    while (length(line) > 0) {
        if (incomment) {
            idx = index(line, "*/")
            if (idx == 0) { line = "" }
            else { incomment = 0; line = substr(line, idx + 2) }
        } else {
            idx = index(line, "/*")
            if (idx == 0) { out = out line; line = "" }
            else { out = out substr(line, 1, idx - 1); incomment = 1; line = substr(line, idx + 2) }
        }
    }
    print out
}' "$target")

failed=0

# Drop url(...) fragment references so an SVG ref such as `url(#grad)` is not
# mistaken for a hex colour, while a real colour inside any other function
# (e.g. `linear-gradient(#fff, #000)`) is still scanned. url() is matched
# case-insensitively.
scrubbed=$(printf '%s\n' "$stripped" | sed 's/[Uu][Rr][Ll]([^)]*)/url()/g')

# Hex colour: a `#` token of exactly 3, 4, 6, or 8 hex digits, bounded by a
# non-identifier character so longer identifiers (e.g. `#accent-gradient`) and
# id selectors with non-hex names are not flagged. A bare all-hex id selector
# (e.g. `#abc {`) is the one accepted edge; the webui uses class selectors and
# a hex-shaped value is far likelier to be a hardcoded colour.
hex=$(printf '%s\n' "$scrubbed" | grep -nE '#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{4}|[0-9a-fA-F]{3})([^0-9A-Za-z_-]|$)' || true)
if [ -n "$hex" ]; then
    printf '%s: hardcoded hex colour(s); use a design-system var(--token) instead:\n%s\n' "$target" "$hex" >&2
    failed=1
fi

# rem value: a (optionally negative) number ending in `rem`, accepting
# leading-decimal forms (.5rem). Bounded on both sides by a non-identifier
# character so identifiers that merely contain the text (e.g. `--rem-x`,
# `.m1rem`, `1remington`) are not flagged, while a real `-1.5rem` still is.
rem=$(printf '%s\n' "$scrubbed" | grep -nE '(^|[^0-9A-Za-z_-])-?(\.[0-9]+|[0-9]+(\.[0-9]+)?)rem([^0-9A-Za-z_-]|$)' || true)
if [ -n "$rem" ]; then
    printf '%s: hardcoded rem value(s); use a design-system var(--token) instead:\n%s\n' "$target" "$rem" >&2
    failed=1
fi

exit "$failed"
