#!/usr/bin/env python3

import argparse
import json
import sys
from collections import Counter
from pathlib import Path
from typing import Any

from current_l2_checked_reasons_assist import read_fixture_checked_reasons
from current_l2_reason_codes_assist import (
    fixture_declares_typed_reason_codes,
    read_actual_reason_code_candidates,
    read_fixture_checked_reason_codes,
)


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def iter_fixture_paths(fixture_directory: Path) -> list[Path]:
    return sorted(
        path
        for path in fixture_directory.glob("*.json")
        if not path.name.endswith(".host-plan.json")
    )


def fixture_enters_evaluation(fixture: dict[str, Any]) -> bool:
    expected_runtime = fixture.get("expected_runtime")
    if not isinstance(expected_runtime, dict):
        return False
    return bool(expected_runtime.get("enters_evaluation"))


def fixture_id_for_display(fixture_path: Path, fixture: dict[str, Any]) -> str:
    fixture_id = fixture.get("fixture_id")
    if isinstance(fixture_id, str) and fixture_id:
        return fixture_id
    return fixture_path.stem


def reason_code_kinds(rows: list[dict[str, Any]]) -> list[str]:
    kinds: list[str] = []
    for row in rows:
        kind = row.get("kind")
        if isinstance(kind, str):
            kinds.append(kind)
    return kinds


def kinds_for_display(rows: list[dict[str, Any]] | None) -> str:
    if not rows:
        return "none"
    return ", ".join(reason_code_kinds(rows))


def checker_cluster_name_for_kind(kind: str) -> str | None:
    if kind in {
        "missing_lineage_assertion",
        "lineage_assertion_edge_mismatch",
        "declared_target_missing",
        "declared_target_mismatch",
    }:
        return "same_lineage_evidence_floor"
    if kind == "capability_strengthens":
        return "capability_strengthening_floor"
    if kind in {
        "missing_chain_head_option",
        "missing_predecessor_option",
        "missing_successor_option",
    }:
        return "missing_option_structure_floor"
    return None


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 static-only fixture corpus を横断し、display-only "
            "reason_codes suggestion の readiness を human-readable に要約する "
            "non-production helper。fixture JSON は書き換えない。"
        )
    )
    parser.add_argument("fixture_directory")
    parser.add_argument("artifact_directory")
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    fixture_directory = Path(args.fixture_directory)
    artifact_directory = Path(args.artifact_directory)

    fixture_paths = iter_fixture_paths(fixture_directory)
    static_rows: list[dict[str, Any]] = []
    runtime_skipped = 0

    for fixture_path in fixture_paths:
        fixture = read_json(fixture_path)
        if fixture_enters_evaluation(fixture):
            runtime_skipped += 1
            continue

        if fixture_declares_typed_reason_codes(fixture):
            print(
                "fixture already contains unsupported expected_static.reason_codes field: "
                f"{fixture_path}",
                file=sys.stderr,
            )
            return 2

        artifact_path = artifact_directory / f"{fixture_path.stem}.static-gate.json"
        artifact = read_json(artifact_path)

        checked_reasons = read_fixture_checked_reasons(fixture)
        _, reason_code_rows = read_actual_reason_code_candidates(artifact)
        fixture_checked_reason_codes = read_fixture_checked_reason_codes(fixture)
        static_rows.append(
            {
                "fixture_id": fixture_id_for_display(fixture_path, fixture),
                "checked_reasons_present": checked_reasons is not None,
                "checked_reason_codes_present": fixture_checked_reason_codes is not None,
                "checked_reason_codes_rows": fixture_checked_reason_codes,
                "reason_code_kinds": reason_code_kinds(reason_code_rows),
                "actual_reason_code_rows": reason_code_rows,
            }
        )

    kind_counter = Counter()
    cluster_counter = Counter()
    for row in static_rows:
        kind_counter.update(row["reason_code_kinds"])
        for kind in row["reason_code_kinds"]:
            cluster_name = checker_cluster_name_for_kind(kind)
            if cluster_name is not None:
                cluster_counter.update([cluster_name])

    rows_with_suggestions = [
        row for row in static_rows if row["reason_code_kinds"]
    ]
    rows_without_suggestions = [
        row for row in static_rows if not row["reason_code_kinds"]
    ]
    stable_coexistence_rows = [
        row
        for row in static_rows
        if row["checked_reasons_present"]
        and row["checked_reason_codes_present"]
        and row["checked_reason_codes_rows"] == row["actual_reason_code_rows"]
    ]
    missing_checked_reasons_rows = [
        row
        for row in static_rows
        if row["checked_reason_codes_present"] and not row["checked_reasons_present"]
    ]
    mismatch_rows = [
        row
        for row in static_rows
        if row["checked_reason_codes_present"]
        and row["checked_reason_codes_rows"] != row["actual_reason_code_rows"]
    ]

    print(f"fixture directory: {fixture_directory}")
    print(f"artifact directory: {artifact_directory}")
    print(f"static-only fixtures scanned: {len(static_rows)}")
    print(f"runtime fixtures skipped: {runtime_skipped}")
    print(
        "fixtures with checked_reasons: "
        + str(sum(1 for row in static_rows if row["checked_reasons_present"]))
    )
    print(
        "fixtures with reason_codes suggestions: " + str(len(rows_with_suggestions))
    )
    print(
        "fixtures with checked_reason_codes: "
        + str(sum(1 for row in static_rows if row["checked_reason_codes_present"]))
    )
    print(
        "fixtures with stable coexistence anchors: "
        + str(len(stable_coexistence_rows))
    )
    print(
        "fixtures with checked_reason_codes but missing checked_reasons: "
        + str(len(missing_checked_reasons_rows))
    )
    print(
        "fixtures with checked_reason_codes mismatching actual suggestion: "
        + str(len(mismatch_rows))
    )
    print("suggested reason-code kinds:")
    if kind_counter:
        for kind, count in sorted(kind_counter.items()):
            print(f"  - {kind}: {count}")
    else:
        print("  - none")
    print("checker cluster coverage:")
    if cluster_counter:
        for cluster_name, count in sorted(cluster_counter.items()):
            print(f"  - {cluster_name}: {count}")
    else:
        print("  - none")

    print("fixtures with reason_codes suggestions:")
    if rows_with_suggestions:
        for row in rows_with_suggestions:
            checked = "present" if row["checked_reasons_present"] else "absent"
            typed = "present" if row["checked_reason_codes_present"] else "absent"
            kinds = ", ".join(row["reason_code_kinds"])
            print(
                f"  - {row['fixture_id']} "
                f"[checked_reasons={checked}, checked_reason_codes={typed}] "
                f"kinds={kinds}"
            )
    else:
        print("  - none")

    print("fixtures without reason_codes suggestions:")
    if rows_without_suggestions:
        for row in rows_without_suggestions:
            checked = "present" if row["checked_reasons_present"] else "absent"
            typed = "present" if row["checked_reason_codes_present"] else "absent"
            print(
                f"  - {row['fixture_id']} "
                f"[checked_reasons={checked}, checked_reason_codes={typed}]"
            )
    else:
        print("  - none")

    print("fixtures needing coexistence follow-up:")
    follow_up_rows: list[str] = []
    for row in missing_checked_reasons_rows:
        follow_up_rows.append(
            f"  - {row['fixture_id']} [missing checked_reasons, "
            f"kinds={kinds_for_display(row['actual_reason_code_rows'])}]"
        )
    for row in mismatch_rows:
        follow_up_rows.append(
            f"  - {row['fixture_id']} [checked_reason_codes mismatch, "
            f"fixture={kinds_for_display(row['checked_reason_codes_rows'])}, "
            f"actual={kinds_for_display(row['actual_reason_code_rows'])}]"
        )
    if follow_up_rows:
        for line in follow_up_rows:
            print(line)
    else:
        print("  - none")

    return 0


if __name__ == "__main__":
    sys.exit(main())
