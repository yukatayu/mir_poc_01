//! RED contracts for I3-2 process images and the pre-socket process-runtime
//! seam.  The named surface is deliberately `doc(hidden)`/provisional: this
//! file specifies only the bounded I3-2 implementation seam, never a public
//! artifact, deployment, carrier, or runtime API.

use std::collections::BTreeSet;

use mir_runtime::{
    sys5_i3_process_runtime::{
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3ProcessArtifact, Sys5I3ProcessCohort,
        Sys5I3ProcessImage, Sys5I3ProcessImageTamper, Sys5I3ProcessRuntime,
        Sys5I3ProcessRuntimeErrorKind, Sys5I3RetainedEdgeContract,
    },
    sys5_local_slice::{Sys5LocalProject, Sys5LocalSliceError, Sys5SourceInput, build_project},
};

const CANONICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const CANONICAL_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const REQUESTER_SLOT: &str = "process-a";
const OWNER_SLOT: &str = "process-b";
const OWNER_ONLY_SOURCE: &str = r#"
module Mirrorea.Sys5.I3OwnerOnly

locus WorldAuthority
locus ParticipantA
principal self
type Player

state avatar[id: Player] at WorldAuthority {
  hp: Int
  visible observer_safe fields (hp)
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

/// The only source construction in each fixture is this ordinary-source
/// build/project call.  Every deployment, image, and runtime constructor
/// below consumes that checked project; none accepts source text or a host
/// source path.
fn build_once(source_text: &str) -> Sys5LocalProject {
    build_project(Sys5SourceInput::inline(CANONICAL_SOURCE_PATH, source_text))
        .expect("the canonical finite I2 ordinary source must remain checkable")
}

fn two_nonempty_slots(project: &Sys5LocalProject) -> Sys5I3Deployment {
    Sys5I3Deployment::from_checked_project(
        project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41001",
                ["ParticipantA", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41002",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect("the four declared loci must map exactly once to two nonempty slots")
}

fn evaluator_isolated_two_slot_deployment(project: &Sys5LocalProject) -> Sys5I3Deployment {
    Sys5I3Deployment::from_checked_project(
        project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41001",
                ["ParticipantA", "ParticipantB", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(OWNER_SLOT, "127.0.0.1:41002", ["WorldAuthority"]),
        ],
    )
    .expect("two remote designated-input dependencies may isolate WorldAuthority in its own slot")
}

fn owner_only_two_slot_deployment(project: &Sys5LocalProject) -> Sys5I3Deployment {
    Sys5I3Deployment::from_checked_project(
        project,
        [
            Sys5I3DeploymentSlot::new(REQUESTER_SLOT, "127.0.0.1:41001", ["ParticipantA"]),
            Sys5I3DeploymentSlot::new(OWNER_SLOT, "127.0.0.1:41002", ["WorldAuthority"]),
        ],
    )
    .expect("the designated-free owner source must map exactly once to two nonempty slots")
}

fn two_dependency_same_operation_source() -> String {
    let with_second_dependency = CANONICAL_SOURCE.replacen(
        "state bird_pose[id: Bird] at ParticipantB {\n  x: Int\n  y: Int\n  visible observer_safe fields (x, y)\n}\n",
        "state participant_input_b[id: Player] at ParticipantB {\n  focus: Int\n  visible observer_safe fields (focus)\n}\n\nstate bird_pose[id: Bird] at ParticipantB {\n  x: Int\n  y: Int\n  visible observer_safe fields (x, y)\n}\n",
        1,
    );
    with_second_dependency.replacen(
        "participant_input[self].focus + 1",
        "participant_input[self].focus + participant_input_b[self].focus",
        1,
    )
}

fn single_coordinator_cohort(
    project: &Sys5LocalProject,
    deployment: &Sys5I3Deployment,
) -> Sys5I3ProcessCohort {
    let cohort = Sys5I3ProcessCohort::from_checked_project(project, deployment)
        .expect("one coordinator must derive the checked cohort exactly once");
    let summary = cohort.observer_safe_summary();
    assert_eq!(
        summary.full_admission_count(),
        1,
        "Candidate A must perform full admission only once for a checked process cohort"
    );
    assert_eq!(
        summary.authority_generation_count(),
        1,
        "Candidate A must generate authority only once for a checked process cohort"
    );
    cohort
}

fn take_process_image(cohort: &mut Sys5I3ProcessCohort, slot: &str) -> Sys5I3ProcessImage {
    cohort
        .take_process_image(slot)
        .expect("a declared deployment slot may be taken once as one nonduplicating child image")
}

fn canonical_image_for_slot(slot: &str) -> Sys5I3ProcessImage {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    take_process_image(&mut cohort, slot)
}

fn image_for_source_and_slot(source_text: &str, slot: &str) -> Sys5I3ProcessImage {
    let project = build_once(source_text);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    take_process_image(&mut cohort, slot)
}

fn assert_checked_image_contract(image: &Sys5I3ProcessImage) {
    assert!(
        !image.executable_artifacts().is_empty(),
        "each nonempty deployment slot must retain executable artifacts"
    );
    assert!(
        image
            .executable_artifacts()
            .iter()
            .all(|artifact: &Sys5I3ProcessArtifact| image
                .assigned_loci()
                .iter()
                .any(|locus| locus == artifact.locus())),
        "a process image may retain executable artifacts only for its assigned loci"
    );
    assert!(
        image
            .required_edge_contracts()
            .iter()
            .all(|contract: &Sys5I3RetainedEdgeContract| contract.is_reference_only()),
        "cross-process contracts retained in a process image must remain reference-only"
    );
    assert!(
        image
            .required_edge_contracts()
            .iter()
            .all(|contract: &Sys5I3RetainedEdgeContract| {
                image.assigned_loci().iter().any(|locus| {
                    locus == contract.source_locus() || locus == contract.target_locus()
                })
            }),
        "an image may retain only generated edge contracts incident to one assigned locus"
    );

    let manifest = image.observer_safe_manifest();
    assert!(
        !manifest.carries_source_text(),
        "process images must not carry ordinary source text"
    );
    assert!(
        !manifest.carries_host_path(),
        "process images must not carry a host path"
    );
    assert!(
        !manifest.carries_expected_result(),
        "process images must not carry fixture-derived expected results"
    );
}

fn assert_exact_incident_edge_inventory(project: &Sys5LocalProject, image: &Sys5I3ProcessImage) {
    let assigned = image.assigned_loci().into_iter().collect::<BTreeSet<_>>();
    let expected_all = project
        .semantic_summary()
        .generated_communication
        .iter()
        .filter(|edge| assigned.contains(&edge.from_locus) || assigned.contains(&edge.to_locus))
        .map(|edge| edge.edge_ref.clone())
        .collect::<BTreeSet<_>>();
    let actual_all = image
        .required_edge_contracts()
        .iter()
        .map(|edge| edge.edge_ref().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_all, expected_all,
        "retained edge refs must equal every checked-Core-derived edge incident to an assigned locus"
    );

    let expected_outbound = project
        .semantic_summary()
        .generated_communication
        .iter()
        .filter(|edge| assigned.contains(&edge.from_locus) && !assigned.contains(&edge.to_locus))
        .map(|edge| edge.edge_ref.clone())
        .collect::<BTreeSet<_>>();
    let actual_outbound = image
        .required_edge_contracts()
        .iter()
        .filter(|edge| {
            assigned.contains(edge.source_locus()) && !assigned.contains(edge.target_locus())
        })
        .map(|edge| edge.edge_ref().to_string())
        .collect::<BTreeSet<_>>();
    assert!(
        !expected_outbound.is_empty(),
        "the canonical two-process partition must retain at least one outbound incident edge"
    );
    assert_eq!(
        actual_outbound, expected_outbound,
        "outbound incident edge retention must be complete"
    );

    let expected_inbound = project
        .semantic_summary()
        .generated_communication
        .iter()
        .filter(|edge| !assigned.contains(&edge.from_locus) && assigned.contains(&edge.to_locus))
        .map(|edge| edge.edge_ref.clone())
        .collect::<BTreeSet<_>>();
    let actual_inbound = image
        .required_edge_contracts()
        .iter()
        .filter(|edge| {
            !assigned.contains(edge.source_locus()) && assigned.contains(edge.target_locus())
        })
        .map(|edge| edge.edge_ref().to_string())
        .collect::<BTreeSet<_>>();
    assert!(
        !expected_inbound.is_empty(),
        "the canonical two-process partition must retain at least one inbound incident edge"
    );
    assert_eq!(
        actual_inbound, expected_inbound,
        "inbound incident edge retention must be complete"
    );
}

fn assert_candidate_a_child_seed(image: &Sys5I3ProcessImage) {
    let seed = image.observer_safe_child_seed();
    assert!(
        !seed.carries_authority_publisher_or_issuer(),
        "a child process seed must not mint, publish, or issue authority"
    );
    assert!(
        !seed.carries_full_prepared_admission(),
        "a child process seed must not retain the full Sys5PreparedAdmission"
    );
    assert!(
        !seed.carries_full_fabric_program(),
        "a child process seed must not retain the full FabricProgram"
    );

    let closure = seed.required_local_authority_closure();
    assert!(
        closure.is_reference_only(),
        "the child authority closure must contain observer-safe reference-only evidence only"
    );
    assert!(
        closure.is_exact_for_image(),
        "the child authority closure must attest the exact required semantic bindings for this image"
    );
    assert!(
        closure.has_no_unassigned_semantic_rows(),
        "the child authority closure must not retain a semantic row for an unassigned locus"
    );
    assert!(
        !closure.opaque_digest_ref().is_empty(),
        "the child authority closure must expose an opaque integrity digest rather than raw authority rows"
    );
    assert!(
        !closure.opaque_cohort_ref().is_empty(),
        "the child authority closure must expose an opaque cohort reference rather than raw authority rows"
    );
}

fn assert_candidate_a_child_runtime(runtime: &Sys5I3ProcessRuntime) {
    let summary = runtime.observer_safe_runtime_summary();
    assert!(
        !summary.carries_authority_publisher_or_issuer(),
        "a child process runtime must not mint, publish, or issue authority"
    );
    assert!(
        !summary.carries_full_admission_or_fabric_program(),
        "a child process runtime must not retain the coordinator's full admission or FabricProgram"
    );
}

#[test]
fn g0_checked_project_derives_complete_two_slot_images_with_only_assigned_artifacts_and_reference_contracts()
 {
    let project = build_once(CANONICAL_SOURCE);
    assert_eq!(
        project
            .semantic_summary()
            .loci
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["ParticipantA", "ParticipantB", "ViewerC", "WorldAuthority"]),
        "the canonical checked source must retain exactly its four declared loci"
    );

    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let owner_image = take_process_image(&mut cohort, OWNER_SLOT);

    assert_eq!(
        requester_image
            .assigned_loci()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["ParticipantA", "ViewerC"]),
        "deployment may assign logical loci but may not create new loci"
    );
    assert_eq!(
        owner_image
            .assigned_loci()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["ParticipantB", "WorldAuthority"]),
        "deployment must assign every remaining declared locus exactly once"
    );

    assert_checked_image_contract(&requester_image);
    assert_checked_image_contract(&owner_image);
    assert_exact_incident_edge_inventory(&project, &requester_image);
    assert_exact_incident_edge_inventory(&project, &owner_image);
    assert_candidate_a_child_seed(&requester_image);
    assert_candidate_a_child_seed(&owner_image);

    assert_eq!(
        requester_image
            .observer_safe_child_seed()
            .parent_checked_program_ref(),
        owner_image
            .observer_safe_child_seed()
            .parent_checked_program_ref(),
        "all child images must prove one parent checked program without carrying that program"
    );
    assert_eq!(
        requester_image.observer_safe_child_seed().projection_ref(),
        owner_image.observer_safe_child_seed().projection_ref(),
        "all child images must prove one checked projection without carrying the global projection"
    );
    assert_eq!(
        requester_image
            .observer_safe_child_seed()
            .m9_generation_ref(),
        owner_image.observer_safe_child_seed().m9_generation_ref(),
        "all child images must prove one sealed M9 generation without receiving an authority issuer"
    );
    assert_eq!(
        cohort
            .take_process_image(REQUESTER_SLOT)
            .expect_err("a process image must be nonduplicating after its sole take")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken
    );

    let deployment_view = deployment.observer_safe_manifest();
    assert!(
        deployment_view.has_only_locus_slot_endpoint_assignments(),
        "deployment may map loci to slots/endpoints, but cannot supply Core, routes, authority, state, or results"
    );
}

#[test]
fn g0_deployment_rejects_missing_extra_and_duplicate_locus_assignments_before_image_derivation() {
    let project = build_once(CANONICAL_SOURCE);

    let missing = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(REQUESTER_SLOT, "127.0.0.1:41001", ["ParticipantA"]),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41002",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect_err("a missing declared locus must fail before a process image exists");
    assert_eq!(
        missing.kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingLocusAssignment
    );

    let extra = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41001",
                ["ParticipantA", "ViewerC", "UnlistedLocus"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41002",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect_err("a locus absent from checked projection must fail closed");
    assert_eq!(
        extra.kind(),
        Sys5I3ProcessRuntimeErrorKind::ExtraLocusAssignment
    );

    let duplicate = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41001",
                ["ParticipantA", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41002",
                ["WorldAuthority", "ParticipantB", "ParticipantA"],
            ),
        ],
    )
    .expect_err("one logical locus assigned to two slots must fail closed");
    assert_eq!(
        duplicate.kind(),
        Sys5I3ProcessRuntimeErrorKind::DuplicateLocusAssignment
    );

    let duplicate_within_one_slot = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41001",
                ["ParticipantA", "ParticipantA", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41002",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect_err("raw duplicate loci within one slot must survive construction until validation");
    assert_eq!(
        duplicate_within_one_slot.kind(),
        Sys5I3ProcessRuntimeErrorKind::DuplicateLocusAssignment
    );

    let one_slot = Sys5I3Deployment::from_checked_project(
        &project,
        [Sys5I3DeploymentSlot::new(
            "single-process",
            "127.0.0.1:41001",
            ["ParticipantA", "ParticipantB", "ViewerC", "WorldAuthority"],
        )],
    )
    .expect_err("I3-2 requires at least two nonempty deployment slots");
    assert_eq!(
        one_slot.kind(),
        Sys5I3ProcessRuntimeErrorKind::InsufficientDeploymentSlots
    );
}

#[test]
fn g0_process_image_tamper_consumes_the_only_image_and_rejects_before_start() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let owner_image = take_process_image(&mut cohort, OWNER_SLOT);

    let foreign_artifact = owner_image
        .executable_artifacts()
        .iter()
        .find(|artifact: &&Sys5I3ProcessArtifact| artifact.locus() == "WorldAuthority")
        .expect("the owner image must retain its generated WorldAuthority artifact")
        .clone();
    // The tamper seam consumes the sole derived image.  The original image is
    // moved here (and therefore cannot subsequently start), while the cohort
    // has no second copy to return.
    let artifact_tamper = requester_image.into_test_only_tamper(
        Sys5I3ProcessImageTamper::append_foreign_artifact(foreign_artifact),
    );
    assert_eq!(
        cohort
            .take_process_image(REQUESTER_SLOT)
            .expect_err("tampering the sole image must not leave a second image in the cohort")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken
    );
    assert_eq!(
        Sys5I3ProcessRuntime::start(artifact_tamper)
            .expect_err("a foreign executable artifact must fail before runtime start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ForeignArtifact
    );

    let integrity_tamper = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(Sys5I3ProcessImageTamper::corrupt_image_integrity());
    assert_eq!(
        Sys5I3ProcessRuntime::start(integrity_tamper)
            .expect_err("process-image integrity tamper must fail before runtime start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch
    );

    let missing_designated_requirement = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(
            Sys5I3ProcessImageTamper::remove_projected_designated_remote_input_requirement(),
        );
    assert_eq!(
        Sys5I3ProcessRuntime::start(missing_designated_requirement)
            .expect_err(
                "a designated-input request/receipt edge without its source-derived requirement must fail before start",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch
    );

    let mismatched_designated_tuple = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(
            Sys5I3ProcessImageTamper::mismatch_projected_designated_remote_input_request_receipt(),
        );
    assert_eq!(
        Sys5I3ProcessRuntime::start(mismatched_designated_tuple)
            .expect_err(
                "a designated-input request/receipt requirement tuple mismatch must fail before start",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch
    );

    let missing_restricted_owner_binding = canonical_image_for_slot(OWNER_SLOT)
        .into_test_only_tamper(
            Sys5I3ProcessImageTamper::remove_actual_restricted_owner_binding_from_private_seed(),
        );
    assert_eq!(
        Sys5I3ProcessRuntime::start(missing_restricted_owner_binding)
            .expect_err(
                "removing an actual restricted M9 owner binding must fail before runtime start or mutation",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingRequiredAuthorityEvidence
    );

    let missing_designated_lineage = canonical_image_for_slot(OWNER_SLOT).into_test_only_tamper(
        Sys5I3ProcessImageTamper::remove_actual_designated_remote_input_lineage_from_private_seed(),
    );
    assert_eq!(
        Sys5I3ProcessRuntime::start(missing_designated_lineage)
            .expect_err(
                "removing actual designated request/receipt lineage must fail before runtime start or mutation",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingRequiredAuthorityEvidence
    );

    let cohort_mismatch = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(Sys5I3ProcessImageTamper::mismatched_parent_checked_program_ref());
    assert_eq!(
        Sys5I3ProcessRuntime::start(cohort_mismatch)
            .expect_err("a parent-program cohort mismatch must fail before start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch
    );

    let projection_mismatch = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(Sys5I3ProcessImageTamper::mismatched_projection_ref());
    assert_eq!(
        Sys5I3ProcessRuntime::start(projection_mismatch)
            .expect_err("a projection cohort mismatch must fail before start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch
    );

    let generation_mismatch = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(Sys5I3ProcessImageTamper::mismatched_m9_generation_ref());
    assert_eq!(
        Sys5I3ProcessRuntime::start(generation_mismatch)
            .expect_err("an M9 generation mismatch must fail before start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch
    );

    let digest_mismatch = canonical_image_for_slot(REQUESTER_SLOT)
        .into_test_only_tamper(Sys5I3ProcessImageTamper::mismatched_authority_closure_digest());
    assert_eq!(
        Sys5I3ProcessRuntime::start(digest_mismatch)
            .expect_err("an authority-closure digest mismatch must fail before start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::AuthorityClosureDigestMismatch
    );
}

#[test]
fn g0_process_image_rejects_changed_cohort_substitution_and_duplicate_rows_even_after_local_integrity_recompute()
 {
    let changed_source =
        CANONICAL_SOURCE.replacen("avatar[self].hp = 21", "avatar[self].hp = 34", 1);
    let changed_requester_image = image_for_source_and_slot(&changed_source, REQUESTER_SLOT);

    let changed_same_locus_artifact = changed_requester_image
        .executable_artifacts()
        .iter()
        .find(|artifact: &&Sys5I3ProcessArtifact| artifact.locus() == "ParticipantA")
        .expect("the changed-source requester image must retain a ParticipantA artifact")
        .clone();
    let artifact_substitution = canonical_image_for_slot(REQUESTER_SLOT).into_test_only_tamper(
        Sys5I3ProcessImageTamper::substitute_same_locus_artifact_and_recompute_integrity(
            changed_same_locus_artifact,
        ),
    );
    assert_eq!(
        Sys5I3ProcessRuntime::start(artifact_substitution)
            .expect_err(
                "same-locus artifact substitution from a different checked cohort must fail"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageInventoryProvenanceMismatch
    );

    let canonical_incident_edge = canonical_image_for_slot(REQUESTER_SLOT)
        .required_edge_contracts()
        .first()
        .expect("the requester image must retain an incident generated edge")
        .clone();
    let changed_same_incident_edge = changed_requester_image
        .required_edge_contracts()
        .iter()
        .find(|edge: &&Sys5I3RetainedEdgeContract| {
            edge.source_locus() == canonical_incident_edge.source_locus()
                && edge.target_locus() == canonical_incident_edge.target_locus()
        })
        .expect("the changed cohort must retain the same logical incident edge shape")
        .clone();
    let edge_substitution = canonical_image_for_slot(REQUESTER_SLOT).into_test_only_tamper(
        Sys5I3ProcessImageTamper::substitute_same_incident_edge_and_recompute_integrity(
            changed_same_incident_edge,
        ),
    );
    assert_eq!(
        Sys5I3ProcessRuntime::start(edge_substitution)
            .expect_err("same-edge substitution from a different checked cohort must fail")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageInventoryProvenanceMismatch
    );

    let duplicate_artifact = canonical_image_for_slot(REQUESTER_SLOT).into_test_only_tamper(
        Sys5I3ProcessImageTamper::duplicate_artifact_row_and_recompute_integrity(),
    );
    assert_eq!(
        Sys5I3ProcessRuntime::start(duplicate_artifact)
            .expect_err("a duplicate artifact row must fail exact image inventory validation")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageInventoryProvenanceMismatch
    );

    let duplicate_edge = canonical_image_for_slot(REQUESTER_SLOT).into_test_only_tamper(
        Sys5I3ProcessImageTamper::duplicate_edge_contract_row_and_recompute_integrity(),
    );
    assert_eq!(
        Sys5I3ProcessRuntime::start(duplicate_edge)
            .expect_err("a duplicate edge row must fail exact image inventory validation")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageInventoryProvenanceMismatch
    );
}

#[test]
fn g0_evaluator_image_retains_two_exact_designated_request_receipt_pairs_for_one_operation() {
    let source = two_dependency_same_operation_source();
    let project = build_once(&source);
    let deployment = evaluator_isolated_two_slot_deployment(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let evaluator_image = take_process_image(&mut cohort, OWNER_SLOT);

    let closure = evaluator_image.observer_safe_designated_remote_input_closure();
    assert!(
        closure.is_reference_only(),
        "designated input closure must not expose dependency tuples or private values"
    );
    assert!(
        closure.is_exact_for_image(),
        "evaluator image must retain the exact source-derived designated remote-input closure"
    );
    assert!(
        closure.is_derived_from_request_receipt_edges(),
        "designated input closure must derive from paired request/receipt edges, not operation name grouping"
    );
    assert_eq!(
        closure.request_receipt_pair_count(),
        2,
        "two remote dependencies in one designated operation require two distinct request/receipt pairs"
    );
    assert_eq!(
        closure.distinct_operation_count(),
        1,
        "the fixture intentionally exercises two dependencies of one operation"
    );
    assert!(
        closure.pairs_are_distinguished_beyond_operation(),
        "the closure must not collapse distinct remote dependencies merely because their operation matches"
    );
}

#[test]
fn g0_owner_only_designated_free_source_has_a_symmetric_empty_closure_and_runs_by_value() {
    let project = build_once(OWNER_ONLY_SOURCE);
    let deployment = owner_only_two_slot_deployment(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let owner_image = take_process_image(&mut cohort, OWNER_SLOT);

    for image in [&requester_image, &owner_image] {
        let closure = image.observer_safe_designated_remote_input_closure();
        assert_eq!(
            closure.request_receipt_pair_count(),
            0,
            "a designated-free source retains no synthetic designated request/receipt pair"
        );
        assert_eq!(
            closure.distinct_operation_count(),
            0,
            "an empty designated closure must not manufacture an operation grouping"
        );
        assert!(
            closure.is_symmetric_empty_for_image(),
            "both requester and owner images must accept the same source-derived empty designated closure"
        );
    }

    let mut requester =
        Sys5I3ProcessRuntime::start(requester_image).expect("owner-only requester image starts");
    let mut owner =
        Sys5I3ProcessRuntime::start(owner_image).expect("owner-only owner image starts");
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("owner-only source still generates its remote owner request");
    let reply = owner
        .accept_inbound(request)
        .expect("owner-only owner admits by-value request")
        .expect("owner execution produces a typed reply");
    let receipt = requester
        .accept_inbound(reply)
        .expect("requester admits by-value reply")
        .expect("reply consumption produces a local receipt");
    assert!(
        receipt.has_no_transportable_carrier(),
        "the owner-only path remains a two-runtime by-value exchange, not a fabricated receipt transport"
    );
    assert_eq!(
        owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect("owner-only source preserves its source-derived owner state"),
        21
    );
}

fn run_remote_init_avatar_hp(source_text: &str) -> i64 {
    let project = build_once(source_text);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let owner_image = take_process_image(&mut cohort, OWNER_SLOT);
    let mut requester =
        Sys5I3ProcessRuntime::start(requester_image).expect("requester process image must start");
    let mut owner =
        Sys5I3ProcessRuntime::start(owner_image).expect("owner process image must start");

    assert_candidate_a_child_runtime(&requester);
    assert_candidate_a_child_runtime(&owner);

    assert_ne!(
        requester.local_store_identity_ref(),
        owner.local_store_identity_ref(),
        "independent process runtimes must not share a state store"
    );
    assert_eq!(
        requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("the requester image must have no WorldAuthority state before dispatch")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );

    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("ParticipantA may emit only its generated owner-request carrier");
    assert!(
        request.transport_binding().is_none(),
        "this G1 seam transfers a typed carrier by value before sockets; transport remains absent and non-authoritative"
    );
    let request_identity = request.semantic_request_identity_ref().to_string();
    assert_eq!(
        requester.local_authoritative_mutation_count(),
        0,
        "the requester must not mutate WorldAuthority state while emitting a request"
    );

    let reply = owner
        .accept_inbound(request)
        .expect("the owner must admit the generated request")
        .expect("owner request admission must emit one typed reply");
    assert_eq!(
        reply.linked_request_identity_ref(),
        Some(request_identity.as_str()),
        "the typed reply must retain request lineage"
    );
    assert_eq!(
        owner.local_authoritative_mutation_count(),
        1,
        "only the remote WorldAuthority runtime may execute the owner mutation"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "an accepted owner request count must be observed separately from writes"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "only an actual owner state transition may increment the owner write count"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the requester must retain no actual owner write before reply consumption"
    );

    let receipt = requester
        .accept_inbound(reply)
        .expect("requester must admit the generated owner reply")
        .expect("reply admission must emit one linked receipt");
    assert_eq!(
        receipt.linked_request_identity_ref(),
        Some(request_identity.as_str()),
        "the receipt must retain the same request lineage"
    );
    assert!(
        receipt.is_observer_safe_typed_result_or_receipt(),
        "requester reply consumption must return an observer-safe typed local result or receipt"
    );
    assert!(
        receipt.has_no_transportable_carrier(),
        "requester-local receipt completion must not fabricate a third carrier to the owner"
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        0,
        "the owner must not accept a fabricated third receipt carrier"
    );
    assert_eq!(
        requester.local_authoritative_mutation_count(),
        0,
        "receipt consumption must not make the requester an owner"
    );
    assert_eq!(
        requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err(
                "the requester image must have no owner state after local reply consumption"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );

    owner
        .authoritative_i64_state("avatar", "self", "hp")
        .expect("the owner-local state view must contain the source-derived hp write")
}

#[test]
fn g1_two_independent_runtimes_complete_remote_owner_request_reply_and_receipt_from_generated_carriers()
 {
    assert_eq!(
        run_remote_init_avatar_hp(CANONICAL_SOURCE),
        21,
        "the canonical source literal must be evaluated by WorldAuthority, not supplied by deployment"
    );
}

#[test]
fn g1_source_literal_variation_changes_owner_result_without_a_hard_coded_process_image_result() {
    let variant_source =
        CANONICAL_SOURCE.replacen("avatar[self].hp = 21", "avatar[self].hp = 34", 1);
    assert_ne!(
        variant_source, CANONICAL_SOURCE,
        "the test must alter ordinary source"
    );

    assert_eq!(run_remote_init_avatar_hp(&variant_source), 34);
    assert_ne!(
        run_remote_init_avatar_hp(CANONICAL_SOURCE),
        run_remote_init_avatar_hp(&variant_source),
        "a source-literal variation must change the owner evaluation rather than reuse a fixture result"
    );
}

fn semantic_request_and_store_identity_for_source(source_text: &str) -> (String, String) {
    let project = build_once(source_text);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let mut requester =
        Sys5I3ProcessRuntime::start(requester_image).expect("requester image must start");
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked source must derive the canonical owner request");

    let request_basis = request.observer_safe_identity_basis();
    assert!(request_basis.includes_checked_program_ref());
    assert!(request_basis.includes_projection_ref());
    assert!(request_basis.includes_cohort_ref());
    assert!(request_basis.includes_logical_origin_ref());
    assert!(request_basis.includes_ordinal());
    assert!(!request_basis.includes_process_id());
    assert!(!request_basis.includes_network_identity());

    let store_basis = requester.observer_safe_store_identity_basis();
    assert!(store_basis.includes_checked_program_ref());
    assert!(store_basis.includes_projection_ref());
    assert!(store_basis.includes_cohort_ref());
    assert!(store_basis.includes_logical_origin_ref());
    assert!(store_basis.includes_ordinal());
    assert!(!store_basis.includes_process_id());
    assert!(!store_basis.includes_network_identity());

    (
        request.semantic_request_identity_ref().to_string(),
        requester.local_store_identity_ref().to_string(),
    )
}

#[test]
fn g1_changed_source_cohorts_produce_distinct_semantic_request_and_process_store_identities() {
    let changed_source =
        CANONICAL_SOURCE.replacen("avatar[self].hp = 21", "avatar[self].hp = 34", 1);
    let (canonical_request_identity, canonical_store_identity) =
        semantic_request_and_store_identity_for_source(CANONICAL_SOURCE);
    let (changed_request_identity, changed_store_identity) =
        semantic_request_and_store_identity_for_source(&changed_source);

    assert_ne!(
        canonical_request_identity, changed_request_identity,
        "logical request identity must remain bound to checked program/projection/cohort rather than PID or network occurrence"
    );
    assert_ne!(
        canonical_store_identity, changed_store_identity,
        "process-local store identity must remain bound to checked program/projection/cohort rather than slot name alone"
    );
}

#[test]
fn g1_same_source_cohorts_have_distinct_activation_and_logical_occurrences_without_pid_or_transport_identity()
 {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);

    let mut first_cohort = single_coordinator_cohort(&project, &deployment);
    let first_summary = first_cohort.observer_safe_summary();
    let first_image = take_process_image(&mut first_cohort, REQUESTER_SLOT);
    let mut first_runtime =
        Sys5I3ProcessRuntime::start(first_image).expect("first requester process image starts");
    let first_request = first_runtime
        .emit_generated_owner_request("init_avatar_hp")
        .expect("first cohort emits its generated owner request");

    let mut second_cohort = single_coordinator_cohort(&project, &deployment);
    let second_summary = second_cohort.observer_safe_summary();
    let second_image = take_process_image(&mut second_cohort, REQUESTER_SLOT);
    let mut second_runtime =
        Sys5I3ProcessRuntime::start(second_image).expect("second requester process image starts");
    let second_request = second_runtime
        .emit_generated_owner_request("init_avatar_hp")
        .expect("second cohort emits its generated owner request");

    assert_eq!(
        first_summary.parent_checked_program_ref(),
        second_summary.parent_checked_program_ref(),
        "independent activations of the same source share the checked-program identity"
    );
    assert_eq!(
        first_summary.projection_ref(),
        second_summary.projection_ref(),
        "independent activations of the same source share the projection identity"
    );
    assert_ne!(
        first_summary.activation_occurrence_ref(),
        second_summary.activation_occurrence_ref(),
        "each activation must have a fresh occurrence reference even for identical source"
    );
    assert_ne!(
        first_summary.cohort_occurrence_ref(),
        second_summary.cohort_occurrence_ref(),
        "each derived child cohort must have a fresh cohort occurrence reference"
    );
    assert_ne!(
        first_runtime.local_store_identity_ref(),
        second_runtime.local_store_identity_ref(),
        "same-source process stores must not collide across independent cohorts"
    );
    assert_ne!(
        first_request.semantic_request_identity_ref(),
        second_request.semantic_request_identity_ref(),
        "first logical requests must remain distinct across independent cohort occurrences"
    );

    for request in [&first_request, &second_request] {
        let basis = request.observer_safe_identity_basis();
        assert!(basis.includes_checked_program_ref());
        assert!(basis.includes_projection_ref());
        assert!(basis.includes_cohort_ref());
        assert!(basis.includes_logical_origin_ref());
        assert!(basis.includes_ordinal());
        assert!(!basis.includes_process_id());
        assert!(!basis.includes_network_identity());
    }
    for runtime in [&first_runtime, &second_runtime] {
        let basis = runtime.observer_safe_store_identity_basis();
        assert!(basis.includes_checked_program_ref());
        assert!(basis.includes_projection_ref());
        assert!(basis.includes_cohort_ref());
        assert!(basis.includes_logical_origin_ref());
        assert!(basis.includes_ordinal());
        assert!(!basis.includes_process_id());
        assert!(!basis.includes_network_identity());
    }
}

#[test]
fn g1_cross_cohort_request_and_reply_are_rejected_without_state_or_receipt_minting() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);

    let mut cohort_a = single_coordinator_cohort(&project, &deployment);
    let requester_image_a = take_process_image(&mut cohort_a, REQUESTER_SLOT);
    let owner_image_a = take_process_image(&mut cohort_a, OWNER_SLOT);
    let mut requester_a =
        Sys5I3ProcessRuntime::start(requester_image_a).expect("cohort A requester starts");
    let mut owner_a = Sys5I3ProcessRuntime::start(owner_image_a).expect("cohort A owner starts");

    let mut cohort_b = single_coordinator_cohort(&project, &deployment);
    let requester_image_b = take_process_image(&mut cohort_b, REQUESTER_SLOT);
    let owner_image_b = take_process_image(&mut cohort_b, OWNER_SLOT);
    let mut requester_b =
        Sys5I3ProcessRuntime::start(requester_image_b).expect("cohort B requester starts");
    let mut owner_b = Sys5I3ProcessRuntime::start(owner_image_b).expect("cohort B owner starts");

    assert_eq!(
        owner_b
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("canonical source has no initial avatar hp in cohort B owner")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    let owner_summary_before = owner_b.observer_safe_runtime_summary();
    let owner_outbox_before = owner_b.observer_safe_outbox_summary();
    let request_from_a = requester_a
        .emit_generated_owner_request("init_avatar_hp")
        .expect("cohort A emits its normal generated request");

    assert_eq!(
        owner_b
            .accept_inbound(request_from_a)
            .expect_err("a cohort A carrier must not admit into cohort B's owner runtime")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
        "cohort occurrence is a private provenance/admission namespace, never authority or an M9 validation substitute"
    );
    assert_eq!(
        owner_b
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("cross-cohort rejection must not materialize B owner avatar state")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert_eq!(
        owner_b
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        owner_summary_before.served_owner_request_count(),
        "a foreign-cohort request must not count as a served owner request"
    );
    assert_eq!(
        owner_b
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        owner_summary_before.actual_owner_write_count(),
        "a foreign-cohort request must not count as an actual owner write"
    );
    assert_eq!(
        owner_b
            .observer_safe_outbox_summary()
            .pending_carrier_count(),
        owner_outbox_before.pending_carrier_count(),
        "a rejected cross-cohort request must not mint a B-owner reply carrier"
    );
    assert_eq!(
        owner_b
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        owner_summary_before.accepted_inbound_receipt_count(),
        "a rejected cross-cohort request must not mint or accept a receipt"
    );

    let normal_reply_from_a = owner_a
        .accept_inbound(
            requester_a
                .emit_generated_owner_request("init_avatar_hp")
                .expect("cohort A generates a second normal request for its own owner"),
        )
        .expect("cohort A owner admits cohort A request")
        .expect("cohort A owner returns a normal typed reply");
    let requester_summary_before = requester_b.observer_safe_runtime_summary();
    let requester_outbox_before = requester_b.observer_safe_outbox_summary();
    assert_eq!(
        requester_b
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("cohort B requester does not own the WorldAuthority state before rejection")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );

    assert_eq!(
        requester_b
            .accept_inbound(normal_reply_from_a)
            .expect_err("a cohort A reply must not admit into cohort B's requester runtime")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
        "reply provenance must bind the same private cohort namespace as its source request"
    );
    assert_eq!(
        requester_b
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("cross-cohort reply rejection must not install owner state in B requester")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert_eq!(
        requester_b
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        requester_summary_before.actual_owner_write_count(),
        "a rejected cross-cohort reply must not become an owner write"
    );
    assert_eq!(
        requester_b
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        requester_summary_before.accepted_inbound_receipt_count(),
        "a rejected cross-cohort reply must not mint a local receipt"
    );
    assert_eq!(
        requester_b
            .observer_safe_outbox_summary()
            .pending_carrier_count(),
        requester_outbox_before.pending_carrier_count(),
        "a rejected cross-cohort reply must leave no pending carrier in B requester"
    );
}

#[test]
fn g1_rejected_outbound_extraction_preserves_the_pending_generated_carrier_without_mutation() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let mut requester =
        Sys5I3ProcessRuntime::start(requester_image).expect("requester image must start");
    let outbox_before = requester.observer_safe_outbox_summary();
    assert_eq!(
        outbox_before.pending_carrier_count(),
        0,
        "the fresh requester runtime must start with an empty outbox"
    );

    // The injection mechanism is intentionally not part of the I3-2 contract;
    // this test fixes only the failed-extraction preservation property.
    requester.test_only_reject_next_outbound_extraction();
    assert_eq!(
        requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect_err("a deliberately rejected extraction must be typed")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OutboundExtractionRejected
    );

    let outbox_after = requester.observer_safe_outbox_summary();
    assert_eq!(
        outbox_after.pending_carrier_count(),
        1,
        "failed extraction must leave the newly generated outbound carrier pending in the outbox"
    );
    assert!(
        outbox_after.contains_generated_owner_request("init_avatar_hp"),
        "failed extraction must preserve the exact generated owner-request carrier"
    );
    assert_eq!(
        requester.local_authoritative_mutation_count(),
        0,
        "failed outbound extraction must not mutate semantic owner state"
    );
}

#[test]
fn g0_absent_designated_trigger_frontier_is_a_typed_rejection_not_an_empty_frontier_value() {
    let no_trigger_frontier_source = CANONICAL_SOURCE.replacen(
        "designated evaluate WorldAuthority on tick world_tick",
        "designated evaluate WorldAuthority",
        1,
    );
    assert!(
        matches!(
            build_project(Sys5SourceInput::inline(
                CANONICAL_SOURCE_PATH,
                no_trigger_frontier_source,
            )),
            Err(Sys5LocalSliceError::SurfaceCheckFailed { .. })
        ),
        "an absent designated trigger frontier must be typed as source failure, never represented as an empty valid frontier"
    );
}

#[test]
fn g1_nonowner_serve_is_rejected_without_requester_owner_state_or_mutation() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let mut requester =
        Sys5I3ProcessRuntime::start(requester_image).expect("requester image must start");
    assert_eq!(
        requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("requester must not have owner state before a rejected serve")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the source-derived request must be available for the negative serve attempt");
    let mutation_count_before = requester.local_authoritative_mutation_count();

    assert_eq!(
        requester
            .attempt_owner_serve(&request)
            .expect_err("ParticipantA must not serve a WorldAuthority request")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::NonOwnerServe
    );
    assert_eq!(
        requester.local_authoritative_mutation_count(),
        mutation_count_before,
        "a rejected non-owner serve must be non-mutating"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "a rejected non-owner serve attempt must not count as a served owner request"
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "a rejected non-owner serve attempt must not count as an owner write"
    );
    assert_eq!(
        requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("requester must not acquire owner state after a rejected serve")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
}
