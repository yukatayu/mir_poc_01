import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_avatar as runner  # noqa: E402


class PracticalAlpha1AvatarTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_avatar_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "practical-alpha1-avatar" for row in rows))

    def test_closeout_marks_avatar_floor_complete_once_all_rows_match(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 3,
                "passed": ["AV-A1-01", "AV-A1-02", "AV-A1-03"],
                "failed": [],
                "avatar_preview_first_floor_complete": True,
                "placeholder_preview_present": True,
                "custom_runtime_preview_present": True,
                "unsupported_runtime_fallback_present": True,
                "native_execution_claimed": False,
                "final_avatar_abi_frozen": False,
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["avatar_preview_first_floor_complete"])
        self.assertTrue(payload["placeholder_preview_present"])
        self.assertTrue(payload["custom_runtime_preview_present"])
        self.assertTrue(payload["unsupported_runtime_fallback_present"])
        self.assertFalse(payload["native_execution_claimed"])
        self.assertFalse(payload["final_avatar_abi_frozen"])

    def test_check_all_requires_visible_fallback_row(self) -> None:
        reports = [
            {
                "sample_id": "AV-A1-01",
                "preview_kind": "placeholder_avatar_runtime_preview",
                "selected_representation": "static_capsule_placeholder",
            },
            {
                "sample_id": "AV-A1-02",
                "preview_kind": "custom_mir_avatar_runtime_preview",
                "selected_representation": "mir_humanoid_runtime_preview",
            },
            {
                "sample_id": "AV-A1-03",
                "preview_kind": "unsupported_runtime_visible_fallback_preview",
                "fallback_applied": False,
                "fallback_reason": None,
            },
        ]
        with mock.patch.object(runner, "run_sample", side_effect=reports):
            payload = runner.check_all()
        self.assertFalse(payload["unsupported_runtime_fallback_present"])
        self.assertTrue(payload["avatar_preview_first_floor_complete"])

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
        drifted["preview_kind"] = "wrong_preview_kind"
        with mock.patch.object(runner, "run_path", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected avatar preview report drift"):
                runner.run_sample(row["sample_id"])

    def test_normalize_argv_promotes_direct_package_path_to_check(self) -> None:
        args = runner.normalize_argv(
            [
                "samples/practical-alpha1/packages/av-a1-02-custom-mir-avatar-runtime",
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
                "samples/practical-alpha1/packages/av-a1-02-custom-mir-avatar-runtime",
            ],
        )


if __name__ == "__main__":
    unittest.main()
