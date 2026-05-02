import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_family_runtime_mirror_support as support  # noqa: E402


class CurrentL2FamilyRuntimeMirrorSupportTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_run_family_runtime_mirror_reports_matched_filtered_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            target_path = temp_root / "target.expected.json"
            source_path = temp_root / "source.expected.json"
            self.write_json(
                target_path,
                {
                    "sample_id": "VAR-08",
                    "runtime_mirror": {
                        "scope": "alpha-runtime-mirror-floor",
                        "source_family": "layer-insertion",
                        "source_sample_id": "LI-04",
                        "source_sidecar": str(source_path),
                        "required_source_status": "runtime-floor",
                        "required_source_claims": {
                            "runnable": True,
                            "implemented": True,
                            "active_root": False,
                        },
                        "required_source_current_validation_mode": "rust-runtime-floor",
                        "checked_runtime_mirror_rows": [
                            {
                                "kind": "declared_failure_runtime_preview",
                                "target_sample_id": "VAR-08",
                                "source_sample_id": "LI-04",
                                "terminal_outcome": "accepted",
                                "preview_terminal_outcome": "rejected",
                                "required_preview_reason_refs": [
                                    "RateLimited",
                                    "rate_limit_budget_exhausted",
                                ],
                            },
                            {
                                "kind": "other_kind",
                                "target_sample_id": "VAR-X",
                                "source_sample_id": "LI-X",
                            },
                        ],
                    },
                },
            )
            self.write_json(
                source_path,
                {
                    "sample_id": "LI-04",
                    "family": "layer-insertion",
                    "status": "runtime-floor",
                    "mirrors": ["VAR-08", "HP-05"],
                    "expected_runtime": {
                        "terminal_outcome": "accepted",
                        "preview_terminal_outcome": "rejected",
                        "required_preview_reason_refs": [
                            "RateLimited",
                            "rate_limit_budget_exhausted",
                        ],
                    },
                    "claims": {
                        "runnable": True,
                        "implemented": True,
                        "active_root": False,
                    },
                    "current_validation": {"mode": "rust-runtime-floor"},
                    "detached_noncore": {
                        "reason_codes_scope": "wrong-floor",
                        "reason_codes": [{"kind": "declared_failure_runtime_preview"}],
                        "acceptance_scope": "wrong-floor",
                        "acceptance_rows": [{"kind": "declared_failure_runtime_preview"}],
                    },
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_runtime_mirror_checker(
                    argv=[str(target_path), str(source_path)],
                    cluster_name="alpha_runtime_mirror_cluster",
                    description="alpha runtime-mirror helper",
                    kinds={"declared_failure_runtime_preview"},
                    missing_status="sample_expected_runtime_mirror_missing",
                    expected_scope="alpha-runtime-mirror-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_runtime_mirror_cluster", output)
        self.assertIn("status: matched", output)
        self.assertIn("declared_failure_runtime_preview", output)
        self.assertNotIn("other_kind", output)
        self.assertNotIn("reason_codes", output)
        self.assertNotIn("acceptance_rows", output)

    def test_run_family_runtime_mirror_reports_missing_expected_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            target_path = temp_root / "target.expected.json"
            source_path = temp_root / "source.expected.json"
            self.write_json(target_path, {"sample_id": "VAR-11"})
            self.write_json(
                source_path,
                {
                    "sample_id": "LI-01",
                    "family": "layer-insertion",
                    "status": "runtime-floor",
                    "mirrors": ["VAR-11"],
                    "expected_runtime": {
                        "terminal_outcome": "accepted",
                        "activated_layer": "debug_trace_layer",
                        "redaction_level": "subject_and_payload_redacted",
                        "post_attach_trace_rows": 2,
                    },
                    "claims": {
                        "runnable": True,
                        "implemented": True,
                        "active_root": False,
                    },
                    "current_validation": {"mode": "rust-runtime-floor"},
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_runtime_mirror_checker(
                    argv=[str(target_path), str(source_path)],
                    cluster_name="alpha_runtime_mirror_cluster",
                    description="alpha runtime-mirror helper",
                    kinds={"redacted_debug_layer_runtime_preview"},
                    missing_status="sample_expected_runtime_mirror_missing",
                    expected_scope="alpha-runtime-mirror-floor",
                )

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "status: sample_expected_runtime_mirror_missing",
            stdout.getvalue(),
        )

    def test_run_family_runtime_mirror_reports_out_of_scope(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            target_path = temp_root / "target.expected.json"
            source_path = temp_root / "source.expected.json"
            self.write_json(
                target_path,
                {
                    "sample_id": "VAR-11",
                    "runtime_mirror": {
                        "scope": "alpha-runtime-mirror-floor",
                        "source_family": "layer-insertion",
                        "source_sample_id": "LI-01",
                        "source_sidecar": str(source_path),
                        "required_source_status": "runtime-floor",
                        "required_source_claims": {
                            "runnable": True,
                            "implemented": True,
                            "active_root": False,
                        },
                        "required_source_current_validation_mode": "rust-runtime-floor",
                        "checked_runtime_mirror_rows": [
                            {
                                "kind": "unsupported_kind",
                                "target_sample_id": "VAR-11",
                                "source_sample_id": "LI-01",
                            }
                        ],
                    },
                },
            )
            self.write_json(
                source_path,
                {
                    "sample_id": "LI-01",
                    "family": "layer-insertion",
                    "status": "runtime-floor",
                    "mirrors": ["VAR-11"],
                    "expected_runtime": {
                        "terminal_outcome": "accepted",
                        "activated_layer": "debug_trace_layer",
                    },
                    "claims": {
                        "runnable": True,
                        "implemented": True,
                        "active_root": False,
                    },
                    "current_validation": {"mode": "rust-runtime-floor"},
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_runtime_mirror_checker(
                    argv=[str(target_path), str(source_path)],
                    cluster_name="alpha_runtime_mirror_cluster",
                    description="alpha runtime-mirror helper",
                    kinds={"redacted_debug_layer_runtime_preview"},
                    missing_status="sample_expected_runtime_mirror_missing",
                    expected_scope="alpha-runtime-mirror-floor",
                )

        self.assertEqual(exit_code, 0)
        self.assertIn("status: out_of_scope", stdout.getvalue())

    def test_run_family_runtime_mirror_reports_scope_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            target_path = temp_root / "target.expected.json"
            source_path = temp_root / "source.expected.json"
            self.write_json(
                target_path,
                {
                    "sample_id": "VAR-13",
                    "runtime_mirror": {
                        "scope": "wrong-floor",
                        "source_family": "layer-insertion",
                        "source_sample_id": "LI-03",
                        "source_sidecar": str(source_path),
                        "required_source_status": "runtime-floor",
                        "required_source_claims": {
                            "runnable": True,
                            "implemented": True,
                            "active_root": False,
                        },
                        "required_source_current_validation_mode": "rust-runtime-floor",
                        "checked_runtime_mirror_rows": [
                            {
                                "kind": "auth_contract_update_runtime_preview",
                                "target_sample_id": "VAR-13",
                                "source_sample_id": "LI-03",
                                "terminal_outcome": "accepted_contract_update",
                                "accepted_path": "explicit_contract_update",
                                "required_contract_update_ref": "activation_cut#auth_contract_update",
                            }
                        ],
                    },
                },
            )
            self.write_json(
                source_path,
                {
                    "sample_id": "LI-03",
                    "family": "layer-insertion",
                    "status": "runtime-floor",
                    "mirrors": ["VAR-13"],
                    "expected_runtime": {
                        "terminal_outcome": "accepted_contract_update",
                        "accepted_path": "explicit_contract_update",
                        "required_contract_update_ref": "activation_cut#auth_contract_update",
                    },
                    "claims": {
                        "runnable": True,
                        "implemented": True,
                        "active_root": False,
                    },
                    "current_validation": {"mode": "rust-runtime-floor"},
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_runtime_mirror_checker(
                    argv=[str(target_path), str(source_path)],
                    cluster_name="alpha_runtime_mirror_cluster",
                    description="alpha runtime-mirror helper",
                    kinds={"auth_contract_update_runtime_preview"},
                    missing_status="sample_expected_runtime_mirror_missing",
                    expected_scope="alpha-runtime-mirror-floor",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: scope_mismatch", output)
        self.assertIn("expected_runtime_mirror_scope: alpha-runtime-mirror-floor", output)
        self.assertIn("target_runtime_mirror_scope: wrong-floor", output)


if __name__ == "__main__":
    unittest.main()
