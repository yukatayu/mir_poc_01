#[path = "support/current_l2_stage1_parser_spike_support.rs"]
mod current_l2_stage1_parser_spike_support;

use current_l2_stage1_parser_spike_support::{
    Stage1FixtureSubset, Stage1ReconnectClusters, load_expected_fixture_subset,
    lower_stage1_chain_decl_to_fixture_chain, lower_stage1_option_decl_to_fixture_option,
    parse_stage1_program_text, summarize_stage1_reconnect_clusters,
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

const MISSING_LINEAGE_INPUT: &str = r#"
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live
chain profile_ref = primary
fallback mirror
"#;

const OPTION_ADMIT_SPILLOVER_INPUT: &str = r#"
option owner_writer on profile_doc capability write lease live admit owner_is(session_user)
"#;

const E13_INPUT: &str = r#"
option primary on profile_doc capability read lease live
option mirror on profile_doc capability write lease live
chain profile_ref = primary
fallback mirror @ lineage(primary -> mirror)
"#;

const E16_INPUT: &str = r#"
option primary on profile_doc capability read lease live
chain profile_ref = ghost
"#;

const E18_INPUT: &str = r#"
option primary on profile_doc capability read lease live
chain profile_ref = primary
fallback ghost @ lineage(primary -> ghost)
"#;

const E20_INPUT: &str = r#"
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live
option escalated on profile_doc capability write lease live
chain profile_ref = primary
fallback mirror @ lineage(primary -> mirror)
fallback escalated @ lineage(mirror -> escalated)
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
fn stage1_parser_spike_matches_e13_fixture_subset() {
    let actual = lower_for_compare(E13_INPUT);
    let expected = load_expected_fixture_subset("e13-malformed-capability-strengthening.json")
        .expect("expected e13 fixture subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage1_parser_spike_matches_e16_fixture_subset() {
    let actual = lower_for_compare(E16_INPUT);
    let expected = load_expected_fixture_subset("e16-malformed-missing-chain-head-option.json")
        .expect("expected e16 fixture subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage1_parser_spike_matches_e18_fixture_subset() {
    let actual = lower_for_compare(E18_INPUT);
    let expected = load_expected_fixture_subset("e18-malformed-missing-successor-option.json")
        .expect("expected e18 fixture subset should load");

    assert_eq!(actual, expected);
}

#[test]
fn stage1_parser_spike_matches_e20_fixture_subset() {
    let actual = lower_for_compare(E20_INPUT);
    let expected = load_expected_fixture_subset("e20-malformed-late-capability-strengthening.json")
        .expect("expected e20 fixture subset should load");

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

#[test]
fn stage1_parser_spike_rejects_missing_edge_local_lineage_metadata() {
    let error = parse_stage1_program_text(MISSING_LINEAGE_INPUT)
        .expect_err("stage 1 spike should reject fallback rows without lineage metadata");

    assert!(
        error.contains("missing edge-local lineage metadata"),
        "expected missing-lineage wording, got: {error}"
    );
}

#[test]
fn stage1_parser_spike_rejects_option_local_admit_spillover() {
    let error = parse_stage1_program_text(OPTION_ADMIT_SPILLOVER_INPUT)
        .expect_err("stage 1 spike should reject option-local admit spillover");

    assert!(
        error.contains("option-local admit is outside stage 1 accepted cluster"),
        "expected option-local admit wording, got: {error}"
    );
}

#[test]
fn stage1_parser_spike_marks_same_lineage_reconnect_cluster_for_e4() {
    let subset = lower_for_compare(E4_INPUT);

    assert_eq!(
        summarize_stage1_reconnect_clusters(&subset),
        Stage1ReconnectClusters {
            same_lineage_floor: true,
            missing_option_structure_floor: false,
            capability_strengthening_floor: false,
        }
    );
}

#[test]
fn stage1_parser_spike_marks_capability_reconnect_cluster_for_e13() {
    let subset = lower_for_compare(E13_INPUT);

    assert_eq!(
        summarize_stage1_reconnect_clusters(&subset),
        Stage1ReconnectClusters {
            same_lineage_floor: true,
            missing_option_structure_floor: false,
            capability_strengthening_floor: true,
        }
    );
}

#[test]
fn stage1_parser_spike_marks_missing_option_reconnect_cluster_for_e16() {
    let subset = lower_for_compare(E16_INPUT);

    assert_eq!(
        summarize_stage1_reconnect_clusters(&subset),
        Stage1ReconnectClusters {
            same_lineage_floor: false,
            missing_option_structure_floor: true,
            capability_strengthening_floor: false,
        }
    );
}

#[test]
fn stage1_parser_spike_marks_missing_option_reconnect_cluster_for_e18() {
    let subset = lower_for_compare(E18_INPUT);

    assert_eq!(
        summarize_stage1_reconnect_clusters(&subset),
        Stage1ReconnectClusters {
            same_lineage_floor: true,
            missing_option_structure_floor: true,
            capability_strengthening_floor: false,
        }
    );
}

#[test]
fn stage1_parser_spike_marks_capability_reconnect_cluster_for_e20() {
    let subset = lower_for_compare(E20_INPUT);

    assert_eq!(
        summarize_stage1_reconnect_clusters(&subset),
        Stage1ReconnectClusters {
            same_lineage_floor: true,
            missing_option_structure_floor: false,
            capability_strengthening_floor: true,
        }
    );
}
