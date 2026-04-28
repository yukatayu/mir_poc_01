#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from functools import lru_cache
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
MANIFEST_PATH = (
    REPO_ROOT / "samples" / "generated" / "projection-placement" / "manifest.json"
)


def _relative(path: Path) -> str:
    return str(path.relative_to(REPO_ROOT))


def _load_manifest() -> dict[str, Any]:
    return json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))


def _artifact_rows() -> list[dict[str, Any]]:
    return list(_load_manifest()["artifacts"])


def _artifact_row(artifact_id: str) -> dict[str, Any]:
    for row in _artifact_rows():
        if row["artifact_id"] == artifact_id:
            return row
    raise KeyError(f"unknown projection/codegen artifact: {artifact_id}")


def _run_json_command(command: list[str]) -> Any:
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(command)}\n"
            f"stdout:\n{completed.stdout}\n"
            f"stderr:\n{completed.stderr}"
        )
    return json.loads(completed.stdout)


def _extract_projection_source(authority: str) -> str:
    match = re.fullmatch(r"InspectProjection\((.+)\)", authority)
    if not match:
        raise ValueError(f"unexpected projection authority field: {authority!r}")
    return match.group(1)


def _normalize_sugoroku_projection(payload: dict[str, Any]) -> dict[str, Any]:
    for row in payload.get("visualization_views", []):
        if row.get("view_id") != "projection_view":
            continue
        summary = row["summary"]
        return {
            "system_source": summary["system_source"],
            "server_places": list(summary["server_places"]),
            "authority_place": summary["authority_place"],
            "participant_places": list(summary["participant_places"]),
            "adapter_transport": summary["adapter_transport"],
            "observer_views": list(summary["observer_views"]),
            "membership_epoch": summary["membership_frontier"]["membership_epoch"],
            "active_players": list(summary["membership_frontier"]["active_players"]),
            "telemetry_refs": list(summary["telemetry_refs"]),
            "retention_scope": row["retention_scope"],
            "notes": list(row["notes"]),
        }
    raise KeyError("projection_view not found in sugoroku anchor output")


def _normalize_runtime_projection(payload: dict[str, Any]) -> dict[str, Any]:
    for row in payload.get("visualization_views", []):
        if row.get("view_name") != "cross_place_projection":
            continue
        focus_subjects = list(row["focus_subjects"])
        return {
            "system_source": _extract_projection_source(row["authority"]),
            "authority_place": next(
                subject.split(":", 1)[1]
                for subject in focus_subjects
                if subject.startswith("authority_place:")
            ),
            "place_refs": [
                subject.split(":", 1)[1]
                for subject in focus_subjects
                if subject.startswith("place:")
            ],
            "projection_refs": [
                subject.split(":", 1)[1]
                for subject in focus_subjects
                if subject.startswith("projection:")
            ],
            "witness_refs": [
                subject.split(":", 1)[1]
                for subject in focus_subjects
                if subject.startswith("witness:")
            ],
            "redaction_rules": list(row["redaction_rules"]),
            "message_envelope_refs": list(row["message_envelope_refs"]),
            "retention_scope": row["retention_scope"],
            "notes": list(row["notes"]),
        }
    raise KeyError("cross_place_projection not found in runtime anchor output")


@lru_cache(maxsize=1)
def _collect_anchor_snapshots() -> dict[str, dict[str, Any]]:
    sugoroku_payload = _run_json_command(
        [
            "python3",
            "scripts/sugoroku_world_samples.py",
            "run",
            "03_roll_publish_handoff",
            "--debug",
            "projection",
            "--format",
            "json",
        ]
    )
    runtime_payload = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--bin",
            "mir-clean-near-end",
            "--",
            "run-sample",
            "05_delegated_rng_service",
            "--format",
            "json",
        ]
    )
    return {
        "sugoroku_projection_view": _normalize_sugoroku_projection(sugoroku_payload),
        "clean_near_end_cross_place_projection": _normalize_runtime_projection(
            runtime_payload
        ),
    }


def _evaluate_rule(anchor: dict[str, Any], rule: dict[str, Any]) -> dict[str, Any]:
    field = rule["field"]
    kind = rule["kind"]
    expected = rule["value"]
    actual = anchor[field]
    if kind == "equals":
        passed = actual == expected
    elif kind == "contains":
        if isinstance(actual, str):
            passed = str(expected) in actual
        else:
            passed = expected in actual
    elif kind == "contains_all":
        passed = all(item in actual for item in expected)
    else:  # pragma: no cover - manifest keeps kinds closed.
        raise ValueError(f"unsupported alignment rule kind: {kind}")
    return {
        "field": field,
        "kind": kind,
        "expected": expected,
        "actual": actual,
        "review_category": rule["review_category"],
        "passed": passed,
    }


def _run_alignment(artifact: dict[str, Any], anchors: dict[str, dict[str, Any]]) -> dict[str, Any]:
    anchor = anchors[artifact["anchor_kind"]]
    checks = [_evaluate_rule(anchor, rule) for rule in artifact["alignment_rules"]]
    return {
        "live_anchor": anchor,
        "checks_passed": [check for check in checks if check["passed"]],
        "checks_failed": [check for check in checks if not check["passed"]],
        "alignment_passed": all(check["passed"] for check in checks),
        "equivalence_review_categories": sorted(
            {check["review_category"] for check in checks}
        ),
    }


def list_samples() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for row in _artifact_rows():
        rows.append(
            {
                "artifact_id": row["artifact_id"],
                "artifact_label": row["artifact_label"],
                "artifact_kind": row["artifact_kind"],
                "materialization": row["materialization"],
                "anchor_kind": row["anchor_kind"],
                "place_categories": list(row["place_categories"]),
                "manifest_path": _relative(MANIFEST_PATH),
            }
        )
    return rows


def run_sample(artifact_id: str) -> dict[str, Any]:
    manifest = _load_manifest()
    artifact = _artifact_row(artifact_id)
    alignment = _run_alignment(artifact, _collect_anchor_snapshots())
    return {
        "artifact_id": artifact["artifact_id"],
        "artifact_label": artifact["artifact_label"],
        "artifact_kind": artifact["artifact_kind"],
        "family": "projection-codegen",
        "projection_scope": manifest["projection_scope"],
        "artifact_boundary": manifest["artifact_boundary"],
        "materialization": artifact["materialization"],
        "manifest_path": _relative(MANIFEST_PATH),
        "anchor_kind": artifact["anchor_kind"],
        "place_categories": list(artifact["place_categories"]),
        "anchor_refs": list(artifact["anchor_refs"]),
        "live_anchor": alignment["live_anchor"],
        "checks_passed": alignment["checks_passed"],
        "checks_failed": alignment["checks_failed"],
        "alignment_passed": alignment["alignment_passed"],
        "equivalence_review_categories": alignment["equivalence_review_categories"],
    }


def check_all() -> dict[str, Any]:
    anchors = _collect_anchor_snapshots()
    passed: list[str] = []
    failed: list[str] = []
    failure_details: dict[str, list[dict[str, Any]]] = {}
    for row in _artifact_rows():
        result = _run_alignment(row, anchors)
        if result["checks_failed"]:
            failed.append(row["artifact_id"])
            failure_details[row["artifact_id"]] = result["checks_failed"]
        else:
            passed.append(row["artifact_id"])
    manifest = _load_manifest()
    return {
        "projection_scope": manifest["projection_scope"],
        "artifact_boundary": manifest["artifact_boundary"],
        "artifact_count": len(_artifact_rows()),
        "passed": passed,
        "failed": failed,
        "failure_details": failure_details,
        "equivalence_review_categories": list(manifest["equivalence_review_categories"]),
    }


def closeout() -> dict[str, Any]:
    manifest = _load_manifest()
    return {
        "projection_scope": manifest["projection_scope"],
        "artifact_boundary": manifest["artifact_boundary"],
        "artifact_count": len(manifest["artifacts"]),
        "generated_reserve_inventory": list(manifest["generated_reserve_inventory"]),
        "generated_bridge_artifact_inventory": [
            {
                "artifact_id": row["artifact_id"],
                "artifact_label": row["artifact_label"],
                "artifact_kind": row["artifact_kind"],
                "materialization": row["materialization"],
                "anchor_kind": row["anchor_kind"],
                "place_categories": list(row["place_categories"]),
            }
            for row in manifest["artifacts"]
        ],
        "equivalence_review_categories": list(
            manifest["equivalence_review_categories"]
        ),
        "kept_later_gates": list(manifest["kept_later_gates"]),
        "validation_floor": list(manifest["validation_floor"]),
        "helper_script": _relative(Path(__file__).resolve()),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["GENERATED RESERVE ARTIFACTS"]
        for row in payload:
            lines.append(
                f"- {row['artifact_id']} {row['artifact_label']} "
                f"[{','.join(row['place_categories'])}] {row['materialization']}"
            )
        return "\n".join(lines)
    if not isinstance(payload, dict):
        return str(payload)
    if "failure_details" in payload and "passed" in payload:
        lines = ["CHECK-ALL SUMMARY"]
        lines.append(f"- artifact count: {payload['artifact_count']}")
        lines.append(f"- passed: {', '.join(payload['passed']) or '-'}")
        lines.append(f"- failed: {', '.join(payload['failed']) or 'none'}")
        return "\n".join(lines)
    if "generated_reserve_inventory" in payload:
        lines = ["CLOSEOUT SUMMARY"]
        lines.append(f"- projection_scope: {payload['projection_scope']}")
        lines.append(f"- artifact_boundary: {payload['artifact_boundary']}")
        lines.append(
            "- generated_bridge_artifacts: "
            + ", ".join(
                row["artifact_id"]
                for row in payload["generated_bridge_artifact_inventory"]
            )
        )
        lines.append(
            "- equivalence_review_categories: "
            + ", ".join(payload["equivalence_review_categories"])
        )
        return "\n".join(lines)
    lines = [f"ARTIFACT {payload['artifact_id']}"]
    lines.append(f"- label: {payload['artifact_label']}")
    lines.append(f"- anchor_kind: {payload['anchor_kind']}")
    lines.append(f"- materialization: {payload['materialization']}")
    lines.append(f"- alignment_passed: {payload['alignment_passed']}")
    lines.append("ALIGNMENT CHECKS")
    for check in payload["checks_passed"]:
        lines.append(
            f"- ok {check['field']} {check['kind']} {check['expected']}"
        )
    for check in payload["checks_failed"]:
        lines.append(
            f"- fail {check['field']} {check['kind']} {check['expected']} actual={check['actual']}"
        )
    return "\n".join(lines)


def _print_payload(payload: Any, output_format: str) -> None:
    if output_format == "json":
        json.dump(payload, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return
    sys.stdout.write(format_pretty(payload))
    sys.stdout.write("\n")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="command", required=True)

    list_parser = sub.add_parser("list")
    list_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    run_parser = sub.add_parser("run")
    run_parser.add_argument(
        "sample", choices=[row["artifact_id"] for row in _artifact_rows()]
    )
    run_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    check_parser = sub.add_parser("check-all")
    check_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    closeout_parser = sub.add_parser("closeout")
    closeout_parser.add_argument(
        "--format", choices=["pretty", "json"], default="pretty"
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    if args.command == "list":
        _print_payload(list_samples(), args.format)
    elif args.command == "run":
        _print_payload(run_sample(args.sample), args.format)
    elif args.command == "check-all":
        _print_payload(check_all(), args.format)
    elif args.command == "closeout":
        _print_payload(closeout(), args.format)
    else:  # pragma: no cover - argparse keeps this unreachable.
        raise AssertionError(f"unknown command {args.command}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
