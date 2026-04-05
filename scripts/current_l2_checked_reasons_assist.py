#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_fixture_checked_reasons(fixture: dict[str, Any]) -> list[str] | None:
    expected_static = fixture.get("expected_static")
    if not isinstance(expected_static, dict):
        return None
    checked_reasons = expected_static.get("checked_reasons")
    if checked_reasons is None:
        return None
    return [str(reason) for reason in checked_reasons]


def read_actual_checked_reason_candidates(artifact: dict[str, Any]) -> list[str]:
    checker_core = artifact.get("checker_core")
    if not isinstance(checker_core, dict):
        return []
    reasons = checker_core.get("reasons")
    if not isinstance(reasons, list):
        return []
    return [str(reason) for reason in reasons]


def checked_reasons_snippet(reasons: list[str]) -> str:
    return json.dumps(
        {"checked_reasons": reasons},
        ensure_ascii=False,
        indent=2,
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 static gate artifact から checked_reasons 候補を表示する "
            "non-production helper。fixture JSON は書き換えない。"
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

    current_checked_reasons = read_fixture_checked_reasons(fixture)
    actual_reasons = read_actual_checked_reason_candidates(artifact)

    print(f"fixture: {fixture_path}")
    print(f"artifact: {artifact_path}")

    if current_checked_reasons is None:
        print("current expected_static.checked_reasons: absent")
    else:
        print(
            "current expected_static.checked_reasons: "
            + json.dumps(current_checked_reasons, ensure_ascii=False)
        )

    if not actual_reasons:
        print("actual static gate reasons are empty; no checked_reasons suggestion")
        return 0

    if current_checked_reasons == actual_reasons:
        print("current expected_static.checked_reasons already matches actual static gate reasons")
        return 0

    print("suggested snippet:")
    print(checked_reasons_snippet(actual_reasons))
    return 0


if __name__ == "__main__":
    sys.exit(main())
