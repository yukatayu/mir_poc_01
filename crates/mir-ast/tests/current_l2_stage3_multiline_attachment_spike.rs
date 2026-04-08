#[allow(dead_code)]
#[path = "support/current_l2_stage3_multiline_attachment_spike_support.rs"]
mod current_l2_stage3_multiline_attachment_spike_support;
#[allow(dead_code)]
#[path = "support/current_l2_stage3_predicate_fragment_spike_support.rs"]
mod current_l2_stage3_predicate_fragment_spike_support;

use current_l2_stage3_multiline_attachment_spike_support::{
    extract_stage3_option_admit_multiline_fragment_text,
    extract_stage3_request_clause_multiline_fragment_text,
};
use current_l2_stage3_predicate_fragment_spike_support::{
    Stage3PredicateFragment, load_fixture_request_clause_fragment,
    parse_stage3_minimal_predicate_fragment_text,
};

const OPTION_ADMIT_MULTILINE_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live
  admit:
    (
      owner_is(session_user)
      and well_formed(profile_draft)
    )
"#;

const REQUEST_MULTILINE_INPUT: &str = r#"
perform write_profile on profile_doc
  require:
    (
      owner_is(session_user)
      and well_formed(profile_draft)
    )
  ensure:
    owner_is(session_user)
"#;

const MISSING_ADMIT_BLOCK_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live
  admit:
"#;

const MISSING_REQUIRE_BLOCK_INPUT: &str = r#"
perform write_profile on profile_doc
  require:
  ensure:
    owner_is(session_user)
"#;

const NESTED_ENSURE_LIKE_LINE_INPUT: &str = r#"
perform write_profile on profile_doc
  require:
    owner_is(session_user)
    ensure:
      owner_is(session_user)
"#;

const BLANK_LINE_IN_ADMIT_BLOCK_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live
  admit:
    owner_is(session_user)

    well_formed(profile_draft)
"#;

#[test]
fn stage3_multiline_attachment_spike_extracts_option_local_admit_fragment() {
    let extracted = extract_stage3_option_admit_multiline_fragment_text(OPTION_ADMIT_MULTILINE_INPUT)
        .expect("multiline attachment spike should extract option-local admit block");
    let actual = parse_stage3_minimal_predicate_fragment_text(&extracted)
        .expect("extracted admit fragment should parse");
    let expected =
        load_fixture_request_clause_fragment("e2-try-fallback.json", 1, "require", 0)
            .expect("fixture grouped and fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_multiline_attachment_spike_extracts_request_local_require_fragment() {
    let extracted =
        extract_stage3_request_clause_multiline_fragment_text(REQUEST_MULTILINE_INPUT, "require")
            .expect("multiline attachment spike should extract request-local require block");
    let actual = parse_stage3_minimal_predicate_fragment_text(&extracted)
        .expect("extracted require fragment should parse");
    let expected =
        load_fixture_request_clause_fragment("e2-try-fallback.json", 1, "require", 0)
            .expect("fixture grouped and fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_multiline_attachment_spike_extracts_request_local_ensure_fragment() {
    let extracted =
        extract_stage3_request_clause_multiline_fragment_text(REQUEST_MULTILINE_INPUT, "ensure")
            .expect("multiline attachment spike should extract request-local ensure block");
    let actual = parse_stage3_minimal_predicate_fragment_text(&extracted)
        .expect("extracted ensure fragment should parse");
    let expected = load_fixture_request_clause_fragment(
        "e10-perform-on-ensure-failure.json",
        0,
        "ensure",
        0,
    )
    .expect("fixture ensure fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_multiline_attachment_spike_keeps_grouped_and_structure() {
    let extracted = extract_stage3_option_admit_multiline_fragment_text(OPTION_ADMIT_MULTILINE_INPUT)
        .expect("multiline attachment spike should extract option-local admit block");
    let actual = parse_stage3_minimal_predicate_fragment_text(&extracted)
        .expect("extracted admit fragment should parse");

    assert_eq!(
        actual,
        Stage3PredicateFragment::And {
            terms: vec![
                Stage3PredicateFragment::Call {
                    callee: "owner_is".to_string(),
                    args: vec!["session_user".to_string()],
                },
                Stage3PredicateFragment::Call {
                    callee: "well_formed".to_string(),
                    args: vec!["profile_draft".to_string()],
                },
            ],
        }
    );
}

#[test]
fn stage3_multiline_attachment_spike_rejects_missing_admit_block_payload() {
    let error = extract_stage3_option_admit_multiline_fragment_text(MISSING_ADMIT_BLOCK_INPUT)
        .expect_err("multiline attachment spike should reject missing admit block payload");

    assert!(
        error.contains("missing multiline predicate block after admit:"),
        "expected missing-admit-block wording, got: {error}"
    );
}

#[test]
fn stage3_multiline_attachment_spike_rejects_missing_require_block_payload() {
    let error =
        extract_stage3_request_clause_multiline_fragment_text(MISSING_REQUIRE_BLOCK_INPUT, "require")
            .expect_err("multiline attachment spike should reject missing require block payload");

    assert!(
        error.contains("missing multiline predicate block after require:"),
        "expected missing-require-block wording, got: {error}"
    );
}

#[test]
fn stage3_multiline_attachment_spike_rejects_nested_clause_header_search() {
    let error = extract_stage3_request_clause_multiline_fragment_text(
        NESTED_ENSURE_LIKE_LINE_INPUT,
        "ensure",
    )
    .expect_err("multiline attachment spike should not search nested clause-like lines");

    assert!(
        error.contains("missing `ensure:` attachment header"),
        "expected missing-ensure-header wording, got: {error}"
    );
}

#[test]
fn stage3_multiline_attachment_spike_rejects_blank_line_inside_block() {
    let error = extract_stage3_option_admit_multiline_fragment_text(BLANK_LINE_IN_ADMIT_BLOCK_INPUT)
        .expect_err("multiline attachment spike should reject blank lines inside block");

    assert!(
        error.contains("blank line is not allowed inside multiline predicate block after admit:"),
        "expected blank-line wording, got: {error}"
    );
}
