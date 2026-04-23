from __future__ import annotations

import json
import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import clean_near_end_samples


class CleanNearEndSamplesTests(unittest.TestCase):
    def test_runtime_command_targets_clean_binary(self) -> None:
        command = clean_near_end_samples.runtime_command("list", fmt="json")
        self.assertIn("mir-clean-near-end", command)
        self.assertEqual(command[-2:], ["--format", "json"])

    @mock.patch("clean_near_end_samples.run_runtime")
    def test_smoke_all_collects_every_family(self, run_runtime: mock.Mock) -> None:
        run_runtime.side_effect = [
            "[]",
            "[]",
            "[]",
            "[]",
            "[]",
            "{}",
            "{}",
        ]
        payload = json.loads(clean_near_end_samples.smoke_all("json"))
        self.assertIn("typing", payload)
        self.assertIn("order_handoff", payload)
        self.assertIn("model_check", payload)
        self.assertIn("modal", payload)


if __name__ == "__main__":
    unittest.main()
