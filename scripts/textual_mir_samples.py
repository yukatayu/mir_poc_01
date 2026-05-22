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
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final public grammar",
    "no typed IR completion yet",
    "no runtime execution yet",
    "no generated package artifact yet",
]
NON_CLAIMS = [
    "alpha parser only",
    "product alpha-1 package workflow remains separate",
    "package.mir.json compatibility generation remains later work",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-ast --test textual_mir_alpha -- --nocapture",
    "python3 -m unittest scripts.tests.test_textual_mir_samples",
    "python3 scripts/textual_mir_samples.py matrix --format json",
    "python3 scripts/textual_mir_samples.py check-all --format json",
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


def _parse_source(path: Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-ast",
            "--example",
            "textual_mir_alpha_parse",
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
            f"parser example did not return JSON for `{path}`: {completed.stderr}"
        ) from error
    payload["returncode"] = completed.returncode
    return payload


def _payload_projection(payload: dict[str, Any]) -> dict[str, Any]:
    module = payload.get("module") or {}
    items = module.get("items") or []
    stmt_span_markers: list[str] = []
    contract_span_markers: list[str] = []
    expr_span_markers: list[str] = []

    def _span_marker(span: dict[str, Any] | None) -> str | None:
        if not span:
            return None
        return (
            f"{span.get('line')}:{span.get('column')}"
            f"[{span.get('start')}..{span.get('end')}]"
        )

    def _type_name(type_payload: Any) -> str:
        if isinstance(type_payload, str):
            return type_payload
        if isinstance(type_payload, dict):
            if "Named" in type_payload:
                return str(type_payload["Named"])
            if "FixedArray" in type_payload:
                inner = type_payload["FixedArray"]
                return f"[{_type_name(inner['element'])}; {inner['length']}]"
        return json.dumps(type_payload, sort_keys=True, ensure_ascii=False)

    def _variant(payload: dict[str, Any]) -> tuple[str, Any]:
        return next(iter(payload.items()))

    def _collect_expr(expr: dict[str, Any]) -> None:
        span_marker = _span_marker(expr.get("span"))
        if span_marker is not None:
            expr_span_markers.append(span_marker)
        kind_name, kind_payload = _variant(expr.get("kind") or {})
        if kind_name == "ArrayLiteral":
            for element in kind_payload:
                _collect_expr(element)
        elif kind_name == "RecordConstruct":
            for field in kind_payload.get("fields") or []:
                _collect_expr(field["value"])
        elif kind_name == "Call":
            _collect_expr(kind_payload["callee"])
            for argument in kind_payload.get("arguments") or []:
                _collect_expr(argument)
        elif kind_name == "Index":
            _collect_expr(kind_payload["base"])
            _collect_expr(kind_payload["index"])
        elif kind_name == "FieldAccess":
            _collect_expr(kind_payload["base"])
        elif kind_name == "Unary":
            _collect_expr(kind_payload["expr"])
        elif kind_name == "Binary":
            _collect_expr(kind_payload["left"])
            _collect_expr(kind_payload["right"])

    def _stmt_summary(stmt: dict[str, Any]) -> dict[str, Any]:
        stmt_name, stmt_payload = _variant(stmt)
        span_marker = _span_marker(stmt_payload.get("span"))
        if span_marker is not None:
            stmt_span_markers.append(span_marker)
        if stmt_name == "Let":
            _collect_expr(stmt_payload["value"])
        elif stmt_name == "Assign":
            _collect_expr(stmt_payload["value"])
        elif stmt_name == "If":
            _collect_expr(stmt_payload["condition"])
            for nested in stmt_payload.get("then_body") or []:
                _stmt_summary(nested)
            for nested in stmt_payload.get("else_body") or []:
                _stmt_summary(nested)
        elif stmt_name == "While":
            _collect_expr(stmt_payload["condition"])
            for nested in stmt_payload.get("body") or []:
                _stmt_summary(nested)
        elif stmt_name == "For":
            _collect_expr(stmt_payload["start"])
            _collect_expr(stmt_payload["end"])
            for nested in stmt_payload.get("body") or []:
                _stmt_summary(nested)
        elif stmt_name == "Bind":
            bind_name, bind_payload = _variant(stmt_payload["value"])
            if bind_name == "Expr":
                _collect_expr(bind_payload)
            else:
                for argument in bind_payload.get("arguments") or []:
                    _collect_expr(argument)
            for clause in stmt_payload.get("contract_clauses") or []:
                contract_span = _span_marker(clause.get("span"))
                if contract_span is not None:
                    contract_span_markers.append(contract_span)
                _collect_expr(clause["condition"])
        elif stmt_name == "Perform":
            for argument in stmt_payload["call"].get("arguments") or []:
                _collect_expr(argument)
            for clause in stmt_payload.get("contract_clauses") or []:
                contract_span = _span_marker(clause.get("span"))
                if contract_span is not None:
                    contract_span_markers.append(contract_span)
                _collect_expr(clause["condition"])
        elif stmt_name == "Return":
            _collect_expr(stmt_payload["value"])
        return {
            "kind": stmt_name,
            "contract_clause_kinds": [
                clause["kind"] for clause in stmt_payload.get("contract_clauses") or []
            ],
        }

    function_summaries = []
    for item in items:
        item_name, item_payload = _variant(item)
        if item_name != "Function":
            continue
        function_summaries.append(
            {
                "function_name": item_payload["function_name"],
                "parameter_name": item_payload["parameter_name"],
                "input_type": _type_name(item_payload["input_type"]),
                "output_type": _type_name(item_payload["output_type"]),
                "statement_kinds": [
                    summary["kind"] for summary in map(_stmt_summary, item_payload["body"])
                ],
            }
        )

    transition_summaries = []
    for transition in module.get("transitions") or []:
        stmt_summaries = [_stmt_summary(stmt) for stmt in transition["body"]]
        transition_summaries.append(
            {
                "transition_name": transition["transition_name"],
                "place_ref": transition["place_ref"],
                "required_capabilities": transition["required_capabilities"],
                "statement_kinds": [summary["kind"] for summary in stmt_summaries],
                "contract_clause_kinds": [
                    clause_kind
                    for summary in stmt_summaries
                    for clause_kind in summary["contract_clause_kinds"]
                ],
            }
        )

    return {
        "accepted": payload.get("accepted"),
        "module_path": module.get("module_path"),
        "module_span": _span_marker(module.get("span")),
        "import_paths": [row["module_path"] for row in module.get("imports") or []],
        "import_span_markers": [
            _span_marker(row.get("span")) for row in module.get("imports") or []
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
                "parameter_names": [param["name"] for param in row.get("parameters") or []],
                "parameter_types": [
                    _type_name(param["param_type"]) for param in row.get("parameters") or []
                ],
                "required_capabilities": row["required_capabilities"],
                "output_name": (row.get("output") or {}).get("name"),
                "output_type": _type_name((row.get("output") or {}).get("output_type"))
                if row.get("output")
                else None,
                "failure_row": row["failure_row"],
            }
            for row in module.get("effects") or []
        ],
        "function_summaries": function_summaries,
        "transition_summaries": transition_summaries,
        "stmt_span_markers": stmt_span_markers,
        "contract_span_markers": contract_span_markers,
        "expr_span_markers": expr_span_markers,
        "diagnostics": [
            {
                "code": row["code"],
                "line": row["span"]["line"],
                "column": row["span"]["column"],
            }
            for row in payload.get("diagnostics") or []
        ],
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise ValueError(f"unknown textual Mir sample `{sample_id}`")
    payload = _parse_source(_row_source_path(row))
    expected = json.loads(_row_expected_path(row).read_text(encoding="utf-8"))
    actual = _payload_projection(payload)
    mismatches = [
        key
        for key, expected_value in expected.items()
        if actual.get(key) != expected_value
    ]
    return {
        "command": "run",
        "family": data["family"],
        "sample_id": sample_id,
        "source": str(_row_source_path(row).relative_to(REPO_ROOT)),
        "expected": expected,
        "actual": actual,
        "accepted": not mismatches,
        "mismatches": mismatches,
        "raw_parse_report": payload,
    }


def check_all() -> dict[str, Any]:
    status = matrix()
    results = [run_sample(row["sample_id"]) for row in _load_matrix()["rows"]]
    failed = [
        result["sample_id"]
        for result in results
        if result["mismatches"] or result["raw_parse_report"]["returncode"] not in {0, 2}
    ]
    if status["validation_errors"]:
        failed.extend(
            sorted(
                {
                    row["sample_id"]
                    for row in _load_matrix()["rows"]
                    if any(error["sample_id"] == row["sample_id"] for error in status["validation_errors"])
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
        lines = ["TEXTUAL MIR ALPHA SAMPLES"]
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
