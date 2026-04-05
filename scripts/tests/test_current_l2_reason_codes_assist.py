import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_reason_codes_assist as assist  # noqa: E402


def fixture_document(*, typed_reason_codes=None) -> dict:
    expected_static = {
        "verdict": "underdeclared",
        "reasons": ["explanatory note"],
        "checked_reasons": [
            "declared access target is missing for writer -> readonly"
        ],
    }
    if typed_reason_codes is not None:
        expected_static["reason_codes"] = typed_reason_codes
    return {
        "schema_version": "current-l2-ast-fixture-v0",
        "fixture_id": "e12_underdeclared_target_missing",
        "source_example_id": "E12",
        "program": {"kind": "Program", "body": []},
        "expected_static": expected_static,
        "expected_runtime": {
            "enters_evaluation": False,
            "final_outcome": "not_evaluated",
            "notes": [],
        },
        "expected_trace_audit": {
            "event_kinds": [],
            "non_admissible_metadata": [],
            "narrative_explanations": [],
            "must_explain": [],
        },
    }


def static_gate_artifact(*, reason_codes: list[dict] | None) -> dict:
    artifact = {
        "schema_version": "draft-current-l2-static-gate-v0",
        "artifact_kind": "current-l2-static-gate-detached-sketch",
        "fixture_context": {
            "fixture_id": "e12_underdeclared_target_missing",
            "fixture_path": "fixtures/e12-underdeclared-target-missing.json",
            "source_example_id": "E12",
        },
        "checker_core": {
            "static_verdict": "underdeclared",
            "reasons": ["declared access target is missing for writer -> readonly"],
        },
    }
    if reason_codes is not None:
        artifact["detached_noncore"] = {
            "reason_codes_scope": "stable-clusters-only",
            "reason_codes": reason_codes,
        }
    return artifact


class CurrentL2ReasonCodesAssistTests(unittest.TestCase):
    def test_main_prints_reference_only_reason_code_rows(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e12-underdeclared-target-missing.json"
            artifact_path = tmpdir_path / "e12-underdeclared-target-missing.static-gate.json"
            fixture_path.write_text(
                json.dumps(fixture_document(), ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(
                    static_gate_artifact(
                        reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ]
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = assist.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn(
            "current fixture-side typed reason code carrier: absent (no current fixture field)",
            output,
        )
        self.assertIn("reason_codes_scope: stable-clusters-only", output)
        self.assertIn("suggested reason code rows (reference-only):", output)
        self.assertIn('"kind": "declared_target_missing"', output)

    def test_main_reports_no_suggestion_when_reason_codes_are_absent(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e12-underdeclared-target-missing.json"
            artifact_path = tmpdir_path / "e12-underdeclared-target-missing.static-gate.json"
            fixture_path.write_text(
                json.dumps(fixture_document(), ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(
                    static_gate_artifact(reason_codes=None),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = assist.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("detached artifact has no reason_codes suggestion", output)

    def test_main_reports_no_suggestion_when_reason_codes_are_empty(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e12-underdeclared-target-missing.json"
            artifact_path = tmpdir_path / "e12-underdeclared-target-missing.static-gate.json"
            fixture_path.write_text(
                json.dumps(fixture_document(), ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(
                    static_gate_artifact(reason_codes=[]),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = assist.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("detached artifact has no reason_codes suggestion", output)

    def test_main_rejects_fixture_side_typed_reason_code_field(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e12-underdeclared-target-missing.json"
            artifact_path = tmpdir_path / "e12-underdeclared-target-missing.static-gate.json"
            fixture_path.write_text(
                json.dumps(
                    fixture_document(
                        typed_reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ]
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(
                    static_gate_artifact(
                        reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ]
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            stderr = io.StringIO()
            with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
                exit_code = assist.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 2)
        self.assertEqual(stdout.getvalue(), "")
        self.assertIn(
            "fixture already contains unsupported expected_static.reason_codes field",
            stderr.getvalue(),
        )


if __name__ == "__main__":
    unittest.main()
