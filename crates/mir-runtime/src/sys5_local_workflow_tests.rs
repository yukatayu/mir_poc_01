use crate::{
    sys5_local_slice::{
        Sys5LocalAdmissionRequest, Sys5LocalRuntimeProfile, Sys5PatchDiagnosticKind,
        Sys5PatchVerdict, Sys5RelationBootstrapPolicy, Sys5SemanticSummary, Sys5SourceInput,
        Sys5VerticalDiagnosticKind, build_project,
    },
    sys5_local_workflow::{
        Sys5LocalWorkflowInput, Sys5LocalWorkflowPatchProject, Sys5LocalWorkflowReport,
        Sys5LocalWorkflowStep, run_local_workflow_from_project,
    },
};
use serde_json::Value;

const SYS5_WORKFLOW_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";

const SYS5_WORKFLOW_SOURCE: &str = r#"
module Mirrorea.Sys5.LocalToy

locus WorldAuthority
locus ParticipantA
locus ParticipantB
locus ViewerC
principal self
principal target
type Player
type Bird

state avatar[id: Player] at WorldAuthority {
  hp: Int
  atk: Int
  visible observer_safe fields (hp)
}

state participant_input[id: Player] at ParticipantA {
  focus: Int
  visible observer_safe fields (focus)
}

state bird_pose[id: Bird] at ParticipantB {
  x: Int
  y: Int
  visible observer_safe fields (x, y)
}

Role[self] at ParticipantA {
  when init_avatar_hp() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[self].hp = 21
    }
  }

  when init_avatar_atk() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[self].atk = 5
    }
  }

  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
    }
  }
}

Role[self] at WorldAuthority {
  when init_focus() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at ParticipantA {
      participant_input[self].focus = 10
    }
  }
}

relation bird_follow at ParticipantB {
  subject bird: Bird
  primary participant_a_shoulder at ParticipantA epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder at ParticipantB epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = participant_input[self].focus + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

#[test]
fn workflow_api_consumes_prebuilt_projects_and_returns_joined_observer_safe_report() {
    let report: Sys5LocalWorkflowReport = run_canonical_workflow();

    assert_eq!(report.runtime_profile(), "ST");
    assert_eq!(report.local_fabric_instance_count(), 1);
    assert_eq!(report.source_authority(), "ordinary_mir_source");
    assert!(!report.public_api_or_wire_contract());
    assert!(!report.final_public_api_frozen());
    assert!(!report.public_wire_frozen());
    assert!(!report.runtime_reparsed_source());
    assert!(!report.used_fixture_name_or_expected_json());
    assert!(!report.accepted_manual_route_or_interface());
    assert!(!report.accepted_runtime_core_or_authority_injection());

    assert_contains_steps(&report, &all_expected_steps());

    assert_eq!(
        report.patch_verdict("designated-plus-two"),
        Some(Sys5PatchVerdict::Accepted),
        "workflow must accept the source-first designated-only patch by checked content"
    );
    assert_eq!(report.patch_diagnostic("designated-plus-two"), None);
    assert_eq!(
        report.patch_verdict("owner-rmw-change"),
        Some(Sys5PatchVerdict::Rejected),
        "workflow must reject owner-RMW semantic changes by checked content"
    );
    assert_eq!(
        report.patch_diagnostic("owner-rmw-change"),
        Some(Sys5PatchDiagnosticKind::OwnerRmwExpressionChanged)
    );
    assert!(report.has_typed_failure(Sys5VerticalDiagnosticKind::MissingConsumerCapability));
    assert!(
        report.failure_rejected_before_state_mutation(
            Sys5VerticalDiagnosticKind::MissingConsumerCapability
        ),
        "revoked ViewerC consume must fail closed before cache or state mutation"
    );

    assert_joined_rows_include(
        &report,
        &[
            "source_span",
            "core_operation",
            "per_locus_artifact",
            "generated_communication_edge",
            "runtime_occurrence",
            "owner_state_mutation",
            "authority_failure",
            "relation_selected_fallback",
            "designated_result_version",
            "save_cut",
            "restore_cut",
            "patch_lifecycle",
            "verification_residual",
        ],
    );
    assert_observer_safe_no_raw_material(&report.render_compact());
}

#[test]
fn workflow_output_is_deterministic_for_same_checked_projects_and_patch_projects() {
    let first: Sys5LocalWorkflowReport = run_canonical_workflow();
    let second: Sys5LocalWorkflowReport = run_canonical_workflow();

    assert_eq!(
        first.render_compact(),
        second.render_compact(),
        "same prebuilt checked project and patch projects should produce deterministic observer-safe workflow output"
    );
    assert_eq!(first.observer_safe_digest(), second.observer_safe_digest());
    assert_contains_steps(&first, &all_expected_steps());
    assert_contains_steps(&second, &all_expected_steps());
}

#[test]
fn workflow_report_embeds_exact_projection_summary_and_actual_causal_segments() {
    let (report, summary, checked_program_identity_ref) = run_canonical_workflow_with_summary();
    let rendered = report.render_compact();
    let value: Value =
        serde_json::from_str(&rendered).expect("workflow report must render as JSON");
    let mut failures = Vec::new();

    expect_string_field(
        &value,
        "checked_program_identity_ref",
        &checked_program_identity_ref,
        &mut failures,
    );
    expect_exact_json_field(
        &value,
        "source_core_artifact_mappings",
        serde_json::to_value(&summary.source_core_artifact_mappings)
            .expect("source/Core/artifact mappings serialize"),
        &mut failures,
    );
    expect_exact_json_field(
        &value,
        "locus_programs",
        serde_json::to_value(&summary.artifacts).expect("locus artifact summaries serialize"),
        &mut failures,
    );
    expect_exact_json_field(
        &value,
        "generated_communication",
        serde_json::to_value(&summary.generated_communication)
            .expect("generated communication summaries serialize"),
        &mut failures,
    );
    expect_mapping_spans_and_attack_linkage(
        value.get("source_core_artifact_mappings"),
        &checked_program_identity_ref,
        &mut failures,
    );
    expect_runtime_summary_evidence(value.get("runtime_summary"), &mut failures);
    expect_typed_causal_segment(
        &value,
        "typed-segment:owner-request:",
        &[
            "logical_path=",
            "source_span=",
            "core_ref=",
            "source_fragment_ref=",
            "target_fragment_ref=",
            "edge_ref=",
            "request_identity=",
            "request_enqueue_occurrence_id=",
            "dispatch_occurrence_id=",
            "receive_occurrence_id=",
            "serve_occurrence_id=",
        ],
        &mut failures,
    );
    expect_typed_causal_segment(
        &value,
        "typed-segment:designated-input-request:",
        &[
            "logical_path=",
            "source_span=",
            "core_ref=",
            "source_fragment_ref=",
            "target_fragment_ref=",
            "edge_ref=",
            "request_identity=",
            "request_enqueue_occurrence_id=",
            "dispatch_occurrence_id=",
            "receive_occurrence_id=",
            "serve_occurrence_id=",
        ],
        &mut failures,
    );
    expect_typed_causal_segment(
        &value,
        "typed-segment:designated-result-delivery:",
        &[
            "logical_path=",
            "source_span=",
            "core_ref=",
            "source_fragment_ref=",
            "target_fragment_ref=",
            "edge_ref=",
            "request_identity=",
            "request_enqueue_occurrence_id=",
            "dispatch_occurrence_id=",
            "receive_occurrence_id=",
            "serve_occurrence_id=",
        ],
        &mut failures,
    );
    expect_typed_causal_segment(
        &value,
        "typed-segment:relation-projection-publication:",
        &[
            "logical_path=",
            "source_span=",
            "core_ref=",
            "source_fragment_ref=",
            "target_fragment_ref=",
            "edge_ref=",
            "request_identity=",
            "owner_publish_occurrence_id=",
            "request_enqueue_occurrence_id=",
            "dispatch_occurrence_id=",
            "receive_occurrence_id=",
            "consumer_observe_occurrence_id=",
            "serve_occurrence_id=",
        ],
        &mut failures,
    );
    assert_observer_safe_no_raw_material(&rendered);

    assert!(
        failures.is_empty(),
        "workflow report missing SYS-5 devtools evidence:\n{}",
        failures.join("\n")
    );
}

#[test]
fn workflow_report_retains_actual_presentation_gap_result_evidence() {
    let report = run_canonical_workflow();
    let rendered = report.render_compact();
    let value: Value =
        serde_json::from_str(&rendered).expect("workflow report must render as JSON");
    let mut failures = Vec::new();

    expect_actual_presentation_gap_evidence(&value, &mut failures);
    assert_observer_safe_no_raw_material(&rendered);

    assert!(
        failures.is_empty(),
        "workflow report missing actual presentation-gap evidence:\n{}",
        failures.join("\n")
    );
}

#[test]
fn workflow_joined_runtime_rows_have_observer_safe_execution_branches() {
    let report = run_canonical_workflow();
    let rendered = report.render_compact();
    let value: Value =
        serde_json::from_str(&rendered).expect("workflow report must render as JSON");
    let mut failures = Vec::new();

    expect_execution_branch_evidence(&value, &mut failures);
    assert_observer_safe_no_raw_material(&rendered);

    assert!(
        failures.is_empty(),
        "workflow report missing execution-branch evidence:\n{}",
        failures.join("\n")
    );
}

#[test]
fn workflow_rejects_unsafe_patch_project_logical_paths_without_reporting_raw_value() {
    for (case, logical_path) in [
        ("absolute-host-path", "/home/codex/secret-patch.mir"),
        ("traversal", "../secret-patch.mir"),
        ("control-character", "patches/\nsecret-patch.mir"),
        ("secret-like", "patches/token=sk-test-secret.mir"),
    ] {
        let result = run_workflow_with_patch_logical_path(logical_path);
        match result {
            Ok(report) => {
                let rendered = report.render_compact();
                let leak_labels = observer_unsafe_fragment_labels(&rendered);
                if !leak_labels.is_empty() {
                    panic!(
                        "unsafe patch logical path case `{case}` was accepted and report leaked {leak_labels:?}"
                    );
                }
                panic!(
                    "unsafe patch logical path case `{case}` was accepted into a workflow report"
                );
            }
            Err(error) => {
                let rendered = error.to_string();
                let leak_labels = observer_unsafe_fragment_labels(&rendered);
                assert!(
                    leak_labels.is_empty(),
                    "unsafe patch logical path case `{case}` rejection leaked {leak_labels:?}"
                );
                assert!(
                    !rendered.contains(logical_path),
                    "unsafe patch logical path case `{case}` leaked in rejection"
                );
            }
        }
    }
}

#[test]
fn workflow_patch_provenance_matches_actual_patch_lifecycle_occurrence_refs() {
    let report = run_canonical_workflow();
    let rendered = report.render_compact();
    let value: Value =
        serde_json::from_str(&rendered).expect("workflow report must render as JSON");
    let mut failures = Vec::new();

    expect_patch_occurrence_provenance(
        &value,
        "designated-plus-two",
        "accepted",
        "PatchAccepted",
        &mut failures,
    );
    expect_patch_occurrence_provenance(
        &value,
        "owner-rmw-change",
        "rejected",
        "PatchRejected",
        &mut failures,
    );
    assert_observer_safe_no_raw_material(&rendered);

    assert!(
        failures.is_empty(),
        "workflow report missing patch occurrence provenance:\n{}",
        failures.join("\n")
    );
}

#[test]
fn workflow_uses_source_derived_participant_leave_for_relation_invalidation() {
    let report = run_canonical_workflow();
    let rendered = report.render_compact();
    let value: Value =
        serde_json::from_str(&rendered).expect("workflow report must render as JSON");
    let mut failures = Vec::new();

    expect_source_derived_participant_leave(&value, &mut failures);
    assert_observer_safe_no_raw_material(&rendered);

    assert!(
        failures.is_empty(),
        "workflow report missing source-derived ParticipantA leave evidence:\n{}",
        failures.join("\n")
    );
}

fn run_canonical_workflow() -> Sys5LocalWorkflowReport {
    run_canonical_workflow_with_summary().0
}

fn run_workflow_with_patch_logical_path(
    logical_path: &str,
) -> Result<Sys5LocalWorkflowReport, crate::sys5_local_workflow::Sys5LocalWorkflowError> {
    let base_project = build_project(Sys5SourceInput::inline(
        SYS5_WORKFLOW_PATH,
        SYS5_WORKFLOW_SOURCE,
    ))
    .expect("base ordinary source must check and project before workflow execution");
    let base_admission = base_project
        .prepare_finite_admission(source_declared_request())
        .expect("base admission must be sealed before workflow execution");
    let plus_two_project = build_project(Sys5SourceInput::inline(
        SYS5_WORKFLOW_PATH,
        designated_plus_two_source(),
    ))
    .expect("patch source must check and project before workflow execution");
    let plus_two_admission = plus_two_project
        .prepare_finite_admission(source_declared_request())
        .expect("patch admission must be sealed before workflow execution");
    let input = Sys5LocalWorkflowInput::from_project_and_admission(base_project, base_admission)
        .with_patch_project(
            Sys5LocalWorkflowPatchProject::from_project_and_admission(
                "unsafe-logical-path-candidate",
                plus_two_project,
                plus_two_admission,
            )
            .with_logical_path(logical_path),
        );

    run_local_workflow_from_project(input)
}

fn run_canonical_workflow_with_summary() -> (Sys5LocalWorkflowReport, Sys5SemanticSummary, String) {
    let base_project = build_project(Sys5SourceInput::inline(
        SYS5_WORKFLOW_PATH,
        SYS5_WORKFLOW_SOURCE,
    ))
    .expect("base ordinary source must check and project before workflow execution");
    let base_summary = base_project.semantic_summary().clone();
    let checked_program_identity_ref = base_project.checked_program_identity_ref().to_string();
    let base_admission = base_project
        .prepare_finite_admission(source_declared_request())
        .expect("base admission must be sealed before workflow execution");

    let plus_two_project = build_project(Sys5SourceInput::inline(
        SYS5_WORKFLOW_PATH,
        designated_plus_two_source(),
    ))
    .expect("accepted patch source must be checked/projected before workflow execution");
    let plus_two_admission = plus_two_project
        .prepare_finite_admission(source_declared_request())
        .expect("accepted patch admission must be sealed before workflow execution");

    let owner_rmw_project = build_project(Sys5SourceInput::inline(
        SYS5_WORKFLOW_PATH,
        owner_rmw_changed_plus_two_source(),
    ))
    .expect("rejected patch source must still be checked/projected before workflow execution");
    let owner_rmw_admission = owner_rmw_project
        .prepare_finite_admission(source_declared_request())
        .expect("rejected patch admission must be sealed before workflow execution");

    let input = Sys5LocalWorkflowInput::from_project_and_admission(base_project, base_admission)
        .with_patch_project(Sys5LocalWorkflowPatchProject::from_project_and_admission(
            "designated-plus-two",
            plus_two_project,
            plus_two_admission,
        ))
        .with_patch_project(Sys5LocalWorkflowPatchProject::from_project_and_admission(
            "owner-rmw-change",
            owner_rmw_project,
            owner_rmw_admission,
        ));

    let report = run_local_workflow_from_project(input)
        .expect("canonical local workflow should run from checked projects only");
    (report, base_summary, checked_program_identity_ref)
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

fn designated_plus_two_source() -> String {
    SYS5_WORKFLOW_SOURCE.replace(
        "participant_input[self].focus + 1",
        "participant_input[self].focus + 2",
    )
}

fn owner_rmw_changed_plus_two_source() -> String {
    designated_plus_two_source().replace(
        "avatar[target].hp = avatar[target].hp - avatar[self].atk",
        "avatar[target].hp = avatar[target].hp + 1",
    )
}

fn all_expected_steps() -> [Sys5LocalWorkflowStep; 15] {
    [
        Sys5LocalWorkflowStep::Startup,
        Sys5LocalWorkflowStep::Attack,
        Sys5LocalWorkflowStep::DesignatedPublish,
        Sys5LocalWorkflowStep::ViewerConsume,
        Sys5LocalWorkflowStep::RelationPrimary,
        Sys5LocalWorkflowStep::ParticipantALeave,
        Sys5LocalWorkflowStep::PresentationGap,
        Sys5LocalWorkflowStep::FreshReacquire,
        Sys5LocalWorkflowStep::Save,
        Sys5LocalWorkflowStep::Restore,
        Sys5LocalWorkflowStep::PatchAccepted,
        Sys5LocalWorkflowStep::PatchRejected,
        Sys5LocalWorkflowStep::ConsumerCapabilityRevoke,
        Sys5LocalWorkflowStep::FailedConsume,
        Sys5LocalWorkflowStep::Verification,
    ]
}

fn assert_contains_steps(report: &Sys5LocalWorkflowReport, expected: &[Sys5LocalWorkflowStep]) {
    for step in expected {
        assert!(report.has_step(*step), "workflow missing step {step:?}");
    }
}

fn assert_joined_rows_include(report: &Sys5LocalWorkflowReport, expected_kinds: &[&str]) {
    for kind in expected_kinds {
        assert!(
            report.has_joined_row_kind(kind),
            "joined workflow report missing `{kind}` row:\n{}",
            report.render_compact()
        );
    }
}

fn expect_string_field(value: &Value, field: &str, expected: &str, failures: &mut Vec<String>) {
    match value.get(field).and_then(Value::as_str) {
        Some(actual) if actual == expected => {}
        Some(actual) => failures.push(format!(
            "{field} mismatch: expected `{expected}`, got `{actual}`"
        )),
        None => failures.push(format!("{field} missing or not a string")),
    }
}

fn expect_exact_json_field(
    value: &Value,
    field: &str,
    expected: Value,
    failures: &mut Vec<String>,
) {
    match value.get(field) {
        Some(actual) if actual == &expected => {}
        Some(actual) => failures.push(format!(
            "{field} did not exactly mirror the checked semantic summary: expected {expected:#}, got {actual:#}"
        )),
        None => failures.push(format!("{field} missing from workflow report JSON")),
    }
}

fn expect_mapping_spans_and_attack_linkage(
    mappings: Option<&Value>,
    checked_program_identity_ref: &str,
    failures: &mut Vec<String>,
) {
    let Some(rows) = mappings.and_then(Value::as_array) else {
        failures.push("source_core_artifact_mappings missing or not an array".to_string());
        return;
    };
    let mut spans = std::collections::BTreeSet::new();
    for row in rows {
        let span = &row["source_span"];
        let start_line = span["start_line"].as_u64().unwrap_or_default();
        let start_column = span["start_column"].as_u64().unwrap_or_default();
        let end_line = span["end_line"].as_u64().unwrap_or_default();
        let end_column = span["end_column"].as_u64().unwrap_or_default();
        if start_line == 0 || end_line == 0 || (start_line, start_column) == (end_line, end_column)
        {
            failures.push(format!("mapping has zero or empty source span: {row:#}"));
        }
        spans.insert(format!(
            "{start_line}:{start_column}-{end_line}:{end_column}"
        ));
    }
    if spans.len() < 2 {
        failures.push(format!(
            "source_core_artifact_mappings must retain distinct source spans, got {spans:?}"
        ));
    }
    let attack_linked = rows.iter().any(|row| {
        row["operation_id"].as_str() == Some("attack")
            && row["artifact_locus"].as_str() == Some("WorldAuthority")
            && row["artifact_kind"].as_str() == Some("owner-rmw-evaluation")
            && row["source_path"].as_str() == Some(SYS5_WORKFLOW_PATH)
            && row["checked_program_identity"].as_str() == Some(checked_program_identity_ref)
            && row["core_ref"]
                .as_str()
                .is_some_and(|core_ref| !core_ref.is_empty())
            && row["fragment_ref"]
                .as_str()
                .is_some_and(|fragment_ref| !fragment_ref.is_empty())
    });
    if !attack_linked {
        failures.push(
            "source_core_artifact_mappings lacks attack -> WorldAuthority owner-rmw linkage"
                .to_string(),
        );
    }
}

fn expect_runtime_summary_evidence(runtime_summary: Option<&Value>, failures: &mut Vec<String>) {
    let Some(summary) = runtime_summary.and_then(Value::as_object) else {
        failures.push("runtime_summary missing or not an object".to_string());
        return;
    };
    expect_nonempty_object_string(summary, "state_digest", failures);
    expect_runtime_relation(summary.get("relations"), failures);
    expect_designated_value(summary.get("designated_values"), failures);
    expect_lifecycle_ref(summary.get("save_lifecycle_refs"), "SaveCut", failures);
    expect_lifecycle_ref(summary.get("save_lifecycle_refs"), "RestoreCut", failures);
    expect_lifecycle_ref(
        summary.get("patch_lifecycle_refs"),
        "PatchAccepted",
        failures,
    );
    expect_lifecycle_ref(
        summary.get("patch_lifecycle_refs"),
        "PatchRejected",
        failures,
    );
}

fn expect_runtime_relation(relations: Option<&Value>, failures: &mut Vec<String>) {
    let Some(rows) = relations.and_then(Value::as_array) else {
        failures.push("runtime_summary.relations missing or not an array".to_string());
        return;
    };
    let has_relation = rows.iter().any(|row| {
        row["relation"].as_str() == Some("bird_follow")
            && row["selected_anchor"].as_str() == Some("participant_a_shoulder")
            && row["selected_floor"].as_str() == Some("live-primary")
            && nonempty_json_string(&row["semantic_digest"])
            && nonempty_json_string(&row["lineage_ref"])
    });
    if !has_relation {
        failures.push(
            "runtime_summary.relations lacks bird_follow semantic_digest/selected anchor/floor/lineage_ref"
                .to_string(),
        );
    }
}

fn expect_designated_value(values: Option<&Value>, failures: &mut Vec<String>) {
    let Some(rows) = values.and_then(Value::as_array) else {
        failures.push("runtime_summary.designated_values missing or not an array".to_string());
        return;
    };
    let has_designated = rows.iter().any(|row| {
        row["value_name"].as_str() == Some("WorldAuthority.result")
            && nonempty_json_string(&row["cache_ref"])
            && nonempty_json_string(&row["version_ref"])
            && row["latest_value"].as_i64().is_some()
    });
    if !has_designated {
        failures.push(
            "runtime_summary.designated_values lacks cache/version evidence for WorldAuthority.result"
                .to_string(),
        );
    }
}

fn expect_lifecycle_ref(refs: Option<&Value>, kind: &str, failures: &mut Vec<String>) {
    let Some(rows) = refs.and_then(Value::as_array) else {
        failures.push(format!("runtime_summary lifecycle refs missing for {kind}"));
        return;
    };
    let has_ref = rows.iter().any(|row| {
        row["kind"].as_str() == Some(kind)
            && nonempty_json_string(&row["occurrence_ref"])
            && nonempty_json_string(&row["before_frontier_ref"])
            && nonempty_json_string(&row["after_frontier_ref"])
    });
    if !has_ref {
        failures.push(format!(
            "runtime_summary lifecycle refs lack {kind} occurrence/frontier refs"
        ));
    }
}

fn expect_nonempty_object_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
    failures: &mut Vec<String>,
) {
    if !object.get(field).is_some_and(nonempty_json_string) {
        failures.push(format!("runtime_summary.{field} missing or empty"));
    }
}

fn expect_typed_causal_segment(
    value: &Value,
    prefix: &str,
    required_fragments: &[&str],
    failures: &mut Vec<String>,
) {
    let joined_rows = value
        .get("joined_rows")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let matching_detail = joined_rows
        .iter()
        .filter_map(|row| row["detail_ref"].as_str())
        .find(|detail| {
            detail.starts_with(prefix)
                && required_fragments
                    .iter()
                    .all(|fragment| detail.contains(fragment))
                && !detail.contains("source_span=0:0-0:0")
        });
    if matching_detail.is_none() {
        failures.push(format!(
            "joined_rows lacks actual {prefix} causal row with logical_path, nonzero source_span, Core/artifact/edge refs, and runtime occurrence IDs"
        ));
    }
}

fn expect_actual_presentation_gap_evidence(value: &Value, failures: &mut Vec<String>) {
    let empty_results = Vec::new();
    let results = match value
        .get("presentation_gap_results")
        .and_then(Value::as_array)
    {
        Some(results) => results,
        None => {
            failures.push("presentation_gap_results missing or not an array".to_string());
            &empty_results
        }
    };
    let matching_result = results.iter().find(|row| {
        row["relation"].as_str() == Some("bird_follow")
            && row["consumer_locus"].as_str() == Some("ViewerC")
            && row["projection_kind"].as_str() == Some("consumer-local-fallback")
            && row["publishes_value"].as_bool() == Some(false)
            && row["absolute_stream_count"].as_u64() == Some(0)
            && row["restriction"].as_str() == Some("restricted")
            && nonempty_json_string(&row["restriction_ref"])
            && row["redaction"].as_str() == Some("restricted")
            && nonempty_json_string(&row["redaction_ref"])
            && nonempty_json_string(&row["selected_anchor"])
            && nonempty_json_string(&row["selected_floor"])
            && nonempty_json_string(&row["context_frontier_ref"])
            && nonempty_json_string(&row["selected_anchor_ref"])
            && nonempty_json_string(&row["selected_floor_ref"])
            && row["semantic_digest_before"].as_str() == row["semantic_digest_after"].as_str()
            && nonempty_json_string(&row["semantic_digest_before"])
            && row["endpoint_count_before"].as_u64() == row["endpoint_count_after"].as_u64()
            && row["endpoint_count_before"].as_u64().is_some()
            && row["derived_from_actual_action"].as_bool() == Some(true)
    });
    if matching_result.is_none() {
        failures.push(
            "presentation_gap_results lacks actual typed bird_follow ViewerC consumer-local-fallback result with no value publication, no absolute stream, restricted/redacted refs, stable semantic digest, stable endpoint count, and anchor/floor/context frontier refs"
                .to_string(),
        );
    }

    let empty_joined_rows = Vec::new();
    let joined_detail = value
        .get("joined_rows")
        .and_then(Value::as_array)
        .unwrap_or(&empty_joined_rows)
        .iter()
        .filter_map(|row| row["detail_ref"].as_str())
        .find(|detail| {
            detail.starts_with("typed-presentation-gap:")
                && detail.contains("relation=bird_follow")
                && detail.contains("consumer_locus=ViewerC")
                && detail.contains("projection_kind=consumer-local-fallback")
                && detail.contains("publishes_value=false")
                && detail.contains("absolute_stream_count=0")
                && detail.contains("restriction=restricted")
                && detail.contains("redaction=restricted")
                && detail.contains("semantic_digest_before=")
                && detail.contains("semantic_digest_after=")
                && detail.contains("endpoint_count_before=")
                && detail.contains("endpoint_count_after=")
                && detail.contains("selected_anchor=")
                && detail.contains("selected_floor=")
                && detail.contains("context_frontier_ref=")
                && detail.contains("derived_from_actual_action=true")
        });
    if joined_detail.is_none() {
        failures.push(
            "joined_rows lacks typed-presentation-gap detail derived from the actual ViewerC presentation-gap action"
                .to_string(),
        );
    }
}

fn expect_execution_branch_evidence(value: &Value, failures: &mut Vec<String>) {
    let Some(rows) = value.get("joined_rows").and_then(Value::as_array) else {
        failures.push("joined_rows missing or not an array".to_string());
        return;
    };
    let mut missing_branch = 0usize;
    let mut branches = std::collections::BTreeSet::new();
    let mut branch_occurrences = std::collections::BTreeSet::new();
    let mut duplicate_branch_occurrences = Vec::new();
    let mut active_transient_rows = Vec::new();

    for row in rows.iter().filter(|row| is_runtime_or_typed_row(row)) {
        let Some(branch) = row_execution_branch(row) else {
            missing_branch += 1;
            continue;
        };
        if !matches!(
            branch,
            "active_prefix" | "discarded_post_cut" | "active_restored"
        ) {
            failures.push(format!(
                "joined row has unknown execution_branch `{branch}`"
            ));
            continue;
        }
        branches.insert(branch.to_string());
        let detail_ref = row["detail_ref"].as_str().unwrap_or_default();
        let kind = row["kind"].as_str().unwrap_or_default();
        if branch != "discarded_post_cut"
            && (kind == "post_cut_transition"
                || detail_ref.contains("workflow-post-cut")
                || detail_ref.contains("discarded_post_cut"))
        {
            active_transient_rows.push(detail_ref.to_string());
        }
        for occurrence_ref in occurrence_refs_from_joined_row(row) {
            if !branch_occurrences.insert((branch.to_string(), occurrence_ref.clone())) {
                duplicate_branch_occurrences.push(format!("{branch}:{occurrence_ref}"));
            }
        }
    }

    if missing_branch > 0 {
        failures.push(format!(
            "{missing_branch} joined runtime/typed rows missing observer-safe execution_branch"
        ));
    }
    for required in ["active_prefix", "discarded_post_cut", "active_restored"] {
        if !branches.contains(required) {
            failures.push(format!(
                "execution_branch `{required}` has no joined runtime row"
            ));
        }
    }
    if !duplicate_branch_occurrences.is_empty() {
        failures.push(format!(
            "(execution_branch, occurrence_ref) is not unique: {duplicate_branch_occurrences:?}"
        ));
    }
    if !active_transient_rows.is_empty() {
        failures.push(format!(
            "discarded post-cut transition rows appeared as active: {active_transient_rows:?}"
        ));
    }
}

fn is_runtime_or_typed_row(row: &Value) -> bool {
    let kind = row["kind"].as_str().unwrap_or_default();
    let detail_ref = row["detail_ref"].as_str().unwrap_or_default();
    matches!(
        kind,
        "runtime_occurrence"
            | "typed_causal_segment"
            | "presentation_gap"
            | "owner_state_mutation"
            | "designated_result_version"
            | "relation_selected_fallback"
            | "save_cut"
            | "restore_cut"
            | "patch_lifecycle"
            | "post_cut_transition"
    ) || detail_ref.starts_with("typed-segment:")
        || detail_ref.starts_with("typed-presentation-gap:")
}

fn row_execution_branch(row: &Value) -> Option<&str> {
    row["execution_branch"].as_str().or_else(|| {
        row.get("detail")
            .and_then(|detail| detail["execution_branch"].as_str())
    })
}

fn occurrence_refs_from_joined_row(row: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(detail_ref) = row["detail_ref"].as_str() {
        refs.extend(occurrence_refs_from_text(detail_ref));
    }
    if let Some(detail) = row.get("detail").and_then(Value::as_object) {
        refs.extend(detail.iter().filter_map(|(key, value)| {
            (key.contains("occurrence"))
                .then(|| value.as_str())
                .flatten()
                .map(str::to_string)
        }));
    }
    refs.sort();
    refs.dedup();
    refs
}

fn occurrence_refs_from_text(text: &str) -> Vec<String> {
    text.split(';')
        .filter_map(|part| {
            let (key, value) = part.split_once('=')?;
            (key.contains("occurrence") && !value.is_empty()).then(|| value.to_string())
        })
        .collect()
}

fn expect_patch_occurrence_provenance(
    value: &Value,
    patch_id: &str,
    verdict: &str,
    lifecycle_kind: &str,
    failures: &mut Vec<String>,
) {
    let provenance = value
        .get("patch_provenance")
        .and_then(Value::as_array)
        .and_then(|rows| {
            rows.iter().find(|row| {
                row["patch_id"].as_str() == Some(patch_id)
                    && row["verdict"].as_str() == Some(verdict)
            })
        });
    let occurrence_ref = provenance
        .and_then(|row| row["patch_occurrence_ref"].as_str())
        .filter(|text| !text.is_empty());
    let Some(occurrence_ref) = occurrence_ref else {
        failures.push(format!(
            "patch_provenance lacks exact patch_occurrence_ref for {patch_id}/{verdict}"
        ));
        return;
    };

    let lifecycle_matches = value
        .pointer("/runtime_summary/patch_lifecycle_refs")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter(|row| {
                    row["kind"].as_str() == Some(lifecycle_kind)
                        && row["occurrence_ref"].as_str() == Some(occurrence_ref)
                })
                .count()
        })
        .unwrap_or_default();
    if lifecycle_matches != 1 {
        failures.push(format!(
            "patch_occurrence_ref for {patch_id}/{verdict} must match exactly one {lifecycle_kind} runtime_summary lifecycle ref, got {lifecycle_matches}"
        ));
    }

    let joined_matches = value
        .get("joined_rows")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter_map(|row| row["detail_ref"].as_str())
                .filter(|detail| {
                    detail.contains(&format!("lifecycle:{lifecycle_kind}:"))
                        && detail.contains(&format!("patch_occurrence_ref={occurrence_ref}"))
                })
                .count()
        })
        .unwrap_or_default();
    if joined_matches != 1 {
        failures.push(format!(
            "patch_occurrence_ref for {patch_id}/{verdict} must match exactly one joined lifecycle row, got {joined_matches}"
        ));
    }
}

fn expect_source_derived_participant_leave(value: &Value, failures: &mut Vec<String>) {
    let actual_steps = value
        .get("actual_steps")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let has_leave_step = actual_steps
        .iter()
        .any(|step| step.as_str() == Some("participant_a_leave"));
    if !has_leave_step {
        failures.push("actual_steps lacks ParticipantA leave semantic action".to_string());
    }
    let has_direct_shortcut_step = actual_steps
        .iter()
        .any(|step| step.as_str() == Some("relation_invalidate_fallback"));
    if has_direct_shortcut_step {
        failures.push("workflow still exposes direct relation invalidation step label".to_string());
    }

    let leave_results = value
        .get("participant_leave_results")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let has_leave_result = leave_results.iter().any(|row| {
        row["participant_locus"].as_str() == Some("ParticipantA")
            && row["relation"].as_str() == Some("bird_follow")
            && row["source_derived"].as_bool() == Some(true)
            && row["membership_epoch_monotone"].as_bool() == Some(true)
            && nonempty_json_string(&row["membership_epoch_before_ref"])
            && nonempty_json_string(&row["membership_epoch_after_ref"])
            && nonempty_json_string(&row["incarnation_before_ref"])
            && nonempty_json_string(&row["incarnation_after_ref"])
            && row["invalidates_relation_primary"].as_bool() == Some(true)
            && row["selected_anchor_after"].as_str() == Some("participant_b_shoulder")
            && row["selected_floor_after"].as_str() == Some("fallback-anchor")
            && row["direct_owner_mutation"].as_bool() == Some(false)
            && row["direct_consumer_mutation"].as_bool() == Some(false)
            && row["fixture_schedule_authority_injection"].as_bool() == Some(false)
    });
    if !has_leave_result {
        failures.push(
            "participant_leave_results lacks source-derived monotone membership/incarnation transition joined to relation fallback without direct owner/consumer mutation"
                .to_string(),
        );
    }

    let joined_leave = value
        .get("joined_rows")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
        .iter()
        .filter_map(|row| row["detail_ref"].as_str())
        .any(|detail| {
            detail.starts_with("typed-participant-leave:")
                && detail.contains("participant_locus=ParticipantA")
                && detail.contains("relation=bird_follow")
                && detail.contains("source_derived=true")
                && detail.contains("membership_epoch_monotone=true")
                && detail.contains("invalidates_relation_primary=true")
                && detail.contains("selected_floor_after=fallback-anchor")
                && detail.contains("direct_owner_mutation=false")
                && detail.contains("direct_consumer_mutation=false")
        });
    if !joined_leave {
        failures.push(
            "joined_rows lacks typed/source-derived ParticipantA leave occurrence joined to fallback causality"
                .to_string(),
        );
    }
}

fn nonempty_json_string(value: &Value) -> bool {
    value.as_str().is_some_and(|text| !text.is_empty())
}

fn assert_observer_safe_no_raw_material(text: &str) {
    for denied in [
        "/home/",
        "/root/",
        "source_text",
        "raw_source",
        "avatar[target].hp =",
        "avatar[self].atk",
        "participant_input[self].focus +",
        "hp - atk",
        "hp + 1",
        "focus + 1",
        "focus + 2",
        "raw_authority",
        "raw_capability",
        "raw_credential",
        "raw_witness",
        "capability_secret",
        "witness_secret",
        "credential_secret",
        "manual_route",
        "route_override",
        "expected_result",
        "expected_json",
        "private avatar atk",
    ] {
        assert!(
            !text.contains(denied),
            "observer-safe workflow output leaked denied fragment `{denied}`:\n{text}"
        );
    }
}

fn observer_unsafe_fragment_labels(text: &str) -> Vec<&'static str> {
    let mut leaks = Vec::new();
    for (label, denied) in [
        ("home-path", "/home/"),
        ("root-path", "/root/"),
        ("raw-source-field", "source_text"),
        ("raw-source-label", "raw_source"),
        ("private-owner-expression", "avatar[target].hp ="),
        ("private-atk-read", "avatar[self].atk"),
        ("designated-expression", "participant_input[self].focus +"),
        ("raw-authority", "raw_authority"),
        ("raw-capability", "raw_capability"),
        ("raw-credential", "raw_credential"),
        ("raw-witness", "raw_witness"),
        ("capability-secret", "capability_secret"),
        ("witness-secret", "witness_secret"),
        ("credential-secret", "credential_secret"),
        ("manual-route", "manual_route"),
        ("route-override", "route_override"),
        ("expected-result", "expected_result"),
        ("expected-json", "expected_json"),
        ("private-avatar-atk", "private avatar atk"),
    ] {
        if text.contains(denied) {
            leaks.push(label);
        }
    }
    leaks
}
