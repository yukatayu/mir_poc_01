use mir_runtime::current_l2::current_l2_phase6_parser_side_followup_package_sequencing_manifest;

#[test]
fn current_l2_phase6_parser_side_followup_package_sequencing_manifest_keeps_minimum_cut() {
    let manifest = current_l2_phase6_parser_side_followup_package_sequencing_manifest();

    assert_eq!(
        manifest.sequencing_kind,
        "phase6_parser_followup_next_package_selection"
    );
    assert_eq!(
        manifest.fixed_entry_criteria_refs,
        &[
            "phase6_parser_second_tranche_first_package",
            "phase6_reserve_formal_tool_binding_inventory",
            "stage3_multiline_attachment_first_tranche_actualization",
        ]
    );
    assert_eq!(
        manifest.selected_next_package_ref,
        "phase6_parser_second_tranche_shared_single_attachment_frame_first_package"
    );
    assert_eq!(
        manifest.deferred_reopen_refs,
        &[
            "phase6_request_clause_suite_publicization",
            "phase6_perform_head_final_public_parser_api",
            "phase6_span_rich_diagnostics",
            "phase6_final_grammar",
        ]
    );
    assert_eq!(
        manifest.guard_refs,
        &[
            "reuse_existing_stage3_minimal_predicate_fragment_surface",
            "keep_request_head_and_suite_ordering_out_of_scope",
            "keep_source_sample_path_after_parser_followup_cut",
        ]
    );
}
