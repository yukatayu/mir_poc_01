import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_visualization_samples as runner  # noqa: E402


class AlphaVisualizationSamplesTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_closeout_reports_subset_and_stage_e_incomplete(self) -> None:
        payload = runner.closeout()
        self.assertEqual(
            payload["implemented_rows"],
            [
                "VIS-01",
                "VIS-02",
                "VIS-03",
                "VIS-05",
                "VIS-06",
                "VIS-07",
                "VIS-08",
                "VIS-10",
                "VIS-11",
            ],
        )
        self.assertIn("VIS-04", payload["planned_only_rows"])
        self.assertIn("VIS-09", payload["planned_only_rows"])
        self.assertIn("VIS-12", payload["planned_only_rows"])
        self.assertIn(
            "python3 scripts/alpha_visualization_samples.py check-all --format json",
            payload["validation_floor"],
        )
        self.assertFalse(payload["stage_e_complete"])

    def test_list_samples_exposes_subset_runner_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "VIS-01",
                "VIS-02",
                "VIS-03",
                "VIS-05",
                "VIS-06",
                "VIS-07",
                "VIS-08",
                "VIS-10",
                "VIS-11",
            ],
        )
        self.assertTrue(all(row["family"] == "alpha-visualization" for row in rows))

    def test_run_sample_accepts_exact_sidecar(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        sidecar = json.loads(self.sidecar_path(row["expected_sidecar"]).read_text())
        with mock.patch.object(runner, "_build_sample_report", return_value=sidecar):
            report = runner.run_sample(row["sample_id"])
        self.assertEqual(report, sidecar)

    def test_validate_expected_fields_rejects_trace_before_attach(self) -> None:
        row = runner._implemented_row("VIS-10")
        report = {
            "sample_id": "VIS-10",
            "terminal_outcome": "accepted",
            "evidence_summary": {
                "pre_attach_trace_count": 1,
                "post_attach_trace_count": 2,
                "rejected_attach_trace_count": 0,
                "trace_mode": "on_demand",
            },
        }
        with self.assertRaisesRegex(RuntimeError, "pre_attach_trace_count"):
            runner._validate_expected_fields("VIS-10", row, report)

    def test_validate_expected_fields_rejects_missing_place_graph(self) -> None:
        row = runner._implemented_row("VIS-02")
        report = {
            "sample_id": "VIS-02",
            "terminal_outcome": "accepted",
            "evidence_summary": {
                "place_count": 0,
                "place_ids": [],
                "participant_place_ids": [],
            },
        }
        with self.assertRaisesRegex(RuntimeError, "place_count"):
            runner._validate_expected_fields("VIS-02", row, report)

    def test_implemented_row_rejects_planned_only_witness_timeline(self) -> None:
        with self.assertRaisesRegex(ValueError, "VIS-04"):
            runner._implemented_row("VIS-04")

    def test_validate_expected_fields_rejects_missing_membership_timeline(self) -> None:
        row = runner._implemented_row("VIS-05")
        report = {
            "sample_id": "VIS-05",
            "terminal_outcome": "accepted",
            "evidence_summary": {
                "timeline_entry_count": 0,
                "membership_epochs": [],
                "membership_principals": [],
            },
        }
        with self.assertRaisesRegex(RuntimeError, "timeline_entry_count"):
            runner._validate_expected_fields("VIS-05", row, report)

    def test_validate_expected_fields_rejects_over_retention(self) -> None:
        row = runner._implemented_row("VIS-11")
        report = {
            "sample_id": "VIS-11",
            "terminal_outcome": "accepted",
            "evidence_summary": {
                "observed_retention_scopes": ["helper_local_ephemeral", "session_cache"],
                "over_retention_detected": True,
                "rejected_reason_refs": ["retention_scope_widened"],
            },
        }
        with self.assertRaisesRegex(RuntimeError, "over_retention_detected"):
            runner._validate_expected_fields("VIS-11", row, report)


if __name__ == "__main__":
    unittest.main()
