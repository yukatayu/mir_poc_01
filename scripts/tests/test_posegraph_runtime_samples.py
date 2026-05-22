from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/posegraph_runtime_samples.py", *args, "--format", "json"],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"helper failed ({completed.returncode}): {completed.stderr}\n{completed.stdout}"
        )
    return json.loads(completed.stdout)


class PosegraphRuntimeSamplesTests(unittest.TestCase):
    def test_matrix_reports_runtime_posegraph_row_set(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["family"], "full_system_v1_posegraph_runtime")
        self.assertEqual(payload["sample_count"], 9)
        self.assertEqual(payload["executable_count"], 8)
        self.assertEqual(payload["planned_count"], 1)
        self.assertEqual(payload["accepted_count"], 4)
        self.assertEqual(payload["violation_count"], 1)
        self.assertEqual(payload["runtime_rejection_count"], 3)
        self.assertEqual(payload["validation_errors"], [])

    def test_no_split_frame_positive_preserves_switch_and_fallback_state(self) -> None:
        payload = _run_helper("run", "pose-04-no-split-frame-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "Accepted")
        self.assertEqual(payload["actual"]["pose_snapshot_frontier"], "snapshot#avatar-017")
        self.assertEqual(payload["actual"]["anchor_switch_sequences"], [41])
        self.assertEqual(payload["actual"]["fallback_chain_lengths"], [2])

    def test_split_frame_negative_exports_violation(self) -> None:
        payload = _run_helper("run", "pose-05-split-frame-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "ViolationExport")
        self.assertEqual(payload["actual"]["violation_kind"], "no_split_frame")

    def test_stale_anchor_negative_exports_runtime_rejection(self) -> None:
        payload = _run_helper("run", "pose-07-stale-anchor-after-membership-advance")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "RuntimeRejection")
        self.assertEqual(
            payload["actual"]["rejection_code"], "stale_anchor_membership_epoch"
        )

    def test_reacquire_negative_exports_runtime_rejection(self) -> None:
        payload = _run_helper("run", "pose-09-stale-anchor-reacquire-required")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "RuntimeRejection")
        self.assertEqual(payload["actual"]["rejection_code"], "reacquire_required")
        self.assertEqual(payload["actual"]["reacquire_required"], ["object#hat-017"])

    def test_check_all_passes_every_executable_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(len(payload["passed"]), 8)
        self.assertEqual(payload["planned"], ["pose-06-save-load-roundtrip"])

    def test_closeout_reports_only_planned_rows(self) -> None:
        payload = _run_helper("closeout")

        self.assertEqual(payload["planned_sample_ids"], ["pose-06-save-load-roundtrip"])
        self.assertEqual(len(payload["executable_sample_ids"]), 8)


if __name__ == "__main__":
    unittest.main()
