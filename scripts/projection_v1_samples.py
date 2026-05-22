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
SAMPLE_ROOT = REPO_ROOT / "samples" / "full-system-v1" / "projection"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no executable server/client split runtime yet",
    "no packet or FFI payload schema semantics completion yet",
    "no LLVM/backend code generation",
    "no provider admission completion",
]
NON_CLAIMS = [
    "projection IR plus target manifest floor only",
    "packet and FFI schema payload semantics remain later work",
    "projection does not yet execute split roles",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-runtime --test projection_ir -- --nocapture",
    "cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture",
    "python3 -m unittest scripts.tests.test_projection_v1_samples",
    "python3 scripts/projection_v1_samples.py check-all --format json",
]


def _load_matrix() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _row_source_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["source"]


def _row_request_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["request"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["expected"]


def _row_generated_artifact(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["generated_artifact"]


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = SAMPLE_ROOT / row["root_name"]
        readme_path = root_path / "README.md"
        source_path = _row_source_path(row)
        request_path = _row_request_path(row)
        expected_path = _row_expected_path(row)
        generated_artifact = _row_generated_artifact(row)
        for kind, path in [
            ("missing_root", root_path),
            ("missing_readme", readme_path),
            ("missing_source", source_path),
            ("missing_request", request_path),
            ("missing_expected", expected_path),
            ("missing_generated_artifact", generated_artifact),
        ]:
            if not path.exists():
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": kind,
                        "detail": f"missing required path `{path}`",
                    }
                )
        if row["generated_kind"] not in {"target_manifests", "rejection_report"}:
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_generated_kind",
                    "detail": f"unsupported generated_kind `{row['generated_kind']}`",
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
        "expected": str(_row_expected_path(row).relative_to(REPO_ROOT)),
        "generated_artifact": str(_row_generated_artifact(row).relative_to(REPO_ROOT)),
        "generated_kind": row["generated_kind"],
    }


def list_samples() -> list[dict[str, Any]]:
    return [_materialize_row(row) for row in _load_matrix()["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(data["rows"])
    executable_rows = [
        row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"
    ]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": True,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def _run_projection(path: Path, request_path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_full_system_v1_projection",
            "--",
            str(path),
            "--request",
            str(request_path),
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
            f"mir_full_system_v1_projection did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _payload_projection(payload: dict[str, Any]) -> dict[str, Any]:
    preservation = payload.get("preservation_report") or {}
    target_manifests = payload.get("target_manifests") or []
    return {
        "accepted": payload.get("accepted"),
        "projection_id": payload.get("projection_id"),
        "target_ids": sorted(row["target_id"] for row in target_manifests),
        "target_roles": sorted(row["role"] for row in target_manifests),
        "packet_schema_refs": sorted(preservation.get("packet_schema_refs") or []),
        "ffi_schema_refs": sorted(preservation.get("ffi_schema_refs") or []),
        "checked_effect_rows": sorted(preservation.get("checked_effect_rows") or []),
        "checked_failure_rows": sorted(preservation.get("checked_failure_rows") or []),
        "checked_capability_rows": sorted(
            preservation.get("checked_capability_rows") or []
        ),
        "checked_authority_rows": sorted(
            preservation.get("checked_authority_rows") or []
        ),
        "checked_provider_policy_rows": sorted(
            preservation.get("checked_provider_policy_rows") or []
        ),
        "checked_rollback_replay_cut_rows": sorted(
            preservation.get("checked_rollback_replay_cut_rows") or []
        ),
        "rejected_rows": sorted(preservation.get("rejected_rows") or []),
        "residual_obligation_codes": sorted(
            row["code"] for row in payload.get("residual_obligations") or []
        ),
        "diagnostic_codes": sorted(row["code"] for row in payload.get("diagnostics") or []),
    }


def _generated_projection(payload: dict[str, Any], generated_kind: str) -> Any:
    if generated_kind == "target_manifests":
        return payload.get("target_manifests") or []
    if generated_kind == "rejection_report":
        preservation = payload.get("preservation_report") or {}
        return {
            "diagnostic_codes": sorted(
                row["code"] for row in payload.get("diagnostics") or []
            ),
            "rejected_rows": sorted(preservation.get("rejected_rows") or []),
        }
    raise ValueError(f"unsupported generated_kind `{generated_kind}`")


def _run_row(row: dict[str, Any]) -> dict[str, Any]:
    source_path = _row_source_path(row)
    request_path = _row_request_path(row)
    expected_path = _row_expected_path(row)
    generated_artifact = _row_generated_artifact(row)
    actual_payload = _run_projection(source_path, request_path)
    actual = _payload_projection(actual_payload)
    expected = json.loads(expected_path.read_text(encoding="utf-8"))
    generated_expected = json.loads(generated_artifact.read_text(encoding="utf-8"))
    generated_actual = _generated_projection(actual_payload, row["generated_kind"])
    passed = actual == expected and generated_actual == generated_expected
    return {
        "sample_id": row["sample_id"],
        "source": str(source_path.relative_to(REPO_ROOT)),
        "request": str(request_path.relative_to(REPO_ROOT)),
        "expected_path": str(expected_path.relative_to(REPO_ROOT)),
        "generated_artifact": str(generated_artifact.relative_to(REPO_ROOT)),
        "accepted": bool(actual.get("accepted")),
        "returncode": actual_payload["returncode"],
        "passed": passed,
        "actual": actual,
        "expected": expected,
        "generated_actual": generated_actual,
        "generated_expected": generated_expected,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise KeyError(sample_id)
    return _run_row(row)


def check_all() -> dict[str, Any]:
    data = _load_matrix()
    validation_errors = validate_rows(data["rows"])
    if validation_errors:
        return {
            "command": "check-all",
            "family": data["family"],
            "passed": [],
            "failed": [],
            "validation_errors": validation_errors,
        }

    passed = []
    failed = []
    for row in data["rows"]:
        result = _run_row(row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)
    return {
        "command": "check-all",
        "family": data["family"],
        "passed": passed,
        "failed": failed,
        "validation_errors": validation_errors,
    }


def closeout() -> dict[str, Any]:
    summary = check_all()
    return {
        "command": "closeout",
        "passed": summary["passed"],
        "failed": summary["failed"],
        "validation_errors": summary["validation_errors"],
        "validation_floor": VALIDATION_FLOOR,
        "stop_lines": STOP_LINES,
        "non_claims": NON_CLAIMS,
    }


def _emit(payload: dict[str, Any], output_format: str) -> None:
    print(json.dumps(payload, indent=2, ensure_ascii=False))


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Run Full System V1 projection samples.")
    parser.add_argument("command", choices=sorted(KNOWN_COMMANDS))
    parser.add_argument("sample_id", nargs="?")
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    args = parser.parse_args(argv)

    try:
        if args.command == "list":
            _emit({"command": "list", "rows": list_samples()}, args.format)
            return 0
        if args.command == "matrix":
            payload = matrix()
            _emit(payload, args.format)
            return 0 if not payload["validation_errors"] else 2
        if args.command == "run":
            if args.sample_id is None:
                raise SystemExit("sample_id is required for run")
            payload = run_sample(args.sample_id)
            _emit(payload, args.format)
            return 0 if payload["passed"] else 2
        if args.command == "check-all":
            payload = check_all()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
        if args.command == "closeout":
            payload = closeout()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
    except KeyError as error:
        _emit({"command": args.command, "status": "error", "sample_id": str(error)}, args.format)
        return 2
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
