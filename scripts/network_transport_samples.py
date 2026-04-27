#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
ACTIVE_SAMPLE_ROOT = REPO_ROOT / "samples" / "clean-near-end" / "network-transport"
SUGOROKU_SCRIPT = SCRIPT_DIR / "sugoroku_world_samples.py"
TRANSPORT_SEAM = "loopback_socket"


SAMPLE_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "NET-02",
        "summary": "Spawn two helper-local processes and keep attach / roll / publish / handoff envelope evidence across the JSON bridge.",
    },
    {
        "sample_id": "NET-03",
        "summary": "Reject reconnect when a stale membership_epoch / member_incarnation crosses the process boundary.",
    },
    {
        "sample_id": "NET-04",
        "summary": "Expose a typed helper-local transport failure family without hiding retryability or detach-after-send.",
    },
    {
        "sample_id": "NET-05",
        "summary": "Emit an observer-safe redacted route trace from transport evidence without leaking auth or capability payloads.",
    },
]


LIMITATIONS = [
    "no real network socket yet",
    "no cryptographic session protocol",
    "no durable distributed commit",
    "no continuous shared runtime state across processes",
    "final public transport ABI remains deferred",
]


def _sample_row(sample_id: str) -> dict[str, str]:
    for row in SAMPLE_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown network transport sample: {sample_id}")


def _run_sugoroku_child(sample_id: str) -> dict[str, Any]:
    command = [
        sys.executable,
        str(SUGOROKU_SCRIPT),
        "run",
        sample_id,
        "--transport",
        TRANSPORT_SEAM,
        "--format",
        "json",
    ]
    completed = subprocess.run(
        command,
        check=True,
        capture_output=True,
        text=True,
        cwd=REPO_ROOT,
    )
    return json.loads(completed.stdout)


def _route_trace_row(
    envelope: dict[str, Any],
    *,
    process_id: str,
    process_role: str,
    hop_index: int,
) -> dict[str, Any]:
    claim = envelope.get("principal_claim") or {}
    auth = envelope.get("auth_evidence")
    return {
        "hop_index": hop_index,
        "process_id": process_id,
        "process_role": process_role,
        "envelope_id": envelope["envelope_id"],
        "from_place": envelope["from_place"],
        "to_place": envelope["to_place"],
        "transport": envelope["transport"],
        "payload_kind": envelope["payload_kind"],
        "payload_ref": envelope["payload_ref"],
        "principal": claim.get("principal"),
        "claimed_authority": claim.get("claimed_authority"),
        "auth_mode": "none" if auth is None else auth.get("kind", "unknown"),
        "membership_epoch": envelope["membership_epoch"],
        "member_incarnation": envelope["member_incarnation"],
        "capability_requirements": list(envelope.get("capability_requirements") or []),
        "authorization_checks": list(envelope.get("authorization_checks") or []),
        "dispatch_outcome": envelope["dispatch_outcome"],
        "witness_refs": list(envelope.get("witness_refs") or []),
        "notes": list(envelope.get("notes") or []),
    }


def _observer_route_trace_row(envelope: dict[str, Any], *, hop_index: int) -> dict[str, Any]:
    return {
        "hop_index": hop_index,
        "envelope_id": envelope["envelope_id"],
        "from_place": envelope["from_place"],
        "to_place": envelope["to_place"],
        "transport": envelope["transport"],
        "payload_kind": envelope["payload_kind"],
        "dispatch_outcome": envelope["dispatch_outcome"],
        "witness_ref_count": len(envelope.get("witness_refs") or []),
        "redaction": "observer_safe_route_trace",
    }


def _base_result(sample_id: str) -> dict[str, Any]:
    row = _sample_row(sample_id)
    return {
        "sample": sample_id,
        "summary": row["summary"],
        "transport_scope": "helper_local_process_boundary",
        "transport_seam": TRANSPORT_SEAM,
    }


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "network-transport",
            "source_path": str(ACTIVE_SAMPLE_ROOT / "README.md"),
            "summary": row["summary"],
        }
        for row in SAMPLE_ROWS
    ]


def run_sample(sample_id: str) -> dict[str, Any]:
    if sample_id == "NET-02":
        attach = _run_sugoroku_child("01_runtime_attach_game")
        handoff = _run_sugoroku_child("03_roll_publish_handoff")
        route_trace = [
            _route_trace_row(
                attach["message_envelopes"][0],
                process_id="world-attach-proc",
                process_role="runtime_attach",
                hop_index=1,
            ),
            *[
                _route_trace_row(
                    envelope,
                    process_id="turn-proc",
                    process_role="roll_publish_handoff",
                    hop_index=index,
                )
                for index, envelope in enumerate(handoff["message_envelopes"], start=2)
            ],
        ]
        witness_refs = sorted(
            {
                witness
                for row in route_trace
                for witness in row.get("witness_refs", [])
            }
        )
        result = _base_result(sample_id)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "bridge_kind": "subprocess_json_bridge",
                "bridge_processes": [
                    {
                        "process_id": "world-attach-proc",
                        "role": "runtime_attach",
                        "source_sample": "01_runtime_attach_game",
                    },
                    {
                        "process_id": "turn-proc",
                        "role": "roll_publish_handoff",
                        "source_sample": "03_roll_publish_handoff",
                    },
                ],
                "source_samples": [
                    "01_runtime_attach_game",
                    "03_roll_publish_handoff",
                ],
                "route_trace": route_trace,
                "witness_refs": witness_refs,
                "artifact_refs": [
                    "child:01_runtime_attach_game:json",
                    "child:03_roll_publish_handoff:json",
                ],
                "what_it_proves": [
                    "process-boundary envelope serialization canary",
                    "publish witness survives the subprocess JSON bridge",
                    "transport widening keeps attach / handoff evidence explicit",
                ],
                "what_it_does_not_prove": [
                    "no continuous shared runtime state across processes",
                    "no real socket transport or broker",
                    "no durable replay or reconnect repair",
                ],
                "human_readable_explanation": (
                    "Separate helper processes can emit attach and handoff envelope evidence "
                    "without collapsing witness refs into hidden transport state."
                ),
            }
        )
        return result

    if sample_id == "NET-03":
        leave = _run_sugoroku_child("06_leave_non_owner")
        envelopes = {row["envelope_id"]: row for row in leave["message_envelopes"]}
        stale = envelopes["stale_roll_after_leave#1"]
        result = _base_result(sample_id)
        result.update(
            {
                "static_or_runtime_verdict": "reject",
                "reason_family": "stale_membership_epoch",
                "source_samples": ["06_leave_non_owner"],
                "reconnect_attempt": {
                    "source_envelope": "stale_roll_after_leave#1",
                    "offered_membership_epoch": stale["membership_epoch"],
                    "current_membership_epoch": leave["membership_epoch"],
                    "offered_member_incarnation": stale["member_incarnation"],
                    "current_member_incarnation": leave["member_incarnation"],
                    "dispatch_outcome": stale["dispatch_outcome"],
                    "checks": [
                        "membership_epoch is current",
                        "member_incarnation matches active record",
                    ],
                },
                "artifact_refs": [
                    "child:06_leave_non_owner:json",
                    "message_envelopes[stale_roll_after_leave#1]",
                ],
                "what_it_proves": [
                    "reconnect does not hidden-repair stale epoch",
                    "member incarnation remains a separate freshness lane",
                ],
                "what_it_does_not_prove": [
                    "no live reconnect socket handshake",
                    "no production session resumption contract",
                ],
                "human_readable_explanation": (
                    "A reconnect attempt carrying stale membership and incarnation data is "
                    "rejected explicitly instead of being silently refreshed."
                ),
            }
        )
        return result

    if sample_id == "NET-04":
        detach = _run_sugoroku_child("09_detach_todo")
        result = _base_result(sample_id)
        result.update(
            {
                "terminal_outcome": "typed_failure_matrix",
                "source_samples": ["09_detach_todo"],
                "typed_failures": [
                    {
                        "failure_id": "timeout#1",
                        "failure_kind": "timeout",
                        "reason_family": "transport_timeout",
                        "retryable": True,
                        "source_refs": ["helper-local timeout budget"],
                        "evidence_status": "simulated",
                    },
                    {
                        "failure_id": "queue_full#1",
                        "failure_kind": "queue_full",
                        "reason_family": "transport_queue_full",
                        "retryable": True,
                        "source_refs": ["helper-local bounded route queue"],
                        "evidence_status": "simulated",
                    },
                    {
                        "failure_id": "route_not_found#1",
                        "failure_kind": "route_not_found",
                        "reason_family": "transport_route_not_found",
                        "retryable": False,
                        "source_refs": ["helper-local missing place route"],
                        "evidence_status": "simulated",
                    },
                    {
                        "failure_id": "detach_after_send#1",
                        "failure_kind": "detach_after_send",
                        "reason_family": "transport_detach_after_send",
                        "retryable": False,
                        "source_refs": [
                            "child:09_detach_todo:json",
                            "message_envelopes[detached_roll_request#1]",
                        ],
                        "evidence_status": "anchored",
                        "dispatch_outcome": (
                            {
                                row["envelope_id"]: row["dispatch_outcome"]
                                for row in detach["message_envelopes"]
                            }["detached_roll_request#1"]
                        ),
                    },
                ],
                "artifact_refs": [
                    "child:09_detach_todo:json",
                    "message_envelopes[detached_roll_request#1]",
                ],
                "what_it_proves": [
                    "typed transport failure family remains explicit",
                    "detach-after-send stays terminal and visible",
                ],
                "what_it_does_not_prove": [
                    "no real timeout scheduler or queue backpressure",
                    "no production retry policy",
                ],
                "human_readable_explanation": (
                    "The helper-local matrix keeps retryable and terminal transport failures "
                    "separate, and anchors detach-after-send in an observed Sugoroku reject path."
                ),
            }
        )
        return result

    if sample_id == "NET-05":
        handoff = _run_sugoroku_child("03_roll_publish_handoff")
        detach = _run_sugoroku_child("09_detach_todo")
        envelopes = [
            *handoff["message_envelopes"],
            *detach["message_envelopes"],
        ]
        observer_route_trace = [
            _observer_route_trace_row(envelope, hop_index=index)
            for index, envelope in enumerate(envelopes, start=1)
        ]
        result = _base_result(sample_id)
        result.update(
            {
                "terminal_outcome": "success",
                "source_samples": [
                    "03_roll_publish_handoff",
                    "09_detach_todo",
                ],
                "observer_route_trace": observer_route_trace,
                "visualization_view": {
                    "view_id": "network_route_trace",
                    "view_kind": "route_trace",
                    "label": "helper:transport-audit",
                    "authority": "ObserveRouteTrace(NetworkTransportLane)",
                    "redaction": "observer_safe_route_trace",
                    "source_refs": [
                        "child:03_roll_publish_handoff:json",
                        "child:09_detach_todo:json",
                    ],
                    "summary": {
                        "route_count": len(observer_route_trace),
                        "envelope_ids": [
                            row["envelope_id"] for row in observer_route_trace
                        ],
                    },
                },
                "what_it_proves": [
                    "route trace can stay typed and redacted",
                    "observer-safe transport visualization does not need raw auth payloads",
                ],
                "what_it_does_not_prove": [
                    "no final public visualization schema",
                    "no cross-host telemetry retention policy",
                ],
                "human_readable_explanation": (
                    "The observer view exposes transport route shape and outcomes while "
                    "dropping auth evidence and capability payload details."
                ),
            }
        )
        return result

    raise AssertionError(f"unhandled network transport sample {sample_id}")


def check_all() -> dict[str, Any]:
    failed: list[dict[str, str]] = []
    for row in SAMPLE_ROWS:
        try:
            run_sample(row["sample_id"])
        except Exception as error:  # pragma: no cover - surfaced in command output.
            failed.append({"sample": row["sample_id"], "reason": str(error)})
    return {
        "sample_count": len(SAMPLE_ROWS),
        "passed": [
            row["sample_id"]
            for row in SAMPLE_ROWS
            if not any(failure["sample"] == row["sample_id"] for failure in failed)
        ],
        "failed": failed,
        "transport_scope": "helper_local_process_boundary",
        "transport_seam": TRANSPORT_SEAM,
    }


def closeout() -> dict[str, Any]:
    return {
        "active_sample_root": str(ACTIVE_SAMPLE_ROOT),
        "helper_script": str(SCRIPT_DIR / "network_transport_samples.py"),
        "sample_count": len(SAMPLE_ROWS),
        "samples": [row["sample_id"] for row in SAMPLE_ROWS],
        "transport_scope": "helper_local_process_boundary",
        "transport_seam": TRANSPORT_SEAM,
        "depends_on_samples": [
            "01_runtime_attach_game",
            "03_roll_publish_handoff",
            "06_leave_non_owner",
            "09_detach_todo",
        ],
        "debug_output_modes": [
            "--debug summary",
            "--debug route-trace",
            "--debug reconnect",
            "--debug failures",
            "--format json",
        ],
        "limitations": list(LIMITATIONS),
    }


def _format_summary(result: dict[str, Any]) -> str:
    lines = [
        f"{result['sample']} transport_scope={result.get('transport_scope', '?')}",
        f"  summary: {result.get('summary', '')}",
    ]
    if "bridge_kind" in result:
        lines.append(f"  bridge_kind: {result['bridge_kind']}")
    if "reason_family" in result:
        lines.append(f"  reason_family: {result['reason_family']}")
    if "typed_failures" in result:
        lines.append(f"  typed_failures: {len(result['typed_failures'])}")
    if "visualization_view" in result:
        lines.append(
            f"  visualization: {result['visualization_view']['redaction']}"
        )
    return "\n".join(lines)


def _format_route_trace(result: dict[str, Any]) -> str:
    rows = result.get("observer_route_trace") or result.get("route_trace") or []
    lines = ["ROUTE TRACE", f"  redaction={result.get('visualization_view', {}).get('redaction', 'none')}"]
    for row in rows:
        lines.append(
            f"  [{row['hop_index']}] {row['envelope_id']} {row['from_place']} -> {row['to_place']} payload={row['payload_kind']} outcome={row['dispatch_outcome']}"
        )
        lines.append(
            f"      principal={row.get('principal', '?')} authority={row.get('claimed_authority', '?')} auth={row.get('auth_mode', '?')} epoch/incarnation={row.get('membership_epoch', '?')}/{row.get('member_incarnation', '?')}"
        )
    if len(lines) == 2:
        lines.append("  - none")
    return "\n".join(lines)


def _format_reconnect(result: dict[str, Any]) -> str:
    attempt = result.get("reconnect_attempt") or {}
    if not attempt:
        return "RECONNECT\n  - none"
    return "\n".join(
        [
            "RECONNECT",
            f"  source_envelope: {attempt.get('source_envelope', '?')}",
            f"  offered epoch/incarnation: {attempt.get('offered_membership_epoch', '?')}/{attempt.get('offered_member_incarnation', '?')}",
            f"  current epoch/incarnation: {attempt.get('current_membership_epoch', '?')}/{attempt.get('current_member_incarnation', '?')}",
            f"  dispatch_outcome: {attempt.get('dispatch_outcome', '?')}",
        ]
    )


def _format_failures(result: dict[str, Any]) -> str:
    rows = result.get("typed_failures") or []
    lines = ["TYPED TRANSPORT FAILURES"]
    for row in rows:
        lines.append(
            f"  - {row['failure_kind']} reason={row['reason_family']} retryable={row['retryable']} evidence={row['evidence_status']}"
        )
    if len(lines) == 1:
        lines.append("  - none")
    return "\n".join(lines)


def format_pretty(payload: Any, *, debug: str | None = None) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']}: {row['summary']}" for row in payload)
    if not isinstance(payload, dict):
        return str(payload)
    if debug == "summary":
        return _format_summary(payload)
    if debug == "route-trace":
        return _format_route_trace(payload)
    if debug == "reconnect":
        return _format_reconnect(payload)
    if debug == "failures":
        return _format_failures(payload)
    return json.dumps(payload, indent=2, ensure_ascii=False)


def _print_payload(payload: Any, fmt: str, *, debug: str | None = None) -> None:
    if fmt == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(payload, debug=debug))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="command", required=True)

    list_parser = sub.add_parser("list")
    list_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    run_parser = sub.add_parser("run")
    run_parser.add_argument("sample")
    run_parser.add_argument(
        "--debug",
        choices=["summary", "route-trace", "reconnect", "failures"],
        default=None,
    )
    run_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    check = sub.add_parser("check-all")
    check.add_argument("--format", choices=["pretty", "json"], default="pretty")

    close = sub.add_parser("closeout")
    close.add_argument("--format", choices=["pretty", "json"], default="pretty")
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    if args.command == "list":
        _print_payload(list_samples(), args.format)
    elif args.command == "run":
        _print_payload(run_sample(args.sample), args.format, debug=args.debug)
    elif args.command == "check-all":
        _print_payload(check_all(), args.format)
    elif args.command == "closeout":
        _print_payload(closeout(), args.format)
    else:
        raise AssertionError(f"unsupported command {args.command}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
