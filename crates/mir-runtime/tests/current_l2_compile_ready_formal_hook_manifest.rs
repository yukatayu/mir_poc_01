use mir_runtime::current_l2::current_l2_compile_ready_verification_and_formal_hook_manifest;

#[test]
fn current_l2_compile_ready_formal_hook_manifest_keeps_minimum_cut() {
    let manifest = current_l2_compile_ready_verification_and_formal_hook_manifest();

    assert_eq!(
        manifest.verification_gate_refs,
        &[
            "cargo_test_mir_ast",
            "cargo_test_mir_runtime",
            "cargo_test_mir_semantics_current_l2_minimal_interpreter",
            "cargo_test_mir_semantics_current_l2_static_gate_support",
            "cargo_test_mir_semantics_current_l2_detached_bundle_support",
            "cargo_test_mir_semantics_current_l2_formal_hook_support",
            "python_unittest_current_l2_static_and_detached_loop",
        ]
    );
    assert_eq!(
        manifest.smoke_gate_refs,
        &["smoke_formal_hook_static", "smoke_formal_hook_runtime",]
    );
    assert_eq!(
        manifest.formal_hook_artifact_kind_ref,
        "current_l2_tool_neutral_formal_hook"
    );
    assert_eq!(
        manifest.formal_hook_subject_kind_refs,
        &["fixture_static_cluster", "runtime_try_cut_cluster",]
    );
    assert_eq!(
        manifest.formal_hook_contract_row_core_refs,
        &["obligation_kind", "evidence_refs",]
    );
    assert_eq!(
        manifest.formal_hook_evidence_ref_family_refs,
        &["ref_kind", "ref_id",]
    );
    assert_eq!(
        manifest.formal_hook_obligation_kind_refs,
        &[
            "canonical_normalization_law",
            "no_re_promotion",
            "rollback_cut_non_interference",
        ]
    );
    assert_eq!(
        manifest.source_artifact_refs,
        &["detached_static_gate_artifact", "detached_bundle_artifact",]
    );
    assert_eq!(
        manifest.validation_refs,
        &["input_schema_version_guard", "input_artifact_kind_guard",]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "concrete_theorem_tool_binding",
            "concrete_model_check_tool_binding",
            "parser_second_tranche_widen",
            "final_public_surface",
        ]
    );
}
