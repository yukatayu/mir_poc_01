use std::{collections::BTreeSet, sync::mpsc, time::Duration};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    m9_finite_refinement::{
        M9CompositeContractCandidate, M9CompositeFiniteRefinementChecker,
        M9ReadOnlyProviderEffectCoverage,
    },
    surface_v0_pipeline::{CheckedSurfaceV0, check_and_elaborate_surface_v0},
};

use crate::{
    i3_read_only_provider_composite::{
        I3InactiveReadOnlyProviderComposite, I3ProviderCompositeErrorKind,
        I3ReadOnlyProviderCompositeCandidate, I3TrustedReadOnlyProviderFixtureSetup,
    },
    m8_runtime_admission::{M8I3PrivateSnapshot, M8I3PrivateSnapshotError, M8RuntimeInstance},
    sys3_projection::{
        DeclaredLogicalTopology, project_read_only_provider_effect_static,
        verify_read_only_provider_effect_static_projection,
    },
    sys5_i3_process_runtime::{
        Sys5I3DeploymentSlot, Sys5I3InactiveProviderCohort, Sys5I3InactiveProviderImageTamper,
        Sys5I3InactiveProviderImageValidation, Sys5I3PrivateProcessCodec, Sys5I3ProcessRuntime,
        Sys5I3ProcessRuntimeError, Sys5I3ProcessRuntimeErrorKind, Sys5I3UntrustedProcessImage,
    },
};

const PROVIDER_EFFECT_SOURCE_PATH: &str =
    "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir";
const PROVIDER_EFFECT_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir");
const A_SLOT: &str = "provider-a";
const B_SLOT: &str = "provider-b";

fn checked_provider_effect_source() -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        PROVIDER_EFFECT_SOURCE_PATH,
        PROVIDER_EFFECT_SOURCE,
    ))
    .expect("the genuine provider source checks before its Stage 2c handoff")
}

fn checked_provider_effect_source_with_distinct_operation() -> CheckedSurfaceV0 {
    let source = PROVIDER_EFFECT_SOURCE.replacen(
        "effect sample_value by",
        "effect sample_value_foreign_identity by",
        1,
    );
    check_and_elaborate_surface_v0(FixtureSource::new(
        "samples/clean-near-end/mirrorea-i3-provider-effect/foreign-identity.mir",
        source,
    ))
    .expect("a distinct declared provider operation remains a valid checked mixed source")
}

fn canonical_static_plan(
    checked: &CheckedSurfaceV0,
) -> crate::sys3_projection::ReadOnlyProviderEffectStaticProjection {
    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
    )
    .expect("the genuine provider source retains all four declared loci");
    let coverage = M9ReadOnlyProviderEffectCoverage::from_checked(checked)
        .expect("the checked provider source derives exact static coverage");
    let static_plan = project_read_only_provider_effect_static(checked, &topology, coverage)
        .expect("the dedicated static projector accepts the genuine provider source");
    verify_read_only_provider_effect_static_projection(checked, &topology, &static_plan)
        .expect("the static plan retains exact checked provider coverage");
    static_plan
}

fn sealed_provider_composite() -> (
    I3TrustedReadOnlyProviderFixtureSetup,
    I3InactiveReadOnlyProviderComposite,
    CheckedSurfaceV0,
) {
    let checked = checked_provider_effect_source();
    let static_plan = canonical_static_plan(&checked);
    let composite_discharge = M9CompositeFiniteRefinementChecker::default()
        .discharge_composite_candidate(
            &checked,
            M9CompositeContractCandidate::from_checked_surface(&checked)
                .expect("the checked provider source derives its composite candidate")
                .membership_auth_strengthening(),
        )
        .expect("the genuine provider source passes the finite composite verifier");
    let trusted_setup = I3TrustedReadOnlyProviderFixtureSetup::provision_default()
        .expect("trusted setup provisions the actual declared provider resource context");
    let bootstrap = trusted_setup
        .bootstrap_facts(&checked, &static_plan)
        .expect("T0 derives actual bootstrap facts from the checked static plan");
    let binding = trusted_setup
        .admit_resource_binding(&checked, &static_plan)
        .expect(
            "trusted setup binds the declared provider slot to its actual resource incarnation",
        );
    let verified = I3ReadOnlyProviderCompositeCandidate::from_trusted_inputs(
        checked.clone(),
        static_plan,
        Some(bootstrap),
        Some(composite_discharge),
        Some(binding),
    )
    .expect("the genuine checked source and actual trusted context form one composite candidate")
    .verify_membership_and_discharge()
    .expect("the actual M9 membership and finite discharge verify before policy sealing");
    let policy = trusted_setup
        .decide_fixed_policy(&verified)
        .expect("the separate provider policy accepts the verified composite");
    let inactive = verified
        .seal_with_policy(Some(policy))
        .expect("the actual policy seal yields only an inactive scoped component");

    (trusted_setup, inactive, checked)
}

fn provider_slots() -> [Sys5I3DeploymentSlot; 2] {
    [
        Sys5I3DeploymentSlot::new(A_SLOT, "127.0.0.1:41501", ["ParticipantA", "ViewerC"]),
        Sys5I3DeploymentSlot::new(
            B_SLOT,
            "127.0.0.1:41502",
            ["WorldAuthority", "ParticipantB"],
        ),
    ]
}

fn decode_inactive_provider_image(
    cohort: &mut Sys5I3InactiveProviderCohort,
    slot_name: &str,
    case: &str,
) -> Sys5I3UntrustedProcessImage {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    codec
        .decode_untrusted_image(
            &codec
                .encode_image(
                    cohort
                        .take_process_image(slot_name)
                        .unwrap_or_else(|error| {
                            panic!("{case}: exact inactive image must be available: {error:?}")
                        }),
                )
                .unwrap_or_else(|error| panic!("{case}: private image must encode: {error:?}")),
        )
        .unwrap_or_else(|error| {
            panic!("{case}: private codec must decode only an untrusted candidate: {error:?}")
        })
}

fn encode_inactive_provider_image(
    cohort: &mut Sys5I3InactiveProviderCohort,
    slot_name: &str,
    case: &str,
) -> Vec<u8> {
    Sys5I3PrivateProcessCodec::private_provisional_v1()
        .encode_image(
            cohort
                .take_process_image(slot_name)
                .unwrap_or_else(|error| {
                    panic!("{case}: exact inactive image must be available: {error:?}")
                }),
        )
        .unwrap_or_else(|error| panic!("{case}: private image must encode: {error:?}"))
}

fn assert_inactive_provider_image_rejected(
    result: Result<Sys5I3InactiveProviderImageValidation, Sys5I3ProcessRuntimeError>,
    expected_kind: Sys5I3ProcessRuntimeErrorKind,
    case: &str,
) {
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!("{case}: must reject the tainted inactive-image candidate"),
    };
    assert_eq!(
        error.kind(),
        expected_kind,
        "{case}: must fail closed with its typed inactive-image rejection",
    );
}

fn assert_inactive_image_validation(
    validation: &Sys5I3InactiveProviderImageValidation,
    checked: &CheckedSurfaceV0,
    expected_loci: &[&str],
    case: &str,
) {
    assert!(
        validation.activation_pending(),
        "{case}: activation remains pending"
    );
    assert!(
        !validation.provider_runtime_active(),
        "{case}: Stage 2c validation must not activate a provider runtime",
    );
    assert_eq!(
        validation.provider_call_count(),
        0,
        "{case}: inactive validation exposes no provider runtime call at this structural boundary",
    );
    assert!(
        validation.binding_was_current_at_validation(),
        "{case}: the actual admitted resource binding was current at validation",
    );
    assert!(
        validation.matches_checked_program_identity(checked),
        "{case}: the image retains the complete checked program identity",
    );
    assert!(
        validation.has_exact_provider_static_coverage(),
        "{case}: the image retains exact checked provider roles and generated edges",
    );
    assert_eq!(
        validation.declared_provider_lowering_count(),
        4,
        "{case}: exactly the four provider lowerings remain declared and excluded",
    );
    assert!(
        validation.retains_exact_non_provider_ordered_lowering_associations(checked),
        "{case}: exclusion preserves the remaining ordinal/source/Core lowering associations",
    );
    assert!(
        validation.has_retained_composite_seal_association_at_validation(),
        "{case}: validation retained the sealed discharge, M9 authority facts, policy, and binding association at validation",
    );
    assert!(
        validation.has_exact_translated_legacy_m9_authority_inventory(),
        "{case}: child validation retains the genuine translated legacy M9 authority inventory",
    );
    assert!(
        validation.matches_m9_execution_restriction_for_assigned_loci(),
        "{case}: retained legacy authority satisfies the existing assigned-locus M9 restriction",
    );
    assert!(
        validation.retains_assigned_provider_static_descriptors_only(),
        "{case}: executable provider descriptors remain restricted to the assigned child loci",
    );
    assert_eq!(
        validation
            .assigned_loci()
            .into_iter()
            .collect::<BTreeSet<_>>(),
        expected_loci
            .iter()
            .map(|locus| (*locus).to_string())
            .collect::<BTreeSet<_>>(),
        "{case}: validation retains the exact assigned child locus scope",
    );
}

#[test]
fn i3_provider_composite_validates_exact_all_locus_inactive_images_after_codec_roundtrip() {
    let (_trusted_setup, inactive, checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("inactive handoff should build exact A/B provider images");
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();

    let a_expected = cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains A's separately trusted expected-start binding");
    let a_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(
                    cohort
                        .take_process_image(A_SLOT)
                        .expect("A's exact inactive image is consumed once"),
                )
                .expect("A's private image encodes only as tainted child bytes"),
        )
        .expect("A's codec output restores only an untrusted structural image candidate");
    let a_validation = cohort
        .validate_inactive_untrusted_image(a_image, a_expected)
        .expect("A's separately trusted expected-start binding validates its exact inactive image");
    assert_inactive_image_validation(
        &a_validation,
        &checked,
        &["ParticipantA", "ViewerC"],
        "A inactive provider image",
    );

    let b_expected = cohort
        .parent_held_expected_start_binding(B_SLOT)
        .expect("the parent retains B's separately trusted expected-start binding");
    let b_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(
                    cohort
                        .take_process_image(B_SLOT)
                        .expect("B's exact inactive image is consumed once"),
                )
                .expect("B's private image encodes only as tainted child bytes"),
        )
        .expect("B's codec output restores only an untrusted structural image candidate");
    let b_validation = cohort
        .validate_inactive_untrusted_image(b_image, b_expected)
        .expect("B's separately trusted expected-start binding validates its exact inactive image");
    assert_inactive_image_validation(
        &b_validation,
        &checked,
        &["ParticipantB", "WorldAuthority"],
        "B inactive provider image",
    );
}

#[test]
fn i3_provider_composite_ordinary_process_start_is_rejected_before_local_fabric() {
    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("inactive handoff should build the ordinary-start escape candidate");
    let image = cohort
        .take_process_image(A_SLOT)
        .expect("the exact A image is consumed once by the ordinary-start negative");

    let error = Sys5I3ProcessRuntime::start(image).expect_err(
        "ordinary SYS5 start must reject a provider-scoped image before LocalFabric bootstrap",
    );
    assert_eq!(
        error.kind(),
        Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
        "ordinary SYS5 start must retain its existing typed pre-bootstrap rejection",
    );
}

#[test]
fn i3_provider_composite_decoded_image_cannot_use_generic_start_promotion() {
    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine composite must produce an A image for the generic-start negative");
    let expected = cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains the genuine A expected-start binding");
    let candidate = decode_inactive_provider_image(
        &mut cohort,
        A_SLOT,
        "decoded generic start-promotion negative",
    );

    let result = Sys5I3PrivateProcessCodec::private_provisional_v1()
        .validate_and_start_image(candidate, expected);
    let error = match result {
        Err(error) => error,
        Ok(_) => {
            panic!("a decoded provider image must not use the generic codec start-promotion path")
        }
    };
    assert_eq!(
        error.kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
        "generic codec start promotion rejects an inactive provider image before runtime creation",
    );
}

#[test]
fn i3_provider_composite_validation_does_not_accept_a_different_checked_identity() {
    let (_trusted_setup, inactive, checked) = sealed_provider_composite();
    let foreign_checked = checked_provider_effect_source_with_distinct_operation();
    assert_ne!(
        checked.program_identity().stable_key(),
        foreign_checked.program_identity().stable_key(),
        "the foreign source edit must produce a distinct checked program identity",
    );
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine composite must produce its inactive image before query validation");
    let expected = cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains the genuine A expected-start binding");
    let image = decode_inactive_provider_image(
        &mut cohort,
        A_SLOT,
        "foreign checked identity validation query",
    );
    let validation = cohort
        .validate_inactive_untrusted_image(image, expected)
        .expect("the genuine image validates against its own parent-held binding");

    assert!(
        validation.matches_checked_program_identity(&checked),
        "the validation recognizes the exact checked source that produced the image",
    );
    assert!(
        validation.retains_exact_non_provider_ordered_lowering_associations(&checked),
        "the validation recognizes the exact checked source for retained lowering associations",
    );
    assert!(
        !validation.matches_checked_program_identity(&foreign_checked),
        "a valid but distinct checked source cannot satisfy the image identity query",
    );
    assert!(
        !validation.retains_exact_non_provider_ordered_lowering_associations(&foreign_checked),
        "a valid but distinct checked source cannot satisfy the retained lowering association query",
    );
}

#[test]
fn i3_provider_composite_codec_component_cannot_restore_as_an_ordinary_m8_instance() {
    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine composite must produce a canonical inactive image");
    let encoded = encode_inactive_provider_image(
        &mut cohort,
        A_SLOT,
        "ordinary M8 restoration from extracted provider component",
    );
    assert!(
        encoded.len() >= 4,
        "the canonical private codec frame retains its fixed length prefix",
    );
    let body_length = u32::from_be_bytes(
        encoded[..4]
            .try_into()
            .expect("the fixed image frame prefix has four bytes"),
    ) as usize;
    assert_eq!(
        body_length,
        encoded.len() - 4,
        "the canonical image frame body is exact before this test inspects it",
    );
    let image_json: serde_json::Value = serde_json::from_slice(&encoded[4..])
        .expect("the actual codec body remains syntactically valid private JSON");
    let component_json = image_json
        .get("component_snapshot")
        .and_then(|snapshot| snapshot.get("component"))
        .cloned()
        .expect("the canonical inactive provider image retains its M8 component DTO");
    let ordinary_candidate = serde_json::from_value::<M8I3PrivateSnapshot>(component_json.clone());

    if let Ok(snapshot) = ordinary_candidate {
        assert!(
            matches!(
                M8RuntimeInstance::from_i3_private_snapshot(snapshot),
                Err(M8I3PrivateSnapshotError::StructuralMismatch)
            ),
            "extracting a provider component DTO from canonical image bytes must not restore an ordinary M8 runtime",
        );
    }

    for (case, replacement_scope) in [
        ("missing provider component scope", None),
        (
            "unknown provider component scope",
            Some("unknown_read_only_provider_component_scope"),
        ),
    ] {
        let mut malformed_component = component_json.clone();
        let object = malformed_component
            .as_object_mut()
            .expect("the canonical M8 component DTO is a JSON object");
        match replacement_scope {
            None => {
                object.remove("scope");
            }
            Some(scope) => {
                object.insert(
                    "scope".to_string(),
                    serde_json::Value::String(scope.to_string()),
                );
            }
        }
        assert!(
            serde_json::from_value::<M8I3PrivateSnapshot>(malformed_component).is_err(),
            "{case}: a provider component scope must never deserialize as an ordinary M8 snapshot",
        );
    }
}

#[test]
fn i3_provider_composite_rejects_single_slot_full_locus_assignment_before_image_handoff() {
    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let result = inactive.into_inactive_process_cohort([Sys5I3DeploymentSlot::new(
        "provider-all-loci",
        "127.0.0.1:41503",
        ["ParticipantA", "ViewerC", "WorldAuthority", "ParticipantB"],
    )]);
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!(
            "one child assigned every provider locus must not produce an inactive process image"
        ),
    };
    assert_eq!(
        error.kind(),
        I3ProviderCompositeErrorKind::InactiveProcessHandoffRejected,
        "the private facade rejects a one-slot/full-locus deployment before any image handoff",
    );
}

#[test]
fn i3_provider_composite_expected_start_binding_debug_redacts_private_component_dtos() {
    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine composite must produce the parent-held A start binding");
    let expected = cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains the genuine A expected-start binding");
    let debug = format!("{expected:?}");

    for private_dto_member in [
        "component_snapshot",
        "legacy_authority_snapshot",
        "ordered_lowering",
    ] {
        assert!(
            !debug.contains(private_dto_member),
            "expected-start Debug must redact private provider component DTO members",
        );
    }
}

#[test]
fn i3_provider_composite_rejects_same_source_foreign_setup_expected_start_binding() {
    let (_trusted_setup_a, inactive_a, _checked_a) = sealed_provider_composite();
    let (_trusted_setup_b, inactive_b, _checked_b) = sealed_provider_composite();
    let mut cohort_a = inactive_a
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine A setup must produce its own parent-held inactive handoff");
    let mut cohort_b = inactive_b
        .into_inactive_process_cohort(provider_slots())
        .expect("the same source under B's distinct setup must produce its own handoff");

    let foreign_expected = cohort_b
        .parent_held_expected_start_binding(A_SLOT)
        .expect("B retains a genuine but foreign parent-held A expected-start binding");
    let local_image = decode_inactive_provider_image(
        &mut cohort_a,
        A_SLOT,
        "same-source foreign setup expected-start binding",
    );

    assert_inactive_provider_image_rejected(
        cohort_a.validate_inactive_untrusted_image(local_image, foreign_expected),
        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
        "same source cannot splice an expected-start binding from another actual setup",
    );
}

#[test]
fn i3_provider_composite_rejects_retired_setup_or_effect_before_untrusted_handoff_validation() {
    let (mut trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut setup_cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine setup must create an inactive handoff before retirement");
    let setup_expected = setup_cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains A's expected-start binding before setup retirement");
    let setup_image =
        decode_inactive_provider_image(&mut setup_cohort, A_SLOT, "retired setup inactive image");
    trusted_setup.retire();
    assert_inactive_provider_image_rejected(
        setup_cohort.validate_inactive_untrusted_image(setup_image, setup_expected),
        Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
        "retiring the actual setup invalidates a decoded inactive image before validation",
    );

    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut effect_cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine effect authority must create an inactive handoff before retirement");
    let effect_expected = effect_cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains A's expected-start binding before effect retirement");
    let effect_image = decode_inactive_provider_image(
        &mut effect_cohort,
        A_SLOT,
        "retired effect authorization inactive image",
    );
    effect_cohort
        .test_only_retire_effect_authorization()
        .expect("the actual parent-held effect authorization retires once");
    assert_inactive_provider_image_rejected(
        effect_cohort.validate_inactive_untrusted_image(effect_image, effect_expected),
        Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
        "retiring the separate effect authorization invalidates before image validation",
    );
}

#[test]
fn i3_provider_composite_rejects_provider_static_or_legacy_lineage_tampering_after_codec_decode() {
    for (case, tamper) in [
        (
            "changed provider role descriptor",
            Sys5I3InactiveProviderImageTamper::ChangeProviderRoleDescriptor,
        ),
        (
            "changed provider edge descriptor",
            Sys5I3InactiveProviderImageTamper::ChangeProviderEdgeDescriptor,
        ),
        (
            "changed retained non-provider M8 lowering association",
            Sys5I3InactiveProviderImageTamper::ChangeRetainedLoweringAssociation,
        ),
        (
            "removed excluded provider source/Core/ordinal lowering association",
            Sys5I3InactiveProviderImageTamper::RemoveExcludedProviderLoweringAssociation,
        ),
        (
            "removed required translated legacy M9 owner or designated lineage",
            Sys5I3InactiveProviderImageTamper::RemoveRequiredLegacyAuthorityLineage,
        ),
    ] {
        let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
        let mut cohort = inactive
            .into_inactive_process_cohort(provider_slots())
            .expect("the genuine composite must produce a source-derived inactive image");
        let expected = cohort
            .parent_held_expected_start_binding(A_SLOT)
            .expect("the parent retains the genuine A expected-start binding");
        let tampered_image = decode_inactive_provider_image(&mut cohort, A_SLOT, case)
            .into_test_only_inactive_provider_tamper(tamper);

        assert_inactive_provider_image_rejected(
            cohort.validate_inactive_untrusted_image(tampered_image, expected),
            Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            case,
        );
    }
}

#[test]
fn i3_provider_composite_rejects_stripped_or_widened_child_scope_after_codec_decode() {
    for (case, tamper) in [
        (
            "stripped assigned locus",
            Sys5I3InactiveProviderImageTamper::StripAssignedLocus,
        ),
        (
            "widened assigned locus",
            Sys5I3InactiveProviderImageTamper::WidenAssignedLocus,
        ),
    ] {
        let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
        let mut cohort = inactive
            .into_inactive_process_cohort(provider_slots())
            .expect("the genuine composite must produce a source-derived inactive image");
        let expected = cohort
            .parent_held_expected_start_binding(A_SLOT)
            .expect("the parent retains the genuine A expected-start binding");
        let tampered_image = decode_inactive_provider_image(&mut cohort, A_SLOT, case)
            .into_test_only_inactive_provider_tamper(tamper);

        assert_inactive_provider_image_rejected(
            cohort.validate_inactive_untrusted_image(tampered_image, expected),
            Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            case,
        );
    }
}

#[test]
fn i3_provider_composite_rejects_endpoint_metadata_tampering_after_codec_decode() {
    let (_trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect("the genuine composite must produce a source-derived inactive image");
    let expected = cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains the genuine A expected-start binding");
    let tampered_image = decode_inactive_provider_image(
        &mut cohort,
        A_SLOT,
        "changed inactive image endpoint metadata",
    )
    .into_test_only_inactive_provider_tamper(
        Sys5I3InactiveProviderImageTamper::ChangeEndpointMetadata,
    );

    assert_inactive_provider_image_rejected(
        cohort.validate_inactive_untrusted_image(tampered_image, expected),
        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
        "refreshing only the candidate integrity cannot authorize changed endpoint metadata",
    );
}

#[test]
fn i3_provider_composite_reloads_actual_setup_currentness_after_structural_validation() {
    let (mut trusted_setup, inactive, _checked) = sealed_provider_composite();
    let mut cohort = inactive
        .into_inactive_process_cohort(provider_slots())
        .expect(
            "the genuine composite must produce an inactive image before concurrent retirement",
        );
    let expected = cohort
        .parent_held_expected_start_binding(A_SLOT)
        .expect("the parent retains the genuine A expected-start binding");
    let image = decode_inactive_provider_image(
        &mut cohort,
        A_SLOT,
        "concurrent actual setup retirement after structural validation",
    );
    let (entered, structural_validation) = mpsc::sync_channel(1);
    let (resume, validated_currentness) = mpsc::sync_channel(1);
    cohort.test_only_pause_after_inactive_structural_validation(entered, validated_currentness);

    let validation =
        std::thread::spawn(move || cohort.validate_inactive_untrusted_image(image, expected));
    structural_validation
        .recv_timeout(Duration::from_secs(5))
        .expect("validation must reach its bounded post-structural synchronization point");
    trusted_setup.retire();
    resume
        .send(())
        .expect("the paused validation must accept its bounded resume signal");

    let result = validation
        .join()
        .expect("the validation thread must not panic while the real setup retires");
    assert_inactive_provider_image_rejected(
        result,
        Sys5I3ProcessRuntimeErrorKind::InactiveProviderBindingNotCurrent,
        "the final receipt currentness reload rejects a real setup retirement after structural validation",
    );
}
