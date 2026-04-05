import argparse
import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_detached_loop as loop  # noqa: E402


class DetachedLoopPathTests(unittest.TestCase):
    def test_aggregate_artifact_path_uses_run_label_and_batch_summary_name(self) -> None:
        path = loop.aggregate_artifact_path(
            REPO_ROOT / "target" / "current-l2-detached",
            "smoke-run",
        )
        self.assertEqual(
            path,
            REPO_ROOT
            / "target"
            / "current-l2-detached"
            / "aggregates"
            / "smoke-run"
            / "batch-summary.detached.json",
        )

    def test_aggregate_artifact_path_reuses_run_label_validation(self) -> None:
        with self.assertRaises(ValueError):
            loop.aggregate_artifact_path(
                REPO_ROOT / "target" / "current-l2-detached",
                "bad/label",
            )

    def test_compare_aggregates_uses_run_label_paths(self) -> None:
        captured: list[tuple[Path, Path]] = []
        original_compare = loop.compare_aggregates

        def fake_compare(left: Path, right: Path) -> int:
            captured.append((left, right))
            return 0

        loop.compare_aggregates = fake_compare
        try:
            exit_code = loop.command_compare_aggregates(
                argparse.Namespace(
                    artifact_root=str(REPO_ROOT / "target" / "current-l2-detached"),
                    left_run_label="run-left",
                    right_run_label="run-right",
                )
            )
        finally:
            loop.compare_aggregates = original_compare

        self.assertEqual(exit_code, 0)
        self.assertEqual(
            captured,
            [
                (
                    REPO_ROOT
                    / "target"
                    / "current-l2-detached"
                    / "aggregates"
                    / "run-left"
                    / "batch-summary.detached.json",
                    REPO_ROOT
                    / "target"
                    / "current-l2-detached"
                    / "aggregates"
                    / "run-right"
                    / "batch-summary.detached.json",
                )
            ],
        )


if __name__ == "__main__":
    unittest.main()
