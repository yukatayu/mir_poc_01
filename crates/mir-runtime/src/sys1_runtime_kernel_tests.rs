use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{check_and_elaborate_surface_v0, CheckedSurfaceV0, EffectKind},
};

use crate::m9_auth_verification::M9RuntimeExecutionSeam;
use crate::semantic_runtime_kernel::{
    CapabilityRef, FailureKind, InputFrontier, KernelDiagnosticKind, KernelSeed, KernelStateKey,
    LocusRef, MembershipEpoch, MembershipIncarnation, OperationId, OwnerRequestCarrier,
    PrincipalRef, RemoteInputConsumeRequest, RemoteInputReleaseTuple, RemoteInputRequestCarrier,
    RemoteInputResult, RequestIdentity, SealedM9RuntimeAdmission, SemanticRuntimeKernel,
    SemanticValue, SourceCoreProvenance, VisibilityClass, WitnessRef,
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const OWNER_FIXTURE: &str = "m7_owner_only_no_residuals.mir";
const DESIGNATED_FIXTURE: &str = "designated_tick_publish_result.mir";

const PRINCIPAL: &str = "self";
const ACTOR_LOCUS: &str = "L_actor";
const OWNER_LOCUS: &str = "S";
const EVALUATOR_LOCUS: &str = "E";
const TARGET_ID: &str = "target";
const OWNER_OPERATION: &str = "attack";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:epoch1";
const OWNER_INCARNATION_REF: &str = "incarnation:self:S:epoch1";
const OWNER_CAPABILITY_REF: &str = "cap:attack:S:self:epoch1";
const OWNER_WITNESS_REF: &str = "witness:attack:S:self:epoch1";
const DESIGNATED_RESULT: &str = "result";
const DESIGNATED_INPUT_FRONTIER: &str = "F";
const DESIGNATED_INPUT_RELEASE_LABEL: &str = "input:player[self].atk:S:F";

fn surface_fixture_path(name: &str) -> String {
    format!("{SURFACE_FIXTURE_DIR}/{name}")
}

fn load_checked(name: &str) -> (String, String, CheckedSurfaceV0) {
    let relative = surface_fixture_path(name);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    let checked =
        check_and_elaborate_surface_v0(FixtureSource::new(relative.clone(), source.clone()))
            .expect("SYS-1 kernel tests start from checked M7 source");
    (relative, source, checked)
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

fn source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
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

fn owner_source_ref(path: &str, source: &str) -> SourceRef {
    source_ref(
        path,
        source,
        "player[target].hp = player[target].hp - player[self].atk",
    )
}

fn designated_source_ref(path: &str, source: &str) -> SourceRef {
    source_ref(
        path,
        source,
        "designated evaluate E on tick F publish result = player[self].atk + 1",
    )
}

fn designated_input_source_ref(path: &str, source: &str) -> SourceRef {
    source_ref(path, source, "player[self].atk")
}

fn hp_key() -> KernelStateKey {
    KernelStateKey::indexed_field("player", TARGET_ID, "hp")
}

fn atk_key() -> KernelStateKey {
    KernelStateKey::indexed_field("player", PRINCIPAL, "atk")
}

fn owner_seed() -> KernelSeed {
    KernelSeed::new()
        .with_int(hp_key(), 100)
        .with_int(atk_key(), 10)
}

fn owner_runtime_failure_seed() -> KernelSeed {
    KernelSeed::new().with_int(atk_key(), 10)
}

fn owner_provenance(checked: &CheckedSurfaceV0, source_ref: SourceRef) -> SourceCoreProvenance {
    SourceCoreProvenance::from_checked_owner_operation(checked, OWNER_OPERATION)
        .with_source_ref(source_ref)
        .with_effect(EffectKind::OwnerRequest)
        .with_effect(EffectKind::OwnerWrite)
        .with_failure(FailureKind::RouteUnavailable)
        .with_failure(FailureKind::MissingCapability)
        .with_failure(FailureKind::MissingWitness)
        .with_visibility(VisibilityClass::ObserverSafeRedacted)
}

fn sealed_owner_admission(checked: &CheckedSurfaceV0) -> SealedM9RuntimeAdmission {
    SealedM9RuntimeAdmission::test_seal_checked_owner_lineage(
        checked,
        PrincipalRef::new(PRINCIPAL),
        LocusRef::new(OWNER_LOCUS),
        MembershipEpoch::new("epoch1"),
        MembershipIncarnation::new(OWNER_INCARNATION_REF),
        CapabilityRef::new(OWNER_CAPABILITY_REF),
        WitnessRef::new(OWNER_WITNESS_REF),
    )
}

fn owner_attack_request(
    checked: &CheckedSurfaceV0,
    operation_source: SourceRef,
) -> OwnerRequestCarrier {
    OwnerRequestCarrier::new(OperationId::new(OWNER_OPERATION))
        .with_origin(PrincipalRef::new(PRINCIPAL), LocusRef::new(ACTOR_LOCUS))
        .with_target_owner(LocusRef::new(OWNER_LOCUS))
        .with_argument("target", TARGET_ID)
        .with_membership_ref(OWNER_MEMBERSHIP_REF)
        .with_membership_epoch(MembershipEpoch::new("epoch1"))
        .with_membership_incarnation(MembershipIncarnation::new(OWNER_INCARNATION_REF))
        .with_capability_ref(CapabilityRef::new(OWNER_CAPABILITY_REF))
        .with_witness_ref(WitnessRef::new(OWNER_WITNESS_REF))
        .with_provenance(owner_provenance(checked, operation_source))
}

fn owner_kernel_with_seed(
    seed: KernelSeed,
) -> (CheckedSurfaceV0, SourceRef, SemanticRuntimeKernel) {
    let (path, source, checked) = load_checked(OWNER_FIXTURE);
    let operation_ref = owner_source_ref(&path, &source);
    let admission = sealed_owner_admission(&checked);
    let kernel = SemanticRuntimeKernel::from_checked_m9(checked.clone(), admission, seed)
        .expect("sealed M9 admission should create the SYS-1 semantic runtime kernel");
    (checked, operation_ref, kernel)
}

fn owner_kernel() -> (CheckedSurfaceV0, SourceRef, SemanticRuntimeKernel) {
    owner_kernel_with_seed(owner_seed())
}

fn designated_release_tuple() -> RemoteInputReleaseTuple {
    RemoteInputReleaseTuple::new(
        PrincipalRef::new(PRINCIPAL),
        LocusRef::new(OWNER_LOCUS),
        LocusRef::new(EVALUATOR_LOCUS),
        DESIGNATED_INPUT_RELEASE_LABEL,
    )
    .with_visibility(VisibilityClass::RestrictedRedacted)
}

fn designated_seed() -> KernelSeed {
    KernelSeed::new().with_int(atk_key(), 10)
}

fn sealed_designated_remote_input_admission(
    checked: &CheckedSurfaceV0,
) -> SealedM9RuntimeAdmission {
    SealedM9RuntimeAdmission::test_seal_checked_designated_remote_input_lineage(
        checked,
        EVALUATOR_LOCUS,
        DESIGNATED_RESULT,
        0,
        PrincipalRef::new(PRINCIPAL),
        LocusRef::new(OWNER_LOCUS),
        LocusRef::new(EVALUATOR_LOCUS),
        MembershipEpoch::new("epoch1"),
        MembershipIncarnation::new(OWNER_INCARNATION_REF),
        designated_release_tuple(),
    )
}

fn designated_remote_input_request(
    checked: &CheckedSurfaceV0,
    input_source_ref: SourceRef,
) -> RemoteInputRequestCarrier {
    RemoteInputRequestCarrier::from_checked_designated_dependency(
        checked,
        EVALUATOR_LOCUS,
        DESIGNATED_RESULT,
        0,
    )
    .with_origin(PrincipalRef::new(PRINCIPAL), LocusRef::new(EVALUATOR_LOCUS))
    .with_source_owner(LocusRef::new(OWNER_LOCUS))
    .with_target_evaluator(LocusRef::new(EVALUATOR_LOCUS))
    .with_input_frontier(InputFrontier::new(DESIGNATED_INPUT_FRONTIER))
    .with_release_tuple(designated_release_tuple())
    .with_membership_ref(OWNER_MEMBERSHIP_REF)
    .with_membership_epoch(MembershipEpoch::new("epoch1"))
    .with_membership_incarnation(MembershipIncarnation::new(OWNER_INCARNATION_REF))
    .with_capability_ref(CapabilityRef::new(OWNER_CAPABILITY_REF))
    .with_witness_ref(WitnessRef::new(OWNER_WITNESS_REF))
    .with_source_ref(input_source_ref)
}

fn designated_kernel() -> (String, String, CheckedSurfaceV0, SemanticRuntimeKernel) {
    let (path, source, checked) = load_checked(DESIGNATED_FIXTURE);
    let admission = sealed_designated_remote_input_admission(&checked);
    let kernel =
        SemanticRuntimeKernel::from_checked_m9(checked.clone(), admission, designated_seed())
            .expect("sealed M9 designated remote-input admission should enter the kernel");
    (path, source, checked, kernel)
}

fn sealed_designated_remote_input_admission_from_real_m9_seam(
    checked: &CheckedSurfaceV0,
) -> SealedM9RuntimeAdmission {
    let seam = M9RuntimeExecutionSeam::test_real_admitted_designated_remote_input_seam_for_kernel(
        checked,
        EVALUATOR_LOCUS,
        DESIGNATED_RESULT,
        0,
    )
    .expect("test helper must build a real admitted M9 execution seam, not arbitrary strings");
    SealedM9RuntimeAdmission::from_m9_execution_seam(checked, &seam)
        .expect("real admitted M9 seam exposes the checked designated remote-input lineage")
}

#[test]
fn owner_request_lifecycle_retains_provenance_and_identity_not_queue_position() {
    let (checked, operation_ref, mut kernel) = owner_kernel();

    let queued = kernel
        .enqueue_owner_request(owner_attack_request(&checked, operation_ref.clone()))
        .expect("checked source plus sealed M9 authority admits owner request");
    assert_ne!(
        queued.request_identity().as_str(),
        queued.queue_position().stable_debug_id(),
        "queue position is scheduling metadata, not request identity"
    );

    let served = kernel
        .serve_next_owner(LocusRef::new(OWNER_LOCUS))
        .expect("owner worker serves the admitted request");
    let reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect("served owner transition emits exactly one typed reply");
    let receipt = kernel
        .receive_reply(reply.clone())
        .expect("reply receive installs exactly one typed receipt");

    assert_eq!(kernel.semantic_snapshot().int(&hp_key()), Some(90));
    assert_eq!(
        kernel.trace().lifecycle_for(queued.request_identity()),
        ["request", "serve", "reply", "receive_receipt",]
    );
    assert_eq!(receipt.request_identity(), queued.request_identity());
    assert_eq!(receipt.origin_principal(), &PrincipalRef::new(PRINCIPAL));
    assert_eq!(receipt.origin_locus(), &LocusRef::new(ACTOR_LOCUS));
    assert_eq!(receipt.target_owner(), &LocusRef::new(OWNER_LOCUS));
    assert_eq!(receipt.operation(), &OperationId::new(OWNER_OPERATION));
    assert_eq!(receipt.source_ref(), &operation_ref);
    assert_eq!(
        receipt.core_ref(),
        owner_provenance(&checked, operation_ref).core_ref()
    );
    assert!(receipt.effect_row().contains(EffectKind::OwnerRequest));
    assert!(receipt
        .failure_row()
        .contains(FailureKind::MissingCapability));
    assert_eq!(
        receipt.capability_refs(),
        [CapabilityRef::new(OWNER_CAPABILITY_REF)]
    );
    assert_eq!(receipt.witness_refs(), [WitnessRef::new(OWNER_WITNESS_REF)]);
    assert_eq!(receipt.membership_epoch(), &MembershipEpoch::new("epoch1"));
    assert_eq!(
        receipt.membership_incarnation(),
        &MembershipIncarnation::new(OWNER_INCARNATION_REF)
    );
    let occurrences = receipt.occurrences();
    assert!(occurrences.all_ids_are_concrete());
    assert!(occurrences.strictly_orders_request_serve_reply_receive());
    assert_eq!(
        kernel.trace().occurrences_for(queued.request_identity()),
        Some(occurrences)
    );
    assert!(receipt.redaction().is_observer_safe());
}

#[test]
fn wrong_target_source_or_identity_fails_closed_before_semantic_mutation() {
    let (checked, operation_ref, mut kernel) = owner_kernel();
    let queued = kernel
        .enqueue_owner_request(owner_attack_request(&checked, operation_ref.clone()))
        .expect("baseline request is queued");
    let before = kernel.semantic_snapshot().clone();

    let wrong_source = SourceRef::new("forged/source-free.mir", 1, 1, 1, 6);
    let malformed = [
        (
            queued
                .carrier()
                .clone()
                .with_target_owner(LocusRef::new("OtherOwner")),
            KernelDiagnosticKind::WrongTargetOwner,
        ),
        (
            queued
                .carrier()
                .clone()
                .with_provenance(owner_provenance(&checked, wrong_source)),
            KernelDiagnosticKind::SourceCoreProvenanceMismatch,
        ),
        (
            queued
                .carrier()
                .clone()
                .with_request_identity(RequestIdentity::new("req:forged:not-queued")),
            KernelDiagnosticKind::UnknownRequestIdentity,
        ),
    ];

    for (carrier, expected) in malformed {
        let diagnostics = kernel
            .serve_owner_carrier(carrier)
            .expect_err("malformed internal carrier must fail closed");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert_eq!(kernel.semantic_snapshot(), &before);
    }

    for provenance in [
        SourceCoreProvenance::from_checked_owner_operation(&checked, OWNER_OPERATION)
            .with_source_ref(operation_ref.clone())
            .with_effect(EffectKind::OwnerWrite)
            .with_failure(FailureKind::RouteUnavailable)
            .with_visibility(VisibilityClass::ObserverSafeRedacted),
        owner_provenance(&checked, operation_ref.clone())
            .with_visibility(VisibilityClass::RestrictedRedacted),
    ] {
        let (_checked, _operation_ref, mut kernel) = owner_kernel();
        let before = kernel.semantic_snapshot().clone();
        let before_receipts = kernel.receipt_store().clone();
        let diagnostics = kernel
            .enqueue_owner_request(
                owner_attack_request(&checked, operation_ref.clone()).with_provenance(provenance),
            )
            .expect_err("failure-row or visibility/redaction mismatch rejects before enqueue");
        assert_eq!(
            diagnostics.primary().kind(),
            KernelDiagnosticKind::SourceCoreProvenanceMismatch
        );
        assert_eq!(kernel.semantic_snapshot(), &before);
        assert_eq!(kernel.receipt_store(), &before_receipts);
    }
}

#[test]
fn duplicate_reply_or_receipt_is_single_assignment_without_replay_mutation() {
    let (checked, operation_ref, mut kernel) = owner_kernel();
    kernel
        .enqueue_owner_request(owner_attack_request(&checked, operation_ref))
        .expect("request queues");
    let served = kernel
        .serve_next_owner(LocusRef::new(OWNER_LOCUS))
        .expect("owner request serves once");
    let reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect("first reply succeeds");
    let receipt = kernel
        .receive_reply(reply.clone())
        .expect("first receipt succeeds");
    let after_first = kernel.semantic_snapshot().clone();
    let receipt_store_after_first = kernel.receipt_store().clone();

    let duplicate_reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect_err("reply is single-assignment for a served occurrence");
    assert_eq!(
        duplicate_reply.primary().kind(),
        KernelDiagnosticKind::DuplicateReply
    );
    assert_eq!(kernel.semantic_snapshot(), &after_first);

    let duplicate_receipt = kernel
        .receive_reply(reply)
        .expect_err("receipt is single-assignment for a request identity");
    assert_eq!(
        duplicate_receipt.primary().kind(),
        KernelDiagnosticKind::DuplicateReceipt
    );
    assert_eq!(kernel.semantic_snapshot(), &after_first);
    assert_eq!(kernel.receipt_store(), &receipt_store_after_first);
    assert!(kernel.receipt_store().contains(receipt.request_identity()));
}

#[test]
fn declared_route_unavailable_yields_typed_failure_reply_and_receipt_without_mutation() {
    let (checked, operation_ref, mut kernel) = owner_kernel_with_seed(owner_runtime_failure_seed());
    let before = kernel.semantic_snapshot().clone();
    let queued = kernel
        .enqueue_owner_request(owner_attack_request(&checked, operation_ref))
        .expect("admitted owner request queues even when runtime data is unavailable");

    let served = kernel
        .serve_next_owner(LocusRef::new(OWNER_LOCUS))
        .expect("declared RouteUnavailable is served as a typed failure occurrence");
    let reply = kernel
        .reply_to_served_request(served.serve_occurrence())
        .expect("declared runtime failure emits a typed reply");
    assert_eq!(reply.failure(), Some(FailureKind::RouteUnavailable));
    let receipt = kernel
        .receive_reply(reply)
        .expect("typed failure reply installs a typed receipt");

    assert_eq!(receipt.failure(), Some(FailureKind::RouteUnavailable));
    assert_eq!(receipt.request_identity(), queued.request_identity());
    assert_eq!(kernel.semantic_snapshot(), &before);
    assert!(kernel
        .trace()
        .contains_typed_failure_receipt(queued.request_identity(), FailureKind::RouteUnavailable));
}

#[test]
fn source_free_or_forged_authority_carrier_cannot_mutate_or_mint_authority() {
    let (checked, operation_ref, mut kernel) = owner_kernel();
    let before = kernel.semantic_snapshot().clone();
    let before_authority = kernel.authority_view().clone();

    let source_free = OwnerRequestCarrier::source_free_for_test(OperationId::new(OWNER_OPERATION))
        .with_origin(PrincipalRef::new(PRINCIPAL), LocusRef::new(ACTOR_LOCUS))
        .with_target_owner(LocusRef::new(OWNER_LOCUS))
        .with_argument("target", TARGET_ID);
    let forged_authority = owner_attack_request(&checked, operation_ref.clone())
        .with_capability_ref(CapabilityRef::new("cap:forged-by-carrier"))
        .with_witness_ref(WitnessRef::new("witness:forged-by-carrier"));
    let stale_incarnation = owner_attack_request(&checked, operation_ref)
        .with_membership_incarnation(MembershipIncarnation::new("incarnation:self:S:stale"));

    for (carrier, expected) in [
        (source_free, KernelDiagnosticKind::SourceFreeCarrierRejected),
        (
            forged_authority,
            KernelDiagnosticKind::AuthorityLineageRejected,
        ),
        (
            stale_incarnation,
            KernelDiagnosticKind::StaleMembershipIncarnation,
        ),
    ] {
        let diagnostics = kernel
            .enqueue_owner_request(carrier)
            .expect_err("carrier cannot mint Core, authority, witness, or state");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert_eq!(kernel.semantic_snapshot(), &before);
        assert_eq!(kernel.authority_view(), &before_authority);
    }
}

#[test]
fn designated_remote_input_lifecycle_is_source_derived_owner_read_then_receipt_consume() {
    let (path, source, checked, mut kernel) = designated_kernel();
    let designated_ref = designated_source_ref(&path, &source);
    let input_ref = designated_input_source_ref(&path, &source);
    let designated = checked
        .designated_result(EVALUATOR_LOCUS, DESIGNATED_RESULT)
        .expect("checked designated result exists");
    let designated_core = designated
        .designated_core()
        .expect("checked designated result retains Core");
    let dependency = designated_core
        .generated_remote_input_dependencies()
        .first()
        .expect("designated input read generates a remote-input dependency");

    assert_eq!(designated.source_ref(), &designated_ref);
    assert_eq!(dependency.designated_evaluator(), EVALUATOR_LOCUS);
    assert_eq!(dependency.source_owner_locus(), OWNER_LOCUS);
    assert_eq!(dependency.typed_state_read().namespace(), "player");
    assert_eq!(dependency.typed_state_read().index(), Some(PRINCIPAL));
    assert_eq!(dependency.typed_state_read().field(), Some("atk"));
    assert_eq!(dependency.typed_state_read().owner_locus(), OWNER_LOCUS);
    assert_eq!(dependency.typed_state_read().source_ref(), input_ref);
    assert!(designated
        .effect_row()
        .entries()
        .iter()
        .any(|entry| entry.kind() == EffectKind::DesignatedRemoteRequest));
    assert!(designated
        .effect_row()
        .entries()
        .iter()
        .any(|entry| entry.kind() == EffectKind::DesignatedReceiptUse));

    let requested = kernel
        .enqueue_remote_input_request(designated_remote_input_request(&checked, input_ref.clone()))
        .expect("checked dependency and sealed M9 lineage admit remote input request");
    let served = kernel
        .serve_next_remote_input(LocusRef::new(OWNER_LOCUS))
        .expect("source owner serves the remote input read");
    let reply = kernel
        .reply_to_remote_input(
            served.serve_occurrence(),
            RemoteInputResult::success(SemanticValue::Int(10)),
        )
        .expect("source owner emits a typed remote input reply");
    let receipt = kernel
        .receive_remote_input_reply(reply)
        .expect("remote input reply installs exactly one typed receipt");
    let consumed = kernel
        .consume_remote_input_receipt(
            RemoteInputConsumeRequest::from_checked_designated_dependency(
                &checked,
                EVALUATOR_LOCUS,
                DESIGNATED_RESULT,
                0,
            )
            .with_receipt(receipt.receipt_id())
            .with_evaluator(LocusRef::new(EVALUATOR_LOCUS)),
        )
        .expect("designated evaluator consumes the exact receipt");

    assert_eq!(
        kernel.trace().lifecycle_for(requested.request_identity()),
        ["request", "serve", "reply", "receive_receipt", "consume",]
    );
    assert_eq!(receipt.request_identity(), requested.request_identity());
    assert_eq!(receipt.source_owner(), &LocusRef::new(OWNER_LOCUS));
    assert_eq!(receipt.target_evaluator(), &LocusRef::new(EVALUATOR_LOCUS));
    assert_eq!(receipt.release_tuple(), &designated_release_tuple());
    assert_eq!(
        receipt.input_frontier(),
        &InputFrontier::new(DESIGNATED_INPUT_FRONTIER)
    );
    assert_eq!(
        receipt.visibility_class(),
        VisibilityClass::RestrictedRedacted
    );
    assert_eq!(receipt.source_ref(), &input_ref);
    assert_eq!(
        receipt.effect_row().entries(),
        [EffectKind::DesignatedRemoteRequest]
    );
    assert!(receipt
        .failure_row()
        .contains(FailureKind::RouteUnavailable));
    assert_eq!(receipt.value(), Some(&SemanticValue::Int(10)));
    assert_eq!(
        receipt.membership_incarnation(),
        &MembershipIncarnation::new(OWNER_INCARNATION_REF)
    );
    let remote_occurrences = receipt.occurrences();
    assert!(remote_occurrences.all_ids_are_concrete());
    assert!(remote_occurrences.strictly_orders_request_serve_reply_receive());
    assert_eq!(
        kernel.trace().occurrences_for(requested.request_identity()),
        Some(remote_occurrences)
    );
    assert_eq!(consumed.value(), Some(&SemanticValue::Int(10)));
    assert_eq!(kernel.semantic_snapshot().int(&atk_key()), Some(10));
    assert_eq!(kernel.semantic_snapshot().int(&hp_key()), None);

    let duplicate = kernel
        .reply_to_remote_input(
            served.serve_occurrence(),
            RemoteInputResult::panic_if_inspected_for_test(
                "duplicate remote input payload must not be inspected",
            ),
        )
        .expect_err("duplicate remote input result is rejected before payload inspection");
    assert_eq!(
        duplicate.primary().kind(),
        KernelDiagnosticKind::DuplicateReply
    );
}

#[test]
fn production_m9_execution_seam_admits_checked_designated_remote_input_without_test_seal() {
    let (path, source, checked) = load_checked(DESIGNATED_FIXTURE);
    let input_ref = designated_input_source_ref(&path, &source);
    let admission = sealed_designated_remote_input_admission_from_real_m9_seam(&checked);
    let mut kernel =
        SemanticRuntimeKernel::from_checked_m9(checked.clone(), admission, designated_seed())
            .expect("production M9 seam-derived admission enters SYS-1 kernel");

    let requested = kernel
        .enqueue_remote_input_request(designated_remote_input_request(&checked, input_ref))
        .expect("real M9 seam exposes exact checked designated remote-input lineage");
    let served = kernel
        .serve_next_remote_input(LocusRef::new(OWNER_LOCUS))
        .expect("source owner serves the M9-sealed remote input request");
    let reply = kernel
        .reply_to_remote_input(
            served.serve_occurrence(),
            RemoteInputResult::success(SemanticValue::Int(10)),
        )
        .expect("M9-sealed remote input emits a typed reply");
    let receipt = kernel
        .receive_remote_input_reply(reply)
        .expect("M9-sealed remote input installs a typed receipt");

    assert_eq!(receipt.request_identity(), requested.request_identity());
    assert_eq!(
        receipt.source_ref(),
        &designated_input_source_ref(&path, &source)
    );
    assert_eq!(receipt.release_tuple(), &designated_release_tuple());
}

#[test]
fn designated_remote_input_rejects_stale_or_mismatched_carriers_without_receipt_or_mutation() {
    let (path, source, checked, mut kernel) = designated_kernel();
    let input_ref = designated_input_source_ref(&path, &source);
    let before = kernel.semantic_snapshot().clone();
    let before_receipts = kernel.receipt_store().clone();
    let base = designated_remote_input_request(&checked, input_ref);

    let stale_incarnation = base
        .clone()
        .with_membership_incarnation(MembershipIncarnation::new("incarnation:self:S:stale"));
    let stale_frontier = base
        .clone()
        .with_input_frontier(InputFrontier::new("stale-F"));
    let wrong_release_tuple = base
        .clone()
        .with_release_tuple(RemoteInputReleaseTuple::new(
            PrincipalRef::new(PRINCIPAL),
            LocusRef::new(OWNER_LOCUS),
            LocusRef::new("WrongEvaluator"),
            DESIGNATED_INPUT_RELEASE_LABEL,
        ));
    let substituted_capability = base
        .clone()
        .with_capability_ref(CapabilityRef::new("cap:attack:S:self:substituted"));
    let substituted_witness =
        base.with_witness_ref(WitnessRef::new("witness:attack:S:self:substituted"));

    for (carrier, expected) in [
        (
            stale_incarnation,
            KernelDiagnosticKind::StaleMembershipIncarnation,
        ),
        (stale_frontier, KernelDiagnosticKind::InputFrontierMismatch),
        (
            wrong_release_tuple,
            KernelDiagnosticKind::ReleaseTupleMismatch,
        ),
        (
            substituted_capability,
            KernelDiagnosticKind::AuthorityLineageRejected,
        ),
        (
            substituted_witness,
            KernelDiagnosticKind::AuthorityLineageRejected,
        ),
    ] {
        let diagnostics = kernel
            .enqueue_remote_input_request(carrier)
            .expect_err("malformed designated remote input carrier fails closed");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert_eq!(kernel.semantic_snapshot(), &before);
        assert_eq!(kernel.receipt_store(), &before_receipts);
    }
}
