from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import typed_external_boundary_samples


class TypedExternalBoundarySamplesTests(unittest.TestCase):
    def test_list_contains_active_subset(self) -> None:
        rows = typed_external_boundary_samples.list_samples()
        self.assertEqual([row["sample_id"] for row in rows], ["EXT-03", "EXT-04"])

    def test_planned_family_contains_residual_ids(self) -> None:
        result = typed_external_boundary_samples.closeout()
        self.assertEqual(result["planned_sample_ids"], ["EXT-01", "EXT-02", "EXT-05"])

    def test_ext_03_keeps_message_route_separate_from_auth(self) -> None:
        result = typed_external_boundary_samples.run_sample("EXT-03")

        self.assertEqual(result["transport_seam"], "local_queue")
        self.assertIsNone(result["message_route"]["auth_evidence"])
        self.assertEqual(result["visualization_view"]["redaction"], "named_message_ref_only")
        self.assertEqual(result["typed_effect"]["scenario_label"], "SendRoomMessage")

    def test_ext_04_exposes_typed_failure_without_domain_commit(self) -> None:
        result = typed_external_boundary_samples.run_sample("EXT-04")

        self.assertEqual(result["terminal_outcome"], "typed_failure")
        failure = result["typed_failures"][0]
        self.assertEqual(failure["reason_family"], "AdapterQueueFull")
        self.assertTrue(failure["retryable"])
        self.assertFalse(failure["domain_mutation_committed"])

    def test_ext_03_visualization_view_stays_redacted(self) -> None:
        result = typed_external_boundary_samples.run_sample("EXT-03")

        view = result["visualization_view"]
        self.assertEqual(view["view_name"], "room_message_route")
        self.assertEqual(view["redaction"], "named_message_ref_only")

    def test_check_all_passes(self) -> None:
        result = typed_external_boundary_samples.check_all()
        self.assertEqual(result["failed"], [])
        self.assertEqual(result["sample_count"], 2)
        self.assertIn("EXT-04", result["passed"])

    def test_closeout_records_debug_modes(self) -> None:
        result = typed_external_boundary_samples.closeout()

        self.assertEqual(result["sample_count"], 2)
        self.assertIn("summary", result["debug_output_modes"])
        self.assertIn("envelopes", result["debug_output_modes"])
        self.assertIn("failures", result["debug_output_modes"])
        self.assertIn("visualization", result["debug_output_modes"])
        self.assertIn(
            "samples/not_implemented/typed-external-boundary",
            result["preview_sample_root"],
        )

    def test_closeout_records_residual_review_matrix(self) -> None:
        result = typed_external_boundary_samples.closeout()

        matrix = result["residual_review_matrix"]
        self.assertEqual([row["sample_id"] for row in matrix], ["EXT-01", "EXT-02", "EXT-05"])
        self.assertEqual(matrix[0]["current_anchor_kind"], "provider_boundary")
        self.assertEqual(matrix[1]["current_anchor_kind"], "visualization_projection_bridge")
        self.assertEqual(matrix[2]["current_anchor_kind"], "visualization_redaction_lane")
        self.assertIn("final_host_schema", matrix[1]["kept_later_gates"])

    def test_list_pretty_prints_preview_subset(self) -> None:
        pretty = typed_external_boundary_samples.format_pretty(
            typed_external_boundary_samples.list_samples()
        )
        self.assertIn("PREVIEW SAMPLES", pretty)
        self.assertIn("EXT-03", pretty)
        self.assertIn("EXT-04", pretty)

    def test_check_all_pretty_prints_planned_family_summary(self) -> None:
        pretty = typed_external_boundary_samples.format_pretty(
            typed_external_boundary_samples.check_all()
        )
        self.assertIn("CHECK-ALL SUMMARY", pretty)
        self.assertIn("planned residual: EXT-01, EXT-02, EXT-05", pretty)

    def test_closeout_pretty_prints_residual_review_summary(self) -> None:
        pretty = typed_external_boundary_samples.format_pretty(
            typed_external_boundary_samples.closeout()
        )
        self.assertIn("CLOSEOUT SUMMARY", pretty)
        self.assertIn("EXT-01 -> provider_boundary", pretty)
        self.assertIn("EXT-05 -> visualization_redaction_lane", pretty)

    def test_envelopes_debug_prints_message_envelopes(self) -> None:
        pretty = typed_external_boundary_samples.format_pretty(
            typed_external_boundary_samples.run_sample("EXT-03"),
            debug="envelopes",
        )
        self.assertIn("MESSAGE ENVELOPES", pretty)
        self.assertIn("room_message_request#1", pretty)
        self.assertIn("caps=cap:room_notice_publish", pretty)

    def test_failures_debug_prints_typed_failure(self) -> None:
        pretty = typed_external_boundary_samples.format_pretty(
            typed_external_boundary_samples.run_sample("EXT-04"),
            debug="failures",
        )
        self.assertIn("TYPED ADAPTER FAILURES", pretty)
        self.assertIn("AdapterQueueFull", pretty)

    def test_visualization_debug_prints_redaction_view(self) -> None:
        pretty = typed_external_boundary_samples.format_pretty(
            typed_external_boundary_samples.run_sample("EXT-03"),
            debug="visualization",
        )
        self.assertIn("VISUALIZATION VIEW", pretty)
        self.assertIn("named_message_ref_only", pretty)


if __name__ == "__main__":
    unittest.main()
