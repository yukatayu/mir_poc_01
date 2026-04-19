use mir_runtime::current_l2::current_l2_phase6_reserve_formal_tool_binding_inventory_manifest;

#[test]
fn current_l2_phase6_reserve_formal_tool_binding_inventory_manifest_keeps_minimum_cut() {
    let manifest = current_l2_phase6_reserve_formal_tool_binding_inventory_manifest();

    assert_eq!(
        manifest.inventory_kind,
        "phase6_postclose_formal_reserve_inventory"
    );
    assert_eq!(
        manifest.fixed_entry_criteria_refs,
        &[
            "phase5_handoff_closeout",
            "phase6_compile_ready_formal_hook",
            "phase6_parser_second_tranche_first_package",
        ]
    );
    assert_eq!(
        manifest.first_reserve_ref,
        "theorem_first_notebook_pressure_concrete_tool_binding_route"
    );
    assert_eq!(
        manifest.second_reserve_ref,
        "model_check_protocol_property_concrete_tool_binding_route"
    );
    assert_eq!(
        manifest.guard_refs,
        &[
            "keep_tool_neutral_formal_hook_as_current_entry_criteria",
            "keep_parser_followup_package_as_current_mainline",
            "avoid_dual_tool_choice_single_package",
            "avoid_public_checker_runtime_surface_backpressure",
        ]
    );
}
