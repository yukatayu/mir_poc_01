import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_transport as runner  # noqa: E402


class PracticalAlpha1TransportTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_transport_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "practical-alpha1-transport" for row in rows))

    def test_closeout_marks_stage_pa1_5_complete_once_all_rows_pass(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 7,
                "passed": [
                    "TR-A1-01",
                    "TR-A1-02",
                    "TR-A1-03",
                    "TR-A1-04",
                    "TR-A1-05",
                    "TR-A1-06",
                    "TR-A1-07",
                ],
                "failed": [],
                "transport_first_floor_complete": True,
                "transport_plan_boundary_present": True,
                "docker_row_complete": True,
                "stale_membership_negative_complete": True,
                "missing_capability_negative_complete": True,
                "missing_witness_negative_complete": True,
                "route_trace_complete": True,
                "auth_lane_complete": True,
                "stage_pa1_5_complete": True,
                "wan_federation_claimed": False,
                "save_load_claimed": False,
                "final_public_transport_abi_claimed": False,
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["stage_pa1_5_complete"])
        self.assertFalse(payload["wan_federation_claimed"])
        self.assertFalse(payload["save_load_claimed"])
        self.assertFalse(payload["final_public_transport_abi_claimed"])

    def test_run_sample_accepts_exact_expected_report(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        with mock.patch.object(runner, "run_path", return_value=expected):
            observed = runner.run_sample(row["sample_id"])
        self.assertEqual(observed, expected)

    def test_run_sample_rejects_report_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        drifted = dict(expected)
        drifted["terminal_outcome"] = "rejected"
        with mock.patch.object(runner, "run_path", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected transport report drift"):
                runner.run_sample(row["sample_id"])

    def test_check_all_requires_transport_plan_boundary(self) -> None:
        with mock.patch.object(
            runner,
            "run_sample",
            return_value={
                "sample_id": "TR-A1-01",
                "terminal_outcome": "accepted",
            },
        ):
            payload = runner.check_all()
        self.assertTrue(payload["transport_first_floor_complete"])
        self.assertFalse(payload["transport_plan_boundary_present"])

    def test_normalize_argv_promotes_direct_package_path_to_check(self) -> None:
        args = runner.normalize_argv(
            [
                "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
                "--format",
                "json",
            ]
        )
        self.assertEqual(
            args,
            [
                "--format",
                "json",
                "check",
                "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
            ],
        )


if __name__ == "__main__":
    unittest.main()
