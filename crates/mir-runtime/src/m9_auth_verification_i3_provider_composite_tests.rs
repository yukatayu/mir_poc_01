use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker,
        M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0},
};

use crate::sys3_projection::{
    DeclaredLogicalTopology, ReadOnlyProviderEffectStaticProjection,
    project_read_only_provider_effect_static, verify_read_only_provider_effect_static_projection,
};

use super::*;

const PROVIDER_EFFECT_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const PROVIDER_EFFECT_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");
const M9_TEST_BINDING_NONCE: [u8; 32] = [0x5a; 32];

fn checked_provider_effect_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect("the genuine mixed provider source checks before direct M9 guard coverage")
}

fn canonical_provider_static_plan(
    checked: &CheckedSurfaceV0,
) -> ReadOnlyProviderEffectStaticProjection {
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("the genuine provider source has its four declared loci");
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(checked)
        .expect("the checked source derives exact provider coverage");
    let static_plan = project_read_only_provider_effect_static(checked, &topology, coverage)
        .expect("the dedicated static projection accepts the genuine provider source");
    verify_read_only_provider_effect_static_projection(checked, &topology, &static_plan)
        .expect("the generated static projection retains exact checked provider coverage");
    static_plan
}

fn verified_provider_composite() -> M9VerifiedReadOnlyProviderComposite {
    let checked = checked_provider_effect_source();
    let static_plan = canonical_provider_static_plan(&checked);
    let composite_discharge = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &checked,
            M9CompositeContractCandidate::from_checked_surface(&checked)
                .expect("the checked source derives its finite composite candidate")
                .membership_auth_strengthening(),
        )
        .expect("the real finite composite checker accepts the genuine provider profile");
    let bootstrap = trusted_read_only_provider_composite_bootstrap(
        &checked,
        &static_plan,
        M9_TEST_BINDING_NONCE,
    )
    .expect("the existing M9 trusted-bootstrap factory accepts the exact checked static plan");

    verify_read_only_provider_composite(
        &checked,
        &static_plan,
        bootstrap,
        &composite_discharge,
        M9_TEST_BINDING_NONCE,
    )
    .expect(
        "the existing M9 verifier yields a genuine provider-scoped base before legacy admission",
    )
}

#[test]
fn i3_provider_scoped_m9_base_is_rejected_by_ordinary_runtime_admission() {
    let verified = verified_provider_composite();
    let evidence = M9FinalAdmissionEvidence {
        membership_ref: verified.primary_membership.ref_id().to_owned(),
        capability_ref: verified.contract_capability.ref_id().to_owned(),
        witness_ref: verified.contract_witness.ref_id().to_owned(),
        finite_refinement: None,
    };

    let diagnostics = M9AdmissionRuntime::default()
        .admit_runtime(verified.base, verified.authority, evidence)
        .expect_err(
            "ordinary M9 runtime admission must reject a genuine provider-scoped base before generic evidence resolution",
        );

    assert_eq!(
        diagnostics.primary().kind(),
        M9AdmissionErrorKind::UnsupportedReadOnlyProviderEffectProfile,
        "ordinary M9 runtime admission must retain the precise typed provider-profile rejection",
    );
    assert!(!diagnostics.has_runtime_success());
    assert!(!diagnostics.grants_authority());
    assert!(!diagnostics.emits_verdict());
}

#[test]
fn i3_provider_scoped_m8_materializer_must_not_return_admitted_runtime() {
    let verified = verified_provider_composite();
    let diagnostics = materialize_m9_resolved_base(verified.base._embedded_m8_base).expect_err(
        "generic M8 materialization must reject a provider-scoped base rather than return an admitted bare runtime",
    );

    assert_eq!(
        diagnostics.primary().kind(),
        crate::m8_runtime_admission::M8AdmissionDiagnosticKind::UnsupportedReadOnlyProviderEffectProfile,
        "generic M8 materialization must retain the precise typed provider-profile rejection",
    );
    assert!(!diagnostics.has_runtime_success());
    assert!(!diagnostics.grants_authority());
    assert!(!diagnostics.emits_verdict());
}
