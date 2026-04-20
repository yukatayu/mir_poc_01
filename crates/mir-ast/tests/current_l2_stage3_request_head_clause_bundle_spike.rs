#[allow(dead_code)]
#[path = "support/current_l2_stage3_predicate_fragment_spike_support.rs"]
mod current_l2_stage3_predicate_fragment_spike_support;

use mir_ast::current_l2::{
    Stage3RequestAttachmentFrameKind, Stage3RequestClauseSuite, Stage3RequestHeadClauseBundle,
    parse_stage3_minimal_predicate_fragment_text, parse_stage3_request_head_clause_bundle_text,
};

use current_l2_stage3_predicate_fragment_spike_support::{
    Stage3RequestContractSubset, load_fixture_perform_head, load_fixture_request_contract_subset,
};

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

const HEAD_ONLY_INPUT: &str = r#"
perform refresh_profile via profile_ref
"#;

const UNSUPPORTED_DIRECT_CHILD_INPUT: &str = r#"
perform write_profile on profile_doc
  require write
  note delegated
"#;

const MISSING_ON_TARGET_INPUT: &str = r#"
perform write_profile on
"#;

fn parse_bundle_contract_subset(
    bundle: &Stage3RequestHeadClauseBundle,
) -> Result<Stage3RequestContractSubset, String> {
    Ok(Stage3RequestContractSubset {
        require_fragment: bundle
            .clause_suite
            .require_fragment_text
            .as_deref()
            .map(parse_stage3_minimal_predicate_fragment_text)
            .transpose()?,
        ensure_fragment: bundle
            .clause_suite
            .ensure_fragment_text
            .as_deref()
            .map(parse_stage3_minimal_predicate_fragment_text)
            .transpose()?,
    })
}

#[test]
fn stage3_request_head_clause_bundle_spike_matches_fixture_subset_for_perform_on_suite() {
    let actual = parse_stage3_request_head_clause_bundle_text(PERFORM_ON_WITH_SUITE_INPUT)
        .expect("bundle spike should extract perform-on head with request clause suite");
    let expected_head = load_fixture_perform_head("e10-perform-on-ensure-failure.json", 0)
        .expect("fixture perform-on head should load");
    let expected_contract =
        load_fixture_request_contract_subset("e10-perform-on-ensure-failure.json", 0)
            .expect("fixture perform-on contract subset should load");

    assert_eq!(actual.perform_head, expected_head);
    assert_eq!(
        parse_bundle_contract_subset(&actual).expect("bundle contract subset should parse"),
        expected_contract
    );
    assert_eq!(
        actual.attachment_frame_kind,
        Stage3RequestAttachmentFrameKind::RequestLocalTwoSlotSuite
    );
}

#[test]
fn stage3_request_head_clause_bundle_spike_matches_fixture_subset_for_perform_via_suite() {
    let actual = parse_stage3_request_head_clause_bundle_text(PERFORM_VIA_WITH_SUITE_INPUT)
        .expect("bundle spike should extract perform-via head with request clause suite");
    let expected_head = load_fixture_perform_head("e11-perform-via-ensure-then-success.json", 0)
        .expect("fixture perform-via head should load");
    let expected_contract =
        load_fixture_request_contract_subset("e11-perform-via-ensure-then-success.json", 0)
            .expect("fixture perform-via contract subset should load");

    assert_eq!(actual.perform_head, expected_head);
    assert_eq!(
        parse_bundle_contract_subset(&actual).expect("bundle contract subset should parse"),
        expected_contract
    );
    assert_eq!(
        actual.attachment_frame_kind,
        Stage3RequestAttachmentFrameKind::RequestLocalTwoSlotSuite
    );
}

#[test]
fn stage3_request_head_clause_bundle_spike_allows_head_only_bundle_with_empty_suite() {
    let actual = parse_stage3_request_head_clause_bundle_text(HEAD_ONLY_INPUT)
        .expect("bundle spike should allow head-only bundle");

    assert_eq!(
        actual.clause_suite,
        Stage3RequestClauseSuite {
            require_fragment_text: None,
            ensure_fragment_text: None,
        }
    );
    assert_eq!(
        actual.attachment_frame_kind,
        Stage3RequestAttachmentFrameKind::RequestLocalTwoSlotSuite
    );
}

#[test]
fn stage3_request_head_clause_bundle_spike_reuses_request_clause_suite_failure() {
    let error = parse_stage3_request_head_clause_bundle_text(UNSUPPORTED_DIRECT_CHILD_INPUT)
        .expect_err("bundle spike should reject unsupported direct child line");

    assert!(
        error.contains("unsupported request-local clause line inside fixed two-slot suite"),
        "expected request clause suite failure wording, got: {error}"
    );
    assert!(
        error.contains("note delegated"),
        "expected offending child line to be preserved, got: {error}"
    );
}

#[test]
fn stage3_request_head_clause_bundle_spike_reuses_perform_head_failure() {
    let error = parse_stage3_request_head_clause_bundle_text(MISSING_ON_TARGET_INPUT)
        .expect_err("bundle spike should reject missing perform-on target");

    assert!(
        error.contains("missing perform-on target after `on`"),
        "expected perform-head failure wording, got: {error}"
    );
}
