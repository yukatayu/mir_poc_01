use std::path::{Path, PathBuf};

use mir_runtime::current_l2_cli::run_current_l2_operational_cli;
use mir_semantics::{FixtureHostPlan, load_host_plan_from_path};
use serde_json::Value;

#[path = "support/current_l2_source_sample_emitted_artifact_support.rs"]
mod current_l2_source_sample_emitted_artifact_support;

use current_l2_source_sample_emitted_artifact_support::{
    CurrentL2EmittedArtifactRouteStatus, CurrentL2SourceSampleMinimalCompanionSurface,
    CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface,
    CurrentL2SourceSampleStageBlockSurface, build_current_l2_source_sample_minimal_companion_surface,
    build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface,
    build_current_l2_source_sample_stage_block_surface,
};

fn order_handoff_prototype_sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/prototype/current-l2-order-handoff")
        .join(name)
}

fn prototype_host_plan_path(sample_path: &Path) -> PathBuf {
    sample_path.with_extension("host-plan.json")
}

fn prototype_host_plan(sample_path: &Path) -> FixtureHostPlan {
    load_host_plan_from_path(&prototype_host_plan_path(sample_path)).unwrap()
}

fn cli_json(sample_argument: &str, host_plan_path: Option<&Path>) -> Value {
    let mut args = vec!["run-source-sample".to_string(), sample_argument.to_string()];
    if let Some(host_plan_path) = host_plan_path {
        args.push("--host-plan".to_string());
        args.push(host_plan_path.to_str().unwrap().to_string());
    }
    args.push("--format".to_string());
    args.push("json".to_string());

    let output = run_current_l2_operational_cli(args).unwrap();
    serde_json::from_str(&output).unwrap()
}

fn route_status_name(status: CurrentL2EmittedArtifactRouteStatus) -> &'static str {
    match status {
        CurrentL2EmittedArtifactRouteStatus::Reached => "reached",
        CurrentL2EmittedArtifactRouteStatus::GuardedNotReached => "guarded_not_reached",
    }
}

fn preview_string_list(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap().to_string())
        .collect()
}

fn assert_preview_entry(
    preview_json: &Value,
    key: &str,
    expected_status: CurrentL2EmittedArtifactRouteStatus,
    expected_guard_reason: Option<&str>,
    expected_lines_key: &str,
    expected_lines: &[String],
    expected_guard_refs: &[String],
    expected_kept_later_refs: &[String],
) {
    let entry = &preview_json["surface_preview"][key];
    assert_eq!(entry["status"], route_status_name(expected_status));
    match expected_guard_reason {
        Some(reason) => assert_eq!(entry["guard_reason"], reason),
        None => assert!(entry["guard_reason"].is_null()),
    }
    assert_eq!(preview_string_list(&entry[expected_lines_key]), expected_lines);
    assert_eq!(preview_string_list(&entry["guard_refs"]), expected_guard_refs);
    assert_eq!(
        preview_string_list(&entry["kept_later_refs"]),
        expected_kept_later_refs
    );
}

fn assert_cli_surface_preview_matches_builders(sample_path: &Path) {
    let preview_json = cli_json(sample_path.to_str().unwrap(), None);
    let minimal =
        build_current_l2_source_sample_minimal_companion_surface(
            sample_path.to_str().unwrap(),
            prototype_host_plan(sample_path),
        )
        .unwrap();
    let stage =
        build_current_l2_source_sample_stage_block_surface(
            sample_path.to_str().unwrap(),
            prototype_host_plan(sample_path),
        )
        .unwrap();
    let serial =
        build_current_l2_source_sample_order_handoff_serial_scope_reserve_surface(
            sample_path.to_str().unwrap(),
            prototype_host_plan(sample_path),
        )
        .unwrap();

    let CurrentL2SourceSampleMinimalCompanionSurface {
        surface_status,
        surface_guard_reason,
        companion_lines,
        guard_refs,
        kept_later_refs,
        ..
    } = minimal;
    assert_preview_entry(
        &preview_json,
        "minimal_companion",
        surface_status,
        surface_guard_reason.as_deref(),
        "lines",
        &companion_lines,
        &guard_refs,
        &kept_later_refs,
    );

    let CurrentL2SourceSampleStageBlockSurface {
        surface_status,
        surface_guard_reason,
        stage_lines,
        guard_refs,
        kept_later_refs,
        ..
    } = stage;
    assert_preview_entry(
        &preview_json,
        "stage_block_secondary",
        surface_status,
        surface_guard_reason.as_deref(),
        "lines",
        &stage_lines,
        &guard_refs,
        &kept_later_refs,
    );

    let CurrentL2SourceSampleOrderHandoffSerialScopeReserveSurface {
        surface_status,
        surface_guard_reason,
        serial_scope_lines,
        guard_refs,
        kept_later_refs,
        ..
    } = serial;
    assert_preview_entry(
        &preview_json,
        "serial_scope_reserve",
        surface_status,
        surface_guard_reason.as_deref(),
        "lines",
        &serial_scope_lines,
        &guard_refs,
        &kept_later_refs,
    );
}

#[test]
fn order_handoff_surface_preview_cli_matches_builders_for_late_join_sample() {
    let sample_path = order_handoff_prototype_sample_path("p07-dice-late-join-visible-history.txt");
    assert_cli_surface_preview_matches_builders(&sample_path);
}

#[test]
fn order_handoff_surface_preview_cli_matches_builders_for_stale_reconnect_sample() {
    let sample_path = order_handoff_prototype_sample_path("p08-dice-stale-reconnect-refresh.txt");
    assert_cli_surface_preview_matches_builders(&sample_path);
}

#[test]
fn order_handoff_surface_preview_cli_matches_builders_for_delegated_provider_sample() {
    let sample_path =
        order_handoff_prototype_sample_path("p09-dice-delegated-rng-provider-placement.txt");
    assert_cli_surface_preview_matches_builders(&sample_path);
}
