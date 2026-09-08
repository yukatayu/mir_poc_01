//! Source-real Stage 3 activation contract for the private provider component.
//!
//! The fixture, checked source, static projection, M9 composite discharge,
//! trusted bootstrap, resource binding, and separate policy are all genuine.
//! This test intentionally supplies neither a semantic result nor an authority
//! substitute: a successful launch preparation is the prerequisite for the
//! existing probe supervisor to transfer its one-shot A/B controls.

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker,
        M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0},
};
use serde_json::Value;

use crate::{
    checked_program_reference::checked_program_identity_ref,
    i3_read_only_provider_composite::{
        I3InactiveReadOnlyProviderComposite, I3ProviderCompositeErrorKind,
        I3ReadOnlyProviderCompositeCandidate, I3TrustedReadOnlyProviderFixtureSetup,
    },
    m9_auth_verification::{
        M9AdmissionErrorKind, M9I3PrivateProviderCarrierBinding,
        M9I3PrivateReadOnlyProviderRoleSnapshot, M9I3ReadOnlyProviderChildRole,
        M9I3ReadOnlyProviderRevalidationFailure, M9I3ReadOnlyProviderRevalidationUse,
    },
    sys3_projection::{
        DeclaredLogicalTopology, project_read_only_provider_effect_static,
        verify_read_only_provider_effect_static_projection,
    },
    sys5_i3_process_runtime::{
        Sys5I3DeploymentSlot, Sys5I3PrivateProcessCodec, Sys5I3PrivateProcessCodecErrorKind,
    },
};

use crate::sys5_i3_process_runtime::{Sys5I3ProcessRuntimeError, Sys5I3ProcessRuntimeErrorKind};

#[cfg(feature = "i3-process-test-seams")]
use crate::sys5_i3_process_runtime::{
    Sys5I3ProviderRevalidationTestPoint, Sys5I3ProviderRevalidationTestRetirement,
};

const PROVIDER_EFFECT_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const PROVIDER_EFFECT_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");

fn checked_provider_effect_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect("the genuine provider source must check before Stage 3 activation")
}

fn provider_slots() -> [Sys5I3DeploymentSlot; 2] {
    [
        Sys5I3DeploymentSlot::new("provider-a", "127.0.0.1:41501", ["ParticipantA", "ViewerC"]),
        Sys5I3DeploymentSlot::new(
            "provider-b",
            "127.0.0.1:41502",
            ["WorldAuthority", "ParticipantB"],
        ),
    ]
}

fn sealed_provider_composite_with_trusted_setup(
    trusted_setup: I3TrustedReadOnlyProviderFixtureSetup,
) -> (
    I3TrustedReadOnlyProviderFixtureSetup,
    I3InactiveReadOnlyProviderComposite,
) {
    let checked = checked_provider_effect_source();
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("the checked provider source retains all four declared loci");
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(&checked)
        .expect("the checked provider source derives exact provider static coverage");
    let static_plan = project_read_only_provider_effect_static(&checked, &topology, coverage)
        .expect("the dedicated static projector accepts the genuine provider source");
    verify_read_only_provider_effect_static_projection(&checked, &topology, &static_plan)
        .expect("the static projection retains exact checked provider coverage");

    let composite_discharge = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &checked,
            M9CompositeContractCandidate::from_checked_surface(&checked)
                .expect("the checked provider source derives its composite candidate")
                .membership_auth_strengthening(),
        )
        .expect("the genuine provider source passes the finite composite verifier");
    let bootstrap = trusted_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("T0 derives bootstrap facts from the checked provider static plan");
    let binding = trusted_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect("the declared provider slot binds to its actual resource incarnation");
    let verified = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .expect("the genuine checked source and trusted context form one composite candidate")
    .verify_membership_and_discharge()
    .expect("actual M9 membership and composite discharge verify before policy sealing");
    let policy = trusted_setup
        .decide_fixed_policy(&verified)
        .expect("the separate fixed policy accepts the verified provider composite");
    let terminal_observation_policy = trusted_setup
        .decide_fixed_terminal_observation_policy(&verified)
        .expect("the separate terminal-observation policy accepts the verified provider composite");
    let inactive = verified
        .seal_with_policies(Some(policy), Some(terminal_observation_policy))
        .expect("the distinct effect and terminal-observation policies seal one scoped inactive provider component");

    (trusted_setup, inactive)
}

fn sealed_provider_composite() -> (
    I3TrustedReadOnlyProviderFixtureSetup,
    I3InactiveReadOnlyProviderComposite,
) {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted setup provisions the genuine declared provider resource context");
    sealed_provider_composite_with_trusted_setup(trusted_setup)
}

fn sealed_provider_composite_without_declared_target() -> (
    I3TrustedReadOnlyProviderFixtureSetup,
    I3InactiveReadOnlyProviderComposite,
) {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_without_declared_target()
        .expect(
            "the genuine declared-absent fixture still provisions its trusted resource context",
        );
    sealed_provider_composite_with_trusted_setup(trusted_setup)
}

fn assert_provider_local_error_kind<T>(
    result: Result<T, Sys5I3ProcessRuntimeError>,
    expected: Sys5I3ProcessRuntimeErrorKind,
    case: &str,
) {
    let error = match result {
        Ok(_) => panic!("{case}: the local provider operation must reject"),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        expected,
        "{case}: the local provider boundary preserves its precise typed rejection"
    );
}

fn genuine_role_snapshot(
    inactive: &I3InactiveReadOnlyProviderComposite,
    role: M9I3ReadOnlyProviderChildRole,
) -> M9I3PrivateReadOnlyProviderRoleSnapshot {
    inactive
        .test_only_i3_private_provider_role_snapshot(role)
        .expect("the sealed source-real composite retains the requested actual M9 role facts")
}

fn replace_snapshot_string(snapshot: &mut Value, pointer: &str, replacement: String, case: &str) {
    let field = snapshot
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("{case}: canonical role snapshot retains {pointer}"));
    assert!(
        field.is_string(),
        "{case}: canonical role snapshot field {pointer} remains a string"
    );
    *field = Value::String(replacement);
}

fn replace_snapshot_unsigned(snapshot: &mut Value, pointer: &str, replacement: u64, case: &str) {
    let field = snapshot
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("{case}: canonical role snapshot retains {pointer}"));
    assert!(
        field.is_u64(),
        "{case}: canonical role snapshot field {pointer} remains an unsigned integer"
    );
    *field = Value::from(replacement);
}

fn frame_untrusted_provider_body(body: Vec<u8>) -> Vec<u8> {
    let mut framed = (body.len() as u32).to_be_bytes().to_vec();
    framed.extend(body);
    framed
}

fn frame_untrusted_provider_terminal_observer_input(body: Vec<u8>) -> Vec<u8> {
    frame_untrusted_provider_body(body)
}

fn assert_role_snapshot_install_rejected(
    case: &str,
    snapshot_role: M9I3ReadOnlyProviderChildRole,
    expected_role: M9I3ReadOnlyProviderChildRole,
    mutate: impl FnOnce(&mut Value),
) {
    let (_setup, inactive) = sealed_provider_composite();
    let mut serialized = serde_json::to_value(genuine_role_snapshot(&inactive, snapshot_role))
        .expect(
            "the trusted-control-only role snapshot serializes for a scoped canonical mutation",
        );
    mutate(&mut serialized);
    let candidate: M9I3PrivateReadOnlyProviderRoleSnapshot = serde_json::from_value(serialized)
        .unwrap_or_else(|_| {
            panic!("{case}: the scoped mutation remains a structurally decodable candidate")
        });
    let error = inactive
        .test_only_install_i3_private_provider_role_snapshot(candidate, expected_role)
        .expect_err("a mismatched actual M9 role fact must not install local authority");
    assert_eq!(
        error.kind(),
        I3ProviderCompositeErrorKind::ResourceBindingMismatch,
        "{case}: the composite rejects the actual M9 fact mismatch at its trusted-control boundary"
    );
}

fn assert_v2_terminal_policy_snapshot_rejected(case: &str, mutate: impl FnOnce(&mut Value)) {
    let (_setup, inactive) = sealed_provider_composite();
    let mut serialized = serde_json::to_value(genuine_role_snapshot(
        &inactive,
        M9I3ReadOnlyProviderChildRole::Executor,
    ))
    .expect("the genuine v2 executor role snapshot serializes for one scoped policy mutation");
    mutate(&mut serialized);
    let candidate =
        match serde_json::from_value::<M9I3PrivateReadOnlyProviderRoleSnapshot>(serialized) {
            Ok(candidate) => candidate,
            Err(_) => return,
        };
    let error = match inactive.test_only_install_i3_private_provider_role_snapshot(
        candidate,
        M9I3ReadOnlyProviderChildRole::Executor,
    ) {
        Ok(_) => panic!("{case}: a mutated v2 terminal policy must not install local authority"),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        I3ProviderCompositeErrorKind::ResourceBindingMismatch,
        "{case}: a structurally decodable v2 policy mutation rejects at the actual install boundary"
    );
}

#[test]
fn provider_source_real_composite_launch_is_ready_for_two_process_supervision() {
    let (trusted_setup, inactive) = sealed_provider_composite();
    let _prepared_launch = inactive
        .prepare_provider_localnet_launch(trusted_setup, provider_slots())
        .expect("the genuine composite must prepare the opaque A/B supervised launch");
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn provider_local_before_reserve_retirement_rejects_without_executor_state_or_rearm() {
    let (trusted_setup, inactive) = sealed_provider_composite();
    let cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine source-real composite builds its inactive A/B cohort");
    let mut pair = cohort
        .test_only_source_real_provider_local_execution_pair(&trusted_setup)
        .expect("the live trusted setup derives the focused local provider pair");

    pair.executor_mut()
        .configure_revalidation_test_retirement(
            Sys5I3ProviderRevalidationTestPoint::BeforeReserve,
            Sys5I3ProviderRevalidationTestRetirement::Membership,
        )
        .expect("one genuine membership retirement may be selected before B reserves");
    let request = pair
        .requester_mut()
        .begin_read_only_provider_request()
        .expect("A retains one genuine pending provider request before B admission");
    assert_provider_local_error_kind(
        pair.executor_mut()
            .admit_read_only_provider_request_and_execute(request),
        Sys5I3ProcessRuntimeErrorKind::StaleMembership,
        "membership retirement before reservation",
    );

    let executor = pair.executor_mut().test_only_ledger_facts();
    assert_eq!(
        executor.capacity(),
        64,
        "B retains the fixed local capacity"
    );
    assert_eq!(
        executor.retained_entry_count(),
        0,
        "B retains no failed admission"
    );
    assert_eq!(
        executor.executor_reserved_count(),
        0,
        "the failure precedes B reservation"
    );
    assert_eq!(
        executor.executor_call_started_count(),
        0,
        "the failure precedes the B-local host call"
    );
    assert_eq!(
        executor.executor_outcome_retained_count(),
        0,
        "the failure retains no B outcome"
    );
    let requester = pair.requester_mut().test_only_ledger_facts();
    assert_eq!(
        requester.requester_pending_count(),
        1,
        "A keeps its original pending request after B rejects before reservation"
    );
    assert_eq!(
        requester.requester_consumed_count(),
        0,
        "A cannot consume after the pre-reservation rejection"
    );
    assert!(
        pair.requester_mut().has_no_owner_execution_occurrences()
            && pair.executor_mut().has_no_owner_execution_occurrences(),
        "a provider-local rejection never creates an owner execution occurrence"
    );

    assert_provider_local_error_kind(
        pair.executor_mut().configure_revalidation_test_retirement(
            Sys5I3ProviderRevalidationTestPoint::BeforeReserve,
            Sys5I3ProviderRevalidationTestRetirement::EffectCapability,
        ),
        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
        "a fired one-shot revalidation retirement cannot be armed again",
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn provider_local_after_reserve_retirements_retain_only_rejected_before_call() {
    for (case, retirement, expected_error) in [
        (
            "capability retirement",
            Sys5I3ProviderRevalidationTestRetirement::EffectCapability,
            Sys5I3ProcessRuntimeErrorKind::MissingCapability,
        ),
        (
            "witness retirement",
            Sys5I3ProviderRevalidationTestRetirement::EffectWitness,
            Sys5I3ProcessRuntimeErrorKind::MissingWitness,
        ),
    ] {
        let (trusted_setup, inactive) = sealed_provider_composite();
        let cohort = inactive
            .into_inactive_process_cohort(provider_slots())
            .expect("the genuine source-real composite builds its inactive A/B cohort");
        let mut pair = cohort
            .test_only_source_real_provider_local_execution_pair(&trusted_setup)
            .expect("the live trusted setup derives the focused local provider pair");

        pair.executor_mut()
            .configure_revalidation_test_retirement(
                Sys5I3ProviderRevalidationTestPoint::AfterReserveBeforeCallStarted,
                retirement,
            )
            .unwrap_or_else(|_| {
                panic!("{case}: one actual M9 retirement is selected after B reservation")
            });
        let request = pair
            .requester_mut()
            .begin_read_only_provider_request()
            .unwrap_or_else(|_| panic!("{case}: A retains one genuine pending provider request"));
        assert_provider_local_error_kind(
            pair.executor_mut()
                .admit_read_only_provider_request_and_execute(request),
            expected_error,
            case,
        );

        let executor = pair.executor_mut().test_only_ledger_facts();
        assert_eq!(
            executor.retained_entry_count(),
            1,
            "{case}: B retains one terminal rejection"
        );
        assert_eq!(
            executor.executor_reserved_count(),
            0,
            "{case}: reservation becomes rejection"
        );
        assert_eq!(
            executor.executor_rejected_before_call_count(),
            1,
            "{case}: B records the exact RejectedBeforeCall state"
        );
        assert_eq!(
            executor.executor_call_started_count(),
            0,
            "{case}: B never starts the physical adapter call"
        );
        assert_eq!(
            executor.executor_outcome_retained_count(),
            0,
            "{case}: B retains no provider outcome after the pre-call rejection"
        );
        assert_eq!(
            executor.executor_released_count(),
            0,
            "{case}: no outcome is released"
        );
        let requester = pair.requester_mut().test_only_ledger_facts();
        assert_eq!(
            requester.requester_pending_count(),
            1,
            "{case}: A remains pending"
        );
        assert_eq!(
            requester.requester_consumed_count(),
            0,
            "{case}: A has no consume event"
        );
        assert!(
            pair.requester_mut().has_no_owner_execution_occurrences()
                && pair.executor_mut().has_no_owner_execution_occurrences(),
            "{case}: the local rejected-before-call state creates no owner execution"
        );
    }
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn provider_local_after_call_retirement_retains_outcome_without_release() {
    let (trusted_setup, inactive) = sealed_provider_composite();
    let cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine source-real composite builds its inactive A/B cohort");
    let mut pair = cohort
        .test_only_source_real_provider_local_execution_pair(&trusted_setup)
        .expect("the live trusted setup derives the focused local provider pair");

    pair.executor_mut()
        .configure_revalidation_test_retirement(
            Sys5I3ProviderRevalidationTestPoint::AfterCallBeforeRelease,
            Sys5I3ProviderRevalidationTestRetirement::EffectWitness,
        )
        .expect("one actual witness retirement is selected after B's local call");
    let request = pair
        .requester_mut()
        .begin_read_only_provider_request()
        .expect("A retains one genuine pending provider request");
    assert_provider_local_error_kind(
        pair.executor_mut()
            .admit_read_only_provider_request_and_execute(request),
        Sys5I3ProcessRuntimeErrorKind::MissingWitness,
        "witness retirement after CallStarted before B release",
    );

    let executor = pair.executor_mut().test_only_ledger_facts();
    assert_eq!(
        executor.retained_entry_count(),
        1,
        "B retains one post-call outcome"
    );
    assert_eq!(
        executor.executor_reserved_count(),
        0,
        "B is no longer reserved"
    );
    assert_eq!(
        executor.executor_rejected_before_call_count(),
        0,
        "the call was started"
    );
    assert_eq!(
        executor.executor_call_started_count(),
        0,
        "CallStarted is represented by the retained post-call state, not a duplicate ledger slot"
    );
    assert_eq!(
        executor.executor_outcome_retained_count(),
        1,
        "B preserves the actual typed provider outcome after the failed release check"
    );
    assert_eq!(executor.executor_released_count(), 0, "B cannot release it");
    let requester = pair.requester_mut().test_only_ledger_facts();
    assert_eq!(requester.requester_pending_count(), 1, "A remains pending");
    assert_eq!(
        requester.requester_consumed_count(),
        0,
        "A has no consume event"
    );
    assert!(
        pair.requester_mut().has_no_owner_execution_occurrences()
            && pair.executor_mut().has_no_owner_execution_occurrences(),
        "the local post-call retention creates no owner execution"
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn provider_local_before_consume_retirement_preserves_pending_request() {
    let (trusted_setup, inactive) = sealed_provider_composite();
    let cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine source-real composite builds its inactive A/B cohort");
    let mut pair = cohort
        .test_only_source_real_provider_local_execution_pair(&trusted_setup)
        .expect("the live trusted setup derives the focused local provider pair");

    let request = pair
        .requester_mut()
        .begin_read_only_provider_request()
        .expect("A retains one genuine pending provider request");
    let result = pair
        .executor_mut()
        .admit_read_only_provider_request_and_execute(request)
        .expect("B reaches its actual local result release before A consumes");
    pair.requester_mut()
        .configure_revalidation_test_retirement(
            Sys5I3ProviderRevalidationTestPoint::BeforeConsume,
            Sys5I3ProviderRevalidationTestRetirement::Membership,
        )
        .expect("one actual membership retirement is selected before A consumption");
    assert_provider_local_error_kind(
        pair.requester_mut()
            .admit_read_only_provider_result_and_consume(result),
        Sys5I3ProcessRuntimeErrorKind::StaleMembership,
        "membership retirement before A consume",
    );

    let requester = pair.requester_mut().test_only_ledger_facts();
    assert_eq!(
        requester.retained_entry_count(),
        1,
        "A retains the original request"
    );
    assert_eq!(requester.requester_pending_count(), 1, "A remains pending");
    assert_eq!(
        requester.requester_consumed_count(),
        0,
        "A has no consume event"
    );
    let executor = pair.executor_mut().test_only_ledger_facts();
    assert_eq!(
        executor.executor_outcome_retained_count(),
        0,
        "B's released state is one slot"
    );
    assert_eq!(
        executor.executor_released_count(),
        1,
        "B retains the actual released outcome"
    );
    assert!(
        pair.requester_mut().has_no_owner_execution_occurrences()
            && pair.executor_mut().has_no_owner_execution_occurrences(),
        "the pending local provider result creates no owner execution"
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn provider_local_ledger_holds_sixty_four_completed_operations_without_refund() {
    let (trusted_setup, inactive) = sealed_provider_composite();
    let cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine source-real composite builds its inactive A/B cohort");
    let mut pair = cohort
        .test_only_source_real_provider_local_execution_pair(&trusted_setup)
        .expect("the live trusted setup derives the focused local provider pair");

    for operation in 0..64 {
        let request = pair
            .requester_mut()
            .begin_read_only_provider_request()
            .unwrap_or_else(|_| panic!("operation {operation}: A retains a distinct request"));
        let result = pair
            .executor_mut()
            .admit_read_only_provider_request_and_execute(request)
            .unwrap_or_else(|_| {
                panic!("operation {operation}: B executes the actual provider call")
            });
        let _receipt = pair
            .requester_mut()
            .admit_read_only_provider_result_and_consume(result)
            .unwrap_or_else(|_| {
                panic!("operation {operation}: A consumes the actual provider result")
            });
    }

    let requester_before = pair.requester_mut().test_only_ledger_facts();
    let executor_before = pair.executor_mut().test_only_ledger_facts();
    assert_eq!(
        requester_before.capacity(),
        64,
        "A has the fixed retained-state bound"
    );
    assert_eq!(
        executor_before.capacity(),
        64,
        "B has the fixed retained-state bound"
    );
    assert_eq!(
        requester_before.requester_consumed_count(),
        64,
        "A retains every one of its completed provider decisions"
    );
    assert_eq!(
        executor_before.executor_released_count(),
        64,
        "B retains every one of its released provider decisions"
    );
    assert_eq!(requester_before.retained_entry_count(), 64, "A is full");
    assert_eq!(executor_before.retained_entry_count(), 64, "B is full");

    assert_provider_local_error_kind(
        pair.requester_mut().begin_read_only_provider_request(),
        Sys5I3ProcessRuntimeErrorKind::ResourceExhausted,
        "the sixty-fifth local provider request",
    );
    assert_eq!(
        pair.requester_mut().test_only_ledger_facts(),
        requester_before,
        "A neither resets nor refunds retained decisions after capacity rejection"
    );
    assert_eq!(
        pair.executor_mut().test_only_ledger_facts(),
        executor_before,
        "B retains its independent completed decisions without reconnect reset"
    );
    assert!(
        pair.requester_mut().has_no_owner_execution_occurrences()
            && pair.executor_mut().has_no_owner_execution_occurrences(),
        "the bounded local provider loop creates no owner execution"
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn provider_local_duplicate_request_and_result_do_not_repeat_call_or_consume() {
    let (trusted_setup, inactive) = sealed_provider_composite();
    let cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine source-real composite builds its inactive A/B cohort");
    let mut pair = cohort
        .test_only_source_real_provider_local_execution_pair(&trusted_setup)
        .expect("the live trusted setup derives the focused local provider pair");

    let request = pair
        .requester_mut()
        .begin_read_only_provider_request()
        .expect("A emits one genuine source-derived provider request");
    let result = pair
        .executor_mut()
        .admit_read_only_provider_request_and_execute(request.clone())
        .expect("B executes the genuine request exactly once");
    assert_provider_local_error_kind(
        pair.executor_mut()
            .admit_read_only_provider_request_and_execute(request),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected,
        "a duplicate genuine request carrier",
    );
    let _receipt = pair
        .requester_mut()
        .admit_read_only_provider_result_and_consume(result.clone())
        .expect("A consumes the genuine result exactly once");
    assert_provider_local_error_kind(
        pair.requester_mut()
            .admit_read_only_provider_result_and_consume(result),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected,
        "a duplicate genuine result carrier",
    );

    let executor = pair.executor_mut().test_only_ledger_facts();
    assert_eq!(executor.retained_entry_count(), 1, "B retains one decision");
    assert_eq!(
        executor.executor_released_count(),
        1,
        "B releases one result"
    );
    assert_eq!(
        executor.executor_call_started_count(),
        0,
        "no second call state is retained"
    );
    assert_eq!(
        executor.executor_outcome_retained_count(),
        0,
        "no second outcome state is retained"
    );
    let requester = pair.requester_mut().test_only_ledger_facts();
    assert_eq!(
        requester.retained_entry_count(),
        1,
        "A retains one decision"
    );
    assert_eq!(requester.requester_consumed_count(), 1, "A consumes once");
    assert_eq!(
        requester.requester_pending_count(),
        0,
        "A has no duplicate pending state"
    );
    assert!(
        pair.requester_mut().has_no_owner_execution_occurrences()
            && pair.executor_mut().has_no_owner_execution_occurrences(),
        "duplicate local carriers never create owner execution occurrences"
    );
}

#[test]
fn provider_result_codec_preserves_injected_adapter_unavailable_candidate_without_claiming_host_failure()
 {
    // The source-derived binding and initial declared-absent result are genuine.
    // Only the tainted result candidate's typed outcome kind changes below; this
    // is codec/admission evidence, not an actual B-host AdapterUnavailable claim.
    let (trusted_setup, inactive) = sealed_provider_composite_without_declared_target();
    let cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine declared-absent source builds its inactive A/B cohort");
    let mut pair = cohort
        .test_only_source_real_provider_local_execution_pair(&trusted_setup)
        .expect("the live trusted declared-absent setup derives the focused local pair");
    let request = pair
        .requester_mut()
        .begin_read_only_provider_request()
        .expect("A retains one genuine pending request before B's declared-absent result");
    let genuine_result = pair
        .executor_mut()
        .admit_read_only_provider_request_and_execute(request)
        .expect("B produces one genuine declared-absent typed result carrier");
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let encoded = codec
        .encode_provider_result(&genuine_result)
        .expect("the genuine private provider result remains within the strict codec bound");
    let mut tainted_body: Value = serde_json::from_slice(
        encoded
            .get(4..)
            .expect("the strict provider result frame retains its fixed prefix"),
    )
    .expect("the genuine framed result body is strict JSON before one candidate mutation");
    let outcome_kind = tainted_body
        .pointer_mut("/carrier/payload/outcome/kind")
        .expect("the genuine declared-absent result has its unit outcome kind");
    assert!(
        outcome_kind.as_str() == Some("provider_resource_not_found"),
        "the source-derived unit outcome has no payload that a candidate mutation could alter"
    );
    *outcome_kind = Value::String("adapter_unavailable".to_string());
    let tainted_body = serde_json::to_vec(&tainted_body)
        .expect("the one-kind tainted candidate remains strict JSON input");
    let tainted_frame = frame_untrusted_provider_body(tainted_body);
    assert!(
        tainted_frame.len() <= codec.limits().max_message_bytes(),
        "the tainted candidate remains inside the existing strict provider-carrier frame bound"
    );
    let decoded_candidate = match codec.decode_untrusted_provider_result(&tainted_frame) {
        Ok(candidate) => candidate,
        Err(_) => panic!("the one-kind tainted source-bound result remains a decodable candidate"),
    };
    let round_tripped = codec
        .encode_provider_result(&decoded_candidate)
        .expect("the decoded tainted candidate re-encodes through the existing private codec");
    let round_tripped_body: Value = serde_json::from_slice(
        round_tripped
            .get(4..)
            .expect("the strict re-encoded result retains its fixed prefix"),
    )
    .expect("the re-encoded tainted candidate remains strict JSON");
    assert!(
        round_tripped_body
            .pointer("/carrier/payload/outcome/kind")
            .and_then(Value::as_str)
            == Some("adapter_unavailable"),
        "the codec preserves the declared AdapterUnavailable atom without exposing a raw result"
    );

    let duplicate_candidate = decoded_candidate.clone();
    let _receipt = pair
        .requester_mut()
        .admit_read_only_provider_result_and_consume(decoded_candidate)
        .expect(
            "A consumes the exact source-bound untrusted candidate once under current M9 facts",
        );
    assert_provider_local_error_kind(
        pair.requester_mut()
            .admit_read_only_provider_result_and_consume(duplicate_candidate),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected,
        "a second delivery of the injected untrusted candidate",
    );
    assert!(
        pair.requester_mut().has_no_owner_execution_occurrences()
            && pair.executor_mut().has_no_owner_execution_occurrences(),
        "codec/admission candidate evidence never creates an owner execution occurrence"
    );
}

#[test]
fn provider_role_snapshots_install_actual_m9_authority_with_role_restriction() {
    let (_setup, inactive) = sealed_provider_composite();

    let mut requester = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::RequesterConsumer),
            M9I3ReadOnlyProviderChildRole::RequesterConsumer,
        )
        .expect("A installs its genuine requester/consumer M9 facts from trusted control");
    assert!(
        requester.is_current_for_request_or_consume(),
        "A's installed membership, contract/effect capabilities, witnesses, epoch, scope, and current map must authorize only its local request/consume use"
    );
    assert!(
        !requester.is_current_for_host_use(),
        "the requester/consumer role never receives B-only host-use custody"
    );

    let mut executor = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::Executor),
            M9I3ReadOnlyProviderChildRole::Executor,
        )
        .expect("B installs its genuine executor M9 facts from trusted control");
    assert!(
        executor.is_current_for_request_or_consume(),
        "B's installed actual M9 facts remain current for the provider request boundary"
    );
    assert!(
        executor.is_current_for_host_use(),
        "only B's executor role has current local host-use custody"
    );
}

#[test]
fn provider_carrier_binding_uses_only_the_checked_program_transport_reference() {
    let checked = checked_provider_effect_source();
    let raw_stable_key = checked.program_identity().stable_key().to_string();
    let expected_transport_reference = checked_program_identity_ref(&raw_stable_key);
    let (_setup, inactive) = sealed_provider_composite();

    let role_snapshot = serde_json::to_value(genuine_role_snapshot(
        &inactive,
        M9I3ReadOnlyProviderChildRole::Executor,
    ))
    .expect("the genuine local role snapshot retains its full checked identity privately");
    assert!(
        role_snapshot
            .pointer("/program_identity_ref")
            .and_then(Value::as_str)
            == Some(raw_stable_key.as_str()),
        "the trusted local role snapshot retains the unmodified full checked stable key"
    );

    let mut executor = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::Executor),
            M9I3ReadOnlyProviderChildRole::Executor,
        )
        .expect("B installs genuine current M9 facts without minting a new authority");
    let binding = executor.provider_carrier_binding();
    let serialized_binding = serde_json::to_value(&binding)
        .expect("the actual issued carrier binding serializes only for a tainted-boundary test");
    assert!(
        serialized_binding
            .pointer("/program_identity_ref")
            .and_then(Value::as_str)
            == Some(expected_transport_reference.as_str()),
        "the carrier transports the established opaque checked-program reference"
    );
    assert!(
        serialized_binding
            .pointer("/program_identity_ref")
            .and_then(Value::as_str)
            != Some(raw_stable_key.as_str()),
        "the carrier never transports the full checked stable key"
    );
    assert!(
        executor.is_current_for_provider_host_carrier(&binding),
        "the original actual issued carrier remains current for executor host use"
    );

    for (case, replacement) in [
        ("raw stable-key substitution", raw_stable_key),
        (
            "different checked-program hash substitution",
            checked_program_identity_ref("different-checked-program-stable-key"),
        ),
    ] {
        let mut tainted = serialized_binding.clone();
        replace_snapshot_string(&mut tainted, "/program_identity_ref", replacement, case);
        let tainted: M9I3PrivateProviderCarrierBinding = serde_json::from_value(tainted)
            .unwrap_or_else(|_| panic!("{case}: the substituted carrier remains tainted input"));
        assert!(
            !executor.is_current_for_provider_host_carrier(&tainted),
            "{case}: a substituted program identity cannot gain current executor authority"
        );
    }
    assert!(
        executor.is_current_for_provider_host_carrier(&binding),
        "rejecting tainted bindings does not change the original current local authority"
    );
}

#[test]
fn provider_role_snapshot_rejects_tampered_actual_m9_lineage() {
    assert_role_snapshot_install_rejected(
        "role substitution",
        M9I3ReadOnlyProviderChildRole::RequesterConsumer,
        M9I3ReadOnlyProviderChildRole::Executor,
        |_| {},
    );
    assert_role_snapshot_install_rejected(
        "membership epoch substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            replace_snapshot_string(
                snapshot,
                "/authority/membership/epoch",
                "foreign-membership-epoch".to_string(),
                "membership epoch substitution",
            );
        },
    );
    assert_role_snapshot_install_rejected(
        "provider scope substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            replace_snapshot_string(
                snapshot,
                "/authority/effect_capability/scope/scope/logical_resource_slot",
                "foreign-resource-slot".to_string(),
                "provider scope substitution",
            );
        },
    );
    assert_role_snapshot_install_rejected(
        "effect capability membership substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            replace_snapshot_string(
                snapshot,
                "/authority/effect_capability/membership_ref",
                "foreign-membership-ref".to_string(),
                "effect capability membership substitution",
            );
        },
    );
    assert_role_snapshot_install_rejected(
        "effect witness capability substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            let contract_capability_ref = snapshot
                .pointer("/authority/contract_capability/reference")
                .and_then(Value::as_str)
                .expect("canonical role snapshot retains the contract capability reference")
                .to_string();
            replace_snapshot_string(
                snapshot,
                "/authority/effect_witness/capability_ref",
                contract_capability_ref,
                "effect witness capability substitution",
            );
        },
    );
    assert_role_snapshot_install_rejected(
        "current membership substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            replace_snapshot_string(
                snapshot,
                "/authority/current_memberships/0/membership_ref",
                "foreign-current-membership-ref".to_string(),
                "current membership substitution",
            );
        },
    );
    assert_role_snapshot_install_rejected(
        "terminal observer operation substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            replace_snapshot_string(
                snapshot,
                "/authority/terminal_observation_use/operation",
                "foreign-provider-operation".to_string(),
                "terminal observer operation substitution",
            );
        },
    );
    assert_role_snapshot_install_rejected(
        "terminal observer membership incarnation substitution",
        M9I3ReadOnlyProviderChildRole::Executor,
        M9I3ReadOnlyProviderChildRole::Executor,
        |snapshot| {
            replace_snapshot_string(
                snapshot,
                "/authority/terminal_observation_use/membership_incarnation",
                "foreign-membership-incarnation".to_string(),
                "terminal observer membership incarnation substitution",
            );
        },
    );

    let (_setup, inactive) = sealed_provider_composite();
    let mut serialized = serde_json::to_value(genuine_role_snapshot(
        &inactive,
        M9I3ReadOnlyProviderChildRole::Executor,
    ))
    .expect("the genuine executor role snapshot serializes for the removal falsifier");
    serialized
        .pointer_mut("/authority")
        .and_then(Value::as_object_mut)
        .expect("the canonical role snapshot retains its authority object")
        .remove("effect_witness");
    assert!(
        serde_json::from_value::<M9I3PrivateReadOnlyProviderRoleSnapshot>(serialized).is_err(),
        "removing the actual effect witness must make the trusted-control candidate undecodable"
    );
}

#[test]
fn provider_role_snapshot_rejects_v2_terminal_observation_policy_downgrades() {
    #[derive(Clone, Copy)]
    enum V2TerminalPolicyMutation {
        V1ProjectionProfile,
        ForeignProjectionSchema,
        ProjectionBodyOverBound,
        ProjectionTransportSlotsOverBound,
        OuterRoleSnapshotV2,
    }

    for (case, mutation) in [
        (
            "v1 terminal projection profile",
            V2TerminalPolicyMutation::V1ProjectionProfile,
        ),
        (
            "foreign terminal projection schema",
            V2TerminalPolicyMutation::ForeignProjectionSchema,
        ),
        (
            "terminal projection body over bound",
            V2TerminalPolicyMutation::ProjectionBodyOverBound,
        ),
        (
            "terminal projection transport slots over bound",
            V2TerminalPolicyMutation::ProjectionTransportSlotsOverBound,
        ),
        (
            "old outer role snapshot version",
            V2TerminalPolicyMutation::OuterRoleSnapshotV2,
        ),
    ] {
        assert_v2_terminal_policy_snapshot_rejected(case, |snapshot| match mutation {
            V2TerminalPolicyMutation::V1ProjectionProfile => replace_snapshot_string(
                snapshot,
                "/authority/terminal_observation_use/terminal_projection",
                "fixed-provider-terminal-audit-v1".to_string(),
                case,
            ),
            V2TerminalPolicyMutation::ForeignProjectionSchema => replace_snapshot_string(
                snapshot,
                "/authority/terminal_observation_use/terminal_projection_schema",
                "foreign-fixed-provider-terminal-schema".to_string(),
                case,
            ),
            V2TerminalPolicyMutation::ProjectionBodyOverBound => replace_snapshot_unsigned(
                snapshot,
                "/authority/terminal_observation_use/terminal_projection_max_body_bytes",
                65_533,
                case,
            ),
            V2TerminalPolicyMutation::ProjectionTransportSlotsOverBound => {
                replace_snapshot_unsigned(
                    snapshot,
                    "/authority/terminal_observation_use/terminal_projection_transport_occurrences_per_kind",
                    3,
                    case,
                )
            }
            V2TerminalPolicyMutation::OuterRoleSnapshotV2 => {
                replace_snapshot_unsigned(snapshot, "/version", 2, case)
            }
        });
    }
}

#[test]
fn provider_local_m9_effect_retirement_revokes_executor_and_requester_use() {
    let (_setup, inactive) = sealed_provider_composite();
    let mut executor = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::Executor),
            M9I3ReadOnlyProviderChildRole::Executor,
        )
        .expect("the genuine executor role installs before its local retirement");
    assert!(
        executor.is_current_for_host_use(),
        "B begins with actual current M9 host-use authority"
    );
    executor
        .retire_effect_authorization()
        .expect("B retires the actual local provider effect capability");
    assert!(
        !executor.is_current_for_host_use(),
        "B's actual M9 capability retirement revokes later local host use"
    );

    let mut requester = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::RequesterConsumer),
            M9I3ReadOnlyProviderChildRole::RequesterConsumer,
        )
        .expect("the genuine requester role installs before its local retirement");
    assert!(
        requester.is_current_for_request_or_consume(),
        "A begins with actual current M9 requester/consume authority"
    );
    requester
        .retire_effect_authorization()
        .expect("A retires the actual local provider effect capability");
    assert!(
        !requester.is_current_for_request_or_consume(),
        "A's actual M9 capability retirement revokes later local consumption"
    );
}

#[test]
fn provider_local_m9_retirements_preserve_typed_executor_host_failures() {
    enum LocalAuthorityRetirement {
        EffectCapability,
        EffectWitness,
        Membership,
    }

    for (case, retirement, expected_failure) in [
        (
            "effect capability retirement",
            LocalAuthorityRetirement::EffectCapability,
            M9I3ReadOnlyProviderRevalidationFailure::MissingCapability,
        ),
        (
            "effect witness retirement",
            LocalAuthorityRetirement::EffectWitness,
            M9I3ReadOnlyProviderRevalidationFailure::MissingWitness,
        ),
        (
            "membership retirement",
            LocalAuthorityRetirement::Membership,
            M9I3ReadOnlyProviderRevalidationFailure::StaleMembership,
        ),
    ] {
        let (_setup, inactive) = sealed_provider_composite();
        let mut executor = inactive
            .test_only_install_i3_private_provider_role_snapshot(
                genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::Executor),
                M9I3ReadOnlyProviderChildRole::Executor,
            )
            .expect("B installs genuine current M9 facts for one isolated retirement");
        let binding = executor.provider_carrier_binding();
        assert!(
            executor
                .revalidate_provider_carrier(&binding, M9I3ReadOnlyProviderRevalidationUse::Host)
                .is_ok(),
            "{case}: a fresh executor authority passes typed host revalidation"
        );

        match retirement {
            LocalAuthorityRetirement::EffectCapability => executor
                .retire_effect_authorization()
                .expect("the real local M9 effect capability transition retires once"),
            LocalAuthorityRetirement::EffectWitness => executor
                .retire_effect_witness_authorization()
                .expect("the real local M9 effect witness transition retires once"),
            LocalAuthorityRetirement::Membership => executor
                .retire_membership_authorization()
                .expect("the real local M9 membership transition retires once"),
        }

        let failure = match executor
            .revalidate_provider_carrier(&binding, M9I3ReadOnlyProviderRevalidationUse::Host)
        {
            Ok(()) => panic!("{case}: retired M9 facts must not keep host authority current"),
            Err(failure) => failure,
        };
        assert!(
            failure == expected_failure,
            "{case}: the actual M9 transition retains its precise typed host failure"
        );
    }
}

#[test]
fn provider_terminal_observer_preflight_is_independent_of_effect_retirement() {
    let (_setup, inactive) = sealed_provider_composite();
    let mut executor = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::Executor),
            M9I3ReadOnlyProviderChildRole::Executor,
        )
        .expect("B installs the genuine observer-bound M9 role snapshot");
    let binding = executor.provider_carrier_binding();
    let permit = executor
        .preflight_provider_terminal_observation_export(&binding)
        .expect("current observer authorization permits the fixed terminal projection");
    assert!(
        permit.permits_fixed_v2_terminal_export(),
        "the M9 permit retains the selected one-terminal fixed-v2 export allowance"
    );
    assert_eq!(
        format!("{permit:?}"),
        "M9I3PrivateProviderTerminalObservationExportPermit(..)",
        "the preflight permit Debug view remains opaque and value-free"
    );

    executor
        .retire_effect_authorization()
        .expect("B retires only its actual provider-effect authorization");
    assert!(
        !executor.is_current_for_host_use(),
        "effect retirement blocks later host use without rewriting the retained terminal fact"
    );
    assert!(
        executor
            .preflight_provider_terminal_observation_export(&binding)
            .expect("effect retirement alone does not revoke separate observer permission")
            .permits_fixed_v2_terminal_export(),
        "a retained post-call terminal fact may still be projected under current observer policy"
    );
}

#[test]
fn provider_terminal_observer_retirement_denies_export_without_revoking_effect() {
    let (_setup, inactive) = sealed_provider_composite();
    let mut executor = inactive
        .test_only_install_i3_private_provider_role_snapshot(
            genuine_role_snapshot(&inactive, M9I3ReadOnlyProviderChildRole::Executor),
            M9I3ReadOnlyProviderChildRole::Executor,
        )
        .expect("B installs the genuine observer-bound M9 role snapshot");
    let binding = executor.provider_carrier_binding();
    assert!(
        executor.is_current_for_host_use(),
        "observer retirement starts with independently current provider-effect authority"
    );
    executor
        .retire_terminal_observation_authorization()
        .expect("B retires only the separately issued terminal observer authorization");
    assert!(
        executor.is_current_for_host_use(),
        "observer retirement does not revoke the still-current provider effect capability"
    );
    let error = executor
        .preflight_provider_terminal_observation_export(&binding)
        .expect_err("a retired observer authorization must deny terminal export");
    assert_eq!(
        error.primary().kind(),
        M9AdmissionErrorKind::InvalidCapabilityLineage,
        "observer-only retirement rejects export through the typed M9 lineage boundary"
    );
}

/// A deliberately synthetic, structurally complete untrusted requester
/// candidate. It demonstrates only decoder invariants; it is not an emitted
/// provider audit, M9 permit, source-derived authority, or process evidence.
fn untrusted_terminal_observer_v2_value() -> Value {
    serde_json::json!({
        "version": 2,
        "profile_ref": "fixed-provider-terminal-audit-v2",
        "schema_ref": "fixed-provider-terminal-audit-network-provenance-v2",
        "role": "RequesterConsumer",
        "request_descriptor": {
            "source_ref": "untrusted-source-ref",
            "core_ref": "untrusted-core-ref",
            "source_artifact_ref": "untrusted-source-artifact-ref",
            "target_artifact_ref": "untrusted-target-artifact-ref",
            "generated_edge_ref": "untrusted-generated-edge-ref"
        },
        "result_descriptor": {
            "source_ref": "untrusted-source-ref",
            "core_ref": "untrusted-core-ref",
            "source_artifact_ref": "untrusted-source-artifact-ref",
            "target_artifact_ref": "untrusted-target-artifact-ref",
            "generated_edge_ref": "untrusted-generated-edge-ref"
        },
        "rows": [{
            "semantic_request_ref": "i3-provider-request-occurrence-sha256-v1:synthetic-request",
            "host_started_occurrence_ref": null,
            "outcome_retained_occurrence_ref": null,
            "release_occurrence_ref": null,
            "local_consume_ref": "i3-provider-terminal-local-occurrence-sha256-v2:synthetic-consume",
            "adapter_entry_occurrence_ref": null,
            "adapter_read_occurrence_ref": null,
            "request_send_reservation_occurrence_refs": [
                "i3-provider-network-occurrence-sha256-v1:synthetic-request-send-reservation",
                null
            ],
            "request_send_completed_occurrence_refs": [
                "i3-provider-network-occurrence-sha256-v1:synthetic-request-send-completed",
                null
            ],
            "request_receive_occurrence_refs": [null, null],
            "result_send_reservation_occurrence_refs": [null, null],
            "result_send_completed_occurrence_refs": [null, null],
            "result_receive_occurrence_refs": [
                "i3-provider-network-occurrence-sha256-v1:synthetic-result-receive",
                null
            ],
            "ordered_predecessor_refs": [
                "i3-provider-request-occurrence-sha256-v1:synthetic-request",
                "i3-provider-network-occurrence-sha256-v1:synthetic-request-send-reservation",
                "i3-provider-network-occurrence-sha256-v1:synthetic-request-send-completed",
                "i3-provider-network-occurrence-sha256-v1:synthetic-result-receive",
                "i3-provider-terminal-local-occurrence-sha256-v2:synthetic-consume"
            ],
            "outcome_class": "value_present"
        }],
        "request_count": 1,
        "reserved_count": 0,
        "rejected_before_call_count": 0,
        "call_started_count": 0,
        "physical_adapter_entry_count": 0,
        "actual_read_count": 0,
        "outcome_retained_count": 0,
        "released_count": 0,
        "consume_count": 1,
        "pending_count": 0
    })
}

fn encode_untrusted_terminal_observer_v2_body(body: Value) -> Vec<u8> {
    serde_json::to_vec(&body)
        .expect("the deliberately untrusted fixed-shape terminal body serializes")
}

fn clear_snapshot_value(snapshot: &mut Value, pointer: &str, case: &str) {
    let field = snapshot
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("{case}: synthetic terminal candidate retains {pointer}"));
    assert!(
        field.is_string(),
        "{case}: synthetic terminal candidate field {pointer} remains a populated reference"
    );
    *field = Value::Null;
}

#[test]
fn provider_terminal_observer_decoder_rejects_non_candidate_inputs() {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let canonical_body = untrusted_terminal_observer_v2_value();
    let canonical = match codec.decode_untrusted_provider_terminal_audit_observer_view(
        &frame_untrusted_provider_terminal_observer_input(
            encode_untrusted_terminal_observer_v2_body(canonical_body.clone()),
        ),
    ) {
        Ok(candidate) => candidate,
        Err(_) => panic!(
            "the synthetic complete v2 requester candidate must establish the decoder structural baseline"
        ),
    };
    assert!(
        canonical.role()
            == crate::sys5_i3_process_runtime::Sys5I3ProviderChildRole::RequesterConsumer
            && canonical.rows().len() == 1,
        "the synthetic baseline is only one structurally valid untrusted requester candidate"
    );

    let malformed = match codec.decode_untrusted_provider_terminal_audit_observer_view(
        &frame_untrusted_provider_terminal_observer_input(b"{".to_vec()),
    ) {
        Ok(_) => panic!("a malformed terminal-observer body must not decode a candidate"),
        Err(error) => error,
    };
    assert_eq!(
        malformed.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed,
        "malformed input remains outside every candidate and authority boundary"
    );

    let mut v1_body = canonical_body.clone();
    replace_snapshot_unsigned(&mut v1_body, "/version", 1, "v1 terminal-observer version");
    let unknown_version = match codec.decode_untrusted_provider_terminal_audit_observer_view(
        &frame_untrusted_provider_terminal_observer_input(
            encode_untrusted_terminal_observer_v2_body(v1_body),
        ),
    ) {
        Ok(_) => panic!("a v1 terminal-observer body must not decode a v2 candidate"),
        Err(error) => error,
    };
    assert_eq!(
        unknown_version.kind(),
        Sys5I3PrivateProcessCodecErrorKind::UnknownVersion,
        "v1 rejection precedes every fixed-profile or authority interpretation"
    );

    #[derive(Clone, Copy)]
    enum UntrustedTerminalMutation {
        WrongProfile,
        WrongSchema,
        MissingRequestSendCompleted,
        SwappedTransportSlot,
        CounterfeitPredecessor,
    }

    for (case, mutation) in [
        (
            "wrong fixed profile",
            UntrustedTerminalMutation::WrongProfile,
        ),
        ("wrong fixed schema", UntrustedTerminalMutation::WrongSchema),
        (
            "missing required request-send completion",
            UntrustedTerminalMutation::MissingRequestSendCompleted,
        ),
        (
            "swapped transport slot with unchanged predecessors",
            UntrustedTerminalMutation::SwappedTransportSlot,
        ),
        (
            "counterfeit ordered predecessor reference",
            UntrustedTerminalMutation::CounterfeitPredecessor,
        ),
    ] {
        let mut body = canonical_body.clone();
        match mutation {
            UntrustedTerminalMutation::WrongProfile => replace_snapshot_string(
                &mut body,
                "/profile_ref",
                "foreign-fixed-profile".to_string(),
                case,
            ),
            UntrustedTerminalMutation::WrongSchema => replace_snapshot_string(
                &mut body,
                "/schema_ref",
                "foreign-fixed-schema".to_string(),
                case,
            ),
            UntrustedTerminalMutation::MissingRequestSendCompleted => clear_snapshot_value(
                &mut body,
                "/rows/0/request_send_completed_occurrence_refs/0",
                case,
            ),
            UntrustedTerminalMutation::SwappedTransportSlot => {
                let result_receive = body
                    .pointer("/rows/0/result_receive_occurrence_refs/0")
                    .and_then(Value::as_str)
                    .expect("the synthetic baseline retains its result-receive transport slot")
                    .to_string();
                replace_snapshot_string(
                    &mut body,
                    "/rows/0/request_send_completed_occurrence_refs/0",
                    result_receive,
                    case,
                );
            }
            UntrustedTerminalMutation::CounterfeitPredecessor => replace_snapshot_string(
                &mut body,
                "/rows/0/ordered_predecessor_refs/2",
                "i3-provider-network-occurrence-sha256-v1:synthetic-counterfeit".to_string(),
                case,
            ),
        }
        let error = match codec.decode_untrusted_provider_terminal_audit_observer_view(
            &frame_untrusted_provider_terminal_observer_input(
                encode_untrusted_terminal_observer_v2_body(body),
            ),
        ) {
            Ok(_) => panic!("{case}: a malformed untrusted terminal candidate must not decode"),
            Err(error) => error,
        };
        assert_eq!(
            error.kind(),
            Sys5I3PrivateProcessCodecErrorKind::Malformed,
            "{case}: fixed v2 terminal structural validation rejects before candidate use"
        );
    }

    let oversized = match codec
        .decode_untrusted_provider_terminal_audit_observer_view(&vec![0; 65_537])
    {
        Ok(_) => panic!("an oversized terminal-observer frame must be rejected before decoding"),
        Err(error) => error,
    };
    assert_eq!(
        oversized.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Oversized,
        "oversized input cannot allocate or create an observer candidate"
    );
}

#[test]
fn normal_v2_terminal_decoder_rejects_unsupported_second_request_send_slot() {
    let case = "normal v2 second request-send slot";
    let mut body = untrusted_terminal_observer_v2_value();
    let second_reservation =
        "i3-provider-network-occurrence-sha256-v1:synthetic-request-send-reservation-second";
    let second_completion =
        "i3-provider-network-occurrence-sha256-v1:synthetic-request-send-completed-second";
    for (pointer, replacement) in [
        (
            "/rows/0/request_send_reservation_occurrence_refs/1",
            second_reservation,
        ),
        (
            "/rows/0/request_send_completed_occurrence_refs/1",
            second_completion,
        ),
    ] {
        let slot = body
            .pointer_mut(pointer)
            .unwrap_or_else(|| panic!("{case}: normal-v2 baseline retains {pointer}"));
        assert!(
            slot.is_null(),
            "{case}: normal-v2 baseline begins with exactly one populated transport slot"
        );
        *slot = Value::String(replacement.to_string());
    }
    let predecessors = body
        .pointer_mut("/rows/0/ordered_predecessor_refs")
        .and_then(Value::as_array_mut)
        .expect("the synthetic normal-v2 baseline retains its ordered predecessor array");
    assert!(
        predecessors.len() == 5,
        "the synthetic normal-v2 baseline starts with exactly one transport slot per kind"
    );
    // This is deliberately the old grouped slot order, which is structurally
    // coherent but cannot express the real late replay chronology that v2's
    // normal terminal export promises to retain.
    predecessors.insert(2, Value::String(second_reservation.to_string()));
    predecessors.insert(4, Value::String(second_completion.to_string()));

    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let error = match codec.decode_untrusted_provider_terminal_audit_observer_view(
        &frame_untrusted_provider_terminal_observer_input(
            encode_untrusted_terminal_observer_v2_body(body),
        ),
    ) {
        Ok(_) => panic!(
            "normal v2 terminal export must fail closed rather than flatten an unsupported second transport slot"
        ),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed,
        "the normal v2 decoder rejects the unsupported second slot before candidate use"
    );
}
