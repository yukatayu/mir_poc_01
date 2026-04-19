use std::path::PathBuf;

use mir_runtime::current_l2_cli::run_current_l2_operational_cli;
use serde_json::Value;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast/tests/fixtures/current-l2")
        .join(name)
}

fn host_plan_path(name: &str) -> PathBuf {
    fixture_path(name).with_extension("host-plan.json")
}

fn order_handoff_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

fn typed_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-typed-proof-model-check")
        .join(name)
}

#[test]
fn operational_cli_pretty_runs_runtime_sample_with_explicit_host_plan_path() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("shell: mir-current-l2"));
    assert!(output.contains("command: run-source-sample"));
    assert!(output.contains("sample: e2-try-fallback"));
    assert!(output.contains("host_plan_path:"));
    assert!(output.contains("static_gate: valid"));
    assert!(output.contains("terminal_outcome: success"));
    assert!(output.contains("- perform-success"));
    assert!(output.contains("- perform-failure"));
    assert!(output.contains("- rollback"));
    assert!(output.contains("verification_preview:"));
    assert!(output.contains("formal_hook_status: reached"));
    assert!(output.contains("subject_kind: runtime_try_cut_cluster"));
    assert!(output.contains("rollback_cut_non_interference"));
}

#[test]
fn operational_cli_json_reports_static_stop_without_entering_evaluation() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        "e13-malformed-capability-strengthening",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["shell"], "mir-current-l2");
    assert_eq!(value["command"], "run-source-sample");
    assert_eq!(value["sample"], "e13-malformed-capability-strengthening");
    assert_eq!(
        value["checker_floor"]["static_gate"]["verdict"],
        "malformed"
    );
    assert_eq!(
        value["checker_floor"]["stage1_reconnect_clusters"]["capability_strengthening_floor"],
        true
    );
    assert_eq!(value["runtime"]["entered_evaluation"], false);
    assert_eq!(value["runtime"]["terminal_outcome"], Value::Null);
    assert_eq!(
        value["verification_preview"]["formal_hook_status"],
        "reached"
    );
    assert_eq!(
        value["verification_preview"]["subject_kind"],
        "fixture_static_cluster"
    );
    assert_eq!(
        value["verification_preview"]["proof_notebook_review_unit_obligations"][0],
        "canonical_normalization_law"
    );
    assert_eq!(
        value["verification_preview"]["proof_notebook_review_unit_obligations"][1],
        "no_re_promotion"
    );
}

#[test]
fn operational_cli_json_reports_underdeclared_verification_preview_and_artifact_preview() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        "e5-underdeclared-lineage",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "e5-underdeclared-lineage");
    assert_eq!(
        value["checker_floor"]["static_gate"]["verdict"],
        "underdeclared"
    );
    assert_eq!(
        value["verification_preview"]["formal_hook_status"],
        "reached"
    );
    assert_eq!(
        value["verification_preview"]["subject_kind"],
        "fixture_static_cluster"
    );
    assert_eq!(
        value["artifact_preview"]["proof_notebook_review_units"][0]["obligation_kind"],
        "canonical_normalization_law"
    );
    assert_eq!(
        value["artifact_preview"]["model_check_concrete_carriers"][1]["obligation_kind"],
        "no_re_promotion"
    );
}

#[test]
fn operational_cli_rejects_missing_host_plan_flag() {
    let error = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--format",
        "pretty",
    ])
    .unwrap_err();

    assert_eq!(error.exit_code(), 2);
    assert!(
        error
            .to_string()
            .contains("missing --host-plan <path> and no adjacent host plan was found")
    );
    assert!(error.to_string().contains(
        "mir-current-l2 run-source-sample <sample-or-path> [--host-plan <path>] --format pretty|json"
    ));
}

#[test]
fn operational_cli_rejects_unknown_format() {
    let error = run_current_l2_operational_cli([
        "run-source-sample",
        "e2-try-fallback",
        "--host-plan",
        host_plan_path("e2-try-fallback.json").to_str().unwrap(),
        "--format",
        "yaml",
    ])
    .unwrap_err();

    assert_eq!(error.exit_code(), 2);
    assert!(error.to_string().contains("unsupported format `yaml`"));
}

#[test]
fn operational_cli_uses_adjacent_host_plan_for_prototype_sample_when_omitted() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        order_handoff_prototype_sample_path("p01-dice-publication-handoff.txt")
            .to_str()
            .unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("sample: p01-dice-publication-handoff"));
    assert!(output.contains("host_plan_path:"));
    assert!(output.contains("terminal_outcome: success"));
    assert!(output.contains("debug_outputs:"));
    assert!(output.contains("dice_debug_text_output:"));
    assert!(output.contains("roll_dice: player_a -> visible"));
    assert!(output.contains("verification_preview:"));
    assert!(output.contains("subject_kind: runtime_try_cut_cluster"));
    assert!(output.contains("rollback_cut_non_interference"));
    assert!(output.contains("artifact_preview:"));
    assert!(output.contains("proof_notebook_review_units:"));
    assert!(output.contains("goal_text: Review that rollback / atomic-cut evidence"));
}

#[test]
fn operational_cli_json_reports_static_verification_preview_for_prototype_sample() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        order_handoff_prototype_sample_path("p04-dice-owner-duplicate-declaration.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "p04-dice-owner-duplicate-declaration");
    assert_eq!(
        value["checker_floor"]["static_gate"]["verdict"],
        "malformed"
    );
    assert_eq!(
        value["verification_preview"]["formal_hook_status"],
        "reached"
    );
    assert_eq!(
        value["verification_preview"]["subject_kind"],
        "fixture_static_cluster"
    );
    assert_eq!(
        value["verification_preview"]["model_check_concrete_carrier_obligations"][0],
        "canonical_normalization_law"
    );
    assert_eq!(
        value["artifact_preview"]["proof_notebook_review_units"][0]["obligation_kind"],
        "canonical_normalization_law"
    );
    assert_eq!(
        value["artifact_preview"]["proof_notebook_review_units"][0]["evidence_refs"][0],
        "fixture:p04-dice-owner-duplicate-declaration"
    );
    assert_eq!(
        value["artifact_preview"]["model_check_concrete_carriers"][1]["obligation_kind"],
        "no_re_promotion"
    );
}

#[test]
fn operational_cli_json_reports_guarded_verification_preview_for_prototype_sample() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        order_handoff_prototype_sample_path("p05-dice-owner-guarded-chain.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "p05-dice-owner-guarded-chain");
    assert_eq!(value["checker_floor"]["static_gate"]["verdict"], "valid");
    assert_eq!(
        value["verification_preview"]["formal_hook_status"],
        "guarded_not_reached"
    );
    assert_eq!(value["verification_preview"]["subject_kind"], Value::Null);
    assert_eq!(
        value["verification_preview"]["proof_notebook_review_unit_obligations"],
        Value::Array(Vec::new())
    );
    assert!(
        value["verification_preview"]["guard_reason"]
            .as_str()
            .unwrap()
            .contains("rollback or atomic-cut evidence")
    );
    assert_eq!(
        value["artifact_preview"]["proof_notebook_review_units"],
        Value::Array(Vec::new())
    );
    assert_eq!(
        value["artifact_preview"]["model_check_concrete_carriers"],
        Value::Array(Vec::new())
    );
}

#[test]
fn operational_cli_pretty_reports_typed_bridge_prototype_preview() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt")
            .to_str()
            .unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("sample: p06-typed-proof-owner-handoff"));
    assert!(output.contains("terminal_outcome: success"));
    assert!(output.contains("debug_outputs:"));
    assert!(output.contains("proof_debug_text_output:"));
    assert!(output.contains("publish_roll_result: current_owner -> visible"));
    assert!(output.contains("verification_preview:"));
    assert!(output.contains("subject_kind: runtime_try_cut_cluster"));
    assert!(output.contains("artifact_preview:"));
    assert!(output.contains("proof_notebook_review_units:"));
    assert!(output.contains("model_check_concrete_carriers:"));
}

#[test]
fn operational_cli_json_pins_typed_bridge_prototype_preview() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        typed_prototype_sample_path("p06-typed-proof-owner-handoff.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "p06-typed-proof-owner-handoff");
    assert_eq!(value["checker_floor"]["static_gate"]["verdict"], "valid");
    assert_eq!(value["runtime"]["terminal_outcome"], "success");
    assert_eq!(
        value["verification_preview"]["formal_hook_status"],
        "reached"
    );
    assert_eq!(
        value["verification_preview"]["subject_kind"],
        "runtime_try_cut_cluster"
    );
    assert_eq!(
        value["verification_preview"]["proof_notebook_review_unit_obligations"][0],
        "rollback_cut_non_interference"
    );
    assert_eq!(
        value["artifact_preview"]["proof_notebook_review_units"][0]["obligation_kind"],
        "rollback_cut_non_interference"
    );
    assert_eq!(
        value["artifact_preview"]["model_check_concrete_carriers"][0]["obligation_kind"],
        "rollback_cut_non_interference"
    );
    assert_eq!(value["theorem_result_object_preview"]["status"], "reached");
    assert_eq!(
        value["theorem_result_object_preview"]["subject_kind"],
        "runtime_try_cut_cluster"
    );
    assert_eq!(
        value["model_check_public_checker_preview"]["status"],
        "reached"
    );
    assert_eq!(
        value["model_check_public_checker_preview"]["subject_kind"],
        "runtime_try_cut_cluster"
    );
    assert_eq!(
        value["theorem_final_public_contract_reopen_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["theorem_final_public_contract_reopen_threshold"]["final_public_contract_reopen_sequence_refs"][0],
        "theorem_final_public_contract_reopen:p06-typed-proof-owner-handoff:result_object_and_payload_first"
    );
    assert_eq!(
        value["model_check_final_public_contract_reopen_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["model_check_final_public_contract_reopen_threshold"]["final_public_contract_reopen_sequence_refs"][3],
        "model_check_final_public_contract_reopen:p06-typed-proof-owner-handoff:final_public_verifier_contract_fourth"
    );
    assert_eq!(
        value["order_handoff_witness_provider_public_seam_compression"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_checker_payload_family_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_family_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_public_checker_api_sketch_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_public_checker_entry_criteria_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["status"],
        "guarded_not_reached"
    );
}

#[test]
fn operational_cli_pretty_reports_late_join_order_handoff_prototype() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt")
            .to_str()
            .unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("sample: p07-dice-late-join-visible-history"));
    assert!(output.contains("terminal_outcome: success"));
    assert!(output.contains("observer_debug_text_output:"));
    assert!(output.contains("late_join_view: player_c sees result+owner history"));
    assert!(output.contains("subject_kind: runtime_try_cut_cluster"));
    assert!(output.contains("theorem_final_public_contract_reopen_threshold:"));
    assert!(output.contains("model_check_final_public_contract_reopen_threshold:"));
    assert!(output.contains("order_handoff_witness_provider_public_seam_compression:"));
}

#[test]
fn operational_cli_json_reports_stale_reconnect_refresh_prototype() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["sample"], "p08-dice-stale-reconnect-refresh");
    assert_eq!(value["checker_floor"]["static_gate"]["verdict"], "valid");
    assert_eq!(value["runtime"]["terminal_outcome"], "success");
    assert_eq!(
        value["runtime"]["events"],
        Value::Array(vec![
            Value::String("perform-failure".into()),
            Value::String("rollback".into()),
            Value::String("perform-success".into()),
        ])
    );
    assert_eq!(
        value["verification_preview"]["formal_hook_status"],
        "reached"
    );
    assert_eq!(
        value["verification_preview"]["subject_kind"],
        "runtime_try_cut_cluster"
    );
    assert_eq!(
        value["artifact_preview"]["proof_notebook_review_units"][0]["obligation_kind"],
        "rollback_cut_non_interference"
    );
    assert_eq!(value["theorem_result_object_preview"]["status"], "reached");
    assert_eq!(
        value["theorem_final_public_contract_reopen_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["theorem_final_public_contract_reopen_threshold"]["final_public_contract_reopen_sequence_refs"][1],
        "theorem_final_public_contract_reopen:p08-dice-stale-reconnect-refresh:prover_brand_and_proof_schema_second"
    );
    assert_eq!(
        value["model_check_public_checker_preview"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["model_check_final_public_contract_reopen_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["order_handoff_witness_provider_public_seam_compression"]["status"],
        "reached"
    );
    assert_eq!(
        value["order_handoff_witness_provider_public_seam_compression"]["public_seam_residual_refs"][0],
        "order_handoff_public_seam_residual:p08-dice-stale-reconnect-refresh:final_source_surface_handoff_wording_later"
    );
    assert!(
        value["model_check_public_checker_preview"]["guard_reason"]
            .as_str()
            .unwrap()
            .contains("`e5` / `p06` / `p07` / `p09`")
    );
    assert!(
        value["model_check_final_public_contract_reopen_threshold"]["guard_reason"]
            .as_str()
            .unwrap()
            .contains("`e5` / `p06` / `p07` / `p09`")
    );
}

#[test]
fn operational_cli_json_reports_model_check_public_checker_preview_for_delegated_rng_sample() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        value["theorem_result_object_preview"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["theorem_final_public_contract_reopen_threshold"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["model_check_public_checker_preview"]["status"],
        "reached"
    );
    assert_eq!(
        value["model_check_final_public_contract_reopen_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["order_handoff_witness_provider_public_seam_compression"]["status"],
        "guarded_not_reached"
    );
    assert_eq!(
        value["model_check_final_public_contract_reopen_threshold"]["final_public_contract_reopen_sequence_refs"][2],
        "model_check_final_public_contract_reopen:p09-dice-delegated-rng-provider-placement:verifier_handoff_and_runtime_policy_contract_third"
    );
    assert_eq!(
        value["model_check_public_checker_preview"]["checker_artifact_preview_refs"][0],
        "model_check_public_checker_preview:p09-dice-delegated-rng-provider-placement:consumer_shaped_artifact_preview_only"
    );
    assert_eq!(
        value["model_check_public_checker_preview"]["verifier_handoff_reserve_refs"][0],
        "model_check_verifier_handoff_reserve:public_checker_migration_later"
    );
    assert!(
        value["theorem_final_public_contract_reopen_threshold"]["guard_reason"]
            .as_str()
            .unwrap()
            .contains("`e5` / `p06` / `p07` / `p08`")
    );
    assert!(
        value["order_handoff_witness_provider_public_seam_compression"]["guard_reason"]
            .as_str()
            .unwrap()
            .contains("route/reserve/bridge/threshold floors")
    );
}

#[test]
fn operational_cli_json_reports_ifc_authority_success_checker_hint_preview() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        typed_prototype_sample_path("p10-typed-authorized-fingerprint-declassification.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["typed_checker_hint_preview"]["status"], "reached");
    assert_eq!(
        value["typed_checker_hint_preview"]["cluster_kind"],
        "ifc_authority_release_cluster"
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["case_label"],
        "authority_sensitive_success"
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["typed_reason_family_hint"]["family_refs"],
        Value::Array(vec![
            Value::String("ifc_label_model_family".into()),
            Value::String("explicit_authority_declassification_family".into()),
        ])
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["typed_reason_family_hint"]["coverage_state"],
        "partial_cluster"
    );
    assert_eq!(
        value["actual_checker_payload_family_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_family_threshold"]["payload_family_kind"],
        "static_reason_code_row_family"
    );
    assert_eq!(
        value["actual_checker_payload_family_threshold"]["source_refs"],
        Value::Array(vec![
            Value::String("fixture_checked_reason_codes".into()),
            Value::String("detached_static_gate_reason_codes".into()),
        ])
    );
    assert_eq!(
        value["actual_checker_payload_row_family_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_family_threshold"]["payload_family_ref"],
        "actual_checker_payload_family"
    );
    assert_eq!(
        value["actual_checker_payload_row_family_threshold"]["row_family_kind"],
        "checked_reason_code_rows"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["payload_row_family_ref"],
        "actual_checker_payload_row_family"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["row_source_ref"],
        "fixture_checked_reason_codes"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["row_reason_kind"],
        "authority_sensitive_success"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["row_body"]["selected_option_ref"],
        "release_authority"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["row_body"]["visibility_target_ref"],
        "room_members"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["payload_row_family_ref"],
        "actual_checker_payload_row_family"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["supported_kind_scope"],
        "stable_clusters_only"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["supported_kind_refs"],
        Value::Array(vec![
            Value::String("missing_lineage_assertion".into()),
            Value::String("lineage_assertion_edge_mismatch".into()),
            Value::String("declared_target_missing".into()),
            Value::String("declared_target_mismatch".into()),
            Value::String("capability_strengthens".into()),
            Value::String("missing_chain_head_option".into()),
            Value::String("missing_predecessor_option".into()),
            Value::String("missing_successor_option".into()),
        ])
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]
            ["actual_checker_payload_family_ref"],
        "actual_checker_payload_family"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]
            ["checker_payload_row_family_ref"],
        "actual_checker_payload_row_family"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]
            ["checker_payload_row_detail_ref"],
        "actual_checker_payload_row_detail"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]["checker_payload_row_body_ref"],
        "actual_checker_payload_row_body"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]
            ["checker_payload_supported_kind_summary_ref"],
        "actual_checker_payload_supported_kind_summary"
    );
    assert_eq!(
        value["actual_public_checker_api_sketch_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_api_sketch_threshold"]["checker_subject"],
        "runtime_try_cut_cluster"
    );
    assert_eq!(
        value["actual_public_checker_api_sketch_threshold"]["public_checker_payload_schema_ref"],
        "public_checker_payload_schema_ready_sketch"
    );
    assert_eq!(
        value["actual_public_checker_entry_criteria_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_entry_criteria_threshold"]["public_checker_api_ref"],
        "public_checker_api_ready_sketch"
    );
    assert_eq!(
        value["actual_public_checker_entry_criteria_threshold"]["next_comparison_target_ref"],
        "public_checker_command_surface_comparison"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["command_surface_kind"],
        "family_facade_checker_commands"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["family_facade_command_refs"][0],
        "same_lineage_checker_command"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["public_checker_api_ref"],
        "public_checker_api_ready_sketch"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["output_contract_kind"],
        "family_checker_row_compare_summary"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["checker_cluster_name"],
        "same_lineage_evidence_floor"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["checker_status"],
        "matched"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["public_checker_payload_schema_ref"],
        "public_checker_payload_schema_ready_sketch"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["boundary_kind"],
        "docs_only_parser_front_checker_boundary"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["public_checker_command_surface_ref"],
        "public_checker_command_surface_ready_sketch"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["shared_output_contract_ref"],
        "shared_output_contract_ready_sketch"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["next_comparison_target_ref"],
        "verifier_handoff_surface_comparison"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["handoff_surface_kind"],
        "docs_only_mixed_row_bundle_verifier_surface"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["public_checker_boundary_ref"],
        "public_checker_boundary_ready_sketch"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["proof_obligation_matrix_ref"],
        "current_l2_proof_obligation_matrix"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["handoff_artifact_mode"],
        "docs_only_mixed_row_bundle"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["next_comparison_target_ref"],
        "minimal_parser_subset_freeze_comparison"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["freeze_kind"],
        "stage1_stage2_structural_parser_floor"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["accepted_cluster_refs"][0],
        "stage1_chain_declaration_structural_floor"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["accepted_cluster_refs"][1],
        "stage2_try_rollback_structural_floor"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["reject_cluster_refs"][0],
        "missing_edge_local_lineage_metadata"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["reject_cluster_refs"][1],
        "missing_fallback_body"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["reject_cluster_refs"][2],
        "atomic_cut_fallback_placement"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["retention_floor_refs"][0],
        "stage3_admit_slot_branch"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["retention_floor_refs"][1],
        "stage3_request_clause_branch"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["retention_floor_refs"][2],
        "stage3_predicate_fragment_branch"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["next_comparison_target_ref"],
        "parser_to_checker_reconnect_freeze_comparison"
    );
}

#[test]
fn operational_cli_pretty_reports_ifc_authority_miss_checker_hint_preview() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        typed_prototype_sample_path("p11-typed-unauthorized-fingerprint-release.txt")
            .to_str()
            .unwrap(),
        "--format",
        "pretty",
    ])
    .unwrap();

    assert!(output.contains("typed_checker_hint_preview:"));
    assert!(output.contains("status: reached"));
    assert!(output.contains("cluster_kind: ifc_authority_release_cluster"));
    assert!(output.contains("case_label: authority_miss_negative"));
    assert!(output.contains("family_refs:"));
    assert!(output.contains("ifc_label_model_family"));
    assert!(output.contains("explicit_authority_declassification_family"));
    assert!(output.contains("coverage_state: partial_cluster"));
    assert!(output.contains("actual_checker_payload_family_threshold:"));
    assert!(output.contains("payload_family_kind: static_reason_code_row_family"));
    assert!(output.contains("source_refs:"));
    assert!(output.contains("fixture_checked_reason_codes"));
    assert!(output.contains("detached_static_gate_reason_codes"));
    assert!(output.contains("actual_checker_payload_row_family_threshold:"));
    assert!(output.contains("payload_family_ref: actual_checker_payload_family"));
    assert!(output.contains("row_family_kind: checked_reason_code_rows"));
    assert!(output.contains("actual_checker_payload_row_detail_threshold:"));
    assert!(output.contains(
        "payload_row_family_ref: actual_checker_payload_row_family"
    ));
    assert!(output.contains("row_source_ref: fixture_checked_reason_codes"));
    assert!(output.contains("row_reason_kind: authority_miss_negative"));
    assert!(output.contains("actual_checker_payload_row_body_threshold:"));
    assert!(output.contains("selected_option_ref: fingerprint_holder"));
    assert!(output.contains("visibility_target_ref: room_members"));
    assert!(output.contains("actual_checker_payload_supported_kind_summary_threshold:"));
    assert!(output.contains("supported_kind_scope: stable_clusters_only"));
    assert!(output.contains("missing_lineage_assertion"));
    assert!(output.contains("missing_successor_option"));
    assert!(output.contains("actual_checker_payload_public_schema_sketch_threshold:"));
    assert!(output.contains(
        "actual_checker_payload_family_ref: actual_checker_payload_family"
    ));
    assert!(output.contains(
        "checker_payload_supported_kind_summary_ref: actual_checker_payload_supported_kind_summary"
    ));
    assert!(output.contains("actual_public_checker_api_sketch_threshold:"));
    assert!(output.contains("checker_subject: runtime_try_cut_cluster"));
    assert!(output.contains(
        "public_checker_payload_schema_ref: public_checker_payload_schema_ready_sketch"
    ));
    assert!(output.contains("actual_public_checker_entry_criteria_threshold:"));
    assert!(output.contains("public_checker_api_ref: public_checker_api_ready_sketch"));
    assert!(output.contains(
        "next_comparison_target_ref: public_checker_command_surface_comparison"
    ));
    assert!(output.contains("smoke-same-lineage-checker"));
    assert!(output.contains("actual_public_checker_command_surface_threshold:"));
    assert!(output.contains(
        "command_surface_kind: family_facade_checker_commands"
    ));
    assert!(output.contains("same_lineage_checker_command"));
    assert!(output.contains("public_checker_api_ref: public_checker_api_ready_sketch"));
    assert!(output.contains("actual_shared_output_contract_threshold:"));
    assert!(output.contains(
        "output_contract_kind: family_checker_row_compare_summary"
    ));
    assert!(output.contains("checker_cluster_name: same_lineage_evidence_floor"));
    assert!(output.contains("checker_status: matched"));
    assert!(output.contains(
        "public_checker_payload_schema_ref: public_checker_payload_schema_ready_sketch"
    ));
    assert!(output.contains(
        "next_comparison_target_ref: public_checker_boundary_comparison"
    ));
    assert!(output.contains("actual_public_checker_boundary_threshold:"));
    assert!(output.contains(
        "boundary_kind: docs_only_parser_front_checker_boundary"
    ));
    assert!(output.contains(
        "public_checker_command_surface_ref: public_checker_command_surface_ready_sketch"
    ));
    assert!(output.contains(
        "shared_output_contract_ref: shared_output_contract_ready_sketch"
    ));
    assert!(output.contains(
        "next_comparison_target_ref: verifier_handoff_surface_comparison"
    ));
    assert!(output.contains("actual_verifier_handoff_surface_threshold:"));
    assert!(output.contains(
        "handoff_surface_kind: docs_only_mixed_row_bundle_verifier_surface"
    ));
    assert!(output.contains(
        "public_checker_boundary_ref: public_checker_boundary_ready_sketch"
    ));
    assert!(output.contains(
        "proof_obligation_matrix_ref: current_l2_proof_obligation_matrix"
    ));
    assert!(output.contains(
        "handoff_artifact_mode: docs_only_mixed_row_bundle"
    ));
    assert!(output.contains(
        "next_comparison_target_ref: minimal_parser_subset_freeze_comparison"
    ));
    assert!(output.contains("actual_minimal_parser_subset_freeze_threshold:"));
    assert!(output.contains(
        "freeze_kind: stage1_stage2_structural_parser_floor"
    ));
    assert!(output.contains(
        "accepted_cluster_refs:"
    ));
    assert!(output.contains(
        "stage1_chain_declaration_structural_floor"
    ));
    assert!(output.contains(
        "stage2_try_rollback_structural_floor"
    ));
    assert!(output.contains(
        "reject_cluster_refs:"
    ));
    assert!(output.contains("missing_edge_local_lineage_metadata"));
    assert!(output.contains("missing_fallback_body"));
    assert!(output.contains("atomic_cut_fallback_placement"));
    assert!(output.contains(
        "retention_floor_refs:"
    ));
    assert!(output.contains("stage3_admit_slot_branch"));
    assert!(output.contains("stage3_request_clause_branch"));
    assert!(output.contains("stage3_predicate_fragment_branch"));
    assert!(output.contains(
        "next_comparison_target_ref: parser_to_checker_reconnect_freeze_comparison"
    ));
}

#[test]
fn operational_cli_json_reports_ifc_label_flow_checker_hint_preview() {
    let output = run_current_l2_operational_cli([
        "run-source-sample",
        typed_prototype_sample_path("p12-typed-classified-fingerprint-publication-block.txt")
            .to_str()
            .unwrap(),
        "--format",
        "json",
    ])
    .unwrap();

    let value: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(value["typed_checker_hint_preview"]["status"], "reached");
    assert_eq!(
        value["typed_checker_hint_preview"]["cluster_kind"],
        "ifc_label_flow_cluster"
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["case_label"],
        "classified_to_public_negative"
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["typed_reason_family_hint"]["family_refs"],
        Value::Array(vec![
            Value::String("ifc_label_model_family".into()),
            Value::String("classified_public_label_flow_family".into()),
        ])
    );
    assert_eq!(
        value["typed_checker_hint_preview"]["typed_reason_family_hint"]["coverage_state"],
        "full_cluster"
    );
    assert_eq!(
        value["actual_checker_payload_family_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_family_threshold"]["coverage_state"],
        "full_cluster"
    );
    assert_eq!(
        value["actual_checker_payload_row_family_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_family_threshold"]["coverage_state"],
        "full_cluster"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["coverage_state"],
        "full_cluster"
    );
    assert_eq!(
        value["actual_checker_payload_row_detail_threshold"]["row_reason_kind"],
        "classified_to_public_negative"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["row_body"]["selected_option_ref"],
        "classified_holder"
    );
    assert_eq!(
        value["actual_checker_payload_row_body_threshold"]["row_body"]["visibility_target_ref"],
        "public_board"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_supported_kind_summary_threshold"]["supported_kind_scope"],
        "stable_clusters_only"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_checker_payload_public_schema_sketch_threshold"]["checker_payload_row_body_ref"],
        "actual_checker_payload_row_body"
    );
    assert_eq!(
        value["actual_public_checker_api_sketch_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_api_sketch_threshold"]["checker_subject"],
        "runtime_try_cut_cluster"
    );
    assert_eq!(
        value["actual_public_checker_entry_criteria_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_entry_criteria_threshold"]["next_comparison_target_ref"],
        "public_checker_command_surface_comparison"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_command_surface_threshold"]["next_comparison_target_ref"],
        "shared_output_contract_comparison"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_shared_output_contract_threshold"]["next_comparison_target_ref"],
        "public_checker_boundary_comparison"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_public_checker_boundary_threshold"]["next_comparison_target_ref"],
        "verifier_handoff_surface_comparison"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_verifier_handoff_surface_threshold"]["next_comparison_target_ref"],
        "minimal_parser_subset_freeze_comparison"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["status"],
        "reached"
    );
    assert_eq!(
        value["actual_minimal_parser_subset_freeze_threshold"]["next_comparison_target_ref"],
        "parser_to_checker_reconnect_freeze_comparison"
    );
}
