//! Bounded physical-write tests for the private provider child-control frame.
//!
//! These use only synthetic nonsecret bytes and a local Unix stream. They do
//! not create a launch, control, provider authority, or provider result.

use std::{
    io::{Cursor, ErrorKind, Read, Write},
    os::unix::net::UnixStream,
    thread,
    time::{Duration, Instant},
};

#[cfg(feature = "i3-process-test-seams")]
use serde_json::Value;

use super::*;

fn prefill_until_would_block(writer: &mut UnixStream) -> usize {
    let mut filled = 0;
    for _ in 0..1024 {
        match writer.write(&[0xa5; 8192]) {
            Ok(0) => panic!("a nonblocking provider-control writer must not report a zero write"),
            Ok(written) => filled += written,
            Err(error) if error.kind() == ErrorKind::WouldBlock => return filled,
            Err(error) => panic!("prefilling the local nonblocking stream must succeed: {error}"),
        }
    }
    panic!("the bounded local Unix stream must eventually apply backpressure");
}

#[test]
fn provider_control_writer_finishes_exactly_one_frame_after_would_block() {
    let (mut writer, mut reader) =
        UnixStream::pair().expect("a local Unix pair provides the physical control writer path");
    writer
        .set_nonblocking(true)
        .expect("the physical writer can be made nonblocking");
    let prefilling_len = prefill_until_would_block(&mut writer);
    assert!(
        prefilling_len > 0,
        "the writer must have buffered real bytes before backpressure"
    );
    reader
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("the draining peer has a finite regression timeout");

    let reader_task = thread::spawn(move || {
        let mut observed = Vec::new();
        reader
            .read_to_end(&mut observed)
            .expect("the peer drains until the one writer closes");
        observed
    });
    let prefix = b"provider-control-prefix";
    let body = b"provider-control-body";
    let write_result = write_provider_control_frame_until(
        &mut writer,
        prefix,
        body,
        Instant::now() + Duration::from_secs(1),
    );
    drop(writer);

    let observed = reader_task
        .join()
        .expect("the physical control reader task completes");
    write_result.expect("a draining peer lets the one bounded provider-control frame complete");
    let mut expected_frame = prefix.to_vec();
    expected_frame.extend(body);
    assert!(
        observed.len() >= prefilling_len,
        "the peer retains every prefilled byte before the control frame"
    );
    assert_eq!(
        &observed[prefilling_len..],
        expected_frame.as_slice(),
        "backpressure recovery writes one prefix/body frame, with no replay or appended frame"
    );
}

#[test]
fn provider_control_writer_rejects_an_expired_deadline_without_blocking() {
    let (mut writer, _reader) =
        UnixStream::pair().expect("a local Unix pair provides the physical control writer path");
    writer
        .set_nonblocking(true)
        .expect("the physical writer can be made nonblocking");

    let failure = write_provider_control_frame_until(
        &mut writer,
        b"provider-control-prefix",
        b"provider-control-body",
        Instant::now() - Duration::from_millis(1),
    )
    .expect_err("an already-expired deadline must fail before a provider-control write");
    assert!(
        matches!(failure, ProviderControlFrameWriteFailure::DeadlineElapsed),
        "expired control writes fail through the precise bounded-deadline branch"
    );
}

#[test]
fn provider_control_writer_expires_while_the_peer_remains_backpressured() {
    let (mut writer, _reader) =
        UnixStream::pair().expect("a local Unix pair provides the physical control writer path");
    writer
        .set_nonblocking(true)
        .expect("the physical writer can be made nonblocking");
    assert!(
        prefill_until_would_block(&mut writer) > 0,
        "a non-draining peer leaves the physical control writer actually backpressured"
    );

    let failure = write_provider_control_frame_until(
        &mut writer,
        b"provider-control-prefix",
        b"provider-control-body",
        Instant::now() + Duration::from_millis(100),
    )
    .expect_err("a full non-draining stream must expire its still-future control deadline");
    assert!(
        matches!(failure, ProviderControlFrameWriteFailure::DeadlineElapsed),
        "actual blocked progress reaches the same finite deadline failure without appending a frame"
    );
}

#[cfg(feature = "i3-process-test-seams")]
fn source_real_compact_peer_descriptor() -> (
    Sys5I3PreparedProviderLocalnetLaunch,
    PrivateProviderPeerStartBinding,
) {
    let launch = crate::prepare_source_real_i3_read_only_provider_localnet_launch()
        .expect("the finite source-real fixture prepares its actual A/B launch");
    let descriptor = launch
        .test_only_peer_start_binding_descriptor_for_role(
            Sys5I3ProviderChildRole::RequesterConsumer,
        )
        .expect("the actual parent-held peer binding derives one compact requester descriptor");
    (launch, descriptor)
}

#[cfg(feature = "i3-process-test-seams")]
fn assert_compact_peer_descriptor_rejected(
    launch: &Sys5I3PreparedProviderLocalnetLaunch,
    candidate: PrivateProviderPeerStartBinding,
    case: &str,
) {
    let error = launch
        .test_only_peer_start_binding_descriptor_matches_parent_held_binding(
            Sys5I3ProviderChildRole::RequesterConsumer,
            &candidate,
        )
        .expect_err(
            "a substituted compact peer descriptor must not match the actual parent-held peer",
        );
    assert_eq!(
        error.kind(),
        Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
        "{case}: the compact peer descriptor rejects before control construction"
    );
}

fn assert_private_provider_control_frame_rejected(case: &str, bytes: Vec<u8>) {
    let error = match read_private_provider_child_control_frame(&mut Cursor::new(bytes)) {
        Ok(_) => panic!("a malformed provider control frame must remain parser-only rejection"),
        Err(error) => error,
    };
    assert_eq!(
        error.kind(),
        Sys5I3ProviderLaunchErrorKind::InheritedProviderControlRejected,
        "{case}: parser rejection occurs before any provider-control installation"
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn source_real_compact_peer_descriptor_matches_each_parent_held_peer_binding() {
    let launch = crate::prepare_source_real_i3_read_only_provider_localnet_launch()
        .expect("the finite source-real fixture prepares its actual A/B launch");
    for role in [
        Sys5I3ProviderChildRole::RequesterConsumer,
        Sys5I3ProviderChildRole::Executor,
    ] {
        let descriptor = launch
            .test_only_peer_start_binding_descriptor_for_role(role)
            .expect("the compact peer descriptor derives only from an actual parent-held binding");
        launch
            .test_only_peer_start_binding_descriptor_matches_parent_held_binding(role, &descriptor)
            .expect("the unmodified compact descriptor matches its parent-held peer binding");
    }
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn source_real_compact_peer_descriptor_rejects_substituted_binding_coordinates() {
    let (launch, descriptor) = source_real_compact_peer_descriptor();

    let mut unknown_field = serde_json::to_value(&descriptor)
        .expect("the compact descriptor serializes only for strict private-boundary tests");
    unknown_field
        .as_object_mut()
        .expect("the compact descriptor is a JSON object")
        .insert("unexpected_peer_authority".to_string(), Value::Bool(true));
    assert!(
        serde_json::from_value::<PrivateProviderPeerStartBinding>(unknown_field).is_err(),
        "an unknown peer-descriptor member must fail strict decode before matching"
    );

    let mut empty_slot = serde_json::to_value(&descriptor)
        .expect("the compact descriptor serializes for the nonempty-slot falsifier");
    *empty_slot
        .pointer_mut("/slot_name")
        .expect("the compact descriptor retains its peer slot coordinate") =
        Value::String(String::new());
    assert_compact_peer_descriptor_rejected(
        &launch,
        serde_json::from_value(empty_slot)
            .expect("an empty peer slot remains a structurally decodable untrusted candidate"),
        "empty peer slot",
    );

    let mut foreign_source = serde_json::to_value(&descriptor)
        .expect("the compact descriptor serializes for the source-coordinate falsifier");
    *foreign_source
        .pointer_mut("/parent_checked_program_ref")
        .expect("the compact descriptor retains the checked-program coordinate") =
        Value::String("foreign-checked-program-ref".to_string());
    assert_compact_peer_descriptor_rejected(
        &launch,
        serde_json::from_value(foreign_source)
            .expect("a foreign program ref remains a structurally decodable untrusted candidate"),
        "foreign checked-program reference",
    );

    let mut raw_stable_key = serde_json::to_value(&descriptor)
        .expect("the compact descriptor serializes for the raw-key-shaped reference falsifier");
    *raw_stable_key
        .pointer_mut("/parent_checked_program_ref")
        .expect("the compact descriptor retains the checked-program coordinate") =
        Value::String("checked-surface-stable-key:read-only-provider-effect".to_string());
    let raw_stable_key: PrivateProviderPeerStartBinding = serde_json::from_value(raw_stable_key)
        .expect("a raw-key-shaped program field remains a decodable untrusted candidate");
    assert!(
        !raw_stable_key.is_structurally_valid(),
        "a raw stable-key-shaped program coordinate is not a transportable checked-program reference"
    );
    assert_compact_peer_descriptor_rejected(
        &launch,
        raw_stable_key,
        "raw stable-key-shaped checked-program reference",
    );

    let mut foreign_image = serde_json::to_value(&descriptor)
        .expect("the compact descriptor serializes for the image-coordinate falsifier");
    *foreign_image
        .pointer_mut("/image_integrity_ref")
        .expect("the compact descriptor retains the image coordinate") =
        Value::String("foreign-image-integrity-ref".to_string());
    assert_compact_peer_descriptor_rejected(
        &launch,
        serde_json::from_value(foreign_image)
            .expect("a foreign image ref remains a structurally decodable untrusted candidate"),
        "foreign image integrity reference",
    );

    let mut foreign_component = serde_json::to_value(&descriptor)
        .expect("the compact descriptor serializes for the component-coordinate falsifier");
    *foreign_component
        .pointer_mut("/component_binding_ref")
        .expect("the compact descriptor retains the scoped component coordinate") =
        Value::String("foreign-component-binding-ref".to_string());
    assert_compact_peer_descriptor_rejected(
        &launch,
        serde_json::from_value(foreign_component)
            .expect("a foreign component ref remains a structurally decodable untrusted candidate"),
        "foreign scoped component reference",
    );

    let error = launch
        .test_only_peer_start_binding_descriptor_matches_parent_held_binding(
            Sys5I3ProviderChildRole::Executor,
            &descriptor,
        )
        .expect_err("a compact descriptor derived for A's peer cannot match B's parent-held peer");
    assert_eq!(
        error.kind(),
        Sys5I3ProviderLaunchErrorKind::ProviderControlConstructionRejected,
        "role substitution is rejected before a compact peer descriptor becomes control"
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn source_real_provider_child_control_size_metrics_are_count_only() {
    for (profile, profile_label) in [
        (crate::I3ReadOnlyProviderFixtureProfile::Value41, "value_41"),
        (
            crate::I3ReadOnlyProviderFixtureProfile::ValueNegative7,
            "value_negative_7",
        ),
        (
            crate::I3ReadOnlyProviderFixtureProfile::DeclaredTargetAbsent,
            "declared_target_absent",
        ),
    ] {
        let mut launch =
            crate::prepare_source_real_i3_read_only_provider_localnet_launch_with_fixture_profile(
                profile,
            )
            .expect("each finite source-real fixture prepares its actual A/B launch");
        for (role, role_label) in [
            (
                Sys5I3ProviderChildRole::RequesterConsumer,
                "requester_consumer",
            ),
            (Sys5I3ProviderChildRole::Executor, "executor"),
        ] {
            let metrics = launch
                .test_only_provider_child_control_snapshot_size_metrics(role)
                .expect("the production control snapshot builder yields its redacted size metrics");

            for (name, bytes) in [
                (
                    "local_static_snapshot_bytes",
                    metrics.local_static_snapshot_bytes,
                ),
                (
                    "local_component_snapshot_bytes",
                    metrics.local_component_snapshot_bytes,
                ),
                (
                    "local_legacy_authority_snapshot_bytes",
                    metrics.local_legacy_authority_snapshot_bytes,
                ),
                ("role_m9_snapshot_bytes", metrics.role_m9_snapshot_bytes),
                ("peer_start_binding_bytes", metrics.peer_start_binding_bytes),
                (
                    "whole_control_empty_transport_bytes",
                    metrics.whole_control_empty_transport_bytes,
                ),
                (
                    "whole_control_max_transport_upper_bound_bytes",
                    metrics.whole_control_max_transport_upper_bound_bytes,
                ),
            ] {
                assert!(
                    bytes > 0,
                    "{profile_label} {role_label} {name} is an actual nonempty size measurement"
                );
            }
            assert!(
                metrics.whole_control_max_transport_upper_bound_bytes
                    >= metrics.whole_control_empty_transport_bytes,
                "{profile_label} {role_label} transport upper bound includes the empty-control baseline"
            );
            assert!(
                metrics.whole_control_max_transport_upper_bound_bytes
                    <= PRIVATE_PROVIDER_CHILD_CONTROL_MAX_BYTES,
                "{profile_label} {role_label} maximum transport allowance stays within the finite provider-control cap"
            );
            eprintln!(
                "profile={profile_label} role={role_label} local_static_snapshot_bytes={} local_component_snapshot_bytes={} local_legacy_authority_snapshot_bytes={} role_m9_snapshot_bytes={} peer_start_binding_bytes={} whole_control_empty_transport_bytes={} whole_control_max_transport_upper_bound_bytes={}",
                metrics.local_static_snapshot_bytes,
                metrics.local_component_snapshot_bytes,
                metrics.local_legacy_authority_snapshot_bytes,
                metrics.role_m9_snapshot_bytes,
                metrics.peer_start_binding_bytes,
                metrics.whole_control_empty_transport_bytes,
                metrics.whole_control_max_transport_upper_bound_bytes,
            );
        }
    }
}

#[test]
fn provider_child_control_frame_parser_rejects_bounded_malformed_inputs_before_install() {
    let declared_over_cap = u32::try_from(PRIVATE_PROVIDER_CHILD_CONTROL_MAX_BYTES + 1)
        .expect("the finite provider-control cap fits the fixed frame prefix")
        .to_be_bytes()
        .to_vec();
    assert_private_provider_control_frame_rejected(
        "declared body exceeds provider control cap",
        declared_over_cap,
    );

    let mut truncated = 2_u32.to_be_bytes().to_vec();
    truncated.extend_from_slice(b"{");
    assert_private_provider_control_frame_rejected("declared body is truncated", truncated);

    let mut trailing_after_zero = 0_u32.to_be_bytes().to_vec();
    trailing_after_zero.push(b'x');
    assert_private_provider_control_frame_rejected(
        "zero-length body has an undeclared trailing byte",
        trailing_after_zero,
    );

    let duplicate_version_body = br#"{"version":3,"version":3}"#;
    let mut duplicate_version = u32::try_from(duplicate_version_body.len())
        .expect("the synthetic duplicate-key body fits the fixed frame prefix")
        .to_be_bytes()
        .to_vec();
    duplicate_version.extend_from_slice(duplicate_version_body);
    assert_private_provider_control_frame_rejected(
        "top-level version is duplicated",
        duplicate_version,
    );

    let unknown_top_level_body = br#"{"version":3,"unexpected_provider_control_member":true}"#;
    let mut unknown_top_level = u32::try_from(unknown_top_level_body.len())
        .expect("the synthetic unknown-field body fits the fixed frame prefix")
        .to_be_bytes()
        .to_vec();
    unknown_top_level.extend_from_slice(unknown_top_level_body);
    assert_private_provider_control_frame_rejected(
        "top-level provider-control member is unknown",
        unknown_top_level,
    );
}

#[test]
fn ordinary_process_message_decoder_rejects_a_body_over_its_existing_bound() {
    let declared = u32::try_from(Sys5I3PrivateProcessCodec::MAX_MESSAGE_BYTES + 1)
        .expect("the existing ordinary message bound fits its fixed frame prefix");
    let error = Sys5I3PrivateProcessCodec::private_provisional_v1()
        .decode_untrusted_message(&declared.to_be_bytes())
        .expect_err(
            "an ordinary process message over its declared bound must reject before decoding",
        );
    assert_eq!(
        error.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Oversized,
        "the ordinary message decoder retains its separate fixed body limit"
    );
}

#[cfg(feature = "i3-process-test-seams")]
#[test]
fn source_real_provider_control_serialized_capacity_stays_within_the_finite_cap() {
    for profile in [
        crate::I3ReadOnlyProviderFixtureProfile::Value41,
        crate::I3ReadOnlyProviderFixtureProfile::ValueNegative7,
        crate::I3ReadOnlyProviderFixtureProfile::DeclaredTargetAbsent,
    ] {
        for role in [
            Sys5I3ProviderChildRole::RequesterConsumer,
            Sys5I3ProviderChildRole::Executor,
        ] {
            let mut launch =
                crate::prepare_source_real_i3_read_only_provider_localnet_launch_with_fixture_profile(
                    profile,
                )
                .expect("each fixed source-real fixture profile prepares an actual A/B launch");
            let capacity = launch
                .test_only_provider_child_control_serialized_capacity_for_role(role)
                .expect("the real consuming parent-held binding path serializes within its cap");
            assert!(
                capacity <= PRIVATE_PROVIDER_CHILD_CONTROL_MAX_BYTES,
                "the bounded serializer allocation capacity stays within the provider-control ceiling"
            );
        }
    }
}
