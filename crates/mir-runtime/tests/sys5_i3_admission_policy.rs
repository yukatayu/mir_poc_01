//! Existing finite-admission policy coverage for I3-3's auth/policy failure
//! matrix row. These cases remain local SYS-5/M9 boundary evidence: they do
//! not claim dynamic policy revocation, external authentication, networking,
//! or a new authority provider.

use mir_runtime::sys5_local_slice::{
    Sys5LocalAdmissionError, Sys5LocalAdmissionErrorKind, Sys5LocalAdmissionRequest,
    Sys5LocalProject, Sys5LocalRuntimeProfile, Sys5RelationBootstrapPolicy, Sys5SourceInput,
    build_project,
};

const ACCEPTED_I2_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const ACCEPTED_I2_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

fn accepted_i2_project() -> Sys5LocalProject {
    build_project(Sys5SourceInput::inline(
        ACCEPTED_I2_SOURCE_PATH,
        ACCEPTED_I2_SOURCE,
    ))
    .expect("the accepted ordinary I2 source checks and projects before finite policy admission")
}

/// This names only the exact non-secret source-declared identity rows and
/// residual labels already checked by the accepted I2 source. It never
/// supplies Core, authority, a route, state, capability, witness, or result.
fn canonical_admission_request(auth_discharge: Option<&str>) -> Sys5LocalAdmissionRequest {
    let request = Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-i3-admission-policy-world",
        "incarnation:self:WorldAuthority:epoch:sys5-i3-admission-policy-world",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "self",
        "ParticipantA",
        "epoch:sys5-i3-admission-policy-a",
        "incarnation:self:ParticipantA:epoch:sys5-i3-admission-policy-a",
    )
    .with_source_declared_membership(
        "self",
        "ParticipantB",
        "epoch:sys5-i3-admission-policy-b",
        "incarnation:self:ParticipantB:epoch:sys5-i3-admission-policy-b",
    )
    .with_source_declared_membership(
        "self",
        "ViewerC",
        "epoch:sys5-i3-admission-policy-c",
        "incarnation:self:ViewerC:epoch:sys5-i3-admission-policy-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_optional_verification_discharge("finite_refinement");
    match auth_discharge {
        Some(name) => request.with_auth_discharge(name),
        None => request,
    }
}

fn assert_pre_issuance_policy_rejection(
    error: Sys5LocalAdmissionError,
    expected_kind: Sys5LocalAdmissionErrorKind,
) {
    assert_eq!(error.kind(), expected_kind);
    assert!(
        error.rejected_before_authority_issuance(),
        "a missing or mismatched source auth discharge must fail before M9 issues authority"
    );
    assert!(
        error.partial_admission().is_none(),
        "policy rejection exposes no prepared admission handle"
    );
    assert!(
        error.rejected_before_live_runtime(),
        "policy rejection occurs before any live local runtime can exist"
    );
    assert!(
        error.partial_runtime().is_none(),
        "policy rejection exposes no partial runtime handle"
    );
}

fn assert_genuine_admission_still_starts(project: &Sys5LocalProject) {
    let prepared = project
        .prepare_finite_admission(canonical_admission_request(Some("MembershipAuth")))
        .expect("the unchanged accepted source remains genuinely admissible after a rejected policy request");
    assert!(
        prepared
            .observer_safe_admission_summary()
            .is_source_derived(),
        "the succeeding prepared admission remains source-derived"
    );
    let _runtime = prepared
        .start_vertical_slice_runtime()
        .expect("only the subsequent genuine admission starts the existing bounded local runtime");
}

#[test]
fn i3_admission_policy_missing_auth_discharge_rejects_before_m9_and_preserves_accepted_project() {
    let project = accepted_i2_project();
    let project_identity_before = project.checked_program_identity_ref().to_string();
    let semantic_summary_before = project.semantic_summary().clone();
    let observer_view_before = project.observer_safe_view().clone();

    let baseline = project
        .prepare_finite_admission(canonical_admission_request(Some("MembershipAuth")))
        .expect(
            "the complete public request is the accepted source-derived finite-admission control",
        );
    assert_eq!(
        baseline
            .observer_safe_admission_summary()
            .checked_program_identity_ref(),
        project_identity_before
    );

    let error = project
        .prepare_finite_admission(canonical_admission_request(None))
        .expect_err("omitting only the actual auth discharge must fail before M9 issuance");
    assert_pre_issuance_policy_rejection(error, Sys5LocalAdmissionErrorKind::MissingAuthDischarge);
    assert_eq!(
        project.checked_program_identity_ref(),
        project_identity_before
    );
    assert_eq!(project.semantic_summary(), &semantic_summary_before);
    assert_eq!(project.observer_safe_view(), &observer_view_before);

    assert_genuine_admission_still_starts(&project);
}

#[test]
fn i3_admission_policy_unknown_auth_discharge_rejects_before_m9_and_preserves_accepted_project() {
    let project = accepted_i2_project();
    let project_identity_before = project.checked_program_identity_ref().to_string();
    let semantic_summary_before = project.semantic_summary().clone();
    let observer_view_before = project.observer_safe_view().clone();

    let baseline = project
        .prepare_finite_admission(canonical_admission_request(Some("MembershipAuth")))
        .expect(
            "the complete public request is the accepted source-derived finite-admission control",
        );
    assert_eq!(
        baseline
            .observer_safe_admission_summary()
            .checked_program_identity_ref(),
        project_identity_before
    );

    let error = project
        .prepare_finite_admission(canonical_admission_request(Some("UnknownAuthDischarge")))
        .expect_err("mismatching only the actual auth discharge must fail before M9 issuance");
    assert_pre_issuance_policy_rejection(error, Sys5LocalAdmissionErrorKind::UnknownAuthDischarge);
    assert_eq!(
        project.checked_program_identity_ref(),
        project_identity_before
    );
    assert_eq!(project.semantic_summary(), &semantic_summary_before);
    assert_eq!(project.observer_safe_view(), &observer_view_before);

    assert_genuine_admission_still_starts(&project);
}
