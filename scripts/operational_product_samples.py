#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
OPS_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "operational"
WORLD_CORE = OPS_ROOT / "world-core"
MEMBERSHIP_CHAT = OPS_ROOT / "membership-chat"
SUGOROKU_WORLD = OPS_ROOT / "sugoroku-world"
PORTAL_WORLDLINK = OPS_ROOT / "portal-worldlink"
TWO_SHARD_HARD_BOUNDARY = OPS_ROOT / "two-shard-hard-boundary"
TWO_SHARD_GRADIENT_OBSERVATION = OPS_ROOT / "two-shard-gradient-observation"
LAYERS_ROOT = OPS_ROOT / "packages"
EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT = 'ChatText:Text("hello room")->Text("room#lobby message accepted: hello room")'
EXPECTED_SUGOROKU_EVENT_KINDS = {
    "sugoroku_roll_requested",
    "sugoroku_roll_published",
    "sugoroku_witness_emitted",
    "sugoroku_turn_handoff",
    "sugoroku_stale_membership_rejected",
}
EXPECTED_SUGOROKU_ROUTE_LANES = {
    "same_session_sugoroku_roll",
    "same_session_sugoroku_handoff",
    "same_session_sugoroku_membership_reject",
}
EXPECTED_SUGOROKU_PROJECTION_PACKET_NAMES = {
    "roll_request_packet",
    "chat_message_packet",
}
EXPECTED_SUGOROKU_PROJECTION_FFI_NAMES = {"host_io_adapter"}
EXPECTED_PORTAL_EVENT_KINDS = {
    "portal_resolve_requested",
    "portal_handoff_offered",
    "portal_handoff_witness_emitted",
    "portal_admission_requested",
    "portal_admission_accepted",
}
EXPECTED_PORTAL_ROUTE_LANES = {
    "same_session_portal_resolve",
    "same_session_portal_handoff",
    "same_session_portal_admit",
}
EXPECTED_TWO_SHARD_EVENT_KINDS = {
    "shard_handoff_offer_published",
    "shard_handoff_prepare_accepted",
    "shard_handoff_commit_applied",
    "shard_old_owner_write_rejected",
    "shard_missing_handoff_witness_rejected",
    "shard_stale_config_rejected",
}
EXPECTED_TWO_SHARD_ROUTE_LANES = {
    "same_session_shard_handoff_offer",
    "same_session_shard_handoff_commit",
    "same_session_shard_old_owner_reject",
    "same_session_shard_missing_witness_reject",
    "same_session_shard_stale_config_reject",
}
EXPECTED_TWO_SHARD_FAILURE_CLASSES = {
    "OldOwnerWriteRejected",
    "MissingHandoffWitness",
    "StaleShardConfig",
}
EXPECTED_TWO_SHARD_GRADIENT_EVENT_KINDS = {
    "gradient_observer_view_emitted",
    "gradient_handoff_hint_projected",
    "gradient_write_capability_rejected",
    "gradient_stale_view_dropped",
    "gradient_missing_freshness_rejected",
}
EXPECTED_TWO_SHARD_GRADIENT_ROUTE_LANES = {
    "same_session_gradient_observe",
    "same_session_gradient_projection",
    "same_session_gradient_write_reject",
    "same_session_gradient_stale_drop",
    "same_session_gradient_missing_freshness_reject",
}
EXPECTED_TWO_SHARD_GRADIENT_FAILURE_CLASSES = {
    "GradientWriteRejected",
    "StaleGradientViewDropped",
    "MissingFreshnessFieldRejected",
}


@dataclass(frozen=True)
class CommandResult:
    name: str
    argv: list[str]
    returncode: int
    stdout: str
    stderr: str
    payload: dict[str, Any] | None


def cargo_alpha_args(*args: str) -> list[str]:
    return ["cargo", "run", "-q", "-p", "mirrorea-cli", "--", *args, "--format", "json"]


def cargo_test_args(*args: str) -> list[str]:
    return ["cargo", "test", *args, "--", "--nocapture"]


def run_command(name: str, argv: list[str], env: dict[str, str] | None = None) -> CommandResult:
    completed = subprocess.run(
        argv,
        cwd=REPO_ROOT,
        env=env,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    payload = None
    if completed.stdout.strip():
        try:
            payload = json.loads(completed.stdout)
        except json.JSONDecodeError:
            payload = None
    return CommandResult(
        name=name,
        argv=argv,
        returncode=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        payload=payload,
    )


def command_payload(result: CommandResult) -> dict[str, Any]:
    return {
        "name": result.name,
        "argv": result.argv,
        "returncode": result.returncode,
        "payload": result.payload,
        "stderr": result.stderr.strip(),
    }


def sample_rows() -> list[dict[str, Any]]:
    return [
        {
            "sample_id": "OPS-01",
            "root": str(WORLD_CORE.relative_to(REPO_ROOT)),
            "package_id": "operational-world-core",
            "package_kind": "world_core",
            "runnable": True,
        },
        {
            "sample_id": "OPS-02",
            "root": str(MEMBERSHIP_CHAT.relative_to(REPO_ROOT)),
            "package_id": "operational-membership-chat",
            "package_kind": "membership_chat",
            "runnable": True,
        },
        {
            "sample_id": "OPS-03",
            "root": str(SUGOROKU_WORLD.relative_to(REPO_ROOT)),
            "package_id": "operational-sugoroku",
            "package_kind": "sugoroku_world",
            "runnable": True,
        },
        {
            "sample_id": "OPS-06",
            "root": str(PORTAL_WORLDLINK.relative_to(REPO_ROOT)),
            "package_id": "operational-portal-worldlink",
            "package_kind": "portal_worldlink",
            "runnable": True,
        },
        {
            "sample_id": "OPS-07",
            "root": str(TWO_SHARD_HARD_BOUNDARY.relative_to(REPO_ROOT)),
            "package_id": "operational-two-shard-hard-boundary",
            "package_kind": "two_shard_hard_boundary",
            "runnable": True,
        },
        {
            "sample_id": "OPS-07G",
            "root": str(TWO_SHARD_GRADIENT_OBSERVATION.relative_to(REPO_ROOT)),
            "package_id": "operational-two-shard-gradient-observation",
            "package_kind": "two_shard_gradient_observation",
            "runnable": True,
        },
    ]


def list_samples() -> dict[str, Any]:
    return {
        "surface_kind": "operational_product_sample_suite_list",
        "package_name": "P-OPS-01 operational product sample suite scaffold and first workflow",
        "sample_root": str(OPS_ROOT.relative_to(REPO_ROOT)),
        "samples": sample_rows(),
        "canonical_entrypoint": "mirrorea-alpha",
        "helper_role": "orchestration_only",
        "final_public_api_frozen": False,
    }


def membership_chat_chat_text_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    if not payload.get("typed_host_io_claimed"):
        return False
    session = payload.get("session") or {}
    host_io_history = session.get("host_io_history") or []
    if not host_io_history or host_io_history[0].get("adapter_kind") != "ChatText":
        return False
    observer_safe_export = session.get("observer_safe_export") or {}
    visible_events = observer_safe_export.get("visible_host_io_events") or []
    return EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT in visible_events


def membership_chat_devtools_chat_text_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    panel_ids = payload.get("panel_ids") or []
    session = payload.get("session") or {}
    observer_safe_export = session.get("observer_safe_export") or {}
    visible_events = observer_safe_export.get("visible_host_io_events") or []
    return (
        EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT in visible_events
        and "event_dag" in panel_ids
    )


def room_chat_scope() -> dict[str, Any]:
    return {
        "lane_kind": "bounded_single_message_room_oriented_chat_text",
        "request_shape": 'ChatText("hello room")',
        "response_shape": 'Text("room#lobby message accepted: hello room")',
        "multi_message_room_surface_defined": False,
        "transport_coupled_chat_lane_defined": False,
        "room_history_service_defined": False,
        "stdio_builtin_defined": False,
    }


def portal_shard_starter_scope() -> dict[str, Any]:
    return {
        "authoring_source_boundary": "active_executable_roots_study_copy",
        "template_catalog_terminal_root": "templates/sugoroku-world-starter",
        "portal_worldlink_starter_defined": False,
        "two_shard_hard_boundary_starter_defined": False,
        "two_shard_gradient_observation_starter_defined": False,
        "future_inventory_executable": False,
        "active_authoring_roots": [
            str(PORTAL_WORLDLINK.relative_to(REPO_ROOT)),
            str(TWO_SHARD_HARD_BOUNDARY.relative_to(REPO_ROOT)),
            str(TWO_SHARD_GRADIENT_OBSERVATION.relative_to(REPO_ROOT)),
        ],
        "future_inventory_roots": [
            "samples/product-alpha1/operational/future/portal-worldlink",
            "samples/product-alpha1/operational/future/two-shard-hard-boundary",
            "samples/product-alpha1/operational/future/gradient-observation.profile.json",
        ],
    }


def sugoroku_scope() -> dict[str, Any]:
    return {
        "scenario_kind": "bounded_deterministic_same_session_sugoroku",
        "roll_publish_witness_handoff_defined": True,
        "stale_membership_reject_defined": True,
        "interactive_turn_choice_surface_defined": False,
        "broader_negative_row_catalog_defined": False,
        "networked_multi_participant_control_defined": False,
    }


def widening_queue_scope() -> dict[str, Any]:
    return {
        "room_chat_reopen_recommended": False,
        "portal_shard_starter_reopen_recommended": False,
        "sugoroku_reopen_recommended": False,
        "next_promoted_reopen_requires_user_decision": True,
        "next_promoted_reopen_point": "later_user_final_distribution_decision",
    }


def user_final_decision_scope() -> dict[str, Any]:
    from product_alpha1_installed_binary_check import distribution_scope

    return {
        **distribution_scope(),
        "current_catalog_scope": "bounded_product_alpha1_narrow_showcase",
        "broader_final_shared_space_catalog_defined": False,
        "self_driven_operational_reopenings_exhausted": True,
        "next_reopen_requires_user_decision": True,
        "next_user_decision_items": [
            "U1_beyond_alpha_packaging_host_target_shipped_surface",
            "final_shared_space_operational_catalog_breadth",
        ],
    }


def sugoroku_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    session = payload.get("session") or {}
    panels = payload.get("panels") or {}
    event_nodes = (
        (session.get("event_dag") or {}).get("nodes")
        or (panels.get("event_dag") or {}).get("nodes")
        or []
    )
    route_entries = (
        (session.get("route_graph") or {}).get("routes")
        or (panels.get("message_route_graph") or {}).get("routes")
        or []
    )
    message_state_lane = (
        (session.get("message_recovery_state") or {}).get("message_state_lane")
        or (panels.get("message_failure_recovery") or {}).get("message_state_lane")
        or []
    )
    event_kinds = {node.get("event_kind") for node in event_nodes}
    route_lanes = {route.get("transport_lane") for route in route_entries}
    stale_membership_rejected = any(
        record.get("failure_class") == "StaleMembership"
        and record.get("state") == "Rejected"
        for record in message_state_lane
    )
    return (
        EXPECTED_SUGOROKU_EVENT_KINDS.issubset(event_kinds)
        and EXPECTED_SUGOROKU_ROUTE_LANES.issubset(route_lanes)
        and stale_membership_rejected
    )


def sugoroku_devtools_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    panel_ids = payload.get("panel_ids") or []
    return (
        "event_dag" in panel_ids
        and "message_route_graph" in panel_ids
        and sugoroku_runtime_evidence_observed(result)
    )


def sugoroku_projection_inventory_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    projection_inventory = payload.get("projection_inventory") or {}
    packet_names = set(projection_inventory.get("packet_boundary_names") or [])
    ffi_names = set(projection_inventory.get("ffi_boundary_names") or [])
    return (
        projection_inventory.get("source_package") == "operational-sugoroku"
        and projection_inventory.get("target_count") == 2
        and projection_inventory.get("packet_boundary_count") == 2
        and projection_inventory.get("ffi_boundary_count") == 1
        and EXPECTED_SUGOROKU_PROJECTION_PACKET_NAMES.issubset(packet_names)
        and EXPECTED_SUGOROKU_PROJECTION_FFI_NAMES.issubset(ffi_names)
        and not projection_inventory.get("llvm_codegen_claimed", True)
        and not projection_inventory.get("direct_mir_to_machine_code_claimed", True)
    )


def portal_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    session = payload.get("session") or {}
    panels = payload.get("panels") or {}
    event_nodes = (
        (session.get("event_dag") or {}).get("nodes")
        or (panels.get("event_dag") or {}).get("nodes")
        or []
    )
    route_entries = (
        (session.get("route_graph") or {}).get("routes")
        or (panels.get("message_route_graph") or {}).get("routes")
        or []
    )
    event_kinds = {node.get("event_kind") for node in event_nodes}
    route_lanes = {route.get("transport_lane") for route in route_entries}
    return EXPECTED_PORTAL_EVENT_KINDS.issubset(
        event_kinds
    ) and EXPECTED_PORTAL_ROUTE_LANES.issubset(route_lanes)


def portal_devtools_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    panel_ids = payload.get("panel_ids") or []
    portal_panel = (payload.get("panels") or {}).get("portal_graph_future") or {}
    return (
        "portal_graph_future" in panel_ids
        and "message_route_graph" in panel_ids
        and portal_panel.get("current_status") == "bounded_discrete_handoff_runtime"
        and portal_runtime_evidence_observed(result)
    )


def two_shard_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    session = payload.get("session") or {}
    panels = payload.get("panels") or {}
    event_nodes = (
        (session.get("event_dag") or {}).get("nodes")
        or (panels.get("event_dag") or {}).get("nodes")
        or []
    )
    route_entries = (
        (session.get("route_graph") or {}).get("routes")
        or (panels.get("message_route_graph") or {}).get("routes")
        or []
    )
    message_state_lane = (
        (session.get("message_recovery_state") or {}).get("message_state_lane")
        or (panels.get("message_failure_recovery") or {}).get("message_state_lane")
        or []
    )
    event_kinds = {node.get("event_kind") for node in event_nodes}
    route_lanes = {route.get("transport_lane") for route in route_entries}
    failure_classes = {
        record.get("failure_class")
        for record in message_state_lane
        if record.get("state") == "Rejected"
    }
    return (
        EXPECTED_TWO_SHARD_EVENT_KINDS.issubset(event_kinds)
        and EXPECTED_TWO_SHARD_ROUTE_LANES.issubset(route_lanes)
        and EXPECTED_TWO_SHARD_FAILURE_CLASSES.issubset(failure_classes)
    )


def two_shard_devtools_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    panel_ids = payload.get("panel_ids") or []
    shard_panel = (payload.get("panels") or {}).get("shard_map_future") or {}
    return (
        "shard_map_future" in panel_ids
        and "message_route_graph" in panel_ids
        and shard_panel.get("current_status") == "bounded_two_shard_runtime"
        and two_shard_runtime_evidence_observed(result)
    )


def two_shard_gradient_runtime_evidence_observed(result: CommandResult) -> bool:
    payload = result.payload or {}
    session = payload.get("session") or {}
    panels = payload.get("panels") or {}
    event_nodes = (
        (session.get("event_dag") or {}).get("nodes")
        or (panels.get("event_dag") or {}).get("nodes")
        or []
    )
    route_entries = (
        (session.get("route_graph") or {}).get("routes")
        or (panels.get("message_route_graph") or {}).get("routes")
        or []
    )
    message_state_lane = (
        (session.get("message_recovery_state") or {}).get("message_state_lane")
        or (panels.get("message_failure_recovery") or {}).get("message_state_lane")
        or []
    )
    event_kinds = {node.get("event_kind") for node in event_nodes}
    route_lanes = {route.get("transport_lane") for route in route_entries}
    failure_classes = {
        record.get("failure_class")
        for record in message_state_lane
        if record.get("state") == "Rejected"
    }
    return (
        EXPECTED_TWO_SHARD_GRADIENT_EVENT_KINDS.issubset(event_kinds)
        and EXPECTED_TWO_SHARD_GRADIENT_ROUTE_LANES.issubset(route_lanes)
        and EXPECTED_TWO_SHARD_GRADIENT_FAILURE_CLASSES.issubset(failure_classes)
    )


def two_shard_gradient_devtools_runtime_evidence_observed(
    result: CommandResult,
) -> bool:
    payload = result.payload or {}
    panel_ids = payload.get("panel_ids") or []
    shard_panel = (payload.get("panels") or {}).get("shard_map_future") or {}
    return (
        "shard_map_future" in panel_ids
        and "message_route_graph" in panel_ids
        and shard_panel.get("current_status")
        == "bounded_gradient_observation_runtime"
        and two_shard_gradient_runtime_evidence_observed(result)
    )


def run_world_package(root: Path) -> dict[str, Any]:
    result = run_command(
        f"run-local:{root.name}",
        cargo_alpha_args("run-local", str(root)),
    )
    semantic_checks: dict[str, bool] = {}
    if root == MEMBERSHIP_CHAT:
        semantic_checks["chat_text_observed"] = membership_chat_chat_text_observed(result)
    elif root == SUGOROKU_WORLD:
        semantic_checks["runtime_evidence_observed"] = (
            sugoroku_runtime_evidence_observed(result)
        )
    elif root == PORTAL_WORLDLINK:
        semantic_checks["runtime_evidence_observed"] = (
            portal_runtime_evidence_observed(result)
        )
    elif root == TWO_SHARD_HARD_BOUNDARY:
        semantic_checks["runtime_evidence_observed"] = (
            two_shard_runtime_evidence_observed(result)
        )
    elif root == TWO_SHARD_GRADIENT_OBSERVATION:
        semantic_checks["runtime_evidence_observed"] = (
            two_shard_gradient_runtime_evidence_observed(result)
        )
    return {
        "surface_kind": "operational_product_sample_run_report",
        "root": str(root.relative_to(REPO_ROOT)),
        "status": "accepted"
        if result.returncode == 0 and all(semantic_checks.values(),)
        else "error",
        "command": command_payload(result),
        "semantic_checks": semantic_checks,
        "room_chat_scope": room_chat_scope() if root == MEMBERSHIP_CHAT else None,
        "sugoroku_scope": sugoroku_scope() if root == SUGOROKU_WORLD else None,
        "final_public_api_frozen": False,
    }


def sugoroku_session_env() -> tuple[str, dict[str, str]]:
    session_dir = tempfile.mkdtemp(prefix="mirrorea-ops-session-")
    env = os.environ.copy()
    env["MIRROREA_ALPHA_SESSION_DIR"] = session_dir
    return session_dir, env


def bootstrap_sugoroku_session() -> tuple[str, dict[str, str], list[CommandResult]]:
    session_dir, env = sugoroku_session_env()
    commands = [
        run_command(
            "run-local:sugoroku",
            cargo_alpha_args("run-local", str(SUGOROKU_WORLD)),
            env=env,
        ),
        run_command(
            "session:sugoroku",
            cargo_alpha_args("session", "session#operational-sugoroku"),
            env=env,
        ),
    ]
    return session_dir, env, commands


def operational_attach_specs() -> list[tuple[str, Path, str]]:
    return [
        ("debug-layer", LAYERS_ROOT / "debug-layer", "accepted"),
        ("auth-layer", LAYERS_ROOT / "auth-layer", "accepted"),
        ("rate-limit-layer", LAYERS_ROOT / "rate-limit-layer", "accepted"),
        ("placeholder-object", LAYERS_ROOT / "placeholder-object", "deferred"),
        ("custom-avatar-preview", LAYERS_ROOT / "custom-avatar-preview", "deferred"),
    ]


def attach_matrix_complete(results: list[CommandResult]) -> bool:
    expected = {name: outcome for name, _, outcome in operational_attach_specs()}
    observed = {
        result.name.removeprefix("attach:"): result.payload.get("terminal_outcome")
        if result.payload
        else None
        for result in results
    }
    return all(observed.get(name) == outcome for name, outcome in expected.items())


def attach_layers() -> dict[str, Any]:
    session_dir, env, bootstrap = bootstrap_sugoroku_session()
    layer_results = [
        run_command(
            f"attach:{name}",
            cargo_alpha_args("attach", "session#operational-sugoroku", str(path)),
            env=env,
        )
        for name, path, _ in operational_attach_specs()
    ]
    matrix_complete = attach_matrix_complete(layer_results)
    return {
        "surface_kind": "operational_product_sample_attach_report",
        "session_dir": session_dir,
        "status": "accepted"
        if all(result.returncode == 0 for result in [*bootstrap, *layer_results])
        and matrix_complete
        else "error",
        "bootstrap": [command_payload(result) for result in bootstrap],
        "attach_results": [command_payload(result) for result in layer_results],
        "attach_matrix_complete": matrix_complete,
        "final_public_api_frozen": False,
    }


def transport(mode: str) -> dict[str, Any]:
    session_dir, env, bootstrap = bootstrap_sugoroku_session()
    result = run_command(
        f"transport:{mode}",
        cargo_alpha_args("transport", "session#operational-sugoroku", "--mode", mode),
        env=env,
    )
    return {
        "surface_kind": "operational_product_sample_transport_report",
        "mode": mode,
        "session_dir": session_dir,
        "status": "accepted"
        if all(item.returncode == 0 for item in [*bootstrap, result])
        else "error",
        "bootstrap": [command_payload(item) for item in bootstrap],
        "transport": command_payload(result),
        "final_public_api_frozen": False,
    }


def export_devtools() -> dict[str, Any]:
    session_dir, env, bootstrap = bootstrap_sugoroku_session()
    viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-viewer-")
    export_result = run_command(
        "export-devtools",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-sugoroku",
            "--out",
            viewer_dir,
        ),
        env=env,
    )
    view_result = run_command(
        "view",
        cargo_alpha_args("view", viewer_dir, "--check"),
    )
    semantic_checks = {
        "runtime_evidence_observed": sugoroku_devtools_runtime_evidence_observed(
            export_result
        )
    }
    return {
        "surface_kind": "operational_product_sample_devtools_report",
        "session_dir": session_dir,
        "viewer_dir": viewer_dir,
        "status": "accepted"
        if all(item.returncode == 0 for item in [*bootstrap, export_result, view_result])
        and all(semantic_checks.values())
        else "error",
        "bootstrap": [command_payload(item) for item in bootstrap],
        "export_devtools": command_payload(export_result),
        "view": command_payload(view_result),
        "semantic_checks": semantic_checks,
        "final_public_api_frozen": False,
    }


def build_native_bundle() -> dict[str, Any]:
    out_dir = tempfile.mkdtemp(prefix="mirrorea-ops-bundle-")
    result = run_command(
        "build-native-bundle",
        cargo_alpha_args("build-native-bundle", str(SUGOROKU_WORLD), "--out", out_dir),
    )
    return {
        "surface_kind": "operational_product_sample_native_bundle_report",
        "bundle_dir": out_dir,
        "status": "accepted" if result.returncode == 0 else "error",
        "command": command_payload(result),
        "final_public_api_frozen": False,
    }


def release_check(skip_docker: bool) -> dict[str, Any]:
    session_dir, env = sugoroku_session_env()
    chat_session_dir, chat_env = sugoroku_session_env()
    portal_session_dir, portal_env = sugoroku_session_env()
    shard_session_dir, shard_env = sugoroku_session_env()
    gradient_session_dir, gradient_env = sugoroku_session_env()
    viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-viewer-")
    chat_viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-chat-viewer-")
    portal_viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-portal-viewer-")
    shard_viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-shard-viewer-")
    gradient_viewer_dir = tempfile.mkdtemp(prefix="mirrorea-ops-gradient-viewer-")
    bundle_dir = tempfile.mkdtemp(prefix="mirrorea-ops-bundle-")
    sugoroku_check = run_command("check:sugoroku-world", cargo_alpha_args("check", str(SUGOROKU_WORLD)))
    membership_chat_run = run_command(
        "run-local:membership-chat",
        cargo_alpha_args("run-local", str(MEMBERSHIP_CHAT)),
        env=chat_env,
    )
    membership_chat_export = run_command(
        "export-devtools:membership-chat",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-membership-chat",
            "--out",
            chat_viewer_dir,
        ),
        env=chat_env,
    )
    membership_chat_view = run_command(
        "view:membership-chat",
        cargo_alpha_args("view", chat_viewer_dir, "--check"),
    )
    portal_check = run_command("check:portal-worldlink", cargo_alpha_args("check", str(PORTAL_WORLDLINK)))
    portal_run = run_command(
        "run-local:portal-worldlink",
        cargo_alpha_args("run-local", str(PORTAL_WORLDLINK)),
        env=portal_env,
    )
    portal_export = run_command(
        "export-devtools:portal-worldlink",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-portal-worldlink",
            "--out",
            portal_viewer_dir,
        ),
        env=portal_env,
    )
    portal_view = run_command(
        "view:portal-worldlink",
        cargo_alpha_args("view", portal_viewer_dir, "--check"),
    )
    shard_check = run_command(
        "check:two-shard-hard-boundary",
        cargo_alpha_args("check", str(TWO_SHARD_HARD_BOUNDARY)),
    )
    shard_run = run_command(
        "run-local:two-shard-hard-boundary",
        cargo_alpha_args("run-local", str(TWO_SHARD_HARD_BOUNDARY)),
        env=shard_env,
    )
    shard_export = run_command(
        "export-devtools:two-shard-hard-boundary",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-two-shard-hard-boundary",
            "--out",
            shard_viewer_dir,
        ),
        env=shard_env,
    )
    shard_view = run_command(
        "view:two-shard-hard-boundary",
        cargo_alpha_args("view", shard_viewer_dir, "--check"),
    )
    gradient_check = run_command(
        "check:two-shard-gradient-observation",
        cargo_alpha_args("check", str(TWO_SHARD_GRADIENT_OBSERVATION)),
    )
    gradient_run = run_command(
        "run-local:two-shard-gradient-observation",
        cargo_alpha_args("run-local", str(TWO_SHARD_GRADIENT_OBSERVATION)),
        env=gradient_env,
    )
    gradient_export = run_command(
        "export-devtools:two-shard-gradient-observation",
        cargo_alpha_args(
            "export-devtools",
            "session#operational-two-shard-gradient-observation",
            "--out",
            gradient_viewer_dir,
        ),
        env=gradient_env,
    )
    gradient_view = run_command(
        "view:two-shard-gradient-observation",
        cargo_alpha_args("view", gradient_viewer_dir, "--check"),
    )
    sugoroku_run = run_command(
        "run-local:sugoroku",
        cargo_alpha_args("run-local", str(SUGOROKU_WORLD)),
        env=env,
    )
    sugoroku_session = run_command(
        "session:sugoroku",
        cargo_alpha_args("session", "session#operational-sugoroku"),
        env=env,
    )
    commands = [
        run_command("check:world-core", cargo_alpha_args("check", str(WORLD_CORE))),
        run_command("check:membership-chat", cargo_alpha_args("check", str(MEMBERSHIP_CHAT))),
        sugoroku_check,
        portal_check,
        shard_check,
        membership_chat_run,
        membership_chat_export,
        membership_chat_view,
        portal_run,
        portal_export,
        portal_view,
        shard_run,
        shard_export,
        shard_view,
        gradient_check,
        gradient_run,
        gradient_export,
        gradient_view,
        sugoroku_run,
        sugoroku_session,
        run_command("save:r0", cargo_alpha_args("save", "session#operational-sugoroku", "--savepoint", "savepoint#ops-r0"), env=env),
        run_command("quiescent-save:r2", cargo_alpha_args("quiescent-save", "session#operational-sugoroku", "--savepoint", "savepoint#ops-r2"), env=env),
        run_command("transport:local", cargo_alpha_args("transport", "session#operational-sugoroku", "--mode", "local"), env=env),
    ]
    attach_results = [
        run_command(
            f"attach:{name}",
            cargo_alpha_args("attach", "session#operational-sugoroku", str(path)),
            env=env,
        )
        for name, path, _ in operational_attach_specs()
    ]
    commands[5:5] = attach_results
    if not skip_docker:
        commands.append(
            run_command(
                "transport:docker",
                cargo_alpha_args("transport", "session#operational-sugoroku", "--mode", "docker"),
                env=env,
            )
        )
    sugoroku_export = run_command(
        "export-devtools",
        cargo_alpha_args(
            "export-devtools", "session#operational-sugoroku", "--out", viewer_dir
        ),
        env=env,
    )
    sugoroku_view = run_command("view", cargo_alpha_args("view", viewer_dir, "--check"))
    sugoroku_bundle = run_command(
        "build-native-bundle",
        cargo_alpha_args("build-native-bundle", str(SUGOROKU_WORLD), "--out", bundle_dir),
    )
    commands.extend([sugoroku_export, sugoroku_view, sugoroku_bundle])
    failed = [result.name for result in commands if result.returncode != 0]
    attach_matrix_ok = attach_matrix_complete(attach_results)
    if not attach_matrix_ok:
        failed.append("attach-matrix")
    membership_chat_chat_text_ok = membership_chat_chat_text_observed(membership_chat_run)
    membership_chat_devtools_ok = membership_chat_devtools_chat_text_observed(
        membership_chat_export
    )
    portal_runtime_ok = portal_runtime_evidence_observed(portal_run)
    portal_devtools_ok = portal_devtools_runtime_evidence_observed(portal_export)
    shard_runtime_ok = two_shard_runtime_evidence_observed(shard_run)
    shard_devtools_ok = two_shard_devtools_runtime_evidence_observed(shard_export)
    gradient_runtime_ok = two_shard_gradient_runtime_evidence_observed(gradient_run)
    gradient_devtools_ok = two_shard_gradient_devtools_runtime_evidence_observed(
        gradient_export
    )
    projection_inventory_ok = sugoroku_projection_inventory_observed(sugoroku_check)
    sugoroku_runtime_ok = sugoroku_runtime_evidence_observed(sugoroku_run)
    sugoroku_devtools_ok = sugoroku_devtools_runtime_evidence_observed(sugoroku_export)
    if not membership_chat_chat_text_ok:
        failed.append("membership-chat-chat-text")
    if not membership_chat_devtools_ok:
        failed.append("membership-chat-devtools")
    if not portal_runtime_ok:
        failed.append("portal-runtime-evidence")
    if not portal_devtools_ok:
        failed.append("portal-devtools")
    if not shard_runtime_ok:
        failed.append("two-shard-runtime-evidence")
    if not shard_devtools_ok:
        failed.append("two-shard-devtools")
    if not gradient_runtime_ok:
        failed.append("two-shard-gradient-runtime-evidence")
    if not gradient_devtools_ok:
        failed.append("two-shard-gradient-devtools")
    if not projection_inventory_ok:
        failed.append("projection-inventory")
    if not sugoroku_runtime_ok:
        failed.append("sugoroku-runtime-evidence")
    if not sugoroku_devtools_ok:
        failed.append("sugoroku-devtools")
    status = "accepted" if not failed and not skip_docker else "partial" if not failed else "error"
    return {
        "surface_kind": "operational_product_sample_release_check_report",
        "status": status,
        "docker_included": not skip_docker,
        "session_dir": session_dir,
        "chat_session_dir": chat_session_dir,
        "portal_session_dir": portal_session_dir,
        "shard_session_dir": shard_session_dir,
        "gradient_session_dir": gradient_session_dir,
        "viewer_dir": viewer_dir,
        "chat_viewer_dir": chat_viewer_dir,
        "portal_viewer_dir": portal_viewer_dir,
        "shard_viewer_dir": shard_viewer_dir,
        "gradient_viewer_dir": gradient_viewer_dir,
        "bundle_dir": bundle_dir,
        "failed_commands": failed,
        "attach_matrix_complete": attach_matrix_ok,
        "membership_chat_chat_text_ok": membership_chat_chat_text_ok,
        "membership_chat_devtools_ok": membership_chat_devtools_ok,
        "room_chat_scope": room_chat_scope(),
        "portal_runtime_ok": portal_runtime_ok,
        "portal_devtools_ok": portal_devtools_ok,
        "shard_runtime_ok": shard_runtime_ok,
        "shard_devtools_ok": shard_devtools_ok,
        "gradient_runtime_ok": gradient_runtime_ok,
        "gradient_devtools_ok": gradient_devtools_ok,
        "projection_inventory_ok": projection_inventory_ok,
        "sugoroku_runtime_ok": sugoroku_runtime_ok,
        "sugoroku_devtools_ok": sugoroku_devtools_ok,
        "commands": [command_payload(result) for result in commands],
        "product_alpha1_ready": False,
        "final_public_api_frozen": False,
        "non_claims": [
            "no final textual .mir grammar",
            "no final server/client binary split",
            "no direct LLVM backend",
            "no WAN federation",
            "no distributed durable save/load",
        ],
    }


def check_all(skip_docker: bool) -> dict[str, Any]:
    validation = [
        run_command(
            "validation:test-validate-docs",
            ["python3", "-m", "unittest", "scripts.tests.test_validate_docs"],
        ),
        run_command(
            "validation:test-operational-helper",
            ["python3", "-m", "unittest", "scripts.tests.test_operational_product_samples"],
        ),
        run_command("validation:source-hierarchy", ["python3", "scripts/check_source_hierarchy.py"]),
        run_command("validation:validate-docs", ["python3", "scripts/validate_docs.py"]),
        run_command("validation:cargo-fmt", ["cargo", "fmt", "--check"]),
        run_command("validation:git-diff-check", ["git", "diff", "--check"]),
        run_command(
            "test:mir-ast-product-schema",
            cargo_test_args("-p", "mir-ast", "--test", "product_alpha1_package_schema"),
        ),
        run_command(
            "test:mir-runtime-session",
            cargo_test_args("-p", "mir-runtime", "--test", "product_alpha1_session"),
        ),
        run_command(
            "test:mir-runtime-devtools",
            cargo_test_args("-p", "mir-runtime", "--test", "product_alpha1_transport_devtools"),
        ),
        run_command(
            "test:mirrorea-cli-alpha",
            cargo_test_args("-p", "mirrorea-cli", "--test", "alpha_cli"),
        ),
    ]
    release = release_check(skip_docker=skip_docker)
    failed = [result.name for result in validation if result.returncode != 0]
    failed.extend(release["failed_commands"])
    status = "accepted" if not failed and not skip_docker else "partial" if not failed else "error"
    return {
        "surface_kind": "operational_product_sample_suite_check_all_report",
        "status": status,
        "docker_included": not skip_docker,
        "failed_commands": failed,
        "validation": [command_payload(result) for result in validation],
        "release_check": release,
        "room_chat_scope": room_chat_scope(),
        "portal_shard_starter_scope": portal_shard_starter_scope(),
        "sugoroku_scope": sugoroku_scope(),
        "widening_queue_scope": widening_queue_scope(),
        "user_final_decision_scope": user_final_decision_scope(),
        "product_alpha1_ready": False,
        "final_public_api_frozen": False,
    }


def format_output(payload: dict[str, Any], fmt: str) -> str:
    if fmt == "json":
        return json.dumps(payload, indent=2, ensure_ascii=False)
    return json.dumps(payload, indent=2, ensure_ascii=False)


def add_subcommand_format(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--format",
        choices=["json", "pretty"],
        dest="subcommand_format",
        default=None,
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list")
    add_subcommand_format(list_parser)

    check_all_parser = subparsers.add_parser("check-all")
    add_subcommand_format(check_all_parser)
    check_all_parser.add_argument("--skip-docker", action="store_true")

    run_world_core_parser = subparsers.add_parser("run-world-core")
    add_subcommand_format(run_world_core_parser)
    run_membership_chat_parser = subparsers.add_parser("run-membership-chat")
    add_subcommand_format(run_membership_chat_parser)
    run_sugoroku_parser = subparsers.add_parser("run-sugoroku")
    add_subcommand_format(run_sugoroku_parser)
    run_portal_worldlink_parser = subparsers.add_parser("run-portal-worldlink")
    add_subcommand_format(run_portal_worldlink_parser)
    run_two_shard_parser = subparsers.add_parser("run-two-shard-hard-boundary")
    add_subcommand_format(run_two_shard_parser)
    run_two_shard_gradient_parser = subparsers.add_parser(
        "run-two-shard-gradient-observation"
    )
    add_subcommand_format(run_two_shard_gradient_parser)
    attach_layers_parser = subparsers.add_parser("attach-layers")
    add_subcommand_format(attach_layers_parser)
    transport_local_parser = subparsers.add_parser("transport-local")
    add_subcommand_format(transport_local_parser)

    transport_docker_parser = subparsers.add_parser("transport-docker")
    add_subcommand_format(transport_docker_parser)
    transport_docker_parser.add_argument("--skip-docker", action="store_true")

    export_devtools_parser = subparsers.add_parser("export-devtools")
    add_subcommand_format(export_devtools_parser)
    build_native_bundle_parser = subparsers.add_parser("build-native-bundle")
    add_subcommand_format(build_native_bundle_parser)

    release_parser = subparsers.add_parser("release-check")
    add_subcommand_format(release_parser)
    release_parser.add_argument("--skip-docker", action="store_true")

    closeout_parser = subparsers.add_parser("closeout")
    add_subcommand_format(closeout_parser)
    closeout_parser.add_argument("--skip-docker", action="store_true")

    args = parser.parse_args(argv)
    output_format = args.subcommand_format or args.format

    if args.command == "list":
        payload = list_samples()
    elif args.command == "check-all":
        payload = check_all(skip_docker=args.skip_docker)
    elif args.command == "run-world-core":
        payload = run_world_package(WORLD_CORE)
    elif args.command == "run-membership-chat":
        payload = run_world_package(MEMBERSHIP_CHAT)
    elif args.command == "run-sugoroku":
        payload = run_world_package(SUGOROKU_WORLD)
    elif args.command == "run-portal-worldlink":
        payload = run_world_package(PORTAL_WORLDLINK)
    elif args.command == "run-two-shard-hard-boundary":
        payload = run_world_package(TWO_SHARD_HARD_BOUNDARY)
    elif args.command == "run-two-shard-gradient-observation":
        payload = run_world_package(TWO_SHARD_GRADIENT_OBSERVATION)
    elif args.command == "attach-layers":
        payload = attach_layers()
    elif args.command == "transport-local":
        payload = transport("local")
    elif args.command == "transport-docker":
        payload = (
            {
                "surface_kind": "operational_product_sample_transport_report",
                "mode": "docker",
                "status": "skipped",
                "reason": "--skip-docker was passed",
                "final_public_api_frozen": False,
            }
            if args.skip_docker
            else transport("docker")
        )
    elif args.command == "export-devtools":
        payload = export_devtools()
    elif args.command == "build-native-bundle":
        payload = build_native_bundle()
    elif args.command == "release-check":
        payload = release_check(skip_docker=args.skip_docker)
    elif args.command == "closeout":
        payload = check_all(skip_docker=args.skip_docker)
    else:
        raise AssertionError(f"unhandled command {args.command}")

    print(format_output(payload, output_format))
    return 0 if payload.get("status") not in {"error"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
