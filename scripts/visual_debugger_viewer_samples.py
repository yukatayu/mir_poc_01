#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import defaultdict
from functools import lru_cache
from pathlib import Path
from typing import Any


SCRIPT_PATH = Path(__file__).resolve()
SCRIPT_DIR = SCRIPT_PATH.parent
REPO_ROOT = SCRIPT_DIR.parent

VIEWER_PROTOTYPE_SCOPE = "first_public_prototype_over_typed_inventories"
PROTOTYPE_BOUNDARY = (
    "typed public prototype inventory over helper/runtime surfaces; "
    "not a final public viewer API"
)
VIEWER_PANEL_LANES = [
    "panel_id",
    "panel_kind",
    "label",
    "authority",
    "redaction",
    "retention_scope",
    "source_refs",
    "focus_refs",
    "notes",
]
VIEWER_TELEMETRY_LANES = [
    "telemetry_id",
    "telemetry_kind",
    "label",
    "authority",
    "redaction",
    "retention_scope",
    "source_refs",
    "channel",
    "value_summary",
    "notes",
]
KEPT_LATER_GATES = [
    "final_public_viewer_api",
    "final_public_visualization_schema",
    "final_public_telemetry_schema",
    "retention_policy",
    "multi_tenant_telemetry_service",
    "event_dag_runtime",
    "place_graph_view",
    "effect_route_graph_view",
    "proof_obligation_graph_view",
    "witness_timeline_view",
    "performance_telemetry_service",
    "ide_embedding",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples",
    "python3 scripts/visual_debugger_viewer_samples.py list --format json",
    "python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json",
    "python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-03 --format json",
    "python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-04 --format json",
    "python3 scripts/visual_debugger_viewer_samples.py check-all --format json",
    "python3 scripts/visual_debugger_viewer_samples.py closeout --format json",
    "python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json",
    "python3 scripts/sugoroku_world_samples.py closeout --format json",
    "python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json",
    "python3 scripts/typed_external_boundary_samples.py run EXT-03 --format json",
    "cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json",
]


def _relative(path: Path) -> str:
    return str(path.relative_to(REPO_ROOT))


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


def _summary_focus_refs(summary: dict[str, Any]) -> list[str]:
    refs: list[str] = []
    for key in ("telemetry_refs", "observer_views", "envelope_ids"):
        value = summary.get(key)
        if isinstance(value, list):
            refs.extend(str(item) for item in value)
    membership_frontier = summary.get("membership_frontier")
    if isinstance(membership_frontier, dict):
        refs.extend(str(item) for item in membership_frontier.get("active_players", []))
    return refs


def _normalize_helper_panel(row: dict[str, Any]) -> dict[str, Any]:
    summary = row.get("summary") or {}
    return {
        "panel_id": row["view_id"],
        "panel_kind": row["view_kind"],
        "label": row["label"],
        "authority": row["authority"],
        "redaction": row["redaction"],
        "retention_scope": row["retention_scope"],
        "source_refs": list(row.get("source_refs", [])),
        "focus_refs": _summary_focus_refs(summary),
        "notes": list(row.get("notes", [])),
    }


def _normalize_runtime_panel(row: dict[str, Any]) -> dict[str, Any]:
    focus_refs = list(row.get("focus_subjects", []))
    focus_refs.extend(str(item) for item in row.get("layer_signature_refs", []))
    focus_refs.extend(str(item) for item in row.get("message_envelope_refs", []))
    return {
        "panel_id": row["view_name"],
        "panel_kind": row["view_kind"],
        "label": row["label"],
        "authority": row["authority"],
        "redaction": row["redaction"],
        "retention_scope": row["retention_scope"],
        "source_refs": list(row.get("source_refs", [])),
        "focus_refs": focus_refs,
        "notes": list(row.get("notes", [])),
    }


def _helper_value_summary(fields: dict[str, Any]) -> str:
    for key in (
        "dispatch_outcome",
        "next_owner",
        "property",
        "visible_roll_count",
        "state",
        "request_envelope",
    ):
        if key in fields:
            return str(fields[key])
    if fields:
        first_key = next(iter(fields))
        return f"{first_key}={fields[first_key]}"
    return "n/a"


def _normalize_helper_telemetry(row: dict[str, Any]) -> dict[str, Any]:
    fields = row.get("fields") or {}
    return {
        "telemetry_id": row["row_id"],
        "telemetry_kind": row["row_kind"],
        "label": row["label"],
        "authority": row["authority"],
        "redaction": row["redaction"],
        "retention_scope": row["retention_scope"],
        "source_refs": list(row.get("source_refs", [])),
        "channel": str(fields.get("place") or fields.get("request_envelope") or "n/a"),
        "value_summary": _helper_value_summary(fields),
        "notes": list(row.get("notes", [])),
    }


def _normalize_runtime_telemetry(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "telemetry_id": row["row_name"],
        "telemetry_kind": row["row_kind"],
        "label": row["label"],
        "authority": row["authority"],
        "redaction": row["redaction"],
        "retention_scope": row["retention_scope"],
        "source_refs": list(row.get("source_refs", [])),
        "channel": row.get("channel", "n/a"),
        "value_summary": f"{row.get('measurement', 'value')}={row.get('value', 'n/a')}",
        "notes": list(row.get("notes", [])),
    }


def _canonicalize_duplicate_ids(
    rows: list[dict[str, Any]], *, id_key: str
) -> list[dict[str, Any]]:
    seen: dict[str, int] = defaultdict(int)
    normalized_rows: list[dict[str, Any]] = []
    for original_row in rows:
        row = dict(original_row)
        source_id = str(row[id_key])
        seen[source_id] += 1
        ordinal = seen[source_id]
        if ordinal > 1:
            row[id_key] = f"{source_id}@{ordinal}"
            notes = list(row.get("notes", []))
            notes.append(f"canonicalized duplicate {id_key} from {source_id}")
            row["notes"] = notes
        normalized_rows.append(row)
    return normalized_rows


def _bundle_from_rows(
    *,
    panels: list[dict[str, Any]],
    telemetry_rows: list[dict[str, Any]],
    catalog_keys: list[str],
) -> dict[str, Any]:
    panels = _canonicalize_duplicate_ids(panels, id_key="panel_id")
    telemetry_rows = _canonicalize_duplicate_ids(
        telemetry_rows, id_key="telemetry_id"
    )
    return {
        "panels": panels,
        "telemetry_rows": telemetry_rows,
        "panel_ids": [row["panel_id"] for row in panels],
        "panel_kinds": sorted({row["panel_kind"] for row in panels}),
        "telemetry_ids": [row["telemetry_id"] for row in telemetry_rows],
        "telemetry_kinds": sorted({row["telemetry_kind"] for row in telemetry_rows}),
        "retention_scopes": sorted(
            {
                *[row["retention_scope"] for row in panels],
                *[row["retention_scope"] for row in telemetry_rows],
            }
        ),
        "redaction_policies": sorted(
            {
                *[row["redaction"] for row in panels],
                *[row["redaction"] for row in telemetry_rows],
            }
        ),
        "catalog_keys": list(catalog_keys),
    }


def _normalize_sugoroku_visualization(payload: dict[str, Any]) -> dict[str, Any]:
    panels = [_normalize_helper_panel(row) for row in payload.get("visualization_views", [])]
    telemetry_rows = [
        _normalize_helper_telemetry(row) for row in payload.get("telemetry_rows", [])
    ]
    return _bundle_from_rows(panels=panels, telemetry_rows=telemetry_rows, catalog_keys=[])


def _normalize_sugoroku_closeout(payload: dict[str, Any]) -> dict[str, Any]:
    panels = [_normalize_helper_panel(row) for row in payload.get("visualization_views", [])]
    telemetry_rows = [
        _normalize_helper_telemetry(row) for row in payload.get("telemetry_rows", [])
    ]
    return _bundle_from_rows(
        panels=panels,
        telemetry_rows=telemetry_rows,
        catalog_keys=[
            "hotplug_scope",
            "hotplug_view_ids",
            "hotplug_telemetry_row_ids",
            "signature_lanes",
            "layer_signature_lanes",
        ],
    )


def _normalize_route_trace(payload: dict[str, Any]) -> dict[str, Any]:
    view = payload["visualization_view"]
    panel = {
        "panel_id": view["view_id"],
        "panel_kind": view["view_kind"],
        "label": view["label"],
        "authority": view["authority"],
        "redaction": view["redaction"],
        "retention_scope": view["retention_scope"],
        "source_refs": list(view.get("source_refs", [])),
        "focus_refs": list(view.get("summary", {}).get("envelope_ids", [])),
        "notes": list(payload.get("what_it_proves", [])),
    }
    telemetry_rows = [
        {
            "telemetry_id": row["envelope_id"],
            "telemetry_kind": "route_hop",
            "label": row["payload_kind"],
            "authority": row["authority"],
            "redaction": row["redaction"],
            "retention_scope": row["retention_scope"],
            "source_refs": list(row.get("source_refs", [])),
            "channel": row["transport_seam"],
            "value_summary": row["dispatch_outcome"],
            "notes": [f"hop_index={row['hop_index']}"],
        }
        for row in payload.get("observer_route_trace", [])
    ]
    return _bundle_from_rows(
        panels=[panel],
        telemetry_rows=telemetry_rows,
        catalog_keys=["what_it_proves", "what_it_does_not_prove"],
    )


def _normalize_runtime_closeout(payload: dict[str, Any]) -> dict[str, Any]:
    panels = [_normalize_runtime_panel(row) for row in payload.get("visualization_views", [])]
    telemetry_rows = [
        _normalize_runtime_telemetry(row) for row in payload.get("telemetry_rows", [])
    ]
    return _bundle_from_rows(
        panels=panels,
        telemetry_rows=telemetry_rows,
        catalog_keys=[
            "visualization_view_lanes",
            "telemetry_row_lanes",
            "retention_scope_names",
            "reserved_visualization_view_names",
        ],
    )


def _normalize_typed_external(payload: dict[str, Any]) -> dict[str, Any]:
    host_boundary = payload["host_boundary"]
    view = payload["visualization_view"]
    panel = {
        "panel_id": view["view_name"],
        "panel_kind": "message_route",
        "label": view["label"],
        "authority": view["authority"],
        "redaction": view["redaction"],
        "retention_scope": host_boundary["scope"],
        "source_refs": list(view.get("source_refs", [])),
        "focus_refs": list(host_boundary["non_collapse_lanes"]),
        "notes": [
            "visualization restriction can be shown without freezing final viewer contract"
        ],
    }
    telemetry_rows = [
        {
            "telemetry_id": row["envelope_id"],
            "telemetry_kind": row["payload_kind"],
            "label": row["label"],
            "authority": row["authority"],
            "redaction": view["redaction"],
            "retention_scope": host_boundary["scope"],
            "source_refs": [row["envelope_id"]],
            "channel": row["transport"],
            "value_summary": row["dispatch_outcome"],
            "notes": list(row.get("notes", [])),
        }
        for row in payload.get("route_trace", [])
    ]
    return _bundle_from_rows(
        panels=[panel],
        telemetry_rows=telemetry_rows,
        catalog_keys=[
            "host_boundary_scope",
            "host_boundary_lanes",
            "non_collapse_lanes",
        ],
    )


@lru_cache(maxsize=1)
def _collect_anchor_snapshots() -> dict[str, dict[str, Any]]:
    sugoroku_visualization = _run_json_command(
        [
            "python3",
            "scripts/sugoroku_world_samples.py",
            "run",
            "03_roll_publish_handoff",
            "--debug",
            "visualization",
            "--format",
            "json",
        ]
    )
    sugoroku_closeout = _run_json_command(
        [
            "python3",
            "scripts/sugoroku_world_samples.py",
            "closeout",
            "--format",
            "json",
        ]
    )
    network_route_trace = _run_json_command(
        [
            "python3",
            "scripts/network_transport_samples.py",
            "run",
            "NET-05",
            "--debug",
            "route-trace",
            "--format",
            "json",
        ]
    )
    runtime_closeout = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--bin",
            "mir-clean-near-end",
            "--",
            "closeout",
            "--format",
            "json",
        ]
    )
    typed_external = _run_json_command(
        [
            "python3",
            "scripts/typed_external_boundary_samples.py",
            "run",
            "EXT-03",
            "--format",
            "json",
        ]
    )
    return {
        "helper_sugoroku_visualization": _normalize_sugoroku_visualization(
            sugoroku_visualization
        ),
        "helper_sugoroku_closeout_catalog": _normalize_sugoroku_closeout(
            sugoroku_closeout
        ),
        "network_route_trace_bundle": _normalize_route_trace(network_route_trace),
        "runtime_closeout_catalog": _normalize_runtime_closeout(runtime_closeout),
        "typed_external_host_route_bundle": _normalize_typed_external(typed_external),
    }


BUNDLE_ROWS = [
    {
        "bundle_id": "P16-VIEW-01",
        "bundle_label": "sugoroku_helper_view_flow_bundle",
        "bundle_kind": "public_prototype_bundle",
        "anchor_kind": "helper_sugoroku_visualization",
        "alignment_rules": [
            {
                "field": "panel_ids",
                "kind": "contains_all",
                "value": [
                    "turn_timeline",
                    "message_route",
                    "verification_summary",
                    "projection_view",
                ],
                "review_category": "helper_view_alignment",
            },
            {
                "field": "telemetry_ids",
                "kind": "contains_all",
                "value": ["roll_request#1", "handoff_notice#1"],
                "review_category": "helper_telemetry_alignment",
            },
            {
                "field": "retention_scopes",
                "kind": "contains",
                "value": "helper_local_ephemeral",
                "review_category": "helper_retention_floor",
            },
            {
                "field": "redaction_policies",
                "kind": "contains",
                "value": "projection_summary_only",
                "review_category": "projection_panel_boundary",
            },
        ],
    },
    {
        "bundle_id": "P16-VIEW-02",
        "bundle_label": "sugoroku_helper_closeout_catalog_bundle",
        "bundle_kind": "public_prototype_bundle",
        "anchor_kind": "helper_sugoroku_closeout_catalog",
        "alignment_rules": [
            {
                "field": "panel_ids",
                "kind": "contains_all",
                "value": ["attach_lifecycle", "membership_snapshot", "detach_lifecycle"],
                "review_category": "hotplug_membership_alignment",
            },
            {
                "field": "telemetry_ids",
                "kind": "contains_all",
                "value": [
                    "attach_activation#1",
                    "late_join_membership#1",
                    "detach_boundary#1",
                ],
                "review_category": "hotplug_membership_telemetry",
            },
            {
                "field": "catalog_keys",
                "kind": "contains_all",
                "value": [
                    "hotplug_scope",
                    "hotplug_view_ids",
                    "hotplug_telemetry_row_ids",
                    "signature_lanes",
                    "layer_signature_lanes",
                ],
                "review_category": "helper_catalog_boundary",
            },
            {
                "field": "retention_scopes",
                "kind": "contains",
                "value": "helper_local_ephemeral",
                "review_category": "helper_retention_floor",
            },
        ],
    },
    {
        "bundle_id": "P16-VIEW-03",
        "bundle_label": "observer_route_trace_bundle",
        "bundle_kind": "public_prototype_bundle",
        "anchor_kind": "network_route_trace_bundle",
        "alignment_rules": [
            {
                "field": "panel_ids",
                "kind": "contains",
                "value": "network_route_trace",
                "review_category": "route_trace_panel_alignment",
            },
            {
                "field": "telemetry_ids",
                "kind": "contains_all",
                "value": [
                    "roll_request#1",
                    "handoff_notice#1",
                    "detach_request#1",
                    "detached_roll_request#1",
                ],
                "review_category": "route_trace_hop_alignment",
            },
            {
                "field": "redaction_policies",
                "kind": "contains",
                "value": "observer_safe_route_trace",
                "review_category": "route_trace_redaction_boundary",
            },
            {
                "field": "catalog_keys",
                "kind": "contains_all",
                "value": ["what_it_proves", "what_it_does_not_prove"],
                "review_category": "route_trace_stop_line",
            },
        ],
    },
    {
        "bundle_id": "P16-VIEW-04",
        "bundle_label": "runtime_report_canonical_catalog_bundle",
        "bundle_kind": "public_prototype_bundle",
        "anchor_kind": "runtime_closeout_catalog",
        "alignment_rules": [
            {
                "field": "panel_ids",
                "kind": "contains_all",
                "value": [
                    "authority_trace_redacted_view",
                    "cross_place_projection",
                    "provider_boundary_redacted_flow",
                ],
                "review_category": "runtime_view_alignment",
            },
            {
                "field": "retention_scopes",
                "kind": "contains",
                "value": "report_local_inventory",
                "review_category": "runtime_canonical_inventory",
            },
            {
                "field": "panel_kinds",
                "kind": "contains_all",
                "value": ["audit_trace", "projection_view", "message_route"],
                "review_category": "runtime_panel_family",
            },
            {
                "field": "catalog_keys",
                "kind": "contains_all",
                "value": [
                    "visualization_view_lanes",
                    "telemetry_row_lanes",
                    "retention_scope_names",
                    "reserved_visualization_view_names",
                ],
                "review_category": "runtime_catalog_boundary",
            },
        ],
    },
    {
        "bundle_id": "P16-VIEW-05",
        "bundle_label": "typed_external_host_route_bundle",
        "bundle_kind": "public_prototype_bundle",
        "anchor_kind": "typed_external_host_route_bundle",
        "alignment_rules": [
            {
                "field": "panel_ids",
                "kind": "contains",
                "value": "room_message_route",
                "review_category": "host_view_alignment",
            },
            {
                "field": "telemetry_ids",
                "kind": "contains_all",
                "value": ["room_message_request#1", "room_message_receipt#1"],
                "review_category": "host_route_alignment",
            },
            {
                "field": "retention_scopes",
                "kind": "contains",
                "value": "helper_local_synthetic_preview",
                "review_category": "host_preview_scope",
            },
            {
                "field": "catalog_keys",
                "kind": "contains_all",
                "value": [
                    "host_boundary_scope",
                    "host_boundary_lanes",
                    "non_collapse_lanes",
                ],
                "review_category": "host_boundary_non_collapse",
            },
        ],
    },
]


def _bundle_row(bundle_id: str) -> dict[str, Any]:
    for row in BUNDLE_ROWS:
        if row["bundle_id"] == bundle_id:
            return row
    raise KeyError(f"unknown visual debugger/viewer bundle: {bundle_id}")


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
    else:  # pragma: no cover - rules remain closed in this file.
        raise ValueError(f"unsupported alignment rule kind: {kind}")
    return {
        "field": field,
        "kind": kind,
        "expected": expected,
        "actual": actual,
        "review_category": rule["review_category"],
        "passed": passed,
    }


def _run_alignment(bundle: dict[str, Any], anchors: dict[str, dict[str, Any]]) -> dict[str, Any]:
    anchor = anchors[bundle["anchor_kind"]]
    checks = [_evaluate_rule(anchor, rule) for rule in bundle["alignment_rules"]]
    return {
        "live_bundle": anchor,
        "checks_passed": [check for check in checks if check["passed"]],
        "checks_failed": [check for check in checks if not check["passed"]],
        "alignment_passed": all(check["passed"] for check in checks),
        "review_categories": sorted({check["review_category"] for check in checks}),
    }


def _actualized_panel_kinds(anchors: dict[str, dict[str, Any]]) -> list[str]:
    return sorted(
        {
            row["panel_kind"]
            for anchor in anchors.values()
            for row in anchor.get("panels", [])
        }
    )


def _actualized_telemetry_kinds(anchors: dict[str, dict[str, Any]]) -> list[str]:
    return sorted(
        {
            row["telemetry_kind"]
            for anchor in anchors.values()
            for row in anchor.get("telemetry_rows", [])
        }
    )


def list_samples() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for row in BUNDLE_ROWS:
        rows.append(
            {
                "bundle_id": row["bundle_id"],
                "bundle_label": row["bundle_label"],
                "bundle_kind": row["bundle_kind"],
                "anchor_kind": row["anchor_kind"],
                "viewer_scope": VIEWER_PROTOTYPE_SCOPE,
            }
        )
    return rows


def run_sample(bundle_id: str) -> dict[str, Any]:
    bundle = _bundle_row(bundle_id)
    alignment = _run_alignment(bundle, _collect_anchor_snapshots())
    return {
        "bundle_id": bundle["bundle_id"],
        "bundle_label": bundle["bundle_label"],
        "bundle_kind": bundle["bundle_kind"],
        "family": "visual-debugger-viewer",
        "viewer_scope": VIEWER_PROTOTYPE_SCOPE,
        "bundle_boundary": PROTOTYPE_BOUNDARY,
        "anchor_kind": bundle["anchor_kind"],
        "live_bundle": alignment["live_bundle"],
        "checks_passed": alignment["checks_passed"],
        "checks_failed": alignment["checks_failed"],
        "alignment_passed": alignment["alignment_passed"],
        "review_categories": alignment["review_categories"],
    }


def check_all() -> dict[str, Any]:
    anchors = _collect_anchor_snapshots()
    passed: list[str] = []
    failed: list[str] = []
    failure_details: dict[str, list[dict[str, Any]]] = {}
    for row in BUNDLE_ROWS:
        result = _run_alignment(row, anchors)
        if result["checks_failed"]:
            failed.append(row["bundle_id"])
            failure_details[row["bundle_id"]] = result["checks_failed"]
        else:
            passed.append(row["bundle_id"])
    return {
        "viewer_scope": VIEWER_PROTOTYPE_SCOPE,
        "bundle_boundary": PROTOTYPE_BOUNDARY,
        "bundle_count": len(BUNDLE_ROWS),
        "passed": passed,
        "failed": failed,
        "failure_details": failure_details,
        "review_categories": sorted(
            {rule["review_category"] for row in BUNDLE_ROWS for rule in row["alignment_rules"]}
        ),
    }


def closeout() -> dict[str, Any]:
    anchors = _collect_anchor_snapshots()
    return {
        "viewer_prototype_scope": VIEWER_PROTOTYPE_SCOPE,
        "prototype_boundary": PROTOTYPE_BOUNDARY,
        "bundle_count": len(BUNDLE_ROWS),
        "prototype_bundles": [
            {
                "bundle_id": row["bundle_id"],
                "bundle_label": row["bundle_label"],
                "bundle_kind": row["bundle_kind"],
                "anchor_kind": row["anchor_kind"],
            }
            for row in BUNDLE_ROWS
        ],
        "viewer_panel_lanes": list(VIEWER_PANEL_LANES),
        "viewer_telemetry_lanes": list(VIEWER_TELEMETRY_LANES),
        "actualized_panel_kinds": _actualized_panel_kinds(anchors),
        "actualized_telemetry_kinds": _actualized_telemetry_kinds(anchors),
        "kept_later_gates": list(KEPT_LATER_GATES),
        "validation_floor": list(VALIDATION_FLOOR),
        "helper_script": _relative(SCRIPT_PATH),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["VIEWER PROTOTYPE BUNDLES"]
        for row in payload:
            lines.append(
                f"- {row['bundle_id']} {row['bundle_label']} "
                f"[{row['bundle_kind']}]"
            )
        return "\n".join(lines)
    if not isinstance(payload, dict):
        return str(payload)
    if "failure_details" in payload and "passed" in payload:
        lines = ["VIEWER CHECK-ALL SUMMARY"]
        lines.append(f"- bundle count: {payload['bundle_count']}")
        lines.append(f"- passed: {', '.join(payload['passed']) or '-'}")
        lines.append(f"- failed: {', '.join(payload['failed']) or 'none'}")
        return "\n".join(lines)
    if "viewer_panel_lanes" in payload:
        lines = ["VIEWER CLOSEOUT SUMMARY"]
        lines.append(f"- viewer_prototype_scope: {payload['viewer_prototype_scope']}")
        lines.append(f"- prototype_boundary: {payload['prototype_boundary']}")
        lines.append(
            "- actualized_panel_kinds: "
            + ", ".join(payload["actualized_panel_kinds"])
        )
        lines.append("- kept_later_gates: " + ", ".join(payload["kept_later_gates"]))
        return "\n".join(lines)
    lines = [f"VIEWER BUNDLE {payload['bundle_id']}"]
    lines.append(f"- label: {payload['bundle_label']}")
    lines.append(f"- anchor_kind: {payload['anchor_kind']}")
    lines.append(f"- alignment_passed: {payload['alignment_passed']}")
    lines.append("ALIGNMENT CHECKS")
    for check in payload["checks_passed"]:
        lines.append(f"- ok {check['field']} {check['kind']} {check['expected']}")
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
        "sample", choices=[row["bundle_id"] for row in BUNDLE_ROWS]
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
    parser = build_parser()
    args = parser.parse_args(argv)
    if args.command == "list":
        payload = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample)
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "closeout":
        payload = closeout()
    else:  # pragma: no cover - argparse keeps this closed.
        raise AssertionError(f"unsupported command: {args.command}")
    _print_payload(payload, args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
