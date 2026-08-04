use std::path::PathBuf;

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9_AUTH_REJECTED_FAILURE, M9_AUTHORITY_OBSERVATION_LABEL,
        M9_AUTHORITY_OBSERVATION_REDACTION, M9_MEMBERSHIP_AUTH_CAPABILITY,
        M9_MEMBERSHIP_AUTH_PRECONDITION, M9ContractCandidate, M9FiniteRefinementChecker,
        M9FiniteRefinementDischarge,
    },
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

use super::*;
use crate::m8_runtime_admission::{
    EvidenceRedaction, EvidenceSecurityLabel, M8AdmissionDiagnosticKind, M8AdmissionEvidence,
    M8Runtime, M8RuntimeAdmission, M8SecurityClass,
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const CANONICAL_FIXTURE: &str = "canonical_attack_bundle.mir";
const RELATION_NAME: &str = "bird_follow";
const VALUE_NAME: &str = "E.result";
const AUTH_NAME: &str = "MembershipAuth";
const VERIFY_NAME: &str = "finite_refinement";

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

fn checked_canonical() -> CheckedSurfaceV0 {
    let (path, source) = load_surface_fixture(CANONICAL_FIXTURE);
    check_and_elaborate_surface_v0(FixtureSource::new(path, source))
        .expect("canonical M9 source fixture checks through ordinary M7")
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

fn shifted_source_ref(source_ref: &SourceRef) -> SourceRef {
    SourceRef::new(
        source_ref.path.clone(),
        source_ref.start_line,
        source_ref.start_column + 1,
        source_ref.end_line,
        source_ref.end_column + 1,
    )
}

fn relation_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationVisibility {
        relation: RELATION_NAME.into(),
        label: EvidenceSecurityLabel::new("relation:bird_follow:consumer-visible")
            .with_class(M8SecurityClass::Restricted),
        redaction: EvidenceRedaction::new("consumer:C"),
        source_ref,
    }
}

fn relation_lifetime_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationLifetime {
        relation: RELATION_NAME.into(),
        live_lease: "bird_binding_frontier/live".into(),
        binding_frontier: "bird_binding_frontier".into(),
        source_ref,
    }
}

fn relation_fallback_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::RelationFallbackValidity {
        relation: RELATION_NAME.into(),
        primary_epoch: "primary_epoch".into(),
        fallback_epoch: "fallback_epoch".into(),
        source_ref,
    }
}

fn designated_visibility_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::ValueVisibilityRedaction {
        value: VALUE_NAME.into(),
        label: EvidenceSecurityLabel::new("value:E.result:publish")
            .with_class(M8SecurityClass::Private),
        redaction: EvidenceRedaction::new("conservative"),
        source_ref,
    }
}

fn deferred_auth_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::AuthDeferred {
        name: AUTH_NAME.into(),
        authority_label: "membership-authority/MembershipAuth".into(),
        source_ref,
    }
}

fn deferred_verify_evidence(source_ref: SourceRef) -> M8AdmissionEvidence {
    M8AdmissionEvidence::VerifyDeferred {
        name: VERIFY_NAME.into(),
        theorem: VERIFY_NAME.into(),
        witness_schema: "m9-proof-witness-required".into(),
        source_ref,
    }
}

fn exact_m8_base_admission_for(checked: &CheckedSurfaceV0) -> M8RuntimeAdmission {
    M8RuntimeAdmission::new(checked.program_identity().clone())
        .with_evidence(relation_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::Visibility,
            RELATION_NAME,
        )))
        .with_evidence(relation_lifetime_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::RelationLifetime,
            RELATION_NAME,
        )))
        .with_evidence(relation_fallback_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::FallbackValidity,
            RELATION_NAME,
        )))
        .with_evidence(designated_visibility_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::ValueVisibilityRedaction,
            VALUE_NAME,
        )))
        .with_evidence(deferred_auth_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::AuthDeferred,
            AUTH_NAME,
        )))
        .with_evidence(deferred_verify_evidence(residual_source_ref(
            checked,
            ResidualObligationKind::VerifyDeferred,
            VERIFY_NAME,
        )))
}

fn auth_binding(source_ref: SourceRef) -> M9ResidualBinding {
    M9ResidualBinding::auth_deferred(AUTH_NAME)
        .with_source_ref(source_ref)
        .with_module_contract("Combat.M6", "membership-authority/MembershipAuth")
}

fn verify_binding(source_ref: SourceRef) -> M9ResidualBinding {
    M9ResidualBinding::verify_deferred(VERIFY_NAME)
        .with_source_ref(source_ref)
        .with_module_contract("Combat.M6", "finite-refinement/MembershipAuth")
}

fn exact_m9_envelope(checked: &CheckedSurfaceV0) -> M9AdmissionEnvelope {
    M9AdmissionEnvelope::for_checked_identity(checked.program_identity().clone())
        .with_original_source_artifact(M9SourceArtifact::from_checked_surface(checked))
        .with_residual_binding(auth_binding(residual_source_ref(
            checked,
            ResidualObligationKind::AuthDeferred,
            AUTH_NAME,
        )))
        .with_residual_binding(verify_binding(residual_source_ref(
            checked,
            ResidualObligationKind::VerifyDeferred,
            VERIFY_NAME,
        )))
}

fn opaque_m9_admitted_base_for(checked: &CheckedSurfaceV0) -> M9AdmittedBase {
    M9AdmissionRuntime::default()
        .admit_source_bound_base(
            checked.clone(),
            exact_m8_base_admission_for(checked),
            exact_m9_envelope(checked),
        )
        .expect("source-bound base admits")
}

fn source_bound_authority_runtime(checked: &CheckedSurfaceV0) -> M9AuthorityRuntime {
    M9AuthorityRuntime::from_outer_admission(
        M9AdmissionRuntime::default()
            .admit_outer(checked.clone(), exact_m9_envelope(checked))
            .expect("outer admission"),
    )
}

fn issue_membership_for_epoch(
    runtime: &mut M9AuthorityRuntime,
    checked: &CheckedSurfaceV0,
    epoch: &str,
) -> M9MembershipAuth {
    let auth_ref = residual_source_ref(checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let attestation = runtime
        .issue_membership_attestation(
            "self",
            "S",
            epoch,
            format!("incarnation:self:S:{epoch}"),
            AUTH_NAME,
            auth_ref.clone(),
        )
        .expect("crate-private provider lane issues membership attestation");
    runtime
        .authenticate_membership(
            M9MembershipRequest::new("self", "S", epoch)
                .with_auth_residual(AUTH_NAME, auth_ref)
                .with_issued_provider_attestation(attestation)
                .with_transport_claim(M9TransportClaim::new("session:self:S")),
        )
        .expect("issued provider attestation can authenticate membership")
}

fn issue_contract_update_authority(
    runtime: &mut M9AuthorityRuntime,
    checked: &CheckedSurfaceV0,
    membership: &M9MembershipAuth,
) -> (M9CapabilityAuth, M9WitnessAuth, M9ContractAuthorityUse) {
    let auth_ref = residual_source_ref(checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let capability = runtime
        .authorize_capability(
            M9CapabilityGrantRequest::new("cap:contract:update:Combat.M6:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::contract_update(
                    "Combat.M6",
                    "membership-authority/MembershipAuth",
                ))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
        )
        .expect("issued membership can grant ContractUpdate capability");
    let witness = runtime
        .materialize_witness(
            M9WitnessRequest::new("witness:contract:update:Combat.M6:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref),
        )
        .expect("witness binds ContractUpdate capability");
    let authority = M9ContractAuthorityUse::from_grant_and_witness(&capability, &witness);
    (capability, witness, authority)
}

fn issue_owner_evaluation_authority(
    runtime: &mut M9AuthorityRuntime,
    checked: &CheckedSurfaceV0,
    membership: &M9MembershipAuth,
) -> (M9CapabilityAuth, M9WitnessAuth) {
    let auth_ref = residual_source_ref(checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let capability = runtime
        .authorize_capability(
            M9CapabilityGrantRequest::new("cap:attack:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("attack", "S"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
        )
        .expect("issued membership can grant owner-evaluation capability");
    let witness = runtime
        .materialize_witness(
            M9WitnessRequest::new("witness:attack:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref),
        )
        .expect("witness binds owner-evaluation capability");
    (capability, witness)
}

fn issue_observer_authority(
    runtime: &mut M9AuthorityRuntime,
    checked: &CheckedSurfaceV0,
    membership: &M9MembershipAuth,
) -> M9ContractObservationUse {
    let auth_ref = residual_source_ref(checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let capability = runtime
        .authorize_capability(
            M9CapabilityGrantRequest::new("cap:contract:observe:Combat.M6:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::bounded_observation("self"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
        )
        .expect("issued membership can grant observer capability");
    let witness = runtime
        .materialize_witness(
            M9WitnessRequest::new("witness:contract:observe:Combat.M6:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref),
        )
        .expect("witness binds observer capability");
    M9ContractObservationUse::from_grant_and_witness(&capability, &witness)
}

fn exact_finite_refinement_discharge(checked: &CheckedSurfaceV0) -> M9FiniteRefinementDischarge {
    M9FiniteRefinementChecker::default()
        .discharge_candidate(
            checked,
            M9ContractCandidate::from_checked_surface(checked).membership_auth_strengthening(),
        )
        .expect("exact finite refinement evidence discharges Verify")
}

fn exact_membership_auth_delta() -> M9ContractDelta {
    M9ContractDelta::new()
        .with_precondition(M9PreconditionDelta::strengthens(
            M9_MEMBERSHIP_AUTH_PRECONDITION,
        ))
        .with_capability_requirement(M9CapabilityRequirementDelta::requires(
            M9_MEMBERSHIP_AUTH_CAPABILITY,
        ))
        .with_failure(M9FailureDelta::adds_declared(M9_AUTH_REJECTED_FAILURE))
        .with_observation(M9ObservationDelta::adds_redacted_label(
            M9_AUTHORITY_OBSERVATION_LABEL,
            M9_AUTHORITY_OBSERVATION_REDACTION,
        ))
}

fn missing_capability_membership_auth_delta() -> M9ContractDelta {
    M9ContractDelta::new()
        .with_precondition(M9PreconditionDelta::strengthens(
            M9_MEMBERSHIP_AUTH_PRECONDITION,
        ))
        .with_failure(M9FailureDelta::adds_declared(M9_AUTH_REJECTED_FAILURE))
        .with_observation(M9ObservationDelta::adds_redacted_label(
            M9_AUTHORITY_OBSERVATION_LABEL,
            M9_AUTHORITY_OBSERVATION_REDACTION,
        ))
}

fn exact_contract_attach_update(
    checked: &CheckedSurfaceV0,
    update_ref: &str,
    authority: M9ContractAuthorityUse,
) -> M9ContractUpdate {
    M9ContractUpdate::new(update_ref, M9ContractUpdateKind::Attach)
        .with_layer(auth_layer_descriptor())
        .with_authority(authority)
        .with_delta(exact_membership_auth_delta())
        .with_finite_refinement(exact_finite_refinement_discharge(checked))
}

fn auth_layer_descriptor() -> M9LayerDescriptor {
    M9LayerDescriptor::new("membership-auth-layer")
        .non_transparent()
        .with_contract_ref("contract:membership-auth-layer:v1")
        .with_module_contract("Combat.M6", "membership-authority/MembershipAuth")
}

fn contract_capability_removal_revocation() -> M9Revocation {
    M9Revocation::capability("cap:contract:update:Combat.M6:self:epoch1")
        .with_witness_ref("witness:contract:update:Combat.M6:self:epoch1")
        .with_dependent_artifact("contract-update:attach-membership-auth-layer:v1")
}

fn runtime_with_contract_and_observer() -> (
    CheckedSurfaceV0,
    M9ContractRuntime,
    M9ContractAuthorityUse,
    M9ContractObservationUse,
    M9Revocation,
) {
    let checked = checked_canonical();
    let prepared = opaque_m9_admitted_base_for(&checked);
    let mut authority_runtime = source_bound_authority_runtime(&checked);
    let membership = issue_membership_for_epoch(&mut authority_runtime, &checked, "epoch1");
    let (contract_capability, contract_witness, contract_authority) =
        issue_contract_update_authority(&mut authority_runtime, &checked, &membership);
    let observer = issue_observer_authority(&mut authority_runtime, &checked, &membership);
    let (unrelated_capability, unrelated_witness) =
        issue_owner_evaluation_authority(&mut authority_runtime, &checked, &membership);
    let unrelated_revocation = M9Revocation::capability(unrelated_capability.ref_id())
        .with_witness_ref(unrelated_witness.ref_id())
        .with_dependent_artifact("contract-update:unrelated-owner-eval:B");
    let final_admission = M9AdmissionRuntime::default()
        .admit_runtime(
            prepared,
            authority_runtime,
            M9FinalAdmissionEvidence::from_lineage(
                &membership,
                &contract_capability,
                &contract_witness,
                exact_finite_refinement_discharge(&checked),
            ),
        )
        .expect("issued M9 evidence admits runtime");
    (
        checked,
        M9ContractRuntime::from_runtime_admitted(final_admission),
        contract_authority,
        observer,
        unrelated_revocation,
    )
}

#[test]
fn source_bound_base_and_outer_admission_reject_missing_m8_or_m9_binding_mutations() {
    let checked = checked_canonical();
    let auth_ref = residual_source_ref(&checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let verify_ref = residual_source_ref(
        &checked,
        ResidualObligationKind::VerifyDeferred,
        VERIFY_NAME,
    );
    let m8_admission = exact_m8_base_admission_for(&checked);

    let direct_m8 = M8Runtime::default()
        .admit(checked.clone(), m8_admission.clone())
        .expect_err("direct M8 admission keeps auth/verify deferred to M9");
    assert_eq!(
        direct_m8.primary().kind(),
        M8AdmissionDiagnosticKind::DeferredToM9
    );
    assert!(!direct_m8.has_runtime_success());
    assert!(!direct_m8.grants_authority());
    assert!(!direct_m8.emits_verdict());

    let admitted = M9AdmissionRuntime::default()
        .admit_source_bound_base(checked.clone(), m8_admission, exact_m9_envelope(&checked))
        .expect("exact checked source, M8 evidence, and M9 bindings admit only an opaque base");
    assert_eq!(admitted.program_identity(), checked.program_identity());
    assert_eq!(admitted.m8_base_evidence().len(), 6);
    assert!(!admitted.has_runtime_success());
    assert!(!admitted.exposes_raw_m8_instance());

    let missing_m8 = M9AdmissionRuntime::default()
        .admit_source_bound_base(
            checked.clone(),
            M8RuntimeAdmission::new(checked.program_identity().clone()),
            exact_m9_envelope(&checked),
        )
        .expect_err("missing M8 evidence cannot be papered over by exact M9 bindings");
    assert_eq!(
        missing_m8.primary().kind(),
        M9AdmissionErrorKind::M8BaseEvidenceMissing
    );
    assert!(!missing_m8.has_runtime_success());

    for (delta, expected) in [
        (
            M9AdmissionBindingDelta::remove(AUTH_NAME),
            M9AdmissionErrorKind::MissingResidualBinding,
        ),
        (
            M9AdmissionBindingDelta::duplicate(AUTH_NAME),
            M9AdmissionErrorKind::DuplicateResidualBinding,
        ),
        (
            M9AdmissionBindingDelta::replace(
                AUTH_NAME,
                auth_binding(shifted_source_ref(&auth_ref)),
            ),
            M9AdmissionErrorKind::SourceRefMismatch,
        ),
        (
            M9AdmissionBindingDelta::replace(
                VERIFY_NAME,
                M9ResidualBinding::auth_deferred(VERIFY_NAME).with_source_ref(verify_ref.clone()),
            ),
            M9AdmissionErrorKind::ResidualKindMismatch,
        ),
    ] {
        let diagnostics = M9AdmissionRuntime::default()
            .admit_outer(
                checked.clone(),
                exact_m9_envelope(&checked).apply_delta(delta),
            )
            .expect_err("outer binding mutation must reject before runtime admission");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert!(!diagnostics.has_runtime_success());
        assert!(!diagnostics.grants_authority());
    }
}

#[test]
fn ambient_claims_source_absent_scopes_and_duplicate_refs_do_not_mint_authority() {
    let checked = checked_canonical();
    let auth_ref = residual_source_ref(&checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let mut runtime = source_bound_authority_runtime(&checked);

    for claim in [
        M9AuthorityClaim::from_transport_session("session:self:S"),
        M9AuthorityClaim::from_locus_name("S"),
        M9AuthorityClaim::from_provider_name("provider:membership-root"),
        M9AuthorityClaim::from_principal_name("self"),
    ] {
        let before = runtime.authority_snapshot();
        let diagnostics = runtime
            .authenticate_membership(
                M9MembershipRequest::new("self", "S", "epoch1")
                    .with_auth_residual(AUTH_NAME, auth_ref.clone())
                    .with_authority_claim(claim),
            )
            .expect_err("ambient identity claims are not typed membership authority");
        assert_eq!(
            diagnostics.primary().kind(),
            M9AdmissionErrorKind::ProviderOrTransportIsNotAuthority
        );
        assert_eq!(runtime.authority_snapshot(), before);
        assert!(!diagnostics.grants_authority());
    }

    let membership = issue_membership_for_epoch(&mut runtime, &checked, "epoch1");
    for claim in [
        M9AuthorityClaim::from_transport_session("session:self:S"),
        M9AuthorityClaim::from_locus_name("S"),
        M9AuthorityClaim::from_provider_name("provider:membership-root"),
        M9AuthorityClaim::from_principal_name("self"),
    ] {
        let before = runtime.authority_snapshot();
        let diagnostics = runtime
            .authorize_capability(
                M9CapabilityGrantRequest::new("cap:attack:S:self:epoch1")
                    .with_membership_ref(membership.ref_id())
                    .with_scope(M9CapabilityScope::owner_evaluation("attack", "S"))
                    .with_authority_claim(claim),
            )
            .expect_err("ambient identity claims are not typed capability authority");
        assert_eq!(
            diagnostics.primary().kind(),
            M9AdmissionErrorKind::ProviderOrTransportIsNotAuthority
        );
        assert_eq!(runtime.authority_snapshot(), before);
        assert!(!diagnostics.grants_authority());
    }

    let before = runtime.authority_snapshot();
    let diagnostics = runtime
        .authorize_capability(
            M9CapabilityGrantRequest::new("cap:heal:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("heal", "S"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
        )
        .expect_err("caller-chosen source-absent owner scope cannot mint authority");
    assert_eq!(
        diagnostics.primary().kind(),
        M9AdmissionErrorKind::CapabilityPolicyRejected
    );
    assert_eq!(runtime.authority_snapshot(), before);
    assert!(
        runtime
            .materialize_witness(
                M9WitnessRequest::new("witness:heal:S:self:epoch1")
                    .with_membership_ref(membership.ref_id())
                    .with_capability_ref("cap:heal:S:self:epoch1")
                    .with_source_ref(auth_ref.clone())
            )
            .is_err()
    );

    let (capability, witness) =
        issue_owner_evaluation_authority(&mut runtime, &checked, &membership);
    for (request, expected) in [
        (
            M9CapabilityGrantRequest::new(capability.ref_id())
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("attack", "S"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
            M9AdmissionErrorKind::DuplicateCapabilityReference,
        ),
        (
            M9CapabilityGrantRequest::new(capability.ref_id())
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("heal", "S"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
            M9AdmissionErrorKind::ConflictingCapabilityReference,
        ),
    ] {
        let before = runtime.authority_snapshot();
        let diagnostics = runtime
            .authorize_capability(request)
            .expect_err("capability ref reissue must reject without last-wins overwrite");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert_eq!(runtime.authority_snapshot(), before);
    }

    for (request, expected) in [
        (
            M9WitnessRequest::new(witness.ref_id())
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref.clone()),
            M9AdmissionErrorKind::DuplicateWitnessReference,
        ),
        (
            M9WitnessRequest::new(witness.ref_id())
                .with_membership_ref("membership:self:S:epoch0")
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref),
            M9AdmissionErrorKind::ConflictingWitnessReference,
        ),
    ] {
        let before = runtime.authority_snapshot();
        let diagnostics = runtime
            .materialize_witness(request)
            .expect_err("witness ref reissue must reject without last-wins overwrite");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert_eq!(runtime.authority_snapshot(), before);
    }
}

#[test]
fn stale_copied_wrong_target_or_revoked_uses_fail_and_record_typed_invalidation() {
    let checked = checked_canonical();
    let auth_ref = residual_source_ref(&checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let mut runtime = source_bound_authority_runtime(&checked);
    let membership = issue_membership_for_epoch(&mut runtime, &checked, "epoch1");
    let (capability, witness) =
        issue_owner_evaluation_authority(&mut runtime, &checked, &membership);

    for attempted_use in [
        M9FactUse::capability("cap:attack:S:self:epoch0")
            .with_membership_ref(membership.ref_id())
            .with_witness_ref(witness.ref_id())
            .with_epoch("epoch0"),
        M9FactUse::capability(capability.ref_id())
            .with_membership_ref("membership:attacker:S:epoch1")
            .with_witness_ref(witness.ref_id())
            .with_copied_from(capability.ref_id()),
        M9FactUse::capability(capability.ref_id())
            .with_membership_ref(membership.ref_id())
            .with_witness_ref(witness.ref_id())
            .with_scope(M9CapabilityScope::owner_evaluation("heal", "S")),
        M9FactUse::capability(capability.ref_id())
            .with_membership_ref(membership.ref_id())
            .with_witness_ref("witness:attack:S:self:revoked")
            .with_revocation_ref("revocation:cap:attack:S:self:epoch1"),
    ] {
        let before = runtime.authority_snapshot();
        let diagnostics = runtime
            .use_authority(attempted_use)
            .expect_err("stale, copied, wrong-target, and revoked grants must fail closed");
        assert_eq!(
            diagnostics.primary().kind(),
            M9AdmissionErrorKind::InvalidCapabilityLineage
        );
        assert_eq!(runtime.authority_snapshot(), before);
    }

    runtime
        .revoke(M9Revocation::capability(capability.ref_id()).with_source_ref(auth_ref))
        .expect("revocation records typed evidence");
    assert!(
        runtime
            .evidence_graph()
            .invalidated_artifacts_for(capability.ref_id())
            .contains(&witness.ref_id().to_string())
    );
    assert!(
        runtime
            .use_authority(
                M9FactUse::capability(capability.ref_id())
                    .with_membership_ref(membership.ref_id())
                    .with_witness_ref(witness.ref_id())
            )
            .is_err()
    );
}

#[test]
fn issued_provider_attestation_can_issue_membership_capability_and_witness() {
    let checked = checked_canonical();
    let auth_ref = residual_source_ref(&checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let mut runtime = source_bound_authority_runtime(&checked);
    let membership = issue_membership_for_epoch(&mut runtime, &checked, "epoch1");
    assert_eq!(membership.ref_id(), "membership:self:S:epoch1");
    assert_eq!(membership.provider_ref(), "provider:membership-root");

    let capability = runtime
        .authorize_capability(
            M9CapabilityGrantRequest::new("cap:attack:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("attack", "S"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
        )
        .expect("issued membership can grant exact owner-evaluation capability");
    let witness = runtime
        .materialize_witness(
            M9WitnessRequest::new("witness:attack:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref),
        )
        .expect("witness binds issued membership and capability");
    assert_eq!(witness.membership_ref(), membership.ref_id());
    assert_eq!(witness.capability_ref(), capability.ref_id());
}

#[test]
fn fresh_epoch_membership_invalidates_prior_epoch_lineage_and_epoch_omission_is_not_authority() {
    let checked = checked_canonical();
    let auth_ref = residual_source_ref(&checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let mut runtime = source_bound_authority_runtime(&checked);
    let membership = issue_membership_for_epoch(&mut runtime, &checked, "epoch1");
    let capability = runtime
        .authorize_capability(
            M9CapabilityGrantRequest::new("cap:attack:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_scope(M9CapabilityScope::owner_evaluation("attack", "S"))
                .with_lineage_epoch("epoch1")
                .with_source_ref(auth_ref.clone()),
        )
        .expect("epoch1 capability");
    let witness = runtime
        .materialize_witness(
            M9WitnessRequest::new("witness:attack:S:self:epoch1")
                .with_membership_ref(membership.ref_id())
                .with_capability_ref(capability.ref_id())
                .with_source_ref(auth_ref),
        )
        .expect("epoch1 witness");
    let epoch2 = issue_membership_for_epoch(&mut runtime, &checked, "epoch2");
    assert_eq!(epoch2.incarnation(), "incarnation:self:S:epoch2");

    for attempted_use in [
        M9FactUse::capability(capability.ref_id())
            .with_membership_ref(membership.ref_id())
            .with_witness_ref(witness.ref_id())
            .with_epoch("epoch1"),
        M9FactUse::capability(capability.ref_id())
            .with_membership_ref(membership.ref_id())
            .with_witness_ref(witness.ref_id()),
    ] {
        let diagnostics = runtime
            .use_authority(attempted_use)
            .expect_err("fresh epoch admission invalidates stale or epochless use");
        assert_eq!(
            diagnostics.primary().kind(),
            M9AdmissionErrorKind::InvalidCapabilityLineage
        );
    }
}

#[test]
fn fresh_epoch_membership_invalidates_prior_contract_update_lineage_before_final_admission() {
    let checked = checked_canonical();
    let prepared = opaque_m9_admitted_base_for(&checked);
    let mut authority_runtime = source_bound_authority_runtime(&checked);
    let epoch1_membership = issue_membership_for_epoch(&mut authority_runtime, &checked, "epoch1");
    let (contract_capability, contract_witness, _contract_authority) =
        issue_contract_update_authority(&mut authority_runtime, &checked, &epoch1_membership);
    let epoch2 = issue_membership_for_epoch(&mut authority_runtime, &checked, "epoch2");
    assert_eq!(epoch2.ref_id(), "membership:self:S:epoch2");

    let diagnostics = M9AdmissionRuntime::default()
        .admit_runtime(
            prepared,
            authority_runtime,
            M9FinalAdmissionEvidence::from_lineage(
                &epoch1_membership,
                &contract_capability,
                &contract_witness,
                exact_finite_refinement_discharge(&checked),
            ),
        )
        .expect_err("fresh epoch admission invalidates epoch1 ContractUpdate final evidence");
    assert_eq!(
        diagnostics.primary().kind(),
        M9AdmissionErrorKind::InvalidCapabilityLineage
    );
    assert!(!diagnostics.has_runtime_success());
}

#[test]
fn finite_bound_contract_update_rejects_delta_mismatch_without_cut_or_m8_mutation_and_accepts_exact_delta()
 {
    let (checked, mut runtime, contract_authority, _observer, _unrelated_revocation) =
        runtime_with_contract_and_observer();
    let before_payload = runtime.m8_payload_snapshot();
    let before_contract = runtime.active_contract().clone();
    let discharge = exact_finite_refinement_discharge(&checked);

    let diagnostics = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:attach-membership-auth-layer:mismatched-delta",
                M9ContractUpdateKind::Attach,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority.clone())
            .with_delta(missing_capability_membership_auth_delta())
            .with_finite_refinement(discharge.clone()),
        )
        .expect_err("actual ContractUpdate delta must match the verifier-bound normalized delta");

    assert_eq!(
        diagnostics.primary().kind(),
        M9ContractUpdateDiagnosticsKind::FiniteRefinementMismatch
    );
    assert!(diagnostics.activation_cut().is_none());
    assert!(!diagnostics.has_runtime_success());
    assert_eq!(runtime.m8_payload_snapshot(), before_payload);
    assert_eq!(runtime.active_contract(), &before_contract);

    let accepted = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:attach-membership-auth-layer:v1",
                M9ContractUpdateKind::Attach,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority)
            .with_delta(exact_membership_auth_delta())
            .with_finite_refinement(discharge),
        )
        .expect("exact verifier-bound ContractUpdate delta activates");

    let activation_cut = accepted
        .activation_cut()
        .expect("exact non-transparent update records an activation cut");
    assert!(
        runtime
            .active_contract()
            .contains_layer("membership-auth-layer")
    );
    assert_eq!(accepted.contract_delta(), &exact_membership_auth_delta());
    assert_eq!(runtime.m8_payload_snapshot(), before_payload);
    assert!(
        activation_cut
            .preserves_m8_payload_invariant(&before_payload, &runtime.m8_payload_snapshot())
    );
}

#[test]
fn implicit_delta_and_public_observation_widening_reject_without_contract_state_mutation() {
    let (checked, mut runtime, contract_authority, _observer, _unrelated_revocation) =
        runtime_with_contract_and_observer();
    let before_payload = runtime.m8_payload_snapshot();
    let before_contract = runtime.active_contract().clone();

    for diagnostics in [
        runtime
            .remove_layer("membership-auth-layer")
            .expect_err("removing a layer requires an explicit ContractUpdate"),
        runtime
            .apply_implicit_contract_delta(exact_membership_auth_delta())
            .expect_err("implicit precondition/failure deltas cannot mutate M9 contract state"),
    ] {
        assert_eq!(
            diagnostics.primary().kind(),
            M9ContractUpdateDiagnosticsKind::MissingContractUpdate
        );
        assert!(diagnostics.activation_cut().is_none());
        assert!(!diagnostics.has_runtime_success());
    }

    let diagnostics = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:attach-membership-auth-layer:public-observation",
                M9ContractUpdateKind::Attach,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority)
            .with_delta(exact_membership_auth_delta().with_observation(
                M9ObservationDelta::widens_label(M9_AUTHORITY_OBSERVATION_LABEL, "public"),
            ))
            .with_finite_refinement(exact_finite_refinement_discharge(&checked)),
        )
        .expect_err("ContractUpdate cannot widen the authority-private observation policy");

    assert_eq!(
        diagnostics.primary().kind(),
        M9ContractUpdateDiagnosticsKind::ObservationPolicyWeakening
    );
    assert!(diagnostics.activation_cut().is_none());
    assert!(!diagnostics.has_runtime_success());
    assert_eq!(runtime.m8_payload_snapshot(), before_payload);
    assert_eq!(runtime.active_contract(), &before_contract);
}

#[test]
fn removal_rejects_unrelated_capability_revocation_and_preserves_contract_state() {
    let (checked, mut runtime, contract_authority, _observer, unrelated_revocation) =
        runtime_with_contract_and_observer();

    runtime
        .apply_contract_update(exact_contract_attach_update(
            &checked,
            "contract-update:attach-membership-auth-layer:v1",
            contract_authority.clone(),
        ))
        .expect("attach layer");
    let before_payload = runtime.m8_payload_snapshot();
    let before_contract = runtime.active_contract().clone();

    let diagnostics = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:remove-membership-auth-layer-with-unrelated-B:v2",
                M9ContractUpdateKind::Remove,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority)
            .with_removal_revocation(unrelated_revocation),
        )
        .expect_err("removal cannot tombstone unrelated capability B in the layer cut");
    assert_eq!(
        diagnostics.primary().kind(),
        M9ContractUpdateDiagnosticsKind::InvalidContractAuthority
    );
    assert_eq!(runtime.m8_payload_snapshot(), before_payload);
    assert_eq!(runtime.active_contract(), &before_contract);
}

#[test]
fn contract_update_lifecycle_covers_duplicate_attach_exact_remove_and_observer_projection() {
    let (checked, mut runtime, contract_authority, observer, _unrelated_revocation) =
        runtime_with_contract_and_observer();
    let initial_payload = runtime.m8_payload_snapshot();

    let missing_active = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:remove-membership-auth-layer:before-attach",
                M9ContractUpdateKind::Remove,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority.clone())
            .with_removal_revocation(contract_capability_removal_revocation()),
        )
        .expect_err("removing a non-active layer must reject before mutation");
    assert_eq!(
        missing_active.primary().kind(),
        M9ContractUpdateDiagnosticsKind::MissingActiveLayer
    );
    assert!(missing_active.activation_cut().is_none());
    assert_eq!(runtime.m8_payload_snapshot(), initial_payload);

    let attached = runtime
        .apply_contract_update(exact_contract_attach_update(
            &checked,
            "contract-update:attach-membership-auth-layer:v1",
            contract_authority.clone(),
        ))
        .expect("exact attach activates the non-transparent MembershipAuth layer");
    let attach_cut = attached
        .activation_cut()
        .expect("attach records an activation cut");
    assert!(
        runtime
            .active_contract()
            .contains_layer("membership-auth-layer")
    );
    assert!(
        attach_cut.preserves_m8_payload_invariant(&initial_payload, &runtime.m8_payload_snapshot())
    );
    let attach_rows = runtime
        .observe_contract_evidence(
            M9ContractObservationRequest::new("observe:normal-attach")
                .with_observer(observer.clone())
                .with_update_ref(attached.provenance().update_ref())
                .with_provenance_ref(attached.provenance().activation_cut_id()),
        )
        .expect("authorized observer can project normal attach provenance");
    assert!(
        attach_rows
            .iter()
            .any(|row| row.kind() == M9ContractObservationRowKind::Provenance)
    );

    let before_duplicate_attach = runtime.active_contract().clone();
    let duplicate_attach = runtime
        .apply_contract_update(exact_contract_attach_update(
            &checked,
            "contract-update:attach-membership-auth-layer:duplicate",
            contract_authority.clone(),
        ))
        .expect_err("duplicate active attach must reject before any new cut");
    assert_eq!(
        duplicate_attach.primary().kind(),
        M9ContractUpdateDiagnosticsKind::DuplicateActiveLayer
    );
    assert!(duplicate_attach.activation_cut().is_none());
    assert_eq!(runtime.active_contract(), &before_duplicate_attach);
    assert_eq!(runtime.m8_payload_snapshot(), initial_payload);

    let before_duplicate_removal = runtime.active_contract().clone();
    let duplicate_removal = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:remove-membership-auth-layer:duplicate-revocation",
                M9ContractUpdateKind::Remove,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority.clone())
            .with_removal_revocation(contract_capability_removal_revocation())
            .with_removal_revocation(contract_capability_removal_revocation()),
        )
        .expect_err("duplicate removal revocations reject before mutation");
    assert_eq!(
        duplicate_removal.primary().kind(),
        M9ContractUpdateDiagnosticsKind::DuplicateRemovalRevocation
    );
    assert!(duplicate_removal.activation_cut().is_none());
    assert_eq!(runtime.active_contract(), &before_duplicate_removal);
    assert_eq!(runtime.m8_payload_snapshot(), initial_payload);

    let removed = runtime
        .apply_contract_update(
            M9ContractUpdate::new(
                "contract-update:remove-membership-auth-layer:v2",
                M9ContractUpdateKind::Remove,
            )
            .with_layer(auth_layer_descriptor())
            .with_authority(contract_authority.clone())
            .with_removal_revocation(contract_capability_removal_revocation()),
        )
        .expect("exact remove and revocation happen in one activation cut");
    let remove_cut = removed
        .activation_cut()
        .expect("exact remove records an activation cut");
    assert!(remove_cut.removes_layer("membership-auth-layer"));
    assert!(remove_cut.tombstones_capability("cap:contract:update:Combat.M6:self:epoch1"));
    assert!(remove_cut.invalidates_dependent("contract-update:attach-membership-auth-layer:v1"));
    assert!(
        !runtime
            .active_contract()
            .contains_layer("membership-auth-layer")
    );
    assert!(
        remove_cut.preserves_m8_payload_invariant(&initial_payload, &runtime.m8_payload_snapshot())
    );

    let remove_rows = runtime
        .observe_contract_evidence(
            M9ContractObservationRequest::new("observe:normal-remove")
                .with_observer(observer)
                .with_update_ref(removed.provenance().update_ref())
                .with_provenance_ref(removed.provenance().activation_cut_id()),
        )
        .expect("authorized observer can project normal remove invalidation");
    assert!(
        remove_rows
            .iter()
            .any(|row| row.kind() == M9ContractObservationRowKind::Invalidation)
    );
}

#[test]
fn direct_revocation_public_bypass_must_emit_observer_safe_invalidation_rows() {
    let (checked, mut runtime, contract_authority, observer, _unrelated_revocation) =
        runtime_with_contract_and_observer();
    let attached = runtime
        .apply_contract_update(exact_contract_attach_update(
            &checked,
            "contract-update:attach-membership-auth-layer:v1",
            contract_authority,
        ))
        .expect("attach layer");

    runtime
        .apply_revocation(contract_capability_removal_revocation())
        .expect("public direct revocation bypass is currently callable");
    let rows = runtime
        .observe_contract_evidence(
            M9ContractObservationRequest::new("observe:direct-revocation")
                .with_observer(observer)
                .with_update_ref(attached.provenance().update_ref())
                .with_provenance_ref(attached.provenance().activation_cut_id()),
        )
        .expect("authorized observer can request safe projection");
    assert!(
        rows.iter()
            .any(|row| row.kind() == M9ContractObservationRowKind::Invalidation),
        "direct public revocation must either be absent or produce an observer-safe invalidation row"
    );
    for row in rows {
        let payload = row.redacted_payload();
        for raw in [
            "membership:self:S:epoch1",
            "cap:contract:update:Combat.M6:self:epoch1",
            "witness:contract:update:Combat.M6:self:epoch1",
            "provider:membership-root",
            "session:self:S",
        ] {
            assert!(
                !payload.contains(raw),
                "observer-safe direct revocation row leaked raw authority payload {raw}"
            );
        }
    }
}
