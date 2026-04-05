import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_try_rollback_structural_checker as checker  # noqa: E402


class CurrentL2TryRollbackStructuralCheckerTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_main_reports_matched_first_tranche_findings(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e23_malformed_try_fallback_missing_fallback_body",
                    "program": {
                        "kind": "Program",
                        "body": [
                            {
                                "kind": "TryFallback",
                                "body": [],
                                "fallback_body": [],
                            }
                        ],
                    },
                    "expected_static": {
                        "verdict": "malformed",
                        "checked_try_rollback_structural_verdict": "findings_present",
                        "checked_try_rollback_structural_findings": [
                            {
                                "subject_kind": "TryFallback",
                                "finding_kind": "missing_fallback_body",
                            }
                        ],
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "checker_core": {
                        "static_verdict": "malformed",
                        "reasons": ["try fallback body must not be empty"],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: try_rollback_structural_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("missing_fallback_body", output)

    def test_main_reports_missing_fixture_expectation_when_findings_exist(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e24_malformed_atomic_cut_fallback_placement",
                    "program": {
                        "kind": "Program",
                        "body": [
                            {
                                "kind": "TryFallback",
                                "body": [],
                                "fallback_body": [{"kind": "AtomicCut"}],
                            }
                        ],
                    },
                    "expected_static": {
                        "verdict": "malformed",
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "checker_core": {
                        "static_verdict": "malformed",
                        "reasons": ["atomic cut may not appear inside fallback body"],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: fixture_try_rollback_expectation_missing", output)
        self.assertIn("disallowed_fallback_placement", output)

    def test_main_reports_out_of_scope_when_no_fields_and_no_findings(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e3_option_admit_chain",
                    "program": {
                        "kind": "Program",
                        "body": [
                            {
                                "kind": "PlaceBlock",
                                "place": "root",
                                "body": [],
                            }
                        ],
                    },
                    "expected_static": {"verdict": "valid"},
                },
            )
            self.write_json(
                artifact_path,
                {
                    "checker_core": {
                        "static_verdict": "valid",
                        "reasons": [],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: out_of_scope", output)

    def test_main_rejects_missing_artifact_static_verdict(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e23_malformed_try_fallback_missing_fallback_body",
                    "expected_static": {
                        "verdict": "malformed",
                        "checked_try_rollback_structural_verdict": "findings_present",
                        "checked_try_rollback_structural_findings": [
                            {
                                "subject_kind": "TryFallback",
                                "finding_kind": "missing_fallback_body",
                            }
                        ],
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "checker_core": {
                        "reasons": ["try fallback body must not be empty"],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        self.assertIn("status: static_gate_verdict_mismatch", stdout.getvalue())

    def test_main_rejects_unrelated_artifact_reasons(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "fixture_id": "e24_malformed_atomic_cut_fallback_placement",
                    "expected_static": {
                        "verdict": "malformed",
                        "checked_try_rollback_structural_verdict": "findings_present",
                        "checked_try_rollback_structural_findings": [
                            {
                                "subject_kind": "AtomicCut",
                                "finding_kind": "disallowed_fallback_placement",
                            }
                        ],
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "checker_core": {
                        "static_verdict": "malformed",
                        "reasons": ["some unrelated malformed reason"],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
