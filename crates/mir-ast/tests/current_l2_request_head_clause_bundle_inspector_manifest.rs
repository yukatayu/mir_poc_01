use std::path::PathBuf;

use mir_ast::current_l2::{
    current_l2_request_head_clause_bundle_inspector_manifest,
    inspect_stage3_request_head_clause_bundle,
    parse_stage3_request_head_clause_bundle_text,
    render_current_l2_request_head_clause_bundle_inspection_json,
    render_current_l2_request_head_clause_bundle_inspection_pretty,
};

fn parser_companion_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-parser-companion")
        .join(file_name)
}

fn read_parser_companion(file_name: &str) -> String {
    std::fs::read_to_string(parser_companion_path(file_name)).unwrap()
}

#[test]
fn current_l2_request_head_clause_bundle_inspector_manifest_keeps_repo_local_cut() {
    let manifest = current_l2_request_head_clause_bundle_inspector_manifest();

    assert_eq!(
        manifest.carrier_kind,
        "current_l2_nonproduction_request_head_clause_bundle_inspector"
    );
    assert_eq!(
        manifest.accepted_surface_refs,
        &["stage3_request_head_clause_bundle_inspector"]
    );
    assert_eq!(
        manifest.code_anchor_refs,
        &[
            "mir_ast_current_l2_module",
            "stage3_request_head_clause_bundle_inspector_tests",
        ]
    );
    assert_eq!(
        manifest.retained_later_refs,
        &[
            "final_public_parser_checker_runtime_surface",
            "full_program_lowering",
            "span_rich_diagnostics",
            "final_grammar",
        ]
    );
}

#[test]
fn request_head_clause_bundle_inspector_renders_representative_samples() {
    let problem1_bundle = parse_stage3_request_head_clause_bundle_text(&read_parser_companion(
        "p06-typed-proof-owner-handoff.request.txt",
    ))
    .unwrap();
    let late_join_bundle = parse_stage3_request_head_clause_bundle_text(&read_parser_companion(
        "p07-dice-late-join-visible-history.request.txt",
    ))
    .unwrap();
    let reconnect_bundle = parse_stage3_request_head_clause_bundle_text(&read_parser_companion(
        "p08-dice-stale-reconnect-refresh.request.txt",
    ))
    .unwrap();

    let problem1_inspection = inspect_stage3_request_head_clause_bundle(&problem1_bundle);
    let late_join_inspection = inspect_stage3_request_head_clause_bundle(&late_join_bundle);
    let reconnect_inspection = inspect_stage3_request_head_clause_bundle(&reconnect_bundle);

    assert_eq!(
        render_current_l2_request_head_clause_bundle_inspection_json(&problem1_inspection),
        concat!(
            "{\n",
            "  \"op\": \"prove_owner_handoff\",\n",
            "  \"target_kind\": \"via\",\n",
            "  \"target_ref\": \"review_unit\",\n",
            "  \"require_fragment_text\": \"typed_guard\",\n",
            "  \"ensure_fragment_text\": \"owner_is(next_owner)\",\n",
            "  \"attachment_frame_kind\": \"request_local_two_slot_suite\"\n",
            "}"
        )
    );
    assert_eq!(
        render_current_l2_request_head_clause_bundle_inspection_pretty(&late_join_inspection),
        concat!(
            "op: publish_visible_history\n",
            "target: on authoritative_room\n",
            "require: authority_ack\n",
            "ensure: history_visible_as_past\n",
            "attachment: request_local_two_slot_suite"
        )
    );
    assert_eq!(
        render_current_l2_request_head_clause_bundle_inspection_pretty(&reconnect_inspection),
        concat!(
            "op: refresh_stale_session\n",
            "target: via authoritative_room\n",
            "require: stale_session\n",
            "ensure: refreshed_before_rejoin\n",
            "attachment: request_local_two_slot_suite"
        )
    );
}
