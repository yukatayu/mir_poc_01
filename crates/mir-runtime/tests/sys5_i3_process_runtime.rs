//! RED contracts for I3-2 process images and the pre-socket process-runtime
//! seam.  The named surface is deliberately `doc(hidden)`/provisional: this
//! file specifies only the bounded I3-2 implementation seam, never a public
//! artifact, deployment, carrier, or runtime API.

use std::collections::BTreeSet;

use mir_runtime::{
    sys5_i3_process_runtime::{
        Sys5I3Deployment, Sys5I3DeploymentSlot, Sys5I3PrivateProcessCodec,
        Sys5I3PrivateProcessCodecErrorKind, Sys5I3ProcessArtifact, Sys5I3ProcessCohort,
        Sys5I3ProcessImage, Sys5I3ProcessRuntime, Sys5I3ProcessRuntimeErrorKind,
        Sys5I3RetainedEdgeContract,
    },
    sys5_local_slice::{Sys5LocalProject, Sys5LocalSliceError, Sys5SourceInput, build_project},
};
use serde_json::Value;

#[cfg(feature = "i3-process-test-seams")]
use mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessImageTamper;

const CANONICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";
const CANONICAL_SOURCE: &str =
    include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
const REQUESTER_SLOT: &str = "process-a";
const OWNER_SLOT: &str = "process-b";
const PRIVATE_PROCESS_CODEC_PREFIX_BYTES: usize = 4;
// These pointers intentionally state the provisional private codec schema the
// implementation must provide.  They are test-owned expectations, not a
// public wire/API commitment; a schema change must update this test together
// with the codec rather than adding a production tamper constructor.
const PRIVATE_PROCESS_IMAGE_ROOT: &str = "/image";
const PRIVATE_PROCESS_MESSAGE_ROOT: &str = "/message";
const PRIVATE_PROCESS_VERSION_PATH: &str = "/version";
const PRIVATE_PROCESS_IMAGE_EDGE_OBJECT_PATH: &str = "/image/required_edge_contracts/0";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_COHORT_PATH: &str = "/message/cohort_provenance_ref";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_KIND_PATH: &str = "/message/kind";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_REQUEST_IDENTITY_PATH: &str =
    "/message/semantic_request_identity_ref";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_LINKED_REQUEST_IDENTITY_PATH: &str =
    "/message/linked_request_identity_ref";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_EDGE_PATH: &str = "/message/carrier/edge_ref";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_TARGET_PATH: &str = "/message/carrier/target_locus";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_SOURCE_PATH: &str = "/message/carrier/source_locus";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_OPERATION_PATH: &str = "/message/carrier/operation_id";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_CARRIER_PROVENANCE_PATH: &str = "/message/carrier/core_ref";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_REQUEST_CARRIER_ID_PATH: &str = "/message/carrier/request_carrier_id";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_M9_OWNER_LINEAGE_PATH: &str = "/message/carrier/m9_owner_lineage_ref";
#[cfg(feature = "i3-process-test-seams")]
const PRIVATE_PROCESS_MESSAGE_REPLY_RECEIPT_REQUEST_ID_PATH: &str =
    "/message/carrier/payload/fields/receipt/request_id";
const PRIVATE_PROCESS_IMAGE_ASSIGNED_LOCI_PATH: &str = "/image/assigned_loci";
const PRIVATE_PROCESS_IMAGE_SEMANTIC_ROWS_PATH: &str =
    "/image/child_seed/required_local_authority_closure/rows";
const PRIVATE_PROCESS_MESSAGE_PAYLOAD_PATH: &str = "/message/carrier/payload";
const PRIVATE_PROCESS_PROJECTION_VERSION_PATH: &str = "/projection/version";
const PRIVATE_PROCESS_ADMISSION_VERSION_PATH: &str = "/admission/version";
const CI_SAFE_UNIQUE_IMAGE_COLLECTION_ITEMS: usize = 128;
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

#[cfg(feature = "i3-process-test-seams")]
fn image_for_source_and_slot(source_text: &str, slot: &str) -> Sys5I3ProcessImage {
    let project = build_once(source_text);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    take_process_image(&mut cohort, slot)
}

fn private_process_json(frame: &[u8], expected_root: &str) -> Value {
    assert!(
        frame.len() >= PRIVATE_PROCESS_CODEC_PREFIX_BYTES,
        "private process codec frames begin with a fixed four-byte length prefix"
    );
    let declared = u32::from_be_bytes(
        frame[..PRIVATE_PROCESS_CODEC_PREFIX_BYTES]
            .try_into()
            .expect("private codec prefix has exactly four bytes"),
    );
    assert_eq!(
        declared as usize,
        frame.len() - PRIVATE_PROCESS_CODEC_PREFIX_BYTES,
        "private process codec frame must declare the exact JSON body length"
    );
    let value: Value = serde_json::from_slice(&frame[PRIVATE_PROCESS_CODEC_PREFIX_BYTES..])
        .expect("canonical private codec body must be JSON for test-only byte mutation");
    assert!(
        value.pointer(expected_root).is_some(),
        "private codec must retain the test-owned expected envelope root"
    );
    value
}

fn private_process_json_frame(value: &Value) -> Vec<u8> {
    let body =
        serde_json::to_vec(value).expect("test-mutated private process envelope remains JSON");
    let length = u32::try_from(body.len()).expect("test JSON body fits the private u32 prefix");
    let mut frame = length.to_be_bytes().to_vec();
    frame.extend_from_slice(&body);
    frame
}

#[cfg(feature = "i3-process-test-seams")]
fn mutate_private_process_string_field(
    frame: &[u8],
    expected_root: &str,
    pointer: &str,
    mutation_label: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let field = value.pointer_mut(pointer).unwrap_or_else(|| {
        panic!("private process codec must expose required test schema field {pointer}")
    });
    let original = field.as_str().unwrap_or_else(|| {
        panic!("private process codec test schema field {pointer} must be a string")
    });
    *field = Value::String(format!("{original}-{mutation_label}"));
    private_process_json_frame(&value)
}

#[cfg(feature = "i3-process-test-seams")]
fn replace_private_process_string_field(
    frame: &[u8],
    expected_root: &str,
    pointer: &str,
    replacement: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let field = value.pointer_mut(pointer).unwrap_or_else(|| {
        panic!("private process codec must expose required test schema field {pointer}")
    });
    assert!(
        field.is_string(),
        "private process codec test schema field {pointer} must be a string"
    );
    *field = Value::String(replacement.to_string());
    private_process_json_frame(&value)
}

#[cfg(feature = "i3-process-test-seams")]
fn replace_private_process_optional_string_field(
    frame: &[u8],
    expected_root: &str,
    pointer: &str,
    replacement: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let field = value.pointer_mut(pointer).unwrap_or_else(|| {
        panic!("private process codec must expose required test schema field {pointer}")
    });
    assert!(
        field.is_null() || field.is_string(),
        "private process codec test schema field {pointer} must be an optional string"
    );
    *field = Value::String(replacement.to_string());
    private_process_json_frame(&value)
}

fn duplicate_private_process_array_element(
    frame: &[u8],
    expected_root: &str,
    pointer: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let array = value
        .pointer_mut(pointer)
        .and_then(Value::as_array_mut)
        .unwrap_or_else(|| {
            panic!("private process codec must expose required test schema array {pointer}")
        });
    let element = array.first().cloned().unwrap_or_else(|| {
        panic!("private process codec test schema array {pointer} must be nonempty")
    });
    array.push(element);
    private_process_json_frame(&value)
}

fn append_private_process_unknown_object_member(
    frame: &[u8],
    expected_root: &str,
    pointer: &str,
    member: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let object = value
        .pointer_mut(pointer)
        .and_then(Value::as_object_mut)
        .unwrap_or_else(|| {
            panic!("private process codec must expose required test schema object {pointer}")
        });
    assert!(
        object
            .insert(
                member.to_string(),
                Value::String("untrusted-extra".to_string())
            )
            .is_none(),
        "private process codec test schema member {member} must not already exist"
    );
    private_process_json_frame(&value)
}

/// Build a CI-safe *unique* untrusted image collection without a wall-clock
/// assertion.  The decoder's byte limit bounds input size; exact source image
/// identity remains the later parent-held start-binding check.  This cannot
/// prove a complexity class black-box, but prevents an implementation from
/// treating every large adversarial collection as a duplicate by default.
fn append_ci_safe_unique_image_inventory(frame: &[u8]) -> Vec<u8> {
    let mut value = private_process_json(frame, PRIVATE_PROCESS_IMAGE_ROOT);
    let assigned_loci = value
        .pointer_mut(PRIVATE_PROCESS_IMAGE_ASSIGNED_LOCI_PATH)
        .and_then(Value::as_array_mut)
        .expect("private process image schema retains its assigned-loci collection");
    for ordinal in 0..CI_SAFE_UNIQUE_IMAGE_COLLECTION_ITEMS {
        assigned_loci.push(Value::String(format!("UntrustedUniqueLocus{ordinal:03}")));
    }

    let semantic_rows = value
        .pointer_mut(PRIVATE_PROCESS_IMAGE_SEMANTIC_ROWS_PATH)
        .and_then(Value::as_array_mut)
        .expect("private process image schema retains its semantic-row collection");
    let prototype = semantic_rows.first().cloned().expect(
        "the canonical source-derived image has one semantic row usable as a unique-row schema witness",
    );
    for ordinal in 0..CI_SAFE_UNIQUE_IMAGE_COLLECTION_ITEMS {
        let mut row = prototype.clone();
        let locus = row
            .pointer_mut("/fields/locus")
            .expect("the canonical first semantic-row witness is an artifact row with a locus");
        *locus = Value::String(format!("UntrustedUniqueLocus{ordinal:03}"));
        semantic_rows.push(row);
    }
    private_process_json_frame(&value)
}

fn replace_private_process_version(frame: &[u8], expected_root: &str) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let field = value.pointer_mut(PRIVATE_PROCESS_VERSION_PATH).unwrap_or_else(|| {
        panic!(
            "private process codec must expose required test schema field {PRIVATE_PROCESS_VERSION_PATH}"
        )
    });
    *field = Value::from(u64::MAX);
    private_process_json_frame(&value)
}

fn replace_private_process_nested_version(
    frame: &[u8],
    expected_root: &str,
    pointer: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let field = value.pointer_mut(pointer).unwrap_or_else(|| {
        panic!("private process codec must expose required nested schema version {pointer}")
    });
    assert!(
        field.is_number(),
        "private process codec nested schema version {pointer} must be numeric"
    );
    *field = Value::from(u64::MAX);
    private_process_json_frame(&value)
}

fn private_process_declared_over_bound_frame(limit: usize) -> Vec<u8> {
    let declared = u32::try_from(limit).expect("finite private codec bounds fit its u32 framing");
    declared.to_be_bytes().to_vec()
}

fn remove_private_process_object_member(
    frame: &[u8],
    expected_root: &str,
    object_pointer: &str,
    member: &str,
) -> Vec<u8> {
    let mut value = private_process_json(frame, expected_root);
    let object = value
        .pointer_mut(object_pointer)
        .and_then(Value::as_object_mut)
        .unwrap_or_else(|| {
            panic!("private process codec must expose object test schema field {object_pointer}")
        });
    assert!(
        object.remove(member).is_some(),
        "private process codec must retain required test schema member {member}"
    );
    private_process_json_frame(&value)
}

fn json_member(name: &str, value: &Value) -> String {
    format!(
        "{}:{}",
        serde_json::to_string(name).expect("test field name serializes"),
        serde_json::to_string(value).expect("test field value serializes")
    )
}

fn private_process_object_with_duplicate_member(
    object: &serde_json::Map<String, Value>,
    duplicate_member: Option<&str>,
    nested_object_replacement: Option<(&str, &str)>,
) -> String {
    let mut members = Vec::new();
    for (name, value) in object {
        let rendered = nested_object_replacement
            .filter(|(replacement_name, _)| name == replacement_name)
            .map(|(_, replacement)| {
                format!(
                    "{}:{replacement}",
                    serde_json::to_string(name).expect("test nested field name serializes")
                )
            })
            .unwrap_or_else(|| json_member(name, value));
        members.push(rendered.clone());
        if duplicate_member == Some(name.as_str()) {
            members.push(rendered);
        }
    }
    if let Some(duplicate_member) = duplicate_member {
        assert!(
            object.contains_key(duplicate_member),
            "private process codec must retain required test schema member {duplicate_member}"
        );
    }
    format!("{{{}}}", members.join(","))
}

fn private_process_frame_with_duplicate_member(
    frame: &[u8],
    duplicate_level: &str,
    duplicate_member: &str,
) -> Vec<u8> {
    let value = private_process_json(frame, PRIVATE_PROCESS_MESSAGE_ROOT);
    let envelope = value
        .as_object()
        .expect("private process envelope is a JSON object");
    let version = envelope
        .get("version")
        .expect("private process envelope has version");
    let message = envelope
        .get("message")
        .and_then(Value::as_object)
        .expect("private process envelope has message object");
    let message_body = match duplicate_level {
        "envelope" => private_process_object_with_duplicate_member(message, None, None),
        "message" => {
            private_process_object_with_duplicate_member(message, Some(duplicate_member), None)
        }
        "carrier" => {
            let carrier = message
                .get("carrier")
                .and_then(Value::as_object)
                .expect("private process message has carrier object");
            let carrier_body =
                private_process_object_with_duplicate_member(carrier, Some(duplicate_member), None);
            private_process_object_with_duplicate_member(
                message,
                None,
                Some(("carrier", carrier_body.as_str())),
            )
        }
        "payload" => {
            let carrier = message
                .get("carrier")
                .and_then(Value::as_object)
                .expect("private process message has carrier object");
            let payload = carrier
                .get("payload")
                .and_then(Value::as_object)
                .expect("private process carrier has tagged payload object");
            let payload_body =
                private_process_object_with_duplicate_member(payload, Some(duplicate_member), None);
            let carrier_body = private_process_object_with_duplicate_member(
                carrier,
                None,
                Some(("payload", payload_body.as_str())),
            );
            private_process_object_with_duplicate_member(
                message,
                None,
                Some(("carrier", carrier_body.as_str())),
            )
        }
        other => panic!("unknown private duplicate test level {other}"),
    };
    let body = match (duplicate_level, duplicate_member) {
        ("envelope", "version") => format!(
            "{{{},{},\"message\":{message_body}}}",
            json_member("version", version),
            json_member("version", version),
        ),
        ("envelope", "message") => format!(
            "{{\"version\":{},\"message\":{message_body},\"message\":{message_body}}}",
            serde_json::to_string(version).expect("version serializes"),
        ),
        ("message", _) | ("carrier", _) | ("payload", _) => format!(
            "{{\"version\":{},\"message\":{message_body}}}",
            serde_json::to_string(version).expect("version serializes"),
        ),
        (_, member) => panic!("unknown private duplicate member {member}"),
    };
    let length = u32::try_from(body.len()).expect("raw duplicate JSON fits private u32 prefix");
    let mut framed = length.to_be_bytes().to_vec();
    framed.extend_from_slice(body.as_bytes());
    framed
}

/// Only the matching coordinator-held binding may promote private bytes into a
/// runtime.  The decoded value remains untrusted until this receiver-owned
/// boundary; it has no direct `Sys5I3ProcessRuntime` constructor.
fn decode_and_start_private_image(
    codec: &Sys5I3PrivateProcessCodec,
    cohort: &mut Sys5I3ProcessCohort,
    slot: &str,
) -> Sys5I3ProcessRuntime {
    let expected_start_binding = cohort
        .parent_held_expected_start_binding(slot)
        .expect("coordinator retains an opaque start binding for its one child image");
    let image = take_process_image(cohort, slot);
    let image_bytes = codec
        .encode_image(image)
        .expect("checked image encodes through the private codec");
    let decoded = codec
        .decode_untrusted_image(&image_bytes)
        .expect("checked private image bytes decode to an untrusted candidate");
    codec
        .validate_and_start_image(decoded, expected_start_binding)
        .expect("matching coordinator-held binding starts the assigned child runtime")
}

/// Default-feature codec fixture. It exercises only source -> image ->
/// binding -> runtime -> generated outbound request, not the raw decoded
/// ingress regression seam.
fn encoded_generated_owner_request() -> (Sys5I3PrivateProcessCodec, Vec<u8>) {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the source-derived requester emits its generated owner request");
    let bytes = codec
        .encode_outbound_message(request)
        .expect("the generated request encodes through the private codec");
    (codec, bytes)
}

/// Restore two images strictly from their private bytes and coordinator-held
/// bindings.  This helper deliberately accepts a prebuilt checked project:
/// once image bytes exist, restore never receives source text or reparses it.
fn restore_runtime_pair_from_private_images(
    project: &Sys5LocalProject,
    deployment: &Sys5I3Deployment,
) -> (Sys5I3ProcessRuntime, Sys5I3ProcessRuntime) {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(project, deployment);
    let requester_binding = cohort
        .parent_held_expected_start_binding(REQUESTER_SLOT)
        .expect("the coordinator retains requester start binding separately from the image");
    let owner_binding = cohort
        .parent_held_expected_start_binding(OWNER_SLOT)
        .expect("the coordinator retains owner start binding separately from the image");
    let requester_bytes = codec
        .encode_image(take_process_image(&mut cohort, REQUESTER_SLOT))
        .expect("the requester image encodes to private bounded bytes");
    let owner_bytes = codec
        .encode_image(take_process_image(&mut cohort, OWNER_SLOT))
        .expect("the owner image encodes to private bounded bytes");
    let requester = codec
        .validate_and_start_image(
            codec
                .decode_untrusted_image(&requester_bytes)
                .expect("private requester bytes decode only to an untrusted candidate"),
            requester_binding,
        )
        .expect("only the separate requester binding restores its runtime");
    let owner = codec
        .validate_and_start_image(
            codec
                .decode_untrusted_image(&owner_bytes)
                .expect("private owner bytes decode only to an untrusted candidate"),
            owner_binding,
        )
        .expect("only the separate owner binding restores its runtime");
    (requester, owner)
}

fn start_runtime_pair_directly(
    project: &Sys5LocalProject,
    deployment: &Sys5I3Deployment,
) -> (Sys5I3ProcessRuntime, Sys5I3ProcessRuntime) {
    let mut cohort = single_coordinator_cohort(project, deployment);
    let requester = Sys5I3ProcessRuntime::start(take_process_image(&mut cohort, REQUESTER_SLOT))
        .expect("the source-derived requester image starts directly in the baseline path");
    let owner = Sys5I3ProcessRuntime::start(take_process_image(&mut cohort, OWNER_SLOT))
        .expect("the source-derived owner image starts directly in the baseline path");
    (requester, owner)
}

#[derive(Debug, PartialEq, Eq)]
struct OwnerRequestBehavior {
    owner_hp: i64,
    owner_served_count: usize,
    owner_write_count: usize,
    requester_receipt_count: usize,
    requester_write_count: usize,
    receipt_is_local_only: bool,
}

fn execute_init_avatar_hp(
    requester: &mut Sys5I3ProcessRuntime,
    owner: &mut Sys5I3ProcessRuntime,
) -> OwnerRequestBehavior {
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the checked source image derives its owner request");
    let reply = owner
        .accept_inbound(request)
        .expect("the owner admits the source-derived request")
        .expect("owner execution yields exactly one typed reply");
    let receipt = requester
        .accept_inbound(reply)
        .expect("the requester admits the owner reply")
        .expect("reply admission creates the requester-local receipt");
    OwnerRequestBehavior {
        owner_hp: owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect("the source-derived owner action writes avatar hp"),
        owner_served_count: owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        owner_write_count: owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        requester_receipt_count: requester
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        requester_write_count: requester
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        receipt_is_local_only: receipt.has_no_transportable_carrier(),
    }
}

#[cfg(feature = "i3-process-test-seams")]
struct PendingPrivateReplyFixture {
    codec: Sys5I3PrivateProcessCodec,
    requester: Sys5I3ProcessRuntime,
    request_identity: String,
    reply_bytes: Vec<u8>,
}

/// Produce a valid owner reply while retaining the requester whose exact
/// source-derived request is still locally pending.  Each reply falsifier
/// receives a fresh fixture so no rejected candidate can be hidden by a
/// previous receipt transition.
#[cfg(feature = "i3-process-test-seams")]
fn pending_private_reply_fixture() -> PendingPrivateReplyFixture {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("source-derived requester operation emits one generated request");
    let request_identity = request.semantic_request_identity_ref().to_string();
    let request_bytes = codec
        .encode_outbound_message(request)
        .expect("the generated request encodes through the private codec");
    let reply = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&request_bytes)
                .expect("the exact request bytes decode only as an untrusted candidate"),
        )
        .expect("owner accepts the source-derived pending request")
        .expect("owner serve returns one source-derived reply");
    let reply_bytes = codec
        .encode_outbound_message(reply)
        .expect("the generated owner reply encodes through the private codec");
    PendingPrivateReplyFixture {
        codec,
        requester,
        request_identity,
        reply_bytes,
    }
}

#[cfg(feature = "i3-process-test-seams")]
fn assert_rejected_private_reply_preserves_requester_pending_state(
    fixture: &mut PendingPrivateReplyFixture,
    candidate_bytes: &[u8],
    expected_request_identity: &str,
) {
    let summary_before = fixture.requester.observer_safe_runtime_summary();
    let outbox_before = fixture.requester.observer_safe_outbox_summary();
    let receipt_before = fixture
        .requester
        .observer_safe_semantic_occurrences()
        .requester_local_receipt_occurrence_ref(expected_request_identity)
        .map(str::to_string);
    assert_eq!(
        fixture
            .requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("the requester never owns WorldAuthority state")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert_eq!(
        fixture
            .requester
            .admit_untrusted_message(
                fixture
                    .codec
                    .decode_untrusted_message(candidate_bytes)
                    .expect("the byte-mutated reply remains only an untrusted candidate"),
            )
            .expect_err("a forged reply must reject before receipt/state/outbox mutation")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        fixture.requester.observer_safe_runtime_summary(),
        summary_before,
        "rejected reply bytes must preserve requester served/write/receipt counters"
    );
    assert_eq!(
        fixture.requester.observer_safe_outbox_summary(),
        outbox_before,
        "rejected reply bytes must preserve the requester outbox"
    );
    assert_eq!(
        fixture
            .requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(expected_request_identity),
        receipt_before.as_deref(),
        "rejected reply bytes must not mint or replace a requester-local receipt occurrence"
    );
    assert_eq!(
        fixture
            .requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("a rejected reply must not install owner state in the requester")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
}

#[cfg(feature = "i3-process-test-seams")]
fn admit_exact_private_reply(fixture: &mut PendingPrivateReplyFixture) -> String {
    let receipt = fixture
        .requester
        .admit_untrusted_message(
            fixture
                .codec
                .decode_untrusted_message(&fixture.reply_bytes)
                .expect("the exact reply remains an untrusted candidate until admission"),
        )
        .expect("the exact locally pending reply is admitted")
        .expect("the exact locally pending reply produces a local receipt");
    assert_eq!(
        receipt.linked_request_identity_ref(),
        Some(fixture.request_identity.as_str())
    );
    fixture
        .requester
        .observer_safe_semantic_occurrences()
        .requester_local_receipt_occurrence_ref(&fixture.request_identity)
        .expect("the accepted exact reply installs one receipt occurrence")
        .to_string()
}

#[cfg(feature = "i3-process-test-seams")]
struct PendingPrivateRequestFixture {
    codec: Sys5I3PrivateProcessCodec,
    owner: Sys5I3ProcessRuntime,
    request_identity: String,
    request_bytes: Vec<u8>,
}

#[cfg(feature = "i3-process-test-seams")]
fn pending_private_request_fixture() -> PendingPrivateRequestFixture {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);
    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("source-derived requester operation emits one generated request");
    let request_identity = request.semantic_request_identity_ref().to_string();
    let request_bytes = codec
        .encode_outbound_message(request)
        .expect("the generated request encodes through the private codec");
    PendingPrivateRequestFixture {
        codec,
        owner,
        request_identity,
        request_bytes,
    }
}

#[cfg(feature = "i3-process-test-seams")]
fn assert_rejected_private_request_preserves_owner_state_and_occurrences(
    fixture: &mut PendingPrivateRequestFixture,
    candidate_bytes: &[u8],
) {
    let summary_before = fixture.owner.observer_safe_runtime_summary();
    let outbox_before = fixture.owner.observer_safe_outbox_summary();
    let occurrences_before = fixture.owner.observer_safe_semantic_occurrences();
    assert_eq!(
        fixture
            .owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("the owner has no source-derived hp before a rejected request")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert_eq!(
        fixture
            .owner
            .admit_untrusted_message(
                fixture
                    .codec
                    .decode_untrusted_message(candidate_bytes)
                    .expect("the byte-mutated request remains only an untrusted candidate"),
            )
            .expect_err(
                "a forged request lineage must reject before owner serve/write/outbox mutation"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        fixture.owner.observer_safe_runtime_summary(),
        summary_before,
        "rejected request bytes must preserve owner serve/write/receipt counters"
    );
    assert_eq!(
        fixture.owner.observer_safe_outbox_summary(),
        outbox_before,
        "rejected request bytes must not mint an owner reply carrier"
    );
    assert_eq!(
        fixture.owner.observer_safe_semantic_occurrences(),
        occurrences_before,
        "rejected request bytes must not mint owner serve/write occurrences"
    );
    assert_eq!(
        fixture
            .owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("a rejected request must not materialize owner state")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
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
#[cfg(feature = "i3-process-test-seams")]
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
#[cfg(feature = "i3-process-test-seams")]
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

#[test]
fn g2_private_image_restore_preserves_owner_only_designated_free_profile_behavior() {
    let direct_project = build_once(OWNER_ONLY_SOURCE);
    let direct_deployment = owner_only_two_slot_deployment(&direct_project);
    let (mut direct_requester, mut direct_owner) =
        start_runtime_pair_directly(&direct_project, &direct_deployment);
    let direct_behavior = execute_init_avatar_hp(&mut direct_requester, &mut direct_owner);

    let restored_project = build_once(OWNER_ONLY_SOURCE);
    let restored_deployment = owner_only_two_slot_deployment(&restored_project);
    let (mut restored_requester, mut restored_owner) =
        restore_runtime_pair_from_private_images(&restored_project, &restored_deployment);
    let restored_behavior = execute_init_avatar_hp(&mut restored_requester, &mut restored_owner);

    assert_eq!(
        restored_behavior, direct_behavior,
        "private image encode/decode plus separate binding must preserve the designated-free owner profile without source reparse during restore"
    );
    assert_eq!(
        restored_behavior,
        OwnerRequestBehavior {
            owner_hp: 21,
            owner_served_count: 1,
            owner_write_count: 1,
            requester_receipt_count: 1,
            requester_write_count: 0,
            receipt_is_local_only: true,
        },
        "the restored designated-free profile remains one remote owner transition and one requester-local receipt"
    );
}

#[test]
fn g2_private_image_restore_preserves_two_distinct_designated_dependency_profile_behavior() {
    let source = two_dependency_same_operation_source();
    let direct_project = build_once(&source);
    let direct_deployment = evaluator_isolated_two_slot_deployment(&direct_project);
    let (mut direct_requester, mut direct_owner) =
        start_runtime_pair_directly(&direct_project, &direct_deployment);
    let direct_behavior = execute_init_avatar_hp(&mut direct_requester, &mut direct_owner);

    let restored_project = build_once(&source);
    let restored_deployment = evaluator_isolated_two_slot_deployment(&restored_project);
    let (mut restored_requester, mut restored_owner) =
        restore_runtime_pair_from_private_images(&restored_project, &restored_deployment);
    let restored_behavior = execute_init_avatar_hp(&mut restored_requester, &mut restored_owner);

    assert_eq!(
        restored_behavior, direct_behavior,
        "private restore must preserve the profile whose one designated operation retains two distinct request/receipt dependencies"
    );
    assert_eq!(
        restored_behavior.owner_hp, 21,
        "the restored image executes the same source-derived remote owner action without conflating designated dependency pairs"
    );
    assert_eq!(
        restored_behavior.requester_receipt_count, 1,
        "the restored profile still completes the owner reply at the requester rather than generating a third carrier"
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
fn g1_owner_serve_write_and_requester_receipt_have_distinct_exact_occurrence_evidence() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let requester_image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let owner_image = take_process_image(&mut cohort, OWNER_SLOT);
    let mut requester =
        Sys5I3ProcessRuntime::start(requester_image).expect("requester process image starts");
    let mut owner = Sys5I3ProcessRuntime::start(owner_image).expect("owner process image starts");

    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("source-derived request emits");
    let request_identity = request.semantic_request_identity_ref().to_string();
    let reply = owner
        .accept_inbound(request)
        .expect("owner admits generated request")
        .expect("owner produces generated reply");

    let owner_occurrences = owner.observer_safe_semantic_occurrences();
    let serve_occurrence = owner_occurrences
        .owner_serve_linearization_occurrence_ref(&request_identity)
        .expect("the admitted request has one observer-safe owner serve linearization occurrence");
    let write_occurrence = owner_occurrences
        .actual_owner_write_occurrence_ref(&request_identity)
        .expect("the source-derived write has one observer-safe owner-write occurrence");
    assert_ne!(
        serve_occurrence, write_occurrence,
        "owner request admission/serve linearization and actual state write are distinct semantic occurrences"
    );

    let receipt = requester
        .accept_inbound(reply)
        .expect("requester admits generated reply")
        .expect("requester completes its local receipt");
    assert_eq!(
        receipt.linked_request_identity_ref(),
        Some(request_identity.as_str())
    );
    assert!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(&request_identity)
            .is_some(),
        "requester receipt completion needs exact observer-safe occurrence evidence, not inference from a counter"
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
    assert!(
        !request_basis.includes_logical_origin_ref(),
        "semantic-request v3 binds kernel-generated request/carrier occurrence facts, not the process-local logical-origin ref"
    );
    assert!(
        !request_basis.includes_ordinal(),
        "semantic-request v3 binds its request/carrier occurrence pair, not the process-local store ordinal"
    );
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
        assert!(
            !basis.includes_logical_origin_ref(),
            "v3 request identity must not be described as process-local logical-origin based"
        );
        assert!(
            !basis.includes_ordinal(),
            "v3 request identity must not be described as process-local store ordinal based"
        );
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
fn g2_private_image_codec_bounds_untrusted_decode_and_requires_parent_held_start_binding() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();

    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let expected_start_binding = cohort
        .parent_held_expected_start_binding(REQUESTER_SLOT)
        .expect("the coordinator retains one opaque expected binding before releasing an image");
    let image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let expected_assigned_loci = image.assigned_loci();
    let expected_seed = image.observer_safe_child_seed();
    let expected_parent_ref = expected_seed.parent_checked_program_ref().to_string();
    let expected_projection_ref = expected_seed.projection_ref().to_string();
    let expected_m9_generation_ref = expected_seed.m9_generation_ref().to_string();
    let expected_cohort_ref = expected_seed
        .required_local_authority_closure()
        .opaque_cohort_ref()
        .to_string();

    // Encoding consumes the only image.  The bytes are now untrusted input;
    // neither this test nor a child can start the original image directly.
    let image_bytes = codec
        .encode_image(image)
        .expect("one checked child image encodes through the private bounded codec");
    assert!(
        image_bytes.len() <= codec.limits().max_image_bytes(),
        "private image encoding must enforce its declared bounded payload limit"
    );

    let decoded = codec
        .decode_untrusted_image(&image_bytes)
        .expect("the exact private image bytes decode only to an untrusted candidate");
    let manifest = decoded.observer_safe_manifest();
    assert_eq!(manifest.assigned_loci(), expected_assigned_loci);
    assert!(
        manifest.has_assigned_artifacts_only(),
        "decoded image manifest must retain executable artifacts only for its assigned loci"
    );
    assert_eq!(manifest.parent_checked_program_ref(), expected_parent_ref);
    assert_eq!(manifest.projection_ref(), expected_projection_ref);
    assert_eq!(manifest.m9_generation_ref(), expected_m9_generation_ref);
    assert_eq!(manifest.cohort_provenance_ref(), expected_cohort_ref);
    assert!(!manifest.carries_source_text());
    assert!(!manifest.carries_host_path());
    assert!(!manifest.carries_expected_result());

    let mut wrong_cohort = single_coordinator_cohort(&project, &deployment);
    let wrong_parent_binding = wrong_cohort
        .parent_held_expected_start_binding(REQUESTER_SLOT)
        .expect("a different coordinator owns a distinct expected cohort binding");
    assert_eq!(
        codec
            .validate_and_start_image(
                codec
                    .decode_untrusted_image(&image_bytes)
                    .expect("same bounded bytes may be decoded again only as untrusted input"),
                wrong_parent_binding,
            )
            .expect_err("a parent-held expected binding from another cohort must reject before runtime start")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch
    );
    let runtime = codec
        .validate_and_start_image(decoded, expected_start_binding)
        .expect("only the matching parent-held binding may convert decoded bytes into a started runtime");
    assert_candidate_a_child_runtime(&runtime);

    let malformed = codec
        // A bounded, complete frame whose one-byte body is invalid JSON.
        // Do not use arbitrary ASCII here: its first four bytes are a length
        // prefix and may correctly classify as declared Oversized.
        .decode_untrusted_image(b"\0\0\0\x01{")
        .expect_err("malformed image bytes must fail closed before admission");
    assert_eq!(
        malformed.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed
    );
    let truncated = codec
        .decode_untrusted_image(&image_bytes[..image_bytes.len() - 1])
        .expect_err("an incomplete image frame must never produce a partial child image");
    assert_eq!(
        truncated.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Incomplete
    );
    let oversized = vec![0_u8; codec.limits().max_image_bytes() + 1];
    let oversized = codec.decode_untrusted_image(&oversized).expect_err(
        "an oversized image frame must reject before child admission or allocation growth",
    );
    assert_eq!(
        oversized.kind(),
        Sys5I3PrivateProcessCodecErrorKind::Oversized
    );
    let unknown_version = replace_private_process_version(&image_bytes, PRIVATE_PROCESS_IMAGE_ROOT);
    assert_eq!(
        codec
            .decode_untrusted_image(&unknown_version)
            .expect_err("unknown private codec version must fail closed")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::UnknownVersion
    );
    let missing_edge_core = remove_private_process_object_member(
        &image_bytes,
        PRIVATE_PROCESS_IMAGE_ROOT,
        PRIVATE_PROCESS_IMAGE_EDGE_OBJECT_PATH,
        "core_ref",
    );
    assert_eq!(
        codec
            .decode_untrusted_image(&missing_edge_core)
            .expect_err("a missing incident-edge Core reference must not default to an empty provenance value")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::MissingRequiredCoreProvenance
    );
}

#[test]
fn g2_private_codec_classifies_declared_image_body_over_finite_bound_as_oversized() {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();

    // A declared body length beyond the finite private bound is capacity
    // failure, not a syntactic error or an incomplete semantic candidate.
    // The bytes carry no body: this specifically exercises the declared-size
    // branch before allocation or JSON parsing.
    assert_eq!(
        codec
            .decode_untrusted_image(&private_process_declared_over_bound_frame(
                codec.limits().max_image_bytes(),
            ))
            .expect_err("declared image body beyond the finite bound must fail closed")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Oversized
    );
}

#[test]
fn g2_private_codec_classifies_declared_message_body_over_finite_bound_as_oversized() {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    assert_eq!(
        codec
            .decode_untrusted_message(&private_process_declared_over_bound_frame(
                codec.limits().max_message_bytes(),
            ))
            .expect_err("declared message body beyond the finite bound must fail closed")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Oversized
    );
}

#[test]
fn g2_private_codec_classifies_versions_and_message_framing_before_candidate_admission() {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();

    let image_bytes = codec
        .encode_image(canonical_image_for_slot(REQUESTER_SLOT))
        .expect("one source-derived image encodes for private codec classification");
    for nested_version_path in [
        PRIVATE_PROCESS_PROJECTION_VERSION_PATH,
        PRIVATE_PROCESS_ADMISSION_VERSION_PATH,
    ] {
        let nested_unknown = replace_private_process_nested_version(
            &image_bytes,
            PRIVATE_PROCESS_IMAGE_ROOT,
            nested_version_path,
        );
        assert_eq!(
            codec
                .decode_untrusted_image(&nested_unknown)
                .expect_err(
                    "a nested projection/admission schema mismatch must reject before image candidate release",
                )
                .kind(),
            // The outer private codec exposes one opaque malformed-candidate
            // class for nested schema failures.  This is deliberately not a
            // public compatibility claim; only the outer version gets the
            // explicit UnknownVersion class.
            Sys5I3PrivateProcessCodecErrorKind::Malformed,
            "nested version path {nested_version_path} follows the private nested-schema policy"
        );
    }

    let (request_codec, request_bytes) = encoded_generated_owner_request();
    let unknown_message_version =
        replace_private_process_version(&request_bytes, PRIVATE_PROCESS_MESSAGE_ROOT);
    assert_eq!(
        request_codec
            .decode_untrusted_message(&unknown_message_version)
            .expect_err("unknown outer message version must remain explicit")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::UnknownVersion
    );
    assert_eq!(
        request_codec
            .decode_untrusted_message(&request_bytes[..request_bytes.len() - 1])
            .expect_err("truncated message frames remain distinct from malformed frames")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Incomplete
    );
    let mut extra_byte_message = request_bytes.clone();
    extra_byte_message.push(0);
    assert_eq!(
        request_codec
            .decode_untrusted_message(&extra_byte_message)
            .expect_err("a complete frame with trailing byte remains malformed, not incomplete")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed
    );
    assert_eq!(
        request_codec
            .decode_untrusted_message(&vec![0_u8; codec.limits().max_message_bytes() + 1])
            .expect_err("actual oversized message bytes remain capacity failures")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Oversized
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_message_codec_keeps_request_reply_receipt_distinct_and_revalidates_at_receiver() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);

    let request_bytes = codec
        .encode_outbound_message(
            requester
                .emit_generated_owner_request("init_avatar_hp")
                .expect("source-derived requester operation emits one generated request"),
        )
        .expect("a trusted outbound request encodes through the private codec");
    assert!(
        request_bytes.len() <= codec.limits().max_message_bytes(),
        "private carrier encoding must enforce the declared message bound"
    );
    assert!(
        codec
            .decode_untrusted_message(&request_bytes)
            .expect("exact request bytes decode only as an untrusted message candidate")
            .observer_safe_manifest()
            .is_request(),
        "request bytes must not collapse into a reply or receipt before receiver admission"
    );

    let owner_summary_before = owner.observer_safe_runtime_summary();
    let owner_outbox_before = owner.observer_safe_outbox_summary();
    for (pointer, mutation_label) in [
        (PRIVATE_PROCESS_MESSAGE_COHORT_PATH, "cohort"),
        (PRIVATE_PROCESS_MESSAGE_EDGE_PATH, "edge"),
        (PRIVATE_PROCESS_MESSAGE_TARGET_PATH, "target"),
        (
            PRIVATE_PROCESS_MESSAGE_CARRIER_PROVENANCE_PATH,
            "carrier-provenance",
        ),
    ] {
        let tampered_bytes = mutate_private_process_string_field(
            &request_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            pointer,
            mutation_label,
        );
        let decoded = codec
            .decode_untrusted_message(&tampered_bytes)
            .expect("tampered bytes remain syntactically decodable only as untrusted input");
        let error = owner.admit_untrusted_message(decoded).expect_err(
            "wrong cohort, edge, target, or provenance must reject before owner execution",
        );
        assert!(
            matches!(
                error.kind(),
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch
                    | Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
            ),
            "each receiver-owned carrier admission failure remains typed"
        );
        assert_eq!(
            owner
                .observer_safe_runtime_summary()
                .served_owner_request_count(),
            owner_summary_before.served_owner_request_count(),
            "rejected untrusted bytes must not produce owner serve linearization"
        );
        assert_eq!(
            owner
                .observer_safe_runtime_summary()
                .actual_owner_write_count(),
            owner_summary_before.actual_owner_write_count(),
            "rejected untrusted bytes must not produce an owner write"
        );
        assert_eq!(
            owner
                .observer_safe_runtime_summary()
                .accepted_inbound_receipt_count(),
            owner_summary_before.accepted_inbound_receipt_count(),
            "rejected untrusted bytes must not mint or accept a receipt"
        );
        assert_eq!(
            owner.observer_safe_outbox_summary().pending_carrier_count(),
            owner_outbox_before.pending_carrier_count(),
            "rejected untrusted bytes must not mint an outbound reply carrier"
        );
    }

    let reply = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&request_bytes)
                .expect("normal request bytes remain untrusted until owner admission"),
        )
        .expect("receiver-owned admission accepts the exact generated request")
        .expect("owner serve returns one generated reply");
    let reply_bytes = codec
        .encode_outbound_message(reply)
        .expect("trusted generated reply encodes through the same private codec");
    assert!(
        codec
            .decode_untrusted_message(&reply_bytes)
            .expect("exact reply bytes decode only as an untrusted candidate")
            .observer_safe_manifest()
            .is_reply(),
        "reply bytes must not collapse into the original request or a local receipt"
    );
    let receipt = requester
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&reply_bytes)
                .expect("normal reply bytes remain untrusted until requester admission"),
        )
        .expect("requester-owned admission accepts the exact generated reply")
        .expect("reply consumption produces one requester-local receipt");
    assert!(receipt.is_observer_safe_typed_result_or_receipt());
    assert!(receipt.has_no_transportable_carrier());
    assert_eq!(
        codec
            .encode_outbound_message(receipt)
            .expect_err("a requester-local receipt must not become a third transport carrier")
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::ReceiptIsLocalOnly
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_request_outer_semantic_identity_must_match_the_exact_source_carrier_before_owner_mutation()
 {
    let mut fixture = pending_private_request_fixture();
    let forged = replace_private_process_string_field(
        &fixture.request_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_REQUEST_IDENTITY_PATH,
        &format!("{}-forged-outer-semantic-id", fixture.request_identity),
    );
    assert_rejected_private_request_preserves_owner_state_and_occurrences(&mut fixture, &forged);
    let exact_request = fixture.request_bytes.clone();
    assert!(
        fixture
            .owner
            .admit_untrusted_message(
                fixture
                    .codec
                    .decode_untrusted_message(&exact_request)
                    .expect("the exact original request remains an untrusted candidate"),
            )
            .expect("the original source-derived request remains admissible after rejection")
            .is_some(),
        "a rejected outer identity candidate must not consume the exact source-derived request"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_request_linked_identity_must_match_the_exact_source_carrier_contract() {
    let mut fixture = pending_private_request_fixture();
    let forged = replace_private_process_optional_string_field(
        &fixture.request_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_LINKED_REQUEST_IDENTITY_PATH,
        "forged-request-linked-identity",
    );
    assert_rejected_private_request_preserves_owner_state_and_occurrences(&mut fixture, &forged);
    let exact_request = fixture.request_bytes.clone();
    assert!(
        fixture
            .owner
            .admit_untrusted_message(
                fixture
                    .codec
                    .decode_untrusted_message(&exact_request)
                    .expect("the exact original request remains an untrusted candidate"),
            )
            .expect("the original source-derived request remains admissible after rejection")
            .is_some(),
        "a rejected linked identity candidate must not consume the exact source-derived request"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_message_outer_kind_must_match_the_exact_carrier_before_owner_or_requester_mutation() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);

    let owner_summary_before = owner.observer_safe_runtime_summary();
    let owner_outbox_before = owner.observer_safe_outbox_summary();
    assert_eq!(
        owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err(
                "owner has no source-derived avatar state before a rejected type-confused request"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    let owner_request_bytes = codec
        .encode_outbound_message(
            requester
                .emit_generated_owner_request("init_avatar_hp")
                .expect("requester emits an exact generated OwnerRequest"),
        )
        .expect("OwnerRequest encodes privately");
    let owner_request_as_reply = replace_private_process_string_field(
        &owner_request_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_KIND_PATH,
        "reply",
    );
    assert_eq!(
        owner
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&owner_request_as_reply)
                    .expect("outer-kind-mutated frame remains syntactically untrusted JSON"),
            )
            .expect_err("an OwnerRequest carrier labelled Reply must reject before owner serve")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("type confusion must not create owner state")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        owner_summary_before.served_owner_request_count()
    );
    assert_eq!(
        owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        owner_summary_before.actual_owner_write_count()
    );
    assert_eq!(
        owner.observer_safe_outbox_summary().pending_carrier_count(),
        owner_outbox_before.pending_carrier_count(),
        "a type-confused request must not mint an owner reply"
    );

    // Establish a normal reply, then retag the outer message as Request while
    // retaining the OwnerReplyReceipt carrier.  The requester must reject
    // before a receipt/serve/outbox transition.
    let request_identity = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("requester emits a second source-derived request")
        .semantic_request_identity_ref()
        .to_string();
    // Re-emit only to obtain bytes; the first request's identity is not used
    // as a receipt claim because no reply has been admitted for it.
    let reply_source_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("requester emits the request whose reply will be retagged");
    let reply = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(
                    &codec
                        .encode_outbound_message(reply_source_request)
                        .expect("normal request encodes"),
                )
                .expect("normal request decodes untrusted"),
        )
        .expect("owner admits the normal generated request")
        .expect("owner produces normal generated reply");
    let reply_as_request = replace_private_process_string_field(
        &codec
            .encode_outbound_message(reply)
            .expect("normal reply encodes"),
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_KIND_PATH,
        "request",
    );
    let requester_summary_before = requester.observer_safe_runtime_summary();
    let requester_outbox_before = requester.observer_safe_outbox_summary();
    assert_eq!(
        requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("requester has no owner state before a retagged reply")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert_eq!(
        requester
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&reply_as_request)
                    .expect("outer-kind-mutated reply remains syntactically untrusted JSON"),
            )
            .expect_err(
                "an OwnerReplyReceipt carrier labelled Request must reject before requester receipt"
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        requester_summary_before.served_owner_request_count()
    );
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        requester_summary_before.actual_owner_write_count()
    );
    assert_eq!(
        requester
            .authoritative_i64_state("avatar", "self", "hp")
            .expect_err("retagged reply must not install owner state in requester")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState
    );
    assert!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(&request_identity)
            .is_none(),
        "retagged reply must not install a requester receipt for any unrelated pending identity"
    );
    assert_eq!(
        requester
            .observer_safe_outbox_summary()
            .pending_carrier_count(),
        requester_outbox_before.pending_carrier_count()
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_outer_kind_must_match_the_exact_reply_carrier_before_receipt_mutation() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let retagged = replace_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_KIND_PATH,
        "request",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &retagged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_requires_exact_locally_pending_request_linkage_and_rejects_replay() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);

    let source_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("requester emits the one locally pending generated request");
    let request_identity = source_request.semantic_request_identity_ref().to_string();
    let reply = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(
                    &codec
                        .encode_outbound_message(source_request)
                        .expect("pending request encodes"),
                )
                .expect("pending request decodes as untrusted input"),
        )
        .expect("owner admits the exact pending request")
        .expect("owner produces one exact reply");
    let reply_bytes = codec
        .encode_outbound_message(reply)
        .expect("exact reply encodes privately");

    let requester_summary_before = requester.observer_safe_runtime_summary();
    let requester_outbox_before = requester.observer_safe_outbox_summary();
    // This codec-only boundary proves structural carrier, pending-request, and
    // M9 lineage integrity.  An arbitrary owner result value cannot be
    // validated here without re-executing owner semantics (forbidden) or an
    // authenticated peer binding; that G2c QUIC-adapter falsifier is kept out
    // of this pre-transport fixture.  Transport identity remains nonauthority.
    let forged_replies = [
        replace_private_process_string_field(
            &reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            PRIVATE_PROCESS_MESSAGE_REQUEST_IDENTITY_PATH,
            "forged-unknown-semantic-request",
        ),
        replace_private_process_string_field(
            &reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            PRIVATE_PROCESS_MESSAGE_LINKED_REQUEST_IDENTITY_PATH,
            "forged-linked-request",
        ),
        replace_private_process_string_field(
            &reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            PRIVATE_PROCESS_MESSAGE_REQUEST_CARRIER_ID_PATH,
            "forged-request-carrier",
        ),
        replace_private_process_optional_string_field(
            &reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            PRIVATE_PROCESS_MESSAGE_M9_OWNER_LINEAGE_PATH,
            "forged-owner-lineage",
        ),
        replace_private_process_string_field(
            &reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            PRIVATE_PROCESS_MESSAGE_CARRIER_PROVENANCE_PATH,
            "forged-core-provenance",
        ),
        replace_private_process_string_field(
            &reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            PRIVATE_PROCESS_MESSAGE_REPLY_RECEIPT_REQUEST_ID_PATH,
            "forged-receipt-request",
        ),
    ];
    for forged_reply in forged_replies {
        assert_eq!(
            requester
                .admit_untrusted_message(
                    codec
                        .decode_untrusted_message(&forged_reply)
                        .expect("forged reply remains syntactically untrusted JSON"),
                )
                .expect_err(
                    "unknown or forged reply lineage must reject before a requester receipt is installed",
                )
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
        );
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .actual_owner_write_count(),
            requester_summary_before.actual_owner_write_count()
        );
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_receipt_count(),
            requester_summary_before.accepted_inbound_receipt_count()
        );
        assert!(
            requester
                .observer_safe_semantic_occurrences()
                .requester_local_receipt_occurrence_ref(&request_identity)
                .is_none(),
            "forged reply must not install a receipt occurrence for the exact locally pending request"
        );
        assert_eq!(
            requester
                .observer_safe_outbox_summary()
                .pending_carrier_count(),
            requester_outbox_before.pending_carrier_count()
        );
    }

    let receipt = requester
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&reply_bytes)
                .expect("untampered reply remains untrusted until requester admission"),
        )
        .expect("the exact locally pending reply is admitted")
        .expect("the exact locally pending reply produces a receipt");
    assert_eq!(
        receipt.linked_request_identity_ref(),
        Some(request_identity.as_str())
    );
    let receipt_occurrence = requester
        .observer_safe_semantic_occurrences()
        .requester_local_receipt_occurrence_ref(&request_identity)
        .expect("accepted exact reply installs one receipt occurrence")
        .to_string();
    assert_eq!(
        requester
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&reply_bytes)
                    .expect("replayed bytes remain syntactically untrusted JSON"),
            )
            .expect_err("replayed reply has no remaining locally pending request")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(&request_identity),
        Some(receipt_occurrence.as_str()),
        "replayed reply must not replace or mint a second receipt occurrence"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_with_unknown_semantic_request_identity_rejects_without_consuming_pending() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let forged = replace_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_REQUEST_IDENTITY_PATH,
        "forged-unknown-semantic-request",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_outer_semantic_identity_must_not_bind_a_different_pending_request() {
    let mut fixture = pending_private_reply_fixture();
    let first_identity = fixture.request_identity.clone();
    let second_identity = fixture
        .requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the same requester may have a second independently pending generated request")
        .semantic_request_identity_ref()
        .to_string();
    assert_ne!(
        first_identity, second_identity,
        "distinct source-derived requests must retain distinct semantic identities"
    );
    let forged = replace_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_REQUEST_IDENTITY_PATH,
        &second_identity,
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &first_identity,
    );
    assert!(
        fixture
            .requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(&second_identity)
            .is_none(),
        "a reply carrier for the first request must not install a receipt for another pending request"
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_linked_request_identity_must_match_the_pending_request() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let forged = replace_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_LINKED_REQUEST_IDENTITY_PATH,
        "forged-linked-request",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_original_request_carrier_must_match_the_pending_request() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let forged = replace_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_REQUEST_CARRIER_ID_PATH,
        "forged-request-carrier",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_owner_lineage_must_not_be_forged_at_the_requester_boundary() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let forged = replace_private_process_optional_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_M9_OWNER_LINEAGE_PATH,
        "forged-owner-lineage",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_source_derived_core_provenance_must_match_the_receiver_image() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let forged = mutate_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_CARRIER_PROVENANCE_PATH,
        "forged-core-provenance",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_edge_loci_and_operation_must_match_the_receiver_image_before_receipt() {
    for (pointer, label) in [
        (PRIVATE_PROCESS_MESSAGE_EDGE_PATH, "edge"),
        (PRIVATE_PROCESS_MESSAGE_SOURCE_PATH, "source locus"),
        (PRIVATE_PROCESS_MESSAGE_TARGET_PATH, "target locus"),
        (PRIVATE_PROCESS_MESSAGE_OPERATION_PATH, "operation"),
    ] {
        let mut fixture = pending_private_reply_fixture();
        let request_identity = fixture.request_identity.clone();
        let forged = mutate_private_process_string_field(
            &fixture.reply_bytes,
            PRIVATE_PROCESS_MESSAGE_ROOT,
            pointer,
            &format!("forged-reply-{label}"),
        );
        assert_rejected_private_reply_preserves_requester_pending_state(
            &mut fixture,
            &forged,
            &request_identity,
        );
        let _ = admit_exact_private_reply(&mut fixture);
    }
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_receipt_request_identity_must_match_the_pending_request() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let forged = replace_private_process_string_field(
        &fixture.reply_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_REPLY_RECEIPT_REQUEST_ID_PATH,
        "forged-receipt-request",
    );
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &forged,
        &request_identity,
    );
    let _ = admit_exact_private_reply(&mut fixture);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn g2_private_reply_replay_after_exact_local_receipt_rejects_without_replacing_occurrence() {
    let mut fixture = pending_private_reply_fixture();
    let request_identity = fixture.request_identity.clone();
    let receipt_occurrence = admit_exact_private_reply(&mut fixture);
    let replayed_bytes = fixture.reply_bytes.clone();
    assert_rejected_private_reply_preserves_requester_pending_state(
        &mut fixture,
        &replayed_bytes,
        &request_identity,
    );
    assert_eq!(
        fixture
            .requester
            .observer_safe_semantic_occurrences()
            .requester_local_receipt_occurrence_ref(&request_identity),
        Some(receipt_occurrence.as_str()),
        "a replay must preserve the exact first requester-local receipt occurrence"
    );
}

#[test]
fn g2_private_message_decoder_rejects_raw_json_duplicate_members_without_last_wins() {
    assert_private_message_duplicate_member_rejected("envelope", "version");
}

fn assert_private_message_duplicate_member_rejected(level: &str, member: &str) {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let request_bytes = codec
        .encode_outbound_message(
            requester
                .emit_generated_owner_request("init_avatar_hp")
                .expect("source-derived request emits"),
        )
        .expect("source-derived request encodes");
    let duplicate = private_process_frame_with_duplicate_member(&request_bytes, level, member);
    assert_eq!(
        codec
            .decode_untrusted_message(&duplicate)
            .expect_err(
                "duplicate private JSON members must reject before serde value decoding can choose a last value",
            )
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed,
        "duplicate {level}.{member} must fail closed"
    );
}

#[test]
fn g2_private_message_decoder_rejects_raw_duplicate_envelope_message() {
    assert_private_message_duplicate_member_rejected("envelope", "message");
}

#[test]
fn g2_private_message_decoder_rejects_raw_duplicate_message_kind() {
    assert_private_message_duplicate_member_rejected("message", "kind");
}

#[test]
fn g2_private_message_decoder_rejects_raw_duplicate_message_cohort_ref() {
    assert_private_message_duplicate_member_rejected("message", "cohort_provenance_ref");
}

#[test]
fn g2_private_message_decoder_rejects_raw_duplicate_carrier_core_ref() {
    assert_private_message_duplicate_member_rejected("carrier", "core_ref");
}

#[test]
fn g2_private_message_decoder_rejects_raw_duplicate_carrier_edge_ref() {
    assert_private_message_duplicate_member_rejected("carrier", "edge_ref");
}

#[test]
fn g2_private_message_decoder_rejects_raw_duplicate_tagged_payload_kind() {
    assert_private_message_duplicate_member_rejected("payload", "kind");
}

#[test]
fn g2_private_image_decoder_rejects_duplicate_assigned_locus_before_candidate_creation() {
    assert_private_image_duplicate_collection_rejected(
        PRIVATE_PROCESS_IMAGE_ASSIGNED_LOCI_PATH,
        "assigned locus",
    );
}

#[test]
fn g2_private_image_decoder_rejects_duplicate_semantic_row_before_candidate_creation() {
    assert_private_image_duplicate_collection_rejected(
        PRIVATE_PROCESS_IMAGE_SEMANTIC_ROWS_PATH,
        "semantic authority row",
    );
}

#[test]
fn g2_private_image_decoder_accepts_ci_bounded_unique_collections_only_as_untrusted_candidates() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let expected_start_binding = cohort
        .parent_held_expected_start_binding(REQUESTER_SLOT)
        .expect("the coordinator retains the exact requester image binding");
    let image = take_process_image(&mut cohort, REQUESTER_SLOT);
    let original = codec
        .encode_image(image)
        .expect("the source-derived one-shot image encodes");
    let unique = append_ci_safe_unique_image_inventory(&original);
    assert!(
        unique.len() <= codec.limits().max_image_bytes(),
        "the CI-safe adversarial inventory must remain inside the private image byte bound"
    );
    let candidate = codec
        .decode_untrusted_image(&unique)
        .expect("a bounded unique collection must not be rejected merely as a duplicate");
    assert_eq!(
        candidate.observer_safe_manifest().assigned_loci().len(),
        2 + CI_SAFE_UNIQUE_IMAGE_COLLECTION_ITEMS,
        "the untrusted decoder must preserve the bounded unique collection rather than collapse it"
    );
    assert_eq!(
        codec
            .validate_and_start_image(candidate, expected_start_binding)
            .expect_err(
                "a large unique untrusted inventory still cannot bypass the parent-held exact image binding",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
        "collection scalability does not turn a decoded candidate into a startable image"
    );
}

fn assert_private_image_duplicate_collection_rejected(pointer: &str, label: &str) {
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let bytes = codec
        .encode_image(canonical_image_for_slot(REQUESTER_SLOT))
        .expect("an exact one-shot child image encodes for collection duplicate mutation");
    let duplicate =
        duplicate_private_process_array_element(&bytes, PRIVATE_PROCESS_IMAGE_ROOT, pointer);
    assert_eq!(
        codec
            .decode_untrusted_image(&duplicate)
            .expect_err(
                "duplicate private image collections must reject before an untrusted candidate or child start exists",
            )
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed,
        "duplicate {label} must not be silently normalized by a set/vector decoder"
    );
}

#[test]
fn g2_private_message_decoder_rejects_unknown_nested_tagged_payload_field_before_admission() {
    let (codec, request_bytes) = encoded_generated_owner_request();
    let tagged_unknown_field = append_private_process_unknown_object_member(
        &request_bytes,
        PRIVATE_PROCESS_MESSAGE_ROOT,
        PRIVATE_PROCESS_MESSAGE_PAYLOAD_PATH,
        "untrusted_tagged_payload_field",
    );
    assert_eq!(
        codec
            .decode_untrusted_message(&tagged_unknown_field)
            .expect_err(
                "unknown nested tagged-payload fields must reject before receiver admission",
            )
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed
    );
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
#[cfg(feature = "i3-process-test-seams")]
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
