#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
import json
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
FULL_SYSTEM_V1_ROOT = REPO_ROOT / "samples" / "full-system-v1"

PROJECTION_SOURCE = (
    FULL_SYSTEM_V1_ROOT
    / "projection"
    / "effectful-sugoroku-positive"
    / "main"
    / "src"
    / "effectful-sugoroku-positive.mir"
)
PROJECTION_REQUEST = (
    FULL_SYSTEM_V1_ROOT
    / "projection"
    / "effectful-sugoroku-positive"
    / "projection.request.json"
)
SPLIT_SOURCE = (
    FULL_SYSTEM_V1_ROOT
    / "server-client"
    / "role-split-positive"
    / "main"
    / "src"
    / "role-split-positive.mir"
)
SPLIT_REQUEST = (
    FULL_SYSTEM_V1_ROOT
    / "server-client"
    / "role-split-positive"
    / "projection.request.json"
)
PROVIDER_SOURCE = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "viewer-diagnostic-positive"
    / "main"
    / "src"
    / "viewer-diagnostic-positive.mir"
)
PROVIDER_REQUEST = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "viewer-diagnostic-positive"
    / "projection.request.json"
)
PROVIDER_MANIFEST = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "viewer-diagnostic-positive"
    / "provider.manifest.json"
)
RENDERER_SOURCE = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "renderer-pose-positive"
    / "main"
    / "src"
    / "renderer-pose-positive.mir"
)
RENDERER_REQUEST = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "renderer-pose-positive"
    / "projection.request.json"
)
RENDERER_PROVIDER = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "renderer-pose-positive"
    / "provider.manifest.json"
)
RENDERER_POSEGRAPH_PACKAGE = (
    FULL_SYSTEM_V1_ROOT
    / "provider-adapter"
    / "renderer-pose-positive"
    / "package.mir.json"
)

VIEWER_SECTIONS = [
    "summary",
    "compatibility-floor",
    "command-results",
    "non-claims",
]

COMPATIBILITY_FLOOR_COMMANDS = [
    "compat:minimal-alpha1",
    "compat:product-alpha1-release-check",
    "compat:operational-product",
]

EXPECTED_PASSED_COUNTS = {
    "helper:textual-mir": 10,
    "helper:full-v1-operational-check": 12,
    "helper:full-v1-check-all": 41,
    "helper:posegraph-runtime": 9,
    "helper:projection-ir": 6,
    "helper:provider-admission": 5,
    "helper:renderer-pose": 3,
}

EXPECTED_RESIDUAL_CODES = {
    "cli:project-full-v1": [
        "packet_ffi_transport_semantics_deferred",
        "provider_admission_deferred",
        "server_client_runtime_split_deferred",
    ],
    "cli:run-full-v1-split": [
        "docker_process_carrier_deferred",
        "packet_ffi_transport_semantics_deferred",
        "provider_admission_deferred",
    ],
    "cli:admit-provider-v1": [
        "docker_process_carrier_deferred",
        "packet_ffi_transport_semantics_deferred",
        "provider_execution_runtime_deferred",
    ],
    "cli:render-pose-backend-v1": [
        "docker_process_carrier_deferred",
        "packet_ffi_transport_semantics_deferred",
        "posegraph_binding_attestation_deferred",
        "provider_execution_runtime_deferred",
        "renderer_vendor_execution_deferred",
    ],
}


@dataclass(frozen=True)
class PlannedCommand:
    name: str
    argv: list[str]
    json_required: bool = True


@dataclass(frozen=True)
class CommandPlan:
    out_dir: Path
    reports_dir: Path
    compat_product_alpha_dir: Path
    bundle_path: Path
    html_path: Path
    commands: list[PlannedCommand]


@dataclass(frozen=True)
class CommandResult:
    name: str
    argv: list[str]
    returncode: int
    stdout: str
    stderr: str
    payload: dict[str, Any] | None
    semantic_errors: list[str]


def cargo_alpha_args(*args: str) -> list[str]:
    return ["cargo", "run", "-q", "-p", "mirrorea-cli", "--", *args, "--format", "json"]


def cargo_test_args(*args: str) -> list[str]:
    return ["cargo", "test", *args, "--", "--nocapture"]


def repo_relative_arg(path: Path) -> str:
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def release_relative_path(path: Path, out_dir: Path) -> str:
    try:
        return path.relative_to(out_dir).as_posix()
    except ValueError:
        return str(path)


def release_display_value(value: Any, out_dir: Path) -> Any:
    if isinstance(value, str):
        path = Path(value)
        if path.is_absolute():
            return release_relative_path(path, out_dir)
        return value
    if isinstance(value, list):
        return [release_display_value(item, out_dir) for item in value]
    if isinstance(value, dict):
        return {
            key: release_display_value(item, out_dir)
            for key, item in value.items()
        }
    return value


def validation_command(name: str, argv: list[str]) -> PlannedCommand:
    return PlannedCommand(name=name, argv=argv, json_required=False)


def helper_command(name: str, script_name: str, *args: str) -> PlannedCommand:
    return PlannedCommand(
        name=name,
        argv=["python3", f"scripts/{script_name}", *args, "--format", "json"],
    )


def plan_check_all(out_dir: Path) -> CommandPlan:
    reports_dir = out_dir / "reports"
    compat_product_alpha_dir = out_dir / "compat-product-alpha1-release"
    bundle_path = out_dir / "bundle.json"
    html_path = out_dir / "index.html"
    commands = [
        validation_command(
            "validation:test-validate-docs",
            ["python3", "-m", "unittest", "scripts.tests.test_validate_docs"],
        ),
        validation_command(
            "validation:source-hierarchy",
            ["python3", "scripts/check_source_hierarchy.py"],
        ),
        validation_command(
            "validation:validate-docs",
            ["python3", "scripts/validate_docs.py"],
        ),
        validation_command("validation:cargo-fmt", ["cargo", "fmt", "--check"]),
        validation_command("validation:git-diff-check", ["git", "diff", "--check"]),
        validation_command(
            "test:release-check",
            ["python3", "-m", "unittest", "scripts.tests.test_full_system_v1_release_check"],
        ),
        validation_command(
            "test:mir-ast-textual-alpha",
            cargo_test_args("-p", "mir-ast", "--test", "textual_mir_alpha"),
        ),
        validation_command(
            "test:mir-semantics-typed-ir",
            cargo_test_args("-p", "mir-semantics", "--test", "typed_ir_interpreter"),
        ),
        validation_command(
            "test:mir-runtime-session",
            cargo_test_args("-p", "mir-runtime", "--test", "full_system_v1_session"),
        ),
        validation_command(
            "test:mir-runtime-posegraph",
            cargo_test_args("-p", "mir-runtime", "--test", "posegraph_runtime"),
        ),
        validation_command(
            "test:mir-runtime-projection",
            cargo_test_args("-p", "mir-runtime", "--test", "projection_ir"),
        ),
        validation_command(
            "test:mir-runtime-provider-admission",
            cargo_test_args("-p", "mir-runtime", "--test", "provider_admission"),
        ),
        validation_command(
            "test:mir-runtime-renderer-pose",
            cargo_test_args("-p", "mir-runtime", "--test", "renderer_pose_backend"),
        ),
        validation_command(
            "test:mirrorea-cli-full-v1",
            cargo_test_args("-p", "mirrorea-cli", "--test", "full_system_v1_cli"),
        ),
        helper_command("helper:textual-mir", "textual_mir_samples.py", "check-all"),
        helper_command(
            "helper:full-v1-operational-matrix",
            "full_system_v1_samples.py",
            "operational-matrix",
        ),
        helper_command(
            "helper:full-v1-operational-check",
            "full_system_v1_samples.py",
            "check-operational-all",
        ),
        helper_command("helper:full-v1-check-all", "full_system_v1_samples.py", "check-all"),
        helper_command("helper:posegraph-runtime", "posegraph_runtime_samples.py", "check-all"),
        helper_command("helper:projection-ir", "projection_v1_samples.py", "check-all"),
        helper_command("helper:provider-admission", "provider_admission_samples.py", "check-all"),
        helper_command(
            "helper:renderer-pose",
            "renderer_pose_backend_samples.py",
            "check-all",
        ),
        helper_command("compat:minimal-alpha1", "minimal_alpha1_patterns.py", "check-all"),
        PlannedCommand(
            "compat:product-alpha1-release-check",
            [
                "python3",
                "scripts/product_alpha1_release_check.py",
                "--format",
                "json",
                "check-all",
                "--out",
                str(compat_product_alpha_dir),
            ],
        ),
        helper_command(
            "compat:operational-product",
            "operational_product_samples.py",
            "check-all",
        ),
        PlannedCommand(
            "cli:project-full-v1",
            cargo_alpha_args(
                "project-full-v1",
                repo_relative_arg(PROJECTION_SOURCE),
                "--request",
                repo_relative_arg(PROJECTION_REQUEST),
            ),
        ),
        PlannedCommand(
            "cli:run-full-v1-split",
            cargo_alpha_args(
                "run-full-v1-split",
                repo_relative_arg(SPLIT_SOURCE),
                "--request",
                repo_relative_arg(SPLIT_REQUEST),
                "--input",
                "40",
            ),
        ),
        PlannedCommand(
            "cli:admit-provider-v1",
            cargo_alpha_args(
                "admit-provider-v1",
                repo_relative_arg(PROVIDER_SOURCE),
                "--request",
                repo_relative_arg(PROVIDER_REQUEST),
                "--provider",
                repo_relative_arg(PROVIDER_MANIFEST),
            ),
        ),
        PlannedCommand(
            "cli:render-pose-backend-v1",
            cargo_alpha_args(
                "render-pose-backend-v1",
                repo_relative_arg(RENDERER_SOURCE),
                "--request",
                repo_relative_arg(RENDERER_REQUEST),
                "--provider",
                repo_relative_arg(RENDERER_PROVIDER),
                "--posegraph-package",
                repo_relative_arg(RENDERER_POSEGRAPH_PACKAGE),
            ),
        ),
    ]
    return CommandPlan(
        out_dir=out_dir,
        reports_dir=reports_dir,
        compat_product_alpha_dir=compat_product_alpha_dir,
        bundle_path=bundle_path,
        html_path=html_path,
        commands=commands,
    )


def run_command(command: PlannedCommand) -> CommandResult:
    completed = subprocess.run(
        command.argv,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    payload = None
    if completed.stdout.strip():
        try:
            payload = json.loads(completed.stdout)
        except json.JSONDecodeError:
            payload = None
    return CommandResult(
        name=command.name,
        argv=command.argv,
        returncode=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        payload=payload,
        semantic_errors=[],
    )


def _list_codes(rows: Any) -> list[str]:
    if not isinstance(rows, list):
        return []
    codes: list[str] = []
    for row in rows:
        if isinstance(row, dict):
            code = row.get("code")
            if isinstance(code, str):
                codes.append(code)
    return sorted(codes)


def _launched_targets(payload: dict[str, Any]) -> list[str]:
    targets = payload.get("target_reports")
    if not isinstance(targets, list):
        targets = (payload.get("local_split_report") or {}).get("target_reports")
    launched: list[str] = []
    if not isinstance(targets, list):
        return launched
    for target in targets:
        if isinstance(target, dict) and target.get("launched_entry_transitions"):
            target_id = target.get("target_id")
            if isinstance(target_id, str):
                launched.append(target_id)
    return sorted(launched)


def _list_strings(rows: Any) -> list[str]:
    if not isinstance(rows, list):
        return []
    values: list[str] = []
    for row in rows:
        if isinstance(row, str):
            values.append(row)
    return values


def command_semantic_errors(command: PlannedCommand, result: CommandResult) -> list[str]:
    if result.returncode != 0:
        return []
    if not command.json_required:
        return []
    if result.payload is None:
        return ["stdout was not valid JSON"]

    payload = result.payload
    errors: list[str] = []

    def expect(condition: bool, message: str) -> None:
        if not condition:
            errors.append(message)

    failed = payload.get("failed")
    if isinstance(failed, list):
        expect(not failed, f"`failed` is not empty: {failed}")
    failed_commands = payload.get("failed_commands")
    if isinstance(failed_commands, list):
        expect(not failed_commands, f"`failed_commands` is not empty: {failed_commands}")
    validation_errors = payload.get("validation_errors")
    if isinstance(validation_errors, list):
        expect(not validation_errors, f"`validation_errors` is not empty: {validation_errors}")
    status = payload.get("status")
    if isinstance(status, str):
        expect(status not in {"error", "failed"}, f"unexpected status `{status}`")

    if command.name == "helper:full-v1-operational-matrix":
        expect(payload.get("sample_count") == 12, "operational matrix sample_count must be 12")
        expect(payload.get("executable_count") == 12, "operational matrix executable_count must be 12")
        expect(payload.get("workflow_ready") is False, "operational matrix must stay evidence-closed")
    elif command.name == "helper:textual-mir":
        expect(payload.get("sample_count") == 10, "textual Mir helper sample_count must be 10")
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "textual Mir helper passed_count mismatch")
        expect(payload.get("workflow_ready") is False, "textual Mir helper must stay evidence-closed")
    elif command.name == "helper:full-v1-operational-check":
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "operational check passed_count mismatch")
    elif command.name == "helper:full-v1-check-all":
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "full V1 check-all passed_count mismatch")
    elif command.name == "helper:posegraph-runtime":
        expect(payload.get("sample_count") == 9, "posegraph helper sample_count must be 9")
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "posegraph helper passed_count mismatch")
        expect(payload.get("workflow_ready") is False, "posegraph helper must stay evidence-closed")
    elif command.name == "helper:projection-ir":
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "projection helper passed_count mismatch")
    elif command.name == "helper:provider-admission":
        expect(payload.get("sample_count") == 5, "provider helper sample_count must be 5")
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "provider helper passed_count mismatch")
        expect(payload.get("workflow_ready") is False, "provider helper must stay evidence-closed")
    elif command.name == "helper:renderer-pose":
        expect(payload.get("sample_count") == 3, "renderer helper sample_count must be 3")
        expect(len(payload.get("passed") or []) == EXPECTED_PASSED_COUNTS[command.name], "renderer helper passed_count mismatch")
        expect(payload.get("workflow_ready") is False, "renderer helper must stay evidence-closed")
    elif command.name == "compat:minimal-alpha1":
        expect(payload.get("status") == "accepted", "minimal alpha-1 verifier must be accepted")
        expect(payload.get("strict_family_count") == 4, "minimal alpha-1 strict_family_count must stay 4")
    elif command.name == "compat:product-alpha1-release-check":
        expect(payload.get("status") == "accepted", "product alpha-1 release check must be accepted")
        expect(
            payload.get("product_alpha1_release_candidate_ready") is True,
            "product alpha-1 release candidate readiness must be true",
        )
        expect(
            len(payload.get("failed_commands") or []) == 0,
            "product alpha-1 release check must not report failed_commands",
        )
        expect(
            len(payload.get("passed_commands") or []) >= 29,
            "product alpha-1 release check passed_commands_count must stay at least 29",
        )
    elif command.name == "compat:operational-product":
        expect(payload.get("status") == "accepted", "operational product suite must be accepted")
        expect(payload.get("docker_included") is True, "operational product suite must include docker replay")
    elif command.name == "cli:project-full-v1":
        expect(payload.get("accepted") is True, "project-full-v1 must be accepted")
        expect(
            payload.get("surface_kind") == "full_system_v1_projection_report",
            "project-full-v1 surface_kind mismatch",
        )
        expect(payload.get("projection_id") == "effectful-sugoroku-projection", "project-full-v1 projection_id mismatch")
        expect(len(payload.get("packet_schemas") or []) == 6, "project-full-v1 packet schema count mismatch")
        expect(len(payload.get("ffi_schemas") or []) == 2, "project-full-v1 ffi schema count mismatch")
        expect(len(payload.get("target_manifests") or []) == 3, "project-full-v1 target count mismatch")
        expect(
            _list_codes(payload.get("residual_obligations")) == EXPECTED_RESIDUAL_CODES[command.name],
            "project-full-v1 residual obligation codes mismatch",
        )
    elif command.name == "cli:run-full-v1-split":
        expect(payload.get("accepted") is True, "run-full-v1-split must be accepted")
        expect(
            payload.get("surface_kind") == "full_system_v1_local_split_report",
            "run-full-v1-split surface_kind mismatch",
        )
        expect(payload.get("projection_id") == "role-split-positive", "run-full-v1-split projection_id mismatch")
        expect(
            payload.get("launch_mode") == "same_binary_local_role_wrapper",
            "run-full-v1-split launch_mode mismatch",
        )
        expect(len(payload.get("target_reports") or []) == 3, "run-full-v1-split target count mismatch")
        expect(
            _launched_targets(payload) == ["world-client", "world-server"],
            "run-full-v1-split launched target set mismatch",
        )
        expect(
            _list_codes(payload.get("residual_obligations")) == EXPECTED_RESIDUAL_CODES[command.name],
            "run-full-v1-split residual obligation codes mismatch",
        )
        expect(_list_strings(payload.get("rejected_rows")) == [], "run-full-v1-split must not report rejected_rows")
    elif command.name == "cli:admit-provider-v1":
        expect(payload.get("accepted") is True, "admit-provider-v1 must be accepted")
        expect(
            payload.get("surface_kind") == "full_system_v1_provider_admission_report",
            "admit-provider-v1 surface_kind mismatch",
        )
        expect(payload.get("projection_id") == "viewer-diagnostic-positive", "admit-provider-v1 projection_id mismatch")
        expect(payload.get("provider_id") == "viewer-diagnostic-exporter", "admit-provider-v1 provider_id mismatch")
        expect(payload.get("provider_kind") == "viewer-diagnostic-exporter", "admit-provider-v1 provider_kind mismatch")
        expect(payload.get("target_id") == "diagnostic-adapter", "admit-provider-v1 target_id mismatch")
        expect(payload.get("terminal_outcome") == "inventory_admitted", "admit-provider-v1 terminal_outcome mismatch")
        expect(payload.get("execution_admitted") is False, "admit-provider-v1 must keep execution disabled")
        expect(
            _list_strings(payload.get("matched_packet_schema_refs")) == [],
            "admit-provider-v1 must not match packet schema refs",
        )
        expect(
            _list_strings(payload.get("matched_ffi_schema_refs")) == ["ffi.diagnostic.export_preview"],
            "admit-provider-v1 matched_ffi_schema_refs mismatch",
        )
        expect(
            _list_codes(payload.get("residual_obligations")) == EXPECTED_RESIDUAL_CODES[command.name],
            "admit-provider-v1 residual obligation codes mismatch",
        )
        expect(_list_strings(payload.get("rejected_rows")) == [], "admit-provider-v1 must not report rejected_rows")
    elif command.name == "cli:render-pose-backend-v1":
        expect(payload.get("accepted") is True, "render-pose-backend-v1 must be accepted")
        expect(payload.get("delivery_admitted") is True, "renderer delivery must be admitted")
        expect(
            payload.get("surface_kind") == "full_system_v1_renderer_pose_backend_report",
            "render-pose-backend-v1 surface_kind mismatch",
        )
        expect(payload.get("projection_id") == "renderer-pose-positive", "renderer pose projection_id mismatch")
        expect(payload.get("provider_id") == "renderer-pose-backend", "renderer pose provider_id mismatch")
        expect(payload.get("provider_kind") == "renderer", "renderer pose provider_kind mismatch")
        expect(payload.get("target_id") == "renderer-adapter", "renderer pose target_id mismatch")
        expect(payload.get("terminal_outcome") == "delivery_admitted", "renderer pose terminal_outcome mismatch")
        expect(payload.get("blocked_reason") is None, "renderer pose blocked_reason must stay null")
        expect(
            payload.get("pose_snapshot_frontier") == "snapshot#avatar-017",
            "renderer pose snapshot frontier mismatch",
        )
        expect(len(payload.get("delivered_nodes") or []) == 2, "renderer pose delivered node count mismatch")
        expect(
            _list_strings(payload.get("matched_packet_schema_refs")) == ["packet.renderer.pose_snapshot"],
            "renderer pose matched_packet_schema_refs mismatch",
        )
        expect(
            _list_strings(payload.get("matched_ffi_schema_refs")) == [],
            "renderer pose matched_ffi_schema_refs must stay empty",
        )
        expect(
            _list_codes(payload.get("residual_obligations")) == EXPECTED_RESIDUAL_CODES[command.name],
            "renderer pose residual obligation codes mismatch",
        )

    return errors


def _sanitize_name(name: str) -> str:
    return name.replace(":", "__")


def summarize_payload(name: str, payload: dict[str, Any]) -> dict[str, Any]:
    summary: dict[str, Any] = {}
    for key in [
        "surface_kind",
        "status",
        "verdict",
        "accepted",
        "terminal_outcome",
        "product_alpha1_release_candidate_ready",
        "product_alpha1_ready",
        "workflow_ready",
        "projection_id",
        "provider_id",
        "provider_kind",
        "target_id",
        "launch_mode",
        "delivery_admitted",
        "execution_admitted",
        "blocked_reason",
        "pose_snapshot_frontier",
    ]:
        if key in payload:
            summary[key] = payload[key]

    if name == "helper:full-v1-operational-matrix":
        summary["sample_count"] = payload.get("sample_count")
        summary["executable_count"] = payload.get("executable_count")
        summary["family_counts"] = payload.get("family_counts")
    elif name in {
        "helper:textual-mir",
        "helper:full-v1-operational-check",
        "helper:full-v1-check-all",
        "helper:posegraph-runtime",
        "helper:projection-ir",
        "helper:provider-admission",
        "helper:renderer-pose",
    }:
        passed = payload.get("passed")
        failed = payload.get("failed")
        if isinstance(passed, list):
            summary["passed_count"] = len(passed)
        if isinstance(failed, list):
            summary["failed"] = failed
    elif name == "compat:minimal-alpha1":
        summary["strict_family_count"] = payload.get("strict_family_count")
    elif name == "compat:product-alpha1-release-check":
        summary["out_dir"] = payload.get("out_dir")
        summary["passed_commands_count"] = len(payload.get("passed_commands") or [])
        summary["failed_commands"] = payload.get("failed_commands") or []
    elif name == "compat:operational-product":
        summary["docker_included"] = payload.get("docker_included")
        summary["portal_runtime_ok"] = payload.get("portal_runtime_ok")
        summary["shard_runtime_ok"] = payload.get("shard_runtime_ok")
        summary["gradient_runtime_ok"] = payload.get("gradient_runtime_ok")
        summary["projection_inventory_ok"] = payload.get("projection_inventory_ok")
    elif name == "cli:project-full-v1":
        summary["packet_schema_count"] = len(payload.get("packet_schemas") or [])
        summary["ffi_schema_count"] = len(payload.get("ffi_schemas") or [])
        summary["target_count"] = len(payload.get("target_manifests") or [])
        summary["residual_obligation_codes"] = _list_codes(payload.get("residual_obligations"))
    elif name == "cli:run-full-v1-split":
        summary["target_count"] = len(payload.get("target_reports") or [])
        summary["launched_targets"] = _launched_targets(payload)
        summary["residual_obligation_codes"] = _list_codes(payload.get("residual_obligations"))
    elif name == "cli:admit-provider-v1":
        summary["matched_packet_schema_refs"] = payload.get("matched_packet_schema_refs") or []
        summary["matched_ffi_schema_refs"] = payload.get("matched_ffi_schema_refs") or []
        summary["residual_obligation_codes"] = _list_codes(payload.get("residual_obligations"))
    elif name == "cli:render-pose-backend-v1":
        summary["delivered_node_count"] = len(payload.get("delivered_nodes") or [])
        summary["matched_packet_schema_refs"] = payload.get("matched_packet_schema_refs") or []
        summary["matched_ffi_schema_refs"] = payload.get("matched_ffi_schema_refs") or []
        summary["residual_obligation_codes"] = _list_codes(payload.get("residual_obligations"))

    if "diagnostics" in payload and isinstance(payload["diagnostics"], list):
        summary["diagnostic_codes"] = _list_codes(payload["diagnostics"])
    if "rejected_rows" in payload and isinstance(payload["rejected_rows"], list):
        summary["rejected_rows"] = payload["rejected_rows"]
    return summary


def command_result_record(
    command: PlannedCommand,
    result: CommandResult,
    reports_dir: Path,
    out_dir: Path,
) -> dict[str, Any]:
    report_path = reports_dir / f"{_sanitize_name(command.name)}.json"
    summary = None if result.payload is None else summarize_payload(command.name, result.payload)
    record = {
        "name": result.name,
        "argv": result.argv,
        "returncode": result.returncode,
        "semantic_errors": result.semantic_errors,
        "report_path": release_relative_path(report_path, out_dir),
        "summary": summary,
    }
    record = release_display_value(record, out_dir)
    report_path.write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return record


def write_bundle(bundle: dict[str, Any], bundle_path: Path, html_path: Path) -> None:
    bundle_path.write_text(json.dumps(bundle, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    rows = []
    for result in bundle["command_results"]:
        row_class = "ok" if not result["semantic_errors"] and result["returncode"] == 0 else "failed"
        rows.append(
            "<tr>"
            f"<td>{html.escape(result['name'])}</td>"
            f"<td class=\"{row_class}\">{'ok' if row_class == 'ok' else 'failed'}</td>"
            f"<td><code>{html.escape(Path(result['report_path']).name)}</code></td>"
            "</tr>"
        )

    html_path.write_text(
        "\n".join(
            [
                "<!doctype html>",
                "<html lang=\"en\">",
                "<head>",
                "  <meta charset=\"utf-8\">",
                "  <title>Full System V1 Release Check</title>",
                "  <style>",
                "    body { font-family: sans-serif; margin: 2rem auto; max-width: 1080px; line-height: 1.5; }",
                "    table { border-collapse: collapse; width: 100%; }",
                "    th, td { border: 1px solid #ccc; padding: 0.5rem; text-align: left; }",
                "    .ok { color: #0a6b2d; }",
                "    .failed { color: #8b1e1e; }",
                "    code { background: #f5f5f5; padding: 0.1rem 0.3rem; }",
                "  </style>",
                "</head>",
                "<body>",
                "  <h1 id=\"summary\">Full System V1 Release Check</h1>",
                f"  <p>Status: <strong>{html.escape(bundle['status'])}</strong></p>",
                "  <h2 id=\"compatibility-floor\">Compatibility Floor</h2>",
                "  <ul>",
                f"    <li>Product Alpha compatibility preserved: {bundle['compatibility_floor_preserved']}</li>",
                f"    <li>Bundle path: <code>{html.escape(bundle['bundle_path'])}</code></li>",
                f"    <li>Viewer sections: <code>{html.escape(', '.join(bundle['viewer_sections']))}</code></li>",
                "  </ul>",
                "  <h2 id=\"command-results\">Command Results</h2>",
                "  <table>",
                "    <thead><tr><th>Command</th><th>Status</th><th>Report</th></tr></thead>",
                f"    <tbody>{''.join(rows)}</tbody>",
                "  </table>",
                "  <h2 id=\"non-claims\">Non-claims</h2>",
                "  <ul>",
                *[f"    <li>{html.escape(item)}</li>" for item in bundle["non_claims"]],
                "  </ul>",
                "</body>",
                "</html>",
            ]
        )
        + "\n",
        encoding="utf-8",
    )


def release_non_claims() -> list[str]:
    return [
        "not final public grammar",
        "not final typed IR or runtime API",
        "not final packet or FFI transport semantics",
        "not final server/client binary split",
        "not arbitrary native or WASM execution",
        "not final engine/provider SDK",
        "not final public viewer/devtools family",
        "not WAN/federation or distributed durable save/load",
    ]


def check_all(out_dir: Path | None = None) -> dict[str, Any]:
    if out_dir is None:
        out_dir = Path(tempfile.mkdtemp(prefix="mirrorea-full-v1-release-"))
    elif out_dir.exists() and any(out_dir.iterdir()):
        return release_display_value(
            {
                "surface_kind": "full_system_v1_release_check_report",
                "status": "error",
                "command": "check-all",
                "diagnostic_code": "output_dir_not_empty",
                "out_dir": str(out_dir),
                "planned_commands": [],
                "passed_commands": [],
                "failed_commands": ["preflight:output-dir-empty"],
                "command_results": [],
                "bundle_path": str(out_dir / "bundle.json"),
                "html_path": str(out_dir / "index.html"),
                "viewer_sections": VIEWER_SECTIONS,
                "release_bundle_built": False,
                "viewer_ready": False,
                "compatibility_floor_preserved": False,
                "full_system_v1_release_check_ready": False,
                "final_public_api_frozen": False,
                "final_public_grammar_frozen": False,
                "non_claims": release_non_claims(),
            },
            out_dir,
        )

    out_dir.mkdir(parents=True, exist_ok=True)
    plan = plan_check_all(out_dir=out_dir)
    plan.reports_dir.mkdir(parents=True, exist_ok=True)

    raw_results = [run_command(command) for command in plan.commands]
    results = [
        CommandResult(
            name=result.name,
            argv=result.argv,
            returncode=result.returncode,
            stdout=result.stdout,
            stderr=result.stderr,
            payload=result.payload,
            semantic_errors=command_semantic_errors(command, result),
        )
        for command, result in zip(plan.commands, raw_results)
    ]
    passed = [
        result.name
        for result in results
        if result.returncode == 0 and not result.semantic_errors
    ]
    failed = [
        result.name
        for result in results
        if result.returncode != 0 or result.semantic_errors
    ]
    compatibility_floor_preserved = all(command_name in passed for command_name in COMPATIBILITY_FLOOR_COMMANDS)
    command_results = [
        command_result_record(command, result, plan.reports_dir, plan.out_dir)
        for command, result in zip(plan.commands, results)
    ]
    status = "accepted" if not failed else "error"
    bundle = release_display_value(
        {
            "surface_kind": "full_system_v1_release_bundle",
            "status": status,
            "out_dir": str(plan.out_dir),
            "compat_product_alpha_dir": str(plan.compat_product_alpha_dir),
            "bundle_path": str(plan.bundle_path),
            "html_path": str(plan.html_path),
            "viewer_sections": VIEWER_SECTIONS,
            "planned_commands": [command.name for command in plan.commands],
            "passed_commands": passed,
            "failed_commands": failed,
            "command_results": command_results,
            "compatibility_floor_preserved": compatibility_floor_preserved,
            "full_system_v1_release_check_ready": not failed,
            "final_public_api_frozen": False,
            "final_public_grammar_frozen": False,
            "viewer_mode": "full_system_v1_nonfinal_static_html_viewer",
            "non_claims": release_non_claims(),
        },
        plan.out_dir,
    )
    write_bundle(bundle, plan.bundle_path, plan.html_path)

    return release_display_value(
        {
            "surface_kind": "full_system_v1_release_check_report",
            "status": status,
            "command": "check-all",
            "out_dir": str(plan.out_dir),
            "reports_dir": str(plan.reports_dir),
            "compat_product_alpha_dir": str(plan.compat_product_alpha_dir),
            "bundle_path": str(plan.bundle_path),
            "html_path": str(plan.html_path),
            "viewer_sections": VIEWER_SECTIONS,
            "planned_commands": [command.name for command in plan.commands],
            "passed_commands": passed,
            "failed_commands": failed,
            "command_results": command_results,
            "release_bundle_built": True,
            "viewer_ready": True,
            "compatibility_floor_preserved": compatibility_floor_preserved,
            "full_system_v1_release_check_ready": not failed,
            "final_public_api_frozen": False,
            "final_public_grammar_frozen": False,
            "non_claims": release_non_claims(),
        },
        plan.out_dir,
    )


def print_payload(payload: dict[str, Any], fmt: str) -> None:
    if fmt == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
        return
    print(f"{payload['status']}: {payload['surface_kind']}")
    for command in payload["planned_commands"]:
        marker = "ok" if command in payload["passed_commands"] else "failed"
        print(f"  {marker}: {command}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["pretty", "json"], default="pretty")
    sub = parser.add_subparsers(dest="command")
    check = sub.add_parser("check-all")
    check.add_argument("--out", type=Path, default=None)
    args = parser.parse_args(argv)
    if args.command != "check-all":
        parser.error("expected subcommand: check-all")
    payload = check_all(out_dir=args.out)
    print_payload(payload, args.format)
    return 0 if payload["status"] == "accepted" else 2


if __name__ == "__main__":
    raise SystemExit(main())
