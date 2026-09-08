use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker, M9ContractCandidate,
        M9FiniteContract, M9FiniteEffectKind, M9FiniteRefinementChecker,
        M9FiniteRefinementErrorKind, M9ReadOnlyProviderEffectContract,
        M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_classification::SourceToCoreKind,
    surface_v0_pipeline::{ResidualObligationKind, check_and_elaborate_surface_v0},
};

const PROVIDER_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const LOCAL_TOY_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";

fn checked_mixed_provider_source() -> mir_semantics::surface_v0_pipeline::CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_SOURCE_PATH,
        include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir"),
    ))
    .expect("the ordinary mixed provider fixture reaches checked M7")
}

fn checked_mixed_provider_source_with_resource(
    resource: &str,
) -> mir_semantics::surface_v0_pipeline::CheckedSurfaceV0 {
    let source =
        include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir")
            .replacen("resource sample_input", &format!("resource {resource}"), 1);
    check_and_elaborate_surface_v0(FixtureSource::new(PROVIDER_SOURCE_PATH, source))
        .expect("a distinct declared logical resource remains a checked mixed source")
}

fn exact_membership_candidate_contract(
    checked: &mir_semantics::surface_v0_pipeline::CheckedSurfaceV0,
) -> M9FiniteContract {
    M9ContractCandidate::from_checked_surface(checked)
        .membership_auth_strengthening()
        .candidate_contract()
        .clone()
}

fn composite_candidate_with_contract(
    checked: &mir_semantics::surface_v0_pipeline::CheckedSurfaceV0,
    contract: M9FiniteContract,
) -> M9CompositeContractCandidate {
    M9CompositeContractCandidate::from_checked_surface(checked)
        .expect("the complete checked mixed source derives an opaque composite candidate")
        .membership_auth_strengthening()
        .with_candidate_contract(contract)
}

fn copied_contract_without(
    contract: &M9FiniteContract,
    removed_capability: Option<&str>,
    removed_failure: Option<&str>,
    removed_effect: Option<M9FiniteEffectKind>,
    removed_observation: Option<&str>,
    changed_observation: Option<(&str, &str)>,
) -> M9FiniteContract {
    let mut copied = M9FiniteContract::new();
    for precondition in contract.preconditions() {
        copied = copied.with_precondition(precondition);
    }
    for capability in contract.capability_requirements() {
        if Some(capability) != removed_capability {
            copied = copied.with_capability_requirement(capability);
        }
    }
    for failure in contract.failures() {
        if Some(failure) != removed_failure {
            copied = copied.with_failure(failure);
        }
    }
    for effect in contract.effects() {
        if Some(effect) != removed_effect {
            copied = copied.with_effect(effect);
        }
    }
    for (label, redaction) in contract.observations() {
        if Some(label) == removed_observation {
            continue;
        }
        let retained_redaction = changed_observation
            .filter(|(changed_label, _)| *changed_label == label)
            .map(|(_, changed_redaction)| changed_redaction)
            .unwrap_or(redaction);
        copied = copied.with_observation(label, retained_redaction);
    }
    copied
}

#[test]
fn mixed_provider_source_has_static_coverage_and_an_inactive_composite_finite_verdict() {
    let checked = checked_mixed_provider_source();
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(&checked)
        .expect("the full checked source has exact static provider coverage");
    let verdict = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &checked,
            M9CompositeContractCandidate::from_checked_surface(&checked)
                .expect("the complete checked mixed source derives an opaque composite candidate")
                .membership_auth_strengthening()
                .with_candidate_contract(exact_membership_candidate_contract(&checked)),
        )
        .expect("the composite verifier covers the genuine mixed source");

    assert_eq!(coverage.program_identity(), checked.program_identity());
    assert!(coverage.matches_checked(&checked));
    assert!(
        checked
            .program_identity()
            .contains_read_only_provider_effect_evaluation(),
        "the identity query must retain the genuine selected provider profile"
    );

    let legacy_checked = check_and_elaborate_surface_v0(FixtureSource::new(
        LOCAL_TOY_SOURCE_PATH,
        include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir"),
    ))
    .expect("the existing I2 local-toy control remains checked");
    assert!(
        !legacy_checked
            .program_identity()
            .contains_read_only_provider_effect_evaluation(),
        "a genuine provider-free I2 program has no selected provider profile"
    );

    let prefix_like_source =
        include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir").replacen(
            "when attack(target: Player)",
            "when ReadOnlyProviderEffect(target: Player)",
            1,
        );
    let prefix_like_checked = check_and_elaborate_surface_v0(FixtureSource::new(
        LOCAL_TOY_SOURCE_PATH,
        prefix_like_source,
    ))
    .expect("a non-provider operation name may equal the provider kind spelling");
    assert!(
        !prefix_like_checked
            .program_identity()
            .contains_read_only_provider_effect_evaluation(),
        "the query must inspect the retained header/Core discriminants, not an operation name"
    );

    assert_eq!(verdict.provider_coverage(), &coverage);
    assert_eq!(verdict.program_identity(), checked.program_identity());
    assert!(verdict.activation_pending());
    assert!(!verdict.grants_authority());
    assert!(!verdict.permits_effect_use());
    assert!(!verdict.discharges_runtime_requirement());
}

#[test]
fn mixed_provider_static_coverage_retains_full_ordered_associations_and_routed_requirement() {
    let checked = checked_mixed_provider_source();
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(&checked)
        .expect("the genuine mixed source derives complete static coverage");
    let expected_contract =
        M9ReadOnlyProviderEffectContract::try_from_checked(&checked, "sample_value")
            .expect("the source-derived provider contract is present");

    assert_eq!(coverage.program_identity(), checked.program_identity());
    assert_eq!(coverage.contract(), &expected_contract);
    assert!(coverage.matches_checked(&checked));
    assert!(!coverage.grants_authority());
    assert!(!coverage.permits_effect_use());
    assert!(!coverage.discharges_runtime_requirement());

    let expected_executable = checked
        .source_map()
        .entries()
        .iter()
        .filter(|entry| entry.kind() != SourceToCoreKind::DeferredPolicy)
        .collect::<Vec<_>>();
    assert_eq!(
        coverage.executable_source_map_associations().len(),
        expected_executable.len(),
        "coverage must retain every executable association, not just provider counts"
    );
    for (actual, expected) in coverage
        .executable_source_map_associations()
        .iter()
        .zip(expected_executable)
    {
        assert_eq!(actual.kind(), expected.kind());
        assert_eq!(actual.core_ref(), expected.core_ref());
        assert_eq!(actual.source_ref(), expected.source_ref());
    }

    let provider_rows = coverage.provider_source_map_associations();
    assert_eq!(provider_rows.len(), 4);
    assert_eq!(
        provider_rows
            .iter()
            .map(|row| row.kind())
            .collect::<Vec<_>>(),
        vec![
            SourceToCoreKind::ReadOnlyProviderEffectRequest,
            SourceToCoreKind::ReadOnlyProviderEffectInvocation,
            SourceToCoreKind::ReadOnlyProviderEffectResult,
            SourceToCoreKind::ReadOnlyProviderEffectResultConsume,
        ]
    );
    assert_eq!(
        provider_rows
            .iter()
            .map(|row| row.core_ref())
            .collect::<Vec<_>>(),
        vec![
            "sample_value:provider-effect-request:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
            "sample_value:provider-effect-invocation:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
            "sample_value:provider-effect-result:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
            "sample_value:provider-effect-result-consume:requester=ParticipantA:executor=WorldAuthority:consumer=ParticipantA",
        ]
    );
    assert!(
        provider_rows
            .iter()
            .all(|row| row.source_ref() == coverage.contract().source_ref())
    );

    let requirement = coverage.runtime_requirement();
    assert_eq!(
        requirement.kind(),
        ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported
    );
    assert_eq!(requirement.operation(), "sample_value");
    assert_eq!(requirement.source_ref(), coverage.contract().source_ref());
    assert!(requirement.activation_pending());
    assert!(!requirement.discharges_runtime_requirement());
}

#[test]
fn generic_finite_checker_remains_a_typed_rejection_for_the_mixed_provider_source() {
    let checked = checked_mixed_provider_source();
    let diagnostics = M9FiniteRefinementChecker::default()
        .discharge_candidate(
            &checked,
            M9ContractCandidate::from_checked_surface(&checked).membership_auth_strengthening(),
        )
        .expect_err("the legacy generic checker must remain closed for provider-bearing input");

    assert_eq!(
        diagnostics.primary().kind(),
        M9FiniteRefinementErrorKind::ReadOnlyProviderEffectRequiresDedicatedRuntime
    );
    assert!(!diagnostics.discharges_obligation());
}

#[test]
fn composite_checker_preserves_baseline_finite_refinement_rejections() {
    let checked = checked_mixed_provider_source();
    let source_contract = M9ContractCandidate::from_checked_surface(&checked)
        .source_contract()
        .clone();
    let exact_candidate = exact_membership_candidate_contract(&checked);
    let baseline_capability = source_contract
        .capability_requirements()
        .into_iter()
        .next()
        .expect("the genuine source retains a baseline capability");
    let baseline_failure = source_contract
        .failures()
        .into_iter()
        .next()
        .expect("the genuine source retains a baseline failure");
    let (baseline_observation, _) = source_contract
        .observations()
        .into_iter()
        .find(|(label, _)| !label.starts_with("provider-effect:"))
        .expect("the mixed fixture retains a non-provider observation baseline");

    let cases = vec![
        (
            composite_candidate_with_contract(
                &checked,
                copied_contract_without(
                    &exact_candidate,
                    Some(baseline_capability),
                    None,
                    None,
                    None,
                    None,
                ),
            ),
            M9FiniteRefinementErrorKind::RemovedBaselineCapability,
            "removing a source-derived capability",
        ),
        (
            composite_candidate_with_contract(
                &checked,
                copied_contract_without(
                    &exact_candidate,
                    None,
                    Some(baseline_failure),
                    None,
                    None,
                    None,
                ),
            ),
            M9FiniteRefinementErrorKind::RemovedBaselineFailure,
            "removing a source-derived failure",
        ),
        (
            composite_candidate_with_contract(
                &checked,
                copied_contract_without(
                    &exact_candidate,
                    None,
                    None,
                    None,
                    None,
                    Some((baseline_observation, "public")),
                ),
            ),
            M9FiniteRefinementErrorKind::ObservationPolicyWeakening,
            "weakening a legacy source observation",
        ),
        (
            composite_candidate_with_contract(
                &checked,
                copied_contract_without(&exact_candidate, None, None, None, None, None)
                    .with_effect(M9FiniteEffectKind::ExternalUndeclared),
            ),
            M9FiniteRefinementErrorKind::EffectExpansion,
            "expanding the source effect row",
        ),
    ];

    for (candidate, expected, falsifier) in cases {
        let diagnostics = M9CompositeFiniteRefinementChecker::default()
            .discharge_composite_candidate(&checked, candidate)
            .expect_err(falsifier);
        assert_eq!(diagnostics.primary().kind(), expected, "{falsifier}");
        assert!(!diagnostics.discharges_obligation(), "{falsifier}");
    }
}

#[test]
fn composite_checker_rejects_each_provider_effect_omission_and_label_row_tampering() {
    let checked = checked_mixed_provider_source();
    let exact_candidate = exact_membership_candidate_contract(&checked);
    let provider_label = "provider-effect:sample_value";

    assert!(
        exact_candidate
            .observations()
            .contains(&(provider_label, "observer_safe")),
        "the candidate must retain the exact selected source label before falsification"
    );

    for omitted_effect in [
        M9FiniteEffectKind::ReadOnlyProviderEffectRequest,
        M9FiniteEffectKind::ReadOnlyProviderEffectInvocation,
        M9FiniteEffectKind::ReadOnlyProviderEffectResult,
        M9FiniteEffectKind::ReadOnlyProviderEffectResultConsume,
    ] {
        let diagnostics = M9CompositeFiniteRefinementChecker::default()
            .discharge_composite_candidate(
                &checked,
                composite_candidate_with_contract(
                    &checked,
                    copied_contract_without(
                        &exact_candidate,
                        None,
                        None,
                        Some(omitted_effect),
                        None,
                        None,
                    ),
                ),
            )
            .expect_err("the composite cannot erase one provider effect from a retained source");
        assert_eq!(
            diagnostics.primary().kind(),
            M9FiniteRefinementErrorKind::ReadOnlyProviderEffectProfileMismatch
        );
        assert!(!diagnostics.discharges_obligation());
    }

    for candidate_contract in [
        copied_contract_without(
            &exact_candidate,
            None,
            None,
            None,
            Some(provider_label),
            None,
        ),
        copied_contract_without(
            &exact_candidate,
            None,
            None,
            None,
            None,
            Some((provider_label, "private")),
        ),
    ] {
        let diagnostics = M9CompositeFiniteRefinementChecker::default()
            .discharge_composite_candidate(
                &checked,
                composite_candidate_with_contract(&checked, candidate_contract),
            )
            .expect_err("the exact source-declared provider label is part of the profile");
        assert_eq!(
            diagnostics.primary().kind(),
            M9FiniteRefinementErrorKind::ReadOnlyProviderEffectProfileMismatch
        );
        assert!(!diagnostics.discharges_obligation());
    }
}

#[test]
fn composite_checker_rejects_a_candidate_derived_from_a_different_checked_resource_source() {
    let source_a = checked_mixed_provider_source();
    let source_b = checked_mixed_provider_source_with_resource("other_input");
    assert_ne!(source_a.program_identity(), source_b.program_identity());

    let diagnostics = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &source_b,
            M9CompositeContractCandidate::from_checked_surface(&source_a)
                .expect("source A derives an opaque composite candidate")
                .membership_auth_strengthening()
                .with_candidate_contract(exact_membership_candidate_contract(&source_a)),
        )
        .expect_err(
            "a composite candidate derived from source A cannot verify source B with another resource",
        );
    assert_eq!(
        diagnostics.primary().kind(),
        M9FiniteRefinementErrorKind::ProgramIdentityMismatch
    );
    assert!(!diagnostics.discharges_obligation());
}
