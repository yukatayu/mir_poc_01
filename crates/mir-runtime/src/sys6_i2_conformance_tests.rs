use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

use crate::sys6_i2_conformance::{
    I2ConformanceFalsifier, I2ConformanceInput, I2ConformanceReport, I2ConformanceStatus,
    run_i2_conformance,
};

const TOY_SOURCE: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const TOY_SOURCE_TEXT: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const PLUS_TWO_PATCH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir";
const OWNER_RMW_PATCH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir";
const SELECTED_OW1_SOURCE: &str =
    "samples/clean-near-end/mirrorea-i2-conformance/ow1-selected-owner-designated.mir";
const SELECTED_OW1_SOURCE_TEXT: &str = include_str!(
    "../../../samples/clean-near-end/mirrorea-i2-conformance/ow1-selected-owner-designated.mir"
);
const M10_ACCEPTED_IMPLEMENTATION_CUT: &str = "23f5a8130334bf0c8516d51e9dcea38b92f50db1";
const ALLOWED_EVIDENCE_CLASSES: &[&str] = &[
    "lean-proved",
    "lean-stated",
    "model-checked-bounded",
    "runtime-monitored",
    "intentionally-deferred",
];

const REQUIRED_I2_ROWS: &[&str] = &[
    "i2.ordinary_source_authority",
    "i2.checked_global_core_identity",
    "i2.core_to_locus_artifacts",
    "i2.generated_communication_complete",
    "i2.actual_dispatch_over_generated_edges",
    "i2.st_ow_selected_correspondence",
    "i2.owner_data_race_freedom_selected_backend",
    "i2.no_hidden_communication",
    "i2.no_direct_remote_store",
    "i2.no_source_free_authority_mint",
    "i2.no_source_free_state_mint",
    "i2.failure_containment",
    "i2.visibility_redaction_preserved",
    "i2.relation_projection_coherence",
    "i2.semantic_presentation_fallback_separation",
    "i2.designated_evaluator_non_reexecution",
    "i2.source_core_artifact_trace_correspondence",
    "i2.save_restore_consistent_local_cut",
    "i2.patch_lifecycle_checked",
    "i2.observer_safe_devtools",
    "i2.projection_determinism",
    "i2.non_claims_and_lifecycle_boundaries",
];

#[test]
fn i2_profile_accepts_exact_bounded_source_first_inventory() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_eq!(report.status(), I2ConformanceStatus::Accepted);
    assert_eq!(value["schema_version"], "mirrorea-i2-conformance-report-v0");
    assert_eq!(
        value["profile_name"],
        "mirrorea-i2-systems-foundation-finite"
    );
    assert_eq!(value["status"], "accepted");
    assert_eq!(value["source_authority"], "ordinary_mir_source");
    assert_eq!(value["profile_scope"], "bounded-finite-i2");
    assert_eq!(value["public_api_or_wire_contract"], false);
    assert_eq!(value["final_public_api_frozen"], false);
    assert_eq!(value["public_wire_frozen"], false);
    assert_json_tree_lacks_key(&value, "m10_release_identity_reused");
    assert_json_tree_lacks_key(&value, "cli_falsifier_surface_available");
    assert_json_tree_lacks_key(&value, "partial_projection_accepted");
    assert_source_first_flags_are_actual_provenance_not_self_attestation(&value);
    assert_bounded_implementation_source_fingerprint(&value);
    assert_json_array_contains_all(
        &value["loci"],
        &["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    );
    assert_exact_row_inventory(&value);
    assert_pass_rows(&value, REQUIRED_I2_ROWS);
    assert_evidence_class_universe(&value);
    assert_rows_have_positive_and_falsifier_evidence(&value);
    assert_controls_cross_join_executed_evidence(&value);
    assert_accepted_negative_evidence_is_actual_control_not_unexecuted_detected(&value);
    assert_representative_controls(&value);
    assert_owner_preservation_subclaim(&value, "pass");
    assert_selected_backend_actions_are_typed_successes(&value);
    assert_offline_cut_corruption_evidence_is_actual(&value);
    assert_observer_sensitive_evidence_binds_actual_marker_candidate_and_redacted_output(&value);
    assert_row_specific_anchors(&value);
    assert_required_provenance_anchor_contracts_are_unique(&value);
    assert_observer_safe_report(&value);
}

#[test]
fn i2_profile_cross_joins_provenance_refs_to_actual_inventories() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_exact_row_inventory(&value);
    assert_nonempty_array(
        &value["inventories"]["checked_program_identity_refs"],
        "checked identity inventory",
    );
    assert_nonempty_array(&value["inventories"]["core_refs"], "Core ref inventory");
    assert_nonempty_array(
        &value["inventories"]["artifact_refs"],
        "artifact ref inventory",
    );
    assert_nonempty_array(
        &value["inventories"]["communication_edge_refs"],
        "communication edge inventory",
    );
    assert_nonempty_array(
        &value["inventories"]["runtime_occurrence_refs"],
        "runtime occurrence inventory",
    );
    assert_rows_cross_join_inventories(&value);

    let dispatch = row(&value, "i2.actual_dispatch_over_generated_edges");
    assert_nonempty_string(&dispatch["edge_ref"], "dispatch edge");
    assert_nonempty_string(&dispatch["request_identity"], "dispatch request identity");
    assert_nonempty_string(
        &dispatch["dispatch_occurrence_ref"],
        "dispatch occurrence ref",
    );
    assert_nonempty_string(
        &dispatch["receive_occurrence_ref"],
        "receive occurrence ref",
    );
    assert_nonempty_string(&dispatch["serve_occurrence_ref"], "serve occurrence ref");
    assert_eq!(
        dispatch["authority_source"], "source_admission_m9",
        "dispatch row must not treat transport, queue position, or reply receipt as authority"
    );
    assert_observer_safe_report(&value);
}

#[test]
fn i2_profile_rejects_projection_tamper_with_validator_and_unchanged_snapshots() {
    for case in [
        (
            I2ConformanceFalsifier::RemoveGeneratedCommunicationEdge {
                operation_id: "attack".to_string(),
                edge_kind: "owner-request".to_string(),
            },
            &["i2.generated_communication_complete"][..],
            "projection_candidate",
            "MissingGeneratedCommunicationEdge",
        ),
        (
            I2ConformanceFalsifier::InsertNonDerivedCommunicationEdge {
                edge_ref: "manual-debug-edge".to_string(),
                operation_id: "attack".to_string(),
                edge_kind: "owner-request".to_string(),
                from_locus: "ParticipantA".to_string(),
                to_locus: "ViewerC".to_string(),
            },
            &["i2.no_hidden_communication"][..],
            "communication_candidate",
            "NonDerivedCommunicationEdge",
        ),
        (
            I2ConformanceFalsifier::MoveOwnerOperation {
                operation_id: "attack".to_string(),
                from_locus: "WorldAuthority".to_string(),
                to_locus: "ParticipantA".to_string(),
            },
            &["i2.core_to_locus_artifacts"][..],
            "artifact_candidate",
            "OwnerOperationMoved",
        ),
        (
            I2ConformanceFalsifier::BreakSourceMap {
                operation_id: "attack".to_string(),
                artifact_ref: "artifact:tampered:attack".to_string(),
            },
            &["i2.source_core_artifact_trace_correspondence"][..],
            "provenance_candidate",
            "SourceMapMismatch",
        ),
    ] {
        let report = run_i2_conformance(canonical_input().with_test_falsifier(case.0))
            .expect("I2 conformance should return a typed rejection report");
        let value = report_json(&report);

        assert_rejection_integrity(&report, &value, case.1, case.2, case.3);
        if case.3 == "OwnerOperationMoved" {
            assert_owner_preservation_subclaim(&value, "fail");
        }
    }
}

#[test]
fn i2_profile_rejects_runtime_shortcuts_without_mutating_semantic_authority_state() {
    for case in [
        (
            I2ConformanceFalsifier::AdmitSourceFreeAuthority {
                principal: "self".to_string(),
                locus: "WorldAuthority".to_string(),
                operation_id: "attack".to_string(),
            },
            &["i2.no_source_free_authority_mint"][..],
            "runtime_candidate",
            "SourceFreeAuthorityMint",
        ),
        (
            I2ConformanceFalsifier::MutateRemoteStore {
                locus: "WorldAuthority".to_string(),
                state: "source_free_shadow_state".to_string(),
                index: "target".to_string(),
                field: "hp".to_string(),
                value: 999,
            },
            &["i2.no_source_free_state_mint"][..],
            "runtime_candidate",
            "SourceFreeStateMint",
        ),
        (
            I2ConformanceFalsifier::MutateRemoteStore {
                locus: "ParticipantA".to_string(),
                state: "avatar".to_string(),
                index: "target".to_string(),
                field: "hp".to_string(),
                value: 999,
            },
            &["i2.no_direct_remote_store"][..],
            "runtime_candidate",
            "DirectRemoteStoreMutation",
        ),
    ] {
        let report = run_i2_conformance(canonical_input().with_test_falsifier(case.0))
            .expect("I2 conformance should return a typed rejection report");
        let value = report_json(&report);

        assert_rejection_integrity(&report, &value, case.1, case.2, case.3);
        if matches!(
            case.3,
            "SourceFreeAuthorityMint" | "SourceFreeStateMint" | "DirectRemoteStoreMutation"
        ) {
            assert_runtime_boundary_rejection_preserves_underlying_state(&value, case.3);
        }
    }
}

#[test]
fn i2_profile_rejects_wrong_diagnostic_substitution_on_bound_controls() {
    for (control_id, substituted_diagnostic, expected_rows, expected_diagnostic) in [
        (
            "i2-evidence:runtime-authority-override-detected",
            "DirectRemoteStoreMutation",
            &[
                "i2.ordinary_source_authority",
                "i2.no_source_free_authority_mint",
            ][..],
            "SourceFreeAuthorityMint",
        ),
        (
            "i2-evidence:runtime-cross-locus-store-detected",
            "SourceFreeAuthorityMint",
            &["i2.no_direct_remote_store", "i2.no_source_free_state_mint"][..],
            "DirectRemoteStoreMutation",
        ),
        (
            "i2-evidence:runtime-source-free-state-mint-detected",
            "DirectRemoteStoreMutation",
            &["i2.no_source_free_state_mint"][..],
            "SourceFreeStateMint",
        ),
        (
            "i2-evidence:offline-cut-corruption-detected",
            "DirectRemoteStoreMutation",
            &["i2.save_restore_consistent_local_cut"][..],
            "OfflineCutCorruption",
        ),
    ] {
        let report = run_i2_conformance(canonical_input().with_test_falsifier(
            I2ConformanceFalsifier::SubstituteRuntimeControlDiagnostic {
                control_id: control_id.to_string(),
                diagnostic_code: substituted_diagnostic.to_string(),
            },
        ))
        .expect("diagnostic substitution falsifier should return a typed report");
        let value = report_json(&report);

        assert_fail_closed_rejection_basic(
            &report,
            &value,
            expected_rows,
            "control_diagnostic_candidate",
            "ControlDiagnosticMismatch",
        );
        assert_control_diagnostic_substitution_details(
            &value,
            control_id,
            expected_diagnostic,
            substituted_diagnostic,
        );
    }
}

#[test]
fn i2_profile_rejects_failed_bound_evidence_for_exact_bound_rows() {
    for (evidence_id, expected_rows) in [
        (
            "i2-evidence:selected-backend-positive",
            &[
                "i2.st_ow_selected_correspondence",
                "i2.owner_data_race_freedom_selected_backend",
            ][..],
        ),
        (
            "i2-evidence:offline-cut-corruption-detected",
            &["i2.save_restore_consistent_local_cut"][..],
        ),
    ] {
        let report = run_i2_conformance(canonical_input().with_test_falsifier(
            I2ConformanceFalsifier::FailBoundEvidence {
                evidence_id: evidence_id.to_string(),
            },
        ))
        .expect("failed bound evidence falsifier should return a typed report");
        let value = report_json(&report);

        assert_fail_closed_rejection_basic(
            &report,
            &value,
            expected_rows,
            "executed_evidence_candidate",
            "BoundEvidenceNotExecuted",
        );
        assert_bound_evidence_failure_details(&value, evidence_id, expected_rows);
    }
}

#[test]
fn i2_profile_rejects_manual_route_or_interface_admission() {
    let report = run_i2_conformance(canonical_input().with_test_falsifier(
        I2ConformanceFalsifier::AdmitManualRouteOrInterface {
            operation_id: "attack".to_string(),
            from_locus: "ParticipantA".to_string(),
            to_locus: "WorldAuthority".to_string(),
        },
    ))
    .expect("manual route/interface admission falsifier should return a typed report");
    let value = report_json(&report);

    assert_fail_closed_rejection_basic(
        &report,
        &value,
        &["i2.ordinary_source_authority", "i2.no_hidden_communication"],
        "source_first_admission_candidate",
        "ManualRouteOrInterfaceAdmitted",
    );
    let candidate = &value["rejection"]["manual_route_or_interface_candidate"];
    assert_eq!(
        candidate["source"], "actual-manual-route-or-interface-candidate",
        "manual route admission must be rejected as an actual sealed-admission candidate: {candidate:#}"
    );
    assert_eq!(candidate["manual_route_or_interface_admitted"], true);
    assert_eq!(candidate["accepted"], false);
    assert_eq!(candidate["mutation_applied"], false);
    assert_nonempty_string(
        &candidate["candidate_ref"],
        "manual route/interface candidate ref",
    );
    assert_nonempty_string(
        &candidate["producer_invocation_ref"],
        "manual route/interface producer invocation ref",
    );
    assert_eq!(
        candidate["semantic_state_before"], candidate["semantic_state_after"],
        "manual route/interface candidate must not mutate semantic state"
    );
}

#[test]
fn source_content_mutation_at_same_logical_name_changes_all_i2_identities_or_rejects() {
    let source = TempMirSource::new("same-path-mutation", TOY_SOURCE_TEXT);
    let original_path = source.path.clone();
    let base = run_i2_conformance(canonical_input_for_source(source.path.clone()))
        .expect("base inline source conformance should run");
    let base_value = report_json(&base);
    assert_eq!(base.status(), I2ConformanceStatus::Accepted);

    let mutated_source = TOY_SOURCE_TEXT.replace(
        "participant_input[self].focus + 1",
        "participant_input[self].focus + 2",
    );
    assert_ne!(
        mutated_source, TOY_SOURCE_TEXT,
        "test mutation must actually change source content at the same logical path"
    );
    source.overwrite(&mutated_source);
    assert_eq!(
        source.path, original_path,
        "source-content mutation control must reuse the exact same host path and logical basename"
    );
    assert_eq!(
        source.path.file_name().and_then(|name| name.to_str()),
        Some("cli-source.mir")
    );
    let mutated = run_i2_conformance(canonical_input_for_source(source.path.clone()))
        .expect("mutated inline source conformance should return a typed report");
    let mutated_value = report_json(&mutated);

    match mutated.status() {
        I2ConformanceStatus::Accepted => {
            let base_checked = checked_identity_ref(&base_value);
            let mutated_checked = checked_identity_ref(&mutated_value);
            let base_artifacts = artifact_identity_ref(&base_value);
            let mutated_artifacts = artifact_identity_ref(&mutated_value);

            assert_ne!(
                base_checked, mutated_checked,
                "checked Core identity must include source content, not only logical path"
            );
            assert_ne!(
                base_artifacts, mutated_artifacts,
                "per-locus artifact identity must change when same-logical-name source content changes"
            );
            assert_ne!(
                base_value["i2_manifest_identity_ref"], mutated_value["i2_manifest_identity_ref"],
                "I2 manifest identity must change when same-logical-name source content changes"
            );
        }
        I2ConformanceStatus::Rejected => {
            assert_eq!(mutated_value["status"], "rejected");
            assert_nonempty_string(
                &mutated_value["typed_rejection"]["diagnostic_code"],
                "typed source mutation rejection diagnostic",
            );
            assert_eq!(mutated_value["source_authority"], "ordinary_mir_source");
            assert_observer_safe_report(&mutated_value);
        }
    }

    assert_ne!(
        base_value["i2_manifest_identity_ref"].as_str(),
        Some(M10_ACCEPTED_IMPLEMENTATION_CUT),
        "I2 conformance identity must not reuse the exact accepted M10 implementation cut"
    );
    assert_ne!(
        mutated_value["i2_manifest_identity_ref"].as_str(),
        Some(M10_ACCEPTED_IMPLEMENTATION_CUT),
        "mutated I2 conformance identity must not reuse the exact accepted M10 implementation cut"
    );
}

#[test]
fn same_content_same_logical_name_different_host_path_keeps_i2_manifest_identity() {
    let first_source = TempMirSource::new("same-content-a", TOY_SOURCE_TEXT);
    let second_source = TempMirSource::new("same-content-b", TOY_SOURCE_TEXT);
    assert_ne!(
        first_source.path, second_source.path,
        "control must use different host paths"
    );
    assert_eq!(
        first_source.path.file_name().and_then(|name| name.to_str()),
        Some("cli-source.mir")
    );
    assert_eq!(
        second_source
            .path
            .file_name()
            .and_then(|name| name.to_str()),
        Some("cli-source.mir")
    );

    let first = run_i2_conformance(canonical_input_for_source(first_source.path.clone()))
        .expect("first same-content source conformance should run");
    let second = run_i2_conformance(canonical_input_for_source(second_source.path.clone()))
        .expect("second same-content source conformance should run");
    let first_value = report_json(&first);
    let second_value = report_json(&second);

    assert_eq!(first.status(), I2ConformanceStatus::Accepted);
    assert_eq!(second.status(), I2ConformanceStatus::Accepted);
    assert_eq!(
        first_value["i2_manifest_identity_ref"], second_value["i2_manifest_identity_ref"],
        "same content at the same logical basename must not derive I2 identity from host temp directory"
    );
    assert_observer_safe_report(&first_value);
    assert_observer_safe_report(&second_value);
}

#[test]
fn selected_ow1_eligibility_does_not_depend_on_literal_s_locus_name() {
    let renamed = renamed_selected_owner_source("Owner");
    let selected_source = TempMirSource::new("ow1-owner-worker", &renamed);
    let report = run_i2_conformance(canonical_input_for_selected_source(
        selected_source.path.clone(),
    ))
    .expect("renamed Owner selected OW1 source should run");
    let value = report_json(&report);
    let row = row(&value, "i2.st_ow_selected_correspondence");

    assert_eq!(
        report.status(),
        I2ConformanceStatus::Accepted,
        "OW1 eligibility must depend on the generated single owner-worker topology, not literal locus name S"
    );
    assert_eq!(row["status"], "pass");
    assert_eq!(row["ow1_backend_telemetry"]["sole_worker_locus"], "Owner");
    assert_eq!(row["ow1_backend_telemetry"]["worker_owned_m8"], true);
    assert_eq!(row["selected_ow1_parse"]["locus_count"], 4);
    assert_selected_ow1_projection_counts(row);
    assert_selected_ow1_row_uses_selected_source_actual_anchor(&value, "Owner");
    assert_observer_safe_report(&value);
}

#[test]
fn i2_profile_uses_row_specific_anchors_for_non_attack_properties() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_row_specific_anchors(&value);
    assert_required_provenance_anchor_contracts_are_unique(&value);
}

#[test]
fn i2_profile_rejects_missing_required_provenance_anchor_for_exact_target_row() {
    let target_row = "i2.designated_evaluator_non_reexecution";
    let report = run_i2_conformance(canonical_input().with_test_falsifier(
        I2ConformanceFalsifier::RemoveRequiredProvenanceAnchor {
            row_id: target_row.to_string(),
        },
    ))
    .expect("I2 conformance should return a typed rejection report");
    let value = report_json(&report);

    assert_rejection_integrity(
        &report,
        &value,
        &[target_row],
        "provenance_anchor_inventory",
        "MissingRequiredProvenanceAnchor",
    );
    assert_exact_failed_rows(&value, &[target_row]);

    let row = row(&value, target_row);
    assert_eq!(
        row["provenance_anchor_ref"], "missing-or-ambiguous-required-provenance",
        "missing required anchor must fail closed instead of falling back to an unrelated workflow anchor"
    );
    for cleared_field in [
        "checked_program_identity_ref",
        "core_ref",
        "artifact_ref",
        "edge_ref",
        "request_identity",
        "dispatch_occurrence_ref",
        "receive_occurrence_ref",
        "serve_occurrence_ref",
        "locus_program_ref",
    ] {
        assert_absent_or_empty_string(row, cleared_field);
    }
}

#[test]
fn i2_profile_source_first_and_no_expected_json_flags_are_proven_not_literal_false_flags() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_source_first_flags_are_actual_provenance_not_self_attestation(&value);
}

#[test]
fn i2_profile_observer_negative_evidence_binds_actual_marker_candidate_and_redacted_output() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_observer_sensitive_evidence_binds_actual_marker_candidate_and_redacted_output(&value);
}

#[test]
fn i2_profile_selected_backend_action_outcomes_are_typed_successes() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_selected_backend_actions_are_typed_successes(&value);
}

#[test]
fn i2_profile_separates_owner_data_race_runtime_row_from_bounded_model_ordering_evidence() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);

    assert_eq!(
        row(&value, "i2.owner_data_race_freedom_selected_backend")["evidence_class"],
        "runtime-monitored",
        "owner data-race row is a runtime backend observation; bounded ordering evidence is separate"
    );

    let executed = executed_evidence_inventory(&value);
    let runtime_evidence = executed
        .get("i2-evidence:selected-backend-positive")
        .expect("selected backend runtime evidence should exist");
    assert_eq!(runtime_evidence["evidence_class"], "runtime-monitored");
    assert_executed_evidence_binds_row(
        runtime_evidence,
        "i2.owner_data_race_freedom_selected_backend",
    );

    let model_evidence = executed
        .get("i2-evidence:model-required-edge-detected")
        .expect("bounded model ordering evidence should exist");
    assert_eq!(model_evidence["evidence_class"], "model-checked-bounded");
    assert_eq!(
        model_evidence["diagnostic_code"],
        "MissingOwnerRequestServeEdge"
    );
    assert_executed_evidence_binds_row(
        model_evidence,
        "i2.owner_data_race_freedom_selected_backend",
    );
}

#[test]
fn i2_profile_selected_ow1_row_reports_actual_backend_telemetry_and_full_toy_residual() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);
    let selected_row = row(&value, "i2.st_ow_selected_correspondence");

    assert_eq!(selected_row["status"], "pass");
    assert_eq!(selected_row["scope"], "selected-one-owner-worker-fragment");
    assert_eq!(selected_row["evidence_class"], "runtime-monitored");
    assert_eq!(selected_row["selected_ow1_source"], SELECTED_OW1_SOURCE);
    assert_eq!(selected_row["selected_ow1_parse"]["locus_count"], 4);
    assert_selected_ow1_projection_counts(selected_row);
    assert_eq!(
        selected_row["st_backend_telemetry"]["runtime_profile"],
        "ST"
    );
    assert_eq!(
        selected_row["ow1_backend_telemetry"]["runtime_profile"],
        "OW1"
    );
    assert_eq!(
        selected_row["ow1_backend_telemetry"]["sole_worker_locus"],
        "S"
    );
    assert_eq!(
        selected_row["ow1_backend_telemetry"]["worker_owned_m8"],
        true
    );
    assert_eq!(selected_row["ow1_backend_telemetry"]["mailbox_fifo"], true);
    assert_selected_ow1_row_uses_selected_source_actual_anchor(&value, "S");
    assert_nonempty_array(
        &selected_row["st_backend_telemetry"]["lifecycle_refs"],
        "ST lifecycle refs",
    );
    assert_nonempty_array(
        &selected_row["ow1_backend_telemetry"]["lifecycle_refs"],
        "OW1 lifecycle refs",
    );
    assert_selected_backend_actions_are_typed_successes(&value);
    assert_same_mailbox_fifo_control(selected_row);
    assert_eq!(
        selected_row["st_semantic_digest"],
        selected_row["ow1_semantic_digest"]
    );
    assert_eq!(selected_row["four_locus_ow1_workflow_claimed"], false);
    assert_eq!(
        selected_row["full_toy_ow1_residual"]["diagnostic_code"],
        "BackendIneligible"
    );
    assert_eq!(
        selected_row["full_toy_ow1_residual"]["reason"],
        "MultipleCombinedOwnerSourceOwnerLoci"
    );
    assert_eq!(selected_row["full_toy_ow1_residual"]["profile"], "OW1");
    assert_eq!(
        selected_row["full_toy_ow1_residual"]["admission_phase"],
        "canonical_local_admission"
    );
    assert_eq!(
        selected_row["full_toy_ow1_residual"]["typed_admission_reason"]["code"],
        "MultipleCombinedOwnerSourceOwnerLoci"
    );
    assert_nonempty_array(
        &selected_row["full_toy_ow1_residual"]["typed_admission_reason"]["owner_loci"],
        "full toy OW1 owner loci",
    );
    assert_nonempty_array(
        &selected_row["full_toy_ow1_residual"]["typed_admission_reason"]["source_owner_loci"],
        "full toy OW1 source-owner loci",
    );
    assert_typed_owner_residual_distinguishes_owner_loci(&selected_row["full_toy_ow1_residual"]);
    assert_eq!(
        selected_row["full_toy_ow1_residual"]["mutated_state"],
        false
    );

    assert_model_fingerprint_covers_execution_backend_if_retained(row(
        &value,
        "i2.owner_data_race_freedom_selected_backend",
    ));
}

#[test]
fn i2_profile_rejects_selected_backend_divergence_candidates_with_typed_details() {
    for (falsifier, diagnostic, differing_fields) in [
        (
            I2ConformanceFalsifier::DivergeSelectedBackendTypedResult,
            "SelectedBackendTypedResultDivergence",
            &["st_typed_result_ref", "ow1_typed_result_ref"][..],
        ),
        (
            I2ConformanceFalsifier::DivergeSelectedBackendState,
            "SelectedBackendStateDivergence",
            &["st_state_digest", "ow1_state_digest"][..],
        ),
        (
            I2ConformanceFalsifier::DivergeSelectedBackendFrontier,
            "SelectedBackendFrontierDivergence",
            &["st_frontier_ref", "ow1_frontier_ref"][..],
        ),
        (
            I2ConformanceFalsifier::DivergeSelectedBackendTrace,
            "SelectedBackendTraceDivergence",
            &["st_trace_digest", "ow1_trace_digest"][..],
        ),
    ] {
        let report = run_i2_conformance(canonical_input().with_test_falsifier(falsifier))
            .expect("selected backend divergence falsifier should return a typed report");
        let value = report_json(&report);

        assert_rejection_integrity(
            &report,
            &value,
            &["i2.st_ow_selected_correspondence"],
            "selected_backend_candidate",
            diagnostic,
        );
        assert_selected_backend_divergence_details(&value, diagnostic, differing_fields);
    }
}

#[test]
fn i2_profile_rejects_offline_cut_corruption_as_cut_candidate_not_remote_store() {
    let report = run_i2_conformance(
        canonical_input().with_test_falsifier(I2ConformanceFalsifier::CorruptOfflineCut),
    )
    .expect("offline cut corruption falsifier should return a typed report");
    let value = report_json(&report);

    assert_rejection_integrity(
        &report,
        &value,
        &["i2.save_restore_consistent_local_cut"],
        "offline_cut_candidate",
        "OfflineCutCorruption",
    );
    assert_offline_cut_rejection_uses_actual_cut_restore_candidate(&value);
    assert_offline_cut_corruption_evidence_is_actual(&value);
}

#[test]
fn i2_profile_rejects_lifecycle_boundary_overclaim_candidate_without_runtime_mutation() {
    let report = run_i2_conformance(
        canonical_input().with_test_falsifier(I2ConformanceFalsifier::FlipLifecycleBoundaryClaim),
    )
    .expect("lifecycle overclaim falsifier should return a typed report");
    let value = report_json(&report);

    assert_rejection_integrity(
        &report,
        &value,
        &["i2.non_claims_and_lifecycle_boundaries"],
        "lifecycle_boundary_candidate",
        "LifecycleBoundaryOverclaim",
    );
    assert_exact_failed_rows(&value, &["i2.non_claims_and_lifecycle_boundaries"]);

    let candidate = &value["rejection"]["lifecycle_boundary_candidate"];
    assert_eq!(
        candidate["source"], "actual-lifecycle-boundary-candidate",
        "lifecycle overclaim must mutate a typed lifecycle candidate, not a static non_claims hash: {candidate:#}"
    );
    assert_nonempty_string(
        &candidate["candidate_ref"],
        "lifecycle overclaim candidate ref",
    );
    assert_nonempty_string(
        &candidate["producer_invocation_ref"],
        "lifecycle overclaim producer invocation ref",
    );
    assert_eq!(candidate["i2_exit_accepted"], true);
    assert_eq!(candidate["public_transport_claim"], true);
    assert_eq!(candidate["accepted"], false);
    assert_eq!(candidate["mutation_applied"], false);
}

#[test]
fn i2_profile_records_exact_lifecycle_pre_acceptance_state() {
    let report = run_i2_conformance(canonical_input()).expect("I2 conformance should run");
    let value = report_json(&report);
    let lifecycle = &value["lifecycle_state"];

    assert_eq!(lifecycle["broad_i1_exit_accepted"], false);
    assert_eq!(lifecycle["i2_entry_accepted"], false);
    assert_eq!(lifecycle["i2_exit_accepted"], false);
    assert_eq!(lifecycle["sys7_goal_active"], false);
    assert_eq!(lifecycle["i3_program_active"], false);
    assert_eq!(lifecycle["public_transport_claim"], false);
    assert_eq!(lifecycle["real_transport_selected"], false);
    assert_eq!(lifecycle["production_deployment_claim"], false);
    assert_eq!(
        row(&value, "i2.non_claims_and_lifecycle_boundaries")["status"],
        "pass"
    );
    assert_json_array_contains_all(
        &value["non_claims"],
        &[
            "real transport",
            "public ABI or wire freeze",
            "durable distributed save/load",
            "general metatheory",
            "arbitrary scheduler fairness",
            "arbitrary relation DAG theorem",
            "four-locus OW1 whole-workflow execution",
            "broad I1 exit",
            "I2 lifecycle exit",
            "I3 activation",
        ],
    );
    assert_lifecycle_boundary_evidence_is_actual_typed_candidate(&value);
}

#[test]
fn i2_conformance_dependency_direction_is_downstream_only() {
    let runtime_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let conformance = fs::read_to_string(runtime_src.join("sys6_i2_conformance.rs"))
        .expect("SYS-6 conformance implementation source should exist once RED turns green");

    assert!(conformance.contains("sys5_local_workflow"));
    assert!(conformance.contains("sys5_local_slice"));
    assert!(conformance.contains("sys2_bounded_model"));
    assert!(!conformance.contains("m10_reference_system"));

    for relative in [
        "sys2_bounded_model.rs",
        "sys2_execution_backend.rs",
        "sys3_projection/mod.rs",
        "sys3_projection/model.rs",
        "sys3_projection/validate.rs",
        "sys4_dispatch.rs",
        "sys5_local_slice.rs",
        "sys5_local_workflow.rs",
    ] {
        let source = fs::read_to_string(runtime_src.join(relative))
            .unwrap_or_else(|error| panic!("could not read {relative}: {error}"));
        assert!(
            !source.contains("sys6_i2_conformance") && !source.contains("I2Conformance"),
            "lower SYS layer must not depend on SYS-6 conformance aggregator: {relative}"
        );
    }
}

#[test]
fn i2_conformance_declares_divergence_and_offline_cut_test_hook_schema() {
    let runtime_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let conformance = fs::read_to_string(runtime_src.join("sys6_i2_conformance.rs"))
        .expect("SYS-6 conformance implementation source should exist once RED turns green");

    for needle in [
        "DivergeSelectedBackendTypedResult",
        "DivergeSelectedBackendState",
        "DivergeSelectedBackendFrontier",
        "DivergeSelectedBackendTrace",
        "CorruptOfflineCut",
        "FlipLifecycleBoundaryClaim",
        "SubstituteRuntimeControlDiagnostic",
        "FailBoundEvidence",
        "AdmitManualRouteOrInterface",
        "SelectedBackendTypedResultDivergence",
        "SelectedBackendStateDivergence",
        "SelectedBackendFrontierDivergence",
        "SelectedBackendTraceDivergence",
        "OfflineCutCorruption",
        "LifecycleBoundaryOverclaim",
        "ControlDiagnosticMismatch",
        "BoundEvidenceNotExecuted",
        "ManualRouteOrInterfaceAdmitted",
        "selected_backend_candidate",
        "offline_cut_candidate",
        "lifecycle_boundary_candidate",
        "control_diagnostic_candidate",
        "executed_evidence_candidate",
        "manual_route_or_interface_candidate",
        "st_typed_result_ref",
        "ow1_typed_result_ref",
        "st_state_digest",
        "ow1_state_digest",
        "st_frontier_ref",
        "ow1_frontier_ref",
        "st_trace_digest",
        "ow1_trace_digest",
        "runtime_endpoint_attempt_ref",
        "offline_cut_ref",
        "corruption_kind",
    ] {
        assert!(
            conformance.contains(needle),
            "SYS-6 test-only falsifier schema must declare `{needle}` so selected ST/OW divergence, runtime endpoint remote-store attempts, and offline cut corruption are typed and separately rejectable"
        );
    }
}

#[test]
fn i2_conformance_source_does_not_use_const_attempted_or_invented_property_refs() {
    let runtime_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let conformance = fs::read_to_string(runtime_src.join("sys6_i2_conformance.rs"))
        .expect("SYS-6 conformance implementation source should exist once RED turns green");
    let normalized = conformance
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();

    assert!(
        !conformance.contains("i2-property-"),
        "SYS-6 conformance must not fabricate row/property anchors with i2-property-* refs; anchors must come from actual workflow/project/model provenance inventories"
    );
    for denied in [
        "constfnattempted(&self)->bool{true}",
        "constfnattempted()->bool{true}",
    ] {
        assert!(
            !normalized.contains(denied),
            "completed generated dispatch cannot be justified by a const attempted()->true helper; action outcomes must bind actual typed receipts"
        );
    }
    for denied in [
        "control_identity(\"observer-before\", &\"safe\")",
        "control_identity(\"observer-after\", &\"redacted\")",
        "control_identity(\"lifecycle-before\", &\"preacceptance\")",
        "control_identity(\"lifecycle-after\", &non_claims())",
    ] {
        assert!(
            !conformance.contains(denied),
            "SYS-6 evidence must hash actual typed candidates, not static lifecycle/observer constants: {denied}"
        );
    }
}

#[test]
fn i2_profile_manifest_identity_is_deterministic_and_not_m10_cut() {
    let first = run_i2_conformance(canonical_input()).expect("first I2 conformance should run");
    let second = run_i2_conformance(canonical_input()).expect("second I2 conformance should run");
    let first_value = report_json(&first);

    assert_eq!(
        first.render_compact(),
        second.render_compact(),
        "I2 conformance output must be deterministic for identical source, patches, and finite profile"
    );
    assert_nonempty_string(
        &first_value["i2_manifest_identity_ref"],
        "I2 manifest identity",
    );
    assert!(
        first_value["i2_manifest_identity_ref"]
            .as_str()
            .is_some_and(|text| text.starts_with("i2-conformance-sha256-v1:")),
        "I2 manifest identity must be I2 namespaced: {}",
        first_value["i2_manifest_identity_ref"]
    );
    assert_ne!(
        first_value["i2_manifest_identity_ref"].as_str(),
        Some(M10_ACCEPTED_IMPLEMENTATION_CUT)
    );
    assert_bounded_implementation_source_fingerprint(&first_value);
    assert_json_array_contains_all(
        &first_value["evidence_classes"],
        &["runtime-monitored", "model-checked-bounded"],
    );
    assert_json_array_lacks_string(&first_value["evidence_classes"], "lean-proved");
    assert_observer_safe_report(&first_value);
}

fn canonical_input() -> I2ConformanceInput {
    canonical_input_for_source(repo_path(TOY_SOURCE))
}

fn canonical_input_for_source(source_path: PathBuf) -> I2ConformanceInput {
    I2ConformanceInput::source_path(source_path)
        .with_patch_path(repo_path(PLUS_TWO_PATCH))
        .with_patch_path(repo_path(OWNER_RMW_PATCH))
        .with_selected_ow1_source_path(repo_path(SELECTED_OW1_SOURCE))
}

fn canonical_input_for_selected_source(selected_source_path: PathBuf) -> I2ConformanceInput {
    I2ConformanceInput::source_path(repo_path(TOY_SOURCE))
        .with_patch_path(repo_path(PLUS_TWO_PATCH))
        .with_patch_path(repo_path(OWNER_RMW_PATCH))
        .with_selected_ow1_source_path(selected_source_path)
}

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mir-runtime")
        .join(relative)
}

fn renamed_selected_owner_source(worker_locus: &str) -> String {
    SELECTED_OW1_SOURCE_TEXT
        .replace("locus S\n", &format!("locus {worker_locus}\n"))
        .replace(" at S", &format!(" at {worker_locus}"))
        .replace("at S {", &format!("at {worker_locus} {{"))
}

struct TempMirSource {
    dir: PathBuf,
    path: PathBuf,
}

impl TempMirSource {
    fn new(label: &str, source_text: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after Unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "mir-i2-conformance-{}-{nanos}-{label}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("temporary Mir source directory should be creatable");
        let path = dir.join("cli-source.mir");
        fs::write(&path, source_text).expect("temporary Mir source should be writable");
        Self { dir, path }
    }

    fn overwrite(&self, source_text: &str) {
        fs::write(&self.path, source_text).expect("temporary Mir source should be overwritable");
    }
}

impl Drop for TempMirSource {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn report_json(report: &impl serde::Serialize) -> Value {
    serde_json::to_value(report).expect("I2 conformance report should serialize")
}

fn checked_identity_ref(value: &Value) -> String {
    if let Some(identity) = value
        .get("checked_program_identity_ref")
        .and_then(Value::as_str)
        .filter(|identity| !identity.is_empty())
    {
        return identity.to_string();
    }
    if let Some(inventory) = value["inventories"]["checked_program_identity_refs"].as_array() {
        let mut identities = inventory
            .iter()
            .filter_map(Value::as_str)
            .filter(|identity| !identity.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        identities.sort();
        identities.dedup();
        if !identities.is_empty() {
            return identities.join("|");
        }
    }
    let mut identities = value["rows"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("report should carry rows when extracting checked identity: {value:#}")
        })
        .iter()
        .filter_map(|row| row["checked_program_identity_ref"].as_str())
        .filter(|identity| !identity.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    identities.sort();
    identities.dedup();
    assert!(
        !identities.is_empty(),
        "report must expose checked program identity refs for source-content identity checks: {value:#}"
    );
    identities.join("|")
}

fn artifact_identity_ref(value: &Value) -> String {
    if let Some(identity) = value
        .get("artifact_inventory_digest")
        .and_then(Value::as_str)
        .filter(|identity| !identity.is_empty())
    {
        return identity.to_string();
    }
    if let Some(inventory) = value["inventories"]["artifact_identity_refs"].as_array() {
        let mut identities = inventory
            .iter()
            .filter_map(Value::as_str)
            .filter(|identity| !identity.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        identities.sort();
        identities.dedup();
        if !identities.is_empty() {
            return identities.join("|");
        }
    }
    let mut refs = value["rows"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("report should carry rows when extracting artifact identity: {value:#}")
        })
        .iter()
        .filter_map(|row| row["artifact_ref"].as_str())
        .filter(|identity| !identity.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    refs.sort();
    refs.dedup();
    assert!(
        !refs.is_empty(),
        "report must expose per-locus artifact identity refs for source-content identity checks: {value:#}"
    );
    refs.join("|")
}

fn row<'a>(value: &'a Value, id: &str) -> &'a Value {
    value["rows"]
        .as_array()
        .expect("rows should be an array")
        .iter()
        .find(|row| row["id"] == id)
        .unwrap_or_else(|| panic!("missing row {id}: {value:#}"))
}

fn assert_exact_row_inventory(value: &Value) {
    let rows = value["rows"].as_array().expect("rows should be an array");
    assert_eq!(
        rows.len(),
        REQUIRED_I2_ROWS.len(),
        "I2 profile row inventory must be exact, not open-ended: {rows:#?}"
    );
    let ids = rows
        .iter()
        .map(|row| row["id"].as_str().expect("row id should be a string"))
        .collect::<Vec<_>>();
    let actual = ids.iter().copied().collect::<BTreeSet<_>>();
    let expected = REQUIRED_I2_ROWS.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "I2 row inventory must be exact");
    let unique = ids.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), ids.len(), "I2 row IDs must be unique");

    for row in rows {
        let id = row["id"]
            .as_str()
            .unwrap_or_else(|| panic!("row id should be a string: {row:#}"));
        let status = row["status"]
            .as_str()
            .unwrap_or_else(|| panic!("row status should be a string: {row:#}"));
        assert!(
            matches!(status, "pass" | "fail"),
            "I2 rows must not use N-A/skip/defer/waiver status: {row:#}"
        );
        for denied in [
            "n/a", "N/A", "skip", "skipped", "defer", "deferred", "waiver",
        ] {
            assert!(
                !serde_json::to_string(row)
                    .expect("row serializes")
                    .contains(denied),
                "I2 row must not contain status waivers or placeholders `{denied}`: {row:#}"
            );
        }
        assert_eq!(
            row["scope"].as_str(),
            Some(expected_scope(id)),
            "I2 row scope must be exact, bounded, and nonempty: {row:#}"
        );
        assert!(
            !row["scope"]
                .as_str()
                .expect("scope already checked")
                .contains("arbitrary"),
            "I2 row scope must stay bounded, not arbitrary: {row:#}"
        );
        assert!(
            ALLOWED_EVIDENCE_CLASSES.contains(&row["evidence_class"].as_str().unwrap_or("")),
            "I2 row evidence_class must be one of the exact assurance classes: {row:#}"
        );
    }
}

fn expected_scope(id: &str) -> &'static str {
    match id {
        "i2.ordinary_source_authority" => "primary-source-first",
        "i2.checked_global_core_identity" => "checked-finite-core",
        "i2.core_to_locus_artifacts" => "four-locus-projection",
        "i2.generated_communication_complete" => "generated-edge-inventory",
        "i2.actual_dispatch_over_generated_edges" => "st-local-dispatch",
        "i2.st_ow_selected_correspondence" => "selected-one-owner-worker-fragment",
        "i2.owner_data_race_freedom_selected_backend" => "selected-one-owner-worker-fragment",
        "i2.no_hidden_communication" => "generated-edge-only",
        "i2.no_direct_remote_store" => "locus-endpoint-boundary",
        "i2.no_source_free_authority_mint" => "m9-admitted-source-authority",
        "i2.no_source_free_state_mint" => "source-admitted-state-only",
        "i2.failure_containment" => "typed-failure-before-mutation",
        "i2.visibility_redaction_preserved" => "observer-safe-redaction",
        "i2.relation_projection_coherence" => "finite-relation-fallback",
        "i2.semantic_presentation_fallback_separation" => "presentation-gap-nonmutation",
        "i2.designated_evaluator_non_reexecution" => "designated-result-delivery",
        "i2.source_core_artifact_trace_correspondence" => "source-core-artifact-occurrence",
        "i2.save_restore_consistent_local_cut" => "st-local-cut-restore",
        "i2.patch_lifecycle_checked" => "checked-patch-lifecycle",
        "i2.observer_safe_devtools" => "reference-only-observer-view",
        "i2.projection_determinism" => "same-source-repeat",
        "i2.non_claims_and_lifecycle_boundaries" => "provisional-internal-boundary",
        other => panic!("unexpected I2 row id while checking scope: {other}"),
    }
}

fn assert_pass_rows(value: &Value, expected_ids: &[&str]) {
    for id in expected_ids {
        assert_eq!(row(value, id)["status"], "pass", "row {id} should pass");
    }
}

fn assert_failed_rows(value: &Value, expected_ids: &[&str]) {
    for id in expected_ids {
        assert_eq!(row(value, id)["status"], "fail", "row {id} should fail");
    }
}

fn assert_exact_failed_rows(value: &Value, expected_ids: &[&str]) {
    let expected = expected_ids.iter().copied().collect::<BTreeSet<_>>();
    let actual = value["rows"]
        .as_array()
        .expect("rows should be an array")
        .iter()
        .filter(|row| row["status"] == "fail")
        .map(|row| {
            row["id"]
                .as_str()
                .unwrap_or_else(|| panic!("failed row should expose id: {row:#}"))
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual, expected,
        "rejection should fail exactly the expected row set"
    );
}

fn assert_rows_have_positive_and_falsifier_evidence(value: &Value) {
    let executed = executed_evidence_inventory(value);
    for row in value["rows"].as_array().expect("rows should be an array") {
        let id = row["id"].as_str().expect("row id");
        assert_nonempty_array(&row["positive_evidence_refs"], id);
        assert_nonempty_array(&row["falsifier_evidence_refs"], id);
        for field in ["positive_evidence_refs", "falsifier_evidence_refs"] {
            for evidence_ref in row[field]
                .as_array()
                .expect("evidence refs should be array")
            {
                let text = evidence_ref
                    .as_str()
                    .unwrap_or_else(|| panic!("{id}/{field} entry must be string: {row:#}"));
                let evidence = executed
                    .get(text)
                    .unwrap_or_else(|| panic!("{id}/{field} ref must cross-join executed_evidence inventory, not be an arbitrary string: {text}"));
                assert_executed_evidence_binds_row(evidence, id);
                assert!(
                    !text.contains("TODO") && !text.contains("placeholder"),
                    "{id}/{field} must carry real evidence refs, not placeholder text: {text}"
                );
                assert!(
                    ALLOWED_EVIDENCE_CLASSES
                        .contains(&evidence["evidence_class"].as_str().unwrap_or("")),
                    "{id}/{field} executed evidence class must be exact: {evidence:#}"
                );
                assert_nonempty_string(&evidence["kind"], "executed evidence kind");
                assert_eq!(
                    evidence["executed"], true,
                    "{id}/{field} evidence must be tied to an executed producer/control invocation: {evidence:#}"
                );
                match field {
                    "positive_evidence_refs" => assert_eq!(
                        evidence["outcome"], "observed",
                        "{id}/{field} evidence must carry the exact positive outcome observed: {evidence:#}"
                    ),
                    "falsifier_evidence_refs" => assert_eq!(
                        evidence["outcome"], "detected",
                        "{id}/{field} evidence must carry the exact falsifier outcome detected: {evidence:#}"
                    ),
                    _ => unreachable!(),
                }
                assert!(
                    evidence["produced_by"].as_str().is_some_and(|producer| {
                        matches!(
                            producer,
                            "sys2-model"
                                | "sys3-projection"
                                | "sys4-dispatch"
                                | "sys5-workflow"
                                | "sys6-validator"
                        )
                    }),
                    "{id}/{field} evidence must name an actual bounded producer: {evidence:#}"
                );
            }
        }
    }
}

fn assert_accepted_negative_evidence_is_actual_control_not_unexecuted_detected(value: &Value) {
    assert_eq!(value["status"], "accepted");
    let executed = executed_evidence_inventory(value);
    for row in value["rows"].as_array().expect("rows should be an array") {
        let id = row["id"].as_str().expect("row id");
        for evidence_ref in row["falsifier_evidence_refs"]
            .as_array()
            .expect("falsifier evidence refs should be an array")
        {
            let evidence_ref = evidence_ref
                .as_str()
                .unwrap_or_else(|| panic!("{id}/falsifier evidence ref should be string"));
            let evidence = executed.get(evidence_ref).unwrap_or_else(|| {
                panic!(
                    "{id}/falsifier evidence ref must cross-join executed evidence: {evidence_ref}"
                )
            });
            assert_executed_evidence_binds_row(evidence, id);
            assert_nonempty_string(&evidence["control_ref"], "negative evidence control ref");
            assert_nonempty_string(
                &evidence["producer_invocation_ref"],
                "negative evidence producer invocation ref",
            );
            let outcome = evidence["outcome"].as_str();
            let kind = evidence["kind"].as_str().unwrap_or("");
            let claims_execution =
                matches!(outcome, Some("detected" | "executed")) || kind.contains("executed");
            if claims_execution {
                assert_eq!(
                    evidence["executed"], true,
                    "accepted report must not mark unexecuted falsifier controls as executed/detected: {evidence:#}"
                );
                assert_nonempty_string(
                    &evidence["candidate_identity_before"],
                    "executed negative evidence candidate before",
                );
                assert_nonempty_string(
                    &evidence["candidate_identity_after"],
                    "executed negative evidence candidate after",
                );
                assert_nonempty_string(
                    &evidence["diagnostic_code"],
                    "executed negative evidence diagnostic",
                );
            } else {
                assert_eq!(
                    evidence["executed"], false,
                    "registered but unrun negative controls must explicitly say executed=false: {evidence:#}"
                );
                assert!(
                    matches!(
                        outcome,
                        Some("control-registered" | "available-control" | "not-run")
                    ),
                    "unexecuted negative controls must not claim detected/executed outcome: {evidence:#}"
                );
            }
        }
    }
}

fn assert_source_first_flags_are_actual_provenance_not_self_attestation(value: &Value) {
    for legacy_flag in [
        "runtime_reparsed_source",
        "name_dispatch_used",
        "expected_json_lookup_used",
        "accepted_runtime_core_or_authority_injection",
        "accepted_runtime_state_injection",
    ] {
        assert!(
            value.get(legacy_flag).is_none(),
            "SYS-6 I2 JSON must not expose literal legacy boolean `{legacy_flag}`; use source-first causal provenance instead: {value:#}"
        );
    }
    for nested_literal_flag in ["fixture_name_dispatch", "expected_json_lookup"] {
        assert_json_tree_lacks_key(value, nested_literal_flag);
    }

    let provenance = source_first_causal_provenance_by_kind(value);
    assert_source_bound_causal_entry(
        provenance.get("ordinary-source-bound").unwrap_or_else(|| {
            panic!("source-first evidence must include ordinary-source-bound causal entry")
        }),
        "sys5-workflow",
    );
    assert_checked_project_causal_entry(
        value,
        provenance
            .get("checked-project")
            .unwrap_or_else(|| panic!("source-first evidence must include checked-project entry")),
    );
    assert_sealed_admission_causal_entry(
        provenance
            .get("sealed-admission")
            .unwrap_or_else(|| panic!("source-first evidence must include sealed-admission entry")),
    );
    assert_generated_dispatch_causal_entry(
        value,
        provenance.get("generated-dispatch").unwrap_or_else(|| {
            panic!("source-first evidence must include generated-dispatch entry")
        }),
    );
    assert_unknown_action_admission_rejection(
        provenance
            .get("unknown-action-admission-rejection")
            .unwrap_or_else(|| {
                panic!(
                    "source-first/no-expected-result evidence must include a real unknown-action admission rejection"
                )
            }),
    );
}

fn source_first_causal_provenance_by_kind(value: &Value) -> BTreeMap<String, &Value> {
    let entries = value["inventories"]["source_first_causal_provenance"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.source_first_causal_provenance: {value:#}"));
    let mut by_kind = BTreeMap::new();
    for entry in entries {
        let id = entry["id"]
            .as_str()
            .unwrap_or_else(|| panic!("source-first causal entry must have id: {entry:#}"));
        let kind = entry["kind"]
            .as_str()
            .unwrap_or_else(|| panic!("source-first causal entry must name kind: {entry:#}"));
        assert!(!id.is_empty(), "source-first causal id must be nonempty");
        assert!(
            !id.starts_with("i2-property-") && !id.starts_with("literal-false"),
            "source-first causal id must not be invented/self-attested: {entry:#}"
        );
        assert_json_tree_lacks_invented_property_ref(entry, id);
        assert_executed_evidence_binds_row(entry, "i2.ordinary_source_authority");
        assert_nonempty_string(
            &entry["producer_invocation_ref"],
            &format!("{kind} producer invocation ref"),
        );
        assert_nonempty_string(
            &entry["typed_producer_ref"],
            &format!("{kind} typed producer ref"),
        );
        let previous = by_kind.insert(kind.to_string(), entry);
        assert!(
            previous.is_none(),
            "source-first causal entries must be unique by kind, duplicate: {kind}"
        );
    }
    by_kind
}

fn assert_source_bound_causal_entry(entry: &Value, expected_producer: &str) {
    assert_eq!(
        entry["source"], "actual-source-bound-causal-inventory",
        "ordinary source authority must be proven by actual source-bound causal inventory: {entry:#}"
    );
    assert_eq!(entry["produced_by"], expected_producer);
    assert_nonempty_string(
        &entry["source_content_identity_ref"],
        "source-bound causal source content identity",
    );
    assert_nonempty_string(
        &entry["logical_source_ref"],
        "source-bound causal logical source ref",
    );
    assert_nonempty_string(
        &entry["source_span_ref"],
        "source-bound causal source span ref",
    );
}

fn assert_checked_project_causal_entry(value: &Value, entry: &Value) {
    assert_eq!(
        entry["source"], "actual-checked-project",
        "checked Core/project identity must be proven by actual checked project provenance: {entry:#}"
    );
    assert!(matches!(
        entry["produced_by"].as_str(),
        Some("sys3-projection" | "sys5-workflow")
    ));
    let checked_ref = entry["checked_program_identity_ref"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("checked-project entry must carry checked_program_identity_ref: {entry:#}")
        });
    assert!(
        inventory_set(value, "checked_program_identity_refs").contains(checked_ref),
        "checked-project checked_program_identity_ref must cross-join checked inventory: {checked_ref}"
    );
}

fn assert_sealed_admission_causal_entry(entry: &Value) {
    assert_eq!(
        entry["source"], "actual-sealed-admission",
        "source-first evidence must bind the checked project to a sealed admission, not a fixture name: {entry:#}"
    );
    assert_eq!(entry["produced_by"], "sys5-workflow");
    assert_nonempty_string(&entry["sealed_admission_ref"], "sealed admission ref");
    assert_eq!(
        entry["manual_route_or_interface_admitted"], false,
        "sealed admission must reject manual route/interface injection"
    );
    assert_eq!(
        entry["runtime_core_or_authority_injection_admitted"], false,
        "sealed admission must reject runtime Core/authority injection"
    );
    assert_eq!(
        entry["runtime_state_injection_admitted"], false,
        "sealed admission must reject runtime state injection"
    );
}

fn assert_generated_dispatch_causal_entry(value: &Value, entry: &Value) {
    assert_eq!(
        entry["source"], "actual-generated-dispatch",
        "source-first/no-name-dispatch claim must bind actual dispatch provenance: {entry:#}"
    );
    assert_eq!(entry["produced_by"], "sys4-dispatch");
    assert_eq!(entry["routing_source"], "generated_communication_plan");
    assert_nonempty_string(
        &entry["communication_plan_ref"],
        "generated dispatch communication plan ref",
    );
    let edge = entry["edge_ref"]
        .as_str()
        .unwrap_or_else(|| panic!("generated-dispatch edge_ref must be string: {entry:#}"));
    let occurrence = entry["dispatch_occurrence_ref"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("generated-dispatch dispatch_occurrence_ref must be string: {entry:#}")
        });
    assert!(
        inventory_set(value, "communication_edge_refs").contains(edge),
        "generated dispatch edge_ref must cross-join actual communication edge inventory: {edge}"
    );
    assert!(
        inventory_set(value, "runtime_occurrence_refs").contains(occurrence),
        "generated dispatch occurrence must cross-join actual runtime occurrence inventory: {occurrence}"
    );
}

fn assert_unknown_action_admission_rejection(entry: &Value) {
    assert_eq!(
        entry["source"], "actual-unknown-action-admission-candidate",
        "source-first/no-expected-result evidence must use an actual rejected unknown action candidate: {entry:#}"
    );
    assert!(matches!(
        entry["produced_by"].as_str(),
        Some("sys4-dispatch" | "sys5-workflow")
    ));
    assert_eq!(entry["candidate_kind"], "unknown-source-action");
    assert_eq!(entry["accepted"], false);
    assert_eq!(entry["state_unchanged"], true);
    assert_eq!(entry["mutation_applied"], false);
    assert!(matches!(
        entry["diagnostic_code"].as_str(),
        Some("UnknownSourceAction" | "SourceActionNotAdmitted")
    ));
    assert_nonempty_string(
        &entry["candidate_ref"],
        "unknown action rejected candidate ref",
    );
    assert_nonempty_string(
        &entry["state_before_ref"],
        "unknown action state before ref",
    );
    assert_nonempty_string(&entry["state_after_ref"], "unknown action state after ref");
    assert_eq!(entry["state_before_ref"], entry["state_after_ref"]);
}

fn assert_observer_sensitive_evidence_binds_actual_marker_candidate_and_redacted_output(
    value: &Value,
) {
    let executed = executed_evidence_inventory(value);
    let evidence = executed
        .get("i2-evidence:observer-sensitive-scan-detected")
        .unwrap_or_else(|| {
            panic!(
                "accepted I2 report must include observer-sensitive marker/redaction control evidence: {value:#}"
            )
        });
    assert_executed_evidence_binds_row(evidence, "i2.visibility_redaction_preserved");
    assert_eq!(evidence["executed"], true);
    assert_eq!(evidence["outcome"], "detected");
    assert_eq!(evidence["diagnostic_code"], "ObserverSensitiveIdentifier");
    assert_eq!(
        evidence["candidate_source"], "actual-marker-bearing-report-candidate",
        "observer evidence must hash the actual marker-bearing report candidate, not a static safe constant: {evidence:#}"
    );
    assert_eq!(
        evidence["redacted_output_source"], "actual-redacted-serialized-output",
        "observer evidence must hash the actual redacted serialized output, not a static redacted constant: {evidence:#}"
    );
    assert_eq!(evidence["marker_present_in_candidate"], true);
    assert_eq!(evidence["marker_absent_after_redaction"], true);
    assert_nonempty_string(
        &evidence["marker_bearing_report_candidate_ref"],
        "observer marker-bearing report candidate ref",
    );
    assert_nonempty_string(
        &evidence["redacted_serialized_output_ref"],
        "observer redacted serialized output ref",
    );
    assert_eq!(
        evidence["candidate_identity_before"], evidence["marker_bearing_report_candidate_ref"],
        "observer candidate_identity_before must equal the hash returned from the actual marker-bearing candidate"
    );
    assert_eq!(
        evidence["candidate_identity_after"], evidence["redacted_serialized_output_ref"],
        "observer candidate_identity_after must equal the hash returned from the actual redacted serialized output"
    );
    assert_ne!(
        evidence["candidate_identity_before"], evidence["candidate_identity_after"],
        "observer evidence must bind a real before/after redaction delta"
    );
}

fn assert_evidence_class_universe(value: &Value) {
    for class in value["evidence_classes"]
        .as_array()
        .expect("top-level evidence_classes should be an array")
    {
        let class = class
            .as_str()
            .unwrap_or_else(|| panic!("evidence class should be a string: {value:#}"));
        assert!(
            ALLOWED_EVIDENCE_CLASSES.contains(&class),
            "top-level evidence class must be one of the exact assurance classes: {class}"
        );
    }
}

fn assert_controls_cross_join_executed_evidence(value: &Value) {
    let executed = executed_evidence_inventory(value);
    for row in value["rows"].as_array().expect("rows should be an array") {
        let row_id = row["id"].as_str().unwrap_or("<unknown>");
        let controls = row["controls"]
            .as_array()
            .unwrap_or_else(|| panic!("row {row_id} should carry explicit controls: {row:#}"));
        for control in controls {
            let control_id = control["id"]
                .as_str()
                .unwrap_or_else(|| panic!("row {row_id} control must have id: {control:#}"));
            let evidence_ref = control["evidence_ref"].as_str().unwrap_or_else(|| {
                panic!("row {row_id} control {control_id} must link executed evidence: {control:#}")
            });
            let evidence = executed.get(evidence_ref).unwrap_or_else(|| {
                panic!(
                    "row {row_id} control {control_id} evidence_ref must cross-join executed_evidence inventory: {evidence_ref}"
                )
            });
            assert_executed_evidence_binds_row(evidence, row_id);
            assert!(
                control["outcome"]
                    .as_str()
                    .is_some_and(|outcome| { evidence["outcome"].as_str() == Some(outcome) }),
                "row {row_id} control {control_id} outcome must match executed evidence: control={control:#} evidence={evidence:#}"
            );
        }
    }
}

fn assert_representative_controls(value: &Value) {
    assert_controls_include(
        row(value, "i2.failure_containment"),
        &["missing-consumer-capability-fail-closed"],
    );
    assert_controls_include(
        row(value, "i2.visibility_redaction_preserved"),
        &["observer-safe-redaction-no-secret-material"],
    );
    assert_controls_include(
        row(value, "i2.relation_projection_coherence"),
        &["relation-primary-fallback-fresh-reacquire"],
    );
    assert_controls_include(
        row(value, "i2.semantic_presentation_fallback_separation"),
        &["presentation-gap-does-not-mutate-semantic-lineage"],
    );
    assert_controls_include(
        row(value, "i2.designated_evaluator_non_reexecution"),
        &["viewer-consumes-versioned-designated-result-without-re-evaluation"],
    );
    assert_controls_include(
        row(value, "i2.save_restore_consistent_local_cut"),
        &["local-cut-restore-retains-artifact-mailbox-and-trace-frontier"],
    );
    assert_controls_include(
        row(value, "i2.patch_lifecycle_checked"),
        &[
            "designated-plus-two-patch-accepted",
            "owner-rmw-change-patch-rejected",
        ],
    );
}

fn assert_controls_include(row: &Value, expected_ids: &[&str]) {
    let controls = row["controls"].as_array().unwrap_or_else(|| {
        panic!(
            "row {} should carry representative controls",
            row["id"].as_str().unwrap_or("<unknown>")
        )
    });
    for expected in expected_ids {
        assert!(
            controls
                .iter()
                .any(|control| control["id"].as_str() == Some(expected)),
            "row {} missing control {expected}: {row:#}",
            row["id"].as_str().unwrap_or("<unknown>")
        );
    }
}

fn assert_owner_preservation_subclaim(value: &Value, expected_status: &str) {
    let artifact_row = row(value, "i2.core_to_locus_artifacts");
    assert_controls_include(artifact_row, &["owner-preservation-worldauthority-attack"]);
    let subclaims = artifact_row["subclaims"].as_array().unwrap_or_else(|| {
        panic!(
            "core_to_locus_artifacts row must carry owner-preservation subclaims: {artifact_row:#}"
        )
    });
    let subclaim = subclaims
        .iter()
        .find(|subclaim| subclaim["id"] == "owner-preservation-worldauthority-attack")
        .unwrap_or_else(|| panic!("missing owner-preservation subclaim: {artifact_row:#}"));
    assert_eq!(
        subclaim["status"], expected_status,
        "move-owner falsifier must fail the explicit owner-preservation subclaim"
    );
    assert_eq!(subclaim["operation_id"], "attack");
    assert_eq!(subclaim["expected_owner_locus"], "WorldAuthority");
    if expected_status == "pass" {
        assert_eq!(subclaim["observed_owner_locus"], "WorldAuthority");
    } else {
        assert_ne!(
            subclaim["observed_owner_locus"], "WorldAuthority",
            "failing owner-preservation subclaim must expose the moved owner locus without changing authority state"
        );
    }
    assert_nonempty_string(
        &subclaim["evidence_ref"],
        "owner-preservation subclaim evidence ref",
    );
}

fn assert_row_specific_anchors(value: &Value) {
    assert_actual_provenance_anchor(
        value,
        "i2.relation_projection_coherence",
        "relation-fallback-lineage",
        &["workflow", "project"],
        &["sys5-workflow", "sys3-projection"],
        "causal_segment_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.designated_evaluator_non_reexecution",
        "designated-result-delivery",
        &["workflow", "project"],
        &["sys5-workflow", "sys3-projection"],
        "causal_segment_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.save_restore_consistent_local_cut",
        "save-cut-lifecycle",
        &["workflow"],
        &["sys5-workflow"],
        "lifecycle_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.patch_lifecycle_checked",
        "patch-lifecycle",
        &["workflow"],
        &["sys5-workflow"],
        "lifecycle_ref",
    );
    assert_actual_provenance_anchor(
        value,
        "i2.owner_data_race_freedom_selected_backend",
        "st-ow-refinement-model",
        &["model"],
        &["sys2-model"],
        "model_ref",
    );

    for row_id in [
        "i2.save_restore_consistent_local_cut",
        "i2.patch_lifecycle_checked",
        "i2.owner_data_race_freedom_selected_backend",
    ] {
        let row = row(value, row_id);
        assert_optional_actual_or_typed_non_applicable_ref(value, row, "core_ref", "core_refs");
        assert_optional_actual_or_typed_non_applicable_ref(
            value,
            row,
            "artifact_ref",
            "artifact_refs",
        );
        assert_optional_actual_or_typed_non_applicable_ref(
            value,
            row,
            "edge_ref",
            "communication_edge_refs",
        );
        assert_optional_actual_or_typed_non_applicable_ref(
            value,
            row,
            "request_identity",
            "request_identity_refs",
        );
    }
}

fn assert_required_provenance_anchor_contracts_are_unique(value: &Value) {
    let anchors = value["inventories"]["provenance_anchors"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.provenance_anchors: {value:#}"));
    for (row_id, domain, kind, source) in required_anchor_contracts() {
        let matches = anchors
            .iter()
            .filter(|anchor| {
                anchor["domain"].as_str() == Some(domain)
                    && anchor["kind"].as_str() == Some(kind)
                    && anchor["source"].as_str() == Some(source)
                    && evidence_entry_binds_row(anchor, row_id)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            matches.len(),
            1,
            "row {row_id} must have exactly one required provenance anchor for domain={domain}, kind={kind}, source={source}; zero means missing/domain mismatch, >1 means duplicate: {matches:#?}"
        );
        let bound_anchor_id = matches[0]["id"].as_str().unwrap_or_else(|| {
            panic!(
                "required provenance anchor for {row_id} must expose id: {:#}",
                matches[0]
            )
        });
        assert_eq!(
            row(value, row_id)["provenance_anchor_ref"],
            bound_anchor_id,
            "row {row_id} must point at its unique required provenance anchor"
        );
    }
}

fn required_anchor_contracts() -> [(&'static str, &'static str, &'static str, &'static str); 7] {
    [
        (
            "i2.relation_projection_coherence",
            "workflow",
            "relation-fallback-lineage",
            "actual",
        ),
        (
            "i2.semantic_presentation_fallback_separation",
            "workflow",
            "relation-fallback-lineage",
            "actual",
        ),
        (
            "i2.designated_evaluator_non_reexecution",
            "workflow",
            "designated-result-delivery",
            "actual",
        ),
        (
            "i2.save_restore_consistent_local_cut",
            "workflow",
            "save-cut-lifecycle",
            "actual",
        ),
        (
            "i2.patch_lifecycle_checked",
            "workflow",
            "patch-lifecycle",
            "actual",
        ),
        (
            "i2.st_ow_selected_correspondence",
            "selected-backend",
            "selected-st-ow-correspondence",
            "actual-selected-ow1-source",
        ),
        (
            "i2.owner_data_race_freedom_selected_backend",
            "model",
            "st-ow-refinement-model",
            "actual",
        ),
    ]
}

fn evidence_entry_binds_row(entry: &Value, row_id: &str) -> bool {
    entry["row_ids"]
        .as_array()
        .is_some_and(|row_ids| row_ids.iter().any(|id| id.as_str() == Some(row_id)))
        || entry["property_ids"]
            .as_array()
            .is_some_and(|property_ids| property_ids.iter().any(|id| id.as_str() == Some(row_id)))
}

fn assert_actual_provenance_anchor(
    value: &Value,
    row_id: &str,
    expected_kind: &str,
    allowed_domains: &[&str],
    allowed_producers: &[&str],
    required_anchor_field: &str,
) {
    let row = row(value, row_id);
    assert_json_tree_lacks_invented_property_ref(row, row_id);
    let anchor_ref = row["provenance_anchor_ref"]
        .as_str()
        .unwrap_or_else(|| panic!("row {row_id} must expose provenance_anchor_ref: {row:#}"));
    assert!(
        !anchor_ref.is_empty() && !anchor_ref.starts_with("i2-property-"),
        "row {row_id} provenance_anchor_ref must be actual, not invented: {anchor_ref}"
    );
    let anchors = provenance_anchor_inventory(value);
    let anchor = anchors.get(anchor_ref).unwrap_or_else(|| {
        panic!("row {row_id} provenance_anchor_ref must cross-join inventories.provenance_anchors: {anchor_ref}")
    });
    assert_eq!(anchor["id"], anchor_ref);
    assert_eq!(
        anchor["kind"], expected_kind,
        "row {row_id} must bind a property-specific actual provenance kind"
    );
    assert!(
        allowed_domains
            .iter()
            .any(|domain| anchor["domain"].as_str() == Some(domain)),
        "row {row_id} provenance anchor must come from an actual accepted domain {allowed_domains:?}: {anchor:#}"
    );
    assert!(
        allowed_producers
            .iter()
            .any(|producer| anchor["produced_by"].as_str() == Some(producer)),
        "row {row_id} provenance anchor must name the actual producing subsystem {allowed_producers:?}: {anchor:#}"
    );
    assert_eq!(
        anchor["source"], "actual",
        "row {row_id} provenance anchor must be actual, not synthetic"
    );
    assert_executed_evidence_binds_row(anchor, row_id);
    assert_nonempty_string(
        &anchor[required_anchor_field],
        &format!("{row_id} {required_anchor_field}"),
    );
    assert_json_tree_lacks_invented_property_ref(anchor, anchor_ref);
}

fn provenance_anchor_inventory(value: &Value) -> BTreeMap<String, &Value> {
    let entries = value["inventories"]["provenance_anchors"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.provenance_anchors: {value:#}"));
    let mut by_id = BTreeMap::new();
    for entry in entries {
        let id = entry["id"]
            .as_str()
            .unwrap_or_else(|| panic!("provenance anchor entry should have id: {entry:#}"));
        assert!(
            !id.starts_with("i2-property-"),
            "provenance anchor IDs must not use invented i2-property-* refs: {entry:#}"
        );
        assert_nonempty_string(&entry["domain"], "provenance anchor domain");
        assert_nonempty_string(&entry["produced_by"], "provenance anchor producer");
        assert_nonempty_string(&entry["kind"], "provenance anchor kind");
        assert_has_binding_field(entry);
        assert_json_tree_lacks_invented_property_ref(entry, id);
        let previous = by_id.insert(id.to_string(), entry);
        assert!(
            previous.is_none(),
            "provenance anchor IDs must be unique, duplicate id: {id}"
        );
    }
    by_id
}

fn assert_optional_actual_or_typed_non_applicable_ref(
    value: &Value,
    row: &Value,
    field: &str,
    inventory: &str,
) {
    let row_id = row["id"].as_str().unwrap_or("<unknown>");
    let Some(candidate) = row.get(field) else {
        return;
    };
    if candidate.is_null() {
        return;
    }
    if let Some(text) = candidate.as_str() {
        if text.is_empty() {
            return;
        }
        assert!(
            !text.starts_with("i2-property-"),
            "row {row_id} field {field} must not use invented i2-property-* refs: {text}"
        );
        let inventory = inventory_set(value, inventory);
        assert!(
            inventory.contains(text),
            "row {row_id} field {field} must be absent, typed non-applicable, or cross-join an actual inventory entry: {text}"
        );
        return;
    }
    if let Some(object) = candidate.as_object() {
        assert_eq!(
            object.get("applicability").and_then(Value::as_str),
            Some("not-applicable"),
            "row {row_id} field {field} object must be typed non-applicable when no actual ref exists: {candidate:#}"
        );
        assert_nonempty_string(
            object.get("reason").unwrap_or(&Value::Null),
            &format!("{row_id} {field} non-applicable reason"),
        );
        return;
    }
    panic!(
        "row {row_id} field {field} must be absent, typed non-applicable, or an actual inventory ref: {candidate:#}"
    );
}

fn assert_selected_backend_actions_are_typed_successes(value: &Value) {
    let row = row(value, "i2.st_ow_selected_correspondence");
    for field in ["st_backend_telemetry", "ow1_backend_telemetry"] {
        let telemetry = &row[field];
        assert_eq!(
            telemetry["all_actions_succeeded"], true,
            "{field} must report all selected generated actions as typed successes"
        );
        let outcomes = telemetry["action_outcomes"]
            .as_array()
            .unwrap_or_else(|| panic!("{field}.action_outcomes must be an array: {telemetry:#}"));
        assert!(
            outcomes.len() >= 3,
            "{field}.action_outcomes must include the selected owner RMW, designated publish, and consume actions: {telemetry:#}"
        );
        for outcome in outcomes {
            assert!(
                outcome.as_object().is_some(),
                "{field}.action_outcomes entries must be typed objects, not string classifications: {outcome:#}"
            );
            assert_nonempty_string(&outcome["action_ref"], &format!("{field} action_ref"));
            assert_eq!(
                outcome["attempted"], true,
                "{field} action must bind an actual attempted runtime action: {outcome:#}"
            );
            assert_eq!(
                outcome["completed"], true,
                "{field} action must bind an actual completed runtime action: {outcome:#}"
            );
            assert_eq!(
                outcome["status"], "typed_success",
                "{field} action must be a typed success, not a typed rejection or missing result: {outcome:#}"
            );
            assert!(
                !matches!(
                    outcome["result_kind"].as_str(),
                    Some("missing" | "rejected" | "typed_rejection")
                ),
                "{field} action result_kind must not be missing or rejected: {outcome:#}"
            );
            assert_nonempty_string(
                &outcome["typed_result_ref"],
                &format!("{field} typed_result_ref"),
            );
            assert_nonempty_string(
                &outcome["receipt_occurrence_ref"],
                &format!("{field} receipt_occurrence_ref"),
            );
            let attempted_provenance = outcome["attempted_provenance_ref"]
                .as_str()
                .unwrap_or_else(|| {
                    panic!(
                        "{field} action must expose attempted_provenance_ref from actual dispatch telemetry: {outcome:#}"
                    )
                });
            assert!(
                !attempted_provenance.is_empty()
                    && !attempted_provenance.starts_with("const:")
                    && !attempted_provenance.starts_with("i2-property-"),
                "{field} attempted_provenance_ref must not be const/invented: {outcome:#}"
            );
            assert!(
                outcome
                    .get("diagnostic_code")
                    .is_none_or(|diagnostic| diagnostic.is_null() || diagnostic == ""),
                "{field} typed success action must not carry a rejection diagnostic: {outcome:#}"
            );
        }
    }
}

fn assert_same_mailbox_fifo_control(row: &Value) {
    let telemetry = &row["ow1_backend_telemetry"];
    let fifo = &telemetry["same_mailbox_fifo_control"];
    let fifo_source = fifo["source"]
        .as_str()
        .unwrap_or_else(|| panic!("OW1 FIFO control source must be typed: {fifo:#}"));
    assert!(
        matches!(
            fifo_source,
            "actual" | "actual-source-derived-queued-owner-pair"
        ),
        "OW1 same-mailbox FIFO claim must come from actual backend telemetry, not self-attestation: {fifo:#}"
    );
    assert_eq!(
        fifo["all_actions_succeeded"], true,
        "OW1 FIFO control must be based on successful selected actions: {fifo:#}"
    );
    assert_eq!(
        fifo["same_mailbox_owner_locus"], telemetry["sole_worker_locus"],
        "OW1 FIFO control must identify the same owner-worker mailbox as the selected telemetry"
    );
    assert_eq!(
        fifo["second_enqueued_before_first_serve"], true,
        "OW1 FIFO control must exercise a real two-message same-mailbox ordering pressure case"
    );
    let request_ids = string_array(&fifo["request_ids"], "same mailbox FIFO request IDs");
    let enqueue_order = string_array(&fifo["enqueue_order"], "same mailbox FIFO enqueue order");
    let serve_order = string_array(&fifo["serve_order"], "same mailbox FIFO serve order");
    assert!(
        request_ids.len() >= 2,
        "OW1 FIFO control must include at least two requests through the same owner mailbox: {fifo:#}"
    );
    assert_eq!(
        request_ids, enqueue_order,
        "OW1 FIFO enqueue order must bind the actual request IDs"
    );
    assert_eq!(
        enqueue_order, serve_order,
        "OW1 FIFO serve order must refine enqueue order for the same owner mailbox"
    );
}

fn assert_selected_ow1_projection_counts(row: &Value) {
    let projection = &row["selected_ow1_projection"];
    assert_eq!(
        projection["count_source"], "actual-projection-inventory",
        "selected OW1 counts must be derived from the actual projected artifact/edge inventories, not stale literals: {projection:#}"
    );
    let artifact_refs = string_array(
        &projection["actual_artifact_refs"],
        "selected OW1 actual artifact refs",
    );
    let edge_refs = string_array(
        &projection["actual_generated_edge_refs"],
        "selected OW1 actual generated edge refs",
    );
    let artifact_count = projection["artifact_count"]
        .as_u64()
        .unwrap_or_else(|| panic!("selected OW1 artifact_count must be numeric: {projection:#}"));
    let generated_edge_count = projection["generated_edge_count"]
        .as_u64()
        .unwrap_or_else(|| {
            panic!("selected OW1 generated_edge_count must be numeric: {projection:#}")
        });
    assert_eq!(
        artifact_count as usize,
        artifact_refs.len(),
        "selected OW1 artifact_count must match actual_artifact_refs length"
    );
    assert_eq!(
        generated_edge_count as usize,
        edge_refs.len(),
        "selected OW1 generated_edge_count must match actual_generated_edge_refs length"
    );
    assert!(
        artifact_count >= 5,
        "selected OW1 bounded projection should include at least the original five generated artifacts; extra initializer artifacts are allowed when semantically justified: {projection:#}"
    );
    assert!(
        generated_edge_count >= 5,
        "selected OW1 bounded projection should include at least the original five generated communication edges; extra initializer edges are allowed when semantically justified: {projection:#}"
    );
}

fn assert_selected_ow1_row_uses_selected_source_actual_anchor(value: &Value, worker_locus: &str) {
    let row = row(value, "i2.st_ow_selected_correspondence");
    let anchor_ref = row["provenance_anchor_ref"].as_str().unwrap_or_else(|| {
        panic!("selected ST/OW row must expose selected OW1 provenance_anchor_ref: {row:#}")
    });
    let anchors = provenance_anchor_inventory(value);
    let anchor = anchors.get(anchor_ref).unwrap_or_else(|| {
        panic!(
            "selected ST/OW row provenance anchor must cross-join provenance inventory: {anchor_ref}"
        )
    });
    assert_eq!(
        anchor["kind"], "selected-st-ow-correspondence",
        "ST/OW row must not reuse the primary toy attack anchor"
    );
    assert_eq!(
        anchor["source"], "actual-selected-ow1-source",
        "ST/OW row anchor must come from the selected OW1 source, not the primary WorldAuthority toy"
    );
    assert_eq!(anchor["owner_worker_locus"], worker_locus);
    assert_eq!(anchor["requester_locus"], "A");
    assert_eq!(anchor["designated_evaluator_locus"], "E");
    assert_eq!(anchor["consumer_locus"], "C");
    assert_json_array_contains_all(&anchor["source_loci"], &["A", worker_locus, "E", "C"]);
    let rendered_anchor = serde_json::to_string(anchor).expect("anchor JSON should serialize");
    for primary_locus in ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"] {
        assert!(
            !rendered_anchor.contains(primary_locus),
            "selected ST/OW provenance anchor must not point at primary toy locus {primary_locus}: {anchor:#}"
        );
    }
    for field in [
        "core_ref",
        "artifact_ref",
        "edge_ref",
        "request_identity",
        "locus_program_ref",
    ] {
        let value = row[field]
            .as_str()
            .unwrap_or_else(|| panic!("selected ST/OW row {field} must be a selected-source ref"));
        for primary_locus in ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"] {
            assert!(
                !value.contains(primary_locus),
                "selected ST/OW row {field} must use selected OW1 refs/loci, not primary toy {primary_locus}: {row:#}"
            );
        }
    }
}

fn assert_selected_backend_divergence_details(
    value: &Value,
    diagnostic_code: &str,
    differing_fields: &[&str],
) {
    let details = &value["rejection"]["selected_backend_divergence"];
    assert_eq!(
        details["diagnostic_code"], diagnostic_code,
        "selected backend rejection must carry typed divergence details: {details:#}"
    );
    assert_eq!(details["source"], "actual-selected-backend-candidate");
    assert_nonempty_string(
        &details["control_ref"],
        "selected backend divergence control ref",
    );
    assert_nonempty_string(
        &details["producer_invocation_ref"],
        "selected backend divergence producer invocation ref",
    );
    for field in differing_fields {
        assert_nonempty_string(
            &details[*field],
            &format!("selected backend divergence {field}"),
        );
    }
    if differing_fields.len() == 2 {
        assert_ne!(
            details[differing_fields[0]], details[differing_fields[1]],
            "selected backend divergence details must expose the differing ST/OW1 values"
        );
    }
}

fn assert_offline_cut_rejection_uses_actual_cut_restore_candidate(value: &Value) {
    let candidate = &value["rejection"]["offline_cut_candidate"];
    assert_eq!(
        candidate["diagnostic_code"], "OfflineCutCorruption",
        "offline cut falsifier must reject through a typed cut/restore candidate: {candidate:#}"
    );
    assert_eq!(
        candidate["source"], "actual-cut-restore-control",
        "offline cut rejection must not be a hash-only synthetic candidate: {candidate:#}"
    );
    assert_nonempty_string(&candidate["cut_ref"], "offline cut candidate cut ref");
    assert_nonempty_string(
        &candidate["restore_attempt_ref"],
        "offline cut candidate restore attempt ref",
    );
    assert_eq!(
        candidate["restore_result"], "typed_rejected",
        "offline cut corruption must be fail-closed at restore validation"
    );
    assert_eq!(
        candidate["mutation_applied"], false,
        "offline cut corruption must not mutate live state"
    );
    assert_nonempty_string(
        &candidate["state_digest_before"],
        "offline cut state digest before",
    );
    assert_nonempty_string(
        &candidate["state_digest_after"],
        "offline cut state digest after",
    );
    assert_eq!(
        value["rejection"]
            .get("runtime_endpoint_attempt_ref")
            .and_then(Value::as_str),
        None,
        "offline cut corruption must be distinct from runtime endpoint remote-store mutation"
    );
}

fn assert_offline_cut_corruption_evidence_is_actual(value: &Value) {
    let executed = executed_evidence_inventory(value);
    let evidence = executed
        .get("i2-evidence:offline-cut-corruption-detected")
        .unwrap_or_else(|| {
            panic!(
                "accepted I2 report must include actual offline cut corruption control evidence: {value:#}"
            )
        });
    assert_executed_evidence_binds_row(evidence, "i2.save_restore_consistent_local_cut");
    assert_eq!(evidence["executed"], true);
    assert_eq!(evidence["outcome"], "detected");
    assert_eq!(evidence["diagnostic_code"], "OfflineCutCorruption");
    assert_nonempty_string(&evidence["control_ref"], "offline cut control ref");
    assert_nonempty_string(
        &evidence["producer_invocation_ref"],
        "offline cut producer invocation ref",
    );
    assert_nonempty_string(&evidence["offline_cut_ref"], "offline cut ref");
    assert_eq!(evidence["corruption_kind"], "OfflineCutCorruption");
    assert_nonempty_string(
        &evidence["candidate_identity_before"],
        "offline cut candidate before",
    );
    assert_nonempty_string(
        &evidence["candidate_identity_after"],
        "offline cut candidate after",
    );
    assert_ne!(
        evidence["candidate_identity_before"], evidence["candidate_identity_after"],
        "offline cut evidence must bind a real rejected candidate delta"
    );
}

fn assert_lifecycle_boundary_evidence_is_actual_typed_candidate(value: &Value) {
    let lifecycle = &value["lifecycle_state"];
    let executed = executed_evidence_inventory(value);
    let evidence = executed
        .get("i2-evidence:lifecycle-boundary-detected")
        .unwrap_or_else(|| {
            panic!(
                "accepted I2 report must include actual lifecycle boundary control evidence: {value:#}"
            )
        });
    assert_executed_evidence_binds_row(evidence, "i2.non_claims_and_lifecycle_boundaries");
    assert_eq!(evidence["executed"], true);
    assert_eq!(evidence["outcome"], "detected");
    assert_eq!(evidence["diagnostic_code"], "LifecycleBoundaryOverclaim");
    assert_eq!(
        evidence["candidate_source"],
        "actual-lifecycle-boundary-candidate"
    );
    assert_nonempty_string(
        &evidence["producer_invocation_ref"],
        "lifecycle boundary producer invocation ref",
    );
    assert_nonempty_string(
        &evidence["candidate_identity_before"],
        "lifecycle boundary candidate before",
    );
    assert_nonempty_string(
        &evidence["candidate_identity_after"],
        "lifecycle boundary candidate after",
    );
    assert_ne!(
        evidence["candidate_identity_before"], evidence["candidate_identity_after"],
        "lifecycle boundary evidence must bind an actual overclaim candidate delta"
    );

    let observed_candidate = &evidence["lifecycle_boundary_candidate"];
    assert_eq!(
        observed_candidate["source"], "actual-lifecycle-boundary-candidate",
        "normal lifecycle row must derive from a typed lifecycle candidate, not static non_claims text: {observed_candidate:#}"
    );
    for field in [
        "broad_i1_exit_accepted",
        "i2_entry_accepted",
        "i2_exit_accepted",
        "sys7_goal_active",
        "i3_program_active",
        "public_transport_claim",
        "real_transport_selected",
        "production_deployment_claim",
    ] {
        assert_eq!(
            observed_candidate[field], lifecycle[field],
            "normal lifecycle row must derive {field} from the typed lifecycle candidate"
        );
    }

    let overclaim_candidate = &evidence["overclaim_candidate"];
    assert_eq!(
        overclaim_candidate["source"], "actual-lifecycle-boundary-candidate",
        "lifecycle falsifier control must mutate a typed lifecycle candidate"
    );
    assert_eq!(overclaim_candidate["i2_exit_accepted"], true);
    assert_eq!(overclaim_candidate["public_transport_claim"], true);
    assert_eq!(overclaim_candidate["accepted"], false);
    assert_eq!(overclaim_candidate["mutation_applied"], false);
    assert_nonempty_string(
        &overclaim_candidate["candidate_ref"],
        "lifecycle overclaim candidate ref",
    );
}

fn assert_runtime_boundary_rejection_preserves_underlying_state(
    value: &Value,
    diagnostic_code: &str,
) {
    let attempt = &value["rejection"]["runtime_endpoint_attempt"];
    assert_eq!(
        attempt["diagnostic_code"], diagnostic_code,
        "runtime boundary rejection must carry the exact typed diagnostic: {attempt:#}"
    );
    assert_nonempty_string(
        &attempt["attempt_ref"],
        "runtime boundary rejected attempt ref",
    );
    assert_nonempty_string(
        &attempt["producer_invocation_ref"],
        "runtime boundary rejected producer invocation ref",
    );
    let underlying_unchanged =
        attempt["underlying_state_before"] == attempt["underlying_state_after"];
    let authority_unchanged = attempt["authority_state_before"] == attempt["authority_state_after"];
    let semantic_unchanged = attempt["semantic_state_before"] == attempt["semantic_state_after"];
    assert!(
        underlying_unchanged,
        "rejected authority/store runtime endpoint controls must preserve underlying state"
    );
    assert!(
        authority_unchanged,
        "rejected authority/store runtime endpoint controls must preserve authority state"
    );
    assert!(
        semantic_unchanged,
        "rejected authority/store runtime endpoint controls must preserve semantic state"
    );
    if let Some(mutation_applied) = attempt.get("mutation_applied").and_then(Value::as_bool) {
        assert_eq!(
            mutation_applied,
            !(underlying_unchanged && authority_unchanged && semantic_unchanged),
            "runtime endpoint mutation_applied must be derived from before/after state equality when present, not trusted as an independent literal: {attempt:#}"
        );
    }
}

fn assert_typed_owner_residual_distinguishes_owner_loci(residual: &Value) {
    let reason = &residual["typed_admission_reason"];
    assert_nonempty_array(&reason["owner_loci"], "residual owner_loci");
    assert_nonempty_array(&reason["source_owner_loci"], "residual source_owner_loci");
    assert_eq!(
        reason["owner_loci_semantics"], "runtime_combined_owner_loci",
        "OW residual must type owner_loci instead of collapsing it with source_owner_loci"
    );
    assert_eq!(
        reason["source_owner_loci_semantics"], "source_declared_owner_loci",
        "OW residual must type source_owner_loci instead of collapsing it with owner_loci"
    );
}

fn assert_model_fingerprint_covers_execution_backend_if_retained(row: &Value) {
    let retained = [
        "model_fingerprint_ref",
        "bounded_model_fingerprint_ref",
        "model_st_fingerprint",
        "model_ow1_fingerprint",
    ]
    .iter()
    .any(|field| {
        row.get(*field)
            .and_then(Value::as_str)
            .is_some_and(|text| !text.is_empty())
    });
    if retained {
        assert_json_array_contains_all(
            &row["model_fingerprint_components"],
            &["sys2_execution_backend"],
        );
    }
}

fn assert_bounded_implementation_source_fingerprint(value: &Value) {
    assert_json_tree_lacks_key(value, "implementation_cut");
    let fingerprint = &value["bounded_implementation_source_fingerprint"];
    assert!(
        fingerprint.as_object().is_some(),
        "provisional I2 report must expose bounded_implementation_source_fingerprint as typed metadata, not an implementation_cut release claim: {fingerprint:#}"
    );
    assert_nonempty_string(
        &fingerprint["id"],
        "bounded implementation source fingerprint id",
    );
    assert!(
        fingerprint["id"]
            .as_str()
            .is_some_and(|id| id.starts_with("i2-bounded-implementation-source-sha256-v1:")),
        "bounded implementation source fingerprint must be I2/provisional-source namespaced, not a Git/release cut: {fingerprint:#}"
    );
    assert_ne!(
        fingerprint["id"].as_str(),
        Some(M10_ACCEPTED_IMPLEMENTATION_CUT),
        "bounded implementation source fingerprint must not reuse the exact accepted M10 implementation cut"
    );
    assert_eq!(
        fingerprint["scope"], "i2-provisional-implementation-source",
        "bounded implementation fingerprint scope must not claim public/release acceptance"
    );
    assert_eq!(
        fingerprint["runtime_identity_claim"], false,
        "bounded implementation source fingerprint is evidence metadata, not runtime identity"
    );
    assert_eq!(
        fingerprint["public_release_cut"], false,
        "bounded implementation source fingerprint must not claim a public release cut"
    );
    assert_json_array_contains_all(
        &fingerprint["source_components"],
        &[
            "crates/mir-runtime/src/sys6_i2_conformance.rs",
            "crates/mir-runtime/src/sys2_execution_backend.rs",
        ],
    );
    assert!(
        fingerprint
            .get("acceptance_metadata")
            .is_none_or(|metadata| metadata
                .get("exact_git_cut")
                .is_none_or(|cut| cut.is_string())),
        "exact Git cut, if present, belongs only in later acceptance_metadata.exact_git_cut: {fingerprint:#}"
    );
}

fn string_array(value: &Value, label: &str) -> Vec<String> {
    let entries = value
        .as_array()
        .unwrap_or_else(|| panic!("{label} must be an array: {value:#}"));
    entries
        .iter()
        .map(|entry| {
            let text = entry
                .as_str()
                .unwrap_or_else(|| panic!("{label} entries must be strings: {entry:#}"));
            assert!(!text.is_empty(), "{label} entries must be nonempty");
            text.to_string()
        })
        .collect()
}

fn assert_json_tree_lacks_invented_property_ref(value: &Value, label: &str) {
    let rendered = serde_json::to_string(value).expect("JSON tree should serialize");
    assert!(
        !rendered.contains("i2-property-"),
        "{label} must not contain invented i2-property-* refs: {rendered}"
    );
}

fn assert_json_tree_lacks_key(value: &Value, denied_key: &str) {
    match value {
        Value::Object(object) => {
            assert!(
                !object.contains_key(denied_key),
                "provisional I2 report must not expose `{denied_key}`; use bounded_implementation_source_fingerprint for source fingerprint metadata: {value:#}"
            );
            for child in object.values() {
                assert_json_tree_lacks_key(child, denied_key);
            }
        }
        Value::Array(entries) => {
            for child in entries {
                assert_json_tree_lacks_key(child, denied_key);
            }
        }
        _ => {}
    }
}

fn assert_rows_cross_join_inventories(value: &Value) {
    let checked = inventory_set(value, "checked_program_identity_refs");
    let core = inventory_set(value, "core_refs");
    let artifacts = inventory_set(value, "artifact_refs");
    let edges = inventory_set(value, "communication_edge_refs");
    let requests = inventory_set(value, "request_identity_refs");
    let occurrences = inventory_set(value, "runtime_occurrence_refs");

    for row in value["rows"].as_array().expect("rows should be an array") {
        assert_optional_ref_in_inventory(row, "checked_program_identity_ref", &checked);
        assert_optional_ref_in_inventory(row, "core_ref", &core);
        assert_optional_ref_in_inventory(row, "artifact_ref", &artifacts);
        assert_optional_ref_in_inventory(row, "locus_program_ref", &artifacts);
        assert_optional_ref_in_inventory(row, "edge_ref", &edges);
        assert_optional_ref_in_inventory(row, "request_identity", &requests);

        for (field, candidate) in row.as_object().expect("row object") {
            if field.ends_with("_occurrence_ref") {
                let Some(actual) = candidate.as_str() else {
                    assert_eq!(
                        candidate.get("applicability").and_then(Value::as_str),
                        Some("not-applicable"),
                        "{field} should be a string occurrence ref or typed non-applicable: {row:#}"
                    );
                    continue;
                };
                if actual.is_empty() {
                    continue;
                }
                assert!(
                    occurrences.contains(actual),
                    "{field} `{actual}` must cross-join runtime occurrence inventory"
                );
            }
        }
    }
}

fn inventory_set(value: &Value, field: &str) -> BTreeSet<String> {
    value["inventories"][field]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventory {field}: {value:#}"))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("inventory {field} entry should be string: {entry:#}"))
                .to_string()
        })
        .collect()
}

fn executed_evidence_inventory(value: &Value) -> BTreeMap<String, &Value> {
    let entries = value["inventories"]["executed_evidence"]
        .as_array()
        .unwrap_or_else(|| panic!("missing inventories.executed_evidence: {value:#}"));
    let mut by_id = BTreeMap::new();
    for entry in entries {
        let id = entry["id"]
            .as_str()
            .unwrap_or_else(|| panic!("executed evidence entry should have id: {entry:#}"));
        assert_nonempty_string(&entry["kind"], "executed evidence kind");
        assert_nonempty_string(&entry["outcome"], "executed evidence outcome");
        assert_has_binding_field(entry);
        assert!(
            !id.contains("TODO") && !id.contains("placeholder"),
            "executed evidence id must not be placeholder text: {id}"
        );
        let previous = by_id.insert(id.to_string(), entry);
        assert!(
            previous.is_none(),
            "executed evidence IDs must be unique, duplicate id: {id}"
        );
    }
    by_id
}

fn assert_has_binding_field(evidence: &Value) {
    let has_row_ids = evidence["row_ids"]
        .as_array()
        .is_some_and(|row_ids| !row_ids.is_empty());
    let has_property_ids = evidence["property_ids"]
        .as_array()
        .is_some_and(|property_ids| !property_ids.is_empty());
    assert!(
        has_row_ids || has_property_ids,
        "executed evidence must bind at least one row_ids or property_ids entry: {evidence:#}"
    );
}

fn assert_executed_evidence_binds_row(evidence: &Value, row_id: &str) {
    assert_has_binding_field(evidence);
    let row_bound = evidence["row_ids"]
        .as_array()
        .is_some_and(|row_ids| row_ids.iter().any(|id| id.as_str() == Some(row_id)));
    let property_bound = evidence["property_ids"]
        .as_array()
        .is_some_and(|property_ids| property_ids.iter().any(|id| id.as_str() == Some(row_id)));
    assert!(
        row_bound || property_bound,
        "executed evidence referenced by row {row_id} must bind that row through row_ids or property_ids: {evidence:#}"
    );
}

fn assert_optional_ref_in_inventory(row: &Value, field: &str, inventory: &BTreeSet<String>) {
    let Some(actual) = row.get(field).and_then(Value::as_str) else {
        return;
    };
    if actual.is_empty() {
        return;
    }
    assert!(
        inventory.contains(actual),
        "row {} field {field} must cross-join actual inventory: {actual}",
        row["id"].as_str().unwrap_or("<unknown>")
    );
}

fn assert_fail_closed_rejection_basic(
    report: &I2ConformanceReport,
    value: &Value,
    failed_rows: &[&str],
    mutation_stage: &str,
    diagnostic_code: &str,
) {
    assert_eq!(report.status(), I2ConformanceStatus::Rejected);
    assert_eq!(value["status"], "rejected");
    assert_eq!(value["test_only_falsifier"], true);
    assert_json_tree_lacks_key(value, "cli_falsifier_surface_available");
    assert_json_tree_lacks_key(value, "partial_projection_accepted");
    assert_exact_row_inventory(value);
    assert_exact_failed_rows(value, failed_rows);
    assert_evidence_class_universe(value);

    let rejection = &value["rejection"];
    assert_eq!(rejection["mutation_stage"], mutation_stage);
    assert_eq!(rejection["diagnostic_code"], diagnostic_code);
    assert_eq!(rejection["validator_invocation"]["invoked"], true);
    assert_eq!(rejection["validator_invocation"]["result"], "rejected");
    assert_nonempty_string(
        &rejection["validator_invocation"]["validator_invocation_ref"],
        "validator invocation ref",
    );
    assert_nonempty_string(
        &rejection["candidate_identity_before"],
        "candidate identity before falsifier",
    );
    assert_nonempty_string(
        &rejection["candidate_identity_after"],
        "candidate identity after falsifier",
    );
    assert_ne!(
        rejection["candidate_identity_before"], rejection["candidate_identity_after"],
        "negative report must expose the actual bounded candidate mutation"
    );
    assert_eq!(
        rejection["snapshots"]["semantic_before"], rejection["snapshots"]["semantic_after"],
        "semantic state must be unchanged on conformance rejection"
    );
    assert_eq!(
        rejection["snapshots"]["runtime_before"], rejection["snapshots"]["runtime_after"],
        "runtime state must be unchanged on conformance rejection"
    );
    assert_eq!(
        rejection["snapshots"]["authority_before"], rejection["snapshots"]["authority_after"],
        "authority state must be unchanged on conformance rejection"
    );
}

fn assert_control_diagnostic_substitution_details(
    value: &Value,
    control_id: &str,
    expected_diagnostic: &str,
    observed_diagnostic: &str,
) {
    let candidate = &value["rejection"]["control_diagnostic_candidate"];
    assert_eq!(
        candidate["source"], "actual-control-diagnostic-candidate",
        "diagnostic substitution must mutate an actual control candidate, not a row verdict: {candidate:#}"
    );
    assert_eq!(candidate["control_id"], control_id);
    assert_eq!(candidate["expected_diagnostic_code"], expected_diagnostic);
    assert_eq!(candidate["observed_diagnostic_code"], observed_diagnostic);
    assert_eq!(candidate["accepted"], false);
    assert_eq!(candidate["mutation_applied"], false);
    assert_nonempty_string(
        &candidate["candidate_ref"],
        "control diagnostic candidate ref",
    );
    assert_nonempty_string(
        &candidate["producer_invocation_ref"],
        "control diagnostic producer invocation ref",
    );
}

fn assert_bound_evidence_failure_details(value: &Value, evidence_id: &str, expected_rows: &[&str]) {
    let candidate = &value["rejection"]["executed_evidence_candidate"];
    assert_eq!(
        candidate["source"], "actual-executed-evidence-candidate",
        "bound evidence failure must mutate an actual executed evidence candidate: {candidate:#}"
    );
    assert_eq!(candidate["evidence_id"], evidence_id);
    assert_eq!(candidate["accepted"], false);
    assert_eq!(candidate["executed"], false);
    assert!(
        !matches!(candidate["outcome"].as_str(), Some("observed" | "detected")),
        "failed bound evidence must not retain an accepted observed/detected outcome: {candidate:#}"
    );
    assert_json_array_exact_string_set(&candidate["affected_row_ids"], expected_rows);
    assert_nonempty_string(
        &candidate["candidate_ref"],
        "executed evidence candidate ref",
    );
    assert_nonempty_string(
        &candidate["producer_invocation_ref"],
        "executed evidence producer invocation ref",
    );
}

fn assert_rejection_integrity(
    report: &I2ConformanceReport,
    value: &Value,
    failed_rows: &[&str],
    mutation_stage: &str,
    diagnostic_code: &str,
) {
    assert_eq!(report.status(), I2ConformanceStatus::Rejected);
    assert_eq!(value["status"], "rejected");
    assert_eq!(value["test_only_falsifier"], true);
    assert_json_tree_lacks_key(value, "cli_falsifier_surface_available");
    assert_json_tree_lacks_key(value, "partial_projection_accepted");
    assert_exact_row_inventory(value);
    assert_failed_rows(value, failed_rows);
    assert_evidence_class_universe(value);
    assert_rows_have_positive_and_falsifier_evidence(value);
    assert_controls_cross_join_executed_evidence(value);

    let rejection = &value["rejection"];
    assert_eq!(rejection["mutation_stage"], mutation_stage);
    assert_eq!(rejection["diagnostic_code"], diagnostic_code);
    assert_eq!(rejection["validator_invocation"]["invoked"], true);
    assert_eq!(rejection["validator_invocation"]["result"], "rejected");
    assert_nonempty_string(
        &rejection["validator_invocation"]["validator_invocation_ref"],
        "validator invocation ref",
    );
    assert_nonempty_string(
        &rejection["candidate_identity_before"],
        "candidate identity before falsifier",
    );
    assert_nonempty_string(
        &rejection["candidate_identity_after"],
        "candidate identity after falsifier",
    );
    assert_ne!(
        rejection["candidate_identity_before"], rejection["candidate_identity_after"],
        "negative report must expose the actual bounded candidate mutation"
    );
    assert_eq!(
        rejection["snapshots"]["semantic_before"], rejection["snapshots"]["semantic_after"],
        "semantic state must be unchanged on conformance rejection"
    );
    assert_eq!(
        rejection["snapshots"]["runtime_before"], rejection["snapshots"]["runtime_after"],
        "runtime state must be unchanged on conformance rejection"
    );
    assert_eq!(
        rejection["snapshots"]["authority_before"], rejection["snapshots"]["authority_after"],
        "authority state must be unchanged on conformance rejection"
    );
    assert_observer_safe_report(value);
}

fn assert_json_array_contains_all(value: &Value, expected: &[&str]) {
    let array = value.as_array().expect("value should be a JSON array");
    for fragment in expected {
        assert!(
            array.iter().any(|entry| entry.as_str() == Some(fragment)),
            "array missing `{fragment}`: {value:#}"
        );
    }
}

fn assert_json_array_lacks_string(value: &Value, denied: &str) {
    let array = value.as_array().expect("value should be a JSON array");
    assert!(
        !array.iter().any(|entry| entry.as_str() == Some(denied)),
        "array must not contain `{denied}`: {value:#}"
    );
}

fn assert_json_array_exact_string_set(value: &Value, expected: &[&str]) {
    let actual = value
        .as_array()
        .unwrap_or_else(|| panic!("expected JSON string array: {value:#}"))
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .unwrap_or_else(|| panic!("array entry must be string: {entry:#}"))
        })
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "JSON array string set mismatch");
}

fn assert_nonempty_array(value: &Value, label: &str) {
    assert!(
        value.as_array().is_some_and(|entries| !entries.is_empty()),
        "{label} should be a nonempty array: {value:#}"
    );
}

fn assert_nonempty_string(value: &Value, label: &str) {
    assert!(
        value.as_str().is_some_and(|text| !text.is_empty()),
        "{label} should be a nonempty string: {value:#}"
    );
}

fn assert_absent_or_empty_string(row: &Value, field: &str) {
    assert!(
        row.get(field)
            .is_none_or(|value| value.as_str() == Some("")),
        "missing required anchor must clear or omit stale {field} rather than preserving an unbound ref: {row:#}"
    );
}

fn assert_observer_safe_report(value: &Value) {
    let rendered = serde_json::to_string(value).expect("report JSON should serialize");
    for denied in [
        "/home/",
        "/root/",
        "/tmp/",
        "avatar[self].atk",
        "avatar[target].hp =",
        "participant_input[self].focus +",
        "raw_source",
        "source_text",
        "raw_authority",
        "raw_capability",
        "raw_witness",
        "raw_credential",
        "capability_secret",
        "witness_secret",
        "credential_secret",
        "host_absolute_path",
        "expected_result",
    ] {
        assert!(
            !rendered.contains(denied),
            "observer-safe I2 report leaked denied fragment `{denied}`: {rendered}"
        );
    }
}
