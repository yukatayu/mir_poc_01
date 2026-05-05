#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
sys.path.insert(0, str(SCRIPT_DIR))

import practical_alpha1_export_devtools  # noqa: E402
import practical_alpha1_save_load  # noqa: E402


IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "OA05-01",
        "summary": "start, observe, save, and load one accepted local runtime session",
        "package_dir": "samples/practical-alpha1/packages/run-01-local-sugoroku",
        "kind": "session_start_save_load",
    },
    {
        "sample_id": "OA05-02",
        "summary": "inspect stale-membership rejection through the session carrier",
        "package_dir": "samples/practical-alpha1/packages/run-02-stale-membership-rejected",
        "kind": "session_start_observe",
    },
    {
        "sample_id": "OA05-03",
        "summary": "inspect missing-capability rejection through the session carrier",
        "package_dir": "samples/practical-alpha1/packages/run-03-missing-capability-rejected",
        "kind": "session_start_observe",
    },
    {
        "sample_id": "OA05-04",
        "summary": "inspect missing-witness rejection through the session carrier",
        "package_dir": "samples/practical-alpha1/packages/run-04-missing-witness-rejected",
        "kind": "session_start_observe",
    },
    {
        "sample_id": "OA05-05",
        "summary": "reuse exact local save/load evidence for stale-membership non-resurrection",
        "package_dir": "samples/practical-alpha1/packages/sl-a1-02-local-load-stale-membership-rejected",
        "kind": "save_load_source",
    },
    {
        "sample_id": "OA05-06",
        "summary": "reuse exact fallback degradation visibility as an alpha-0.5 operational matrix row",
        "package_dir": "samples/practical-alpha1/packages/av-a1-03-unsupported-runtime-fallback",
        "kind": "fallback_source",
    },
    {
        "sample_id": "OA05-07",
        "summary": "run one minimal typed external AddOne host-I/O lane on the live alpha-0.5 session",
        "package_dir": "samples/practical-alpha1/packages/oa05-07-add-one-host-io",
        "kind": "session_host_io",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-0.5 session command as same-session hot-plug completion",
    "do not treat the practical alpha-0.5 session command as distributed durable save/load completion",
    "do not treat the practical alpha-0.5 session command as a final public runtime or devtools ABI",
]

LIMITATIONS = [
    "alpha-local non-final practical alpha-0.5 session floor only",
    "typed external host-I/O is limited to one minimal AddOne adapter family",
    "same-session hot-plug, distributed durable save/load, and final public ABI remain later",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-0.5 session sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha05-session",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _cargo_session(*args: str) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha05_session",
            "--",
            *args,
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def _run_session_start(package_path: str | Path, session_path: str | Path) -> dict[str, Any]:
    return _cargo_session("start", str(package_path), str(session_path))


def _run_session_save(session_path: str | Path, savepoint_id: str) -> dict[str, Any]:
    return _cargo_session("save", str(session_path), savepoint_id, str(session_path))


def _run_session_load(session_path: str | Path, savepoint_id: str) -> dict[str, Any]:
    return _cargo_session("load", str(session_path), savepoint_id, str(session_path))


def _run_session_observe(session_path: str | Path) -> dict[str, Any]:
    return _cargo_session("observe", str(session_path))


def _run_session_host_io(session_path: str | Path, package_path: str | Path) -> dict[str, Any]:
    return _cargo_session("host-io", str(session_path), str(package_path), str(session_path))


def _run_session_sample(row: dict[str, str]) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix=f"{row['sample_id'].lower()}-") as temp_dir:
        session_path = Path(temp_dir) / "session.json"
        started = _run_session_start(REPO_ROOT / row["package_dir"], session_path)
        observed = _run_session_observe(session_path)
        payload: dict[str, Any] = {
            "sample_id": row["sample_id"],
            "family": "practical-alpha05-session",
            "session_report": started,
            "observer_safe_export": observed,
        }
        if row["kind"] == "session_start_save_load":
            saved = _run_session_save(session_path, "savepoint#1")
            loaded = _run_session_load(session_path, "savepoint#1")
            payload["saved_session"] = saved
            payload["loaded_session"] = loaded
            payload["loaded_observer_safe_export"] = _run_session_observe(session_path)
        return payload


def _run_session_host_io_sample(row: dict[str, str]) -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix=f"{row['sample_id'].lower()}-") as temp_dir:
        session_path = Path(temp_dir) / "session.json"
        started = _run_session_start(REPO_ROOT / row["package_dir"], session_path)
        host_io_report = _run_session_host_io(session_path, REPO_ROOT / row["package_dir"])
        observer_after_host_io = _run_session_observe(session_path)
        return {
            "sample_id": row["sample_id"],
            "family": "practical-alpha05-session",
            "session_report_before_host_io": started,
            "host_io_report": host_io_report,
            "observer_safe_export_after_host_io": observer_after_host_io,
        }


def _run_save_load_source(row: dict[str, str]) -> dict[str, Any]:
    report = practical_alpha1_save_load.run_sample("SL-A1-02")
    return {
        "sample_id": row["sample_id"],
        "family": "practical-alpha05-session",
        "source_family": "practical-alpha1-save-load",
        "source_report": report,
    }


def _run_fallback_source(row: dict[str, str]) -> dict[str, Any]:
    bundle = practical_alpha1_export_devtools.run_sample("VIS-A1-05")
    return {
        "sample_id": row["sample_id"],
        "family": "practical-alpha05-session",
        "source_family": "practical-alpha1-devtools-export",
        "source_bundle": bundle,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    if row["kind"] in {"session_start_observe", "session_start_save_load"}:
        return _run_session_sample(row)
    if row["kind"] == "session_host_io":
        return _run_session_host_io_sample(row)
    if row["kind"] == "save_load_source":
        return _run_save_load_source(row)
    return _run_fallback_source(row)


def _validate_sample(sample_id: str, payload: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    if sample_id == "OA05-01":
        session = payload["session_report"]
        observer = payload["observer_safe_export"]
        saved = payload["saved_session"]
        loaded = payload["loaded_session"]
        if session["session_phase"] != "started":
            failures.append("session phase must start as `started`")
        if observer["redaction"] != "observer_safe_session_summary":
            failures.append("observer-safe export redaction must stay explicit")
        if not saved["savepoints"] or not saved["savepoints"][0]["state_roundtrip_equal"]:
            failures.append("savepoint roundtrip must be explicit and equal")
        if loaded["session_phase"] != "loaded":
            failures.append("loaded session must enter `loaded` phase")
        if loaded["event_dag"] != session["event_dag"]:
            failures.append("load must restore the same event DAG frontier")
    elif sample_id in {"OA05-02", "OA05-03", "OA05-04"}:
        session = payload["session_report"]
        observer = payload["observer_safe_export"]
        expected_outcome = {
            "OA05-02": "rejected_stale_membership",
            "OA05-03": "rejected_missing_capability",
            "OA05-04": "rejected_missing_witness",
        }[sample_id]
        if session["dispatch_records"][0]["dispatch_outcome"] != expected_outcome:
            failures.append(f"dispatch outcome must be {expected_outcome}")
        if observer["visible_history"] != []:
            failures.append("negative rows must not publish visible history")
    elif sample_id == "OA05-05":
        report = payload["source_report"]
        if report["resumed_dispatch_records"][0]["dispatch_outcome"] != "rejected_stale_membership":
            failures.append("save/load source row must reject stale membership")
        if report["state_roundtrip_equal"] is not True:
            failures.append("save/load source row must preserve local frontier roundtrip")
    elif sample_id == "OA05-06":
        bundle = payload["source_bundle"]
        if bundle["sample_id"] != "VIS-A1-05":
            failures.append("fallback source row must point at VIS-A1-05")
        if bundle["export_sections"]["fallback_degradation"]["fallback_visible"] is not True:
            failures.append("fallback degradation must remain visible")
    elif sample_id == "OA05-07":
        report = payload["host_io_report"]
        observer = payload["observer_safe_export_after_host_io"]
        if report["adapter_kind"] != "add_one":
            failures.append("typed host-I/O row must execute the AddOne adapter")
        if report["request_payload"] != {"kind": "int", "value": 41}:
            failures.append("typed host-I/O row must preserve the typed request payload")
        if report["response_payload"] != {"kind": "int", "value": 42}:
            failures.append("typed host-I/O row must preserve the typed response payload")
        if report["terminal_outcome"] != "accepted":
            failures.append("typed host-I/O row must finish accepted")
        if "host_response#1" not in report["session_event_ids_after"]:
            failures.append("typed host-I/O row must append a host response event")
        if "host_io:AddOne(41)->42" not in observer["host_io_events"]:
            failures.append("observer-safe export must carry the redacted host-I/O summary")
    return failures


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    rows: dict[str, dict[str, Any]] = {}
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            payload = run_sample(sample_id)
            failures = _validate_sample(sample_id, payload)
            if failures:
                raise RuntimeError("; ".join(failures))
            passed.append(sample_id)
            rows[sample_id] = payload
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})

    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "session_carrier_ready": not failed,
        "session_bound_event_dag_present": "OA05-01" in passed,
        "observer_safe_export_present": "OA05-01" in passed,
        "local_save_load_roundtrip_present": "OA05-01" in passed,
        "stale_membership_non_resurrection_present": "OA05-05" in passed,
        "fallback_degradation_visible_present": "OA05-06" in passed,
        "typed_host_io_demo_present": "OA05-07" in passed,
        "operational_alpha05_ready": not failed and "OA05-07" in passed,
    }


def closeout() -> dict[str, Any]:
    payload = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture",
            "python3 scripts/practical_alpha1_run_local.py check-all --format json",
            "python3 scripts/practical_alpha05_session.py check-all --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        **payload,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-0.5 session carrier over the practical local-runtime floor. "
            "This remains non-final and includes one minimal typed external AddOne host-I/O lane."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("list")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
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
    return [*hoisted_root_options, *remainder]


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))
    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
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
