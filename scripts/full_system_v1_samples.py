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
FULL_SYSTEM_V1_ROOT = REPO_ROOT / "samples" / "full-system-v1"
COMPUTATIONAL_ROOT = FULL_SYSTEM_V1_ROOT / "computational"
CHECKER_MATRIX_PATH = COMPUTATIONAL_ROOT / "typed-ir-matrix.json"
RUNTIME_MATRIX_PATH = COMPUTATIONAL_ROOT / "runtime-matrix.json"
OPERATIONAL_FAMILIES = [
    {
        "family_id": "world-core",
        "family_name": "full_system_v1_world_core",
        "package_kind": "world_core",
        "root": FULL_SYSTEM_V1_ROOT / "world-core",
        "matrix_path": FULL_SYSTEM_V1_ROOT / "world-core" / "matrix.json",
    },
    {
        "family_id": "membership-chat",
        "family_name": "full_system_v1_membership_chat",
        "package_kind": "membership_chat",
        "root": FULL_SYSTEM_V1_ROOT / "membership-chat",
        "matrix_path": FULL_SYSTEM_V1_ROOT / "membership-chat" / "matrix.json",
    },
    {
        "family_id": "sugoroku-world",
        "family_name": "full_system_v1_sugoroku_world",
        "package_kind": "sugoroku_world",
        "root": FULL_SYSTEM_V1_ROOT / "sugoroku-world",
        "matrix_path": FULL_SYSTEM_V1_ROOT / "sugoroku-world" / "matrix.json",
    },
    {
        "family_id": "portal-worldlink",
        "family_name": "full_system_v1_portal_worldlink",
        "package_kind": "portal_worldlink",
        "root": FULL_SYSTEM_V1_ROOT / "portal-worldlink",
        "matrix_path": FULL_SYSTEM_V1_ROOT / "portal-worldlink" / "matrix.json",
    },
    {
        "family_id": "two-shard-hard-boundary",
        "family_name": "full_system_v1_two_shard_hard_boundary",
        "package_kind": "two_shard_hard_boundary",
        "root": FULL_SYSTEM_V1_ROOT / "two-shard-hard-boundary",
        "matrix_path": FULL_SYSTEM_V1_ROOT / "two-shard-hard-boundary" / "matrix.json",
    },
    {
        "family_id": "gradient-observation",
        "family_name": "full_system_v1_gradient_observation",
        "package_kind": "gradient_observation",
        "root": FULL_SYSTEM_V1_ROOT / "gradient-observation",
        "matrix_path": FULL_SYSTEM_V1_ROOT / "gradient-observation" / "matrix.json",
    },
]
KNOWN_COMMANDS = {
    "list",
    "matrix",
    "run",
    "checker-check-all",
    "operational-list",
    "operational-matrix",
    "run-operational",
    "check-operational-all",
    "runtime-list",
    "runtime-matrix",
    "run-runtime",
    "check-runtime-all",
    "check-all",
    "closeout",
}
STOP_LINES = [
    "no final public grammar",
    "no final typed IR or public runtime API",
    "no final effect grammar or public effect ABI",
    "no package artifact generation yet",
]
NON_CLAIMS = [
    "alpha checker plus bounded source-first runtime only",
    "ambient effect/failure containment remains residual",
    "runtime cut/save semantics remain bounded local evidence only",
    "source-first operational suite is bounded local execution evidence only",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture",
    "cargo test -p mir-runtime --test full_system_v1_session -- --nocapture",
    "python3 -m unittest scripts.tests.test_full_system_v1_samples",
    "python3 scripts/full_system_v1_samples.py operational-matrix --format json",
    "python3 scripts/full_system_v1_samples.py check-all --format json",
]


def _load_matrix(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def _checker_matrix() -> dict[str, Any]:
    return _load_matrix(CHECKER_MATRIX_PATH)


def _runtime_matrix() -> dict[str, Any]:
    return _load_matrix(RUNTIME_MATRIX_PATH)


def _load_operational_matrix(config: dict[str, Any]) -> dict[str, Any]:
    return _load_matrix(config["matrix_path"])


def _row_source_path(row: dict[str, Any]) -> Path:
    return COMPUTATIONAL_ROOT / row["source"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return COMPUTATIONAL_ROOT / row["expected"]


def _operational_source_path(config: dict[str, Any], row: dict[str, Any]) -> Path:
    return config["root"] / row["source"]


def _operational_manifest_expected_path(config: dict[str, Any], row: dict[str, Any]) -> Path:
    return config["root"] / row["expected_manifest"]


def _operational_run_expected_path(config: dict[str, Any], row: dict[str, Any]) -> Path:
    return config["root"] / row["expected_run"]


def repo_cli_arg(path: Path) -> str:
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = COMPUTATIONAL_ROOT / row["root_name"]
        readme_path = root_path / "README.md"
        source_path = _row_source_path(row)
        expected_path = _row_expected_path(row)
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not readme_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_readme",
                    "detail": f"missing sample readme `{readme_path}`",
                }
            )
        if not source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_source",
                    "detail": f"missing source `{source_path}`",
                }
            )
        if not expected_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_expected",
                    "detail": f"missing expected file `{expected_path}`",
                }
            )
        if "entry_function" in row and not isinstance(row["entry_function"], str):
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_entry_function",
                    "detail": "runtime row must declare a string `entry_function`",
                }
            )
        if "input" in row and not isinstance(row["input"], int):
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_input",
                    "detail": "runtime row must declare an integer `input`",
                }
            )
    return errors


def operational_rows_with_configs() -> list[tuple[dict[str, Any], dict[str, Any]]]:
    rows: list[tuple[dict[str, Any], dict[str, Any]]] = []
    for config in OPERATIONAL_FAMILIES:
        data = _load_operational_matrix(config)
        for row in data["rows"]:
            rows.append((config, row))
    return rows


def validate_operational_rows(
    rows_with_configs: list[tuple[dict[str, Any], dict[str, Any]]]
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for config, row in rows_with_configs:
        family_readme_path = config["root"] / "README.md"
        root_path = config["root"] / row["root_name"]
        readme_path = root_path / "README.md"
        source_path = _operational_source_path(config, row)
        manifest_expected_path = _operational_manifest_expected_path(config, row)
        run_expected_path = _operational_run_expected_path(config, row)
        if not family_readme_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_family_readme",
                    "detail": f"missing family readme `{family_readme_path}`",
                }
            )
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not readme_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_readme",
                    "detail": f"missing sample readme `{readme_path}`",
                }
            )
        if not source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_source",
                    "detail": f"missing source `{source_path}`",
                }
            )
        if not manifest_expected_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_expected_manifest",
                    "detail": f"missing expected manifest `{manifest_expected_path}`",
                }
            )
        if not run_expected_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_expected_run",
                    "detail": f"missing expected runtime report `{run_expected_path}`",
                }
            )
        if "entry_function" in row and not isinstance(row["entry_function"], str):
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_entry_function",
                    "detail": "operational row must declare a string `entry_function`",
                }
            )
        if "input" in row and not isinstance(row["input"], int):
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "invalid_input",
                    "detail": "operational row must declare an integer `input`",
                }
            )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    payload = {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "source": str(_row_source_path(row).relative_to(REPO_ROOT)),
        "expected": str(_row_expected_path(row).relative_to(REPO_ROOT)),
    }
    if "entry_function" in row:
        payload["entry_function"] = row["entry_function"]
    if "input" in row:
        payload["input"] = row["input"]
    return payload


def _materialize_operational_row(config: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    payload = {
        "sample_id": row["sample_id"],
        "family_id": config["family_id"],
        "family_name": config["family_name"],
        "package_kind": config["package_kind"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "source": str(_operational_source_path(config, row).relative_to(REPO_ROOT)),
        "expected_manifest": str(
            _operational_manifest_expected_path(config, row).relative_to(REPO_ROOT)
        ),
        "expected_run": str(
            _operational_run_expected_path(config, row).relative_to(REPO_ROOT)
        ),
    }
    if "entry_function" in row:
        payload["entry_function"] = row["entry_function"]
    if "input" in row:
        payload["input"] = row["input"]
    return payload


def list_checker_samples() -> list[dict[str, Any]]:
    data = _checker_matrix()
    return [_materialize_row(row) for row in data["rows"]]


def list_runtime_samples() -> list[dict[str, Any]]:
    data = _runtime_matrix()
    return [_materialize_row(row) for row in data["rows"]]


def list_operational_samples() -> list[dict[str, Any]]:
    return [
        _materialize_operational_row(config, row)
        for config, row in operational_rows_with_configs()
    ]


def matrix() -> dict[str, Any]:
    data = _checker_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(data["rows"])
    executable_rows = [
        row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"
    ]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(COMPUTATIONAL_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(CHECKER_MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def runtime_matrix() -> dict[str, Any]:
    data = _runtime_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(data["rows"])
    executable_rows = [
        row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"
    ]
    return {
        "command": "runtime-matrix",
        "family": data["family"],
        "sample_root": str(COMPUTATIONAL_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(RUNTIME_MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": True,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def operational_matrix() -> dict[str, Any]:
    rows_with_configs = operational_rows_with_configs()
    rows = [
        _materialize_operational_row(config, row) for config, row in rows_with_configs
    ]
    validation_errors = validate_operational_rows(rows_with_configs)
    executable_rows = [
        row["sample_id"] for _, row in rows_with_configs if row["current_status"] == "executable"
    ]
    family_counts = []
    for config in OPERATIONAL_FAMILIES:
        data = _load_operational_matrix(config)
        family_counts.append(
            {
                "family_id": config["family_id"],
                "family_name": config["family_name"],
                "matrix_path": str(config["matrix_path"].relative_to(REPO_ROOT)),
                "sample_count": len(data["rows"]),
                "executable_count": len(
                    [row for row in data["rows"] if row["current_status"] == "executable"]
                ),
            }
        )
    return {
        "command": "operational-matrix",
        "family": "full_system_v1_source_operational_suite",
        "sample_root": str(FULL_SYSTEM_V1_ROOT.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "workflow_ready": False,
        "rows": rows,
        "family_counts": family_counts,
        "validation_errors": validation_errors,
    }


def _check_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-semantics",
            "--example",
            "full_system_v1_check",
            "--",
            repo_cli_arg(path),
            "--format",
            "json",
        ],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"full_system_v1_check example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _run_runtime_source(path: Path, entry_function: str, input_value: int) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_full_system_v1_session",
            "--",
            repo_cli_arg(path),
            "--entry",
            entry_function,
            "--input",
            str(input_value),
            "--format",
            "json",
        ],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"mir_full_system_v1_session example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _type_name(type_payload: Any) -> str | None:
    if type_payload is None:
        return None
    if isinstance(type_payload, str):
        return type_payload
    if isinstance(type_payload, dict):
        if "Named" in type_payload:
            return str(type_payload["Named"])
        if "FixedArray" in type_payload:
            inner = type_payload["FixedArray"]
            return f"[{_type_name(inner['element'])}; {inner['length']}]"
        if "Error" in type_payload:
            return "Error"
    return json.dumps(type_payload, sort_keys=True, ensure_ascii=False)


def _variant(payload: dict[str, Any]) -> tuple[str, Any]:
    return next(iter(payload.items()))


def _stmt_summary(stmt: dict[str, Any]) -> dict[str, Any]:
    stmt_name, stmt_payload = _variant(stmt)
    binding_type = None
    perform_effect = None
    contract_clause_kinds: list[str] = []
    if stmt_name == "Bind":
        binding_type = _type_name(stmt_payload.get("binding_type"))
        bind_name, bind_payload = _variant(stmt_payload["value"])
        if bind_name == "Perform":
            perform_effect = bind_payload["effect_name"]
        contract_clause_kinds = [
            clause["kind"] for clause in stmt_payload.get("contract_clauses") or []
        ]
    elif stmt_name == "Perform":
        perform_effect = stmt_payload["call"]["effect_name"]
        contract_clause_kinds = [
            clause["kind"] for clause in stmt_payload.get("contract_clauses") or []
        ]
    return {
        "kind": stmt_name,
        "binding_type": binding_type,
        "perform_effect": perform_effect,
        "contract_clause_kinds": contract_clause_kinds,
    }


def _repo_relative_path(path_text: str) -> str:
    path = Path(path_text)
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return path_text


def _payload_checker_projection(payload: dict[str, Any]) -> dict[str, Any]:
    module = payload.get("module") or {}
    transition_summaries = []
    for transition in module.get("transitions") or []:
        stmt_summaries = [_stmt_summary(stmt) for stmt in transition.get("body") or []]
        transition_summaries.append(
            {
                "transition_name": transition["transition_name"],
                "place_ref": transition["place_ref"],
                "required_capabilities": transition["required_capabilities"],
                "statement_kinds": [row["kind"] for row in stmt_summaries],
                "binding_types": [
                    row["binding_type"]
                    for row in stmt_summaries
                    if row["binding_type"] is not None
                ],
                "perform_effects": [
                    row["perform_effect"]
                    for row in stmt_summaries
                    if row["perform_effect"] is not None
                ],
                "contract_clause_kinds": [
                    kind
                    for row in stmt_summaries
                    for kind in row["contract_clause_kinds"]
                ],
            }
        )

    return {
        "accepted": payload.get("accepted"),
        "module_path": module.get("module_path"),
        "import_paths": [row["module_path"] for row in module.get("imports") or []],
        "resolved_paths": [
            _repo_relative_path(row["resolved_path"])
            for row in module.get("imports") or []
        ],
        "capability_names": [
            row["capability_name"] for row in module.get("capabilities") or []
        ],
        "record_summaries": [
            {
                "record_name": row["record_name"],
                "field_names": [field["field_name"] for field in row.get("fields") or []],
                "field_types": [
                    _type_name(field["field_type"]) for field in row.get("fields") or []
                ],
            }
            for row in module.get("records") or []
        ],
        "effect_summaries": [
            {
                "effect_name": row["effect_name"],
                "parameter_types": [
                    _type_name(parameter["param_type"])
                    for parameter in row.get("parameters") or []
                ],
                "output_type": _type_name((row.get("output") or {}).get("output_type")),
                "required_capabilities": row.get("required_capabilities") or [],
                "failure_row": row.get("failure_row") or [],
            }
            for row in module.get("effects") or []
        ],
        "function_summaries": [
            {
                "function_name": row["function_name"],
                "parameter_type": _type_name(row["parameter"]["param_type"]),
                "output_type": _type_name(row["output_type"]),
                "statement_kinds": [_variant(stmt)[0] for stmt in row.get("body") or []],
            }
            for row in module.get("functions") or []
        ],
        "transition_summaries": transition_summaries,
        "accepted_obligation_codes": [
            row["code"] for row in payload.get("accepted_obligations") or []
        ],
        "residual_obligation_codes": [
            row["code"] for row in payload.get("residual_obligations") or []
        ],
        "diagnostic_codes": [row["code"] for row in payload.get("diagnostics") or []],
    }


def _payload_runtime_projection(payload: dict[str, Any]) -> dict[str, Any]:
    runtime = payload.get("runtime") or {}
    traces = runtime.get("compute_trace") or []
    effect_session = runtime.get("effect_session") or {}
    return {
        "accepted": runtime.get("accepted"),
        "outcome": runtime.get("outcome"),
        "entry_kind": runtime.get("entry_kind"),
        "entry_function": payload.get("entry_function"),
        "output_summary": (runtime.get("output") or {}).get("summary"),
        "diagnostic_codes": [row["code"] for row in runtime.get("diagnostics") or []],
        "runtime_rejection_code": (runtime.get("runtime_rejection") or {}).get("code"),
        "program_module_paths": runtime.get("program_module_paths") or [],
        "trace_functions": [
            f"{row['module_path']}.{row['function_id']}" for row in traces
        ],
        "trace_branch_taken": [
            branch for row in traces for branch in row.get("branch_taken") or []
        ],
        "trace_output_summaries": [
            (row.get("outputs") or {}).get("summary") if row.get("outputs") else None
            for row in traces
        ],
        "trace_rejection_codes": [
            (row.get("rejected_reason") or {}).get("code")
            if row.get("rejected_reason")
            else None
            for row in traces
        ],
        "trace_event_kinds": [
            event["kind"] for row in traces for event in row.get("events") or []
        ],
        "effect_session": {
            "host_input_remaining": effect_session.get("host_input_remaining"),
            "host_output_summaries": [
                row["summary"] for row in effect_session.get("host_output") or []
            ],
            "published_channels": effect_session.get("published_channels") or [],
            "observed_channels": effect_session.get("observed_channels") or [],
            "witness_refs": effect_session.get("witness_refs") or [],
            "handoff_refs": effect_session.get("handoff_refs") or [],
            "accepted_cuts": effect_session.get("accepted_cuts") or [],
            "all_places_sealed": effect_session.get("all_places_sealed"),
            "no_in_flight": effect_session.get("no_in_flight"),
            "no_post_cut_send": effect_session.get("no_post_cut_send"),
        },
        "observer_safe_summary": payload.get("observer_safe_summary"),
    }


def _run_checker_row(row: dict[str, Any]) -> dict[str, Any]:
    source_path = _row_source_path(row)
    expected_path = _row_expected_path(row)
    actual_payload = _check_source(source_path)
    actual = _payload_checker_projection(actual_payload)
    expected = json.loads(expected_path.read_text(encoding="utf-8"))
    returncode = actual_payload["returncode"]
    returncode_expected = 0 if actual.get("accepted") else 2
    returncode_passed = returncode == returncode_expected
    return {
        "sample_id": row["sample_id"],
        "source": str(source_path.relative_to(REPO_ROOT)),
        "expected_path": str(expected_path.relative_to(REPO_ROOT)),
        "accepted": bool(actual.get("accepted")),
        "returncode": returncode,
        "returncode_expected": returncode_expected,
        "returncode_passed": returncode_passed,
        "passed": actual == expected and returncode_passed,
        "actual": actual,
        "expected": expected,
    }


def _run_runtime_row(row: dict[str, Any]) -> dict[str, Any]:
    source_path = _row_source_path(row)
    expected_path = _row_expected_path(row)
    actual_payload = _run_runtime_source(
        source_path, row["entry_function"], row["input"]
    )
    actual = _payload_runtime_projection(actual_payload)
    expected = json.loads(expected_path.read_text(encoding="utf-8"))
    returncode = actual_payload["returncode"]
    returncode_expected = 0 if actual.get("accepted") else 2
    returncode_passed = returncode == returncode_expected
    return {
        "sample_id": row["sample_id"],
        "source": str(source_path.relative_to(REPO_ROOT)),
        "entry_function": row["entry_function"],
        "input": row["input"],
        "expected_path": str(expected_path.relative_to(REPO_ROOT)),
        "accepted": bool(actual.get("accepted")),
        "returncode": returncode,
        "returncode_expected": returncode_expected,
        "returncode_passed": returncode_passed,
        "passed": actual == expected and returncode_passed,
        "actual": actual,
        "expected": expected,
    }


def _operational_manifest_projection(
    row: dict[str, Any], source_path: Path, payload: dict[str, Any]
) -> dict[str, Any]:
    checker = _payload_checker_projection(payload)
    return {
        "schema_version": "full-system-v1-generated-package-manifest-v0",
        "sample_id": row["sample_id"],
        "package_kind": row["package_kind"],
        "semantic_source_of_truth": str(source_path.relative_to(REPO_ROOT)),
        "module_path": checker["module_path"],
        "import_paths": checker["import_paths"],
        "resolved_paths": checker["resolved_paths"],
        "capability_names": checker["capability_names"],
        "record_summaries": checker["record_summaries"],
        "effect_summaries": checker["effect_summaries"],
        "function_summaries": checker["function_summaries"],
        "transition_summaries": checker["transition_summaries"],
        "accepted_obligation_codes": checker["accepted_obligation_codes"],
        "residual_obligation_codes": checker["residual_obligation_codes"],
        "diagnostic_codes": checker["diagnostic_codes"],
        "compatibility_artifact_only": True,
        "final_public_api_frozen": False,
    }


def _run_operational_row(config: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    source_path = _operational_source_path(config, row)
    manifest_expected_path = _operational_manifest_expected_path(config, row)
    run_expected_path = _operational_run_expected_path(config, row)
    check_payload = _check_source(source_path)
    runtime_payload = _run_runtime_source(source_path, row["entry_function"], row["input"])
    manifest_row = dict(row)
    manifest_row["package_kind"] = config["package_kind"]
    manifest_actual = _operational_manifest_projection(
        manifest_row, source_path, check_payload
    )
    runtime_actual = _payload_runtime_projection(runtime_payload)
    manifest_expected = json.loads(manifest_expected_path.read_text(encoding="utf-8"))
    runtime_expected = json.loads(run_expected_path.read_text(encoding="utf-8"))
    manifest_passed = manifest_actual == manifest_expected
    runtime_passed = runtime_actual == runtime_expected
    manifest_returncode = check_payload["returncode"]
    runtime_returncode = runtime_payload["returncode"]
    manifest_returncode_passed = manifest_returncode == 0
    runtime_returncode_expected = 0 if runtime_actual.get("accepted") else 2
    runtime_returncode_passed = runtime_returncode == runtime_returncode_expected
    return {
        "sample_id": row["sample_id"],
        "family_id": config["family_id"],
        "family_name": config["family_name"],
        "package_kind": config["package_kind"],
        "source": str(source_path.relative_to(REPO_ROOT)),
        "entry_function": row["entry_function"],
        "input": row["input"],
        "accepted": bool(runtime_actual.get("accepted")),
        "manifest_expected_path": str(manifest_expected_path.relative_to(REPO_ROOT)),
        "run_expected_path": str(run_expected_path.relative_to(REPO_ROOT)),
        "manifest_returncode": manifest_returncode,
        "manifest_returncode_passed": manifest_returncode_passed,
        "runtime_returncode": runtime_returncode,
        "runtime_returncode_expected": runtime_returncode_expected,
        "runtime_returncode_passed": runtime_returncode_passed,
        "manifest_passed": manifest_passed,
        "runtime_passed": runtime_passed,
        "passed": (
            manifest_passed
            and runtime_passed
            and manifest_returncode_passed
            and runtime_returncode_passed
        ),
        "manifest_actual": manifest_actual,
        "manifest_expected": manifest_expected,
        "runtime_actual": runtime_actual,
        "runtime_expected": runtime_expected,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _checker_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise KeyError(sample_id)
    return _run_checker_row(row)


def run_runtime_sample(sample_id: str) -> dict[str, Any]:
    data = _runtime_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise KeyError(sample_id)
    return _run_runtime_row(row)


def run_operational_sample(sample_id: str) -> dict[str, Any]:
    for config in OPERATIONAL_FAMILIES:
        data = _load_operational_matrix(config)
        row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
        if row is not None:
            return _run_operational_row(config, row)
    raise KeyError(sample_id)


def checker_check_all() -> dict[str, Any]:
    data = _checker_matrix()
    validation_errors = validate_rows(data["rows"])
    if validation_errors:
        return {
            "command": "checker-check-all",
            "family": data["family"],
            "failed": [],
            "passed": [],
            "validation_errors": validation_errors,
        }

    passed = []
    failed = []
    for row in data["rows"]:
        result = _run_checker_row(row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)

    return {
        "command": "checker-check-all",
        "family": data["family"],
        "passed": passed,
        "failed": failed,
        "validation_errors": validation_errors,
    }


def runtime_check_all() -> dict[str, Any]:
    data = _runtime_matrix()
    validation_errors = validate_rows(data["rows"])
    if validation_errors:
        return {
            "command": "check-runtime-all",
            "family": data["family"],
            "failed": [],
            "passed": [],
            "validation_errors": validation_errors,
        }

    passed = []
    failed = []
    for row in data["rows"]:
        result = _run_runtime_row(row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)

    return {
        "command": "check-runtime-all",
        "family": data["family"],
        "passed": passed,
        "failed": failed,
        "validation_errors": validation_errors,
    }


def operational_check_all() -> dict[str, Any]:
    rows_with_configs = operational_rows_with_configs()
    validation_errors = validate_operational_rows(rows_with_configs)
    if validation_errors:
        return {
            "command": "check-operational-all",
            "family": "full_system_v1_source_operational_suite",
            "failed": [],
            "passed": [],
            "validation_errors": validation_errors,
        }

    passed = []
    failed = []
    for config, row in rows_with_configs:
        result = _run_operational_row(config, row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)

    return {
        "command": "check-operational-all",
        "family": "full_system_v1_source_operational_suite",
        "passed": passed,
        "failed": failed,
        "validation_errors": validation_errors,
    }


def check_all() -> dict[str, Any]:
    checker_summary = checker_check_all()
    runtime_summary = runtime_check_all()
    operational_summary = operational_check_all()
    return {
        "command": "check-all",
        "passed": (
            checker_summary["passed"]
            + runtime_summary["passed"]
            + operational_summary["passed"]
        ),
        "failed": (
            checker_summary["failed"]
            + runtime_summary["failed"]
            + operational_summary["failed"]
        ),
        "validation_errors": (
            checker_summary["validation_errors"]
            + runtime_summary["validation_errors"]
            + operational_summary["validation_errors"]
        ),
        "checker": checker_summary,
        "runtime": runtime_summary,
        "operational": operational_summary,
    }


def closeout() -> dict[str, Any]:
    summary = check_all()
    return {
        "command": "closeout",
        "passed": summary["passed"],
        "failed": summary["failed"],
        "validation_errors": summary["validation_errors"],
        "checker": summary["checker"],
        "runtime": summary["runtime"],
        "operational": summary["operational"],
        "validation_floor": VALIDATION_FLOOR,
        "stop_lines": STOP_LINES,
        "non_claims": NON_CLAIMS,
    }


def _emit(payload: dict[str, Any], output_format: str) -> None:
    if output_format == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
        return
    print(json.dumps(payload, indent=2, ensure_ascii=False))


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Run Full System V1 checker, runtime, and operational source samples."
    )
    parser.add_argument("command", choices=sorted(KNOWN_COMMANDS))
    parser.add_argument("sample_id", nargs="?")
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    args = parser.parse_args(argv)

    try:
        if args.command == "list":
            payload = {"command": "list", "rows": list_checker_samples()}
            _emit(payload, args.format)
            return 0
        if args.command == "operational-list":
            payload = {"command": "operational-list", "rows": list_operational_samples()}
            _emit(payload, args.format)
            return 0
        if args.command == "runtime-list":
            payload = {"command": "runtime-list", "rows": list_runtime_samples()}
            _emit(payload, args.format)
            return 0
        if args.command == "matrix":
            payload = matrix()
            _emit(payload, args.format)
            return 0 if not payload["validation_errors"] else 2
        if args.command == "operational-matrix":
            payload = operational_matrix()
            _emit(payload, args.format)
            return 0 if not payload["validation_errors"] else 2
        if args.command == "runtime-matrix":
            payload = runtime_matrix()
            _emit(payload, args.format)
            return 0 if not payload["validation_errors"] else 2
        if args.command == "run":
            if args.sample_id is None:
                raise SystemExit("sample_id is required for run")
            payload = run_sample(args.sample_id)
            _emit(payload, args.format)
            return 0 if payload["passed"] else 2
        if args.command == "run-operational":
            if args.sample_id is None:
                raise SystemExit("sample_id is required for run-operational")
            payload = run_operational_sample(args.sample_id)
            _emit(payload, args.format)
            return 0 if payload["passed"] else 2
        if args.command == "run-runtime":
            if args.sample_id is None:
                raise SystemExit("sample_id is required for run-runtime")
            payload = run_runtime_sample(args.sample_id)
            _emit(payload, args.format)
            return 0 if payload["passed"] else 2
        if args.command == "checker-check-all":
            payload = checker_check_all()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
        if args.command == "check-operational-all":
            payload = operational_check_all()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
        if args.command == "check-runtime-all":
            payload = runtime_check_all()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
        if args.command == "check-all":
            payload = check_all()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
        if args.command == "closeout":
            payload = closeout()
            _emit(payload, args.format)
            return 0 if not payload["failed"] and not payload["validation_errors"] else 2
    except KeyError as error:
        _emit({"command": args.command, "status": "error", "sample_id": str(error)}, args.format)
        return 2
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
