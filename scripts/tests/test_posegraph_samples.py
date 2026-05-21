from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import posegraph_samples


EXPECTED_SAMPLE_IDS = [
    "pose-01-avatar-head-transform",
    "pose-02-anchored-object",
    "pose-03-sparkle-fallback-anchor",
    "pose-04-no-split-frame-positive",
    "pose-05-split-frame-negative",
    "pose-06-save-load-roundtrip",
    "pose-07-stale-anchor-after-membership-advance",
    "pose-08-anchor-switch-frontier-negative",
    "pose-09-stale-anchor-reacquire-required",
]


class PosegraphSamplesTests(unittest.TestCase):
    def test_list_contains_all_planned_rows(self) -> None:
        rows = posegraph_samples.list_samples()

        self.assertEqual([row["sample_id"] for row in rows], EXPECTED_SAMPLE_IDS)
        self.assertTrue(all(row["current_status"] == "planned_only" for row in rows))

    def test_matrix_reports_planned_only_family(self) -> None:
        result = posegraph_samples.matrix()

        self.assertEqual(result["sample_count"], 9)
        self.assertEqual(result["planned_count"], 9)
        self.assertEqual(result["executable_count"], 0)
        self.assertEqual(result["matrix_status"], "planned_only")
        self.assertFalse(result["workflow_ready"])

    def test_run_positive_pose_rejects_as_planned_only(self) -> None:
        result = posegraph_samples.run_sample("pose-04-no-split-frame-positive")

        self.assertEqual(result["current_status"], "planned_only")
        self.assertEqual(result["terminal_outcome"], "planned_only")
        self.assertIn("P-POSE-02", result["rejection_reason"])

    def test_run_negative_pose_rejects_as_planned_only(self) -> None:
        result = posegraph_samples.run_sample("pose-05-split-frame-negative")

        self.assertEqual(result["current_status"], "planned_only")
        self.assertEqual(result["terminal_outcome"], "planned_only")
        self.assertIn("P-POSE-02", result["rejection_reason"])

    def test_check_all_passes_when_planned_roots_exist(self) -> None:
        result = posegraph_samples.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["planned"], EXPECTED_SAMPLE_IDS)
        self.assertEqual(result["sample_count"], 9)
        self.assertFalse(result["workflow_ready"])

    def test_closeout_records_stop_lines_and_validation_floor(self) -> None:
        result = posegraph_samples.closeout()

        self.assertEqual(result["planned_sample_ids"], EXPECTED_SAMPLE_IDS)
        self.assertEqual(
            result["current_no_split_frame_reading"],
            "same_client_same_observation_snapshot",
        )
        self.assertIn("no PoseGraph runtime completion yet", result["stop_lines"])
        self.assertIn(
            "python3 scripts/posegraph_samples.py check-all --format json",
            result["validation_floor"],
        )
        self.assertFalse(result["workflow_ready"])

    def test_validate_rows_rejects_missing_root(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            sample_root = Path(tmp)
            rows = [
                {
                    "sample_id": "pose-missing",
                    "root_name": "missing-root",
                    "current_status": "planned_only",
                    "representative_source": "missing-root/missing-root.mir",
                }
            ]

            errors = posegraph_samples.validate_rows(sample_root, rows)

        self.assertEqual(len(errors), 2)
        self.assertEqual(errors[0]["kind"], "missing_root")
        self.assertEqual(errors[1]["kind"], "missing_representative_source")

    def test_pretty_formats_check_all_summary(self) -> None:
        pretty = posegraph_samples.format_pretty(posegraph_samples.check_all())

        self.assertIn("POSEGRAPH CHECK-ALL", pretty)
        self.assertIn("planned-only: 9", pretty)

    def test_normalize_argv_hoists_root_format_before_known_subcommand(self) -> None:
        args = posegraph_samples.normalize_argv(["check-all", "--format", "json"])

        self.assertEqual(args, ["--format", "json", "check-all"])

    def test_normalize_argv_promotes_bare_sample_id_to_run(self) -> None:
        args = posegraph_samples.normalize_argv(
            ["pose-04-no-split-frame-positive", "--format", "json"]
        )

        self.assertEqual(
            args,
            ["--format", "json", "run", "pose-04-no-split-frame-positive"],
        )


if __name__ == "__main__":
    unittest.main()
