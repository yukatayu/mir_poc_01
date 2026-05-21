from __future__ import annotations

import importlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

try:
    projection_boundary_samples = importlib.import_module("projection_boundary_samples")
except ModuleNotFoundError:
    projection_boundary_samples = None


EXPECTED_SAMPLE_IDS = [
    "proj-01-server-client-target-manifest",
    "proj-01-packet-boundary-schema",
    "proj-01-ffi-boundary-schema",
    "proj-01-manifest-provider-compatibility",
]

EXPECTED_ACCEPTED_COMPATIBILITY_ID = "compat-accepted-renderer-view"
EXPECTED_REJECTED_COMPATIBILITY_ID = "compat-rejected-missing-capability"


class ProjectionBoundarySamplesTests(unittest.TestCase):
    def test_helper_module_exists(self) -> None:
        self.assertIsNotNone(
            projection_boundary_samples,
            "scripts/projection_boundary_samples.py must exist for P-PROJ-01",
        )

    def test_projection_root_and_matrix_exist(self) -> None:
        sample_root = (
            Path(__file__).resolve().parents[2]
            / "samples"
            / "product-alpha1"
            / "projection"
        )
        matrix_path = sample_root / "matrix.json"

        self.assertTrue(sample_root.exists(), "projection sample root is missing")
        self.assertTrue(matrix_path.exists(), "projection matrix.json is missing")

    def test_matrix_file_declares_planned_only_rows(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        matrix = json.loads(projection_boundary_samples.MATRIX_PATH.read_text())

        self.assertEqual(matrix["family"], "projection_boundary")
        self.assertEqual(matrix["current_status"], "planned_only")
        self.assertEqual(
            [row["sample_id"] for row in matrix["rows"]],
            EXPECTED_SAMPLE_IDS,
        )
        self.assertEqual(
            matrix["compatibility_inventory"]["accepted_rows"],
            [EXPECTED_ACCEPTED_COMPATIBILITY_ID],
        )
        self.assertEqual(
            matrix["compatibility_inventory"]["rejected_rows"],
            [EXPECTED_REJECTED_COMPATIBILITY_ID],
        )

    def test_list_contains_planned_rows(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        rows = projection_boundary_samples.list_samples()

        self.assertEqual([row["sample_id"] for row in rows], EXPECTED_SAMPLE_IDS)
        self.assertTrue(all(row["current_status"] == "planned_only" for row in rows))
        self.assertTrue(all(row["workflow_ready"] is False for row in rows))

    def test_check_all_reports_planned_only_family(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        result = projection_boundary_samples.check_all()

        self.assertEqual(result["sample_count"], 4)
        self.assertEqual(result["planned"], EXPECTED_SAMPLE_IDS)
        self.assertEqual(result["failed"], [])
        self.assertFalse(result["workflow_ready"])
        self.assertIn(EXPECTED_ACCEPTED_COMPATIBILITY_ID, result["accepted_rows"])
        self.assertIn(EXPECTED_REJECTED_COMPATIBILITY_ID, result["rejected_rows"])

    def test_run_rejects_planned_only_row(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        result = projection_boundary_samples.run_sample(
            "proj-01-server-client-target-manifest"
        )

        self.assertEqual(result["current_status"], "planned_only")
        self.assertEqual(result["terminal_outcome"], "planned_only")
        self.assertIn("later projection realization package", result["rejection_reason"])
        self.assertIn("no generated server/client binary", result["stop_lines"])

    def test_closeout_records_validation_floor_and_stop_lines(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        result = projection_boundary_samples.closeout()

        self.assertEqual(result["planned_sample_ids"], EXPECTED_SAMPLE_IDS)
        self.assertFalse(result["workflow_ready"])
        self.assertEqual(
            result["accepted_compatibility_rows"],
            [EXPECTED_ACCEPTED_COMPATIBILITY_ID],
        )
        self.assertEqual(
            result["rejected_compatibility_rows"],
            [EXPECTED_REJECTED_COMPATIBILITY_ID],
        )
        self.assertIn(
            "python3 scripts/projection_boundary_samples.py check-all --format json",
            result["validation_floor"],
        )
        self.assertIn("no LLVM/backend execution", result["stop_lines"])

    def test_validate_rows_rejects_missing_root(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        with tempfile.TemporaryDirectory() as tmp:
            sample_root = Path(tmp)
            rows = [
                {
                    "sample_id": "proj-missing",
                    "root_name": "missing-root",
                    "current_status": "planned_only",
                    "representative_artifact": "missing-root/README.md",
                }
            ]

            errors = projection_boundary_samples.validate_rows(sample_root, rows)

        self.assertEqual(len(errors), 2)
        self.assertEqual(errors[0]["kind"], "missing_root")
        self.assertEqual(errors[1]["kind"], "missing_representative_artifact")

    def test_pretty_formats_check_all_summary(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        pretty = projection_boundary_samples.format_pretty(
            projection_boundary_samples.check_all()
        )

        self.assertIn("CHECK-ALL SUMMARY", pretty)
        self.assertIn("planned-only: 4", pretty)

    def test_normalize_argv_hoists_root_format_before_subcommand(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        args = projection_boundary_samples.normalize_argv(
            ["check-all", "--format", "json"]
        )

        self.assertEqual(args, ["--format", "json", "check-all"])

    def test_normalize_argv_promotes_bare_sample_id_to_run(self) -> None:
        if projection_boundary_samples is None:
            self.fail("projection boundary helper missing")

        args = projection_boundary_samples.normalize_argv(
            ["proj-01-packet-boundary-schema", "--format", "json"]
        )

        self.assertEqual(
            args,
            ["--format", "json", "run", "proj-01-packet-boundary-schema"],
        )


if __name__ == "__main__":
    unittest.main()
