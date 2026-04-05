import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_checked_reasons_assist as assist  # noqa: E402


def fixture_document(*, checked_reasons=None) -> dict:
    expected_static = {
        "verdict": "malformed",
        "reasons": ["explanatory note"],
    }
    if checked_reasons is not None:
        expected_static["checked_reasons"] = checked_reasons
    return {
        "schema_version": "current-l2-ast-fixture-v0",
        "fixture_id": "e4_malformed_lineage",
        "source_example_id": "E4",
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


def static_gate_artifact(reasons: list[str]) -> dict:
    return {
        "schema_version": "draft-current-l2-static-gate-v0",
        "artifact_kind": "current-l2-static-gate-detached-sketch",
        "fixture_context": {
            "fixture_id": "e4_malformed_lineage",
            "fixture_path": "fixtures/e4-malformed-lineage.json",
            "source_example_id": "E4",
        },
        "checker_core": {
            "static_verdict": "malformed",
            "reasons": reasons,
        },
    }


class CurrentL2CheckedReasonsAssistTests(unittest.TestCase):
    def test_main_prints_json_snippet_when_fixture_has_no_checked_reasons(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e4-malformed-lineage.json"
            artifact_path = tmpdir_path / "e4-malformed-lineage.static-gate.json"
            fixture_path.write_text(
                json.dumps(fixture_document(), ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(
                    static_gate_artifact(
                        ["missing lineage assertion for primary -> mirror"]
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
        self.assertIn("current expected_static.checked_reasons: absent", output)
        self.assertIn('"checked_reasons": [', output)
        self.assertIn("missing lineage assertion for primary -> mirror", output)

    def test_main_reports_when_checked_reasons_already_match(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e4-malformed-lineage.json"
            artifact_path = tmpdir_path / "e4-malformed-lineage.static-gate.json"
            reasons = ["missing lineage assertion for primary -> mirror"]
            fixture_path.write_text(
                json.dumps(
                    fixture_document(checked_reasons=reasons),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(static_gate_artifact(reasons), ensure_ascii=False, indent=2)
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = assist.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("already matches actual static gate reasons", output)

    def test_main_reports_no_suggestion_when_actual_static_gate_reasons_are_empty(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_path = tmpdir_path / "e3-option-admit-chain.json"
            artifact_path = tmpdir_path / "e3-option-admit-chain.static-gate.json"
            fixture_path.write_text(
                json.dumps(fixture_document(), ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )
            artifact_path.write_text(
                json.dumps(static_gate_artifact([]), ensure_ascii=False, indent=2) + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = assist.main([str(fixture_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("actual static gate reasons are empty", output)


if __name__ == "__main__":
    unittest.main()
