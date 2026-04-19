use std::path::{Path, PathBuf};

use mir_runtime::current_l2::run_current_l2_source_sample;
use mir_semantics::{FixtureHostPlan, StaticGateVerdict, load_host_plan_from_path};

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

#[test]
fn order_handoff_missing_publication_witness_stops_before_runtime() {
    let sample_path =
        order_handoff_prototype_sample_path("p13-dice-late-join-missing-publication-witness.txt");
    let report = run_current_l2_source_sample(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        report.sample_id,
        "p13-dice-late-join-missing-publication-witness"
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Underdeclared
    );
    assert!(
        report
            .runtime_report
            .checker_floor
            .static_gate
            .reasons
            .iter()
            .any(|reason| reason
                == "missing publication witness before handoff for late-join visibility at root / room / dice_authority")
    );
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}

#[test]
fn order_handoff_publish_after_handoff_stops_before_runtime() {
    let sample_path =
        order_handoff_prototype_sample_path("p14-dice-late-join-handoff-before-publication.txt");
    let report = run_current_l2_source_sample(
        sample_path.to_str().unwrap(),
        prototype_host_plan(&sample_path),
    )
    .unwrap();

    assert_eq!(
        report.sample_id,
        "p14-dice-late-join-handoff-before-publication"
    );
    assert_eq!(
        report.runtime_report.checker_floor.static_gate.verdict,
        StaticGateVerdict::Malformed
    );
    assert!(
        report
            .runtime_report
            .checker_floor
            .static_gate
            .reasons
            .iter()
            .any(|reason| reason
                == "handoff appears before publish for late-join visibility at root / room / dice_authority")
    );
    assert!(!report.runtime_report.run_report.entered_evaluation);
    assert_eq!(report.runtime_report.run_report.terminal_outcome, None);
}
