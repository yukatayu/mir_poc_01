import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_run_local as runner  # noqa: E402


class PracticalAlpha1RunLocalTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_active_runtime_rows_cover_operational_alpha05_negatives(self) -> None:
        self.assertEqual(
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
            ["RUN-01", "RUN-02", "RUN-03", "RUN-04"],
        )

    def test_list_samples_matches_runtime_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "practical-alpha1-local-runtime" for row in rows))

    def test_closeout_keeps_runtime_plan_boundary_explicit(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 2,
                "passed": ["RUN-01", "RUN-02"],
                "failed": [],
                "local_runtime_first_floor_complete": True,
                "runtime_plan_boundary_present": True,
                "run_docker_claimed": False,
                "package_hotplug_claimed": False,
                "save_load_claimed": False,
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["local_runtime_first_floor_complete"])
        self.assertTrue(payload["runtime_plan_boundary_present"])
        self.assertFalse(payload["run_docker_claimed"])
        self.assertFalse(payload["package_hotplug_claimed"])
        self.assertFalse(payload["save_load_claimed"])
        self.assertIn(
            "python3 scripts/practical_alpha1_run_local.py check-all --format json",
            payload["validation_floor"],
        )

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
            with self.assertRaisesRegex(RuntimeError, "expected runtime report drift"):
                runner.run_sample(row["sample_id"])

    def test_check_all_collects_failures(self) -> None:
        def fake_run(sample_id: str) -> dict:
            if sample_id == "RUN-02":
                raise RuntimeError("membership mismatch")
            return {
                "package_id": sample_id,
                "runtime_plan_scope": "practical-alpha1-runtime-plan-floor",
                "runtime_plan_emitted": False,
            }

        with mock.patch.object(runner, "run_sample", side_effect=fake_run):
            payload = runner.check_all()
        self.assertFalse(payload["local_runtime_first_floor_complete"])
        self.assertEqual(payload["failed"][0]["sample_id"], "RUN-02")
        self.assertFalse(payload["runtime_plan_boundary_present"])

    def test_check_all_requires_runtime_plan_boundary_in_reports(self) -> None:
        with mock.patch.object(
            runner,
            "run_sample",
            return_value={"package_id": "RUN-01", "runtime_plan_emitted": False},
        ):
            payload = runner.check_all()
        self.assertTrue(payload["local_runtime_first_floor_complete"])
        self.assertFalse(payload["runtime_plan_boundary_present"])

    def test_normalize_argv_promotes_direct_package_path_to_check(self) -> None:
        args = runner.normalize_argv(
            ["samples/practical-alpha1/packages/run-01-local-sugoroku", "--format", "json"]
        )
        self.assertEqual(
            args,
            [
                "--format",
                "json",
                "check",
                "samples/practical-alpha1/packages/run-01-local-sugoroku",
            ],
        )

    def test_normalize_argv_hoists_root_format_before_known_subcommand(self) -> None:
        args = runner.normalize_argv(["check-all", "--format", "json"])
        self.assertEqual(args, ["--format", "json", "check-all"])


if __name__ == "__main__":
    unittest.main()
