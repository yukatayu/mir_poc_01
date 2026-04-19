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
    assert_eq!(
        value["typed_checker_hint_preview"]["status"],
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
    assert_eq!(
        value["typed_checker_hint_preview"]["status"],
        "reached"
    );
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
    assert_eq!(
        value["typed_checker_hint_preview"]["status"],
        "reached"
    );
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
}
