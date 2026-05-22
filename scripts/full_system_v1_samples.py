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
SAMPLE_ROOT = REPO_ROOT / "samples" / "full-system-v1" / "computational"
MATRIX_PATH = SAMPLE_ROOT / "typed-ir-matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final public grammar",
    "no final typed IR or public checker API",
    "no interpreter execution yet",
    "no package artifact generation yet",
]
NON_CLAIMS = [
    "alpha checker only",
    "ambient effect/failure containment remains residual",
    "product alpha-1 package workflow remains separate",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture",
    "python3 -m unittest scripts.tests.test_full_system_v1_samples",
    "python3 scripts/full_system_v1_samples.py matrix --format json",
    "python3 scripts/full_system_v1_samples.py check-all --format json",
]


def _load_matrix() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _row_source_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["source"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["expected"]


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = SAMPLE_ROOT / row["root_name"]
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
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "source": str(_row_source_path(row).relative_to(REPO_ROOT)),
        "expected": str(_row_expected_path(row).relative_to(REPO_ROOT)),
    }


def list_samples() -> list[dict[str, Any]]:
    data = _load_matrix()
    return [_materialize_row(row) for row in data["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(data["rows"])
    executable_rows = [row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
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
            f"full_system_v1_check example did not return JSON for `{path}`: {completed.stderr}"
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
        contract_clause_kinds = [clause["kind"] for clause in stmt_payload.get("contract_clauses") or []]
    elif stmt_name == "Perform":
        perform_effect = stmt_payload["call"]["effect_name"]
        contract_clause_kinds = [clause["kind"] for clause in stmt_payload.get("contract_clauses") or []]
    return {
        "kind": stmt_name,
        "binding_type": binding_type,
        "perform_effect": perform_effect,
        "contract_clause_kinds": contract_clause_kinds,
    }


def _payload_projection(payload: dict[str, Any]) -> dict[str, Any]:
    module = payload.get("module") or {}
    transition_summaries = []
    for transition in module.get("transitions") or []:
        stmt_summaries = [_stmt_summary(stmt) for stmt in transition.get("body") or []]
        transition_summaries.append(
            {
                "transition_name": transition["transition_name"],
                "required_capabilities": transition["required_capabilities"],
                "statement_kinds": [row["kind"] for row in stmt_summaries],
                "binding_types": [row["binding_type"] for row in stmt_summaries if row["binding_type"] is not None],
                "perform_effects": [row["perform_effect"] for row in stmt_summaries if row["perform_effect"] is not None],
                "contract_clause_kinds": [
                    kind for row in stmt_summaries for kind in row["contract_clause_kinds"]
                ],
            }
        )

    return {
        "accepted": payload.get("accepted"),
        "module_path": module.get("module_path"),
        "import_paths": [row["module_path"] for row in module.get("imports") or []],
        "resolved_paths": [
            _repo_relative_path(row["resolved_path"]) for row in module.get("imports") or []
        ],
        "capability_names": [row["capability_name"] for row in module.get("capabilities") or []],
        "record_summaries": [
            {
                "record_name": row["record_name"],
                "field_names": [field["field_name"] for field in row.get("fields") or []],
                "field_types": [_type_name(field["field_type"]) for field in row.get("fields") or []],
            }
            for row in module.get("records") or []
        ],
        "effect_summaries": [
            {
                "effect_name": row["effect_name"],
                "parameter_types": [
                    _type_name(parameter["param_type"]) for parameter in row.get("parameters") or []
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


def _repo_relative_path(path_text: str) -> str:
    path = Path(path_text)
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return path_text


def _run_row(row: dict[str, Any]) -> dict[str, Any]:
    source_path = _row_source_path(row)
    expected_path = _row_expected_path(row)
    actual_payload = _check_source(source_path)
    actual = _payload_projection(actual_payload)
    expected = json.loads(expected_path.read_text(encoding="utf-8"))
    passed = actual == expected
    return {
        "sample_id": row["sample_id"],
        "source": str(source_path.relative_to(REPO_ROOT)),
        "expected_path": str(expected_path.relative_to(REPO_ROOT)),
        "accepted": bool(actual.get("accepted")),
        "returncode": actual_payload["returncode"],
        "passed": passed,
        "actual": actual,
        "expected": expected,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise KeyError(sample_id)
    return _run_row(row)


def check_all() -> dict[str, Any]:
    data = _load_matrix()
    validation_errors = validate_rows(data["rows"])
    if validation_errors:
        return {
            "command": "check-all",
            "family": data["family"],
            "failed": [],
            "passed": [],
            "validation_errors": validation_errors,
        }

    passed = []
    failed = []
    for row in data["rows"]:
        result = _run_row(row)
        if result["passed"]:
            passed.append(result["sample_id"])
        else:
            failed.append(result)
    return {
        "command": "check-all",
        "family": data["family"],
        "passed": passed,
        "failed": failed,
        "validation_errors": validation_errors,
    }


def closeout() -> dict[str, Any]:
    summary = check_all()
    return {
        "command": "closeout",
        "family": summary["family"],
        "passed": summary["passed"],
        "failed": summary["failed"],
        "validation_errors": summary["validation_errors"],
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
    parser = argparse.ArgumentParser(description="Run Full System V1 typed checker samples.")
    parser.add_argument("command", choices=sorted(KNOWN_COMMANDS))
    parser.add_argument("sample_id", nargs="?")
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    args = parser.parse_args(argv)

    try:
        if args.command == "list":
            payload = {"command": "list", "rows": list_samples()}
            _emit(payload, args.format)
            return 0
        if args.command == "matrix":
            payload = matrix()
            _emit(payload, args.format)
            return 0 if not payload["validation_errors"] else 2
        if args.command == "run":
            if args.sample_id is None:
                raise SystemExit("sample_id is required for run")
            payload = run_sample(args.sample_id)
            _emit(payload, args.format)
            return 0 if payload["passed"] else 2
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
