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

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "AV-A1-01",
        "summary": "render a non-final placeholder avatar preview over the practical hotplug object package route",
        "package_dir": "samples/practical-alpha1/packages/av-a1-01-placeholder-avatar-runtime",
        "expected_report": "samples/practical-alpha1/expected/av-a1-01-placeholder-avatar-runtime.expected.json",
    },
    {
        "sample_id": "AV-A1-02",
        "summary": "render a non-final custom Mir avatar preview without claiming native execution",
        "package_dir": "samples/practical-alpha1/packages/av-a1-02-custom-mir-avatar-runtime",
        "expected_report": "samples/practical-alpha1/expected/av-a1-02-custom-mir-avatar-runtime.expected.json",
    },
    {
        "sample_id": "AV-A1-03",
        "summary": "render a visible monotone placeholder fallback after unsupported custom runtime rejection",
        "package_dir": "samples/practical-alpha1/packages/av-a1-03-unsupported-runtime-fallback",
        "expected_report": "samples/practical-alpha1/expected/av-a1-03-unsupported-runtime-fallback.expected.json",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-1 avatar preview as native execution or final avatar ABI completion",
    "do not treat the practical alpha-1 avatar preview as successful unsupported-runtime execution",
    "do not treat the practical alpha-1 avatar preview as an active runnable-root promotion",
]

LIMITATIONS = [
    "alpha-local non-final practical avatar preview floor only",
    "limited AV-A1 practical sample families only",
    "no native execution, final avatar ABI, or monolithic product runtime completion",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 avatar sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-avatar",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_report(row: dict[str, str]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def _build_avatar_report(package_path: str | Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha1_avatar",
            "--",
            str(package_path),
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:  # pragma: no cover
        raise RuntimeError(
            f"avatar command did not return JSON for {package_path}: {completed.stdout}"
        ) from error


def run_path(package_path: str | Path) -> dict[str, Any]:
    return _build_avatar_report(package_path)


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = run_path(REPO_ROOT / row["package_dir"])
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected avatar preview report drift\n"
            f"expected={json.dumps(expected, ensure_ascii=False, sort_keys=True)}\n"
            f"actual={json.dumps(actual, ensure_ascii=False, sort_keys=True)}"
        )
    return actual


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    reports: list[dict[str, Any]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            report = run_sample(sample_id)
            passed.append(sample_id)
            reports.append(report)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})

    placeholder_preview_present = any(
        report.get("sample_id") == "AV-A1-01"
        and report.get("preview_kind") == "placeholder_avatar_runtime_preview"
        and report.get("selected_representation") == "static_capsule_placeholder"
        for report in reports
    )
    custom_runtime_preview_present = any(
        report.get("sample_id") == "AV-A1-02"
        and report.get("preview_kind") == "custom_mir_avatar_runtime_preview"
        and report.get("selected_representation") == "mir_humanoid_runtime_preview"
        for report in reports
    )
    unsupported_runtime_fallback_present = any(
        report.get("sample_id") == "AV-A1-03"
        and report.get("preview_kind")
        == "unsupported_runtime_visible_fallback_preview"
        and report.get("fallback_applied") is True
        and report.get("fallback_reason") == "missing_host_capability:HostMirAvatarVM"
        for report in reports
    )
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "avatar_preview_first_floor_complete": not failed,
        "placeholder_preview_present": placeholder_preview_present,
        "custom_runtime_preview_present": custom_runtime_preview_present,
        "unsupported_runtime_fallback_present": unsupported_runtime_fallback_present,
        "native_execution_claimed": False,
        "final_avatar_abi_frozen": False,
    }


def closeout() -> dict[str, Any]:
    summary = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_avatar -- --nocapture",
            "python3 scripts/practical_alpha1_avatar.py check-all --format json",
        ],
        "avatar_preview_first_floor_complete": summary[
            "avatar_preview_first_floor_complete"
        ],
        "placeholder_preview_present": summary["placeholder_preview_present"],
        "custom_runtime_preview_present": summary["custom_runtime_preview_present"],
        "unsupported_runtime_fallback_present": summary[
            "unsupported_runtime_fallback_present"
        ],
        "native_execution_claimed": False,
        "final_avatar_abi_frozen": False,
        "stop_lines": STOP_LINES,
        "limitations": LIMITATIONS,
    }


def normalize_argv(argv: list[str]) -> list[str]:
    normalized = list(argv)
    if "--format" in normalized:
        index = normalized.index("--format")
        if index + 1 >= len(normalized):
            raise SystemExit("--format requires a value")
        format_pair = normalized[index : index + 2]
        del normalized[index : index + 2]
        normalized = [*format_pair, *normalized]
    command_index = 0
    while command_index < len(normalized) and normalized[command_index].startswith("-"):
        command_index += 2 if normalized[command_index] == "--format" else 1
    if command_index < len(normalized) and normalized[command_index] not in {
        "list",
        "run",
        "check",
        "check-all",
        "closeout",
    }:
        target = normalized[command_index]
        del normalized[command_index]
        normalized.insert(command_index, "check")
        normalized.insert(command_index + 1, target)
        return normalized
    return normalized


def _emit(payload: Any, output_format: str) -> None:
    if output_format == "json":
        json.dump(payload, sys.stdout, ensure_ascii=False, indent=2)
        sys.stdout.write("\n")
        return
    print(payload)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Practical alpha-1 avatar preview floor helper"
    )
    parser.add_argument("--format", choices=("json", "text"), default="text")
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("list")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
    check_parser = subparsers.add_parser("check")
    check_parser.add_argument("package_path")
    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")
    return parser


def main(argv: list[str] | None = None) -> int:
    normalized = normalize_argv(sys.argv[1:] if argv is None else argv)
    parser = build_parser()
    args = parser.parse_args(normalized)

    if args.command == "list":
        _emit(list_samples(), args.format)
        return 0
    if args.command == "run":
        _emit(run_sample(args.sample_id), args.format)
        return 0
    if args.command == "check":
        _emit(run_path(args.package_path), args.format)
        return 0
    if args.command == "check-all":
        _emit(check_all(), args.format)
        return 0
    if args.command == "closeout":
        _emit(closeout(), args.format)
        return 0
    parser.error(f"unknown command {args.command}")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
