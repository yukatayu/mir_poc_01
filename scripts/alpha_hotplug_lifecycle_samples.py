#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

import alpha_avatar_runtime_samples as avatar_runtime_samples


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent

LAYER_IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "LI-01",
        "summary": "Authorized debug layer attach activates only after the activation cut.",
        "expected_sidecar": "samples/alpha/layer-insertion/li-01-debug_layer_attach_authorized.expected.json",
    },
    {
        "sample_id": "LI-02",
        "summary": "Non-admin debug layer attach is rejected before activation.",
        "expected_sidecar": "samples/alpha/layer-insertion/li-02-debug_layer_non_admin_rejected.expected.json",
    },
    {
        "sample_id": "LI-03",
        "summary": "Auth layer attach is accepted only through an explicit contract-update path.",
        "expected_sidecar": "samples/alpha/layer-insertion/li-03-auth_layer_contract_update_path.expected.json",
    },
    {
        "sample_id": "LI-04",
        "summary": "Rate-limit layer attach preserves declared-failure preview discipline.",
        "expected_sidecar": "samples/alpha/layer-insertion/li-04-ratelimit_declared_failure.expected.json",
    },
    {
        "sample_id": "LI-05",
        "summary": "Incompatible layer patch is rejected before activation.",
        "expected_sidecar": "samples/alpha/layer-insertion/li-05-incompatible_patch_rejected.expected.json",
    },
]

LAYER_REQUIRED_ROWS = [row["sample_id"] for row in LAYER_IMPLEMENTED_ROWS]
AVATAR_PACKAGE_REQUIRED_ROWS = [
    row["sample_id"] for row in avatar_runtime_samples.IMPLEMENTED_ROWS
]

STOP_LINES = [
    "do not treat Stage D closeout as detach runtime completion",
    "do not treat Stage D closeout as durable migration or distributed activation ordering completion",
    "do not treat Stage D closeout as native execution realization or final public avatar/package/layer ABI",
    "do not promote samples/alpha to an active runnable root",
]

LIMITATIONS = [
    "current-scope Stage D closeout bundles existing attach-time layer and runtime-private package/avatar floors only",
    "no dependent-aware detach lifecycle",
    "no durable migration or distributed activation ordering",
    "no native binary execution in alpha",
    "no final public layer/package/avatar ABI",
]


def _layer_row(sample_id: str) -> dict[str, Any]:
    for row in LAYER_IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented layer row: {sample_id}")


def _load_layer_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    sidecar_path = REPO_ROOT / row["expected_sidecar"]
    return json.loads(sidecar_path.read_text())


def list_samples() -> list[dict[str, str]]:
    layer_rows = [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-layer-insertion",
            "source_root": "samples/alpha/layer-insertion",
            "summary": row["summary"],
        }
        for row in LAYER_IMPLEMENTED_ROWS
    ]
    avatar_rows = [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-avatar-runtime",
            "source_root": "samples/alpha/avatar-runtime + samples/alpha/hotplug-runtime",
            "summary": row["summary"],
        }
        for row in avatar_runtime_samples.IMPLEMENTED_ROWS
    ]
    return layer_rows + avatar_rows


def _run_layer_closeout_reports() -> list[dict[str, Any]]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_layer_insertion_runtime",
            "--",
            "closeout",
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    payload = json.loads(completed.stdout)
    if not isinstance(payload, list):
        raise RuntimeError("layer closeout output must be a JSON list")
    return payload


def _find_layer_report(reports: list[dict[str, Any]], sample_id: str) -> dict[str, Any]:
    for report in reports:
        if report.get("sample_id") == sample_id:
            return report
    raise RuntimeError(f"missing layer closeout report for {sample_id}")


def _validate_layer_closeout_reports(reports: list[dict[str, Any]]) -> None:
    observed_sample_ids = [report.get("sample_id") for report in reports]
    if observed_sample_ids != LAYER_REQUIRED_ROWS:
        raise RuntimeError(
            f"layer closeout sample IDs drifted: expected {LAYER_REQUIRED_ROWS!r} "
            f"but observed {observed_sample_ids!r}"
        )

    for row in LAYER_IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        expected_runtime = _load_layer_expected_sidecar(row).get("expected_runtime", {})
        report = _find_layer_report(reports, sample_id)

        if report.get("terminal_outcome") != expected_runtime.get("terminal_outcome"):
            raise RuntimeError(
                f"{sample_id}: expected terminal_outcome "
                f"{expected_runtime.get('terminal_outcome')!r} but observed "
                f"{report.get('terminal_outcome')!r}"
            )

        retained_later_refs = set(report.get("retained_later_refs") or [])
        for kept_later in [
            "completed_hotplug_lifecycle",
            "detach_runtime",
            "durable_migration",
            "distributed_activation_ordering",
            "runtime_package_avatar_admission",
            "final_public_layer_attachment_abi",
        ]:
            if kept_later not in retained_later_refs:
                raise RuntimeError(
                    f"{sample_id}: missing kept-later ref {kept_later!r}"
                )

        if sample_id == "LI-01":
            if report.get("active_layers_after") != [expected_runtime.get("activated_layer")]:
                raise RuntimeError("LI-01: activated layer mismatch")
            if report.get("pre_attach_trace_rows"):
                raise RuntimeError("LI-01: pre-attach trace rows must stay empty")
            post_attach_rows = report.get("post_attach_trace_rows") or []
            if len(post_attach_rows) != expected_runtime.get("post_attach_trace_rows"):
                raise RuntimeError("LI-01: post-attach trace row count mismatch")
            if any(
                row_payload.get("redaction_level") != expected_runtime.get("redaction_level")
                for row_payload in post_attach_rows
            ):
                raise RuntimeError("LI-01: redaction level drifted in post-attach trace")
            if report.get("source_runtime_sample_ref") != expected_runtime.get(
                "source_runtime_sample_ref"
            ):
                raise RuntimeError("LI-01: source runtime sample ref mismatch")
        elif sample_id == "LI-02":
            authorization_reason_refs = set(
                (
                    report.get("hotplug_runtime_report", {})
                    .get("verdict", {})
                    .get("authorization_reason_refs")
                )
                or []
            )
            required_reason = expected_runtime.get("required_authorization_reason_ref")
            if required_reason not in authorization_reason_refs:
                raise RuntimeError(
                    f"LI-02: missing authorization reason ref {required_reason!r}"
                )
            if len(report.get("active_layers_after") or []) != expected_runtime.get(
                "active_layers_after"
            ):
                raise RuntimeError("LI-02: active layer count drifted")
            if len(report.get("post_attach_trace_rows") or []) != expected_runtime.get(
                "post_attach_trace_rows"
            ):
                raise RuntimeError("LI-02: post-attach trace row count drifted")
        elif sample_id == "LI-03":
            if report.get("compatibility", {}).get("accepted_path") != expected_runtime.get(
                "accepted_path"
            ):
                raise RuntimeError("LI-03: accepted_path drifted")
            if report.get("attach_request", {}).get("contract_update_ref") != expected_runtime.get(
                "required_contract_update_ref"
            ):
                raise RuntimeError("LI-03: contract update ref drifted")
        elif sample_id == "LI-04":
            runtime_preview = report.get("runtime_preview") or {}
            if runtime_preview.get("terminal_outcome") != expected_runtime.get(
                "preview_terminal_outcome"
            ):
                raise RuntimeError("LI-04: preview terminal outcome drifted")
            preview_reason_refs = set(runtime_preview.get("reason_refs") or [])
            for required_reason in expected_runtime.get("required_preview_reason_refs") or []:
                if required_reason not in preview_reason_refs:
                    raise RuntimeError(
                        f"LI-04: missing preview reason ref {required_reason!r}"
                    )
        elif sample_id == "LI-05":
            failed_reason_refs = set(
                (report.get("compatibility", {}).get("failed_reason_refs")) or []
            )
            for required_reason in expected_runtime.get("required_failure_refs") or []:
                if required_reason not in failed_reason_refs:
                    raise RuntimeError(
                        f"LI-05: missing compatibility failure ref {required_reason!r}"
                    )
            if report.get("active_layers_after"):
                raise RuntimeError("LI-05: rejected layer must not remain active")
        else:  # pragma: no cover - guarded by LAYER_IMPLEMENTED_ROWS
            raise RuntimeError(f"unsupported layer row {sample_id}")


def check_layer_closeout() -> dict[str, Any]:
    try:
        reports = _run_layer_closeout_reports()
        _validate_layer_closeout_reports(reports)
        return {
            "sample_count": len(LAYER_REQUIRED_ROWS),
            "passed": list(LAYER_REQUIRED_ROWS),
            "failed": [],
        }
    except Exception as error:  # pragma: no cover
        return {
            "sample_count": len(LAYER_REQUIRED_ROWS),
            "passed": [],
            "failed": [{"floor": "layer-closeout", "error": str(error)}],
        }


def _validate_avatar_package_inventory(payload: dict[str, Any]) -> None:
    if payload.get("implemented_rows") != AVATAR_PACKAGE_REQUIRED_ROWS:
        raise RuntimeError(
            "avatar/package implemented rows drifted: "
            f"{payload.get('implemented_rows')!r}"
        )
    if payload.get("planned_only_rows") != list(avatar_runtime_samples.PLANNED_ONLY_ROWS):
        raise RuntimeError("avatar/package planned-only rows drifted")
    if payload.get("mirrored_elsewhere_rows") != list(
        avatar_runtime_samples.MIRRORED_ELSEWHERE_ROWS
    ):
        raise RuntimeError("avatar/package mirrored-elsewhere rows drifted")
    limitations = set(payload.get("limitations") or [])
    for kept_later in [
        "no final avatar API",
        "no full VRM / VRChat / Unity compatibility",
        "no native binary execution in alpha",
        "no dependent-aware detach lifecycle",
    ]:
        if kept_later not in limitations:
            raise RuntimeError(
                f"avatar/package inventory missing limitation {kept_later!r}"
            )


def _validate_avatar_package_check(payload: dict[str, Any]) -> None:
    if payload.get("sample_count") != len(AVATAR_PACKAGE_REQUIRED_ROWS):
        raise RuntimeError("avatar/package sample_count drifted")
    if payload.get("passed") != AVATAR_PACKAGE_REQUIRED_ROWS:
        raise RuntimeError(
            "avatar/package passed rows drifted: "
            f"{payload.get('passed')!r}"
        )
    if payload.get("failed"):
        raise RuntimeError(
            f"avatar/package floor reported failures: {payload.get('failed')!r}"
        )


def check_avatar_package_floor() -> dict[str, Any]:
    try:
        inventory = avatar_runtime_samples.closeout()
        _validate_avatar_package_inventory(inventory)
        check = avatar_runtime_samples.check_all()
        _validate_avatar_package_check(check)
        return {
            "sample_count": len(AVATAR_PACKAGE_REQUIRED_ROWS),
            "passed": list(AVATAR_PACKAGE_REQUIRED_ROWS),
            "failed": [],
            "inventory": inventory,
        }
    except Exception as error:  # pragma: no cover
        return {
            "sample_count": len(AVATAR_PACKAGE_REQUIRED_ROWS),
            "passed": [],
            "failed": [{"floor": "avatar-package", "error": str(error)}],
            "inventory": avatar_runtime_samples.closeout(),
        }


def closeout() -> dict[str, Any]:
    avatar_inventory = avatar_runtime_samples.closeout()
    return {
        "sample_root": (
            "samples/alpha/layer-insertion + samples/alpha/avatar-runtime + "
            "samples/alpha/hotplug-runtime"
        ),
        "layer_rows": list(LAYER_REQUIRED_ROWS),
        "runtime_package_avatar_rows": list(AVATAR_PACKAGE_REQUIRED_ROWS),
        "planned_package_rows": list(avatar_inventory["planned_only_rows"]),
        "mirrored_hotplug_rows": list(avatar_inventory["mirrored_elsewhere_rows"]),
        "validation_floor": [
            "cargo test -p mir-runtime --test alpha_layer_insertion_runtime",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout",
            "cargo test -p mir-runtime --test alpha_avatar_runtime",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout",
            "python3 scripts/alpha_avatar_runtime_samples.py check-all --format json",
            "python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json",
            "python3 -m unittest scripts.tests.test_alpha_hotplug_lifecycle_samples",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "stage_d_closeout_scope": (
            "attach-time layer subset + runtime-private package/avatar admission subset only"
        ),
        "active_root_promoted": False,
    }


def stage_d_closeout() -> dict[str, Any]:
    layer_check = check_layer_closeout()
    avatar_package_floor = check_avatar_package_floor()
    return {
        "stage": "Stage D",
        "stage_name": "alpha 0.8 hot-plug lifecycle",
        "stage_d_complete": not layer_check.get("failed") and not avatar_package_floor.get(
            "failed"
        ),
        "layer_rows": list(LAYER_REQUIRED_ROWS),
        "runtime_package_avatar_rows": list(AVATAR_PACKAGE_REQUIRED_ROWS),
        "layer_closeout_check": layer_check,
        "runtime_package_avatar_floor": avatar_package_floor,
        "detach_runtime_complete": False,
        "durable_migration_complete": False,
        "distributed_activation_ordering_complete": False,
        "native_execution_claimed": False,
        "final_public_layer_attachment_abi_claimed": False,
        "final_public_runtime_package_abi_claimed": False,
        "active_root_promoted": False,
        "note": (
            "Stage D closeout is satisfied only by LI-01/02/03/04/05 plus "
            "AV-01/02/06/08/09 and HP-11/12/15. It does not claim detach runtime, "
            "durable migration, distributed activation ordering, native execution, "
            "or final public layer/package/avatar ABI."
        ),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and payload.get("stage") == "Stage D":
        lines = [
            f"{payload['stage']} {payload['stage_name']}",
            f"  stage_d_complete: {payload.get('stage_d_complete')}",
            f"  layer_rows: {', '.join(payload.get('layer_rows', []))}",
            "  runtime_package_avatar_rows: "
            + ", ".join(payload.get("runtime_package_avatar_rows", [])),
        ]
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list", help="list Stage D implemented rows")
    list_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    closeout_parser = subparsers.add_parser("closeout", help="show Stage D inventory")
    closeout_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    stage_d_parser = subparsers.add_parser(
        "stage-d-closeout", help="run the Stage D current-scope closeout"
    )
    stage_d_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    args = parser.parse_args(argv)
    if args.command == "list":
        payload = list_samples()
    elif args.command == "closeout":
        payload = closeout()
    elif args.command == "stage-d-closeout":
        payload = stage_d_closeout()
    else:  # pragma: no cover - argparse enforces choices.
        raise AssertionError(f"unknown command {args.command}")

    if args.format == "pretty":
        print(format_pretty(payload))
    else:
        print(json.dumps(payload, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
