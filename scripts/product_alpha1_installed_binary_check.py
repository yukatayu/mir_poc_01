#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PACKAGE = REPO_ROOT / "samples/product-alpha1/demo"
DEFAULT_BINARY = REPO_ROOT / "target" / "debug" / "mirrorea-alpha"


@dataclass(frozen=True)
class PlannedCommand:
    name: str
    argv: list[str]
    json_required: bool = True


@dataclass(frozen=True)
class CommandPlan:
    out_dir: Path
    session_dir: Path
    native_bundle_dir: Path
    demo_dir: Path
    binary_path: Path
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


def binary_alpha_args(binary_path: Path, *args: str) -> list[str]:
    return [str(binary_path), *args, "--format", "json"]


def validation_command(name: str, argv: list[str]) -> PlannedCommand:
    return PlannedCommand(name=name, argv=argv, json_required=False)


def plan_check_all(out_dir: Path, include_docker: bool = True, binary_path: Path = DEFAULT_BINARY) -> CommandPlan:
    session_dir = out_dir / "session-store"
    native_bundle_dir = out_dir / "native-bundle"
    demo_dir = out_dir / "demo"
    package = str(DEFAULT_PACKAGE)
    demo_args = ["demo", package, "--out", str(demo_dir)]
    if not include_docker:
        demo_args.append("--skip-docker")
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
            "build-binary",
            ["cargo", "build", "-q", "-p", "mirrorea-cli", "--bin", "mirrorea-alpha"],
        ),
        PlannedCommand("binary-check", binary_alpha_args(binary_path, "check", package)),
        PlannedCommand(
            "binary-build-native-bundle",
            binary_alpha_args(binary_path, "build-native-bundle", package, "--out", str(native_bundle_dir)),
        ),
        PlannedCommand(
            "bundle-run-check",
            ["sh", str(native_bundle_dir / "run.sh"), "check"],
        ),
        PlannedCommand(
            "bundle-run-view",
            ["sh", str(native_bundle_dir / "run.sh"), "view"],
        ),
        PlannedCommand("binary-demo", binary_alpha_args(binary_path, *demo_args)),
    ]
    return CommandPlan(
        out_dir=out_dir,
        session_dir=session_dir,
        native_bundle_dir=native_bundle_dir,
        demo_dir=demo_dir,
        binary_path=binary_path,
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

    if command.name == "binary-check":
        expect(payload.get("surface_kind") == "mirrorea_product_alpha1_check_report", "wrong check surface_kind")
        expect(payload.get("verdict") == "accepted", "check verdict is not accepted")
        expect(payload.get("product_alpha1_ready") is False, "check must not claim product readiness")
    elif command.name == "binary-build-native-bundle":
        expect(payload.get("status") == "accepted", "native bundle not accepted")
        expect(payload.get("host_launch_bundle_claimed") is True, "host launch bundle missing")
        expect(payload.get("package_native_execution_claimed") is False, "native package execution overclaimed")
        expect(payload.get("signature_is_safety_claimed") is False, "signature safety overclaimed")
    elif command.name in {"bundle-run-check", "bundle-run-view"}:
        expect(payload.get("status") == "accepted" or payload.get("verdict") == "accepted", f"{command.name} not accepted")
    elif command.name == "binary-demo":
        expect(payload.get("surface_kind") == "product_alpha1_demo_report", "wrong demo surface_kind")
        expect(payload.get("same_session_reopen_checked") is True, "demo session store reopen evidence missing")
        expect(payload.get("attach_matrix_verified") is True, "demo attach matrix not verified")
        expect(payload.get("complete_redaction_proof_claimed") is False, "demo must not claim complete redaction proof")
        if include_docker:
            expect(payload.get("status") == "accepted", "full demo not accepted")
            expect(payload.get("docker_transport_included") is True, "full demo skipped Docker")
            expect(payload.get("docker_transport_status") == "accepted", "Docker demo status mismatch")
            expect(payload.get("product_alpha1_release_candidate_ready") is True, "full demo not release-candidate ready")
            expect(payload.get("product_alpha1_ready") is True, "full demo not product ready")
        else:
            expect(payload.get("product_alpha1_release_candidate_ready") is False, "skip-docker demo overclaimed readiness")
            expect(payload.get("product_alpha1_ready") is False, "skip-docker demo overclaimed readiness")
    return errors


def installed_binary_non_claims(include_docker: bool) -> list[str]:
    claims = [
        "not final public CLI/API/ABI",
        "not final textual .mir grammar",
        "not hosted service",
        "not WAN/federation",
        "not distributed durable save/load",
        "not arbitrary native package execution",
        "not signature-is-safety",
        "not final public viewer or telemetry service",
    ]
    if not include_docker:
        claims.append("Docker Compose TCP transport skipped; this is a local probe, not release-candidate evidence")
    return claims


def check_all(
    out_dir: Path | None = None,
    include_docker: bool = True,
    binary_path: Path = DEFAULT_BINARY,
) -> dict[str, Any]:
    if out_dir is None:
        out_dir = Path(tempfile.mkdtemp(prefix="mirrorea-alpha1-installed-binary-"))
    elif out_dir.exists() and any(out_dir.iterdir()):
        return {
            "surface_kind": "product_alpha1_installed_binary_check_report",
            "status": "error",
            "command": "check-all",
            "diagnostic_code": "output_dir_not_empty",
            "out_dir": str(out_dir),
            "include_docker": include_docker,
            "binary_path": str(binary_path),
            "planned_commands": [],
            "passed_commands": [],
            "failed_commands": ["preflight:output-dir-empty"],
            "command_results": [],
            "installed_binary_candidate_ready": False,
            "public_packaging_candidate": "installed_binary_plus_native_host_launch_bundle",
            "final_public_api_frozen": False,
            "non_claims": installed_binary_non_claims(include_docker),
        }
    out_dir.mkdir(parents=True, exist_ok=True)
    plan = plan_check_all(out_dir=out_dir, include_docker=include_docker, binary_path=binary_path)
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
    candidate_ready = not failed and include_docker
    return {
        "surface_kind": "product_alpha1_installed_binary_check_report",
        "status": status,
        "command": "check-all",
        "out_dir": str(out_dir),
        "session_dir": str(plan.session_dir),
        "native_bundle_dir": str(plan.native_bundle_dir),
        "demo_dir": str(plan.demo_dir),
        "binary_path": str(plan.binary_path),
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
        "installed_binary_candidate_ready": candidate_ready,
        "public_packaging_candidate": "installed_binary_plus_native_host_launch_bundle",
        "final_public_api_frozen": False,
        "non_claims": installed_binary_non_claims(include_docker),
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
    check.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    args = parser.parse_args(argv)
    if args.command != "check-all":
        parser.error("expected subcommand: check-all")
    payload = check_all(
        out_dir=args.out,
        include_docker=not args.skip_docker,
        binary_path=args.binary,
    )
    print_payload(payload, args.format)
    return 0 if payload["status"] == "accepted" else 2


if __name__ == "__main__":
    raise SystemExit(main())
