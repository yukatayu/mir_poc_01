#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "projection"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
COMPATIBILITY_PATH = (
    SAMPLE_ROOT
    / "manifest-provider-compatibility"
    / "manifest-provider-compatibility.json"
)
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no generated server/client binary",
    "no LLVM/backend execution",
    "no direct Mir-to-machine-code",
    "no deployment planner completion",
    "no placement optimizer completion",
    "no projection equivalence checker completion",
    "no arbitrary native or WASM execution",
]
NON_CLAIMS = [
    "current executable/native truth remains host launch bundle only",
    "projection inventory is supplementary and non-codegen",
    "provider compatibility inventory is not runtime admission",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_projection_boundary_samples",
    "python3 scripts/projection_boundary_samples.py matrix --format json",
    "python3 scripts/projection_boundary_samples.py check-all --format json",
    "python3 scripts/projection_boundary_samples.py run proj-01-server-client-target-manifest --format json",
]


def _load_matrix_file() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _load_compatibility_file() -> dict[str, Any]:
    return json.loads(COMPATIBILITY_PATH.read_text(encoding="utf-8"))


def validate_rows(
    sample_root: Path, rows: list[dict[str, Any]]
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = sample_root / row["root_name"]
        artifact_path = sample_root / row["representative_artifact"]
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not artifact_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_representative_artifact",
                    "detail": f"missing representative artifact `{artifact_path}`",
                }
            )
    return errors


def _validate_compatibility_inventory(
    compatibility_data: dict[str, Any], expected: dict[str, Any]
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    rows = compatibility_data["rows"]
    row_ids = {row["compatibility_row_id"]: row for row in rows}
    for row_id in expected["accepted_rows"]:
        if row_id not in row_ids:
            errors.append(
                {
                    "sample_id": "proj-01-manifest-provider-compatibility",
                    "kind": "missing_compatibility_row",
                    "detail": f"missing accepted compatibility row `{row_id}`",
                }
            )
            continue
        if row_ids[row_id]["compatibility_status"] != "accepted":
            errors.append(
                {
                    "sample_id": "proj-01-manifest-provider-compatibility",
                    "kind": "compatibility_status_mismatch",
                    "detail": f"row `{row_id}` is not marked accepted",
                }
            )
    for row_id in expected["rejected_rows"]:
        if row_id not in row_ids:
            errors.append(
                {
                    "sample_id": "proj-01-manifest-provider-compatibility",
                    "kind": "missing_compatibility_row",
                    "detail": f"missing rejected compatibility row `{row_id}`",
                }
            )
            continue
        if row_ids[row_id]["compatibility_status"] != "rejected":
            errors.append(
                {
                    "sample_id": "proj-01-manifest-provider-compatibility",
                    "kind": "compatibility_status_mismatch",
                    "detail": f"row `{row_id}` is not marked rejected",
                }
            )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    root_path = SAMPLE_ROOT / row["root_name"]
    artifact_path = SAMPLE_ROOT / row["representative_artifact"]
    return {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "inventory_kind": row["inventory_kind"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "workflow_ready": False,
        "root_path": str(root_path.relative_to(REPO_ROOT)),
        "representative_artifact": str(artifact_path.relative_to(REPO_ROOT)),
    }


def list_samples() -> list[dict[str, Any]]:
    data = _load_matrix_file()
    return [_materialize_row(row) for row in data["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix_file()
    compatibility_data = _load_compatibility_file()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(SAMPLE_ROOT, data["rows"])
    validation_errors.extend(
        _validate_compatibility_inventory(
            compatibility_data, data["compatibility_inventory"]
        )
    )
    planned_only_rows = [
        row["sample_id"] for row in rows if row["current_status"] == "planned_only"
    ]
    accepted_rows = list(data["compatibility_inventory"]["accepted_rows"])
    rejected_rows = list(data["compatibility_inventory"]["rejected_rows"])
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "projection_scope": data["projection_scope"],
        "current_native_truth": data["current_native_truth"],
        "pipeline": list(data["pipeline"]),
        "sample_count": len(rows),
        "planned_count": len(planned_only_rows),
        "executable_count": 0,
        "planned_only_rows": planned_only_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "accepted_rows": accepted_rows,
        "rejected_rows": rejected_rows,
        "compatibility_relation": compatibility_data["relation_name"],
        "compatibility_rows": list(compatibility_data["rows"]),
        "rows": rows,
        "validation_errors": validation_errors,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix_file()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise ValueError(f"unknown projection boundary sample `{sample_id}`")
    realized = _materialize_row(row)
    return {
        "command": "run",
        "family": data["family"],
        "sample_id": sample_id,
        "current_status": row["current_status"],
        "terminal_outcome": "planned_only",
        "rejection_reason": (
            f"{row['stage']} is inventory-only; the later projection realization package is not implemented yet"
        ),
        "projection_scope": data["projection_scope"],
        "current_native_truth": data["current_native_truth"],
        "stop_lines": list(STOP_LINES),
        "workflow_ready": False,
        "row": realized,
    }


def check_all() -> dict[str, Any]:
    status = matrix()
    failed = [error["sample_id"] for error in status["validation_errors"]]
    return {
        "command": "check-all",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "sample_count": status["sample_count"],
        "planned": list(status["planned_only_rows"]),
        "passed": [],
        "failed": failed,
        "accepted_rows": list(status["accepted_rows"]),
        "rejected_rows": list(status["rejected_rows"]),
        "current_native_truth": status["current_native_truth"],
        "workflow_ready": False,
        "validation_errors": status["validation_errors"],
    }


def closeout() -> dict[str, Any]:
    status = matrix()
    return {
        "command": "closeout",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "planned_sample_ids": [row["sample_id"] for row in status["rows"]],
        "projection_scope": status["projection_scope"],
        "current_native_truth": status["current_native_truth"],
        "workflow_ready": False,
        "accepted_compatibility_rows": list(status["accepted_rows"]),
        "rejected_compatibility_rows": list(status["rejected_rows"]),
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
        "validation_errors": status["validation_errors"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["PROJECTION BOUNDARY SAMPLES"]
        for row in payload:
            lines.append(
                f"- {row['sample_id']} [{row['current_status']}] -> {row['representative_artifact']}"
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
                "accepted compatibility: " + ", ".join(payload["accepted_rows"]),
                "rejected compatibility: " + ", ".join(payload["rejected_rows"]),
            ]
        )
    if command == "run":
        return "\n".join(
            [
                "RUN SUMMARY",
                f"sample: {payload['sample_id']}",
                f"status: {payload['current_status']}",
                f"outcome: {payload['terminal_outcome']}",
                f"reason: {payload['rejection_reason']}",
            ]
        )
    if command == "check-all":
        return "\n".join(
            [
                "CHECK-ALL SUMMARY",
                f"sample count: {payload['sample_count']}",
                f"planned-only: {len(payload['planned'])}",
                "planned ids: " + ", ".join(payload["planned"]),
                "accepted compatibility: " + ", ".join(payload["accepted_rows"]),
                "rejected compatibility: " + ", ".join(payload["rejected_rows"]),
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
    if remainder and remainder[0] not in KNOWN_COMMANDS and not remainder[0].startswith(
        "-"
    ):
        return [*hoisted_root_options, "run", *remainder]
    return values


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))
    if args.command == "list":
        _print(list_samples(), args.format)
    elif args.command == "matrix":
        _print(matrix(), args.format)
    elif args.command == "run":
        _print(run_sample(args.sample_id), args.format)
    elif args.command == "check-all":
        _print(check_all(), args.format)
    else:
        _print(closeout(), args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
