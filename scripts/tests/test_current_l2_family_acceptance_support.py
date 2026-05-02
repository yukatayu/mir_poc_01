import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_family_acceptance_support as support  # noqa: E402


class CurrentL2FamilyAcceptanceSupportTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_run_family_acceptance_reports_matched_filtered_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_acceptance": {
                        "verdict": "valid",
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [
                            {"kind": "alpha_kind", "value": 1},
                            {"kind": "beta_kind", "value": 2},
                        ],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [
                            {"kind": "alpha_kind", "value": 1},
                            {"kind": "beta_kind", "value": 2},
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_acceptance_cluster", output)
        self.assertIn("status: matched", output)
        self.assertIn("alpha_kind", output)
        self.assertNotIn("beta_kind", output)

    def test_run_family_acceptance_reports_missing_fixture_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(fixture_path, {"sample_id": "LIF-02"})
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: fixture_alpha_rows_missing", output)

    def test_run_family_acceptance_reports_out_of_scope(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_rows": [{"kind": "beta_kind", "value": 2}]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [{"kind": "beta_kind", "value": 2}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: out_of_scope", output)

    def test_run_family_acceptance_reports_scope_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "wrong-floor",
                        "acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: scope_mismatch", output)
        self.assertIn("expected_acceptance_scope: alpha-acceptance-floor", output)
        self.assertIn("artifact_acceptance_scope: wrong-floor", output)

    def test_run_family_acceptance_reports_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [{"kind": "alpha_kind", "value": 2}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: mismatch", output)

    def test_run_family_acceptance_ignores_reason_codes(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                        "reason_codes_scope": "wrong-floor",
                        "reason_codes": [{"kind": "alpha_kind", "value": 999}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertNotIn("999", output)

    def test_run_family_acceptance_ignores_snapshot_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [{"kind": "alpha_kind", "value": 1}],
                        "snapshot_scope": "wrong-floor",
                        "snapshot_rows": [{"kind": "alpha_kind", "value": 997}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_acceptance_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_acceptance_cluster",
                    description="alpha acceptance helper",
                    kinds={"alpha_kind"},
                    missing_status="fixture_alpha_rows_missing",
                    expected_scope="alpha-acceptance-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertNotIn("997", output)


if __name__ == "__main__":
    unittest.main()
