import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_reason_code_readiness as readiness  # noqa: E402


def static_fixture_document(
    fixture_id: str,
    *,
    checked_reasons: list[str] | None,
    checked_reason_codes: list[dict] | None = None,
    typed_reason_codes=None,
) -> dict:
    expected_static = {
        "verdict": "malformed",
        "reasons": ["explanatory note"],
    }
    if checked_reasons is not None:
        expected_static["checked_reasons"] = checked_reasons
    if checked_reason_codes is not None:
        expected_static["checked_reason_codes"] = checked_reason_codes
    if typed_reason_codes is not None:
        expected_static["reason_codes"] = typed_reason_codes
    return {
        "schema_version": "current-l2-ast-fixture-v0",
        "fixture_id": fixture_id,
        "source_example_id": fixture_id.upper(),
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


def runtime_fixture_document(fixture_id: str) -> dict:
    return {
        "schema_version": "current-l2-ast-fixture-v0",
        "fixture_id": fixture_id,
        "source_example_id": fixture_id.upper(),
        "program": {"kind": "Program", "body": []},
        "expected_static": {
            "verdict": "valid",
            "reasons": [],
        },
        "expected_runtime": {
            "enters_evaluation": True,
            "final_outcome": "success",
            "notes": [],
        },
        "expected_trace_audit": {
            "event_kinds": [],
            "non_admissible_metadata": [],
            "narrative_explanations": [],
            "must_explain": [],
        },
    }


def static_gate_artifact(
    fixture_id: str,
    *,
    reason_codes: list[dict] | None,
) -> dict:
    artifact = {
        "schema_version": "draft-current-l2-static-gate-v0",
        "artifact_kind": "current-l2-static-gate-detached-sketch",
        "fixture_context": {
            "fixture_id": fixture_id,
            "fixture_path": f"fixtures/{fixture_id}.json",
            "source_example_id": fixture_id.upper(),
        },
        "checker_core": {
            "static_verdict": "malformed",
            "reasons": ["placeholder"],
        },
    }
    if reason_codes is not None:
        artifact["detached_noncore"] = {
            "reason_codes_scope": "stable-clusters-only",
            "reason_codes": reason_codes,
        }
    return artifact


class CurrentL2ReasonCodeReadinessTests(unittest.TestCase):
    def test_main_reports_kind_counts_and_fixture_groups(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_dir = tmpdir_path / "fixtures"
            artifact_dir = tmpdir_path / "artifacts"
            fixture_dir.mkdir()
            artifact_dir.mkdir()

            fixture_a = fixture_dir / "e12-underdeclared-target-missing.json"
            fixture_a.write_text(
                json.dumps(
                    static_fixture_document(
                        "e12_underdeclared_target_missing",
                        checked_reasons=[
                            "declared access target is missing for writer -> readonly"
                        ],
                        checked_reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            (artifact_dir / "e12-underdeclared-target-missing.static-gate.json").write_text(
                json.dumps(
                    static_gate_artifact(
                        "e12_underdeclared_target_missing",
                        reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            fixture_b = fixture_dir / "e14-malformed-duplicate-option-declaration.json"
            fixture_b.write_text(
                json.dumps(
                    static_fixture_document(
                        "e14_malformed_duplicate_option_declaration",
                        checked_reasons=None,
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            (artifact_dir / "e14-malformed-duplicate-option-declaration.static-gate.json").write_text(
                json.dumps(
                    static_gate_artifact(
                        "e14_malformed_duplicate_option_declaration",
                        reason_codes=None,
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            runtime_fixture = fixture_dir / "e3-option-admit-chain.json"
            runtime_fixture.write_text(
                json.dumps(
                    runtime_fixture_document("e3_option_admit_chain"),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = readiness.main([str(fixture_dir), str(artifact_dir)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("static-only fixtures scanned: 2", output)
        self.assertIn("runtime fixtures skipped: 1", output)
        self.assertIn("fixtures with checked_reasons: 1", output)
        self.assertIn("fixtures with reason_codes suggestions: 1", output)
        self.assertIn("fixtures with checked_reason_codes: 1", output)
        self.assertIn("fixtures with stable coexistence anchors: 1", output)
        self.assertIn("fixtures with checked_reason_codes but missing checked_reasons: 0", output)
        self.assertIn("fixtures with checked_reason_codes mismatching actual suggestion: 0", output)
        self.assertIn("declared_target_missing: 1", output)
        self.assertIn("checker cluster coverage:", output)
        self.assertIn("same_lineage_evidence_floor: 1", output)
        self.assertIn("e12_underdeclared_target_missing", output)
        self.assertIn("e14_malformed_duplicate_option_declaration", output)
        self.assertIn("fixtures needing coexistence follow-up:", output)
        self.assertIn("  - none", output)

    def test_main_rejects_fixture_side_typed_reason_codes(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_dir = tmpdir_path / "fixtures"
            artifact_dir = tmpdir_path / "artifacts"
            fixture_dir.mkdir()
            artifact_dir.mkdir()

            fixture_path = fixture_dir / "e12-underdeclared-target-missing.json"
            fixture_path.write_text(
                json.dumps(
                    static_fixture_document(
                        "e12_underdeclared_target_missing",
                        checked_reasons=None,
                        typed_reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            (artifact_dir / "e12-underdeclared-target-missing.static-gate.json").write_text(
                json.dumps(
                    static_gate_artifact(
                        "e12_underdeclared_target_missing",
                        reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
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
                exit_code = readiness.main([str(fixture_dir), str(artifact_dir)])

        self.assertEqual(exit_code, 2)
        self.assertEqual(stdout.getvalue(), "")
        self.assertIn(
            "fixture already contains unsupported expected_static.reason_codes field",
            stderr.getvalue(),
        )

    def test_main_reports_coexistence_follow_up_groups(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir_path = Path(tmpdir)
            fixture_dir = tmpdir_path / "fixtures"
            artifact_dir = tmpdir_path / "artifacts"
            fixture_dir.mkdir()
            artifact_dir.mkdir()

            fixture_missing_checked_reasons = fixture_dir / "e12-underdeclared-target-missing.json"
            fixture_missing_checked_reasons.write_text(
                json.dumps(
                    static_fixture_document(
                        "e12_underdeclared_target_missing",
                        checked_reasons=None,
                        checked_reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            (artifact_dir / "e12-underdeclared-target-missing.static-gate.json").write_text(
                json.dumps(
                    static_gate_artifact(
                        "e12_underdeclared_target_missing",
                        reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            fixture_mismatch = fixture_dir / "e19-malformed-target-mismatch.json"
            fixture_mismatch.write_text(
                json.dumps(
                    static_fixture_document(
                        "e19_malformed_target_mismatch",
                        checked_reasons=[
                            "declared access target mismatch between writer and readonly"
                        ],
                        checked_reason_codes=[
                            {
                                "kind": "declared_target_missing",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )
            (artifact_dir / "e19-malformed-target-mismatch.static-gate.json").write_text(
                json.dumps(
                    static_gate_artifact(
                        "e19_malformed_target_mismatch",
                        reason_codes=[
                            {
                                "kind": "declared_target_mismatch",
                                "predecessor": "writer",
                                "successor": "readonly",
                            }
                        ],
                    ),
                    ensure_ascii=False,
                    indent=2,
                )
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = readiness.main([str(fixture_dir), str(artifact_dir)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("fixtures with stable coexistence anchors: 0", output)
        self.assertIn("fixtures with checked_reason_codes but missing checked_reasons: 1", output)
        self.assertIn("fixtures with checked_reason_codes mismatching actual suggestion: 1", output)
        self.assertIn("checker cluster coverage:", output)
        self.assertIn("same_lineage_evidence_floor: 2", output)
        self.assertIn(
            "e12_underdeclared_target_missing [missing checked_reasons, kinds=declared_target_missing]",
            output,
        )
        self.assertIn(
            "e19_malformed_target_mismatch [checked_reason_codes mismatch, fixture=declared_target_missing, actual=declared_target_mismatch]",
            output,
        )

    def test_cluster_name_maps_stable_reason_kinds(self) -> None:
        self.assertEqual(
            readiness.checker_cluster_name_for_kind("missing_lineage_assertion"),
            "same_lineage_evidence_floor",
        )
        self.assertEqual(
            readiness.checker_cluster_name_for_kind("declared_target_mismatch"),
            "same_lineage_evidence_floor",
        )
        self.assertEqual(
            readiness.checker_cluster_name_for_kind("capability_strengthens"),
            "capability_strengthening_floor",
        )
        self.assertEqual(
            readiness.checker_cluster_name_for_kind("missing_successor_option"),
            "missing_option_structure_floor",
        )
        self.assertIsNone(
            readiness.checker_cluster_name_for_kind("duplicate_option_declaration")
        )


if __name__ == "__main__":
    unittest.main()
