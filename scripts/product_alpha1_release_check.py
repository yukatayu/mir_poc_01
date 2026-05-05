#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PACKAGE = REPO_ROOT / "samples/product-alpha1/demo"


@dataclass(frozen=True)
class PlannedCommand:
    name: str
    argv: list[str]
    json_required: bool = True


@dataclass(frozen=True)
class CommandPlan:
    out_dir: Path
    session_dir: Path
    devtools_dir: Path
    native_bundle_dir: Path
    demo_dir: Path
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


def validation_command(name: str, argv: list[str]) -> PlannedCommand:
    return PlannedCommand(name=name, argv=argv, json_required=False)


def plan_check_all(out_dir: Path, include_docker: bool = True) -> CommandPlan:
    session_dir = out_dir / "session-store"
    devtools_dir = out_dir / "devtools"
    native_bundle_dir = out_dir / "native-bundle"
    demo_dir = out_dir / "demo"
    session_id = "session#product-alpha1-demo"
    package = str(DEFAULT_PACKAGE)
    commands = [
        validation_command(
            "validation:test-validate-docs",
            ["python3", "-m", "unittest", "scripts.tests.test_validate_docs"],
        ),
        validation_command("validation:source-hierarchy", ["python3", "scripts/check_source_hierarchy.py"]),
        validation_command("validation:validate-docs", ["python3", "scripts/validate_docs.py"]),
        validation_command("validation:cargo-fmt", ["cargo", "fmt", "--check"]),
        validation_command("validation:git-diff-check", ["git", "diff", "--check"]),
        validation_command(
            "test:release-check",
            ["python3", "-m", "unittest", "scripts.tests.test_product_alpha1_release_check"],
        ),
        validation_command(
            "test:mir-ast-product-schema",
            cargo_test_args("-p", "mir-ast", "--test", "product_alpha1_package_schema"),
        ),
        validation_command(
            "test:mir-runtime-session",
            cargo_test_args("-p", "mir-runtime", "--test", "product_alpha1_session"),
        ),
        validation_command(
            "test:mir-runtime-devtools",
            cargo_test_args("-p", "mir-runtime", "--test", "product_alpha1_transport_devtools"),
        ),
        validation_command(
            "test:mirrorea-cli-alpha",
            cargo_test_args("-p", "mirrorea-cli", "--test", "alpha_cli"),
        ),
        PlannedCommand("check", cargo_alpha_args("check", package)),
        PlannedCommand("run-local", cargo_alpha_args("run-local", package)),
        PlannedCommand("session", cargo_alpha_args("session", session_id)),
        PlannedCommand(
            "attach-debug-layer",
            cargo_alpha_args("attach", session_id, package + "/packages/debug-layer"),
        ),
        PlannedCommand(
            "attach-auth-layer",
            cargo_alpha_args("attach", session_id, package + "/packages/auth-layer"),
        ),
        PlannedCommand(
            "attach-rate-limit-layer",
            cargo_alpha_args("attach", session_id, package + "/packages/rate-limit-layer"),
        ),
        PlannedCommand(
            "attach-placeholder-object",
            cargo_alpha_args("attach", session_id, package + "/packages/placeholder-object"),
        ),
        PlannedCommand(
            "attach-custom-avatar-preview",
            cargo_alpha_args("attach", session_id, package + "/packages/custom-avatar-preview"),
        ),
        PlannedCommand("save", cargo_alpha_args("save", session_id, "--savepoint", "savepoint#r0-release")),
        PlannedCommand(
            "load",
            cargo_alpha_args("load", "savepoint#r0-release", "--session", session_id),
        ),
        PlannedCommand(
            "quiescent-save",
            cargo_alpha_args(
                "quiescent-save",
                session_id,
                "--savepoint",
                "savepoint#r2-release",
            ),
        ),
        PlannedCommand("transport-local", cargo_alpha_args("transport", session_id, "--mode", "local")),
    ]
    if include_docker:
        commands.append(
            PlannedCommand("transport-docker", cargo_alpha_args("transport", session_id, "--mode", "docker"))
        )
    commands.extend(
        [
            PlannedCommand(
                "export-devtools",
                cargo_alpha_args("export-devtools", session_id, "--out", str(devtools_dir)),
            ),
            PlannedCommand("view", cargo_alpha_args("view", str(devtools_dir), "--check")),
            PlannedCommand(
                "build-native-bundle",
                cargo_alpha_args("build-native-bundle", package, "--out", str(native_bundle_dir)),
            ),
            PlannedCommand(
                "native-run-check",
                ["sh", str(native_bundle_dir / "run.sh"), "check"],
            ),
            PlannedCommand(
                "native-run-view",
                ["sh", str(native_bundle_dir / "run.sh"), "view"],
            ),
            PlannedCommand(
                "demo",
                cargo_alpha_args("demo", package, "--out", str(demo_dir))
                if include_docker
                else cargo_alpha_args("demo", package, "--out", str(demo_dir), "--skip-docker"),
            ),
        ]
    )
    return CommandPlan(
        out_dir=out_dir,
        session_dir=session_dir,
        devtools_dir=devtools_dir,
        native_bundle_dir=native_bundle_dir,
        demo_dir=demo_dir,
        commands=commands,
    )


def run_command(command: PlannedCommand, env: dict[str, str] | None = None) -> CommandResult:
    completed = subprocess.run(
        command.argv,
        cwd=REPO_ROOT,
        env=env,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
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


def command_semantic_errors(command: PlannedCommand, result: CommandResult, include_docker: bool) -> list[str]:
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

    if command.name == "check":
        expect(payload.get("surface_kind") == "mirrorea_product_alpha1_check_report", "wrong check surface_kind")
        expect(payload.get("verdict") == "accepted", "check verdict is not accepted")
        expect(payload.get("product_alpha1_ready") is False, "check must not claim product readiness")
    elif command.name == "run-local":
        expect(payload.get("surface_kind") == "product_alpha1_run_local_report", "wrong run-local surface_kind")
        expect(payload.get("runtime_plan_emitted") is True, "runtime plan missing")
        expect(payload.get("typed_host_io_claimed") is True, "typed host I/O evidence missing")
        expect(payload.get("product_alpha1_ready") is False, "run-local must not claim product readiness")
    elif command.name == "session":
        expect(payload.get("surface_kind") == "product_alpha1_session_report", "wrong session surface_kind")
        expect(payload.get("session", {}).get("session_id") == "session#product-alpha1-demo", "session id mismatch")
    elif command.name.startswith("attach-"):
        expected = {
            "attach-debug-layer": ("product-alpha1-debug-layer", "accepted"),
            "attach-auth-layer": ("product-alpha1-auth-layer", "accepted"),
            "attach-rate-limit-layer": ("product-alpha1-rate-limit-layer", "accepted"),
            "attach-placeholder-object": ("product-alpha1-placeholder-object", "deferred"),
            "attach-custom-avatar-preview": ("product-alpha1-custom-avatar-preview", "deferred"),
        }[command.name]
        expect(payload.get("surface_kind") == "product_alpha1_attach_report", "wrong attach surface_kind")
        expect(payload.get("package_id") == expected[0], "attach package mismatch")
        expect(payload.get("terminal_outcome") == expected[1], "attach terminal outcome mismatch")
    elif command.name == "save":
        expect(payload.get("terminal_outcome") == "saved", "save terminal outcome mismatch")
        expect(payload.get("state_roundtrip_equal") is True, "save roundtrip evidence missing")
    elif command.name == "load":
        expect(payload.get("terminal_outcome") == "loaded", "load terminal outcome mismatch")
        expect(payload.get("loaded_savepoint_class") == "R0_Local", "load savepoint class mismatch")
    elif command.name == "quiescent-save":
        expect(payload.get("terminal_outcome") == "saved", "quiescent-save terminal outcome mismatch")
        expect(payload.get("no_inflight") is True, "NoInFlight evidence missing")
        expect(payload.get("all_places_sealed") is True, "AllPlacesSealed evidence missing")
        expect(payload.get("no_post_cut_send") is True, "NoPostCutSend evidence missing")
    elif command.name == "transport-local":
        expect(payload.get("terminal_outcome") == "accepted", "local transport not accepted")
        expect(payload.get("local_transport_executed") is True, "local transport execution missing")
        expect(payload.get("wan_federation_claimed") is False, "transport must not claim WAN/federation")
    elif command.name == "transport-docker":
        expect(payload.get("terminal_outcome") == "accepted", "docker transport not accepted")
        expect(payload.get("docker_compose_tcp_claimed") is True, "Docker Compose TCP claim missing")
        expect(payload.get("docker_compose_executed") is True, "Docker Compose execution missing")
        expect(payload.get("wire_roundtrip_executed") is True, "Docker wire roundtrip missing")
        expect(payload.get("wan_federation_claimed") is False, "docker transport must not claim WAN/federation")
    elif command.name == "export-devtools":
        expect(payload.get("surface_kind") == "product_alpha1_devtools_export_report", "wrong devtools surface_kind")
        expect(payload.get("final_public_viewer_frozen") is False, "viewer must not claim final public freeze")
        expect(payload.get("durable_audit_claimed") is False, "devtools must not claim durable audit")
        expect(len(payload.get("panel_ids", [])) >= 13, "required devtools panels missing")
    elif command.name == "view":
        expect(payload.get("status") == "accepted", "viewer check not accepted")
        expect(payload.get("bundle_valid") is True, "viewer bundle invalid")
        expect(payload.get("html_contains_required_panels") is True, "viewer HTML missing required panels")
    elif command.name == "build-native-bundle":
        expect(payload.get("status") == "accepted", "native bundle not accepted")
        expect(payload.get("host_launch_bundle_claimed") is True, "host launch bundle missing")
        expect(payload.get("package_native_execution_claimed") is False, "native package execution overclaimed")
        expect(payload.get("signature_is_safety_claimed") is False, "signature safety overclaimed")
    elif command.name in {"native-run-check", "native-run-view"}:
        expect(payload.get("status") == "accepted" or payload.get("verdict") == "accepted", f"{command.name} not accepted")
    elif command.name == "demo":
        expect(payload.get("surface_kind") == "product_alpha1_demo_report", "wrong demo surface_kind")
        expect(payload.get("same_session_reopen_checked") is True, "demo session store reopen evidence missing")
        expect(payload.get("attach_matrix_verified") is True, "demo attach matrix not verified")
        expect(payload.get("complete_redaction_proof_claimed") is False, "demo must not claim complete redaction proof")
        if include_docker:
            expect(payload.get("status") == "accepted", "full demo not accepted")
            expect(payload.get("docker_transport_included") is True, "full demo skipped Docker")
            expect(payload.get("docker_transport_status") == "accepted", "Docker demo status mismatch")
            expect(payload.get("product_alpha1_release_candidate_ready") is True, "full demo not release-candidate ready")
        else:
            expect(payload.get("product_alpha1_release_candidate_ready") is False, "skip-docker demo overclaimed readiness")
    return errors


def release_non_claims(include_docker: bool) -> list[str]:
    claims = [
        "not final public CLI/API/ABI",
        "not final textual .mir grammar",
        "not WAN/federation",
        "not distributed durable save/load",
        "not arbitrary native package execution",
        "not signature-is-safety",
        "not final public viewer or telemetry service",
    ]
    if not include_docker:
        claims.append("Docker Compose TCP transport skipped; this is a local probe, not release-candidate evidence")
    return claims


def check_all(out_dir: Path | None = None, include_docker: bool = True) -> dict[str, Any]:
    if out_dir is None:
        out_dir = Path(tempfile.mkdtemp(prefix="mirrorea-alpha1-release-"))
    elif out_dir.exists() and any(out_dir.iterdir()):
        return {
            "surface_kind": "product_alpha1_release_check_report",
            "status": "error",
            "command": "check-all",
            "diagnostic_code": "output_dir_not_empty",
            "out_dir": str(out_dir),
            "include_docker": include_docker,
            "planned_commands": [],
            "passed_commands": [],
            "failed_commands": ["preflight:output-dir-empty"],
            "command_results": [],
            "product_alpha1_release_candidate_ready": False,
            "product_alpha1_ready": False,
            "final_product_claimed": False,
            "final_public_api_frozen": False,
            "non_claims": release_non_claims(include_docker),
        }
    out_dir.mkdir(parents=True, exist_ok=True)
    plan = plan_check_all(out_dir=out_dir, include_docker=include_docker)
    plan.session_dir.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env["MIRROREA_ALPHA_SESSION_DIR"] = str(plan.session_dir)

    raw_results = [run_command(command, env=env) for command in plan.commands]
    results = [
        CommandResult(
            name=result.name,
            argv=result.argv,
            returncode=result.returncode,
            stdout=result.stdout,
            stderr=result.stderr,
            payload=result.payload,
            semantic_errors=command_semantic_errors(command, result, include_docker),
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
    status = "accepted" if not failed and include_docker else "partial" if not failed else "error"
    release_ready = not failed and include_docker
    return {
        "surface_kind": "product_alpha1_release_check_report",
        "status": status,
        "command": "check-all",
        "out_dir": str(out_dir),
        "session_dir": str(plan.session_dir),
        "devtools_dir": str(plan.devtools_dir),
        "native_bundle_dir": str(plan.native_bundle_dir),
        "demo_dir": str(plan.demo_dir),
        "include_docker": include_docker,
        "planned_commands": [command.name for command in plan.commands],
        "passed_commands": passed,
        "failed_commands": failed,
        "command_results": [
            {
                "name": result.name,
                "argv": result.argv,
                "returncode": result.returncode,
                "payload_status": None if result.payload is None else result.payload.get("status"),
                "payload_verdict": None if result.payload is None else result.payload.get("verdict"),
                "semantic_errors": result.semantic_errors,
                "stderr": result.stderr,
            }
            for result in results
        ],
        "product_alpha1_release_candidate_ready": release_ready,
        "product_alpha1_ready": release_ready,
        "final_product_claimed": False,
        "final_public_api_frozen": False,
        "non_claims": release_non_claims(include_docker),
    }


def print_payload(payload: dict[str, Any], fmt: str) -> None:
    if fmt == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
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
    check.add_argument("--skip-docker", action="store_true")
    args = parser.parse_args(argv)
    if args.command != "check-all":
        parser.error("expected subcommand: check-all")
    payload = check_all(out_dir=args.out, include_docker=not args.skip_docker)
    print_payload(payload, args.format)
    return 0 if payload["status"] == "accepted" else 2


if __name__ == "__main__":
    raise SystemExit(main())
