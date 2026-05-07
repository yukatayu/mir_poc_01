import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import product_alpha1_installed_binary_check as runner  # noqa: E402


def empty_out_dir() -> Path:
    return Path(tempfile.mkdtemp(prefix="mirrorea-alpha1-installed-binary-unit-"))


class ProductAlpha1InstalledBinaryCheckTests(unittest.TestCase):
    def test_plan_commands_include_installed_binary_and_bundle_probe(self) -> None:
        plan = runner.plan_check_all(
            out_dir=Path("/tmp/mirrorea-alpha1-installed-binary"),
            include_docker=True,
        )
        names = [command.name for command in plan.commands]
        self.assertEqual(
            names,
            [
                "validation:test-validate-docs",
                "validation:source-hierarchy",
                "validation:validate-docs",
                "validation:cargo-fmt",
                "validation:git-diff-check",
                "build-binary",
                "binary-check",
                "binary-build-native-bundle",
                "bundle-run-check",
                "bundle-run-view",
                "binary-demo",
            ],
        )

    def test_check_all_reports_installed_binary_candidate_without_final_api_claim(self) -> None:
        def fake_run(command, env=None):
            payload = payload_for(command.name)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=json.dumps(payload),
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        with mock.patch.object(runner, "run_command", side_effect=fake_run):
            payload = runner.check_all(
                out_dir=empty_out_dir(),
                include_docker=True,
            )
        self.assertEqual(payload["status"], "accepted")
        self.assertTrue(payload["installed_binary_candidate_ready"])
        self.assertEqual(
            payload["public_packaging_candidate"],
            "installed_binary_plus_native_host_launch_bundle",
        )
        self.assertFalse(payload["final_public_api_frozen"])
        self.assertEqual(
            payload["compatibility_scope"]["package_format"],
            "versioned_package_mir_json",
        )
        self.assertEqual(
            payload["compatibility_scope"]["cli_surface"],
            "mirrorea_alpha_documented_command_family",
        )
        self.assertFalse(payload["compatibility_scope"]["final_textual_mir_grammar_frozen"])
        self.assertFalse(payload["compatibility_scope"]["final_rust_library_abi_frozen"])
        self.assertNotIn("binary-demo", payload["failed_commands"])

    def test_check_all_skip_docker_is_partial_non_release_probe(self) -> None:
        def fake_run(command, env=None):
            payload = payload_for(command.name)
            if command.name == "binary-demo":
                payload = {
                    **payload,
                    "status": "partial",
                    "docker_transport_included": False,
                    "docker_transport_status": "skipped_by_flag_non_release",
                    "product_alpha1_release_candidate_ready": False,
                    "product_alpha1_ready": False,
                }
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=json.dumps(payload),
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        with mock.patch.object(runner, "run_command", side_effect=fake_run):
            payload = runner.check_all(
                out_dir=empty_out_dir(),
                include_docker=False,
            )
        self.assertEqual(payload["status"], "partial")
        self.assertFalse(payload["installed_binary_candidate_ready"])
        self.assertIn("Docker Compose TCP transport skipped", "\n".join(payload["non_claims"]))

    def test_check_all_collects_command_failure(self) -> None:
        def fake_run(command, env=None):
            payload = payload_for(command.name)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=5 if command.name == "build-binary" else 0,
                stdout=json.dumps(payload),
                stderr="cargo build failed",
                payload=payload,
                semantic_errors=[],
            )

        with mock.patch.object(runner, "run_command", side_effect=fake_run):
            payload = runner.check_all(
                out_dir=empty_out_dir(),
                include_docker=True,
            )
        self.assertEqual(payload["status"], "error")
        self.assertFalse(payload["installed_binary_candidate_ready"])
        self.assertIn("build-binary", payload["failed_commands"])

    def test_check_all_rejects_non_empty_output_root_before_running_commands(self) -> None:
        out_dir = empty_out_dir()
        marker = out_dir / "stale.json"
        marker.write_text("{}\n")
        try:
            with mock.patch.object(runner, "run_command") as run_command:
                payload = runner.check_all(out_dir=out_dir, include_docker=True)
            run_command.assert_not_called()
            self.assertEqual(payload["status"], "error")
            self.assertEqual(payload["diagnostic_code"], "output_dir_not_empty")
            self.assertFalse(payload["installed_binary_candidate_ready"])
            self.assertEqual(
                payload["compatibility_scope"]["bundle_surface"],
                "native_host_launch_bundle_run_sh",
            )
        finally:
            marker.unlink(missing_ok=True)
            out_dir.rmdir()


def payload_for(name: str) -> dict:
    payloads = {
        "build-binary": {"status": "accepted"},
        "binary-check": {
            "surface_kind": "mirrorea_product_alpha1_check_report",
            "verdict": "accepted",
            "product_alpha1_ready": False,
        },
        "binary-build-native-bundle": {
            "status": "accepted",
            "host_launch_bundle_claimed": True,
            "package_native_execution_claimed": False,
            "signature_is_safety_claimed": False,
        },
        "bundle-run-check": {"verdict": "accepted"},
        "bundle-run-view": {"status": "accepted"},
        "binary-demo": {
            "surface_kind": "product_alpha1_demo_report",
            "status": "accepted",
            "same_session_reopen_checked": True,
            "attach_matrix_verified": True,
            "complete_redaction_proof_claimed": False,
            "docker_transport_included": True,
            "docker_transport_status": "accepted",
            "product_alpha1_release_candidate_ready": True,
            "product_alpha1_ready": True,
        },
    }
    return payloads.get(name, {"status": "accepted"})


if __name__ == "__main__":
    unittest.main()
