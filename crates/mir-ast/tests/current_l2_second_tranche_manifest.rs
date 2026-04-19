use mir_ast::current_l2::current_l2_second_tranche_manifest;

#[test]
fn current_l2_second_tranche_manifest_keeps_attached_slot_and_predicate_cut() {
    let manifest = current_l2_second_tranche_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "current_l2_nonproduction_parser_second_tranche_carrier"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &[
            "stage3_decl_admit_slot_surface",
            "stage3_minimal_predicate_fragment_surface",
        ]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &[
            "mir_ast_current_l2_module",
            "stage3_admit_slot_tests",
            "stage3_predicate_fragment_tests",
            "stage3_multiline_and_suite_tests_reusing_fragment_parser",
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "shared_single_attachment_frame",
            "request_clause_suite_publicization",
            "perform_head_final_public_api",
            "span_rich_diagnostics",
            "final_grammar",
            "theorem_model_check_concrete_binding",
        ]
    );
}
