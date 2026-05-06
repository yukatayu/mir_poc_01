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
OPS_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "operational"
WORLD_CORE = OPS_ROOT / "world-core"
MEMBERSHIP_CHAT = OPS_ROOT / "membership-chat"
SUGOROKU_WORLD = OPS_ROOT / "sugoroku-world"
LAYERS_ROOT = OPS_ROOT / "packages"
EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT = 'EchoText:Text("Taro")->Text("Hello, Taro!")'


@dataclass(frozen=True)
class CommandResult:
    name: str
    argv: list[str]
    returncode: int
    stdout: str
    stderr: str
    payload: dict[str, Any] | None


def cargo_alpha_args(*args: str) -> list[str]:
    return ["cargo", "run", "-q", "-p", "mirrorea-cli", "--", *args, "--format", "json"]


def cargo_test_args(*args: str) -> list[str]:
    return ["cargo", "test", *args, "--", "--nocapture"]


def run_command(name: str, argv: list[str], env: dict[str, str] | None = None) -> CommandResult:
    completed = subprocess.run(
        argv,
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
        name=name,
        argv=argv,
        returncode=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        payload=payload,
    )


def command_payload(result: CommandResult) -> dict[str, Any]:
    return {
        "name": result.name,
        "argv": result.argv,
        "returncode": result.returncode,
        "payload": result.payload,
        "stderr": result.stderr.strip(),
    }


def sample_rows() -> list[dict[str, Any]]:
    return [
        {
            "sample_id": "OPS-01",
            "root": str(WORLD_CORE.relative_to(REPO_ROOT)),
            "package_id": "operational-world-core",
            "package_kind": "world_core",
            "runnable": True,
        },
        {
            "sample_id": "OPS-02",
            "root": str(MEMBERSHIP_CHAT.relative_to(REPO_ROOT)),
            "package_id": "operational-membership-chat",
            "package_kind": "membership_chat",
            "runnable": True,
        },
        {
            "sample_id": "OPS-03",
            "root": str(SUGOROKU_WORLD.relative_to(REPO_ROOT)),
            "package_id": "operational-sugoroku",
            "package_kind": "sugoroku_world",
            "runnable": True,
        },
        {
            "sample_id": "OPS-06",
            "root": str((OPS_ROOT / "future" / "portal-worldlink").relative_to(REPO_ROOT)),
            "package_id": "operational-portal-worldlink",
            "package_kind": "portal_worldlink",
            "runnable": False,
        },
    ]


def list_samples() -> dict[str, Any]:
    return {
        "surface_kind": "operational_product_sample_suite_list",
        "package_name": "P-OPS-01 operational product sample suite scaffold and first workflow",
        "sample_root": str(OPS_ROOT.relative_to(REPO_ROOT)),
        "samples": sample_rows(),
        "canonical_entrypoint": "mirrorea-alpha",
        "helper_role": "orchestration_only",
        "final_public_api_frozen": False,
    }


def membership_chat_echo_text_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    if not payload.get("typed_host_io_claimed"):
        return False
    session = payload.get("session") or {}
    host_io_history = session.get("host_io_history") or []
    if not host_io_history or host_io_history[0].get("adapter_kind") != "EchoText":
        return False
    observer_safe_export = session.get("observer_safe_export") or {}
    visible_events = observer_safe_export.get("visible_host_io_events") or []
    return EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT in visible_events


def membership_chat_devtools_echo_text_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    panel_ids = payload.get("panel_ids") or []
    session = payload.get("session") or {}
    observer_safe_export = session.get("observer_safe_export") or {}
    visible_events = observer_safe_export.get("visible_host_io_events") or []
    return (
        EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT in visible_events
        and "event_dag" in panel_ids
    )


def run_world_package(root: Path) -> dict[str, Any]:
    result = run_command(
        f"run-local:{root.name}",
        cargo_alpha_args("run-local", str(root)),
    )
    semantic_checks: dict[str, bool] = {}
    if root == MEMBERSHIP_CHAT:
        semantic_checks["echo_text_observed"] = membership_chat_echo_text_observed(result)
    return {
        "surface_kind": "operational_product_sample_run_report",
        "root": str(root.relative_to(REPO_ROOT)),
        "status": "accepted"
        if result.returncode == 0 and all(semantic_checks.values(),)
        else "error",
        "command": command_payload(result),
        "semantic_checks": semantic_checks,
        "final_public_api_frozen": False,
    }


def sugoroku_session_env() -> tuple[str, dict[str, str]]:
    session_dir = tempfile.mkdtemp(prefix="mirrorea-ops-session-")
    env = os.environ.copy()
    env["MIRROREA_ALPHA_SESSION_DIR"] = session_dir
    return session_dir, env


def bootstrap_sugoroku_session() -> tuple[str, dict[str, str], list[CommandResult]]:
    session_dir, env = sugoroku_session_env()
    commands = [
        run_command(
            "run-local:sugoroku",
            cargo_alpha_args("run-local", str(SUGOROKU_WORLD)),
            env=env,
        ),
        run_command(
            "session:sugoroku",
            cargo_alpha_args("session", "session#operational-sugoroku"),
            env=env,
        ),
    ]
    return session_dir, env, commands


def operational_attach_specs() -> list[tuple[str, Path, str]]:
    return [
        ("debug-layer", LAYERS_ROOT / "debug-layer", "accepted"),
        ("auth-layer", LAYERS_ROOT / "auth-layer", "accepted"),
        ("rate-limit-layer", LAYERS_ROOT / "rate-limit-layer", "accepted"),
        ("placeholder-object", LAYERS_ROOT / "placeholder-object", "deferred"),
        ("custom-avatar-preview", LAYERS_ROOT / "custom-avatar-preview", "deferred"),
    ]


def attach_matrix_complete(results: list[CommandResult]) -> bool:
    expected = {name: outcome for name, _, outcome in operational_attach_specs()}
    observed = {
        result.name.removeprefix("attach:"): result.payload.get("terminal_outcome")
        if result.payload
        else None
        for result in results
    }
    return all(observed.get(name) == outcome for name, outcome in expected.items())


def attach_layers() -> dict[str, Any]:
    session_dir, env, bootstrap = bootstrap_sugoroku_session()
    layer_results = [
        run_command(
            f"attach:{name}",
            cargo_alpha_args("attach", "session#operational-sugoroku", str(path)),
            env=env,
        )
        for name, path, _ in operational_attach_specs()
    ]
    matrix_complete = attach_matrix_complete(layer_results)
    return {
        "surface_kind": "operational_product_sample_attach_report",
        "session_dir": session_dir,
        "status": "accepted"
        if all(result.returncode == 0 for result in [*bootstrap, *layer_results])
        and matrix_complete
        else "error",
        "bootstrap": [command_payload(result) for result in bootstrap],
        "attach_results": [command_payload(result) for result in layer_results],
        "attach_matrix_complete": matrix_complete,
        "final_public_api_frozen": False,
    }


def transport(mode: str) -> dict[str, Any]:
    session_dir, env, bootstrap = bootstrap_sugoroku_session()
    result = run_command(
        f"transport:{mode}",
        cargo_alpha_args("transport", "session#operational-sugoroku", "--mode", mode),
        env=env,
    )
    return {
        "surface_kind": "operational_product_sample_transport_report",
        "mode": mode,
        "session_dir": session_dir,
        "status": "accepted"
        if all(item.returncode == 0 for item in [*bootstrap, result])
        else "error",
        "bootstrap": [command_payload(item) for item in bootstrap],
        "transport": command_payload(result),
        "final_public_api_frozen": False,
    }


def export_devtools() -> dict[str, Any]:
    session_dir, env, bootstrap = bootstrap_sugoroku_session()
    viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-viewer-")
    export_result = run_command(
        "export-devtools",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-sugoroku",
            "--out",
            viewer_dir,
        ),
        env=env,
    )
    view_result = run_command(
        "view",
        cargo_alpha_args("view", viewer_dir, "--check"),
    )
    return {
        "surface_kind": "operational_product_sample_devtools_report",
        "session_dir": session_dir,
        "viewer_dir": viewer_dir,
        "status": "accepted"
        if all(item.returncode == 0 for item in [*bootstrap, export_result, view_result])
        else "error",
        "bootstrap": [command_payload(item) for item in bootstrap],
        "export_devtools": command_payload(export_result),
        "view": command_payload(view_result),
        "final_public_api_frozen": False,
    }


def build_native_bundle() -> dict[str, Any]:
    out_dir = tempfile.mkdtemp(prefix="mirrorea-ops-bundle-")
    result = run_command(
        "build-native-bundle",
        cargo_alpha_args("build-native-bundle", str(SUGOROKU_WORLD), "--out", out_dir),
    )
    return {
        "surface_kind": "operational_product_sample_native_bundle_report",
        "bundle_dir": out_dir,
        "status": "accepted" if result.returncode == 0 else "error",
        "command": command_payload(result),
        "final_public_api_frozen": False,
    }


def release_check(skip_docker: bool) -> dict[str, Any]:
    session_dir, env = sugoroku_session_env()
    chat_session_dir, chat_env = sugoroku_session_env()
    viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-viewer-")
    chat_viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-chat-viewer-")
    bundle_dir = tempfile.mkdtemp(prefix="mirrorea-ops-bundle-")
    membership_chat_run = run_command(
        "run-local:membership-chat",
        cargo_alpha_args("run-local", str(MEMBERSHIP_CHAT)),
        env=chat_env,
    )
    membership_chat_export = run_command(
        "export-devtools:membership-chat",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-membership-chat",
            "--out",
            chat_viewer_dir,
        ),
        env=chat_env,
    )
    membership_chat_view = run_command(
        "view:membership-chat",
        cargo_alpha_args("view", chat_viewer_dir, "--check"),
    )
    commands = [
        run_command("check:world-core", cargo_alpha_args("check", str(WORLD_CORE))),
        run_command("check:membership-chat", cargo_alpha_args("check", str(MEMBERSHIP_CHAT))),
        run_command("check:sugoroku-world", cargo_alpha_args("check", str(SUGOROKU_WORLD))),
        membership_chat_run,
        membership_chat_export,
        membership_chat_view,
        run_command("run-local:sugoroku", cargo_alpha_args("run-local", str(SUGOROKU_WORLD)), env=env),
        run_command("session:sugoroku", cargo_alpha_args("session", "session#operational-sugoroku"), env=env),
        run_command("save:r0", cargo_alpha_args("save", "session#operational-sugoroku", "--savepoint", "savepoint#ops-r0"), env=env),
        run_command("quiescent-save:r2", cargo_alpha_args("quiescent-save", "session#operational-sugoroku", "--savepoint", "savepoint#ops-r2"), env=env),
        run_command("transport:local", cargo_alpha_args("transport", "session#operational-sugoroku", "--mode", "local"), env=env),
    ]
    attach_results = [
        run_command(
            f"attach:{name}",
            cargo_alpha_args("attach", "session#operational-sugoroku", str(path)),
            env=env,
        )
        for name, path, _ in operational_attach_specs()
    ]
    commands[5:5] = attach_results
    if not skip_docker:
        commands.append(
            run_command(
                "transport:docker",
                cargo_alpha_args("transport", "session#operational-sugoroku", "--mode", "docker"),
                env=env,
            )
        )
    commands.extend(
        [
            run_command("export-devtools", cargo_alpha_args("export-devtools", "session#operational-sugoroku", "--out", viewer_dir), env=env),
            run_command("view", cargo_alpha_args("view", viewer_dir, "--check")),
            run_command("build-native-bundle", cargo_alpha_args("build-native-bundle", str(SUGOROKU_WORLD), "--out", bundle_dir)),
        ]
    )
    failed = [result.name for result in commands if result.returncode != 0]
    attach_matrix_ok = attach_matrix_complete(attach_results)
    if not attach_matrix_ok:
        failed.append("attach-matrix")
    membership_chat_echo_text_ok = membership_chat_echo_text_observed(membership_chat_run)
    membership_chat_devtools_ok = membership_chat_devtools_echo_text_observed(
        membership_chat_export
    )
    if not membership_chat_echo_text_ok:
        failed.append("membership-chat-echo-text")
    if not membership_chat_devtools_ok:
        failed.append("membership-chat-devtools")
    status = "accepted" if not failed and not skip_docker else "partial" if not failed else "error"
    return {
        "surface_kind": "operational_product_sample_release_check_report",
        "status": status,
        "docker_included": not skip_docker,
        "session_dir": session_dir,
        "chat_session_dir": chat_session_dir,
        "viewer_dir": viewer_dir,
        "chat_viewer_dir": chat_viewer_dir,
        "bundle_dir": bundle_dir,
        "failed_commands": failed,
        "attach_matrix_complete": attach_matrix_ok,
        "membership_chat_echo_text_ok": membership_chat_echo_text_ok,
        "membership_chat_devtools_ok": membership_chat_devtools_ok,
        "commands": [command_payload(result) for result in commands],
        "product_alpha1_ready": False,
        "final_public_api_frozen": False,
        "non_claims": [
            "no final textual .mir grammar",
            "no final server/client binary split",
            "no direct LLVM backend",
            "no WAN federation",
            "no distributed durable save/load",
        ],
    }


def check_all(skip_docker: bool) -> dict[str, Any]:
    validation = [
        run_command(
            "validation:test-validate-docs",
            ["python3", "-m", "unittest", "scripts.tests.test_validate_docs"],
        ),
        run_command(
            "validation:test-operational-helper",
            ["python3", "-m", "unittest", "scripts.tests.test_operational_product_samples"],
        ),
        run_command("validation:source-hierarchy", ["python3", "scripts/check_source_hierarchy.py"]),
        run_command("validation:validate-docs", ["python3", "scripts/validate_docs.py"]),
        run_command("validation:cargo-fmt", ["cargo", "fmt", "--check"]),
        run_command("validation:git-diff-check", ["git", "diff", "--check"]),
        run_command(
            "test:mir-ast-product-schema",
            cargo_test_args("-p", "mir-ast", "--test", "product_alpha1_package_schema"),
        ),
        run_command(
            "test:mir-runtime-session",
            cargo_test_args("-p", "mir-runtime", "--test", "product_alpha1_session"),
        ),
        run_command(
            "test:mir-runtime-devtools",
            cargo_test_args("-p", "mir-runtime", "--test", "product_alpha1_transport_devtools"),
        ),
        run_command(
            "test:mirrorea-cli-alpha",
            cargo_test_args("-p", "mirrorea-cli", "--test", "alpha_cli"),
        ),
    ]
    release = release_check(skip_docker=skip_docker)
    failed = [result.name for result in validation if result.returncode != 0]
    failed.extend(release["failed_commands"])
    status = "accepted" if not failed and not skip_docker else "partial" if not failed else "error"
    return {
        "surface_kind": "operational_product_sample_suite_check_all_report",
        "status": status,
        "docker_included": not skip_docker,
        "failed_commands": failed,
        "validation": [command_payload(result) for result in validation],
        "release_check": release,
        "product_alpha1_ready": False,
        "final_public_api_frozen": False,
    }


def format_output(payload: dict[str, Any], fmt: str) -> str:
    if fmt == "json":
        return json.dumps(payload, indent=2, ensure_ascii=False)
    return json.dumps(payload, indent=2, ensure_ascii=False)


def add_subcommand_format(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--format",
        choices=["json", "pretty"],
        dest="subcommand_format",
        default=None,
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list")
    add_subcommand_format(list_parser)

    check_all_parser = subparsers.add_parser("check-all")
    add_subcommand_format(check_all_parser)
    check_all_parser.add_argument("--skip-docker", action="store_true")

    run_world_core_parser = subparsers.add_parser("run-world-core")
    add_subcommand_format(run_world_core_parser)
    run_membership_chat_parser = subparsers.add_parser("run-membership-chat")
    add_subcommand_format(run_membership_chat_parser)
    run_sugoroku_parser = subparsers.add_parser("run-sugoroku")
    add_subcommand_format(run_sugoroku_parser)
    attach_layers_parser = subparsers.add_parser("attach-layers")
    add_subcommand_format(attach_layers_parser)
    transport_local_parser = subparsers.add_parser("transport-local")
    add_subcommand_format(transport_local_parser)

    transport_docker_parser = subparsers.add_parser("transport-docker")
    add_subcommand_format(transport_docker_parser)
    transport_docker_parser.add_argument("--skip-docker", action="store_true")

    export_devtools_parser = subparsers.add_parser("export-devtools")
    add_subcommand_format(export_devtools_parser)
    build_native_bundle_parser = subparsers.add_parser("build-native-bundle")
    add_subcommand_format(build_native_bundle_parser)

    release_parser = subparsers.add_parser("release-check")
    add_subcommand_format(release_parser)
    release_parser.add_argument("--skip-docker", action="store_true")

    closeout_parser = subparsers.add_parser("closeout")
    add_subcommand_format(closeout_parser)
    closeout_parser.add_argument("--skip-docker", action="store_true")

    args = parser.parse_args(argv)
    output_format = args.subcommand_format or args.format

    if args.command == "list":
        payload = list_samples()
    elif args.command == "check-all":
        payload = check_all(skip_docker=args.skip_docker)
    elif args.command == "run-world-core":
        payload = run_world_package(WORLD_CORE)
    elif args.command == "run-membership-chat":
        payload = run_world_package(MEMBERSHIP_CHAT)
    elif args.command == "run-sugoroku":
        payload = run_world_package(SUGOROKU_WORLD)
    elif args.command == "attach-layers":
        payload = attach_layers()
    elif args.command == "transport-local":
        payload = transport("local")
    elif args.command == "transport-docker":
        payload = (
            {
                "surface_kind": "operational_product_sample_transport_report",
                "mode": "docker",
                "status": "skipped",
                "reason": "--skip-docker was passed",
                "final_public_api_frozen": False,
            }
            if args.skip_docker
            else transport("docker")
        )
    elif args.command == "export-devtools":
        payload = export_devtools()
    elif args.command == "build-native-bundle":
        payload = build_native_bundle()
    elif args.command == "release-check":
        payload = release_check(skip_docker=args.skip_docker)
    elif args.command == "closeout":
        payload = check_all(skip_docker=args.skip_docker)
    else:
        raise AssertionError(f"unhandled command {args.command}")

    print(format_output(payload, output_format))
    return 0 if payload.get("status") not in {"error"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
