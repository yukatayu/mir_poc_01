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

import practical_alpha1_avatar  # noqa: E402


BASE_SESSION_PACKAGE = "samples/practical-alpha1/packages/run-01-local-sugoroku"

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "OA08-01",
        "summary": "attach one debug trace layer into the live alpha-0.5 session and observe active layer growth",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-02",
        "summary": "reject non-admin debug attach without mutating the live session frontier",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-02-non-admin-debug-rejected",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-03",
        "summary": "attach one auth gate layer through the explicit contract-update path",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-03-auth-layer-contract-update",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-04",
        "summary": "attach one rate-limit layer and keep the declared failure preview visible in-session",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-04-ratelimit-declared-failure",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-05",
        "summary": "reject one incompatible patch without mutating the live session frontier",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-05-incompatible-patch-rejected",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-06",
        "summary": "reject one stale-membership attach without mutating the live session frontier",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-04b1-stale-membership-attach-rejected",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-07",
        "summary": "reject one missing-witness attach without mutating the live session frontier",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-04b2-missing-witness-attach-rejected",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-08",
        "summary": "attach one placeholder object package preview into the live session inventory",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-06-object-package-attach",
        "kind": "attach",
    },
    {
        "sample_id": "OA08-09",
        "summary": "keep unsupported runtime fallback explicit through same-session rejection plus avatar companion preview",
        "package_dir": "samples/practical-alpha1/packages/av-a1-03-unsupported-runtime-fallback",
        "kind": "fallback_source",
    },
    {
        "sample_id": "OA08-10",
        "summary": "project one accepted attach plus one deferred detach boundary into the same live session export",
        "package_dir": "samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract",
        "kind": "detach_sequence",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-0.8 session hot-plug command as distributed durable save/load completion",
    "do not treat the practical alpha-0.8 session hot-plug command as alpha-0.9 live viewer completion",
    "do not treat practical object/avatar preview visibility as native execution or a final public runtime/devtools/hotplug ABI",
]

LIMITATIONS = [
    "alpha-local non-final practical alpha-0.8 same-session hot-plug floor only",
    "rejected attaches stay report-visible but do not mutate the live session frontier",
    "distributed durable save/load, alpha-0.9 viewer, and final public ABI remain later",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-0.8 session hot-plug sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha08-session-hotplug",
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


def _run_session_observe(session_path: str | Path) -> dict[str, Any]:
    return _cargo_session("observe", str(session_path))


def _run_session_attach(session_path: str | Path, package_path: str | Path) -> dict[str, Any]:
    return _cargo_session("attach", str(session_path), str(package_path), str(session_path))


def _base_session_payload() -> tuple[dict[str, Any], dict[str, Any], Path, tempfile.TemporaryDirectory[str]]:
    temp_dir = tempfile.TemporaryDirectory(prefix="oa08-")
    session_path = Path(temp_dir.name) / "session.json"
    started = _run_session_start(REPO_ROOT / BASE_SESSION_PACKAGE, session_path)
    observed = _run_session_observe(session_path)
    return started, observed, session_path, temp_dir


def _run_attach_row(row: dict[str, str]) -> dict[str, Any]:
    started, observer_before, session_path, temp_dir = _base_session_payload()
    try:
        attach_report = _run_session_attach(session_path, REPO_ROOT / row["package_dir"])
        observer_after = _run_session_observe(session_path)
        return {
            "sample_id": row["sample_id"],
            "family": "practical-alpha08-session-hotplug",
            "session_report_before_attach": started,
            "observer_safe_export_before_attach": observer_before,
            "attach_report": attach_report,
            "observer_safe_export_after_attach": observer_after,
        }
    finally:
        temp_dir.cleanup()


def _run_fallback_row(row: dict[str, str]) -> dict[str, Any]:
    payload = _run_attach_row(row)
    payload["avatar_preview_source"] = practical_alpha1_avatar.run_sample("AV-A1-03")
    return payload


def _run_detach_sequence_row(row: dict[str, str]) -> dict[str, Any]:
    started, observer_before, session_path, temp_dir = _base_session_payload()
    try:
        debug_attach = _run_session_attach(
            session_path,
            REPO_ROOT / "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
        )
        detach_attach = _run_session_attach(session_path, REPO_ROOT / row["package_dir"])
        observer_after = _run_session_observe(session_path)
        return {
            "sample_id": row["sample_id"],
            "family": "practical-alpha08-session-hotplug",
            "session_report_before_attach": started,
            "observer_safe_export_before_attach": observer_before,
            "accepted_attach_report": debug_attach,
            "detach_report": detach_attach,
            "observer_safe_export_after_attach": observer_after,
        }
    finally:
        temp_dir.cleanup()


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    if row["kind"] == "attach":
        return _run_attach_row(row)
    if row["kind"] == "fallback_source":
        return _run_fallback_row(row)
    return _run_detach_sequence_row(row)


def _validate_attach_row(sample_id: str, payload: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    before = payload["observer_safe_export_before_attach"]
    report = payload["attach_report"]
    after = payload["observer_safe_export_after_attach"]

    if sample_id == "OA08-01":
        if report["terminal_outcome"] != "accepted":
            failures.append("debug attach must be accepted")
        if report["active_layers_after"] != ["debug_trace_layer"]:
            failures.append("debug attach must expose one active debug layer")
        if after["active_layers"] != ["debug_trace_layer"]:
            failures.append("observer export must show the active debug layer")
        if after["hotplug_events"] != ["hotplug:HP-A1-01:accepted"]:
            failures.append("observer export must show one accepted hot-plug event")
    elif sample_id == "OA08-02":
        if report["terminal_outcome"] != "rejected":
            failures.append("non-admin debug attach must be rejected")
        if report["reason_family"] != "authorization":
            failures.append("non-admin debug attach must reject on authorization")
        if report["session_mutated"] is not False:
            failures.append("rejected attach must not mutate the live session")
        if before["event_ids"] != after["event_ids"] or before["active_layers"] != after["active_layers"]:
            failures.append("rejected attach must leave runtime event DAG and active layers unchanged")
        if after["hotplug_events"] != ["hotplug:HP-A1-02:rejected"]:
            failures.append("rejected attach must still become visible as a session observation")
    elif sample_id == "OA08-03":
        if report["terminal_outcome"] != "accepted_contract_update":
            failures.append("auth layer must use the accepted contract-update path")
        if report["activation_cut_ref"] != "activation_cut#auth_contract_update":
            failures.append("auth layer must preserve the activation cut")
        if "auth_gate_layer" not in report["active_layers_after"]:
            failures.append("auth layer must become visible in the session active layer set")
        if "auth_contract_update_active" not in after["runtime_behavior_markers"]:
            failures.append("observer export must show the auth contract update marker")
    elif sample_id == "OA08-04":
        if report["terminal_outcome"] != "accepted":
            failures.append("rate-limit layer must be accepted at the current floor")
        if "declared_failure_preview:RateLimited" not in report["runtime_behavior_delta"]:
            failures.append("rate-limit attach must preserve the declared failure preview")
        if "declared_failure_preview:RateLimited" not in after["runtime_behavior_markers"]:
            failures.append("observer export must show the declared failure preview marker")
    elif sample_id == "OA08-05":
        if report["reason_family"] != "compatibility":
            failures.append("incompatible patch must reject on compatibility")
        if report["session_mutated"] is not False:
            failures.append("incompatible patch reject must not mutate the live session")
        if before["event_ids"] != after["event_ids"] or before["active_layers"] != after["active_layers"]:
            failures.append("incompatible patch reject must leave runtime event DAG and active layers unchanged")
        if after["hotplug_events"] != ["hotplug:HP-A1-05:rejected"]:
            failures.append("incompatible patch reject must still become visible as a session observation")
    elif sample_id == "OA08-06":
        if report["reason_family"] != "membership_freshness":
            failures.append("stale-membership attach must reject on membership freshness")
        if report["session_mutated"] is not False:
            failures.append("stale-membership reject must not mutate the live session")
        if before["event_ids"] != after["event_ids"] or before["active_layers"] != after["active_layers"]:
            failures.append("stale-membership reject must leave runtime event DAG and active layers unchanged")
        if after["hotplug_events"] != ["hotplug:HP-A1-04B1:rejected"]:
            failures.append("stale-membership reject must still become visible as a session observation")
    elif sample_id == "OA08-07":
        if report["reason_family"] != "witness":
            failures.append("missing-witness attach must reject on witness evidence")
        if report["session_mutated"] is not False:
            failures.append("missing-witness reject must not mutate the live session")
        if before["event_ids"] != after["event_ids"] or before["active_layers"] != after["active_layers"]:
            failures.append("missing-witness reject must leave runtime event DAG and active layers unchanged")
        if after["hotplug_events"] != ["hotplug:HP-A1-04B2:rejected"]:
            failures.append("missing-witness reject must still become visible as a session observation")
    elif sample_id == "OA08-08":
        if report["terminal_outcome"] != "accepted_object_attach_preview":
            failures.append("object package attach must stay preview-only but accepted")
        if "static_capsule_placeholder" not in "".join(after["object_preview_inventory"]):
            failures.append("observer export must show the placeholder object preview inventory")
        if "object_preview_inventory_visible" not in report["devtools_delta"]:
            failures.append("object package attach must mutate the observer/devtools surface")
    return failures


def _validate_fallback_row(payload: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    before = payload["observer_safe_export_before_attach"]
    after = payload["observer_safe_export_after_attach"]
    report = payload["attach_report"]
    preview = payload["avatar_preview_source"]
    if report["terminal_outcome"] != "rejected":
        failures.append("unsupported runtime attach must stay rejected in the live session path")
    if report["reason_family"] != "compatibility":
        failures.append("unsupported runtime attach must reject on compatibility")
    if report["session_mutated"] is not False:
        failures.append("unsupported runtime rejection must not mutate the live session")
    if before["event_ids"] != after["event_ids"] or before["active_layers"] != after["active_layers"]:
        failures.append("unsupported runtime rejection must leave runtime event DAG and active layers unchanged")
    if after["hotplug_events"] != ["hotplug:AV-A1-03:rejected"]:
        failures.append("unsupported runtime rejection must still become visible as a session observation")
    if preview["fallback_visible"] is not True:
        failures.append("avatar companion preview must keep fallback visibility explicit")
    if preview["fallback_reason"] != "missing_host_capability:HostMirAvatarVM":
        failures.append("avatar companion preview must preserve the fallback reason")
    return failures


def _validate_detach_sequence_row(payload: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    accepted = payload["accepted_attach_report"]
    detach = payload["detach_report"]
    after = payload["observer_safe_export_after_attach"]
    if accepted["terminal_outcome"] != "accepted":
        failures.append("detach sequence must begin with one accepted attach")
    if detach["terminal_outcome"] != "deferred_detach_minimal_contract":
        failures.append("detach sequence must stay at the deferred minimal-contract boundary")
    if detach["detach_boundary_ref"] != "detach_boundary#alpha_local_hotplug_minimal_contract":
        failures.append("deferred detach must preserve the detach boundary ref")
    if after["hotplug_events"] != [
        "hotplug:HP-A1-01:accepted",
        "hotplug:HP-A1-07:deferred_detach_minimal_contract",
    ]:
        failures.append("observer export must show the same-session attach and deferred detach lifecycle")
    return failures


def _validate_sample(sample_id: str, payload: dict[str, Any]) -> list[str]:
    if sample_id == "OA08-09":
        return _validate_fallback_row(payload)
    if sample_id == "OA08-10":
        return _validate_detach_sequence_row(payload)
    return _validate_attach_row(sample_id, payload)


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
        "same_session_hotplug_ready": not failed,
        "accepted_debug_attach_present": "OA08-01" in passed,
        "auth_contract_update_present": "OA08-03" in passed,
        "rate_limit_preview_present": "OA08-04" in passed,
        "rejected_attach_no_mutation_present": all(
            sample_id in passed for sample_id in ["OA08-02", "OA08-05", "OA08-06", "OA08-07"]
        ),
        "object_preview_present": "OA08-08" in passed,
        "unsupported_runtime_fallback_present": "OA08-09" in passed,
        "deferred_detach_boundary_present": "OA08-10" in passed,
        "hotplug_lifecycle_export_present": "OA08-10" in passed,
        "same_session_behavior_change_present": all(
            sample_id in passed for sample_id in ["OA08-01", "OA08-03", "OA08-04", "OA08-08"]
        ),
        "operational_alpha08_ready": not failed
        and "OA08-10" in passed
        and "OA08-09" in passed,
    }


def closeout() -> dict[str, Any]:
    payload = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture",
            "python3 scripts/practical_alpha08_session_hotplug.py check-all --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug",
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
            "Practical alpha-0.8 same-session hot-plug helper over the alpha-0.5 session carrier. "
            "This remains non-final and bounded to local observer-safe session exports."
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
