#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent

IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "CUT-04",
        "summary": "Local-only save/load bridge restores state and resumes one dispatch.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/cut-save-load/cut-04-local_save_load_valid.expected.json",
    },
    {
        "sample_id": "CUT-17",
        "summary": "A restored local savepoint does not resurrect stale membership into accepted resumed dispatch.",
        "expected_terminal_outcome": "rejected",
        "expected_sidecar": "samples/alpha/cut-save-load/cut-17-load_does_not_resurrect_stale_membership.expected.json",
    },
]

CHECKER_BACKED_ROWS = [
    "CUT-05",
    "CUT-07",
    "CUT-08",
    "CUT-09",
    "CUT-11",
    "CUT-13",
    "CUT-14",
    "CUT-15",
]
PLANNED_ONLY_ROWS = [
    "CUT-01",
    "CUT-02",
    "CUT-03",
    "CUT-06",
    "CUT-10",
    "CUT-12",
    "CUT-16",
]

STOP_LINES = [
    "do not treat local save/load as distributed save/load completion",
    "do not mark durable_cut implemented in Mir-0",
    "do not mark Z-cycle handling complete in this package",
    "do not promote samples/alpha/cut-save-load to an active runnable root",
]

LIMITATIONS = [
    "local-only save/load bridge over the Rust alpha runtime floor",
    "checker-backed invalid/disallowed rows remain synthetic inventory only",
    "no distributed durable save/load, replay service, or consensus",
    "no parser/front-door execution of samples/alpha/cut-save-load/*.mir",
]


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented alpha cut/save/load sample: {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-cut-save-load",
            "source_root": "samples/alpha/cut-save-load",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    sidecar_path = REPO_ROOT / row["expected_sidecar"]
    return json.loads(sidecar_path.read_text())


def _build_runtime_report(sample_id: str) -> dict[str, Any]:
    scenario_by_sample_id = {
        "CUT-04": "save-load-resume",
        "CUT-17": "save-load-stale-membership",
    }
    scenario = scenario_by_sample_id.get(sample_id)
    if scenario is None:
        raise ValueError(f"unsupported runtime sample: {sample_id}")
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_local_runtime",
            "--",
            scenario,
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def _validate_expected_fields(sample_id: str, row: dict[str, Any], report: dict[str, Any]) -> None:
    if report.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: report sample_id mismatch: {report.get('sample_id')!r}"
        )
    if report.get("terminal_outcome") != row["expected_terminal_outcome"]:
        raise RuntimeError(
            f"{sample_id}: expected terminal_outcome {row['expected_terminal_outcome']!r} "
            f"but observed {report.get('terminal_outcome')!r}"
        )
    if not report.get("state_roundtrip_equal"):
        raise RuntimeError(f"{sample_id}: expected state_roundtrip_equal to be true")
    if sample_id == "CUT-17":
        dispatch_records = report.get("resumed_dispatch_records") or []
        if len(dispatch_records) != 1:
            raise RuntimeError(f"{sample_id}: expected exactly one resumed dispatch record")
        if dispatch_records[0].get("dispatch_outcome") != "rejected_stale_membership":
            raise RuntimeError(
                f"{sample_id}: expected resumed dispatch outcome 'rejected_stale_membership'"
            )
        if report.get("visible_history_after_resume") != report.get("restored_visible_history"):
            raise RuntimeError(
                f"{sample_id}: expected visible history to remain unchanged after rejection"
            )


def _validate_sidecar_parity(
    sample_id: str, report: dict[str, Any], expected_sidecar: dict[str, Any]
) -> None:
    if report != expected_sidecar:
        raise RuntimeError(f"{sample_id}: sidecar drift detected")


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    report = _build_runtime_report(sample_id)
    _validate_expected_fields(sample_id, row, report)
    _validate_sidecar_parity(sample_id, report, _load_expected_sidecar(row))
    return report


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            run_sample(sample_id)
            passed.append(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "distributed_save_load_claimed": False,
        "durable_cut_claimed": False,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": "samples/alpha/cut-save-load",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "checker_backed_rows": list(CHECKER_BACKED_ROWS),
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "validation_floor": [
            "cargo test -p mirrorea-core --test runtime_substrate",
            "cargo test -p mir-runtime --test alpha_local_runtime",
            "cargo test -p mir-runtime --test alpha_cut_save_load_runtime",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership",
            "python3 scripts/alpha_cut_save_load_samples.py check-all --format json",
            "python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "distributed_save_load_claimed": False,
        "durable_cut_claimed": False,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and "sample_id" in payload:
        lines = [
            f"{payload['sample_id']} alpha_cut_save_load",
            f"  outcome: {payload.get('terminal_outcome')}",
            f"  saved_owner: {payload.get('saved_owner')}",
            f"  resumed_owner: {payload.get('resumed_owner')}",
            f"  state_roundtrip_equal: {payload.get('state_roundtrip_equal')}",
        ]
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list", help="list implemented runtime-backed rows")
    list_parser.add_argument("--format", choices=("text", "json"), default="text")

    run_parser = subparsers.add_parser("run", help="run one runtime-backed row")
    run_parser.add_argument("sample_id")
    run_parser.add_argument("--format", choices=("text", "json"), default="text")

    check_parser = subparsers.add_parser("check-all", help="run all runtime-backed rows")
    check_parser.add_argument("--format", choices=("text", "json"), default="text")

    closeout_parser = subparsers.add_parser("closeout", help="report implemented/checker/planned inventory")
    closeout_parser.add_argument("--format", choices=("text", "json"), default="text")

    args = parser.parse_args(argv)

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
