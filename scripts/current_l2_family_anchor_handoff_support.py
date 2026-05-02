#!/usr/bin/env python3

import argparse
import json
from pathlib import Path
from typing import Any


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_family_anchor_handoff_parser(description: str) -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument("fixture_path")
    parser.add_argument("artifact_path")
    return parser


def read_fixture_checked_anchor_handoff_scope(fixture: dict[str, Any]) -> str | None:
    expected_anchor_handoff = fixture.get("expected_anchor_handoff")
    if not isinstance(expected_anchor_handoff, dict):
        return None

    scope = expected_anchor_handoff.get("checked_anchor_handoff_scope")
    if not isinstance(scope, str):
        return None
    return scope


def read_fixture_checked_anchor_handoff_rows(
    fixture: dict[str, Any],
) -> list[dict[str, Any]] | None:
    expected_anchor_handoff = fixture.get("expected_anchor_handoff")
    if not isinstance(expected_anchor_handoff, dict):
        return None

    checked_anchor_handoff_rows = expected_anchor_handoff.get(
        "checked_anchor_handoff_rows"
    )
    if checked_anchor_handoff_rows is None:
        return None

    normalized_rows: list[dict[str, Any]] = []
    for row in checked_anchor_handoff_rows:
        if isinstance(row, dict):
            normalized_rows.append(row)
    return normalized_rows


def read_actual_anchor_handoff_candidates(
    artifact: dict[str, Any],
) -> tuple[str | None, list[dict[str, Any]]]:
    detached_noncore = artifact.get("detached_noncore")
    if not isinstance(detached_noncore, dict):
        return None, []

    scope = detached_noncore.get("anchor_handoff_scope")
    if not isinstance(scope, str):
        scope = None

    anchor_handoff_rows = detached_noncore.get("anchor_handoff_rows")
    if not isinstance(anchor_handoff_rows, list):
        return scope, []

    normalized_rows: list[dict[str, Any]] = []
    for row in anchor_handoff_rows:
        if isinstance(row, dict):
            normalized_rows.append(row)
    return scope, normalized_rows


def filter_anchor_handoff_rows(
    rows: list[dict[str, Any]] | None,
    kinds: set[str],
) -> list[dict[str, Any]]:
    if not rows:
        return []
    return [
        row for row in rows if isinstance(row, dict) and row.get("kind") in kinds
    ]


def status_for_rows(
    fixture_rows: list[dict[str, Any]],
    actual_rows: list[dict[str, Any]],
    *,
    missing_status: str,
) -> tuple[str, int]:
    if not fixture_rows and not actual_rows:
        return "out_of_scope", 0
    if not fixture_rows and actual_rows:
        return missing_status, 1
    if fixture_rows == actual_rows:
        return "matched", 0
    return "mismatch", 1


def snippet(rows: list[dict[str, Any]]) -> str:
    return json.dumps(rows, ensure_ascii=False, indent=2)


def run_family_anchor_handoff_checker(
    *,
    argv: list[str] | None,
    cluster_name: str,
    description: str,
    kinds: set[str],
    missing_status: str,
    expected_scope: str | None = None,
) -> int:
    parser = build_family_anchor_handoff_parser(description)
    args = parser.parse_args(argv)

    fixture_path = Path(args.fixture_path)
    artifact_path = Path(args.artifact_path)

    fixture = read_json(fixture_path)
    artifact = read_json(artifact_path)

    fixture_rows = filter_anchor_handoff_rows(
        read_fixture_checked_anchor_handoff_rows(fixture),
        kinds,
    )
    fixture_scope = read_fixture_checked_anchor_handoff_scope(fixture)
    actual_scope, actual_anchor_handoff_rows = read_actual_anchor_handoff_candidates(
        artifact
    )
    actual_rows = filter_anchor_handoff_rows(actual_anchor_handoff_rows, kinds)

    if fixture_rows and fixture_scope is None:
        raise ValueError(
            "fixture checked_anchor_handoff_scope is required when checked_anchor_handoff_rows are present"
        )

    if fixture_scope is not None and expected_scope is not None and fixture_scope != expected_scope:
        raise ValueError(
            "fixture checked_anchor_handoff_scope does not match the helper floor scope"
        )

    resolved_expected_scope = fixture_scope if fixture_scope is not None else expected_scope
    scope_mismatch = (
        resolved_expected_scope is not None
        and bool(actual_rows)
        and actual_scope != resolved_expected_scope
    )

    if scope_mismatch:
        status, exit_code = "scope_mismatch", 1
        checked_actual_rows: list[dict[str, Any]] = []
    else:
        checked_actual_rows = actual_rows
        status, exit_code = status_for_rows(
            fixture_rows,
            checked_actual_rows,
            missing_status=missing_status,
        )

    print(f"fixture: {fixture_path}")
    print(f"artifact: {artifact_path}")
    print(f"cluster: {cluster_name}")
    print(f"expected_anchor_handoff_scope: {resolved_expected_scope}")
    print(f"artifact_anchor_handoff_scope: {actual_scope}")
    print(f"status: {status}")
    if scope_mismatch:
        print(
            "scope mismatch: artifact anchor-handoff rows are outside this anchor-handoff floor"
        )
    print("fixture rows:")
    print(snippet(fixture_rows))
    print("actual rows:")
    print(snippet(checked_actual_rows))

    return exit_code
