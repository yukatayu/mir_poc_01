#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SURFACE_ROOT = REPO_ROOT / "samples" / "full-system-v1-surface"
SYNTAX_ROOT = SURFACE_ROOT / "syntax"
SYNTAX_MATRIX_PATH = SYNTAX_ROOT / "matrix.json"
INDEXED_STATE_ROOT = SURFACE_ROOT / "indexed-state"
INDEXED_STATE_MATRIX_PATH = INDEXED_STATE_ROOT / "matrix.json"
ELABORATION_ROOT = SURFACE_ROOT / "elaboration"
ELABORATION_MATRIX_PATH = ELABORATION_ROOT / "matrix.json"
ROLE_ADMISSION_ROOT = SURFACE_ROOT / "role-admission"
ROLE_ADMISSION_MATRIX_PATH = ROLE_ADMISSION_ROOT / "matrix.json"
SOURCE_PATCH_ROOT = SURFACE_ROOT / "source-patch"
SOURCE_PATCH_MATRIX_PATH = SOURCE_PATCH_ROOT / "matrix.json"
DEVTOOLS_ROOT = SURFACE_ROOT / "devtools"
DEVTOOLS_MATRIX_PATH = DEVTOOLS_ROOT / "matrix.json"
OPERATIONAL_ROOT = SURFACE_ROOT
OPERATIONAL_MATRIX_PATH = SURFACE_ROOT / "operational-matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final public grammar / ABI / SDK",
    "no final runtime execution or final source patch hot-plug ABI completion yet",
    "no production identity provider / hardware attestation / WAN admission",
    "no generated package artifact authority",
]
NON_CLAIMS = [
    "P-SURF-08 devtools diagnostics are static source/Core evidence only",
    "no runtime MessageEnvelope dispatch completion",
    "no final Surface devtools viewer or telemetry ABI completion",
    "no production identity provider or hardware attestation",
    "no distributed durable source patch migration",
    "no final Surface operational runtime or transport completion",
    "no general TypeMismatch typechecker discharge in the Surface alpha floor",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-ast --test surface_mir_parser -- --nocapture",
    "cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture",
    "cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture",
    "cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture",
    "cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture",
    "cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture",
    "python3 -m unittest scripts.tests.test_surface_mir_samples",
    "python3 scripts/surface_mir_samples.py matrix --format json",
    "python3 scripts/surface_mir_samples.py check-all --format json",
]


def _load_matrix(matrix_path: Path) -> dict[str, Any]:
    return json.loads(matrix_path.read_text(encoding="utf-8"))


def _matrix_specs() -> list[dict[str, Any]]:
    return [
        {
            "family_key": "syntax",
            "root": SYNTAX_ROOT,
            "matrix_path": SYNTAX_MATRIX_PATH,
            "runner": "parser",
            "data": _load_matrix(SYNTAX_MATRIX_PATH),
        },
        {
            "family_key": "indexed_state",
            "root": INDEXED_STATE_ROOT,
            "matrix_path": INDEXED_STATE_MATRIX_PATH,
            "runner": "indexed_state",
            "data": _load_matrix(INDEXED_STATE_MATRIX_PATH),
        },
        {
            "family_key": "elaboration",
            "root": ELABORATION_ROOT,
            "matrix_path": ELABORATION_MATRIX_PATH,
            "runner": "elaboration",
            "data": _load_matrix(ELABORATION_MATRIX_PATH),
        },
        {
            "family_key": "role_admission",
            "root": ROLE_ADMISSION_ROOT,
            "matrix_path": ROLE_ADMISSION_MATRIX_PATH,
            "runner": "role_admission",
            "data": _load_matrix(ROLE_ADMISSION_MATRIX_PATH),
        },
        {
            "family_key": "source_patch",
            "root": SOURCE_PATCH_ROOT,
            "matrix_path": SOURCE_PATCH_MATRIX_PATH,
            "runner": "source_patch",
            "data": _load_matrix(SOURCE_PATCH_MATRIX_PATH),
        },
        {
            "family_key": "devtools",
            "root": DEVTOOLS_ROOT,
            "matrix_path": DEVTOOLS_MATRIX_PATH,
            "runner": "devtools_bundle",
            "data": _load_matrix(DEVTOOLS_MATRIX_PATH),
        },
        {
            "family_key": "operational",
            "root": OPERATIONAL_ROOT,
            "matrix_path": OPERATIONAL_MATRIX_PATH,
            "runner": "operational_source",
            "data": _load_matrix(OPERATIONAL_MATRIX_PATH),
        },
    ]


def _row_source_path(root: Path, row: dict[str, Any]) -> Path:
    return root / row["source"]


def _row_expected_path(root: Path, row: dict[str, Any]) -> Path:
    return root / row["expected"]


def _row_patch_source_path(root: Path, row: dict[str, Any]) -> Path | None:
    patch_source = row.get("patch_source")
    if not patch_source:
        return None
    return root / patch_source


def validate_rows(spec: dict[str, Any]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    root = spec["root"]
    for row in spec["data"]["rows"]:
        root_path = root / row["root_name"]
        readme_path = root_path / "README.md"
        source_path = _row_source_path(root, row)
        expected_path = _row_expected_path(root, row)
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "family": spec["family_key"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not readme_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "family": spec["family_key"],
                    "kind": "missing_readme",
                    "detail": f"missing sample readme `{readme_path}`",
                }
            )
        if not source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "family": spec["family_key"],
                    "kind": "missing_source",
                    "detail": f"missing source `{source_path}`",
                }
            )
        if not expected_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "family": spec["family_key"],
                    "kind": "missing_expected",
                    "detail": f"missing expected projection `{expected_path}`",
                }
            )
        patch_source_path = _row_patch_source_path(root, row)
        if patch_source_path is not None and not patch_source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "family": spec["family_key"],
                    "kind": "missing_patch_source",
                    "detail": f"missing patch source `{patch_source_path}`",
                }
            )
    return errors


def _materialize_row(spec: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    materialized = {
        "sample_id": row["sample_id"],
        "family": spec["data"]["family"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "runner": row.get("runner", spec["runner"]),
        "source": str(_row_source_path(spec["root"], row).relative_to(REPO_ROOT)),
        "expected": str(_row_expected_path(spec["root"], row).relative_to(REPO_ROOT)),
    }
    patch_source_path = _row_patch_source_path(spec["root"], row)
    if patch_source_path is not None:
        materialized["patch_source"] = str(patch_source_path.relative_to(REPO_ROOT))
    return materialized


def list_samples() -> list[dict[str, Any]]:
    return [
        _materialize_row(spec, row)
        for spec in _matrix_specs()
        for row in spec["data"]["rows"]
    ]


def matrix() -> dict[str, Any]:
    specs = _matrix_specs()
    rows = [
        _materialize_row(spec, row)
        for spec in specs
        for row in spec["data"]["rows"]
    ]
    validation_errors = [
        error for spec in specs for error in validate_rows(spec)
    ]
    executable_rows = [
        row["sample_id"] for row in rows if row["current_status"] == "executable"
    ]
    return {
        "command": "matrix",
        "family": "surface_mir_alpha_source",
        "sample_root": str(SURFACE_ROOT.relative_to(REPO_ROOT)),
        "syntax_root": str(SYNTAX_ROOT.relative_to(REPO_ROOT)),
        "indexed_state_root": str(INDEXED_STATE_ROOT.relative_to(REPO_ROOT)),
        "elaboration_root": str(ELABORATION_ROOT.relative_to(REPO_ROOT)),
        "role_admission_root": str(ROLE_ADMISSION_ROOT.relative_to(REPO_ROOT)),
        "source_patch_root": str(SOURCE_PATCH_ROOT.relative_to(REPO_ROOT)),
        "devtools_root": str(DEVTOOLS_ROOT.relative_to(REPO_ROOT)),
        "operational_root": str(OPERATIONAL_ROOT.relative_to(REPO_ROOT)),
        "matrix_paths": [
            str(spec["matrix_path"].relative_to(REPO_ROOT)) for spec in specs
        ],
        "family_count": len(specs),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": {
            spec["data"]["family"]: spec["data"]["current_status"] for spec in specs
        },
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
    }


def _parse_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-ast",
            "--example",
            "surface_mir_alpha_parse",
            "--",
            str(path),
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
            f"surface parser example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _check_indexed_state_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-semantics",
            "--example",
            "surface_indexed_state_check",
            "--",
            str(path),
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
            f"surface indexed-state example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _elaborate_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-semantics",
            "--example",
            "surface_to_core_elaborate",
            "--",
            str(path),
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
            f"surface elaboration example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _check_role_admission_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-semantics",
            "--example",
            "surface_role_admission_check",
            "--",
            str(path),
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
            f"surface role-admission example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _patch_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mirrorea-cli",
            "--",
            "patch-source",
            "session#surface-sample",
            str(path),
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
            f"surface source-patch CLI did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _diagnostic_codes(payload: dict[str, Any]) -> list[str]:
    if payload.get("diagnostic_codes") is not None:
        return list(payload.get("diagnostic_codes") or [])
    return [row["code"] for row in payload.get("diagnostics") or []]


def _consistent_value(values: list[Any]) -> Any:
    if not values:
        return None
    first = values[0]
    if all(value == first for value in values):
        return first
    return {"conflicting_values": values}


def _check_operational_source(path: Path, required_checks: list[str]) -> dict[str, Any]:
    payloads: dict[str, dict[str, Any]] = {"parse": _parse_source(path)}
    if "indexed_state" in required_checks:
        payloads["indexed_state"] = _check_indexed_state_source(path)
    if "role_admission" in required_checks:
        payloads["role_admission"] = _check_role_admission_source(path)
    if "elaboration" in required_checks:
        payloads["elaboration"] = _elaborate_source(path)
    return {
        "surface_kind": "surface_operational_source_report",
        "required_checks": required_checks,
        "payloads": payloads,
        "returncode": 0
        if all(
            payloads[check].get("returncode") in {0, 2}
            for check in required_checks
        )
        else 2,
    }


def _check_devtools_bundle(
    path: Path,
    patch_path: Path | None,
    required_checks: list[str],
) -> dict[str, Any]:
    payloads: dict[str, dict[str, Any]] = {"parse": _parse_source(path)}
    if "indexed_state" in required_checks:
        payloads["indexed_state"] = _check_indexed_state_source(path)
    if "role_admission" in required_checks:
        payloads["role_admission"] = _check_role_admission_source(path)
    if "elaboration" in required_checks:
        payloads["elaboration"] = _elaborate_source(path)
    if "source_patch" in required_checks and patch_path is not None:
        payloads["source_patch"] = _patch_source(patch_path)
    return {
        "surface_kind": "surface_devtools_bundle_report",
        "required_checks": required_checks,
        "payloads": payloads,
        "returncode": 0
        if all(
            payloads.get(check, {}).get("returncode") in {0, 2}
            for check in required_checks
        )
        else 2,
    }


def _variant_name(payload: dict[str, Any]) -> str:
    return next(iter(payload.keys()))


def _init_kind(value: dict[str, Any] | None) -> str | None:
    if not value:
        return None
    kind = value.get("kind") or {}
    if not isinstance(kind, dict):
        return str(kind)
    return _variant_name(kind)


def _state_summaries(module: dict[str, Any]) -> list[dict[str, Any]]:
    summaries: list[dict[str, Any]] = []
    for block in module.get("place_blocks") or []:
        for item in block.get("items") or []:
            if "State" not in item:
                continue
            state = item["State"]
            index = state.get("index") or {}
            visible = state.get("visible") or {}
            summaries.append(
                {
                    "state_name": state["state_name"],
                    "owner_place": state["owner_place"],
                    "index_name": index.get("name"),
                    "key_type": index.get("key_type_text"),
                    "value_type": state["value_type_text"],
                    "init_kind": _init_kind(state.get("initial_value")),
                    "visible_fields": visible.get("fields") or [],
                }
            )
    return summaries


def _role_instance_summaries(module: dict[str, Any]) -> list[dict[str, Any]]:
    summaries: list[dict[str, Any]] = []
    for block in module.get("role_instance_blocks") or []:
        join_targets: list[str] = []
        when_events: list[str] = []
        failure_rows: list[list[str]] = []
        for when in block.get("whens") or []:
            when_events.append(when["event_name"])
            failure_rows.append(when.get("failure_row") or [])
            for stmt in when.get("body") or []:
                if "Join" not in stmt:
                    continue
                join = stmt["Join"]
                join_targets.append(
                    f"{join['target_place']} as {join['role_ref']} via {join['admission_place']}"
                )
        summaries.append(
            {
                "role_ref": block["role_ref"],
                "instance_ref": block["instance_ref"],
                "when_events": when_events,
                "failure_rows": failure_rows,
                "join_targets": join_targets,
            }
        )
    return summaries


def _payload_projection(payload: dict[str, Any]) -> dict[str, Any]:
    module = payload.get("module") or {}
    return {
        "accepted": payload.get("accepted"),
        "module_path": module.get("module_path"),
        "diagnostic_codes": _diagnostic_codes(payload),
        "role_names": [row["role_name"] for row in module.get("roles") or []],
        "place_names": [row["place_name"] for row in module.get("places") or []],
        "record_names": [row["record_name"] for row in module.get("records") or []],
        "place_block_refs": [
            row["place_ref"] for row in module.get("place_blocks") or []
        ],
        "state_summaries": _state_summaries(module),
        "role_instance_summaries": _role_instance_summaries(module),
        "canonical_place_scope_syntax": payload.get("canonical_place_scope_syntax"),
        "final_public_grammar_frozen": payload.get("final_public_grammar_frozen"),
    }


def _indexed_state_projection(payload: dict[str, Any]) -> dict[str, Any]:
    return {
        "accepted": payload.get("accepted"),
        "module_path": payload.get("module_path"),
        "diagnostic_codes": _diagnostic_codes(payload),
        "indexed_state_summaries": [
            {
                "state_name": row["state_name"],
                "owner_locus": row["owner_locus"],
                "key_name": row["key_name"],
                "keyspace_type": row["keyspace_type"],
                "value_type": row["value_type"],
                "visible_fields": row.get("visible_fields") or [],
                "authority_model": row["authority_model"],
            }
            for row in payload.get("indexed_states") or []
        ],
        "access_summaries": [
            {
                "state_name": row["state_name"],
                "owner_locus": row["owner_locus"],
                "access_locus": row["access_locus"],
                "key_expr": row["key_expr"],
                "access_kind": row["access_kind"],
                "accepted": row["accepted"],
                "reason_code": row.get("reason_code"),
                "key_authority_granted": row["key_authority_granted"],
            }
            for row in payload.get("access_checks") or []
        ],
        "source_authority": payload.get("source_authority"),
        "final_public_api_frozen": payload.get("final_public_api_frozen"),
    }


def _elaboration_projection(payload: dict[str, Any]) -> dict[str, Any]:
    core_ir = payload.get("core_ir") or {}
    return {
        "accepted": payload.get("accepted"),
        "module_path": payload.get("module_path"),
        "diagnostic_codes": _diagnostic_codes(payload),
        "transition_kinds": [
            row["kind"] for row in core_ir.get("transitions") or []
        ],
        "remote_request_summaries": [
            {
                "request_kind": row["request_kind"],
                "requester_locus": row["requester_locus"],
                "owner_locus": row["owner_locus"],
                "state_name": row["state_name"],
                "key_expr": row["key_expr"],
                "generated_from": row["generated_from"],
                "failure_row_complete": row["failure_row_complete"],
            }
            for row in core_ir.get("remote_requests") or []
        ],
        "message_envelope_summaries": [
            {
                "envelope_kind": row["envelope_kind"],
                "from_locus": row["from_locus"],
                "to_locus": row["to_locus"],
                "state_name": row["state_name"],
                "key_expr": row["key_expr"],
                "field_name": row.get("field_name"),
                "visibility_channel": row.get("visibility_channel"),
                "redaction_label": row["redaction_label"],
                "retention_scope": row["retention_scope"],
            }
            for row in core_ir.get("message_envelopes") or []
        ],
        "publication_summaries": [
            {
                "publisher_locus": row["publisher_locus"],
                "channel": row["channel"],
                "state_name": row["state_name"],
                "key_expr": row["key_expr"],
                "field_name": row.get("field_name"),
                "redaction_label": row["redaction_label"],
                "retention_scope": row["retention_scope"],
            }
            for row in core_ir.get("publications") or []
        ],
        "observation_summaries": [
            {
                "observer_locus": row["observer_locus"],
                "owner_locus": row["owner_locus"],
                "channel": row["channel"],
                "state_name": row["state_name"],
                "key_expr": row["key_expr"],
                "field_name": row.get("field_name"),
                "redaction_label": row["redaction_label"],
                "retention_scope": row["retention_scope"],
            }
            for row in core_ir.get("observations") or []
        ],
        "generated_edge_kinds": [
            row["edge_kind"] for row in core_ir.get("generated_edges") or []
        ],
        "source_span_entity_kinds": [
            row["entity_kind"] for row in core_ir.get("source_spans") or []
        ],
        "obligation_codes": [
            row["code"] for row in core_ir.get("obligations") or []
        ],
        "source_authority": payload.get("source_authority"),
        "final_public_api_frozen": payload.get("final_public_api_frozen"),
    }


def _role_admission_projection(payload: dict[str, Any]) -> dict[str, Any]:
    return {
        "accepted": payload.get("accepted"),
        "module_path": payload.get("module_path"),
        "diagnostic_codes": _diagnostic_codes(payload),
        "role_claim_summaries": [
            {
                "principal": row["principal"],
                "claimed_role": row["claimed_role"],
                "supported_features": row.get("supported_features") or [],
            }
            for row in payload.get("role_claims") or []
        ],
        "admission_request_summaries": [
            {
                "principal": row["principal"],
                "claimed_role": row["claimed_role"],
                "target_place": row["target_place"],
                "admission_locus": row["admission_locus"],
                "requested_capabilities": row.get("requested_capabilities") or [],
            }
            for row in payload.get("admission_requests") or []
        ],
        "admission_verdict_summaries": [
            {
                "verdict": row["verdict"],
                "principal": row["principal"],
                "admitted_role": row["admitted_role"],
                "target_place": row["target_place"],
                "membership_epoch": row["membership_epoch"],
                "member_incarnation": row["member_incarnation"],
                "granted_capabilities": row.get("granted_capabilities") or [],
                "admission_witness_ref": row["admission_witness_ref"],
            }
            for row in payload.get("admission_verdicts") or []
        ],
        "capability_grant_summaries": [
            {
                "principal": row["principal"],
                "role": row["role"],
                "target_place": row["target_place"],
                "capability": row["capability"],
                "authority_source": row["authority_source"],
                "admission_witness_ref": row["admission_witness_ref"],
            }
            for row in payload.get("capability_grants") or []
        ],
        "authority_check_summaries": [
            {
                "principal": row["principal"],
                "claimed_role": row["claimed_role"],
                "target_place": row["target_place"],
                "operation": row["operation"],
                "required_capability": row["required_capability"],
                "accepted": row["accepted"],
                "authority_source": row.get("authority_source"),
                "reason_code": row.get("reason_code"),
            }
            for row in payload.get("authority_checks") or []
        ],
        "stale_rejection_summaries": [
            {
                "principal": row["principal"],
                "claimed_role": row["claimed_role"],
                "target_place": row["target_place"],
                "reason_code": row["reason_code"],
            }
            for row in payload.get("stale_rejections") or []
        ],
        "hash_binding_summaries": [
            {
                "principal": row["principal"],
                "claimed_role": row["claimed_role"],
                "package_hash": row["package_hash"],
                "runtime_hash": row["runtime_hash"],
                "semantic_safety_proof": row["semantic_safety_proof"],
            }
            for row in payload.get("optional_hash_bindings") or []
        ],
        "obligation_codes": [
            row["code"] for row in payload.get("accepted_obligations") or []
        ],
        "source_authority": payload.get("source_authority"),
        "final_public_api_frozen": payload.get("final_public_api_frozen"),
    }


def _source_patch_projection(payload: dict[str, Any]) -> dict[str, Any]:
    compatibility = payload.get("compatibility") or {}
    verdict = payload.get("hotplug_verdict") or {}
    activation_cut = payload.get("activation_cut")
    return {
        "accepted": payload.get("accepted"),
        "module_path": payload.get("module_path"),
        "diagnostic_codes": payload.get("diagnostic_codes") or [],
        "stage_summaries": [
            {
                "stage": row["stage"],
                "accepted": row["accepted"],
                "diagnostic_codes": row.get("diagnostic_codes") or [],
            }
            for row in payload.get("stage_summaries") or []
        ],
        "state_addition_summaries": [
            {
                "owner_locus": row["owner_locus"],
                "state_name": row["state_name"],
                "keyspace_type": row.get("keyspace_type"),
                "value_type": row["value_type"],
                "visible_fields": row.get("visible_fields") or [],
                "initializer_present": row["initializer_present"],
            }
            for row in compatibility.get("state_additions") or []
        ],
        "hotplug_request_present": payload.get("hotplug_request") is not None,
        "hotplug_verdict_kind": verdict.get("verdict_kind"),
        "activation_cut_present": activation_cut is not None,
        "activation_cut_kind": (activation_cut or {}).get("cut_kind"),
        "runtime_mutation_applied": payload.get("runtime_mutation_applied"),
        "direct_eval_performed": payload.get("direct_eval_performed"),
        "core_ir_diff": compatibility.get("core_ir_diff") or {},
        "source_authority": payload.get("source_authority"),
        "final_public_api_frozen": payload.get("final_public_api_frozen"),
    }


def _single_payload_verification_report(payload: dict[str, Any]) -> dict[str, Any]:
    module = payload.get("module") or {}
    core_ir = payload.get("core_ir") or {}
    compatibility = payload.get("compatibility") or {}
    verdict = payload.get("hotplug_verdict") or {}
    activation_cut = payload.get("activation_cut")
    report: dict[str, Any] = {
        "surface_kind": payload.get("surface_kind"),
        "returncode": payload.get("returncode"),
        "accepted": payload.get("accepted"),
        "module_path": payload.get("module_path") or module.get("module_path"),
        "diagnostic_codes": _diagnostic_codes(payload),
        "source_authority": payload.get("source_authority"),
        "final_public_api_frozen": payload.get("final_public_api_frozen"),
        "canonical_place_scope_syntax": payload.get("canonical_place_scope_syntax"),
        "redacted": True,
    }
    if payload.get("indexed_states") is not None:
        report["indexed_state_count"] = len(payload.get("indexed_states") or [])
        report["indexed_state_names"] = [
            row["state_name"] for row in payload.get("indexed_states") or []
        ]
    if core_ir:
        report["core_ir_counts"] = {
            "transitions": len(core_ir.get("transitions") or []),
            "remote_requests": len(core_ir.get("remote_requests") or []),
            "message_envelopes": len(core_ir.get("message_envelopes") or []),
            "publications": len(core_ir.get("publications") or []),
            "observations": len(core_ir.get("observations") or []),
            "generated_edges": len(core_ir.get("generated_edges") or []),
            "source_spans": len(core_ir.get("source_spans") or []),
            "obligations": len(core_ir.get("obligations") or []),
        }
    if payload.get("role_claims") is not None:
        report["role_admission_counts"] = {
            "role_claims": len(payload.get("role_claims") or []),
            "admission_requests": len(payload.get("admission_requests") or []),
            "admission_verdicts": len(payload.get("admission_verdicts") or []),
            "capability_grants": len(payload.get("capability_grants") or []),
            "authority_checks": len(payload.get("authority_checks") or []),
            "stale_rejections": len(payload.get("stale_rejections") or []),
        }
    if payload.get("stage_summaries") is not None:
        report["source_patch_summary"] = {
            "stage_names": [
                row["stage"] for row in payload.get("stage_summaries") or []
            ],
            "hotplug_request_present": payload.get("hotplug_request") is not None,
            "hotplug_verdict_kind": verdict.get("verdict_kind"),
            "activation_cut_present": activation_cut is not None,
            "direct_eval_performed": payload.get("direct_eval_performed"),
            "core_ir_diff": compatibility.get("core_ir_diff") or {},
        }
    report["contains_sensitive_devtools_material"] = (
        _contains_sensitive_devtools_material(report)
    )
    return report


def _verification_report(payload: dict[str, Any]) -> dict[str, Any]:
    if "payloads" not in payload:
        return _single_payload_verification_report(payload)
    stage_reports = {
        check: _single_payload_verification_report(check_payload)
        for check, check_payload in (payload.get("payloads") or {}).items()
    }
    report = {
        "surface_kind": payload.get("surface_kind"),
        "returncode": payload.get("returncode"),
        "required_checks": payload.get("required_checks") or [],
        "stage_reports": stage_reports,
        "redacted": True,
    }
    report["contains_sensitive_devtools_material"] = (
        _contains_sensitive_devtools_material(report)
    )
    return report


def _operational_source_projection(
    payload: dict[str, Any],
    row: dict[str, Any],
) -> dict[str, Any]:
    required_checks = row.get("required_checks") or ["parse"]
    payloads = payload.get("payloads") or {}
    parse_payload = payloads.get("parse") or {}
    parse_projection = _payload_projection(parse_payload)
    indexed_payload = payloads.get("indexed_state") or {}
    role_payload = payloads.get("role_admission") or {}
    elaboration_payload = payloads.get("elaboration") or {}
    core_ir = elaboration_payload.get("core_ir") or {}
    stage_acceptance = {
        check: payloads.get(check, {}).get("accepted") for check in required_checks
    }
    diagnostic_codes: list[str] = []
    for check in required_checks:
        diagnostic_codes.extend(_diagnostic_codes(payloads.get(check) or {}))
    source_authorities = [
        check_payload.get("source_authority")
        for check_payload in payloads.values()
        if check_payload.get("source_authority") is not None
    ]
    final_public_api_values = [
        check_payload.get("final_public_api_frozen")
        for check_payload in payloads.values()
        if "final_public_api_frozen" in check_payload
    ]
    return {
        "accepted": all(stage_acceptance.values()),
        "operational_root": row.get("operational_root"),
        "required_checks": required_checks,
        "stage_acceptance": stage_acceptance,
        "module_path": parse_projection.get("module_path"),
        "diagnostic_codes": diagnostic_codes,
        "place_block_refs": parse_projection.get("place_block_refs") or [],
        "state_names": [
            row["state_name"] for row in indexed_payload.get("indexed_states") or []
        ],
        "state_owner_loci": [
            row["owner_locus"] for row in indexed_payload.get("indexed_states") or []
        ],
        "role_claim_count": len(role_payload.get("role_claims") or []),
        "admission_verdict_count": len(role_payload.get("admission_verdicts") or []),
        "accepted_authority_check_count": len(
            [
                check
                for check in role_payload.get("authority_checks") or []
                if check.get("accepted") is True
            ]
        ),
        "remote_request_count": len(core_ir.get("remote_requests") or []),
        "message_envelope_count": len(core_ir.get("message_envelopes") or []),
        "publication_count": len(core_ir.get("publications") or []),
        "observation_count": len(core_ir.get("observations") or []),
        "generated_edge_kinds": [
            edge["edge_kind"] for edge in core_ir.get("generated_edges") or []
        ],
        "source_authority": _consistent_value(source_authorities),
        "final_public_api_frozen": _consistent_value(final_public_api_values),
    }


DEVTOOLS_PANEL_IDS = [
    "surface_source",
    "generated_core_ir",
    "indexed_state_map",
    "generated_communication",
    "role_admission",
    "patch_lifecycle",
    "source_spans",
]

DEVTOOLS_SENSITIVE_KEYS = {
    "activation_cut",
    "auth_evidence_ref",
    "capability_frontier_ref",
    "capability_refs",
    "hotplug_request",
    "membership_frontier_ref",
    "required_capability_witness_refs",
    "required_membership_witness_refs",
    "witness_refs",
}
DEVTOOLS_SENSITIVE_STRING_MARKERS = {
    "admission-witness-",
    "auth-evidence-",
    "capability-frontier-",
    "membership-frontier-",
    "private_token",
    "witness-",
}


def _contains_sensitive_devtools_material(value: Any) -> bool:
    if isinstance(value, dict):
        return any(
            key in DEVTOOLS_SENSITIVE_KEYS
            or _contains_sensitive_devtools_material(nested)
            for key, nested in value.items()
        )
    if isinstance(value, list):
        return any(_contains_sensitive_devtools_material(nested) for nested in value)
    if isinstance(value, str):
        return any(marker in value for marker in DEVTOOLS_SENSITIVE_STRING_MARKERS)
    return False


def _source_authority_fields(payloads: dict[str, dict[str, Any]]) -> dict[str, Any]:
    source_authorities = [
        check_payload.get("source_authority")
        for check_payload in payloads.values()
        if check_payload.get("source_authority") is not None
    ]
    final_public_api_values = [
        check_payload.get("final_public_api_frozen")
        for check_payload in payloads.values()
        if "final_public_api_frozen" in check_payload
    ]
    return {
        "source_authority": _consistent_value(source_authorities),
        "final_public_api_frozen": _consistent_value(final_public_api_values),
    }


def _devtools_bundle_projection(
    payload: dict[str, Any],
    row: dict[str, Any],
) -> dict[str, Any]:
    required_checks = row.get("required_checks") or ["parse"]
    payloads = payload.get("payloads") or {}
    parse_payload = payloads.get("parse") or {}
    parse_projection = _payload_projection(parse_payload)
    indexed_payload = payloads.get("indexed_state") or {}
    role_payload = payloads.get("role_admission") or {}
    elaboration_payload = payloads.get("elaboration") or {}
    source_patch_payload = payloads.get("source_patch") or {}
    core_ir = elaboration_payload.get("core_ir") or {}
    source_patch_projection = (
        _source_patch_projection(source_patch_payload)
        if source_patch_payload
        else {}
    )
    stage_acceptance = {
        check: payloads.get(check, {}).get("accepted") for check in required_checks
    }
    diagnostic_codes: list[str] = []
    for check in required_checks:
        diagnostic_codes.extend(_diagnostic_codes(payloads.get(check) or {}))
    source_fields = _source_authority_fields(payloads)
    source_span_entity_kinds = sorted(
        {
            row["entity_kind"]
            for row in core_ir.get("source_spans") or []
            if row.get("entity_kind")
        }
    )
    required_panel_ids = row.get("required_panels") or DEVTOOLS_PANEL_IDS
    indexed_state_names = [
        row["state_name"] for row in indexed_payload.get("indexed_states") or []
    ]
    indexed_state_owner_loci = [
        row["owner_locus"] for row in indexed_payload.get("indexed_states") or []
    ]
    panel_summaries = {
        "surface_source": {
            "module_path": parse_projection.get("module_path"),
            "place_block_refs": parse_projection.get("place_block_refs") or [],
        },
        "generated_core_ir": {
            "transition_kinds": [row["kind"] for row in core_ir.get("transitions") or []],
            "remote_request_count": len(core_ir.get("remote_requests") or []),
            "source_span_count": len(core_ir.get("source_spans") or []),
        },
        "indexed_state_map": {
            "state_names": indexed_state_names,
            "owner_loci": indexed_state_owner_loci,
            "semantic_backing": "indexed_state" in payloads
            and bool(indexed_payload.get("indexed_states") or []),
            "diagnostic_codes": _diagnostic_codes(indexed_payload),
        },
        "generated_communication": {
            "message_envelope_count": len(core_ir.get("message_envelopes") or []),
            "publication_count": len(core_ir.get("publications") or []),
            "observation_count": len(core_ir.get("observations") or []),
            "generated_edge_kinds": [
                row["edge_kind"] for row in core_ir.get("generated_edges") or []
            ],
        },
        "role_admission": {
            "role_claim_count": len(role_payload.get("role_claims") or []),
            "admission_verdict_count": len(role_payload.get("admission_verdicts") or []),
            "accepted_authority_check_count": len(
                [
                    check
                    for check in role_payload.get("authority_checks") or []
                    if check.get("accepted") is True
                ]
            ),
        },
        "patch_lifecycle": {
            "stage_names": [
                stage["stage"]
                for stage in source_patch_projection.get("stage_summaries") or []
            ],
            "hotplug_verdict_kind": source_patch_projection.get("hotplug_verdict_kind"),
            "activation_cut_present": source_patch_projection.get(
                "activation_cut_present"
            ),
            "direct_eval_performed": source_patch_projection.get(
                "direct_eval_performed"
            ),
        },
        "source_spans": {
            "source_span_count": len(core_ir.get("source_spans") or []),
            "entity_kinds": source_span_entity_kinds,
        },
    }
    panel_ids = [panel_id for panel_id in DEVTOOLS_PANEL_IDS if panel_id in panel_summaries]
    raw_private_payload_exposed = _contains_sensitive_devtools_material(panel_summaries)
    return {
        "accepted": all(stage_acceptance.values()),
        "required_checks": required_checks,
        "stage_acceptance": stage_acceptance,
        "panel_ids": panel_ids,
        "panel_count": len(panel_ids),
        "all_required_panels_present": set(required_panel_ids).issubset(set(panel_ids)),
        "observer_safe": not raw_private_payload_exposed,
        "final_public_viewer_frozen": False,
        "module_path": parse_projection.get("module_path"),
        "diagnostic_codes": diagnostic_codes,
        "surface_source_module": parse_projection.get("module_path"),
        "place_block_refs": parse_projection.get("place_block_refs") or [],
        "indexed_state_names": indexed_state_names,
        "indexed_state_owner_loci": indexed_state_owner_loci,
        "indexed_state_semantic_backing": panel_summaries["indexed_state_map"][
            "semantic_backing"
        ],
        "indexed_state_diagnostic_codes": panel_summaries["indexed_state_map"][
            "diagnostic_codes"
        ],
        "role_claim_count": len(role_payload.get("role_claims") or []),
        "admission_verdict_count": len(role_payload.get("admission_verdicts") or []),
        "accepted_authority_check_count": len(
            [
                check
                for check in role_payload.get("authority_checks") or []
                if check.get("accepted") is True
            ]
        ),
        "transition_kinds": [
            row["kind"] for row in core_ir.get("transitions") or []
        ],
        "remote_request_count": len(core_ir.get("remote_requests") or []),
        "message_envelope_count": len(core_ir.get("message_envelopes") or []),
        "publication_count": len(core_ir.get("publications") or []),
        "observation_count": len(core_ir.get("observations") or []),
        "generated_edge_kinds": [
            row["edge_kind"] for row in core_ir.get("generated_edges") or []
        ],
        "source_span_count": len(core_ir.get("source_spans") or []),
        "source_span_entity_kinds": source_span_entity_kinds,
        "patch_stage_names": [
            stage["stage"]
            for stage in source_patch_projection.get("stage_summaries") or []
        ],
        "patch_hotplug_verdict_kind": source_patch_projection.get(
            "hotplug_verdict_kind"
        ),
        "patch_activation_cut_present": source_patch_projection.get(
            "activation_cut_present"
        ),
        "patch_direct_eval_performed": source_patch_projection.get(
            "direct_eval_performed"
        ),
        "raw_private_payload_exposed": raw_private_payload_exposed,
        **source_fields,
    }


def _find_row(sample_id: str) -> tuple[dict[str, Any], dict[str, Any]]:
    for spec in _matrix_specs():
        row = next(
            (row for row in spec["data"]["rows"] if row["sample_id"] == sample_id),
            None,
        )
        if row is not None:
            return spec, row
    raise ValueError(f"unknown Surface Mir sample `{sample_id}`")


def run_sample(sample_id: str) -> dict[str, Any]:
    spec, row = _find_row(sample_id)
    runner = row.get("runner", spec["runner"])
    source_path = _row_source_path(spec["root"], row)
    if runner == "indexed_state":
        payload = _check_indexed_state_source(source_path)
        actual = _indexed_state_projection(payload)
    elif runner == "elaboration":
        payload = _elaborate_source(source_path)
        actual = _elaboration_projection(payload)
    elif runner == "role_admission":
        payload = _check_role_admission_source(source_path)
        actual = _role_admission_projection(payload)
    elif runner == "source_patch":
        payload = _patch_source(source_path)
        actual = _source_patch_projection(payload)
    elif runner == "operational_source":
        payload = _check_operational_source(
            source_path,
            row.get("required_checks") or ["parse"],
        )
        actual = _operational_source_projection(payload, row)
    elif runner == "devtools_bundle":
        payload = _check_devtools_bundle(
            source_path,
            _row_patch_source_path(spec["root"], row),
            row.get("required_checks") or ["parse"],
        )
        actual = _devtools_bundle_projection(payload, row)
    else:
        payload = _parse_source(source_path)
        actual = _payload_projection(payload)
    expected = json.loads(_row_expected_path(spec["root"], row).read_text(encoding="utf-8"))
    mismatches = [
        key for key, expected_value in expected.items() if actual.get(key) != expected_value
    ]
    if not expected:
        mismatches.append("expected_projection_empty")
    return {
        "command": "run",
        "family": spec["data"]["family"],
        "sample_id": sample_id,
        "runner": runner,
        "source": str(source_path.relative_to(REPO_ROOT)),
        "expected": expected,
        "actual": actual,
        "accepted": not mismatches,
        "mismatches": mismatches,
        "verification_report": _verification_report(payload),
    }


def check_all() -> dict[str, Any]:
    status = matrix()
    results = [run_sample(row["sample_id"]) for row in status["rows"]]
    failed = [
        result["sample_id"]
        for result in results
        if result["mismatches"]
        or result["verification_report"]["returncode"] not in {0, 2}
    ]
    if status["validation_errors"]:
        failed.extend(
            sorted(
                {
                    row["sample_id"] for row in status["rows"]
                    if any(
                        error["sample_id"] == row["sample_id"]
                        for error in status["validation_errors"]
                    )
                }
            )
        )
    passed = [result["sample_id"] for result in results if result["sample_id"] not in failed]
    return {
        "command": "check-all",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "sample_count": status["sample_count"],
        "passed": passed,
        "failed": failed,
        "workflow_ready": False,
        "results": results,
        "validation_errors": status["validation_errors"],
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
    }


def closeout() -> dict[str, Any]:
    status = check_all()
    return {
        "command": "closeout",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "sample_count": status["sample_count"],
        "passed": status["passed"],
        "failed": status["failed"],
        "validation_errors": status["validation_errors"],
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["SURFACE MIR SAMPLES"]
        for row in payload:
            lines.append(f"- {row['sample_id']} [{row['current_status']}] -> {row['source']}")
        return "\n".join(lines)

    command = payload.get("command")
    if command == "matrix":
        return "\n".join(
            [
                "MATRIX SUMMARY",
                f"sample root: {payload['sample_root']}",
                f"samples: {payload['sample_count']}",
                f"executable: {payload['executable_count']}",
                f"validation errors: {len(payload['validation_errors'])}",
            ]
        )
    if command == "run":
        return "\n".join(
            [
                f"SAMPLE {payload['sample_id']}",
                f"source: {payload['source']}",
                f"accepted: {payload['accepted']}",
                f"mismatches: {', '.join(payload['mismatches']) if payload['mismatches'] else 'none'}",
            ]
        )
    if command == "check-all":
        return "\n".join(
            [
                "CHECK-ALL",
                f"sample root: {payload['sample_root']}",
                f"passed: {len(payload['passed'])}",
                f"failed: {len(payload['failed'])}",
                f"validation errors: {len(payload['validation_errors'])}",
            ]
        )
    if command == "closeout":
        return "\n".join(
            [
                "CLOSEOUT",
                f"sample root: {payload['sample_root']}",
                f"sample count: {payload['sample_count']}",
                f"passed rows: {', '.join(payload['passed']) if payload['passed'] else 'none'}",
                f"failed rows: {', '.join(payload['failed']) if payload['failed'] else 'none'}",
                f"validation errors: {len(payload['validation_errors'])}",
            ]
        )
    return json.dumps(payload, indent=2, ensure_ascii=False)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=sorted(KNOWN_COMMANDS))
    parser.add_argument("sample_id", nargs="?")
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    args = parser.parse_args(argv)

    if args.command == "list":
        payload = list_samples()
    elif args.command == "matrix":
        payload = matrix()
    elif args.command == "run":
        if not args.sample_id:
            parser.error("run requires sample_id")
        payload = run_sample(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(payload))

    if args.command == "run":
        return 0 if payload["accepted"] else 2
    if args.command in {"check-all", "closeout"}:
        return 0 if not payload["failed"] and not payload["validation_errors"] else 2
    if args.command == "matrix":
        return 0 if not payload["validation_errors"] else 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
