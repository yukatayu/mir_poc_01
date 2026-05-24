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
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
STOP_LINES = [
    "no final public grammar / ABI / SDK",
    "no Surface-to-Core elaboration completion yet",
    "no runtime execution or source patch hot-plug completion yet",
    "no generated package artifact authority",
]
NON_CLAIMS = [
    "P-SURF-01 parser evidence only",
    "Core IR generation remains P-SURF-03",
    "role admission authority remains P-SURF-05",
    "source patch activation remains P-SURF-06",
]
VALIDATION_FLOOR = [
    "cargo test -p mir-ast --test surface_mir_parser -- --nocapture",
    "python3 -m unittest scripts.tests.test_surface_mir_samples",
    "python3 scripts/surface_mir_samples.py matrix --format json",
    "python3 scripts/surface_mir_samples.py check-all --format json",
]


def _load_matrix() -> dict[str, Any]:
    return json.loads(SYNTAX_MATRIX_PATH.read_text(encoding="utf-8"))


def _row_source_path(row: dict[str, Any]) -> Path:
    return SYNTAX_ROOT / row["source"]


def _row_expected_path(row: dict[str, Any]) -> Path:
    return SYNTAX_ROOT / row["expected"]


def validate_rows(rows: list[dict[str, Any]]) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = SYNTAX_ROOT / row["root_name"]
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
                    "detail": f"missing expected projection `{expected_path}`",
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
    return [_materialize_row(row) for row in _load_matrix()["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(data["rows"])
    executable_rows = [
        row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"
    ]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SURFACE_ROOT.relative_to(REPO_ROOT)),
        "syntax_root": str(SYNTAX_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(SYNTAX_MATRIX_PATH.relative_to(REPO_ROOT)),
        "sample_count": len(rows),
        "executable_count": len(executable_rows),
        "executable_rows": executable_rows,
        "matrix_status": data["current_status"],
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


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise ValueError(f"unknown Surface Mir sample `{sample_id}`")
    payload = _parse_source(_row_source_path(row))
    expected = json.loads(_row_expected_path(row).read_text(encoding="utf-8"))
    actual = _payload_projection(payload)
    mismatches = [
        key for key, expected_value in expected.items() if actual.get(key) != expected_value
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
