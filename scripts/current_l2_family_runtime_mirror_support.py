#!/usr/bin/env python3

import argparse
import json
from pathlib import Path
from typing import Any


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_family_runtime_mirror_parser(description: str) -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument("target_path")
    parser.add_argument("source_path")
    return parser


def read_target_runtime_mirror(target: dict[str, Any]) -> dict[str, Any] | None:
    runtime_mirror = target.get("runtime_mirror")
    if not isinstance(runtime_mirror, dict):
        return None
    return runtime_mirror


def read_target_checked_runtime_mirror_rows(
    target: dict[str, Any],
) -> list[dict[str, Any]] | None:
    runtime_mirror = read_target_runtime_mirror(target)
    if runtime_mirror is None:
        return None

    checked_rows = runtime_mirror.get("checked_runtime_mirror_rows")
    if checked_rows is None:
        return None

    normalized_rows: list[dict[str, Any]] = []
    for row in checked_rows:
        if isinstance(row, dict):
            normalized_rows.append(row)
    return normalized_rows


def read_target_runtime_mirror_scope(target: dict[str, Any]) -> str | None:
    runtime_mirror = read_target_runtime_mirror(target)
    if runtime_mirror is None:
        return None

    scope = runtime_mirror.get("scope")
    if not isinstance(scope, str):
        return None
    return scope


def filter_runtime_mirror_rows(
    rows: list[dict[str, Any]] | None,
    kinds: set[str],
) -> list[dict[str, Any]]:
    if not rows:
        return []
    return [
        row
        for row in rows
        if isinstance(row, dict) and row.get("kind") in kinds
    ]


def snippet(rows: list[dict[str, Any]]) -> str:
    return json.dumps(rows, ensure_ascii=False, indent=2)


def source_claims_match(
    source: dict[str, Any],
    required_claims: dict[str, Any],
) -> bool:
    claims = source.get("claims")
    if not isinstance(claims, dict):
        return False
    for key, value in required_claims.items():
        if claims.get(key) != value:
            return False
    return True


def source_validation_mode_matches(
    source: dict[str, Any],
    required_mode: str | None,
) -> bool:
    if required_mode is None:
        return True

    current_validation = source.get("current_validation")
    if not isinstance(current_validation, dict):
        return False
    return current_validation.get("mode") == required_mode


def list_contains_all(actual: Any, required_items: list[Any]) -> bool:
    if not isinstance(actual, list):
        return False
    return all(item in actual for item in required_items)


def row_matches_expected_runtime(
    row: dict[str, Any],
    *,
    target_sample_id: str | None,
    source_sample_id: str | None,
    expected_runtime: dict[str, Any],
) -> bool:
    if target_sample_id is not None and row.get("target_sample_id") != target_sample_id:
        return False
    if source_sample_id is not None and row.get("source_sample_id") != source_sample_id:
        return False

    for key, value in row.items():
        if key in {"kind", "target_sample_id", "source_sample_id"}:
            continue
        if key == "required_preview_reason_refs":
            if not isinstance(value, list):
                return False
            if not list_contains_all(expected_runtime.get(key), value):
                return False
            continue
        if key.startswith("minimum_"):
            runtime_key = key[len("minimum_") :]
            actual_value = expected_runtime.get(runtime_key)
            if not isinstance(actual_value, (int, float)):
                return False
            if actual_value < value:
                return False
            continue
        if expected_runtime.get(key) != value:
            return False
    return True


def source_is_runtime_like(source: dict[str, Any]) -> bool:
    return (
        isinstance(source.get("status"), str)
        and isinstance(source.get("claims"), dict)
        and isinstance(source.get("current_validation"), dict)
        and isinstance(source.get("expected_runtime"), dict)
    )


def resolve_target_rows_against_source(
    *,
    target: dict[str, Any],
    target_path: Path,
    source: dict[str, Any],
    source_path: Path,
    rows: list[dict[str, Any]],
) -> list[dict[str, Any]] | None:
    runtime_mirror = read_target_runtime_mirror(target)
    if runtime_mirror is None:
        return None

    source_sidecar = runtime_mirror.get("source_sidecar")
    if not isinstance(source_sidecar, str):
        return None
    if Path(source_sidecar).resolve() != source_path.resolve():
        return None

    source_family = runtime_mirror.get("source_family")
    if not isinstance(source_family, str) or source.get("family") != source_family:
        return None

    source_sample_id = runtime_mirror.get("source_sample_id")
    if not isinstance(source_sample_id, str) or source.get("sample_id") != source_sample_id:
        return None

    required_status = runtime_mirror.get("required_source_status")
    if not isinstance(required_status, str) or source.get("status") != required_status:
        return None

    required_claims = runtime_mirror.get("required_source_claims")
    if not isinstance(required_claims, dict) or not source_claims_match(source, required_claims):
        return None

    required_validation_mode = runtime_mirror.get("required_source_current_validation_mode")
    if not isinstance(required_validation_mode, str):
        return None
    if not source_validation_mode_matches(source, required_validation_mode):
        return None

    target_sample_id = target.get("sample_id")
    if not isinstance(target_sample_id, str):
        return None

    mirrors = source.get("mirrors")
    if not isinstance(mirrors, list) or target_sample_id not in mirrors:
        return None

    expected_runtime = source.get("expected_runtime")
    if not isinstance(expected_runtime, dict):
        return None

    resolved_rows: list[dict[str, Any]] = []
    for row in rows:
        if not row_matches_expected_runtime(
            row,
            target_sample_id=target_sample_id,
            source_sample_id=source_sample_id,
            expected_runtime=expected_runtime,
        ):
            return None
        resolved_rows.append(row)
    return resolved_rows


def status_for_rows(
    fixture_rows: list[dict[str, Any]],
    actual_rows: list[dict[str, Any]],
    *,
    missing_status: str,
    source_like: bool,
    target_missing_expected_rows: bool,
) -> tuple[str, int]:
    if not fixture_rows and not actual_rows:
        if target_missing_expected_rows and source_like:
            return missing_status, 1
        return "out_of_scope", 0
    if not fixture_rows and actual_rows:
        return missing_status, 1
    if fixture_rows == actual_rows:
        return "matched", 0
    return "mismatch", 1


def run_family_runtime_mirror_checker(
    *,
    argv: list[str] | None,
    cluster_name: str,
    description: str,
    kinds: set[str],
    missing_status: str,
    expected_scope: str | None = None,
) -> int:
    parser = build_family_runtime_mirror_parser(description)
    args = parser.parse_args(argv)

    target_path = Path(args.target_path)
    source_path = Path(args.source_path)

    target = read_json(target_path)
    source = read_json(source_path)

    raw_target_rows = read_target_checked_runtime_mirror_rows(target)
    fixture_rows = filter_runtime_mirror_rows(raw_target_rows, kinds)
    target_scope = read_target_runtime_mirror_scope(target)
    target_missing_expected_rows = raw_target_rows is None

    scope_mismatch = (
        expected_scope is not None
        and bool(fixture_rows)
        and target_scope != expected_scope
    )

    if scope_mismatch:
        status, exit_code = "scope_mismatch", 1
        checked_actual_rows: list[dict[str, Any]] = []
    else:
        resolved_rows = resolve_target_rows_against_source(
            target=target,
            target_path=target_path,
            source=source,
            source_path=source_path,
            rows=fixture_rows,
        )
        checked_actual_rows = resolved_rows or []
        status, exit_code = status_for_rows(
            fixture_rows,
            checked_actual_rows,
            missing_status=missing_status,
            source_like=source_is_runtime_like(source),
            target_missing_expected_rows=target_missing_expected_rows,
        )

    print(f"target: {target_path}")
    print(f"source: {source_path}")
    print(f"cluster: {cluster_name}")
    print(f"expected_runtime_mirror_scope: {expected_scope}")
    print(f"target_runtime_mirror_scope: {target_scope}")
    print(f"status: {status}")
    if scope_mismatch:
        print("scope mismatch: target runtime_mirror rows are outside this mirror floor")
    print("target rows:")
    print(snippet(fixture_rows))
    print("checked source rows:")
    print(snippet(checked_actual_rows))

    return exit_code
