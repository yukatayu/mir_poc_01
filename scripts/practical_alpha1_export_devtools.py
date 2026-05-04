#!/usr/bin/env python3

from __future__ import annotations

import argparse
import html
import json
import sys
import tempfile
from functools import lru_cache
from pathlib import Path
from typing import Any

import practical_alpha1_attach
import practical_alpha1_avatar
import practical_alpha1_run_local
import practical_alpha1_transport


SCRIPT_PATH = Path(__file__).resolve()
SCRIPT_DIR = SCRIPT_PATH.parent
REPO_ROOT = SCRIPT_DIR.parent

DEVTOOLS_SCOPE = "practical-alpha1-devtools-export-floor"
VIEWER_SCOPE = "practical-alpha1-devtools-viewer-floor"
SURFACE_KIND = "practical_alpha1_nonfinal_devtools_bundle"
VIEWER_MODE = "static_html_bundle"
BUNDLE_BOUNDARY = (
    "non-final practical devtools export bundle over exact practical reports; "
    "not a final public viewer or telemetry API"
)

PANEL_LANES = [
    "panel_id",
    "panel_kind",
    "label",
    "authority",
    "redaction",
    "retention_scope",
    "source_report_refs",
    "focus_refs",
    "notes",
]

TELEMETRY_LANES = [
    "telemetry_id",
    "telemetry_kind",
    "label",
    "authority",
    "redaction",
    "retention_scope",
    "source_report_refs",
    "channel",
    "value_summary",
    "notes",
]

STOP_LINES = [
    "do not treat this export bundle as full practical devtools completion",
    "do not treat this export bundle as membership timeline or retention/on-demand completion",
    "do not treat this export bundle as save/load, product prototype, or final public runtime/devtools/telemetry ABI completion",
    "do not promote samples/practical-alpha1 to an active runnable root in the devtools package",
]

LIMITATIONS = [
    "non-final practical devtools export floor only",
    "exact practical reports reused as sources; no new runtime semantics added",
    "remaining practical devtools observables stay deferred",
]

COMMON_NON_CLAIMS = [
    "final public viewer API",
    "final public telemetry schema",
    "save/load completion",
    "product prototype completion",
    "WAN/federation completion",
]

DEFERRED_OBSERVABLES = [
    "VIS-A1-03",
    "VIS-A1-07",
]

IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "VIS-A1-01",
        "summary": "event DAG export/view over the practical local-runtime report",
        "expected_report": "samples/practical-alpha1/expected/vis-a1-01-event-dag-export.expected.json",
        "bundle_kind": "event_dag_export",
        "actualized_observable": "VIS-A1-01",
    },
    {
        "sample_id": "VIS-A1-02",
        "summary": "observer-safe route trace export/view over the practical transport report",
        "expected_report": "samples/practical-alpha1/expected/vis-a1-02-route-trace-export.expected.json",
        "bundle_kind": "route_trace_export",
        "actualized_observable": "VIS-A1-02",
    },
    {
        "sample_id": "VIS-A1-04",
        "summary": "hot-plug lifecycle boundary export/view over exact practical hotplug reports",
        "expected_report": "samples/practical-alpha1/expected/vis-a1-04-hotplug-lifecycle.expected.json",
        "bundle_kind": "hotplug_lifecycle_export",
        "actualized_observable": "VIS-A1-04",
    },
    {
        "sample_id": "VIS-A1-05",
        "summary": "fallback degradation export/view over the exact practical avatar preview report",
        "expected_report": "samples/practical-alpha1/expected/vis-a1-05-fallback-degradation.expected.json",
        "bundle_kind": "fallback_degradation_export",
        "actualized_observable": "VIS-A1-05",
    },
    {
        "sample_id": "VIS-A1-06",
        "summary": "redacted observer view over the practical auth-lane transport report",
        "expected_report": "samples/practical-alpha1/expected/vis-a1-06-redacted-observer-view.expected.json",
        "bundle_kind": "redacted_observer_view",
        "actualized_observable": "VIS-A1-06",
    },
]


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 devtools sample {sample_id}")


def _panel(
    *,
    panel_id: str,
    panel_kind: str,
    label: str,
    authority: str,
    redaction: str,
    retention_scope: str,
    source_report_refs: list[str],
    focus_refs: list[str],
    notes: list[str],
) -> dict[str, Any]:
    return {
        "panel_id": panel_id,
        "panel_kind": panel_kind,
        "label": label,
        "authority": authority,
        "redaction": redaction,
        "retention_scope": retention_scope,
        "source_report_refs": list(source_report_refs),
        "focus_refs": list(focus_refs),
        "notes": list(notes),
    }


def _telemetry(
    *,
    telemetry_id: str,
    telemetry_kind: str,
    label: str,
    authority: str,
    redaction: str,
    retention_scope: str,
    source_report_refs: list[str],
    channel: str,
    value_summary: str,
    notes: list[str],
) -> dict[str, Any]:
    return {
        "telemetry_id": telemetry_id,
        "telemetry_kind": telemetry_kind,
        "label": label,
        "authority": authority,
        "redaction": redaction,
        "retention_scope": retention_scope,
        "source_report_refs": list(source_report_refs),
        "channel": channel,
        "value_summary": value_summary,
        "notes": list(notes),
    }


@lru_cache(maxsize=None)
def _run_local_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_run_local.run_sample(sample_id)


@lru_cache(maxsize=None)
def _hotplug_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_attach.run_sample(sample_id)


@lru_cache(maxsize=None)
def _transport_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_transport.run_sample(sample_id)


@lru_cache(maxsize=None)
def _avatar_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_avatar.run_sample(sample_id)


def _source_report_ref(
    *,
    family: str,
    sample_id: str,
    carrier_scope: str,
    surface_kind: str,
) -> dict[str, str]:
    return {
        "family": family,
        "sample_id": sample_id,
        "carrier_scope": carrier_scope,
        "surface_kind": surface_kind,
    }


def _reason_refs(verdict: dict[str, Any]) -> list[str]:
    refs: list[str] = []
    for field in (
        "compatibility_reason_refs",
        "authorization_reason_refs",
        "membership_freshness_reason_refs",
        "witness_reason_refs",
    ):
        refs.extend(list(verdict.get(field, [])))
    return refs


def _event_dag_bundle() -> dict[str, Any]:
    report = _run_local_report("RUN-01")
    event_dag = report["event_dag"]
    dispatch_records = report["dispatch_records"]
    source_report_refs = [
        _source_report_ref(
            family="practical-alpha1-local-runtime",
            sample_id="RUN-01",
            carrier_scope=report["runtime_scope"],
            surface_kind=report["surface_kind"],
        )
    ]
    panels = [
        _panel(
            panel_id="event_dag_graph",
            panel_kind="event_dag",
            label="practical:event-dag",
            authority="InspectEventDag(WorldPlace[AlphaRoom#1])",
            redaction="event_structure_only",
            retention_scope="report_local_inventory",
            source_report_refs=["RUN-01"],
            focus_refs=[node["event_id"] for node in event_dag["nodes"]],
            notes=[
                "non-final practical event DAG export bundle",
                "publication/witness/handoff relations stay typed in the export payload",
            ],
        ),
        _panel(
            panel_id="publication_relation_summary",
            panel_kind="relation_summary",
            label="practical:publication-witness-handoff",
            authority="InspectPublicationWitnessHandoff(WorldPlace[AlphaRoom#1])",
            redaction="published_history_only",
            retention_scope="report_local_inventory",
            source_report_refs=["RUN-01"],
            focus_refs=[edge["relation"] for edge in event_dag["edges"]],
            notes=[
                "derived from the same exact runtime report as the event DAG graph",
            ],
        ),
    ]
    telemetry_rows = [
        _telemetry(
            telemetry_id=node["event_id"],
            telemetry_kind=node["event_kind"],
            label=f"practical:event:{node['event_kind']}",
            authority="InspectEventDag(WorldPlace[AlphaRoom#1])",
            redaction="event_structure_only",
            retention_scope="report_local_inventory",
            source_report_refs=["RUN-01"],
            channel=node["place_ref"],
            value_summary=node["envelope_ref"],
            notes=list(node.get("notes", [])),
        )
        for node in event_dag["nodes"]
    ]
    telemetry_rows.extend(
        _telemetry(
            telemetry_id=record["envelope_id"],
            telemetry_kind="dispatch_record",
            label="practical:dispatch",
            authority="InspectRuntimeDispatch(WorldPlace[AlphaRoom#1])",
            redaction="event_structure_only",
            retention_scope="report_local_inventory",
            source_report_refs=["RUN-01"],
            channel=f"{record['from_place']} -> {record['to_place']}",
            value_summary=record["dispatch_outcome"],
            notes=list(record.get("notes", [])),
        )
        for record in dispatch_records
    )
    return {
        "sample_id": "VIS-A1-01",
        "bundle_kind": "event_dag_export",
        "family": "practical-alpha1-devtools-export",
        "devtools_scope": DEVTOOLS_SCOPE,
        "viewer_scope": VIEWER_SCOPE,
        "surface_kind": SURFACE_KIND,
        "viewer_mode": VIEWER_MODE,
        "bundle_boundary": BUNDLE_BOUNDARY,
        "actualized_observable": "VIS-A1-01",
        "source_reports": source_report_refs,
        "panel_lanes": list(PANEL_LANES),
        "telemetry_lanes": list(TELEMETRY_LANES),
        "panels": panels,
        "telemetry_rows": telemetry_rows,
        "panel_ids": [panel["panel_id"] for panel in panels],
        "panel_kinds": sorted({panel["panel_kind"] for panel in panels}),
        "telemetry_ids": [row["telemetry_id"] for row in telemetry_rows],
        "telemetry_kinds": sorted({row["telemetry_kind"] for row in telemetry_rows}),
        "retention_scopes": sorted({panel["retention_scope"] for panel in panels}),
        "redaction_policies": sorted({panel["redaction"] for panel in panels}),
        "export_sections": {
            "event_dag": event_dag,
            "dispatch_records": dispatch_records,
            "message_envelope_lanes": report["message_envelope_lanes"],
            "visible_history": report["visible_history"],
            "current_owner": report["current_owner"],
            "membership_snapshot": report["runtime_snapshot"]["membership"],
        },
        "what_it_proves": [
            "exact practical runtime report is consumable as a distinct event DAG export bundle",
            "publication, witness, and handoff relations remain typed in the export payload",
            "viewer/export first floor remains separate from runtime-plan and transport carriers",
        ],
        "what_it_does_not_prove": list(COMMON_NON_CLAIMS)
        + [
            "full devtools stage completion",
            "membership timeline completion",
            "hot-plug lifecycle completion",
            "fallback degradation completion",
        ],
    }


def _route_trace_bundle() -> dict[str, Any]:
    report = _transport_report("TR-A1-06")
    route_trace = report["observer_route_trace"]
    panels = [
        _panel(
            panel_id="observer_route_trace",
            panel_kind="route_trace",
            label="practical:observer-route-trace",
            authority="ObserveRouteTrace(NetworkTransportLane)",
            redaction="observer_safe_route_trace",
            retention_scope="helper_local_ephemeral",
            source_report_refs=["TR-A1-06"],
            focus_refs=[row["envelope_id"] for row in route_trace],
            notes=[
                "observer-safe route trace stays typed and redacted",
                "non-final practical transport report is the only source",
            ],
        )
    ]
    telemetry_rows = [
        _telemetry(
            telemetry_id=row["envelope_id"],
            telemetry_kind="route_hop",
            label="practical:route-hop",
            authority=row["authority"],
            redaction=row["redaction"],
            retention_scope=row["retention_scope"],
            source_report_refs=["TR-A1-06"],
            channel=f"{row['from_place']} -> {row['to_place']}",
            value_summary=row["dispatch_outcome"],
            notes=[
                f"hop_index={row['hop_index']}",
                f"payload_kind={row['payload_kind']}",
            ],
        )
        for row in route_trace
    ]
    return {
        "sample_id": "VIS-A1-02",
        "bundle_kind": "route_trace_export",
        "family": "practical-alpha1-devtools-export",
        "devtools_scope": DEVTOOLS_SCOPE,
        "viewer_scope": VIEWER_SCOPE,
        "surface_kind": SURFACE_KIND,
        "viewer_mode": VIEWER_MODE,
        "bundle_boundary": BUNDLE_BOUNDARY,
        "actualized_observable": "VIS-A1-02",
        "source_reports": [
            _source_report_ref(
                family="practical-alpha1-transport",
                sample_id="TR-A1-06",
                carrier_scope=report["transport_scope"],
                surface_kind=report["surface_kind"],
            )
        ],
        "panel_lanes": list(PANEL_LANES),
        "telemetry_lanes": list(TELEMETRY_LANES),
        "panels": panels,
        "telemetry_rows": telemetry_rows,
        "panel_ids": [panel["panel_id"] for panel in panels],
        "panel_kinds": sorted({panel["panel_kind"] for panel in panels}),
        "telemetry_ids": [row["telemetry_id"] for row in telemetry_rows],
        "telemetry_kinds": sorted({row["telemetry_kind"] for row in telemetry_rows}),
        "retention_scopes": sorted({panel["retention_scope"] for panel in panels}),
        "redaction_policies": sorted({panel["redaction"] for panel in panels}),
        "export_sections": {
            "observer_route_trace": route_trace,
            "message_envelope_lanes": report["message_envelope_lanes"],
            "transport_plan_scope": report["transport_plan_scope"],
            "transport_surface": report["transport_surface"],
            "world_membership_epoch": report["world_membership_epoch"],
            "world_active_participants": report["world_active_participants"],
        },
        "what_it_proves": [
            "observer-safe route trace can be exported from the exact practical transport report",
            "route trace remains typed and redacted at the export boundary",
            "transport report stays distinct from the devtools bundle that consumes it",
        ],
        "what_it_does_not_prove": list(COMMON_NON_CLAIMS)
        + [
            "full devtools stage completion",
            "auth-lane/redacted observer completion beyond the exact route-trace report",
            "full membership timeline completion",
        ],
    }


def _hotplug_lifecycle_bundle() -> dict[str, Any]:
    attach_report = _hotplug_report("HP-A1-01")
    detach_report = _hotplug_report("HP-A1-07")
    attach_request = attach_report["hotplug_runtime_report"]["request"]
    attach_verdict = attach_report["hotplug_runtime_report"]["verdict"]
    detach_request = detach_report["hotplug_runtime_report"]["request"]
    detach_verdict = detach_report["hotplug_runtime_report"]["verdict"]
    attach_membership = attach_report["hotplug_runtime_report"]["runtime_snapshot"][
        "membership"
    ]
    detach_membership = detach_report["hotplug_runtime_report"]["runtime_snapshot"][
        "membership"
    ]
    shared_members = sorted(
        set(attach_membership["members"].keys()) & set(detach_membership["members"].keys())
    )
    panels = [
        _panel(
            panel_id="attach_lifecycle",
            panel_kind="hotplug_lifecycle",
            label="practical:attach-lifecycle",
            authority="InspectHotPlugBoundary(AttachPoint[AlphaRoom#1::MessageDispatch])",
            redaction="membership_and_boundary_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-01"],
            focus_refs=[
                attach_request["request_id"],
                attach_report["activation_cut_ref"],
            ],
            notes=[
                "exact practical attach report is consumed as a typed lifecycle boundary",
                "no lifecycle state machine is imported beyond request/verdict/activation-cut evidence",
            ],
        ),
        _panel(
            panel_id="membership_snapshot",
            panel_kind="membership_snapshot",
            label="practical:hotplug-membership-snapshot",
            authority="InspectMembershipFrontier(WorldPlace[AlphaRoom#1])",
            redaction="membership_frontier_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-01", "HP-A1-07"],
            focus_refs=shared_members,
            notes=[
                "attach and deferred-detach reports keep membership frontier evidence explicit",
                "membership timeline widening remains a later observable",
            ],
        ),
        _panel(
            panel_id="detach_lifecycle",
            panel_kind="hotplug_lifecycle",
            label="practical:detach-lifecycle",
            authority="InspectHotPlugBoundary(AttachPoint[AlphaRoom#1::MessageDispatch])",
            redaction="membership_and_boundary_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-07"],
            focus_refs=[
                detach_request["request_id"],
                detach_report["detach_boundary_ref"],
            ],
            notes=[
                "current practical floor exports detach only as an explicit deferred boundary",
                "rollback, migration, and detach execution remain later",
            ],
        ),
    ]
    telemetry_rows = [
        _telemetry(
            telemetry_id=attach_request["request_id"],
            telemetry_kind="hotplug_request",
            label="practical:attach-request",
            authority="InspectHotPlugBoundary(AttachPoint[AlphaRoom#1::MessageDispatch])",
            redaction="membership_and_boundary_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-01"],
            channel=attach_request["attachpoint_ref"],
            value_summary=attach_report["terminal_outcome"],
            notes=[
                f"operation_kind={attach_request['operation_kind']}",
                f"attach_profile={attach_report['attach_profile']}",
            ],
        ),
        _telemetry(
            telemetry_id=attach_report["activation_cut_ref"],
            telemetry_kind="activation_cut",
            label="practical:activation-cut",
            authority="InspectHotPlugBoundary(AttachPoint[AlphaRoom#1::MessageDispatch])",
            redaction="membership_and_boundary_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-01"],
            channel=attach_request["attachpoint_ref"],
            value_summary="accepted_attach_boundary",
            notes=_reason_refs(attach_verdict),
        ),
        _telemetry(
            telemetry_id="membership_frontier#HP-A1-01+07",
            telemetry_kind="membership_frontier",
            label="practical:hotplug-membership-frontier",
            authority="InspectMembershipFrontier(WorldPlace[AlphaRoom#1])",
            redaction="membership_frontier_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-01", "HP-A1-07"],
            channel="WorldPlace[AlphaRoom#1]",
            value_summary=(
                f"epoch={attach_membership['membership_epoch']} "
                f"shared_members={len(shared_members)}"
            ),
            notes=[
                f"detach_epoch={detach_membership['membership_epoch']}",
                "exact report-local snapshot only; no full membership timeline claimed",
            ],
        ),
        _telemetry(
            telemetry_id=detach_request["request_id"],
            telemetry_kind="hotplug_request",
            label="practical:detach-request",
            authority="InspectHotPlugBoundary(AttachPoint[AlphaRoom#1::MessageDispatch])",
            redaction="membership_and_boundary_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-07"],
            channel=detach_request["attachpoint_ref"],
            value_summary=detach_report["terminal_outcome"],
            notes=[
                f"operation_kind={detach_request['operation_kind']}",
                f"reason_family={detach_report['reason_family']}",
            ],
        ),
        _telemetry(
            telemetry_id=detach_report["detach_boundary_ref"],
            telemetry_kind="detach_boundary",
            label="practical:detach-boundary",
            authority="InspectHotPlugBoundary(AttachPoint[AlphaRoom#1::MessageDispatch])",
            redaction="membership_and_boundary_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["HP-A1-07"],
            channel=detach_request["attachpoint_ref"],
            value_summary=detach_report["terminal_outcome"],
            notes=list(detach_report["rejection_reason_refs"]) + _reason_refs(detach_verdict),
        ),
    ]
    return {
        "sample_id": "VIS-A1-04",
        "bundle_kind": "hotplug_lifecycle_export",
        "family": "practical-alpha1-devtools-export",
        "devtools_scope": DEVTOOLS_SCOPE,
        "viewer_scope": VIEWER_SCOPE,
        "surface_kind": SURFACE_KIND,
        "viewer_mode": VIEWER_MODE,
        "bundle_boundary": BUNDLE_BOUNDARY,
        "actualized_observable": "VIS-A1-04",
        "source_reports": [
            _source_report_ref(
                family="practical-alpha1-hotplug",
                sample_id="HP-A1-01",
                carrier_scope=attach_report["hotplug_scope"],
                surface_kind=attach_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-hotplug",
                sample_id="HP-A1-07",
                carrier_scope=detach_report["hotplug_scope"],
                surface_kind=detach_report["surface_kind"],
            ),
        ],
        "panel_lanes": list(PANEL_LANES),
        "telemetry_lanes": list(TELEMETRY_LANES),
        "panels": panels,
        "telemetry_rows": telemetry_rows,
        "panel_ids": [panel["panel_id"] for panel in panels],
        "panel_kinds": sorted({panel["panel_kind"] for panel in panels}),
        "telemetry_ids": [row["telemetry_id"] for row in telemetry_rows],
        "telemetry_kinds": sorted({row["telemetry_kind"] for row in telemetry_rows}),
        "retention_scopes": sorted({panel["retention_scope"] for panel in panels}),
        "redaction_policies": sorted({panel["redaction"] for panel in panels}),
        "export_sections": {
            "attach_boundary": {
                "sample_id": attach_report["sample_id"],
                "operation_kind": attach_request["operation_kind"],
                "request_id": attach_request["request_id"],
                "terminal_outcome": attach_report["terminal_outcome"],
                "activation_cut_ref": attach_report["activation_cut_ref"],
                "active_layers_after": attach_report["active_layers_after"],
                "reason_refs": _reason_refs(attach_verdict),
            },
            "membership_snapshot": {
                "attach_membership_epoch": attach_membership["membership_epoch"],
                "detach_membership_epoch": detach_membership["membership_epoch"],
                "same_epoch": attach_membership["membership_epoch"]
                == detach_membership["membership_epoch"],
                "shared_members": shared_members,
            },
            "detach_boundary": {
                "sample_id": detach_report["sample_id"],
                "operation_kind": detach_request["operation_kind"],
                "request_id": detach_request["request_id"],
                "terminal_outcome": detach_report["terminal_outcome"],
                "detach_boundary_ref": detach_report["detach_boundary_ref"],
                "reason_family": detach_report["reason_family"],
                "rejection_reason_refs": detach_report["rejection_reason_refs"],
                "reason_refs": _reason_refs(detach_verdict),
            },
            "request_lanes": {
                "attach": attach_report["hotplug_runtime_report"]["request_lanes"],
                "detach": detach_report["hotplug_runtime_report"]["request_lanes"],
            },
            "verdict_lanes": {
                "attach": attach_report["hotplug_runtime_report"]["verdict_lanes"],
                "detach": detach_report["hotplug_runtime_report"]["verdict_lanes"],
            },
        },
        "what_it_proves": [
            "exact practical hotplug reports are consumable as a distinct hot-plug lifecycle export bundle without adding new runtime semantics",
            "accepted attach, explicit membership frontier snapshot, and deferred detach boundary remain separate and typed at the viewer boundary",
            "devtools export reuses exact practical hotplug reports instead of importing helper-local lifecycle ids as canonical runtime state",
        ],
        "what_it_does_not_prove": list(COMMON_NON_CLAIMS)
        + [
            "full devtools stage completion",
            "membership timeline completion",
            "detach runtime lifecycle execution",
            "rollback or durable migration completion",
            "fallback degradation completion",
            "retention/on-demand completion",
            "final object package attach completion",
        ],
    }


def _fallback_degradation_bundle() -> dict[str, Any]:
    report = _avatar_report("AV-A1-03")
    panels = [
        _panel(
            panel_id="avatar_fallback_degradation",
            panel_kind="fallback_degradation",
            label="practical:avatar-fallback-degradation",
            authority="InspectAvatarFallbackPreview(WorldPlace[AlphaRoom#1])",
            redaction="degraded_role_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["AV-A1-03"],
            focus_refs=[report["fallback_reason"], *report["degraded_roles"]],
            notes=[
                "exact avatar preview report stays the only source of fallback/degraded-role evidence",
                "rejected source lane remains explicit at the viewer boundary",
            ],
        ),
        _panel(
            panel_id="host_capability_gap",
            panel_kind="compatibility_gap",
            label="practical:avatar-host-capability-gap",
            authority="InspectAvatarFallbackPreview(WorldPlace[AlphaRoom#1])",
            redaction="capability_gap_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["AV-A1-03"],
            focus_refs=list(report["missing_host_capabilities"]),
            notes=[
                "missing host capability is exported as typed compatibility evidence",
                "no runtime execution is implied by the export bundle",
            ],
        ),
    ]
    telemetry_rows = [
        _telemetry(
            telemetry_id="avatar_fallback#AV-A1-03",
            telemetry_kind="fallback_transition",
            label="practical:avatar-fallback-transition",
            authority="InspectAvatarFallbackPreview(WorldPlace[AlphaRoom#1])",
            redaction="degraded_role_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["AV-A1-03"],
            channel=report["requested_runtime_route"],
            value_summary=(
                f"selected={report['selected_representation']} "
                f"fallback_applied={report['fallback_applied']}"
            ),
            notes=[
                f"source_terminal_outcome={report['source_hotplug_terminal_outcome']}",
                f"fallback_reason={report['fallback_reason']}",
            ],
        ),
        _telemetry(
            telemetry_id="host_capability_gap#AV-A1-03",
            telemetry_kind="capability_gap",
            label="practical:avatar-host-gap",
            authority="InspectAvatarFallbackPreview(WorldPlace[AlphaRoom#1])",
            redaction="capability_gap_summary",
            retention_scope="report_local_inventory",
            source_report_refs=["AV-A1-03"],
            channel="custom_mir_avatar_runtime",
            value_summary="missing_host_capability:HostMirAvatarVM",
            notes=[
                f"required={','.join(report['required_host_capabilities'])}",
                f"available={','.join(report['available_host_capabilities'])}",
            ],
        ),
    ]
    return {
        "sample_id": "VIS-A1-05",
        "bundle_kind": "fallback_degradation_export",
        "family": "practical-alpha1-devtools-export",
        "devtools_scope": DEVTOOLS_SCOPE,
        "viewer_scope": VIEWER_SCOPE,
        "surface_kind": SURFACE_KIND,
        "viewer_mode": VIEWER_MODE,
        "bundle_boundary": BUNDLE_BOUNDARY,
        "actualized_observable": "VIS-A1-05",
        "source_reports": [
            _source_report_ref(
                family="practical-alpha1-avatar-preview",
                sample_id="AV-A1-03",
                carrier_scope=report["avatar_scope"],
                surface_kind=report["surface_kind"],
            )
        ],
        "panel_lanes": list(PANEL_LANES),
        "telemetry_lanes": list(TELEMETRY_LANES),
        "panels": panels,
        "telemetry_rows": telemetry_rows,
        "panel_ids": [panel["panel_id"] for panel in panels],
        "panel_kinds": sorted({panel["panel_kind"] for panel in panels}),
        "telemetry_ids": [row["telemetry_id"] for row in telemetry_rows],
        "telemetry_kinds": sorted({row["telemetry_kind"] for row in telemetry_rows}),
        "retention_scopes": sorted({panel["retention_scope"] for panel in panels}),
        "redaction_policies": sorted({panel["redaction"] for panel in panels}),
        "export_sections": {
            "source_avatar_preview": {
                "sample_id": report["sample_id"],
                "preview_kind": report["preview_kind"],
                "requested_runtime_route": report["requested_runtime_route"],
                "source_hotplug_terminal_outcome": report[
                    "source_hotplug_terminal_outcome"
                ],
                "source_hotplug_reason_family": report["source_hotplug_reason_family"],
                "source_hotplug_rejection_reason_refs": report[
                    "source_hotplug_rejection_reason_refs"
                ],
            },
            "fallback_degradation": {
                "selected_representation": report["selected_representation"],
                "fallback_representation": report["fallback_representation"],
                "fallback_applied": report["fallback_applied"],
                "fallback_reason": report["fallback_reason"],
                "fallback_visible": report["fallback_visible"],
                "active_roles": report["active_roles"],
                "degraded_roles": report["degraded_roles"],
            },
            "host_capability_gap": {
                "required_host_capabilities": report["required_host_capabilities"],
                "available_host_capabilities": report["available_host_capabilities"],
                "missing_host_capabilities": report["missing_host_capabilities"],
                "native_execution_performed": report["native_execution_performed"],
            },
        },
        "what_it_proves": [
            "exact avatar preview fallback evidence is consumable as a distinct fallback degradation export bundle",
            "rejected source lane, degraded roles, and missing host capability remain explicit at the viewer boundary",
            "fallback degradation export stays separate from runtime execution and product-preview attachment claims",
        ],
        "what_it_does_not_prove": list(COMMON_NON_CLAIMS)
        + [
            "full devtools stage completion",
            "full membership timeline completion",
            "retention/on-demand completion",
            "native execution",
            "same-session runtime attach/detach execution",
            "unsupported-runtime execution success",
        ],
    }


def _redacted_observer_bundle() -> dict[str, Any]:
    report = _transport_report("TR-A1-07")
    route_trace = report["observer_route_trace"]
    auth_lane = report["auth_lane"]
    panels = [
        _panel(
            panel_id="redacted_observer_route",
            panel_kind="redacted_observer_view",
            label="practical:redacted-observer-route",
            authority="ObserveRouteTrace(NetworkTransportLane)",
            redaction="observer_safe_route_trace",
            retention_scope="helper_local_ephemeral",
            source_report_refs=["TR-A1-07"],
            focus_refs=[row["envelope_id"] for row in route_trace],
            notes=[
                "auth lane remains separate from transport metadata",
                "route trace omits raw auth payloads",
            ],
        ),
        _panel(
            panel_id="auth_lane_summary",
            panel_kind="auth_lane",
            label="practical:auth-lane-summary",
            authority="InspectAuthLane(NetworkTransportLane)",
            redaction="binding_summary_only",
            retention_scope="helper_local_ephemeral",
            source_report_refs=["TR-A1-07"],
            focus_refs=list(auth_lane["bindings"]) if auth_lane is not None else [],
            notes=list(auth_lane.get("notes", [])) if auth_lane is not None else [],
        ),
    ]
    telemetry_rows = [
        _telemetry(
            telemetry_id=row["envelope_id"],
            telemetry_kind="observer_route_hop",
            label="practical:redacted-observer-hop",
            authority=row["authority"],
            redaction=row["redaction"],
            retention_scope=row["retention_scope"],
            source_report_refs=["TR-A1-07"],
            channel=f"{row['from_place']} -> {row['to_place']}",
            value_summary=row["dispatch_outcome"],
            notes=[
                f"hop_index={row['hop_index']}",
                f"auth_lane_present={row['auth_lane_present']}",
            ],
        )
        for row in route_trace
    ]
    if auth_lane is not None:
        telemetry_rows.append(
            _telemetry(
                telemetry_id="auth_lane#TR-A1-07",
                telemetry_kind="auth_lane",
                label="practical:auth-lane",
                authority="InspectAuthLane(NetworkTransportLane)",
                redaction="binding_summary_only",
                retention_scope="helper_local_ephemeral",
                source_report_refs=["TR-A1-07"],
                channel=auth_lane["transport_lane"],
                value_summary=f"subject={auth_lane['subject']} issuer={auth_lane['issuer']}",
                notes=list(auth_lane.get("notes", [])),
            )
        )
    return {
        "sample_id": "VIS-A1-06",
        "bundle_kind": "redacted_observer_view",
        "family": "practical-alpha1-devtools-export",
        "devtools_scope": DEVTOOLS_SCOPE,
        "viewer_scope": VIEWER_SCOPE,
        "surface_kind": SURFACE_KIND,
        "viewer_mode": VIEWER_MODE,
        "bundle_boundary": BUNDLE_BOUNDARY,
        "actualized_observable": "VIS-A1-06",
        "source_reports": [
            _source_report_ref(
                family="practical-alpha1-transport",
                sample_id="TR-A1-07",
                carrier_scope=report["transport_scope"],
                surface_kind=report["surface_kind"],
            )
        ],
        "panel_lanes": list(PANEL_LANES),
        "telemetry_lanes": list(TELEMETRY_LANES),
        "panels": panels,
        "telemetry_rows": telemetry_rows,
        "panel_ids": [panel["panel_id"] for panel in panels],
        "panel_kinds": sorted({panel["panel_kind"] for panel in panels}),
        "telemetry_ids": [row["telemetry_id"] for row in telemetry_rows],
        "telemetry_kinds": sorted({row["telemetry_kind"] for row in telemetry_rows}),
        "retention_scopes": sorted({panel["retention_scope"] for panel in panels}),
        "redaction_policies": sorted({panel["redaction"] for panel in panels}),
        "export_sections": {
            "observer_route_trace": route_trace,
            "auth_lane": auth_lane,
            "auth_evidence_lanes": report["auth_evidence_lanes"],
            "message_envelope_lanes": report["message_envelope_lanes"],
            "transport_plan_scope": report["transport_plan_scope"],
        },
        "what_it_proves": [
            "redacted observer view is exportable from the exact practical auth-lane transport report",
            "auth lane stays separate from transport metadata in the exported bundle",
            "observer route trace remains typed and redacted at the viewer boundary",
        ],
        "what_it_does_not_prove": list(COMMON_NON_CLAIMS)
        + [
            "full devtools stage completion",
            "full membership timeline completion",
            "full hot-plug lifecycle completion",
            "full fallback degradation completion",
            "retention/on-demand completion",
        ],
    }


def list_samples() -> list[dict[str, Any]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-devtools-export",
            "summary": row["summary"],
            "bundle_kind": row["bundle_kind"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def build_bundle(sample_id: str) -> dict[str, Any]:
    if sample_id == "VIS-A1-01":
        return _event_dag_bundle()
    if sample_id == "VIS-A1-02":
        return _route_trace_bundle()
    if sample_id == "VIS-A1-04":
        return _hotplug_lifecycle_bundle()
    if sample_id == "VIS-A1-05":
        return _fallback_degradation_bundle()
    if sample_id == "VIS-A1-06":
        return _redacted_observer_bundle()
    raise ValueError(f"unknown practical alpha-1 devtools sample {sample_id}")


def _load_expected_report(row: dict[str, Any]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = build_bundle(sample_id)
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected devtools bundle drift\n"
            f"expected={json.dumps(expected, ensure_ascii=False, sort_keys=True)}\n"
            f"actual={json.dumps(actual, ensure_ascii=False, sort_keys=True)}"
        )
    return actual


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    actualized_observables: list[str] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            bundle = run_sample(sample_id)
            passed.append(sample_id)
            actualized_observables.append(bundle["actualized_observable"])
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "devtools_export_first_floor_complete": not failed,
        "actualized_observables": actualized_observables,
        "deferred_observables": list(DEFERRED_OBSERVABLES),
        "stage_pa1_6_complete": False,
    }


def _render_bundle_html(bundle: dict[str, Any]) -> str:
    title = html.escape(f"{bundle['sample_id']} {bundle['bundle_kind']}")
    boundary = html.escape(bundle.get("bundle_boundary", BUNDLE_BOUNDARY))
    panel_items = "\n".join(
        "<li><strong>{}</strong> [{}] - {}</li>".format(
            html.escape(panel["panel_id"]),
            html.escape(panel["panel_kind"]),
            html.escape(panel["label"]),
        )
        for panel in bundle["panels"]
    )
    telemetry_items = "\n".join(
        "<li><strong>{}</strong> [{}] - {}</li>".format(
            html.escape(row["telemetry_id"]),
            html.escape(row["telemetry_kind"]),
            html.escape(row["value_summary"]),
        )
        for row in bundle["telemetry_rows"]
    )
    export_json = html.escape(
        json.dumps(bundle.get("export_sections", {}), ensure_ascii=False, indent=2)
    )
    proves = "\n".join(
        f"<li>{html.escape(line)}</li>" for line in bundle["what_it_proves"]
    )
    non_claims = "\n".join(
        f"<li>{html.escape(line)}</li>" for line in bundle["what_it_does_not_prove"]
    )
    return f"""<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="utf-8" />
  <title>{title}</title>
  <style>
    body {{ font-family: sans-serif; margin: 2rem; line-height: 1.5; }}
    code, pre {{ font-family: monospace; }}
    pre {{ background: #f6f6f6; padding: 1rem; overflow-x: auto; }}
    section {{ margin-bottom: 2rem; }}
  </style>
</head>
<body>
  <h1>{title}</h1>
  <p>{boundary}</p>
  <section>
    <h2>Panels</h2>
    <ul>{panel_items}</ul>
  </section>
  <section>
    <h2>Telemetry</h2>
    <ul>{telemetry_items}</ul>
  </section>
  <section>
    <h2>What It Proves</h2>
    <ul>{proves}</ul>
  </section>
  <section>
    <h2>What It Does Not Prove</h2>
    <ul>{non_claims}</ul>
  </section>
  <section>
    <h2>Export Sections</h2>
    <pre>{export_json}</pre>
  </section>
</body>
</html>
"""


def render_html(sample_id: str, output_path: str | None = None) -> dict[str, Any]:
    bundle = run_sample(sample_id)
    rendered = _render_bundle_html(bundle)
    if output_path is None:
        temp = tempfile.NamedTemporaryFile(
            prefix="practical-a1-devtools-",
            suffix=".html",
            delete=False,
        )
        temp_path = Path(temp.name)
        temp.close()
    else:
        temp_path = Path(output_path)
        temp_path.parent.mkdir(parents=True, exist_ok=True)
    temp_path.write_text(rendered)
    return {
        "sample_id": sample_id,
        "bundle_kind": bundle["bundle_kind"],
        "viewer_mode": VIEWER_MODE,
        "html_path": str(temp_path),
        "html": rendered,
    }


def closeout() -> dict[str, Any]:
    summary = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "actualized_observables": summary["actualized_observables"],
        "deferred_observables": list(DEFERRED_OBSERVABLES),
        "validation_floor": [
            "python3 scripts/practical_alpha1_check.py check-all --format json",
            "python3 scripts/practical_alpha1_run_local.py check-all --format json",
            "python3 scripts/practical_alpha1_attach.py check-all --format json",
            "python3 scripts/practical_alpha1_avatar.py check-all --format json",
            "python3 scripts/practical_alpha1_transport.py check-all --format json",
            "python3 scripts/practical_alpha1_export_devtools.py list --format json",
            "python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-01 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-02 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-04 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-05 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-06 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-04 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-05 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-06 --format json",
            "python3 scripts/practical_alpha1_export_devtools.py check-all --format json",
            "python3 scripts/practical_alpha1_export_devtools.py closeout --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_practical_alpha1_avatar scripts.tests.test_practical_alpha1_transport scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "devtools_export_first_floor_complete": summary[
            "devtools_export_first_floor_complete"
        ],
        "stage_pa1_6_complete": summary.get("stage_pa1_6_complete", False),
        "viewer_mode": VIEWER_MODE,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(
            f"{row['sample_id']} {row['summary']} [{row['bundle_kind']}]"
            for row in payload
        )
    if "html_path" in payload:
        return (
            f"{payload['sample_id']} {payload['bundle_kind']}\n"
            f"html_path: {payload['html_path']}"
        )
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 first devtools export floor over exact practical reports. "
            "This remains a non-final bundle/viewer surface."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")

    check_parser = subparsers.add_parser("check")
    check_parser.add_argument("sample_id")

    render_parser = subparsers.add_parser("render-html")
    render_parser.add_argument("sample_id")
    render_parser.add_argument("--output")

    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")

    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    return parser


def normalize_argv(argv: list[str] | None) -> list[str]:
    values = list(sys.argv[1:] if argv is None else argv)
    hoisted_root_options: list[str] = []
    remainder: list[str] = []
    index = 0
    while index < len(values):
        current = values[index]
        if current == "--format" and index + 1 < len(values):
            hoisted_root_options.extend(values[index : index + 2])
            index += 2
            continue
        remainder.append(current)
        index += 1
    values = [*hoisted_root_options, *remainder]
    known = {"list", "run", "check", "render-html", "check-all", "closeout"}
    if remainder and remainder[0] not in known and not remainder[0].startswith("-"):
        return [*hoisted_root_options, "check", *remainder]
    return values


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check":
        payload = build_bundle(args.sample_id)
    elif args.command == "render-html":
        payload = render_html(args.sample_id, args.output)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
