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
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "computational"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final textual grammar",
    "no direct LLVM/native backend",
    "do not treat current AddOne as Mir-owned computation everywhere",
]
NON_CLAIMS = [
    "only one Mir-owned runtime row exists today",
    "no final textual `.mir` grammar",
    "no direct LLVM/native backend",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_mir_computational_samples",
    "python3 scripts/mir_computational_samples.py matrix --format json",
    "python3 scripts/mir_computational_samples.py check-all --format json",
    "python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json",
]


def _load_matrix_file() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def validate_rows(
    sample_root: Path, rows: list[dict[str, Any]]
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = sample_root / row["root_name"]
        source_path = sample_root / row["representative_source"]
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_representative_source",
                    "detail": f"missing representative source `{source_path}`",
                }
            )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    root_path = SAMPLE_ROOT / row["root_name"]
    source_path = SAMPLE_ROOT / row["representative_source"]
    return {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "root_path": str(root_path.relative_to(REPO_ROOT)),
        "representative_source": str(source_path.relative_to(REPO_ROOT)),
        "runnable": row["current_status"] == "executable",
    }


def list_samples() -> list[dict[str, Any]]:
    data = _load_matrix_file()
    return [_materialize_row(row) for row in data["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix_file()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(SAMPLE_ROOT, data["rows"])
    planned_only_rows = [
        row["sample_id"] for row in rows if row["current_status"] == "planned_only"
    ]
    executable_rows = [
        row["sample_id"] for row in rows if row["current_status"] == "executable"
    ]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "current_add_one_reading": data["current_add_one_reading"],
        "sample_count": len(rows),
        "planned_count": len(planned_only_rows),
        "executable_count": len(executable_rows),
        "planned_only_rows": planned_only_rows,
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def _cargo_alpha_args(*args: str) -> list[str]:
    return ["cargo", "run", "-q", "-p", "mirrorea-cli", "--", *args, "--format", "json"]


def _run_json_command(argv: list[str], env: dict[str, str]) -> dict[str, Any]:
    completed = subprocess.run(
        argv,
        cwd=REPO_ROOT,
        env=env,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n{completed.stderr.strip()}"
        )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"command did not return JSON: {' '.join(argv)}"
        ) from error


def _run_product_alpha1_local_session(sample_root: Path) -> dict[str, Any]:
    env = os.environ.copy()
    with tempfile.TemporaryDirectory(prefix="mirrorea-comp-02-session-") as session_dir:
        env["MIRROREA_ALPHA_SESSION_DIR"] = session_dir
        return _run_json_command(_cargo_alpha_args("run-local", str(sample_root)), env)


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise RuntimeError(detail)


def _validate_comp02_runtime_payload(payload: dict[str, Any]) -> dict[str, Any]:
    _require(
        payload.get("surface_kind") == "product_alpha1_run_local_report",
        "comp-02 must return product_alpha1_run_local_report",
    )
    _require(
        payload.get("typed_host_io_claimed") is True,
        "comp-02 must keep typed_host_io_claimed",
    )
    _require(
        payload.get("mir_computation_claimed") is True,
        "comp-02 must set mir_computation_claimed",
    )

    session = payload.get("session") or {}
    host_io_history = session.get("host_io_history") or []
    mir_compute_history = session.get("mir_compute_history") or []
    event_nodes = (session.get("event_dag") or {}).get("nodes") or []
    event_kinds = [
        node.get("event_kind")
        for node in event_nodes
        if isinstance(node, dict) and node.get("event_kind")
    ]

    _require(len(host_io_history) == 2, "comp-02 must emit two host I/O history rows")
    _require(
        host_io_history[0].get("adapter_kind") == "ReadInt",
        "first host I/O row must be ReadInt",
    )
    _require(
        host_io_history[1].get("adapter_kind") == "WriteInt",
        "second host I/O row must be WriteInt",
    )
    _require(
        host_io_history[0].get("response_summary") == "Int(41)",
        "ReadInt must preserve Int(41)",
    )
    _require(
        host_io_history[1].get("response_summary") == "Int(42)",
        "WriteInt must preserve Int(42)",
    )
    _require(
        len(mir_compute_history) == 1,
        "comp-02 must emit one mir_compute_history row",
    )
    _require(
        mir_compute_history[0].get("function_id") == "add_one",
        "comp-02 must evaluate add_one",
    )
    _require(
        mir_compute_history[0].get("input_summary") == "Int(41)",
        "comp-02 compute input must be Int(41)",
    )
    _require(
        mir_compute_history[0].get("output_summary") == "Int(42)",
        "comp-02 compute output must be Int(42)",
    )

    required_sequence = [
        "host_input_received",
        "mir_compute_step",
        "host_output_emitted",
    ]
    indices = [event_kinds.index(kind) for kind in required_sequence]
    _require(indices == sorted(indices), "comp-02 event order must preserve input -> compute -> output")

    return {
        "session_id": session.get("session_id"),
        "host_io_history": host_io_history,
        "mir_compute_history": mir_compute_history,
        "event_kinds": event_kinds,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix_file()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise ValueError(f"unknown mir computational sample `{sample_id}`")
    realized = _materialize_row(row)
    if row["current_status"] != "executable":
        return {
            "command": "run",
            "family": data["family"],
            "sample_id": sample_id,
            "current_status": row["current_status"],
            "terminal_outcome": "planned_only",
            "rejection_reason": (
                f"{row['stage']} is not implemented yet; this root is scaffold-only in P-COMP-01"
            ),
            "current_add_one_reading": data["current_add_one_reading"],
            "stop_lines": list(STOP_LINES),
            "row": realized,
        }

    payload = _run_product_alpha1_local_session(SAMPLE_ROOT / row["root_name"])
    runtime = _validate_comp02_runtime_payload(payload)
    return {
        "command": "run",
        "family": data["family"],
        "sample_id": sample_id,
        "current_status": row["current_status"],
        "terminal_outcome": "accepted",
        "current_add_one_reading": data["current_add_one_reading"],
        "typed_host_io_claimed": payload["typed_host_io_claimed"],
        "mir_computation_claimed": payload["mir_computation_claimed"],
        "mir_compute_function": runtime["mir_compute_history"][0]["function_id"],
        "event_kinds_after": runtime["event_kinds"],
        "session_id": runtime["session_id"],
        "row": realized,
    }


def check_all() -> dict[str, Any]:
    status = matrix()
    failed = [error["sample_id"] for error in status["validation_errors"]]
    passed: list[str] = []
    runtime_failures: list[dict[str, str]] = []
    for row in status["rows"]:
        if row["current_status"] != "executable":
            continue
        try:
            result = run_sample(row["sample_id"])
            if result["terminal_outcome"] == "accepted":
                passed.append(row["sample_id"])
            else:
                failed.append(row["sample_id"])
        except Exception as error:  # pragma: no cover - exercised via CLI/runtime failures
            failed.append(row["sample_id"])
            runtime_failures.append(
                {
                    "sample_id": row["sample_id"],
                    "detail": str(error),
                }
            )
    return {
        "command": "check-all",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "sample_count": status["sample_count"],
        "planned": list(status["planned_only_rows"]),
        "passed": passed,
        "failed": failed,
        "matrix_status": status["matrix_status"],
        "current_add_one_reading": status["current_add_one_reading"],
        "workflow_ready": False,
        "validation_errors": status["validation_errors"],
        "runtime_failures": runtime_failures,
    }


def closeout() -> dict[str, Any]:
    status = matrix()
    return {
        "command": "closeout",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "planned_sample_ids": [row["sample_id"] for row in status["rows"]],
        "current_add_one_reading": status["current_add_one_reading"],
        "workflow_ready": False,
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
        "validation_errors": status["validation_errors"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["PREVIEW SAMPLES"]
        for row in payload:
            lines.append(
                f"- {row['sample_id']} [{row['current_status']}] -> {row['representative_source']}"
            )
        return "\n".join(lines)

    command = payload.get("command")
    if command == "matrix":
        return "\n".join(
            [
                "MATRIX SUMMARY",
                f"sample root: {payload['sample_root']}",
                f"samples: {payload['sample_count']}",
                f"planned-only: {payload['planned_count']}",
                f"executable: {payload['executable_count']}",
            ]
        )
    if command == "run":
        lines = [
            "RUN SUMMARY",
            f"sample: {payload['sample_id']}",
            f"status: {payload['current_status']}",
            f"outcome: {payload['terminal_outcome']}",
        ]
        if payload["terminal_outcome"] == "accepted":
            lines.extend(
                [
                    f"mir function: {payload['mir_compute_function']}",
                    "events: " + ", ".join(payload["event_kinds_after"]),
                ]
            )
        else:
            lines.append(f"reason: {payload['rejection_reason']}")
        return "\n".join(lines)
    if command == "check-all":
        return "\n".join(
            [
                "CHECK-ALL SUMMARY",
                f"sample count: {payload['sample_count']}",
                f"planned-only: {len(payload['planned'])}",
                "planned ids: " + ", ".join(payload["planned"]),
                f"passed rows: {len(payload['passed'])}",
                f"failed rows: {len(payload['failed'])}",
            ]
        )
    if command == "closeout":
        return "\n".join(
            [
                "CLOSEOUT SUMMARY",
                f"sample root: {payload['sample_root']}",
                "planned ids: " + ", ".join(payload["planned_sample_ids"]),
                "stop lines: " + "; ".join(payload["stop_lines"]),
            ]
        )
    return json.dumps(payload, indent=2, ensure_ascii=False)


def _print(payload: Any, fmt: str) -> None:
    if fmt == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(payload))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["pretty", "json"], default="pretty")
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("list")
    subparsers.add_parser("matrix")
    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
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
    if remainder and remainder[0] not in KNOWN_COMMANDS and not remainder[0].startswith("-"):
        return [*hoisted_root_options, "run", *remainder]
    return values


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))

    if args.command == "list":
        payload = list_samples()
    elif args.command == "matrix":
        payload = matrix()
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "closeout":
        payload = closeout()
    else:
        payload = run_sample(args.sample_id)

    _print(payload, args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
