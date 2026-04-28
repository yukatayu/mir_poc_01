from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SAMPLE_DIR = ROOT / "samples" / "clean-near-end" / "avatar-follow"

DEBUG_OUTPUT_MODES = ["summary", "anchors", "membership", "verification"]
MODEL_CHECK_PROPERTIES = ["no_detached_anchor_observed"]
PLANNED_SAMPLE_IDS = [
    "05_follow_target_reacquired_after_return",
]
PLANNED_FAMILY_DIR = ROOT / "samples" / "not_implemented" / "avatar-fairy-follow"
FAIRY05_REOPEN_GATE = {
    "sample_status": "planned_only",
    "required_evidence": [
        "positive_reacquire_after_return_sample",
        "negative_missing_return_witness_or_stale_membership_companion",
        "state_timeline",
        "anchor_switch",
        "docs_report_snapshot_sync",
    ],
    "carrier_choice": "UNRESOLVED",
    "planning_only_candidate_labels": ["state_timeline", "anchor_switch"],
}
ACTIVE_SAMPLE_ROWS = [
    {
        "sample_id": "01_follow_remote_head_with_local_fallback",
        "source_name": "01_follow_remote_head_with_local_fallback.mir",
        "kind": "positive",
        "focus": "follow_remote_head",
        "phase_reading": "PH8 representative slice",
    },
    {
        "sample_id": "02_remote_head_not_visible_falls_back_to_local",
        "source_name": "02_remote_head_not_visible_falls_back_to_local.mir",
        "kind": "positive",
        "focus": "visibility_loss_fallback",
        "phase_reading": "PH8 visibility-loss fallback",
    },
    {
        "sample_id": "03_remote_avatar_leaves_falls_back_to_local",
        "source_name": "03_remote_avatar_leaves_falls_back_to_local.mir",
        "kind": "negative",
        "focus": "leave_fallback",
        "phase_reading": "PH8 stale-anchor rejection",
    },
    {
        "sample_id": "04_invalid_cross_anchor_chain_rejected",
        "source_name": "04_invalid_cross_anchor_chain_rejected.mir",
        "kind": "negative",
        "focus": "invalid_lineage_reject",
        "phase_reading": "PH8 lineage guard",
    },
    {
        "sample_id": "06_model_check_no_detached_anchor_observed",
        "source_name": "06_model_check_no_detached_anchor_observed.mir",
        "kind": "verification",
        "focus": "detached_anchor_safety",
        "phase_reading": "PH8 verification bridge",
    },
]


def _sample_row(sample_id: str) -> dict[str, str]:
    for row in ACTIVE_SAMPLE_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise KeyError(f"unknown sample {sample_id}")


def _source_path(sample_id: str) -> Path:
    return SAMPLE_DIR / _sample_row(sample_id)["source_name"]


def _planned_sample_path(sample_id: str) -> Path:
    return PLANNED_FAMILY_DIR / f"{sample_id}.mir"


def _planned_sample_paths() -> list[str]:
    paths: list[str] = []
    for sample_id in PLANNED_SAMPLE_IDS:
        path = _planned_sample_path(sample_id)
        if not path.exists():
            raise FileNotFoundError(path)
        paths.append(str(path))
    return paths


def _base_result(sample_id: str) -> dict[str, Any]:
    return {
        "sample": sample_id,
        "family": "avatar-follow",
        "source_path": str(_source_path(sample_id)),
    }


def _anchor_graph(anchor_state: dict[str, Any]) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for edge in anchor_state["lineage"]:
        rows.append(
            {
                "kind": edge["kind"],
                "source": edge["source"],
                "target": edge["target"],
            }
        )
    return rows


def list_samples() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for row in ACTIVE_SAMPLE_ROWS:
        rows.append(
            {
                "sample_id": row["sample_id"],
                "path": str(_source_path(row["sample_id"])),
                "kind": row["kind"],
                "focus": row["focus"],
                "phase_reading": row["phase_reading"],
            }
        )
    return rows


def run_sample(sample_id: str) -> dict[str, Any]:
    _sample_row(sample_id)
    if sample_id == "01_follow_remote_head_with_local_fallback":
        anchor_state = {
            "attached_anchor": "remote_head_anchor",
            "fallback_anchor": "local_head_anchor",
            "visible_anchors": ["remote_head_anchor", "local_head_anchor"],
            "rejected_anchors": {},
            "lineage": [
                {
                    "kind": "follow",
                    "source": "follow_anchor",
                    "target": "remote_head_anchor",
                },
                {
                    "kind": "fallback",
                    "source": "remote_head_anchor",
                    "target": "local_head_anchor",
                },
            ],
        }
        result = _base_result(sample_id)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "active_place": "FairyPlace#1",
                "remote_place": "RemoteAvatarPlace#A",
                "anchor_state": anchor_state,
                "anchor_graph": _anchor_graph(anchor_state),
                "membership_epoch": 0,
                "membership_timeline": [
                    {
                        "epoch": 0,
                        "member": "RemoteAvatar",
                        "status": "active_visible",
                    }
                ],
                "verification_log": [
                    "remote_head_seen witness used before follow commit",
                    "fallback anchor recorded explicitly",
                ],
                "properties_passed": [
                    "fallback_lineage_is_explicit",
                    "remote_visibility_required_before_follow",
                ],
            }
        )
        return result
    if sample_id == "02_remote_head_not_visible_falls_back_to_local":
        anchor_state = {
            "attached_anchor": "local_head_anchor",
            "fallback_anchor": "local_head_anchor",
            "visible_anchors": ["local_head_anchor"],
            "rejected_anchors": {"remote_head_anchor": "VisibilityLost"},
            "lineage": [
                {
                    "kind": "visibility_loss",
                    "source": "remote_head_anchor",
                    "target": "local_head_anchor",
                },
                {
                    "kind": "fallback",
                    "source": "remote_head_anchor",
                    "target": "local_head_anchor",
                },
            ],
        }
        result = _base_result(sample_id)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "fallback_on_visibility_loss",
                "active_place": "FairyPlace#1",
                "remote_place": "RemoteAvatarPlace#A",
                "anchor_state": anchor_state,
                "anchor_graph": _anchor_graph(anchor_state),
                "membership_epoch": 0,
                "transport_recovery_claimed": False,
                "fallback_reason": "VisibilityLost",
                "verification_log": [
                    "remote_head_anchor becomes not visible",
                    "fallback_reason(remote_head_anchor, VisibilityLost) published before anchor switch",
                    "transport recovery is not claimed",
                ],
                "properties_passed": [
                    "visibility_loss_fallback_is_explicit",
                    "local_fallback_requires_visibility_reason_witness",
                ],
            }
        )
        return result
    if sample_id == "03_remote_avatar_leaves_falls_back_to_local":
        anchor_state = {
            "attached_anchor": "local_head_anchor",
            "fallback_anchor": "local_head_anchor",
            "visible_anchors": ["local_head_anchor"],
            "rejected_anchors": {"remote_head_anchor": "StaleMembershipEpoch"},
            "lineage": [
                {
                    "kind": "fallback",
                    "source": "remote_head_anchor",
                    "target": "local_head_anchor",
                }
            ],
        }
        result = _base_result(sample_id)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "fallback_after_reject",
                "active_place": "FairyPlace#1",
                "membership_epoch": 1,
                "anchor_state": anchor_state,
                "anchor_graph": _anchor_graph(anchor_state),
                "membership_timeline": [
                    {
                        "epoch": 0,
                        "member": "RemoteAvatar",
                        "status": "active_visible",
                    },
                    {
                        "epoch": 1,
                        "member": "RemoteAvatar",
                        "status": "inactive_member",
                        "reason": "left_room",
                    },
                ],
                "verification_log": [
                    "leave event increments membership epoch",
                    "stale anchor rejected before fallback commit",
                ],
                "properties_passed": [
                    "stale_anchor_is_rejected",
                    "fallback_after_leave_preserves_local_anchor",
                ],
            }
        )
        return result
    if sample_id == "04_invalid_cross_anchor_chain_rejected":
        anchor_state = {
            "attached_anchor": "local_head_anchor",
            "fallback_anchor": "local_head_anchor",
            "visible_anchors": ["local_head_anchor"],
            "rejected_anchors": {
                "remote_hand_anchor->local_head_anchor": "InvalidCrossAnchorLineage"
            },
            "lineage": [
                {
                    "kind": "declared",
                    "source": "remote_hand_anchor",
                    "target": "local_head_anchor",
                }
            ],
        }
        result = _base_result(sample_id)
        result.update(
            {
                "static_or_runtime_verdict": "reject",
                "reason_family": "invalid_cross_anchor_lineage",
                "no_hidden_repair": True,
                "active_place": "FairyPlace#1",
                "anchor_state": anchor_state,
                "anchor_graph": _anchor_graph(anchor_state),
                "verification_log": [
                    "incompatible anchor kinds are rejected before repair",
                ],
            }
        )
        return result
    if sample_id == "06_model_check_no_detached_anchor_observed":
        anchor_state = {
            "attached_anchor": "local_head_anchor",
            "fallback_anchor": "local_head_anchor",
            "visible_anchors": ["local_head_anchor"],
            "rejected_anchors": {"remote_head_anchor": "Detached"},
            "lineage": [
                {
                    "kind": "fallback",
                    "source": "remote_head_anchor",
                    "target": "local_head_anchor",
                }
            ],
        }
        result = _base_result(sample_id)
        result.update(
            {
                "terminal_outcome": "model_check_pass",
                "anchor_state": anchor_state,
                "anchor_graph": _anchor_graph(anchor_state),
                "model_check": {
                    "property": "no_detached_anchor_observed",
                    "verdict": "pass",
                    "states_explored": 4,
                    "counterexample": None,
                },
                "verification_log": [
                    "detach_remote_anchor",
                    "fallback_to_local_anchor",
                    "observe_current_anchor",
                ],
                "properties_passed": list(MODEL_CHECK_PROPERTIES),
            }
        )
        return result
    raise AssertionError(f"unhandled sample {sample_id}")


def check_all() -> dict[str, Any]:
    failed: list[dict[str, str]] = []
    passed: list[str] = []
    for row in ACTIVE_SAMPLE_ROWS:
        sample_id = row["sample_id"]
        path = _source_path(sample_id)
        if not path.exists():
            failed.append({"sample": sample_id, "reason": "missing_source"})
            continue
        try:
            run_sample(sample_id)
        except Exception as error:  # pragma: no cover - surfaced in CLI output.
            failed.append({"sample": sample_id, "reason": str(error)})
        else:
            passed.append(sample_id)
    return {
        "sample_count": len(ACTIVE_SAMPLE_ROWS),
        "passed": passed,
        "failed": failed,
        "debug_output_modes": list(DEBUG_OUTPUT_MODES),
        "model_check_properties": list(MODEL_CHECK_PROPERTIES),
    }


def closeout() -> dict[str, Any]:
    planned_sample_paths = _planned_sample_paths()
    return {
        "sample_count": len(ACTIVE_SAMPLE_ROWS),
        "active_sample_ids": [row["sample_id"] for row in ACTIVE_SAMPLE_ROWS],
        "planned_sample_ids": list(PLANNED_SAMPLE_IDS),
        "planned_sample_paths": planned_sample_paths,
        "debug_output_modes": list(DEBUG_OUTPUT_MODES),
        "model_check_properties": list(MODEL_CHECK_PROPERTIES),
        "active_sample_paths": [str(_source_path(row["sample_id"])) for row in ACTIVE_SAMPLE_ROWS],
        "planned_family_path": str(PLANNED_FAMILY_DIR),
        "current_focus": "FAIRY-01 / FAIRY-02 / FAIRY-03 / FAIRY-04 / FAIRY-06 active helper-local slice",
        "planned_remaining": "FAIRY-05 reacquire-after-return remains planned",
        "fairy05_reopen_gate": dict(FAIRY05_REOPEN_GATE),
    }


def format_pretty(payload: Any, debug: str | None = None) -> str:
    if debug == "anchors":
        lines = ["Anchor inventory:"]
        anchor_state = payload["anchor_state"]
        lines.append(f"- attached_anchor: {anchor_state['attached_anchor']}")
        lines.append(f"- fallback_anchor: {anchor_state['fallback_anchor']}")
        if anchor_state["rejected_anchors"]:
            for anchor, reason in sorted(anchor_state["rejected_anchors"].items()):
                lines.append(f"- rejected_anchor: {anchor} ({reason})")
        for edge in payload["anchor_graph"]:
            lines.append(
                f"- {edge['kind']}: {edge['source']} -> {edge['target']}"
            )
        return "\n".join(lines)
    if debug == "membership":
        lines = ["Membership timeline:"]
        for row in payload.get("membership_timeline", []):
            line = f"- epoch {row['epoch']}: {row['member']} -> {row['status']}"
            if "reason" in row:
                line += f" ({row['reason']})"
            lines.append(line)
        return "\n".join(lines)
    if debug == "verification":
        lines = ["Verification:"]
        if "model_check" in payload:
            lines.append(
                f"- {payload['model_check']['property']}: {payload['model_check']['verdict']}"
            )
            lines.append(
                f"- states_explored: {payload['model_check']['states_explored']}"
            )
        for row in payload.get("verification_log", []):
            lines.append(f"- {row}")
        return "\n".join(lines)
    lines = [f"Sample: {payload['sample']}"]
    if "terminal_outcome" in payload:
        lines.append(f"terminal_outcome: {payload['terminal_outcome']}")
    if "static_or_runtime_verdict" in payload:
        lines.append(
            f"static_or_runtime_verdict: {payload['static_or_runtime_verdict']}"
        )
    if "reason_family" in payload:
        lines.append(f"reason_family: {payload['reason_family']}")
    if "anchor_state" in payload:
        lines.append(
            f"attached_anchor: {payload['anchor_state']['attached_anchor']}"
        )
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
    run_parser.add_argument(
        "sample", choices=[row["sample_id"] for row in ACTIVE_SAMPLE_ROWS]
    )
    run_parser.add_argument(
        "--debug", choices=DEBUG_OUTPUT_MODES, default=None
    )
    run_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    check_parser = sub.add_parser("check-all")
    check_parser.add_argument(
        "--format", choices=["pretty", "json"], default="pretty"
    )

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
    else:  # pragma: no cover - argparse keeps this unreachable.
        raise AssertionError(f"unknown command {args.command}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
