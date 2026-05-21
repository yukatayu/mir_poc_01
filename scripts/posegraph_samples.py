#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "posegraph"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
HELPER_EXECUTION_SURFACE = "helper_posegraph_runtime"
STOP_LINES = [
    "no PoseGraph runtime completion yet",
    "no global cross-client simultaneity",
    "do not treat renderer state as semantic owner",
]
NON_CLAIMS = [
    "only bounded no-split-frame helper rows exist today",
    "no direct PoseGraph runtime integration yet",
    "no Unity / Unreal / VRM / VRChat compatibility",
    "no continuous spatial sync or WAN/federation",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_posegraph_samples",
    "python3 scripts/posegraph_samples.py matrix --format json",
    "python3 scripts/posegraph_samples.py check-all --format json",
    "python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json",
    "python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json",
]


def _load_matrix_file() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _row_package_path(row: dict[str, Any]) -> Path | None:
    package_input = row.get("package_input")
    if not package_input:
        return None
    return SAMPLE_ROOT / package_input


def _load_package_json(package_path: Path) -> dict[str, Any]:
    return json.loads(package_path.read_text(encoding="utf-8"))


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise RuntimeError(detail)


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
        package_path = sample_root / row["package_input"] if row.get("package_input") else None
        if row["current_status"] == "executable":
            if package_path is None or not package_path.exists():
                detail = (
                    f"missing package input `{package_path}`"
                    if package_path is not None
                    else "missing executable package input"
                )
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": "missing_package_input",
                        "detail": detail,
                    }
                )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    root_path = SAMPLE_ROOT / row["root_name"]
    source_path = SAMPLE_ROOT / row["representative_source"]
    package_path = _row_package_path(row)
    realized = {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "root_path": str(root_path.relative_to(REPO_ROOT)),
        "representative_source": str(source_path.relative_to(REPO_ROOT)),
        "runnable": row["current_status"] == "executable",
    }
    if row.get("execution_surface"):
        realized["execution_surface"] = row["execution_surface"]
    if row.get("expected_outcome"):
        realized["expected_outcome"] = dict(row["expected_outcome"])
    if package_path is not None:
        realized["package_input"] = str(package_path.relative_to(REPO_ROOT))
        if package_path.exists():
            package_data = _load_package_json(package_path)
            if package_data.get("package_id"):
                realized["package_id"] = package_data["package_id"]
            if package_data.get("module_id"):
                realized["module_id"] = package_data["module_id"]
            if package_data.get("transition_id"):
                realized["transition_id"] = package_data["transition_id"]
    return realized


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
    accepted_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row["current_status"] == "executable"
        and (row.get("expected_outcome") or {}).get("terminal_outcome") == "accepted"
    ]
    violation_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row["current_status"] == "executable"
        and (row.get("expected_outcome") or {}).get("terminal_outcome")
        == "violation_export"
    ]
    planned_count = sum(1 for row in rows if row["current_status"] == "planned_only")
    executable_count = sum(1 for row in rows if row["current_status"] == "executable")
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "current_posegraph_reading": data["current_posegraph_reading"],
        "current_no_split_frame_reading": data["current_posegraph_reading"],
        "sample_count": len(rows),
        "planned_count": planned_count,
        "executable_count": executable_count,
        "accepted_count": len(accepted_rows),
        "violation_count": len(violation_rows),
        "planned_only_rows": planned_only_rows,
        "executable_rows": executable_rows,
        "accepted_rows": accepted_rows,
        "violation_rows": violation_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def _load_posegraph_helper_contract(row: dict[str, Any]) -> dict[str, Any]:
    package_path = _row_package_path(row)
    if package_path is None:
        raise RuntimeError(f"sample `{row['sample_id']}` does not define package_input")
    payload = _load_package_json(package_path)
    runtime_input = payload.get("runtime_input") or {}
    posegraph = runtime_input.get("posegraph") or {}
    target_pose = posegraph.get("target_pose") or {}
    anchored_pose = posegraph.get("anchored_pose") or {}

    target_snapshot_ref = target_pose.get("pose_snapshot_ref")
    anchored_snapshot_ref = anchored_pose.get("pose_snapshot_ref")
    target_pose_version = target_pose.get("pose_version")
    anchored_pose_version = anchored_pose.get("pose_version")

    _require(
        isinstance(target_snapshot_ref, str),
        f"sample `{row['sample_id']}` must declare target pose snapshot ref",
    )
    _require(
        isinstance(anchored_snapshot_ref, str),
        f"sample `{row['sample_id']}` must declare anchored pose snapshot ref",
    )
    _require(
        isinstance(target_pose_version, int),
        f"sample `{row['sample_id']}` must declare target pose version",
    )
    _require(
        isinstance(anchored_pose_version, int),
        f"sample `{row['sample_id']}` must declare anchored pose version",
    )

    return {
        "package_id": payload.get("package_id"),
        "package_input": str(package_path.relative_to(REPO_ROOT)),
        "module_id": payload.get("module_id"),
        "transition_id": payload.get("transition_id"),
        "target_pose_version": target_pose_version,
        "anchored_pose_version": anchored_pose_version,
        "target_pose_snapshot_ref": target_snapshot_ref,
        "anchored_pose_snapshot_ref": anchored_snapshot_ref,
    }


def _evaluate_posegraph_contract(contract: dict[str, Any]) -> dict[str, Any]:
    target_pose_version = contract["target_pose_version"]
    anchored_pose_version = contract["anchored_pose_version"]
    target_snapshot_ref = contract["target_pose_snapshot_ref"]
    anchored_snapshot_ref = contract["anchored_pose_snapshot_ref"]
    if (
        target_pose_version == anchored_pose_version
        and target_snapshot_ref == anchored_snapshot_ref
    ):
        return {
            "terminal_outcome": "accepted",
            "target_pose_version": target_pose_version,
            "anchored_pose_version": anchored_pose_version,
            "pose_snapshot_ref": target_snapshot_ref,
            "pose_summary": f"{target_snapshot_ref}@{target_pose_version}",
        }

    violation_details: list[str] = []
    if target_snapshot_ref != anchored_snapshot_ref:
        violation_details.append(
            "snapshot mismatch: "
            f"target={target_snapshot_ref}, anchored={anchored_snapshot_ref}"
        )
    if target_pose_version != anchored_pose_version:
        violation_details.append(
            "pose version mismatch: "
            f"target={target_pose_version}, anchored={anchored_pose_version}"
        )
    return {
        "terminal_outcome": "violation_export",
        "violation_kind": "no_split_frame",
        "actual_violation_detail": "; ".join(violation_details),
        "target_pose_version": target_pose_version,
        "anchored_pose_version": anchored_pose_version,
        "target_pose_snapshot_ref": target_snapshot_ref,
        "anchored_pose_snapshot_ref": anchored_snapshot_ref,
    }


def _expected_outcome_matches(
    expected_outcome: dict[str, Any], actual_outcome: dict[str, Any]
) -> bool:
    if expected_outcome.get("terminal_outcome") != actual_outcome.get("terminal_outcome"):
        return False
    if actual_outcome.get("terminal_outcome") == "accepted":
        return expected_outcome.get("pose_summary") == actual_outcome.get("pose_summary")
    if actual_outcome.get("terminal_outcome") == "violation_export":
        if expected_outcome.get("violation_kind") != actual_outcome.get("violation_kind"):
            return False
        expected_detail = expected_outcome.get("violation_contains")
        actual_detail = actual_outcome.get("actual_violation_detail") or ""
        return isinstance(expected_detail, str) and expected_detail.lower() in actual_detail.lower()
    return False


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix_file()
    row = next(
        (
            row
            for row in data["rows"]
            if row["sample_id"] == sample_id or row["root_name"] == sample_id
        ),
        None,
    )
    if row is None:
        raise ValueError(f"unknown posegraph sample `{sample_id}`")
    realized = _materialize_row(row)
    if row["current_status"] != "executable":
        return {
            "command": "run",
            "family": data["family"],
            "sample_id": sample_id,
            "current_status": row["current_status"],
            "terminal_outcome": "planned_only",
            "rejection_reason": (
                f"{row['stage']} is not implemented yet; this root is scaffold-only in P-POSE-01"
            ),
            "current_posegraph_reading": data["current_posegraph_reading"],
            "current_no_split_frame_reading": data["current_posegraph_reading"],
            "workflow_ready": False,
            "stop_lines": list(STOP_LINES),
            "row": realized,
        }

    if row.get("execution_surface") != HELPER_EXECUTION_SURFACE:
        raise RuntimeError(
            f"unsupported execution surface `{row.get('execution_surface')}` for `{sample_id}`"
        )
    expected_outcome = dict(row["expected_outcome"])
    contract = _load_posegraph_helper_contract(row)
    actual_outcome = _evaluate_posegraph_contract(contract)
    outcome_matches_expected = _expected_outcome_matches(
        expected_outcome, actual_outcome
    )
    return {
        "command": "run",
        "family": data["family"],
        "sample_id": sample_id,
        "current_status": row["current_status"],
        "execution_surface": HELPER_EXECUTION_SURFACE,
        "expected_outcome": expected_outcome,
        "outcome_matches_expected": outcome_matches_expected,
        "package_id": contract["package_id"],
        "package_input": contract["package_input"],
        "module_id": contract["module_id"],
        "transition_id": contract["transition_id"],
        **actual_outcome,
        "row": realized,
    }


def check_all() -> dict[str, Any]:
    status = matrix()
    failed = [error["sample_id"] for error in status["validation_errors"]]
    planned = list(status["planned_only_rows"])
    accepted: list[str] = []
    violations: list[str] = []
    runtime_failures: list[dict[str, str]] = []

    for row in status["rows"]:
        if row["current_status"] != "executable":
            continue
        try:
            result = run_sample(row["sample_id"])
            if not result["outcome_matches_expected"]:
                failed.append(row["sample_id"])
                runtime_failures.append(
                    {
                        "sample_id": row["sample_id"],
                        "detail": "actual outcome did not match expected outcome",
                    }
                )
                continue
            if result["terminal_outcome"] == "accepted":
                accepted.append(row["sample_id"])
            elif result["terminal_outcome"] == "violation_export":
                violations.append(row["sample_id"])
            else:
                failed.append(row["sample_id"])
                runtime_failures.append(
                    {
                        "sample_id": row["sample_id"],
                        "detail": f"unexpected executable outcome `{result['terminal_outcome']}`",
                    }
                )
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
        "planned": planned,
        "accepted": accepted,
        "violations": violations,
        "passed": accepted + violations,
        "failed": failed,
        "matrix_status": status["matrix_status"],
        "current_posegraph_reading": status["current_posegraph_reading"],
        "current_no_split_frame_reading": status["current_no_split_frame_reading"],
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
        "planned_only_sample_ids": list(status["planned_only_rows"]),
        "executable_sample_ids": list(status["executable_rows"]),
        "current_posegraph_reading": status["current_posegraph_reading"],
        "current_no_split_frame_reading": status["current_no_split_frame_reading"],
        "workflow_ready": False,
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
        "validation_errors": status["validation_errors"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["POSEGRAPH SAMPLES"]
        for row in payload:
            line = (
                f"- {row['sample_id']} [{row['current_status']}]"
                f" -> {row['representative_source']}"
            )
            expected_outcome = row.get("expected_outcome")
            if expected_outcome is not None:
                line += f" ({expected_outcome['terminal_outcome']})"
            lines.append(line)
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
                f"accepted rows: {payload['accepted_count']}",
                f"violation rows: {payload['violation_count']}",
            ]
        )
    if command == "run":
        lines = [
            "RUN SUMMARY",
            f"sample: {payload['sample_id']}",
            f"status: {payload['current_status']}",
            f"outcome: {payload['terminal_outcome']}",
        ]
        if "rejection_reason" in payload:
            lines.append(f"reason: {payload['rejection_reason']}")
        if "pose_snapshot_ref" in payload:
            lines.append(f"pose snapshot: {payload['pose_snapshot_ref']}")
        if "actual_violation_detail" in payload:
            lines.append(f"violation: {payload['actual_violation_detail']}")
        return "\n".join(lines)
    if command == "check-all":
        return "\n".join(
            [
                "POSEGRAPH CHECK-ALL SUMMARY",
                f"sample count: {payload['sample_count']}",
                f"planned-only: {len(payload['planned'])}",
                f"accepted rows: {len(payload['accepted'])}",
                f"violation rows: {len(payload['violations'])}",
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
