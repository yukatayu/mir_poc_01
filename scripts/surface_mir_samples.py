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
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final public grammar / ABI / SDK",
    "no runtime execution or source patch hot-plug completion yet",
    "no production identity provider / hardware attestation / WAN admission",
    "no generated package artifact authority",
]
NON_CLAIMS = [
    "P-SURF-05 role admission capability grant is report-level Surface evidence only",
    "no runtime MessageEnvelope dispatch completion",
    "no production identity provider or hardware attestation",
    "source patch activation remains P-SURF-06",
    "no general TypeMismatch typechecker discharge in P-SURF-05",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-ast --test surface_mir_parser -- --nocapture",
    "cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture",
    "cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture",
    "cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture",
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
    ]


def _row_source_path(root: Path, row: dict[str, Any]) -> Path:
    return root / row["source"]


def _row_expected_path(root: Path, row: dict[str, Any]) -> Path:
    return root / row["expected"]


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
    return errors


def _materialize_row(spec: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    return {
        "sample_id": row["sample_id"],
        "family": spec["data"]["family"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "runner": row.get("runner", spec["runner"]),
        "source": str(_row_source_path(spec["root"], row).relative_to(REPO_ROOT)),
        "expected": str(_row_expected_path(spec["root"], row).relative_to(REPO_ROOT)),
    }


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
        "diagnostic_codes": [
            row["code"] for row in payload.get("diagnostics") or []
        ],
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
        "diagnostic_codes": [
            row["code"] for row in payload.get("diagnostics") or []
        ],
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
        "diagnostic_codes": [
            row["code"] for row in payload.get("diagnostics") or []
        ],
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
        "diagnostic_codes": [
            row["code"] for row in payload.get("diagnostics") or []
        ],
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
        "raw_parse_report": payload,
    }


def check_all() -> dict[str, Any]:
    status = matrix()
    results = [run_sample(row["sample_id"]) for row in status["rows"]]
    failed = [
        result["sample_id"]
        for result in results
        if result["mismatches"] or result["raw_parse_report"]["returncode"] not in {0, 2}
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
