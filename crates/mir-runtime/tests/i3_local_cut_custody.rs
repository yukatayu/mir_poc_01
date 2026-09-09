//! RED contracts for the finite I3-3 process-local cut custody boundary.
//!
//! These are LOCAL cohort/bootstrap checks only. They neither start children
//! nor claim transport, persistence, distributed quiescence, a saved image,
//! or a general cut/restore mechanism. The separate probe test owns the
//! actual A requester-local cut between two ordinary checked round trips.

#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
use mir_runtime::{
    sys5_i3_process_runtime::{
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3LocalnetControlErrorKind,
        Sys5I3PrivateProcessCodec, Sys5I3ProcessCohort, Sys5I3ProcessLocalCutLaunchErrorKind,
        Sys5I3ProcessRuntimeErrorKind,
    },
    sys5_local_slice::{
        Sys5I3AdapterCarrierContract, Sys5LocalProject, Sys5SourceInput, build_project,
    },
};

#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
const CANONICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
const CANONICAL_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
const REQUESTER_SLOT: &str = "process-a";
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
const OWNER_SLOT: &str = "process-b";

#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn canonical_project() -> Sys5LocalProject {
    build_project(Sys5SourceInput::inline(
        CANONICAL_SOURCE_PATH,
        CANONICAL_SOURCE,
    ))
    .expect("the accepted ordinary four-locus source must remain checkable")
}

#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn canonical_deployment(project: &Sys5LocalProject) -> Sys5I3Deployment {
    Sys5I3Deployment::from_checked_project(
        project,
        [
            Sys5I3DeploymentSlot::new(
                REQUESTER_SLOT,
                "127.0.0.1:41301",
                ["ParticipantA", "ViewerC"],
            ),
            Sys5I3DeploymentSlot::new(
                OWNER_SLOT,
                "127.0.0.1:41302",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect("the accepted four loci map exactly once to the two ordinary process slots")
}

#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn canonical_owner_request_contract(project: &Sys5LocalProject) -> Sys5I3AdapterCarrierContract {
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.operation_id == "init_avatar_hp" && edge.kind == "owner-request")
        .expect("the accepted source retains its checked owner-request edge");
    project
        .i3_adapter_carrier_contract(&edge.edge_ref)
        .expect("the checked owner-request edge has its exact adapter contract")
}

#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn canonical_cohort(
    project: &Sys5LocalProject,
    deployment: &Sys5I3Deployment,
) -> Sys5I3ProcessCohort {
    Sys5I3ProcessCohort::from_checked_project(project, deployment)
        .expect("one ordinary source-derived cohort performs its single admitted construction")
}

#[test]
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn i3_3_local_cut_launch_rejects_b_staged_a_untagged_whole_cohort_without_consuming_stage() {
    // LOCAL factory evidence. The B lifecycle stage leaves A's ordinary image
    // untagged, so only a complete cohort-inventory check can reject it.
    let project = canonical_project();
    let deployment = canonical_deployment(&project);
    let owner_request = canonical_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let run_ref = "i3-3-cut-cohort-b-staged-a-untagged";
    let mut cohort = canonical_cohort(&project, &deployment);

    cohort
        .prestage_owner_capability_revocation(run_ref, &owner_request)
        .expect(
            "the genuine B lifecycle stage leaves the same cohort pending before cut admission",
        );
    let rejection = match cohort.take_i3_process_local_cut_launch(
        &codec,
        run_ref,
        "requester-spki:i3-3-cut-whole-cohort",
        "owner-spki:i3-3-cut-whole-cohort",
    ) {
        Ok(_) => panic!(
            "a B-staged cohort with an untagged A image must not escape through local-cut launch admission"
        ),
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessLocalCutLaunchErrorKind::StagedLifecycle,
        "whole-cohort eligibility rejects the genuine pending B lifecycle before either bootstrap is taken"
    );

    let (_requester_control, _owner_control) = cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            run_ref,
            REQUESTER_SLOT,
            "requester-spki:i3-3-cut-whole-cohort",
            OWNER_SLOT,
            "owner-spki:i3-3-cut-whole-cohort",
        )
        .expect("the rejected cut factory preserves the original same-cohort lifecycle stage and its usable controls");
}

#[test]
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn i3_3_local_cut_launch_rejects_membership_b_staged_a_untagged_whole_cohort_without_consuming_stage()
 {
    // LOCAL factory evidence. Membership staging uses a distinct pending
    // registration from capability staging, but it must be equally visible
    // to the whole-cohort cut eligibility check even while A is untagged.
    let project = canonical_project();
    let deployment = canonical_deployment(&project);
    let owner_request = canonical_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let run_ref = "i3-3-cut-cohort-membership-b-staged-a-untagged";
    let mut cohort = canonical_cohort(&project, &deployment);

    cohort
        .prestage_source_declared_owner_membership_retirement(run_ref, &owner_request)
        .expect("the genuine B membership lifecycle stage remains pending before cut admission");
    let rejection = match cohort.take_i3_process_local_cut_launch(
        &codec,
        run_ref,
        "requester-spki:i3-3-cut-membership-whole-cohort",
        "owner-spki:i3-3-cut-membership-whole-cohort",
    ) {
        Ok(_) => panic!(
            "a membership-staged B with an untagged A image must not escape through local-cut launch admission"
        ),
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3ProcessLocalCutLaunchErrorKind::StagedLifecycle,
        "whole-cohort eligibility rejects the distinct membership lifecycle before either bootstrap is taken"
    );

    let (_requester_control, _owner_control) = cohort
        .split_trusted_localnet_controls_with_prestaged_source_declared_owner_membership_lifecycle(
            &codec,
            run_ref,
            REQUESTER_SLOT,
            "requester-spki:i3-3-cut-membership-whole-cohort",
            OWNER_SLOT,
            "owner-spki:i3-3-cut-membership-whole-cohort",
        )
        .expect("the rejected cut factory preserves the same cohort's genuine pending membership controls");
}

#[test]
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn i3_3_local_cut_launch_consumes_only_a_fresh_whole_cohort_and_blocks_later_prestage() {
    // LOCAL custody evidence. This proves launch admission consumes both
    // bootstrap paths exactly once; it does not construct a child capsule or
    // claim actual QUIC execution.
    let project = canonical_project();
    let deployment = canonical_deployment(&project);
    let owner_request = canonical_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let run_ref = "i3-3-cut-fresh-whole-cohort";
    let mut cohort = canonical_cohort(&project, &deployment);
    let mut launch = cohort
        .take_i3_process_local_cut_launch(
            &codec,
            run_ref,
            "requester-spki:i3-3-cut-fresh-cohort",
            "owner-spki:i3-3-cut-fresh-cohort",
        )
        .expect("only the whole fresh ordinary A/B cohort admits one process-local-cut launch");

    let _requester_bootstrap = launch
        .take_requester_bootstrap()
        .expect("the admitted launch owns exactly one requester bootstrap");
    let _owner_bootstrap = launch
        .take_owner_bootstrap()
        .expect("the admitted launch owns exactly one owner bootstrap");
    let second_requester_take = match launch.take_requester_bootstrap() {
        Ok(_) => panic!("the consumed requester bootstrap must not be reissued"),
        Err(error) => error,
    };
    assert_eq!(
        second_requester_take.kind(),
        Sys5I3ProcessLocalCutLaunchErrorKind::BootstrapAlreadyTaken,
        "one launch cannot duplicate its requester child custody"
    );
    let second_owner_take = match launch.take_owner_bootstrap() {
        Ok(_) => panic!("the consumed owner bootstrap must not be reissued"),
        Err(error) => error,
    };
    assert_eq!(
        second_owner_take.kind(),
        Sys5I3ProcessLocalCutLaunchErrorKind::BootstrapAlreadyTaken,
        "one launch cannot duplicate its owner child custody"
    );

    let later_stage = match cohort.prestage_owner_capability_revocation(run_ref, &owner_request) {
        Ok(_) => panic!(
            "a launch that consumed the whole cohort must block any later lifecycle stage rather than admitting a mixed generation"
        ),
        Err(error) => error,
    };
    assert_eq!(
        later_stage.kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
        "post-launch lifecycle staging rejects before it can recreate a child bootstrap path"
    );
}

#[test]
#[cfg(all(feature = "i3-private-quic", feature = "i3-process-test-seams"))]
fn i3_3_cut_purpose_bootstrap_cannot_escape_through_ordinary_localnet_start() {
    // LOCAL bootstrap-purpose evidence. These frames were issued only from a
    // genuine whole-cohort cut launch; this test neither constructs control
    // frames nor starts a child process.
    let project = canonical_project();
    let deployment = canonical_deployment(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = canonical_cohort(&project, &deployment);
    let mut launch = cohort
        .take_i3_process_local_cut_launch(
            &codec,
            "i3-3-cut-purpose-ordinary-start",
            "requester-spki:i3-3-cut-purpose-ordinary-start",
            "owner-spki:i3-3-cut-purpose-ordinary-start",
        )
        .expect("the ordinary fresh whole cohort issues the paired cut bootstrap once");
    let (image_frame, control_frame) = launch
        .take_requester_bootstrap()
        .expect("the issued requester cut bootstrap remains one-use custody material")
        .into_private_child_frames()
        .expect("the issued requester bootstrap contains exactly one paired image/control frame")
        .into_image_and_trusted_control_frames();
    let image = codec
        .decode_untrusted_image(&image_frame)
        .expect("the issued image frame decodes only as an untrusted candidate");
    let control = codec
        .decode_trusted_localnet_control(&control_frame)
        .expect("the factory-issued trusted control frame remains codec-valid");

    let rejection = match codec.validate_and_start_image_with_localnet_control(image, control) {
        Ok(_) => panic!(
            "a ProcessLocalCut-purpose bootstrap must not escape through the ordinary localnet start path"
        ),
        Err(error) => error,
    };
    assert_eq!(
        rejection.kind(),
        Sys5I3LocalnetControlErrorKind::StartBindingRejected,
        "ordinary start rejects the genuine ProcessLocalCut control before runtime start"
    );
}
