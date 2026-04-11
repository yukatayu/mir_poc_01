#[allow(dead_code)]
#[path = "support/current_l2_stage3_request_clause_suite_spike_support.rs"]
mod current_l2_stage3_request_clause_suite_spike_support;
#[allow(dead_code)]
#[path = "support/current_l2_stage3_predicate_fragment_spike_support.rs"]
mod current_l2_stage3_predicate_fragment_spike_support;

use mir_ast::current_l2::parse_stage3_minimal_predicate_fragment_text;

use current_l2_stage3_predicate_fragment_spike_support::{
    load_fixture_request_clause_fragment, load_fixture_request_contract_subset,
    Stage3RequestContractSubset,
};
use current_l2_stage3_request_clause_suite_spike_support::{
    extract_stage3_request_clause_suite,
    Stage3RequestClauseSuite,
};

const SINGLE_LINE_REQUIRE_ENSURE_INPUT: &str = r#"
perform write_profile on profile_doc
  require write
  ensure owner_is(session_user)
"#;

const MULTILINE_REQUIRE_SINGLE_ENSURE_INPUT: &str = r#"
perform write_profile on profile_doc
  require:
    (
      owner_is(session_user)
      and well_formed(profile_draft)
    )
  ensure owner_is(session_user)
"#;

const ENSURE_ONLY_INPUT: &str = r#"
perform load_profile via profile_ref
  ensure owner_is(session_user)
"#;

const ENSURE_BEFORE_REQUIRE_INPUT: &str = r#"
perform write_profile on profile_doc
  ensure owner_is(session_user)
  require write
"#;

const DUPLICATE_REQUIRE_INPUT: &str = r#"
perform write_profile on profile_doc
  require write
  require owner_is(session_user)
"#;

const BLANK_LINE_BETWEEN_CLAUSES_INPUT: &str = r#"
perform write_profile on profile_doc
  require write

  ensure owner_is(session_user)
"#;

const DUPLICATE_ENSURE_INPUT: &str = r#"
perform write_profile on profile_doc
  ensure owner_is(session_user)
  ensure owner_is(session_user)
"#;

const UNSUPPORTED_DIRECT_CHILD_INPUT: &str = r#"
perform write_profile on profile_doc
  require write
  note delegated
"#;

const MISSING_MULTILINE_ENSURE_BLOCK_INPUT: &str = r#"
perform write_profile on profile_doc
  ensure:
"#;

fn parse_suite_contract_subset(
    suite: &Stage3RequestClauseSuite,
) -> Result<Stage3RequestContractSubset, String> {
    Ok(Stage3RequestContractSubset {
        require_fragment: suite
            .require_fragment_text
            .as_deref()
            .map(parse_stage3_minimal_predicate_fragment_text)
            .transpose()?,
        ensure_fragment: suite
            .ensure_fragment_text
            .as_deref()
            .map(parse_stage3_minimal_predicate_fragment_text)
            .transpose()?,
    })
}

#[test]
fn stage3_request_clause_suite_spike_extracts_single_line_require_and_ensure_slots() {
    let suite = extract_stage3_request_clause_suite(SINGLE_LINE_REQUIRE_ENSURE_INPUT)
        .expect("suite spike should extract single-line require/ensure slots");

    assert_eq!(
        suite,
        Stage3RequestClauseSuite {
            require_fragment_text: Some("write".to_string()),
            ensure_fragment_text: Some("owner_is(session_user)".to_string()),
        }
    );
}

#[test]
fn stage3_request_clause_suite_spike_matches_fixture_fragments_for_mixed_suite() {
    let suite = extract_stage3_request_clause_suite(MULTILINE_REQUIRE_SINGLE_ENSURE_INPUT)
        .expect("suite spike should extract mixed multiline/single-line clause slots");

    let require_actual = parse_stage3_minimal_predicate_fragment_text(
        suite
            .require_fragment_text
            .as_deref()
            .expect("require slot should be present"),
    )
    .expect("require fragment should parse");
    let require_expected =
        load_fixture_request_clause_fragment("e2-try-fallback.json", 1, "require", 0)
            .expect("fixture require fragment should load");

    let ensure_actual = parse_stage3_minimal_predicate_fragment_text(
        suite
            .ensure_fragment_text
            .as_deref()
            .expect("ensure slot should be present"),
    )
    .expect("ensure fragment should parse");
    let ensure_expected = load_fixture_request_clause_fragment(
        "e10-perform-on-ensure-failure.json",
        0,
        "ensure",
        0,
    )
    .expect("fixture ensure fragment should load");

    assert_eq!(require_actual, require_expected);
    assert_eq!(ensure_actual, ensure_expected);
}

#[test]
fn stage3_request_clause_suite_spike_allows_ensure_only_suite() {
    let suite = extract_stage3_request_clause_suite(ENSURE_ONLY_INPUT)
        .expect("suite spike should allow ensure-only suite");

    let ensure_actual = parse_stage3_minimal_predicate_fragment_text(
        suite
            .ensure_fragment_text
            .as_deref()
            .expect("ensure slot should be present"),
    )
    .expect("ensure fragment should parse");
    let ensure_expected = load_fixture_request_clause_fragment(
        "e11-perform-via-ensure-then-success.json",
        0,
        "ensure",
        0,
    )
    .expect("fixture ensure fragment should load");

    assert!(suite.require_fragment_text.is_none());
    assert_eq!(ensure_actual, ensure_expected);
}

#[test]
fn stage3_request_clause_suite_spike_rejects_require_after_ensure() {
    let error = extract_stage3_request_clause_suite(ENSURE_BEFORE_REQUIRE_INPUT)
        .expect_err("suite spike should reject require appearing after ensure");

    assert!(
        error.contains("require clause cannot appear after ensure clause"),
        "expected require-after-ensure wording, got: {error}"
    );
}

#[test]
fn stage3_request_clause_suite_spike_rejects_duplicate_require_clause() {
    let error = extract_stage3_request_clause_suite(DUPLICATE_REQUIRE_INPUT)
        .expect_err("suite spike should reject duplicate require clause");

    assert!(
        error.contains("duplicate `require` clause is not allowed"),
        "expected duplicate-require wording, got: {error}"
    );
}

#[test]
fn stage3_request_clause_suite_spike_rejects_blank_line_between_clauses() {
    let error = extract_stage3_request_clause_suite(BLANK_LINE_BETWEEN_CLAUSES_INPUT)
        .expect_err("suite spike should reject blank line between clauses");

    assert!(
        error.contains("blank line is not allowed between request-local clauses"),
        "expected clause-between-blank-line wording, got: {error}"
    );
}

#[test]
fn stage3_request_clause_suite_spike_rejects_duplicate_ensure_clause() {
    let error = extract_stage3_request_clause_suite(DUPLICATE_ENSURE_INPUT)
        .expect_err("suite spike should reject duplicate ensure clause");

    assert!(
        error.contains("duplicate `ensure` clause is not allowed"),
        "expected duplicate-ensure wording, got: {error}"
    );
}

#[test]
fn stage3_request_clause_suite_spike_rejects_unsupported_direct_child_line() {
    let error = extract_stage3_request_clause_suite(UNSUPPORTED_DIRECT_CHILD_INPUT)
        .expect_err("suite spike should reject unsupported direct child line");

    assert!(
        error.contains("unsupported request-local clause line inside fixed two-slot suite"),
        "expected suite-local unsupported-child wording, got: {error}"
    );
    assert!(
        error.contains("note delegated"),
        "expected offending line text to be preserved, got: {error}"
    );
}

#[test]
fn stage3_request_clause_suite_spike_rejects_missing_multiline_ensure_block() {
    let error = extract_stage3_request_clause_suite(MISSING_MULTILINE_ENSURE_BLOCK_INPUT)
        .expect_err("suite spike should reject missing multiline ensure block");

    assert!(
        error.contains("missing multiline predicate block after ensure:"),
        "expected missing-multiline-ensure wording, got: {error}"
    );
}

#[test]
fn stage3_request_clause_suite_spike_matches_fixture_contract_subset_for_perform_on() {
    let suite = extract_stage3_request_clause_suite(SINGLE_LINE_REQUIRE_ENSURE_INPUT)
        .expect("suite spike should extract single-line require/ensure slots");
    let actual = parse_suite_contract_subset(&suite)
        .expect("suite spike should parse extracted slots into a contract subset");
    let expected =
        load_fixture_request_contract_subset("e10-perform-on-ensure-failure.json", 0)
            .expect("fixture contract subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_request_clause_suite_spike_matches_fixture_contract_subset_for_perform_via() {
    let suite = extract_stage3_request_clause_suite(SINGLE_LINE_REQUIRE_ENSURE_INPUT)
        .expect("suite spike should extract single-line require/ensure slots");
    let actual = parse_suite_contract_subset(&suite)
        .expect("suite spike should parse extracted slots into a contract subset");
    let expected =
        load_fixture_request_contract_subset("e11-perform-via-ensure-then-success.json", 0)
            .expect("fixture contract subset should load");

    assert_eq!(actual, expected);
}
