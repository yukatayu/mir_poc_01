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
SAMPLE_ROOT = REPO_ROOT / "samples" / "alpha" / "network-docker"
COMPOSE_FILE = SAMPLE_ROOT / "docker-compose.alpha-net.yml"
BINARY_PATH = REPO_ROOT / "target" / "debug" / "examples" / "mirrorea_alpha_network_runtime"
TRANSPORT_MEDIUM = "docker_bridge_tcp"


IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "NET-02",
        "summary": "World/participant exchange over Docker Compose TCP bridge.",
        "expected_terminal_outcome": "accepted",
        "expected_reason_family": None,
        "expected_sidecar": "samples/alpha/network-docker/net-02-docker_two_process_envelope.expected.json",
    },
    {
        "sample_id": "NET-03",
        "summary": "Reject stale membership epoch/incarnation over the Docker bridge.",
        "expected_terminal_outcome": "rejected",
        "expected_reason_family": "membership_freshness",
        "expected_sidecar": "samples/alpha/network-docker/net-03-stale_membership_rejected.expected.json",
    },
    {
        "sample_id": "NET-04",
        "summary": "Reject missing capability across the Docker bridge.",
        "expected_terminal_outcome": "rejected",
        "expected_reason_family": "capability",
        "expected_sidecar": "samples/alpha/network-docker/net-04-missing_capability_rejected.expected.json",
    },
    {
        "sample_id": "NET-05",
        "summary": "Reject missing required witness across the Docker bridge.",
        "expected_terminal_outcome": "rejected",
        "expected_reason_family": "witness",
        "expected_sidecar": "samples/alpha/network-docker/net-05-missing_witness_rejected.expected.json",
    },
    {
        "sample_id": "NET-07",
        "summary": "Emit observer-safe route trace without raw auth/capability leakage.",
        "expected_terminal_outcome": "accepted",
        "expected_reason_family": None,
        "expected_sidecar": "samples/alpha/network-docker/net-07-observer_safe_route_trace.expected.json",
    },
    {
        "sample_id": "NET-09",
        "summary": "Keep auth evidence in a separate lane from transport delivery.",
        "expected_terminal_outcome": "accepted",
        "expected_reason_family": None,
        "expected_sidecar": "samples/alpha/network-docker/net-09-auth_evidence_lane_preserved.expected.json",
    },
]

PLANNED_ONLY_ROWS = ["NET-01", "NET-06", "NET-08", "NET-10"]
STAGE_C_REQUIRED_ROWS = ["NET-02", "NET-03", "NET-04", "NET-05", "NET-07", "NET-09"]

LIMITATIONS = [
    "narrow local Docker Compose bridge only",
    "no production WAN/session/replay claim",
    "no continuous shared runtime state across worlds",
    "no final public transport ABI",
]


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented alpha network sample: {sample_id}")


def _load_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    sidecar_path = REPO_ROOT / row["expected_sidecar"]
    return json.loads(sidecar_path.read_text())


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-network-docker",
            "source_root": str(SAMPLE_ROOT),
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


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
        raise RuntimeError(
            "docker is not installed or not on PATH; P-A0-09 cannot close honestly"
        ) from error
    except subprocess.CalledProcessError as error:
        raise RuntimeError(
            f"docker compose is unavailable: {error.stderr.strip() or error.stdout.strip()}"
        ) from error


def _check_binary_available() -> None:
    if not BINARY_PATH.exists():
        raise RuntimeError(
            "missing prebuilt runtime example "
            f"`{BINARY_PATH}`; run `cargo build -p mir-runtime --example mirrorea_alpha_network_runtime` first"
        )


def _run_compose(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    _check_binary_available()
    _check_docker_available()

    with tempfile.TemporaryDirectory(prefix="alpha-net-") as tmpdir:
        output_dir = Path(tmpdir)
        env = os.environ.copy()
        env.update(
            {
                "MIRROREA_ALPHA_NETWORK_BINARY": str(BINARY_PATH),
                "MIRROREA_ALPHA_NETWORK_OUTPUT_DIR": str(output_dir),
                "MIRROREA_ALPHA_NETWORK_SAMPLE_ID": sample_id,
                "MIRROREA_ALPHA_NETWORK_TRANSPORT_MEDIUM": TRANSPORT_MEDIUM,
            }
        )
        up_cmd = [
            "docker",
            "compose",
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
            "-f",
            str(COMPOSE_FILE),
            "down",
            "--remove-orphans",
            "-v",
        ]

        try:
            completed = subprocess.run(
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
                f"Docker Compose run for {sample_id} failed: {stderr}"
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

        world_path = output_dir / "world.json"
        participant_path = output_dir / "participant.json"
        world = _read_json_file(world_path)
        participant = _read_json_file(participant_path)
        _validate_outputs(
            sample_id,
            row,
            _load_expected_sidecar(row),
            world,
            participant,
            compose_transport_surface="docker_compose_tcp",
        )

        return {
            "sample_id": sample_id,
            "summary": row["summary"],
            "transport_surface": "docker_compose_tcp",
            "transport_medium": TRANSPORT_MEDIUM,
            "compose_file": str(COMPOSE_FILE),
            "docker_stdout": completed.stdout.strip().splitlines(),
            "world": world,
            "participant": participant,
        }


def _read_json_file(path: Path) -> dict[str, Any]:
    if not path.exists():
        raise RuntimeError(f"expected JSON output missing: {path}")
    return json.loads(path.read_text())


def _validate_outputs(
    sample_id: str,
    row: dict[str, Any],
    expected_sidecar: dict[str, Any],
    world: dict[str, Any],
    participant: dict[str, Any],
    *,
    compose_transport_surface: str,
) -> None:
    expected_runtime = expected_sidecar.get("expected_runtime", {})
    if world.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: world output sample_id mismatch: {world.get('sample_id')!r}"
        )
    if participant.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: participant output sample_id mismatch: {participant.get('sample_id')!r}"
        )
    if participant.get("terminal_outcome") != expected_runtime.get("terminal_outcome"):
        raise RuntimeError(
            f"{sample_id}: expected terminal_outcome {expected_runtime.get('terminal_outcome')!r} "
            f"but observed {participant.get('terminal_outcome')!r}"
        )
    expected_reason_family = expected_runtime.get("reason_family")
    if participant.get("reason_family") != expected_reason_family:
        raise RuntimeError(
            f"{sample_id}: expected reason_family {expected_reason_family!r} "
            f"but observed {participant.get('reason_family')!r}"
        )
    expected_transport_surfaces = set(expected_runtime.get("transport_surfaces") or [])
    if expected_transport_surfaces:
        observed_transport_surfaces = {
            participant.get("transport_surface"),
            compose_transport_surface,
        }
        observed_transport_surfaces.discard(None)
        if compose_transport_surface not in expected_transport_surfaces:
            raise RuntimeError(
                f"{sample_id}: compose transport surface {compose_transport_surface!r} "
                f"is outside expected transport surfaces {sorted(expected_transport_surfaces)!r}"
            )
        if not observed_transport_surfaces:
            raise RuntimeError(f"{sample_id}: no observed transport surface evidence")
        if not observed_transport_surfaces.issubset(expected_transport_surfaces):
            raise RuntimeError(
                f"{sample_id}: observed transport surfaces {sorted(observed_transport_surfaces)!r} "
                f"escape expected transport surfaces {sorted(expected_transport_surfaces)!r}"
            )

    retained_later_refs = set(participant.get("retained_later_refs") or [])
    for kept_later in [
        "route_rebinding_no_shadow",
        "network_partition_explicit_failure",
        "transport_medium_change_preserves_contract",
        "production_wan_federation",
        "final_public_transport_abi",
    ]:
        if kept_later not in retained_later_refs:
            raise RuntimeError(f"{sample_id}: missing kept-later ref {kept_later!r}")

    if sample_id == "NET-07":
        expected_rows = expected_runtime.get("observer_route_trace_rows")
        rows = participant.get("observer_route_trace")
        if not isinstance(rows, list) or not rows:
            raise RuntimeError("NET-07: observer_route_trace missing or empty")
        if expected_rows is not None and len(rows) != expected_rows:
            raise RuntimeError(
                f"NET-07: expected {expected_rows} route-trace rows but observed {len(rows)}"
            )
        if participant.get("terminal_outcome") != "accepted":
            raise RuntimeError("NET-07: expected accepted route-trace outcome")
        for route_row in rows:
            if route_row.get("redaction") != expected_runtime.get("route_trace_redaction"):
                raise RuntimeError("NET-07: route trace redaction mismatch")
            for forbidden_key in expected_runtime.get("no_raw_fields") or []:
                if forbidden_key in route_row:
                    raise RuntimeError(
                        f"NET-07: raw field {forbidden_key!r} leaked into observer trace"
                    )
    elif sample_id == "NET-02":
        required_witness_ref = expected_runtime.get("required_witness_ref")
        required_witness_refs = set(participant.get("required_witness_refs") or [])
        if required_witness_ref and required_witness_ref not in required_witness_refs:
            raise RuntimeError(
                f"NET-02: missing required witness ref {required_witness_ref!r}"
            )
        expected_rows = expected_runtime.get("observer_route_trace_rows")
        route_rows = participant.get("observer_route_trace") or []
        if expected_rows is not None and len(route_rows) != expected_rows:
            raise RuntimeError(
                f"NET-02: expected {expected_rows} route-trace rows but observed {len(route_rows)}"
            )
    elif sample_id == "NET-03":
        reason_refs = set(participant.get("rejection_reason_refs") or [])
        for required_reason in expected_runtime.get("required_reason_refs") or []:
            if required_reason not in reason_refs:
                raise RuntimeError(
                    f"NET-03: missing required reason ref {required_reason!r}"
                )
    elif sample_id == "NET-04":
        missing_capability = expected_runtime.get("missing_capability")
        reason_refs = set(participant.get("rejection_reason_refs") or [])
        expected_ref = (
            f"missing_capability:{missing_capability}" if missing_capability else None
        )
        if expected_ref and expected_ref not in reason_refs:
            raise RuntimeError(
                f"NET-04: missing capability rejection ref {expected_ref!r}"
            )
    elif sample_id == "NET-05":
        missing_witness = expected_runtime.get("missing_witness")
        reason_refs = set(participant.get("rejection_reason_refs") or [])
        expected_ref = f"missing_witness:{missing_witness}" if missing_witness else None
        if expected_ref and expected_ref not in reason_refs:
            raise RuntimeError(
                f"NET-05: missing witness rejection ref {expected_ref!r}"
            )
    if sample_id == "NET-09":
        auth_lane = participant.get("auth_lane")
        if not isinstance(auth_lane, dict):
            raise RuntimeError("NET-09: auth lane evidence missing")
        if auth_lane.get("auth_present") != expected_runtime.get("auth_present"):
            raise RuntimeError(
                f"NET-09: expected auth_present {expected_runtime.get('auth_present')!r} "
                f"but observed {auth_lane.get('auth_present')!r}"
            )
        if auth_lane.get("preserved_separately") != expected_runtime.get("preserved_separately"):
            raise RuntimeError("NET-09: auth lane preservation evidence missing")
        observed_bindings = set(auth_lane.get("bindings") or [])
        required_bindings = set(expected_runtime.get("required_bindings") or [])
        if not required_bindings.issubset(observed_bindings):
            raise RuntimeError(
                f"NET-09: missing required auth bindings {sorted(required_bindings - observed_bindings)!r}"
            )


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            _run_compose(sample_id)
        except Exception as error:  # pragma: no cover - surfaced in command output.
            failed.append({"sample_id": sample_id, "reason": str(error)})
        else:
            passed.append(sample_id)
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "transport_surface": "docker_compose_tcp",
        "transport_medium": TRANSPORT_MEDIUM,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": str(SAMPLE_ROOT),
        "compose_file": str(COMPOSE_FILE),
        "binary_path": str(BINARY_PATH),
        "implemented_samples": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "stage_c_required_rows": list(STAGE_C_REQUIRED_ROWS),
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "transport_surface": "docker_compose_tcp",
        "transport_medium": TRANSPORT_MEDIUM,
        "validation_floor": [
            "cargo build -p mir-runtime --example mirrorea_alpha_network_runtime",
            "cargo test -p mir-runtime --test alpha_network_runtime",
            "python3 scripts/alpha_network_docker_e2e.py check-all --format json",
            "python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json",
        ],
        "stop_lines": [
            "do not treat helper-local scripts/network_transport_samples.py as Alpha-0 Docker validation",
            "do not claim production WAN/session/replay/runtime completion",
            "do not mark NET-06/08/10 implemented in this package",
        ],
        "limitations": list(LIMITATIONS),
    }


def stage_c_closeout() -> dict[str, Any]:
    network_payload = check_all()
    failed = network_payload.get("failed") or []
    return {
        "stage": "Stage C",
        "stage_name": "alpha 0.7 transport",
        "stage_c_complete": not failed,
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "network_check": network_payload,
        "wan_federation_claimed": False,
        "network_partition_complete": False,
        "transport_medium_change_complete": False,
        "active_root_promoted": False,
        "final_public_transport_abi_claimed": False,
        "note": (
            "Stage C closeout is satisfied only by NET-02/03/04/05/07/09 over the "
            "existing Docker/local-subprocess floor. It does not claim WAN federation, "
            "partition completion, medium-substitution completion, or final public transport ABI."
        ),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(
            f"{row['sample_id']} {row['summary']}" for row in payload
        )
    if isinstance(payload, dict) and "sample_id" in payload and "participant" in payload:
        participant = payload["participant"]
        lines = [
            f"{payload['sample_id']} docker_compose_tcp",
            f"  outcome: {participant.get('terminal_outcome')}",
            f"  reason_family: {participant.get('reason_family')}",
        ]
        return "\n".join(lines)
    if isinstance(payload, dict) and payload.get("stage") == "Stage C":
        lines = [
            f"{payload['stage']} {payload['stage_name']}",
            f"  stage_c_complete: {payload.get('stage_c_complete')}",
            f"  implemented_rows: {', '.join(payload.get('implemented_rows', []))}",
            f"  planned_only_rows: {', '.join(payload.get('planned_only_rows', []))}",
        ]
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Alpha-0 Docker network E2E runner")
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list")
    list_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
    run_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    check_all_parser = subparsers.add_parser("check-all")
    check_all_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    closeout_parser = subparsers.add_parser("closeout")
    closeout_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    stage_c_parser = subparsers.add_parser("stage-c-closeout")
    stage_c_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "list":
        payload = list_samples()
    elif args.command == "run":
        payload = _run_compose(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "stage-c-closeout":
        payload = stage_c_closeout()
    elif args.command == "closeout":
        payload = closeout()
    else:  # pragma: no cover - argparse enforces command choices.
        parser.error(f"unknown command {args.command!r}")

    if args.format == "json":
        print(json.dumps(payload, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
