#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_actual_reason_code_candidates(
    artifact: dict[str, Any],
) -> tuple[str | None, list[dict[str, Any]]]:
    detached_noncore = artifact.get("detached_noncore")
    if not isinstance(detached_noncore, dict):
        return None, []

    scope = detached_noncore.get("reason_codes_scope")
    if not isinstance(scope, str):
        scope = None

    reason_codes = detached_noncore.get("reason_codes")
    if not isinstance(reason_codes, list):
        return scope, []

    normalized_rows: list[dict[str, Any]] = []
    for row in reason_codes:
        if isinstance(row, dict):
            normalized_rows.append(row)
    return scope, normalized_rows


def fixture_declares_typed_reason_codes(fixture: dict[str, Any]) -> bool:
    expected_static = fixture.get("expected_static")
    if not isinstance(expected_static, dict):
        return False
    return "reason_codes" in expected_static


def reason_codes_snippet(reason_codes: list[dict[str, Any]]) -> str:
    return json.dumps(reason_codes, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 static gate artifact から reason_codes 候補を表示する "
            "non-production helper。fixture JSON は書き換えず、current fixture schema に "
            "typed field が未導入であることも明示する。"
        )
    )
    parser.add_argument("fixture_path")
    parser.add_argument("artifact_path")
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    fixture_path = Path(args.fixture_path)
    artifact_path = Path(args.artifact_path)

    fixture = read_json(fixture_path)
    artifact = read_json(artifact_path)

    if fixture_declares_typed_reason_codes(fixture):
        print(
            "fixture already contains unsupported expected_static.reason_codes field",
            file=sys.stderr,
        )
        return 2

    reason_codes_scope, reason_codes = read_actual_reason_code_candidates(artifact)

    print(f"fixture: {fixture_path}")
    print(f"artifact: {artifact_path}")
    print("current fixture-side typed reason code carrier: absent (no current fixture field)")

    if not reason_codes:
        print("detached artifact has no reason_codes suggestion")
        return 0

    if reason_codes_scope is not None:
        print(f"reason_codes_scope: {reason_codes_scope}")
    print("suggested reason code rows (reference-only):")
    print(reason_codes_snippet(reason_codes))
    return 0


if __name__ == "__main__":
    sys.exit(main())
