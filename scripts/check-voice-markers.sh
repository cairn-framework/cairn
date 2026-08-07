#!/usr/bin/env bash
# Voice screen for absorbed or authored prose: em-dashes fail, AI-marker
# vocabulary and smart typography warn. Scope is changed files (or the
# paths passed as arguments), mirroring the pre-commit em-dash hook, so
# legacy prose never fires. Exceptions: archive/ and docs/research/.
# Usage: scripts/check-voice-markers.sh [file ...]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
targets="$(mktemp)"
trap 'rm -f "$targets"' EXIT
if [ "$#" -gt 0 ]; then
  printf '%s\n' "$@" > "$targets"
else
  { git diff --name-only HEAD; git ls-files --others --exclude-standard; } \
    | sort -u > "$targets"
fi
python3 - "$targets" <<'PY'
import re, sys
FAIL_CH = {"\u2014": "em-dash"}
WARN_CH = {"\u2013": "en-dash", "\u2018": "curly quote", "\u2019": "curly quote",
           "\u201c": "curly quote", "\u201d": "curly quote",
           "\u2026": "ellipsis", "\u00a0": "no-break space"}
LEX = re.compile(r"\b(delve|leverag(e|ed|es|ing)|utili[sz]e[sd]?|seamless(ly)?"
                 r"|game.?changer|dive into|landscape of|tapestry|testament to"
                 r"|it is important to note|in conclusion|harness the power)\b", re.I)
fails = warns = 0
with open(sys.argv[1], encoding="utf-8") as listing:
    targets = listing.read().splitlines()
for raw in targets:
    f = raw.strip()
    if not f or not f.endswith((".md", ".html")):
        continue
    if f.startswith(("archive/", "docs/research/")):
        continue
    try:
        text = open(f, encoding="utf-8").read()
    except (FileNotFoundError, IsADirectoryError, UnicodeDecodeError):
        continue
    for i, line in enumerate(text.splitlines(), 1):
        for ch, name in FAIL_CH.items():
            if ch in line:
                print(f"FAIL {f}:{i}: {name}: {line.strip()[:90]}"); fails += 1
        for ch, name in WARN_CH.items():
            if ch in line:
                print(f"warn {f}:{i}: {name}: {line.strip()[:90]}"); warns += 1
        m = LEX.search(line)
        if m:
            print(f"warn {f}:{i}: marker '{m.group(0)}': {line.strip()[:90]}"); warns += 1
print(f"voice screen: {fails} fail, {warns} warn")
sys.exit(1 if fails else 0)
PY
