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
    },
    {
        "sample_id": "NET-03",
        "summary": "Reject stale membership epoch/incarnation over the Docker bridge.",
        "expected_terminal_outcome": "rejected",
        "expected_reason_family": "membership_freshness",
    },
    {
        "sample_id": "NET-04",
        "summary": "Reject missing capability across the Docker bridge.",
        "expected_terminal_outcome": "rejected",
        "expected_reason_family": "capability",
    },
    {
        "sample_id": "NET-05",
        "summary": "Reject missing required witness across the Docker bridge.",
        "expected_terminal_outcome": "rejected",
        "expected_reason_family": "witness",
    },
    {
        "sample_id": "NET-07",
        "summary": "Emit observer-safe route trace without raw auth/capability leakage.",
        "expected_terminal_outcome": "accepted",
        "expected_reason_family": None,
    },
    {
        "sample_id": "NET-09",
        "summary": "Keep auth evidence in a separate lane from transport delivery.",
        "expected_terminal_outcome": "accepted",
        "expected_reason_family": None,
    },
]

PLANNED_ONLY_ROWS = ["NET-01", "NET-06", "NET-08", "NET-10"]

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
        _validate_outputs(sample_id, row, world, participant)

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
    world: dict[str, Any],
    participant: dict[str, Any],
) -> None:
    if world.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: world output sample_id mismatch: {world.get('sample_id')!r}"
        )
    if participant.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: participant output sample_id mismatch: {participant.get('sample_id')!r}"
        )
    if participant.get("terminal_outcome") != row["expected_terminal_outcome"]:
        raise RuntimeError(
            f"{sample_id}: expected terminal_outcome {row['expected_terminal_outcome']!r} "
            f"but observed {participant.get('terminal_outcome')!r}"
        )
    expected_reason_family = row["expected_reason_family"]
    if participant.get("reason_family") != expected_reason_family:
        raise RuntimeError(
            f"{sample_id}: expected reason_family {expected_reason_family!r} "
            f"but observed {participant.get('reason_family')!r}"
        )
    if sample_id == "NET-07":
        rows = participant.get("observer_route_trace")
        if not isinstance(rows, list) or not rows:
            raise RuntimeError("NET-07: observer_route_trace missing or empty")
        for route_row in rows:
            if route_row.get("redaction") != "observer_safe_route_trace":
                raise RuntimeError("NET-07: route trace redaction mismatch")
            for forbidden_key in [
                "principal",
                "claimed_authority",
                "claimed_capabilities",
                "auth_evidence",
                "witness_refs",
            ]:
                if forbidden_key in route_row:
                    raise RuntimeError(
                        f"NET-07: raw field {forbidden_key!r} leaked into observer trace"
                    )
    if sample_id == "NET-09":
        auth_lane = participant.get("auth_lane")
        if not isinstance(auth_lane, dict) or not auth_lane.get("preserved_separately"):
            raise RuntimeError("NET-09: auth lane preservation evidence missing")


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
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "transport_surface": "docker_compose_tcp",
        "transport_medium": TRANSPORT_MEDIUM,
        "validation_floor": [
            "cargo build -p mir-runtime --example mirrorea_alpha_network_runtime",
            "cargo test -p mir-runtime --test alpha_network_runtime",
            "python3 scripts/alpha_network_docker_e2e.py check-all --format json",
        ],
        "stop_lines": [
            "do not treat helper-local scripts/network_transport_samples.py as Alpha-0 Docker validation",
            "do not claim production WAN/session/replay/runtime completion",
            "do not mark NET-06/08/10 implemented in this package",
        ],
        "limitations": list(LIMITATIONS),
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
