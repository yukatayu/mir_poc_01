from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/surface_mir_samples.py", *args, "--format", "json"],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"helper failed ({completed.returncode}): {completed.stderr}\n{completed.stdout}"
        )
    return json.loads(completed.stdout)


class SurfaceMirSamplesTests(unittest.TestCase):
    def test_matrix_reports_p_surf_04_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["family"], "surface_mir_alpha_source")
        self.assertEqual(payload["sample_count"], 24)
        self.assertEqual(payload["executable_count"], 24)
        self.assertEqual(payload["family_count"], 3)
        self.assertEqual(
            payload["matrix_status"]["surface_mir_elaboration"],
            "p_surf_04_auto_communication_elaboration_evidence",
        )
        self.assertEqual(payload["validation_errors"], [])
        self.assertFalse(payload["workflow_ready"])

    def test_positive_brace_place_sample_is_accepted(self) -> None:
        payload = _run_helper("run", "SURF-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(payload["actual"]["place_block_refs"], ["S"])
        self.assertEqual(payload["actual"]["state_summaries"][0]["owner_place"], "S")

    def test_bracket_place_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "SURF-02")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["bracket_place_scope_not_supported"],
        )

    def test_role_instance_sample_keeps_join_shape(self) -> None:
        payload = _run_helper("run", "SURF-05")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["role_instance_summaries"][0]["join_targets"],
            ["World as BrowserClient via WorldAdmission"],
        )

    def test_undeclared_place_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "SURF-06")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["undeclared_place_block_head"],
        )

    def test_invalid_role_binder_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "SURF-08")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["invalid_role_instance_binder"],
        )

    def test_role_named_s_stays_role_instance(self) -> None:
        payload = _run_helper("run", "SURF-09")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(payload["actual"]["role_names"], ["S"])
        self.assertEqual(
            payload["actual"]["role_instance_summaries"][0]["role_ref"],
            "S",
        )

    def test_indexed_state_positive_keeps_owner_and_keyspace_split(self) -> None:
        payload = _run_helper("run", "IDX-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        state = payload["actual"]["indexed_state_summaries"][0]
        self.assertEqual(state["owner_locus"], "S")
        self.assertEqual(state["keyspace_type"], "Participant")
        self.assertEqual(state["authority_model"], "owner_locus_or_explicit_capability")

    def test_indexed_state_key_authority_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "IDX-02")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["indexed_state_key_is_not_authority"],
        )
        self.assertFalse(
            payload["actual"]["access_summaries"][0]["key_authority_granted"]
        )

    def test_indexed_state_stale_key_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "IDX-03")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["stale_indexed_state_key"],
        )

    def test_indexed_state_compaction_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "IDX-04")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["indexed_state_compaction_blocked_by_retained_evidence"],
        )

    def test_indexed_state_nested_place_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "IDX-05")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["indexed_state_nested_place_requires_generated_request"],
        )
        self.assertEqual(
            payload["actual"]["access_summaries"][0]["access_locus"],
            "role:BrowserClient",
        )

    def test_elaboration_cross_locus_read_generates_observe_request(self) -> None:
        payload = _run_helper("run", "ELAB-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertIn("message_envelope", payload["actual"]["generated_edge_kinds"])
        self.assertIn("auto_observe", payload["actual"]["generated_edge_kinds"])
        self.assertIn("observe_request", payload["actual"]["generated_edge_kinds"])
        self.assertEqual(
            payload["actual"]["remote_request_summaries"][0]["request_kind"],
            "read",
        )
        self.assertEqual(
            payload["actual"]["message_envelope_summaries"][0]["envelope_kind"],
            "remote_read",
        )
        self.assertEqual(
            payload["actual"]["observation_summaries"][0]["field_name"],
            "hp",
        )

    def test_elaboration_cross_locus_write_generates_remote_write_request(self) -> None:
        payload = _run_helper("run", "ELAB-02")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertIn("message_envelope", payload["actual"]["generated_edge_kinds"])
        self.assertIn("remote_write_request", payload["actual"]["generated_edge_kinds"])
        self.assertEqual(
            payload["actual"]["remote_request_summaries"][0]["generated_from"],
            "nested_place_block",
        )
        self.assertEqual(payload["actual"]["publication_summaries"], [])

    def test_elaboration_private_field_auto_publish_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ELAB-03")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["private_field_auto_publish_rejected"],
        )
        self.assertEqual(payload["actual"]["publication_summaries"], [])
        self.assertEqual(payload["actual"]["observation_summaries"], [])

    def test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ELAB-04")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["generated_failure_not_declared"],
        )
        self.assertFalse(
            payload["actual"]["remote_request_summaries"][0]["failure_row_complete"]
        )

    def test_elaboration_source_span_sample_has_span_evidence(self) -> None:
        payload = _run_helper("run", "ELAB-05")

        self.assertTrue(payload["accepted"])
        self.assertIn(
            "remote_request",
            payload["actual"]["source_span_entity_kinds"],
        )
        self.assertIn(
            "surface_core_source_spans_preserved",
            payload["actual"]["obligation_codes"],
        )

    def test_elaboration_unsupported_statement_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ELAB-06")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["unsupported_surface_statement_for_elaboration"],
        )
        self.assertEqual(payload["actual"]["remote_request_summaries"], [])

    def test_elaboration_write_underdeclared_failure_row_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ELAB-07")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["generated_failure_not_declared"],
        )
        self.assertEqual(
            payload["actual"]["remote_request_summaries"][0]["request_kind"],
            "write",
        )
        self.assertFalse(
            payload["actual"]["remote_request_summaries"][0]["failure_row_complete"]
        )

    def test_elaboration_nested_place_read_keeps_owner_directed_shape(self) -> None:
        payload = _run_helper("run", "ELAB-08")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["remote_request_summaries"][0]["generated_from"],
            "nested_place_block",
        )

    def test_elaboration_visible_write_generates_publish_and_observe_rows(self) -> None:
        payload = _run_helper("run", "ELAB-09")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["message_envelope_summaries"][0]["envelope_kind"],
            "remote_write",
        )
        self.assertEqual(payload["actual"]["publication_summaries"][0]["field_name"], "hp")
        self.assertEqual(payload["actual"]["observation_summaries"][0]["field_name"], "hp")
        self.assertIn("auto_publish", payload["actual"]["generated_edge_kinds"])
        self.assertIn("auto_observe", payload["actual"]["generated_edge_kinds"])

    def test_elaboration_visibility_failure_row_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ELAB-10")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["generated_failure_not_declared"],
        )
        self.assertFalse(
            payload["actual"]["remote_request_summaries"][0]["failure_row_complete"]
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(len(payload["passed"]), 24)


if __name__ == "__main__":
    unittest.main()
