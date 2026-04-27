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

LIMITATIONS = [
    "no final public adapter API",
    "no browser/network/VR host schema",
    "no real transport widening beyond helper-local evidence",
    "no production visualization service contract",
]


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
        "debug_output_modes": list(DEBUG_OUTPUT_MODES),
        "preview_sample_root": str(PREVIEW_SAMPLE_ROOT),
        "planned_family_path": str(PLANNED_SAMPLE_ROOT),
        "helper_script": str(Path(__file__).resolve()),
        "current_focus": "EXT-03 / EXT-04 helper-local synthetic preview subset",
        "planned_remaining": (
            "EXT-01 local console, EXT-02 world overlay, and EXT-05 standalone "
            "visualization scenario remain planned"
        ),
        "validation_floor": (
            "helper self-consistency plus provider-boundary/local-queue anchor comparison; "
            "not direct semantic execution of the phase-9 .mir files"
        ),
        "limitations": list(LIMITATIONS),
    }


def format_pretty(payload: Any, debug: str | None = None) -> str:
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
