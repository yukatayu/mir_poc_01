import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_capability_checker as checker  # noqa: E402


class CurrentL2CapabilityCheckerTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_main_reports_matched_capability_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e13_malformed_capability_strengthening",
                    "expected_static": {
                        "checked_reason_codes": [
                            {
                                "kind": "capability_strengthens",
                                "from_capability": "read",
                                "to_capability": "write",
                            },
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "primary",
                                "successor": "mirror",
                            },
                        ]
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "stable-clusters-only",
                        "reason_codes": [
                            {
                                "kind": "capability_strengthens",
                                "from_capability": "read",
                                "to_capability": "write",
                            },
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "primary",
                                "successor": "mirror",
                            },
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: capability_strengthening_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("capability_strengthens", output)
        self.assertNotIn("declared_target_missing", output)

    def test_main_reports_missing_fixture_capability_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e20_malformed_late_capability_strengthening",
                    "expected_static": {"checked_reason_codes": []},
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "stable-clusters-only",
                        "reason_codes": [
                            {
                                "kind": "capability_strengthens",
                                "from_capability": "read",
                                "to_capability": "write",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: fixture_capability_rows_missing", output)
        self.assertIn("capability_strengthens", output)

    def test_main_reports_out_of_scope_when_cluster_rows_absent(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e4_malformed_lineage",
                    "expected_static": {
                        "checked_reason_codes": [
                            {
                                "kind": "lineage_assertion_edge_mismatch",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ]
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "stable-clusters-only",
                        "reason_codes": [
                            {
                                "kind": "lineage_assertion_edge_mismatch",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: out_of_scope", output)


if __name__ == "__main__":
    unittest.main()
