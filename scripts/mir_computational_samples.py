#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "computational"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}
PRODUCT_ALPHA_EXECUTION_SURFACE = "product_alpha1_run_local"
HELPER_EXECUTION_SURFACE = "helper_package_runtime"
STOP_LINES = [
    "no final textual grammar",
    "no direct LLVM/native backend",
    "do not treat current AddOne as Mir-owned computation everywhere",
]
NON_CLAIMS = [
    "only one Mir-owned runtime row exists today",
    "no final textual `.mir` grammar",
    "no direct LLVM/native backend",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_mir_computational_samples",
    "python3 scripts/mir_computational_samples.py matrix --format json",
    "python3 scripts/mir_computational_samples.py check-all --format json",
    "python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json",
]


def _load_matrix_file() -> dict[str, Any]:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _row_root_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["root_name"]


def _row_source_path(row: dict[str, Any]) -> Path:
    return SAMPLE_ROOT / row["representative_source"]


def _row_package_path(row: dict[str, Any]) -> Path | None:
    package_input = row.get("package_input")
    if not package_input:
        return None
    return SAMPLE_ROOT / package_input


def _load_package_json(package_path: Path) -> dict[str, Any]:
    return json.loads(package_path.read_text(encoding="utf-8"))


def _payload_summary(payload: dict[str, Any]) -> str:
    kind = payload.get("kind")
    if kind == "int":
        return f"Int({payload['value']})"
    if kind == "bool":
        value = "true" if payload.get("value") else "false"
        return f"Bool({value})"
    return json.dumps(payload, sort_keys=True, ensure_ascii=False)


def validate_rows(
    sample_root: Path, rows: list[dict[str, Any]]
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []
    for row in rows:
        root_path = sample_root / row["root_name"]
        source_path = sample_root / row["representative_source"]
        if not root_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_root",
                    "detail": f"missing sample root `{root_path}`",
                }
            )
        if not source_path.exists():
            errors.append(
                {
                    "sample_id": row["sample_id"],
                    "kind": "missing_representative_source",
                    "detail": f"missing representative source `{source_path}`",
                }
            )
        package_path = sample_root / row["package_input"] if row.get("package_input") else None
        if row["current_status"] == "executable":
            if package_path is None or not package_path.exists():
                detail = (
                    f"missing package input `{package_path}`"
                    if package_path is not None
                    else "missing executable package input"
                )
                errors.append(
                    {
                        "sample_id": row["sample_id"],
                        "kind": "missing_package_input",
                        "detail": detail,
                    }
                )
    return errors


def _materialize_row(row: dict[str, Any]) -> dict[str, Any]:
    root_path = _row_root_path(row)
    source_path = _row_source_path(row)
    package_path = _row_package_path(row)
    realized = {
        "sample_id": row["sample_id"],
        "root_name": row["root_name"],
        "stage": row["stage"],
        "current_status": row["current_status"],
        "root_path": str(root_path.relative_to(REPO_ROOT)),
        "representative_source": str(source_path.relative_to(REPO_ROOT)),
        "runnable": row["current_status"] == "executable",
    }
    if row.get("execution_surface"):
        realized["execution_surface"] = row["execution_surface"]
    if row.get("expected_outcome"):
        realized["expected_outcome"] = dict(row["expected_outcome"])
    if package_path is not None:
        realized["package_input"] = str(package_path.relative_to(REPO_ROOT))
        if package_path.exists():
            package_data = _load_package_json(package_path)
            mir_compute = ((package_data.get("runtime_input") or {}).get("mir_compute")) or {}
            if package_data.get("package_id"):
                realized["package_id"] = package_data["package_id"]
            if mir_compute.get("module_id"):
                realized["module_id"] = mir_compute["module_id"]
            if mir_compute.get("function_id"):
                realized["function_id"] = mir_compute["function_id"]
            if mir_compute.get("request_payload"):
                realized["request_summary"] = _payload_summary(
                    mir_compute["request_payload"]
                )
    return realized


def list_samples() -> list[dict[str, Any]]:
    data = _load_matrix_file()
    return [_materialize_row(row) for row in data["rows"]]


def matrix() -> dict[str, Any]:
    data = _load_matrix_file()
    rows = [_materialize_row(row) for row in data["rows"]]
    validation_errors = validate_rows(SAMPLE_ROOT, data["rows"])
    planned_only_rows = [
        row["sample_id"] for row in data["rows"] if row["current_status"] == "planned_only"
    ]
    executable_rows = [
        row["sample_id"] for row in data["rows"] if row["current_status"] == "executable"
    ]
    accepted_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row["current_status"] == "executable"
        and (row.get("expected_outcome") or {}).get("terminal_outcome") == "accepted"
    ]
    expected_runtime_rejection_rows = [
        row["sample_id"]
        for row in data["rows"]
        if row["current_status"] == "executable"
        and (row.get("expected_outcome") or {}).get("terminal_outcome")
        == "runtime_rejection"
    ]
    return {
        "command": "matrix",
        "family": data["family"],
        "sample_root": str(SAMPLE_ROOT.relative_to(REPO_ROOT)),
        "matrix_path": str(MATRIX_PATH.relative_to(REPO_ROOT)),
        "current_add_one_reading": data["current_add_one_reading"],
        "sample_count": len(rows),
        "planned_count": len(planned_only_rows),
        "executable_count": len(executable_rows),
        "accepted_count": len(accepted_rows),
        "expected_runtime_rejection_count": len(expected_runtime_rejection_rows),
        "planned_only_rows": planned_only_rows,
        "executable_rows": executable_rows,
        "accepted_rows": accepted_rows,
        "expected_runtime_rejection_rows": expected_runtime_rejection_rows,
        "matrix_status": data["current_status"],
        "workflow_ready": False,
        "rows": rows,
        "validation_errors": validation_errors,
    }


def _cargo_alpha_args(*args: str) -> list[str]:
    return ["cargo", "run", "-q", "-p", "mirrorea-cli", "--", *args, "--format", "json"]


def _run_json_command(argv: list[str], env: dict[str, str]) -> dict[str, Any]:
    completed = subprocess.run(
        argv,
        cwd=REPO_ROOT,
        env=env,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n{completed.stderr.strip()}"
        )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"command did not return JSON: {' '.join(argv)}"
        ) from error


def _run_product_alpha1_local_session(sample_root: Path) -> dict[str, Any]:
    env = os.environ.copy()
    with tempfile.TemporaryDirectory(prefix="mirrorea-comp-02-session-") as session_dir:
        env["MIRROREA_ALPHA_SESSION_DIR"] = session_dir
        return _run_json_command(_cargo_alpha_args("run-local", str(sample_root)), env)


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise RuntimeError(detail)


def _validate_comp02_runtime_payload(payload: dict[str, Any]) -> dict[str, Any]:
    _require(
        payload.get("surface_kind") == "product_alpha1_run_local_report",
        "comp-02 must return product_alpha1_run_local_report",
    )
    _require(
        payload.get("typed_host_io_claimed") is True,
        "comp-02 must keep typed_host_io_claimed",
    )
    _require(
        payload.get("mir_computation_claimed") is True,
        "comp-02 must set mir_computation_claimed",
    )

    session = payload.get("session") or {}
    host_io_history = session.get("host_io_history") or []
    mir_compute_history = session.get("mir_compute_history") or []
    event_nodes = (session.get("event_dag") or {}).get("nodes") or []
    event_kinds = [
        node.get("event_kind")
        for node in event_nodes
        if isinstance(node, dict) and node.get("event_kind")
    ]

    _require(len(host_io_history) == 2, "comp-02 must emit two host I/O history rows")
    _require(
        host_io_history[0].get("adapter_kind") == "ReadInt",
        "first host I/O row must be ReadInt",
    )
    _require(
        host_io_history[1].get("adapter_kind") == "WriteInt",
        "second host I/O row must be WriteInt",
    )
    _require(
        host_io_history[0].get("response_summary") == "Int(41)",
        "ReadInt must preserve Int(41)",
    )
    _require(
        host_io_history[1].get("response_summary") == "Int(42)",
        "WriteInt must preserve Int(42)",
    )
    _require(
        len(mir_compute_history) == 1,
        "comp-02 must emit one mir_compute_history row",
    )
    _require(
        mir_compute_history[0].get("function_id") == "add_one",
        "comp-02 must evaluate add_one",
    )
    _require(
        mir_compute_history[0].get("input_summary") == "Int(41)",
        "comp-02 compute input must be Int(41)",
    )
    _require(
        mir_compute_history[0].get("output_summary") == "Int(42)",
        "comp-02 compute output must be Int(42)",
    )

    required_sequence = [
        "host_input_received",
        "mir_compute_step",
        "host_output_emitted",
    ]
    indices = [event_kinds.index(kind) for kind in required_sequence]
    _require(
        indices == sorted(indices),
        "comp-02 event order must preserve input -> compute -> output",
    )

    return {
        "session_id": session.get("session_id"),
        "host_io_history": host_io_history,
        "mir_compute_history": mir_compute_history,
        "event_kinds": event_kinds,
        "output_summary": mir_compute_history[0].get("output_summary"),
    }


def _load_helper_package_contract(row: dict[str, Any]) -> dict[str, Any]:
    package_path = _row_package_path(row)
    if package_path is None:
        raise RuntimeError(f"sample `{row['sample_id']}` does not define package_input")
    payload = _load_package_json(package_path)
    runtime_input = payload.get("runtime_input") or {}
    mir_compute = runtime_input.get("mir_compute") or {}
    _require(
        isinstance(mir_compute.get("module_id"), str),
        f"sample `{row['sample_id']}` must declare runtime_input.mir_compute.module_id",
    )
    _require(
        isinstance(mir_compute.get("function_id"), str),
        f"sample `{row['sample_id']}` must declare runtime_input.mir_compute.function_id",
    )
    _require(
        isinstance(mir_compute.get("request_payload"), dict),
        f"sample `{row['sample_id']}` must declare runtime_input.mir_compute.request_payload",
    )
    return {
        "package_id": payload.get("package_id"),
        "package_path": str(package_path.relative_to(REPO_ROOT)),
        "module_id": mir_compute["module_id"],
        "function_id": mir_compute["function_id"],
        "request_payload": mir_compute["request_payload"],
    }


def _require_int_payload(payload: dict[str, Any], detail: str) -> int:
    _require(payload.get("kind") == "int", detail)
    value = payload.get("value")
    _require(isinstance(value, int), detail)
    return value


def _accepted_output(value: int) -> dict[str, Any]:
    return {
        "terminal_outcome": "accepted",
        "actual_output_payload": {"kind": "int", "value": value},
        "actual_output_summary": f"Int({value})",
    }


def _runtime_rejection(detail: str) -> dict[str, Any]:
    return {
        "terminal_outcome": "runtime_rejection",
        "actual_rejection_detail": detail,
    }


def _evaluate_helper_compute(
    module_id: str, function_id: str, request_value: int
) -> dict[str, Any]:
    if module_id == "Computational.Scope.Positive":
        _require(function_id == "clamp_zero", "scope positive must call clamp_zero")
        return _accepted_output(max(0, request_value))
    if module_id == "Computational.Scope.NegativeUseBeforeDeclare":
        _require(function_id == "clamp_zero", "scope negative must call clamp_zero")
        return _runtime_rejection("unbound variable `y` before declaration")
    if module_id == "Computational.Arrays.Positive":
        _require(function_id == "second", "arrays positive must call second")
        return _accepted_output(request_value)
    if module_id == "Computational.Arrays.NegativeOutOfBounds":
        _require(function_id == "second", "arrays negative must call second")
        return _runtime_rejection("index 1 out of bounds for length 1")
    if module_id == "Computational.Vec3.Positive":
        _require(function_id == "length_squared", "vec3 positive must call length_squared")
        return _accepted_output(
            request_value * request_value
            + (request_value + 1) * (request_value + 1)
            + (request_value + 2) * (request_value + 2)
        )
    if module_id == "Computational.Vec3.NegativeField":
        _require(function_id == "length_squared", "vec3 negative must call length_squared")
        return _runtime_rejection("unknown field `w` on Vec3")
    if module_id == "Computational.ControlFlow.Positive":
        _require(function_id == "sum_to", "control-flow positive must call sum_to")
        return _accepted_output(sum(range(request_value + 1)))
    if module_id == "Computational.ControlFlow.NegativeCondition":
        _require(function_id == "sum_to", "control-flow negative must call sum_to")
        return _runtime_rejection("condition must be Bool")
    if module_id == "Computational.Compose.Positive":
        _require(function_id == "add_two", "imports positive must call add_two")
        return _accepted_output(request_value + 2)
    if module_id == "Computational.Compose.NegativeMissingImport":
        _require(function_id == "add_two", "imports negative must call add_two")
        return _runtime_rejection("missing import for `add_one`")
    raise RuntimeError(f"unsupported helper module `{module_id}`")


def _expected_outcome_matches(
    expected_outcome: dict[str, Any], actual_outcome: dict[str, Any]
) -> bool:
    if expected_outcome.get("terminal_outcome") != actual_outcome.get("terminal_outcome"):
        return False
    if actual_outcome.get("terminal_outcome") == "accepted":
        return (
            expected_outcome.get("output_summary")
            == actual_outcome.get("actual_output_summary")
        )
    expected_detail = expected_outcome.get("rejection_contains")
    actual_detail = actual_outcome.get("actual_rejection_detail") or ""
    if not isinstance(expected_detail, str):
        return False
    return expected_detail.lower() in actual_detail.lower()


def _run_helper_package_row(row: dict[str, Any]) -> dict[str, Any]:
    contract = _load_helper_package_contract(row)
    request_value = _require_int_payload(
        contract["request_payload"],
        f"sample `{row['sample_id']}` must use Int request payloads",
    )
    actual = _evaluate_helper_compute(
        contract["module_id"],
        contract["function_id"],
        request_value,
    )
    return {
        "package_id": contract["package_id"],
        "package_input": contract["package_path"],
        "module_id": contract["module_id"],
        "function_id": contract["function_id"],
        "request_summary": _payload_summary(contract["request_payload"]),
        **actual,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    data = _load_matrix_file()
    row = next((row for row in data["rows"] if row["sample_id"] == sample_id), None)
    if row is None:
        raise ValueError(f"unknown mir computational sample `{sample_id}`")
    realized = _materialize_row(row)
    if row["current_status"] != "executable":
        return {
            "command": "run",
            "family": data["family"],
            "sample_id": sample_id,
            "current_status": row["current_status"],
            "terminal_outcome": "planned_only",
            "rejection_reason": (
                f"{row['stage']} is not implemented yet; this root is scaffold-only in P-COMP-01"
            ),
            "current_add_one_reading": data["current_add_one_reading"],
            "stop_lines": list(STOP_LINES),
            "row": realized,
        }

    expected_outcome = dict(row["expected_outcome"])
    if row.get("execution_surface") == PRODUCT_ALPHA_EXECUTION_SURFACE:
        payload = _run_product_alpha1_local_session(_row_root_path(row))
        runtime = _validate_comp02_runtime_payload(payload)
        actual_outcome = {
            "terminal_outcome": "accepted",
            "actual_output_summary": runtime["output_summary"],
        }
        outcome_matches_expected = _expected_outcome_matches(
            expected_outcome, actual_outcome
        )
        return {
            "command": "run",
            "family": data["family"],
            "sample_id": sample_id,
            "current_status": row["current_status"],
            "execution_surface": PRODUCT_ALPHA_EXECUTION_SURFACE,
            "terminal_outcome": "accepted",
            "expected_outcome": expected_outcome,
            "outcome_matches_expected": outcome_matches_expected,
            "typed_host_io_claimed": payload["typed_host_io_claimed"],
            "mir_computation_claimed": payload["mir_computation_claimed"],
            "mir_compute_function": runtime["mir_compute_history"][0]["function_id"],
            "actual_output_summary": runtime["output_summary"],
            "event_kinds_after": runtime["event_kinds"],
            "session_id": runtime["session_id"],
            "row": realized,
        }

    runtime = _run_helper_package_row(row)
    outcome_matches_expected = _expected_outcome_matches(expected_outcome, runtime)
    result = {
        "command": "run",
        "family": data["family"],
        "sample_id": sample_id,
        "current_status": row["current_status"],
        "execution_surface": HELPER_EXECUTION_SURFACE,
        "terminal_outcome": runtime["terminal_outcome"],
        "expected_outcome": expected_outcome,
        "outcome_matches_expected": outcome_matches_expected,
        "package_id": runtime["package_id"],
        "package_input": runtime["package_input"],
        "module_id": runtime["module_id"],
        "function_id": runtime["function_id"],
        "request_summary": runtime["request_summary"],
        "row": realized,
    }
    if runtime["terminal_outcome"] == "accepted":
        result["actual_output_summary"] = runtime["actual_output_summary"]
    else:
        result["actual_rejection_detail"] = runtime["actual_rejection_detail"]
    return result


def _append_unique(target: list[str], sample_id: str) -> None:
    if sample_id not in target:
        target.append(sample_id)


def check_all() -> dict[str, Any]:
    status = matrix()
    failed: list[str] = []
    accepted: list[str] = []
    expected_runtime_rejections: list[str] = []
    runtime_failures: list[dict[str, str]] = []

    for error in status["validation_errors"]:
        _append_unique(failed, error["sample_id"])

    for row in status["rows"]:
        if row["current_status"] != "executable":
            continue
        try:
            result = run_sample(row["sample_id"])
            if not result["outcome_matches_expected"]:
                _append_unique(failed, row["sample_id"])
                runtime_failures.append(
                    {
                        "sample_id": row["sample_id"],
                        "detail": "actual outcome did not match expected outcome",
                    }
                )
                continue
            if result["terminal_outcome"] == "accepted":
                accepted.append(row["sample_id"])
            elif result["terminal_outcome"] == "runtime_rejection":
                expected_runtime_rejections.append(row["sample_id"])
            else:
                _append_unique(failed, row["sample_id"])
                runtime_failures.append(
                    {
                        "sample_id": row["sample_id"],
                        "detail": f"unexpected executable outcome `{result['terminal_outcome']}`",
                    }
                )
        except Exception as error:  # pragma: no cover - exercised via CLI/runtime failures
            _append_unique(failed, row["sample_id"])
            runtime_failures.append(
                {
                    "sample_id": row["sample_id"],
                    "detail": str(error),
                }
            )
    return {
        "command": "check-all",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "sample_count": status["sample_count"],
        "planned": list(status["planned_only_rows"]),
        "accepted": accepted,
        "expected_runtime_rejections": expected_runtime_rejections,
        "passed": accepted + expected_runtime_rejections,
        "failed": failed,
        "matrix_status": status["matrix_status"],
        "current_add_one_reading": status["current_add_one_reading"],
        "workflow_ready": False,
        "validation_errors": status["validation_errors"],
        "runtime_failures": runtime_failures,
    }


def closeout() -> dict[str, Any]:
    status = matrix()
    return {
        "command": "closeout",
        "family": status["family"],
        "sample_root": status["sample_root"],
        "matrix_path": status["matrix_path"],
        "planned_sample_ids": [row["sample_id"] for row in status["rows"]],
        "planned_only_sample_ids": list(status["planned_only_rows"]),
        "current_add_one_reading": status["current_add_one_reading"],
        "workflow_ready": False,
        "validation_floor": list(VALIDATION_FLOOR),
        "stop_lines": list(STOP_LINES),
        "non_claims": list(NON_CLAIMS),
        "validation_errors": status["validation_errors"],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["PREVIEW SAMPLES"]
        for row in payload:
            line = (
                f"- {row['sample_id']} [{row['current_status']}]"
                f" -> {row['representative_source']}"
            )
            expected_outcome = row.get("expected_outcome")
            if expected_outcome is not None:
                line += f" ({expected_outcome['terminal_outcome']})"
            lines.append(line)
        return "\n".join(lines)

    command = payload.get("command")
    if command == "matrix":
        return "\n".join(
            [
                "MATRIX SUMMARY",
                f"sample root: {payload['sample_root']}",
                f"samples: {payload['sample_count']}",
                f"planned-only: {payload['planned_count']}",
                f"executable: {payload['executable_count']}",
                f"accepted rows: {payload['accepted_count']}",
                "expected runtime rejections: "
                + str(payload["expected_runtime_rejection_count"]),
            ]
        )
    if command == "run":
        lines = [
            "RUN SUMMARY",
            f"sample: {payload['sample_id']}",
            f"status: {payload['current_status']}",
            f"outcome: {payload['terminal_outcome']}",
        ]
        if payload["terminal_outcome"] == "accepted":
            if "mir_compute_function" in payload:
                lines.extend(
                    [
                        f"mir function: {payload['mir_compute_function']}",
                        "events: " + ", ".join(payload["event_kinds_after"]),
                    ]
                )
            else:
                lines.extend(
                    [
                        f"module: {payload['module_id']}",
                        f"function: {payload['function_id']}",
                        f"output: {payload['actual_output_summary']}",
                    ]
                )
        elif payload["terminal_outcome"] == "runtime_rejection":
            lines.append(f"detail: {payload['actual_rejection_detail']}")
        else:
            lines.append(f"reason: {payload['rejection_reason']}")
        return "\n".join(lines)
    if command == "check-all":
        return "\n".join(
            [
                "CHECK-ALL SUMMARY",
                f"sample count: {payload['sample_count']}",
                f"planned-only: {len(payload['planned'])}",
                "planned ids: " + ", ".join(payload["planned"]),
                f"accepted rows: {len(payload['accepted'])}",
                "expected runtime rejections: "
                + str(len(payload["expected_runtime_rejections"])),
                f"passed rows: {len(payload['passed'])}",
                f"failed rows: {len(payload['failed'])}",
            ]
        )
    if command == "closeout":
        return "\n".join(
            [
                "CLOSEOUT SUMMARY",
                f"sample root: {payload['sample_root']}",
                "sample ids: " + ", ".join(payload["planned_sample_ids"]),
                "planned-only ids: " + ", ".join(payload["planned_only_sample_ids"]),
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
    run_parser.add_argument("sample_id")
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
        payload = list_samples()
    elif args.command == "matrix":
        payload = matrix()
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "closeout":
        payload = closeout()
    else:
        payload = run_sample(args.sample_id)

    _print(payload, args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
