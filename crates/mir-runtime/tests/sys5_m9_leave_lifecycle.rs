use mir_runtime::sys5_local_slice::{
    Sys5LocalAdmissionRequest, Sys5LocalRuntimeProfile, Sys5RelationBootstrapPolicy,
    Sys5SourceInput, Sys5VerticalAction, build_project,
};
use mir_runtime::sys5_local_workflow::{
    Sys5LocalWorkflowInput, Sys5LocalWorkflowReport, run_local_workflow_from_project,
};
use serde_json::Value;

const TOY_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const TOY_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

#[test]
fn participant_a_leave_retires_exact_m9_membership_before_relation_degrades() {
    let value = canonical_workflow_json();
    let mut failures = Vec::new();

    let leave = object_matching(&value, "participant_leave_results", |row| {
        row["action"].as_str() == Some("participant_a_leave")
            && row["source_derived"].as_bool() == Some(true)
            && row["external_lifecycle_request"].as_bool() == Some(true)
            && row["principal"].as_str() == Some("self")
            && row["participant_locus"].as_str() == Some("ParticipantA")
            && row["retired_membership_locus"].as_str() == Some("ParticipantA")
            && row["m9_transition_kind"].as_str() == Some("participant-membership-retired")
            && row["checked_membership_identity_exact"].as_bool() == Some(true)
            && row["retired_lineage_capability"].as_bool() == Some(true)
            && row["retired_lineage_witness"].as_bool() == Some(true)
            && row["relation"].as_str() == Some("bird_follow")
            && row["relation_owner_locus"].as_str() == Some("ParticipantB")
            && row["relation_owner_authority_preserved"].as_bool() == Some(true)
            && row["m8_state_mutated_before_m9_retirement"].as_bool() == Some(false)
            && row["direct_consumer_mutation"].as_bool() == Some(false)
    });
    let Some(leave) = leave else {
        failures.push("participant_leave_results lacks the source-derived ParticipantA M9 membership retirement row".to_string());
        panic_with_failures(failures);
    };

    assert_distinct_identity_and_occurrences(
        leave,
        &[
            "request_enqueue_occurrence_ref",
            "dispatch_occurrence_ref",
            "receive_occurrence_ref",
            "serve_occurrence_ref",
            "receipt_occurrence_ref",
            "m9_retire_occurrence_ref",
        ],
        &mut failures,
    );
    assert_ref_fields(
        leave,
        &[
            "checked_membership_identity_ref",
            "membership_epoch_before_ref",
            "membership_epoch_after_ref",
            "incarnation_before_ref",
            "incarnation_after_ref",
            "prior_generation_ref",
            "successor_generation_ref",
            "successor_tombstone_ref",
            "capability_lineage_ref",
            "witness_lineage_ref",
            "request_frontier_ref",
            "result_frontier_ref",
        ],
        &mut failures,
    );
    assert_not_raw_membership_material(leave, &mut failures);

    let degradation = object_matching(&value, "relation_degradation_results", |row| {
        row["trigger_action"].as_str() == Some("participant_a_leave")
            && row["source_derived"].as_bool() == Some(true)
            && row["external_lifecycle_request"].as_bool() == Some(true)
            && row["relation"].as_str() == Some("bird_follow")
            && row["owner_locus"].as_str() == Some("ParticipantB")
            && row["prior_selected_anchor"].as_str() == Some("participant_a_shoulder")
            && row["selected_anchor_after"].as_str() == Some("participant_b_shoulder")
            && row["selected_floor_after"].as_str() == Some("fallback-anchor")
            && row["m9_retirement_precedes_relation_publication"].as_bool() == Some(true)
            && row["participant_b_owner_authority_preserved"].as_bool() == Some(true)
            && row["direct_consumer_mutation"].as_bool() == Some(false)
    });
    let Some(degradation) = degradation else {
        failures.push("relation_degradation_results lacks the causal ParticipantA-leave -> ParticipantB-owned fallback publication row".to_string());
        panic_with_failures(failures);
    };
    assert_ref_fields(
        degradation,
        &[
            "m9_retire_occurrence_ref",
            "relation_publish_occurrence_ref",
            "prior_relation_lineage_ref",
            "successor_relation_lineage_ref",
            "semantic_digest_before_ref",
            "semantic_digest_after_ref",
            "owner_authority_ref",
        ],
        &mut failures,
    );
    assert_not_raw_membership_material(degradation, &mut failures);

    let joined_leave = array(&value, "joined_rows")
        .iter()
        .filter_map(|row| row["detail_ref"].as_str())
        .any(|detail| {
            detail.starts_with("typed-participant-leave:")
                && detail.contains("participant_locus=ParticipantA")
                && detail.contains("retired_membership_locus=ParticipantA")
                && detail.contains("external_lifecycle_request=true")
                && detail.contains("m9_transition_kind=participant-membership-retired")
                && detail.contains("relation_owner_locus=ParticipantB")
                && detail.contains("selected_floor_after=fallback-anchor")
                && detail.contains("request_identity=")
                && detail.contains("request_enqueue_occurrence_ref=")
                && detail.contains("m9_retire_occurrence_ref=")
                && detail.contains("relation_publish_occurrence_ref=")
        });
    if !joined_leave {
        failures.push(
            "joined_rows lacks a typed participant-leave segment joining source, M9 retirement, SYS-4 dispatch, and relation fallback"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "ParticipantA leave lifecycle contract failed:\n{}",
        failures.join("\n")
    );
}

#[test]
fn duplicate_participant_leave_attempt_fails_closed_without_partial_membership_or_m8_mutation() {
    let value = canonical_workflow_json();
    let mut failures = Vec::new();

    let row = object_matching(&value, "participant_leave_failures", |row| {
        row["attempt"].as_str() == Some("duplicate_leave")
            && row["diagnostic"].as_str() == Some("DuplicateParticipantLeave")
            && row["source_derived"].as_bool() == Some(true)
            && row["external_lifecycle_request"].as_bool() == Some(true)
            && row["failed_closed"].as_bool() == Some(true)
            && row["partial_membership_retired"].as_bool() == Some(false)
            && row["capability_or_witness_partially_retired"].as_bool() == Some(false)
            && row["m9_successor_installed"].as_bool() == Some(false)
            && row["m8_state_mutated"].as_bool() == Some(false)
            && row["m8_relation_mutated"].as_bool() == Some(false)
            && row["m8_designated_result_mutated"].as_bool() == Some(false)
            && row["m8_state_digest_before_ref"] == row["m8_state_digest_after_ref"]
            && row["m8_relation_digest_before_ref"] == row["m8_relation_digest_after_ref"]
            && row["preserved_successful_m8_result_ref"].as_bool() == Some(true)
    });
    let Some(row) = row else {
        failures.push(
            "participant_leave_failures lacks fail-closed duplicate_leave/DuplicateParticipantLeave evidence"
                .to_string(),
        );
        panic_with_failures(failures);
    };

    assert_distinct_identity_and_occurrences(
        row,
        &[
            "request_enqueue_occurrence_ref",
            "reject_occurrence_ref",
            "receipt_occurrence_ref",
        ],
        &mut failures,
    );
    assert_ref_fields(
        row,
        &[
            "checked_membership_identity_ref",
            "active_generation_ref",
            "m8_state_digest_before_ref",
            "m8_state_digest_after_ref",
            "m8_relation_digest_before_ref",
            "m8_relation_digest_after_ref",
            "last_successful_m8_result_ref",
        ],
        &mut failures,
    );
    assert_not_raw_membership_material(row, &mut failures);

    assert!(
        failures.is_empty(),
        "duplicate ParticipantA leave attempt was not fail-closed:\n{}",
        failures.join("\n")
    );
}

#[test]
fn fresh_reacquire_after_leave_creates_fresh_m9_lineage_before_primary_publication() {
    let value = canonical_workflow_json();
    let mut failures = Vec::new();

    let reacquire = object_matching(&value, "fresh_reacquire_results", |row| {
        row["action"].as_str() == Some("participant_a_fresh_reacquire")
            && row["source_derived"].as_bool() == Some(true)
            && row["external_lifecycle_request"].as_bool() == Some(true)
            && row["relation"].as_str() == Some("bird_follow")
            && row["participant_locus"].as_str() == Some("ParticipantA")
            && row["fresh_membership_locus"].as_str() == Some("ParticipantA")
            && row["m9_transition_kind"].as_str() == Some("participant-membership-fresh")
            && row["fresh_membership_epoch_distinct"].as_bool() == Some(true)
            && row["fresh_incarnation_distinct"].as_bool() == Some(true)
            && row["fresh_lineage_capability"].as_bool() == Some(true)
            && row["fresh_lineage_witness"].as_bool() == Some(true)
            && row["relation_owner_locus"].as_str() == Some("ParticipantB")
            && row["relation_owner_authority_preserved"].as_bool() == Some(true)
            && row["prior_selected_anchor"].as_str() == Some("participant_b_shoulder")
            && row["selected_anchor_after"].as_str() == Some("participant_a_shoulder")
            && row["selected_floor_after"].as_str() == Some("live-primary")
            && row["m9_fresh_membership_precedes_relation_publication"].as_bool() == Some(true)
            && row["caller_supplied_epoch_or_incarnation"].as_bool() == Some(false)
            && row["caller_supplied_membership_ref"].as_bool() == Some(false)
            && row["caller_supplied_authority"].as_bool() == Some(false)
            && row["fixture_schedule_authority_injection"].as_bool() == Some(false)
            && row["direct_consumer_mutation"].as_bool() == Some(false)
    });
    let Some(reacquire) = reacquire else {
        failures.push(
            "fresh_reacquire_results lacks source-bound external ParticipantA fresh-membership reacquire evidence"
                .to_string(),
        );
        panic_with_failures(failures);
    };
    let leave = object_matching(&value, "participant_leave_results", |row| {
        row["action"].as_str() == Some("participant_a_leave")
            && row["participant_locus"].as_str() == Some("ParticipantA")
            && row["relation"].as_str() == Some("bird_follow")
            && row["external_lifecycle_request"].as_bool() == Some(true)
    });
    let Some(leave) = leave else {
        failures.push(
            "participant_leave_results lacks matching ParticipantA leave row for fresh reacquire lineage join"
                .to_string(),
        );
        panic_with_failures(failures);
    };

    assert_distinct_identity_and_occurrences(
        reacquire,
        &[
            "request_enqueue_occurrence_ref",
            "dispatch_occurrence_ref",
            "receive_occurrence_ref",
            "serve_occurrence_ref",
            "receipt_occurrence_ref",
            "m9_fresh_membership_occurrence_ref",
            "relation_publish_occurrence_ref",
        ],
        &mut failures,
    );
    assert_ref_fields(
        reacquire,
        &[
            "retired_membership_epoch_ref",
            "fresh_membership_epoch_ref",
            "retired_incarnation_ref",
            "fresh_incarnation_ref",
            "prior_generation_ref",
            "successor_generation_ref",
            "fresh_capability_lineage_ref",
            "fresh_witness_lineage_ref",
            "prior_relation_lineage_ref",
            "successor_relation_lineage_ref",
            "semantic_digest_before_ref",
            "semantic_digest_after_ref",
            "owner_authority_ref",
            "retired_membership_ref",
            "fresh_membership_ref",
        ],
        &mut failures,
    );
    assert_ref_fields(
        leave,
        &["successor_tombstone_ref", "membership_epoch_after_ref"],
        &mut failures,
    );
    assert_cross_row_equal_ref(
        reacquire,
        "retired_membership_ref",
        leave,
        "successor_tombstone_ref",
        &mut failures,
    );
    assert_cross_row_equal_ref(
        reacquire,
        "retired_membership_epoch_ref",
        leave,
        "membership_epoch_after_ref",
        &mut failures,
    );
    assert_distinct_ref_pair(
        reacquire,
        "retired_membership_epoch_ref",
        "fresh_membership_epoch_ref",
        &mut failures,
    );
    assert_distinct_ref_pair(
        reacquire,
        "retired_incarnation_ref",
        "fresh_incarnation_ref",
        &mut failures,
    );
    assert_not_raw_membership_material(reacquire, &mut failures);

    let joined_reacquire = array(&value, "joined_rows")
        .iter()
        .filter_map(|row| row["detail_ref"].as_str())
        .any(|detail| {
            detail.starts_with("typed-participant-fresh-reacquire:")
                && detail.contains("participant_locus=ParticipantA")
                && detail.contains("external_lifecycle_request=true")
                && detail.contains("m9_transition_kind=participant-membership-fresh")
                && detail.contains("relation_owner_locus=ParticipantB")
                && detail.contains("selected_floor_after=live-primary")
                && detail.contains("caller_supplied_authority=false")
                && detail.contains("request_identity=")
                && detail.contains("m9_fresh_membership_occurrence_ref=")
                && detail.contains("relation_publish_occurrence_ref=")
        });
    if !joined_reacquire {
        failures.push(
            "joined_rows lacks typed ParticipantA fresh reacquire segment joining M9 fresh membership to relation primary publication"
                .to_string(),
        );
    }

    let final_relation_primary = array(&value["runtime_summary"], "relations")
        .iter()
        .any(|row| {
            row["relation"].as_str() == Some("bird_follow")
                && row["selected_anchor"].as_str() == Some("participant_a_shoulder")
                && row["selected_floor"].as_str() == Some("live-primary")
        });
    if !final_relation_primary {
        failures.push(
            "runtime_summary.relations does not show final bird_follow primary after fresh reacquire"
                .to_string(),
        );
    }

    assert!(
        failures.is_empty(),
        "fresh ParticipantA reacquire contract failed:\n{}",
        failures.join("\n")
    );
}

#[test]
fn save_restore_after_participant_leave_allows_fresh_reacquire_with_retired_lineage_join() {
    let project = build_project(Sys5SourceInput::inline(TOY_SOURCE_PATH, TOY_SOURCE))
        .expect("SYS-5 toy source must check and project");
    let runtime_admission = project
        .prepare_finite_admission(source_declared_request())
        .expect("SYS-5 runtime admission must be source-derived and complete");
    let restore_admission = project
        .prepare_finite_admission(source_declared_request())
        .expect("SYS-5 restore admission must target the same checked source inventory");
    let mut runtime = runtime_admission
        .start_vertical_slice_runtime()
        .expect("source-derived vertical slice starts before leave cut");

    runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("initial relation primary publishes before leave cut");
    let leave_receipt = runtime
        .dispatch(Sys5VerticalAction::participant_a_leave_relation_primary(
            "bird_follow",
        ))
        .expect("ParticipantA leave publishes fallback before save cut");
    let leave = serde_json::to_value(
        leave_receipt
            .participant_leave_evidence()
            .expect("leave receipt exposes observer-safe M9 retirement evidence"),
    )
    .expect("leave evidence serializes for explicit lineage join assertions");
    let cut = runtime
        .save_local_cut("sys5-after-participant-a-leave-before-fresh-reacquire")
        .expect("SYS-5 cut must capture completed leave lineage before fresh reacquire");
    let mut restored = restore_admission
        .restore_vertical_slice_runtime(&cut)
        .expect("matching admission restores the saved post-leave SYS-5 cut");
    let fresh_receipt = restored
        .dispatch(Sys5VerticalAction::fresh_reacquire_relation_primary(
            "bird_follow",
        ))
        .expect("restored post-leave cut must retain enough retired lineage for fresh reacquire");
    let fresh = serde_json::to_value(
        fresh_receipt
            .fresh_reacquire_evidence()
            .expect("fresh receipt exposes observer-safe M9 fresh lineage evidence"),
    )
    .expect("fresh evidence serializes for explicit lineage join assertions");

    let mut failures = Vec::new();
    assert_cross_row_equal_ref(
        &fresh,
        "retired_membership_ref",
        &leave,
        "successor_tombstone_ref",
        &mut failures,
    );
    assert_cross_row_equal_ref(
        &fresh,
        "retired_membership_epoch_ref",
        &leave,
        "membership_epoch_after_ref",
        &mut failures,
    );
    if fresh["external_lifecycle_request"].as_bool() != Some(true) {
        failures.push(
            "restored fresh reacquire must remain an explicit external lifecycle request"
                .to_string(),
        );
    }
    if fresh["caller_supplied_epoch_or_incarnation"].as_bool() != Some(false)
        || fresh["caller_supplied_membership_ref"].as_bool() != Some(false)
        || fresh["caller_supplied_authority"].as_bool() != Some(false)
        || fresh["fixture_schedule_authority_injection"].as_bool() != Some(false)
    {
        failures.push(
            "restored fresh reacquire must not accept caller-supplied epoch/ref/authority injection"
                .to_string(),
        );
    }
    let Some(shadow) = fresh_receipt.observer_relation_shadow("ViewerC", "bird_follow") else {
        failures.push("fresh receipt after restore lacks ViewerC relation shadow".to_string());
        panic_with_failures(failures);
    };
    if shadow.selected_anchor() != "participant_a_shoulder"
        || shadow.selected_floor() != "live-primary"
    {
        failures.push(format!(
            "fresh reacquire after restore must republish primary, got {} / {}",
            shadow.selected_anchor(),
            shadow.selected_floor()
        ));
    }

    assert!(
        failures.is_empty(),
        "save/restore post-leave fresh reacquire contract failed:\n{}",
        failures.join("\n")
    );
}

fn canonical_workflow_json() -> Value {
    let report = canonical_workflow_report();
    serde_json::from_str(&report.render_compact()).expect("workflow report must be JSON")
}

fn canonical_workflow_report() -> Sys5LocalWorkflowReport {
    let project = build_project(Sys5SourceInput::inline(TOY_SOURCE_PATH, TOY_SOURCE))
        .expect("SYS-5 toy source must check and project");
    let admission = project
        .prepare_finite_admission(source_declared_request())
        .expect("SYS-5 toy admission must be source-derived and complete");
    run_local_workflow_from_project(Sys5LocalWorkflowInput::from_project_and_admission(
        project, admission,
    ))
    .expect("SYS-5 canonical workflow must execute from checked/projected input")
}

fn source_declared_request() -> Sys5LocalAdmissionRequest {
    Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-workflow-world",
        "incarnation:self:WorldAuthority:epoch:sys5-workflow-world",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "self",
        "ParticipantA",
        "epoch:sys5-workflow-a",
        "incarnation:self:ParticipantA:epoch:sys5-workflow-a",
    )
    .with_source_declared_membership(
        "self",
        "ParticipantB",
        "epoch:sys5-workflow-b",
        "incarnation:self:ParticipantB:epoch:sys5-workflow-b",
    )
    .with_source_declared_membership(
        "self",
        "ViewerC",
        "epoch:sys5-workflow-c",
        "incarnation:self:ViewerC:epoch:sys5-workflow-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn object_matching<'a>(
    value: &'a Value,
    field: &str,
    predicate: impl Fn(&Value) -> bool,
) -> Option<&'a Value> {
    array(value, field).into_iter().find(|&row| predicate(row))
}

fn array<'a>(value: &'a Value, field: &str) -> Vec<&'a Value> {
    value
        .get(field)
        .and_then(Value::as_array)
        .map(|rows| rows.iter().collect())
        .unwrap_or_default()
}

fn assert_distinct_identity_and_occurrences(
    row: &Value,
    occurrence_fields: &[&str],
    failures: &mut Vec<String>,
) {
    let Some(request_identity) = row["request_identity"]
        .as_str()
        .filter(|value| !value.is_empty())
    else {
        failures.push(format!(
            "row lacks non-empty request_identity: {}",
            compact(row)
        ));
        return;
    };
    for field in occurrence_fields {
        let Some(occurrence_ref) = row[*field].as_str().filter(|value| !value.is_empty()) else {
            failures.push(format!("{field} is missing from {}", compact(row)));
            continue;
        };
        if occurrence_ref == request_identity {
            failures.push(format!(
                "{field} must be an occurrence ref, not the request identity {request_identity}"
            ));
        }
    }
}

fn assert_ref_fields(row: &Value, fields: &[&str], failures: &mut Vec<String>) {
    for field in fields {
        let Some(value) = row[*field].as_str().filter(|value| !value.is_empty()) else {
            failures.push(format!("{field} is missing or empty in {}", compact(row)));
            continue;
        };
        if value.contains("incarnation:")
            || value.contains("epoch:sys5-workflow")
            || value.contains("capability:")
            || value.contains("witness:")
            || value.contains("membership:self")
        {
            failures.push(format!("{field} exposes raw authority material: {value}"));
        }
    }
}

fn assert_distinct_ref_pair(row: &Value, left: &str, right: &str, failures: &mut Vec<String>) {
    let left_value = row[left].as_str().unwrap_or_default();
    let right_value = row[right].as_str().unwrap_or_default();
    if !left_value.is_empty() && left_value == right_value {
        failures.push(format!(
            "{left} and {right} must be distinct opaque refs after fresh reacquire"
        ));
    }
}

fn assert_cross_row_equal_ref(
    left_row: &Value,
    left_field: &str,
    right_row: &Value,
    right_field: &str,
    failures: &mut Vec<String>,
) {
    let left_value = left_row[left_field].as_str().unwrap_or_default();
    let right_value = right_row[right_field].as_str().unwrap_or_default();
    if left_value.is_empty() || right_value.is_empty() {
        failures.push(format!(
            "{left_field} and {right_field} must both be non-empty explicit shared refs"
        ));
        return;
    }
    if left_value != right_value {
        failures.push(format!(
            "{left_field} must equal {right_field} to join leave retirement to fresh reacquire, got {left_value} != {right_value}"
        ));
    }
}

fn assert_not_raw_membership_material(row: &Value, failures: &mut Vec<String>) {
    let rendered = compact(row);
    for denied in [
        "incarnation:self:ParticipantA",
        "epoch:sys5-workflow-a",
        "capability:",
        "witness:",
        "membership:self",
    ] {
        if rendered.contains(denied) {
            failures.push(format!(
                "observer-safe leave row exposes raw material {denied}"
            ));
        }
    }
}

fn compact(value: &Value) -> String {
    serde_json::to_string(value).expect("test JSON row serializes")
}

fn panic_with_failures(failures: Vec<String>) -> ! {
    panic!("{}", failures.join("\n"))
}
