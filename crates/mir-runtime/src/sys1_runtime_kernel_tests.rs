use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{CheckedSurfaceV0, EffectKind, check_and_elaborate_surface_v0},
};

use crate::semantic_runtime_kernel::{
    CapabilityRef, EffectHandlerAdmission, EffectHandlerFailure, EffectHandlerRequest,
    EffectHandlerResult, EffectHandlerSite, FailureKind, FailureRow, KernelDiagnosticKind,
    KernelSeed, KernelStateKey, LocusRef, MembershipEpoch, OperationId, OwnerRequestCarrier,
    PrincipalRef, ProviderRef, RequestIdentity, SealedM9RuntimeAdmission, SemanticRuntimeKernel,
    SemanticValue, SourceCoreProvenance, VisibilityClass, WitnessRef,
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const OWNER_FIXTURE: &str = "m7_owner_only_no_residuals.mir";
const DESIGNATED_FIXTURE: &str = "designated_tick_publish_result.mir";

const PRINCIPAL: &str = "self";
const ACTOR_LOCUS: &str = "L_actor";
const OWNER_LOCUS: &str = "S";
const EVALUATOR_LOCUS: &str = "E";
const PROVIDER_LOCUS: &str = "P_effect";
const TARGET_ID: &str = "target";
const OWNER_OPERATION: &str = "attack";
const OWNER_MEMBERSHIP_REF: &str = "membership:self:S:epoch1";
const OWNER_CAPABILITY_REF: &str = "cap:attack:S:self:epoch1";
const OWNER_WITNESS_REF: &str = "witness:attack:S:self:epoch1";

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
        .with_capability_ref(CapabilityRef::new(OWNER_CAPABILITY_REF))
        .with_witness_ref(WitnessRef::new(OWNER_WITNESS_REF))
        .with_provenance(owner_provenance(checked, operation_source))
}

fn owner_kernel() -> (CheckedSurfaceV0, SourceRef, SemanticRuntimeKernel) {
    let (path, source, checked) = load_checked(OWNER_FIXTURE);
    let operation_ref = owner_source_ref(&path, &source);
    let admission = sealed_owner_admission(&checked);
    let kernel = SemanticRuntimeKernel::from_checked_m9(checked.clone(), admission, owner_seed())
        .expect("sealed M9 admission should create the SYS-1 semantic runtime kernel");
    (checked, operation_ref, kernel)
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
    assert!(
        receipt
            .failure_row()
            .contains(FailureKind::MissingCapability)
    );
    assert_eq!(
        receipt.capability_refs(),
        [CapabilityRef::new(OWNER_CAPABILITY_REF)]
    );
    assert_eq!(receipt.witness_refs(), [WitnessRef::new(OWNER_WITNESS_REF)]);
    assert_eq!(receipt.membership_epoch(), &MembershipEpoch::new("epoch1"));
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
fn source_free_or_forged_authority_carrier_cannot_mutate_or_mint_authority() {
    let (checked, operation_ref, mut kernel) = owner_kernel();
    let before = kernel.semantic_snapshot().clone();
    let before_authority = kernel.authority_view().clone();

    let source_free = OwnerRequestCarrier::source_free_for_test(OperationId::new(OWNER_OPERATION))
        .with_origin(PrincipalRef::new(PRINCIPAL), LocusRef::new(ACTOR_LOCUS))
        .with_target_owner(LocusRef::new(OWNER_LOCUS))
        .with_argument("target", TARGET_ID);
    let forged_authority = owner_attack_request(&checked, operation_ref)
        .with_capability_ref(CapabilityRef::new("cap:forged-by-carrier"))
        .with_witness_ref(WitnessRef::new("witness:forged-by-carrier"));

    for (carrier, expected) in [
        (source_free, KernelDiagnosticKind::SourceFreeCarrierRejected),
        (
            forged_authority,
            KernelDiagnosticKind::AuthorityLineageRejected,
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
fn designated_remote_request_effect_handler_is_typed_without_owner_mutation_authority() {
    let (path, source, checked) = load_checked(DESIGNATED_FIXTURE);
    let designated_ref = designated_source_ref(&path, &source);
    let admission = SealedM9RuntimeAdmission::test_seal_checked_designated_lineage(
        &checked,
        PrincipalRef::new(PRINCIPAL),
        LocusRef::new(EVALUATOR_LOCUS),
        MembershipEpoch::new("epoch1"),
    );
    let mut kernel =
        SemanticRuntimeKernel::from_checked_m9(checked.clone(), admission, KernelSeed::new())
            .expect("designated source plus sealed M9 admission enters kernel");
    let provider = ProviderRef::new("effect-provider:bounded-designated-input");
    let handler = EffectHandlerAdmission::new(EffectHandlerSite::provider(
        LocusRef::new(PROVIDER_LOCUS),
        provider.clone(),
    ))
    .with_effect(EffectKind::DesignatedRemoteRequest)
    .with_declared_failures(FailureRow::new([
        FailureKind::RouteUnavailable,
        FailureKind::HandlerRejected,
    ]));
    let request =
        EffectHandlerRequest::from_checked_designated_remote_request(&checked, "E", "result", 0)
            .with_origin(PrincipalRef::new(PRINCIPAL), LocusRef::new(ACTOR_LOCUS))
            .with_handler(handler.clone())
            .with_source_ref(designated_ref);

    let pending = kernel
        .request_effect(request)
        .expect("DesignatedRemoteRequest becomes a typed effect request");
    assert!(
        pending
            .effect_row()
            .contains(EffectKind::DesignatedRemoteRequest)
    );
    assert!(!pending.handler_site().is_semantic_owner());

    let result = kernel
        .complete_effect(
            pending.request_identity(),
            EffectHandlerResult::success(SemanticValue::Int(11)).from_provider(provider.clone()),
        )
        .expect("admitted provider returns typed success");
    assert_eq!(result.provider(), &provider);
    assert_eq!(result.value(), Some(&SemanticValue::Int(11)));
    assert!(result.owner_mutation().is_none());
    assert!(kernel.trace().contains_effect_lifecycle(
        pending.request_identity(),
        ["effect_request", "handler_result"]
    ));

    let rejected = kernel
        .complete_effect(
            pending.request_identity(),
            EffectHandlerResult::success(SemanticValue::Int(12))
                .from_provider(provider)
                .with_owner_mutation(hp_key(), SemanticValue::Int(0)),
        )
        .expect_err("provider result cannot carry owner mutation authority");
    assert_eq!(
        rejected.primary().kind(),
        KernelDiagnosticKind::ProviderCannotMutateOwnerState
    );

    let failed_pending = kernel
        .request_effect(
            EffectHandlerRequest::declared_for_test(EffectKind::DesignatedRemoteRequest)
                .with_handler(handler),
        )
        .expect("declared effect request is admitted before handler failure");
    let failed = kernel
        .complete_effect(
            failed_pending.request_identity(),
            EffectHandlerResult::failure(EffectHandlerFailure::declared(
                FailureKind::HandlerRejected,
            )),
        )
        .expect("declared handler failure is typed and traceable");
    assert_eq!(
        failed.failure(),
        Some(&EffectHandlerFailure::declared(
            FailureKind::HandlerRejected
        ))
    );
}
