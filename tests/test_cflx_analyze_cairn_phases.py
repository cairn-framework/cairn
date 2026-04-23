"""Unit tests for the Cairn phase ordering helper."""

import importlib.util
from pathlib import Path
import unittest


SCRIPT_PATH = Path(__file__).resolve().parents[1] / "scripts" / "cflx-analyze-cairn-phases.py"
SPEC = importlib.util.spec_from_file_location("cflx_analyze_cairn_phases", SCRIPT_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
sort_key = MODULE.sort_key


class SortKeyTests(unittest.TestCase):
    def test_orders_integer_decimal_and_suffix_phases(self) -> None:
        change_ids = [
            "phase-8-summariser",
            "phase-7.5b-cleansing-splits",
            "phase-7.5a-test-fortification",
            "phase-7-mcp",
            "phase-8.0-tests",
            "phase-10-distribution",
            "phase-9-brownfield",
        ]

        ordered = sorted(change_ids, key=sort_key)

        self.assertEqual(
            ordered,
            [
                "phase-7-mcp",
                "phase-7.5a-test-fortification",
                "phase-7.5b-cleansing-splits",
                "phase-8.0-tests",
                "phase-8-summariser",
                "phase-9-brownfield",
                "phase-10-distribution",
            ],
        )

    def test_test_first_pre_phase_sorts_before_feature_phase(self) -> None:
        ordered = sorted(
            [
                "phase-9-brownfield",
                "phase-9.0-tests",
                "phase-10.0-tests",
                "phase-10-distribution",
            ],
            key=sort_key,
        )
        self.assertEqual(
            ordered,
            [
                "phase-9.0-tests",
                "phase-9-brownfield",
                "phase-10.0-tests",
                "phase-10-distribution",
            ],
        )

    def test_non_matching_ids_sort_after_phase_ids(self) -> None:
        ordered = sorted(["misc-change", "phase-2-kernel"], key=sort_key)
        self.assertEqual(ordered, ["phase-2-kernel", "misc-change"])


if __name__ == "__main__":
    unittest.main()
