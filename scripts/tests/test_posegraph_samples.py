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
EXECUTABLE_ACCEPTED_SAMPLE_IDS = [
    "pose-04-no-split-frame-positive",
]
EXECUTABLE_VIOLATION_SAMPLE_IDS = [
    "pose-05-split-frame-negative",
]
PLANNED_ONLY_SAMPLE_IDS = [
    "pose-01-avatar-head-transform",
    "pose-02-anchored-object",
    "pose-03-sparkle-fallback-anchor",
    "pose-06-save-load-roundtrip",
    "pose-07-stale-anchor-after-membership-advance",
    "pose-08-anchor-switch-frontier-negative",
    "pose-09-stale-anchor-reacquire-required",
]


class PosegraphSamplesTests(unittest.TestCase):
    def test_list_contains_executable_and_planned_rows(self) -> None:
        rows = posegraph_samples.list_samples()
        by_id = {row["sample_id"]: row for row in rows}

        self.assertEqual([row["sample_id"] for row in rows], EXPECTED_SAMPLE_IDS)
        self.assertEqual(
            by_id["pose-04-no-split-frame-positive"]["current_status"],
            "executable",
        )
        self.assertEqual(
            by_id["pose-05-split-frame-negative"]["current_status"],
            "executable",
        )
        self.assertEqual(
            by_id["pose-04-no-split-frame-positive"]["expected_outcome"],
            {
                "terminal_outcome": "accepted",
                "pose_summary": "snapshot#avatar-017@17",
            },
        )
        self.assertEqual(
            by_id["pose-05-split-frame-negative"]["expected_outcome"],
            {
                "terminal_outcome": "violation_export",
                "violation_kind": "no_split_frame",
                "violation_contains": "snapshot mismatch",
            },
        )
        self.assertEqual(
            by_id["pose-04-no-split-frame-positive"]["package_input"],
            "samples/product-alpha1/posegraph/no-split-frame-positive/package.mir.json",
        )

    def test_matrix_reports_executable_and_planned_rows(self) -> None:
        result = posegraph_samples.matrix()

        self.assertEqual(result["sample_count"], 9)
        self.assertEqual(result["planned_count"], 7)
        self.assertEqual(result["executable_count"], 2)
        self.assertEqual(result["accepted_count"], 1)
        self.assertEqual(result["violation_count"], 1)
        self.assertEqual(result["planned_only_rows"], PLANNED_ONLY_SAMPLE_IDS)
        self.assertEqual(result["accepted_rows"], EXECUTABLE_ACCEPTED_SAMPLE_IDS)
        self.assertEqual(
            result["violation_rows"],
            EXECUTABLE_VIOLATION_SAMPLE_IDS,
        )
        self.assertEqual(result["matrix_status"], "mixed")
        self.assertFalse(result["workflow_ready"])

    def test_run_positive_pose_is_accepted(self) -> None:
        result = posegraph_samples.run_sample("pose-04-no-split-frame-positive")

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["execution_surface"], "helper_posegraph_runtime")
        self.assertEqual(result["terminal_outcome"], "accepted")
        self.assertEqual(result["target_pose_version"], 17)
        self.assertEqual(result["anchored_pose_version"], 17)
        self.assertEqual(result["pose_snapshot_ref"], "snapshot#avatar-017")
        self.assertTrue(result["outcome_matches_expected"])

    def test_run_negative_pose_exports_violation(self) -> None:
        result = posegraph_samples.run_sample("pose-05-split-frame-negative")

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["execution_surface"], "helper_posegraph_runtime")
        self.assertEqual(result["terminal_outcome"], "violation_export")
        self.assertEqual(result["violation_kind"], "no_split_frame")
        self.assertIn("snapshot mismatch", result["actual_violation_detail"])
        self.assertTrue(result["outcome_matches_expected"])

    def test_check_all_passes_when_rows_match_contract(self) -> None:
        result = posegraph_samples.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["planned"], PLANNED_ONLY_SAMPLE_IDS)
        self.assertEqual(result["accepted"], EXECUTABLE_ACCEPTED_SAMPLE_IDS)
        self.assertEqual(
            result["violations"],
            EXECUTABLE_VIOLATION_SAMPLE_IDS,
        )
        self.assertEqual(
            result["passed"],
            EXECUTABLE_ACCEPTED_SAMPLE_IDS + EXECUTABLE_VIOLATION_SAMPLE_IDS,
        )
        self.assertEqual(result["sample_count"], 9)
        self.assertFalse(result["workflow_ready"])

    def test_closeout_records_stop_lines_and_validation_floor(self) -> None:
        result = posegraph_samples.closeout()

        self.assertEqual(result["planned_sample_ids"], EXPECTED_SAMPLE_IDS)
        self.assertEqual(
            result["executable_sample_ids"],
            EXECUTABLE_ACCEPTED_SAMPLE_IDS + EXECUTABLE_VIOLATION_SAMPLE_IDS,
        )
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
        self.assertIn("planned-only: 7", pretty)
        self.assertIn("accepted rows: 1", pretty)
        self.assertIn("violation rows: 1", pretty)

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
