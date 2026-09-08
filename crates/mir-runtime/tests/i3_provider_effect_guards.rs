//! Stage-1 fail-closed contracts for the declared read-only provider effect.
//!
//! These tests use only ordinary checked source and public legacy entry
//! points.  They deliberately do not construct a provider request, handler,
//! authority, receipt, or synthetic checked/projected artifact.  Until a
//! dedicated provider runtime exists, every legacy path must reject the
//! retained Core before it can treat the program as an ordinary owner/pure
//! program.

use mir_ast::surface_v0::FixtureSource;
use mir_runtime::{
    m8_runtime_admission::{M8AdmissionDiagnosticKind, M8Runtime, M8RuntimeAdmission},
    m9_auth_verification::{
        M9AdmissionEnvelope, M9AdmissionErrorKind, M9AdmissionRuntime, M9ResidualBinding,
        M9SourceArtifact,
    },
    m10_reference_system::{M10CliFacadeCommand, M10ReferenceSystem},
    sys5_i3_process_runtime::{Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3ProcessCohort},
    sys5_local_slice::{Sys5LocalSliceError, Sys5SourceInput, build_project},
};
use mir_semantics::surface_v0_pipeline::{
    CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
};
use serde_json::json;

const PROVIDER_EFFECT_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const PROVIDER_EFFECT_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");
const ACCEPTED_I2_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const ACCEPTED_I2_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

fn checked_provider_effect() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect("the ordinary provider-effect source must reach legacy runtime guards after M7 checks")
}

fn checked_i2_control() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        ACCEPTED_I2_SOURCE_PATH,
        ACCEPTED_I2_SOURCE,
    ))
    .expect("the unchanged ordinary I2 control must remain source-checkable")
}

/// Reconstruct only the public M9 envelope rows M9 already owns.  The
/// provider residual deliberately has no legacy M9 binding: its absence must
/// cause the typed profile rejection, not a forged provider authorization.
fn public_m9_envelope(checked: &CheckedSurfaceV0) -> M9AdmissionEnvelope {
    let mut envelope =
        M9AdmissionEnvelope::for_checked_identity(checked.program_identity().clone())
            .with_original_source_artifact(M9SourceArtifact::from_checked_surface(checked));

    for residual in checked.residual_obligations().entries() {
        let binding = match residual.kind() {
            ResidualObligationKind::AuthDeferred => {
                M9ResidualBinding::auth_deferred(residual.name())
                    .with_source_ref(residual.source_ref().clone())
                    .with_module_contract(
                        checked.program_identity().module(),
                        format!("membership-authority/{}", residual.name()),
                    )
            }
            ResidualObligationKind::VerifyDeferred => {
                M9ResidualBinding::verify_deferred(residual.name())
                    .with_source_ref(residual.source_ref().clone())
                    .with_module_contract(
                        checked.program_identity().module(),
                        "finite-refinement/MembershipAuth",
                    )
            }
            _ => continue,
        };
        envelope = envelope.with_residual_binding(binding);
    }

    envelope
}

fn existing_typed_owner_schedule() -> serde_json::Value {
    json!({
        "schema_version": "m10-i1plus-typed-cli-schedule-v0",
        "kind": "typed_conformance_input",
        "direct_mutation_api": false,
        "requests": [
            {"event": "attack", "principal": "self", "target": "target", "attacks": 1}
        ]
    })
}

fn assert_unsupported_m8_profile(
    diagnostics: mir_runtime::m8_runtime_admission::M8AdmissionDiagnostics,
) {
    assert_eq!(
        diagnostics.primary().kind(),
        M8AdmissionDiagnosticKind::UnsupportedReadOnlyProviderEffectProfile
    );
    assert_eq!(
        diagnostics.primary().residual_kind(),
        Some(ResidualObligationKind::ReadOnlyProviderEffectRuntimeUnsupported),
        "the runtime guard must retain the M7 unsupported-provider residual rather than silently erase it"
    );
    assert!(!diagnostics.has_runtime_success());
    assert!(!diagnostics.grants_authority());
    assert!(!diagnostics.emits_verdict());
}

fn assert_unsupported_m9_profile(
    diagnostics: mir_runtime::m9_auth_verification::M9AdmissionDiagnostics,
) {
    assert_eq!(
        diagnostics.primary().kind(),
        M9AdmissionErrorKind::UnsupportedReadOnlyProviderEffectProfile
    );
    assert!(!diagnostics.has_runtime_success());
    assert!(!diagnostics.grants_authority());
    assert!(!diagnostics.emits_verdict());
    assert!(diagnostics.admitted_base().is_none());
    assert!(diagnostics.runtime_admission().is_none());
    assert!(diagnostics.m8_semantic_state().is_none());
}

#[test]
fn i3_provider_effect_m8_admission_rejects_before_legacy_residual_or_authority() {
    let checked = checked_provider_effect();
    let diagnostics = M8Runtime::default()
        .admit(
            checked.clone(),
            M8RuntimeAdmission::new(checked.program_identity().clone()),
        )
        .expect_err(
            "the retained read-only provider Core must reject before legacy M8 deferred-residual handling or lowering",
        );

    assert_unsupported_m8_profile(diagnostics);
}

#[test]
fn i3_provider_effect_public_m9_admission_rejects_before_legacy_base_or_authority() {
    let checked = checked_provider_effect();
    let runtime = M9AdmissionRuntime::default();

    let outer_diagnostics = runtime
        .admit_outer(checked.clone(), public_m9_envelope(&checked))
        .expect_err(
            "M9 outer admission must reject the retained provider Core before reduced source scopes can form a legacy base",
        );
    assert_unsupported_m9_profile(outer_diagnostics);

    let base_diagnostics = runtime
        .admit_source_bound_base(
            checked.clone(),
            M8RuntimeAdmission::new(checked.program_identity().clone()),
            public_m9_envelope(&checked),
        )
        .expect_err(
            "M9 source-bound base admission must reject before M8 evidence, authority, or a legacy base can be created",
        );
    assert_unsupported_m9_profile(base_diagnostics);
}

#[test]
fn i3_provider_effect_m10_source_run_rejects_at_existing_legacy_helper_before_execution() {
    let error = M10ReferenceSystem::deterministic_profile("i3-provider-effect-legacy-guard")
        .run_cli(
            M10CliFacadeCommand::run()
                .source_path(PROVIDER_EFFECT_SOURCE_PATH)
                .typed_schedule_json(existing_typed_owner_schedule()),
        )
        .expect_err(
            "the existing source-first M10 run must reject the provider residual before its legacy M8/M9 execution route",
        );

    assert_eq!(
        error,
        "M10 legacy finite helper rejects read-only provider effects without a dedicated runtime"
    );
}

#[test]
fn i3_provider_effect_sys5_build_rejects_before_project_or_local_execution() {
    let error = build_project(Sys5SourceInput::inline(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect_err(
        "SYS-5 must not create a project that could issue legacy grants or start local execution after provider Core projection rejects",
    );

    assert_eq!(error, Sys5LocalSliceError::ProjectionFailed);

    // `ProjectionDiagnosticKind` and direct projection construction are
    // crate-private.  A public provider source therefore stops here: there is
    // no genuine `Sys5LocalProject` to pass to finite admission or cohort
    // construction.  This test intentionally avoids fabricating either.
}

#[test]
fn i3_provider_effect_guards_preserve_the_unchanged_i2_public_admission_control() {
    let checked = checked_i2_control();
    M9AdmissionRuntime::default()
        .admit_outer(checked.clone(), public_m9_envelope(&checked))
        .expect("the unchanged I2 control remains admissible at the existing public M9 boundary");

    let project = build_project(Sys5SourceInput::inline(
        ACCEPTED_I2_SOURCE_PATH,
        ACCEPTED_I2_SOURCE,
    ))
    .expect("the unchanged I2 control remains projectable");
    project
        .prepare_canonical_local_st_admission()
        .expect("the unchanged I2 control retains the existing finite local admission path");

    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(
                "provider-effect-control-a",
                "127.0.0.1:42001",
                ["ParticipantA", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(
                "provider-effect-control-b",
                "127.0.0.1:42002",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect("the unchanged I2 control still derives a complete two-child deployment");
    let cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("the unchanged I2 control still reaches the existing checked cohort boundary");
    let summary = cohort.observer_safe_summary();
    assert_eq!(summary.full_admission_count(), 1);
    assert_eq!(summary.authority_generation_count(), 1);
}
