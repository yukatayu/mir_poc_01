#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

from current_l2_family_checker_support import filter_reason_rows, status_for_rows
from current_l2_reason_codes_assist import (
    read_actual_reason_code_candidates,
    read_fixture_checked_reason_codes,
    read_json,
)


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
E2E_ROOT = REPO_ROOT / "samples" / "alpha" / "e2e"
CUT_ROOT = REPO_ROOT / "samples" / "alpha" / "cut-save-load"


IMPLEMENTED_ROWS: list[dict[str, Any]] = [
    {
        "sample_id": "E2E-01",
        "summary": "Bridge the local Sugoroku runtime floor into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-01-local_integrated_sugoroku.expected.json",
    },
    {
        "sample_id": "E2E-02",
        "summary": "Bridge the Docker two-node transport floor into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-02-docker_two_node_sugoroku.expected.json",
    },
    {
        "sample_id": "E2E-03",
        "summary": "Bridge debug-layer attach evidence into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-03-hotplug_debug_layer_runtime.expected.json",
    },
    {
        "sample_id": "E2E-04",
        "summary": "Bridge declared rate-limit behavior into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-04-hotplug_ratelimit_runtime.expected.json",
    },
    {
        "sample_id": "E2E-05",
        "summary": "Bridge placeholder/custom avatar package admission into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-05-avatar_runtime_package.expected.json",
    },
    {
        "sample_id": "E2E-06",
        "summary": "Bridge the local-only save/load positive row into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-06-local_save_load_continue.expected.json",
    },
    {
        "sample_id": "E2E-07",
        "summary": "Bridge checker-backed invalid distributed cut rejection into an integrated Stage-F row.",
        "expected_terminal_outcome": "rejected",
        "expected_sidecar": "samples/alpha/e2e/e2e-07-distributed_inconsistent_save_negative.expected.json",
    },
    {
        "sample_id": "E2E-09",
        "summary": "Bridge auth-layer contract-update attach into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted_contract_update",
        "expected_sidecar": "samples/alpha/e2e/e2e-09-layer_auth_then_hotplug.expected.json",
    },
    {
        "sample_id": "E2E-10",
        "summary": "Bridge runtime-unavailable placeholder fallback into an integrated Stage-F row.",
        "expected_terminal_outcome": "accepted",
        "expected_sidecar": "samples/alpha/e2e/e2e-10-package_missing_runtime_fallback.expected.json",
    },
]

PLANNED_ONLY_ROWS = ["E2E-08"]

STOP_LINES = [
    "do not promote samples/alpha/e2e to an active runnable root",
    "do not mark Stage F complete while remaining Stage-E visualization/devtools rows remain incomplete",
    "do not treat checker-backed invalid distributed cut rejection as distributed save/load completion",
    "do not treat helper-local or Docker-local evidence as production runtime / WAN / durable replay evidence",
    "do not treat Reversed Library seed rows as Mirrorea Spaces alpha completion evidence",
]

LIMITATIONS = [
    "thin integrated runner over existing Stage B/C/D/F bridge floors only",
    "no parser/front-door execution of samples/alpha/e2e/*.mir",
    "Stage-E subset runner exists, but Stage E completion is still partial",
    "no Stage F completion claim",
]

NEGATIVE_COVERAGE = {
    "stale_membership_reject": "NET-03",
    "missing_capability_reject": "NET-04",
    "missing_witness_reject": "NET-05",
    "incompatible_patch_reject": "LI-05",
    "invalid_cut_reject": "CUT-05 -> E2E-07",
    "unsigned_native_package_reject": "HP-11",
}

POSITIVE_COVERAGE = {
    "local_runtime": "LR-01 -> E2E-01",
    "docker_network": "NET-02 -> E2E-02",
    "layer_hotplug": "LI-01/03/04 -> E2E-03/04/09",
    "avatar_package": "AV-01/02/08 -> E2E-05/10",
    "local_save_load": "CUT-04 -> E2E-06",
}

CUT_REASON_KINDS = {"orphan_receive"}


def _implemented_row(sample_id: str) -> dict[str, Any]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown implemented alpha E2E sample: {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "alpha-e2e",
            "source_root": str(E2E_ROOT),
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _run_json_command(command: list[str]) -> dict[str, Any]:
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"command did not return JSON: {' '.join(command)}\n{completed.stdout}"
        ) from error


def _load_component_report(kind: str, sample_id: str) -> dict[str, Any]:
    if kind == "local-runtime" and sample_id == "LR-01":
        return _run_json_command(
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
    if kind == "layer-insertion":
        scenario = {
            "LI-01": "debug-admin",
            "LI-03": "auth-update",
            "LI-04": "rate-limit",
        }.get(sample_id)
        if scenario is None:
            raise ValueError(f"unsupported layer-insertion sample for E2E bridge: {sample_id}")
        return _run_json_command(
            [
                "cargo",
                "run",
                "-q",
                "-p",
                "mir-runtime",
                "--example",
                "mirrorea_alpha_layer_insertion_runtime",
                "--",
                scenario,
            ]
        )
    if kind == "network-docker" and sample_id == "NET-02":
        return _run_json_command(
            [
                "python3",
                "scripts/alpha_network_docker_e2e.py",
                "run",
                sample_id,
                "--format",
                "json",
            ]
        )
    if kind == "avatar-runtime" and sample_id in {"AV-01", "AV-02", "AV-08"}:
        return _run_json_command(
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
                sample_id,
            ]
        )
    if kind == "cut-save-load" and sample_id == "CUT-04":
        return _run_json_command(
            [
                "python3",
                "scripts/alpha_cut_save_load_samples.py",
                "run",
                sample_id,
                "--format",
                "json",
            ]
        )
    raise ValueError(f"unsupported component request: {kind} {sample_id}")


def _build_cut05_bridge_report() -> dict[str, Any]:
    fixture_path = CUT_ROOT / "cut-05-inconsistent_distributed_snapshot_rejected.expected.json"
    fixture = read_json(fixture_path)
    expected_rows = filter_reason_rows(
        read_fixture_checked_reason_codes(fixture), CUT_REASON_KINDS
    )

    with tempfile.TemporaryDirectory(prefix="alpha-e2e-cut-") as temp_dir:
        artifact_path = Path(temp_dir) / "artifact.json"
        artifact_path.write_text(
            json.dumps(
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "orphan_receive",
                                "receive_event": "receive_envelope",
                                "missing_predecessor": "send_envelope",
                            }
                        ],
                    }
                }
            ),
            encoding="utf-8",
        )
        artifact = read_json(artifact_path)

    scope, actual_reason_rows = read_actual_reason_code_candidates(artifact)
    actual_rows = filter_reason_rows(actual_reason_rows, CUT_REASON_KINDS)
    status, exit_code = status_for_rows(
        expected_rows,
        actual_rows,
        missing_status="sample_expected_reason_rows_missing",
    )
    if exit_code != 0:
        raise RuntimeError(
            f"CUT-05 bridge validation failed with status={status}: expected {expected_rows!r} actual {actual_rows!r}"
        )
    return {
        "fixture_sample_id": "CUT-05",
        "checker_cluster": "alpha_cut_save_load_static_floor",
        "reason_codes_scope": scope,
        "status": status,
        "matched_reason_rows": actual_rows,
        "note": "checker-backed invalid distributed cut rejection only; no distributed save/load runtime claim",
    }


def _base_report(
    *,
    sample_id: str,
    sample_name: str,
    kind: str,
    purpose: str,
    expected: str,
) -> dict[str, Any]:
    return {
        "sample_id": sample_id,
        "sample_name": sample_name,
        "family": "e2e",
        "status": "implemented-thin-runner",
        "phase": "Phase 8",
        "stage": "Stage F",
        "kind": kind,
        "purpose": purpose,
        "expected": expected,
        "claims": {
            "runnable": True,
            "implemented": True,
            "active_root": False,
            "stage_f_complete": False,
        },
        "current_validation": {
            "mode": "thin_runner_over_existing_component_floors",
            "runner": f"python3 scripts/alpha_e2e_samples.py run {sample_id} --format json",
            "note": "Composes existing alpha-local runtime/checker floors without parsing samples/alpha/e2e/*.mir.",
        },
    }


def _build_e2e_01_report() -> dict[str, Any]:
    local = _load_component_report("local-runtime", "LR-01")
    report = _base_report(
        sample_id="E2E-01",
        sample_name="local_integrated_sugoroku",
        kind="positive",
        purpose="local runtime",
        expected="roll/publish/handoff",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "local-runtime",
                    "sample_id": "LR-01",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku",
                }
            ],
            "evidence_summary": {
                "dispatch_outcomes": [
                    record["dispatch_outcome"]
                    for record in local.get("dispatch_records", [])
                ],
                "visible_history_count": len(local.get("visible_history", [])),
                "event_dag_node_count": len(local.get("event_dag", {}).get("nodes", [])),
                "event_dag_edge_count": len(local.get("event_dag", {}).get("edges", [])),
                "visualization_surface": "event_dag_json",
                "message_envelope_lanes": local.get("message_envelope_lanes", []),
            },
            "terminal_outcome": local.get("terminal_outcome"),
            "reason_family": local.get("reason_family"),
            "what_it_proves": [
                "a single integrated local world/participant path exists in the Stage-B runtime floor",
                "roll/publish/witness/handoff remain explicit in the event DAG export",
            ],
            "what_it_does_not_prove": [
                "Docker/network execution",
                "save/load completion",
                "final Stage F completion",
            ],
            "deferred": [
                "remaining Stage-E visualization/devtools rows",
                "parser/front-door execution",
            ],
        }
    )
    return report


def _build_e2e_02_report() -> dict[str, Any]:
    network = _load_component_report("network-docker", "NET-02")
    participant = network["participant"]
    report = _base_report(
        sample_id="E2E-02",
        sample_name="docker_two_node_sugoroku",
        kind="positive",
        purpose="network E2E",
        expected="envelope over Docker network",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "network-docker",
                    "sample_id": "NET-02",
                    "command": "python3 scripts/alpha_network_docker_e2e.py run NET-02 --format json",
                }
            ],
            "evidence_summary": {
                "transport_surface": network.get("transport_surface"),
                "transport_medium": network.get("transport_medium"),
                "bridge_process_count": len(participant.get("bridge_processes", [])),
                "observer_route_trace_count": len(
                    participant.get("observer_route_trace", [])
                ),
                "world_active_participants": participant.get("world_active_participants", []),
            },
            "terminal_outcome": participant.get("terminal_outcome"),
            "reason_family": participant.get("reason_family"),
            "what_it_proves": [
                "an integrated world/participant path crosses a Docker Compose TCP boundary",
                "transport evidence stays separate from capability/witness/admission semantics",
            ],
            "what_it_does_not_prove": [
                "production WAN/session/replay",
                "distributed save/load",
                "final transport ABI",
            ],
            "deferred": [
                "route rebinding/no-shadow row",
                "network partition typed failure row",
                "medium-change contract-preservation row",
            ],
        }
    )
    return report


def _build_e2e_03_report() -> dict[str, Any]:
    layer = _load_component_report("layer-insertion", "LI-01")
    report = _base_report(
        sample_id="E2E-03",
        sample_name="hotplug_debug_layer_runtime",
        kind="positive",
        purpose="live layer insert",
        expected="trace begins after attach",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "layer-insertion",
                    "sample_id": "LI-01",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- debug-admin",
                }
            ],
            "evidence_summary": {
                "pre_attach_trace_count": len(layer.get("pre_attach_trace_rows", [])),
                "post_attach_trace_count": len(layer.get("post_attach_trace_rows", [])),
                "active_layers_after": layer.get("active_layers_after", []),
                "source_runtime_sample_ref": layer.get("source_runtime_sample_ref"),
                "visualization_surface": "post_attach_trace_rows",
            },
            "terminal_outcome": layer.get("terminal_outcome"),
            "reason_family": None,
            "what_it_proves": [
                "debug-layer attach is authority-gated and visible only after activation",
                "typed trace output is available in the integrated bridge path",
            ],
            "what_it_does_not_prove": [
                "detach lifecycle completion",
                "final public hot-plug ABI",
                "Stage E completion",
            ],
            "deferred": [
                "detach/no-dangling runtime row",
                "runtime-library hot-plug attach row",
            ],
        }
    )
    return report


def _build_e2e_04_report() -> dict[str, Any]:
    layer = _load_component_report("layer-insertion", "LI-04")
    preview = layer.get("runtime_preview", {})
    report = _base_report(
        sample_id="E2E-04",
        sample_name="hotplug_ratelimit_runtime",
        kind="positive/negative",
        purpose="rate-limit behavior",
        expected="Reject(RateLimited)",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "layer-insertion",
                    "sample_id": "LI-04",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- rate-limit",
                }
            ],
            "evidence_summary": {
                "compatibility_path": layer.get("compatibility", {}).get("accepted_path"),
                "runtime_preview_kind": preview.get("preview_kind"),
                "runtime_preview_terminal_outcome": preview.get("terminal_outcome"),
                "runtime_preview_reason_refs": preview.get("reason_refs", []),
                "post_attach_trace_count": len(layer.get("post_attach_trace_rows", [])),
            },
            "terminal_outcome": layer.get("terminal_outcome"),
            "reason_family": None,
            "what_it_proves": [
                "declared-failure rate-limit behavior is preserved through the integrated attach path",
                "layer insertion does not need to hide rejection reasons behind transport/runtime errors",
            ],
            "what_it_does_not_prove": [
                "global scheduling/fairness behavior",
                "detach/migration ordering",
                "Stage F completion",
            ],
            "deferred": [
                "capability-missing hot-plug row",
                "activation-cut rollback-prevention row",
            ],
        }
    )
    return report


def _build_e2e_05_report() -> dict[str, Any]:
    placeholder = _load_component_report("avatar-runtime", "AV-01")
    custom = _load_component_report("avatar-runtime", "AV-02")
    report = _base_report(
        sample_id="E2E-05",
        sample_name="avatar_runtime_package",
        kind="positive",
        purpose="runtime package attach",
        expected="placeholder/custom avatar",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "avatar-runtime",
                    "sample_id": "AV-01",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- run AV-01",
                },
                {
                    "family": "avatar-runtime",
                    "sample_id": "AV-02",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- run AV-02",
                },
            ],
            "evidence_summary": {
                "variant_outcomes": [
                    {
                        "sample_id": "AV-01",
                        "terminal_outcome": placeholder.get("terminal_outcome"),
                        "selected_representation": placeholder.get(
                            "representation_state", {}
                        ).get("selected_representation"),
                    },
                    {
                        "sample_id": "AV-02",
                        "terminal_outcome": custom.get("terminal_outcome"),
                        "selected_representation": custom.get(
                            "representation_state", {}
                        ).get("selected_representation"),
                    },
                ],
                "manifest_field_lanes": placeholder.get("manifest_field_lanes", []),
                "abstract_role_inventory": custom.get("abstract_role_inventory", []),
                "visualization_surface": "representation_state_json",
            },
            "terminal_outcome": "accepted",
            "reason_family": None,
            "what_it_proves": [
                "both placeholder and custom Mir avatar paths fit the current runtime-private admission floor",
                "avatar/package evidence remains abstract-role/package oriented rather than engine-core semantics",
            ],
            "what_it_does_not_prove": [
                "native execution",
                "VRM / VRChat / Unity completion",
                "final avatar API",
            ],
            "deferred": [
                "trusted native sandbox-limited row",
                "detach active avatar lifecycle row",
            ],
        }
    )
    return report


def _build_e2e_06_report() -> dict[str, Any]:
    save_load = _load_component_report("cut-save-load", "CUT-04")
    report = _base_report(
        sample_id="E2E-06",
        sample_name="local_save_load_continue",
        kind="positive",
        purpose="local save/load",
        expected="resume local runtime frontier",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "cut-save-load",
                    "sample_id": "CUT-04",
                    "command": "python3 scripts/alpha_cut_save_load_samples.py run CUT-04 --format json",
                }
            ],
            "evidence_summary": {
                "saved_owner": save_load.get("saved_owner"),
                "resumed_owner": save_load.get("resumed_owner"),
                "state_roundtrip_equal": save_load.get("state_roundtrip_equal"),
                "restored_membership_epoch": save_load.get("saved_runtime_snapshot", {})
                .get("membership", {})
                .get("membership_epoch"),
                "restored_visible_history_count": len(
                    save_load.get("restored_visible_history", [])
                ),
                "visible_history_after_resume_count": len(
                    save_load.get("visible_history_after_resume", [])
                ),
                "resumed_dispatch_outcomes": [
                    record.get("dispatch_outcome")
                    for record in save_load.get("resumed_dispatch_records", [])
                ],
                "visualization_surface": "resumed_event_dag_json",
            },
            "terminal_outcome": save_load.get("terminal_outcome"),
            "reason_family": None,
            "what_it_proves": [
                "a room-local runtime savepoint can restore the runtime snapshot, saved owner marker, and visible-history frontier before resumed dispatch",
                "Stage F keeps local save/load explicit without widening to distributed durable persistence",
            ],
            "what_it_does_not_prove": [
                "distributed save/load runtime",
                "Z-cycle handling completion",
                "Stage E completion",
            ],
            "deferred": [
                "CUT-11/12 Z-cycle widening",
                "CUT-16/17 stale witness/membership load verdict split",
                "Stage F completion after remaining Stage-E rows close",
            ],
        }
    )
    return report


def _build_e2e_07_report() -> dict[str, Any]:
    cut = _build_cut05_bridge_report()
    report = _base_report(
        sample_id="E2E-07",
        sample_name="distributed_inconsistent_save_negative",
        kind="negative",
        purpose="invalid cut",
        expected="rejected",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "cut-save-load",
                    "sample_id": "CUT-05",
                    "command": "python3 scripts/alpha_e2e_samples.py run E2E-07 --format json",
                }
            ],
            "evidence_summary": cut,
            "terminal_outcome": "rejected",
            "reason_family": "invalid_cut",
            "what_it_proves": [
                "distributed inconsistent snapshot rejection is checker-backed in the current alpha line",
                "Stage F can keep a distributed non-claim explicit instead of silently widening scope",
            ],
            "what_it_does_not_prove": [
                "distributed save/load runtime",
                "local save/load positive path",
                "Z-cycle handling completion",
            ],
            "deferred": [
                "CUT-11/12 Z-cycle widening",
                "CUT-16/17 stale witness/membership load verdict split",
            ],
        }
    )
    return report


def _build_e2e_09_report() -> dict[str, Any]:
    layer = _load_component_report("layer-insertion", "LI-03")
    report = _base_report(
        sample_id="E2E-09",
        sample_name="layer_auth_then_hotplug",
        kind="positive/negative",
        purpose="auth layer affects hot-plug",
        expected="correct contract handling",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "layer-insertion",
                    "sample_id": "LI-03",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- auth-update",
                }
            ],
            "evidence_summary": {
                "contract_mode": layer.get("compatibility", {}).get("contract_mode"),
                "compatibility_path": layer.get("compatibility", {}).get("accepted_path"),
                "contract_update_ref": layer.get("attach_request", {}).get(
                    "contract_update_ref"
                ),
                "active_layers_after": layer.get("active_layers_after", []),
                "source_runtime_sample_ref": layer.get("source_runtime_sample_ref"),
            },
            "terminal_outcome": layer.get("terminal_outcome"),
            "reason_family": None,
            "what_it_proves": [
                "auth-layer insertion can proceed via explicit contract-update path rather than silent strengthening",
                "integrated hot-plug evidence keeps contract variance explicit",
            ],
            "what_it_does_not_prove": [
                "authorization product completion",
                "distributed activation ordering",
                "final public contract-update ABI",
            ],
            "deferred": [
                "observer redaction visualization family",
                "detach lifecycle completion",
            ],
        }
    )
    return report


def _build_e2e_10_report() -> dict[str, Any]:
    avatar = _load_component_report("avatar-runtime", "AV-08")
    representation_state = avatar.get("representation_state", {})
    verdict = avatar.get("hotplug_skeleton", {}).get("verdict", {})
    report = _base_report(
        sample_id="E2E-10",
        sample_name="package_missing_runtime_fallback",
        kind="positive",
        purpose="browser-like fallback",
        expected="placeholder",
    )
    report.update(
        {
            "component_refs": [
                {
                    "family": "avatar-runtime",
                    "sample_id": "AV-08",
                    "command": "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- run AV-08",
                }
            ],
            "evidence_summary": {
                "requested_package_verdict_kind": verdict.get("verdict_kind"),
                "selected_package_id": representation_state.get("selected_package_id"),
                "selected_representation": representation_state.get(
                    "selected_representation"
                ),
                "fallback_applied": representation_state.get("fallback_applied"),
                "fallback_reason": representation_state.get("fallback_reason"),
                "visualization_surface": "representation_state_json",
            },
            "terminal_outcome": avatar.get("terminal_outcome"),
            "reason_family": avatar.get("reason_family"),
            "what_it_proves": [
                "runtime-unavailable package requests can degrade to an explicit placeholder path",
                "fallback extends guarded logical availability rather than the package lifetime itself",
            ],
            "what_it_does_not_prove": [
                "native execution fallback",
                "full browser/client runtime support",
                "final avatar/package ABI",
            ],
            "deferred": [
                "runtime package detach lifecycle",
                "trusted native sandbox-limited provider row",
            ],
        }
    )
    return report


def _build_report(sample_id: str) -> dict[str, Any]:
    if sample_id == "E2E-01":
        return _build_e2e_01_report()
    if sample_id == "E2E-02":
        return _build_e2e_02_report()
    if sample_id == "E2E-03":
        return _build_e2e_03_report()
    if sample_id == "E2E-04":
        return _build_e2e_04_report()
    if sample_id == "E2E-05":
        return _build_e2e_05_report()
    if sample_id == "E2E-06":
        return _build_e2e_06_report()
    if sample_id == "E2E-07":
        return _build_e2e_07_report()
    if sample_id == "E2E-09":
        return _build_e2e_09_report()
    if sample_id == "E2E-10":
        return _build_e2e_10_report()
    raise ValueError(f"unsupported alpha E2E sample: {sample_id}")


def _load_expected_sidecar(row: dict[str, Any]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_sidecar"]).read_text(encoding="utf-8"))


def _validate_report(sample_id: str, row: dict[str, Any], report: dict[str, Any]) -> None:
    if report.get("sample_id") != sample_id:
        raise RuntimeError(
            f"{sample_id}: report sample_id mismatch: {report.get('sample_id')!r}"
        )
    if report.get("terminal_outcome") != row["expected_terminal_outcome"]:
        raise RuntimeError(
            f"{sample_id}: expected terminal_outcome {row['expected_terminal_outcome']!r} "
            f"but observed {report.get('terminal_outcome')!r}"
        )
    expected_sidecar = _load_expected_sidecar(row)
    if report != expected_sidecar:
        raise RuntimeError(f"{sample_id}: sidecar drift detected")


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    report = _build_report(sample_id)
    _validate_report(sample_id, row, report)
    return report


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            run_sample(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
        else:
            passed.append(sample_id)
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "stage_f_complete": False,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": str(E2E_ROOT),
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "planned_only_rows": list(PLANNED_ONLY_ROWS),
        "positive_coverage_refs": dict(POSITIVE_COVERAGE),
        "negative_coverage_refs": dict(NEGATIVE_COVERAGE),
        "visualization_surface_refs": [
            "LR-01.event_dag",
            "CUT-04.resumed_event_dag",
            "LI-01.post_attach_trace_rows",
            "NET-02.observer_route_trace",
            "AV-08.representation_state",
        ],
        "validation_floor": [
            "cargo test -p mirrorea-core --test runtime_substrate",
            "cargo test -p mir-runtime --test alpha_local_runtime --test alpha_layer_insertion_runtime --test alpha_network_runtime --test alpha_avatar_runtime",
            "cargo test -p mir-runtime --test alpha_cut_save_load_runtime",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- save-load-resume",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout",
            "cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout",
            "python3 -m unittest scripts.tests.test_alpha_cut_save_load_checker scripts.tests.test_alpha_cut_save_load_samples scripts.tests.test_alpha_visualization_samples scripts.tests.test_alpha_e2e_samples scripts.tests.test_validate_docs",
            "python3 scripts/alpha_cut_save_load_samples.py check-all --format json",
            "python3 scripts/alpha_visualization_samples.py check-all --format json",
            "python3 scripts/alpha_network_docker_e2e.py check-all --format json",
            "python3 scripts/alpha_avatar_runtime_samples.py check-all --format json",
            "python3 scripts/alpha_e2e_samples.py run E2E-06 --format json",
            "python3 scripts/alpha_e2e_samples.py check-all --format json",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "stage_e_complete": False,
        "stage_f_complete": False,
        "remaining_blockers": [
            "remaining Stage-E rows `VIS-02/04/05/09/12` still need honest actualization or explicit deferral",
            "Stage F completion claim remains blocked on the above plus broader lifecycle widening",
        ],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    if isinstance(payload, dict) and "sample_id" in payload:
        lines = [
            f"{payload['sample_id']} alpha_e2e",
            f"  outcome: {payload.get('terminal_outcome')}",
            f"  reason_family: {payload.get('reason_family')}",
        ]
        evidence_summary = payload.get("evidence_summary")
        if isinstance(evidence_summary, dict):
            lines.append(f"  evidence_keys: {', '.join(sorted(evidence_summary.keys()))}")
        return "\n".join(lines)
    return json.dumps(payload, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Alpha-0 integrated E2E bridge runner")
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser("list")
    list_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
    run_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    check_parser = subparsers.add_parser("check-all")
    check_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")

    closeout_parser = subparsers.add_parser("closeout")
    closeout_parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    return parser


def main(argv: list[str]) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "list":
        payload = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "closeout":
        payload = closeout()
    else:  # pragma: no cover
        raise AssertionError(f"unhandled command: {args.command}")

    if args.format == "json":
        json.dump(payload, sys.stdout, indent=2)
        sys.stdout.write("\n")
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
