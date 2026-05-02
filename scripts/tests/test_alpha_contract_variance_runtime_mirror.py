import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_contract_variance_runtime_mirror as checker  # noqa: E402


class AlphaContractVarianceRuntimeMirrorTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def target_sidecar_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "contract-variance"
            / f"{stem}.expected.json"
        )

    def source_sidecar_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "layer-insertion"
            / f"{stem}.expected.json"
        )

    def write_target_copy_with_source_path(
        self,
        temp_root: Path,
        *,
        target_stem: str,
        source_path: Path,
    ) -> Path:
        target_path = temp_root / f"{target_stem}.expected.json"
        payload = json.loads(self.target_sidecar_path(target_stem).read_text(encoding="utf-8"))
        payload["runtime_mirror"]["source_sidecar"] = str(source_path)
        self.write_json(target_path, payload)
        return target_path

    def test_sidecars_expose_runtime_mirror_rows(self) -> None:
        cases = {
            "var-08-ratelimit_declared_failure_valid": (
                "LI-04",
                "declared_failure_runtime_preview",
            ),
            "var-11-redaction_layer_valid": (
                "LI-01",
                "redacted_debug_layer_runtime_preview",
            ),
            "var-13-auth_layer_contract_update_valid": (
                "LI-03",
                "auth_contract_update_runtime_preview",
            ),
        }

        for stem, (expected_source_id, expected_kind) in cases.items():
            payload = json.loads(self.target_sidecar_path(stem).read_text(encoding="utf-8"))
            runtime_mirror = payload["runtime_mirror"]
            self.assertEqual(runtime_mirror["scope"], "alpha-runtime-mirror-floor")
            self.assertEqual(runtime_mirror["source_sample_id"], expected_source_id)
            self.assertEqual(
                runtime_mirror["checked_runtime_mirror_rows"][0]["kind"],
                expected_kind,
            )

    def test_var_08_real_target_and_source_match(self) -> None:
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout):
            exit_code = checker.main(
                [
                    str(
                        self.target_sidecar_path(
                            "var-08-ratelimit_declared_failure_valid"
                        )
                    ),
                    str(
                        self.source_sidecar_path(
                            "li-04-ratelimit_declared_failure"
                        )
                    ),
                ]
            )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertIn("declared_failure_runtime_preview", output)

    def test_var_11_real_target_and_source_match(self) -> None:
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout):
            exit_code = checker.main(
                [
                    str(self.target_sidecar_path("var-11-redaction_layer_valid")),
                    str(
                        self.source_sidecar_path(
                            "li-01-debug_layer_attach_authorized"
                        )
                    ),
                ]
            )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertIn("redacted_debug_layer_runtime_preview", output)

    def test_var_13_real_target_and_source_match(self) -> None:
        stdout = io.StringIO()
        with contextlib.redirect_stdout(stdout):
            exit_code = checker.main(
                [
                    str(self.target_sidecar_path("var-13-auth_layer_contract_update_valid")),
                    str(
                        self.source_sidecar_path(
                            "li-03-auth_layer_contract_update_path"
                        )
                    ),
                ]
            )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertIn("auth_contract_update_runtime_preview", output)

    def test_scope_mismatch_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            target_path = temp_root / "var-08.expected.json"
            payload = json.loads(
                self.target_sidecar_path(
                    "var-08-ratelimit_declared_failure_valid"
                ).read_text(encoding="utf-8")
            )
            payload["runtime_mirror"]["scope"] = "wrong-floor"
            self.write_json(target_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(
                            self.source_sidecar_path(
                                "li-04-ratelimit_declared_failure"
                            )
                        ),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: scope_mismatch", stdout.getvalue())

    def test_target_without_runtime_mirror_reports_missing(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            target_path = temp_root / "var-negative.expected.json"
            self.write_json(
                target_path,
                {
                    "sample_id": "VAR-02",
                    "family": "contract-variance",
                    "expected_static": {
                        "checked_reason_codes": [
                            {"kind": "precondition_strengthening"}
                        ]
                    },
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(
                            self.source_sidecar_path(
                                "li-04-ratelimit_declared_failure"
                            )
                        ),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "status: sample_expected_runtime_mirror_missing",
            stdout.getvalue(),
        )

    def test_source_claim_mismatch_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_path = temp_root / "li-04.expected.json"
            target_path = self.write_target_copy_with_source_path(
                temp_root,
                target_stem="var-08-ratelimit_declared_failure_valid",
                source_path=source_path,
            )
            payload = json.loads(
                self.source_sidecar_path(
                    "li-04-ratelimit_declared_failure"
                ).read_text(encoding="utf-8")
            )
            payload["claims"]["active_root"] = True
            self.write_json(source_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(source_path),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())

    def test_source_runnable_false_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_path = temp_root / "li-01.expected.json"
            target_path = self.write_target_copy_with_source_path(
                temp_root,
                target_stem="var-11-redaction_layer_valid",
                source_path=source_path,
            )
            payload = json.loads(
                self.source_sidecar_path(
                    "li-01-debug_layer_attach_authorized"
                ).read_text(encoding="utf-8")
            )
            payload["claims"]["runnable"] = False
            self.write_json(source_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(source_path),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())

    def test_source_implemented_false_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_path = temp_root / "li-03.expected.json"
            target_path = self.write_target_copy_with_source_path(
                temp_root,
                target_stem="var-13-auth_layer_contract_update_valid",
                source_path=source_path,
            )
            payload = json.loads(
                self.source_sidecar_path(
                    "li-03-auth_layer_contract_update_path"
                ).read_text(encoding="utf-8")
            )
            payload["claims"]["implemented"] = False
            self.write_json(source_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(source_path),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())

    def test_source_missing_target_mirror_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_path = temp_root / "li-01.expected.json"
            target_path = self.write_target_copy_with_source_path(
                temp_root,
                target_stem="var-11-redaction_layer_valid",
                source_path=source_path,
            )
            payload = json.loads(
                self.source_sidecar_path(
                    "li-01-debug_layer_attach_authorized"
                ).read_text(encoding="utf-8")
            )
            payload["mirrors"] = ["HP-02", "VIS-10"]
            self.write_json(source_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(source_path),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())

    def test_missing_required_runtime_field_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_path = temp_root / "li-03.expected.json"
            target_path = self.write_target_copy_with_source_path(
                temp_root,
                target_stem="var-13-auth_layer_contract_update_valid",
                source_path=source_path,
            )
            payload = json.loads(
                self.source_sidecar_path(
                    "li-03-auth_layer_contract_update_path"
                ).read_text(encoding="utf-8")
            )
            del payload["expected_runtime"]["required_contract_update_ref"]
            self.write_json(source_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(source_path),
                    ]
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())

    def test_runtime_mirror_helper_ignores_reason_codes_and_acceptance_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_path = temp_root / "li-04.expected.json"
            target_path = self.write_target_copy_with_source_path(
                temp_root,
                target_stem="var-08-ratelimit_declared_failure_valid",
                source_path=source_path,
            )
            payload = json.loads(
                self.source_sidecar_path(
                    "li-04-ratelimit_declared_failure"
                ).read_text(encoding="utf-8")
            )
            payload["detached_noncore"] = {
                "reason_codes_scope": "alpha-static-floor",
                "reason_codes": [
                    {
                        "kind": "effect_row_widening",
                        "effect_delta": ["telemetry_write"],
                    }
                ],
                "acceptance_scope": "alpha-acceptance-floor",
                "acceptance_rows": [
                    {
                        "kind": "transparent_observe_only_layer",
                        "sample_id": "VAR-01",
                    }
                ],
            }
            self.write_json(source_path, payload)

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main(
                    [
                        str(target_path),
                        str(source_path),
                    ]
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertNotIn("effect_row_widening", output)
        self.assertNotIn("transparent_observe_only_layer", output)


if __name__ == "__main__":
    unittest.main()
