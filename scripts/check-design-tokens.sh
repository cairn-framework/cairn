#!/bin/sh
# Design-token conformance gate for the hand-authored web surfaces.
#
# Both the webui stylesheet (src/ui_assets/style.css) and the landing page
# (docs/landing/index.html) ride on the design-system tokens
# (docs/design-system/tokens.css) and must source every colour and rem-based
# size from a `var(--token)`, never a hardcoded literal. The stylesheet header,
# the landing's <style> header, CLAUDE.md, and AGENTS.md all carry this rule;
# biome's recommended rules cannot express it, so this gate enforces it
# deterministically.
#
# Fails (exit 1) when any target contains a hardcoded hex colour or a hardcoded
# rem value; passes (exit 0) otherwise. HTML and CSS comments are stripped first
# so hex/rem mentioned in prose does not trip the gate.
#
# By default both surfaces are checked. Override with a single explicit target
# via CAIRN_DESIGN_TOKENS_TARGET (used by tests/check_design_tokens.rs).
set -eu

# strip_comments OPEN CLOSE  (reads stdin, writes stdout): blank out every span
# between an OPEN and a CLOSE marker, multi-line aware, while preserving line
# numbers so reported line numbers match the original file.
strip_comments() {
    awk -v omark="$1" -v cmark="$2" '
    {
        line = $0
        out = ""
        while (length(line) > 0) {
            if (incomment) {
                idx = index(line, cmark)
                if (idx == 0) { line = "" }
                else { incomment = 0; line = substr(line, idx + length(cmark)) }
            } else {
                idx = index(line, omark)
                if (idx == 0) { out = out line; line = "" }
                else { out = out substr(line, 1, idx - 1); incomment = 1; line = substr(line, idx + length(omark)) }
            }
        }
        print out
    }'
}

# check_target FILE: scan one target, reporting offenders to stderr. Returns 1
# on a hardcoded hex colour or rem value, 0 otherwise.
check_target() {
    target=$1

    if [ ! -f "$target" ]; then
        printf '%s: design-token target not found\n' "$target" >&2
        return 1
    fi

    # Strip HTML comments then CSS comments (each multi-line aware), so a hex or
    # rem mentioned in prose, in either an .html or a .css surface, is exempt.
    # Comment markers are stripped document-wide; this assumes a literal `<!--`
    # never appears inside a <script> or <style> body (true for the surfaces
    # gated here). Revisit if a scanned page embeds one.
    stripped=$(strip_comments '<!--' '-->' <"$target" | strip_comments '/*' '*/')

    # Drop url(...) fragment references so an SVG ref such as `url(#grad)` is not
    # mistaken for a hex colour, while a real colour inside any other function
    # (e.g. `linear-gradient(#fff, #000)`) is still scanned. url() is matched
    # case-insensitively.
    scrubbed=$(printf '%s\n' "$stripped" | sed 's/[Uu][Rr][Ll]([^)]*)/url()/g')

    rc=0

    # Hex colour: a `#` token of exactly 3, 4, 6, or 8 hex digits, bounded by a
    # non-identifier character so longer identifiers (e.g. `#accent-gradient`)
    # and id selectors with non-hex names are not flagged. A bare all-hex id
    # selector (e.g. `#abc {`) is the one accepted edge; the surfaces use class
    # selectors and a hex-shaped value is far likelier to be a hardcoded colour.
    hex=$(printf '%s\n' "$scrubbed" | grep -nE '#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{4}|[0-9a-fA-F]{3})([^0-9A-Za-z_-]|$)' || true)
    if [ -n "$hex" ]; then
        printf '%s: hardcoded hex colour(s); use a design-system var(--token) instead:\n%s\n' "$target" "$hex" >&2
        rc=1
    fi

    # rem value: a (optionally negative) number ending in `rem`, accepting
    # leading-decimal forms (.5rem). Bounded on both sides by a non-identifier
    # character so identifiers that merely contain the text (e.g. `--rem-x`,
    # `.m1rem`, `1remington`) are not flagged, while a real `-1.5rem` still is.
    rem=$(printf '%s\n' "$scrubbed" | grep -nE '(^|[^0-9A-Za-z_-])-?(\.[0-9]+|[0-9]+(\.[0-9]+)?)rem([^0-9A-Za-z_-]|$)' || true)
    if [ -n "$rem" ]; then
        printf '%s: hardcoded rem value(s); use a design-system var(--token) instead:\n%s\n' "$target" "$rem" >&2
        rc=1
    fi

    return "$rc"
}

if [ -n "${CAIRN_DESIGN_TOKENS_TARGET:-}" ]; then
    targets=$CAIRN_DESIGN_TOKENS_TARGET
else
    targets='src/ui_assets/style.css docs/landing/index.html'
fi

failed=0
for target in $targets; do
    check_target "$target" || failed=1
done

exit "$failed"
