use mir_ast::current_l2::current_l2_request_clause_suite_manifest;

#[test]
fn current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut() {
    let manifest = current_l2_request_clause_suite_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "current_l2_nonproduction_request_clause_suite_carrier"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &[
            "stage3_request_clause_suite_surface",
            "stage3_request_clause_multiline_extraction_surface",
            "stage3_minimal_predicate_fragment_surface",
        ]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &[
            "mir_ast_current_l2_module",
            "stage3_request_clause_suite_tests",
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "perform_head_final_public_parser_api",
            "span_rich_diagnostics",
            "final_grammar",
        ]
    );
}
