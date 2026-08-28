use std::{collections::BTreeMap, fs, path::PathBuf};

use crate::sys5_local_slice::{
    Sys5CommunicationSummary, Sys5LocalAdmissionRequest, Sys5LocalRuntimeProfile,
    Sys5LocalSliceError, Sys5RelationAction, Sys5RelationBootstrapPolicy,
    Sys5RelationDispatchEventKind, Sys5SemanticSummary, Sys5SourceInput, Sys5SourceSpan,
    Sys5VerticalAction, Sys5VerticalDiagnosticKind, Sys5VerticalSliceRuntime, build_project,
};

const SYS5_VERTICAL_PATH: &str = "tests/inline/sys5_vertical_refined_source.mir";

const SYS5_VERTICAL_SOURCE: &str = r#"
module Mirrorea.Sys5.VerticalRefined

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
  primary participant_a_shoulder epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = participant_input[self].focus + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

const SYS5_PRIVATE_RESULT_SOURCE: &str = r#"
module Mirrorea.Sys5.VerticalPrivateResult

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
  primary participant_a_shoulder epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = avatar[self].atk + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

fn source_declared_request() -> Sys5LocalAdmissionRequest {
    Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-vertical-world",
        "incarnation:self:WorldAuthority:epoch:sys5-vertical-world",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "self",
        "ParticipantA",
        "epoch:sys5-vertical-a",
        "incarnation:self:ParticipantA:epoch:sys5-vertical-a",
    )
    .with_source_declared_membership(
        "self",
        "ParticipantB",
        "epoch:sys5-vertical-b",
        "incarnation:self:ParticipantB:epoch:sys5-vertical-b",
    )
    .with_source_declared_membership(
        "self",
        "ViewerC",
        "epoch:sys5-vertical-c",
        "incarnation:self:ViewerC:epoch:sys5-vertical-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn vertical_runtime(source: impl Into<String>) -> Sys5VerticalSliceRuntime {
    let project = build_project(Sys5SourceInput::inline(SYS5_VERTICAL_PATH, source.into()))
        .expect("refined SYS-5 ordinary source must check and project");
    let summary = project.semantic_summary();

    for (operation, source_locus, owner_locus) in [
        ("init_avatar_hp", "ParticipantA", "WorldAuthority"),
        ("init_avatar_atk", "ParticipantA", "WorldAuthority"),
        ("init_focus", "WorldAuthority", "ParticipantA"),
        ("attack", "ParticipantA", "WorldAuthority"),
    ] {
        assert_source_core_and_owner_endpoint(summary, operation, source_locus, owner_locus);
    }

    let prepared = project
        .prepare_finite_admission(source_declared_request())
        .expect("refined SYS-5 admission uses source-declared identity without caller values");
    let admission_summary = prepared.observer_safe_admission_summary();
    assert!(admission_summary.is_source_derived());
    assert!(admission_summary.derived_from_sealed_admission());
    assert!(admission_summary.is_complete_for_projection());
    assert!(!admission_summary.public_api_or_wire_contract());
    let verification = admission_summary
        .verification_discharge("finite_refinement")
        .expect("optional verifier discharge remains separately visible");
    assert!(verification.is_discharged());
    assert!(verification.has_source_ref());
    assert!(verification.has_finite_refinement_evidence_ref());
    assert!(!verification.is_merged_into_auth());

    prepared.start_vertical_slice_runtime().expect(
        "startup must dispatch three source/Core/generated endpoint init ops from an empty seed",
    )
}

fn assert_source_core_and_owner_endpoint(
    summary: &Sys5SemanticSummary,
    operation: &str,
    source_locus: &str,
    owner_locus: &str,
) {
    let invocation = summary
        .artifacts
        .iter()
        .find(|row| {
            row.operation_id == operation
                && row.locus == source_locus
                && row.kind == "owner-request-invocation"
                && row.derived_from_checked_core
        })
        .unwrap_or_else(|| panic!("{operation} is missing its source-locus invocation artifact"));
    assert!(!invocation.core_ref.is_empty());
    assert!(!invocation.fragment_ref.is_empty());

    let owner = summary
        .artifacts
        .iter()
        .find(|row| {
            row.operation_id == operation
                && row.locus == owner_locus
                && row.kind == "owner-rmw-evaluation"
                && row.derived_from_checked_core
        })
        .unwrap_or_else(|| panic!("{operation} is missing its owner-locus Core artifact"));
    assert!(!owner.core_ref.is_empty());
    assert!(!owner.fragment_ref.is_empty());

    for (kind, from, to) in [
        ("owner-request", source_locus, owner_locus),
        ("owner-reply-receipt", owner_locus, source_locus),
    ] {
        let edge = summary
            .generated_communication
            .iter()
            .find(|row| {
                row.operation_id == operation
                    && row.kind == kind
                    && row.from_locus == from
                    && row.to_locus == to
                    && row.derived_from_checked_core
                    && !row.transfers_authority
            })
            .unwrap_or_else(|| panic!("{operation} is missing generated {kind} {from}->{to}"));
        assert!(!edge.edge_ref.is_empty());
        assert!(
            edge.core_ref
                .as_ref()
                .is_some_and(|core_ref| !core_ref.is_empty()),
            "{operation} {kind} must retain checked Core provenance"
        );
        assert!(!edge.source_fragment_ref.is_empty());
        assert!(!edge.target_fragment_ref.is_empty());
    }
}

fn generated_edge<'a>(
    summary: &'a Sys5SemanticSummary,
    operation: &str,
    kind: &str,
    from_locus: &str,
    to_locus: &str,
) -> &'a Sys5CommunicationSummary {
    summary
        .generated_communication
        .iter()
        .find(|row| {
            row.operation_id == operation
                && row.kind == kind
                && row.from_locus == from_locus
                && row.to_locus == to_locus
                && row.derived_from_checked_core
        })
        .unwrap_or_else(|| {
            panic!("{operation} missing generated {kind} edge {from_locus}->{to_locus}")
        })
}

fn source_span_key(span: Sys5SourceSpan) -> String {
    format!(
        "{}:{}-{}:{}",
        span.start_line, span.start_column, span.end_line, span.end_column
    )
}

fn typed_segment_for_edge<'a>(
    report: &'a str,
    segment_kind: &str,
    edge_ref: &str,
) -> BTreeMap<&'a str, &'a str> {
    let prefix = format!("typed-segment:{segment_kind}:");
    let matches = report
        .lines()
        .filter_map(|line| line.strip_prefix(&prefix))
        .map(parse_segment_fields)
        .filter(|fields| fields.get("edge_ref").copied() == Some(edge_ref))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "joined report must carry exactly one inspectable typed {segment_kind} segment for edge {edge_ref}; report was:\n{report}"
    );
    matches.into_iter().next().expect("single segment")
}

fn typed_lifecycle_segment<'a>(
    report: &'a str,
    segment_kind: &str,
    value_name: &str,
    consumer_locus: &str,
) -> BTreeMap<&'a str, &'a str> {
    let prefix = format!("typed-lifecycle-segment:{segment_kind}:");
    let matches = report
        .lines()
        .filter_map(|line| line.strip_prefix(&prefix))
        .map(parse_segment_fields)
        .filter(|fields| {
            fields.get("value_name").copied() == Some(value_name)
                && fields.get("consumer_locus").copied() == Some(consumer_locus)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "joined report must carry exactly one inspectable M9 lifecycle {segment_kind} segment for {value_name}/{consumer_locus}; report was:\n{report}"
    );
    matches
        .into_iter()
        .next()
        .expect("single lifecycle segment")
}

fn parse_segment_fields(row: &str) -> BTreeMap<&str, &str> {
    let mut fields = BTreeMap::new();
    for entry in row.split(';') {
        let (key, value) = entry
            .split_once('=')
            .unwrap_or_else(|| panic!("typed segment entry `{entry}` must be key=value"));
        assert!(
            !key.is_empty() && !value.is_empty(),
            "typed segment entries must be nonempty: `{entry}`"
        );
        assert!(
            fields.insert(key, value).is_none(),
            "typed segment duplicated field `{key}`"
        );
    }
    fields
}

fn single_report_row(report: &str, prefix: &str) -> String {
    let rows = report
        .lines()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        1,
        "report must carry exactly one row with prefix `{prefix}`; report was:\n{report}"
    );
    rows[0].to_string()
}

fn assert_exact_segment_fields(fields: &BTreeMap<&str, &str>, expected_keys: &[&str]) {
    let actual = fields.keys().copied().collect::<Vec<_>>();
    let mut expected = expected_keys.to_vec();
    expected.sort_unstable();
    assert_eq!(
        actual, expected,
        "typed joined segment must expose the exact inspectable field set"
    );
}

fn assert_logical_source_span_redacted(
    fields: &BTreeMap<&str, &str>,
    source_path: &str,
    source_span: Sys5SourceSpan,
) {
    assert_eq!(fields.get("logical_path").copied(), Some(source_path));
    assert_eq!(
        fields.get("source_span").copied(),
        Some(source_span_key(source_span).as_str())
    );
    let logical_path = fields
        .get("logical_path")
        .expect("logical source path is present");
    assert!(
        !logical_path.starts_with('/') && !logical_path.contains("/home/"),
        "joined provenance must expose a logical path only, not an absolute host path: {logical_path}"
    );
    for value in fields.values() {
        assert!(
            !value.contains("avatar[self]")
                && !value.contains("avatar[target]")
                && !value.contains("participant_input[self]"),
            "joined provenance must not embed source text in a redacted source span: {value}"
        );
    }
}

fn assert_request_identity(label: &str, id: &str) {
    assert!(
        id.starts_with("sys4-request-") || id.starts_with("sys5-relation-request:"),
        "{label} must be retained as request identity, got {id}"
    );
}

fn assert_causal_occurrence_id(label: &str, id: &str) {
    assert!(
        id.starts_with("sys4-") || id.starts_with("sys4-m8:"),
        "{label} must be an actual SYS-4/M8 occurrence id, got {id}"
    );
    assert!(
        !id.starts_with("sys4-request-") && !id.starts_with("sys5-relation-request:"),
        "{label} must not substitute request identity for a causal occurrence: {id}"
    );
}

fn typed_segment_field_injection_issues(rows: &[&str]) -> Vec<String> {
    let mut issues = Vec::new();
    for row in rows {
        let body = row
            .splitn(3, ':')
            .nth(2)
            .unwrap_or_else(|| panic!("typed segment row has a kind and field body: {row}"));
        let mut counts = BTreeMap::<&str, usize>::new();
        for field in body.split(';') {
            let Some((key, value)) = field.split_once('=') else {
                issues.push(format!("malformed-field:{field}"));
                continue;
            };
            *counts.entry(key).or_insert(0) += 1;
            if key == "core_ref" && value == "forged.mir" {
                issues.push("forged-core-ref-field:core_ref=forged.mir".to_string());
            }
        }
        for (key, count) in counts {
            if count > 1 {
                issues.push(format!("duplicate-field:{key}:{count}"));
            }
        }
    }
    issues
}

#[test]
fn public_vertical_admission_surface_exposes_no_bootstrap_value_or_state_seed_api() {
    let source = runtime_source("sys5_local_slice.rs");
    let admission_surface = source_slice(
        &source,
        "pub struct Sys5LocalAdmissionRequest",
        "pub struct Sys5PreparedAdmission",
    );

    assert_contains_all(
        admission_surface,
        &[
            "pub struct Sys5LocalAdmissionRequest",
            "source_declared",
            "with_source_declared_membership",
            "with_relation_bootstrap_policy",
            "with_auth_discharge",
            "with_optional_verification_discharge",
        ],
    );
    assert_contains_none(
        admission_surface,
        &[
            "Sys5SourceBootstrapInt",
            "Sys5BootstrapIntShape",
            "source_bootstrap_ints",
            "with_source_bootstrap_int",
            "Sys4InitialStateSeed",
            "initial_state_seed",
            "with_seed_value",
            "with_state_seed",
            "with_state_value",
            "with_result_value",
        ],
    );
}

#[test]
fn typed_devtools_rejects_reserved_logical_path_before_segment_emission() {
    let invalid_paths = [
        (
            "semicolon_equals_field_injection",
            "tests/x;core_ref=forged.mir",
        ),
        ("equals_metacharacter", "tests/x=forged.mir"),
        ("newline_control", "tests/x\ncore_ref=forged.mir"),
        (
            "unit_separator_control",
            "tests/x\u{001f}core_ref=forged.mir",
        ),
    ];
    let mut violations = Vec::new();

    for (case, logical_path) in invalid_paths {
        match build_project(Sys5SourceInput::inline(logical_path, SYS5_VERTICAL_SOURCE)) {
            Err(Sys5LocalSliceError::InvalidLogicalSourcePath) => {}
            Err(error) => violations.push(format!(
                "{case}: rejected with {error:?}, expected InvalidLogicalSourcePath allowlist failure"
            )),
            Ok(project) => {
                let mut detail = format!(
                    "{case}: reserved logical path {logical_path:?} was admitted; expected fail-closed rejection before observer typed segment emission"
                );
                if let Some(mut runtime) = project
                    .prepare_finite_admission(source_declared_request())
                    .ok()
                    .and_then(|prepared| prepared.start_vertical_slice_runtime().ok())
                {
                    let _ = runtime
                        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target());
                    let report = runtime.observer_safe_joined_report().render_compact();
                    let typed_rows = report
                        .lines()
                        .filter(|line| line.starts_with("typed-segment:"))
                        .collect::<Vec<_>>();
                    let injection_issues = typed_segment_field_injection_issues(&typed_rows);
                    detail.push_str(&format!(
                        "; emitted_typed_segment_count={}; field_issues={:?}",
                        typed_rows.len(),
                        injection_issues
                    ));
                    assert!(
                        typed_rows.is_empty(),
                        "{detail}; typed_rows={typed_rows:?}"
                    );
                    assert!(
                        injection_issues.is_empty(),
                        "{detail}; typed_rows={typed_rows:?}"
                    );
                }
                violations.push(detail);
            }
        }
    }

    assert!(
        violations.is_empty(),
        "logical source path allowlist must reject reserved typed-devtools metacharacters before any observer typed segment can be emitted:\n{}",
        violations.join("\n")
    );
}

#[test]
fn startup_dispatches_three_literal_initializers_from_empty_seed_with_created_none_receipts() {
    assert_contains_all(
        SYS5_VERTICAL_SOURCE,
        &[
            "init_avatar_hp",
            "avatar[self].hp = 21",
            "init_avatar_atk",
            "avatar[self].atk = 5",
            "init_focus",
            "participant_input[self].focus = 10",
        ],
    );

    let runtime = vertical_runtime(SYS5_VERTICAL_SOURCE);
    assert_eq!(runtime.local_fabric_instance_count(), 1);
    assert_eq!(
        runtime.observer_safe_int("WorldAuthority", "avatar", "self", "hp"),
        Some(21)
    );
    assert_eq!(
        runtime.observer_safe_int("ParticipantA", "participant_input", "self", "focus"),
        Some(10)
    );
    assert_eq!(
        runtime.observer_safe_int("WorldAuthority", "avatar", "self", "atk"),
        None,
        "private atk initialization must not be exposed through observer-safe state"
    );

    let report = runtime.observer_safe_joined_report().render_compact();
    assert_contains_all(
        &report,
        &[
            "startup-receipt:init_avatar_hp:ParticipantA->WorldAuthority:avatar[self].hp:Created(None->21)",
            "startup-receipt:init_focus:WorldAuthority->ParticipantA:participant_input[self].focus:Created(None->10)",
            "startup-occurrence:init_avatar_hp:",
            "startup-occurrence:init_avatar_atk:",
            "startup-occurrence:init_focus:",
        ],
    );
    let private_atk_startup = single_report_row(
        &report,
        "startup-receipt:init_avatar_atk:ParticipantA->WorldAuthority:",
    );
    assert!(
        private_atk_startup.contains(":private-cell-ref:sys5-relation-sha256-v1:"),
        "private startup cell identity must be an opaque private-cell-ref marker: {private_atk_startup}"
    );
    assert!(
        private_atk_startup.ends_with(":Created(None->[private])"),
        "private startup value remains redacted while retaining Created(None->[private]): {private_atk_startup}"
    );
    assert_contains_none(&private_atk_startup, &["avatar[self].atk"]);
    let repeat_report = vertical_runtime(SYS5_VERTICAL_SOURCE)
        .observer_safe_joined_report()
        .render_compact();
    assert_eq!(
        private_atk_startup,
        single_report_row(
            &repeat_report,
            "startup-receipt:init_avatar_atk:ParticipantA->WorldAuthority:"
        ),
        "opaque private-cell-ref must be stable for the same source/runtime startup"
    );
    assert_contains_none(
        &report,
        &[
            "avatar[self].atk",
            "old=0",
            "0->21",
            "0->5",
            "0->10",
            "source-bootstrap",
            "caller-seed",
            "state-seed",
        ],
    );
}

#[test]
fn owner_attack_uses_actual_generated_endpoint_and_reports_observer_safe_hp_old_new() {
    let mut runtime = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let fabric_ref = runtime.local_fabric_instance_ref().to_string();
    let before_digest = runtime.observer_safe_state_digest();

    let receipt = runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("ParticipantA attack dispatches through the generated owner endpoint");
    assert_eq!(receipt.fabric_instance_ref(), fabric_ref);
    assert!(receipt.is_source_derived());
    assert_eq!(receipt.source_locus(), "ParticipantA");
    assert_eq!(receipt.owner_locus(), Some("WorldAuthority"));
    assert!(receipt.no_direct_cross_locus_store_mutation());
    assert_eq!(
        receipt.generated_endpoint_chain().source_locus(),
        "ParticipantA"
    );
    assert_eq!(
        receipt.generated_endpoint_chain().target_locus(),
        "WorldAuthority"
    );
    assert_eq!(
        receipt
            .owner_mutation("WorldAuthority", "avatar", "self", "hp")
            .expect("observer-safe hp mutation is reported")
            .old_new_int(),
        (21, 16)
    );
    assert_eq!(
        runtime.observer_safe_int("WorldAuthority", "avatar", "self", "hp"),
        Some(16)
    );
    assert_ne!(
        runtime.observer_safe_state_digest(),
        before_digest,
        "public hp mutation must change the observer-safe state digest"
    );

    let chain_debug = format!("{:?}", receipt.generated_endpoint_chain());
    assert_contains_all(
        &chain_debug,
        &[
            "source_ref",
            "core_ref",
            "artifact_ref",
            "edge_ref",
            "request_ref",
            "receive_ref",
            "serve_ref",
        ],
    );
    assert_contains_none(
        &chain_debug,
        &[
            "core_ref: \"none\"",
            "artifact_ref: \"none\"",
            "edge_ref: \"none\"",
        ],
    );
}

#[test]
fn joined_report_retains_one_typed_source_core_edge_occurrence_segment() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_VERTICAL_PATH,
        SYS5_VERTICAL_SOURCE,
    ))
    .expect("refined SYS-5 ordinary source must check and project");
    let edge = generated_edge(
        project.semantic_summary(),
        "attack",
        "owner-request",
        "ParticipantA",
        "WorldAuthority",
    )
    .clone();
    let mut runtime = project
        .prepare_finite_admission(source_declared_request())
        .expect("refined SYS-5 admission uses source-declared identity without caller values")
        .start_vertical_slice_runtime()
        .expect("startup dispatches source-derived literal initializers");

    runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("owner attack dispatches through the generated endpoint");
    let report = runtime.observer_safe_joined_report().render_compact();
    let segment = typed_segment_for_edge(&report, "owner-request", &edge.edge_ref);

    assert_exact_segment_fields(
        &segment,
        &[
            "provenance_kind",
            "logical_path",
            "source_span",
            "core_ref",
            "source_fragment_ref",
            "target_fragment_ref",
            "edge_ref",
            "request_identity",
            "request_enqueue_occurrence_id",
            "dispatch_occurrence_id",
            "receive_occurrence_id",
            "serve_occurrence_id",
            "causal_path",
        ],
    );
    assert_eq!(
        segment.get("provenance_kind").copied(),
        Some("OrdinarySourceCore")
    );
    assert_logical_source_span_redacted(&segment, &edge.source_path, edge.source_span);
    assert_eq!(segment.get("core_ref").copied(), edge.core_ref.as_deref());
    assert_eq!(
        segment.get("source_fragment_ref").copied(),
        Some(edge.source_fragment_ref.as_str())
    );
    assert_eq!(
        segment.get("target_fragment_ref").copied(),
        Some(edge.target_fragment_ref.as_str())
    );
    assert_eq!(
        segment.get("edge_ref").copied(),
        Some(edge.edge_ref.as_str())
    );
    assert_request_identity(
        "owner request identity",
        segment
            .get("request_identity")
            .expect("request identity is separate from occurrences"),
    );
    for field in [
        "request_enqueue_occurrence_id",
        "dispatch_occurrence_id",
        "receive_occurrence_id",
        "serve_occurrence_id",
    ] {
        assert_causal_occurrence_id(field, segment.get(field).expect("occurrence field"));
        assert_ne!(
            segment.get("request_identity"),
            segment.get(field),
            "{field} must not reuse the request identity"
        );
    }
    assert_eq!(
        segment.get("causal_path").copied(),
        Some(
            "request_enqueue_occurrence_id->dispatch_occurrence_id->receive_occurrence_id->serve_occurrence_id"
        )
    );
}

#[test]
fn designated_tick_consume_and_retry_use_concrete_causal_segments_without_reevaluation() {
    let mut missing = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let before_missing = missing.observer_safe_state_digest();
    let err = missing
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect_err("consume before publish must fail closed");
    assert_eq!(
        err.kind(),
        Sys5VerticalDiagnosticKind::MissingPublishedDesignatedValue
    );
    assert!(err.rejected_before_m8_cache_or_state_mutation());
    assert_eq!(missing.observer_safe_state_digest(), before_missing);

    let mut runtime = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let tick = runtime
        .dispatch(Sys5VerticalAction::world_tick("tick:world:1"))
        .expect("world tick publishes the source-derived designated value");
    assert_eq!(tick.designated_value_name(), Some("WorldAuthority.result"));
    assert!(tick.evaluator_locus_is("WorldAuthority"));
    assert_eq!(tick.typed_int(), Some(11));
    assert!(!runtime.viewer_has_designated_evaluator("ViewerC", "WorldAuthority.result"));
    assert_eq!(
        runtime.designated_evaluation_count("WorldAuthority.result"),
        1
    );

    let consume = runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("ViewerC consumes the published designated result once");
    assert_eq!(consume.consumer_locus(), Some("ViewerC"));
    assert_eq!(consume.typed_int(), Some(11));
    assert!(consume.performed_m8_semantic_consumption());
    assert!(!consume.returned_from_designated_cache_after_authority_revalidation());
    assert_eq!(
        runtime.designated_semantic_consumption_count("WorldAuthority.result", "ViewerC"),
        1
    );
    let cache_digest = runtime.designated_cache_digest("WorldAuthority.result", "ViewerC");

    let retry = runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("same consumer retry returns the sealed cached result without re-consuming M8");
    assert_eq!(retry.typed_int(), Some(11));
    assert!(!retry.performed_m8_semantic_consumption());
    assert!(retry.returned_from_designated_cache_after_authority_revalidation());
    assert_eq!(
        runtime.designated_evaluation_count("WorldAuthority.result"),
        1,
        "retry must not re-evaluate the designated value"
    );
    assert_eq!(
        runtime.designated_semantic_consumption_count("WorldAuthority.result", "ViewerC"),
        1,
        "retry must not re-consume the M8 publication"
    );
    assert_eq!(
        runtime.designated_cache_digest("WorldAuthority.result", "ViewerC"),
        cache_digest,
        "retry validation must not rewrite the sealed cache identity"
    );

    let report = runtime.observer_safe_joined_report().render_compact();
    assert_contains_all(
        &report,
        &[
            "segment:designated-input-request:WorldAuthority->ParticipantA:",
            "segment:designated-input-receipt:ParticipantA->WorldAuthority:",
            "segment:designated-result-delivery:WorldAuthority->ViewerC:",
            "causality:designated-input-request->designated-input-receipt",
            "causality:designated-input-receipt->designated-result-delivery",
            "causality:designated-result-delivery->viewer-consume",
        ],
    );
    assert_contains_none(
        &report,
        &[
            "core-ref:none",
            "artifact-ref:none",
            "edge-ref:none",
            "op-hash-fallback",
            "request-id-substitution",
            "segment:none",
        ],
    );
}

#[test]
fn relation_publish_and_m9_consumer_revocation_stay_on_the_same_local_fabric() {
    let mut runtime = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let fabric_ref = runtime.local_fabric_instance_ref().to_string();

    let relation = runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("relation publication uses the same admitted fabric as owner/designated actions");
    assert_eq!(relation.fabric_instance_ref(), fabric_ref);
    assert_eq!(relation.source_locus(), "ParticipantB");
    assert_eq!(relation.owner_locus(), Some("ParticipantB"));
    assert!(relation.no_direct_cross_locus_store_mutation());
    assert!(
        relation
            .observer_relation_shadow("ViewerC", "bird_follow")
            .is_some()
    );

    runtime
        .dispatch(Sys5VerticalAction::world_tick("tick:world:before-revoke"))
        .expect("publish a designated result before revoking its consumer capability");
    let revoke = runtime
        .dispatch(Sys5VerticalAction::revoke_viewer_c_consumer_capability(
            "WorldAuthority.result",
        ))
        .expect("M9 consumer capability revocation is admitted through the same fabric");
    assert_eq!(revoke.fabric_instance_ref(), fabric_ref);
    assert_eq!(revoke.consumer_locus(), Some("ViewerC"));

    let err = runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect_err("revoked M9 consumer capability must fail closed before M8 consume/cache use");
    assert_eq!(
        err.kind(),
        Sys5VerticalDiagnosticKind::MissingConsumerCapability
    );
    assert!(err.rejected_before_m8_cache_or_state_mutation());
    assert_eq!(runtime.local_fabric_instance_count(), 1);

    let report = runtime.observer_safe_joined_report().render_compact();
    assert_contains_all(
        &report,
        &[
            "auth:consumer-capability-revoked",
            "failure:MissingConsumerCapability",
            "relation",
            "bird_follow",
        ],
    );
}

#[test]
fn relation_publication_is_source_core_provenance_not_m9_lifecycle() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_VERTICAL_PATH,
        SYS5_VERTICAL_SOURCE,
    ))
    .expect("refined SYS-5 ordinary source must check and project");
    let relation_edge = generated_edge(
        project.semantic_summary(),
        "bird_follow",
        "relation-projection-publication",
        "ParticipantB",
        "ViewerC",
    )
    .clone();
    let mut runtime = project
        .prepare_finite_admission(source_declared_request())
        .expect("refined SYS-5 admission uses source-declared identity without caller values")
        .start_vertical_slice_runtime()
        .expect("startup dispatches source-derived literal initializers");

    let relation = runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("ordinary relation publication dispatches through the generated endpoint");
    assert!(
        relation.is_source_derived(),
        "ordinary relation publication is source/Core-derived; only external/admin M9 lifecycle requests such as consumer revocation are non-source-derived"
    );
    assert_eq!(relation.source_locus(), "ParticipantB");
    assert_eq!(relation.owner_locus(), Some("ParticipantB"));
    assert_eq!(
        relation.generated_endpoint_chain().source_locus(),
        "ParticipantB"
    );
    assert_eq!(
        relation.generated_endpoint_chain().target_locus(),
        "ViewerC"
    );

    let report = runtime.observer_safe_joined_report().render_compact();
    let relation_segment = typed_segment_for_edge(
        &report,
        "relation-projection-publication",
        &relation_edge.edge_ref,
    );
    assert_exact_segment_fields(
        &relation_segment,
        &[
            "provenance_kind",
            "logical_path",
            "source_span",
            "core_ref",
            "source_fragment_ref",
            "target_fragment_ref",
            "edge_ref",
            "request_identity",
            "owner_publish_occurrence_id",
            "request_enqueue_occurrence_id",
            "dispatch_occurrence_id",
            "receive_occurrence_id",
            "consumer_observe_occurrence_id",
            "serve_occurrence_id",
            "causal_path",
        ],
    );
    assert_eq!(
        relation_segment.get("provenance_kind").copied(),
        Some("OrdinarySourceCore")
    );
    assert_logical_source_span_redacted(
        &relation_segment,
        &relation_edge.source_path,
        relation_edge.source_span,
    );
    assert_eq!(
        relation_segment.get("core_ref").copied(),
        relation_edge.core_ref.as_deref()
    );
    assert_eq!(
        relation_segment.get("source_fragment_ref").copied(),
        Some(relation_edge.source_fragment_ref.as_str())
    );
    assert_eq!(
        relation_segment.get("target_fragment_ref").copied(),
        Some(relation_edge.target_fragment_ref.as_str())
    );
    assert_eq!(
        relation_segment.get("edge_ref").copied(),
        Some(relation_edge.edge_ref.as_str())
    );
    for denied in ["lifecycle_kind", "m9_transition_ref", "m9_generation_ref"] {
        assert!(
            !relation_segment.contains_key(denied),
            "ordinary source/Core relation segment must not be classified as M9 lifecycle field `{denied}`"
        );
    }

    let revoke = runtime
        .dispatch(Sys5VerticalAction::revoke_viewer_c_consumer_capability(
            "WorldAuthority.result",
        ))
        .expect("M9 consumer capability revocation is admitted through the lifecycle seam");
    assert!(
        !revoke.is_source_derived(),
        "M9 admin lifecycle revocation remains distinct from ordinary relation publication"
    );
    let report = runtime.observer_safe_joined_report().render_compact();
    let revoke_segment = typed_lifecycle_segment(
        &report,
        "consumer-capability-revocation",
        "WorldAuthority.result",
        "ViewerC",
    );
    assert_eq!(
        revoke_segment.get("provenance_kind").copied(),
        Some("M9AdmittedLifecycle")
    );
    assert_eq!(
        revoke_segment.get("lifecycle_kind").copied(),
        Some("consumer-capability-revocation")
    );
}

#[test]
fn m9_consumer_capability_revocation_is_admin_lifecycle_not_source_core_operation() {
    let mut runtime = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let fabric_ref = runtime.local_fabric_instance_ref().to_string();

    let revoke = runtime
        .dispatch(Sys5VerticalAction::revoke_viewer_c_consumer_capability(
            "WorldAuthority.result",
        ))
        .expect("M9 consumer capability revocation is admitted through the lifecycle seam");
    assert_eq!(revoke.fabric_instance_ref(), fabric_ref);
    assert!(
        !revoke.is_source_derived(),
        "capability revocation is an external/admin M9 lifecycle request, not an ordinary source/Core operation"
    );
    assert_ne!(
        revoke.source_locus(),
        "WorldAuthority",
        "M9 revocation must not be attributed to the designated evaluator locus"
    );
    assert_eq!(
        revoke.owner_locus(),
        None,
        "M9 revocation must not fabricate an ordinary source owner/Core endpoint"
    );
    assert_eq!(revoke.consumer_locus(), Some("ViewerC"));

    let report = runtime.observer_safe_joined_report().render_compact();
    let segment = typed_lifecycle_segment(
        &report,
        "consumer-capability-revocation",
        "WorldAuthority.result",
        "ViewerC",
    );
    assert_exact_segment_fields(
        &segment,
        &[
            "provenance_kind",
            "lifecycle_kind",
            "value_name",
            "consumer_locus",
            "m9_transition_ref",
            "m9_generation_ref",
        ],
    );
    assert_eq!(
        segment.get("provenance_kind").copied(),
        Some("M9AdmittedLifecycle")
    );
    assert_eq!(
        segment.get("lifecycle_kind").copied(),
        Some("consumer-capability-revocation")
    );
    for denied in ["source_locus", "core_ref", "edge_ref", "occurrence_ref"] {
        assert!(
            !segment.contains_key(denied),
            "M9 lifecycle segment must not expose ordinary source/Core {denied}"
        );
    }
}

#[test]
fn relation_publish_report_keeps_request_identity_distinct_from_causal_occurrences() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_VERTICAL_PATH,
        SYS5_VERTICAL_SOURCE,
    ))
    .expect("refined SYS-5 ordinary source must check and project");
    let edge = generated_edge(
        project.semantic_summary(),
        "bird_follow",
        "relation-projection-publication",
        "ParticipantB",
        "ViewerC",
    )
    .clone();
    let mut runtime = project
        .prepare_finite_admission(source_declared_request())
        .expect("refined SYS-5 admission uses source-declared identity without caller values")
        .start_relation_dispatch_runtime()
        .expect("relation runtime uses the admitted source-derived fabric");

    let receipt = runtime
        .dispatch_relation(Sys5RelationAction::publish_current("bird_follow"))
        .expect("relation publish dispatches through the generated relation endpoint");
    assert_eq!(
        receipt.event_kind(),
        Sys5RelationDispatchEventKind::PublishCurrent
    );
    let chain = receipt.single_endpoint_chain();
    assert_eq!(chain.edge_ref(), edge.edge_ref);
    assert_eq!(chain.core_ref(), edge.core_ref.as_deref());
    assert_eq!(chain.source_fragment_ref(), edge.source_fragment_ref);
    assert_eq!(chain.target_fragment_ref(), edge.target_fragment_ref);
    assert_request_identity("relation request identity", chain.request_identity());
    for (field, id) in [
        (
            "owner_publish_occurrence_id",
            chain.owner_publish_occurrence_id(),
        ),
        (
            "request_enqueue_occurrence_id",
            chain.request_enqueue_occurrence_id(),
        ),
        ("dispatch_occurrence_id", chain.dispatch_occurrence_id()),
        ("receive_occurrence_id", chain.receive_occurrence_id()),
        (
            "consumer_observe_occurrence_id",
            chain.consumer_observe_occurrence_id(),
        ),
        ("serve_occurrence_id", chain.serve_occurrence_id()),
    ] {
        assert_causal_occurrence_id(field, id);
        assert_ne!(
            chain.request_identity(),
            id,
            "{field} must not reuse the relation request identity"
        );
    }

    let segment = typed_segment_for_edge(
        receipt.observer_safe_report(),
        "relation-projection-publication",
        &edge.edge_ref,
    );
    assert_exact_segment_fields(
        &segment,
        &[
            "provenance_kind",
            "logical_path",
            "source_span",
            "core_ref",
            "source_fragment_ref",
            "target_fragment_ref",
            "edge_ref",
            "request_identity",
            "owner_publish_occurrence_id",
            "request_enqueue_occurrence_id",
            "dispatch_occurrence_id",
            "receive_occurrence_id",
            "consumer_observe_occurrence_id",
            "serve_occurrence_id",
            "causal_path",
        ],
    );
    assert_eq!(
        segment.get("provenance_kind").copied(),
        Some("OrdinarySourceCore")
    );
    assert_logical_source_span_redacted(&segment, &edge.source_path, edge.source_span);
    assert_eq!(
        segment.get("request_identity").copied(),
        Some(chain.request_identity())
    );
    assert_eq!(
        segment.get("owner_publish_occurrence_id").copied(),
        Some(chain.owner_publish_occurrence_id())
    );
    assert_eq!(
        segment.get("request_enqueue_occurrence_id").copied(),
        Some(chain.request_enqueue_occurrence_id())
    );
    assert_eq!(
        segment.get("dispatch_occurrence_id").copied(),
        Some(chain.dispatch_occurrence_id())
    );
    assert_eq!(
        segment.get("receive_occurrence_id").copied(),
        Some(chain.receive_occurrence_id())
    );
    assert_eq!(
        segment.get("consumer_observe_occurrence_id").copied(),
        Some(chain.consumer_observe_occurrence_id())
    );
    assert_eq!(
        segment.get("serve_occurrence_id").copied(),
        Some(chain.serve_occurrence_id())
    );
    assert_eq!(
        segment.get("causal_path").copied(),
        Some(
            "owner_publish_occurrence_id->request_enqueue_occurrence_id->dispatch_occurrence_id->receive_occurrence_id->consumer_observe_occurrence_id->serve_occurrence_id"
        )
    );
}

#[test]
fn private_only_state_change_does_not_change_observer_digest_or_leak_private_atk_value() {
    let baseline = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let changed_private_source =
        SYS5_VERTICAL_SOURCE.replace("avatar[self].atk = 5", "avatar[self].atk = 8");
    let changed = vertical_runtime(changed_private_source);

    assert_eq!(
        baseline.observer_safe_int("WorldAuthority", "avatar", "self", "hp"),
        changed.observer_safe_int("WorldAuthority", "avatar", "self", "hp")
    );
    assert_eq!(
        baseline.observer_safe_int("ParticipantA", "participant_input", "self", "focus"),
        changed.observer_safe_int("ParticipantA", "participant_input", "self", "focus")
    );
    assert_eq!(
        baseline.observer_safe_int("WorldAuthority", "avatar", "self", "atk"),
        None
    );
    assert_eq!(
        changed.observer_safe_int("WorldAuthority", "avatar", "self", "atk"),
        None
    );
    assert_eq!(
        baseline.observer_safe_state_digest(),
        changed.observer_safe_state_digest(),
        "observer-safe state digest must ignore private-only atk changes"
    );

    let baseline_report = baseline.observer_safe_joined_report().render_compact();
    let changed_report = changed.observer_safe_joined_report().render_compact();
    assert_contains_none(
        &baseline_report,
        &[
            "Created(None->5)",
            "avatar[self].atk raw",
            "private-atk-value",
        ],
    );
    assert_contains_none(
        &changed_report,
        &[
            "Created(None->8)",
            "avatar[self].atk raw",
            "private-atk-value",
        ],
    );
}

#[test]
fn private_target_or_result_variant_is_rejected_or_redacted_before_viewer_raw_disclosure() {
    let mut runtime = vertical_runtime(SYS5_PRIVATE_RESULT_SOURCE);
    let report_before = runtime.observer_safe_joined_report().render_compact();
    assert_contains_none(
        &report_before,
        &[
            "Created(None->5)",
            "avatar[self].atk raw",
            "private-atk-value",
        ],
    );

    match runtime.dispatch(Sys5VerticalAction::world_tick("tick:world:private-result")) {
        Ok(receipt) => {
            assert_eq!(
                receipt.typed_int(),
                None,
                "private designated result must be redacted from observer-visible receipts"
            );
            assert_contains_none(
                &format!("{receipt:?}"),
                &["typed_int: Some(6)", "value: 6", "avatar[self].atk"],
            );
        }
        Err(err) => {
            assert!(
                matches!(
                    err.kind(),
                    Sys5VerticalDiagnosticKind::DispatchRejected
                        | Sys5VerticalDiagnosticKind::UnknownSourceValue
                        | Sys5VerticalDiagnosticKind::MissingPublishedDesignatedValue
                ),
                "private result may reject, but not as authority/state fabrication: {err:?}"
            );
            assert!(err.rejected_before_m8_cache_or_state_mutation());
        }
    }
}

#[test]
fn unknown_actions_fail_before_endpoint_authority_or_state_and_action_surface_carries_no_route() {
    let source = runtime_source("sys5_local_slice.rs");
    let action_surface = source_slice(
        &source,
        "pub enum Sys5VerticalAction",
        "pub struct Sys5VerticalSliceRuntime",
    );
    assert_contains_all(
        action_surface,
        &[
            "ParticipantAAttackDeclaredTarget",
            "WorldTick",
            "ViewerCConsumeWorldResult",
            "PublishRelation",
            "RevokeViewerCConsumerCapability",
        ],
    );
    assert_contains_none(
        action_surface,
        &[
            "RouteOverride",
            "TargetOverride",
            "CoreRef",
            "AuthorityGrant",
            "StateSeed",
            "StateDelta",
            "ExpectedResult",
            "RuntimeValue",
            "Sys4InitialStateSeed",
            "with_route",
            "with_target",
            "with_core",
            "with_authority",
            "with_state",
            "with_result",
            "with_value",
        ],
    );

    let mut runtime = vertical_runtime(SYS5_VERTICAL_SOURCE);
    let before = runtime.observer_safe_state_digest();
    let unknown_op = runtime
        .dispatch(Sys5VerticalAction::for_test_unknown_source_operation(
            "fabricated_core_write",
        ))
        .expect_err("unknown operation cannot reach generated endpoint dispatch");
    assert_eq!(
        unknown_op.kind(),
        Sys5VerticalDiagnosticKind::UnknownSourceOperation
    );
    assert!(unknown_op.rejected_before_generated_endpoint());
    assert!(unknown_op.rejected_before_m9_authority_use());
    assert!(unknown_op.rejected_before_m8_cache_or_state_mutation());

    let unknown_value = runtime
        .dispatch(Sys5VerticalAction::for_test_unknown_designated_value(
            "WorldAuthority.private",
        ))
        .expect_err("unknown value cannot be substituted for the source-designated result");
    assert_eq!(
        unknown_value.kind(),
        Sys5VerticalDiagnosticKind::UnknownSourceValue
    );
    assert!(unknown_value.rejected_before_generated_endpoint());
    assert!(unknown_value.rejected_before_m9_authority_use());
    assert!(unknown_value.rejected_before_m8_cache_or_state_mutation());
    assert_eq!(
        runtime.observer_safe_state_digest(),
        before,
        "unknown source actions must not mutate state or observer digest"
    );
}

fn runtime_source(relative: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(relative);
    fs::read_to_string(path).expect("runtime source is readable")
}

fn source_slice<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start = source
        .find(start)
        .unwrap_or_else(|| panic!("source slice start `{start}` exists"));
    let end = source[start..]
        .find(end)
        .map(|offset| start + offset)
        .unwrap_or_else(|| panic!("source slice end `{end}` exists after `{start}`"));
    &source[start..end]
}

fn assert_contains_all(text: &str, expected_fragments: &[&str]) {
    for fragment in expected_fragments {
        assert!(
            text.contains(fragment),
            "text missing intended fragment `{fragment}`"
        );
    }
}

fn assert_contains_none(text: &str, denied_fragments: &[&str]) {
    for fragment in denied_fragments {
        assert!(
            !text.contains(fragment),
            "text leaked or accepted denied fragment `{fragment}`"
        );
    }
}
