#!/usr/bin/env python3

import argparse
import json
from pathlib import Path
from typing import Any

from current_l2_reason_codes_assist import (
    read_actual_reason_code_candidates,
    read_fixture_checked_reason_codes,
    read_json,
)


def build_family_checker_parser(description: str) -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument("fixture_path")
    parser.add_argument("artifact_path")
    return parser


def filter_reason_rows(
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


def run_family_checker(
    *,
    argv: list[str] | None,
    cluster_name: str,
    description: str,
    kinds: set[str],
    missing_status: str,
) -> int:
    parser = build_family_checker_parser(description)
    args = parser.parse_args(argv)

    fixture_path = Path(args.fixture_path)
    artifact_path = Path(args.artifact_path)

    fixture = read_json(fixture_path)
    artifact = read_json(artifact_path)

    fixture_rows = filter_reason_rows(read_fixture_checked_reason_codes(fixture), kinds)
    _, actual_reason_rows = read_actual_reason_code_candidates(artifact)
    actual_rows = filter_reason_rows(actual_reason_rows, kinds)

    status, exit_code = status_for_rows(
        fixture_rows,
        actual_rows,
        missing_status=missing_status,
    )

    print(f"fixture: {fixture_path}")
    print(f"artifact: {artifact_path}")
    print(f"cluster: {cluster_name}")
    print(f"status: {status}")
    print("fixture rows:")
    print(snippet(fixture_rows))
    print("actual rows:")
    print(snippet(actual_rows))

    return exit_code
