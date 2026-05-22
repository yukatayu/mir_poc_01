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
SAMPLE_ROOT = REPO_ROOT / "samples" / "full-system-v1" / "avatar-pose"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no pose-aware save/load completion yet",
    "no final devtools panel family yet",
    "no renderer or engine semantic ownership",
    "no Unity / Unreal / VRM / VRChat compatibility",
]
NON_CLAIMS = [
    "bounded same-client posegraph runtime evidence only",
    "no global simultaneity or WAN/federation",
    "no distributed durable pose save/load",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-runtime --test posegraph_runtime -- --nocapture",
    "python3 -m unittest scripts.tests.test_posegraph_runtime_samples",
    "python3 scripts/posegraph_runtime_samples.py matrix --format json",
    "python3 scripts/posegraph_runtime_samples.py check-all --format json",
]


def _load_matrix() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _row_source_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["representative_source"]


def _row_package_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["package_input"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["expected"]


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = SAMPLE_ROOT / row["root_name"]
        readme_path = root_path / "README.md"
        source_path = _row_source_path(row)
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not readme_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_readme",
                    "detail": f"missing sample readme `{readme_path}`",
                }
            )
        if not source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_source",
                    "detail": f"missing source `{source_path}`",
                }
            )
        if row["current_status"] == "executable":
            package_path = _row_package_path(row)
            expected_path = _row_expected_path(row)
            if not package_path.exists():
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": "missing_package_input",
                        "detail": f"missing package input `{package_path}`",
                    }
                )
            if not expected_path.exists():
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": "missing_expected",
                        "detail": f"missing expected file `{expected_path}`",
                    }
                )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    payload = {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "representative_source": str(_row_source_path(row).relative_to(REPO_ROOT)),
        "runnable": row["current_status"] == "executable",
    }
    if row.get("package_input"):
        payload["package_input"] = str(_row_package_path(row).relative_to(REPO_ROOT))
    if row.get("expected"):
        payload["expected"] = str(_row_expected_path(row).relative_to(REPO_ROOT))
    if row.get("expected_terminal_outcome"):
        payload["expected_terminal_outcome"] = row["expected_terminal_outcome"]
    return payload


def list_samples() -> list[dict[str, Any]]:
    data = _load_matrix()
    return [_materialize_row(row) for row in data["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(data["rows"])
    executable_rows = [row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"]
    planned_rows = [row["sample_id"] for row in data["rows"] if row["current_status"] == "planned_only"]
    accepted_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row.get("expected_terminal_outcome") == "Accepted"
    ]
    violation_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row.get("expected_terminal_outcome") == "ViolationExport"
    ]
    runtime_rejection_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row.get("expected_terminal_outcome") == "RuntimeRejection"
    ]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "planned_count": len(planned_rows),
        "accepted_count": len(accepted_rows),
        "violation_count": len(violation_rows),
        "runtime_rejection_count": len(runtime_rejection_rows),
        "executable_rows": executable_rows,
        "planned_rows": planned_rows,
        "accepted_rows": accepted_rows,
        "violation_rows": violation_rows,
        "runtime_rejection_rows": runtime_rejection_rows,
        "matrix_status": data["current_status"],
        "current_posegraph_reading": data["current_posegraph_reading"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def _run_posegraph_package(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "posegraph_runtime_session",
            "--",
            str(path),
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
            f"posegraph_runtime_session did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _payload_projection(payload: dict[str, Any]) -> dict[str, Any]:
    runtime_state = payload.get("runtime_state") or {}
    save_load_state = payload.get("save_load_state") or {}
    devtools_export = payload.get("devtools_export") or {}
    devtools_sections = devtools_export.get("sections") or {}
    return {
        "accepted": payload.get("accepted"),
        "terminal_outcome": payload.get("terminal_outcome"),
        "package_id": payload.get("package_id"),
        "module_id": payload.get("module_id"),
        "transition_id": payload.get("transition_id"),
        "violation_kind": (payload.get("violation") or {}).get("violation_kind"),
        "rejection_code": (payload.get("rejection") or {}).get("code"),
        "pose_snapshot_frontier": runtime_state.get("pose_snapshot_frontier"),
        "node_entities": [row["entity_ref"] for row in runtime_state.get("nodes") or []],
        "node_pose_versions": {
            row["entity_ref"]: row["pose_version"] for row in runtime_state.get("nodes") or []
        },
        "anchor_bindings": [
            {
                "entity_ref": row["entity_ref"],
                "anchor_ref": row["anchor_ref"],
                "state": row["state"],
                "membership_epoch": row["membership_epoch"],
                "owner_epoch": row["owner_epoch"],
            }
            for row in runtime_state.get("anchor_bindings") or []
        ],
        "anchor_switch_sequences": [
            row["sequence"] for row in runtime_state.get("anchor_switch_log") or []
        ],
        "anchor_switch_required_capabilities": [
            row["required_capability"]
            for row in runtime_state.get("anchor_switch_log") or []
        ],
        "fallback_active_anchors": [
            row["active_anchor"] for row in runtime_state.get("fallback_state") or []
        ],
        "fallback_chain_lengths": [
            len(row["fallback_chain"]) for row in runtime_state.get("fallback_state") or []
        ],
        "reacquire_required": runtime_state.get("reacquire_required") or [],
        "savepoint_ref": save_load_state.get("savepoint_ref"),
        "save_load_load_admissible": save_load_state.get("load_admissible"),
        "save_load_state_roundtrip_equal": save_load_state.get("state_roundtrip_equal"),
        "save_load_saved_pose_snapshot_frontier": save_load_state.get(
            "saved_pose_snapshot_frontier"
        ),
        "save_load_restored_pose_snapshot_frontier": save_load_state.get(
            "restored_pose_snapshot_frontier"
        ),
        "devtools_panel_ids": devtools_export.get("panel_ids") or [],
        "devtools_pose_snapshot_entries": [
            row["snapshot_ref"] for row in devtools_sections.get("pose_snapshot_timeline") or []
        ],
        "devtools_stale_reacquire_entities": [
            row["entity_ref"] for row in devtools_sections.get("stale_reacquire_events") or []
        ],
        "devtools_no_split_frame_status": (
            (devtools_sections.get("no_split_frame_rows") or [{}])[0].get("outcome")
        ),
        "observer_safe_summary": payload.get("observer_safe_summary"),
    }


def _run_row(row: dict[str, Any]) -> dict[str, Any]:
    package_path = _row_package_path(row)
    expected_path = _row_expected_path(row)
    actual_payload = _run_posegraph_package(package_path)
    actual = _payload_projection(actual_payload)
    expected = json.loads(expected_path.read_text(encoding="utf-8"))
    passed = actual == expected
    return {
        "sample_id": row["sample_id"],
        "package_input": str(package_path.relative_to(REPO_ROOT)),
        "expected_path": str(expected_path.relative_to(REPO_ROOT)),
        "accepted": bool(actual.get("accepted")),
        "returncode": actual_payload["returncode"],
        "passed": passed,
        "actual": actual,
        "expected": expected,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise KeyError(sample_id)
    if row["current_status"] != "executable":
        return {
            "sample_id": sample_id,
            "current_status": row["current_status"],
            "terminal_outcome": "planned_only",
            "workflow_ready": False,
        }
    return _run_row(row)


def check_all() -> dict[str, Any]:
    data = _load_matrix()
    validation_errors = validate_rows(data["rows"])
    if validation_errors:
        return {
            "command": "check-all",
            "passed": [],
            "failed": [],
            "planned": [],
            "validation_errors": validation_errors,
            "workflow_ready": False,
        }

    passed = []
    failed = []
    planned = []
    for row in data["rows"]:
        if row["current_status"] != "executable":
            planned.append(row["sample_id"])
            continue
        result = _run_row(row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)

    return {
        "command": "check-all",
        "sample_count": len(data["rows"]),
        "passed": passed,
        "failed": failed,
        "planned": planned,
        "validation_errors": validation_errors,
        "workflow_ready": False,
    }


def closeout() -> dict[str, Any]:
    matrix_payload = matrix()
    return {
        "command": "closeout",
        "family": matrix_payload["family"],
        "sample_root": matrix_payload["sample_root"],
        "planned_sample_ids": [
            row["sample_id"]
            for row in _load_matrix()["rows"]
            if row["current_status"] == "planned_only"
        ],
        "executable_sample_ids": matrix_payload["executable_rows"],
        "current_posegraph_reading": matrix_payload["current_posegraph_reading"],
        "validation_floor": VALIDATION_FLOOR,
        "stop_lines": STOP_LINES,
        "non_claims": NON_CLAIMS,
        "workflow_ready": False,
    }


def format_pretty(payload: dict[str, Any]) -> str:
    command = payload.get("command", "unknown").upper()
    lines = [f"POSEGRAPH RUNTIME {command}"]
    if "sample_count" in payload:
        lines.append(f"sample_count: {payload['sample_count']}")
    if "executable_count" in payload:
        lines.append(f"executable: {payload['executable_count']}")
    if "planned_count" in payload:
        lines.append(f"planned-only: {payload['planned_count']}")
    if "accepted_count" in payload:
        lines.append(f"accepted rows: {payload['accepted_count']}")
    if "violation_count" in payload:
        lines.append(f"violation rows: {payload['violation_count']}")
    if "runtime_rejection_count" in payload:
        lines.append(f"runtime rejection rows: {payload['runtime_rejection_count']}")
    if "passed" in payload:
        lines.append(f"passed: {len(payload['passed'])}")
    if "planned" in payload:
        lines.append(f"planned: {len(payload['planned'])}")
    if payload.get("validation_errors"):
        lines.append("validation_errors:")
        for error in payload["validation_errors"]:
            lines.append(f"- {error['sample_id']}: {error['kind']}")
    return "\n".join(lines)


def normalize_argv(argv: list[str]) -> list[str]:
    if not argv:
        return ["matrix"]
    format_prefix: list[str] = []
    remainder = list(argv)
    if "--format" in remainder:
        format_index = remainder.index("--format")
        if format_index + 1 < len(remainder):
            format_prefix = remainder[format_index : format_index + 2]
            remainder = remainder[:format_index] + remainder[format_index + 2 :]
    if remainder and remainder[0] in KNOWN_COMMANDS:
        return format_prefix + remainder
    for index, token in enumerate(remainder):
        if token in KNOWN_COMMANDS:
            return format_prefix + remainder[index:] + remainder[:index]
    return format_prefix + ["run", *remainder]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Full System V1 PoseGraph runtime helper")
    parser.add_argument("--format", choices=("pretty", "json"), default="pretty")
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list")
    subparsers.add_parser("matrix")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")

    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(normalize_argv(argv or sys.argv[1:]))
    command = args.command
    if command == "list":
        payload = {"command": "list", "rows": list_samples()}
    elif command == "matrix":
        payload = matrix()
    elif command == "run":
        payload = run_sample(args.sample_id)
    elif command == "check-all":
        payload = check_all()
    elif command == "closeout":
        payload = closeout()
    else:
        raise AssertionError(f"unsupported command {command}")

    if args.format == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(payload))

    if command == "run" and payload.get("current_status") == "planned_only":
        return 2
    if payload.get("failed"):
        return 2
    if command == "run" and not payload.get("passed", True):
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
