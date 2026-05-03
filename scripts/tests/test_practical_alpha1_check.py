import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_check as runner  # noqa: E402


class PracticalAlpha1CheckTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_checker_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "practical-alpha1-checker" for row in rows))

    def test_closeout_keeps_checker_only_boundary(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["first_checker_floor_complete"])
        self.assertFalse(payload["spec18_typed_checking_complete"])
        self.assertFalse(payload["public_cli_frozen"])
        self.assertFalse(payload["runtime_plan_emitted"])
        self.assertFalse(payload["run_local_claimed"])
        self.assertFalse(payload["run_docker_claimed"])
        self.assertIn(
            "python3 scripts/practical_alpha1_check.py check-all --format json",
            payload["validation_floor"],
        )

    def test_run_sample_accepts_exact_expected_report(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        with mock.patch.object(runner, "check_path", return_value=expected):
            observed = runner.run_sample(row["sample_id"])
        self.assertEqual(observed, expected)

    def test_run_sample_rejects_report_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        drifted = dict(expected)
        drifted["verdict"] = "accepted"
        with mock.patch.object(runner, "check_path", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected checker report drift"):
                runner.run_sample(row["sample_id"])

    def test_check_all_collects_failures(self) -> None:
        def fake_run(sample_id: str) -> dict:
            if sample_id == "CHK-VAR-02":
                raise RuntimeError("variance mismatch")
            return {"sample_id": sample_id}

        with mock.patch.object(runner, "run_sample", side_effect=fake_run):
            payload = runner.check_all()
        self.assertFalse(payload["first_checker_floor_complete"])
        self.assertEqual(payload["failed"][0]["sample_id"], "CHK-VAR-02")

    def test_normalize_argv_promotes_direct_package_path_to_check(self) -> None:
        args = runner.normalize_argv(
            ["samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid", "--format", "json"]
        )
        self.assertEqual(
            args,
            [
                "--format",
                "json",
                "check",
                "samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid",
            ],
        )

    def test_normalize_argv_hoists_root_format_before_known_subcommand(self) -> None:
        args = runner.normalize_argv(["check-all", "--format", "json"])
        self.assertEqual(args, ["--format", "json", "check-all"])


if __name__ == "__main__":
    unittest.main()
