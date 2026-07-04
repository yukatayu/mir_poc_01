#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter"
MATRIX_PATH = SAMPLE_ROOT / "renderer-pose-matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no renderer-owned world semantics",
    "no Unity / Unreal / WASM renderer execution claim",
    "no arbitrary native package execution",
    "no final public engine/provider SDK",
]
NON_CLAIMS = [
    "renderer pose backend is bounded same-process delivery evidence only",
    "provider admission remains inventory-bounded and native-disabled by default",
    "posegraph runtime remains the semantic owner of pose acceptance and rejection",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture",
    "cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture",
    "python3 -m unittest scripts.tests.test_renderer_pose_backend_samples",
    "python3 scripts/renderer_pose_backend_samples.py check-all --format json",
]


def _load_matrix() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _row_source_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["source"]


def _row_request_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["request"]


def _row_provider_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["provider_manifest"]


def _row_posegraph_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["posegraph_package"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["expected"]


def _row_generated_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["generated"]


def _row_generated_provider_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["generated_provider_admission"]


def _repo_relative_arg(path: Path) -> str:
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = SAMPLE_ROOT / row["root_name"]
        readme_path = root_path / "README.md"
        for kind, path in [
            ("missing_root", root_path),
            ("missing_readme", readme_path),
            ("missing_source", _row_source_path(row)),
            ("missing_request", _row_request_path(row)),
            ("missing_provider_manifest", _row_provider_path(row)),
            ("missing_posegraph_package", _row_posegraph_path(row)),
            ("missing_expected", _row_expected_path(row)),
        ]:
            if not path.exists():
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": kind,
                        "detail": f"missing path `{path}`",
                    }
                )
        if not isinstance(row.get("input"), int):
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_input",
                    "detail": "sample rows must declare integer `input`",
                }
            )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "source": str(_row_source_path(row).relative_to(REPO_ROOT)),
        "request": str(_row_request_path(row).relative_to(REPO_ROOT)),
        "provider_manifest": str(_row_provider_path(row).relative_to(REPO_ROOT)),
        "posegraph_package": str(_row_posegraph_path(row).relative_to(REPO_ROOT)),
        "expected": str(_row_expected_path(row).relative_to(REPO_ROOT)),
        "generated": str(_row_generated_path(row).relative_to(REPO_ROOT)),
        "generated_provider_admission": str(
            _row_generated_provider_path(row).relative_to(REPO_ROOT)
        ),
        "input": row["input"],
    }


def list_samples() -> list[dict[str, Any]]:
    return [_materialize_row(row) for row in _load_matrix()["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    executable_rows = [row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"]
    validation_errors = validate_rows(data["rows"])
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def _run_renderer_pose_backend(
    source: Path, request: Path, provider: Path, posegraph_package: Path, input_value: int
) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mirrorea-cli",
            "--",
            "render-pose-backend-v1",
            _repo_relative_arg(source),
            "--request",
            _repo_relative_arg(request),
            "--provider",
            _repo_relative_arg(provider),
            "--posegraph-package",
            _repo_relative_arg(posegraph_package),
            "--input",
            str(input_value),
            "--format",
            "json",
        ],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"renderer pose backend CLI did not return JSON for `{source}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _summary(payload: dict[str, Any]) -> dict[str, Any]:
    return {
        "accepted": payload.get("accepted"),
        "delivery_admitted": payload.get("delivery_admitted"),
        "terminal_outcome": payload.get("terminal_outcome"),
        "blocked_reason": payload.get("blocked_reason"),
        "provider_id": payload.get("provider_id"),
        "provider_kind": payload.get("provider_kind"),
        "target_id": payload.get("target_id"),
        "target_provider_policy": payload.get("target_provider_policy"),
        "pose_snapshot_frontier": payload.get("pose_snapshot_frontier"),
        "delivered_pose_snapshot_ref": payload.get("delivered_pose_snapshot_ref"),
        "expected_binding_context": payload.get("expected_binding_context"),
        "posegraph_binding_context": payload.get("posegraph_binding_context"),
        "delivered_node_entities": [
            row["entity_ref"] for row in (payload.get("delivered_nodes") or [])
        ],
        "delivered_node_pose_versions": {
            row["entity_ref"]: row["pose_version"]
            for row in (payload.get("delivered_nodes") or [])
        },
        "matched_packet_schema_refs": sorted(payload.get("matched_packet_schema_refs") or []),
        "matched_ffi_schema_refs": sorted(payload.get("matched_ffi_schema_refs") or []),
        "diagnostic_codes": sorted(row["code"] for row in (payload.get("diagnostics") or [])),
        "provider_terminal_outcome": (
            payload.get("provider_admission_report") or {}
        ).get("terminal_outcome"),
        "posegraph_terminal_outcome": (
            payload.get("posegraph_runtime_report") or {}
        ).get("terminal_outcome"),
        "posegraph_violation_kind": (
            (payload.get("posegraph_runtime_report") or {}).get("violation") or {}
        ).get("violation_kind"),
        "posegraph_rejection_code": (
            (payload.get("posegraph_runtime_report") or {}).get("rejection") or {}
        ).get("code"),
        "residual_obligation_codes": sorted(
            row["code"] for row in (payload.get("residual_obligations") or [])
        ),
    }


def _generated_payload(payload: dict[str, Any]) -> dict[str, Any]:
    return {
        "surface_kind": payload.get("surface_kind"),
        "projection_id": payload.get("projection_id"),
        "provider_id": payload.get("provider_id"),
        "provider_kind": payload.get("provider_kind"),
        "target_id": payload.get("target_id"),
        "target_provider_policy": payload.get("target_provider_policy"),
        "semantic_owner": payload.get("semantic_owner"),
        "accepted": payload.get("accepted"),
        "delivery_admitted": payload.get("delivery_admitted"),
        "terminal_outcome": payload.get("terminal_outcome"),
        "blocked_reason": payload.get("blocked_reason"),
        "pose_snapshot_frontier": payload.get("pose_snapshot_frontier"),
        "delivered_pose_snapshot_ref": payload.get("delivered_pose_snapshot_ref"),
        "expected_binding_context": payload.get("expected_binding_context"),
        "posegraph_binding_context": payload.get("posegraph_binding_context"),
        "delivered_nodes": payload.get("delivered_nodes") or [],
        "matched_packet_schema_refs": payload.get("matched_packet_schema_refs") or [],
        "matched_ffi_schema_refs": payload.get("matched_ffi_schema_refs") or [],
        "diagnostic_codes": [row["code"] for row in (payload.get("diagnostics") or [])],
        "residual_obligation_codes": [
            row["code"] for row in (payload.get("residual_obligations") or [])
        ],
        "provider_terminal_outcome": (
            payload.get("provider_admission_report") or {}
        ).get("terminal_outcome"),
        "posegraph_terminal_outcome": (
            payload.get("posegraph_runtime_report") or {}
        ).get("terminal_outcome"),
    }


def run(sample_id: str) -> dict[str, Any]:
    data = _load_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise ValueError(f"unknown renderer pose backend sample `{sample_id}`")

    actual_payload = _run_renderer_pose_backend(
        _row_source_path(row),
        _row_request_path(row),
        _row_provider_path(row),
        _row_posegraph_path(row),
        row["input"],
    )
    actual = _summary(actual_payload)
    expected = json.loads(_row_expected_path(row).read_text(encoding="utf-8"))
    generated_actual = _generated_payload(actual_payload)

    generated_path = _row_generated_path(row)
    generated_path.parent.mkdir(parents=True, exist_ok=True)
    generated_path.write_text(
        json.dumps(generated_actual, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    provider_generated_path = _row_generated_provider_path(row)
    provider_generated_path.parent.mkdir(parents=True, exist_ok=True)
    provider_generated_path.write_text(
        json.dumps(
            actual_payload.get("provider_admission_report") or {},
            indent=2,
            ensure_ascii=False,
        )
        + "\n",
        encoding="utf-8",
    )

    return {
        "command": "run",
        "family": data["family"],
        "sample_id": sample_id,
        "accepted": actual_payload.get("accepted"),
        "returncode": actual_payload.get("returncode"),
        "expected": expected,
        "actual": actual,
        "generated_actual": generated_actual,
        "matches_expected": actual == expected,
    }


def check_all() -> dict[str, Any]:
    data = _load_matrix()
    passed: list[str] = []
    failed: list[str] = []
    for row in data["rows"]:
        payload = run(row["sample_id"])
        if payload["matches_expected"]:
            passed.append(row["sample_id"])
        else:
            failed.append(row["sample_id"])
    status = matrix()
    return {
        "command": "check-all",
        "family": data["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "sample_count": status["sample_count"],
        "passed": passed,
        "failed": failed,
        "validation_errors": status["validation_errors"],
        "workflow_ready": False,
    }


def closeout() -> dict[str, Any]:
    status = matrix()
    return {
        "command": "closeout",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "sample_ids": [row["sample_id"] for row in status["rows"]],
        "workflow_ready": False,
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
        "validation_errors": status["validation_errors"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["RENDERER POSE BACKEND SAMPLES"]
        for row in payload:
            lines.append(f"- {row['sample_id']} [{row['current_status']}] -> {row['source']}")
        return "\n".join(lines)
    command = payload.get("command")
    if command == "matrix":
        return "\n".join(
            [
                "MATRIX SUMMARY",
                f"sample root: {payload['sample_root']}",
                f"samples: {payload['sample_count']}",
                f"executable: {payload['executable_count']}",
            ]
        )
    if command == "run":
        return "\n".join(
            [
                "RUN SUMMARY",
                f"sample: {payload['sample_id']}",
                f"accepted: {payload['accepted']}",
                f"matches expected: {payload['matches_expected']}",
            ]
        )
    if command == "check-all":
        return "\n".join(
            [
                "CHECK-ALL SUMMARY",
                f"sample count: {payload['sample_count']}",
                f"passed: {len(payload['passed'])}",
                f"failed: {len(payload['failed'])}",
            ]
        )
    if command == "closeout":
        return "\n".join(
            [
                "CLOSEOUT SUMMARY",
                f"sample root: {payload['sample_root']}",
                "sample ids: " + ", ".join(payload["sample_ids"]),
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
        payload = run(args.sample_id)

    _print(payload, args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
