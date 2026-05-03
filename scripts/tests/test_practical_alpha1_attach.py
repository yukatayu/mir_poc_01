import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_attach as runner  # noqa: E402


class PracticalAlpha1AttachTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_hotplug_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "practical-alpha1-hotplug" for row in rows))
        self.assertIn("HP-A1-04B1", [row["sample_id"] for row in rows])
        self.assertIn("HP-A1-04B2", [row["sample_id"] for row in rows])
        self.assertIn("HP-A1-06", [row["sample_id"] for row in rows])
        self.assertIn("HP-A1-07", [row["sample_id"] for row in rows])

    def test_closeout_marks_stage_pa1_4_complete_once_detach_contract_exists(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 9,
                "passed": [
                    "HP-A1-01",
                    "HP-A1-02",
                    "HP-A1-03",
                    "HP-A1-04",
                    "HP-A1-05",
                    "HP-A1-04B1",
                    "HP-A1-04B2",
                    "HP-A1-06",
                    "HP-A1-07",
                ],
                "failed": [],
                "package_hotplug_first_floor_complete": True,
                "hotplug_plan_boundary_present": True,
                "object_attach_seam_present": True,
                "object_attach_claimed": False,
                "freshness_negative_complete": True,
                "detach_minimal_contract_complete": True,
                "stage_pa1_4_complete": True,
                "run_docker_claimed": False,
                "save_load_claimed": False,
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["package_hotplug_first_floor_complete"])
        self.assertTrue(payload["hotplug_plan_boundary_present"])
        self.assertTrue(payload["object_attach_seam_present"])
        self.assertFalse(payload["object_attach_claimed"])
        self.assertTrue(payload["freshness_negative_complete"])
        self.assertTrue(payload["detach_minimal_contract_complete"])
        self.assertTrue(payload["stage_pa1_4_complete"])
        self.assertFalse(payload["run_docker_claimed"])
        self.assertFalse(payload["save_load_claimed"])

    def test_check_all_requires_hotplug_plan_boundary_in_reports(self) -> None:
        with mock.patch.object(
            runner,
            "run_sample",
            return_value={
                "package_id": "HP-A1-01",
                "terminal_outcome": "accepted",
                "hotplug_runtime_report": {"request": {"operation_kind": "attach"}},
            },
        ):
            payload = runner.check_all()
        self.assertTrue(payload["package_hotplug_first_floor_complete"])
        self.assertFalse(payload["hotplug_plan_boundary_present"])

    def test_check_all_requires_detach_contract_row_for_stage_pa1_4_complete(self) -> None:
        reports = [
            {
                "sample_id": "HP-A1-04B1",
                "reason_family": "membership_freshness",
                "hotplug_plan_scope": "practical-alpha1-hotplug-plan-floor",
                "hotplug_runtime_report": {"request": {"operation_kind": "attach"}},
            },
            {
                "sample_id": "HP-A1-04B2",
                "reason_family": "witness",
                "hotplug_plan_scope": "practical-alpha1-hotplug-plan-floor",
                "hotplug_runtime_report": {"request": {"operation_kind": "attach"}},
            },
            {
                "sample_id": "HP-A1-06",
                "reason_family": None,
                "hotplug_plan_scope": "practical-alpha1-hotplug-plan-floor",
                "object_attach_preview": {"preview_kind": "runtime_package_avatar_preview"},
                "hotplug_runtime_report": {"request": {"operation_kind": "attach"}},
            },
        ]
        with mock.patch.object(runner, "run_sample", side_effect=reports * 3):
            payload = runner.check_all()
        self.assertFalse(payload["detach_minimal_contract_complete"])
        self.assertFalse(payload["stage_pa1_4_complete"])

    def test_check_all_recognizes_deferred_detach_contract_row(self) -> None:
        reports = []
        for sample_id in [
            "HP-A1-01",
            "HP-A1-02",
            "HP-A1-03",
            "HP-A1-04",
            "HP-A1-05",
            "HP-A1-04B1",
            "HP-A1-04B2",
            "HP-A1-06",
        ]:
            report = {
                "sample_id": sample_id,
                "reason_family": None,
                "hotplug_plan_scope": "practical-alpha1-hotplug-plan-floor",
                "hotplug_runtime_report": {"request": {"operation_kind": "attach"}},
            }
            if sample_id == "HP-A1-04B1":
                report["reason_family"] = "membership_freshness"
            if sample_id == "HP-A1-04B2":
                report["reason_family"] = "witness"
            if sample_id == "HP-A1-06":
                report["object_attach_preview"] = {
                    "preview_kind": "runtime_package_avatar_preview"
                }
            reports.append(report)
        reports.append(
            {
                "sample_id": "HP-A1-07",
                "terminal_outcome": "deferred_detach_minimal_contract",
                "reason_family": "detach_contract",
                "hotplug_plan_scope": "practical-alpha1-hotplug-plan-floor",
                "hotplug_runtime_report": {"request": {"operation_kind": "detach"}},
            }
        )
        with mock.patch.object(runner, "run_sample", side_effect=reports):
            payload = runner.check_all()
        self.assertTrue(payload["detach_minimal_contract_complete"])
        self.assertTrue(payload["stage_pa1_4_complete"])

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
            with self.assertRaisesRegex(RuntimeError, "expected hotplug report drift"):
                runner.run_sample(row["sample_id"])

    def test_normalize_argv_promotes_direct_package_path_to_check(self) -> None:
        args = runner.normalize_argv(
            [
                "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
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
                "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
            ],
        )


if __name__ == "__main__":
    unittest.main()
