from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import current_l2_lean_sample_sync as sync


class CurrentL2LeanSampleSyncTests(unittest.TestCase):
    def test_sanitize_module_name_handles_numeric_prefix(self) -> None:
        self.assertEqual(
            sync.sanitize_module_name("01_authorized_declassification"),
            "CleanNearEnd.S01.Authorized.Declassification",
        )

    def test_theorem_name_is_stable(self) -> None:
        self.assertEqual(
            sync.theorem_name("03_broken_mutex_counterexample"),
            "s03_broken_mutex_counterexample__alpha_ready_subject",
        )

    def test_render_lean_stub_mentions_theorem(self) -> None:
        text = sync.render_lean_stub("01_authorized_declassification")
        self.assertIn("alpha_ready_subject", text)
        self.assertIn("namespace CleanNearEnd", text)


if __name__ == "__main__":
    unittest.main()
