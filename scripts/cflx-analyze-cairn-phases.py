#!/usr/bin/env python3
"""Emit deterministic Conflux analysis for the sequential Cairn phase campaign."""

from __future__ import annotations

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CHANGE_RE = re.compile(r"^phase-(\d+)(?:\.(\d+))?([a-z]?)-")


def sort_key(change_id: str) -> tuple[int, int, int, int, str, str]:
    """Order phases as (major, minor, minor_explicit, suffix) for ids like phase-7.5a-foo.

    Missing minor is treated as 0; missing suffix is the empty string.
    When two ids tie on (major, minor), the one that declared minor explicitly
    sorts first. This enforces the test-first convention: `phase-N.0-tests`
    (explicit minor 0) precedes `phase-N-<feature>` (implicit minor 0).
    Non-matching ids sort after all matched phases, by id.
    """
    match = CHANGE_RE.match(change_id)
    if match is None:
        return (1, 0, 0, 1, "", change_id)
    major = int(match.group(1))
    minor_is_explicit = match.group(2) is not None
    minor = int(match.group(2)) if minor_is_explicit else 0
    minor_explicit_rank = 0 if minor_is_explicit else 1
    suffix = match.group(3) or ""
    return (0, major, minor, minor_explicit_rank, suffix, change_id)


def main() -> None:
    changes_dir = ROOT / "openspec" / "changes"
    change_ids = [
        path.name
        for path in changes_dir.iterdir()
        if path.is_dir() and path.name != "archive" and (path / "proposal.md").exists()
    ]
    order = sorted(change_ids, key=sort_key)

    dependencies: dict[str, list[str]] = {}
    previous_phase: str | None = None
    for change_id in order:
        if CHANGE_RE.match(change_id) is None:
            continue
        if previous_phase is not None:
            dependencies[change_id] = [previous_phase]
        previous_phase = change_id

    print(json.dumps({"order": order, "dependencies": dependencies}, separators=(",", ":")))


if __name__ == "__main__":
    main()
