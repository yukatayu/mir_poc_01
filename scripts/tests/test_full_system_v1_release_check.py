from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import full_system_v1_release_check as runner  # noqa: E402


def empty_out_dir() -> Path:
    return Path(tempfile.mkdtemp(prefix="mirrorea-full-v1-release-unit-"))


class FullSystemV1ReleaseCheckTests(unittest.TestCase):
    def test_plan_commands_include_full_source_first_flow(self) -> None:
        plan = runner.plan_check_all(out_dir=Path("/tmp/mirrorea-full-v1-release"))
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
                "test:mir-ast-textual-alpha",
                "test:mir-semantics-typed-ir",
                "test:mir-runtime-session",
                "test:mir-runtime-posegraph",
                "test:mir-runtime-projection",
                "test:mir-runtime-provider-admission",
                "test:mir-runtime-renderer-pose",
                "test:mirrorea-cli-full-v1",
                "helper:textual-mir",
                "helper:full-v1-operational-matrix",
                "helper:full-v1-operational-check",
                "helper:full-v1-check-all",
                "helper:posegraph-runtime",
                "helper:projection-ir",
                "helper:provider-admission",
                "helper:renderer-pose",
                "compat:minimal-alpha1",
                "compat:product-alpha1-release-check",
                "compat:operational-product",
                "cli:project-full-v1",
                "cli:run-full-v1-split",
                "cli:admit-provider-v1",
                "cli:render-pose-backend-v1",
            ],
        )

    def test_plan_cli_commands_use_repo_relative_sample_paths(self) -> None:
        plan = runner.plan_check_all(out_dir=Path("/tmp/mirrorea-full-v1-release"))
        cli_commands = [
            command for command in plan.commands if command.name.startswith("cli:")
        ]
        sample_args = [
            arg
            for command in cli_commands
            for arg in command.argv
            if "samples/full-system-v1" in arg
        ]

        self.assertTrue(sample_args)
        for arg in sample_args:
            self.assertFalse(Path(arg).is_absolute(), arg)
            self.assertFalse(arg.startswith(str(REPO_ROOT)), arg)

    def test_release_outputs_do_not_serialize_home_shaped_output_paths(self) -> None:
        def fake_run(command: runner.PlannedCommand) -> runner.CommandResult:
            payload = payload_for(command.name)
            stdout = "" if payload is None else json.dumps(payload)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=stdout,
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        with tempfile.TemporaryDirectory(prefix="mirrorea-release-path-test-") as tmpdir:
            out_dir = Path(tmpdir) / "home" / "codex" / "release"
            with mock.patch.object(runner, "run_command", side_effect=fake_run):
                payload = runner.check_all(out_dir=out_dir)

            self.assertEqual(payload["status"], "accepted")
            paths_to_scan = [
                out_dir / "bundle.json",
                out_dir / "index.html",
                *sorted((out_dir / "reports").glob("*.json")),
            ]
            self.assertTrue(paths_to_scan)
            for path in paths_to_scan:
                text = path.read_text(encoding="utf-8")
                self.assertNotIn("/home/", text, str(path))
                self.assertNotIn("/Users/", text, str(path))

    def test_repo_relative_helpers_preserve_external_paths(self) -> None:
        external_path = Path("/var/tmp/mirrorea-external/sample.mir")

        self.assertEqual(runner.repo_relative_arg(external_path), str(external_path))

    def test_check_all_builds_bundle_and_viewer(self) -> None:
        def fake_run(command: runner.PlannedCommand) -> runner.CommandResult:
            payload = payload_for(command.name)
            stdout = "" if payload is None else json.dumps(payload)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=stdout,
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        out_dir = empty_out_dir()
        try:
            with mock.patch.object(runner, "run_command", side_effect=fake_run):
                payload = runner.check_all(out_dir=out_dir)
            self.assertEqual(payload["status"], "accepted")
            self.assertTrue(payload["compatibility_floor_preserved"])
            self.assertTrue(payload["full_system_v1_release_check_ready"])
            self.assertTrue(payload["release_bundle_built"])
            self.assertTrue(payload["viewer_ready"])
            self.assertTrue((out_dir / "bundle.json").exists())
            self.assertTrue((out_dir / "index.html").exists())
            self.assertTrue((out_dir / "reports").exists())
            self.assertIn("command-results", (out_dir / "index.html").read_text(encoding="utf-8"))
        finally:
            for path in sorted(out_dir.rglob("*"), reverse=True):
                if path.is_file():
                    path.unlink()
                elif path.is_dir():
                    path.rmdir()
            out_dir.rmdir()

    def test_check_all_collects_semantic_failure(self) -> None:
        def fake_run(command: runner.PlannedCommand) -> runner.CommandResult:
            payload = payload_for(command.name)
            if command.name == "compat:product-alpha1-release-check":
                payload = {
                    "status": "accepted",
                    "product_alpha1_release_candidate_ready": False,
                    "failed_commands": [],
                }
            stdout = "" if payload is None else json.dumps(payload)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=stdout,
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        out_dir = empty_out_dir()
        try:
            with mock.patch.object(runner, "run_command", side_effect=fake_run):
                payload = runner.check_all(out_dir=out_dir)
            self.assertEqual(payload["status"], "error")
            self.assertIn("compat:product-alpha1-release-check", payload["failed_commands"])
            failed_result = next(
                result
                for result in payload["command_results"]
                if result["name"] == "compat:product-alpha1-release-check"
            )
            self.assertTrue(failed_result["semantic_errors"])
        finally:
            for path in sorted(out_dir.rglob("*"), reverse=True):
                if path.is_file():
                    path.unlink()
                elif path.is_dir():
                    path.rmdir()
            out_dir.rmdir()

    def test_check_all_rejects_helper_count_drift(self) -> None:
        def fake_run(command: runner.PlannedCommand) -> runner.CommandResult:
            payload = payload_for(command.name)
            if command.name == "helper:full-v1-check-all":
                payload = {
                    "failed": [],
                    "validation_errors": [],
                    "passed": [f"row-{index}" for index in range(40)],
                }
            stdout = "" if payload is None else json.dumps(payload)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=stdout,
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        out_dir = empty_out_dir()
        try:
            with mock.patch.object(runner, "run_command", side_effect=fake_run):
                payload = runner.check_all(out_dir=out_dir)
            self.assertEqual(payload["status"], "error")
            self.assertIn("helper:full-v1-check-all", payload["failed_commands"])
        finally:
            for path in sorted(out_dir.rglob("*"), reverse=True):
                if path.is_file():
                    path.unlink()
                elif path.is_dir():
                    path.rmdir()
            out_dir.rmdir()

    def test_check_all_rejects_malformed_split_cli_payload(self) -> None:
        def fake_run(command: runner.PlannedCommand) -> runner.CommandResult:
            payload = payload_for(command.name)
            if command.name == "cli:run-full-v1-split":
                payload = {
                    "accepted": True,
                    "surface_kind": "full_system_v1_local_split_report",
                    "projection_id": "role-split-positive",
                    "launch_mode": "same_binary_local_role_wrapper",
                    "target_reports": [
                        {"target_id": "world-server", "launched_entry_transitions": ["main"]},
                    ],
                    "rejected_rows": [],
                    "residual_obligations": [{"code": "docker_process_carrier_deferred"}],
                }
            stdout = "" if payload is None else json.dumps(payload)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=stdout,
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        out_dir = empty_out_dir()
        try:
            with mock.patch.object(runner, "run_command", side_effect=fake_run):
                payload = runner.check_all(out_dir=out_dir)
            self.assertEqual(payload["status"], "error")
            self.assertIn("cli:run-full-v1-split", payload["failed_commands"])
        finally:
            for path in sorted(out_dir.rglob("*"), reverse=True):
                if path.is_file():
                    path.unlink()
                elif path.is_dir():
                    path.rmdir()
            out_dir.rmdir()

    def test_check_all_marks_compatibility_floor_false_when_minimal_alpha_fails(self) -> None:
        def fake_run(command: runner.PlannedCommand) -> runner.CommandResult:
            payload = payload_for(command.name)
            if command.name == "compat:minimal-alpha1":
                payload = {
                    "status": "error",
                    "strict_family_count": 4,
                }
            stdout = "" if payload is None else json.dumps(payload)
            return runner.CommandResult(
                name=command.name,
                argv=command.argv,
                returncode=0,
                stdout=stdout,
                stderr="",
                payload=payload,
                semantic_errors=[],
            )

        out_dir = empty_out_dir()
        try:
            with mock.patch.object(runner, "run_command", side_effect=fake_run):
                payload = runner.check_all(out_dir=out_dir)
            self.assertEqual(payload["status"], "error")
            self.assertFalse(payload["compatibility_floor_preserved"])
            self.assertIn("compat:minimal-alpha1", payload["failed_commands"])
        finally:
            for path in sorted(out_dir.rglob("*"), reverse=True):
                if path.is_file():
                    path.unlink()
                elif path.is_dir():
                    path.rmdir()
            out_dir.rmdir()

    def test_check_all_rejects_non_empty_output_root_before_running_commands(self) -> None:
        out_dir = empty_out_dir()
        marker = out_dir / "stale.json"
        marker.write_text("{}\n", encoding="utf-8")
        try:
            with mock.patch.object(runner, "run_command") as run_command:
                payload = runner.check_all(out_dir=out_dir)
            run_command.assert_not_called()
            self.assertEqual(payload["status"], "error")
            self.assertEqual(payload["diagnostic_code"], "output_dir_not_empty")
            self.assertFalse(payload["full_system_v1_release_check_ready"])
        finally:
            marker.unlink(missing_ok=True)
            out_dir.rmdir()


def payload_for(name: str) -> dict | None:
    payloads: dict[str, dict | None] = {
        "validation:test-validate-docs": None,
        "validation:source-hierarchy": None,
        "validation:validate-docs": None,
        "validation:cargo-fmt": None,
        "validation:git-diff-check": None,
        "test:release-check": None,
        "test:mir-ast-textual-alpha": None,
        "test:mir-semantics-typed-ir": None,
        "test:mir-runtime-session": None,
        "test:mir-runtime-posegraph": None,
        "test:mir-runtime-projection": None,
        "test:mir-runtime-provider-admission": None,
        "test:mir-runtime-renderer-pose": None,
        "test:mirrorea-cli-full-v1": None,
        "helper:textual-mir": {
            "sample_count": 10,
            "failed": [],
            "validation_errors": [],
            "passed": [f"textual-{index}" for index in range(10)],
            "workflow_ready": False,
        },
        "helper:full-v1-operational-matrix": {
            "sample_count": 12,
            "executable_count": 12,
            "workflow_ready": False,
            "validation_errors": [],
            "family_counts": [],
        },
        "helper:full-v1-operational-check": {
            "failed": [],
            "validation_errors": [],
            "passed": [f"row-{index}" for index in range(12)],
        },
        "helper:full-v1-check-all": {
            "failed": [],
            "validation_errors": [],
            "passed": [f"row-{index}" for index in range(41)],
        },
        "helper:posegraph-runtime": {
            "sample_count": 9,
            "failed": [],
            "validation_errors": [],
            "passed": [f"pose-{index}" for index in range(9)],
            "workflow_ready": False,
        },
        "helper:projection-ir": {
            "failed": [],
            "validation_errors": [],
            "passed": [f"projection-{index}" for index in range(6)],
        },
        "helper:provider-admission": {
            "sample_count": 5,
            "failed": [],
            "validation_errors": [],
            "passed": [f"provider-{index}" for index in range(5)],
            "workflow_ready": False,
        },
        "helper:renderer-pose": {
            "sample_count": 3,
            "failed": [],
            "validation_errors": [],
            "passed": [f"renderer-{index}" for index in range(3)],
            "workflow_ready": False,
        },
        "compat:minimal-alpha1": {
            "status": "accepted",
            "strict_family_count": 4,
        },
        "compat:product-alpha1-release-check": {
            "status": "accepted",
            "product_alpha1_release_candidate_ready": True,
            "failed_commands": [],
            "passed_commands": [f"command-{index}" for index in range(29)],
            "out_dir": "/tmp/mirrorea-alpha1-release-unit",
        },
        "compat:operational-product": {
            "status": "accepted",
            "portal_runtime_ok": True,
            "shard_runtime_ok": True,
            "gradient_runtime_ok": True,
            "projection_inventory_ok": True,
            "docker_included": True,
            "failed_commands": [],
        },
        "cli:project-full-v1": {
            "accepted": True,
            "surface_kind": "full_system_v1_projection_report",
            "projection_id": "effectful-sugoroku-projection",
            "packet_schemas": [1, 2, 3, 4, 5, 6],
            "ffi_schemas": [1, 2],
            "target_manifests": [1, 2, 3],
            "residual_obligations": [
                {"code": "packet_ffi_transport_semantics_deferred"},
                {"code": "provider_admission_deferred"},
                {"code": "server_client_runtime_split_deferred"},
            ],
        },
        "cli:run-full-v1-split": {
            "accepted": True,
            "surface_kind": "full_system_v1_local_split_report",
            "projection_id": "role-split-positive",
            "launch_mode": "same_binary_local_role_wrapper",
            "target_reports": [
                {"target_id": "world-server", "launched_entry_transitions": ["main"]},
                {"target_id": "world-client", "launched_entry_transitions": ["render_preview"]},
                {"target_id": "observer-panel", "launched_entry_transitions": []},
            ],
            "residual_obligations": [
                {"code": "docker_process_carrier_deferred"},
                {"code": "packet_ffi_transport_semantics_deferred"},
                {"code": "provider_admission_deferred"},
            ],
            "rejected_rows": [],
        },
        "cli:admit-provider-v1": {
            "accepted": True,
            "surface_kind": "full_system_v1_provider_admission_report",
            "projection_id": "viewer-diagnostic-positive",
            "provider_id": "viewer-diagnostic-exporter",
            "provider_kind": "viewer-diagnostic-exporter",
            "target_id": "diagnostic-adapter",
            "terminal_outcome": "inventory_admitted",
            "execution_admitted": False,
            "matched_packet_schema_refs": [],
            "matched_ffi_schema_refs": ["ffi.diagnostic.export_preview"],
            "residual_obligations": [
                {"code": "docker_process_carrier_deferred"},
                {"code": "packet_ffi_transport_semantics_deferred"},
                {"code": "provider_execution_runtime_deferred"},
            ],
            "diagnostics": [],
            "rejected_rows": [],
        },
        "cli:render-pose-backend-v1": {
            "accepted": True,
            "delivery_admitted": True,
            "surface_kind": "full_system_v1_renderer_pose_backend_report",
            "projection_id": "renderer-pose-positive",
            "provider_id": "renderer-pose-backend",
            "provider_kind": "renderer",
            "target_id": "renderer-adapter",
            "terminal_outcome": "delivery_admitted",
            "blocked_reason": None,
            "pose_snapshot_frontier": "snapshot#avatar-017",
            "delivered_nodes": [{"entity_ref": "avatar#017/head"}, {"entity_ref": "avatar#017/body"}],
            "matched_packet_schema_refs": ["packet.renderer.pose_snapshot"],
            "matched_ffi_schema_refs": [],
            "residual_obligations": [
                {"code": "docker_process_carrier_deferred"},
                {"code": "packet_ffi_transport_semantics_deferred"},
                {"code": "posegraph_binding_attestation_deferred"},
                {"code": "provider_execution_runtime_deferred"},
                {"code": "renderer_vendor_execution_deferred"},
            ],
            "diagnostics": [],
        },
    }
    return payloads[name]


if __name__ == "__main__":
    unittest.main()
