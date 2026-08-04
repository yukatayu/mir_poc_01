use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_runtime::m8_runtime_admission::{
    EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionEvidence, M8Runtime, M8RuntimeAdmission,
    M8RuntimeInstance, M8SecurityClass,
};
use mir_runtime::m8_runtime_authority::{
    M8AuthorityState, M8CapabilityGrant, M8MembershipRecord, M8WitnessRecord,
};
use mir_runtime::m8_runtime_relation_projection::{
    M8AnchorSample, M8BindingInvalidation, M8LeaseInventory, M8LeaseRecord, M8Point,
    M8PresentationContext, M8PresentationFallback, M8ProjectionDiagnosticKind, M8ProjectionKind,
    M8RelationAuthorityUse, M8RelationDiagnosticKind, M8RelationProjectionRuntime,
    M8RelationProjectionSeed, M8RelationReacquire, M8RelationTraceKind, M8RestrictionPolicy,
    M8Transform2,
};
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const OWNER: &str = "S";
const RELATION_NAME: &str = "bird_follow";
const BINDING_FRONTIER: &str = "bird_binding_frontier";
const REACQUIRED_FRONTIER: &str = "bird_binding_frontier:reacquired";
const RELATION_LEASE_REF: &str = "bird_binding_frontier/live";
const REACQUIRE_RELATION_LEASE_REF: &str = "lease:bird_follow:binding_epoch:2";
const RELATION_MEMBERSHIP_EPOCH1_REF: &str = "membership:self:S:relation-binding-epoch1";
const RELATION_MEMBERSHIP_EPOCH2_REF: &str = "membership:self:S:relation-binding-epoch2";
const INVALIDATE_RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:invalidate_primary:binding_epoch1";
const INVALIDATE_RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:invalidate_primary:witness_epoch1";
const REACQUIRE_RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:reacquire_primary:binding_epoch2";
const REACQUIRE_RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:reacquire_primary:witness_epoch2";
const ABSENT_RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:invalidate_primary:absent";
const ABSENT_RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:invalidate_primary:absent";
const ABSENT_REACQUIRE_RELATION_CAPABILITY_REF: &str =
    "cap:relation:bird_follow:S:self:reacquire_primary:absent";
const ABSENT_REACQUIRE_RELATION_WITNESS_REF: &str =
    "witness:relation:bird_follow:S:self:reacquire_primary:absent";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_surface_fixture(name: &str) -> (String, String) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    (relative, source)
}

fn checked_relation_fixture() -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture("maintained_bird_relation.mir");
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("maintained relation fixture checks through M7 before M8 admission");
    (path, source, checked)
}

fn residual_source_ref(
    checked: &CheckedSurfaceV0,
    kind: ResidualObligationKind,
    name: &str,
) -> SourceRef {
    checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|entry| entry.kind() == kind && entry.name() == name)
        .unwrap_or_else(|| panic!("missing residual {kind:?}/{name}"))
        .source_ref()
        .clone()
}

fn relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: "bird_follow".into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:consumer-visible"),
        redaction: EvidenceRedaction::new("consumer:C"),
        source_ref,
    }
}

fn private_relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: "bird_follow".into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:private")
            .with_class(M8SecurityClass::Private),
        redaction: EvidenceRedaction::new("relation:redact-private"),
        source_ref,
    }
}

fn relation_lifetime_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationLifetime {
        relation: "bird_follow".into(),
        live_lease: RELATION_LEASE_REF.into(),
        binding_frontier: BINDING_FRONTIER.into(),
        source_ref,
    }
}

fn relation_fallback_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationFallbackValidity {
        relation: "bird_follow".into(),
        primary_epoch: "primary_epoch".into(),
        fallback_epoch: "fallback_epoch".into(),
        source_ref,
    }
}

fn relation_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::Visibility,
            "bird_follow",
        )))
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::RelationLifetime,
            "bird_follow",
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::FallbackValidity,
            "bird_follow",
        )))
}

fn private_relation_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(private_relation_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::Visibility,
            "bird_follow",
        )))
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::RelationLifetime,
            "bird_follow",
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::FallbackValidity,
            "bird_follow",
        )))
}

fn admitted_relation_instance() -> (String, String, M8RuntimeInstance) {
    let (path, source, checked) = checked_relation_fixture();
    let admission = relation_admission_for(&checked);
    let instance = M8Runtime::default()
        .admit(checked, admission)
        .expect("exact relation residual evidence admits through M8 Phase 1");
    (path, source, instance)
}

fn live_relation_lease() -> M8LeaseRecord {
    M8LeaseRecord::live(RELATION_LEASE_REF)
        .for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_binding_frontier(BINDING_FRONTIER)
        .with_epoch("binding_epoch:1")
}

fn fresh_reacquire_relation_lease() -> M8LeaseRecord {
    M8LeaseRecord::live(REACQUIRE_RELATION_LEASE_REF)
        .for_relation(RELATION_NAME)
        .with_owner_locus(OWNER)
        .with_binding_frontier(REACQUIRED_FRONTIER)
        .with_epoch("binding_epoch:2")
        .with_anchor_epoch("primary_epoch:2")
}

fn relation_lease_inventory() -> M8LeaseInventory {
    M8LeaseInventory::default()
        .with_live_lease(live_relation_lease())
        .with_live_lease(fresh_reacquire_relation_lease())
}

fn relation_runtime() -> (String, String, M8RelationProjectionRuntime) {
    let (path, source, instance) = admitted_relation_instance();
    let runtime = instance.into_relation_projection(
        M8RelationProjectionSeed::new()
            .with_authority_state(relation_authority_state())
            .with_live_leases(relation_lease_inventory())
            .with_relation_policy("bird_follow", M8RestrictionPolicy::Restricted)
            .with_subject_policy("bird", M8RestrictionPolicy::Restricted)
            .with_anchor_policy("perch_anchor", M8RestrictionPolicy::Public)
            .with_anchor_policy("nest_anchor", M8RestrictionPolicy::Private),
    );
    (path, source, runtime)
}

fn private_relation_runtime_with_public_presentation_seed()
-> (String, String, M8RelationProjectionRuntime) {
    let (path, source, checked) = checked_relation_fixture();
    let admission = private_relation_admission_for(&checked);
    let instance = M8Runtime::default()
        .admit(checked, admission)
        .expect("private relation evidence admits before presentation");
    let runtime = instance.into_relation_projection(
        M8RelationProjectionSeed::new()
            .with_authority_state(relation_authority_state())
            .with_live_leases(relation_lease_inventory())
            .with_relation_policy("bird_follow", M8RestrictionPolicy::Public)
            .with_subject_policy("bird", M8RestrictionPolicy::Public)
            .with_anchor_policy("perch_anchor", M8RestrictionPolicy::Public)
            .with_anchor_policy("nest_anchor", M8RestrictionPolicy::Public),
    );
    (path, source, runtime)
}

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("fixture contains {needle:?}"));
    start..start + needle.len()
}

fn line_column(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 1_u32;
    let mut column = 1_u32;
    for byte in source[..byte_offset].bytes() {
        if byte == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn expected_source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
    let range = byte_range(source, lexeme);
    let (start_line, start_column) = line_column(source, range.start);
    let (end_line, end_column) = line_column(source, range.end);
    SourceRef::new(
        path.to_owned(),
        start_line,
        start_column,
        end_line,
        end_column,
    )
}

fn relation_source_ref(path: &str, source: &str) -> SourceRef {
    expected_source_ref(
        path,
        source,
        "relation bird_follow at S {\n  subject bird: Bird\n  primary perch_anchor epoch primary_epoch transform translate(3, -2)\n  fallback nest_anchor epoch fallback_epoch transform identity\n  bind frontier bird_binding_frontier\n  publish relation\n  project at C local\n}",
    )
}

fn primary_context() -> M8PresentationContext {
    M8PresentationContext::for_consumer("C")
        .with_frontier("bird_binding_frontier")
        .with_anchor_sample(
            M8AnchorSample::new("perch_anchor")
                .with_epoch("primary_epoch")
                .with_frontier("bird_binding_frontier")
                .with_pose(M8Point::new(10, 20))
                .with_policy(M8RestrictionPolicy::Public),
        )
        .with_anchor_sample(
            M8AnchorSample::new("nest_anchor")
                .with_epoch("fallback_epoch")
                .with_frontier("bird_binding_frontier")
                .with_pose(M8Point::new(-30, 5))
                .with_policy(M8RestrictionPolicy::Private),
        )
}

fn split_frame_context() -> M8PresentationContext {
    M8PresentationContext::for_consumer("C")
        .with_frontier("bird_binding_frontier")
        .with_anchor_sample(
            M8AnchorSample::new("perch_anchor")
                .with_epoch("primary_epoch")
                .with_frontier("other_frontier")
                .with_pose(M8Point::new(10, 20))
                .with_policy(M8RestrictionPolicy::Public),
        )
}

fn stale_primary_context() -> M8PresentationContext {
    M8PresentationContext::for_consumer("C")
        .with_frontier("bird_binding_frontier")
        .with_anchor_sample(
            M8AnchorSample::new("perch_anchor")
                .with_epoch("stale_primary_epoch")
                .with_frontier("bird_binding_frontier")
                .with_pose(M8Point::new(10, 20))
                .with_policy(M8RestrictionPolicy::Public),
        )
}

fn shortage_context_with_local_fallback() -> M8PresentationContext {
    M8PresentationContext::for_consumer("C")
        .with_frontier("bird_binding_frontier")
        .with_presentation_fallback(
            M8PresentationFallback::hold_last_local("bird", M8Point::new(40, 40))
                .with_policy(M8RestrictionPolicy::Private),
        )
}

fn public_shortage_context_with_local_fallback() -> M8PresentationContext {
    M8PresentationContext::for_consumer("C")
        .with_frontier("bird_binding_frontier")
        .with_presentation_fallback(
            M8PresentationFallback::hold_last_local("bird", M8Point::new(40, 40))
                .with_policy(M8RestrictionPolicy::Public),
        )
}

fn relation_authority_state() -> M8AuthorityState {
    M8AuthorityState::new()
        .with_membership_record(
            M8MembershipRecord::already_admitted(RELATION_MEMBERSHIP_EPOCH1_REF)
                .with_principal("self")
                .with_locus("S")
                .with_epoch("binding_epoch:1"),
        )
        .with_membership_record(
            M8MembershipRecord::already_admitted(RELATION_MEMBERSHIP_EPOCH2_REF)
                .with_principal("self")
                .with_locus("S")
                .with_epoch("binding_epoch:2"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(INVALIDATE_RELATION_CAPABILITY_REF)
                .for_relation_transition("bird_follow", "invalidate_primary")
                .with_owner_locus("S")
                .with_principal("self")
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
                .with_binding_epoch("binding_epoch:1"),
        )
        .with_witness_record(
            M8WitnessRecord::live(INVALIDATE_RELATION_WITNESS_REF)
                .for_capability(INVALIDATE_RELATION_CAPABILITY_REF)
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
                .with_epoch("witness_epoch:1"),
        )
        .with_capability_grant(
            M8CapabilityGrant::already_admitted(REACQUIRE_RELATION_CAPABILITY_REF)
                .for_relation_transition("bird_follow", "reacquire_primary")
                .with_owner_locus("S")
                .with_principal("self")
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
                .with_binding_epoch("binding_epoch:2"),
        )
        .with_witness_record(
            M8WitnessRecord::live(REACQUIRE_RELATION_WITNESS_REF)
                .for_capability(REACQUIRE_RELATION_CAPABILITY_REF)
                .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
                .with_epoch("witness_epoch:2"),
        )
}

fn invalidate_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation("bird_follow")
        .with_owner_locus("S")
        .with_transition("invalidate_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
        .with_capability_ref(INVALIDATE_RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:1")
        .with_witness_ref(INVALIDATE_RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:1")
}

fn absent_invalidate_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation("bird_follow")
        .with_owner_locus("S")
        .with_transition("invalidate_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_EPOCH1_REF)
        .with_capability_ref(ABSENT_RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:1")
        .with_witness_ref(ABSENT_RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:1")
}

fn reacquire_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation("bird_follow")
        .with_owner_locus("S")
        .with_transition("reacquire_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
        .with_capability_ref(REACQUIRE_RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:2")
        .with_witness_ref(REACQUIRE_RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:2")
}

fn absent_reacquire_relation_authority_use() -> M8RelationAuthorityUse {
    M8RelationAuthorityUse::for_relation("bird_follow")
        .with_owner_locus("S")
        .with_transition("reacquire_primary")
        .with_principal("self")
        .with_membership_ref(RELATION_MEMBERSHIP_EPOCH2_REF)
        .with_capability_ref(ABSENT_REACQUIRE_RELATION_CAPABILITY_REF)
        .with_binding_epoch("binding_epoch:2")
        .with_witness_ref(ABSENT_REACQUIRE_RELATION_WITNESS_REF)
        .with_witness_epoch("witness_epoch:2")
}

#[test]
fn relation_projection_is_consumer_local_and_never_changes_semantic_owner_or_publishes_absolute_stream()
 {
    let (_, _, mut runtime) = relation_runtime();
    assert_eq!(runtime.semantic_relation("bird_follow").owner_locus(), "S");
    let before = runtime.semantic_snapshot();

    let projection = runtime
        .project_relation("bird_follow", primary_context())
        .expect("coherent consumer presentation context projects relation locally");

    assert_eq!(projection.relation(), "bird_follow");
    assert_eq!(projection.consumer_locus(), "C");
    assert_eq!(
        projection.kind(),
        M8ProjectionKind::ConsumerLocalPresentation
    );
    assert!(!projection.consumer_is_semantic_owner());
    assert_eq!(runtime.semantic_relation("bird_follow").owner_locus(), "S");
    assert_eq!(runtime.semantic_snapshot(), before);
    assert!(!projection.publishes_value());
    assert!(projection.absolute_value_stream().is_empty());
    assert!(
        runtime
            .semantic_snapshot()
            .published_values_for("bird")
            .is_empty()
    );
}

#[test]
fn consumer_projection_uses_one_presentation_frame_and_preserves_primary_relative_transform() {
    let (_, _, mut runtime) = relation_runtime();

    let projection = runtime
        .project_relation("bird_follow", primary_context())
        .expect("primary projection has a coherent single presentation frame");

    assert_eq!(projection.subject(), "bird");
    assert_eq!(projection.selected_anchor(), "perch_anchor");
    assert_eq!(projection.context_frontier(), "bird_binding_frontier");
    assert!(projection.uses_single_presentation_frame());
    assert_eq!(
        projection
            .anchor_sample("perch_anchor")
            .expect("primary anchor sample retained")
            .frontier(),
        "bird_binding_frontier"
    );
    assert_eq!(
        projection.relative_transform(),
        &M8Transform2::translate(3, -2)
    );
    assert_eq!(projection.anchor_pose(), Some(M8Point::new(10, 20)));
    assert_eq!(projection.derived_pose(), Some(M8Point::new(13, 18)));
}

#[test]
fn split_frame_or_stale_anchor_sample_rejects_without_semantic_mutation() {
    for (context, expected_kind) in [
        (
            split_frame_context(),
            M8ProjectionDiagnosticKind::SplitFramePresentationContext,
        ),
        (
            stale_primary_context(),
            M8ProjectionDiagnosticKind::StaleAnchorSample,
        ),
    ] {
        let (path, source, mut runtime) = relation_runtime();
        let relation_ref = relation_source_ref(&path, &source);
        let before = runtime.semantic_snapshot();
        let before_relation = runtime.semantic_relation("bird_follow").clone();

        let diagnostics = runtime
            .project_relation("bird_follow", context)
            .expect_err("incoherent presentation input rejects");

        assert_eq!(diagnostics.primary().kind(), expected_kind);
        assert_eq!(diagnostics.primary().source_ref(), &relation_ref);
        assert_eq!(runtime.semantic_snapshot(), before);
        assert_eq!(runtime.semantic_relation("bird_follow"), &before_relation);
    }
}

#[test]
fn semantic_primary_invalidation_advances_monotonically_to_fallback_without_same_lineage_repromotion()
 {
    let (path, source, mut runtime) = relation_runtime();
    let relation_ref = relation_source_ref(&path, &source);
    let initial = runtime.semantic_relation("bird_follow").clone();
    let initial_snapshot = runtime.semantic_snapshot();
    let initial_trace = runtime.trace().clone();
    assert_eq!(initial.selected_option_index(), 0);
    assert_eq!(initial.selected_anchor(), "perch_anchor");

    let rejected = runtime
        .invalidate_primary(
            "bird_follow",
            absent_invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier("bird_binding_frontier:degraded"),
        )
        .expect_err("semantic relation mutation without typed owner authority rejects");
    assert_eq!(
        rejected.primary().kind(),
        M8RelationDiagnosticKind::MissingRelationAuthority
    );
    assert_eq!(rejected.primary().relation(), "bird_follow");
    assert_eq!(rejected.primary().source_ref(), &relation_ref);
    assert_eq!(runtime.semantic_relation("bird_follow"), &initial);
    assert_eq!(runtime.semantic_snapshot(), initial_snapshot);
    assert_eq!(runtime.trace(), &initial_trace);

    let advanced = runtime
        .invalidate_primary(
            "bird_follow",
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier("bird_binding_frontier:degraded"),
        )
        .expect("semantic primary invalidation advances to fallback");
    assert_eq!(advanced.previous_option_index(), 0);
    assert_eq!(advanced.current_option_index(), 1);
    assert_eq!(
        runtime.semantic_relation("bird_follow").selected_anchor(),
        "nest_anchor"
    );
    assert_ne!(
        runtime
            .semantic_relation("bird_follow")
            .activation_frontier(),
        initial.activation_frontier()
    );
    assert!(
        runtime
            .semantic_relation("bird_follow")
            .lineage()
            .starts_with(initial.lineage())
    );

    let unchanged = runtime
        .note_primary_available_same_lineage("bird_follow", "perch_anchor")
        .expect("same-lineage primary availability is only an ignored observation");
    assert_eq!(unchanged.current_option_index(), 1);
    assert_eq!(
        runtime.semantic_relation("bird_follow").selected_anchor(),
        "nest_anchor"
    );
    assert_eq!(
        runtime.trace().kinds(),
        vec![
            M8RelationTraceKind::SemanticPrimaryInvalidated,
            M8RelationTraceKind::RelationOptionAdvanced,
            M8RelationTraceKind::SameLineagePrimaryReturnIgnored,
        ]
    );
}

#[test]
fn fresh_reacquire_with_new_epoch_and_witness_starts_new_lineage_and_may_select_primary() {
    let (_, _, mut runtime) = relation_runtime();
    runtime
        .invalidate_primary(
            "bird_follow",
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier("bird_binding_frontier:degraded"),
        )
        .expect("semantic primary invalidation advances to fallback");
    let degraded = runtime.semantic_relation("bird_follow").clone();
    let degraded_snapshot = runtime.semantic_snapshot();
    let degraded_trace = runtime.trace().clone();

    let rejected = runtime
        .reacquire_primary(
            "bird_follow",
            absent_reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier("bird_binding_frontier:reacquired"),
        )
        .expect_err("fresh reacquire witness alone cannot mint relation authority");
    assert_eq!(
        rejected.primary().kind(),
        M8RelationDiagnosticKind::MissingRelationAuthority
    );
    assert_eq!(runtime.semantic_relation("bird_follow"), &degraded);
    assert_eq!(runtime.semantic_snapshot(), degraded_snapshot);
    assert_eq!(runtime.trace(), &degraded_trace);

    let reacquired = runtime
        .reacquire_primary(
            "bird_follow",
            reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier("bird_binding_frontier:reacquired"),
        )
        .expect("fresh witness and binding epoch permit a new primary lineage");

    assert_eq!(reacquired.previous_option_index(), 1);
    assert_eq!(reacquired.current_option_index(), 0);
    let relation = runtime.semantic_relation("bird_follow");
    assert_eq!(relation.selected_anchor(), "perch_anchor");
    assert_eq!(relation.primary_epoch(), "primary_epoch:2");
    assert_eq!(relation.binding_epoch(), "binding_epoch:2");
    assert_eq!(
        relation.activation_frontier(),
        "bird_binding_frontier:reacquired"
    );
    assert_eq!(
        reacquired.authority().capability_ref(),
        Some(REACQUIRE_RELATION_CAPABILITY_REF)
    );
    assert_eq!(
        reacquired.authority().witness_ref(),
        Some(REACQUIRE_RELATION_WITNESS_REF)
    );
    assert_eq!(
        reacquired.fresh_reacquire_witness(),
        REACQUIRE_RELATION_WITNESS_REF
    );
    assert_ne!(relation.lineage(), degraded.lineage());
    assert!(
        runtime
            .trace()
            .kinds()
            .contains(&M8RelationTraceKind::FreshRelationLineageReacquired)
    );
}

#[test]
fn private_admitted_relation_evidence_cannot_be_weakened_by_public_presentation_seed() {
    let (_, _, mut runtime) = private_relation_runtime_with_public_presentation_seed();
    let before = runtime.semantic_snapshot();

    let projection = runtime
        .project_relation("bird_follow", primary_context())
        .expect("coherent presentation still projects relation locally");

    assert_eq!(runtime.semantic_snapshot(), before);
    assert!(
        projection
            .derived_visibility()
            .is_at_least(M8RestrictionPolicy::Private)
    );
    assert_eq!(projection.redaction_policy(), "relation:redact-private");
    assert!(projection.absolute_value_stream().is_empty());
}

#[test]
fn private_admitted_relation_evidence_is_retained_on_consumer_local_fallback_projection() {
    let (_, _, mut runtime) = private_relation_runtime_with_public_presentation_seed();
    let before = runtime.semantic_snapshot();

    let projection = runtime
        .project_relation("bird_follow", public_shortage_context_with_local_fallback())
        .expect("sample shortage may use a consumer-local fallback");

    assert_eq!(runtime.semantic_snapshot(), before);
    assert_eq!(projection.kind(), M8ProjectionKind::ConsumerLocalFallback);
    assert!(projection.is_consumer_local_fallback());
    assert!(
        projection
            .derived_visibility()
            .is_at_least(M8RestrictionPolicy::Private)
    );
    assert_eq!(projection.redaction_policy(), "relation:redact-private");
    assert!(projection.absolute_value_stream().is_empty());
}

#[test]
fn fresh_reacquire_requires_inventory_bound_witness_ref_exactly() {
    let (_, _, mut runtime) = relation_runtime();
    runtime
        .invalidate_primary(
            "bird_follow",
            invalidate_relation_authority_use(),
            M8BindingInvalidation::anchor_unavailable("perch_anchor")
                .with_frontier("bird_binding_frontier:degraded"),
        )
        .expect("semantic primary invalidation advances to fallback");
    let degraded = runtime.semantic_relation("bird_follow").clone();
    let degraded_snapshot = runtime.semantic_snapshot();
    let degraded_trace = runtime.trace().clone();

    let forged = runtime
        .reacquire_primary(
            "bird_follow",
            reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness("witness:relation:bird_follow:forged")
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier("bird_binding_frontier:reacquired"),
        )
        .expect_err("fresh witness must be the exact live REACQUIRE witness reference");
    assert_eq!(
        forged.primary().kind(),
        M8RelationDiagnosticKind::MissingRelationAuthority
    );
    assert_eq!(runtime.semantic_relation("bird_follow"), &degraded);
    assert_eq!(runtime.semantic_snapshot(), degraded_snapshot);
    assert_eq!(runtime.trace(), &degraded_trace);

    let accepted = runtime
        .reacquire_primary(
            "bird_follow",
            reacquire_relation_authority_use(),
            M8RelationReacquire::new("perch_anchor")
                .with_anchor_epoch("primary_epoch:2")
                .with_binding_epoch("binding_epoch:2")
                .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                .with_frontier("bird_binding_frontier:reacquired"),
        )
        .expect("inventory-bound exact REACQUIRE witness succeeds");
    assert_eq!(
        accepted.fresh_reacquire_witness(),
        REACQUIRE_RELATION_WITNESS_REF
    );
}

#[test]
fn fresh_reacquire_primary_epoch_and_frontier_must_match_pre_admitted_authority_binding() {
    for (anchor_epoch, frontier) in [
        ("primary_epoch:forged", "bird_binding_frontier:reacquired"),
        ("primary_epoch:2", "bird_binding_frontier:forged"),
    ] {
        let (_, _, mut runtime) = relation_runtime();
        runtime
            .invalidate_primary(
                "bird_follow",
                invalidate_relation_authority_use(),
                M8BindingInvalidation::anchor_unavailable("perch_anchor")
                    .with_frontier("bird_binding_frontier:degraded"),
            )
            .expect("semantic primary invalidation advances to fallback");
        let degraded = runtime.semantic_relation("bird_follow").clone();
        let degraded_snapshot = runtime.semantic_snapshot();
        let degraded_trace = runtime.trace().clone();

        let rejected = runtime
            .reacquire_primary(
                "bird_follow",
                reacquire_relation_authority_use(),
                M8RelationReacquire::new("perch_anchor")
                    .with_anchor_epoch(anchor_epoch)
                    .with_binding_epoch("binding_epoch:2")
                    .with_fresh_witness(REACQUIRE_RELATION_WITNESS_REF)
                    .with_fresh_lease_ref(REACQUIRE_RELATION_LEASE_REF)
                    .with_frontier(frontier),
            )
            .expect_err("fresh authority does not permit forged epoch/frontier values");
        assert_eq!(
            rejected.primary().kind(),
            M8RelationDiagnosticKind::MissingRelationAuthority
        );
        assert_eq!(runtime.semantic_relation("bird_follow"), &degraded);
        assert_eq!(runtime.semantic_snapshot(), degraded_snapshot);
        assert_eq!(runtime.trace(), &degraded_trace);
    }
}

#[test]
fn presentation_sample_shortage_uses_consumer_local_fallback_and_admitted_redaction_only() {
    let (_, _, mut runtime) = relation_runtime();
    let before_relation = runtime.semantic_relation("bird_follow").clone();
    let before_snapshot = runtime.semantic_snapshot();

    let projection = runtime
        .project_relation("bird_follow", shortage_context_with_local_fallback())
        .expect("sample shortage may use a consumer-local presentation fallback");

    assert_eq!(projection.kind(), M8ProjectionKind::ConsumerLocalFallback);
    assert!(projection.is_consumer_local_fallback());
    assert_eq!(projection.fallback_pose(), Some(M8Point::new(40, 40)));
    assert_eq!(runtime.semantic_relation("bird_follow"), &before_relation);
    assert_eq!(runtime.semantic_snapshot(), before_snapshot);
    assert!(!runtime.semantic_snapshot().contains_presentation_contexts());
    assert_eq!(
        projection.derived_visibility(),
        M8RestrictionPolicy::Private
    );
    assert!(
        projection
            .derived_visibility()
            .is_at_least(M8RestrictionPolicy::Restricted)
    );
    assert_eq!(projection.redaction_policy(), "consumer:C");
}
