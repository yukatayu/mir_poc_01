use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0};

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

fn checked_two_owner_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        FOUR_LOCUS_SOURCE_PATH,
        FOUR_LOCUS_SOURCE,
    ))
    .expect("the two-owner fixture must pass the real source/check pipeline")
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
