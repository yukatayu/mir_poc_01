import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_contract_variance_checker as checker  # noqa: E402


class AlphaContractVarianceCheckerTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def sidecar_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "contract-variance"
            / f"{stem}.expected.json"
        )

    def sample_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "contract-variance"
            / f"{stem}.mir"
        )

    def test_seed_sidecars_expose_expected_checker_rows(self) -> None:
        cases = {
            "var-02-precondition_strengthening_rejected": "precondition_strengthening",
            "var-03-postcondition_weakening_rejected": "postcondition_weakening",
            "var-07-failure_row_widening_rejected": "failure_row_widening",
            "var-09-effect_row_widening_rejected": "effect_row_widening",
            "var-10-cost_degradation_rejected": "cost_degradation",
            "var-15-hidden_shadowing_rejected": "hidden_shadowing",
        }

        for stem, expected_kind in cases.items():
            payload = json.loads(self.sidecar_path(stem).read_text(encoding="utf-8"))
            checker_rows = payload["expected_static"]["checked_reason_codes"]
            self.assertEqual(checker_rows[0]["kind"], expected_kind)

    def test_main_reports_matched_rows_from_real_seed_sidecar(self) -> None:
        sidecar_path = self.sidecar_path("var-02-precondition_strengthening_rejected")

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "precondition_strengthening",
                                "base_precondition": "member",
                                "layer_precondition": "admin",
                            },
                            {
                                "kind": "missing_lineage_evidence",
                                "option": "fallback_access",
                            },
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_contract_variance_static_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("precondition_strengthening", output)
        self.assertNotIn("missing_lineage_evidence", output)

    def test_main_reports_missing_expected_rows_when_artifact_has_variance_kind(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            sidecar_path = temp_root / "var-positive.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "sample_id": "VAR-01",
                    "family": "contract-variance",
                    "expected": "valid overlay",
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "hidden_shadowing",
                                "capability": "roll_request",
                                "provider": "shadow_layer",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: sample_expected_reason_rows_missing", output)
        self.assertIn("hidden_shadowing", output)

    def test_var07_seed_matches_sample_purpose(self) -> None:
        stem = "var-07-failure_row_widening_rejected"
        payload = json.loads(self.sidecar_path(stem).read_text(encoding="utf-8"))
        sample_text = self.sample_path(stem).read_text(encoding="utf-8")
        row = payload["expected_static"]["checked_reason_codes"][0]

        self.assertEqual(row["kind"], "failure_row_widening")
        self.assertEqual(row["added_failure"], "RateLimited")
        self.assertIn(f"Purpose: {row['added_failure']} added undeclared", sample_text)


if __name__ == "__main__":
    unittest.main()
