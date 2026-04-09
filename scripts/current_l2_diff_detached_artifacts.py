#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


CORE_FIELDS = [
    "static_verdict",
    "entered_evaluation",
    "terminal_outcome",
    "event_kinds",
    "non_admissible_metadata",
    "narrative_explanations",
]


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_payload_core(document: dict[str, Any]) -> dict[str, Any]:
    payload = document.get("payload_core")
    if not isinstance(payload, dict):
        raise ValueError("missing object field: payload_core")
    return payload


def compare_payload_core(left: dict[str, Any], right: dict[str, Any]) -> list[str]:
    differences: list[str] = []
    for field in CORE_FIELDS:
        left_value = left.get(field)
        right_value = right.get(field)
        if left_value != right_value:
            differences.append(
                f"- payload_core.{field}: left={json.dumps(left_value, ensure_ascii=False)} "
                f"right={json.dumps(right_value, ensure_ascii=False)}"
            )
    return differences


def compare_reference_section(
    label: str, left: dict[str, Any], right: dict[str, Any]
) -> list[str]:
    if left == right:
        return []
    if isinstance(left, dict) and isinstance(right, dict):
        differences: list[str] = []
        for field in sorted(set(left.keys()) | set(right.keys())):
            left_value = left.get(field)
            right_value = right.get(field)
            if left_value != right_value:
                differences.append(
                    f"- {label}.{field}: left={json.dumps(left_value, ensure_ascii=False)} "
                    f"right={json.dumps(right_value, ensure_ascii=False)}"
                )
        return differences
    return [
        f"- {label}: left={json.dumps(left, ensure_ascii=False)} "
        f"right={json.dumps(right, ensure_ascii=False)}"
    ]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 detached artifact の payload_core を比較する最小 helper。"
            " detached_noncore と bundle_context は参考表示に留める。"
        )
    )
    parser.add_argument("left", help="left artifact path")
    parser.add_argument("right", help="right artifact path")
    args = parser.parse_args(argv)

    left_path = Path(args.left)
    right_path = Path(args.right)

    left_doc = load_json(left_path)
    right_doc = load_json(right_path)

    left_payload = read_payload_core(left_doc)
    right_payload = read_payload_core(right_doc)
    payload_differences = compare_payload_core(left_payload, right_payload)

    context_differences = compare_reference_section(
        "bundle_context",
        left_doc.get("bundle_context", {}),
        right_doc.get("bundle_context", {}),
    )
    noncore_differences = compare_reference_section(
        "detached_noncore",
        left_doc.get("detached_noncore", {}),
        right_doc.get("detached_noncore", {}),
    )

    print("=== current L2 detached artifact diff ===")
    print(f"left : {left_path}")
    print(f"right: {right_path}")

    if payload_differences:
        print("")
        print("payload_core differences:")
        for line in payload_differences:
            print(line)
    else:
        print("")
        print("payload_core: exact-compare core matched")

    if context_differences or noncore_differences:
        print("")
        print("reference-only differences:")
        for line in context_differences + noncore_differences:
            print(line)

    print("")
    print("note: must_explain と long-form explanation は比較対象に含めない")

    return 1 if payload_differences else 0


if __name__ == "__main__":
    sys.exit(main())
