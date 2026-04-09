#!/usr/bin/env python3

import argparse
import json
import sys
from pathlib import Path
from typing import Any


CORE_FIELDS = [
    "total_bundles",
    "runtime_bundles",
    "static_only_bundles",
    "passed",
    "failed",
    "bundle_failure_kind_counts_scope",
    "bundle_failure_kind_counts",
]


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_summary_core(document: dict[str, Any]) -> dict[str, Any]:
    summary = document.get("summary_core")
    if not isinstance(summary, dict):
        raise ValueError("missing object field: summary_core")
    return summary


def compare_summary_core(left: dict[str, Any], right: dict[str, Any]) -> list[str]:
    differences: list[str] = []
    for field in CORE_FIELDS:
        left_value = left.get(field)
        right_value = right.get(field)
        if left_value != right_value:
            differences.append(
                f"- summary_core.{field}: left={json.dumps(left_value, ensure_ascii=False)} "
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


def build_parser() -> argparse.ArgumentParser:
    return argparse.ArgumentParser(
        description=(
            "current L2 detached aggregate artifact の summary_core を比較する最小 helper。"
            " aggregate_context と detached_noncore は参考表示に留める。"
        )
    )


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    parser.add_argument("left", help="left aggregate artifact path")
    parser.add_argument("right", help="right aggregate artifact path")
    args = parser.parse_args(argv)

    left_path = Path(args.left)
    right_path = Path(args.right)

    left_doc = load_json(left_path)
    right_doc = load_json(right_path)

    left_summary = read_summary_core(left_doc)
    right_summary = read_summary_core(right_doc)
    summary_differences = compare_summary_core(left_summary, right_summary)

    context_differences = compare_reference_section(
        "aggregate_context",
        left_doc.get("aggregate_context", {}),
        right_doc.get("aggregate_context", {}),
    )
    noncore_differences = compare_reference_section(
        "detached_noncore",
        left_doc.get("detached_noncore", {}),
        right_doc.get("detached_noncore", {}),
    )

    print("=== current L2 detached aggregate diff ===")
    print(f"left : {left_path}")
    print(f"right: {right_path}")

    if summary_differences:
        print("")
        print("summary_core differences:")
        for line in summary_differences:
            print(line)
    else:
        print("")
        print("summary_core: typed aggregate core matched")

    if context_differences or noncore_differences:
        print("")
        print("reference-only differences:")
        for line in context_differences + noncore_differences:
            print(line)

    print("")
    print(
        "note: current comparison keeps bundle_failure_kind_counts_scope = "
        '"migrated-kinds-only" as part of the typed aggregate core'
    )

    return 1 if summary_differences else 0


if __name__ == "__main__":
    sys.exit(main())
