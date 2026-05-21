#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "computational"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no runtime completion yet",
    "no final textual grammar",
    "do not treat current AddOne as Mir-owned computation",
]
NON_CLAIMS = [
    "no Mir-owned runtime execution yet",
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
    planned_count = sum(1 for row in rows if row["current_status"] == "planned_only")
    executable_count = sum(1 for row in rows if row["current_status"] == "executable")
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "current_add_one_reading": data["current_add_one_reading"],
        "sample_count": len(rows),
        "planned_count": planned_count,
        "executable_count": executable_count,
        "planned_only_rows": planned_only_rows,
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
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
    raise NotImplementedError("executable computational rows are introduced after P-COMP-01")


def check_all() -> dict[str, Any]:
    status = matrix()
    failed = [error["sample_id"] for error in status["validation_errors"]]
    planned = list(status["planned_only_rows"])
    passed = list(status["executable_rows"])
    return {
        "command": "check-all",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "sample_count": status["sample_count"],
        "planned": planned,
        "passed": passed,
        "failed": failed,
        "matrix_status": status["matrix_status"],
        "current_add_one_reading": status["current_add_one_reading"],
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
