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
BINARY_PATH = REPO_ROOT / "target" / "debug" / "examples" / "mirrorea_alpha_avatar_runtime"

IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "AV-01",
        "summary": "Placeholder avatar runtime accepted as an abstract-role package.",
        "expected_terminal_outcome": "accepted",
        "expected_verdict_kind": "accepted",
        "expected_reason_family": None,
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/avatar-runtime/av-01-placeholder_avatar_runtime.expected.json",
    },
    {
        "sample_id": "AV-02",
        "summary": "Custom Mir avatar runtime accepted without importing concrete formats into core.",
        "expected_terminal_outcome": "accepted",
        "expected_verdict_kind": "accepted",
        "expected_reason_family": None,
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/avatar-runtime/av-02-custom_mir_avatar_runtime.expected.json",
    },
    {
        "sample_id": "AV-06",
        "summary": "Reject untrusted native avatar package before activation.",
        "expected_terminal_outcome": "rejected",
        "expected_verdict_kind": "rejected",
        "expected_reason_family": "provenance_policy",
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/avatar-runtime/av-06-untrusted_native_avatar_rejected.expected.json",
    },
    {
        "sample_id": "AV-08",
        "summary": "Visible placeholder fallback when runtime support is unavailable.",
        "expected_terminal_outcome": "accepted",
        "expected_verdict_kind": "rejected",
        "expected_reason_family": None,
        "expected_fallback": True,
        "expected_sidecar": "samples/alpha/avatar-runtime/av-08-runtime_unavailable_placeholder.expected.json",
    },
    {
        "sample_id": "AV-09",
        "summary": "Reject undeclared effect widening in avatar adapter manifest.",
        "expected_terminal_outcome": "rejected",
        "expected_verdict_kind": "rejected",
        "expected_reason_family": "effect_manifest",
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/avatar-runtime/av-09-adapter_attempts_undeclared_effect.expected.json",
    },
    {
        "sample_id": "HP-11",
        "summary": "Reject unsigned native runtime package.",
        "expected_terminal_outcome": "rejected",
        "expected_verdict_kind": "rejected",
        "expected_reason_family": "provenance_policy",
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json",
    },
    {
        "sample_id": "HP-12",
        "summary": "Reject signed runtime package that exceeds capability budget.",
        "expected_terminal_outcome": "rejected",
        "expected_verdict_kind": "rejected",
        "expected_reason_family": "capability_policy",
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/hotplug-runtime/hp-12-signed_overcapability_package_rejected.expected.json",
    },
    {
        "sample_id": "HP-15",
        "summary": "Reject revoked or stale-signed native runtime package.",
        "expected_terminal_outcome": "rejected",
        "expected_verdict_kind": "rejected",
        "expected_reason_family": "provenance_policy",
        "expected_fallback": False,
        "expected_sidecar": "samples/alpha/hotplug-runtime/hp-15-revoked_signed_package_rejected.expected.json",
    },
]

PLANNED_ONLY_ROWS = [
    "AV-03",
    "AV-04",
    "AV-05",
    "AV-07",
    "AV-10",
    "HP-01",
    "HP-07",
    "HP-08",
    "HP-09",
    "HP-10",
    "HP-13",
    "HP-14",
]
MIRRORED_ELSEWHERE_ROWS = ["HP-02", "HP-03", "HP-04", "HP-05", "HP-06"]

LIMITATIONS = [
    "non-public runtime-private manifest/admission floor only",
    "no final avatar API",
    "no full VRM / VRChat / Unity compatibility",
    "no native binary execution in alpha",
    "no dependent-aware detach lifecycle",
]


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented alpha avatar/package sample: {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-avatar-runtime",
            "source_root": "samples/alpha/avatar-runtime + samples/alpha/hotplug-runtime",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _check_binary_available() -> None:
    if not BINARY_PATH.exists():
        raise RuntimeError(
            "missing prebuilt runtime example "
            f"`{BINARY_PATH}`; run `cargo build -p mir-runtime --example mirrorea_alpha_avatar_runtime` first"
        )


def _run_binary(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    _check_binary_available()
    completed = subprocess.run(
        [str(BINARY_PATH), "run", sample_id],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    report = json.loads(completed.stdout)
    _validate_report(sample_id, row, report)
    return report


def _load_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    sidecar_path = REPO_ROOT / row["expected_sidecar"]
    return json.loads(sidecar_path.read_text())


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
    if report.get("reason_family") != row["expected_reason_family"]:
        raise RuntimeError(
            f"{sample_id}: expected reason_family {row['expected_reason_family']!r} "
            f"but observed {report.get('reason_family')!r}"
        )
    verdict = report.get("hotplug_skeleton", {}).get("verdict", {})
    if verdict.get("verdict_kind") != row["expected_verdict_kind"]:
        raise RuntimeError(
            f"{sample_id}: expected verdict_kind {row['expected_verdict_kind']!r} "
            f"but observed {verdict.get('verdict_kind')!r}"
        )
    representation_state = report.get("representation_state", {})
    if bool(representation_state.get("fallback_applied")) != row["expected_fallback"]:
        raise RuntimeError(
            f"{sample_id}: expected fallback_applied={row['expected_fallback']!r} "
            f"but observed {representation_state.get('fallback_applied')!r}"
        )
    if representation_state.get("native_execution_performed"):
        raise RuntimeError(f"{sample_id}: native execution unexpectedly performed")


def _validate_sidecar_parity(
    sample_id: str, report: dict[str, Any], expected_sidecar: dict[str, Any]
) -> None:
    if report != expected_sidecar:
        raise RuntimeError(f"{sample_id}: sidecar drift detected")


def _validate_report(sample_id: str, row: dict[str, Any], report: dict[str, Any]) -> None:
    _validate_expected_fields(sample_id, row, report)
    _validate_sidecar_parity(sample_id, report, _load_expected_sidecar(row))


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            _run_binary(sample_id)
            passed.append(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": "samples/alpha/avatar-runtime + samples/alpha/hotplug-runtime",
        "binary_path": str(BINARY_PATH),
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "mirrored_elsewhere_rows": list(MIRRORED_ELSEWHERE_ROWS),
        "validation_floor": [
            "cargo build -p mir-runtime --example mirrorea_alpha_avatar_runtime",
            "cargo test -p mir-runtime --test alpha_avatar_runtime",
            "python3 scripts/alpha_avatar_runtime_samples.py check-all --format json",
        ],
        "stop_lines": [
            "do not treat signatures as semantic safety proofs",
            "do not promote VRM / VRChat / Unity concepts into Mir core primitives",
            "do not mark AV-03/04/05/07/10 or HP-01/07/08/09/10/13/14 implemented in this package",
            "do not treat HP-02/03/04/05/06 as implemented by this runner; they are mirrored elsewhere",
        ],
        "limitations": list(LIMITATIONS),
    }

def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and "sample_id" in payload:
        lines = [
            f"{payload['sample_id']} alpha_avatar_runtime",
            f"  outcome: {payload.get('terminal_outcome')}",
            f"  reason_family: {payload.get('reason_family')}",
        ]
        representation_state = payload.get("representation_state", {})
        if representation_state:
            lines.append(
                f"  fallback_applied: {representation_state.get('fallback_applied')}"
            )
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Alpha-0 avatar/runtime package runner")
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list")
    list_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
    run_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    check_parser = subparsers.add_parser("check-all")
    check_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    closeout_parser = subparsers.add_parser("closeout")
    closeout_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    return parser


def main(argv: list[str]) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "list":
        payload = list_samples()
    elif args.command == "run":
        payload = _run_binary(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "closeout":
        payload = closeout()
    else:  # pragma: no cover
        raise AssertionError(f"unhandled command: {args.command}")

    if args.format == "json":
        json.dump(payload, sys.stdout, indent=2)
        sys.stdout.write("\n")
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
