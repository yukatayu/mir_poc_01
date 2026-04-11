#[path = "support/current_l2_stage3_predicate_fragment_spike_support.rs"]
mod current_l2_stage3_predicate_fragment_spike_support;

use mir_ast::current_l2::{
    Stage3PredicateFragment, parse_stage3_minimal_predicate_fragment_text,
};

use current_l2_stage3_predicate_fragment_spike_support::{
    load_fixture_option_admit_fragment,
    load_fixture_request_clause_fragment,
};

const E3_OWNER_WRITER_ADMIT: &str = "owner_is(session_user)";
const E3_DELEGATED_WRITER_ADMIT: &str = "delegate_granted(session_user)";
const E10_REQUIRE_FRAGMENT: &str = "write";
const E10_ENSURE_FRAGMENT: &str = "owner_is(session_user)";
const E11_ENSURE_FRAGMENT: &str = "owner_is(session_user)";
const E2_GROUPED_AND_FRAGMENT: &str = "(owner_is(session_user) and well_formed(profile_draft))";

#[test]
fn stage3_predicate_fragment_spike_matches_e3_owner_writer_admit() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E3_OWNER_WRITER_ADMIT)
        .expect("predicate fragment spike should parse owner_writer admit");
    let expected = load_fixture_option_admit_fragment("e3-option-admit-chain.json", "owner_writer")
        .expect("fixture admit fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_predicate_fragment_spike_matches_e3_delegated_writer_admit() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E3_DELEGATED_WRITER_ADMIT)
        .expect("predicate fragment spike should parse delegated_writer admit");
    let expected =
        load_fixture_option_admit_fragment("e3-option-admit-chain.json", "delegated_writer")
            .expect("fixture admit fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_predicate_fragment_spike_matches_e10_request_local_require() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E10_REQUIRE_FRAGMENT)
        .expect("predicate fragment spike should parse e10 require fragment");
    let expected = load_fixture_request_clause_fragment(
        "e10-perform-on-ensure-failure.json",
        0,
        "require",
        0,
    )
    .expect("fixture request clause fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_predicate_fragment_spike_matches_e10_request_local_ensure() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E10_ENSURE_FRAGMENT)
        .expect("predicate fragment spike should parse e10 ensure fragment");
    let expected =
        load_fixture_request_clause_fragment("e10-perform-on-ensure-failure.json", 0, "ensure", 0)
            .expect("fixture request clause fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_predicate_fragment_spike_matches_e11_request_local_ensure() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E11_ENSURE_FRAGMENT)
        .expect("predicate fragment spike should parse e11 ensure fragment");
    let expected = load_fixture_request_clause_fragment(
        "e11-perform-via-ensure-then-success.json",
        0,
        "ensure",
        0,
    )
    .expect("fixture request clause fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_predicate_fragment_spike_matches_grouped_and_fixture_fragment() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E2_GROUPED_AND_FRAGMENT)
        .expect("predicate fragment spike should parse grouped and fragment");
    let expected =
        load_fixture_request_clause_fragment("e2-try-fallback.json", 1, "require", 0)
            .expect("fixture and fragment should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_predicate_fragment_spike_keeps_explicit_and_terms() {
    let actual = parse_stage3_minimal_predicate_fragment_text(E2_GROUPED_AND_FRAGMENT)
        .expect("predicate fragment spike should parse grouped and fragment");

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
