use mir_ast::current_l2::current_l2_request_head_clause_bundle_manifest;

#[test]
fn current_l2_request_head_clause_bundle_manifest_keeps_thin_wrapper_cut() {
    let manifest = current_l2_request_head_clause_bundle_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "current_l2_nonproduction_request_head_clause_bundle"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &[
            "stage3_perform_on_head_surface",
            "stage3_perform_via_head_surface",
            "stage3_request_clause_suite_surface",
        ]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &[
            "mir_ast_current_l2_module",
            "stage3_request_head_clause_bundle_tests",
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "span_rich_diagnostics",
            "final_grammar",
            "final_public_parser_checker_runtime_surface",
            "full_program_lowering",
        ]
    );
}
