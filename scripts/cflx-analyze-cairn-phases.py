#!/usr/bin/env python3
"""Emit deterministic Conflux analysis for the sequential Cairn phase campaign."""

from __future__ import annotations

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CHANGE_RE = re.compile(r"^phase-(\d+)(?:\.(\d+))?(?:\.(\d+))?([a-z]?)-")


def sort_key(change_id: str) -> tuple[int, int, int, int, int, int, str, str]:
    """Order phases as (major, minor, minor_explicit, patch, patch_explicit, suffix).

    Handles two-level (phase-8.0-tests) and three-level (phase-7.6.0-tests) IDs.
    Missing minor/patch is treated as 0; missing suffix is the empty string.
    Explicit levels sort before implicit, enforcing test-first: `phase-N.0-tests`
    precedes `phase-N-<feature>`, and `phase-N.M.0-tests` precedes `phase-N.M-<feature>`.
    Non-matching ids sort after all matched phases, by id.
    """
    match = CHANGE_RE.match(change_id)
    if match is None:
        return (1, 0, 0, 0, 0, 1, "", change_id)
    major = int(match.group(1))
    minor_is_explicit = match.group(2) is not None
    minor = int(match.group(2)) if minor_is_explicit else 0
    minor_explicit_rank = 0 if minor_is_explicit else 1
    patch_is_explicit = match.group(3) is not None
    patch = int(match.group(3)) if patch_is_explicit else 0
    patch_explicit_rank = 0 if patch_is_explicit else 1
    suffix = match.group(4) or ""
    return (0, major, minor, minor_explicit_rank, patch, patch_explicit_rank, suffix, change_id)


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
