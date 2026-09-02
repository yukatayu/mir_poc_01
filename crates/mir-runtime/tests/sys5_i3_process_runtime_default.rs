//! Default-feature regression floor for the I3-2 process-image runtime.
//!
//! The legacy decoded-carrier falsifiers intentionally require the isolated
//! `i3-process-test-seams` feature.  This target must nevertheless compile
//! and execute in the default feature set, exercising only the normal
//! source-first image/start/outbound path.  It cannot construct the opaque
//! authenticated-ingress token and never calls the test-only decoded ingress.

#![allow(unused_crate_dependencies)]

use mir_runtime::{
    sys5_i3_process_runtime::{
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3PrivateProcessCodec, Sys5I3ProcessCohort,
    },
    sys5_local_slice::{Sys5SourceInput, build_project},
};

const ACTIVE_I2_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const ACTIVE_I2_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

#[test]
fn default_features_start_a_source_derived_image_and_emit_only_a_generated_owner_request() {
    let project = build_project(Sys5SourceInput::inline(
        ACTIVE_I2_SOURCE_PATH,
        ACTIVE_I2_SOURCE,
    ))
    .expect("the accepted ordinary I2 source remains buildable without raw ingress test seams");
    let deployment = Sys5I3Deployment::from_checked_project(
        &project,
        [
            Sys5I3DeploymentSlot::new("process-a", "127.0.0.1:0", ["ParticipantA", "ViewerC"]),
            Sys5I3DeploymentSlot::new(
                "process-b",
                "127.0.0.1:0",
                ["WorldAuthority", "ParticipantB"],
            ),
        ],
    )
    .expect("the finite deployment is only an exact locus-to-slot map");
    let mut cohort = Sys5I3ProcessCohort::from_checked_project(&project, &deployment)
        .expect("source-derived cohort admission must remain normal-build available");
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let binding = cohort
        .parent_held_expected_start_binding("process-a")
        .expect("the supervisor retains the exact start binding before image consumption");
    let image = cohort
        .take_process_image("process-a")
        .expect("the assigned process image is consumed once");
    let encoded = codec
        .encode_image(image)
        .expect("the private image codec accepts the source-derived image");
    let decoded = codec
        .decode_untrusted_image(&encoded)
        .expect("image bytes remain untrusted until the retained binding validates them");
    let mut runtime = codec
        .validate_and_start_image(decoded, binding)
        .expect("the normal build starts only an image agreeing with its retained binding");
    let request = runtime
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the started requester emits only the generated source operation");
    let request_bytes = codec
        .encode_outbound_message(request)
        .expect("the generated request remains codec-transportable in the normal build");

    assert!(!request_bytes.is_empty());
    assert_eq!(
        runtime
            .observer_safe_outbox_summary()
            .pending_carrier_count(),
        0,
        "successful by-value extraction transfers the generated carrier out of the local outbox; a later remote reply creates the requester-local pending/reply linkage"
    );
}
