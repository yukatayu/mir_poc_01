import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import product_alpha1_release_check as runner  # noqa: E402


def empty_out_dir() -> Path:
    return Path(tempfile.mkdtemp(prefix="mirrorea-alpha1-release-unit-"))


class ProductAlpha1ReleaseCheckTests(unittest.TestCase):
    def test_plan_cli_commands_use_repo_relative_sample_paths(self) -> None:
        plan = runner.plan_check_all(
            out_dir=Path("/tmp/mirrorea-alpha1-release"),
            include_docker=True,
        )
        sample_args = [
            arg
            for command in plan.commands
            for arg in command.argv
            if "product-alpha1/demo" in arg
        ]

        self.assertTrue(sample_args)
        self.assertTrue(
            all(arg.startswith("samples/product-alpha1/demo") for arg in sample_args),
            sample_args,
        )
        self.assertFalse(any(arg.startswith(str(REPO_ROOT)) for arg in sample_args))

    def test_check_all_serializes_release_owned_paths_without_host_prefixes(self) -> None:
        root = Path(tempfile.mkdtemp(prefix="mirrorea-alpha1-release-home-"))
        out_dir = root / "home" / "someone" / "mirrorea-alpha1-release"

        def fake_run(command, env=None):
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=json.dumps(payload_for(command.name)),
                stderr=(
                    f"warning at {REPO_ROOT}/samples/product-alpha1/demo "
                    f"and {out_dir}/devtools/bundle.json"
                )
                if command.name == "demo"
                else "",
                payload=payload_for(command.name),
                semantic_errors=[],
            )

        with mock.patch.object(runner, "run_command", side_effect=fake_run):
            payload = runner.check_all(
                out_dir=out_dir,
                include_docker=True,
            )

        text = json.dumps(payload, sort_keys=True)
        self.assertNotIn("/home/", text)
        self.assertNotIn("/Users/", text)
        self.assertEqual(payload["out_dir"], ".")
        self.assertEqual(payload["session_dir"], "session-store")
        self.assertEqual(payload["devtools_dir"], "devtools")
        self.assertEqual(payload["native_bundle_dir"], "native-bundle")
        self.assertEqual(payload["demo_dir"], "demo")
        demo_result = next(
            result for result in payload["command_results"] if result["name"] == "demo"
        )
        self.assertIn("samples/product-alpha1/demo", demo_result["stderr"])
        self.assertIn("devtools/bundle.json", demo_result["stderr"])

    def test_repo_relative_helpers_preserve_external_paths(self) -> None:
        external_path = Path("/opt/external/product-alpha1-demo")

        self.assertEqual(runner.repo_relative_arg(external_path), str(external_path))
        self.assertEqual(
            runner.release_relative_path(external_path, Path("/tmp/mirrorea-alpha1-release")),
            str(external_path),
        )

    def test_plan_commands_includes_full_product_flow(self) -> None:
        plan = runner.plan_check_all(
            out_dir=Path("/tmp/mirrorea-alpha1-release"),
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
                "test:release-check",
                "test:mir-ast-product-schema",
                "test:mir-runtime-session",
                "test:mir-runtime-devtools",
                "test:mirrorea-cli-alpha",
                "check",
                "run-local",
                "session",
                "attach-debug-layer",
                "attach-auth-layer",
                "attach-rate-limit-layer",
                "attach-placeholder-object",
                "attach-custom-avatar-preview",
                "save",
                "load",
                "quiescent-save",
                "transport-local",
                "transport-docker",
                "export-devtools",
                "view",
                "build-native-bundle",
                "native-run-check",
                "native-run-view",
                "demo",
            ],
        )

    def test_check_all_reports_release_candidate_ready_without_final_api_claim(self) -> None:
        def fake_run(command, env=None):
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=json.dumps(payload_for(command.name)),
                stderr="",
                payload=payload_for(command.name),
                semantic_errors=[],
            )

        with mock.patch.object(runner, "run_command", side_effect=fake_run):
            payload = runner.check_all(
                out_dir=empty_out_dir(),
                include_docker=True,
            )
        self.assertEqual(payload["status"], "accepted")
        self.assertTrue(payload["product_alpha1_release_candidate_ready"])
        self.assertTrue(payload["product_alpha1_ready"])
        self.assertFalse(payload["final_public_api_frozen"])
        self.assertFalse(payload["final_product_claimed"])
        self.assertIn("transport-docker", payload["passed_commands"])
        self.assertFalse(any(result["semantic_errors"] for result in payload["command_results"]))
        build_bundle = next(
            result for result in payload["command_results"] if result["name"] == "build-native-bundle"
        )
        self.assertEqual(build_bundle["semantic_errors"], [])

    def test_check_all_skip_docker_is_partial_non_release_probe(self) -> None:
        def fake_run(command, env=None):
            payload = payload_for(command.name)
            if command.name == "demo":
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
        self.assertFalse(payload["product_alpha1_release_candidate_ready"])
        self.assertNotIn("transport-docker", payload["planned_commands"])

    def test_check_all_collects_command_failure(self) -> None:
        def fake_run(command, env=None):
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=2 if command.name == "transport-docker" else 0,
                stdout=json.dumps(payload_for(command.name)),
                stderr="docker unavailable",
                payload=payload_for(command.name),
                semantic_errors=[],
            )

        with mock.patch.object(runner, "run_command", side_effect=fake_run):
            payload = runner.check_all(
                out_dir=empty_out_dir(),
                include_docker=True,
            )
        self.assertEqual(payload["status"], "error")
        self.assertFalse(payload["product_alpha1_release_candidate_ready"])
        self.assertIn("transport-docker", payload["failed_commands"])

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
            self.assertFalse(payload["product_alpha1_release_candidate_ready"])
        finally:
            marker.unlink(missing_ok=True)
            out_dir.rmdir()


def payload_for(name: str) -> dict:
    payloads = {
        "check": {
            "surface_kind": "mirrorea_product_alpha1_check_report",
            "verdict": "accepted",
            "product_alpha1_ready": False,
        },
        "run-local": {
            "surface_kind": "product_alpha1_run_local_report",
            "runtime_plan_emitted": True,
            "typed_host_io_claimed": True,
            "product_alpha1_ready": False,
        },
        "session": {
            "surface_kind": "product_alpha1_session_report",
            "session": {"session_id": "session#product-alpha1-demo"},
        },
        "attach-debug-layer": {
            "surface_kind": "product_alpha1_attach_report",
            "package_id": "product-alpha1-debug-layer",
            "terminal_outcome": "accepted",
        },
        "attach-auth-layer": {
            "surface_kind": "product_alpha1_attach_report",
            "package_id": "product-alpha1-auth-layer",
            "terminal_outcome": "accepted",
        },
        "attach-rate-limit-layer": {
            "surface_kind": "product_alpha1_attach_report",
            "package_id": "product-alpha1-rate-limit-layer",
            "terminal_outcome": "accepted",
        },
        "attach-placeholder-object": {
            "surface_kind": "product_alpha1_attach_report",
            "package_id": "product-alpha1-placeholder-object",
            "terminal_outcome": "deferred",
        },
        "attach-custom-avatar-preview": {
            "surface_kind": "product_alpha1_attach_report",
            "package_id": "product-alpha1-custom-avatar-preview",
            "terminal_outcome": "deferred",
        },
        "save": {"terminal_outcome": "saved", "state_roundtrip_equal": True},
        "load": {"terminal_outcome": "loaded", "loaded_savepoint_class": "R0_Local"},
        "quiescent-save": {
            "terminal_outcome": "saved",
            "no_inflight": True,
            "all_places_sealed": True,
            "no_post_cut_send": True,
        },
        "transport-local": {
            "terminal_outcome": "accepted",
            "local_transport_executed": True,
            "wan_federation_claimed": False,
        },
        "transport-docker": {
            "terminal_outcome": "accepted",
            "docker_compose_tcp_claimed": True,
            "docker_compose_executed": True,
            "wire_roundtrip_executed": True,
            "wan_federation_claimed": False,
        },
        "export-devtools": {
            "surface_kind": "product_alpha1_devtools_export_report",
            "final_public_viewer_frozen": False,
            "durable_audit_claimed": False,
            "panel_ids": [str(index) for index in range(13)],
        },
        "view": {
            "status": "accepted",
            "bundle_valid": True,
            "html_contains_required_panels": True,
        },
        "build-native-bundle": {
            "status": "accepted",
            "host_launch_bundle_claimed": True,
            "package_native_execution_claimed": False,
            "signature_is_safety_claimed": False,
            "shipped_surface": {
                "delivery_model": "installed_binary_plus_native_host_launch_bundle",
                "supported_replay_commands": ["check", "view"],
            },
        },
        "native-run-check": {"verdict": "accepted"},
        "native-run-view": {"status": "accepted"},
        "demo": {
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
