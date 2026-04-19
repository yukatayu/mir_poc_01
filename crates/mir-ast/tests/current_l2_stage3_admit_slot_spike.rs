#[path = "support/current_l2_stage3_admit_slot_spike_support.rs"]
mod current_l2_stage3_admit_slot_spike_support;

use mir_ast::current_l2::parse_stage3_admit_slot_program_text;

use current_l2_stage3_admit_slot_spike_support::{
    Stage3FixtureStructuralSubset, load_expected_fixture_structural_subset,
    lower_stage3_chain_decl_to_fixture_chain,
    lower_stage3_option_decl_to_fixture_structural_option,
};

const E3_ADMIT_SLOT_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live admit owner_is(session_user)
option delegated_writer on profile_doc capability write lease live admit delegate_granted(session_user)
chain profile_ref = owner_writer
fallback delegated_writer @ lineage(owner_writer -> delegated_writer)
"#;

const MISSING_ADMIT_PAYLOAD_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live admit
"#;

const PERFORM_VIA_SPILLOVER_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live admit owner_is(session_user)
chain profile_ref = owner_writer
perform write_profile via profile_ref
"#;

const REQUEST_LOCAL_REQUIRE_SPILLOVER_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live admit owner_is(session_user)
chain profile_ref = owner_writer
require write
"#;

const REQUEST_LOCAL_ENSURE_SPILLOVER_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live admit owner_is(session_user)
chain profile_ref = owner_writer
ensure owner_is(session_user)
"#;

fn lower_for_compare(source: &str) -> Stage3FixtureStructuralSubset {
    let parsed = parse_stage3_admit_slot_program_text(source)
        .expect("stage 3 admit-slot spike should parse test input");

    Stage3FixtureStructuralSubset {
        options: parsed
            .options
            .iter()
            .map(lower_stage3_option_decl_to_fixture_structural_option)
            .collect(),
        chains: parsed
            .chains
            .iter()
            .map(lower_stage3_chain_decl_to_fixture_chain)
            .collect(),
    }
}

#[test]
fn stage3_admit_slot_parser_spike_matches_e3_fixture_structural_subset() {
    let actual = lower_for_compare(E3_ADMIT_SLOT_INPUT);
    let expected = load_expected_fixture_structural_subset("e3-option-admit-chain.json")
        .expect("expected e3 structural fixture subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage3_admit_slot_parser_spike_keeps_decl_admit_slot_surface_text() {
    let parsed = parse_stage3_admit_slot_program_text(E3_ADMIT_SLOT_INPUT)
        .expect("stage 3 admit-slot spike should parse test input");
    let owner_writer = parsed
        .options
        .iter()
        .find(|option| option.name == "owner_writer")
        .expect("owner_writer option should exist");
    let delegated_writer = parsed
        .options
        .iter()
        .find(|option| option.name == "delegated_writer")
        .expect("delegated_writer option should exist");

    assert_eq!(
        owner_writer
            .decl_admit_slot
            .as_ref()
            .expect("owner_writer admit slot should exist")
            .surface_text,
        "owner_is(session_user)"
    );
    assert_eq!(
        delegated_writer
            .decl_admit_slot
            .as_ref()
            .expect("delegated_writer admit slot should exist")
            .surface_text,
        "delegate_granted(session_user)"
    );
}

#[test]
fn stage3_admit_slot_parser_spike_rejects_missing_admit_slot_payload() {
    let error = parse_stage3_admit_slot_program_text(MISSING_ADMIT_PAYLOAD_INPUT)
        .expect_err("stage 3 admit-slot spike should reject missing admit slot payload");

    assert!(
        error.contains("missing declaration-side admit slot payload"),
        "expected missing-admit wording, got: {error}"
    );
}

#[test]
fn stage3_admit_slot_parser_spike_rejects_request_head_spillover() {
    let error = parse_stage3_admit_slot_program_text(PERFORM_VIA_SPILLOVER_INPUT)
        .expect_err("stage 3 admit-slot spike should reject request-head spillover");

    assert!(
        error.contains("request head is outside stage 3 admit-slot first tranche"),
        "expected request-head wording, got: {error}"
    );
}

#[test]
fn stage3_admit_slot_parser_spike_rejects_request_local_require_spillover() {
    let error = parse_stage3_admit_slot_program_text(REQUEST_LOCAL_REQUIRE_SPILLOVER_INPUT)
        .expect_err("stage 3 admit-slot spike should reject request-local require spillover");

    assert!(
        error.contains("request-local require clause is outside stage 3 admit-slot first tranche"),
        "expected request-local require wording, got: {error}"
    );
}

#[test]
fn stage3_admit_slot_parser_spike_rejects_request_local_ensure_spillover() {
    let error = parse_stage3_admit_slot_program_text(REQUEST_LOCAL_ENSURE_SPILLOVER_INPUT)
        .expect_err("stage 3 admit-slot spike should reject request-local ensure spillover");

    assert!(
        error.contains("request-local ensure clause is outside stage 3 admit-slot first tranche"),
        "expected request-local ensure wording, got: {error}"
    );
}
