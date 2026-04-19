use mir_ast::current_l2::current_l2_perform_head_manifest;

#[test]
fn current_l2_perform_head_manifest_keeps_minimum_cut() {
    let manifest = current_l2_perform_head_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "current_l2_nonproduction_perform_head_carrier"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &[
            "stage3_perform_owner_surface",
            "stage3_perform_on_head_surface",
            "stage3_perform_via_head_surface",
        ]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &["mir_ast_current_l2_module", "stage3_perform_head_tests"]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "request_clause_suite_bundle_attachment",
            "span_rich_diagnostics",
            "final_grammar",
        ]
    );
}
