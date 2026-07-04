import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_check  # noqa: E402
import practical_alpha1_save_load as runner  # noqa: E402


class PracticalAlpha1SaveLoadTests(unittest.TestCase):
    def test_list_samples_matches_save_load_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(
            all(row["family"] == "practical-alpha1-save-load" for row in rows)
        )

    def test_closeout_reuses_invalid_distributed_cut_checker_guard(self) -> None:
        payload = runner.closeout()
        self.assertEqual(payload["checker_guard_refs"], ["CHK-CUT-01"])
        self.assertTrue(payload["invalid_distributed_cut_row_actualized"])
        self.assertTrue(payload["stage_pa1_7_complete"])

    def test_check_all_requires_checker_guard(self) -> None:
        original = practical_alpha1_check.run_sample

        def fake_run_sample(sample_id: str):
            if sample_id == "CHK-CUT-01":
                raise RuntimeError("checker guard missing")
            return original(sample_id)

        practical_alpha1_check.run_sample = fake_run_sample
        try:
            payload = runner.check_all()
        finally:
            practical_alpha1_check.run_sample = original

        self.assertFalse(payload["local_save_load_first_floor_complete"])
        self.assertFalse(payload["invalid_distributed_cut_guard_present"])

    def test_run_sample_sl_a1_03_returns_checker_backed_preflight_reject(self) -> None:
        payload = runner.run_sample("SL-A1-03")
        self.assertEqual(payload["terminal_outcome"], "rejected_invalid_distributed_cut_preflight")
        self.assertEqual(payload["checker_guard_refs"], ["CHK-CUT-01"])
        self.assertEqual(
            payload["source_checker_report"]["sample_id"],
            "CHK-CUT-01",
        )
        self.assertEqual(
            payload["source_checker_report"]["rejected_kind"],
            "orphan_receive",
        )
        self.assertFalse(payload["distributed_save_load_claimed"])

    def test_repo_cli_arg_relativizes_repo_owned_package_dir(self) -> None:
        package_dir = REPO_ROOT / "samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume"
        self.assertEqual(
            runner.repo_cli_arg(package_dir),
            "samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume",
        )

    def test_repo_cli_arg_keeps_external_path_absolute(self) -> None:
        external = Path("/tmp/mirrorea-external-save-load-package")
        self.assertEqual(runner.repo_cli_arg(external), str(external))

    def test_build_runtime_save_load_report_uses_repo_relative_package_arg(self) -> None:
        completed = subprocess_completed(stdout=json.dumps({"status": "ok"}))
        package_dir = REPO_ROOT / "samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume"
        with mock.patch.object(runner.subprocess, "run", return_value=completed) as mocked_run:
            payload = runner._build_runtime_save_load_report(package_dir)
        self.assertEqual(payload, {"status": "ok"})
        argv = mocked_run.call_args.args[0]
        self.assertEqual(
            argv[-1],
            "samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume",
        )
        self.assertFalse(any(str(arg).startswith(f"{runner.REPO_ROOT}/") for arg in argv))

    def test_checker_preflight_branch_delegates_to_checker_without_runtime_subprocess(self) -> None:
        row = runner.IMPLEMENTED_ROWS[2]
        package_dir = REPO_ROOT / row["package_dir"]
        checker_payload = {
            "verdict": "rejected",
            "rejected_rows": [{"kind": "orphan_receive"}],
            "diagnostics": [{"message": "orphan receive"}],
            "package_id": "pkg#cut",
            "sample_id": "CHK-CUT-01",
            "checker_scope": "practical-alpha1-checker-floor",
            "surface_kind": "practical_alpha1_checker_report",
        }
        with mock.patch.object(
            practical_alpha1_check,
            "check_path",
            return_value=checker_payload,
        ) as mocked_check_path, mock.patch.object(
            runner,
            "_build_runtime_save_load_report",
            side_effect=AssertionError("runtime branch should not run"),
        ):
            payload = runner._report_for_row(row, package_dir)

        self.assertEqual(payload["terminal_outcome"], "rejected_invalid_distributed_cut_preflight")
        self.assertEqual(mocked_check_path.call_args.args, (package_dir,))


def subprocess_completed(stdout: str) -> object:
    return mock.Mock(stdout=stdout)


if __name__ == "__main__":
    unittest.main()
