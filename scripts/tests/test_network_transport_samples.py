from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import network_transport_samples


class NetworkTransportSamplesTests(unittest.TestCase):
    def test_list_contains_net_02_through_05(self) -> None:
        samples = network_transport_samples.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in samples],
            ["NET-02", "NET-03", "NET-04", "NET-05"],
        )

    def test_net_02_process_boundary_keeps_envelope_and_witness_trace(self) -> None:
        result = network_transport_samples.run_sample("NET-02")

        self.assertEqual(result["transport_scope"], "helper_local_process_boundary")
        self.assertEqual(result["bridge_kind"], "subprocess_json_bridge")
        self.assertEqual(len(result["bridge_processes"]), 2)
        route_ids = [row["envelope_id"] for row in result["route_trace"]]
        self.assertIn("attach_request#1", route_ids)
        self.assertIn("roll_request#1", route_ids)
        self.assertIn("handoff_notice#1", route_ids)
        attach_row = result["route_trace"][0]
        self.assertEqual(attach_row["membership_epoch"], 0)
        self.assertEqual(attach_row["member_incarnation"], 0)
        self.assertEqual(attach_row["auth_mode"], "none")
        self.assertIn(
            "AttachComponent(SugorokuGamePackage)",
            attach_row["capability_requirements"],
        )
        self.assertIn(
            "authority(Server) >= GameAuthority.Server",
            attach_row["authorization_checks"],
        )
        self.assertIn("draw_pub#1", result["witness_refs"])
        self.assertIn(
            "process-boundary envelope serialization canary",
            result["what_it_proves"],
        )
        self.assertIn(
            "no continuous shared runtime state across processes",
            result["what_it_does_not_prove"],
        )

    def test_net_03_reconnect_epoch_guard_rejects_stale_membership(self) -> None:
        result = network_transport_samples.run_sample("NET-03")

        attempt = result["reconnect_attempt"]
        self.assertEqual(result["static_or_runtime_verdict"], "reject")
        self.assertEqual(result["reason_family"], "stale_membership_epoch")
        self.assertLess(
            attempt["offered_membership_epoch"], attempt["current_membership_epoch"]
        )
        self.assertLess(
            attempt["offered_member_incarnation"],
            attempt["current_member_incarnation"],
        )
        self.assertEqual(attempt["source_envelope"], "stale_roll_after_leave#1")

    def test_net_04_exposes_typed_transport_failure_family(self) -> None:
        result = network_transport_samples.run_sample("NET-04")

        failure_kinds = {row["failure_kind"] for row in result["typed_failures"]}
        self.assertEqual(
            failure_kinds,
            {"timeout", "queue_full", "route_not_found", "detach_after_send"},
        )
        self.assertTrue(
            any(row["retryable"] for row in result["typed_failures"]),
            "at least one failure should remain retryable",
        )
        self.assertTrue(
            any(not row["retryable"] for row in result["typed_failures"]),
            "at least one failure should stay terminal",
        )

    def test_net_05_emits_redacted_observer_safe_route_trace(self) -> None:
        result = network_transport_samples.run_sample("NET-05")

        view = result["visualization_view"]
        self.assertEqual(view["view_kind"], "route_trace")
        self.assertEqual(view["redaction"], "observer_safe_route_trace")
        self.assertEqual(view["authority"], "ObserveRouteTrace(NetworkTransportLane)")
        self.assertGreaterEqual(len(result["observer_route_trace"]), 2)
        for row in result["observer_route_trace"]:
            self.assertNotIn("auth_evidence", row)
            self.assertNotIn("claimed_capabilities", row)
            self.assertIn("payload_kind", row)
            self.assertIn("dispatch_outcome", row)

    def test_check_all_covers_all_transport_canaries(self) -> None:
        result = network_transport_samples.check_all()

        self.assertEqual(result["sample_count"], 4)
        self.assertEqual(result["failed"], [])
        self.assertIn("NET-02", result["passed"])
        self.assertIn("NET-05", result["passed"])

    def test_closeout_records_debug_modes_and_active_root(self) -> None:
        result = network_transport_samples.closeout()

        self.assertIn(
            "samples/clean-near-end/network-transport", result["active_sample_root"]
        )
        self.assertIn("scripts/network_transport_samples.py", result["helper_script"])
        self.assertEqual(result["sample_count"], 4)
        self.assertIn("--debug route-trace", result["debug_output_modes"])
        self.assertIn("--debug failures", result["debug_output_modes"])
        self.assertIn("no real network socket yet", result["limitations"])

    def test_route_trace_debug_prints_redacted_rows(self) -> None:
        pretty = network_transport_samples.format_pretty(
            network_transport_samples.run_sample("NET-05"),
            debug="route-trace",
        )

        self.assertIn("ROUTE TRACE", pretty)
        self.assertIn("observer_safe_route_trace", pretty)
        self.assertIn("handoff_notice#1", pretty)

    def test_failures_debug_prints_typed_failure_rows(self) -> None:
        pretty = network_transport_samples.format_pretty(
            network_transport_samples.run_sample("NET-04"),
            debug="failures",
        )

        self.assertIn("TYPED TRANSPORT FAILURES", pretty)
        self.assertIn("timeout", pretty)
        self.assertIn("detach_after_send", pretty)


if __name__ == "__main__":
    unittest.main()
