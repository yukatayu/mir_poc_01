from __future__ import annotations

import json
import sys
import unittest
from io import StringIO
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import sugoroku_world_samples


class SugorokuWorldSamplesTests(unittest.TestCase):
    def test_list_contains_required_vertical_slice_samples(self) -> None:
        samples = sugoroku_world_samples.list_samples()
        sample_ids = [sample["sample_id"] for sample in samples]

        self.assertEqual(
            sample_ids,
            [
                "00_world_bootstrap",
                "01_runtime_attach_game",
                "02_admin_start_reset",
                "03_roll_publish_handoff",
                "04_non_owner_roll_rejected",
                "05_late_join_history_visible",
                "06_leave_non_owner",
                "07_owner_leave_reassign",
                "08_reset_interleaving_model_check",
                "09_detach_todo",
            ],
        )

    def test_world_bootstrap_builds_logical_places_and_members(self) -> None:
        result = sugoroku_world_samples.run_sample("00_world_bootstrap")

        self.assertEqual(result["static_verdict"], "valid")
        self.assertEqual(result["world"], "EmptyWorld")
        self.assertEqual(result["membership_epoch"], 0)
        self.assertEqual(result["active_members"], ["Alice", "Bob", "Carol"])
        self.assertIn("WorldServerPlace", result["places"])
        self.assertIn("ParticipantPlace[Alice]", result["places"])

    def test_runtime_attach_appoints_alice_admin_and_game_place(self) -> None:
        result = sugoroku_world_samples.run_sample("01_runtime_attach_game")

        self.assertEqual(result["terminal_outcome"], "success")
        self.assertEqual(result["game"]["game_place"], "SugorokuGamePlace#1")
        self.assertEqual(result["game"]["admin"], "Alice")
        self.assertEqual(result["game"]["phase"], "Attached")
        self.assertEqual(result["game"]["dice_owner"], "Alice")

    def test_roll_publish_handoff_moves_dice_owner_to_bob(self) -> None:
        result = sugoroku_world_samples.run_sample("03_roll_publish_handoff")

        self.assertEqual(result["terminal_outcome"], "success")
        self.assertEqual(result["game"]["dice_owner"], "Bob")
        self.assertEqual(result["roll"]["roller"], "Alice")
        self.assertEqual(result["roll"]["published_witness"], "draw_pub#1")
        self.assertIn("roll_is_published_before_handoff", result["properties_passed"])

    def test_non_owner_roll_is_rejected(self) -> None:
        result = sugoroku_world_samples.run_sample("04_non_owner_roll_rejected")

        self.assertEqual(result["static_or_runtime_verdict"], "reject")
        self.assertEqual(result["reason_family"], "dice_owner_requirement_failed")
        self.assertEqual(result["required"], "dice_owner(SugorokuGame#1) = Carol")
        self.assertEqual(result["actual"], "Bob")

    def test_late_join_sees_history_but_is_pending(self) -> None:
        result = sugoroku_world_samples.run_sample("05_late_join_history_visible")

        self.assertEqual(result["terminal_outcome"], "success")
        self.assertTrue(result["membership_epoch_incremented"])
        self.assertTrue(result["Dave"]["active"])
        self.assertTrue(result["Dave"]["published_history_visible"])
        self.assertFalse(result["Dave"]["in_turn_order"])
        self.assertTrue(result["Dave"]["pending_player"])

    def test_model_check_reports_required_properties(self) -> None:
        result = sugoroku_world_samples.model_check()

        self.assertEqual(result["model_check_result"], "pass")
        self.assertIn("no_double_dice_owner", result["properties"])
        self.assertIn("reset_invalidates_pending_actions", result["properties"])
        self.assertIn("admin_reset_does_not_interleave_with_roll_commit_badly", result["properties"])
        self.assertEqual(result["broken_variant"]["model_check_result"], "counterexample")

    def test_check_all_covers_static_runtime_and_model_checks(self) -> None:
        result = sugoroku_world_samples.check_all()

        self.assertEqual(result["sample_count"], 10)
        self.assertEqual(result["failed"], [])
        self.assertIn("admin-only start/reset", result["static_checks"])
        self.assertIn("handoff requires publish witness", result["runtime_guards"])
        self.assertIn("detach_rejects_domain_actions", result["model_check_properties"])

    def test_closeout_records_limitations(self) -> None:
        result = sugoroku_world_samples.closeout()

        self.assertIn("PlaceRuntime", result["runtime_components"])
        self.assertIn("samples/clean-near-end/sugoroku-world", result["active_sample_root"])
        self.assertIn("no real network yet", result["limitations"])
        self.assertIn("detach is TODO lifecycle boundary", result["limitations"])

    def test_roll_publish_handoff_exposes_term_signatures(self) -> None:
        result = sugoroku_world_samples.run_sample("03_roll_publish_handoff")

        signatures = {
            (row["kind"], row["name"], row["evidence_role"])
            for row in result["term_signatures"]
        }
        self.assertIn(("transition", "take_turn_alice", "source_decl"), signatures)
        self.assertIn(("transition", "take_turn_alice", "sample_transition"), signatures)
        self.assertIn(("effect", "roll_dice", "source_decl"), signatures)
        self.assertIn(("witness", "draw_pub#1", "runtime_witness"), signatures)
        self.assertIn(("relation", "publication_order", "derived_relation"), signatures)

    def test_signatures_debug_prints_signature_inventory(self) -> None:
        pretty = sugoroku_world_samples.format_pretty(
            sugoroku_world_samples.run_sample("03_roll_publish_handoff"),
            debug="signatures",
        )

        self.assertIn("TERM SIGNATURES", pretty)
        self.assertIn("transition: take_turn_alice [source_decl]", pretty)
        self.assertIn("transition: take_turn_alice [sample_transition]", pretty)
        self.assertIn("effect: roll_dice", pretty)

    def test_closeout_records_signature_debug_mode(self) -> None:
        result = sugoroku_world_samples.closeout()

        self.assertIn("--debug signatures", result["debug_output_modes"])
        self.assertEqual(
            result["signature_lanes"],
            ["kind", "name", "evidence_role"],
        )
        self.assertEqual(result["signature_scope"], "representative_slice")
        self.assertIn("source_decl", result["signature_evidence_roles"])
        self.assertIn("sample_transition", result["signature_evidence_roles"])
        self.assertIn("runtime_witness", result["signature_evidence_roles"])
        self.assertIn("derived_relation", result["signature_evidence_roles"])
        self.assertIn("validation_property", result["signature_evidence_roles"])
        self.assertIn("message", result["reserved_signature_kinds"])
        self.assertIn("adapter", result["reserved_signature_kinds"])
        self.assertIn("layer", result["reserved_signature_kinds"])

    def test_roll_publish_handoff_exposes_layer_signatures(self) -> None:
        result = sugoroku_world_samples.run_sample("03_roll_publish_handoff")

        layers = {row["layer"]: row for row in result["layer_signatures"]}
        self.assertIn("verification", layers)
        self.assertIn("runtime_trace", layers)
        self.assertIn("publication_order", layers["verification"]["requires"])
        self.assertIn("term_signatures", layers["verification"]["emits"])
        self.assertIn("dice_owner:Alice->Bob", layers["runtime_trace"]["transforms"])

    def test_membership_sample_exposes_membership_layer(self) -> None:
        result = sugoroku_world_samples.run_sample("05_late_join_history_visible")

        layers = {row["layer"]: row for row in result["layer_signatures"]}
        self.assertIn("membership", layers)
        self.assertIn("membership_epoch", layers["membership"]["requires"])
        self.assertIn("membership", layers["membership"]["emits"])

    def test_layers_debug_prints_layer_inventory(self) -> None:
        pretty = sugoroku_world_samples.format_pretty(
            sugoroku_world_samples.run_sample("03_roll_publish_handoff"),
            debug="layers",
        )

        self.assertIn("LAYER SIGNATURES", pretty)
        self.assertIn("verification", pretty)
        self.assertIn("runtime_trace", pretty)
        self.assertIn("requires: publication_order", pretty)

    def test_closeout_records_layer_debug_mode(self) -> None:
        result = sugoroku_world_samples.closeout()

        self.assertIn("--debug layers", result["debug_output_modes"])
        self.assertIn("verification", result["layer_signature_kinds"])
        self.assertIn("membership", result["layer_signature_kinds"])
        self.assertIn("auth", result["reserved_layer_signature_kinds"])

    def test_roll_publish_handoff_exposes_message_envelopes(self) -> None:
        result = sugoroku_world_samples.run_sample("03_roll_publish_handoff")

        envelopes = {row["envelope_id"]: row for row in result["message_envelopes"]}
        self.assertIn("roll_request#1", envelopes)
        self.assertIn("handoff_notice#1", envelopes)
        self.assertEqual(envelopes["roll_request#1"]["auth_evidence"], None)
        self.assertEqual(envelopes["roll_request#1"]["transport"], "local_queue")
        self.assertIn(
            "DiceOwner(Alice)",
            envelopes["roll_request#1"]["capability_requirements"],
        )
        self.assertIn(
            "draw_pub#1",
            envelopes["handoff_notice#1"]["witness_refs"],
        )
        self.assertEqual(
            envelopes["handoff_notice#1"]["principal_claim"]["principal"], "Alice"
        )

    def test_runtime_attach_loopback_transport_preserves_attach_request_parity(self) -> None:
        result = sugoroku_world_samples.run_sample(
            "01_runtime_attach_game", transport="loopback_socket"
        )

        self.assertEqual(result["transport_seam"], "loopback_socket")
        self.assertEqual(len(result["message_envelopes"]), 1)
        envelope = result["message_envelopes"][0]
        self.assertEqual(envelope["envelope_id"], "attach_request#1")
        self.assertEqual(envelope["transport"], "loopback_socket")
        self.assertEqual(envelope["auth_evidence"], None)
        self.assertEqual(envelope["membership_epoch"], 0)
        self.assertEqual(envelope["witness_refs"], [])
        self.assertIn(
            "helper-local loopback preview only; same-process envelope parity",
            envelope["notes"],
        )

    def test_runtime_attach_exposes_hotplug_lifecycle_summary(self) -> None:
        result = sugoroku_world_samples.run_sample("01_runtime_attach_game")

        lifecycle = result["hotplug_lifecycle"]
        self.assertEqual(lifecycle["attachpoint_id"], "AttachPoint[SugorokuGame#1]")
        self.assertEqual(lifecycle["lifecycle_state"], "attached_active")
        self.assertEqual(lifecycle["compatibility"]["result"], "compatible")
        self.assertEqual(
            lifecycle["activation_cut"]["request_envelope"], "attach_request#1"
        )
        self.assertEqual(
            lifecycle["migration_contract"]["status"], "not_started"
        )

    def test_detach_todo_exposes_hotplug_detach_boundary(self) -> None:
        result = sugoroku_world_samples.run_sample("09_detach_todo")

        lifecycle = result["hotplug_lifecycle"]
        self.assertEqual(lifecycle["attachpoint_id"], "AttachPoint[SugorokuGame#1]")
        self.assertEqual(lifecycle["lifecycle_state"], "detached_todo_boundary")
        self.assertEqual(
            lifecycle["detach_boundary"]["post_detach_action"]["verdict"], "reject"
        )
        self.assertIn(
            "phase(SugorokuGame#1) != GamePhase.Detached",
            lifecycle["detach_boundary"]["guards"],
        )
        self.assertEqual(lifecycle["migration_contract"]["status"], "deferred")

    def test_detach_todo_keeps_hotplug_summary_grounded_in_message_envelopes(self) -> None:
        result = sugoroku_world_samples.run_sample("09_detach_todo")

        envelopes = {row["envelope_id"]: row for row in result["message_envelopes"]}
        self.assertIn("detach_request#1", envelopes)
        self.assertIn("detached_roll_request#1", envelopes)
        self.assertEqual(envelopes["detach_request#1"]["dispatch_outcome"], "accepted")
        self.assertEqual(envelopes["detach_request#1"]["auth_evidence"], None)
        self.assertIn(
            "DetachComponent(SugorokuGamePackage)",
            envelopes["detach_request#1"]["capability_requirements"],
        )
        self.assertIn(
            "authority(Server) >= GameAuthority.Server",
            envelopes["detach_request#1"]["authorization_checks"],
        )
        self.assertEqual(
            envelopes["detached_roll_request#1"]["dispatch_outcome"], "rejected"
        )
        self.assertEqual(
            result["hotplug_lifecycle"]["activation_cut"]["request_envelope"],
            "detach_request#1",
        )

    def test_hotplug_debug_prints_lifecycle_inventory(self) -> None:
        pretty = sugoroku_world_samples.format_pretty(
            sugoroku_world_samples.run_sample("09_detach_todo"),
            debug="hotplug",
        )

        self.assertIn("HOT-PLUG LIFECYCLE", pretty)
        self.assertIn("AttachPoint[SugorokuGame#1]", pretty)
        self.assertIn("state=detached_todo_boundary", pretty)
        self.assertIn("post_detach_action: reject", pretty)

    def test_closeout_records_hotplug_debug_mode_and_layer(self) -> None:
        result = sugoroku_world_samples.closeout()

        self.assertIn("--debug hotplug", result["debug_output_modes"])
        self.assertIn("hot-plug", result["layer_signature_kinds"])
        self.assertNotIn("hot-plug", result["reserved_layer_signature_kinds"])
        self.assertIn("attached_active", result["hotplug_lifecycle_states"])
        self.assertIn("detached_todo_boundary", result["hotplug_lifecycle_states"])
        self.assertIn("hotplug_activation", result["telemetry_row_kinds"])
        self.assertIn("hotplug_detach", result["telemetry_row_kinds"])
        self.assertIn("hotplug_lifecycle", result["visualization_view_kinds"])

    def test_roll_publish_handoff_loopback_transport_preserves_envelope_parity(self) -> None:
        result = sugoroku_world_samples.run_sample(
            "03_roll_publish_handoff", transport="loopback_socket"
        )

        envelopes = {row["envelope_id"]: row for row in result["message_envelopes"]}
        self.assertEqual(result["transport_seam"], "loopback_socket")
        self.assertEqual(envelopes["roll_request#1"]["transport"], "loopback_socket")
        self.assertEqual(envelopes["roll_request#1"]["auth_evidence"], None)
        self.assertEqual(envelopes["roll_request#1"]["membership_epoch"], 0)
        self.assertIn(
            "helper-local loopback preview only; same-process envelope parity",
            envelopes["roll_request#1"]["notes"],
        )
        self.assertEqual(
            envelopes["handoff_notice#1"]["witness_refs"],
            ["draw_pub#1"],
        )

    def test_non_owner_rejection_exposes_envelope_reject_path(self) -> None:
        result = sugoroku_world_samples.run_sample("04_non_owner_roll_rejected")

        self.assertEqual(len(result["message_envelopes"]), 1)
        envelope = result["message_envelopes"][0]
        self.assertEqual(envelope["dispatch_outcome"], "rejected")
        self.assertIn(
            "dice_owner(SugorokuGame#1) = Carol",
            envelope["authorization_checks"],
        )
        self.assertIn("actual current owner is Bob", envelope["notes"])

    def test_non_owner_rejection_loopback_transport_keeps_reject_path(self) -> None:
        result = sugoroku_world_samples.run_sample(
            "04_non_owner_roll_rejected", transport="loopback_socket"
        )

        self.assertEqual(len(result["message_envelopes"]), 1)
        envelope = result["message_envelopes"][0]
        self.assertEqual(envelope["transport"], "loopback_socket")
        self.assertEqual(envelope["dispatch_outcome"], "rejected")
        self.assertIn(
            "helper-local loopback preview only; same-process envelope parity",
            envelope["notes"],
        )

    def test_envelopes_debug_prints_message_envelope_inventory(self) -> None:
        pretty = sugoroku_world_samples.format_pretty(
            sugoroku_world_samples.run_sample("03_roll_publish_handoff"),
            debug="envelopes",
        )

        self.assertIn("MESSAGE ENVELOPES", pretty)
        self.assertIn("roll_request#1", pretty)
        self.assertIn("auth=none", pretty)
        self.assertIn("witness_refs: draw_pub#1", pretty)

    def test_closeout_records_message_envelope_debug_mode(self) -> None:
        result = sugoroku_world_samples.closeout()

        self.assertIn("--debug envelopes", result["debug_output_modes"])
        self.assertIn("none", result["auth_evidence_modes"])
        self.assertIn("session_token", result["reserved_auth_evidence_modes"])
        self.assertIn("local_queue", result["transport_seams"])
        self.assertIn("loopback_socket", result["transport_seams"])
        self.assertEqual(result["reserved_transport_seams"], ["network_link"])

    def test_check_all_supports_loopback_transport_preview(self) -> None:
        result = sugoroku_world_samples.check_all(transport="loopback_socket")

        self.assertEqual(result["transport_seam"], "loopback_socket")
        self.assertEqual(result["failed"], [])

    def test_roll_publish_handoff_exposes_visualization_and_telemetry(self) -> None:
        result = sugoroku_world_samples.run_sample("03_roll_publish_handoff")

        views = {row["view_id"]: row for row in result["visualization_views"]}
        self.assertIn("turn_timeline", views)
        self.assertIn("message_route", views)
        self.assertIn("projection_view", views)
        self.assertEqual(views["turn_timeline"]["view_kind"], "turn_timeline")
        self.assertEqual(views["turn_timeline"]["label"], "helper:published-history")
        self.assertEqual(
            views["turn_timeline"]["authority"],
            "ObservePublishedHistory(SugorokuGame#1)",
        )
        self.assertEqual(
            views["turn_timeline"]["redaction"], "published_history_only"
        )
        self.assertIn("roll", views["turn_timeline"]["source_refs"])
        self.assertEqual(
            views["turn_timeline"]["summary"]["published_witness"], "draw_pub#1"
        )
        self.assertEqual(views["turn_timeline"]["summary"]["next_owner"], "Bob")
        self.assertEqual(views["projection_view"]["view_kind"], "projection_view")
        self.assertEqual(views["projection_view"]["label"], "helper:projection")
        self.assertEqual(
            views["projection_view"]["authority"],
            "InspectProjection(SugorokuWorldSource#1)",
        )
        self.assertEqual(
            views["projection_view"]["redaction"], "projection_summary_only"
        )
        self.assertEqual(
            views["projection_view"]["summary"]["authority_place"],
            "SugorokuGamePlace#1",
        )
        self.assertIn(
            "ParticipantPlace[Alice]",
            views["projection_view"]["summary"]["participant_places"],
        )
        self.assertEqual(
            views["projection_view"]["summary"]["adapter_transport"],
            "local_queue",
        )
        self.assertIn(
            "message_route",
            views["projection_view"]["summary"]["observer_views"],
        )

        telemetry = {row["row_id"]: row for row in result["telemetry_rows"]}
        self.assertIn("roll_request#1", telemetry)
        self.assertIn("handoff_notice#1", telemetry)
        self.assertEqual(telemetry["roll_request#1"]["row_kind"], "message_dispatch")
        self.assertEqual(
            telemetry["roll_request#1"]["label"], "helper:game-transition"
        )
        self.assertEqual(
            telemetry["roll_request#1"]["authority"],
            "InspectLocalQueue(SugorokuGame#1)",
        )
        self.assertEqual(
            telemetry["roll_request#1"]["redaction"], "omit_auth_evidence_payload"
        )
        self.assertEqual(
            telemetry["handoff_notice#1"]["fields"]["dispatch_outcome"], "accepted"
        )
        self.assertEqual(
            telemetry["handoff_notice#1"]["fields"]["witness_count"], 1
        )

    def test_late_join_visualization_marks_membership_redaction_boundary(self) -> None:
        result = sugoroku_world_samples.run_sample("05_late_join_history_visible")

        views = {row["view_id"]: row for row in result["visualization_views"]}
        self.assertIn("membership_snapshot", views)
        self.assertEqual(views["membership_snapshot"]["label"], "helper:membership")
        self.assertEqual(
            views["membership_snapshot"]["authority"],
            "ObserveMembership(WorldMembers)",
        )
        self.assertEqual(
            views["membership_snapshot"]["redaction"], "pending_turn_order_only"
        )
        self.assertEqual(
            views["membership_snapshot"]["summary"]["pending_players"], ["Dave"]
        )
        self.assertEqual(
            views["membership_snapshot"]["summary"]["visible_roll_count"], 1
        )

    def test_visualization_debug_prints_views_and_telemetry(self) -> None:
        pretty = sugoroku_world_samples.format_pretty(
            sugoroku_world_samples.run_sample("03_roll_publish_handoff"),
            debug="visualization",
        )

        self.assertIn("VISUALIZATION", pretty)
        self.assertIn("turn_timeline", pretty)
        self.assertIn("label=helper:published-history", pretty)
        self.assertIn("authority=ObservePublishedHistory(SugorokuGame#1)", pretty)
        self.assertIn("TELEMETRY", pretty)
        self.assertIn("roll_request#1", pretty)
        self.assertIn("redaction=omit_auth_evidence_payload", pretty)

    def test_projection_debug_prints_projection_view(self) -> None:
        pretty = sugoroku_world_samples.format_pretty(
            sugoroku_world_samples.run_sample("03_roll_publish_handoff"),
            debug="projection",
        )

        self.assertIn("PROJECTION", pretty)
        self.assertIn("projection_view", pretty)
        self.assertIn("SugorokuWorldSource#1", pretty)
        self.assertIn("SugorokuGamePlace#1", pretty)
        self.assertIn("local_queue", pretty)

    def test_closeout_records_visualization_debug_mode(self) -> None:
        result = sugoroku_world_samples.closeout()

        self.assertIn("--debug visualization", result["debug_output_modes"])
        self.assertIn("--debug projection", result["debug_output_modes"])
        self.assertIn("turn_timeline", result["visualization_view_kinds"])
        self.assertIn("message_route", result["visualization_view_kinds"])
        self.assertIn("projection_view", result["visualization_view_kinds"])
        self.assertIn("message_dispatch", result["telemetry_row_kinds"])
        self.assertIn("published_roll", result["telemetry_row_kinds"])
        self.assertIn("place_graph", result["reserved_visualization_view_kinds"])

    def test_cli_run_json_prints_json_payload(self) -> None:
        buffer = StringIO()
        with mock.patch("sys.stdout", buffer):
            exit_code = sugoroku_world_samples.main(
                ["run", "00_world_bootstrap", "--format", "json"]
            )

        self.assertEqual(exit_code, 0)
        payload = json.loads(buffer.getvalue())
        self.assertEqual(payload["sample"], "00_world_bootstrap")


if __name__ == "__main__":
    unittest.main()
