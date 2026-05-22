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
FULL_SYSTEM_V1_ROOT = REPO_ROOT / "samples" / "full-system-v1"
SAMPLE_ROOTS = {
    "projection": FULL_SYSTEM_V1_ROOT / "projection",
    "server-client": FULL_SYSTEM_V1_ROOT / "server-client",
}
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final packet or FFI payload schema semantics completion yet",
    "no generated server/client binaries or distributed planner yet",
    "no LLVM/backend code generation",
    "no provider admission completion",
]
NON_CLAIMS = [
    "projection IR plus boundary-schema plus same-binary local role split floor only",
    "no final transport semantics or deployment planner completion",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-runtime --test projection_ir -- --nocapture",
    "cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture",
    "python3 -m unittest scripts.tests.test_projection_v1_samples",
    "python3 scripts/projection_v1_samples.py check-all --format json",
]
LEGACY_PROJECTION_ARTIFACTS = [
    "generated/target-manifest.json",
]
SUPPORTED_GENERATED_KINDS = {
    "projection_artifacts",
    "rejection_report",
    "local_split_report",
    "local_split_rejection",
}
SUPPORTED_RUNNERS = {"projection", "local_split"}


def _load_rows() -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    rows: list[dict[str, Any]] = []
    roots: list[dict[str, Any]] = []
    for root_name, root_path in SAMPLE_ROOTS.items():
        matrix_path = root_path / "matrix.json"
        data = json.loads(matrix_path.read_text(encoding="utf-8"))
        roots.append(
            {
                "root_name": root_name,
                "family": data["family"],
                "matrix_path": matrix_path,
                "matrix_status": data["current_status"],
                "sample_root": root_path,
            }
        )
        for row in data["rows"]:
            normalized = dict(row)
            normalized["runner_kind"] = row.get("runner_kind", "projection")
            normalized["_root_name"] = root_name
            normalized["_family"] = data["family"]
            normalized["_matrix_path"] = matrix_path
            normalized["_sample_root"] = root_path
            normalized["_matrix_status"] = data["current_status"]
            rows.append(normalized)
    return rows, roots


def _row_source_path(row: dict[str, Any]) -> Path:
    return row["_sample_root"] / row["source"]


def _row_request_path(row: dict[str, Any]) -> Path:
    return row["_sample_root"] / row["request"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return row["_sample_root"] / row["expected"]


def _row_generated_artifact(row: dict[str, Any]) -> Path:
    return row["_sample_root"] / row["generated_artifact"]


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        sample_root = row["_sample_root"]
        root_path = sample_root / row["root_name"]
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
        if row["generated_kind"] not in SUPPORTED_GENERATED_KINDS:
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_generated_kind",
                    "detail": f"unsupported generated_kind `{row['generated_kind']}`",
                }
            )
        if row["runner_kind"] not in SUPPORTED_RUNNERS:
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_runner_kind",
                    "detail": f"unsupported runner_kind `{row['runner_kind']}`",
                }
            )
        for legacy_path in LEGACY_PROJECTION_ARTIFACTS:
            stale_path = root_path / legacy_path
            if stale_path.exists():
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": "stale_generated_artifact",
                        "detail": f"unexpected legacy artifact `{stale_path}` is still present",
                    }
                )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    payload = {
        "sample_id": row["sample_id"],
        "family": row["_family"],
        "root_group": row["_root_name"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "runner_kind": row["runner_kind"],
        "source": str(_row_source_path(row).relative_to(REPO_ROOT)),
        "request": str(_row_request_path(row).relative_to(REPO_ROOT)),
        "expected": str(_row_expected_path(row).relative_to(REPO_ROOT)),
        "generated_artifact": str(_row_generated_artifact(row).relative_to(REPO_ROOT)),
        "generated_kind": row["generated_kind"],
    }
    if "input" in row:
        payload["input"] = row["input"]
    if "target_id" in row:
        payload["target_id"] = row["target_id"]
    if "entry_override" in row:
        payload["entry_override"] = row["entry_override"]
    return payload


def list_samples() -> list[dict[str, Any]]:
    rows, _ = _load_rows()
    return [_materialize_row(row) for row in rows]


def matrix() -> dict[str, Any]:
    rows, roots = _load_rows()
    materialized_rows = [_materialize_row(row) for row in rows]
    validation_errors = validate_rows(rows)
    executable_rows = [
        row["sample_id"] for row in rows if row["current_status"] == "executable"
    ]
    return {
        "command": "matrix",
        "family": "full_system_v1_projection_backend",
        "sample_root": str(FULL_SYSTEM_V1_ROOT.relative_to(REPO_ROOT)),
        "matrix_paths": [
            str(root["matrix_path"].relative_to(REPO_ROOT)) for root in roots
        ],
        "sample_count": len(materialized_rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": sorted({root["matrix_status"] for root in roots}),
        "workflow_ready": True,
        "roots": [
            {
                "root_name": root["root_name"],
                "family": root["family"],
                "matrix_path": str(root["matrix_path"].relative_to(REPO_ROOT)),
                "matrix_status": root["matrix_status"],
            }
            for root in roots
        ],
        "rows": materialized_rows,
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


def _run_local_split(row: dict[str, Any], path: Path, request_path: Path) -> dict[str, Any]:
    command = [
        "cargo",
        "run",
        "-q",
        "-p",
        "mir-runtime",
        "--example",
        "mir_full_system_v1_local_split",
        "--",
        str(path),
        "--request",
        str(request_path),
        "--input",
        str(row.get("input", 0)),
        "--format",
        "json",
    ]
    if row.get("target_id"):
        command.extend(["--target", row["target_id"]])
    if row.get("entry_override"):
        command.extend(["--entry", row["entry_override"]])
    completed = subprocess.run(
        command,
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
            f"mir_full_system_v1_local_split did not return JSON for `{path}`: {completed.stderr}"
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
        "packet_schema_count": len(payload.get("packet_schemas") or []),
        "ffi_schema_count": len(payload.get("ffi_schemas") or []),
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


def _payload_local_split(payload: dict[str, Any]) -> dict[str, Any]:
    target_reports = payload.get("target_reports") or []
    target_summaries = {}
    for target in target_reports:
        host_output_summaries = []
        published_channels = []
        runtime_acceptance = []
        for session in target.get("runtime_sessions") or []:
            runtime = session.get("runtime") or {}
            runtime_acceptance.append(runtime.get("accepted"))
            effect_session = runtime.get("effect_session") or {}
            host_output_summaries.extend(
                row.get("summary") for row in effect_session.get("host_output") or []
            )
            published_channels.extend(effect_session.get("published_channels") or [])
        target_summaries[target["target_id"]] = {
            "accepted": target.get("accepted"),
            "role": target.get("role"),
            "execution_kind": target.get("execution_kind"),
            "admitted_entry_transitions": sorted(
                target.get("admitted_entry_transitions") or []
            ),
            "launched_entry_transitions": sorted(
                target.get("launched_entry_transitions") or []
            ),
            "runtime_session_count": len(target.get("runtime_sessions") or []),
            "runtime_acceptance": runtime_acceptance,
            "host_output_summaries": host_output_summaries,
            "published_channels": sorted(set(published_channels)),
            "observer_safe_summary": target.get("observer_safe_summary"),
        }
    return {
        "accepted": payload.get("accepted"),
        "projection_id": payload.get("projection_id"),
        "selected_target_id": payload.get("selected_target_id"),
        "entry_override": payload.get("entry_override"),
        "launch_mode": payload.get("launch_mode"),
        "target_ids": sorted(target_summaries.keys()),
        "target_summaries": target_summaries,
        "rejected_rows": sorted(payload.get("rejected_rows") or []),
        "diagnostic_codes": sorted(row["code"] for row in payload.get("diagnostics") or []),
        "residual_obligation_codes": sorted(
            row["code"] for row in payload.get("residual_obligations") or []
        ),
    }


def _generated_payload(payload: dict[str, Any], generated_kind: str) -> Any:
    if generated_kind == "projection_artifacts":
        return {
            "target_manifests": payload.get("target_manifests") or [],
            "packet_schemas": payload.get("packet_schemas") or [],
            "ffi_schemas": payload.get("ffi_schemas") or [],
        }
    if generated_kind == "rejection_report":
        preservation = payload.get("preservation_report") or {}
        return {
            "diagnostic_codes": sorted(
                row["code"] for row in payload.get("diagnostics") or []
            ),
            "rejected_rows": sorted(preservation.get("rejected_rows") or []),
        }
    if generated_kind == "local_split_report":
        return {
            "launch_mode": payload.get("launch_mode"),
            "target_reports": [
                {
                    "target_id": target["target_id"],
                    "role": target["role"],
                    "execution_kind": target["execution_kind"],
                    "launched_entry_transitions": sorted(
                        target.get("launched_entry_transitions") or []
                    ),
                    "inbound_boundary_refs": sorted(
                        target.get("inbound_boundary_refs") or []
                    ),
                    "outbound_boundary_refs": sorted(
                        target.get("outbound_boundary_refs") or []
                    ),
                    "inbound_packet_schema_refs": sorted(
                        target.get("inbound_packet_schema_refs") or []
                    ),
                    "outbound_packet_schema_refs": sorted(
                        target.get("outbound_packet_schema_refs") or []
                    ),
                    "inbound_ffi_schema_refs": sorted(
                        target.get("inbound_ffi_schema_refs") or []
                    ),
                    "outbound_ffi_schema_refs": sorted(
                        target.get("outbound_ffi_schema_refs") or []
                    ),
                    "runtime_session_ids": sorted(
                        row["session_id"] for row in target.get("runtime_sessions") or []
                    ),
                    "observer_safe_summary": target.get("observer_safe_summary"),
                }
                for target in sorted(
                    payload.get("target_reports") or [],
                    key=lambda row: row["target_id"],
                )
            ],
        }
    if generated_kind == "local_split_rejection":
        return {
            "diagnostic_codes": sorted(
                row["code"] for row in payload.get("diagnostics") or []
            ),
            "rejected_rows": sorted(payload.get("rejected_rows") or []),
        }
    raise ValueError(f"unsupported generated_kind `{generated_kind}`")


def _run_row(row: dict[str, Any]) -> dict[str, Any]:
    source_path = _row_source_path(row)
    request_path = _row_request_path(row)
    expected_path = _row_expected_path(row)
    generated_artifact = _row_generated_artifact(row)

    if row["runner_kind"] == "projection":
        actual_payload = _run_projection(source_path, request_path)
        actual = _payload_projection(actual_payload)
    elif row["runner_kind"] == "local_split":
        actual_payload = _run_local_split(row, source_path, request_path)
        actual = _payload_local_split(actual_payload)
    else:
        raise ValueError(f"unsupported runner_kind `{row['runner_kind']}`")

    expected = json.loads(expected_path.read_text(encoding="utf-8"))
    generated_expected = json.loads(generated_artifact.read_text(encoding="utf-8"))
    generated_actual = _generated_payload(actual_payload, row["generated_kind"])
    passed = actual == expected and generated_actual == generated_expected
    return {
        "sample_id": row["sample_id"],
        "family": row["_family"],
        "runner_kind": row["runner_kind"],
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
    rows, _ = _load_rows()
    row = next((row for row in rows if row["sample_id"] == sample_id), None)
    if row is None:
        raise KeyError(sample_id)
    return _run_row(row)


def check_all() -> dict[str, Any]:
    rows, _ = _load_rows()
    validation_errors = validate_rows(rows)
    if validation_errors:
        return {
            "command": "check-all",
            "family": "full_system_v1_projection_backend",
            "passed": [],
            "failed": [],
            "validation_errors": validation_errors,
        }

    passed = []
    failed = []
    for row in rows:
        result = _run_row(row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)
    return {
        "command": "check-all",
        "family": "full_system_v1_projection_backend",
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
    parser = argparse.ArgumentParser(
        description="Run Full System V1 projection and local split samples."
    )
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
