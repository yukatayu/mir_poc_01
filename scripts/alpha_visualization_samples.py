#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent

IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "VIS-01",
        "summary": "Event DAG export over the local runtime roll/publish/witness/handoff path.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-01-event_dag_export.expected.json",
    },
    {
        "sample_id": "VIS-02",
        "summary": "Place-catalog projection bundle over the alpha layer-insertion runtime snapshot.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-02-place_graph_export.expected.json",
    },
    {
        "sample_id": "VIS-03",
        "summary": "Route trace export over the Docker-backed alpha network boundary.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-03-route_trace_export.expected.json",
    },
    {
        "sample_id": "VIS-05",
        "summary": "Membership epoch/incarnation timeline over the local save/load stale-membership bridge.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-05-membership_timeline.expected.json",
    },
    {
        "sample_id": "VIS-06",
        "summary": "Typed hot-plug lifecycle bundle exposes attach/membership/detach surfaces.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-06-hotplug_lifecycle_view.expected.json",
    },
    {
        "sample_id": "VIS-07",
        "summary": "Fallback degradation view stays explicit at the avatar/package boundary.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-07-fallback_degradation_view.expected.json",
    },
    {
        "sample_id": "VIS-08",
        "summary": "Observer-safe route trace stays typed and redacted.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-08-observer_redacted_view.expected.json",
    },
    {
        "sample_id": "VIS-10",
        "summary": "Debug traces appear only after accepted attach and stay absent before it.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-10-on_demand_trace_only.expected.json",
    },
    {
        "sample_id": "VIS-11",
        "summary": "Accepted traces stay within retention policy and widened retention is rejected.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/visualization/vis-11-retention_policy_enforced.expected.json",
    },
]

PLANNED_ONLY_ROWS = ["VIS-04", "VIS-09", "VIS-12"]

STOP_LINES = [
    "do not treat samples/alpha/visualization as an active runnable root",
    "do not freeze a final public viewer API or telemetry schema from this runner",
    "do not mark VIS-04/09/12 implemented in this package",
    "do not treat observer-safe or redacted views as privileged admin-full visibility",
]

LIMITATIONS = [
    "thin runner over existing alpha/helper/runtime JSON emitters only",
    "no parser/front-door execution of samples/alpha/visualization/*.mir",
    "no first-class witness timeline, admin-full view, or detach-stop runtime completion",
    "no final public viewer API, retention service, or telemetry backend",
]


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented alpha visualization sample: {sample_id}")


def _run_json_command(command: list[str]) -> Any:
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-visualization",
            "source_root": "samples/alpha/visualization",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    sidecar_path = REPO_ROOT / row["expected_sidecar"]
    return json.loads(sidecar_path.read_text())


def _base_report(
    *,
    sample_id: str,
    sample_name: str,
    kind: str,
    purpose: str,
    expected: str,
    component_refs: list[dict[str, str]],
    evidence_summary: dict[str, Any],
    what_it_proves: list[str],
    what_it_does_not_prove: list[str],
    deferred: list[str],
) -> dict[str, Any]:
    return {
        "sample_id": sample_id,
        "sample_name": sample_name,
        "family": "visualization",
        "status": "implemented-thin-runner",
        "phase": "Phase 4 / 5",
        "stage": "Stage E",
        "kind": kind,
        "purpose": purpose,
        "expected": expected,
        "claims": {
            "runnable": True,
            "implemented": True,
            "active_root": False,
            "stage_e_complete": False,
        },
        "current_validation": {
            "mode": "thin_runner_over_existing_alpha_evidence",
            "runner": f"python3 scripts/alpha_visualization_samples.py run {sample_id} --format json",
            "note": (
                "Composes existing alpha/helper/runtime JSON evidence without parsing "
                "samples/alpha/visualization/*.mir or freezing a final public viewer API."
            ),
        },
        "component_refs": component_refs,
        "evidence_summary": evidence_summary,
        "terminal_outcome": "accepted",
        "reason_family": None,
        "what_it_proves": what_it_proves,
        "what_it_does_not_prove": what_it_does_not_prove,
        "deferred": deferred,
    }


def _build_vis01() -> dict[str, Any]:
    payload = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_local_runtime",
            "--",
            "local-sugoroku",
        ]
    )
    event_dag = payload["event_dag"]
    return _base_report(
        sample_id="VIS-01",
        sample_name="event_dag_export",
        kind="positive",
        purpose="event graph",
        expected="JSON/HTML view",
        component_refs=[
            {
                "family": "local-runtime",
                "sample_id": "LR-01",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku",
            }
        ],
        evidence_summary={
            "event_dag_scope": event_dag["scope"],
            "event_dag_node_count": len(event_dag["nodes"]),
            "event_dag_edge_count": len(event_dag["edges"]),
            "event_ids": [node["event_id"] for node in event_dag["nodes"]],
            "edge_relations": [edge["relation"] for edge in event_dag["edges"]],
            "current_owner": payload["current_owner"],
            "visualization_surface": "event_dag_json",
        },
        what_it_proves=[
            "the alpha local runtime already exposes a typed event DAG JSON surface",
            "roll, publish, witness, and handoff order remain explicit in the exported graph",
        ],
        what_it_does_not_prove=[
            "no final public HTML viewer",
            "no place graph or witness timeline completion",
            "no network or distributed visualization completion",
        ],
        deferred=[
            "final viewer rendering contract",
            "dedicated place graph export",
            "first-class witness timeline carrier",
        ],
    )


def _build_vis02() -> dict[str, Any]:
    payload = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_layer_insertion_runtime",
            "--",
            "debug-admin",
        ]
    )
    places = payload["runtime_snapshot"]["place_catalog"]["places"]
    place_ids = sorted(places)
    participant_place_ids = [place_id for place_id in place_ids if places[place_id] == "ParticipantPlace"]
    attachpoint_ids = [place_id for place_id in place_ids if places[place_id] == "AttachPoint"]
    world_place_ids = [place_id for place_id in place_ids if places[place_id] == "WorldPlace"]
    game_place_ids = [place_id for place_id in place_ids if places[place_id] == "SugorokuGamePlace"]
    return _base_report(
        sample_id="VIS-02",
        sample_name="place_graph_export",
        kind="positive",
        purpose="place-catalog projection",
        expected="view",
        component_refs=[
            {
                "family": "layer-insertion",
                "sample_id": "LI-01",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-admin",
            }
        ],
        evidence_summary={
            "place_count": len(place_ids),
            "place_ids": place_ids,
            "place_kinds": sorted(set(places.values())),
            "attachpoint_ids": attachpoint_ids,
            "world_place_ids": world_place_ids,
            "game_place_ids": game_place_ids,
            "participant_place_ids": participant_place_ids,
            "visualization_surface": "place_catalog_snapshot_json",
        },
        what_it_proves=[
            "the alpha runtime already exposes a typed place-catalog snapshot that can back a report-local place graph export",
            "attachpoint, world, game, and participant places remain distinct instead of being collapsed into a single host/debug bucket",
        ],
        what_it_does_not_prove=[
            "no final public place graph viewer API",
            "no cross-place equivalence or placement optimizer completion",
            "no emitted place-program synthesis or distributed topology completion",
        ],
        deferred=[
            "final public place graph viewer contract",
            "effect/proof route graph families",
            "generated place-program synthesis and placement planning",
        ],
    )


def _build_vis03() -> dict[str, Any]:
    payload = _run_json_command(
        ["python3", "scripts/alpha_network_docker_e2e.py", "run", "NET-02", "--format", "json"]
    )
    participant = payload["participant"]
    route = participant["observer_route_trace"]
    return _base_report(
        sample_id="VIS-03",
        sample_name="route_trace_export",
        kind="positive",
        purpose="envelope route",
        expected="view",
        component_refs=[
            {
                "family": "network-docker",
                "sample_id": "NET-02",
                "command": "python3 scripts/alpha_network_docker_e2e.py run NET-02 --format json",
            }
        ],
        evidence_summary={
            "route_hop_count": len(route),
            "route_hop_ids": [row["envelope_id"] for row in route],
            "redaction_policies": sorted({row["redaction"] for row in route}),
            "retention_scopes": sorted({row["retention_scope"] for row in route}),
            "auth_lane_present_values": sorted({bool(row["auth_lane_present"]) for row in route}),
            "transport_medium": participant["transport_medium"],
            "transport_surface": participant["transport_surface"],
            "visualization_surface": "observer_route_trace_json",
        },
        what_it_proves=[
            "route trace export survives a real world/participant Docker Compose boundary",
            "the transport seam remains inspectable without collapsing witness or capability lanes",
        ],
        what_it_does_not_prove=[
            "no final public viewer API",
            "no privileged admin-full visibility surface",
            "no production WAN/session/replay semantics",
        ],
        deferred=[
            "viewer-specific rendering contract",
            "place graph export",
            "admin-full visualization path",
        ],
    )


def _build_vis05() -> dict[str, Any]:
    payload = _run_json_command(
        ["python3", "scripts/alpha_cut_save_load_samples.py", "run", "CUT-17", "--format", "json"]
    )
    saved_membership = payload["saved_runtime_snapshot"]["membership"]
    restored_membership = payload["restored_runtime_snapshot"]["membership"]
    restored_members = restored_membership["members"]
    membership_update_event_ids = [
        node["event_id"]
        for node in payload["resumed_event_dag"]["nodes"]
        if node["event_kind"] == "membership_update"
    ]
    membership_principals = sorted(restored_members)
    timeline_entries = [
        {
            "entry_id": "saved_membership_frontier#1",
            "membership_epoch": saved_membership["membership_epoch"],
            "principals": sorted(saved_membership["members"]),
            "source_ref": "saved_runtime_snapshot.membership",
        },
        {
            "entry_id": "membership_advance#1",
            "membership_epoch": restored_membership["membership_epoch"],
            "principals": [
                principal
                for principal, member in sorted(restored_members.items())
                if member["joined_at_epoch"] == restored_membership["membership_epoch"]
            ],
            "source_ref": "resumed_event_dag.nodes[membership_update]",
        },
        {
            "entry_id": "restored_membership_frontier#1",
            "membership_epoch": restored_membership["membership_epoch"],
            "principals": membership_principals,
            "source_ref": "restored_runtime_snapshot.membership",
        },
    ]
    return _base_report(
        sample_id="VIS-05",
        sample_name="membership_timeline",
        kind="positive",
        purpose="epoch/incarnation",
        expected="view",
        component_refs=[
            {
                "family": "cut-save-load",
                "sample_id": "CUT-17",
                "command": "python3 scripts/alpha_cut_save_load_samples.py run CUT-17 --format json",
            }
        ],
        evidence_summary={
            "timeline_entry_count": len(timeline_entries),
            "timeline_entries": timeline_entries,
            "membership_epochs": [
                saved_membership["membership_epoch"],
                restored_membership["membership_epoch"],
            ],
            "membership_principals": membership_principals,
            "active_principals": [
                principal for principal, member in sorted(restored_members.items()) if member["active"]
            ],
            "joined_at_epochs": {
                principal: member["joined_at_epoch"]
                for principal, member in sorted(restored_members.items())
            },
            "member_incarnations": {
                principal: member["incarnation"]
                for principal, member in sorted(restored_members.items())
            },
            "membership_update_event_ids": membership_update_event_ids,
            "visualization_surface": "membership_timeline_json",
        },
        what_it_proves=[
            "existing alpha save/load evidence already exposes membership_epoch, joined_at_epoch, and incarnation fields as a report-local membership timeline bundle",
            "a later membership-frontier advance stays explicit instead of being silently collapsed into resumed dispatch success",
        ],
        what_it_does_not_prove=[
            "no distributed save/load completion or consistent-cut repair",
            "no final public membership telemetry service",
            "no witness/lease co-timeline or detach history completion",
        ],
        deferred=[
            "distributed durable membership timeline across multi-place cuts",
            "final public membership visualization contract",
            "witness/lease co-timeline and detach history surfaces",
        ],
    )


def _build_vis06() -> dict[str, Any]:
    payload = _run_json_command(
        ["python3", "scripts/visual_debugger_viewer_samples.py", "run", "P16-VIEW-02", "--format", "json"]
    )
    bundle = payload["live_bundle"]
    return _base_report(
        sample_id="VIS-06",
        sample_name="hotplug_lifecycle_view",
        kind="positive",
        purpose="request/verdict/activation",
        expected="view",
        component_refs=[
            {
                "family": "viewer-prototype",
                "sample_id": "P16-VIEW-02",
                "command": "python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-02 --format json",
            }
        ],
        evidence_summary={
            "panel_ids": bundle["panel_ids"],
            "telemetry_ids": bundle["telemetry_ids"],
            "catalog_keys": bundle["catalog_keys"],
            "retention_scopes": bundle["retention_scopes"],
            "redaction_policies": bundle["redaction_policies"],
            "visualization_surface": "typed_viewer_bundle_json",
        },
        what_it_proves=[
            "attach, membership, and detach surfaces are already bundled into a typed lifecycle view",
            "lifecycle visualization keeps redaction and retention explicit instead of treating debug output as untyped leakage",
        ],
        what_it_does_not_prove=[
            "no completed detach runtime",
            "no final public hot-plug lifecycle API",
            "no durable migration or distributed activation ordering completion",
        ],
        deferred=[
            "completed detach runtime semantics",
            "final public lifecycle service contract",
            "distributed activation ordering visualization",
        ],
    )


def _build_vis07() -> dict[str, Any]:
    payload = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_avatar_runtime",
            "--",
            "run",
            "AV-08",
        ]
    )
    representation = payload["representation_state"]
    verdict = payload["hotplug_skeleton"]["verdict"]
    return _base_report(
        sample_id="VIS-07",
        sample_name="fallback_degradation_view",
        kind="positive",
        purpose="option degradation",
        expected="view",
        component_refs=[
            {
                "family": "avatar-runtime",
                "sample_id": "AV-08",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- run AV-08",
            }
        ],
        evidence_summary={
            "requested_package_verdict_kind": verdict["verdict_kind"],
            "selected_package_id": representation["selected_package_id"],
            "selected_representation": representation["selected_representation"],
            "fallback_applied": representation["fallback_applied"],
            "fallback_reason": representation["fallback_reason"],
            "degraded_roles": representation["degraded_roles"],
            "visualization_surface": "representation_state_json",
        },
        what_it_proves=[
            "fallback remains visible and monotone at the runtime package/avatar boundary",
            "fallback extends guarded logical availability rather than the lifetime of the rejected richer runtime",
        ],
        what_it_does_not_prove=[
            "no native execution fallback",
            "no final avatar runtime API",
            "no completed package detach or migration lifecycle",
        ],
        deferred=[
            "trusted native sandbox-limited provider row",
            "runtime package detach lifecycle",
            "final public runtime package/avatar ABI",
        ],
    )


def _build_vis08() -> dict[str, Any]:
    payload = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_network_runtime",
            "--",
            "run",
            "NET-07",
        ]
    )
    route = payload["observer_route_trace"]
    return _base_report(
        sample_id="VIS-08",
        sample_name="observer_redacted_view",
        kind="positive",
        purpose="safe observer",
        expected="no raw high-label data",
        component_refs=[
            {
                "family": "network-runtime",
                "sample_id": "NET-07",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- run NET-07",
            }
        ],
        evidence_summary={
            "route_hop_count": len(route),
            "redaction_policies": sorted({row["redaction"] for row in route}),
            "retention_scopes": sorted({row["retention_scope"] for row in route}),
            "auth_lane_present_values": sorted({bool(row["auth_lane_present"]) for row in route}),
            "witness_ref_counts": [row["witness_ref_count"] for row in route],
            "transport_medium": payload["transport_medium"],
            "visualization_surface": "observer_redacted_route_trace_json",
        },
        what_it_proves=[
            "observer-safe route trace stays typed and redacted on the alpha network floor",
            "raw auth payloads are not required to inspect transport hops",
        ],
        what_it_does_not_prove=[
            "no privileged admin-full view",
            "no raw witness or auth payload viewer",
            "no final public transport or devtools ABI",
        ],
        deferred=[
            "privileged full-detail admin view",
            "final public viewer contract",
            "multi-tenant telemetry service",
        ],
    )


def _build_vis10() -> dict[str, Any]:
    accepted = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_layer_insertion_runtime",
            "--",
            "debug-admin",
        ]
    )
    rejected = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_layer_insertion_runtime",
            "--",
            "debug-non-admin",
        ]
    )
    requested_layer = accepted["attach_request"]["requested_layer"]
    return _base_report(
        sample_id="VIS-10",
        sample_name="on_demand_trace_only",
        kind="positive/perf",
        purpose="tracing only after attach",
        expected="no trace before attach",
        component_refs=[
            {
                "family": "layer-insertion",
                "sample_id": "LI-01",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-admin",
            },
            {
                "family": "layer-insertion",
                "sample_id": "LI-02",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-non-admin",
            },
        ],
        evidence_summary={
            "pre_attach_trace_count": len(accepted["pre_attach_trace_rows"]),
            "post_attach_trace_count": len(accepted["post_attach_trace_rows"]),
            "rejected_attach_trace_count": len(rejected["post_attach_trace_rows"]),
            "trace_mode": requested_layer["trace_mode"],
            "redaction_level": requested_layer["redaction_level"],
            "retention_scope": requested_layer["retention_scope"],
            "active_layers_after": accepted["active_layers_after"],
            "visualization_surface": "post_attach_trace_rows",
        },
        what_it_proves=[
            "trace rows stay absent before activation and appear only after accepted attach",
            "the current alpha debug layer remains on-demand, redacted, and authority-gated",
        ],
        what_it_does_not_prove=[
            "no detach-stops-trace runtime completion",
            "no final public debugging ABI",
            "no always-on observer tracing policy",
        ],
        deferred=[
            "post-detach trace shutdown runtime",
            "final public trace viewer contract",
            "distributed trace transport",
        ],
    )


def _build_vis11() -> dict[str, Any]:
    accepted = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_layer_insertion_runtime",
            "--",
            "debug-admin",
        ]
    )
    rejected = _run_json_command(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mirrorea_alpha_layer_insertion_runtime",
            "--",
            "incompatible",
        ]
    )
    accepted_scope = accepted["attach_request"]["requested_layer"]["retention_scope"]
    allowed_scopes = accepted["attachpoint_policy"]["allowed_retention_scopes"]
    rejected_reasons = rejected["hotplug_runtime_report"]["verdict"]["compatibility_reason_refs"]
    return _base_report(
        sample_id="VIS-11",
        sample_name="retention_policy_enforced",
        kind="negative/positive",
        purpose="telemetry retention",
        expected="no over-retention",
        component_refs=[
            {
                "family": "layer-insertion",
                "sample_id": "LI-01",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-admin",
            },
            {
                "family": "layer-insertion",
                "sample_id": "LI-05",
                "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- incompatible",
            },
        ],
        evidence_summary={
            "observed_retention_scopes": sorted({accepted_scope, *allowed_scopes}),
            "accepted_retention_scope": accepted_scope,
            "rejected_retention_scope": rejected["attach_request"]["requested_layer"]["retention_scope"],
            "rejected_reason_refs": rejected_reasons,
            "over_retention_detected": False,
            "over_retention_rejected": "retention_scope_widened" in rejected_reasons,
            "visualization_surface": "layer_retention_policy_json",
        },
        what_it_proves=[
            "accepted debug traces stay within the declared helper-local retention floor",
            "a widened retention request is rejected before activation instead of silently over-retaining telemetry",
        ],
        what_it_does_not_prove=[
            "no final public retention service",
            "no multi-tenant storage or deletion workflow",
            "no global telemetry backend completion",
        ],
        deferred=[
            "final retention policy service",
            "multi-tenant storage and deletion semantics",
            "global telemetry backend",
        ],
    )


def _build_sample_report(sample_id: str) -> dict[str, Any]:
    if sample_id == "VIS-01":
        return _build_vis01()
    if sample_id == "VIS-02":
        return _build_vis02()
    if sample_id == "VIS-03":
        return _build_vis03()
    if sample_id == "VIS-05":
        return _build_vis05()
    if sample_id == "VIS-06":
        return _build_vis06()
    if sample_id == "VIS-07":
        return _build_vis07()
    if sample_id == "VIS-08":
        return _build_vis08()
    if sample_id == "VIS-10":
        return _build_vis10()
    if sample_id == "VIS-11":
        return _build_vis11()
    raise ValueError(f"unsupported visualization sample: {sample_id}")


def _validate_expected_fields(sample_id: str, row: dict[str, Any], report: dict[str, Any]) -> None:
    if report.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: report sample_id mismatch: {report.get('sample_id')!r}"
        )
    if report.get("terminal_outcome") != row["expected_terminal_outcome"]:
        raise RuntimeError(
            f"{sample_id}: expected terminal_outcome {row['expected_terminal_outcome']!r} "
            f"but observed {report.get('terminal_outcome')!r}"
        )
    evidence = report.get("evidence_summary")
    if not isinstance(evidence, dict):
        raise RuntimeError(f"{sample_id}: missing evidence_summary")

    if sample_id == "VIS-01":
        if evidence.get("event_dag_node_count", 0) <= 0:
            raise RuntimeError(f"{sample_id}: expected event_dag_node_count > 0")
        if evidence.get("event_dag_edge_count", 0) <= 0:
            raise RuntimeError(f"{sample_id}: expected event_dag_edge_count > 0")
    elif sample_id == "VIS-02":
        if evidence.get("place_count", 0) <= 0:
            raise RuntimeError(f"{sample_id}: expected place_count > 0")
        if not evidence.get("world_place_ids"):
            raise RuntimeError(f"{sample_id}: expected world_place_ids to be non-empty")
        if not evidence.get("game_place_ids"):
            raise RuntimeError(f"{sample_id}: expected game_place_ids to be non-empty")
        if not evidence.get("participant_place_ids"):
            raise RuntimeError(f"{sample_id}: expected participant_place_ids to be non-empty")
        if not evidence.get("attachpoint_ids"):
            raise RuntimeError(f"{sample_id}: expected attachpoint_ids to be non-empty")
    elif sample_id == "VIS-03":
        if evidence.get("route_hop_count", 0) <= 0:
            raise RuntimeError(f"{sample_id}: expected route_hop_count > 0")
        if not all(value is False for value in evidence.get("auth_lane_present_values", [])):
            raise RuntimeError(f"{sample_id}: expected auth_lane_present_values to stay false")
    elif sample_id == "VIS-05":
        if evidence.get("timeline_entry_count", 0) <= 0:
            raise RuntimeError(f"{sample_id}: expected timeline_entry_count > 0")
        if len(evidence.get("membership_epochs", [])) < 2:
            raise RuntimeError(f"{sample_id}: expected at least two membership_epochs")
        if not evidence.get("membership_principals"):
            raise RuntimeError(f"{sample_id}: expected membership_principals to be non-empty")
        if not evidence.get("membership_update_event_ids"):
            raise RuntimeError(f"{sample_id}: expected membership_update_event_ids to be non-empty")
    elif sample_id == "VIS-06":
        panel_ids = set(evidence.get("panel_ids", []))
        required = {"attach_lifecycle", "membership_snapshot", "detach_lifecycle"}
        if not required.issubset(panel_ids):
            raise RuntimeError(f"{sample_id}: panel_ids missing required lifecycle views")
    elif sample_id == "VIS-07":
        if not evidence.get("fallback_applied"):
            raise RuntimeError(f"{sample_id}: expected fallback_applied to be true")
    elif sample_id == "VIS-08":
        if evidence.get("redaction_policies") != ["observer_safe_route_trace"]:
            raise RuntimeError(f"{sample_id}: unexpected redaction_policies")
        if not all(value is False for value in evidence.get("auth_lane_present_values", [])):
            raise RuntimeError(f"{sample_id}: expected auth_lane_present_values to stay false")
    elif sample_id == "VIS-10":
        if evidence.get("pre_attach_trace_count") != 0:
            raise RuntimeError(f"{sample_id}: expected pre_attach_trace_count == 0")
        if evidence.get("post_attach_trace_count", 0) <= 0:
            raise RuntimeError(f"{sample_id}: expected post_attach_trace_count > 0")
        if evidence.get("rejected_attach_trace_count") != 0:
            raise RuntimeError(f"{sample_id}: expected rejected_attach_trace_count == 0")
        if evidence.get("trace_mode") != "on_demand":
            raise RuntimeError(f"{sample_id}: expected trace_mode == 'on_demand'")
    elif sample_id == "VIS-11":
        if evidence.get("over_retention_detected"):
            raise RuntimeError(f"{sample_id}: over_retention_detected unexpectedly true")
        if "retention_scope_widened" not in evidence.get("rejected_reason_refs", []):
            raise RuntimeError(f"{sample_id}: rejected_reason_refs missing retention_scope_widened")
        if not evidence.get("over_retention_rejected"):
            raise RuntimeError(f"{sample_id}: expected over_retention_rejected to be true")


def _validate_sidecar_parity(
    sample_id: str, report: dict[str, Any], expected_sidecar: dict[str, Any]
) -> None:
    if report != expected_sidecar:
        raise RuntimeError(f"{sample_id}: sidecar drift detected")


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    report = _build_sample_report(sample_id)
    _validate_expected_fields(sample_id, row, report)
    _validate_sidecar_parity(sample_id, report, _load_expected_sidecar(row))
    return report


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            run_sample(sample_id)
            passed.append(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "stage_e_complete": False,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": "samples/alpha/visualization",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "validation_floor": [
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku",
            "python3 scripts/alpha_cut_save_load_samples.py run CUT-17 --format json",
            "python3 scripts/alpha_network_docker_e2e.py run NET-02 --format json",
            "python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-02 --format json",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- run AV-08",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- run NET-07",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-admin",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-non-admin",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- incompatible",
            "python3 scripts/alpha_visualization_samples.py check-all --format json",
            "python3 -m unittest scripts.tests.test_alpha_visualization_samples",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "stage_e_complete": False,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and "sample_id" in payload:
        lines = [
            f"{payload['sample_id']} alpha_visualization",
            f"  outcome: {payload.get('terminal_outcome')}",
            f"  surface: {payload.get('evidence_summary', {}).get('visualization_surface')}",
        ]
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list", help="list implemented runtime-backed rows")
    list_parser.add_argument("--format", choices=("text", "json"), default="text")

    run_parser = subparsers.add_parser("run", help="run one runtime-backed row")
    run_parser.add_argument("sample_id")
    run_parser.add_argument("--format", choices=("text", "json"), default="text")

    check_parser = subparsers.add_parser("check-all", help="run all runtime-backed rows")
    check_parser.add_argument("--format", choices=("text", "json"), default="text")

    closeout_parser = subparsers.add_parser("closeout", help="report implemented/planned inventory")
    closeout_parser.add_argument("--format", choices=("text", "json"), default="text")

    args = parser.parse_args(argv)

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
