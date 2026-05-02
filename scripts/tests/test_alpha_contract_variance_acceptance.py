import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_contract_variance_acceptance as checker  # noqa: E402


class AlphaContractVarianceAcceptanceTests(unittest.TestCase):
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

    def test_sidecars_expose_expected_acceptance_rows(self) -> None:
        cases = {
            "var-01-logging_layer_valid": "transparent_observe_only_layer",
            "var-04-output_covariance_valid": "output_covariance_checked",
            "var-06-readonly_covariance_valid": "readonly_covariance_checked",
        }

        for stem, expected_kind in cases.items():
            payload = json.loads(self.sidecar_path(stem).read_text(encoding="utf-8"))
            acceptance = payload["expected_acceptance"]
            self.assertEqual(
                acceptance["checked_acceptance_scope"],
                "alpha-acceptance-floor",
            )
            self.assertEqual(
                acceptance["checked_acceptance_rows"][0]["kind"],
                expected_kind,
            )

    def test_main_reports_matched_rows_from_real_seed_sidecar(self) -> None:
        sidecar_path = self.sidecar_path("var-01-logging_layer_valid")

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [
                            {
                                "kind": "transparent_observe_only_layer",
                                "sample_id": "VAR-01",
                                "layer_kind": "Debug",
                                "effect_delta": [],
                                "failure_delta": [],
                                "precondition_strengthened": False,
                                "postcondition_weakened": False,
                            },
                            {
                                "kind": "fallback_chain_canonicalized",
                                "sample_id": "LIF-02",
                            },
                        ],
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "precondition_strengthening",
                                "base_precondition": "member(room)",
                                "layer_precondition": "member(room) AND admin",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_contract_variance_acceptance_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("transparent_observe_only_layer", output)
        self.assertNotIn("fallback_chain_canonicalized", output)
        self.assertNotIn("precondition_strengthening", output)

    def test_main_reports_scope_mismatch(self) -> None:
        sidecar_path = self.sidecar_path("var-04-output_covariance_valid")

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "wrong-floor",
                        "acceptance_rows": [
                            {
                                "kind": "output_covariance_checked",
                                "sample_id": "VAR-04",
                                "base_output": "Animal",
                                "layer_output": "Cat",
                                "relation": "Cat <: Animal",
                                "admissible": True,
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
        self.assertIn("status: scope_mismatch", output)
        self.assertIn("expected_acceptance_scope: alpha-acceptance-floor", output)
        self.assertIn("artifact_acceptance_scope: wrong-floor", output)

    def test_main_reports_missing_expected_rows_when_sidecar_has_no_acceptance(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            sidecar_path = temp_root / "var-negative.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "sample_id": "VAR-02",
                    "family": "contract-variance",
                    "expected_static": {
                        "checked_reason_codes": [{"kind": "precondition_strengthening"}]
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [
                            {
                                "kind": "readonly_covariance_checked",
                                "sample_id": "VAR-06",
                                "from": "Read<Cat>",
                                "to": "Read<Animal>",
                                "mutable": False,
                                "write_capability": False,
                                "admissible": True,
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
        self.assertIn("status: sample_expected_acceptance_rows_missing", output)

    def test_unsupported_rows_are_out_of_scope(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            sidecar_path = temp_root / "lif-sidecar.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [
                            {
                                "kind": "fallback_chain_canonicalized",
                                "sample_id": "LIF-02",
                            }
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
                            {
                                "kind": "fallback_chain_canonicalized",
                                "sample_id": "LIF-02",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: out_of_scope", output)


if __name__ == "__main__":
    unittest.main()
