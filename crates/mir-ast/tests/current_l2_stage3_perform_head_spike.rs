#[allow(dead_code)]
#[path = "support/current_l2_stage3_predicate_fragment_spike_support.rs"]
mod current_l2_stage3_predicate_fragment_spike_support;

use mir_ast::current_l2::{
    Stage3PerformHead, Stage3PerformTargetRef, parse_stage3_perform_head_text,
};

use current_l2_stage3_predicate_fragment_spike_support::load_fixture_perform_head;

const PERFORM_ON_WITH_SUITE_INPUT: &str = r#"
perform update_authority on profile_authority
  require write
  ensure owner_is(session_user)
"#;

const PERFORM_VIA_WITH_SUITE_INPUT: &str = r#"
perform write_profile via profile_ref
  require write
  ensure owner_is(session_user)
"#;

const MISSING_ON_TARGET_INPUT: &str = r#"
perform write_profile on
"#;

const MISSING_VIA_TARGET_INPUT: &str = r#"
perform write_profile via
"#;

const UNSUPPORTED_CHANNEL_INPUT: &str = r#"
perform write_profile at profile_ref
"#;

const EXTRA_TOKEN_INPUT: &str = r#"
perform write_profile on profile_ref extra
"#;

#[test]
fn stage3_perform_head_spike_matches_fixture_subset_for_perform_on() {
    let actual = parse_stage3_perform_head_text(PERFORM_ON_WITH_SUITE_INPUT)
        .expect("perform-head spike should extract perform-on head");
    let expected = load_fixture_perform_head("e10-perform-on-ensure-failure.json", 0)
        .expect("fixture perform-on head should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_perform_head_spike_matches_fixture_subset_for_perform_via() {
    let actual = parse_stage3_perform_head_text(PERFORM_VIA_WITH_SUITE_INPUT)
        .expect("perform-head spike should extract perform-via head");
    let expected = load_fixture_perform_head("e11-perform-via-ensure-then-success.json", 0)
        .expect("fixture perform-via head should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_perform_head_spike_extracts_target_kind_and_payload() {
    let perform_on = parse_stage3_perform_head_text(PERFORM_ON_WITH_SUITE_INPUT)
        .expect("perform-on head should parse");
    let perform_via = parse_stage3_perform_head_text(PERFORM_VIA_WITH_SUITE_INPUT)
        .expect("perform-via head should parse");

    assert_eq!(
        perform_on,
        Stage3PerformHead {
            op: "update_authority".to_string(),
            target_ref: Stage3PerformTargetRef::On("profile_authority".to_string()),
        }
    );
    assert_eq!(
        perform_via,
        Stage3PerformHead {
            op: "write_profile".to_string(),
            target_ref: Stage3PerformTargetRef::Via("profile_ref".to_string()),
        }
    );
}

#[test]
fn stage3_perform_head_spike_rejects_missing_on_target() {
    let error = parse_stage3_perform_head_text(MISSING_ON_TARGET_INPUT)
        .expect_err("perform-head spike should reject missing `on` target");

    assert!(
        error.contains("missing perform-on target after `on`"),
        "expected missing perform-on target wording, got: {error}"
    );
}

#[test]
fn stage3_perform_head_spike_rejects_missing_via_target() {
    let error = parse_stage3_perform_head_text(MISSING_VIA_TARGET_INPUT)
        .expect_err("perform-head spike should reject missing `via` target");

    assert!(
        error.contains("missing perform-via chain ref after `via`"),
        "expected missing perform-via target wording, got: {error}"
    );
}

#[test]
fn stage3_perform_head_spike_rejects_unsupported_channel_keyword() {
    let error = parse_stage3_perform_head_text(UNSUPPORTED_CHANNEL_INPUT)
        .expect_err("perform-head spike should reject unsupported channel keyword");

    assert!(
        error.contains("perform head must use `on` or `via`"),
        "expected unsupported-channel wording, got: {error}"
    );
}

#[test]
fn stage3_perform_head_spike_rejects_extra_tokens_after_target() {
    let error = parse_stage3_perform_head_text(EXTRA_TOKEN_INPUT)
        .expect_err("perform-head spike should reject extra tokens after target");

    assert!(
        error.contains("unsupported perform head shape"),
        "expected extra-token wording, got: {error}"
    );
}
