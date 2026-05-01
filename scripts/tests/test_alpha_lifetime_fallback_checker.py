import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_lifetime_fallback_checker as checker  # noqa: E402


class AlphaLifetimeFallbackCheckerTests(unittest.TestCase):
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

    def test_seed_sidecars_expose_expected_checker_rows(self) -> None:
        cases = {
            "lif-05-underdeclared_fallback_static_error": "missing_lineage_evidence",
            "lif-06-incompatible_access_target_rejected": "incompatible_access_target",
            "lif-07-capability_promotion_rejected": "capability_promotion",
            "lif-08-write_after_read_only_fallback_rejected": "write_through_readonly_fallback",
        }

        for stem, expected_kind in cases.items():
            payload = json.loads(self.sidecar_path(stem).read_text(encoding="utf-8"))
            checker_rows = payload["expected_static"]["checked_reason_codes"]
            self.assertEqual(checker_rows[0]["kind"], expected_kind)

    def test_main_reports_matched_rows_from_real_seed_sidecar(self) -> None:
        sidecar_path = self.sidecar_path("lif-05-underdeclared_fallback_static_error")

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "missing_lineage_evidence",
                                "option": "fallback_access",
                                "requires_edge": "primary_access -> fallback_access",
                            },
                            {
                                "kind": "hidden_shadowing",
                                "provider": "debug_layer",
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
        self.assertIn("cluster: alpha_lifetime_fallback_static_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("missing_lineage_evidence", output)
        self.assertNotIn("hidden_shadowing", output)

    def test_main_reports_missing_expected_rows_when_artifact_has_lifetime_kind(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            sidecar_path = temp_root / "lif-positive.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "sample_id": "LIF-02",
                    "family": "lifetime-fallback",
                    "expected": "valid",
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
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

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: sample_expected_reason_rows_missing", output)
        self.assertIn("capability_promotion", output)


if __name__ == "__main__":
    unittest.main()
