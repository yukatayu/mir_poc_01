import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_lifetime_fallback_acceptance as checker  # noqa: E402


class AlphaLifetimeFallbackAcceptanceTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def sidecar_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "lifetime-fallback"
            / f"{stem}.expected.json"
        )

    def test_sidecars_expose_expected_acceptance_rows(self) -> None:
        cases = {
            "lif-02-fallback_extends_access_path": "fallback_chain_canonicalized",
            "lif-03-nested_inherit_chain_valid": "inherited_chain_spliced_with_lineage",
            "lif-04-plain_ref_does_not_inherit": "plain_ref_boundary_preserved",
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
        sidecar_path = self.sidecar_path("lif-02-fallback_extends_access_path")

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "alpha-acceptance-floor",
                        "acceptance_rows": [
                            {
                                "kind": "fallback_chain_canonicalized",
                                "sample_id": "LIF-02",
                                "role": "demo_access_path",
                                "canonical_chain": ["c", "a"],
                                "capability": "read",
                                "monotone_degradation": True,
                            },
                            {
                                "kind": "transparent_observe_only_layer",
                                "sample_id": "VAR-01",
                            },
                        ],
                        "reason_codes_scope": "wrong-floor",
                        "reason_codes": [
                            {
                                "kind": "capability_promotion",
                                "from_capability": "read",
                                "to_capability": "write",
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
        self.assertIn("cluster: alpha_lifetime_fallback_acceptance_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("fallback_chain_canonicalized", output)
        self.assertNotIn("transparent_observe_only_layer", output)
        self.assertNotIn("capability_promotion", output)

    def test_main_reports_scope_mismatch(self) -> None:
        sidecar_path = self.sidecar_path("lif-03-nested_inherit_chain_valid")

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "acceptance_scope": "wrong-floor",
                        "acceptance_rows": [
                            {
                                "kind": "inherited_chain_spliced_with_lineage",
                                "sample_id": "LIF-03",
                                "source_chain": ["e", "b"],
                                "appended_fallback": "a",
                                "canonical_chain": ["e", "b", "a"],
                                "lineage_edges": ["e->b", "b->a"],
                                "implicit_propagation": False,
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
            sidecar_path = temp_root / "lif-negative.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "sample_id": "LIF-05",
                    "family": "lifetime-fallback",
                    "expected_static": {
                        "checked_reason_codes": [{"kind": "missing_lineage_evidence"}]
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
                                "kind": "plain_ref_boundary_preserved",
                                "sample_id": "LIF-04",
                                "ref_cell": "d",
                                "fallback_target": "a",
                                "inherited_options": [],
                                "does_not_splice_inner_chain": True,
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
            sidecar_path = temp_root / "var-sidecar.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "expected_acceptance": {
                        "checked_acceptance_scope": "alpha-acceptance-floor",
                        "checked_acceptance_rows": [
                            {
                                "kind": "transparent_observe_only_layer",
                                "sample_id": "VAR-01",
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
                                "kind": "transparent_observe_only_layer",
                                "sample_id": "VAR-01",
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
