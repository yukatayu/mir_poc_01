use std::{fmt::Debug, path::PathBuf};

use crate::{
    sys3_projection::{BackendProfile, CommunicationEdgeKind},
    sys4_dispatch::{
        ExternalAction, FabricProgram, FaultInjection, LocalFabric,
        RelationPublicationFailureDisposition, SealedFabricAdmission, Sys4DiagnosticKind,
        Sys4DispatchDiagnostics,
    },
    sys5_local_slice::{
        Sys5LocalAdmissionRequest, Sys5LocalRuntimeProfile, Sys5RelationAction,
        Sys5RelationBootstrapPolicy, Sys5RelationDispatchDiagnosticKind,
        Sys5RelationDispatchEventKind, Sys5RelationProjectionKind, Sys5SourceInput, build_project,
    },
};

const SYS5_LOCAL_TOY_PATH: &str = "tests/inline/sys5_relation_dispatch_surface_v0.mir";

const SYS5_LOCAL_TOY_SOURCE: &str = r#"
module Mirrorea.Sys5.RelationDispatch

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
  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
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

fn valid_admission_request() -> Sys5LocalAdmissionRequest {
    Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-local-1",
        "incarnation:self:WorldAuthority:epoch:sys5-local-1",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "self",
        "ParticipantA",
        "epoch:sys5-local-a",
        "incarnation:self:ParticipantA:epoch:sys5-local-a",
    )
    .with_source_declared_membership(
        "self",
        "ParticipantB",
        "epoch:sys5-local-b",
        "incarnation:self:ParticipantB:epoch:sys5-local-b",
    )
    .with_source_declared_membership(
        "self",
        "ViewerC",
        "epoch:sys5-local-c",
        "incarnation:self:ViewerC:epoch:sys5-local-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

#[test]
fn fresh_reacquire_requires_prior_invalidation_and_preserves_one_shot_binding() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 relation source checks and projects");
    let prepared = project
        .prepare_finite_admission(valid_admission_request())
        .expect("canonical source-derived admission seals M9 inventory");
    let mut runtime = prepared
        .start_relation_dispatch_runtime()
        .expect("prepared admission starts the SYS-5 relation dispatch runtime");

    let before = runtime.observer_safe_relation_state();
    let before_digest = runtime.relation_semantic_digest("bird_follow");
    let before_endpoint_count = runtime.total_endpoint_carrier_count();
    let premature = runtime
        .dispatch_relation(Sys5RelationAction::fresh_reacquire("bird_follow"))
        .expect_err("fresh reacquire before invalidation must fail closed");
    assert_eq!(
        premature.kind(),
        Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected
    );
    assert_eq!(premature.partial_relation_receipt(), None);
    assert_eq!(
        runtime.observer_safe_relation_state(),
        before,
        "premature fresh reacquire must not consume the dormant M9 binding or mutate relation state"
    );
    assert_eq!(
        runtime.relation_semantic_digest("bird_follow"),
        before_digest
    );
    assert_eq!(
        runtime.total_endpoint_carrier_count(),
        before_endpoint_count,
        "premature fresh reacquire must not enqueue a generated endpoint carrier"
    );

    let invalidated = runtime
        .dispatch_relation(Sys5RelationAction::invalidate_primary("bird_follow"))
        .expect("valid invalidation after rejected fresh reacquire still works");
    assert_eq!(
        sys5_relation_request_suffix(invalidated.single_endpoint_chain().request_occurrence_id()),
        0,
        "rejected fresh reacquire must not burn the first generated relation request id"
    );
    assert_eq!(
        invalidated
            .observer_shadow("ViewerC", "bird_follow")
            .expect("ViewerC receives fallback after invalidation")
            .selected_floor(),
        "fallback-anchor"
    );

    let reacquired = runtime
        .dispatch_relation(Sys5RelationAction::fresh_reacquire("bird_follow"))
        .expect("fresh reacquire succeeds exactly once after invalidation");
    assert_eq!(
        sys5_relation_request_suffix(reacquired.single_endpoint_chain().request_occurrence_id()),
        1,
        "accepted fresh reacquire must use the next generated relation request id"
    );
    let after_reacquire = runtime.observer_safe_relation_state();
    let after_reacquire_count = runtime.total_endpoint_carrier_count();

    let duplicate = runtime
        .dispatch_relation(Sys5RelationAction::fresh_reacquire("bird_follow"))
        .expect_err("the finite M9 fresh binding cannot be consumed twice");
    assert_eq!(
        duplicate.kind(),
        Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected
    );
    assert_eq!(duplicate.partial_relation_receipt(), None);
    assert_eq!(runtime.observer_safe_relation_state(), after_reacquire);
    assert_eq!(
        runtime.total_endpoint_carrier_count(),
        after_reacquire_count
    );

    let next_publish = runtime
        .dispatch_relation(Sys5RelationAction::publish_current("bird_follow"))
        .expect("ordinary publication still recovers after duplicate fresh rejection");
    assert_eq!(
        sys5_relation_request_suffix(next_publish.single_endpoint_chain().request_occurrence_id()),
        2,
        "duplicate fresh rejection must not burn a generated relation request id"
    );
}

#[test]
fn canonical_relation_dispatch_runs_through_generated_endpoint_and_preserves_presentation_boundary()
{
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 relation source checks and projects");
    let prepared = project
        .prepare_finite_admission(valid_admission_request())
        .expect("canonical source-derived admission seals M9 inventory");
    let mut runtime = prepared
        .start_relation_dispatch_runtime()
        .expect("prepared admission starts the SYS-5 relation dispatch runtime");

    let initial = runtime
        .dispatch_relation(Sys5RelationAction::publish_current("bird_follow"))
        .expect("initial relation publication crosses generated endpoint");
    let initial_edge = initial.single_endpoint_chain();
    assert_eq!(
        initial.event_kind(),
        Sys5RelationDispatchEventKind::PublishCurrent
    );
    assert_eq!(
        initial_edge.edge_kind(),
        CommunicationEdgeKind::RelationProjectionPublication
    );
    assert_eq!(initial_edge.source_locus(), "ParticipantB");
    assert_eq!(initial_edge.target_locus(), "ViewerC");
    assert!(
        initial_edge
            .request_occurrence_id()
            .starts_with("sys5-relation-request:")
    );
    assert!(
        initial_edge
            .request_enqueue_occurrence_id()
            .starts_with("sys4-outbox-enqueue-")
    );
    assert!(
        initial_edge
            .dispatch_occurrence_id()
            .starts_with("sys4-outbox-dequeue-")
    );
    assert!(
        initial_edge
            .receive_occurrence_id()
            .starts_with("sys4-inbox-enqueue-")
    );
    assert!(
        initial_edge
            .serve_occurrence_id()
            .starts_with("sys5-relation-serve:")
    );
    assert!(!initial_edge.edge_ref().is_empty());
    assert!(!initial_edge.source_fragment_ref().is_empty());
    assert!(!initial_edge.target_fragment_ref().is_empty());
    assert!(initial_edge.core_ref().is_some());
    assert_eq!(
        initial.checked_program_identity_ref(),
        project.checked_program_identity_ref()
    );

    let initial_shadow = initial
        .observer_shadow("ViewerC", "bird_follow")
        .expect("ViewerC receives the initial relation shadow");
    assert_eq!(initial_shadow.owner_locus(), "ParticipantB");
    assert_eq!(initial_shadow.consumer_locus(), "ViewerC");
    assert_eq!(initial_shadow.selected_anchor(), "participant_a_shoulder");
    assert_eq!(initial_shadow.selected_floor(), "live-primary");
    assert!(!initial_shadow.lineage_ref().is_empty());
    assert!(!initial_shadow.semantic_digest().is_empty());
    assert!(initial_shadow.capability_and_witness_are_redacted());

    let semantic_before_gap = runtime
        .relation_semantic_digest("bird_follow")
        .expect("relation semantic digest is observable without payload");
    let endpoint_count_before_gap = runtime.endpoint_carrier_count_for_relation("bird_follow");
    let gap = runtime
        .dispatch_relation(Sys5RelationAction::viewer_presentation_gap("bird_follow"))
        .expect("temporary ViewerC gap is consumer-local presentation only");
    assert_eq!(
        runtime
            .relation_semantic_digest("bird_follow")
            .expect("presentation gap keeps semantic digest observable"),
        semantic_before_gap,
        "presentation fallback must not mutate semantic relation state"
    );
    assert_eq!(
        runtime.endpoint_carrier_count_for_relation("bird_follow"),
        endpoint_count_before_gap,
        "presentation fallback must not enqueue a generated endpoint carrier"
    );
    assert_eq!(
        gap.viewer_projection_kind(),
        Sys5RelationProjectionKind::PresentationFallback
    );
    assert_eq!(
        gap.observer_shadow("ViewerC", "bird_follow")
            .expect("gap report still joins the current semantic shadow")
            .lineage_ref(),
        initial_shadow.lineage_ref(),
        "presentation fallback must keep semantic lineage unchanged"
    );

    let invalidated = runtime
        .dispatch_relation(Sys5RelationAction::invalidate_primary("bird_follow"))
        .expect("A leave invalidates the primary through sealed relation authority");
    let fallback_shadow = invalidated
        .observer_shadow("ViewerC", "bird_follow")
        .expect("ViewerC receives the fallback relation shadow");
    assert_eq!(
        invalidated.event_kind(),
        Sys5RelationDispatchEventKind::InvalidatePrimary
    );
    assert_eq!(
        invalidated.single_endpoint_chain().edge_kind(),
        CommunicationEdgeKind::RelationProjectionPublication
    );
    assert_eq!(fallback_shadow.selected_anchor(), "participant_b_shoulder");
    assert_eq!(fallback_shadow.selected_floor(), "fallback-anchor");
    assert_ne!(fallback_shadow.lineage_ref(), initial_shadow.lineage_ref());
    assert_ne!(
        fallback_shadow.semantic_digest(),
        initial_shadow.semantic_digest(),
        "semantic invalidation must advance relation state before publication"
    );

    let reacquired = runtime
        .dispatch_relation(Sys5RelationAction::fresh_reacquire("bird_follow"))
        .expect("fresh lifecycle binding is required to return to the primary");
    let reacquired_shadow = reacquired
        .observer_shadow("ViewerC", "bird_follow")
        .expect("ViewerC receives the fresh primary relation shadow");
    assert_eq!(
        reacquired.event_kind(),
        Sys5RelationDispatchEventKind::FreshReacquire
    );
    assert_eq!(
        reacquired.single_endpoint_chain().edge_kind(),
        CommunicationEdgeKind::RelationProjectionPublication
    );
    assert_eq!(
        reacquired_shadow.selected_anchor(),
        "participant_a_shoulder"
    );
    assert_eq!(reacquired_shadow.selected_floor(), "live-primary");
    assert_ne!(
        reacquired_shadow.lineage_ref(),
        initial_shadow.lineage_ref()
    );
    assert_ne!(
        reacquired_shadow.semantic_epoch(),
        initial_shadow.semantic_epoch()
    );

    let observer = reacquired.observer_safe_report();
    assert!(observer.contains("source-ref:"));
    assert!(observer.contains("core-ref:"));
    assert!(observer.contains("artifact-ref:"));
    assert!(observer.contains("edge-ref:"));
    assert!(observer.contains("sys5-relation-request:"));
    assert_contains_none(
        observer,
        &[
            "raw_authority_payload",
            "raw_capability_payload",
            "raw_witness_payload",
            "capability_secret",
            "witness_secret",
            "lease_ref",
            "fresh_lease",
            "fresh_witness",
        ],
    );
}

#[test]
fn relation_endpoint_causality_preserves_publish_dispatch_receive_observe_serve_dependencies() {
    let (program, admission, checked_program_ref) = sys4_relation_fabric_parts();
    let mut fabric = LocalFabric::bootstrap(program, admission, BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");

    let receipt = fabric
        .publish_relation_current("bird_follow")
        .expect("relation publication crosses the generated endpoint");
    let edge = receipt.edge();
    assert_eq!(
        edge.kind(),
        CommunicationEdgeKind::RelationProjectionPublication
    );
    assert_eq!(edge.operation_id(), "bird_follow");
    assert_eq!(edge.source_locus(), "ParticipantB");
    assert_eq!(edge.target_locus(), "ViewerC");
    assert_eq!(edge.source_ref().path, SYS5_LOCAL_TOY_PATH);
    assert!(edge.core_ref().is_some());
    assert!(
        !fabric.projected_artifact_identity().stable_key().is_empty(),
        "runtime fabric must retain checked program identity"
    );
    assert!(
        checked_program_ref.starts_with("sys5-checked-program-sha256-v1:"),
        "SYS-5 exposes only the observer-safe checked-program reference"
    );

    let transport = receipt.transport();
    let graph = fabric.causality();
    assert_eq!(
        graph.predecessor_ids(receipt.request_enqueue_occurrence_id()),
        vec![receipt.owner_publish_occurrence_id().to_string()],
        "generated request enqueue must depend on the M8 owner publication occurrence"
    );
    let dispatch_predecessors =
        graph.predecessor_ids(transport.source_outbox_dequeue_occurrence_id());
    assert_eq!(
        dispatch_predecessors,
        vec![receipt.request_enqueue_occurrence_id().to_string()],
        "dispatch must depend on the source outbox enqueue occurrence"
    );
    assert_eq!(
        graph.predecessor_ids(transport.target_inbox_enqueue_occurrence_id()),
        vec![transport.source_outbox_dequeue_occurrence_id().to_string()],
        "receive must depend on the generated endpoint dispatch"
    );
    let observe_predecessors = graph.predecessor_ids(receipt.consumer_observe_occurrence_id());
    assert_eq!(observe_predecessors.len(), 1);
    assert!(
        observe_predecessors[0].starts_with("sys4-locus-dequeue-"),
        "consumer observation must be after target locus dequeue"
    );
    assert_eq!(
        graph.predecessor_ids(&observe_predecessors[0]),
        vec![transport.target_inbox_enqueue_occurrence_id().to_string()],
        "target locus dequeue must depend on endpoint receive"
    );
    assert_eq!(
        graph.predecessor_ids(receipt.consumer_serve_occurrence_id()),
        vec![receipt.consumer_observe_occurrence_id().to_string()],
        "relation serve must depend on the imported consumer shadow"
    );

    let shadow_digest = receipt.shadow().semantic_digest();
    assert_eq!(
        fabric.relation_semantic_digest("bird_follow"),
        Some(shadow_digest.as_str()),
        "fabric devtools digest must be derived from the imported relation shadow"
    );
}

#[test]
fn relation_cut_restore_preserves_shadow_digest_and_used_fresh_binding() {
    let (program, admission, _checked_program_ref) = sys4_relation_fabric_parts();
    let mut fabric = LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");

    fabric
        .publish_relation_current("bird_follow")
        .expect("initial relation shadow imports before semantic invalidation");
    fabric
        .invalidate_relation_primary("bird_follow")
        .expect("relation invalidation publishes fallback");
    let reacquired = fabric
        .fresh_reacquire_relation_primary("bird_follow")
        .expect("fresh relation binding reacquires the primary once");
    let before_shadow = fabric
        .relation_imported_shadow("bird_follow", "ViewerC")
        .expect("relation shadow lookup succeeds before cut")
        .expect("ViewerC has imported relation shadow before cut");
    let before_digest = fabric
        .relation_semantic_digest("bird_follow")
        .expect("relation digest exists before cut")
        .to_string();
    let before_shadow_digest = before_shadow.semantic_digest();
    let before_lineage = before_shadow.semantic().lineage().to_vec();
    let before_epoch = before_shadow.semantic().binding_epoch().to_string();
    let before_observe = before_shadow
        .consumer_observe_occurrence_id()
        .expect("live stored relation shadow keeps the qualified observe occurrence")
        .to_string();
    assert_eq!(
        before_observe,
        reacquired.consumer_observe_occurrence_id(),
        "stored imported shadow must retain the same qualified observe occurrence as the endpoint receipt"
    );
    assert!(
        before_observe.starts_with("sys4-m8:ViewerC:"),
        "stored imported shadow must use the fabric-qualified ViewerC M8 occurrence namespace"
    );
    let before_endpoint_count = fabric.total_endpoint_carrier_count();
    let last_request = sys5_relation_request_suffix(reacquired.request_id());

    let cut = fabric
        .save_local_cut("sys5-relation-cut-after-fresh-reacquire")
        .expect("ST whole-fabric cut captures relation endpoint and M8 shadows");
    let mut restored = LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut)
        .expect("relation cut restores through the same source-first program/admission");
    assert_eq!(
        restored.relation_semantic_digest("bird_follow"),
        Some(before_digest.as_str()),
        "cut/restore must retain the relation devtools digest"
    );
    let restored_shadow = restored
        .relation_imported_shadow("bird_follow", "ViewerC")
        .expect("relation shadow lookup succeeds after restore")
        .expect("ViewerC imported shadow survives restore");
    assert_eq!(restored_shadow.semantic_digest(), before_shadow_digest);
    assert_eq!(
        restored_shadow.consumer_observe_occurrence_id(),
        Some(before_observe.as_str()),
        "cut/restore must preserve the qualified observe occurrence stored with the imported shadow"
    );
    assert_eq!(
        restored_shadow.semantic().lineage(),
        before_lineage.as_slice()
    );
    assert_eq!(restored_shadow.semantic().binding_epoch(), before_epoch);
    assert_eq!(
        restored_shadow.semantic().selected_anchor(),
        "participant_a_shoulder"
    );

    assert_sys4_diag(
        restored.fresh_reacquire_relation_primary("bird_follow"),
        Sys4DiagnosticKind::M8ExecutionRejected,
    );
    assert_eq!(
        restored.relation_semantic_digest("bird_follow"),
        Some(before_digest.as_str()),
        "restored used-fresh binding must fail without mutating relation state"
    );
    assert_eq!(
        restored.total_endpoint_carrier_count(),
        before_endpoint_count,
        "restored used-fresh binding must fail before endpoint publication"
    );

    let next = restored
        .publish_relation_current("bird_follow")
        .expect("restored relation fabric continues after preserving used binding");
    assert_eq!(
        sys5_relation_request_suffix(next.request_id()),
        last_request + 1,
        "restore must retain the relation request counter above the accepted fresh reacquire"
    );
}

#[test]
fn relation_cut_restore_rejects_tampered_nonempty_digest_against_m8_shadow() {
    let (program, admission, _checked_program_ref) = sys4_relation_fabric_parts();
    let mut fabric = LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");
    let receipt = fabric
        .publish_relation_current("bird_follow")
        .expect("relation publication imports a ViewerC shadow before cut tamper");
    let shadow_digest = receipt.shadow().semantic_digest();
    assert_eq!(
        fabric.relation_semantic_digest("bird_follow"),
        Some(shadow_digest.as_str())
    );

    let mut cut = fabric
        .save_local_cut("sys5-relation-cut-tampered-digest")
        .expect("ST cut captures relation digest and M8 imported shadow");
    cut.for_test_set_relation_semantic_digest(
        "bird_follow",
        "sys5-relation-sha256-v1:tampered-nonempty-digest",
    );

    assert_sys4_diag(
        LocalFabric::restore_local_cut(program, admission, BackendProfile::St, &cut),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn relation_route_failure_discards_undelivered_carrier_and_retry_reuses_publication_occurrence() {
    let (program, admission, _checked_program_ref) = sys4_relation_fabric_parts();
    let edge_ref = relation_publication_edge_ref();
    let mut fabric = LocalFabric::bootstrap(program, admission, BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");

    fabric
        .dispatch_external_action(ExternalAction::fault_event(
            FaultInjection::route_unavailable_for_edge(edge_ref.clone()),
        ))
        .expect("external route fault is registered against the source-derived relation edge");
    let before_failure_endpoint_count = fabric.total_endpoint_carrier_count();
    let failed = assert_sys4_diag(
        fabric.publish_relation_current("bird_follow"),
        Sys4DiagnosticKind::RouteUnavailable,
    );
    assert_eq!(
        failed.relation_publication_failure_disposition(),
        Some(RelationPublicationFailureDisposition::DiscardedUndelivered),
        "route failure must explicitly discard the unaccepted relation carrier"
    );
    assert_eq!(
        fabric.for_test_pending_relation_publication_count("ParticipantB"),
        0,
        "failed route must not strand a duplicate relation carrier in the source outbox"
    );
    assert_eq!(
        fabric.total_endpoint_carrier_count(),
        before_failure_endpoint_count,
        "failed route must not create endpoint send/receive history"
    );
    assert_eq!(fabric.relation_semantic_digest("bird_follow"), None);
    assert!(
        fabric
            .relation_imported_shadow("bird_follow", "ViewerC")
            .expect("shadow lookup after failed route remains typed")
            .is_none(),
        "failed route must not install a consumer shadow"
    );

    fabric.for_test_clear_route_fault(&edge_ref);
    let retry = fabric
        .publish_relation_current("bird_follow")
        .expect("cleared route lets the relation publication retry succeed");
    assert_eq!(
        retry.shadow().publication_occurrence(),
        0,
        "failed undelivered route must not commit a relation publication sequence gap"
    );
    assert_eq!(
        fabric.for_test_pending_relation_publication_count("ParticipantB"),
        0,
        "successful retry must not leave a duplicate source relation carrier"
    );
    assert_eq!(
        fabric.endpoint_carrier_count_for_relation("bird_follow"),
        1,
        "successful retry records exactly one generated relation publication carrier"
    );
}

#[test]
fn relation_identifier_exhaustion_rejects_before_m8_mailbox_or_state_mutation() {
    assert_relation_identifier_exhaustion_is_preflight(u64::MAX, 0, "request counter exhaustion");
    assert_relation_identifier_exhaustion_is_preflight(
        0,
        u64::MAX - 8,
        "endpoint/mailbox occurrence exhaustion",
    );
}

#[test]
fn missing_relation_publish_authority_rejects_before_m8_or_endpoint_mutation() {
    let (program, admission, _checked_program_ref) = sys4_relation_fabric_parts();
    let mut publish_missing =
        LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St)
            .expect("complete source-derived admission boots SYS-4 relation fabric");
    publish_missing.for_test_remove_relation_publish_authority("bird_follow");
    assert_relation_publish_rejects_without_m8_endpoint_or_shadow_mutation(
        &mut publish_missing,
        |fabric| fabric.publish_relation_current("bird_follow"),
        Sys4DiagnosticKind::M8ExecutionRejected,
        "publish with missing publish authority",
    );

    let mut invalidate_missing =
        LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St)
            .expect("complete source-derived admission boots SYS-4 relation fabric");
    invalidate_missing.for_test_remove_relation_publish_authority("bird_follow");
    assert_relation_publish_rejects_without_m8_endpoint_or_shadow_mutation(
        &mut invalidate_missing,
        |fabric| fabric.invalidate_relation_primary("bird_follow"),
        Sys4DiagnosticKind::M8ExecutionRejected,
        "invalidate with missing publish authority",
    );

    let mut reacquire_missing = LocalFabric::bootstrap(program, admission, BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");
    reacquire_missing
        .invalidate_relation_primary("bird_follow")
        .expect("relation first reaches fallback so fresh reacquire is admissible");
    let fallback_digest = reacquire_missing
        .relation_semantic_digest("bird_follow")
        .expect("fallback publication installed before removing publish authority")
        .to_string();
    reacquire_missing.for_test_remove_relation_publish_authority("bird_follow");
    assert_relation_publish_rejects_without_m8_endpoint_or_shadow_mutation(
        &mut reacquire_missing,
        |fabric| fabric.fresh_reacquire_relation_primary("bird_follow"),
        Sys4DiagnosticKind::M8ExecutionRejected,
        "fresh reacquire with missing publish authority",
    );
    assert_eq!(
        reacquire_missing.relation_semantic_digest("bird_follow"),
        Some(fallback_digest.as_str()),
        "missing publish authority on fresh reacquire must preserve the prior fallback digest"
    );
}

#[test]
fn relation_cut_restore_rejects_corrupt_endpoint_inventory_and_request_floor() {
    let (program, admission, _checked_program_ref) = sys4_relation_fabric_parts();
    let mut fabric = LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");
    let receipt = fabric
        .publish_relation_current("bird_follow")
        .expect("relation publication creates matched endpoint records before cut tamper");
    let transport = receipt.transport();

    let clean_cut = fabric
        .save_local_cut("sys5-relation-cut-corrupt-endpoint-source")
        .expect("ST cut captures matched relation source and target endpoint records");

    let mut missing_source = clean_cut.clone();
    missing_source.for_test_drop_outgoing_endpoint_record(
        "ParticipantB",
        receipt.request_id(),
        transport.carrier_id(),
    );
    assert_sys4_diag(
        LocalFabric::restore_local_cut(
            program.clone(),
            admission.clone(),
            BackendProfile::St,
            &missing_source,
        ),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );

    let mut missing_target = clean_cut.clone();
    missing_target.for_test_drop_incoming_endpoint_record(
        "ViewerC",
        receipt.request_id(),
        transport.carrier_id(),
        transport.target_inbox_enqueue_record_id(),
    );
    assert_sys4_diag(
        LocalFabric::restore_local_cut(
            program.clone(),
            admission.clone(),
            BackendProfile::St,
            &missing_target,
        ),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );

    let mut rolled_back_request_floor = clean_cut.clone();
    rolled_back_request_floor.for_test_set_next_request_below_retained_max(receipt.request_id());
    assert_sys4_diag(
        LocalFabric::restore_local_cut(
            program,
            admission,
            BackendProfile::St,
            &rolled_back_request_floor,
        ),
        Sys4DiagnosticKind::ProgramProjectionMismatch,
    );
}

#[test]
fn unknown_relation_action_fails_before_endpoint_or_semantic_mutation() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 relation source checks and projects");
    let prepared = project
        .prepare_finite_admission(valid_admission_request())
        .expect("canonical source-derived admission seals M9 inventory");
    let mut runtime = prepared
        .start_relation_dispatch_runtime()
        .expect("prepared admission starts the SYS-5 relation dispatch runtime");
    let before = runtime.observer_safe_relation_state();
    let before_endpoint_count = runtime.total_endpoint_carrier_count();

    let err = runtime
        .dispatch_relation(Sys5RelationAction::publish_current("unknown_relation"))
        .expect_err("unknown relation ID must fail before M8 relation execution");
    assert_eq!(
        err.kind(),
        Sys5RelationDispatchDiagnosticKind::UnknownSourceRelation
    );
    assert!(err.rejected_before_generated_endpoint());
    assert!(err.rejected_before_m9_authority_use());
    assert!(err.rejected_before_m8_relation_transition());
    assert!(err.partial_relation_receipt().is_none());
    assert_eq!(
        runtime.observer_safe_relation_state(),
        before,
        "unknown relation must not mutate semantic state"
    );
    assert_eq!(
        runtime.total_endpoint_carrier_count(),
        before_endpoint_count,
        "unknown relation must not enqueue endpoint carriers"
    );

    let next = runtime
        .dispatch_relation(Sys5RelationAction::publish_current("bird_follow"))
        .expect("valid relation publication recovers after unknown relation rejection");
    assert_eq!(
        sys5_relation_request_suffix(next.single_endpoint_chain().request_occurrence_id()),
        0,
        "unknown relation rejection must not burn the first generated relation request id"
    );
}

#[test]
fn relation_action_surface_cannot_carry_route_authority_state_or_expected_result() {
    let source = runtime_source("sys5_local_slice.rs");
    let relation_action_surface = relation_action_surface_source(&source);
    let compact = relation_action_surface
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    assert_contains_all(
        &compact,
        &[
            "struct Sys5RelationAction",
            "relation",
            "Sys5RelationDispatchEventKind",
            "PublishCurrent",
            "InvalidatePrimary",
            "ViewerPresentationGap",
            "FreshReacquire",
        ],
    );
    assert_contains_none(
        &compact,
        &[
            "target_locus",
            "target_override",
            "edge_ref",
            "core_ref",
            "checked_core",
            "authority",
            "authority_grant",
            "state_delta",
            "expected_result",
            "epoch",
            "lease",
            "lease_ref",
            "fresh_lease",
            "witness",
            "witness_ref",
            "capability",
            "capability_ref",
        ],
    );
}

fn relation_action_surface_source(source: &str) -> &str {
    let start = source
        .find("struct Sys5RelationAction")
        .expect("SYS-5 relation action surface is defined");
    let end = source[start..]
        .find("struct Sys5RelationDispatchRuntime")
        .map(|offset| start + offset)
        .expect("Sys5RelationDispatchRuntime marks the next top-level relation type");
    &source[start..end]
}

fn runtime_source(relative: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(relative);
    std::fs::read_to_string(path).expect("runtime source is readable")
}

fn sys4_relation_fabric_parts() -> (FabricProgram, SealedFabricAdmission, String) {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 relation source checks and projects");
    let checked_program_ref = project.checked_program_identity_ref().to_string();
    let prepared = project
        .prepare_finite_admission(valid_admission_request())
        .expect("canonical source-derived admission seals M9 inventory");
    let (program, admission) = prepared.into_parts_for_sys4();
    (program, admission, checked_program_ref)
}

fn relation_publication_edge_ref() -> String {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_LOCAL_TOY_PATH,
        SYS5_LOCAL_TOY_SOURCE,
    ))
    .expect("canonical SYS-5 relation source checks and projects");
    project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| {
            edge.operation_id == "bird_follow"
                && edge.kind == "relation-projection-publication"
                && edge.from_locus == "ParticipantB"
                && edge.to_locus == "ViewerC"
        })
        .map(|edge| edge.edge_ref.clone())
        .expect("SYS-5 projection exposes the relation publication edge ref")
}

fn assert_relation_identifier_exhaustion_is_preflight(
    next_request: u64,
    next_endpoint_occurrence: u64,
    label: &str,
) {
    let (program, admission, _checked_program_ref) = sys4_relation_fabric_parts();
    let mut fabric = LocalFabric::bootstrap(program, admission, BackendProfile::St)
        .expect("complete source-derived admission boots SYS-4 relation fabric");
    fabric.for_test_set_relation_identifier_counters(next_request, next_endpoint_occurrence);
    assert_relation_publish_rejects_without_m8_endpoint_or_shadow_mutation(
        &mut fabric,
        |fabric| fabric.publish_relation_current("bird_follow"),
        Sys4DiagnosticKind::IdentifierExhausted,
        label,
    );
}

fn assert_relation_publish_rejects_without_m8_endpoint_or_shadow_mutation<T: Debug>(
    fabric: &mut LocalFabric,
    operation: impl FnOnce(&mut LocalFabric) -> Result<T, Sys4DispatchDiagnostics>,
    expected_kind: Sys4DiagnosticKind,
    label: &str,
) {
    let before_m8 = fabric
        .m8_actual_trace()
        .expect("M8 observer is available before relation rejection")
        .stable_digest();
    let before_digest = fabric
        .relation_semantic_digest("bird_follow")
        .map(str::to_string);
    let before_shadow = fabric
        .relation_imported_shadow("bird_follow", "ViewerC")
        .expect("relation shadow lookup is typed before relation rejection");
    let before_endpoint_count = fabric.total_endpoint_carrier_count();
    let before_relation_endpoint_count = fabric.endpoint_carrier_count_for_relation("bird_follow");
    let before_pending = fabric.for_test_pending_relation_publication_count("ParticipantB");

    let diagnostics = operation(fabric).expect_err("relation operation must fail closed");
    assert_eq!(
        diagnostics.primary().kind(),
        expected_kind,
        "{label}: unexpected diagnostic"
    );
    assert_eq!(
        diagnostics.relation_publication_failure_disposition(),
        None,
        "{label}: preflight/authority rejection must occur before a generated relation carrier is attempted"
    );
    assert_eq!(
        fabric
            .m8_actual_trace()
            .expect("M8 observer is available after relation rejection")
            .stable_digest(),
        before_m8,
        "{label}: relation rejection must not mutate M8 trace/state"
    );
    assert_eq!(
        fabric
            .relation_semantic_digest("bird_follow")
            .map(str::to_string),
        before_digest,
        "{label}: relation rejection must not change the relation digest index"
    );
    assert_eq!(
        fabric
            .relation_imported_shadow("bird_follow", "ViewerC")
            .expect("relation shadow lookup is typed after relation rejection"),
        before_shadow,
        "{label}: relation rejection must not replace the imported shadow"
    );
    assert_eq!(
        fabric.total_endpoint_carrier_count(),
        before_endpoint_count,
        "{label}: relation rejection must not create endpoint carrier history"
    );
    assert_eq!(
        fabric.endpoint_carrier_count_for_relation("bird_follow"),
        before_relation_endpoint_count,
        "{label}: relation rejection must not create a relation publication endpoint"
    );
    assert_eq!(
        fabric.for_test_pending_relation_publication_count("ParticipantB"),
        before_pending,
        "{label}: relation rejection must not strand a pending relation carrier"
    );
}

fn assert_sys4_diag<T: Debug>(
    result: Result<T, Sys4DispatchDiagnostics>,
    kind: Sys4DiagnosticKind,
) -> Sys4DispatchDiagnostics {
    let diagnostics = result.expect_err("operation must fail with SYS-4 diagnostics");
    assert_eq!(diagnostics.primary().kind(), kind);
    diagnostics
}

fn sys5_relation_request_suffix(identifier: &str) -> u64 {
    identifier
        .strip_prefix("sys5-relation-request:")
        .and_then(|suffix| suffix.parse::<u64>().ok())
        .unwrap_or_else(|| panic!("relation request id has expected SYS-5 prefix: {identifier}"))
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
