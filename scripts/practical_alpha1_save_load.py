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
sys.path.insert(0, str(SCRIPT_DIR))

import practical_alpha1_check  # noqa: E402


IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "SL-A1-01",
        "mode": "runtime_exact_report",
        "summary": "save and restore one exact practical local-runtime frontier, then resume one dispatch",
        "package_dir": "samples/practical-alpha1/packages/sl-a1-01-local-save-load-resume",
        "expected_report": "samples/practical-alpha1/expected/sl-a1-01-local-save-load-resume.expected.json",
    },
    {
        "sample_id": "SL-A1-02",
        "mode": "runtime_exact_report",
        "summary": "reject resumed dispatch after a later live membership frontier advance",
        "package_dir": "samples/practical-alpha1/packages/sl-a1-02-local-load-stale-membership-rejected",
        "expected_report": "samples/practical-alpha1/expected/sl-a1-02-local-load-stale-membership-rejected.expected.json",
    },
    {
        "sample_id": "SL-A1-03",
        "mode": "checker_preflight_reject",
        "summary": "reject invalid distributed cut at the save-load command preflight via exact checker guard reuse",
        "package_dir": "samples/practical-alpha1/packages/sl-a1-03-invalid-distributed-cut-preflight",
        "expected_report": "samples/practical-alpha1/expected/sl-a1-03-invalid-distributed-cut-preflight.expected.json",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-1 save-load command as distributed or durable save-load completion",
    "do not treat the practical alpha-1 save-load command as queue/channel/transport persistence completion",
    "do not treat CHK-CUT-01 guard reuse as full consistent-cut or Z-cycle completion",
    "do not promote samples/practical-alpha1 to an active runnable root in the save-load package",
]

LIMITATIONS = [
    "alpha-local non-final practical save-load floor only",
    "limited SL practical sample families only",
    "depends on one exact practical local-runtime frontier before serialization",
    "no distributed durable save/load, stale witness/lease completion, or final public ABI",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 save-load sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-save-load",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_report(row: dict[str, str]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def _build_runtime_save_load_report(package_path: str | Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha1_save_load",
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
            f"save-load command did not return JSON for {package_path}: {completed.stdout}"
        ) from error


def _checker_preflight_report(
    row: dict[str, str],
    package_path: str | Path,
) -> dict[str, Any]:
    checker_report = practical_alpha1_check.check_path(package_path)
    rejected_rows = checker_report.get("rejected_rows", [])
    diagnostics = checker_report.get("diagnostics", [])
    if checker_report.get("verdict") != "rejected" or not rejected_rows or not diagnostics:
        raise RuntimeError(
            f"{row['sample_id']}: exact checker guard did not return a rejected report"
        )

    rejected = rejected_rows[0]
    diagnostic = diagnostics[0]
    if rejected.get("kind") != "orphan_receive":
        raise RuntimeError(
            f"{row['sample_id']}: expected orphan_receive cut reject, found {rejected.get('kind')}"
        )

    return {
        "surface_kind": "practical_alpha1_nonfinal_save_load_preflight_report",
        "scope_kind": "alpha_local",
        "save_load_scope": "practical-alpha1-save-load-floor",
        "preflight_scope": "practical-alpha1-save-load-preflight-floor",
        "sample_id": row["sample_id"],
        "package_id": checker_report["package_id"],
        "package_kind": "world",
        "preflight_mode": "exact_checker_guard_reuse_only",
        "terminal_outcome": "rejected_invalid_distributed_cut_preflight",
        "checker_guard_refs": ["CHK-CUT-01"],
        "source_checker_report": {
            "sample_id": checker_report["sample_id"],
            "checker_scope": checker_report["checker_scope"],
            "surface_kind": checker_report["surface_kind"],
            "rejected_kind": rejected["kind"],
            "diagnostic_message": diagnostic["message"],
        },
        "notes": [
            "save/load command reuses the exact checker guard as a distinct preflight reject row",
            "no saved local frontier is built when the distributed cut is already invalid",
            "no runtime execution or distributed checkpoint attempt is performed in this row",
        ],
        "retained_later_refs": [
            "distributed_durable_save_load",
            "stale_witness_nonresurrection",
            "stale_lease_nonresurrection",
            "docker_transport_save_load",
            "hotplug_lifecycle_persistence",
            "final_public_save_load_api",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS)
        + [
            "checker-backed save-load preflight reject row only",
        ],
        "save_load_claimed": True,
        "run_local_claimed": False,
        "base_runtime_frontier_required": False,
        "saved_local_frontier_emitted": False,
        "distributed_save_load_claimed": False,
        "stale_witness_claimed": False,
        "stale_lease_claimed": False,
    }


def _report_for_row(row: dict[str, str], package_path: str | Path) -> dict[str, Any]:
    if row["mode"] == "checker_preflight_reject":
        return _checker_preflight_report(row, package_path)
    return _build_runtime_save_load_report(package_path)


def _row_for_package_path(package_path: str | Path) -> dict[str, str] | None:
    target = Path(package_path).resolve()
    for row in IMPLEMENTED_ROWS:
        if (REPO_ROOT / row["package_dir"]).resolve() == target:
            return row
    return None


def run_path(package_path: str | Path) -> dict[str, Any]:
    row = _row_for_package_path(package_path)
    if row is not None:
        return _report_for_row(row, package_path)
    return _build_runtime_save_load_report(package_path)


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = _report_for_row(row, REPO_ROOT / row["package_dir"])
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected save-load report drift\n"
            f"expected={json.dumps(expected, ensure_ascii=False, sort_keys=True)}\n"
            f"actual={json.dumps(actual, ensure_ascii=False, sort_keys=True)}"
        )
    return actual


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    reports: list[dict[str, Any]] = []
    guard_error: str | None = None
    try:
        practical_alpha1_check.run_sample("CHK-CUT-01")
        invalid_distributed_cut_guard_present = True
    except Exception as error:  # pragma: no cover
        invalid_distributed_cut_guard_present = False
        guard_error = str(error)
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            report = run_sample(sample_id)
            passed.append(sample_id)
            reports.append(report)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    save_load_plan_boundary_present = (
        not failed
        and bool(reports)
        and all(
            (
                report.get("surface_kind")
                == "practical_alpha1_nonfinal_save_load_report"
                and report.get("save_load_plan_scope")
                == "practical-alpha1-save-load-plan-floor"
                and report.get("runtime_plan_scope")
                == "practical-alpha1-runtime-plan-floor"
                and report.get("save_load_claimed") is True
                and report.get("run_local_claimed") is False
            )
            or (
                report.get("surface_kind")
                == "practical_alpha1_nonfinal_save_load_preflight_report"
                and report.get("preflight_scope")
                == "practical-alpha1-save-load-preflight-floor"
                and report.get("source_checker_report", {}).get("checker_scope")
                == "practical-alpha1-checker-floor"
                and report.get("save_load_claimed") is True
                and report.get("run_local_claimed") is False
            )
            for report in reports
        )
    )
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "local_save_load_first_floor_complete": not failed
        and invalid_distributed_cut_guard_present,
        "stage_pa1_7_complete": not failed and invalid_distributed_cut_guard_present,
        "save_load_plan_boundary_present": save_load_plan_boundary_present,
        "invalid_distributed_cut_guard_present": invalid_distributed_cut_guard_present,
        "invalid_distributed_cut_row_actualized": "SL-A1-03" in passed,
        "checker_guard_refs": ["CHK-CUT-01"],
        "guard_error": guard_error,
        "distributed_save_load_claimed": False,
        "stale_witness_claimed": False,
        "stale_lease_claimed": False,
    }


def closeout() -> dict[str, Any]:
    check_all_summary = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "checker_guard_refs": ["CHK-CUT-01"],
        "validation_floor": [
            "python3 scripts/practical_alpha1_check.py run CHK-CUT-01 --format json",
            "python3 scripts/practical_alpha1_save_load.py run SL-A1-01 --format json",
            "python3 scripts/practical_alpha1_save_load.py run SL-A1-02 --format json",
            "python3 scripts/practical_alpha1_save_load.py run SL-A1-03 --format json",
            "python3 scripts/practical_alpha1_save_load.py check-all --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_validate_docs",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "local_save_load_first_floor_complete": check_all_summary[
            "local_save_load_first_floor_complete"
        ],
        "stage_pa1_7_complete": check_all_summary["stage_pa1_7_complete"],
        "save_load_plan_boundary_present": check_all_summary[
            "save_load_plan_boundary_present"
        ],
        "invalid_distributed_cut_guard_present": check_all_summary[
            "invalid_distributed_cut_guard_present"
        ],
        "invalid_distributed_cut_row_actualized": check_all_summary[
            "invalid_distributed_cut_row_actualized"
        ],
        "distributed_save_load_claimed": check_all_summary[
            "distributed_save_load_claimed"
        ],
        "stale_witness_claimed": check_all_summary["stale_witness_claimed"],
        "stale_lease_claimed": check_all_summary["stale_lease_claimed"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 local save/load command surface. This restores one exact "
            "checked local frontier through a distinct save-load carrier and remains non-final."
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
