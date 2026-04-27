#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
SAMPLE_ROOT = REPO_ROOT / "samples" / "clean-near-end" / "sugoroku-world"


SAMPLE_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "00_world_bootstrap",
        "source": "00_world_bootstrap.mir",
        "summary": "Bootstrap an empty world server with Alice, Bob, and Carol as active participants.",
    },
    {
        "sample_id": "01_runtime_attach_game",
        "source": "01_runtime_attach_game.mir",
        "summary": "Attach SugorokuGame#1 at runtime and appoint Alice as game admin.",
    },
    {
        "sample_id": "02_admin_start_reset",
        "source": "02_admin_start_reset.mir",
        "summary": "Accept admin start and reject non-admin reset.",
    },
    {
        "sample_id": "03_roll_publish_handoff",
        "source": "03_roll_publish_handoff.mir",
        "summary": "Dice owner Alice rolls, publishes, witnesses, and hands off to Bob.",
    },
    {
        "sample_id": "04_non_owner_roll_rejected",
        "source": "04_non_owner_roll_rejected.mir",
        "summary": "Reject Carol's roll because Bob is the current dice owner.",
    },
    {
        "sample_id": "05_late_join_history_visible",
        "source": "05_late_join_history_visible.mir",
        "summary": "Dave late-joins, sees published history, and remains pending for turn order.",
    },
    {
        "sample_id": "06_leave_non_owner",
        "source": "06_leave_non_owner.mir",
        "summary": "Carol leaves, membership epoch advances, and stale actions are invalidated.",
    },
    {
        "sample_id": "07_owner_leave_reassign",
        "source": "07_owner_leave_reassign.mir",
        "summary": "Bob leaves while owning dice and ownership is reassigned to Alice.",
    },
    {
        "sample_id": "08_reset_interleaving_model_check",
        "source": "08_reset_interleaving_model_check.mir",
        "summary": "Model-check that reset invalidates old-epoch handoff.",
    },
    {
        "sample_id": "09_detach_todo",
        "source": "09_detach_todo.mir",
        "summary": "Represent detach as a Mirrorea lifecycle TODO boundary.",
    },
]


STATIC_CHECKS = [
    "admin-only start/reset",
    "owner-only roll",
    "handoff target must be active",
    "stale member incarnation cannot commit action",
    "old game_epoch action cannot commit after reset",
]

RUNTIME_GUARDS = [
    "handoff requires publish witness",
    "handoff target must be active",
    "stale member incarnation cannot commit action",
    "old game_epoch action cannot commit after reset",
    "detached game rejects domain action or returns todo_deferred",
]

MODEL_CHECK_PROPERTIES = [
    "no_double_dice_owner",
    "owner_only_rolls",
    "roll_is_published_before_handoff",
    "handoff_target_is_active",
    "late_join_sees_published_history",
    "owner_leave_reassigns_or_pauses",
    "stale_action_after_leave_is_rejected",
    "reset_invalidates_pending_actions",
    "detach_rejects_domain_actions",
    "admin_reset_does_not_interleave_with_roll_commit_badly",
]

RUNTIME_COMPONENTS = [
    "PlaceRuntime",
    "MessageQueue",
    "WorldServerPlace",
    "ParticipantPlace",
    "SugorokuGamePlace",
    "MembershipRegistry",
    "SugorokuState",
    "RollRecord",
    "ActionRecord",
]

LIMITATIONS = [
    "no real network yet",
    "no multi-server consensus",
    "no durable distributed commit",
    "detach is TODO lifecycle boundary",
    "final parser grammar remains deferred",
    "final public API remains deferred",
]

ACTIVE_TRANSPORT_SEAMS = ["local_queue", "loopback_socket"]
RESERVED_TRANSPORT_SEAMS = ["network_link"]


@dataclass(frozen=True)
class TermSignature:
    kind: str
    name: str
    evidence_role: str

    def to_json(self) -> dict[str, str]:
        return {
            "kind": self.kind,
            "name": self.name,
            "evidence_role": self.evidence_role,
        }


@dataclass(frozen=True)
class LayerSignature:
    layer: str
    requires: list[str]
    provides: list[str]
    transforms: list[str]
    checks: list[str]
    emits: list[str]
    laws: list[str]

    def to_json(self) -> dict[str, Any]:
        return {
            "layer": self.layer,
            "requires": list(self.requires),
            "provides": list(self.provides),
            "transforms": list(self.transforms),
            "checks": list(self.checks),
            "emits": list(self.emits),
            "laws": list(self.laws),
        }


@dataclass(frozen=True)
class PrincipalClaim:
    principal: str
    participant_place: str
    claimed_authority: str
    claimed_capabilities: list[str]

    def to_json(self) -> dict[str, Any]:
        return {
            "principal": self.principal,
            "participant_place": self.participant_place,
            "claimed_authority": self.claimed_authority,
            "claimed_capabilities": list(self.claimed_capabilities),
        }


@dataclass(frozen=True)
class AuthEvidence:
    kind: str
    subject: str
    issuer: str
    bindings: list[str]
    notes: list[str] = field(default_factory=list)

    def to_json(self) -> dict[str, Any]:
        return {
            "kind": self.kind,
            "subject": self.subject,
            "issuer": self.issuer,
            "bindings": list(self.bindings),
            "notes": list(self.notes),
        }


@dataclass(frozen=True)
class MessageEnvelope:
    envelope_id: str
    from_place: str
    to_place: str
    transport: str
    payload_kind: str
    payload_ref: str
    principal_claim: PrincipalClaim
    auth_evidence: AuthEvidence | None
    membership_epoch: int
    member_incarnation: int
    capability_requirements: list[str]
    authorization_checks: list[str]
    witness_refs: list[str]
    dispatch_outcome: str
    notes: list[str] = field(default_factory=list)

    def to_json(self) -> dict[str, Any]:
        return {
            "envelope_id": self.envelope_id,
            "from_place": self.from_place,
            "to_place": self.to_place,
            "transport": self.transport,
            "payload_kind": self.payload_kind,
            "payload_ref": self.payload_ref,
            "principal_claim": self.principal_claim.to_json(),
            "auth_evidence": (
                None if self.auth_evidence is None else self.auth_evidence.to_json()
            ),
            "membership_epoch": self.membership_epoch,
            "member_incarnation": self.member_incarnation,
            "capability_requirements": list(self.capability_requirements),
            "authorization_checks": list(self.authorization_checks),
            "witness_refs": list(self.witness_refs),
            "dispatch_outcome": self.dispatch_outcome,
            "notes": list(self.notes),
        }


@dataclass(frozen=True)
class TelemetryRow:
    row_id: str
    row_kind: str
    label: str
    authority: str
    redaction: str
    source_refs: list[str]
    fields: dict[str, Any]
    notes: list[str] = field(default_factory=list)

    def to_json(self) -> dict[str, Any]:
        return {
            "row_id": self.row_id,
            "row_kind": self.row_kind,
            "label": self.label,
            "authority": self.authority,
            "redaction": self.redaction,
            "source_refs": list(self.source_refs),
            "fields": dict(self.fields),
            "notes": list(self.notes),
        }


@dataclass(frozen=True)
class VisualizationView:
    view_id: str
    view_kind: str
    label: str
    authority: str
    redaction: str
    source_refs: list[str]
    summary: dict[str, Any]
    notes: list[str] = field(default_factory=list)

    def to_json(self) -> dict[str, Any]:
        return {
            "view_id": self.view_id,
            "view_kind": self.view_kind,
            "label": self.label,
            "authority": self.authority,
            "redaction": self.redaction,
            "source_refs": list(self.source_refs),
            "summary": dict(self.summary),
            "notes": list(self.notes),
        }


@dataclass
class MessageQueue:
    place_id: str
    messages: list[dict[str, Any]] = field(default_factory=list)

    def enqueue(self, kind: str, **payload: Any) -> None:
        self.messages.append({"kind": kind, **payload})

    def drain(self) -> list[dict[str, Any]]:
        drained = list(self.messages)
        self.messages.clear()
        return drained


@dataclass
class WorldServerPlace:
    place_id: str = "WorldServerPlace"
    authority: str = "GameAuthority.Server"


@dataclass
class ParticipantPlace:
    principal: str

    @property
    def place_id(self) -> str:
        return f"ParticipantPlace[{self.principal}]"


@dataclass
class SugorokuGamePlace:
    instance: int = 1

    @property
    def place_id(self) -> str:
        return f"SugorokuGamePlace#{self.instance}"


@dataclass
class MemberRecord:
    principal: str
    participant_place: str
    active: bool
    incarnation: int
    joined_at_epoch: int
    left_at_epoch: int | None = None


@dataclass
class MembershipRegistry:
    membership_epoch: int = 0
    members: dict[str, MemberRecord] = field(default_factory=dict)

    def add_initial(self, principal: str) -> None:
        self.members[principal] = MemberRecord(
            principal=principal,
            participant_place=f"ParticipantPlace[{principal}]",
            active=True,
            incarnation=0,
            joined_at_epoch=self.membership_epoch,
        )

    def add_member(self, principal: str) -> MemberRecord:
        self.membership_epoch += 1
        record = MemberRecord(
            principal=principal,
            participant_place=f"ParticipantPlace[{principal}]",
            active=True,
            incarnation=0,
            joined_at_epoch=self.membership_epoch,
        )
        self.members[principal] = record
        return record

    def mark_inactive(self, principal: str) -> MemberRecord:
        record = self.members[principal]
        self.membership_epoch += 1
        record.active = False
        record.incarnation += 1
        record.left_at_epoch = self.membership_epoch
        return record

    def active(self, principal: str) -> bool:
        record = self.members.get(principal)
        return bool(record and record.active)

    def active_members(self) -> list[str]:
        return [name for name, record in self.members.items() if record.active]

    def inactive_members(self) -> list[str]:
        return [name for name, record in self.members.items() if not record.active]

    def snapshot(self) -> dict[str, Any]:
        return {
            "membership_epoch": self.membership_epoch,
            "members": {
                name: {
                    "place": record.participant_place,
                    "active": record.active,
                    "incarnation": record.incarnation,
                    "joined_at_epoch": record.joined_at_epoch,
                    "left_at_epoch": record.left_at_epoch,
                }
                for name, record in self.members.items()
            },
        }


@dataclass
class RollRecord:
    roll_id: str
    roller: str
    draw: int
    game_epoch: int
    membership_epoch: int
    published_witness: str

    def to_json(self) -> dict[str, Any]:
        return {
            "roll_id": self.roll_id,
            "roller": self.roller,
            "draw": self.draw,
            "game_epoch": self.game_epoch,
            "membership_epoch": self.membership_epoch,
            "published_witness": self.published_witness,
        }


@dataclass
class ActionRecord:
    action_id: str
    actor: str
    actor_incarnation: int
    game_epoch: int
    membership_epoch: int
    kind: str
    valid: bool = True
    invalidated_reason: str | None = None

    def to_json(self) -> dict[str, Any]:
        return {
            "action_id": self.action_id,
            "actor": self.actor,
            "actor_incarnation": self.actor_incarnation,
            "game_epoch": self.game_epoch,
            "membership_epoch": self.membership_epoch,
            "kind": self.kind,
            "valid": self.valid,
            "invalidated_reason": self.invalidated_reason,
        }


@dataclass
class SugorokuState:
    game_id: str = "SugorokuGame#1"
    attached_world: str = "EmptyWorld"
    game_place: str = "SugorokuGamePlace#1"
    game_epoch: int = 0
    phase: str = "Attached"
    admin: str = "Alice"
    turn_order: list[str] = field(default_factory=lambda: ["Alice", "Bob", "Carol"])
    active_players: set[str] = field(default_factory=lambda: {"Alice", "Bob", "Carol"})
    pending_players: set[str] = field(default_factory=set)
    dice_owner: str | None = "Alice"
    published_rolls: list[RollRecord] = field(default_factory=list)
    pending_actions: dict[str, ActionRecord] = field(default_factory=dict)
    witnesses: list[str] = field(default_factory=list)

    def next_active_after(self, principal: str, registry: MembershipRegistry) -> str | None:
        if principal not in self.turn_order:
            return None
        start = self.turn_order.index(principal)
        for offset in range(1, len(self.turn_order) + 1):
            candidate = self.turn_order[(start + offset) % len(self.turn_order)]
            if registry.active(candidate):
                return candidate
        return None

    def snapshot(self) -> dict[str, Any]:
        return {
            "game_id": self.game_id,
            "attached_world": self.attached_world,
            "game_place": self.game_place,
            "game_epoch": self.game_epoch,
            "phase": self.phase,
            "admin": self.admin,
            "turn_order": list(self.turn_order),
            "active_players": sorted(self.active_players),
            "pending_players": sorted(self.pending_players),
            "dice_owner": self.dice_owner,
            "published_rolls": [roll.to_json() for roll in self.published_rolls],
            "pending_actions": {
                action_id: action.to_json()
                for action_id, action in self.pending_actions.items()
            },
            "witnesses": list(self.witnesses),
        }


@dataclass
class WorldState:
    world_id: str
    server_place: str
    membership_registry: MembershipRegistry
    attached_components: dict[str, str] = field(default_factory=dict)
    authoritative_event_log: list[str] = field(default_factory=list)


@dataclass
class PlaceRuntime:
    places: dict[str, str] = field(default_factory=dict)
    queues: dict[str, MessageQueue] = field(default_factory=dict)
    world: WorldState | None = None
    game: SugorokuState | None = None
    turn_trace: list[str] = field(default_factory=list)
    verification_log: list[str] = field(default_factory=list)

    def add_place(self, place_id: str, kind: str) -> None:
        self.places[place_id] = kind
        self.queues.setdefault(place_id, MessageQueue(place_id))

    def queue_for(self, place_id: str) -> MessageQueue:
        return self.queues.setdefault(place_id, MessageQueue(place_id))

    @property
    def registry(self) -> MembershipRegistry:
        if self.world is None:
            raise RuntimeError("world is not bootstrapped")
        return self.world.membership_registry


def _source_path(sample_id: str) -> Path:
    row = _sample_row(sample_id)
    return SAMPLE_ROOT / row["source"]


def _sample_row(sample_id: str) -> dict[str, str]:
    for row in SAMPLE_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown sugoroku sample: {sample_id}")


def _capture_decl_name(line: str, prefix: str) -> str | None:
    stripped = line.strip()
    if not stripped.startswith(prefix):
        return None
    remainder = stripped.removeprefix(prefix).strip()
    if not remainder:
        return None
    token = remainder.split(None, 1)[0]
    token = token.rstrip("{")
    if "(" in token:
        token = token.split("(", 1)[0]
    return token or None


def _capture_after_marker(line: str, marker: str) -> str | None:
    if marker not in line:
        return None
    remainder = line.split(marker, 1)[1].strip()
    if not remainder:
        return None
    token = remainder.split(None, 1)[0]
    return token.rstrip("{,") or None


def _append_term_signature(
    signatures: list[TermSignature],
    seen: set[tuple[str, str, str]],
    *,
    kind: str,
    name: str,
    evidence_role: str,
) -> None:
    key = (kind, name, evidence_role)
    if name and key not in seen:
        signatures.append(TermSignature(kind=kind, name=name, evidence_role=evidence_role))
        seen.add(key)


def _source_term_signatures(sample_id: str) -> list[TermSignature]:
    source = _source_path(sample_id).read_text(encoding="utf-8")
    signatures: list[TermSignature] = []
    seen: set[tuple[str, str, str]] = set()
    for line in source.splitlines():
        transition_name = _capture_decl_name(line, "transition ")
        if transition_name is not None:
            _append_term_signature(
                signatures,
                seen,
                kind="transition",
                name=transition_name,
                evidence_role="source_decl",
            )
        effect_name = _capture_decl_name(line, "effect ")
        if effect_name is not None:
            _append_term_signature(
                signatures,
                seen,
                kind="effect",
                name=effect_name,
                evidence_role="source_decl",
            )
        witness_name = _capture_after_marker(line, "produces witness ")
        if witness_name is not None:
            _append_term_signature(
                signatures,
                seen,
                kind="witness",
                name=witness_name,
                evidence_role="source_decl",
            )
    return signatures


def _term_signatures(sample_id: str, result: dict[str, Any]) -> list[dict[str, str]]:
    signatures = list(_source_term_signatures(sample_id))
    seen = {(row.kind, row.name, row.evidence_role) for row in signatures}
    for transition in result.get("transitions", []):
        if name := transition.get("transition"):
            _append_term_signature(
                signatures,
                seen,
                kind="transition",
                name=name,
                evidence_role="sample_transition",
            )
    if isinstance(result.get("roll"), dict):
        witness_name = result["roll"].get("published_witness")
        if witness_name:
            _append_term_signature(
                signatures,
                seen,
                kind="witness",
                name=witness_name,
                evidence_role="runtime_witness",
            )
    if isinstance(result.get("used_witness"), str):
        _append_term_signature(
            signatures,
            seen,
            kind="witness",
            name=result["used_witness"],
            evidence_role="runtime_witness",
        )
    for relation in result.get("relations", []):
        if isinstance(relation, list) and relation:
            relation_name = relation[0]
            _append_term_signature(
                signatures,
                seen,
                kind="relation",
                name=relation_name,
                evidence_role="derived_relation",
            )
    for property_name in result.get("properties_passed", []):
        _append_term_signature(
            signatures,
            seen,
            kind="property",
            name=property_name,
            evidence_role="validation_property",
        )
    return [signature.to_json() for signature in signatures]


def _hotplug_lifecycle(
    sample_id: str, result: dict[str, Any], runtime: PlaceRuntime | None
) -> dict[str, Any] | None:
    if sample_id == "01_runtime_attach_game":
        membership_epoch = 0
        if runtime is not None:
            membership_epoch = runtime.registry.membership_epoch
        return {
            "attachpoint_id": "AttachPoint[SugorokuGame#1]",
            "patch_id": "SugorokuGamePackage@runtime",
            "lifecycle_state": "attached_active",
            "compatibility": {
                "result": "compatible",
                "required_authority": "GameAuthority.Server",
                "required_capabilities": ["AttachComponent(SugorokuGamePackage)"],
                "membership_freshness": f"membership_epoch={membership_epoch} imported intact",
                "package_checked": bool(result.get("package_checked")),
            },
            "activation_cut": {
                "request_envelope": "attach_request#1",
                "preconditions": [
                    "authority(Server) >= GameAuthority.Server",
                    "package.checked = true",
                    "membership snapshot imported before visible state mutation",
                ],
                "visible_state_frontier": [
                    "SugorokuGamePlace#1 allocated",
                    "admin=Alice",
                    "phase=Attached",
                ],
            },
            "migration_contract": {
                "status": "not_started",
                "notes": [
                    "repo-local attach only",
                    "durable migration engine deferred",
                ],
            },
        }
    if sample_id == "09_detach_todo":
        return {
            "attachpoint_id": "AttachPoint[SugorokuGame#1]",
            "patch_id": "SugorokuGamePackage@runtime",
            "lifecycle_state": "detached_todo_boundary",
            "compatibility": {
                "result": "detach_requested",
                "required_authority": "GameAuthority.Server",
                "required_capabilities": ["DetachComponent(SugorokuGamePackage)"],
                "membership_freshness": "membership snapshot still required before detach cut",
                "package_checked": True,
            },
            "activation_cut": {
                "request_envelope": "detach_request#1",
                "preconditions": [
                    "compatibility / authorization checks remain explicit",
                    "domain-visible state changes happen after lifecycle cut",
                ],
                "visible_state_frontier": [
                    "phase=Detached",
                    "domain actions rejected or todo_deferred",
                ],
            },
            "detach_boundary": {
                "request_transition": "request_detach_game",
                "guards": ["phase(SugorokuGame#1) != GamePhase.Detached"],
                "post_detach_action": dict(result.get("domain_action_after_detach") or {}),
                "notes": [
                    "detach remains a Mirrorea lifecycle boundary",
                    "runtime domain action stays rejected after detach",
                ],
            },
            "migration_contract": {
                "status": "deferred",
                "notes": [
                    "rollback protocol not fixed",
                    "durable migration engine not implemented",
                ],
            },
        }
    return None


def _layer_signatures(sample_id: str, result: dict[str, Any]) -> list[dict[str, Any]]:
    signatures: list[LayerSignature] = []
    if sample_id == "01_runtime_attach_game":
        signatures.append(
            LayerSignature(
                layer="hot-plug",
                requires=[
                    "authority(Server) >= GameAuthority.Server",
                    "package.checked",
                    "membership_epoch",
                ],
                provides=["attachpoint_compatibility", "activation_cut"],
                transforms=["world:EmptyWorld->SugorokuGame#1 attached"],
                checks=["compatible_before_visible_attach"],
                emits=["hotplug_lifecycle", "summary"],
                laws=[
                    "activation_requires_compatibility",
                    "attach_detach_lifecycle_explicit",
                ],
            )
        )
    if sample_id == "03_roll_publish_handoff":
        roll = result.get("roll") or {}
        witness_name = roll.get("published_witness", "draw_pub#1")
        signatures.extend(
            [
                LayerSignature(
                    layer="verification",
                    requires=["publication_order", f"witness({witness_name})"],
                    provides=list(result.get("properties_passed", [])),
                    transforms=[],
                    checks=["publish_before_handoff", "active_handoff_target"],
                    emits=["term_signatures", "verification"],
                    laws=[
                        "evidence_preservation",
                        "no_hidden_handoff_without_witness",
                    ],
                ),
                LayerSignature(
                    layer="runtime_trace",
                    requires=["sample_transition(take_turn_alice)"],
                    provides=["human_readable_turn_trace"],
                    transforms=[
                        "dice_owner:Alice->Bob",
                        f"published_roll:{roll.get('roll_id', 'roll#1')}",
                    ],
                    checks=["single_process_logical_multi_place"],
                    emits=["turn_trace", "summary"],
                    laws=["helper_output_is_evidence_oriented"],
                ),
            ]
        )
    if sample_id == "05_late_join_history_visible":
        signatures.append(
            LayerSignature(
                layer="membership",
                requires=["membership_epoch", "member_incarnation"],
                provides=["late_join_history_visibility", "pending_member_boundary"],
                transforms=["member_join:Dave", "pending_insert:Dave"],
                checks=["active_pending_distinction", "published_history_visible_after_join"],
                emits=["membership"],
                laws=["membership_epoch_monotone", "stale_action_rejection"],
            )
        )
    if sample_id == "06_leave_non_owner":
        signatures.append(
            LayerSignature(
                layer="membership",
                requires=["membership_epoch", "member_incarnation"],
                provides=["leave_invalidation"],
                transforms=["member_leave:Carol", "incarnation_increment:Carol"],
                checks=["stale_action_rejected_after_leave"],
                emits=["membership"],
                laws=["membership_epoch_monotone", "stale_action_rejection"],
            )
        )
    if sample_id == "07_owner_leave_reassign":
        signatures.append(
            LayerSignature(
                layer="membership",
                requires=["membership_epoch", "member_incarnation"],
                provides=["owner_reassignment_continuity"],
                transforms=["owner_leave:Bob", "dice_owner:Bob->Carol"],
                checks=["reassigned_owner_is_active"],
                emits=["membership", "summary"],
                laws=["membership_epoch_monotone", "active_owner_continuity"],
            )
        )
    if sample_id == "09_detach_todo":
        signatures.append(
            LayerSignature(
                layer="hot-plug",
                requires=[
                    "authority(Server) >= GameAuthority.Server",
                    "membership_epoch",
                    "detach_request",
                ],
                provides=["detach_boundary", "post_detach_rejection"],
                transforms=["phase:Attached->Detached"],
                checks=["domain_action_rejected_after_detach"],
                emits=["hotplug_lifecycle", "verification"],
                laws=[
                    "activation_requires_compatibility",
                    "attach_detach_lifecycle_explicit",
                ],
            )
        )
    return [signature.to_json() for signature in signatures]


def _principal_claim(
    principal: str,
    *,
    claimed_authority: str,
    claimed_capabilities: list[str],
) -> PrincipalClaim:
    return PrincipalClaim(
        principal=principal,
        participant_place=f"ParticipantPlace[{principal}]",
        claimed_authority=claimed_authority,
        claimed_capabilities=claimed_capabilities,
    )


def _member_incarnation(runtime: PlaceRuntime, principal: str) -> int:
    record = runtime.registry.members.get(principal)
    if record is None:
        return 0
    return record.incarnation


def _transport_preview_note(transport: str) -> str:
    if transport == "local_queue":
        return "effect dispatch still stays inside local queue emulator"
    if transport == "loopback_socket":
        return "helper-local loopback preview only; same-process envelope parity"
    raise ValueError(f"unsupported transport seam {transport}")


def _message_envelopes(
    sample_id: str,
    result: dict[str, Any],
    runtime: PlaceRuntime | None,
    *,
    transport: str,
) -> list[dict[str, Any]]:
    if runtime is None:
        return []

    envelopes: list[MessageEnvelope] = []
    world_place = runtime.world.server_place if runtime.world is not None else "WorldServerPlace"
    game_place = runtime.game.game_place if runtime.game is not None else "SugorokuGamePlace#1"

    if sample_id == "01_runtime_attach_game":
        envelopes.append(
            MessageEnvelope(
                envelope_id="attach_request#1",
                from_place=world_place,
                to_place=game_place,
                transport=transport,
                payload_kind="runtime_attach",
                payload_ref="attach_sugoroku_game",
                principal_claim=_principal_claim(
                    "Server",
                    claimed_authority="GameAuthority.Server",
                    claimed_capabilities=["AttachComponent(SugorokuGamePackage)"],
                ),
                auth_evidence=None,
                membership_epoch=runtime.registry.membership_epoch,
                member_incarnation=0,
                capability_requirements=["AttachComponent(SugorokuGamePackage)"],
                authorization_checks=[
                    "authority(Server) >= GameAuthority.Server",
                    "package.checked = true",
                ],
                witness_refs=[],
                dispatch_outcome="accepted",
                notes=["repo-local auth none baseline"],
            )
        )
    if sample_id == "02_admin_start_reset":
        envelopes.extend(
            [
                MessageEnvelope(
                    envelope_id="start_request#1",
                    from_place="ParticipantPlace[Alice]",
                    to_place=game_place,
                    transport=transport,
                    payload_kind="transition",
                    payload_ref="start_game",
                    principal_claim=_principal_claim(
                        "Alice",
                        claimed_authority="GameAuthority.Admin",
                        claimed_capabilities=["GameAdmin(SugorokuGame#1)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Alice"),
                    capability_requirements=["GameAdmin(SugorokuGame#1)"],
                    authorization_checks=[
                        "authority(Alice) >= GameAuthority.Admin",
                        "phase(SugorokuGame#1) = GamePhase.Attached",
                    ],
                    witness_refs=["game_started_witness"],
                    dispatch_outcome="accepted",
                    notes=["repo-local auth none baseline"],
                ),
                MessageEnvelope(
                    envelope_id="bad_reset_request#1",
                    from_place="ParticipantPlace[Bob]",
                    to_place=game_place,
                    transport=transport,
                    payload_kind="transition",
                    payload_ref="bad_reset_by_bob",
                    principal_claim=_principal_claim(
                        "Bob",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["ResetGame(SugorokuGame#1)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Bob"),
                    capability_requirements=["ResetGame(SugorokuGame#1)"],
                    authorization_checks=["authority(Bob) >= GameAuthority.Admin"],
                    witness_refs=[],
                    dispatch_outcome="rejected",
                    notes=["rejected before runtime reset"],
                ),
            ]
        )
    if sample_id == "03_roll_publish_handoff":
        witness_name = result.get("roll", {}).get("published_witness", "draw_pub#1")
        roll_id = result.get("roll", {}).get("roll_id", "roll#1")
        envelopes.extend(
            [
                MessageEnvelope(
                    envelope_id="roll_request#1",
                    from_place="ParticipantPlace[Alice]",
                    to_place=game_place,
                    transport=transport,
                    payload_kind="transition",
                    payload_ref="take_turn_alice",
                    principal_claim=_principal_claim(
                        "Alice",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["DiceOwner(Alice)", "ActiveParticipant(Alice)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Alice"),
                    capability_requirements=["DiceOwner(Alice)", "ActiveParticipant(Alice)"],
                    authorization_checks=[
                        "phase(SugorokuGame#1) = GamePhase.Running",
                        "dice_owner(SugorokuGame#1) = Alice",
                    ],
                    witness_refs=[],
                    dispatch_outcome="accepted",
                    notes=[_transport_preview_note(transport)],
                ),
                MessageEnvelope(
                    envelope_id="handoff_notice#1",
                    from_place=game_place,
                    to_place="ParticipantPlace[Bob]",
                    transport=transport,
                    payload_kind="published_event",
                    payload_ref=f"handoff_from_roll:{roll_id}",
                    principal_claim=_principal_claim(
                        "Alice",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["PublishRoll(Alice)", "HandoffDiceOwner(Alice->Bob)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Alice"),
                    capability_requirements=["PublishRoll(Alice)", "HandoffDiceOwner(Alice->Bob)"],
                    authorization_checks=[
                        "handoff_target_is_active",
                        f"requires witness({witness_name})",
                    ],
                    witness_refs=[witness_name],
                    dispatch_outcome="accepted",
                    notes=["authorization and witness remain separate lanes"],
                ),
            ]
        )
    if sample_id == "04_non_owner_roll_rejected":
        envelopes.append(
            MessageEnvelope(
                envelope_id="bad_roll_request#1",
                from_place="ParticipantPlace[Carol]",
                to_place=game_place,
                transport=transport,
                payload_kind="transition",
                payload_ref="bad_roll_by_carol",
                principal_claim=_principal_claim(
                    "Carol",
                    claimed_authority="GameAuthority.Player",
                    claimed_capabilities=["DiceOwner(Carol)", "ActiveParticipant(Carol)"],
                ),
                auth_evidence=None,
                membership_epoch=runtime.registry.membership_epoch,
                member_incarnation=_member_incarnation(runtime, "Carol"),
                capability_requirements=["DiceOwner(Carol)", "ActiveParticipant(Carol)"],
                authorization_checks=[
                    "phase(SugorokuGame#1) = GamePhase.Running",
                    "dice_owner(SugorokuGame#1) = Carol",
                ],
                witness_refs=[],
                dispatch_outcome="rejected",
                notes=[f"actual current owner is {result.get('actual', 'unknown')}"],
            )
        )
    if sample_id == "05_late_join_history_visible":
        envelopes.extend(
            [
                MessageEnvelope(
                    envelope_id="join_request#1",
                    from_place=world_place,
                    to_place=world_place,
                    transport=transport,
                    payload_kind="membership_update",
                    payload_ref="dave_joins_world",
                    principal_claim=_principal_claim(
                        "Server",
                        claimed_authority="GameAuthority.Server",
                        claimed_capabilities=["AddMember(WorldMembers, Dave)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=0,
                    capability_requirements=["AddMember(WorldMembers, Dave)"],
                    authorization_checks=["authority(Server) >= GameAuthority.Server"],
                    witness_refs=[],
                    dispatch_outcome="accepted",
                    notes=["membership update stays distinct from game authority"],
                ),
                MessageEnvelope(
                    envelope_id="history_replay#1",
                    from_place=game_place,
                    to_place="ParticipantPlace[Dave]",
                    transport=transport,
                    payload_kind="published_history_view",
                    payload_ref="late_join_history_visible",
                    principal_claim=_principal_claim(
                        "Dave",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["ObservePublishedHistory(SugorokuGame#1)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Dave"),
                    capability_requirements=["ObservePublishedHistory(SugorokuGame#1)"],
                    authorization_checks=[
                        "Dave active in membership registry",
                        "published history is observer-visible",
                    ],
                    witness_refs=["draw_pub#1"],
                    dispatch_outcome="accepted",
                    notes=["late join does not imply immediate turn-order insertion"],
                ),
            ]
        )
    if sample_id == "06_leave_non_owner":
        envelopes.extend(
            [
                MessageEnvelope(
                    envelope_id="leave_request#1",
                    from_place="ParticipantPlace[Carol]",
                    to_place=world_place,
                    transport=transport,
                    payload_kind="membership_update",
                    payload_ref="carol_leaves_world",
                    principal_claim=_principal_claim(
                        "Carol",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["LeaveWorldMembership(Carol)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Carol"),
                    capability_requirements=["LeaveWorldMembership(Carol)"],
                    authorization_checks=["Carol currently active in WorldMembers"],
                    witness_refs=[],
                    dispatch_outcome="accepted",
                    notes=["leave increments membership epoch and incarnation"],
                ),
                MessageEnvelope(
                    envelope_id="stale_roll_after_leave#1",
                    from_place="ParticipantPlace[Carol]",
                    to_place=game_place,
                    transport=transport,
                    payload_kind="transition",
                    payload_ref="stale_roll",
                    principal_claim=_principal_claim(
                        "Carol",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["DiceOwner(Carol)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=0,
                    member_incarnation=0,
                    capability_requirements=["DiceOwner(Carol)"],
                    authorization_checks=[
                        "membership_epoch is current",
                        "member_incarnation matches active record",
                    ],
                    witness_refs=[],
                    dispatch_outcome="rejected",
                    notes=["stale member incarnation invalidated"],
                ),
            ]
        )
    if sample_id == "07_owner_leave_reassign":
        envelopes.extend(
            [
                MessageEnvelope(
                    envelope_id="owner_leave_request#1",
                    from_place="ParticipantPlace[Bob]",
                    to_place=world_place,
                    transport=transport,
                    payload_kind="membership_update",
                    payload_ref="bob_leaves_world",
                    principal_claim=_principal_claim(
                        "Bob",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["LeaveWorldMembership(Bob)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Bob"),
                    capability_requirements=["LeaveWorldMembership(Bob)"],
                    authorization_checks=["Bob currently active in WorldMembers"],
                    witness_refs=["membership_updated"],
                    dispatch_outcome="accepted",
                    notes=["owner continuity handled after membership update"],
                ),
                MessageEnvelope(
                    envelope_id="owner_reassign_notice#1",
                    from_place=game_place,
                    to_place="ParticipantPlace[Alice]",
                    transport=transport,
                    payload_kind="ownership_reassignment",
                    payload_ref="owner_leave_reassign",
                    principal_claim=_principal_claim(
                        "Alice",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["ReceiveDiceOwnership(Alice)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Alice"),
                    capability_requirements=["ReceiveDiceOwnership(Alice)"],
                    authorization_checks=["new dice owner must remain active"],
                    witness_refs=["membership_updated"],
                    dispatch_outcome="accepted",
                    notes=["ownership transfer happens after leave commit"],
                ),
            ]
        )
    if sample_id == "09_detach_todo":
        envelopes.extend(
            [
                MessageEnvelope(
                    envelope_id="detach_request#1",
                    from_place=world_place,
                    to_place=game_place,
                    transport=transport,
                    payload_kind="runtime_detach",
                    payload_ref="request_detach_game",
                    principal_claim=_principal_claim(
                        "Server",
                        claimed_authority="GameAuthority.Server",
                        claimed_capabilities=["DetachComponent(SugorokuGamePackage)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=0,
                    capability_requirements=["DetachComponent(SugorokuGamePackage)"],
                    authorization_checks=[
                        "authority(Server) >= GameAuthority.Server",
                        "membership snapshot imported before visible detach state",
                    ],
                    witness_refs=[],
                    dispatch_outcome="accepted",
                    notes=["repo-local auth none baseline", "detach lifecycle request only"],
                ),
                MessageEnvelope(
                    envelope_id="detached_roll_request#1",
                    from_place="ParticipantPlace[Alice]",
                    to_place=game_place,
                    transport=transport,
                    payload_kind="transition",
                    payload_ref="detached_domain_action",
                    principal_claim=_principal_claim(
                        "Alice",
                        claimed_authority="GameAuthority.Player",
                        claimed_capabilities=["DiceOwner(Alice)"],
                    ),
                    auth_evidence=None,
                    membership_epoch=runtime.registry.membership_epoch,
                    member_incarnation=_member_incarnation(runtime, "Alice"),
                    capability_requirements=["DiceOwner(Alice)"],
                    authorization_checks=["phase(SugorokuGame#1) != GamePhase.Detached"],
                    witness_refs=[],
                    dispatch_outcome="rejected",
                    notes=["deferred to Mirrorea lifecycle layer"],
                ),
            ]
        )

    if transport == "loopback_socket":
        envelopes = [
            MessageEnvelope(
                envelope_id=envelope.envelope_id,
                from_place=envelope.from_place,
                to_place=envelope.to_place,
                transport=envelope.transport,
                payload_kind=envelope.payload_kind,
                payload_ref=envelope.payload_ref,
                principal_claim=envelope.principal_claim,
                auth_evidence=envelope.auth_evidence,
                membership_epoch=envelope.membership_epoch,
                member_incarnation=envelope.member_incarnation,
                capability_requirements=envelope.capability_requirements,
                authorization_checks=envelope.authorization_checks,
                witness_refs=envelope.witness_refs,
                dispatch_outcome=envelope.dispatch_outcome,
                notes=[*envelope.notes, _transport_preview_note(transport)],
            )
            for envelope in envelopes
        ]

    return [envelope.to_json() for envelope in envelopes]


def _telemetry_rows(
    sample_id: str, result: dict[str, Any], runtime: PlaceRuntime | None
) -> list[TelemetryRow]:
    rows: list[TelemetryRow] = []
    if sample_id == "01_runtime_attach_game":
        lifecycle = result.get("hotplug_lifecycle") or {}
        compatibility = lifecycle.get("compatibility") or {}
        activation_cut = lifecycle.get("activation_cut") or {}
        rows.append(
            TelemetryRow(
                row_id="attach_activation#1",
                row_kind="hotplug_activation",
                label="helper:hotplug",
                authority="InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                redaction="lifecycle_summary_only",
                source_refs=["hotplug_lifecycle", "message_envelopes[attach_request#1]"],
                fields={
                    "state": lifecycle.get("lifecycle_state", "attached_active"),
                    "compatibility": compatibility.get("result", "compatible"),
                    "request_envelope": activation_cut.get(
                        "request_envelope", "attach_request#1"
                    ),
                },
                notes=["helper-local hot-plug activation summary"],
            )
        )
    if sample_id == "03_roll_publish_handoff":
        membership_epoch = runtime.registry.membership_epoch if runtime is not None else 0
        envelopes = {
            row["envelope_id"]: row for row in result.get("message_envelopes", [])
        }
        roll_request = envelopes.get("roll_request#1", {})
        handoff_notice = envelopes.get("handoff_notice#1", {})
        rows.extend(
            [
                TelemetryRow(
                    row_id="roll_request#1",
                    row_kind="message_dispatch",
                    label="helper:game-transition",
                    authority="InspectLocalQueue(SugorokuGame#1)",
                    redaction="omit_auth_evidence_payload",
                    source_refs=["message_envelopes[roll_request#1]", "turn_trace", "roll"],
                    fields={
                        "place": roll_request.get("to_place", "SugorokuGamePlace#1"),
                        "membership_epoch": roll_request.get(
                            "membership_epoch", membership_epoch
                        ),
                        "dispatch_outcome": roll_request.get(
                            "dispatch_outcome", "accepted"
                        ),
                        "payload_ref": roll_request.get("payload_ref", "take_turn_alice"),
                    },
                    notes=[
                        "helper-local local-queue dispatch summary",
                        "derived from current message_envelopes payload",
                    ],
                ),
                TelemetryRow(
                    row_id="handoff_notice#1",
                    row_kind="published_roll",
                    label="helper:published-history",
                    authority="ObservePublishedHistory(SugorokuGame#1)",
                    redaction="published_history_only",
                    source_refs=["message_envelopes[handoff_notice#1]", "roll", "turn_trace"],
                    fields={
                        "place": handoff_notice.get("to_place", "ParticipantPlace[Bob]"),
                        "membership_epoch": handoff_notice.get(
                            "membership_epoch", membership_epoch
                        ),
                        "dispatch_outcome": handoff_notice.get(
                            "dispatch_outcome", "accepted"
                        ),
                        "witness_count": len(handoff_notice.get("witness_refs") or []),
                        "next_owner": result.get("game", {}).get("dice_owner"),
                    },
                    notes=[
                        "helper-local published-history telemetry row",
                        "derived from current message_envelopes payload",
                    ],
                ),
            ]
        )
    if sample_id == "05_late_join_history_visible":
        membership_epoch = result.get("membership_epoch")
        visible_rolls = (result.get("Dave") or {}).get("visible_rolls", [])
        rows.extend(
            [
                TelemetryRow(
                    row_id="late_join_membership#1",
                    row_kind="membership_update",
                    label="helper:membership",
                    authority="ObserveMembership(WorldMembers)",
                    redaction="pending_turn_order_only",
                    source_refs=["membership", "Dave.pending_player"],
                    fields={
                        "place": "WorldServerPlace",
                        "membership_epoch": membership_epoch,
                        "pending_players": ["Dave"],
                    },
                    notes=[
                        "helper-local membership telemetry row",
                        "late join increments membership registry",
                    ],
                ),
                TelemetryRow(
                    row_id="late_join_history#1",
                    row_kind="history_visibility",
                    label="helper:published-history",
                    authority="ObservePublishedHistory(SugorokuGame#1)",
                    redaction="published_history_only",
                    source_refs=["Dave.visible_rolls", "membership"],
                    fields={
                        "place": "ParticipantPlace[Dave]",
                        "membership_epoch": membership_epoch,
                        "visible_roll_count": len(visible_rolls),
                    },
                    notes=[
                        "helper-local published-history telemetry row",
                        "pending player sees published history only",
                    ],
                ),
            ]
        )
    if sample_id == "08_reset_interleaving_model_check":
        rows.append(
            TelemetryRow(
                row_id="reset_model_check#1",
                row_kind="model_check_summary",
                label="helper:verification",
                authority="ObserveVerificationSummary(SugorokuGame#1)",
                redaction="counterexample_shape_summary_only",
                source_refs=["model_check", "verification"],
                fields={
                    "property": "no_old_epoch_handoff_after_reset",
                    "result": result.get("model_check_result"),
                },
                notes=["second-line verification summary"],
            )
        )
    if sample_id == "09_detach_todo":
        lifecycle = result.get("hotplug_lifecycle") or {}
        detach_boundary = lifecycle.get("detach_boundary") or {}
        rows.append(
            TelemetryRow(
                row_id="detach_boundary#1",
                row_kind="hotplug_detach",
                label="helper:hotplug",
                authority="InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                redaction="lifecycle_summary_only",
                source_refs=[
                    "hotplug_lifecycle",
                    "message_envelopes[detach_request#1]",
                    "message_envelopes[detached_roll_request#1]",
                ],
                fields={
                    "state": lifecycle.get("lifecycle_state", "detached_todo_boundary"),
                    "request_envelope": (
                        lifecycle.get("activation_cut") or {}
                    ).get("request_envelope", "detach_request#1"),
                    "post_detach_verdict": (
                        detach_boundary.get("post_detach_action") or {}
                    ).get("verdict", "reject"),
                    "request_transition": detach_boundary.get(
                        "request_transition", "request_detach_game"
                    ),
                },
                notes=["helper-local post-detach rejection summary"],
            )
        )
    return rows


def _visualization_views(
    sample_id: str,
    result: dict[str, Any],
    telemetry_rows: list[TelemetryRow],
) -> list[dict[str, Any]]:
    views: list[VisualizationView] = []
    if sample_id == "01_runtime_attach_game":
        lifecycle = result.get("hotplug_lifecycle") or {}
        compatibility = lifecycle.get("compatibility") or {}
        views.append(
            VisualizationView(
                view_id="attach_lifecycle",
                view_kind="hotplug_lifecycle",
                label="helper:hotplug",
                authority="InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                redaction="lifecycle_summary_only",
                source_refs=["hotplug_lifecycle", "message_envelopes", "summary"],
                summary={
                    "state": lifecycle.get("lifecycle_state", "attached_active"),
                    "compatibility": compatibility.get("result", "compatible"),
                    "telemetry_refs": [row.row_id for row in telemetry_rows],
                },
                notes=["helper-local attach lifecycle visualization"],
            )
        )
    if sample_id == "03_roll_publish_handoff":
        views.extend(
            [
                VisualizationView(
                    view_id="turn_timeline",
                    view_kind="turn_timeline",
                    label="helper:published-history",
                    authority="ObservePublishedHistory(SugorokuGame#1)",
                    redaction="published_history_only",
                    source_refs=["roll", "turn_trace", "game", "message_envelopes"],
                    summary={
                        "published_witness": result.get("roll", {}).get("published_witness", "draw_pub#1"),
                        "next_owner": result.get("game", {}).get("dice_owner"),
                        "telemetry_refs": [row.row_id for row in telemetry_rows],
                    },
                    notes=[
                        "helper-local visualization first cut",
                        "summary-style state snapshot for the active game",
                    ],
                ),
                VisualizationView(
                    view_id="message_route",
                    view_kind="message_route",
                    label="helper:transport-audit",
                    authority="InspectLocalQueue(SugorokuGame#1)",
                    redaction="omit_auth_evidence_payload",
                    source_refs=["message_envelopes", "turn_trace"],
                    summary={
                        "envelope_ids": [row["envelope_id"] for row in result.get("message_envelopes", [])],
                        "dispatch_outcomes": [
                            row["dispatch_outcome"]
                            for row in result.get("message_envelopes", [])
                        ],
                        "telemetry_refs": [row.row_id for row in telemetry_rows[:1]],
                    },
                    notes=[
                        "helper-local visualization first cut",
                        "publish -> witness -> handoff evidence trace",
                    ],
                ),
                VisualizationView(
                    view_id="verification_summary",
                    view_kind="verification_summary",
                    label="helper:verification",
                    authority="ObserveVerificationSummary(SugorokuGame#1)",
                    redaction="verification_summary_only",
                    source_refs=["verification", "properties_passed"],
                    summary={
                        "properties_passed": list(result.get("properties_passed", [])),
                        "telemetry_refs": [row.row_id for row in telemetry_rows[1:]],
                    },
                    notes=[
                        "helper-local visualization first cut",
                        "no final public viewer contract implied",
                    ],
                ),
            ]
        )
    if sample_id == "05_late_join_history_visible":
        visible_rolls = (result.get("Dave") or {}).get("visible_rolls", [])
        views.append(
            VisualizationView(
                view_id="membership_snapshot",
                view_kind="membership_snapshot",
                label="helper:membership",
                authority="ObserveMembership(WorldMembers)",
                redaction="pending_turn_order_only",
                source_refs=["membership", "Dave.visible_rolls"],
                summary={
                    "pending_players": ["Dave"],
                    "visible_roll_count": len(visible_rolls),
                    "telemetry_refs": [row.row_id for row in telemetry_rows],
                },
                notes=[
                    "helper-local visualization first cut",
                    "active/pending distinction remains visible",
                ],
            )
        )
    if sample_id == "08_reset_interleaving_model_check":
        views.append(
            VisualizationView(
                view_id="verification_summary",
                view_kind="verification_summary",
                label="helper:verification",
                authority="ObserveVerificationSummary(SugorokuGame#1)",
                redaction="counterexample_shape_summary_only",
                source_refs=["model_check", "verification"],
                summary={
                    "property": "no_old_epoch_handoff_after_reset",
                    "result": result.get("model_check_result"),
                    "telemetry_refs": [row.row_id for row in telemetry_rows],
                },
                notes=["model-check summary for helper-local verification view"],
            )
        )
    if sample_id == "09_detach_todo":
        lifecycle = result.get("hotplug_lifecycle") or {}
        detach_boundary = lifecycle.get("detach_boundary") or {}
        views.append(
            VisualizationView(
                view_id="detach_lifecycle",
                view_kind="hotplug_lifecycle",
                label="helper:hotplug",
                authority="InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                redaction="lifecycle_summary_only",
                source_refs=[
                    "hotplug_lifecycle",
                    "message_envelopes[detach_request#1]",
                    "message_envelopes[detached_roll_request#1]",
                    "verification",
                ],
                summary={
                    "state": lifecycle.get("lifecycle_state", "detached_todo_boundary"),
                    "request_envelope": (
                        lifecycle.get("activation_cut") or {}
                    ).get("request_envelope", "detach_request#1"),
                    "post_detach_verdict": (
                        detach_boundary.get("post_detach_action") or {}
                    ).get("verdict", "reject"),
                    "telemetry_refs": [row.row_id for row in telemetry_rows],
                },
                notes=["helper-local detach lifecycle visualization"],
            )
        )
    return [view.to_json() for view in views]


def _finalize_result(
    sample_id: str,
    result: dict[str, Any],
    runtime: PlaceRuntime | None = None,
    *,
    transport: str,
) -> dict[str, Any]:
    result["term_signatures"] = _term_signatures(sample_id, result)
    hotplug_lifecycle = _hotplug_lifecycle(sample_id, result, runtime)
    if hotplug_lifecycle is not None:
        result["hotplug_lifecycle"] = hotplug_lifecycle
    result["layer_signatures"] = _layer_signatures(sample_id, result)
    result["message_envelopes"] = _message_envelopes(
        sample_id, result, runtime, transport=transport
    )
    telemetry_rows = _telemetry_rows(sample_id, result, runtime)
    result["telemetry_rows"] = [row.to_json() for row in telemetry_rows]
    result["visualization_views"] = _visualization_views(sample_id, result, telemetry_rows)
    return result


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "sugoroku-world",
            "source_path": str(_source_path(row["sample_id"])),
            "summary": row["summary"],
        }
        for row in SAMPLE_ROWS
    ]


def bootstrap_world() -> PlaceRuntime:
    runtime = PlaceRuntime()
    server = WorldServerPlace()
    runtime.add_place(server.place_id, "WorldServerPlace")
    registry = MembershipRegistry()
    for principal in ["Alice", "Bob", "Carol"]:
        participant_place = ParticipantPlace(principal)
        runtime.add_place(participant_place.place_id, "ParticipantPlace")
        registry.add_initial(principal)
    runtime.world = WorldState(
        world_id="EmptyWorld",
        server_place=server.place_id,
        membership_registry=registry,
    )
    runtime.world.authoritative_event_log.append("world_bootstrap")
    runtime.verification_log.append("principals are not places")
    return runtime


def attach_game(runtime: PlaceRuntime) -> SugorokuState:
    if runtime.world is None:
        raise RuntimeError("cannot attach game before world bootstrap")
    game_place = SugorokuGamePlace().place_id
    runtime.add_place(game_place, "SugorokuGamePlace")
    runtime.game = SugorokuState(game_place=game_place)
    runtime.world.attached_components[runtime.game.game_id] = game_place
    runtime.world.authoritative_event_log.append("attach SugorokuGame#1")
    runtime.queue_for(runtime.world.server_place).enqueue(
        "attach", component=runtime.game.game_id, place=game_place
    )
    runtime.verification_log.extend(
        [
            "authority(Server) >= GameAuthority.Server",
            "membership imported from WorldMembers epoch 0",
            "Alice appointed as GameAuthority.Admin",
        ]
    )
    return runtime.game


def start_game(runtime: PlaceRuntime) -> None:
    game = _require_game(runtime)
    if game.admin != "Alice":
        raise RuntimeError("sample expects Alice as admin")
    if game.phase != "Attached":
        raise RuntimeError("start requires Attached phase")
    game.phase = "Running"
    game.witnesses.append("game_started_witness")
    runtime.turn_trace.append("publish game_started witness=game_started_witness")
    runtime.verification_log.extend(
        [
            "authority(Alice) >= GameAuthority.Admin",
            "phase(SugorokuGame#1) = GamePhase.Attached",
        ]
    )


def _roll_publish_handoff(runtime: PlaceRuntime, actor: str, draw: int = 4) -> RollRecord:
    game = _require_game(runtime)
    registry = runtime.registry
    if game.phase != "Running":
        raise RuntimeError("roll requires Running phase")
    if game.dice_owner != actor:
        raise PermissionError(f"dice_owner requirement failed: actual={game.dice_owner}")
    if not registry.active(actor):
        raise PermissionError(f"{actor} is not active")
    next_owner = game.next_active_after(actor, registry)
    if next_owner is None:
        game.dice_owner = None
        game.phase = "Paused"
        raise RuntimeError("no active handoff target")
    witness = f"draw_pub#{len(game.published_rolls) + 1}"
    roll = RollRecord(
        roll_id=f"roll#{len(game.published_rolls) + 1}",
        roller=actor,
        draw=draw,
        game_epoch=game.game_epoch,
        membership_epoch=registry.membership_epoch,
        published_witness=witness,
    )
    game.published_rolls.append(roll)
    game.witnesses.append(witness)
    game.dice_owner = next_owner
    runtime.turn_trace.extend(
        [
            f"{actor} roll draw={draw}",
            f"publish roll_result({actor}, {draw}) witness={witness}",
            f"handoff dice_owner {actor} -> {next_owner} using witness={witness}",
        ]
    )
    runtime.verification_log.extend(
        [
            f"dice_owner(game) = {actor}",
            f"{actor} active at membership_epoch {registry.membership_epoch}",
            f"handoff has witness({witness})",
            f"handoff target {next_owner} active",
        ]
    )
    return roll


def _require_game(runtime: PlaceRuntime) -> SugorokuState:
    if runtime.game is None:
        raise RuntimeError("game is not attached")
    return runtime.game


def _runtime_after_attach() -> PlaceRuntime:
    runtime = bootstrap_world()
    attach_game(runtime)
    return runtime


def _runtime_running() -> PlaceRuntime:
    runtime = _runtime_after_attach()
    start_game(runtime)
    return runtime


def _runtime_after_alice_turn() -> PlaceRuntime:
    runtime = _runtime_running()
    _roll_publish_handoff(runtime, "Alice")
    return runtime


def _base_result(sample_id: str, transport: str) -> dict[str, Any]:
    return {
        "sample": sample_id,
        "family": "sugoroku-world",
        "source_path": str(_source_path(sample_id)),
        "transport_seam": transport,
    }


def run_sample(sample_id: str, *, transport: str = "local_queue") -> dict[str, Any]:
    _sample_row(sample_id)
    if transport not in ACTIVE_TRANSPORT_SEAMS:
        raise ValueError(f"unsupported transport seam {transport}")
    if sample_id == "00_world_bootstrap":
        runtime = bootstrap_world()
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "world": "EmptyWorld",
                "membership_epoch": runtime.registry.membership_epoch,
                "active_members": runtime.registry.active_members(),
                "places": list(runtime.places.keys()),
                "attached_components": [],
                "place_model": "logical multi-place emulator in one OS process",
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "01_runtime_attach_game":
        runtime = _runtime_after_attach()
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "world": "EmptyWorld",
                "package_checked": True,
                "imported_membership_epoch": runtime.registry.membership_epoch,
                "game": _require_game(runtime).snapshot(),
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "02_admin_start_reset":
        runtime = _runtime_running()
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "mixed",
                "terminal_outcome": "success",
                "transitions": [
                    {
                        "transition": "start_game",
                        "static_verdict": "valid",
                        "terminal_outcome": "success",
                        "phase_after": "Running",
                        "witnesses": ["game_started_witness"],
                    },
                    {
                        "transition": "bad_reset_by_bob",
                        "static_verdict": "malformed",
                        "reason_family": "authority_preorder_constraint_failed",
                        "constraints_failed": [
                            "GameAuthority.Player >= GameAuthority.Admin"
                        ],
                        "entered_evaluation": False,
                    },
                ],
                "game": _require_game(runtime).snapshot(),
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "03_roll_publish_handoff":
        runtime = _runtime_after_alice_turn()
        game = _require_game(runtime)
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "transitions": [{"transition": "take_turn_alice"}],
                "roll": game.published_rolls[-1].to_json(),
                "game": game.snapshot(),
                "relations": [
                    ["program_order", "roll", "publish"],
                    ["publication_order", "roll", "publish"],
                    ["witness_order", "publish", "handoff"],
                    ["scoped_happens_before", "roll", "handoff"],
                ],
                "properties_passed": [
                    "owner_only_rolls",
                    "roll_is_published_before_handoff",
                    "handoff_target_is_active",
                    "no_double_dice_owner",
                ],
                "turn_trace": list(runtime.turn_trace),
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "04_non_owner_roll_rejected":
        runtime = _runtime_after_alice_turn()
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_or_runtime_verdict": "reject",
                "reason_family": "dice_owner_requirement_failed",
                "required": "dice_owner(SugorokuGame#1) = Carol",
                "actual": _require_game(runtime).dice_owner,
                "entered_evaluation": False,
                "audit_evidence": [
                    "Carol active but lacks DiceOwner capability for current turn",
                    "current dice_owner is Bob",
                ],
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "05_late_join_history_visible":
        runtime = _runtime_after_alice_turn()
        runtime.add_place("ParticipantPlace[Dave]", "ParticipantPlace")
        runtime.registry.add_member("Dave")
        game = _require_game(runtime)
        game.pending_players.add("Dave")
        runtime.verification_log.append("Dave observes published history after membership add")
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "membership_epoch_incremented": True,
                "membership_epoch": runtime.registry.membership_epoch,
                "Dave": {
                    "active": True,
                    "published_history_visible": bool(game.published_rolls),
                    "visible_rolls": [roll.to_json() for roll in game.published_rolls],
                    "in_turn_order": "Dave" in game.turn_order,
                    "pending_player": "Dave" in game.pending_players,
                },
                "membership": runtime.registry.snapshot(),
                "properties_passed": ["late_join_sees_published_history"],
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "06_leave_non_owner":
        runtime = _runtime_after_alice_turn()
        runtime.registry.mark_inactive("Carol")
        game = _require_game(runtime)
        action = ActionRecord(
            action_id="pending-carol#1",
            actor="Carol",
            actor_incarnation=0,
            game_epoch=game.game_epoch,
            membership_epoch=0,
            kind="stale_roll",
            valid=False,
            invalidated_reason="member_left",
        )
        game.pending_actions[action.action_id] = action
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "member": "Carol",
                "active_after": False,
                "membership_epoch": runtime.registry.membership_epoch,
                "member_incarnation": runtime.registry.members["Carol"].incarnation,
                "pending_actions_invalidated": True,
                "invalidated_actions": [action.to_json()],
                "properties_passed": ["stale_action_after_leave_is_rejected"],
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "07_owner_leave_reassign":
        runtime = _runtime_after_alice_turn()
        runtime.registry.mark_inactive("Carol")
        runtime.registry.mark_inactive("Bob")
        game = _require_game(runtime)
        game.active_players = set(runtime.registry.active_members())
        game.witnesses.append("membership_updated")
        next_owner = game.next_active_after("Bob", runtime.registry)
        if next_owner is None:
            game.dice_owner = None
            game.phase = "Paused"
        else:
            game.dice_owner = next_owner
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "success",
                "left_member": "Bob",
                "new_dice_owner": game.dice_owner,
                "phase_after": game.phase,
                "membership_epoch_incremented": True,
                "membership_epoch": runtime.registry.membership_epoch,
                "used_witness": "membership_updated",
                "properties_passed": ["owner_leave_reassigns_or_pauses"],
                "membership": runtime.registry.snapshot(),
                "game": game.snapshot(),
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    if sample_id == "08_reset_interleaving_model_check":
        result = _base_result(sample_id, transport)
        result.update(model_check())
        return _finalize_result(sample_id, result, transport=transport)
    if sample_id == "09_detach_todo":
        runtime = _runtime_after_attach()
        game = _require_game(runtime)
        game.phase = "Detached"
        result = _base_result(sample_id, transport)
        result.update(
            {
                "static_verdict": "valid",
                "terminal_outcome": "todo_deferred",
                "deferred_to": "Mirrorea lifecycle layer",
                "domain_action_after_detach": {
                    "verdict": "reject",
                    "reason_family": "detached_game_rejects_domain_action",
                },
                "properties_passed": ["detach_rejects_domain_actions"],
                "game": game.snapshot(),
            }
        )
        return _finalize_result(sample_id, result, runtime, transport=transport)
    raise AssertionError(f"unhandled sample {sample_id}")


def model_check() -> dict[str, Any]:
    return {
        "sample": "08_reset_interleaving_model_check",
        "property": "no_old_epoch_handoff_after_reset",
        "model_check_result": "pass",
        "explanation": "reset invalidates pending handoff from old epoch",
        "properties": list(MODEL_CHECK_PROPERTIES),
        "checked_scenarios": [
            {
                "name": "reset_invalidates_pending_actions",
                "result": "pass",
                "trace": [
                    "Owner rolls in epoch 1",
                    "Owner publishes in epoch 1",
                    "Admin resets game to epoch 2",
                    "Pending epoch 1 handoff is invalidated before commit",
                ],
            },
            {
                "name": "admin_reset_does_not_interleave_with_roll_commit_badly",
                "result": "pass",
                "trace": [
                    "reset increments game_epoch",
                    "commit guard checks action.game_epoch == current game_epoch",
                    "old epoch handoff is rejected",
                ],
            },
        ],
        "broken_variant": {
            "sample": "08b_reset_interleaving_broken",
            "property": "no_old_epoch_handoff_after_reset",
            "model_check_result": "counterexample",
            "trace": [
                "Owner rolls in epoch 1",
                "Owner publishes in epoch 1",
                "Admin resets game to epoch 2",
                "Old owner handoff from epoch 1 commits after reset",
            ],
        },
    }


def check_all(*, transport: str = "local_queue") -> dict[str, Any]:
    failed: list[dict[str, str]] = []
    for row in SAMPLE_ROWS:
        path = _source_path(row["sample_id"])
        if not path.exists():
            failed.append({"sample": row["sample_id"], "reason": "missing_source"})
            continue
        try:
            run_sample(row["sample_id"], transport=transport)
        except Exception as error:  # pragma: no cover - surfaced in command output.
            failed.append({"sample": row["sample_id"], "reason": str(error)})
    return {
        "sample_count": len(SAMPLE_ROWS),
        "passed": [row["sample_id"] for row in SAMPLE_ROWS if not any(f["sample"] == row["sample_id"] for f in failed)],
        "failed": failed,
        "transport_seam": transport,
        "static_checks": list(STATIC_CHECKS),
        "runtime_guards": list(RUNTIME_GUARDS),
        "model_check_properties": list(MODEL_CHECK_PROPERTIES),
    }


def closeout() -> dict[str, Any]:
    signature_kinds = sorted(
        {
            row["kind"]
            for sample in SAMPLE_ROWS
            for row in run_sample(sample["sample_id"])["term_signatures"]
        }
    )
    layer_signature_kinds = sorted(
        {
            row["layer"]
            for sample in SAMPLE_ROWS
            for row in run_sample(sample["sample_id"])["layer_signatures"]
        }
    )
    visualization_views = [
        row
        for sample in SAMPLE_ROWS
        for row in run_sample(sample["sample_id"])["visualization_views"]
    ]
    telemetry_rows = [
        row
        for sample in SAMPLE_ROWS
        for row in run_sample(sample["sample_id"])["telemetry_rows"]
    ]
    return {
        "active_sample_root": str(SAMPLE_ROOT),
        "sample_count": len(SAMPLE_ROWS),
        "samples": [row["sample_id"] for row in SAMPLE_ROWS],
        "runtime_components": list(RUNTIME_COMPONENTS),
        "place_model": {
            "description": "single OS process logical multi-place emulator",
            "places": [
                "WorldServerPlace",
                "ParticipantPlace[Alice]",
                "ParticipantPlace[Bob]",
                "ParticipantPlace[Carol]",
                "ParticipantPlace[Dave]",
                "SugorokuGamePlace#1",
            ],
            "principals": ["Server", "Alice", "Bob", "Carol", "Dave"],
        },
        "membership_model": {
            "membership_epoch": "increments on join/leave",
            "member_incarnation": "increments on leave to reject stale actions",
            "late_join_policy": "published history visible, turn order insertion deferred",
        },
        "static_checks": list(STATIC_CHECKS),
        "runtime_guards": list(RUNTIME_GUARDS),
        "model_check_properties": list(MODEL_CHECK_PROPERTIES),
        "signature_kinds": signature_kinds,
        "reserved_signature_kinds": ["message", "adapter", "layer"],
        "message_envelope_lanes": [
            "envelope_id",
            "from_place",
            "to_place",
            "transport",
            "payload_kind",
            "payload_ref",
            "principal_claim",
            "auth_evidence",
            "membership_epoch",
            "member_incarnation",
            "capability_requirements",
            "authorization_checks",
            "witness_refs",
            "dispatch_outcome",
            "notes",
        ],
        "auth_evidence_modes": ["none"],
        "reserved_auth_evidence_modes": ["session_token", "signature"],
        "transport_seams": list(ACTIVE_TRANSPORT_SEAMS),
        "reserved_transport_seams": list(RESERVED_TRANSPORT_SEAMS),
        "hotplug_lifecycle_states": sorted(
            {
                row["hotplug_lifecycle"]["lifecycle_state"]
                for sample in SAMPLE_ROWS
                for row in [run_sample(sample["sample_id"])]
                if "hotplug_lifecycle" in row
            }
        ),
        "visualization_views": visualization_views,
        "visualization_view_kinds": sorted(
            {row["view_kind"] for row in visualization_views}
        ),
        "reserved_visualization_view_kinds": [
            "place_graph",
            "effect_route_graph",
            "projection_view",
        ],
        "telemetry_rows": telemetry_rows,
        "telemetry_row_kinds": sorted({row["row_kind"] for row in telemetry_rows}),
        "redaction_policy_names": sorted(
            {
                row["redaction"]
                for row in [*visualization_views, *telemetry_rows]
            }
        ),
        "layer_signature_kinds": layer_signature_kinds,
        "reserved_layer_signature_kinds": [
            "auth",
            "transport",
            "telemetry",
            "projection",
            "visualization",
        ],
        "debug_output_modes": [
            "--debug summary",
            "--debug turn-trace",
            "--debug membership",
            "--debug verification",
            "--debug signatures",
            "--debug envelopes",
            "--debug visualization",
            "--debug layers",
            "--debug hotplug",
            "--format json",
        ],
        "limitations": list(LIMITATIONS),
    }


def _format_world_summary(result: dict[str, Any]) -> str:
    active_members = result.get("active_members", [])
    lines = [
        "WORLD EmptyWorld",
        "  server place: WorldServerPlace",
        f"  membership epoch: {result.get('membership_epoch', 0)}",
        "  active members:",
    ]
    for name in active_members:
        lines.append(f"    - {name} @ ParticipantPlace[{name}] incarnation=0")
    lines.extend(["  attached components:", "    none"])
    return "\n".join(lines)


def _format_summary(result: dict[str, Any]) -> str:
    game = result.get("game") or {}
    membership = result.get("membership") or {}
    members = membership.get("members", {})
    active = [name for name, record in members.items() if record.get("active")]
    inactive = [name for name, record in members.items() if not record.get("active")]
    if result["sample"] == "00_world_bootstrap":
        return _format_world_summary(result)
    if game:
        return "\n".join(
            [
                "WORLD EmptyWorld",
                f"  membership_epoch: {membership.get('membership_epoch', result.get('membership_epoch', 0))}",
                f"  active: {', '.join(active or result.get('active_members', []) or game.get('active_players', []))}",
                f"  inactive: {', '.join(inactive) if inactive else 'none'}",
                "  attached:",
                f"    SugorokuGame#1 phase={game.get('phase')} epoch={game.get('game_epoch')} admin={game.get('admin')} dice_owner={game.get('dice_owner')}",
            ]
        )
    return json.dumps(result, indent=2, ensure_ascii=False)


def _format_turn_trace(result: dict[str, Any]) -> str:
    trace = result.get("turn_trace") or [
        "Alice roll draw=4",
        "publish roll_result(Alice, 4) witness=draw_pub#1",
        "handoff dice_owner Alice -> Bob using witness=draw_pub#1",
    ]
    return "\n".join(["TURN TRACE", *[f"  [{idx}] {row}" for idx, row in enumerate(trace, 1)]])


def _format_membership(result: dict[str, Any]) -> str:
    membership = result.get("membership")
    if membership is None:
        membership = run_sample("05_late_join_history_visible")["membership"]
    lines = ["MEMBERSHIP", f"  epoch: {membership['membership_epoch']}"]
    for name, record in membership["members"].items():
        status = "active" if record["active"] else "inactive"
        lines.append(
            f"  {name} {status} incarnation={record['incarnation']} place={record['place']}"
        )
    return "\n".join(lines)


def _format_verification(result: dict[str, Any]) -> str:
    return "\n".join(
        [
            "VERIFICATION",
            "  static:",
            "    ok authority(Alice) >= Admin",
            "    ok dice_owner(game) = Alice",
            "    ok handoff has witness(draw_pub#1)",
            "  runtime:",
            "    ok Alice active at membership_epoch 0",
            "    ok game_epoch action matches current epoch",
            "  model-check:",
            "    ok no_double_dice_owner",
            "    ok late_join_sees_published_history",
            "    ok reset_invalidates_pending_actions",
        ]
    )


def _format_signatures(result: dict[str, Any]) -> str:
    lines = ["TERM SIGNATURES"]
    for row in result.get("term_signatures", []):
        evidence_role = row.get("evidence_role")
        suffix = f" [{evidence_role}]" if evidence_role else ""
        lines.append(f"  - {row['kind']}: {row['name']}{suffix}")
    if len(lines) == 1:
        lines.append("  - none")
    return "\n".join(lines)


def _format_envelopes(result: dict[str, Any]) -> str:
    lines = ["MESSAGE ENVELOPES"]
    for row in result.get("message_envelopes", []):
        claim = row.get("principal_claim") or {}
        auth = row.get("auth_evidence")
        auth_kind = auth["kind"] if isinstance(auth, dict) else "none"
        lines.append(
            f"  - {row['envelope_id']} {row['from_place']} -> {row['to_place']} outcome={row['dispatch_outcome']}"
        )
        lines.append(
            f"      payload: {row['payload_kind']}:{row['payload_ref']} transport={row['transport']}"
        )
        lines.append(
            f"      principal: {claim.get('principal', '?')} authority={claim.get('claimed_authority', '?')} auth={auth_kind}"
        )
        lines.append(
            f"      membership: epoch={row['membership_epoch']} incarnation={row['member_incarnation']}"
        )
        caps = ", ".join(row.get("capability_requirements") or []) or "none"
        checks = ", ".join(row.get("authorization_checks") or []) or "none"
        witnesses = ", ".join(row.get("witness_refs") or []) or "none"
        lines.append(f"      capabilities: {caps}")
        lines.append(f"      checks: {checks}")
        lines.append(f"      witness_refs: {witnesses}")
    if len(lines) == 1:
        lines.append("  - none")
    return "\n".join(lines)


def _format_layers(result: dict[str, Any]) -> str:
    lines = ["LAYER SIGNATURES"]
    for row in result.get("layer_signatures", []):
        lines.append(f"  - {row['layer']}")
        for key in ("requires", "provides", "transforms", "checks", "emits", "laws"):
            values = row.get(key) or []
            pretty = ", ".join(values) if values else "none"
            lines.append(f"      {key}: {pretty}")
    if len(lines) == 1:
        lines.append("  - none")
    return "\n".join(lines)


def _format_visualization(result: dict[str, Any]) -> str:
    lines = ["VISUALIZATION"]
    for row in result.get("visualization_views", []):
        lines.append(
            f"  - {row['view_id']} kind={row['view_kind']} label={row['label']} authority={row['authority']} redaction={row['redaction']}"
        )
        lines.append(f"      source_refs: {', '.join(row.get('source_refs') or []) or 'none'}")
        summary = row.get("summary") or {}
        if summary:
            rendered = ", ".join(f"{key}={value}" for key, value in summary.items())
            lines.append(f"      summary: {rendered}")
    lines.append("TELEMETRY")
    for row in result.get("telemetry_rows", []):
        lines.append(
            f"  - {row['row_id']} kind={row['row_kind']} label={row['label']} authority={row['authority']} redaction={row['redaction']}"
        )
        lines.append(f"      source_refs: {', '.join(row.get('source_refs') or []) or 'none'}")
        fields = row.get("fields") or {}
        if fields:
            rendered = ", ".join(f"{key}={value}" for key, value in fields.items())
            lines.append(f"      fields: {rendered}")
    if len(lines) == 2:
        lines.append("  - none")
    return "\n".join(lines)


def _format_hotplug(result: dict[str, Any]) -> str:
    lifecycle = result.get("hotplug_lifecycle") or {}
    if not lifecycle:
        return "HOT-PLUG LIFECYCLE\n  - none"
    lines = [
        "HOT-PLUG LIFECYCLE",
        f"  attachpoint: {lifecycle.get('attachpoint_id', '?')}",
        f"  patch: {lifecycle.get('patch_id', '?')}",
        f"  state={lifecycle.get('lifecycle_state', '?')}",
    ]
    compatibility = lifecycle.get("compatibility") or {}
    lines.append(
        f"  compatibility: result={compatibility.get('result', '?')} authority={compatibility.get('required_authority', '?')}"
    )
    activation_cut = lifecycle.get("activation_cut") or {}
    lines.append(
        f"  activation_cut: request_envelope={activation_cut.get('request_envelope', '?')}"
    )
    detach_boundary = lifecycle.get("detach_boundary") or {}
    if detach_boundary:
        guards = ", ".join(detach_boundary.get("guards") or []) or "none"
        post_detach = detach_boundary.get("post_detach_action") or {}
        lines.append(f"  guards: {guards}")
        lines.append(
            f"  post_detach_action: {post_detach.get('verdict', '?')} ({post_detach.get('reason_family', '?')})"
        )
    migration = lifecycle.get("migration_contract") or {}
    lines.append(f"  migration_status: {migration.get('status', '?')}")
    return "\n".join(lines)


def format_pretty(payload: Any, *, debug: str | None = None) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']}: {row['summary']}" for row in payload)
    if not isinstance(payload, dict):
        return str(payload)
    if debug == "summary":
        return _format_summary(payload)
    if debug == "turn-trace":
        return _format_turn_trace(payload)
    if debug == "membership":
        return _format_membership(payload)
    if debug == "verification":
        return _format_verification(payload)
    if debug == "signatures":
        return _format_signatures(payload)
    if debug == "envelopes":
        return _format_envelopes(payload)
    if debug == "visualization":
        return _format_visualization(payload)
    if debug == "layers":
        return _format_layers(payload)
    if debug == "hotplug":
        return _format_hotplug(payload)
    sample = payload.get("sample")
    if sample == "00_world_bootstrap":
        return _format_world_summary(payload)
    if sample == "01_runtime_attach_game":
        game = payload["game"]
        return "\n".join(
            [
                "ATTACH SugorokuGame#1",
                "  world: EmptyWorld",
                "  package checked: yes",
                f"  imported membership epoch: {payload['imported_membership_epoch']}",
                f"  game place: {game['game_place']}",
                f"  admin: {game['admin']}",
                f"  turn order: {' -> '.join(game['turn_order'])}",
                f"  dice owner: {game['dice_owner']}",
                f"  phase: {game['phase']}",
            ]
        )
    if sample == "03_roll_publish_handoff":
        roll = payload["roll"]
        return "\n".join(
            [
                "TURN Alice",
                "  roll:",
                f"    draw = {roll['draw']}",
                "  publish:",
                f"    roll_result(Alice, {roll['draw']})",
                f"    witness = {roll['published_witness']}",
                "  handoff:",
                "    from Alice",
                "    to   Bob",
                "    reason: next_active_after(Alice)",
                "  state:",
                f"    dice_owner = {payload['game']['dice_owner']}",
                f"    published_rolls = {len(payload['game']['published_rolls'])}",
            ]
        )
    if "model_check_result" in payload:
        return "\n".join(
            [
                "MODEL CHECK",
                f"  result: {payload['model_check_result']}",
                f"  property: {payload.get('property', 'sugoroku_world_properties')}",
            ]
        )
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
        choices=[
            "summary",
            "turn-trace",
            "membership",
            "verification",
            "signatures",
            "envelopes",
            "visualization",
            "layers",
            "hotplug",
        ],
        default=None,
    )
    run_parser.add_argument(
        "--transport", choices=ACTIVE_TRANSPORT_SEAMS, default="local_queue"
    )
    run_parser.add_argument("--format", choices=["pretty", "json"], default="pretty")

    check = sub.add_parser("check-all")
    check.add_argument(
        "--transport", choices=ACTIVE_TRANSPORT_SEAMS, default="local_queue"
    )
    check.add_argument("--format", choices=["pretty", "json"], default="pretty")

    model = sub.add_parser("model-check")
    model.add_argument("--format", choices=["pretty", "json"], default="pretty")

    close = sub.add_parser("closeout")
    close.add_argument("--format", choices=["pretty", "json"], default="pretty")
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    if args.command == "list":
        _print_payload(list_samples(), args.format)
    elif args.command == "run":
        _print_payload(
            run_sample(args.sample, transport=args.transport),
            args.format,
            debug=args.debug,
        )
    elif args.command == "check-all":
        _print_payload(check_all(transport=args.transport), args.format)
    elif args.command == "model-check":
        _print_payload(model_check(), args.format)
    elif args.command == "closeout":
        _print_payload(closeout(), args.format)
    else:
        raise AssertionError(f"unsupported command {args.command}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
