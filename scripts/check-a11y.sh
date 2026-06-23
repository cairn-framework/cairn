#!/bin/sh
# Accessibility static-audit gate for the hand-authored web surfaces.
#
# The webui shell (src/ui_assets/index.html), the preact app
# (src/ui_assets/app.js), and the landing page (docs/landing/index.html) are
# hand-authored markup. A handful of WCAG-aligned accessibility invariants are
# statically decidable from the source text. biome's recommended rules do not
# express them (and cannot see markup inside JS template literals), so this gate
# enforces them deterministically, mirroring scripts/check-design-tokens.sh.
#
# Element-level checks run on every surface:
#   - WCAG 1.1.1: every <img> carries an alt attribute.
#   - WCAG 2.4.3: no positive tabindex (1+) overrides natural focus order.
# Document-level checks run only on full HTML documents (those with an <html>
# root), so JS/htm fragments are exempt from them:
#   - WCAG 3.1.1: <html> declares a lang.
#   - WCAG 2.4.2: the document has a <title>.
#   - WCAG 1.4.4: the viewport meta does not disable pinch zoom
#     (user-scalable=no or maximum-scale=1).
#
# Fails (exit 1) on any violation; passes (exit 0) otherwise. HTML and CSS/JS
# block comments are stripped first so markup mentioned in prose does not trip
# the gate.
#
# JS // single-line comments are not stripped (mirroring check-design-tokens.sh),
# so avoid writing markup-like examples inside // comments on a gated surface.
#
# By default all three surfaces are checked. Override with a single explicit
# target via CAIRN_A11Y_TARGET (used by tests/check_a11y.rs).
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
# on any accessibility violation, 0 otherwise.
check_target() {
    target=$1

    if [ ! -f "$target" ]; then
        printf '%s: a11y target not found\n' "$target" >&2
        return 1
    fi

    # Strip HTML comments then CSS/JS block comments (each multi-line aware), so
    # markup mentioned in prose (an old <img>, a tabindex note) is exempt.
    stripped=$(strip_comments '<!--' '-->' <"$target" | strip_comments '/*' '*/')

    rc=0

    # WCAG 1.1.1: every <img> must carry an alt attribute. Tag-aware (not
    # line-aware) by splitting on `>`, so an <img> whose attributes span several
    # lines is judged as one tag; <image> (SVG) is excluded by requiring a
    # non-letter after `img`.
    img_bad=$(printf '%s\n' "$stripped" | awk '
        BEGIN { RS = ">" }
        {
            s = $0
            while (match(s, /<img([^a-zA-Z]|$)/)) {
                tag = substr(s, RSTART)
                if (tag !~ /[[:space:]]alt[[:space:]]*=/) {
                    gsub(/[[:space:]]+/, " ", tag)
                    print "  " tag ">"
                }
                s = substr(s, RSTART + 4)
            }
        }
    ')
    if [ -n "$img_bad" ]; then
        printf '%s: <img> without an alt attribute (WCAG 1.1.1):\n%s\n' "$target" "$img_bad" >&2
        rc=1
    fi

    # WCAG 2.4.3: a positive tabindex (a value beginning with 1-9) overrides the
    # natural focus order. tabindex="0" and tabindex="-1" are legitimate.
    tab=$(printf '%s\n' "$stripped" | grep -nE '(^|[[:space:]]|<)tabindex[[:space:]]*=[[:space:]]*["'\''"]?[1-9]' || true)
    if [ -n "$tab" ]; then
        printf '%s: positive tabindex overrides focus order; use 0 or -1 (WCAG 2.4.3):\n%s\n' "$target" "$tab" >&2
        rc=1
    fi

    # Document-level checks apply only to full HTML documents. A JS/htm fragment
    # has no <html> root and is exempt from them.
    if printf '%s\n' "$stripped" | grep -qi '<html'; then
        # WCAG 3.1.1: the <html> element must declare a lang attribute.
        if ! printf '%s\n' "$stripped" | grep -qiE '<html[^>]*[[:space:]]lang[[:space:]]*='; then
            printf '%s: <html> is missing a lang attribute (WCAG 3.1.1)\n' "$target" >&2
            rc=1
        fi

        # WCAG 2.4.2: the document must have a <title> element.
        if ! printf '%s\n' "$stripped" | grep -qiE '<title[[:space:]]*>'; then
            printf '%s: document has no <title> element (WCAG 2.4.2)\n' "$target" >&2
            rc=1
        fi

        # WCAG 1.4.4: the viewport meta must not disable pinch zoom. Inside the
        # quoted content="..." value, user-scalable=no and maximum-scale=1 are
        # unquoted, so no quote handling is needed here.
        zoom=$(printf '%s\n' "$stripped" | grep -niE 'user-scalable[[:space:]]*=[[:space:]]*no|maximum-scale[[:space:]]*=[[:space:]]*1(\.0+)?([^.0-9]|$)' || true)
        if [ -n "$zoom" ]; then
            printf '%s: viewport disables pinch zoom (WCAG 1.4.4):\n%s\n' "$target" "$zoom" >&2
            rc=1
        fi
    fi

    return "$rc"
}

if [ -n "${CAIRN_A11Y_TARGET:-}" ]; then
    targets=$CAIRN_A11Y_TARGET
else
    targets='src/ui_assets/index.html docs/landing/index.html src/ui_assets/app.js'
fi

failed=0
for target in $targets; do
    check_target "$target" || failed=1
done

exit "$failed"
