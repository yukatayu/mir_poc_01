from __future__ import annotations

import copy
import json
import sys
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import projection_codegen_samples


STUB_ANCHORS = {
    "sugoroku_projection_view": {
        "system_source": "SugorokuWorldSource#1",
        "server_places": ["WorldServerPlace", "SugorokuGamePlace#1"],
        "authority_place": "SugorokuGamePlace#1",
        "participant_places": [
            "ParticipantPlace[Alice]",
            "ParticipantPlace[Bob]",
            "ParticipantPlace[Carol]",
        ],
        "adapter_transport": "local_queue",
        "observer_views": [
            "turn_timeline",
            "message_route",
            "verification_summary",
        ],
        "membership_epoch": 0,
        "active_players": ["Alice", "Bob", "Carol"],
        "telemetry_refs": ["roll_request#1", "handoff_notice#1"],
        "retention_scope": "helper_local_ephemeral",
        "notes": [
            "helper-local projection preview over existing carriers",
            "place split stays distinct from participant/principal identity",
            "not a final emitted place-specific program",
        ],
    },
    "clean_near_end_cross_place_projection": {
        "system_source": "DelegatedRngSource#1",
        "authority_place": "ParticipantPlace[Alice]",
        "place_refs": [
            "ParticipantPlace[Alice]",
            "ProviderPlace[AuthorityRng]",
        ],
        "projection_refs": ["provider_boundary_draw_route"],
        "witness_refs": ["provider_receipt"],
        "redaction_rules": [
            "projection_summary_only",
            "auth_evidence:none_baseline",
        ],
        "message_envelope_refs": [
            "provider_request#1",
            "provider_receipt#1",
        ],
        "retention_scope": "report_local_inventory",
        "notes": [
            "report-local projection preview over current layer/message inventory",
            "authority placement stays distinct from provider placement",
            "not a final emitted place-specific program",
        ],
    },
}


class ProjectionCodegenSamplesTests(unittest.TestCase):
    def test_manifest_file_marks_bridge_evidence_only(self) -> None:
        manifest = json.loads(projection_codegen_samples.MANIFEST_PATH.read_text())

        self.assertEqual(manifest["projection_scope"], "generated_reserve_bridge_evidence")
        self.assertIn("bridge evidence", manifest["artifact_boundary"])
        self.assertIn("not a final emitted executable program", manifest["artifact_boundary"])

    def test_list_contains_generated_reserve_inventory(self) -> None:
        rows = projection_codegen_samples.list_samples()

        self.assertEqual(
            [row["artifact_id"] for row in rows],
            ["P15-GEN-01", "P15-GEN-02", "P15-GEN-03", "P15-GEN-04"],
        )
        self.assertTrue(
            all(row["materialization"] == "manifest_bridge_only" for row in rows)
        )

    def test_run_sample_returns_alignment_for_sugoroku_bundle(self) -> None:
        with mock.patch.object(
            projection_codegen_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            result = projection_codegen_samples.run_sample("P15-GEN-02")

        self.assertEqual(result["artifact_id"], "P15-GEN-02")
        self.assertEqual(result["anchor_kind"], "sugoroku_projection_view")
        self.assertEqual(result["materialization"], "manifest_bridge_only")
        self.assertTrue(result["alignment_passed"])
        self.assertEqual(result["live_anchor"]["membership_epoch"], 0)
        self.assertIn("preview_only_boundary", result["equivalence_review_categories"])
        self.assertIn("not a final emitted executable program", result["artifact_boundary"])

    def test_run_sample_returns_alignment_for_runtime_bundle(self) -> None:
        with mock.patch.object(
            projection_codegen_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            result = projection_codegen_samples.run_sample("P15-GEN-03")

        self.assertEqual(result["artifact_id"], "P15-GEN-03")
        self.assertEqual(result["anchor_kind"], "clean_near_end_cross_place_projection")
        self.assertTrue(result["alignment_passed"])
        self.assertEqual(result["live_anchor"]["authority_place"], "ParticipantPlace[Alice]")
        self.assertIn("ProviderPlace[AuthorityRng]", result["live_anchor"]["place_refs"])
        self.assertIn("authority_provider_non_collapse", result["equivalence_review_categories"])

    def test_check_all_passes_with_matching_stub_anchors(self) -> None:
        with mock.patch.object(
            projection_codegen_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            result = projection_codegen_samples.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["artifact_count"], 4)
        self.assertEqual(
            result["passed"],
            ["P15-GEN-01", "P15-GEN-02", "P15-GEN-03", "P15-GEN-04"],
        )

    def test_check_all_reports_alignment_drift(self) -> None:
        drifted = copy.deepcopy(STUB_ANCHORS)
        drifted["clean_near_end_cross_place_projection"]["authority_place"] = (
            "ParticipantPlace[Bob]"
        )

        with mock.patch.object(
            projection_codegen_samples,
            "_collect_anchor_snapshots",
            return_value=drifted,
        ):
            result = projection_codegen_samples.check_all()

        self.assertIn("P15-GEN-03", result["failed"])
        failures = result["failure_details"]["P15-GEN-03"]
        self.assertTrue(
            any(check["field"] == "authority_place" for check in failures),
            failures,
        )

    def test_closeout_records_requested_inventory(self) -> None:
        result = projection_codegen_samples.closeout()

        self.assertEqual(result["projection_scope"], "generated_reserve_bridge_evidence")
        self.assertEqual(result["artifact_count"], 4)
        self.assertEqual(
            result["generated_reserve_inventory"][0]["committed_files"],
            [
                "samples/generated/projection-placement/manifest.json",
            ],
        )
        self.assertEqual(
            [
                row["artifact_id"]
                for row in result["generated_bridge_artifact_inventory"]
            ],
            ["P15-GEN-01", "P15-GEN-02", "P15-GEN-03", "P15-GEN-04"],
        )
        self.assertIn(
            "authority_provider_non_collapse",
            result["equivalence_review_categories"],
        )
        self.assertIn("final_projection_ir", result["kept_later_gates"])
        self.assertIn(
            "python3 scripts/projection_codegen_samples.py check-all --format json",
            result["validation_floor"],
        )
        self.assertIn(
            "python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json",
            result["validation_floor"],
        )
        self.assertIn(
            "find samples/generated -maxdepth 3 -type f | sort",
            result["validation_floor"],
        )
        self.assertIn("manifest bridge evidence", result["artifact_boundary"])

    def test_list_pretty_prints_generated_reserve_artifacts(self) -> None:
        pretty = projection_codegen_samples.format_pretty(
            projection_codegen_samples.list_samples()
        )

        self.assertIn("GENERATED RESERVE ARTIFACTS", pretty)
        self.assertIn("P15-GEN-01", pretty)
        self.assertIn("manifest_bridge_only", pretty)

    def test_check_all_pretty_prints_alignment_summary(self) -> None:
        with mock.patch.object(
            projection_codegen_samples,
            "_collect_anchor_snapshots",
            return_value=copy.deepcopy(STUB_ANCHORS),
        ):
            pretty = projection_codegen_samples.format_pretty(
                projection_codegen_samples.check_all()
            )

        self.assertIn("CHECK-ALL SUMMARY", pretty)
        self.assertIn("passed: P15-GEN-01, P15-GEN-02, P15-GEN-03, P15-GEN-04", pretty)

    def test_closeout_pretty_prints_top_level_inventory(self) -> None:
        pretty = projection_codegen_samples.format_pretty(
            projection_codegen_samples.closeout()
        )

        self.assertIn("CLOSEOUT SUMMARY", pretty)
        self.assertIn("projection_scope: generated_reserve_bridge_evidence", pretty)
        self.assertIn("manifest bridge evidence only", pretty)
        self.assertIn("authority_provider_non_collapse", pretty)


if __name__ == "__main__":
    unittest.main()
