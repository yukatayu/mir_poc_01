#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "engine-adapter"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
WORLD_SEMANTICS_OWNER = "mir_mirrorea"
DEFAULT_NATIVE_EXECUTION_POLICY = "Disabled"
DEFAULT_WASM_EXECUTION_POLICY = "InventoryOnly"
ALLOWED_ROLLBACK_REPLAY_CUT_POLICIES = {
    "Replayable",
    "CompensatingActionRequired",
    "IrreversibleCutBarrier",
    "InventoryOnly",
    "Disabled",
}
REQUIRED_ROW_FIELDS = (
    "provider_id",
    "provider_kind",
    "root_name",
    "stage",
    "current_status",
    "representative_source",
    "input_schema",
    "output_schema",
    "effect_row",
    "failure_row",
    "required_capability",
    "authority_policy",
    "observation_policy",
    "redaction_policy",
    "packet_boundary",
    "ffi_boundary",
    "resource_policy",
    "sandbox_policy",
    "native_execution_policy",
    "wasm_execution_policy",
    "rollback_replay_cut_policy",
)
STOP_LINES = [
    "no Unity / Unreal integration claim",
    "no arbitrary native package execution",
    "no arbitrary WASM execution",
    "no final engine adapter ABI or SDK",
    "no renderer-owned world semantics",
]
NON_CLAIMS = [
    "no Unity / Unreal integration",
    "no VRM / VRChat compatibility",
    "no arbitrary native package execution",
    "no arbitrary WASM package execution",
    "no final FFI ABI or engine adapter SDK",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples",
    "python3 scripts/engine_adapter_boundary_samples.py matrix --format json",
    "python3 scripts/engine_adapter_boundary_samples.py check-all --format json",
    "python3 scripts/engine_adapter_boundary_samples.py run wasm-sandbox --format json",
]


def _load_matrix_file() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def validate_rows(
    sample_root: Path,
    rows: list[dict[str, Any]],
    default_native_execution_policy: str,
    default_wasm_execution_policy: str,
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    seen_provider_ids: set[str] = set()
    for row in rows:
        provider_id = row.get("provider_id", "<unknown>")
        if provider_id in seen_provider_ids:
            errors.append(
                {
                    "provider_id": provider_id,
                    "kind": "duplicate_provider_id",
                    "detail": f"duplicate provider_id `{provider_id}`",
                }
            )
        seen_provider_ids.add(provider_id)

        for field in REQUIRED_ROW_FIELDS:
            if field not in row:
                errors.append(
                    {
                        "provider_id": provider_id,
                        "kind": "missing_required_field",
                        "detail": f"missing required field `{field}`",
                    }
                )

        root_name = row.get("root_name")
        if root_name is not None:
            root_path = sample_root / root_name
            if not root_path.exists():
                errors.append(
                    {
                        "provider_id": provider_id,
                        "kind": "missing_root",
                        "detail": f"missing provider root `{root_path}`",
                    }
                )

        representative_source = row.get("representative_source")
        if representative_source is not None:
            source_path = sample_root / representative_source
            if not source_path.exists():
                errors.append(
                    {
                        "provider_id": provider_id,
                        "kind": "missing_representative_source",
                        "detail": f"missing representative source `{source_path}`",
                    }
                )

        policy = row.get("rollback_replay_cut_policy")
        if (
            policy is not None
            and policy not in ALLOWED_ROLLBACK_REPLAY_CUT_POLICIES
        ):
            errors.append(
                {
                    "provider_id": provider_id,
                    "kind": "invalid_rollback_replay_cut_policy",
                    "detail": f"unsupported rollback/replay/cut policy `{policy}`",
                }
            )

        if (
            row.get("native_execution_policy") is not None
            and row["native_execution_policy"] != default_native_execution_policy
        ):
            errors.append(
                {
                    "provider_id": provider_id,
                    "kind": "native_execution_policy_mismatch",
                    "detail": (
                        "native execution policy must stay "
                        f"`{default_native_execution_policy}` in P-ENG-01"
                    ),
                }
            )

        if (
            row.get("wasm_execution_policy") is not None
            and row["wasm_execution_policy"] != default_wasm_execution_policy
        ):
            errors.append(
                {
                    "provider_id": provider_id,
                    "kind": "wasm_execution_policy_mismatch",
                    "detail": (
                        "WASM execution policy must stay "
                        f"`{default_wasm_execution_policy}` in P-ENG-01"
                    ),
                }
            )

        authority_policy = row.get("authority_policy")
        if (
            isinstance(authority_policy, dict)
            and authority_policy.get("semantic_authority_owner")
            not in {None, WORLD_SEMANTICS_OWNER}
        ):
            errors.append(
                {
                    "provider_id": provider_id,
                    "kind": "semantic_authority_owner_mismatch",
                    "detail": (
                        "authority policy must preserve semantic ownership in "
                        f"`{WORLD_SEMANTICS_OWNER}`"
                    ),
                }
            )

        resource_policy = row.get("resource_policy")
        if (
            isinstance(resource_policy, dict)
            and resource_policy.get("semantic_state_owner")
            not in {None, WORLD_SEMANTICS_OWNER}
        ):
            errors.append(
                {
                    "provider_id": provider_id,
                    "kind": "semantic_state_owner_mismatch",
                    "detail": (
                        "resource policy must preserve semantic ownership in "
                        f"`{WORLD_SEMANTICS_OWNER}`"
                    ),
                }
            )

    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    root_path = SAMPLE_ROOT / row["root_name"]
    source_path = SAMPLE_ROOT / row["representative_source"]
    payload = dict(row)
    payload["root_path"] = str(root_path.relative_to(REPO_ROOT))
    payload["representative_source"] = str(source_path.relative_to(REPO_ROOT))
    payload["runnable"] = row["current_status"] == "executable"
    payload["workflow_ready"] = False
    return payload


def list_providers() -> list[dict[str, Any]]:
    data = _load_matrix_file()
    return [_materialize_row(row) for row in data["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix_file()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(
        SAMPLE_ROOT,
        data["rows"],
        data["default_native_execution_policy"],
        data["default_wasm_execution_policy"],
    )
    planned_only_rows = [
        row["provider_id"] for row in rows if row["current_status"] == "planned_only"
    ]
    executable_rows = [
        row["provider_id"] for row in rows if row["current_status"] == "executable"
    ]
    planned_count = sum(1 for row in rows if row["current_status"] == "planned_only")
    executable_count = sum(1 for row in rows if row["current_status"] == "executable")
    return {
        "command": "matrix",
        "family": data["family"],
        "provider_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "provider_count": len(rows),
        "planned_count": planned_count,
        "executable_count": executable_count,
        "planned_only_rows": planned_only_rows,
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "inventory_status": data["inventory_status"],
        "world_semantics_owner": data["world_semantics_owner"],
        "packet_ffi_transport_split": data["packet_ffi_transport_split"],
        "default_native_execution_policy": data["default_native_execution_policy"],
        "default_wasm_execution_policy": data["default_wasm_execution_policy"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def run_provider(provider_id: str) -> dict[str, Any]:
    data = _load_matrix_file()
    row = next((row for row in data["rows"] if row["provider_id"] == provider_id), None)
    if row is None:
        raise ValueError(f"unknown engine adapter provider `{provider_id}`")
    realized = _materialize_row(row)
    if row["current_status"] != "executable":
        return {
            "command": "run",
            "family": data["family"],
            "provider_id": provider_id,
            "provider_kind": row["provider_kind"],
            "current_status": row["current_status"],
            "terminal_outcome": "planned_only",
            "rejection_reason": (
                "P-ENG-01 only actualizes inventory-only provider rows; "
                "bounded provider admission remains a later/user-spec-required gate"
            ),
            "world_semantics_owner": data["world_semantics_owner"],
            "default_native_execution_policy": data["default_native_execution_policy"],
            "default_wasm_execution_policy": data["default_wasm_execution_policy"],
            "stop_lines": list(STOP_LINES),
            "row": realized,
        }
    raise NotImplementedError("executable provider rows are introduced after P-ENG-01")


def check_all() -> dict[str, Any]:
    status = matrix()
    failed = [error["provider_id"] for error in status["validation_errors"]]
    planned = list(status["planned_only_rows"])
    passed = list(status["executable_rows"])
    return {
        "command": "check-all",
        "family": status["family"],
        "provider_root": status["provider_root"],
        "matrix_path": status["matrix_path"],
        "provider_count": status["provider_count"],
        "planned": planned,
        "passed": passed,
        "failed": failed,
        "matrix_status": status["matrix_status"],
        "world_semantics_owner": status["world_semantics_owner"],
        "default_native_execution_policy": status["default_native_execution_policy"],
        "default_wasm_execution_policy": status["default_wasm_execution_policy"],
        "workflow_ready": False,
        "validation_errors": status["validation_errors"],
    }


def closeout() -> dict[str, Any]:
    status = matrix()
    return {
        "command": "closeout",
        "family": status["family"],
        "provider_root": status["provider_root"],
        "matrix_path": status["matrix_path"],
        "provider_ids": [row["provider_id"] for row in status["rows"]],
        "world_semantics_owner": status["world_semantics_owner"],
        "packet_ffi_transport_split": status["packet_ffi_transport_split"],
        "default_native_execution_policy": status["default_native_execution_policy"],
        "default_wasm_execution_policy": status["default_wasm_execution_policy"],
        "workflow_ready": False,
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
        "validation_errors": status["validation_errors"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["PLANNED PROVIDER INVENTORY"]
        for row in payload:
            lines.append(
                f"- {row['provider_id']} [{row['current_status']}] -> {row['representative_source']}"
            )
        return "\n".join(lines)

    command = payload.get("command")
    if command == "matrix":
        return "\n".join(
            [
                "MATRIX SUMMARY",
                f"provider root: {payload['provider_root']}",
                f"providers: {payload['provider_count']}",
                f"planned-only: {payload['planned_count']}",
                f"executable: {payload['executable_count']}",
                (
                    "default execution gating: "
                    f"native={payload['default_native_execution_policy']}, "
                    f"wasm={payload['default_wasm_execution_policy']}"
                ),
            ]
        )
    if command == "run":
        return "\n".join(
            [
                "RUN SUMMARY",
                f"provider: {payload['provider_id']}",
                f"status: {payload['current_status']}",
                f"outcome: {payload['terminal_outcome']}",
                f"reason: {payload['rejection_reason']}",
            ]
        )
    if command == "check-all":
        return "\n".join(
            [
                "CHECK-ALL SUMMARY",
                f"provider count: {payload['provider_count']}",
                f"planned-only: {len(payload['planned'])}",
                "planned ids: " + ", ".join(payload["planned"]),
                f"failed rows: {len(payload['failed'])}",
            ]
        )
    if command == "closeout":
        return "\n".join(
            [
                "CLOSEOUT SUMMARY",
                f"provider root: {payload['provider_root']}",
                "provider ids: " + ", ".join(payload["provider_ids"]),
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
    run_parser.add_argument("provider_id")
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
        payload = list_providers()
    elif args.command == "matrix":
        payload = matrix()
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "closeout":
        payload = closeout()
    else:
        payload = run_provider(args.provider_id)

    _print(payload, args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
