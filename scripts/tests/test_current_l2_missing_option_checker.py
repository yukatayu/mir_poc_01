import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_missing_option_checker as checker  # noqa: E402


class CurrentL2MissingOptionCheckerTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_main_reports_matched_missing_option_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e16_malformed_missing_chain_head_option",
                    "expected_static": {
                        "checked_reason_codes": [
                            {
                                "kind": "missing_chain_head_option",
                                "head": "doc_ref",
                                "scope": "doc_ref",
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
                                "kind": "missing_chain_head_option",
                                "head": "doc_ref",
                                "scope": "doc_ref",
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
        self.assertIn("cluster: missing_option_structure_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("missing_chain_head_option", output)
        self.assertNotIn("declared_target_missing", output)

    def test_main_reports_missing_fixture_missing_option_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e17_malformed_missing_predecessor_option",
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
                                "kind": "missing_predecessor_option",
                                "option": "mirror",
                                "scope": "doc_ref",
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
        self.assertIn("status: fixture_missing_option_rows_missing", output)
        self.assertIn("missing_predecessor_option", output)

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
                                "predecessor": "primary",
                                "successor": "mirror",
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
                                "predecessor": "primary",
                                "successor": "mirror",
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
