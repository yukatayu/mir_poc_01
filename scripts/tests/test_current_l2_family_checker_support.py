import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_family_checker_support as support  # noqa: E402


class CurrentL2FamilyCheckerSupportTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_run_family_checker_reports_matched_filtered_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_static": {
                        "checked_reason_codes": [
                            {"kind": "alpha_kind", "value": 1},
                            {"kind": "beta_kind", "value": 2},
                        ]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "stable-clusters-only",
                        "reason_codes": [
                            {"kind": "alpha_kind", "value": 1},
                            {"kind": "beta_kind", "value": 2},
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_cluster",
                    description="alpha cluster helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="stable-clusters-only",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_cluster", output)
        self.assertIn("status: matched", output)
        self.assertIn("alpha_kind", output)
        self.assertNotIn("beta_kind", output)

    def test_run_family_checker_reports_missing_fixture_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {"expected_static": {"checked_reason_codes": []}},
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_cluster",
                    description="alpha cluster helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="stable-clusters-only",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: fixture_alpha_rows_missing", output)

    def test_run_family_checker_reports_out_of_scope(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_static": {
                        "checked_reason_codes": [{"kind": "beta_kind", "value": 2}]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes": [{"kind": "beta_kind", "value": 2}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_cluster",
                    description="alpha cluster helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="stable-clusters-only",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: out_of_scope", output)

    def test_run_family_checker_reports_scope_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_static": {
                        "checked_reason_codes": [{"kind": "alpha_kind", "value": 1}]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "wrong-floor",
                        "reason_codes": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_cluster",
                    description="alpha cluster helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="stable-clusters-only",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: scope_mismatch", output)
        self.assertIn("expected_reason_codes_scope: stable-clusters-only", output)
        self.assertIn("artifact_reason_codes_scope: wrong-floor", output)

    def test_run_family_checker_reports_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_static": {
                        "checked_reason_codes": [{"kind": "alpha_kind", "value": 1}]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "stable-clusters-only",
                        "reason_codes": [{"kind": "alpha_kind", "value": 2}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_cluster",
                    description="alpha cluster helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="stable-clusters-only",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: mismatch", output)


if __name__ == "__main__":
    unittest.main()
