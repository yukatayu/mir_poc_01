#[path = "support/current_l2_stage2_try_rollback_spike_support.rs"]
mod current_l2_stage2_try_rollback_spike_support;

use current_l2_stage2_try_rollback_spike_support::{
    Stage2TryRollbackFindingRow, Stage2TryRollbackStructuralSummary,
    load_expected_try_rollback_expectation, parse_stage2_try_rollback_text,
    summarize_stage2_try_rollback_findings,
};

const E23_INPUT: &str = r#"
try {
  perform stage_profile_patch on profile_draft
} fallback {
}
"#;

const E24_INPUT: &str = r#"
try {
  perform stage_profile_patch on profile_draft
} fallback {
  atomic_cut
}
"#;

const VALID_CUT_IN_TRY_BODY_INPUT: &str = r#"
try {
  perform stage_profile_patch on profile_draft
  atomic_cut
  perform validate_profile_patch on profile_draft
} fallback {
  perform load_last_snapshot on profile_snapshot
}
"#;

#[test]
fn stage2_try_rollback_spike_matches_e23_fixture_expectation() {
    let parsed =
        parse_stage2_try_rollback_text(E23_INPUT).expect("stage 2 spike should parse e23 input");
    let actual = summarize_stage2_try_rollback_findings(&parsed);
    let expected = load_expected_try_rollback_expectation(
        "e23-malformed-try-fallback-missing-fallback-body.json",
    )
    .expect("expected e23 try/rollback expectation should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage2_try_rollback_spike_matches_e24_fixture_expectation() {
    let parsed =
        parse_stage2_try_rollback_text(E24_INPUT).expect("stage 2 spike should parse e24 input");
    let actual = summarize_stage2_try_rollback_findings(&parsed);
    let expected = load_expected_try_rollback_expectation(
        "e24-malformed-atomic-cut-fallback-placement.json",
    )
    .expect("expected e24 try/rollback expectation should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage2_try_rollback_spike_marks_no_findings_for_atomic_cut_in_try_body() {
    let parsed = parse_stage2_try_rollback_text(VALID_CUT_IN_TRY_BODY_INPUT)
        .expect("stage 2 spike should parse valid try/rollback input");

    assert_eq!(
        summarize_stage2_try_rollback_findings(&parsed),
        Stage2TryRollbackStructuralSummary {
            verdict: "no_findings".to_string(),
            findings: Vec::<Stage2TryRollbackFindingRow>::new(),
        }
    );
}
