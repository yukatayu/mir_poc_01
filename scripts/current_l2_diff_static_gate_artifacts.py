#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


CORE_FIELDS = [
    "static_verdict",
    "reasons",
]


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_checker_core(document: dict[str, Any]) -> dict[str, Any]:
    checker = document.get("checker_core")
    if not isinstance(checker, dict):
        raise ValueError("missing object field: checker_core")
    return checker


def compare_checker_core(left: dict[str, Any], right: dict[str, Any]) -> list[str]:
    differences: list[str] = []
    for field in CORE_FIELDS:
        left_value = left.get(field)
        right_value = right.get(field)
        if left_value != right_value:
            differences.append(
                f"- checker_core.{field}: left={json.dumps(left_value, ensure_ascii=False)} "
                f"right={json.dumps(right_value, ensure_ascii=False)}"
            )
    return differences


def compare_reference_section(
    label: str, left: dict[str, Any], right: dict[str, Any]
) -> list[str]:
    if left == right:
        return []
    return [
        f"- {label}: left={json.dumps(left, ensure_ascii=False)} "
        f"right={json.dumps(right, ensure_ascii=False)}"
    ]


def build_parser() -> argparse.ArgumentParser:
    return argparse.ArgumentParser(
        description=(
            "current L2 static gate artifact の checker_core を比較する最小 helper。"
            " fixture_context は参考表示に留める。"
        )
    )


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    parser.add_argument("left", help="left static gate artifact path")
    parser.add_argument("right", help="right static gate artifact path")
    args = parser.parse_args(argv)

    left_path = Path(args.left)
    right_path = Path(args.right)

    left_doc = load_json(left_path)
    right_doc = load_json(right_path)

    left_core = read_checker_core(left_doc)
    right_core = read_checker_core(right_doc)
    checker_differences = compare_checker_core(left_core, right_core)

    context_differences = compare_reference_section(
        "fixture_context",
        left_doc.get("fixture_context", {}),
        right_doc.get("fixture_context", {}),
    )
    detached_noncore_differences = compare_reference_section(
        "detached_noncore",
        left_doc.get("detached_noncore", {}),
        right_doc.get("detached_noncore", {}),
    )

    print("=== current L2 static gate artifact diff ===")
    print(f"left : {left_path}")
    print(f"right: {right_path}")

    if checker_differences:
        print("")
        print("checker_core differences:")
        for line in checker_differences:
            print(line)
    else:
        print("")
        print("checker_core: exact-compare core matched")

    if context_differences or detached_noncore_differences:
        print("")
        print("reference-only differences:")
        for line in context_differences:
            print(line)
        for line in detached_noncore_differences:
            print(line)

    print("")
    print("note: static gate artifact は runtime trace / must_explain を含めない")

    return 1 if checker_differences else 0


if __name__ == "__main__":
    sys.exit(main())
