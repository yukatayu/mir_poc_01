#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

import alpha_cut_save_load_samples as cut_save_load_samples


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent

IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "LR-01",
        "summary": "Local queue dispatch accepts one Sugoroku roll path and exports an event DAG.",
        "expected_sidecar": "samples/alpha/local-runtime/lr-01-local_sugoroku_roll_publish_handoff.expected.json",
        "scenario": "local-sugoroku",
    },
    {
        "sample_id": "LR-02",
        "summary": "Stale membership is rejected before local runtime state mutation.",
        "expected_sidecar": "samples/alpha/local-runtime/lr-02-stale_membership_rejected.expected.json",
        "scenario": "stale-membership",
    },
]

STAGE_B_SUPPORT_ROWS = ["CUT-04", "CUT-17"]

STOP_LINES = [
    "do not treat the Stage B closeout as distributed save/load completion",
    "do not treat samples/alpha/local-runtime as an active runnable root",
    "do not treat the helper script as parser/runtime front-door execution of .mir sources",
    "do not treat Stage B closeout as Stage C/D/E/F completion",
]

LIMITATIONS = [
    "non-public in-memory Rust local-runtime floor only",
    "local-only save/load subset is reused as Stage B support evidence only",
    "no parser/runtime bridge for samples/alpha/*.mir",
    "no distributed save/load, WAN transport, hot-plug lifecycle completion, or final public ABI",
]


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented alpha local-runtime sample: {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-local-runtime",
            "source_root": "samples/alpha/local-runtime",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    sidecar_path = REPO_ROOT / row["expected_sidecar"]
    return json.loads(sidecar_path.read_text())


def _build_runtime_report(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
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
            row["scenario"],
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def _validate_expected_fields(sample_id: str, report: dict[str, Any], expected: dict[str, Any]) -> None:
    if report.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: report sample_id mismatch: {report.get('sample_id')!r}"
        )

    if report.get("runtime_scope") != "alpha_local_runtime_stage_b_narrow":
        raise RuntimeError(
            f"{sample_id}: unexpected runtime_scope {report.get('runtime_scope')!r}"
        )

    expected_runtime = expected.get("expected_runtime", {})
    if report.get("terminal_outcome") != expected_runtime.get("terminal_outcome"):
        raise RuntimeError(
            f"{sample_id}: expected terminal_outcome "
            f"{expected_runtime.get('terminal_outcome')!r} but observed "
            f"{report.get('terminal_outcome')!r}"
        )

    kept_later = set(report.get("retained_later_refs") or [])
    for kept in [
        "layer_insertion_runtime",
        "network_docker_runtime",
        "runtime_package_avatar_admission",
        "distributed_save_load",
        "final_public_abi",
    ]:
        if kept not in kept_later:
            raise RuntimeError(f"{sample_id}: missing kept-later ref {kept!r}")

    dispatch_records = report.get("dispatch_records") or []
    if len(dispatch_records) != 1:
        raise RuntimeError(f"{sample_id}: expected exactly one dispatch record")
    dispatch_record = dispatch_records[0]

    if sample_id == "LR-01":
        if report.get("current_owner") != expected_runtime.get("current_owner"):
            raise RuntimeError(
                f"{sample_id}: expected current_owner "
                f"{expected_runtime.get('current_owner')!r} but observed "
                f"{report.get('current_owner')!r}"
            )
        if dispatch_record.get("dispatch_outcome") != "accepted":
            raise RuntimeError(
                f"{sample_id}: expected dispatch_outcome 'accepted' but observed "
                f"{dispatch_record.get('dispatch_outcome')!r}"
            )
        relations = {edge.get("relation") for edge in report.get("event_dag", {}).get("edges", [])}
        required_relations = set(expected_runtime.get("required_relations") or [])
        missing_relations = sorted(required_relations - relations)
        if missing_relations:
            raise RuntimeError(
                f"{sample_id}: missing event DAG relations {missing_relations!r}"
            )
        if not report.get("visible_history"):
            raise RuntimeError(f"{sample_id}: expected visible_history to be non-empty")
    elif sample_id == "LR-02":
        if report.get("reason_family") != expected_runtime.get("reason_family"):
            raise RuntimeError(
                f"{sample_id}: expected reason_family "
                f"{expected_runtime.get('reason_family')!r} but observed "
                f"{report.get('reason_family')!r}"
            )
        if dispatch_record.get("dispatch_outcome") != "rejected_stale_membership":
            raise RuntimeError(
                f"{sample_id}: expected dispatch_outcome 'rejected_stale_membership' but observed "
                f"{dispatch_record.get('dispatch_outcome')!r}"
            )
        reason_refs = set(dispatch_record.get("reason_refs") or [])
        for required_reason in expected_runtime.get("required_reason_refs") or []:
            if required_reason not in reason_refs:
                raise RuntimeError(
                    f"{sample_id}: missing required reason ref {required_reason!r}"
                )
        if report.get("visible_history"):
            raise RuntimeError(
                f"{sample_id}: expected visible_history to remain empty after rejection"
            )
    else:  # pragma: no cover - guarded by _implemented_row
        raise RuntimeError(f"unsupported local-runtime row {sample_id}")


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_sidecar(row)
    report = _build_runtime_report(sample_id)
    _validate_expected_fields(sample_id, report, expected)
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
        "active_root_promoted": False,
        "parser_runtime_bridge_claimed": False,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": "samples/alpha/local-runtime",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "stage_b_support_rows": list(STAGE_B_SUPPORT_ROWS),
        "validation_floor": [
            "cargo test -p mir-runtime --test alpha_local_runtime",
            "cargo test -p mir-runtime --test alpha_cut_save_load_runtime",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-stale-membership",
            "python3 scripts/alpha_local_runtime_samples.py check-all --format json",
            "python3 scripts/alpha_cut_save_load_samples.py check-all --format json",
            "python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json",
            "python3 -m unittest scripts.tests.test_alpha_local_runtime_samples scripts.tests.test_alpha_cut_save_load_samples",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "stage_b_closeout_scope": "local runtime + local-only save/load subset only",
        "active_root_promoted": False,
        "parser_runtime_bridge_claimed": False,
    }


def stage_b_closeout() -> dict[str, Any]:
    local_payload = check_all()
    cut_payload = cut_save_load_samples.check_all()
    local_failed = local_payload.get("failed") or []
    cut_failed = cut_payload.get("failed") or []
    return {
        "stage": "Stage B",
        "stage_name": "alpha 0.5 local runtime",
        "stage_b_complete": not local_failed and not cut_failed,
        "local_runtime_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "supporting_local_save_load_rows": list(STAGE_B_SUPPORT_ROWS),
        "local_runtime_check": local_payload,
        "local_save_load_subset_check": cut_payload,
        "cut_family_complete": False,
        "distributed_save_load_claimed": False,
        "active_root_promoted": False,
        "parser_runtime_bridge_claimed": False,
        "note": (
            "Stage B closeout is satisfied only by LR-01/02 plus the local-only "
            "CUT-04/17 subset. It does not claim distributed save/load or CUT family completion."
        ),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and payload.get("sample_id"):
        lines = [
            f"{payload['sample_id']} alpha_local_runtime",
            f"  outcome: {payload.get('terminal_outcome')}",
            f"  current_owner: {payload.get('current_owner')}",
            f"  queue_kind: {payload.get('queue_kind')}",
        ]
        return "\n".join(lines)
    if isinstance(payload, dict) and payload.get("stage") == "Stage B":
        lines = [
            f"{payload['stage']} {payload['stage_name']}",
            f"  stage_b_complete: {payload.get('stage_b_complete')}",
            f"  local_runtime_rows: {', '.join(payload.get('local_runtime_rows', []))}",
            "  supporting_local_save_load_rows: "
            + ", ".join(payload.get("supporting_local_save_load_rows", [])),
        ]
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list", help="list implemented local-runtime rows")
    list_parser.add_argument("--format", choices=("text", "json"), default="text")

    run_parser = subparsers.add_parser("run", help="run one local-runtime row")
    run_parser.add_argument("sample_id")
    run_parser.add_argument("--format", choices=("text", "json"), default="text")

    check_parser = subparsers.add_parser("check-all", help="run all local-runtime rows")
    check_parser.add_argument("--format", choices=("text", "json"), default="text")

    closeout_parser = subparsers.add_parser("closeout", help="report local-runtime inventory")
    closeout_parser.add_argument("--format", choices=("text", "json"), default="text")

    stage_b_parser = subparsers.add_parser(
        "stage-b-closeout",
        help="report the current-scope Stage B closeout result over local runtime and local-only save/load subset",
    )
    stage_b_parser.add_argument("--format", choices=("text", "json"), default="text")

    args = parser.parse_args(argv)

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "stage-b-closeout":
        payload = stage_b_closeout()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
