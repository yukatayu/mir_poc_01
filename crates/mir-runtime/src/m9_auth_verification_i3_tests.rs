use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

use crate::sys3_projection::{DeclaredLogicalTopology, project_checked_core};

use super::*;

const FOUR_LOCUS_SOURCE_PATH: &str =
    "tests/fixtures/surface-v0/sys4_two_owner_four_locus_with_auth.mir";
const FOUR_LOCUS_SOURCE: &str =
    include_str!("../../mir-ast/tests/fixtures/surface-v0/sys4_two_owner_four_locus_with_auth.mir");
const FIRST_OPERATION: &str = "attack_s";
const FIRST_OWNER_LOCUS: &str = "S";
const SECOND_OPERATION: &str = "attack_t";
const SECOND_OWNER_LOCUS: &str = "T";
const OWNER_PRINCIPAL: &str = "self";
const I3_OWNER_MEMBERSHIP_SUCCESSOR_SOURCE_PATH: &str =
    "tests/inline/i3_owner_membership_successor.mir";
const I3_OWNER_MEMBERSHIP_SUCCESSOR_SOURCE: &str = r#"module Mirrorea.I3OwnerMembershipSuccessor

locus WorldAuthority
locus ParticipantA
principal self
type Player

state avatar[id: Player] at WorldAuthority {
  hp: Int
}

Role[self] at ParticipantA {
  when init_avatar_hp() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[self].hp = 21
    }
  }
}

with auth MembershipAuth

verify finite_refinement
"#;
const I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION: &str = "init_avatar_hp";
const I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS: &str = "WorldAuthority";
const I3_ACCEPTED_FOUR_LOCUS_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const I3_ACCEPTED_FOUR_LOCUS_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

fn checked_two_owner_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        FOUR_LOCUS_SOURCE_PATH,
        FOUR_LOCUS_SOURCE,
    ))
    .expect("the two-owner fixture must pass the real source/check pipeline")
}

fn checked_i3_owner_membership_successor_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        I3_OWNER_MEMBERSHIP_SUCCESSOR_SOURCE_PATH,
        I3_OWNER_MEMBERSHIP_SUCCESSOR_SOURCE,
    ))
    .expect("the checked A-to-WorldAuthority owner source must admit before its M9 successor test")
}

fn admitted_i3_accepted_four_locus_finite_local_seam() -> M9RuntimeExecutionSeam {
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        I3_ACCEPTED_FOUR_LOCUS_SOURCE_PATH,
        I3_ACCEPTED_FOUR_LOCUS_SOURCE,
    ))
    .expect("the accepted four-locus source must check before finite-local M9 admission");
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("the accepted source has exactly its four declared loci");
    let projection = project_checked_core(&checked, &topology)
        .expect("the accepted four-locus source has an exact checked Core projection");
    let candidate = M9FiniteLocalAdmissionCandidate::from_checked(
        &checked,
        &projection,
        vec![
            M9FiniteLocalAdmissionFact::anchor_membership(
                OWNER_PRINCIPAL,
                I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
                "epoch:i3-row13-world-authority",
                "incarnation:self:WorldAuthority:epoch:i3-row13-world-authority",
            ),
            M9FiniteLocalAdmissionFact::source_declared_membership(
                OWNER_PRINCIPAL,
                "ParticipantA",
                "epoch:i3-row13-participant-a",
                "incarnation:self:ParticipantA:epoch:i3-row13-participant-a",
            ),
            M9FiniteLocalAdmissionFact::source_declared_membership(
                OWNER_PRINCIPAL,
                "ParticipantB",
                "epoch:i3-row13-participant-b",
                "incarnation:self:ParticipantB:epoch:i3-row13-participant-b",
            ),
            M9FiniteLocalAdmissionFact::source_declared_membership(
                OWNER_PRINCIPAL,
                "ViewerC",
                "epoch:i3-row13-viewer-c",
                "incarnation:self:ViewerC:epoch:i3-row13-viewer-c",
            ),
            M9FiniteLocalAdmissionFact::relation_bootstrap_fresh_at_admission("bird_follow"),
            M9FiniteLocalAdmissionFact::auth_discharge("MembershipAuth"),
            M9FiniteLocalAdmissionFact::optional_verification_discharge("finite_refinement"),
        ],
    )
    .expect("the accepted four-locus facts form one validated finite-local M9 candidate");
    M9RuntimeExecutionSeam::admit_validated_finite_local_candidate(candidate)
        .expect("the finite-local candidate admits through the real M9 execution seam")
}

#[cfg(feature = "i3-process-test-seams")]
fn assert_m9_maps_unchanged(before: &M9AuthorityGeneration, after: &M9AuthorityGeneration) {
    assert_eq!(after.program_identity, before.program_identity);
    assert_eq!(after.generation, before.generation);
    assert_eq!(after.generation_ref, before.generation_ref);
    assert_eq!(after.owner_uses, before.owner_uses);
    assert_eq!(after.relation_uses, before.relation_uses);
    assert_eq!(
        after.fresh_relation_reacquire_bindings,
        before.fresh_relation_reacquire_bindings
    );
    assert_eq!(
        after.designated_evaluation_uses,
        before.designated_evaluation_uses
    );
    assert_eq!(
        after.designated_consumption_uses,
        before.designated_consumption_uses
    );
    assert_eq!(after.kernel_owner_lineages, before.kernel_owner_lineages);
    assert_eq!(
        after.revoked_owner_capabilities,
        before.revoked_owner_capabilities
    );
    assert_eq!(
        after.revoked_designated_consumption_capabilities,
        before.revoked_designated_consumption_capabilities
    );
    assert_eq!(
        after.kernel_designated_remote_input_lineages,
        before.kernel_designated_remote_input_lineages
    );
    assert_eq!(
        after.designated_consumer_failures,
        before.designated_consumer_failures
    );
    assert_eq!(
        after.designated_consumer_witness_retirements,
        before.designated_consumer_witness_retirements
    );
    assert_eq!(
        after.designated_source_release_failures,
        before.designated_source_release_failures
    );
    assert_eq!(
        after.designated_consumer_validation_occurrences,
        before.designated_consumer_validation_occurrences
    );
    assert_eq!(
        after.owner_operation_validation_occurrences,
        before.owner_operation_validation_occurrences
    );
    assert_eq!(
        after.source_release_validation_occurrences,
        before.source_release_validation_occurrences
    );
}

#[test]
fn i3_3_exact_owner_capability_successor_preserves_a_genuine_prior_owner_tombstone() {
    let checked = checked_two_owner_source();
    let seam = M9RuntimeExecutionSeam::test_real_admitted_multi_owner_seam_for_kernel(
        &checked,
        [
            (FIRST_OPERATION, OWNER_PRINCIPAL, FIRST_OWNER_LOCUS),
            (SECOND_OPERATION, OWNER_PRINCIPAL, SECOND_OWNER_LOCUS),
        ],
    )
    .expect("the source-derived two-owner fixture admits through the real M9 seam");
    let mut publisher = seam
        .into_authority_successor_publisher()
        .expect("the admitted M9 seam retains its own genuine successor publisher");
    let g1 = publisher.current_generation_for_restore();
    let g2 = publisher
        .revoke_owner_capability(FIRST_OPERATION, OWNER_PRINCIPAL, FIRST_OWNER_LOCUS)
        .expect("the retained publisher performs the first genuine owner-capability revocation");
    assert!(g2.is_exact_owner_capability_revocation_successor_of(
        &g1,
        FIRST_OPERATION,
        FIRST_OWNER_LOCUS,
    ));

    let mut g3 = publisher
        .revoke_owner_capability(SECOND_OPERATION, OWNER_PRINCIPAL, SECOND_OWNER_LOCUS)
        .expect(
            "the same retained publisher performs the second genuine owner-capability revocation",
        );
    assert!(g3.is_exact_owner_capability_revocation_successor_of(
        &g2,
        SECOND_OPERATION,
        SECOND_OWNER_LOCUS,
    ));
    assert_eq!(
        g3.owner_uses, g2.owner_uses,
        "the genuine G2-to-G3 successor retains every earlier logical owner use, including already revoked S"
    );
    assert_eq!(
        g3.kernel_owner_lineages, g2.kernel_owner_lineages,
        "the genuine G2-to-G3 successor retains every earlier logical owner lineage, including already revoked S"
    );
    let observations_before_pure_revalidation = g3.runtime_validation_observation_snapshot();
    assert_eq!(
        g3.revalidate_owner_operation_without_observation(FIRST_OPERATION, FIRST_OWNER_LOCUS),
        Err(M9OwnerOperationRevalidationFailure::MissingCapability),
        "G3 keeps S's prior logical binding long enough to report its genuine current M9 capability withdrawal"
    );
    assert_eq!(
        g3.revalidate_owner_operation_without_observation(SECOND_OPERATION, SECOND_OWNER_LOCUS),
        Err(M9OwnerOperationRevalidationFailure::MissingCapability),
        "G3 reports its selected T capability withdrawal through the same pure current-generation path"
    );
    assert_eq!(
        g3.runtime_validation_observation_snapshot(),
        observations_before_pure_revalidation,
        "pure revalidation of either revoked owner must not mint an M9 validation observation"
    );

    #[cfg(feature = "i3-process-test-seams")]
    {
        let (_, first_owner_use) = g1
            .owner_authority_for_operation(FIRST_OPERATION, FIRST_OWNER_LOCUS)
            .expect("the genuine G1 snapshot retains the original S authority use only for negative restoration checks");

        let mut missing_prior_owner_binding = g3.clone();
        assert!(
            missing_prior_owner_binding
                .test_only_remove_existing_revoked_owner_binding_for_i3_exact_delta_falsifier(
                    FIRST_OPERATION,
                    FIRST_OWNER_LOCUS,
                ),
            "the narrow falsifier removes only G3's existing already-revoked S use and lineage pair"
        );
        assert!(
            !missing_prior_owner_binding.is_exact_owner_capability_revocation_successor_of(
                &g2,
                SECOND_OPERATION,
                SECOND_OWNER_LOCUS,
            ),
            "removing an earlier revoked logical owner-map pair invalidates the G2-to-G3 exact successor"
        );

        let mut restored_earlier_capability = g3.clone();
        assert!(
            restored_earlier_capability
                .authority_state
                .test_only_restore_owner_capability_from_prior_for_i3(
                    &g1.authority_state,
                    first_owner_use.capability_ref(),
                    first_owner_use.witness_ref(),
                ),
            "the feature-only falsifier restores only the genuine G1 S capability/witness material"
        );
        assert!(
            !restored_earlier_capability.is_exact_owner_capability_revocation_successor_of(
                &g2,
                SECOND_OPERATION,
                SECOND_OWNER_LOCUS,
            ),
            "an earlier revoked S capability restored from the genuine G1 snapshot invalidates the G2-to-G3 exact delta"
        );

        let mut restored_earlier_witness = g3.clone();
        assert!(
            restored_earlier_witness
                .authority_state
                .test_only_restore_owner_witness_from_prior_for_i3(
                    &g1.authority_state,
                    first_owner_use.capability_ref(),
                    first_owner_use.witness_ref(),
                ),
            "the feature-only falsifier restores only the genuine G1 S witness material"
        );
        assert!(
            !restored_earlier_witness.is_exact_owner_capability_revocation_successor_of(
                &g2,
                SECOND_OPERATION,
                SECOND_OWNER_LOCUS,
            ),
            "an earlier revoked S witness restored from the genuine G1 snapshot invalidates the G2-to-G3 exact delta"
        );
    }

    assert!(
        g3.revoked_owner_capabilities.remove(&(
            FIRST_OPERATION.to_string(),
            OWNER_PRINCIPAL.to_string(),
            FIRST_OWNER_LOCUS.to_string(),
        )),
        "the genuine G3 baseline must retain G2's prior owner tombstone before this private predicate falsifier removes it"
    );
    assert!(
        !g3.is_exact_owner_capability_revocation_successor_of(
            &g2,
            SECOND_OPERATION,
            SECOND_OWNER_LOCUS,
        ),
        "a G3 candidate that removes G2's real prior tombstone cannot be an exact owner-capability revocation successor"
    );
}

#[test]
fn i3_3_source_declared_owner_membership_successor_makes_the_retained_g1_use_stale() {
    let checked = checked_i3_owner_membership_successor_source();
    let seam = M9RuntimeExecutionSeam::test_real_admitted_owner_seam_for_kernel(
        &checked,
        I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
        OWNER_PRINCIPAL,
        I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
    )
    .expect("the checked A-to-WorldAuthority owner operation admits through the real M9 seam");
    let publisher = seam
        .into_authority_successor_publisher()
        .expect("the real admitted owner seam retains its source-derived successor publisher");
    let g1 = publisher.current_generation_for_restore();
    let (_, prior_use) = g1
        .owner_authority_for_operation(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        )
        .expect("G1 retains the actual issued owner use for the checked owner operation");

    assert!(
        g1.authority_state()
            .validate_owner_use(
                prior_use.principal(),
                prior_use.membership_ref(),
                prior_use.capability_ref(),
                prior_use.witness_ref(),
                I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
                I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            )
            .is_ok(),
        "the retained G1 use is current before the source-declared membership successor is staged"
    );

    let staged = publisher
        .prestage_exact_source_declared_owner_membership_retirement(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        )
        .expect("M9 alone resolves and stages the exact WorldAuthority membership retirement");
    assert!(
        staged.prior_generation().matches_for_restore(&g1),
        "the staged lifecycle candidate retains the exact genuine G1 generation rather than a cap alias"
    );
    let g2 = staged.successor_generation();
    let observations_before = g2.runtime_validation_observation_snapshot();

    assert!(
        matches!(
            g2.authority_state().validate_owner_use(
                prior_use.principal(),
                prior_use.membership_ref(),
                prior_use.capability_ref(),
                prior_use.witness_ref(),
                I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
                I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            ),
            Err(M8AuthorityValidationFailure::StaleMembership)
        ),
        "the genuine G2 membership successor must reject the retained G1 owner use as stale membership"
    );
    assert!(
        g2.runtime_validation_observation_snapshot() == observations_before,
        "direct M8 stale-membership validation must not mint an M9 validation observation"
    );
}

#[test]
fn i3_3_accepted_four_locus_membership_successor_preserves_unretired_fresh_relation_binding() {
    // LOCAL M9/M8 evidence only. This exercises the accepted four-locus
    // finite-local admission path; it does not construct a process image,
    // carrier, ACK, or network delivery.
    let seam = admitted_i3_accepted_four_locus_finite_local_seam();
    let publisher = seam
        .into_authority_successor_publisher()
        .expect("the admitted finite-local seam retains its source-derived M9 publisher");
    let g1 = publisher.current_generation_for_restore();
    let (_, prior_use) = g1
        .owner_authority_for_operation(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        )
        .expect("accepted G1 retains the actual WorldAuthority owner use");
    assert!(
        !g1.fresh_relation_reacquire_bindings.is_empty()
            && g1
                .fresh_relation_reacquire_bindings
                .contains_key("bird_follow"),
        "the accepted finite-local G1 retains a genuine unretired fresh relation binding"
    );
    assert!(
        g1.authority_state()
            .validate_owner_use(
                prior_use.principal(),
                prior_use.membership_ref(),
                prior_use.capability_ref(),
                prior_use.witness_ref(),
                I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
                I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            )
            .is_ok(),
        "the selected G1 owner use is current before its exact membership retirement"
    );

    let staged = publisher
        .prestage_exact_source_declared_owner_membership_retirement(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        )
        .expect("M9 stages the exact source-declared WorldAuthority membership successor");
    let mut expected_fresh_bindings = g1.fresh_relation_reacquire_bindings.clone();
    for relation in &staged.retired_relation_bindings {
        assert!(
            expected_fresh_bindings.remove(relation).is_some(),
            "a selected retired relation must have been genuinely present in the G1 finite-local binding inventory"
        );
    }
    let g2 = staged.successor_generation();
    assert!(
        g2.fresh_relation_reacquire_bindings == expected_fresh_bindings
            && g2.fresh_relation_reacquire_bindings.get("bird_follow")
                == g1.fresh_relation_reacquire_bindings.get("bird_follow"),
        "G2 preserves every unretired fresh relation binding and removes no relation beyond the exact selected retirement"
    );

    let observations_before = g2.runtime_validation_observation_snapshot();
    assert!(
        matches!(
            g2.authority_state().validate_owner_use(
                prior_use.principal(),
                prior_use.membership_ref(),
                prior_use.capability_ref(),
                prior_use.witness_ref(),
                I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
                I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            ),
            Err(M8AuthorityValidationFailure::StaleMembership)
        ),
        "the selected retired G1 owner membership remains stale in G2 even while unrelated fresh relation bindings survive"
    );
    assert!(
        g2.runtime_validation_observation_snapshot() == observations_before,
        "pure local stale-membership validation cannot mint an M9 validation observation"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_source_declared_membership_successor_rejects_loss_of_an_unretired_fresh_relation_binding() {
    // LOCAL M9 exact-delta evidence only. The negative seam starts with the
    // genuine source-derived G2 candidate and can remove only one actual
    // unretired binding; it cannot construct membership or authority facts.
    let seam = admitted_i3_accepted_four_locus_finite_local_seam();
    let publisher = seam
        .into_authority_successor_publisher()
        .expect("the accepted finite-local seam retains its source-derived M9 publisher");
    let mut staged = publisher
        .prestage_exact_source_declared_owner_membership_retirement(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        )
        .expect("M9 first stages the unmodified exact source-declared membership successor");

    assert!(
        staged.remains_exact_source_declared_owner_membership_retirement_for(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        ),
        "the unmodified source-derived G2 satisfies the complete membership-only exact delta"
    );
    assert!(
        staged.test_only_remove_one_unretired_fresh_relation_binding_for_i3_exact_delta_falsifier(),
        "the genuine G2 contains one unretired fresh relation binding available to the bounded falsifier"
    );
    assert!(
        !staged.remains_exact_source_declared_owner_membership_retirement_for(
            I3_OWNER_MEMBERSHIP_SUCCESSOR_OPERATION,
            I3_OWNER_MEMBERSHIP_SUCCESSOR_LOCUS,
        ),
        "dropping one genuine unretired fresh binding must invalidate the full membership-successor exact delta"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_exact_owner_capability_successor_rejects_an_unrelated_m8_membership_delta() {
    let checked = checked_two_owner_source();
    let seam = M9RuntimeExecutionSeam::test_real_admitted_multi_owner_seam_for_kernel(
        &checked,
        [
            (FIRST_OPERATION, OWNER_PRINCIPAL, FIRST_OWNER_LOCUS),
            (SECOND_OPERATION, OWNER_PRINCIPAL, SECOND_OWNER_LOCUS),
        ],
    )
    .expect("the two real owner uses admit together through one M9 seam");
    let mut publisher = seam
        .into_authority_successor_publisher()
        .expect("the admitted seam retains its own successor publisher");
    let _g1 = publisher.current_generation_for_restore();
    let g2 = publisher
        .revoke_owner_capability(FIRST_OPERATION, OWNER_PRINCIPAL, FIRST_OWNER_LOCUS)
        .expect("the first genuine revocation gives G2 a real prior owner tombstone");
    let mut g3 = publisher
        .revoke_owner_capability(SECOND_OPERATION, OWNER_PRINCIPAL, SECOND_OWNER_LOCUS)
        .expect("the second genuine revocation yields the exact G2-to-G3 baseline");
    assert!(g3.is_exact_owner_capability_revocation_successor_of(
        &g2,
        SECOND_OPERATION,
        SECOND_OWNER_LOCUS,
    ));

    let before_m8_mutation = g3.clone();
    assert!(
        g3.test_only_remove_unrelated_owner_membership_for_i3_exact_delta_falsifier(
            SECOND_OPERATION,
            SECOND_OWNER_LOCUS,
            FIRST_OPERATION,
            FIRST_OWNER_LOCUS,
        ),
        "the test-only falsifier removes one real G2 membership unrelated to G3's selected owner-capability withdrawal"
    );
    assert_m9_maps_unchanged(&before_m8_mutation, &g3);
    assert!(
        !g3.is_exact_owner_capability_revocation_successor_of(
            &g2,
            SECOND_OPERATION,
            SECOND_OWNER_LOCUS,
        ),
        "unchanged M9 maps cannot hide an unrelated M8 membership delta from the exact successor predicate"
    );
}
