#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any

from current_l2_reason_codes_assist import (
    read_actual_reason_code_candidates,
    read_fixture_checked_reason_codes,
    read_json,
)


MISSING_OPTION_KINDS = {
    "missing_chain_head_option",
    "missing_predecessor_option",
    "missing_successor_option",
}


def filter_missing_option_rows(
    rows: list[dict[str, Any]] | None,
) -> list[dict[str, Any]]:
    if not rows:
        return []
    return [
        row
        for row in rows
        if isinstance(row, dict) and row.get("kind") in MISSING_OPTION_KINDS
    ]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 の second checker spike として、fixture-side "
            "`checked_reason_codes` と static gate detached artifact の "
            "missing-option structure reason rows だけを narrow に照合する "
            "non-production helper。"
        )
    )
    parser.add_argument("fixture_path")
    parser.add_argument("artifact_path")
    return parser


def status_for_rows(
    fixture_rows: list[dict[str, Any]],
    actual_rows: list[dict[str, Any]],
) -> tuple[str, int]:
    if not fixture_rows and not actual_rows:
        return "out_of_scope", 0
    if not fixture_rows and actual_rows:
        return "fixture_missing_option_rows_missing", 1
    if fixture_rows == actual_rows:
        return "matched", 0
    return "mismatch", 1


def snippet(rows: list[dict[str, Any]]) -> str:
    return json.dumps(rows, ensure_ascii=False, indent=2)


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    fixture_path = Path(args.fixture_path)
    artifact_path = Path(args.artifact_path)

    fixture = read_json(fixture_path)
    artifact = read_json(artifact_path)

    fixture_rows = filter_missing_option_rows(read_fixture_checked_reason_codes(fixture))
    _, actual_reason_rows = read_actual_reason_code_candidates(artifact)
    actual_rows = filter_missing_option_rows(actual_reason_rows)

    status, exit_code = status_for_rows(fixture_rows, actual_rows)

    print(f"fixture: {fixture_path}")
    print(f"artifact: {artifact_path}")
    print("cluster: missing_option_structure_floor")
    print(f"status: {status}")
    print("fixture rows:")
    print(snippet(fixture_rows))
    print("actual rows:")
    print(snippet(actual_rows))

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
