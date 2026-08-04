use std::path::PathBuf;

use mir_ast::surface_v0::FixtureSource;
use mir_runtime::m9_auth_verification::{
    M9AdmissionEnvelope, M9AdmissionErrorKind, M9AdmissionRuntime, M9AuthorityRuntime,
    M9MembershipRequest, M9ProofRef, M9ProviderProof, M9ResidualBinding, M9SourceArtifact,
    M9TransportClaim,
};
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const CANONICAL_FIXTURE: &str = "canonical_attack_bundle.mir";
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

fn checked_canonical() -> (String, String, CheckedSurfaceV0) {
    let (path, source) = load_surface_fixture(CANONICAL_FIXTURE);
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(path.clone(), source.clone()))
        .expect("canonical M9 source fixture checks through ordinary M7");
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

fn exact_outer_envelope(checked: &CheckedSurfaceV0) -> M9AdmissionEnvelope {
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

fn source_bound_authority_runtime(checked: &CheckedSurfaceV0) -> M9AuthorityRuntime {
    M9AuthorityRuntime::from_outer_admission(
        M9AdmissionRuntime::default()
            .admit_outer(checked.clone(), exact_outer_envelope(checked))
            .expect("outer admission"),
    )
}

fn forged_public_provider_proof(auth_ref: &SourceRef) -> M9ProviderProof {
    M9ProviderProof::new("provider:membership-root")
        .for_auth_kind(AUTH_NAME)
        .with_source_ref(auth_ref.clone())
        .with_proof_ref(M9ProofRef::new("proof:external-caller-forged:epoch1"))
        .with_membership_claim(
            "self",
            "S",
            "epoch1",
            "incarnation:self:S:epoch1",
            "m9-policy-v1",
        )
}

#[test]
fn external_public_provider_proof_claim_cannot_authenticate_membership() {
    let (_path, _source, checked) = checked_canonical();
    let auth_ref = residual_source_ref(&checked, ResidualObligationKind::AuthDeferred, AUTH_NAME);
    let mut runtime = source_bound_authority_runtime(&checked);

    let diagnostics = runtime
        .authenticate_membership(
            M9MembershipRequest::new("self", "S", "epoch1")
                .with_auth_residual(AUTH_NAME, auth_ref.clone())
                .with_provider_proof(forged_public_provider_proof(&auth_ref))
                .with_transport_claim(M9TransportClaim::new("session:self:S")),
        )
        .expect_err("public provider proof claims are not provider-issued attestations");

    assert_eq!(
        diagnostics.primary().kind(),
        M9AdmissionErrorKind::UnadmittedAuthProvider
    );
    assert!(!diagnostics.grants_authority());
}
