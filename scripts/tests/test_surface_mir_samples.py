from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SENSITIVE_DEVTOOLS_KEYS = {
    "activation_cut",
    "auth_evidence_ref",
    "capability_frontier_ref",
    "capability_refs",
    "hotplug_request",
    "membership_frontier_ref",
    "required_capability_witness_refs",
    "required_membership_witness_refs",
    "witness_refs",
}
SENSITIVE_DEVTOOLS_STRING_MARKERS = {
    "admission-witness-",
    "auth-evidence-",
    "capability-frontier-",
    "membership-frontier-",
    "private_token",
    "witness-",
}


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


def _contains_sensitive_devtools_material(value: object) -> bool:
    if isinstance(value, dict):
        return any(
            key in SENSITIVE_DEVTOOLS_KEYS
            or _contains_sensitive_devtools_material(nested)
            for key, nested in value.items()
        )
    if isinstance(value, list):
        return any(_contains_sensitive_devtools_material(nested) for nested in value)
    if isinstance(value, str):
        return any(marker in value for marker in SENSITIVE_DEVTOOLS_STRING_MARKERS)
    return False


class SurfaceMirSamplesTests(unittest.TestCase):
    def test_matrix_reports_p_surf_08_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["family"], "surface_mir_alpha_source")
        self.assertEqual(payload["sample_count"], 48)
        self.assertEqual(payload["executable_count"], 48)
        self.assertEqual(payload["family_count"], 7)
        self.assertEqual(
            payload["matrix_status"]["surface_mir_elaboration"],
            "p_surf_04_auto_communication_elaboration_evidence",
        )
        self.assertEqual(
            payload["matrix_status"]["surface_mir_role_admission"],
            "p_surf_05_role_admission_capability_grant_evidence",
        )
        self.assertEqual(
            payload["matrix_status"]["surface_mir_source_patch"],
            "p_surf_06_source_patch_hotplug_evidence",
        )
        self.assertEqual(
            payload["matrix_status"]["surface_mir_operational_source"],
            "p_surf_07_source_operational_suite_evidence",
        )
        self.assertEqual(
            payload["matrix_status"]["surface_mir_devtools_diagnostics"],
            "p_surf_08_devtools_diagnostics_evidence",
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
        self.assertEqual(
            payload["actual"]["dependency_summaries"][0]["dependency_kind"],
            "rhs_indexed_read",
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
        self.assertEqual(
            payload["actual"]["lab_diagnostic_details"],
            [
                {
                    "legacy_code": "generated_failure_not_declared",
                    "canon_id": "E-ROW-001",
                    "severity": "error",
                    "rule_instance": "BND-001.row-containment",
                    "failed_premise": "generated_failures_subset_declared_fails",
                    "missing_evidence": [
                        "MissingWitness",
                        "RouteUnavailable",
                        "StaleMembership",
                        "VisibilityDenied",
                    ],
                    "refs": [
                        "mirrorea_canon/theory/03-elaboration.md#BND-001",
                        "mirrorea_canon/spec/07-diagnostics-format.md#E-ROW-001",
                        "mirrorea_canon/theory/10-diagnostics.md#OBL-024",
                    ],
                    "lab_non_final": True,
                }
            ],
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
        self.assertEqual(
            payload["actual"]["lab_diagnostic_details"][0]["canon_id"],
            "E-ROW-001",
        )
        self.assertEqual(
            payload["actual"]["lab_diagnostic_details"][0]["missing_evidence"],
            ["MissingWitness", "RouteUnavailable", "StaleMembership"],
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

    def test_elaboration_scn01_same_field_assignment_records_rhs_dependency(self) -> None:
        payload = _run_helper("run", "ELAB-11")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(len(payload["actual"]["remote_request_summaries"]), 1)
        self.assertEqual(len(payload["actual"]["dependency_summaries"]), 1)
        dependency = payload["actual"]["dependency_summaries"][0]
        self.assertEqual(dependency["key_expr"], "self")
        self.assertEqual(dependency["field_name"], "position")
        self.assertEqual(dependency["generated_from"], "nested_place_block_rhs")

    def test_elaboration_scn02_attack_assignment_records_two_rhs_dependencies(self) -> None:
        payload = _run_helper("run", "ELAB-12")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(len(payload["actual"]["remote_request_summaries"]), 1)
        self.assertEqual(len(payload["actual"]["dependency_summaries"]), 2)
        self.assertEqual(
            [
                (row["key_expr"], row["field_name"])
                for row in payload["actual"]["dependency_summaries"]
            ],
            [("target", "hp"), ("self", "atk")],
        )
        self.assertEqual(payload["actual"]["observation_summaries"], [])

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
        self.assertEqual(
            payload["actual"]["lab_diagnostic_details"][0]["canon_id"],
            "E-ROW-002",
        )
        self.assertEqual(
            payload["actual"]["lab_diagnostic_details"][0]["missing_evidence"],
            ["VisibilityDenied"],
        )

    def test_role_admission_join_generates_grant_and_witness(self) -> None:
        payload = _run_helper("run", "ROLE-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["admission_verdict_summaries"][0]["verdict"],
            "accepted",
        )
        self.assertIn(
            "WriteState(World)",
            [
                row["capability"]
                for row in payload["actual"]["capability_grant_summaries"]
            ],
        )
        self.assertEqual(
            payload["actual"]["capability_grant_summaries"][0]["authority_source"],
            "admission_grant",
        )

    def test_role_claim_without_grant_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ROLE-02")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["role_claim_without_capability_grant"],
        )
        self.assertFalse(payload["actual"]["authority_check_summaries"][0]["accepted"])
        self.assertEqual(
            payload["actual"]["authority_check_summaries"][0]["reason_code"],
            "missing_capability_grant",
        )

    def test_stale_membership_negative_reports_expected_diagnostic(self) -> None:
        payload = _run_helper("run", "ROLE-03")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            [
                "stale_membership_message_rejected",
                "stale_membership_authority_rejected",
            ],
        )
        self.assertEqual(
            payload["actual"]["stale_rejection_summaries"][0]["reason_code"],
            "stale_membership",
        )
        self.assertFalse(payload["actual"]["authority_check_summaries"][0]["accepted"])
        self.assertEqual(
            payload["actual"]["authority_check_summaries"][0]["reason_code"],
            "stale_membership",
        )

    def test_hash_binding_metadata_does_not_claim_safety_proof(self) -> None:
        payload = _run_helper("run", "ROLE-04")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["hash_binding_summaries"][0]["package_hash"],
            "pkg_hash_v1",
        )
        self.assertFalse(
            payload["actual"]["hash_binding_summaries"][0]["semantic_safety_proof"]
        )

    def test_source_patch_positive_emits_activation_cut_without_eval(self) -> None:
        payload = _run_helper("run", "PATCH-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertTrue(payload["actual"]["hotplug_request_present"])
        self.assertEqual(payload["actual"]["hotplug_verdict_kind"], "accepted")
        self.assertTrue(payload["actual"]["activation_cut_present"])
        self.assertTrue(payload["actual"]["runtime_mutation_applied"])
        self.assertFalse(payload["actual"]["direct_eval_performed"])

    def test_source_patch_undeclared_failure_rejects_without_mutation(self) -> None:
        payload = _run_helper("run", "PATCH-02")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["generated_failure_not_declared"],
        )
        self.assertEqual(payload["actual"]["hotplug_verdict_kind"], "rejected")
        self.assertFalse(payload["actual"]["activation_cut_present"])
        self.assertFalse(payload["actual"]["runtime_mutation_applied"])

    def test_source_patch_self_grant_rejects_without_mutation(self) -> None:
        payload = _run_helper("run", "PATCH-03")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertIn(
            "patch_self_grant_server_authority_rejected",
            payload["actual"]["diagnostic_codes"],
        )
        self.assertEqual(payload["actual"]["hotplug_verdict_kind"], "rejected")
        self.assertFalse(payload["actual"]["activation_cut_present"])
        self.assertFalse(payload["actual"]["runtime_mutation_applied"])

    def test_source_patch_lifecycle_positive_exposes_devtools_row(self) -> None:
        payload = _run_helper("run", "PATCH-04")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["state_addition_summaries"][0]["state_name"],
            "patch_lifecycle",
        )
        self.assertTrue(payload["actual"]["activation_cut_present"])

    def test_operational_world_core_positive_keeps_source_authority(self) -> None:
        payload = _run_helper("run", "E2E-SURF-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(payload["actual"]["operational_root"], "world-core")
        self.assertEqual(payload["actual"]["state_names"], ["participant"])
        self.assertEqual(payload["actual"]["source_authority"], ".mir")

    def test_operational_membership_chat_positive_runs_admission_and_elaboration(self) -> None:
        payload = _run_helper("run", "E2E-SURF-03")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["required_checks"],
            ["parse", "role_admission", "elaboration"],
        )
        self.assertEqual(payload["actual"]["accepted_authority_check_count"], 1)
        self.assertEqual(payload["actual"]["remote_request_count"], 1)
        self.assertEqual(payload["actual"]["publication_count"], 1)
        self.assertIn("auto_publish", payload["actual"]["generated_edge_kinds"])

    def test_operational_membership_chat_negative_rejects_missing_grant(self) -> None:
        payload = _run_helper("run", "E2E-SURF-04")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            [
                "role_claim_without_capability_grant",
                "generated_failure_not_declared",
            ],
        )

    def test_operational_sugoroku_positive_generates_visible_write_communication(self) -> None:
        payload = _run_helper("run", "E2E-SURF-05")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(payload["actual"]["remote_request_count"], 1)
        self.assertEqual(payload["actual"]["publication_count"], 1)
        self.assertIn("auto_publish", payload["actual"]["generated_edge_kinds"])

    def test_operational_portal_private_negative_rejects_auto_observe(self) -> None:
        payload = _run_helper("run", "E2E-SURF-08")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["private_field_auto_publish_rejected"],
        )

    def test_operational_two_shard_negative_requires_failure_row(self) -> None:
        payload = _run_helper("run", "E2E-SURF-10")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["generated_failure_not_declared"],
        )

    def test_operational_gradient_positive_generates_observe_rows(self) -> None:
        payload = _run_helper("run", "E2E-SURF-11")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertEqual(payload["actual"]["observation_count"], 1)
        self.assertIn("auto_observe", payload["actual"]["generated_edge_kinds"])

    def test_devtools_positive_has_required_panels(self) -> None:
        payload = _run_helper("run", "DEV-01")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["accepted"])
        self.assertTrue(payload["actual"]["all_required_panels_present"])
        self.assertEqual(payload["actual"]["panel_count"], 7)
        self.assertIn("source_spans", payload["actual"]["panel_ids"])
        self.assertTrue(payload["actual"]["indexed_state_semantic_backing"])
        self.assertTrue(payload["actual"]["stage_acceptance"]["indexed_state"])
        self.assertEqual(payload["actual"]["patch_hotplug_verdict_kind"], "accepted")
        self.assertFalse(payload["actual"]["final_public_viewer_frozen"])
        self.assertNotIn("raw_parse_report", payload)
        self.assertTrue(payload["verification_report"]["redacted"])
        self.assertFalse(
            payload["verification_report"]["contains_sensitive_devtools_material"]
        )
        self.assertFalse(
            _contains_sensitive_devtools_material(payload["verification_report"])
        )

    def test_devtools_private_negative_keeps_panels_and_reports_diagnostic(self) -> None:
        payload = _run_helper("run", "DEV-02")

        self.assertTrue(payload["accepted"])
        self.assertFalse(payload["actual"]["accepted"])
        self.assertIn(
            "private_field_auto_publish_rejected",
            payload["actual"]["diagnostic_codes"],
        )
        self.assertTrue(payload["actual"]["all_required_panels_present"])
        self.assertTrue(payload["actual"]["indexed_state_semantic_backing"])
        self.assertFalse(payload["actual"]["raw_private_payload_exposed"])
        self.assertNotIn("raw_parse_report", payload)
        self.assertTrue(payload["verification_report"]["redacted"])
        self.assertFalse(
            payload["verification_report"]["contains_sensitive_devtools_material"]
        )
        self.assertFalse(
            _contains_sensitive_devtools_material(payload["verification_report"])
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(len(payload["passed"]), 48)


if __name__ == "__main__":
    unittest.main()
