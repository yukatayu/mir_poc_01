#[path = "support/current_l2_stage1_parser_spike_support.rs"]
mod current_l2_stage1_parser_spike_support;

use current_l2_stage1_parser_spike_support::{
    Stage1FixtureSubset, load_expected_fixture_subset, lower_stage1_chain_decl_to_fixture_chain,
    lower_stage1_option_decl_to_fixture_option, parse_stage1_program_text,
};

const E4_INPUT: &str = r#"
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live
option archive on profile_doc capability read lease live
chain profile_ref = primary
fallback mirror @ lineage(primary -> archive)
"#;

const E7_INPUT: &str = r#"
option writer on profile_doc capability write lease expired
option delegated_writer on profile_doc capability write lease live
option readonly on profile_doc capability read lease live
chain profile_ref = writer
fallback delegated_writer @ lineage(writer -> delegated_writer)
fallback readonly @ lineage(delegated_writer -> readonly)
"#;

fn lower_for_compare(source: &str) -> Stage1FixtureSubset {
    let parsed = parse_stage1_program_text(source).expect("stage 1 spike should parse test input");

    Stage1FixtureSubset {
        options: parsed
            .options
            .iter()
            .map(lower_stage1_option_decl_to_fixture_option)
            .collect(),
        chains: parsed
            .chains
            .iter()
            .map(lower_stage1_chain_decl_to_fixture_chain)
            .collect(),
    }
}

#[test]
fn stage1_parser_spike_matches_e4_fixture_subset() {
    let actual = lower_for_compare(E4_INPUT);
    let expected = load_expected_fixture_subset("e4-malformed-lineage.json")
        .expect("expected e4 fixture subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage1_parser_spike_matches_e7_fixture_subset() {
    let actual = lower_for_compare(E7_INPUT);
    let expected = load_expected_fixture_subset("e7-write-fallback-after-expiry.json")
        .expect("expected e7 fixture subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage1_parser_spike_keeps_decl_guard_slot_surface_text() {
    let parsed = parse_stage1_program_text(E7_INPUT).expect("stage 1 spike should parse test input");
    let writer = parsed
        .options
        .iter()
        .find(|option| option.name == "writer")
        .expect("writer option should exist");

    assert_eq!(writer.decl_guard_slot.surface_text, "expired");
}
