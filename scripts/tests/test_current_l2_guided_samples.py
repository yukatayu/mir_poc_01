from __future__ import annotations

import io
import sys
import unittest
from contextlib import redirect_stderr
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import current_l2_guided_samples


class CurrentL2GuidedSamplesCompatTests(unittest.TestCase):
    @mock.patch("current_l2_guided_samples.clean_near_end_samples.main")
    def test_list_delegates_to_clean_suite(self, clean_main: mock.Mock) -> None:
        clean_main.return_value = 0
        exit_code = current_l2_guided_samples.main(["list"])
        self.assertEqual(exit_code, 0)
        clean_main.assert_called_once_with(["list"])

    def test_unsupported_legacy_command_is_rejected(self) -> None:
        stderr = io.StringIO()
        with redirect_stderr(stderr):
            exit_code = current_l2_guided_samples.main(["bundle", "problem1"])
        self.assertEqual(exit_code, 2)
        self.assertIn("clean-sample migration", stderr.getvalue())


if __name__ == "__main__":
    unittest.main()
