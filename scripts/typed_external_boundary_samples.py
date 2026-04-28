#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
PLANNED_SAMPLE_ROOT = (
    REPO_ROOT / "samples" / "not_implemented" / "typed-external-boundary"
)
PREVIEW_SAMPLE_ROOT = PLANNED_SAMPLE_ROOT

DEBUG_OUTPUT_MODES = ["summary", "envelopes", "visualization", "failures"]
PLANNED_SAMPLE_IDS = ["EXT-01", "EXT-02", "EXT-05"]
HOST_BOUNDARY_SCOPE = "helper_local_synthetic_preview"
HOST_BOUNDARY_LANES = ["request", "receipt", "failure", "visualization"]
NON_COLLAPSE_LANES = [
    "transport",
    "auth",
    "membership",
    "capability",
    "witness",
    "visualization",
]
HOST_FAMILY_GATES = [
    "final_public_adapter_api",
    "exact_host_schema",
    "browser_network_vr_host_family_split",
]

SAMPLE_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "EXT-03",
        "source_name": "03_send_room_message_local_queue.mir",
        "kind": "positive",
        "focus": "local_queue_room_message",
    },
    {
        "sample_id": "EXT-04",
        "source_name": "04_adapter_failure_typed_result.mir",
        "kind": "negative",
        "focus": "typed_adapter_failure",
    },
]

RESIDUAL_REVIEW_MATRIX: list[dict[str, Any]] = [
    {
        "sample_id": "EXT-01",
        "scenario_label": "LogText",
        "current_anchor_kind": "provider_boundary",
        "current_anchor_refs": [
            "clean-near-end:05_delegated_rng_service.provider_boundary",
            "clean-near-end:05_delegated_rng_service.provider_boundary_dispatch",
        ],
        "why_still_planned": (
            "console-shaped scenario can be misread as standard I/O builtin unless it stays "
            "on an adapter/provider boundary"
        ),
        "reopen_criteria": [
            "typed adapter request/receipt shape stays separate from Mir core primitive reading",
            "provider_boundary anchor can carry console-side request/receipt evidence without fixing final console schema",
        ],
        "debug_surface": ["summary", "envelopes"],
        "kept_later_gates": [
            "final_console_schema",
            "final_public_adapter_api",
        ],
    },
    {
        "sample_id": "EXT-02",
        "scenario_label": "ShowFloatingText",
        "current_anchor_kind": "visualization_projection_bridge",
        "current_anchor_refs": [
            "sugoroku:03_roll_publish_handoff.visualization_views",
            "sugoroku:03_roll_publish_handoff.telemetry_rows",
            "projection:helper_local_preview_floor",
        ],
        "why_still_planned": (
            "overlay scenario sits on the seam between adapter effect and visualization/projection "
            "lane, so the minimal bridge must be fixed before a standalone sample reopens"
        ),
        "reopen_criteria": [
            "label/authority/redaction evidence is visible through the current visualization lane",
            "overlay route can be described without fixing final host family split or emitted-program contract",
        ],
        "debug_surface": ["visualization", "summary"],
        "kept_later_gates": [
            "final_host_schema",
            "browser_network_vr_host_family_split",
            "final_projection_public_api",
        ],
    },
    {
        "sample_id": "EXT-05",
        "scenario_label": "DebugShowLabelledText",
        "current_anchor_kind": "visualization_redaction_lane",
        "current_anchor_refs": [
            "typed-external:EXT-03.visualization_view",
            "sugoroku:03_roll_publish_handoff.visualization_views",
        ],
        "why_still_planned": (
            "current helper cut absorbs label-restriction evidence into EXT-03, so a standalone "
            "sample should only reopen if it adds distinct redaction/authority evidence"
        ),
        "reopen_criteria": [
            "standalone sample would expose a redaction or authority case not already visible in EXT-03",
            "visualization restriction can be shown without freezing final viewer or visualization service contract",
        ],
        "debug_surface": ["visualization"],
        "kept_later_gates": [
            "final_visualization_schema",
            "public_visualization_service_contract",
        ],
    },
]

LIMITATIONS = [
    "no final public adapter API",
    "no browser/network/VR host schema",
    "no real transport widening beyond helper-local evidence",
    "no production visualization service contract",
]

HOST_BOUNDARY_PREVIEWS: dict[str, dict[str, Any]] = {
    "EXT-03": {
        "scope": HOST_BOUNDARY_SCOPE,
        "adapter_entry": "RoomMessageAdapter#local_queue",
        "request_lane": "typed_effect_request",
        "receipt_lane": "typed_effect_receipt",
        "failure_lane": None,
        "visualization_lane": "redacted_observer_view",
        "non_collapse_lanes": list(NON_COLLAPSE_LANES),
    },
    "EXT-04": {
        "scope": HOST_BOUNDARY_SCOPE,
        "adapter_entry": "RoomMessageAdapter#local_queue",
        "request_lane": "typed_effect_request",
        "receipt_lane": None,
        "failure_lane": "typed_adapter_failure",
        "visualization_lane": None,
        "non_collapse_lanes": list(NON_COLLAPSE_LANES),
    },
}


def _sample_row(sample_id: str) -> dict[str, str]:
    for row in SAMPLE_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown typed external boundary sample: {sample_id}")


def _source_path(sample_id: str) -> Path:
    return PREVIEW_SAMPLE_ROOT / _sample_row(sample_id)["source_name"]


def _base_result(sample_id: str) -> dict[str, Any]:
    row = _sample_row(sample_id)
    return {
        "sample": sample_id,
        "family": "typed-external-boundary",
        "source_path": str(_source_path(sample_id)),
        "kind": row["kind"],
        "focus": row["focus"],
    }


def _host_boundary_preview(sample_id: str) -> dict[str, Any]:
    return dict(HOST_BOUNDARY_PREVIEWS[sample_id])


def _route_row(
    *,
    step: int,
    envelope_id: str,
    from_node: str,
    to_node: str,
    transport: str,
    payload_kind: str,
    payload_ref: str,
    label: str,
    authority: str,
    auth_evidence: str | None,
    membership_epoch: int | None,
    member_incarnation: int | None,
    capability_requirements: list[str],
    dispatch_outcome: str,
    witness_refs: list[str],
    notes: list[str],
) -> dict[str, Any]:
    return {
        "step": step,
        "envelope_id": envelope_id,
        "from_node": from_node,
        "to_node": to_node,
        "transport": transport,
        "payload_kind": payload_kind,
        "payload_ref": payload_ref,
        "label": label,
        "authority": authority,
        "auth_evidence": auth_evidence,
        "membership_epoch": membership_epoch,
        "member_incarnation": member_incarnation,
        "capability_requirements": capability_requirements,
        "dispatch_outcome": dispatch_outcome,
        "witness_refs": witness_refs,
        "notes": notes,
    }


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "typed-external-boundary",
            "path": str(_source_path(row["sample_id"])),
            "kind": row["kind"],
            "focus": row["focus"],
        }
        for row in SAMPLE_ROWS
    ]


def run_sample(sample_id: str) -> dict[str, Any]:
    _sample_row(sample_id)

    if sample_id == "EXT-03":
        message_envelopes = [
            _route_row(
                step=1,
                envelope_id="room_message_request#1",
                from_node="ParticipantPlace[Alice]",
                to_node="RoomMessageAdapter#local_queue",
                transport="local_queue",
                payload_kind="typed_effect_request",
                payload_ref="SendRoomMessage(room_notice#1)",
                label="PublishedRoomNotice",
                authority="RoomNoticeAuthority.Publisher",
                auth_evidence=None,
                membership_epoch=0,
                member_incarnation=0,
                capability_requirements=[
                    "cap:room_notice_publish",
                ],
                dispatch_outcome="accepted",
                witness_refs=["room_notice_pub#1"],
                notes=[
                    "message route remains separate from auth evidence",
                    "local queue is an adapter lane, not a core primitive",
                ],
            ),
            _route_row(
                step=2,
                envelope_id="room_message_receipt#1",
                from_node="RoomMessageAdapter#local_queue",
                to_node="ParticipantPlace[Bob]",
                transport="local_queue",
                payload_kind="typed_effect_receipt",
                payload_ref="room_notice_visible#1",
                label="PublishedRoomNotice",
                authority="ObservePublishedRoomNotice",
                auth_evidence=None,
                membership_epoch=0,
                member_incarnation=0,
                capability_requirements=[
                    "cap:room_notice_observe",
                ],
                dispatch_outcome="accepted",
                witness_refs=["room_notice_pub#1"],
                notes=[
                    "published room notice stays distinct from transport auth",
                ],
            ),
        ]
        result = _base_result(sample_id)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "adapter_kind": "local_queue_room_message",
                "transport_seam": "local_queue",
                "typed_effect": {
                    "scenario_label": "SendRoomMessage",
                    "boundary_kind": "typed_effect_request",
                    "note": "scenario labels are helper-local and not final effect names",
                },
                "message_route": {
                    "queue_name": "room_message_queue#local",
                    "membership_epoch": 0,
                    "member_incarnation": 0,
                    "auth_evidence": None,
                },
                "host_boundary": _host_boundary_preview(sample_id),
                "message_envelopes": message_envelopes,
                "route_trace": message_envelopes,
                "visualization_view": {
                    "view_name": "room_message_route",
                    "label": "PublishedRoomNotice",
                    "authority": "ObservePublishedRoomNotice",
                    "redaction": "named_message_ref_only",
                    "source_refs": [
                        "room_message_request#1",
                        "room_message_receipt#1",
                    ],
                },
                "what_it_proves": [
                    "room message publication can stay on a local queue adapter lane",
                    "effect boundary, transport envelope, auth evidence, and witness refs remain separate lanes",
                ],
                "what_it_does_not_prove": [
                    "no real chat broker or multi-process queue",
                    "no final room-message serialization contract",
                ],
                "human_readable_explanation": (
                    "A room notice uses a typed local-queue adapter lane and keeps route, "
                    "authority, and witness evidence separate."
                ),
            }
        )
        return result

    if sample_id == "EXT-04":
        failure_rows = [
            {
                "failure_id": "adapter_failure#1",
                "reason_family": "AdapterQueueFull",
                "retryable": True,
                "lane": "adapter_failure",
                "domain_mutation_committed": False,
                "transport_recovery_claimed": False,
                "notes": [
                    "adapter failure stays typed",
                    "room mutation is not hidden behind retry logic",
                ],
            }
        ]
        result = _base_result(sample_id)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "typed_failure",
                "adapter_kind": "local_queue_room_message",
                "transport_seam": "local_queue",
                "host_boundary": _host_boundary_preview(sample_id),
                "typed_failures": failure_rows,
                "message_envelopes": [
                    _route_row(
                        step=1,
                        envelope_id="room_message_request#failed",
                        from_node="ParticipantPlace[Alice]",
                        to_node="RoomMessageAdapter#local_queue",
                        transport="local_queue",
                        payload_kind="typed_effect_request",
                        payload_ref="SendRoomMessage(backpressure_notice#1)",
                        label="PublishedRoomNotice",
                        authority="RoomNoticeAuthority.Publisher",
                        auth_evidence=None,
                        membership_epoch=0,
                        member_incarnation=0,
                        capability_requirements=[
                            "cap:room_notice_publish",
                        ],
                        dispatch_outcome="typed_failure",
                        witness_refs=[],
                        notes=[
                            "failure is explicit and typed",
                            "auth/membership lanes remain separate",
                        ],
                    )
                ],
                "route_trace": [
                    _route_row(
                        step=1,
                        envelope_id="room_message_request#failed",
                        from_node="ParticipantPlace[Alice]",
                        to_node="RoomMessageAdapter#local_queue",
                        transport="local_queue",
                        payload_kind="typed_effect_request",
                        payload_ref="SendRoomMessage(backpressure_notice#1)",
                        label="PublishedRoomNotice",
                        authority="RoomNoticeAuthority.Publisher",
                        auth_evidence=None,
                        membership_epoch=0,
                        member_incarnation=0,
                        capability_requirements=[
                            "cap:room_notice_publish",
                        ],
                        dispatch_outcome="typed_failure",
                        witness_refs=[],
                        notes=[
                            "failure is explicit and typed",
                            "auth/membership lanes remain separate",
                        ],
                    )
                ],
                "what_it_proves": [
                    "adapter failure is not collapsed into domain rejection or silent retry",
                    "typed failure reason remains visible to the caller",
                ],
                "what_it_does_not_prove": [
                    "no production backpressure policy",
                    "no final retry scheduler",
                ],
                "human_readable_explanation": (
                    "The adapter reports queue pressure as a typed failure result and does not "
                    "pretend that room publication succeeded."
                ),
            }
        )
        return result

    raise AssertionError(f"unhandled sample {sample_id}")


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in SAMPLE_ROWS:
        sample_id = row["sample_id"]
        path = _source_path(sample_id)
        if not path.exists():
            failed.append({"sample": sample_id, "reason": "missing_source"})
            continue
        try:
            run_sample(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample": sample_id, "reason": str(error)})
        else:
            passed.append(sample_id)
    return {
        "sample_count": len(SAMPLE_ROWS),
        "passed": passed,
        "failed": failed,
        "debug_output_modes": list(DEBUG_OUTPUT_MODES),
        "planned_sample_ids": list(PLANNED_SAMPLE_IDS),
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_count": len(SAMPLE_ROWS),
        "preview_sample_ids": [row["sample_id"] for row in SAMPLE_ROWS],
        "planned_sample_ids": list(PLANNED_SAMPLE_IDS),
        "host_boundary_scope": HOST_BOUNDARY_SCOPE,
        "host_boundary_lanes": list(HOST_BOUNDARY_LANES),
        "non_collapse_lanes": list(NON_COLLAPSE_LANES),
        "host_family_gates": list(HOST_FAMILY_GATES),
        "host_boundary_inventory": [
            {"sample_id": sample_id, **_host_boundary_preview(sample_id)}
            for sample_id in [row["sample_id"] for row in SAMPLE_ROWS]
        ],
        "residual_review_matrix": list(RESIDUAL_REVIEW_MATRIX),
        "debug_output_modes": list(DEBUG_OUTPUT_MODES),
        "preview_sample_root": str(PREVIEW_SAMPLE_ROOT),
        "planned_family_path": str(PLANNED_SAMPLE_ROOT),
        "helper_script": str(Path(__file__).resolve()),
        "current_focus": (
            "EXT-03 / EXT-04 helper-local synthetic preview subset plus residual "
            "reopen-criteria matrix for EXT-01 / EXT-02 / EXT-05"
        ),
        "planned_remaining": (
            "EXT-01 local console, EXT-02 world overlay, and EXT-05 standalone "
            "visualization scenario remain planned; each now carries indirect "
            "anchor and reopen criteria"
        ),
        "validation_floor": (
            "helper self-consistency plus provider-boundary/local-queue anchor comparison; "
            "not direct semantic execution of the phase-9 .mir files"
        ),
        "limitations": list(LIMITATIONS),
    }


def format_pretty(payload: Any, debug: str | None = None) -> str:
    if isinstance(payload, list):
        lines = ["PREVIEW SAMPLES"]
        for row in payload:
            lines.append(
                f"- {row['sample_id']} [{row['kind']}] {row['focus']} -> {row['path']}"
            )
        return "\n".join(lines)
    if "failed" in payload and "passed" in payload:
        lines = ["CHECK-ALL SUMMARY"]
        lines.append(f"- preview sample count: {payload['sample_count']}")
        lines.append(f"- passed: {', '.join(payload['passed']) or '-'}")
        lines.append(
            f"- planned residual: {', '.join(payload['planned_sample_ids']) or '-'}"
        )
        if payload["failed"]:
            lines.append(
                "- failed: "
                + ", ".join(
                    f"{row['sample']} ({row['reason']})" for row in payload["failed"]
                )
            )
        else:
            lines.append("- failed: none")
        return "\n".join(lines)
    if "residual_review_matrix" in payload:
        lines = ["CLOSEOUT SUMMARY"]
        lines.append(
            f"- preview subset: {', '.join(payload['preview_sample_ids']) or '-'}"
        )
        lines.append(
            f"- residual planned: {', '.join(payload['planned_sample_ids']) or '-'}"
        )
        for row in payload["residual_review_matrix"]:
            lines.append(
                f"- {row['sample_id']} -> {row['current_anchor_kind']} "
                f"(later: {', '.join(row['kept_later_gates'])})"
            )
        return "\n".join(lines)
    if debug == "envelopes":
        lines = ["MESSAGE ENVELOPES"]
        for row in payload.get("message_envelopes", payload.get("route_trace", [])):
            lines.append(
                f"- step {row['step']}: {row['envelope_id']} "
                f"{row['from_node']} -> {row['to_node']} [{row['transport']}] "
                f"auth={row['auth_evidence']} epoch={row['membership_epoch']} "
                f"incarnation={row['member_incarnation']} "
                f"caps={','.join(row['capability_requirements']) or '-'} "
                f"witness={','.join(row['witness_refs']) or '-'} "
                f"payload={row['payload_ref']} ({row['dispatch_outcome']})"
            )
        return "\n".join(lines)
    if debug == "visualization":
        lines = ["VISUALIZATION VIEW"]
        view = payload.get("visualization_view", {})
        if view:
            lines.append(f"- view_name: {view['view_name']}")
            lines.append(f"- label: {view['label']}")
            lines.append(f"- authority: {view['authority']}")
            lines.append(f"- redaction: {view['redaction']}")
        return "\n".join(lines)
    if debug == "failures":
        lines = ["TYPED ADAPTER FAILURES"]
        for row in payload.get("typed_failures", []):
            lines.append(
                f"- {row['failure_id']}: {row['reason_family']} "
                f"retryable={row['retryable']} lane={row['lane']}"
            )
        return "\n".join(lines)
    lines = [f"Sample: {payload['sample']}"]
    if "terminal_outcome" in payload:
        lines.append(f"terminal_outcome: {payload['terminal_outcome']}")
    if "adapter_kind" in payload:
        lines.append(f"adapter_kind: {payload['adapter_kind']}")
    if "transport_seam" in payload:
        lines.append(f"transport_seam: {payload['transport_seam']}")
    return "\n".join(lines)


def _print_payload(payload: Any, output_format: str, debug: str | None = None) -> None:
    if output_format == "json":
        json.dump(payload, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
        return
    sys.stdout.write(format_pretty(payload, debug=debug))
    sys.stdout.write("\n")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="command", required=True)

    list_parser = sub.add_parser("list")
    list_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    run_parser = sub.add_parser("run")
    run_parser.add_argument("sample", choices=[row["sample_id"] for row in SAMPLE_ROWS])
    run_parser.add_argument(
        "--debug", choices=DEBUG_OUTPUT_MODES, default=None
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
        _print_payload(list_samples(), args.format)
    elif args.command == "run":
        _print_payload(run_sample(args.sample), args.format, debug=args.debug)
    elif args.command == "check-all":
        _print_payload(check_all(), args.format)
    elif args.command == "closeout":
        _print_payload(closeout(), args.format)
    else:  # pragma: no cover
        raise AssertionError(f"unknown command {args.command}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
