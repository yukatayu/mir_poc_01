use mir_runtime::current_l2::current_l2_checker_runtime_first_tranche_manifest;

#[test]
fn current_l2_checker_runtime_first_tranche_manifest_keeps_minimum_cut() {
    let manifest = current_l2_checker_runtime_first_tranche_manifest();

    assert_eq!(
        manifest.skeleton_kind,
        "current_l2_nonproduction_checker_runtime_skeleton"
    );
    assert_eq!(
        manifest.semantic_entry_refs,
        &[
            "static_gate_program_detailed",
            "direct_style_evaluator_from_program",
            "fixture_host_stub_run_program",
        ]
    );
    assert_eq!(
        manifest.runtime_bridge_refs,
        &[
            "mir_runtime_current_l2_module",
            "current_l2_runtime_skeleton_report",
        ]
    );
    assert_eq!(
        manifest.parser_bridge_contract_refs,
        &[
            "stage1_reconnect_clusters",
            "stage2_try_rollback_structural_summary",
            "parser_bridge_consistency_guard",
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "parser_to_program_lowering",
            "stage3_request_predicate_reconnect",
            "richer_host_interface",
            "final_public_runtime_checker_api",
            "formal_hook_concrete_tool_binding",
        ]
    );
}
