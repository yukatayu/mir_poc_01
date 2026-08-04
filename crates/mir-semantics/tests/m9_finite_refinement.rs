use std::{ops::Range, path::PathBuf};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9_AUTH_REJECTED_FAILURE, M9_AUTHORITY_OBSERVATION_LABEL,
        M9_AUTHORITY_OBSERVATION_REDACTION, M9_MEMBERSHIP_AUTH_CAPABILITY,
        M9_MEMBERSHIP_AUTH_PRECONDITION, M9ContractCandidate, M9FiniteContract, M9FiniteEffectKind,
        M9FiniteRefinementChecker, M9FiniteRefinementErrorKind, M9FiniteRefinementEvidence,
        M9LeanObligationIndex, M9ObligationId, M9ProofArtifactHash, M9ProofWitnessSchema,
    },
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const CANONICAL_FIXTURE: &str = "canonical_attack_bundle.mir";
const VERIFY_NAME: &str = "finite_refinement";
const LEAN_FOUNDATION: &str = "../../samples/lean/foundations/MirTheoryV0M9AuthVerification.lean";

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

fn expected_source_ref(path: &str, source: &str, lexeme: &str) -> SourceRef {
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

fn residual_source_ref(checked: &CheckedSurfaceV0) -> SourceRef {
    checked
        .residual_obligations()
        .entries()
        .iter()
        .find(|entry| {
            entry.kind() == ResidualObligationKind::VerifyDeferred && entry.name() == VERIFY_NAME
        })
        .expect("finite refinement residual")
        .source_ref()
        .clone()
}

fn exact_evidence(checked: &CheckedSurfaceV0, verify_ref: SourceRef) -> M9FiniteRefinementEvidence {
    M9FiniteRefinementEvidence::new(M9ObligationId::new("OBL-M9-finite-refinement-Combat.M6"))
        .for_program_identity(checked.program_identity().clone())
        .for_verify_residual(VERIFY_NAME)
        .with_theorem(VERIFY_NAME)
        .with_witness_schema(M9ProofWitnessSchema::new("m9-proof-witness-required"))
        .with_source_ref(verify_ref)
        .with_artifact_hash(M9ProofArtifactHash::new(
            "sha256:m9-finite-refinement-canonical",
        ))
        .with_module_contract("Combat.M6", "finite-refinement/MembershipAuth")
}

fn copied_source_contract(source: &M9FiniteContract) -> M9FiniteContract {
    let mut copied = M9FiniteContract::default();
    for precondition in source.preconditions() {
        copied = copied.with_precondition(precondition);
    }
    for capability in source.capability_requirements() {
        copied = copied.with_capability_requirement(capability);
    }
    for failure in source.failures() {
        copied = copied.with_failure(failure);
    }
    for effect in source.effects() {
        copied = copied.with_effect(effect);
    }
    for (label, redaction) in source.observations() {
        copied = copied.with_observation(label, redaction);
    }
    copied
}

fn exact_membership_auth_contract(source: &M9FiniteContract) -> M9FiniteContract {
    copied_source_contract(source)
        .with_precondition(M9_MEMBERSHIP_AUTH_PRECONDITION)
        .with_capability_requirement(M9_MEMBERSHIP_AUTH_CAPABILITY)
        .with_failure(M9_AUTH_REJECTED_FAILURE)
        .with_observation(
            M9_AUTHORITY_OBSERVATION_LABEL,
            M9_AUTHORITY_OBSERVATION_REDACTION,
        )
}

fn exact_membership_auth_candidate(checked: &CheckedSurfaceV0) -> M9ContractCandidate {
    let base = M9ContractCandidate::from_checked_surface(checked);
    let refined = exact_membership_auth_contract(base.source_contract());
    base.with_candidate_contract(refined)
}

fn candidate_with_contract(
    checked: &CheckedSurfaceV0,
    candidate_contract: M9FiniteContract,
) -> M9ContractCandidate {
    M9ContractCandidate::from_checked_surface(checked).with_candidate_contract(candidate_contract)
}

fn source_contract_without_one_failure(source: &M9FiniteContract) -> M9FiniteContract {
    let removed = *source
        .failures()
        .first()
        .expect("canonical source has at least one source-derived failure row");
    let mut candidate = M9FiniteContract::default();
    for precondition in source.preconditions() {
        candidate = candidate.with_precondition(precondition);
    }
    for capability in source.capability_requirements() {
        candidate = candidate.with_capability_requirement(capability);
    }
    for failure in source
        .failures()
        .into_iter()
        .filter(|failure| *failure != removed)
    {
        candidate = candidate.with_failure(failure);
    }
    for effect in source.effects() {
        candidate = candidate.with_effect(effect);
    }
    for (label, redaction) in source.observations() {
        candidate = candidate.with_observation(label, redaction);
    }
    candidate
        .with_precondition(M9_MEMBERSHIP_AUTH_PRECONDITION)
        .with_capability_requirement(M9_MEMBERSHIP_AUTH_CAPABILITY)
        .with_failure(M9_AUTH_REJECTED_FAILURE)
        .with_observation(
            M9_AUTHORITY_OBSERVATION_LABEL,
            M9_AUTHORITY_OBSERVATION_REDACTION,
        )
}

fn undeclared_effect(source: &M9FiniteContract) -> M9FiniteEffectKind {
    assert!(
        !source
            .effects()
            .contains(&M9FiniteEffectKind::ExternalUndeclared),
        "external undeclared effect must never be source-derived"
    );
    M9FiniteEffectKind::ExternalUndeclared
}

#[test]
fn finite_refinement_evidence_discharges_only_exact_verify_residual() {
    let (path, source, checked) = checked_canonical();
    let verify_ref = expected_source_ref(&path, &source, "verify finite_refinement");
    assert_eq!(residual_source_ref(&checked), verify_ref);

    let candidate = exact_membership_auth_candidate(&checked);
    let source_failures = candidate.source_contract().failures();
    let source_effects = candidate.source_contract().effects();
    assert!(
        !source_failures.is_empty(),
        "positive candidate must retain real source-derived failure rows"
    );
    assert!(
        !source_effects.is_empty(),
        "positive candidate must retain real source-derived effect rows"
    );
    for failure in &source_failures {
        assert!(
            candidate.candidate_contract().failures().contains(failure),
            "candidate erased source-derived failure {failure}"
        );
    }
    assert_eq!(
        candidate.candidate_contract().effects(),
        source_effects,
        "finite refinement must not add runtime effects"
    );
    assert!(
        candidate
            .candidate_contract()
            .preconditions()
            .contains(&M9_MEMBERSHIP_AUTH_PRECONDITION)
    );
    assert!(
        candidate
            .candidate_contract()
            .capability_requirements()
            .contains(&M9_MEMBERSHIP_AUTH_CAPABILITY)
    );
    assert!(
        candidate
            .candidate_contract()
            .failures()
            .contains(&M9_AUTH_REJECTED_FAILURE)
    );
    assert!(candidate.candidate_contract().observations().contains(&(
        M9_AUTHORITY_OBSERVATION_LABEL,
        M9_AUTHORITY_OBSERVATION_REDACTION
    )));

    let discharged = M9FiniteRefinementChecker::default()
        .discharge_candidate(&checked, candidate)
        .expect("checked finite artifact discharges the verify residual");

    assert_eq!(
        discharged.obligation_id().as_str(),
        "OBL-M9-finite-refinement-Combat.M6"
    );
    assert_eq!(
        discharged.residual_kind(),
        ResidualObligationKind::VerifyDeferred
    );
    assert_eq!(discharged.residual_name(), VERIFY_NAME);
    assert_eq!(discharged.source_ref(), &verify_ref);
    assert_eq!(
        discharged.module_contract(),
        ("Combat.M6", "finite-refinement/MembershipAuth")
    );
    assert!(!discharged.grants_authority());
    assert!(!discharged.mutates_runtime_state());
    assert!(
        discharged
            .expected_delta()
            .preconditions()
            .contains(&M9_MEMBERSHIP_AUTH_PRECONDITION)
    );
    assert!(
        discharged
            .expected_delta()
            .capability_requirements()
            .contains(&M9_MEMBERSHIP_AUTH_CAPABILITY)
    );
    assert!(
        discharged
            .expected_delta()
            .failures()
            .contains(&M9_AUTH_REJECTED_FAILURE)
    );
    assert!(discharged.expected_delta().observations().contains(&(
        M9_AUTHORITY_OBSERVATION_LABEL,
        M9_AUTHORITY_OBSERVATION_REDACTION
    )));
    assert!(
        discharged
            .normalized_candidate_fingerprint()
            .contains(M9_AUTH_REJECTED_FAILURE),
        "discharge must bind the normalized candidate, not only the residual name"
    );
}

#[test]
fn finite_refinement_rejects_candidate_that_removes_source_failures() {
    let (_path, _source, checked) = checked_canonical();
    let source = M9ContractCandidate::from_checked_surface(&checked)
        .source_contract()
        .clone();
    let candidate = candidate_with_contract(&checked, source_contract_without_one_failure(&source));

    let diagnostics = M9FiniteRefinementChecker::default()
        .discharge_candidate(&checked, candidate)
        .expect_err("candidate cannot erase a source-derived failure row");

    assert_eq!(
        diagnostics.primary().kind(),
        M9FiniteRefinementErrorKind::RemovedBaselineFailure
    );
    assert!(!diagnostics.discharges_obligation());
}

#[test]
fn finite_refinement_rejects_candidate_that_adds_undeclared_effects() {
    let (_path, _source, checked) = checked_canonical();
    let source = M9ContractCandidate::from_checked_surface(&checked)
        .source_contract()
        .clone();
    let candidate_contract =
        exact_membership_auth_contract(&source).with_effect(undeclared_effect(&source));
    let candidate = candidate_with_contract(&checked, candidate_contract);

    let diagnostics = M9FiniteRefinementChecker::default()
        .discharge_candidate(&checked, candidate)
        .expect_err("finite verifier evidence cannot add undeclared runtime effects");

    assert_eq!(
        diagnostics.primary().kind(),
        M9FiniteRefinementErrorKind::EffectExpansion
    );
    assert!(!diagnostics.discharges_obligation());
}

#[test]
fn finite_refinement_rejects_candidate_that_weakens_observation_redaction() {
    let (_path, _source, checked) = checked_canonical();
    let source = M9ContractCandidate::from_checked_surface(&checked)
        .source_contract()
        .clone();
    let candidate_contract = copied_source_contract(&source)
        .with_precondition(M9_MEMBERSHIP_AUTH_PRECONDITION)
        .with_capability_requirement(M9_MEMBERSHIP_AUTH_CAPABILITY)
        .with_failure(M9_AUTH_REJECTED_FAILURE)
        .with_observation(M9_AUTHORITY_OBSERVATION_LABEL, "public");
    let candidate = candidate_with_contract(&checked, candidate_contract);

    let diagnostics = M9FiniteRefinementChecker::default()
        .discharge_candidate(&checked, candidate)
        .expect_err("candidate cannot widen or weaken the observer redaction policy");

    assert_eq!(
        diagnostics.primary().kind(),
        M9FiniteRefinementErrorKind::ObservationPolicyWeakening
    );
    assert!(!diagnostics.discharges_obligation());
}

#[test]
fn finite_refinement_rejects_candidate_missing_membership_auth_requirement_rows() {
    let (_path, _source, checked) = checked_canonical();
    let source = M9ContractCandidate::from_checked_surface(&checked)
        .source_contract()
        .clone();

    for (candidate_contract, expected) in [
        (
            copied_source_contract(&source)
                .with_capability_requirement(M9_MEMBERSHIP_AUTH_CAPABILITY)
                .with_failure(M9_AUTH_REJECTED_FAILURE)
                .with_observation(
                    M9_AUTHORITY_OBSERVATION_LABEL,
                    M9_AUTHORITY_OBSERVATION_REDACTION,
                ),
            M9FiniteRefinementErrorKind::MissingMembershipAuthPrecondition,
        ),
        (
            copied_source_contract(&source)
                .with_precondition(M9_MEMBERSHIP_AUTH_PRECONDITION)
                .with_failure(M9_AUTH_REJECTED_FAILURE)
                .with_observation(
                    M9_AUTHORITY_OBSERVATION_LABEL,
                    M9_AUTHORITY_OBSERVATION_REDACTION,
                ),
            M9FiniteRefinementErrorKind::MissingMembershipAuthCapability,
        ),
    ] {
        let diagnostics = M9FiniteRefinementChecker::default()
            .discharge_candidate(
                &checked,
                candidate_with_contract(&checked, candidate_contract),
            )
            .expect_err("MembershipAuth strengthening must carry precondition and capability rows");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert!(!diagnostics.discharges_obligation());
    }
}

#[test]
fn finite_refinement_rejects_theorem_schema_source_identity_or_replay_mismatch() {
    let (path, source, checked) = checked_canonical();
    let verify_ref = expected_source_ref(&path, &source, "verify finite_refinement");

    for (evidence, expected) in [
        (
            exact_evidence(&checked, verify_ref.clone()).with_theorem("different_theorem"),
            M9FiniteRefinementErrorKind::TheoremMismatch,
        ),
        (
            exact_evidence(&checked, verify_ref.clone())
                .with_witness_schema(M9ProofWitnessSchema::new("wrong-schema")),
            M9FiniteRefinementErrorKind::WitnessSchemaMismatch,
        ),
        (
            exact_evidence(
                &checked,
                SourceRef::new(
                    verify_ref.path.clone(),
                    verify_ref.start_line,
                    verify_ref.start_column + 1,
                    verify_ref.end_line,
                    verify_ref.end_column + 1,
                ),
            ),
            M9FiniteRefinementErrorKind::SourceRefMismatch,
        ),
        (
            exact_evidence(&checked, verify_ref.clone()).for_module("Combat.Other"),
            M9FiniteRefinementErrorKind::ProgramIdentityMismatch,
        ),
        (
            exact_evidence(&checked, verify_ref.clone())
                .with_module_contract("Combat.M6", "finite-refinement/Other"),
            M9FiniteRefinementErrorKind::ModuleContractMismatch,
        ),
        (
            exact_evidence(&checked, verify_ref.clone())
                .with_replay_source("tests/fixtures/surface-v0/other.mir"),
            M9FiniteRefinementErrorKind::ReplayedEvidence,
        ),
    ] {
        let diagnostics = M9FiniteRefinementChecker::default()
            .discharge(&checked, evidence)
            .expect_err("mismatched finite refinement evidence must reject");
        assert_eq!(diagnostics.primary().kind(), expected);
        assert!(!diagnostics.discharges_obligation());
        assert!(!diagnostics.emits_verdict());
    }
}

#[test]
fn finite_refinement_rejects_publicly_minted_hash_or_same_source_replay_evidence() {
    let (path, source, checked) = checked_canonical();
    let verify_ref = expected_source_ref(&path, &source, "verify finite_refinement");

    for evidence in [
        exact_evidence(&checked, verify_ref.clone()).with_artifact_hash(M9ProofArtifactHash::new(
            "sha256:external-caller-forged-nonempty",
        )),
        exact_evidence(&checked, verify_ref).with_replay_source(checked.source_file()),
    ] {
        let diagnostics = M9FiniteRefinementChecker::default()
            .discharge(&checked, evidence)
            .expect_err(
                "publicly minted hash or same-source replay evidence must not discharge Verify",
            );
        assert!(!diagnostics.discharges_obligation());
        assert!(!diagnostics.emits_verdict());
    }
}

#[test]
fn lean_obligation_correspondence_is_finite_profile_only_and_has_no_admitted_holes() {
    let (_path, _source, checked) = checked_canonical();
    let verify_ref = residual_source_ref(&checked);
    let discharged = M9FiniteRefinementChecker::default()
        .discharge_candidate(&checked, exact_membership_auth_candidate(&checked))
        .expect("runtime finite refinement discharge");
    assert_eq!(
        discharged.obligation_id().as_str(),
        "OBL-M9-finite-refinement-Combat.M6"
    );
    assert_eq!(discharged.source_ref(), &verify_ref);
    assert_eq!(
        discharged.finite_profile(),
        "OBL-026/verifier-evidence-non-authority"
    );
    assert_eq!(
        discharged.lean_theorem_name(),
        "verifier_evidence_cannot_mint_authority_or_activate_contract_update"
    );

    let lean = M9LeanObligationIndex::from_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(LEAN_FOUNDATION),
    )
    .expect("M9 Lean foundation index is readable");
    let theorem = lean
        .theorem_named("verifier_evidence_cannot_mint_authority_or_activate_contract_update")
        .expect("OBL-026 finite verifier theorem exists in the M9 Lean foundation");
    assert_eq!(theorem.name(), discharged.lean_theorem_name());
    assert_eq!(theorem.finite_profile(), discharged.finite_profile());
    assert!(!theorem.contains_sorry());
    assert!(!theorem.contains_admit());
    assert!(!theorem.uses_untrusted_axiom());
}
