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


if __name__ == "__main__":
    unittest.main()
