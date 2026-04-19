use mir_runtime::current_l2::current_l2_phase6_next_reopen_sequencing_manifest;

#[test]
fn current_l2_phase6_next_reopen_sequencing_manifest_keeps_minimum_cut() {
    let manifest = current_l2_phase6_next_reopen_sequencing_manifest();

    assert_eq!(
        manifest.sequencing_kind_ref,
        "phase6_checkpoint_postclose_next_reopen"
    );
    assert_eq!(
        manifest.fixed_entry_criteria_refs,
        &[
            "phase6_parser_first_tranche",
            "phase6_checker_runtime_first_tranche",
            "phase6_compile_ready_formal_hook",
        ]
    );
    assert_eq!(
        manifest.selected_first_reopen_ref,
        "phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route"
    );
    assert_eq!(
        manifest.deferred_reopen_refs,
        &[
            "theorem_first_concrete_tool_binding_route",
            "concrete_model_check_tool_binding",
        ]
    );
    assert_eq!(
        manifest.guard_refs,
        &[
            "keep_tool_neutral_formal_hook_as_entry_criteria",
            "avoid_request_head_clause_suite_richer_diagnostics_bulk_widen",
            "keep_model_check_line_reserve_only",
        ]
    );
}
