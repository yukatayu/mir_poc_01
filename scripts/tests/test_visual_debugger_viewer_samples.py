from __future__ import annotations

import copy
import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import visual_debugger_viewer_samples


STUB_ANCHORS = {
    "helper_sugoroku_visualization": {
        "panel_ids": [
            "turn_timeline",
            "message_route",
            "verification_summary",
            "projection_view",
        ],
        "panel_kinds": [
            "turn_timeline",
            "message_route",
            "verification_summary",
            "projection_view",
        ],
        "telemetry_ids": ["roll_request#1", "handoff_notice#1"],
        "telemetry_kinds": ["message_dispatch", "published_roll"],
        "retention_scopes": ["helper_local_ephemeral"],
        "redaction_policies": [
            "omit_auth_evidence_payload",
            "projection_summary_only",
            "published_history_only",
            "verification_summary_only",
        ],
        "catalog_keys": [],
        "panels": [
            {
                "panel_id": "turn_timeline",
                "panel_kind": "turn_timeline",
                "label": "helper:published-history",
                "authority": "ObservePublishedHistory(SugorokuGame#1)",
                "redaction": "published_history_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["roll", "turn_trace"],
                "focus_refs": ["roll_request#1", "handoff_notice#1"],
                "notes": ["helper-local visualization first cut"],
            },
            {
                "panel_id": "message_route",
                "panel_kind": "message_route",
                "label": "helper:transport-audit",
                "authority": "InspectLocalQueue(SugorokuGame#1)",
                "redaction": "omit_auth_evidence_payload",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["message_envelopes"],
                "focus_refs": ["roll_request#1", "handoff_notice#1"],
                "notes": ["helper-local visualization first cut"],
            },
            {
                "panel_id": "verification_summary",
                "panel_kind": "verification_summary",
                "label": "helper:verification",
                "authority": "ObserveVerificationSummary(SugorokuGame#1)",
                "redaction": "verification_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["verification"],
                "focus_refs": ["handoff_notice#1"],
                "notes": ["helper-local visualization first cut"],
            },
            {
                "panel_id": "projection_view",
                "panel_kind": "projection_view",
                "label": "helper:projection",
                "authority": "InspectProjection(SugorokuWorldSource#1)",
                "redaction": "projection_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": [
                    "game",
                    "message_envelopes",
                    "turn_trace",
                    "telemetry_rows",
                ],
                "focus_refs": ["turn_timeline", "message_route", "verification_summary"],
                "notes": [
                    "helper-local projection preview over existing carriers",
                    "no final public viewer API",
                ],
            }
        ],
        "telemetry_rows": [
            {
                "telemetry_id": "roll_request#1",
                "telemetry_kind": "message_dispatch",
                "label": "helper:game-transition",
                "authority": "InspectLocalQueue(SugorokuGame#1)",
                "redaction": "omit_auth_evidence_payload",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["message_envelopes[roll_request#1]"],
                "channel": "game_action_boundary",
                "value_summary": "accepted",
                "notes": ["helper-local local-queue dispatch summary"],
            }
        ],
    },
    "helper_sugoroku_closeout_catalog": {
        "panel_ids": [
            "attach_lifecycle",
            "turn_timeline",
            "message_route",
            "verification_summary",
            "projection_view",
            "membership_snapshot",
            "detach_lifecycle",
        ],
        "panel_kinds": [
            "hotplug_lifecycle",
            "membership_snapshot",
            "message_route",
            "projection_view",
            "turn_timeline",
            "verification_summary",
        ],
        "telemetry_ids": [
            "attach_activation#1",
            "roll_request#1",
            "handoff_notice#1",
            "late_join_membership#1",
            "late_join_history#1",
            "reset_model_check#1",
            "detach_boundary#1",
        ],
        "telemetry_kinds": [
            "history_visibility",
            "hotplug_activation",
            "hotplug_detach",
            "membership_update",
            "message_dispatch",
            "model_check_summary",
            "published_roll",
        ],
        "retention_scopes": ["helper_local_ephemeral"],
        "redaction_policies": [
            "counterexample_shape_summary_only",
            "lifecycle_summary_only",
            "omit_auth_evidence_payload",
            "pending_turn_order_only",
            "projection_summary_only",
            "published_history_only",
            "verification_summary_only",
        ],
        "catalog_keys": [
            "hotplug_scope",
            "hotplug_view_ids",
            "hotplug_telemetry_row_ids",
            "signature_lanes",
            "layer_signature_lanes",
        ],
        "panels": [
            {
                "panel_id": "attach_lifecycle",
                "panel_kind": "hotplug_lifecycle",
                "label": "helper:hotplug",
                "authority": "InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                "redaction": "lifecycle_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["hotplug_lifecycle"],
                "focus_refs": ["attach_activation#1"],
                "notes": ["helper-local attach lifecycle visualization"],
            },
            {
                "panel_id": "membership_snapshot",
                "panel_kind": "membership_snapshot",
                "label": "helper:membership",
                "authority": "ObserveMembership(WorldMembers)",
                "redaction": "pending_turn_order_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["membership"],
                "focus_refs": ["late_join_membership#1", "late_join_history#1"],
                "notes": ["helper-local membership visualization"],
            },
            {
                "panel_id": "verification_summary",
                "panel_kind": "verification_summary",
                "label": "helper:verification",
                "authority": "ObserveVerificationSummary(SugorokuGame#1)",
                "redaction": "verification_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["verification"],
                "focus_refs": ["handoff_notice#1"],
                "notes": ["helper-local verification summary"],
            },
            {
                "panel_id": "detach_lifecycle",
                "panel_kind": "hotplug_lifecycle",
                "label": "helper:hotplug",
                "authority": "InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                "redaction": "lifecycle_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["hotplug_lifecycle"],
                "focus_refs": ["detach_boundary#1"],
                "notes": ["helper-local detach lifecycle visualization"],
            },
        ],
        "telemetry_rows": [
            {
                "telemetry_id": "attach_activation#1",
                "telemetry_kind": "hotplug_activation",
                "label": "helper:hotplug",
                "authority": "InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                "redaction": "lifecycle_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["hotplug_lifecycle"],
                "channel": "AttachPoint[SugorokuGame#1]",
                "value_summary": "attached_active",
                "notes": ["helper-local hot-plug activation summary"],
            },
            {
                "telemetry_id": "late_join_membership#1",
                "telemetry_kind": "membership_update",
                "label": "helper:membership",
                "authority": "ObserveMembership(WorldMembers)",
                "redaction": "pending_turn_order_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["membership"],
                "channel": "WorldServerPlace",
                "value_summary": "membership_epoch=1",
                "notes": ["helper-local membership telemetry row"],
            },
            {
                "telemetry_id": "late_join_history#1",
                "telemetry_kind": "history_visibility",
                "label": "helper:published-history",
                "authority": "ObservePublishedHistory(SugorokuGame#1)",
                "redaction": "published_history_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["Dave.visible_rolls"],
                "channel": "ParticipantPlace[Dave]",
                "value_summary": "visible_roll_count=1",
                "notes": ["helper-local published-history telemetry row"],
            },
            {
                "telemetry_id": "reset_model_check#1",
                "telemetry_kind": "model_check_summary",
                "label": "helper:verification",
                "authority": "ObserveVerificationSummary(SugorokuGame#1)",
                "redaction": "counterexample_shape_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["model_check"],
                "channel": "verification",
                "value_summary": "pass",
                "notes": ["second-line verification summary"],
            },
            {
                "telemetry_id": "detach_boundary#1",
                "telemetry_kind": "hotplug_detach",
                "label": "helper:hotplug",
                "authority": "InspectAttachLifecycle(AttachPoint[SugorokuGame#1])",
                "redaction": "lifecycle_summary_only",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["hotplug_lifecycle"],
                "channel": "detach_request#1",
                "value_summary": "reject",
                "notes": ["helper-local post-detach rejection summary"],
            },
        ],
    },
    "network_route_trace_bundle": {
        "panel_ids": ["network_route_trace"],
        "panel_kinds": ["route_trace"],
        "telemetry_ids": [
            "roll_request#1",
            "handoff_notice#1",
            "detach_request#1",
            "detached_roll_request#1",
        ],
        "telemetry_kinds": ["route_hop"],
        "retention_scopes": ["helper_local_ephemeral"],
        "redaction_policies": ["observer_safe_route_trace"],
        "catalog_keys": ["what_it_proves", "what_it_does_not_prove"],
        "panels": [
            {
                "panel_id": "network_route_trace",
                "panel_kind": "route_trace",
                "label": "helper:transport-audit",
                "authority": "ObserveRouteTrace(NetworkTransportLane)",
                "redaction": "observer_safe_route_trace",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["child:03_roll_publish_handoff:json"],
                "focus_refs": ["roll_request#1", "handoff_notice#1"],
                "notes": ["route trace can stay typed and redacted"],
            }
        ],
        "telemetry_rows": [
            {
                "telemetry_id": "roll_request#1",
                "telemetry_kind": "route_hop",
                "label": "transition",
                "authority": "ObserveRouteTrace(NetworkTransportLane)",
                "redaction": "observer_safe_route_trace",
                "retention_scope": "helper_local_ephemeral",
                "source_refs": ["message_envelopes[roll_request#1]"],
                "channel": "game_action_boundary",
                "value_summary": "accepted",
                "notes": ["hop_index=1"],
            }
        ],
    },
    "runtime_closeout_catalog": {
        "panel_ids": [
            "authority_trace_redacted_view",
            "cross_place_projection",
            "provider_boundary_redacted_flow",
        ],
        "panel_kinds": ["audit_trace", "message_route", "projection_view"],
        "telemetry_ids": ["audit_trace_dispatch", "provider_boundary_dispatch"],
        "telemetry_kinds": ["message_dispatch"],
        "retention_scopes": ["report_local_inventory"],
        "redaction_policies": [
            "authority_trace_summary_only",
            "dispatch_outcome_only",
            "named_witness_only",
            "projection_summary_only",
        ],
        "catalog_keys": [
            "visualization_view_lanes",
            "telemetry_row_lanes",
            "retention_scope_names",
            "reserved_visualization_view_names",
        ],
        "panels": [
            {
                "panel_id": "authority_trace_redacted_view",
                "panel_kind": "audit_trace",
                "label": "report:audit-trace",
                "authority": "ObserveAuthorityTrace(AuditPlace[AuthorityTrace])",
                "redaction": "authority_trace_summary_only",
                "retention_scope": "report_local_inventory",
                "source_refs": ["message_envelopes[audit_trace_request#1]"],
                "focus_refs": ["witness:draw_pub", "view:audit_trace"],
                "notes": [
                    "authority witness is evidence, not authentication",
                    "visualization stays downstream of witness production",
                ],
            },
            {
                "panel_id": "cross_place_projection",
                "panel_kind": "projection_view",
                "label": "report:projection",
                "authority": "InspectProjection(DelegatedRngSource#1)",
                "redaction": "projection_summary_only",
                "retention_scope": "report_local_inventory",
                "source_refs": ["message_envelopes[provider_request#1]"],
                "focus_refs": ["projection:provider_boundary_draw_route"],
                "notes": [
                    "report-local projection preview over current layer/message inventory",
                    "no final public viewer API",
                ],
            }
        ],
        "telemetry_rows": [
            {
                "telemetry_id": "audit_trace_dispatch",
                "telemetry_kind": "message_dispatch",
                "label": "report:audit-trace",
                "authority": "ObserveAuthorityTrace(AuditPlace[AuthorityTrace])",
                "redaction": "dispatch_outcome_only",
                "retention_scope": "report_local_inventory",
                "source_refs": ["message_envelopes[audit_trace_request#1]"],
                "channel": "audit_trace_boundary",
                "value_summary": "dispatch_outcome=accepted",
                "notes": ["report-local audit flow telemetry only"],
            }
        ],
    },
    "typed_external_host_route_bundle": {
        "panel_ids": ["room_message_route"],
        "panel_kinds": ["message_route"],
        "telemetry_ids": ["room_message_request#1", "room_message_receipt#1"],
        "telemetry_kinds": ["typed_effect_receipt", "typed_effect_request"],
        "retention_scopes": ["helper_local_synthetic_preview"],
        "redaction_policies": ["named_message_ref_only"],
        "catalog_keys": [
            "host_boundary_scope",
            "host_boundary_lanes",
            "non_collapse_lanes",
        ],
        "panels": [
            {
                "panel_id": "room_message_route",
                "panel_kind": "message_route",
                "label": "PublishedRoomNotice",
                "authority": "ObservePublishedRoomNotice",
                "redaction": "named_message_ref_only",
                "retention_scope": "helper_local_synthetic_preview",
                "source_refs": [
                    "room_message_request#1",
                    "room_message_receipt#1",
                ],
                "focus_refs": ["transport", "auth", "membership", "witness"],
                "notes": [
                    "visualization restriction can be shown without freezing final viewer contract",
                ],
            }
        ],
        "telemetry_rows": [
            {
                "telemetry_id": "room_message_request#1",
                "telemetry_kind": "typed_effect_request",
                "label": "PublishedRoomNotice",
                "authority": "RoomNoticeAuthority.Publisher",
                "redaction": "named_message_ref_only",
                "retention_scope": "helper_local_synthetic_preview",
                "source_refs": ["room_message_request#1"],
                "channel": "local_queue",
                "value_summary": "accepted",
                "notes": ["message route remains separate from auth evidence"],
            },
            {
                "telemetry_id": "room_message_receipt#1",
                "telemetry_kind": "typed_effect_receipt",
                "label": "PublishedRoomNotice",
                "authority": "ObservePublishedRoomNotice",
                "redaction": "named_message_ref_only",
                "retention_scope": "helper_local_synthetic_preview",
                "source_refs": ["room_message_receipt#1"],
                "channel": "local_queue",
                "value_summary": "accepted",
                "notes": ["published room notice stays distinct from transport auth"],
            },
        ],
    },
}


class VisualDebuggerViewerSamplesTests(unittest.TestCase):
    def test_list_contains_expected_viewer_bundles(self) -> None:
        rows = visual_debugger_viewer_samples.list_samples()

        self.assertEqual(
            [row["bundle_id"] for row in rows],
            [
                "P16-VIEW-01",
                "P16-VIEW-02",
                "P16-VIEW-03",
                "P16-VIEW-04",
                "P16-VIEW-05",
            ],
        )
        self.assertTrue(
            all(
                row["viewer_scope"]
                == "first_public_prototype_over_typed_inventories"
                for row in rows
            )
        )

    def test_run_sample_returns_helper_view_bundle(self) -> None:
        with mock.patch.object(
            visual_debugger_viewer_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            result = visual_debugger_viewer_samples.run_sample("P16-VIEW-01")

        self.assertEqual(result["bundle_id"], "P16-VIEW-01")
        self.assertEqual(result["anchor_kind"], "helper_sugoroku_visualization")
        self.assertTrue(result["alignment_passed"])
        self.assertIn("projection_view", result["live_bundle"]["panel_ids"])
        self.assertIn("message_dispatch", result["live_bundle"]["telemetry_kinds"])
        self.assertIn("not a final public viewer API", result["bundle_boundary"])

    def test_run_sample_returns_runtime_catalog_bundle(self) -> None:
        with mock.patch.object(
            visual_debugger_viewer_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            result = visual_debugger_viewer_samples.run_sample("P16-VIEW-04")

        self.assertEqual(result["bundle_id"], "P16-VIEW-04")
        self.assertEqual(result["anchor_kind"], "runtime_closeout_catalog")
        self.assertTrue(result["alignment_passed"])
        self.assertEqual(result["live_bundle"]["retention_scopes"], ["report_local_inventory"])
        self.assertIn("projection_view", result["live_bundle"]["panel_kinds"])
        self.assertIn("runtime_canonical_inventory", result["review_categories"])

    def test_check_all_reports_alignment_drift(self) -> None:
        drifted = copy.deepcopy(STUB_ANCHORS)
        drifted["runtime_closeout_catalog"]["retention_scopes"] = [
            "helper_local_ephemeral"
        ]

        with mock.patch.object(
            visual_debugger_viewer_samples,
            "_collect_anchor_snapshots",
            return_value=drifted,
        ):
            result = visual_debugger_viewer_samples.check_all()

        self.assertIn("P16-VIEW-04", result["failed"])
        failures = result["failure_details"]["P16-VIEW-04"]
        self.assertTrue(
            any(check["field"] == "retention_scopes" for check in failures),
            failures,
        )

    def test_normalize_sugoroku_closeout_canonicalizes_duplicate_panel_ids(self) -> None:
        payload = {
            "visualization_views": [
                {
                    "view_id": "verification_summary",
                    "view_kind": "verification_summary",
                    "label": "helper:verification",
                    "authority": "ObserveVerificationSummary(SugorokuGame#1)",
                    "redaction": "verification_summary_only",
                    "retention_scope": "helper_local_ephemeral",
                    "source_refs": ["verification"],
                    "summary": {"telemetry_refs": ["handoff_notice#1"]},
                    "notes": ["primary verification panel"],
                },
                {
                    "view_id": "verification_summary",
                    "view_kind": "verification_summary",
                    "label": "helper:verification",
                    "authority": "ObserveVerificationSummary(SugorokuGame#1)",
                    "redaction": "counterexample_shape_summary_only",
                    "retention_scope": "helper_local_ephemeral",
                    "source_refs": ["model_check"],
                    "summary": {"telemetry_refs": ["reset_model_check#1"]},
                    "notes": ["model-check verification panel"],
                },
            ],
            "telemetry_rows": [],
        }

        result = visual_debugger_viewer_samples._normalize_sugoroku_closeout(payload)

        self.assertEqual(
            result["panel_ids"],
            ["verification_summary", "verification_summary@2"],
        )
        self.assertIn(
            "canonicalized duplicate panel_id from verification_summary",
            result["panels"][1]["notes"],
        )

    def test_closeout_records_public_prototype_inventory(self) -> None:
        with mock.patch.object(
            visual_debugger_viewer_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            result = visual_debugger_viewer_samples.closeout()

        self.assertEqual(
            result["viewer_prototype_scope"],
            "first_public_prototype_over_typed_inventories",
        )
        self.assertEqual(result["bundle_count"], 5)
        self.assertEqual(
            [row["bundle_id"] for row in result["prototype_bundles"]],
            [
                "P16-VIEW-01",
                "P16-VIEW-02",
                "P16-VIEW-03",
                "P16-VIEW-04",
                "P16-VIEW-05",
            ],
        )
        self.assertEqual(
            result["viewer_panel_lanes"],
            [
                "panel_id",
                "panel_kind",
                "label",
                "authority",
                "redaction",
                "retention_scope",
                "source_refs",
                "focus_refs",
                "notes",
            ],
        )
        self.assertIn("telemetry_id", result["viewer_telemetry_lanes"])
        self.assertIn("final_public_viewer_api", result["kept_later_gates"])
        self.assertIn("route_trace", result["actualized_panel_kinds"])
        self.assertIn("audit_trace", result["actualized_panel_kinds"])
        self.assertIn("membership_update", result["actualized_telemetry_kinds"])
        self.assertIn("typed_effect_request", result["actualized_telemetry_kinds"])
        self.assertIn("typed_effect_receipt", result["actualized_telemetry_kinds"])
        self.assertIn(
            "python3 scripts/visual_debugger_viewer_samples.py check-all --format json",
            result["validation_floor"],
        )
        self.assertIn(
            "python3 scripts/network_transport_samples.py run NET-05 --debug route-trace --format json",
            result["validation_floor"],
        )
        self.assertIn("not a final public viewer API", result["prototype_boundary"])

    def test_list_pretty_prints_viewer_bundles(self) -> None:
        pretty = visual_debugger_viewer_samples.format_pretty(
            visual_debugger_viewer_samples.list_samples()
        )

        self.assertIn("VIEWER PROTOTYPE BUNDLES", pretty)
        self.assertIn("P16-VIEW-01", pretty)
        self.assertIn("public_prototype_bundle", pretty)

    def test_closeout_pretty_prints_viewer_inventory(self) -> None:
        with mock.patch.object(
            visual_debugger_viewer_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            pretty = visual_debugger_viewer_samples.format_pretty(
                visual_debugger_viewer_samples.closeout()
            )

        self.assertIn("VIEWER CLOSEOUT SUMMARY", pretty)
        self.assertIn("viewer_prototype_scope: first_public_prototype_over_typed_inventories", pretty)
        self.assertIn("route_trace", pretty)
        self.assertIn("final_public_viewer_api", pretty)


if __name__ == "__main__":
    unittest.main()
