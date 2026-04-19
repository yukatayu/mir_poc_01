use mir_ast::current_l2::current_l2_first_tranche_manifest;

#[test]
fn current_l2_first_tranche_manifest_stays_on_stage1_stage2_floor() {
    let manifest = current_l2_first_tranche_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "current_l2_nonproduction_parser_carrier"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &[
            "stage1_option_decl_chain_surface",
            "stage2_try_fallback_structural_surface",
        ]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &[
            "mir_ast_current_l2_module",
            "stage1_stage2_parser_spike_tests"
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "stage3_admit_slot_surface",
            "stage3_request_clause_suite",
            "stage3_predicate_fragment",
            "perform_head_final_public_api",
            "span_rich_diagnostics",
            "final_grammar",
        ]
    );
}
