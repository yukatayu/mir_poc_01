use std::collections::BTreeSet;

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker,
        M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_classification::SourceToCoreKind,
    surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0},
};

use crate::{
    i3_read_only_provider_composite::{
        I3InactiveReadOnlyProviderComposite, I3ProviderCompositeError,
        I3ProviderCompositeErrorKind, I3ReadOnlyProviderCompositeCandidate,
        I3TrustedReadOnlyProviderFixtureSetup, I3VerifiedReadOnlyProviderComposite,
    },
    sys3_projection::{
        DeclaredLogicalTopology, project_read_only_provider_effect_static,
        verify_read_only_provider_effect_static_projection,
    },
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
    .expect("the genuine mixed provider fixture checks before composite admission")
}

fn checked_provider_effect_source_with_operation(operation: &str) -> CheckedSurfaceV0 {
    let source = PROVIDER_EFFECT_SOURCE.replacen(
        "effect sample_value by",
        &format!("effect {operation} by"),
        1,
    );
    check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_EFFECT_SOURCE_PATH, source))
        .expect("a distinct declared provider operation remains a genuine checked mixed source")
}

fn provider_effect_topology(checked: &CheckedSurfaceV0) -> DeclaredLogicalTopology {
    DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("the genuine provider fixture has its checked four-locus topology")
}

fn canonical_static_plan(
    checked: &CheckedSurfaceV0,
) -> crate::sys3_projection::ReadOnlyProviderEffectStaticProjection {
    let topology = provider_effect_topology(checked);
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(checked)
        .expect("the full checked source derives exact provider static coverage");
    let static_plan = project_read_only_provider_effect_static(checked, &topology, coverage)
        .expect("the dedicated static projector accepts the genuine provider source");
    verify_read_only_provider_effect_static_projection(checked, &topology, &static_plan)
        .expect("the static plan retains the complete checked provider coverage");
    static_plan
}

fn genuine_composite_discharge(
    checked: &CheckedSurfaceV0,
) -> mir_semantics::m9_finite_refinement::M9CompositeFiniteRefinementDischarge {
    M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            checked,
            M9CompositeContractCandidate::from_checked_surface(checked)
                .expect("the checked provider source derives its opaque composite candidate")
                .membership_auth_strengthening(),
        )
        .expect("the exact checked provider source passes the real composite finite verifier")
}

fn verified_composite_for_checked(
    trusted_setup: &I3TrustedReadOnlyProviderFixtureSetup,
    checked: CheckedSurfaceV0,
) -> I3VerifiedReadOnlyProviderComposite {
    let static_plan = canonical_static_plan(&checked);
    let composite_discharge = genuine_composite_discharge(&checked);
    let bootstrap = trusted_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("the T0 coordinator derives bootstrap facts from the checked static plan");
    let binding = trusted_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect("trusted setup binds only the checked declared resource slot to the static plan");
    let candidate = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .expect(
        "genuine checked source, static coverage, composite discharge, trusted bootstrap, and binding form the composite candidate",
    );
    candidate
        .verify_membership_and_discharge()
        .expect("the actual trusted bootstrap authenticates the complete composite candidate")
}

fn verified_composite_with_setup(
    trusted_setup: I3TrustedReadOnlyProviderFixtureSetup,
) -> (
    I3TrustedReadOnlyProviderFixtureSetup,
    I3VerifiedReadOnlyProviderComposite,
) {
    let verified = verified_composite_for_checked(&trusted_setup, checked_provider_effect_source());
    (trusted_setup, verified)
}

fn seal_inactive_composite_with_setup(
    trusted_setup: I3TrustedReadOnlyProviderFixtureSetup,
) -> (
    I3TrustedReadOnlyProviderFixtureSetup,
    I3InactiveReadOnlyProviderComposite,
) {
    let (trusted_setup, verified) = verified_composite_with_setup(trusted_setup);
    let policy = trusted_setup
        .decide_fixed_policy(&verified)
        .expect("the separately trusted fixed policy accepts the verified provider profile");
    let inactive = verified
        .seal_with_policy(Some(policy))
        .expect("the exact trusted policy seals the verified component");
    (trusted_setup, inactive)
}

fn assert_composite_rejected(
    error: I3ProviderCompositeError,
    expected: I3ProviderCompositeErrorKind,
    case: &str,
) {
    assert_eq!(error.kind(), expected, "{case}: unexpected rejection kind");
    assert_composite_rejected_without_kind(error, case);
}

fn assert_composite_rejected_without_kind(error: I3ProviderCompositeError, case: &str) {
    let debug = format!("{error:?}");
    assert!(
        !debug.contains("mir-runtime-i3-read-only-provider-fixture-"),
        "{case}: error Debug must not disclose the trusted fixture namespace"
    );
    assert!(
        !debug.contains("0\\n"),
        "{case}: error Debug must not disclose fixture contents"
    );
    assert!(
        !error.returned_effect_authorization(),
        "{case}: rejection must not return an effect authorization"
    );
    assert!(
        !error.returned_legacy_component(),
        "{case}: rejection must not return a legacy M8 component"
    );
    assert_eq!(
        error.provider_call_count(),
        0,
        "{case}: Stage 2b rejection remains structurally inactive"
    );
}

fn assert_current_inactive_composite(inactive: &I3InactiveReadOnlyProviderComposite, case: &str) {
    assert!(
        inactive.activation_pending(),
        "{case}: activation remains pending"
    );
    assert!(
        !inactive.provider_runtime_active(),
        "{case}: Stage 2b must not activate a provider runtime"
    );
    assert_eq!(
        inactive.provider_call_count(),
        0,
        "{case}: Stage 2b establishes no provider call"
    );
    assert!(
        inactive.has_effect_authorization(),
        "{case}: the sealed result retains current M9 effect authorization"
    );
    assert!(
        inactive.has_legacy_component(),
        "{case}: the sealed result retains its actual scoped legacy M8 component"
    );
    assert!(
        inactive.binding_is_current(),
        "{case}: the sealed result retains its current admitted resource binding"
    );
}

fn is_provider_source_map_kind(kind: SourceToCoreKind) -> bool {
    matches!(
        kind,
        SourceToCoreKind::ReadOnlyProviderEffectRequest
            | SourceToCoreKind::ReadOnlyProviderEffectInvocation
            | SourceToCoreKind::ReadOnlyProviderEffectResult
            | SourceToCoreKind::ReadOnlyProviderEffectResultConsume
    )
}

#[test]
fn i3_provider_composite_rejects_policy_seal_after_actual_binding_retirement() {
    let mut trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let verified = verified_composite_for_checked(&trusted_setup, checked_provider_effect_source());
    let policy = trusted_setup
        .decide_fixed_policy(&verified)
        .expect("the live setup issues a policy decision for its verified composite");

    trusted_setup.retire();

    let error = verified
        .seal_with_policy(Some(policy))
        .expect_err("retiring the actual trusted binding before seal must reject the stale policy");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
        "policy seal after actual trusted binding retirement",
    );
}

#[test]
fn i3_provider_composite_rejects_policy_reused_for_distinct_checked_operation() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("one live trusted fixture setup provisions for both genuine checked candidates");
    let verified_a =
        verified_composite_for_checked(&trusted_setup, checked_provider_effect_source());
    let policy_a = trusted_setup
        .decide_fixed_policy(&verified_a)
        .expect("the live setup issues a policy decision for checked operation A");
    let verified_b = verified_composite_for_checked(
        &trusted_setup,
        checked_provider_effect_source_with_operation("sample_value_second"),
    );

    let error = verified_b.seal_with_policy(Some(policy_a)).expect_err(
        "a policy for checked operation A must not authorize distinct checked operation B",
    );
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::PolicyMismatch,
        "policy reuse across distinct checked provider operations",
    );
}

#[test]
fn i3_provider_composite_scoped_snapshot_requires_its_admitted_binding_context() {
    let setup_a = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("the first trusted setup provisions a genuine binding context");
    let (_setup_a, inactive_a) = seal_inactive_composite_with_setup(setup_a);
    let setup_b = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("the second trusted setup provisions a distinct genuine binding context");
    let (_setup_b, inactive_b) = seal_inactive_composite_with_setup(setup_b);
    let snapshot_for_a = inactive_a.scoped_component_snapshot();
    let snapshot_for_b = inactive_a.scoped_component_snapshot();

    assert!(
        snapshot_for_a.component_scope_retained(),
        "the dedicated snapshot retains an M8 component scope rather than an ordinary instance"
    );
    let restored_a = inactive_a
        .consume_scoped_component_snapshot(snapshot_for_a)
        .expect("the source context consumes its own dedicated scoped snapshot");
    assert!(
        restored_a.is_component_scoped(),
        "same-context consumption restores only a scoped component"
    );

    let error = inactive_b
        .consume_scoped_component_snapshot(snapshot_for_b)
        .expect_err("a scoped snapshot from setup A must not cross into setup B");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::ResourceBindingMismatch,
        "cross-context scoped component snapshot consumption",
    );
}

#[test]
fn i3_provider_composite_scope_retains_exact_non_provider_lowerings_and_current_authority() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let (_trusted_setup, inactive) = seal_inactive_composite_with_setup(trusted_setup);
    let checked = checked_provider_effect_source();
    let expected_retained_ordinals = checked
        .source_map()
        .entries()
        .iter()
        .enumerate()
        .filter_map(|(ordinal, entry)| {
            (!is_provider_source_map_kind(entry.kind())).then_some(ordinal)
        })
        .collect::<Vec<_>>();

    assert_current_inactive_composite(&inactive, "full scoped component inventory");
    let full = inactive.scoped_component_inventory();
    assert!(
        full.matches_checked_program_identity(&checked),
        "the scoped component retains the complete checked identity"
    );
    assert_eq!(
        full.provider_lowering_count(),
        4,
        "the fixed profile always accounts for its four provider source-map rows"
    );
    assert_eq!(
        full.retained_lowering_ordinals(),
        expected_retained_ordinals,
        "the scoped component excludes exactly the provider rows while preserving non-provider order"
    );
    assert!(
        full.retains_exact_non_provider_ordered_lowering_associations(&checked),
        "the full scoped component retains exact source/Core lowering associations after exclusion"
    );

    let restricted = inactive.restricted_component_inventory(&BTreeSet::from([
        "WorldAuthority".to_string(),
        "ParticipantA".to_string(),
    ]));
    assert!(
        restricted.matches_checked_program_identity(&checked),
        "locus restriction retains the original full checked identity"
    );
    assert_eq!(
        restricted.provider_lowering_count(),
        4,
        "restriction cannot erase the four-row provider profile declaration"
    );
    assert!(
        restricted.retains_exact_non_provider_ordered_lowering_associations(&checked),
        "restricted component inventory preserves exact remaining non-provider associations"
    );
}

#[test]
fn i3_provider_composite_allows_only_one_inactive_seal_per_admitted_context() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("one trusted setup provisions one admitted provider context");
    let verified_first =
        verified_composite_for_checked(&trusted_setup, checked_provider_effect_source());
    let first_policy = trusted_setup
        .decide_fixed_policy(&verified_first)
        .expect("the current setup decides fixed policy for the first genuine verification");
    let verified_second =
        verified_composite_for_checked(&trusted_setup, checked_provider_effect_source());
    let second_policy = trusted_setup
        .decide_fixed_policy(&verified_second)
        .expect("the current setup decides fixed policy for the second genuine verification");

    let first = verified_first
        .seal_with_policy(Some(first_policy))
        .expect("the first exact verified/policy pair seals an inactive component");
    assert_current_inactive_composite(&first, "first inactive seal");

    let error = verified_second
        .seal_with_policy(Some(second_policy))
        .expect_err(
            "a second seal for the same admitted context must not return another component",
        );
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::AdmissionAlreadyUsed,
        "second inactive seal in one admitted context",
    );
    assert_current_inactive_composite(
        &first,
        "first inactive seal remains current after the second-seal rejection",
    );
}

#[test]
fn i3_provider_composite_requires_a_separate_fixed_policy_after_verification() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let verified = verified_composite_for_checked(&trusted_setup, checked_provider_effect_source());

    let error = verified
        .seal_with_policy(None)
        .expect_err("verification alone must not synthesize a fixed provider policy decision");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::SeparatePolicyRequired,
        "missing separate fixed policy",
    );
}

#[test]
fn i3_provider_composite_real_m9_authorization_retirement_invalidates_snapshot_consumption() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let (_trusted_setup, mut inactive) = seal_inactive_composite_with_setup(trusted_setup);
    let snapshot = inactive.scoped_component_snapshot();

    inactive
        .retire_effect_authorization()
        .expect("the current inactive composite retires its real M9 effect authorization");

    assert!(!inactive.has_effect_authorization());
    assert!(!inactive.binding_is_current());
    assert!(!inactive.provider_runtime_active());
    assert_eq!(inactive.provider_call_count(), 0);
    let error = inactive
        .consume_scoped_component_snapshot(snapshot)
        .expect_err("a retired M9 effect authorization cannot consume a retained scoped snapshot");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::EffectAuthorizationRetired,
        "scoped snapshot consumption after M9 authorization retirement",
    );
}

#[test]
fn i3_provider_composite_setup_retirement_and_drop_invalidate_binding_context() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let (mut retained_setup, inactive) = seal_inactive_composite_with_setup(trusted_setup);
    let snapshot = inactive.scoped_component_snapshot();
    // The returned setup is the actual lifetime owner; retire it rather than
    // minting a stale flag in test code.
    retained_setup.retire();

    assert!(!inactive.has_effect_authorization());
    assert!(!inactive.binding_is_current());
    let error = inactive
        .consume_scoped_component_snapshot(snapshot)
        .expect_err("setup retirement invalidates its actual scoped binding context");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
        "scoped snapshot consumption after setup retirement",
    );

    let (dropped_inactive, dropped_snapshot) = {
        let setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
            .expect("a separate trusted setup provisions for drop invalidation");
        let (_setup_owner, inactive) = seal_inactive_composite_with_setup(setup);
        let snapshot = inactive.scoped_component_snapshot();
        (inactive, snapshot)
    };
    assert!(!dropped_inactive.has_effect_authorization());
    assert!(!dropped_inactive.binding_is_current());
    let error = dropped_inactive
        .consume_scoped_component_snapshot(dropped_snapshot)
        .expect_err("dropping the setup invalidates the retained actual scoped binding context");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::ResourceBindingNotCurrent,
        "scoped snapshot consumption after setup drop",
    );
}

#[test]
fn i3_provider_composite_rejects_ordinary_snapshot_exports_for_scoped_components() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    assert_eq!(
        format!("{trusted_setup:?}"),
        "I3TrustedReadOnlyProviderFixtureSetup(..)",
        "trusted setup Debug remains observer-safe"
    );
    let (_trusted_setup, inactive) = seal_inactive_composite_with_setup(trusted_setup);
    assert_eq!(
        format!("{inactive:?}"),
        "I3InactiveReadOnlyProviderComposite(..)",
        "inactive composite Debug remains observer-safe"
    );

    let error = inactive
        .ordinary_i3_private_snapshot()
        .expect_err("an inactive scoped component has no ordinary M8 snapshot export");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::OrdinarySnapshotRejected,
        "ordinary snapshot export from inactive scoped component",
    );

    let snapshot = inactive.scoped_component_snapshot();
    assert_eq!(
        format!("{snapshot:?}"),
        "I3ReadOnlyProviderScopedComponentSnapshot(..)",
        "dedicated scoped snapshot Debug remains observer-safe"
    );
    let restored = snapshot
        .restore()
        .expect("the dedicated snapshot restores only its scoped component");
    assert!(restored.is_component_scoped());
    let error = restored
        .ordinary_i3_private_snapshot()
        .expect_err("a restored scoped component cannot escape through an ordinary M8 snapshot");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::OrdinarySnapshotRejected,
        "ordinary snapshot export from restored scoped component",
    );

    let restricted_snapshot = inactive.restricted_component_snapshot(&BTreeSet::from([
        "WorldAuthority".to_string(),
        "ParticipantA".to_string(),
    ]));
    assert!(restricted_snapshot.component_scope_retained());
    let restricted_restored = restricted_snapshot
        .restore()
        .expect("a dedicated restricted snapshot restores only a scoped component");
    assert!(restricted_restored.is_component_scoped());
    let error = restricted_restored
        .ordinary_i3_private_snapshot()
        .expect_err("a restored restricted scoped component cannot escape through ordinary export");
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::OrdinarySnapshotRejected,
        "ordinary snapshot export from restored restricted scoped component",
    );
}

#[test]
fn i3_provider_composite_rejects_same_source_noncanonical_composite_discharge() {
    let checked = checked_provider_effect_source();
    let static_plan = canonical_static_plan(&checked);
    let exact_candidate = M9CompositeContractCandidate::from_checked_surface(&checked)
        .expect("the genuine provider source derives its opaque composite candidate")
        .membership_auth_strengthening();
    let noncanonical_contract = exact_candidate
        .candidate_contract()
        .clone()
        .with_precondition("AdditionalUnimplementedRequirement");
    let noncanonical_discharge = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &checked,
            exact_candidate.with_candidate_contract(noncanonical_contract),
        )
        .expect(
            "the finite composite checker accepts a same-source strengthening that retains MembershipAuth",
        );
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let bootstrap = trusted_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("the T0 coordinator derives bootstrap facts from the checked static plan");
    let binding = trusted_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect("trusted setup binds the genuine checked slot to its exact static plan");

    let error = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(noncanonical_discharge),
        Some(binding),
    )
    .expect_err(
        "runtime composite admission must reject an intact same-source discharge with an unsupported additional requirement",
    );
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::CompositeDischargeMismatch,
        "same-source noncanonical composite discharge",
    );
}

#[test]
fn i3_provider_composite_requires_each_trusted_candidate_input() {
    for (case, expected) in [
        (
            "trusted bootstrap facts",
            I3ProviderCompositeErrorKind::MissingTrustedBootstrapFacts,
        ),
        (
            "complete composite discharge",
            I3ProviderCompositeErrorKind::MissingCompositeDischarge,
        ),
        (
            "admitted resource binding",
            I3ProviderCompositeErrorKind::MissingResourceBinding,
        ),
    ] {
        let checked = checked_provider_effect_source();
        let static_plan = canonical_static_plan(&checked);
        let composite_discharge = genuine_composite_discharge(&checked);
        let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
            .expect("trusted default provider fixture setup provisions its dedicated namespace");
        let bootstrap = trusted_setup
            .bootstrap_facts(&checked, &static_plan)
            .expect("the T0 coordinator derives bootstrap facts from the checked static plan");
        let binding = trusted_setup
            .admit_resource_binding(&checked, &static_plan)
            .expect(
                "trusted setup binds the checked declared resource slot before candidate assembly",
            );

        let error = match case {
            "trusted bootstrap facts" => I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
                checked,
                static_plan,
                None,
                Some(composite_discharge),
                Some(binding),
            ),
            "complete composite discharge" => {
                I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
                    checked,
                    static_plan,
                    Some(bootstrap),
                    None,
                    Some(binding),
                )
            }
            "admitted resource binding" => {
                I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
                    checked,
                    static_plan,
                    Some(bootstrap),
                    Some(composite_discharge),
                    None,
                )
            }
            _ => unreachable!("the candidate-input table has only its three declared cases"),
        }
        .expect_err(
            "omitting any trusted composite input must reject before an inactive component exists",
        );
        assert_composite_rejected(error, expected, case);
    }
}

#[test]
fn i3_provider_composite_rejects_bootstrap_from_a_different_trusted_fixture_context() {
    let checked = checked_provider_effect_source();
    let static_plan = canonical_static_plan(&checked);
    let composite_discharge = genuine_composite_discharge(&checked);
    let bootstrap_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("the first genuine trusted fixture setup provisions");
    let binding_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("the second genuine trusted fixture setup provisions independently");
    let bootstrap = bootstrap_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("the first setup derives source-bound bootstrap facts for its own incarnation");
    let binding = binding_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect("the second setup admits a real binding for the same checked static plan");

    let error = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .expect_err(
        "a real bootstrap from one trusted setup cannot authorize a binding from another setup incarnation",
    );
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::TrustedBootstrapMismatch,
        "cross-fixture trusted bootstrap substitution",
    );
}

#[test]
fn i3_provider_composite_rejects_cross_checked_bootstrap_during_m9_verification() {
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("one trusted setup provisions both genuine checked operation contexts");
    let checked_a = checked_provider_effect_source();
    let static_plan_a = canonical_static_plan(&checked_a);
    let bootstrap_a = trusted_setup
        .bootstrap_facts(&checked_a, &static_plan_a)
        .expect("the actual setup derives bootstrap facts from checked operation A");

    let checked_b = checked_provider_effect_source_with_operation("sample_value_second");
    let static_plan_b = canonical_static_plan(&checked_b);
    let discharge_b = genuine_composite_discharge(&checked_b);
    let binding_b = trusted_setup
        .admit_resource_binding(&checked_b, &static_plan_b)
        .expect("the same actual setup admits a binding for checked operation B");

    let candidate = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked_b,
        static_plan_b,
        Some(bootstrap_a),
        Some(discharge_b),
        Some(binding_b),
    )
    .expect(
        "same-setup bootstrap substitution reaches M9 verification rather than becoming candidate assembly approval",
    );

    let error = candidate.verify_membership_and_discharge().expect_err(
        "bootstrap facts from checked operation A must not authenticate checked operation B",
    );
    assert_composite_rejected(
        error,
        I3ProviderCompositeErrorKind::MembershipAuthenticationFailed,
        "same-setup cross-checked bootstrap substitution during M9 verification",
    );
}

#[test]
fn i3_provider_composite_absent_declared_target_binds_without_invocation() {
    let checked = checked_provider_effect_source();
    let static_plan = canonical_static_plan(&checked);
    let composite_discharge = genuine_composite_discharge(&checked);
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_without_declared_target()
        .expect("a trusted fixture namespace may intentionally omit the declared target");
    let bootstrap = trusted_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("the T0 coordinator derives bootstrap facts from the checked static plan");
    let binding = trusted_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect(
            "binding the declared logical slot must not look up or open its absent target before CallStarted",
        );

    let candidate = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .expect(
        "a missing target affects only later invocation, not trusted binding or composite admission",
    );
    let verified = candidate
        .verify_membership_and_discharge()
        .expect("the actual trusted bootstrap still verifies the missing-target binding");
    let policy = trusted_setup
        .decide_fixed_policy(&verified)
        .expect("the separate policy evaluates the exact binding, not target availability");
    let inactive = verified.seal_with_policy(Some(policy)).expect(
        "the missing-target binding produces only an inactive composite before CallStarted",
    );

    assert_current_inactive_composite(&inactive, "missing target binding");
}

#[test]
fn i3_provider_composite_trusted_binding_yields_only_inactive_scoped_component() {
    let checked = checked_provider_effect_source();
    let static_plan = canonical_static_plan(&checked);
    let composite_discharge = genuine_composite_discharge(&checked);
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted default provider fixture setup provisions its dedicated namespace");
    let bootstrap = trusted_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("the T0 coordinator derives bootstrap facts from the checked static plan");
    let binding = trusted_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect("trusted setup binds only the checked declared resource slot to the static plan");

    let candidate = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked,
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .expect(
        "genuine checked source, static coverage, composite discharge, trusted bootstrap, and binding form the composite candidate",
    );
    let verified = candidate
        .verify_membership_and_discharge()
        .expect("the actual trusted bootstrap authenticates the complete composite candidate");
    let policy = trusted_setup
        .decide_fixed_policy(&verified)
        .expect("the separately trusted fixed policy accepts the verified provider profile");
    let inactive = verified
        .seal_with_policy(Some(policy))
        .expect("the exact trusted policy seals the verified component");

    assert_current_inactive_composite(&inactive, "genuine trusted binding");
}
