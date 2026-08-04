use std::path::PathBuf;

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_model_check_auth::{
        M9AuthModelCase, M9AuthModelChecker, M9AuthModelCounterexampleKind, M9AuthModelEvidenceRef,
        M9AuthModelProperty, M9AuthModelResultKind,
    },
    surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0},
};

const SURFACE_FIXTURE_DIR: &str = "tests/fixtures/surface-v0";
const CANONICAL_FIXTURE: &str = "canonical_attack_bundle.mir";

fn load_checked_canonical() -> CheckedSurfaceV0 {
    let relative = format!("{SURFACE_FIXTURE_DIR}/{CANONICAL_FIXTURE}");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../mir-ast")
        .join(&relative);
    let source = std::fs::read_to_string(&path).expect("surface-v0 fixture is readable");
    check_and_elaborate_surface_v0(FixtureSource::new(relative, source))
        .expect("canonical M9 source fixture checks through ordinary M7")
}

#[test]
fn bounded_model_accepts_monotone_revocation_and_no_mutation_positive_pair() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-revocation-positive")
        .with_checked_surface(checked.clone())
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_property(M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_attempted_use_after_revocation("cap:attack:S:self:epoch1")
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE))
        .with_evidence_ref(M9AuthModelEvidenceRef::runtime_trace(
            "m9-auth-revocation-positive",
        ));

    let result = M9AuthModelChecker::bounded(4)
        .check(case)
        .expect("bounded positive revocation model is checked");

    assert_eq!(result.kind(), M9AuthModelResultKind::Holds);
    assert!(result.counterexample().is_none());
    assert!(result.exhaustively_explored_bounded_state_graph());
    assert!(result.max_explored_depth() <= 4);
    assert!(
        result.explored_state_count() > 1,
        "positive model check must explore a non-trivial state space"
    );
    assert!(
        result.transition_count() > 1,
        "positive model check must explore actual transitions, not only inspect fault flags"
    );
    assert_eq!(
        result.covered_properties(),
        vec![
            M9AuthModelProperty::MonotoneRevocation,
            M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload,
        ]
    );
    assert_eq!(
        result.evidence_refs(),
        vec![
            M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE),
            M9AuthModelEvidenceRef::runtime_trace("m9-auth-revocation-positive"),
        ]
    );
}

#[test]
fn bounded_model_reports_counterexample_for_revoked_replay_or_hidden_mutation() {
    let checked = load_checked_canonical();

    for (case, expected) in [
        (
            M9AuthModelCase::new("m9-auth-revoked-replay-counterexample")
                .with_checked_surface(checked.clone())
                .with_property(M9AuthModelProperty::MonotoneRevocation)
                .with_membership("membership:self:S:epoch1")
                .with_capability("cap:attack:S:self:epoch1")
                .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
                .with_revocation("revocation:cap:attack:S:self:epoch1")
                .allow_replay_after_revocation("cap:attack:S:self:epoch1")
                .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE)),
            M9AuthModelCounterexampleKind::RevokedGrantReplay,
        ),
        (
            M9AuthModelCase::new("m9-auth-hidden-m8-mutation-counterexample")
                .with_checked_surface(checked.clone())
                .with_property(M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload)
                .with_membership("membership:self:S:epoch1")
                .with_capability("cap:attack:S:self:epoch1")
                .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
                .with_rejected_use("cap:attack:S:self:epoch1")
                .allow_hidden_m8_payload_mutation()
                .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE)),
            M9AuthModelCounterexampleKind::HiddenM8PayloadMutation,
        ),
    ] {
        let result = M9AuthModelChecker::bounded(4)
            .check(case)
            .expect("bounded counterexample model is checked");
        assert_eq!(result.kind(), M9AuthModelResultKind::Counterexample);
        let counterexample = result.counterexample().expect("counterexample");
        assert_eq!(counterexample.kind(), expected);
        assert!(
            !counterexample.action_trace().is_empty(),
            "counterexample must carry a concrete action trace"
        );
        assert!(
            counterexample.action_trace().len() <= 4,
            "counterexample trace must fit inside the requested bounded depth"
        );
        assert_eq!(
            counterexample.state_trace().len(),
            counterexample.action_trace().len() + 1,
            "state trace must bracket every action edge"
        );
        let violating_edge = counterexample
            .violating_edge()
            .expect("counterexample marks the violating transition edge");
        assert_eq!(violating_edge.kind(), expected);
        match expected {
            M9AuthModelCounterexampleKind::RevokedGrantReplay => {
                assert_eq!(violating_edge.action_label(), "use_capability");
                assert_eq!(violating_edge.capability_ref(), "cap:attack:S:self:epoch1");
                assert!(
                    violating_edge
                        .pre_state()
                        .contains_revocation("revocation:cap:attack:S:self:epoch1")
                );
                assert!(
                    violating_edge
                        .post_state()
                        .accepted_capability_use("cap:attack:S:self:epoch1")
                );
            }
            M9AuthModelCounterexampleKind::HiddenM8PayloadMutation => {
                assert_eq!(violating_edge.action_label(), "reject_use");
                assert_ne!(
                    violating_edge.pre_state().m8_payload_fingerprint(),
                    violating_edge.post_state().m8_payload_fingerprint()
                );
            }
        }
        assert!(!result.claims_proof_discharge());
    }
}

#[test]
fn bound_zero_or_insufficient_depth_does_not_claim_full_coverage() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-revocation-positive")
        .with_checked_surface(checked.clone())
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_property(M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_attempted_use_after_revocation("cap:attack:S:self:epoch1")
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE));

    assert!(
        M9AuthModelChecker::bounded(0).check(case.clone()).is_err(),
        "bound 0 must not produce a coverage-bearing result"
    );

    let too_shallow = M9AuthModelChecker::bounded(2)
        .check(case)
        .expect("insufficient nonzero bound reports a non-covering model result");
    assert_ne!(
        too_shallow.covered_properties(),
        vec![
            M9AuthModelProperty::MonotoneRevocation,
            M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload,
        ],
        "insufficient depth must not claim the same coverage as depth 4"
    );
    assert!(!too_shallow.exhaustively_explored_bounded_state_graph());
    assert!(too_shallow.max_explored_depth() <= 2);
}

#[test]
fn unrelated_revocation_input_cannot_claim_target_revocation_coverage_from_depth_only() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-unrelated-revocation-input")
        .with_checked_surface(checked)
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:unrelated:B:epoch1")
        .with_attempted_use_after_revocation("cap:attack:S:self:epoch1")
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE));

    match M9AuthModelChecker::bounded(4).check(case) {
        Err(_) => {}
        Ok(result) => {
            assert!(
                result.kind() == M9AuthModelResultKind::Counterexample
                    || !result.exhaustively_explored_bounded_state_graph()
                    || result.covered_properties().is_empty(),
                "unrelated revocation input must not produce exhaustive target coverage"
            );
            if let Some(revoked) = result.state_after_action_trace(&[
                "admit_membership",
                "grant_capability",
                "revoke_capability",
            ]) {
                assert!(
                    !revoked.contains_revocation("revocation:cap:attack:S:self:epoch1"),
                    "model checker must not rewrite unrelated revocation input to the target"
                );
            }
        }
    }
}

#[test]
fn unrelated_attempted_use_input_cannot_claim_target_use_coverage_from_depth_only() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-unrelated-attempted-use-input")
        .with_checked_surface(checked)
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_attempted_use_after_revocation("cap:unrelated:B:epoch1")
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE));

    match M9AuthModelChecker::bounded(4).check(case) {
        Err(_) => {}
        Ok(result) => {
            assert!(
                result.kind() == M9AuthModelResultKind::Counterexample
                    || !result.exhaustively_explored_bounded_state_graph()
                    || result.covered_properties().is_empty(),
                "unrelated attempted-use input must not produce exhaustive target use coverage"
            );
        }
    }
}

#[test]
fn rejected_use_input_controls_hidden_mutation_counterexample_target() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-unrelated-rejected-use-input")
        .with_checked_surface(checked)
        .with_property(M9AuthModelProperty::RejectedUseDoesNotMutateM8Payload)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_rejected_use("cap:unrelated:B:epoch1")
        .allow_hidden_m8_payload_mutation()
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE));

    match M9AuthModelChecker::bounded(4).check(case) {
        Err(_) => {}
        Ok(result) => {
            if result.kind() == M9AuthModelResultKind::Counterexample {
                let edge = result
                    .counterexample()
                    .and_then(|counterexample| counterexample.violating_edge())
                    .expect("counterexample retains violating edge");
                assert_eq!(
                    edge.capability_ref(),
                    "cap:unrelated:B:epoch1",
                    "mutated rejected-use input must control the counterexample target"
                );
            }
        }
    }
}

#[test]
fn reacquire_attempt_input_controls_fresh_epoch_transition_target() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-unrelated-reacquire-input")
        .with_checked_surface(checked)
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_fresh_epoch_evidence("epoch2", "proof:membership-root:epoch2")
        .with_reacquire_attempt("cap:unrelated:B:epoch1")
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE));

    match M9AuthModelChecker::bounded(4).check(case) {
        Err(_) => {}
        Ok(result) => {
            let reacquire = result.state_after_action_trace(&[
                "admit_membership",
                "grant_capability",
                "revoke_capability",
                "reacquire_capability",
            ]);
            if let Some(reacquire) = reacquire {
                assert!(
                    !reacquire.live_capability("cap:attack:S:self:epoch2")
                        && reacquire.fresh_reacquire_evidence_epoch().is_none(),
                    "unrelated reacquire_attempt input must not refresh the target capability"
                );
            }
        }
    }
}

#[test]
fn reacquire_requires_fresh_epoch_evidence_and_never_reuses_revoked_epoch_lineage() {
    let checked = load_checked_canonical();

    let no_fresh = M9AuthModelCase::new("m9-auth-reacquire-without-fresh-evidence")
        .with_checked_surface(checked.clone())
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_reacquire_attempt("cap:attack:S:self:epoch1")
        .without_fresh_epoch_evidence()
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE));

    let no_fresh_result = M9AuthModelChecker::bounded(4)
        .check(no_fresh)
        .expect("bounded reacquire model checks without fresh evidence");

    assert_eq!(no_fresh_result.kind(), M9AuthModelResultKind::Holds);
    assert!(no_fresh_result.exhaustively_explored_bounded_state_graph());
    let no_fresh_reacquire = no_fresh_result
        .state_after_action_trace(&[
            "admit_membership",
            "grant_capability",
            "revoke_capability",
            "reacquire_capability",
        ])
        .expect("bounded model exposes the no-fresh reacquire state");
    assert!(no_fresh_reacquire.contains_revocation("revocation:cap:attack:S:self:epoch1"));
    assert!(!no_fresh_reacquire.live_capability("cap:attack:S:self:epoch1"));
    assert!(!no_fresh_reacquire.live_lineage(
        "membership:self:S:epoch1",
        "cap:attack:S:self:epoch1",
        "witness:attack:S:self:epoch1",
    ));
    assert!(!no_fresh_reacquire.accepted_capability_use("cap:attack:S:self:epoch1"));

    let fresh = M9AuthModelCase::new("m9-auth-reacquire-with-fresh-epoch2-evidence")
        .with_checked_surface(checked)
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_fresh_epoch_evidence("epoch2", "proof:membership-root:epoch2")
        .with_reacquire_attempt("cap:attack:S:self:epoch2")
        .with_expected_new_lineage(
            "membership:self:S:epoch2",
            "cap:attack:S:self:epoch2",
            "witness:attack:S:self:epoch2",
        )
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE))
        .with_evidence_ref(M9AuthModelEvidenceRef::authority_graph(
            "authority-graph:m9-auth-reacquire-epoch2",
        ));

    let fresh_result = M9AuthModelChecker::bounded(4)
        .check(fresh)
        .expect("bounded reacquire model checks with explicit fresh epoch evidence");

    assert_eq!(fresh_result.kind(), M9AuthModelResultKind::Holds);
    assert!(fresh_result.exhaustively_explored_bounded_state_graph());
    let fresh_reacquire = fresh_result
        .state_after_action_trace(&[
            "admit_membership",
            "grant_capability",
            "revoke_capability",
            "reacquire_capability",
        ])
        .expect("bounded model exposes the fresh-epoch reacquire state");
    assert_eq!(fresh_reacquire.epoch_label(), "epoch2");
    assert!(fresh_reacquire.live_lineage(
        "membership:self:S:epoch2",
        "cap:attack:S:self:epoch2",
        "witness:attack:S:self:epoch2",
    ));
    assert!(!fresh_reacquire.live_capability("cap:attack:S:self:epoch1"));
    assert!(!fresh_reacquire.live_lineage(
        "membership:self:S:epoch1",
        "cap:attack:S:self:epoch1",
        "witness:attack:S:self:epoch1",
    ));
}

#[test]
fn model_check_artifact_requires_provenance_and_invalidates_when_source_or_evidence_changes() {
    let checked = load_checked_canonical();
    let case = M9AuthModelCase::new("m9-auth-provenance")
        .with_checked_surface(checked)
        .with_property(M9AuthModelProperty::MonotoneRevocation)
        .with_membership("membership:self:S:epoch1")
        .with_capability("cap:attack:S:self:epoch1")
        .with_witness("witness:attack:S:self:epoch1", "cap:attack:S:self:epoch1")
        .with_revocation("revocation:cap:attack:S:self:epoch1")
        .with_evidence_ref(M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE))
        .with_evidence_ref(M9AuthModelEvidenceRef::authority_graph(
            "authority-graph:m9-auth-provenance",
        ));

    let artifact = M9AuthModelChecker::bounded(4)
        .check(case)
        .expect("bounded model check produces artifact")
        .into_artifact();

    assert_eq!(artifact.property(), M9AuthModelProperty::MonotoneRevocation);
    assert!(
        artifact
            .provenance()
            .contains_ref(&M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE))
    );
    assert!(
        artifact
            .provenance()
            .contains_ref(&M9AuthModelEvidenceRef::authority_graph(
                "authority-graph:m9-auth-provenance",
            ))
    );
    assert!(
        artifact
            .provenance()
            .is_invalidated_by(&M9AuthModelEvidenceRef::fixture(CANONICAL_FIXTURE))
    );
    assert!(
        artifact
            .provenance()
            .is_invalidated_by(&M9AuthModelEvidenceRef::authority_graph(
                "authority-graph:m9-auth-provenance",
            ))
    );
    assert!(!artifact.claims_static_check());
    assert!(!artifact.claims_lean_proof());
}
