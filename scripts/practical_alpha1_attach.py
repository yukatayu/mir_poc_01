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
        "sample_id": "HP-A1-01",
        "summary": "accept practical debug layer attach through a manifest-driven hotplug plan",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-01-debug-layer-attach.expected.json",
    },
    {
        "sample_id": "HP-A1-02",
        "summary": "reject non-admin debug layer attach before activation cut",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-02-non-admin-debug-rejected",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-02-non-admin-debug-rejected.expected.json",
    },
    {
        "sample_id": "HP-A1-03",
        "summary": "accept auth layer only through explicit contract-update path",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-03-auth-layer-contract-update",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-03-auth-layer-contract-update.expected.json",
    },
    {
        "sample_id": "HP-A1-04",
        "summary": "accept declared-failure rate-limit layer with explicit preview reject",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-04-ratelimit-declared-failure",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-04-ratelimit-declared-failure.expected.json",
    },
    {
        "sample_id": "HP-A1-05",
        "summary": "reject incompatible patch before activation cut",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-05-incompatible-patch-rejected",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-05-incompatible-patch-rejected.expected.json",
    },
    {
        "sample_id": "HP-A1-04B1",
        "summary": "reject attach before activation when the offered membership frontier is stale",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-04b1-stale-membership-attach-rejected",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-04b1-stale-membership-attach-rejected.expected.json",
    },
    {
        "sample_id": "HP-A1-04B2",
        "summary": "reject attach before activation when required witness refs are missing",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-04b2-missing-witness-attach-rejected",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-04b2-missing-witness-attach-rejected.expected.json",
    },
    {
        "sample_id": "HP-A1-06",
        "summary": "accept a narrow non-final object package attach preview without claiming final avatar/package completion",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-06-object-package-attach",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-06-object-package-attach.expected.json",
    },
    {
        "sample_id": "HP-A1-07",
        "summary": "defer detach at the current practical floor through an explicit minimal contract without claiming rollback or migration",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract",
        "expected_report": "samples/practical-alpha1/expected/hp-a1-07-detach-minimal-contract.expected.json",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-1 attach command as final avatar/runtime/package completion",
    "do not treat the practical alpha-1 attach command as Docker transport, save/load, or final public hotplug ABI completion",
    "do not treat the practical alpha-1 detach contract as rollback, durable migration, or distributed activation ordering completion",
    "do not promote samples/practical-alpha1 to an active runnable root in the hotplug package",
]

LIMITATIONS = [
    "alpha-local non-final practical hotplug floor only",
    "limited HP-A1 practical sample families only",
    "no detach runtime lifecycle, Docker/local TCP, save/load, or final public ABI",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 hotplug sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-hotplug",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_report(row: dict[str, str]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def _build_hotplug_report(package_path: str | Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha1_attach",
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
            f"hotplug command did not return JSON for {package_path}: {completed.stdout}"
        ) from error


def run_path(package_path: str | Path) -> dict[str, Any]:
    return _build_hotplug_report(package_path)


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = run_path(REPO_ROOT / row["package_dir"])
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected hotplug report drift\n"
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
    hotplug_plan_boundary_present = not failed and bool(reports) and all(
        report.get("hotplug_plan_scope") == "practical-alpha1-hotplug-plan-floor"
        for report in reports
    )
    reason_families = {
        report.get("sample_id"): report.get("reason_family") for report in reports
    }
    freshness_negative_complete = (
        reason_families.get("HP-A1-04B1") == "membership_freshness"
        and reason_families.get("HP-A1-04B2") == "witness"
    )
    object_attach_seam_present = any(
        report.get("sample_id") == "HP-A1-06"
        and report.get("object_attach_preview") is not None
        for report in reports
    )
    detach_minimal_contract_complete = any(
        report.get("sample_id") == "HP-A1-07"
        and report.get("terminal_outcome") == "deferred_detach_minimal_contract"
        and report.get("reason_family") == "detach_contract"
        and report.get("hotplug_runtime_report", {})
        .get("request", {})
        .get("operation_kind")
        == "detach"
        for report in reports
    )
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "package_hotplug_first_floor_complete": not failed,
        "hotplug_plan_boundary_present": hotplug_plan_boundary_present,
        "object_attach_seam_present": object_attach_seam_present,
        "object_attach_claimed": False,
        "freshness_negative_complete": freshness_negative_complete,
        "detach_minimal_contract_complete": detach_minimal_contract_complete,
        "stage_pa1_4_complete": not failed
        and hotplug_plan_boundary_present
        and object_attach_seam_present
        and freshness_negative_complete
        and detach_minimal_contract_complete,
        "run_docker_claimed": False,
        "save_load_claimed": False,
    }


def closeout() -> dict[str, Any]:
    check_all_summary = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture",
            "cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture",
            "cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture",
            "cargo test -p mir-runtime --test alpha_layer_insertion_runtime",
            "python3 scripts/practical_alpha1_attach.py check samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach --format json",
            "python3 scripts/practical_alpha1_attach.py check samples/practical-alpha1/packages/hp-a1-04b1-stale-membership-attach-rejected --format json",
            "python3 scripts/practical_alpha1_attach.py check samples/practical-alpha1/packages/hp-a1-04b2-missing-witness-attach-rejected --format json",
            "python3 scripts/practical_alpha1_attach.py check samples/practical-alpha1/packages/hp-a1-06-object-package-attach --format json",
            "python3 scripts/practical_alpha1_attach.py check samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract --format json",
            "python3 scripts/practical_alpha1_attach.py check-all --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "package_hotplug_first_floor_complete": check_all_summary[
            "package_hotplug_first_floor_complete"
        ],
        "hotplug_plan_boundary_present": check_all_summary[
            "hotplug_plan_boundary_present"
        ],
        "object_attach_seam_present": check_all_summary["object_attach_seam_present"],
        "object_attach_claimed": check_all_summary["object_attach_claimed"],
        "freshness_negative_complete": check_all_summary[
            "freshness_negative_complete"
        ],
        "detach_minimal_contract_complete": check_all_summary[
            "detach_minimal_contract_complete"
        ],
        "stage_pa1_4_complete": check_all_summary["stage_pa1_4_complete"],
        "run_docker_claimed": check_all_summary["run_docker_claimed"],
        "save_load_claimed": check_all_summary["save_load_claimed"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 attach command surface. This consumes practical package "
            "input through a distinct hotplug-plan boundary and remains non-final."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")

    check_parser = subparsers.add_parser("check")
    check_parser.add_argument("package_path")

    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")

    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
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
    known = {"list", "run", "check", "check-all", "closeout"}
    payload = list(remainder)
    if payload and payload[0] not in known and not payload[0].startswith("-"):
        return [*hoisted_root_options, "check", *payload]
    return values


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check":
        payload = run_path(args.package_path)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
