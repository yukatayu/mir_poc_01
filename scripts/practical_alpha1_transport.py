#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
COMPOSE_FILE = REPO_ROOT / "samples" / "practical-alpha1" / "docker" / "docker-compose.practical-alpha1.yml"
BINARY_PATH = REPO_ROOT / "target" / "debug" / "examples" / "mir_practical_alpha1_transport"

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "TR-A1-01",
        "summary": "accept checked practical package transport over a local TCP boundary",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-01-local-tcp-accepted.expected.json",
    },
    {
        "sample_id": "TR-A1-02",
        "summary": "accept checked practical package transport over a Docker Compose TCP boundary",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-02-docker-two-node-accepted",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-02-docker-two-node-accepted.expected.json",
    },
    {
        "sample_id": "TR-A1-03",
        "summary": "reject stale membership over the practical transport carrier",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-03-stale-membership-rejected",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-03-stale-membership-rejected.expected.json",
    },
    {
        "sample_id": "TR-A1-04",
        "summary": "reject missing capability over the practical transport carrier",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-04-missing-capability-rejected",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-04-missing-capability-rejected.expected.json",
    },
    {
        "sample_id": "TR-A1-05",
        "summary": "reject missing witness over the practical transport carrier",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-05-missing-witness-rejected",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-05-missing-witness-rejected.expected.json",
    },
    {
        "sample_id": "TR-A1-06",
        "summary": "emit observer-safe route trace from the practical transport carrier",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-06-observer-safe-route-trace",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-06-observer-safe-route-trace.expected.json",
    },
    {
        "sample_id": "TR-A1-07",
        "summary": "keep auth evidence in a separate lane from practical transport delivery",
        "package_dir": "samples/practical-alpha1/packages/tr-a1-07-auth-lane-preserved",
        "expected_report": "samples/practical-alpha1/expected/tr-a1-07-auth-lane-preserved.expected.json",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-1 transport floor as WAN or federation completion",
    "do not treat the practical alpha-1 transport floor as local save/load, devtools, or product prototype completion",
    "do not treat the practical alpha-1 transport floor as a final public transport ABI",
    "do not promote samples/practical-alpha1 to an active runnable root in the transport package",
]

LIMITATIONS = [
    "alpha-local non-final practical transport floor only",
    "limited TR-A1 practical sample families only",
    "no WAN federation, local save/load command, devtools export, or final public ABI",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 transport sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-transport",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_report(row: dict[str, str]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def _package_file(package_path: str | Path) -> Path:
    path = Path(package_path)
    if path.is_dir():
        return path / "package.mir.json"
    return path


def _load_package(package_path: str | Path) -> dict[str, Any]:
    return json.loads(_package_file(package_path).read_text())


def _transport_surface(package_path: str | Path) -> str:
    package = _load_package(package_path)
    transport = package.get("alpha_local_transport_input") or {}
    surface = transport.get("transport_surface")
    if not isinstance(surface, str) or not surface:
        raise RuntimeError(f"missing alpha_local_transport_input.transport_surface in {package_path}")
    return surface


def _build_local_report(package_path: str | Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha1_transport",
            "--",
            "run",
            str(package_path),
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:  # pragma: no cover
        raise RuntimeError(
            f"practical transport command did not return JSON for {package_path}: {completed.stdout}"
        ) from error


def _check_docker_available() -> None:
    try:
        subprocess.run(
            ["docker", "--version"],
            cwd=REPO_ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        subprocess.run(
            ["docker", "compose", "version"],
            cwd=REPO_ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
    except FileNotFoundError as error:
        raise RuntimeError("docker is not installed or not on PATH; practical transport Docker validation cannot run") from error
    except subprocess.CalledProcessError as error:
        raise RuntimeError(
            f"docker compose is unavailable: {error.stderr.strip() or error.stdout.strip()}"
        ) from error


def _ensure_binary_available() -> None:
    if BINARY_PATH.exists():
        return
    subprocess.run(
        [
            "cargo",
            "build",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha1_transport",
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    if not BINARY_PATH.exists():
        raise RuntimeError(f"expected practical transport example binary missing: {BINARY_PATH}")


def _build_docker_report(package_path: str | Path) -> dict[str, Any]:
    _check_docker_available()
    _ensure_binary_available()
    package_dir = Path(package_path)
    if package_dir.is_file():
        package_dir = package_dir.parent
    package_dir = package_dir.resolve()

    with tempfile.TemporaryDirectory(prefix="practical-a1-transport-") as tmpdir:
        output_dir = Path(tmpdir)
        project_name = f"practical-a1-{output_dir.name}"
        env = os.environ.copy()
        env.update(
            {
                "MIRROREA_PRACTICAL_ALPHA1_BINARY": str(BINARY_PATH),
                "MIRROREA_PRACTICAL_ALPHA1_OUTPUT_DIR": str(output_dir),
                "MIRROREA_PRACTICAL_ALPHA1_PACKAGE_DIR": str(package_dir),
            }
        )
        up_cmd = [
            "docker",
            "compose",
            "-p",
            project_name,
            "-f",
            str(COMPOSE_FILE),
            "up",
            "--abort-on-container-exit",
            "--exit-code-from",
            "participant",
        ]
        down_cmd = [
            "docker",
            "compose",
            "-p",
            project_name,
            "-f",
            str(COMPOSE_FILE),
            "down",
            "--remove-orphans",
            "-v",
        ]
        try:
            subprocess.run(
                up_cmd,
                cwd=REPO_ROOT,
                env=env,
                check=True,
                capture_output=True,
                text=True,
            )
        except subprocess.CalledProcessError as error:
            stderr = error.stderr.strip() or error.stdout.strip()
            raise RuntimeError(
                f"Docker Compose run for {package_dir} failed: {stderr}"
            ) from error
        finally:
            subprocess.run(
                down_cmd,
                cwd=REPO_ROOT,
                env=env,
                check=False,
                capture_output=True,
                text=True,
            )

        world = json.loads((output_dir / "world.json").read_text())
        participant = json.loads((output_dir / "participant.json").read_text())
        if world.get("sample_id") != participant.get("sample_id"):
            raise RuntimeError("docker transport sample_id mismatch between world and participant outputs")
        if world.get("terminal_outcome") != participant.get("terminal_outcome"):
            raise RuntimeError("docker transport terminal_outcome mismatch between world and participant outputs")
        if world.get("reason_family") != participant.get("reason_family"):
            raise RuntimeError("docker transport reason_family mismatch between world and participant outputs")
        return participant


def run_path(package_path: str | Path) -> dict[str, Any]:
    surface = _transport_surface(package_path)
    if surface == "local_tcp":
        return _build_local_report(package_path)
    if surface == "docker_compose_tcp":
        return _build_docker_report(package_path)
    raise RuntimeError(f"unsupported practical transport surface {surface!r}")


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = run_path(REPO_ROOT / row["package_dir"])
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected transport report drift\n"
            f"expected={json.dumps(expected, ensure_ascii=False, sort_keys=True)}\n"
            f"actual={json.dumps(actual, ensure_ascii=False, sort_keys=True)}"
        )
    return actual


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    reports: list[dict[str, Any]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            report = run_sample(sample_id)
            passed.append(sample_id)
            reports.append(report)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    by_id = {report.get("sample_id"): report for report in reports}
    transport_plan_boundary_present = not failed and bool(reports) and all(
        report.get("transport_plan_scope") == "practical-alpha1-transport-plan-floor"
        and report.get("transport_plan_emitted") is False
        for report in reports
    )
    docker_row_complete = (
        by_id.get("TR-A1-02", {}).get("transport_surface") == "docker_compose_tcp"
        and by_id.get("TR-A1-02", {}).get("run_docker_claimed") is True
        and by_id.get("TR-A1-02", {}).get("terminal_outcome") == "accepted"
    )
    stale_membership_negative_complete = (
        by_id.get("TR-A1-03", {}).get("reason_family") == "membership_freshness"
    )
    missing_capability_negative_complete = (
        by_id.get("TR-A1-04", {}).get("reason_family") == "capability"
    )
    missing_witness_negative_complete = (
        by_id.get("TR-A1-05", {}).get("reason_family") == "witness"
    )
    route_trace_complete = (
        len(by_id.get("TR-A1-06", {}).get("observer_route_trace") or []) == 2
        and all(
            row.get("redaction") == "observer_safe_route_trace"
            for row in (by_id.get("TR-A1-06", {}).get("observer_route_trace") or [])
        )
    )
    auth_lane = by_id.get("TR-A1-07", {}).get("auth_lane") or {}
    auth_lane_complete = (
        auth_lane.get("auth_present") is True
        and auth_lane.get("preserved_separately") is True
    )
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "transport_first_floor_complete": not failed,
        "transport_plan_boundary_present": transport_plan_boundary_present,
        "docker_row_complete": docker_row_complete,
        "stale_membership_negative_complete": stale_membership_negative_complete,
        "missing_capability_negative_complete": missing_capability_negative_complete,
        "missing_witness_negative_complete": missing_witness_negative_complete,
        "route_trace_complete": route_trace_complete,
        "auth_lane_complete": auth_lane_complete,
        "stage_pa1_5_complete": not failed
        and transport_plan_boundary_present
        and docker_row_complete
        and stale_membership_negative_complete
        and missing_capability_negative_complete
        and missing_witness_negative_complete
        and route_trace_complete
        and auth_lane_complete,
        "wan_federation_claimed": False,
        "save_load_claimed": False,
        "final_public_transport_abi_claimed": False,
    }


def closeout() -> dict[str, Any]:
    check_all_summary = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "compose_file": str(COMPOSE_FILE),
        "binary_path": str(BINARY_PATH),
        "validation_floor": [
            "docker --version",
            "docker compose version",
            "cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture",
            "cargo test -p mir-ast practical_alpha1_checker -- --nocapture",
            "cargo test -p mir-ast --test practical_alpha1_runtime_plan -- --nocapture",
            "cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture",
            "cargo test -p mir-ast --test practical_alpha1_transport_plan -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture",
            "cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture",
            "cargo test -p mir-runtime --test alpha_layer_insertion_runtime",
            "cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture",
            "python3 scripts/practical_alpha1_check.py check-all --format json",
            "python3 scripts/practical_alpha1_run_local.py check-all --format json",
            "python3 scripts/practical_alpha1_attach.py check-all --format json",
            "python3 scripts/practical_alpha1_transport.py check-all --format json",
            "python3 scripts/practical_alpha1_transport.py closeout --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_transport scripts.tests.test_validate_docs",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "transport_first_floor_complete": check_all_summary["transport_first_floor_complete"],
        "transport_plan_boundary_present": check_all_summary[
            "transport_plan_boundary_present"
        ],
        "docker_row_complete": check_all_summary["docker_row_complete"],
        "stale_membership_negative_complete": check_all_summary[
            "stale_membership_negative_complete"
        ],
        "missing_capability_negative_complete": check_all_summary[
            "missing_capability_negative_complete"
        ],
        "missing_witness_negative_complete": check_all_summary[
            "missing_witness_negative_complete"
        ],
        "route_trace_complete": check_all_summary["route_trace_complete"],
        "auth_lane_complete": check_all_summary["auth_lane_complete"],
        "stage_pa1_5_complete": check_all_summary["stage_pa1_5_complete"],
        "wan_federation_claimed": check_all_summary["wan_federation_claimed"],
        "save_load_claimed": check_all_summary["save_load_claimed"],
        "final_public_transport_abi_claimed": check_all_summary[
            "final_public_transport_abi_claimed"
        ],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and "sample_id" in payload:
        surface = payload.get("transport_surface", "?")
        outcome = payload.get("terminal_outcome", "?")
        reason = payload.get("reason_family", "-")
        return f"{payload['sample_id']} {surface} {outcome} {reason}"
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 transport command surface. This consumes checked practical "
            "package input through a distinct transport-plan boundary and remains non-final."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")

    check_parser = subparsers.add_parser("check")
    check_parser.add_argument("package_path")

    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")

    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    return parser


def normalize_argv(argv: list[str] | None) -> list[str]:
    values = list(sys.argv[1:] if argv is None else argv)
    hoisted_root_options: list[str] = []
    remainder: list[str] = []
    index = 0
    while index < len(values):
        current = values[index]
        if current == "--format" and index + 1 < len(values):
            hoisted_root_options.extend(values[index : index + 2])
            index += 2
            continue
        remainder.append(current)
        index += 1

    values = [*hoisted_root_options, *remainder]
    known = {"list", "run", "check", "check-all", "closeout"}
    payload = list(remainder)
    if payload and payload[0] not in known and not payload[0].startswith("-"):
        return [*hoisted_root_options, "check", *payload]
    return values


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check":
        payload = run_path(args.package_path)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
