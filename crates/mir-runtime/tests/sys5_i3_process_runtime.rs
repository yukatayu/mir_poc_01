//! RED contracts for I3-2 process images and the pre-socket process-runtime
//! seam.  The named surface is deliberately `doc(hidden)`/provisional: this
//! file specifies only the bounded I3-2 implementation seam, never a public
//! artifact, deployment, carrier, or runtime API.

use std::collections::BTreeSet;

#[cfg(all(unix, feature = "i3-process-test-seams"))]
use std::{
    io::Write,
    os::unix::net::UnixStream,
    thread,
    time::{Duration, Instant},
};

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

#[cfg(all(unix, feature = "i3-process-test-seams"))]
use mir_runtime::sys5_i3_process_runtime::Sys5I3ObserverSafeLifecycleOrigin;
#[cfg(feature = "i3-process-test-seams")]
use mir_runtime::sys5_i3_process_runtime::Sys5I3ProcessImageTamper;
#[cfg(all(unix, feature = "i3-process-test-seams"))]
use mir_runtime::sys5_i3_process_runtime::Sys5I3RegisteredOwnerLifecycleAckReader;
#[cfg(feature = "i3-process-test-seams")]
use mir_runtime::sys5_i3_process_runtime::{
    Sys5I3LifecyclePublicationOutcome, Sys5I3OwnerCapabilitySuccessorTamper,
};
#[cfg(feature = "i3-process-test-seams")]
use mir_runtime::sys5_local_slice::Sys5I3AdapterCarrierContract;

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
const PRIVATE_PROCESS_IMAGE_OWNER_ADMISSION_BUDGET_PATH: &str =
    "/admission/instance/owner_execution_plans/0/owner_admission_budget";
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

/// Derive the exact checked owner-request contract from ordinary source.  The
/// test never manufactures an operation, carrier, capability, or authority
/// input for the M9 prestage boundary.
#[cfg(feature = "i3-process-test-seams")]
fn checked_owner_request_contract(project: &Sys5LocalProject) -> Sys5I3AdapterCarrierContract {
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.operation_id == "init_avatar_hp" && edge.kind == "owner-request")
        .expect("the canonical source must retain its checked generated owner-request edge");
    project
        .i3_adapter_carrier_contract(&edge.edge_ref)
        .expect("the checked generated owner-request edge has its exact adapter contract")
}

#[cfg(feature = "i3-process-test-seams")]
fn checked_owner_reply_contract(project: &Sys5LocalProject) -> Sys5I3AdapterCarrierContract {
    let edge = project
        .semantic_summary()
        .generated_communication
        .iter()
        .find(|edge| edge.operation_id == "init_avatar_hp" && edge.kind == "owner-reply-receipt")
        .expect("the canonical source must retain its distinct checked owner-reply edge");
    project
        .i3_adapter_carrier_contract(&edge.edge_ref)
        .expect("the checked generated owner-reply edge has its own adapter contract")
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

fn owner_admission_budget_owner_only_source(ticks: u64) -> String {
    OWNER_ONLY_SOURCE.replacen(
        "MissingWitness, VisibilityDenied, RouteUnavailable) {",
        &format!(
            "MissingWitness, VisibilityDenied, RouteUnavailable, DeadlineExpired) within owner_ticks {ticks} {{"
        ),
        1,
    )
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

#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn private_owner_lifecycle_ack_v2_json(frame: &[u8]) -> Value {
    assert!(
        frame.len() >= PRIVATE_PROCESS_CODEC_PREFIX_BYTES,
        "an installed owner lifecycle ACK frame begins with the fixed four-byte length prefix"
    );
    let declared = u32::from_be_bytes(
        frame[..PRIVATE_PROCESS_CODEC_PREFIX_BYTES]
            .try_into()
            .expect("private ACK codec prefix has exactly four bytes"),
    );
    assert_eq!(
        declared as usize,
        frame.len() - PRIVATE_PROCESS_CODEC_PREFIX_BYTES,
        "the genuine B-installed ACK frame must declare its exact JSON body length"
    );
    let value: Value = serde_json::from_slice(&frame[PRIVATE_PROCESS_CODEC_PREFIX_BYTES..])
        .expect("the genuine B-installed ACK frame body is JSON for test-local wire mutation");
    assert_eq!(
        value.pointer("/version").and_then(Value::as_u64),
        Some(2),
        "the installed ACK component regression intentionally mutates only the current private ACKv2 envelope"
    );
    for field in [
        "stage_identity_binding_ref",
        "prior_generation_ref",
        "successor_generation_ref",
        "candidate_binding_ref",
    ] {
        assert!(
            value
                .pointer(&format!("/{field}"))
                .and_then(Value::as_str)
                .is_some_and(|value| !value.is_empty()),
            "the genuine installed ACKv2 frame retains nonempty {field} before the test mutates exactly that one field"
        );
    }
    value
}

#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn mutate_private_owner_lifecycle_ack_v2_binding(
    genuine_frame: &[u8],
    field: &str,
    mutation_label: &str,
) -> Vec<u8> {
    let mut value = private_owner_lifecycle_ack_v2_json(genuine_frame);
    let object = value
        .as_object_mut()
        .expect("the private ACKv2 envelope remains a JSON object for one-field wire corruption");
    let original = object
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("private ACKv2 must expose string field {field}"));
    object.insert(
        field.to_string(),
        Value::String(format!("{original}-{mutation_label}")),
    );
    private_process_json_frame(&value)
}

#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn genuine_prestaged_owner_lifecycle_ack_frame(
    cohort: &mut Sys5I3ProcessCohort,
    codec: &Sys5I3PrivateProcessCodec,
    run_ref: &str,
    requester_spki_ref: &str,
    owner_spki_ref: &str,
) -> Vec<u8> {
    let (_requester_control, owner_control) = cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            codec,
            run_ref,
            REQUESTER_SLOT,
            requester_spki_ref,
            OWNER_SLOT,
            owner_spki_ref,
        )
        .expect("the staged cohort supplies matching B bootstrap control only for its genuine installed ACK frame");
    let owner_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(take_process_image(cohort, OWNER_SLOT))
                .expect("the genuine B image serializes as tainted child input"),
        )
        .expect("the genuine B image crosses the private child codec boundary");
    let (mut owner, _owner_control, owner_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(owner_image, owner_control)
        .expect(
            "only the matching B image and trusted control produce the opaque lifecycle stimulus",
        );
    let installed_ack = owner
        .install_admitted_owner_capability_successor(
            owner_stimulus.expect("the selected genuine B owns the one install stimulus"),
        )
        .expect("the actual B install produces the sole installed-receipt ACK wrapper");
    let installed = owner
        .observer_safe_installed_owner_capability_lifecycle()
        .expect("the genuine installed B exposes only bounded lifecycle observation");
    assert_eq!(
        installed.origin(),
        Sys5I3ObserverSafeLifecycleOrigin::M9AdmittedLifecycle
    );
    assert!(!installed.source_derived());
    codec
        .encode_installed_owner_lifecycle_ack(installed_ack)
        .expect("only a genuine B installed receipt serializes the ACKv2 frame")
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
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_prestaged_owner_capability_revocation_accepts_only_the_exact_checked_prelaunch_contract() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let exact_owner_request = checked_owner_request_contract(&project);
    let distinct_checked_reply = checked_owner_reply_contract(&project);

    let mut wrong_contract_cohort = single_coordinator_cohort(&project, &deployment);
    assert_eq!(
        wrong_contract_cohort
            .prestage_owner_capability_revocation(
                "i3-3-prestage-exact-contract",
                &distinct_checked_reply,
            )
            .expect_err(
                "a checked reply contract must not select or replace the exact owner-request capability",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
        "prestage rejects the wrong checked generated edge without accepting caller-selected authority"
    );
    let wrong_contract_owner =
        Sys5I3ProcessRuntime::start(take_process_image(&mut wrong_contract_cohort, OWNER_SLOT))
            .expect("a rejected prelaunch request leaves the original G1 owner image startable");
    assert_candidate_a_child_runtime(&wrong_contract_owner);

    let mut after_start_binding_cohort = single_coordinator_cohort(&project, &deployment);
    let _consumed_binding = after_start_binding_cohort
        .parent_held_expected_start_binding(OWNER_SLOT)
        .expect("the owner start binding may be deliberately consumed for this ordering negative");
    assert_eq!(
        after_start_binding_cohort
            .prestage_owner_capability_revocation(
                "i3-3-prestage-after-binding",
                &exact_owner_request,
            )
            .expect_err(
                "prestage is forbidden after any expected start binding has left parent control",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
        "the lifecycle candidate must be sealed before image/bootstrap handoff, never retrofitted into a child"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_prestaged_owner_capability_revocation_binds_the_staged_run_before_controls_are_consumed() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let staged_run = "i3-3-prestage-run-a";
    let mismatched_split_run = "i3-3-prestage-run-b";
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let parent_g1_generation = cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();

    cohort
        .prestage_owner_capability_revocation(staged_run, &contract)
        .expect("the exact checked owner-request contract stages one parent-held lifecycle candidate for Run A");

    let mismatched_split_error = match cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            mismatched_split_run,
            REQUESTER_SLOT,
            "requester-spki:i3-3-prestage-run-a",
            OWNER_SLOT,
            "owner-spki:i3-3-prestage-run-a",
        ) {
        Ok(_) => panic!(
            "a control split for Run B must not consume the candidate or trusted bindings staged for Run A"
        ),
        Err(error) => error,
    };
    assert_eq!(
        mismatched_split_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected,
        "the retained prestage run reference is part of the prelaunch lifecycle binding"
    );

    let after_rejected_split = cohort.observer_safe_lifecycle_publication_summary();
    assert_eq!(
        after_rejected_split.published_authority_generation_ref(),
        parent_g1_generation.as_str(),
        "a mismatched control split cannot publish a successor generation"
    );
    assert_eq!(
        after_rejected_split.publication_outcome(),
        None,
        "a rejected Run B split is neither an acknowledged publication nor a terminal lost-ACK outcome"
    );

    let _controls = cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            staged_run,
            REQUESTER_SLOT,
            "requester-spki:i3-3-prestage-run-a",
            OWNER_SLOT,
            "owner-spki:i3-3-prestage-run-a",
        )
        .expect(
            "the rejected Run B attempt must leave the exact Run A trusted controls and parent-held bindings consumable once",
        );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_prestaged_owner_capability_revocation_installs_only_from_b_opaque_stimulus_without_a_child_issuer()
 {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let parent_g1_generation = cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();

    cohort
        .prestage_owner_capability_revocation("i3-3-prestage-install", &contract)
        .expect("the parent accepts the exact checked owner-request contract before any image/control handoff");
    let pending_publication = cohort.observer_safe_lifecycle_publication_summary();
    assert_eq!(
        pending_publication.published_authority_generation_ref(),
        parent_g1_generation.as_str(),
        "prestage alone must retain G1 at the parent until a B-installed lifecycle acknowledgement completes publication"
    );
    assert_eq!(pending_publication.publication_outcome(), None);

    let (requester_control, owner_control) = cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            "i3-3-prestage-install",
            REQUESTER_SLOT,
            "requester-spki:i3-3-prestage-install",
            OWNER_SLOT,
            "owner-spki:i3-3-prestage-install",
        )
        .expect("the parent splits two independently bound controls only after the prelaunch candidate is sealed");
    let requester_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(take_process_image(&mut cohort, REQUESTER_SLOT))
                .expect("the source-derived requester image encodes as tainted child input"),
        )
        .expect("the requester image remains only an untrusted decode candidate");
    let owner_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(take_process_image(&mut cohort, OWNER_SLOT))
                .expect("the source-derived owner image encodes as tainted child input"),
        )
        .expect("the owner image remains only an untrusted decode candidate");
    let (requester, _requester_control, requester_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(requester_image, requester_control)
        .expect("the independently trusted requester control starts only its matching image");
    assert!(
        requester_stimulus.is_none(),
        "the non-owner child receives no lifecycle stimulus"
    );
    assert_candidate_a_child_runtime(&requester);

    let (mut owner, _owner_control, owner_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(owner_image, owner_control)
        .expect("the independently trusted owner control starts only its matching image");
    let owner_stimulus = owner_stimulus.expect(
        "only the selected owner child receives the opaque, one-use prestaged lifecycle stimulus",
    );
    assert_candidate_a_child_runtime(&owner);
    let _install_ack = owner
        .install_admitted_owner_capability_successor(owner_stimulus)
        .expect("B installs the exact restricted G1-to-G2 successor only by consuming its opaque stimulus");
    assert_candidate_a_child_runtime(&owner);

    let post_install_publication = cohort.observer_safe_lifecycle_publication_summary();
    assert_eq!(
        post_install_publication.published_authority_generation_ref(),
        parent_g1_generation.as_str(),
        "a locally installed B successor is not a parent publication without the registered B-FD acknowledgement path"
    );
    assert_eq!(post_install_publication.publication_outcome(), None);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_prestaged_owner_capability_install_rejects_after_a_real_g1_owner_admission_changes_its_prior()
 {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let parent_g1_generation = cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();
    cohort
        .prestage_owner_capability_revocation("i3-3-stale-install", &contract)
        .expect(
            "the exact source-derived owner contract stages the bounded successor before bootstrap",
        );
    let (requester_control, owner_control) = cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            "i3-3-stale-install",
            REQUESTER_SLOT,
            "requester-spki:i3-3-stale-install",
            OWNER_SLOT,
            "owner-spki:i3-3-stale-install",
        )
        .expect(
            "the staged lifecycle splits into independently bound requester and owner controls",
        );
    let requester_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(take_process_image(&mut cohort, REQUESTER_SLOT))
                .expect("the checked requester image encodes for bounded private bootstrap"),
        )
        .expect("the requester image decodes only as a tainted candidate");
    let owner_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(take_process_image(&mut cohort, OWNER_SLOT))
                .expect("the checked owner image encodes for bounded private bootstrap"),
        )
        .expect("the owner image decodes only as a tainted candidate");
    let (mut requester, _requester_control, requester_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(requester_image, requester_control)
        .expect("the matching requester control starts its source-derived G1 runtime");
    assert!(requester_stimulus.is_none());
    let (mut owner, _owner_control, owner_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(owner_image, owner_control)
        .expect("the matching owner control starts its source-derived G1 runtime");
    let owner_stimulus = owner_stimulus.expect(
        "the selected owner receives one opaque G2 install stimulus but has not consumed it yet",
    );
    assert_candidate_a_child_runtime(&requester);
    assert_candidate_a_child_runtime(&owner);

    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect(
            "ordinary checked source emits the real G1 owner request before the delayed install",
        );
    let request_identity = request.semantic_request_identity_ref().to_string();
    let request_bytes = codec
        .encode_outbound_message(request)
        .expect("the real source-derived request encodes through the private ingress codec");
    let reply = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&request_bytes)
                .expect("the private bytes remain untrusted until the G1 owner admission boundary"),
        )
        .expect(
            "the started G1 owner performs its one genuine source-derived admission before install",
        )
        .expect("that genuine owner admission creates one reply without a requester receipt");
    assert_eq!(reply.semantic_request_identity_ref(), request_identity);
    let owner_after_g1_admission = owner.observer_safe_runtime_summary();
    let owner_occurrences_after_g1_admission = owner.observer_safe_semantic_occurrences();
    let owner_hp_after_g1_admission = owner
        .authoritative_i64_state("avatar", "self", "hp")
        .expect("the one real G1 owner admission writes its ordinary source-derived state");
    assert_eq!(owner_after_g1_admission.served_owner_request_count(), 1);
    assert_eq!(owner_after_g1_admission.actual_owner_write_count(), 1);
    assert_eq!(owner_hp_after_g1_admission, 21);
    assert_eq!(
        requester
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        0,
        "the reply remains undelivered to A, so this setup cannot mint a receipt before the stale install is tested"
    );

    let install_error = match owner.install_admitted_owner_capability_successor(owner_stimulus) {
        Ok(_) => panic!(
            "a prestaged successor whose exact G1 prior changed through real owner admission must not produce an installed receipt or ACK"
        ),
        Err(error) => error,
    };
    assert_eq!(
        install_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecycleInstallRejected,
        "the stale exact-prior check must fail at install rather than accepting a successor with outdated M9 observations"
    );
    assert_eq!(
        owner.observer_safe_runtime_summary(),
        owner_after_g1_admission,
        "rejected install must preserve the one real G1 serve/write exactly and add no mutation, receipt, or lifecycle side effect"
    );
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        owner_occurrences_after_g1_admission,
        "rejected install must preserve the completed G1 use occurrences without minting or replacing one"
    );
    assert_eq!(
        owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect("rejected install cannot erase the prior real G1 store write"),
        owner_hp_after_g1_admission
    );
    let publication = cohort.observer_safe_lifecycle_publication_summary();
    assert_eq!(
        publication.published_authority_generation_ref(),
        parent_g1_generation.as_str(),
        "a failed install leaves parent publication at the retained G1 generation"
    );
    assert_eq!(
        publication.publication_outcome(),
        None,
        "without an installed receipt there is no ACK completion or terminal parent publication outcome"
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_prestage_exact_successor_predicate_rejects_added_or_reanimated_authority_evidence() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);

    // The fresh cohort has no earlier revoked-owner tombstone.  The distinct
    // private M9 predicate unit will establish that genuine prior-tombstone
    // baseline before attempting its removal; this integration test covers
    // only mutations meaningful from G1.
    for tamper in [
        Sys5I3OwnerCapabilitySuccessorTamper::AddUnrelatedOwnerLineage,
        Sys5I3OwnerCapabilitySuccessorTamper::ReanimateSelectedCapability,
        Sys5I3OwnerCapabilitySuccessorTamper::ReanimateSelectedWitness,
    ] {
        let mut cohort = single_coordinator_cohort(&project, &deployment);
        let parent_g1_generation = cohort
            .observer_safe_lifecycle_publication_summary()
            .published_authority_generation_ref()
            .to_string();
        assert_eq!(
            cohort
                .test_only_prestage_owner_capability_revocation_with_tamper(
                    "i3-3-prestage-predicate-falsifier",
                    &contract,
                    tamper,
                )
                .expect_err(
                    "an internally generated successor with any extra, reanimated, or removed authority evidence must fail the exact M9 predicate",
                )
                .kind(),
            Sys5I3ProcessRuntimeErrorKind::LifecyclePrestageRejected
        );
        let publication = cohort.observer_safe_lifecycle_publication_summary();
        assert_eq!(
            publication.published_authority_generation_ref(),
            parent_g1_generation.as_str(),
            "a failed exact-successor predicate must not publish a replacement generation"
        );
        assert_eq!(
            publication.publication_outcome(),
            Some(Sys5I3LifecyclePublicationOutcome::NoPrestageSelected)
        );
        let owner = Sys5I3ProcessRuntime::start(take_process_image(&mut cohort, OWNER_SLOT))
            .expect("a failed prestage leaves the original G1 owner image usable");
        assert_candidate_a_child_runtime(&owner);
    }
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_tainted_correct_field_ack_decode_does_not_complete_prestaged_publication() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let parent_g1_generation = cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();
    cohort
        .prestage_owner_capability_revocation("i3-3-tainted-ack", &contract)
        .expect("the parent may stage the exact checked owner capability before testing tainted acknowledgement input");

    let candidate_derived_bytes = cohort
        .test_only_encode_prestaged_owner_lifecycle_ack_candidate(&codec)
        .expect(
            "the negative-only seam emits syntactically correct candidate-derived bytes without an installed receipt",
        );
    let _tainted_ack = codec
        .decode_owner_lifecycle_ack(&candidate_derived_bytes)
        .expect("correct-field candidate bytes decode only to a tainted acknowledgement candidate");
    let publication = cohort.observer_safe_lifecycle_publication_summary();
    assert_eq!(
        publication.published_authority_generation_ref(),
        parent_g1_generation.as_str(),
        "decoding self-consistent candidate fields alone leaves parent G1 unchanged; only the registered B-FD completion route may publish G2"
    );
    assert_eq!(publication.publication_outcome(), None);
}

#[test]
#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn i3_3_registered_b_completion_cannot_publish_an_identically_staged_separate_cohort() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let run_ref = "i3-3-cross-cohort-completion";
    let reader_setup_budget = Duration::from_secs(5);
    let mut originating_cohort = single_coordinator_cohort(&project, &deployment);
    let mut separate_cohort = single_coordinator_cohort(&project, &deployment);
    let originating_g1 = originating_cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();
    let separate_g1 = separate_cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();

    originating_cohort
        .prestage_owner_capability_revocation(run_ref, &contract)
        .expect("the first coordinator stages the exact source-derived owner revocation");
    separate_cohort
        .prestage_owner_capability_revocation(run_ref, &contract)
        .expect(
            "an independently constructed coordinator may stage the same checked source without sharing completion authority",
        );

    let (parent_ack_stream, mut registered_b_ack_writer) = UnixStream::pair()
        .expect("the component test creates one real local Unix-stream descriptor pair");
    let reader_registration_started = Instant::now();
    let mut originating_reader = originating_cohort
        .take_registered_owner_lifecycle_ack_reader(
            OWNER_SLOT,
            parent_ack_stream,
            reader_setup_budget,
        )
        .expect("only the originating parent cohort may register its B-owned ACK reader");
    let (_requester_control, owner_control) = originating_cohort
        .split_trusted_localnet_controls_with_prestaged_lifecycle(
            &codec,
            run_ref,
            REQUESTER_SLOT,
            "requester-spki:i3-3-cross-cohort",
            OWNER_SLOT,
            "owner-spki:i3-3-cross-cohort",
        )
        .expect("the originating cohort alone consumes its matching prelaunch bindings");
    let owner_image = codec
        .decode_untrusted_image(
            &codec
                .encode_image(take_process_image(&mut originating_cohort, OWNER_SLOT))
                .expect("the originating B image serializes as tainted child input"),
        )
        .expect("the originating B image decodes only through the private child boundary");
    let (mut owner, _owner_control, owner_stimulus) = codec
        .validate_and_start_image_with_prestaged_lifecycle(owner_image, owner_control)
        .expect("the matching originating B control starts its locally bounded runtime");
    let installed_ack = owner
        .install_admitted_owner_capability_successor(
            owner_stimulus.expect(
                "the genuine selected B receives the one opaque prestaged lifecycle stimulus",
            ),
        )
        .expect("only a genuine B install may create the non-forgeable installed receipt wrapper");
    let installed = owner
        .observer_safe_installed_owner_capability_lifecycle()
        .expect("a successful genuine B install has bounded observer-safe lifecycle evidence");
    assert_eq!(
        installed.origin(),
        Sys5I3ObserverSafeLifecycleOrigin::M9AdmittedLifecycle
    );
    assert!(!installed.source_derived());

    let framed_installed_ack = codec
        .encode_installed_owner_lifecycle_ack(installed_ack)
        .expect("only the genuine installed receipt serializes one ACK frame for B's inherited descriptor");
    registered_b_ack_writer
        .write_all(&framed_installed_ack)
        .expect("the test supplies the actual installed B frame only through the reader's paired descriptor");
    let originating_completion_result = originating_reader.read_next_completion(&codec);
    let elapsed_since_reader_registration = reader_registration_started.elapsed();
    let originating_completion = match originating_completion_result {
        Ok(completion) => completion,
        Err(error) => {
            let error_kind = error.kind();
            panic!(
                "the originating registered reader rejected its genuine B-installed ACK frame after {elapsed_since_reader_registration:?} of its finite {reader_setup_budget:?} setup budget; this safe elapsed value diagnoses only the bounded reader lifecycle, not ACK/image contents; typed kind: {error_kind:?}"
            )
        }
    };

    let foreign_publication_error = match separate_cohort
        .publish_registered_owner_lifecycle_completion(originating_completion)
    {
        Ok(()) => panic!(
            "a completion accepted from the originating cohort must not authorize publication in an identically staged separate cohort"
        ),
        Err(error) => error,
    };
    assert_eq!(
        foreign_publication_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
        "registered completion provenance binds the staged run and originating cohort, not merely candidate-shaped M9 fields"
    );

    for (label, cohort, expected_g1) in [
        ("originating", &originating_cohort, originating_g1.as_str()),
        ("separate", &separate_cohort, separate_g1.as_str()),
    ] {
        let publication = cohort.observer_safe_lifecycle_publication_summary();
        assert_eq!(
            publication.published_authority_generation_ref(),
            expected_g1,
            "the {label} cohort retains its parent-held G1 after a cross-cohort completion rejection"
        );
        assert_eq!(
            publication.publication_outcome(),
            None,
            "the {label} cohort remains pending rather than publishing or reporting a terminal ACK outcome"
        );
    }
}

#[test]
#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn i3_3_registered_b_reader_rejects_each_mutated_genuine_ack_v2_binding_before_completion() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();

    for (field, label) in [
        ("stage_identity_binding_ref", "stage identity"),
        ("prior_generation_ref", "prior generation"),
        ("successor_generation_ref", "successor generation"),
        ("candidate_binding_ref", "candidate binding"),
    ] {
        let run_ref = format!("i3-3-mutated-ack-v2-{field}");
        let mut cohort = single_coordinator_cohort(&project, &deployment);
        let parent_g1 = cohort
            .observer_safe_lifecycle_publication_summary()
            .published_authority_generation_ref()
            .to_string();
        cohort
            .prestage_owner_capability_revocation(&run_ref, &contract)
            .expect("the component negative starts from one genuinely staged M9 successor");

        let (parent_ack_stream, mut registered_b_ack_writer) = UnixStream::pair()
            .expect("each field mutation uses one real parent/B Unix-stream pair");
        let mut reader = cohort
            .take_registered_owner_lifecycle_ack_reader(
                OWNER_SLOT,
                parent_ack_stream,
                Duration::from_secs(5),
            )
            .expect(
                "the parent registers only the genuine B descriptor before any ACK bytes exist",
            );
        let genuine_frame = genuine_prestaged_owner_lifecycle_ack_frame(
            &mut cohort,
            &codec,
            &run_ref,
            "requester-spki:i3-3-mutated-ack-v2",
            "owner-spki:i3-3-mutated-ack-v2",
        );
        let mutated_frame =
            mutate_private_owner_lifecycle_ack_v2_binding(&genuine_frame, field, "test-mutation");
        registered_b_ack_writer
            .write_all(&mutated_frame)
            .expect("the test routes the mutated genuine frame only through the real registered B descriptor");
        let rejection = match reader.read_next_completion(&codec) {
            Ok(_) => panic!(
                "a genuine ACKv2 frame with a mutated {label} must reject before creating an opaque registered completion"
            ),
            Err(error) => error,
        };
        assert_eq!(
            rejection.kind(),
            Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
            "the registered reader must validate the exact {label} binding rather than accept self-consistent JSON"
        );

        let publication = cohort.observer_safe_lifecycle_publication_summary();
        assert_eq!(
            publication.published_authority_generation_ref(),
            parent_g1.as_str(),
            "a rejected {label} frame cannot move the parent from its held G1"
        );
        assert_eq!(
            publication.publication_outcome(),
            None,
            "a rejected {label} frame yields no opaque completion and therefore no publication transition"
        );
    }
}

#[test]
#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn i3_3_wrong_slot_ack_registration_leaves_the_genuine_b_registration_usable() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let contract = checked_owner_request_contract(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let run_ref = "i3-3-wrong-slot-ack-registration";
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let parent_g1 = cohort
        .observer_safe_lifecycle_publication_summary()
        .published_authority_generation_ref()
        .to_string();
    cohort
        .prestage_owner_capability_revocation(run_ref, &contract)
        .expect("the wrong-slot negative begins from one exact staged B lifecycle");

    let (wrong_parent_stream, _wrong_slot_writer) =
        UnixStream::pair().expect("the wrong-slot attempt receives a disposable Unix descriptor");
    let wrong_slot_rejection = match cohort.take_registered_owner_lifecycle_ack_reader(
        REQUESTER_SLOT,
        wrong_parent_stream,
        Duration::from_secs(5),
    ) {
        Ok(_) => panic!("a requester-slot descriptor must not register as the staged B ACK route"),
        Err(error) => error,
    };
    assert_eq!(
        wrong_slot_rejection.kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
        "a wrong slot must reject before taking the cohort's exact B ACK registration"
    );

    let (parent_ack_stream, mut registered_b_ack_writer) =
        UnixStream::pair().expect("the genuine B registration receives a fresh Unix descriptor");
    let mut reader = cohort
        .take_registered_owner_lifecycle_ack_reader(
            OWNER_SLOT,
            parent_ack_stream,
            Duration::from_secs(5),
        )
        .expect("a prior wrong-slot attempt must not consume the genuine B registration");
    let genuine_frame = genuine_prestaged_owner_lifecycle_ack_frame(
        &mut cohort,
        &codec,
        run_ref,
        "requester-spki:i3-3-wrong-slot-ack-registration",
        "owner-spki:i3-3-wrong-slot-ack-registration",
    );
    registered_b_ack_writer.write_all(&genuine_frame).expect(
        "the genuine B installed frame travels through the subsequently registered B descriptor",
    );
    let completion = match reader.read_next_completion(&codec) {
        Ok(completion) => completion,
        Err(error) => panic!(
            "the valid B registration after a rejected wrong-slot attempt must still read the genuine installed ACK; typed kind: {:?}",
            error.kind()
        ),
    };
    cohort
        .publish_registered_owner_lifecycle_completion(completion)
        .expect("only the completion from the still-valid registered B descriptor publishes the staged G2");

    let publication = cohort.observer_safe_lifecycle_publication_summary();
    assert_ne!(
        publication.published_authority_generation_ref(),
        parent_g1.as_str(),
        "the valid B completion proves the wrong-slot rejection did not consume the retained registration"
    );
    assert_eq!(
        publication.publication_outcome(),
        Some(Sys5I3LifecyclePublicationOutcome::G2Published),
        "the one genuine B completion performs the sole parent publication transition"
    );
}

#[test]
#[cfg(all(unix, feature = "i3-process-test-seams"))]
fn i3_3_registered_owner_ack_reader_enforces_one_absolute_deadline_across_short_fragments() {
    // This is an I/O-component test only.  It deliberately uses no cohort,
    // child install, or publication path: the reader must reject before a
    // complete frame could decode, so these bytes establish neither an ACK
    // origin nor an owner lifecycle conclusion.
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let total_budget = Duration::from_millis(250);
    let fragment_interval = Duration::from_millis(100);
    let component_frame = vec![0, 0, 0, 1, b'x'];
    let (parent_stream, mut paired_component_writer) =
        UnixStream::pair().expect("the deadline component test uses a real Unix-stream pair");
    let mut reader = Sys5I3RegisteredOwnerLifecycleAckReader::test_only_for_absolute_deadline(
        parent_stream,
        total_budget,
    )
    .expect(
        "the deadline-only reader fixture owns no registration, completion, cohort, or publisher",
    );

    let writer = thread::spawn(move || {
        for (index, byte) in component_frame.into_iter().enumerate() {
            if paired_component_writer.write_all(&[byte]).is_err() {
                return;
            }
            if index + 1 < 5 {
                thread::sleep(fragment_interval);
            }
        }
    });

    let started = Instant::now();
    let read_error = match reader.read_next_completion(&codec) {
        Ok(_) => panic!(
            "a fragmented component stream that exceeds its total read budget must not yield an ACK completion"
        ),
        Err(error) => error,
    };
    let elapsed = started.elapsed();
    writer
        .join()
        .expect("the bounded component writer thread completes after its finite fragments");

    assert_eq!(
        read_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::LifecycleAckRejected,
        "absolute ACK-read expiry is a typed boundary rejection, never a publication or installed-receipt outcome"
    );
    assert!(
        elapsed < total_budget + Duration::from_millis(100),
        "continued short reads must not reset the one absolute {total_budget:?} ACK-read budget; observed {elapsed:?}"
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
fn i3_3_duplicate_source_generated_owner_request_rejects_without_a_second_owner_serve_or_write() {
    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);

    let request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("ordinary checked source emits its generated owner request once");
    let request_identity = request.semantic_request_identity_ref().to_string();
    let request_bytes = codec
        .encode_outbound_message(request)
        .expect("the source-generated request encodes through the private codec");

    let first_reply = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&request_bytes)
                .expect("the first private request delivery decodes as untrusted input"),
        )
        .expect("the first source-generated request is admitted")
        .expect("the first owner admission produces one typed reply");
    assert_eq!(
        first_reply.semantic_request_identity_ref(),
        request_identity
    );
    assert_eq!(
        first_reply.linked_request_identity_ref(),
        Some(request_identity.as_str()),
        "the first reply keeps the exact source-derived request lineage"
    );

    let owner_summary_after_first = owner.observer_safe_runtime_summary();
    let owner_occurrences_after_first = owner.observer_safe_semantic_occurrences();
    assert_eq!(owner_summary_after_first.served_owner_request_count(), 1);
    assert_eq!(owner_summary_after_first.actual_owner_write_count(), 1);
    assert!(
        owner_occurrences_after_first
            .owner_serve_linearization_occurrence_ref(&request_identity)
            .is_some(),
        "the admitted source request has one owner-serve occurrence"
    );
    assert!(
        owner_occurrences_after_first
            .actual_owner_write_occurrence_ref(&request_identity)
            .is_some(),
        "the admitted source request has one owner-write occurrence"
    );

    let duplicate_error = owner
        .admit_untrusted_message(
            codec
                .decode_untrusted_message(&request_bytes)
                .expect("a duplicate delivery of the same bytes remains untrusted input"),
        )
        .expect_err(
            "the same source-generated request identity must reject rather than receive a second owner serve",
        );
    assert_eq!(
        duplicate_error.kind(),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected,
        "the owner must report a typed duplicate-request rejection rather than collapse it into generic carrier admission"
    );
    assert_eq!(
        owner.observer_safe_runtime_summary(),
        owner_summary_after_first,
        "a duplicate request must preserve owner serve/write counters"
    );
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        owner_occurrences_after_first,
        "a duplicate request must not mint or replace owner semantic occurrences"
    );
    assert_eq!(
        owner
            .authoritative_i64_state("avatar", "self", "hp")
            .expect("the one admitted source request leaves exactly its owner-local state"),
        21
    );
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_pre_handoff_owner_failure_retains_an_ambiguous_tombstone_for_exact_replay_rejection() {
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
                .expect(
                    "ordinary checked source emits the owner request before the downstream fault",
                ),
        )
        .expect("the exact source-generated request encodes through the private codec");
    let owner_summary_before = owner.observer_safe_runtime_summary();
    let owner_occurrences_before = owner.observer_safe_semantic_occurrences();
    let owner_outbox_before = owner.observer_safe_outbox_summary();

    owner.test_only_reject_next_owner_admission_after_reservation();
    assert_eq!(
        owner
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&request_bytes)
                    .expect("the exact request remains untrusted until owner admission"),
            )
            .expect_err(
                "the selected downstream fault occurs after reservation but before owner handoff",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected
    );
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        1,
        "the uncertain post-reservation outcome retains its exact owner-local tombstone"
    );
    assert_eq!(owner.observer_safe_runtime_summary(), owner_summary_before);
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        owner_occurrences_before,
        "the pre-handoff fault creates no owner serve/write occurrence"
    );
    assert_eq!(owner.observer_safe_outbox_summary(), owner_outbox_before);

    assert_eq!(
        owner
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&request_bytes)
                    .expect("the replayed exact bytes remain untrusted input"),
            )
            .expect_err(
                "a retained ambiguous reservation rejects replay rather than permitting a second handoff",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected
    );
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        1,
        "the replay must retain rather than overwrite the ambiguous tombstone"
    );
    assert_eq!(owner.observer_safe_runtime_summary(), owner_summary_before);
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        owner_occurrences_before,
        "the rejected replay must not mint a semantic occurrence after the uncertain first handoff"
    );
    assert_eq!(owner.observer_safe_outbox_summary(), owner_outbox_before);
}

#[test]
#[cfg(feature = "i3-process-test-seams")]
fn i3_3_owner_duplicate_ledger_rejects_capacity_without_evicting_prior_source_requests() {
    const INBOUND_OWNER_REQUEST_LEDGER_CAPACITY: usize = 64;

    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut owner = decode_and_start_private_image(&codec, &mut cohort, OWNER_SLOT);
    let mut request_identities = BTreeSet::new();
    let mut first_request_bytes = None;
    let mut first_request_identity = None;

    for expected_count in 1..=INBOUND_OWNER_REQUEST_LEDGER_CAPACITY {
        let request = requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect("the checked source may emit each independently generated owner request");
        let request_identity = request.semantic_request_identity_ref().to_string();
        assert!(
            request_identities.insert(request_identity.clone()),
            "each source-derived emission must retain a new semantic request identity rather than reuse a prior request"
        );
        let request_bytes = codec
            .encode_outbound_message(request)
            .expect("each source-derived request encodes through the private codec");
        if expected_count == 1 {
            first_request_identity = Some(request_identity.clone());
            first_request_bytes = Some(request_bytes.clone());
        }
        let reply = owner
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&request_bytes)
                    .expect("each private request delivery decodes as untrusted input"),
            )
            .expect("each distinct request within the bounded ledger capacity is admitted")
            .expect("each admitted request produces only its typed owner reply");
        let receipt = requester
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(
                        &codec
                            .encode_outbound_message(reply)
                            .expect("each owner reply encodes through the private codec"),
                    )
                    .expect("each owner reply decodes only as untrusted input"),
            )
            .expect("each exact owner reply admits against its locally pending request")
            .expect("each exact owner reply produces one requester-local receipt");
        assert!(receipt.is_observer_safe_typed_result_or_receipt());
        assert!(receipt.has_no_transportable_carrier());
        assert_eq!(
            receipt.semantic_request_identity_ref(),
            request_identity.as_str(),
            "the requester receipt completes the exact source-generated request"
        );
        assert_eq!(
            requester.observer_safe_pending_owner_request_count(),
            0,
            "a validated local receipt frees only the completed requester pending record"
        );
        assert_eq!(
            requester
                .observer_safe_runtime_summary()
                .accepted_inbound_receipt_count(),
            expected_count,
            "each completed owner reply has one actual requester-local receipt"
        );
        assert_eq!(
            owner
                .observer_safe_runtime_summary()
                .served_owner_request_count(),
            expected_count,
            "every admitted identity has one actual owner serve"
        );
        assert_eq!(
            owner
                .observer_safe_runtime_summary()
                .actual_owner_write_count(),
            expected_count,
            "every admitted identity has one actual owner write"
        );
        assert_eq!(
            owner.observer_safe_inbound_owner_request_tombstone_count(),
            expected_count,
            "owner completion does not release a prior duplicate reservation"
        );
    }

    let first_request_bytes =
        first_request_bytes.expect("the capacity loop records its first request");
    let first_request_identity =
        first_request_identity.expect("the capacity loop records its first request identity");
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        INBOUND_OWNER_REQUEST_LEDGER_CAPACITY,
        "the owner retains every bounded reservation without exposing request payloads or prior results"
    );
    let owner_summary_at_capacity = owner.observer_safe_runtime_summary();
    let owner_occurrences_at_capacity = owner.observer_safe_semantic_occurrences();

    let overflow_request = requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the requester may form one next source-derived request beyond owner capacity");
    let overflow_identity = overflow_request.semantic_request_identity_ref().to_string();
    assert!(
        request_identities.insert(overflow_identity),
        "the capacity outcome must apply to a new semantic request, not a duplicate"
    );
    let overflow_bytes = codec
        .encode_outbound_message(overflow_request)
        .expect("the new overflow request still encodes as an untrusted-delivery candidate");
    assert_eq!(
        owner
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&overflow_bytes)
                    .expect("the overflow request decodes as untrusted input"),
            )
            .expect_err("the bounded owner ledger must reject a new identity rather than evict")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::InboundRequestLedgerExhausted
    );
    assert_eq!(
        owner.observer_safe_runtime_summary(),
        owner_summary_at_capacity
    );
    assert_eq!(
        owner.observer_safe_semantic_occurrences(),
        owner_occurrences_at_capacity,
        "capacity rejection must not mint a serve/write occurrence"
    );
    assert_eq!(
        owner.observer_safe_inbound_owner_request_tombstone_count(),
        INBOUND_OWNER_REQUEST_LEDGER_CAPACITY,
        "capacity rejection must retain, not replace, all prior reservations"
    );

    assert_eq!(
        owner
            .admit_untrusted_message(
                codec
                    .decode_untrusted_message(&first_request_bytes)
                    .expect("the first request replay remains untrusted input"),
            )
            .expect_err("the first reservation must survive later capacity rejection")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected,
        "no-eviction preserves the first request's duplicate disposition"
    );
    assert_eq!(
        owner.observer_safe_runtime_summary(),
        owner_summary_at_capacity
    );
    assert_eq!(
        owner
            .observer_safe_semantic_occurrences()
            .actual_owner_write_occurrence_ref(&first_request_identity),
        owner_occurrences_at_capacity.actual_owner_write_occurrence_ref(&first_request_identity),
        "the replay must retain the first write occurrence rather than mint a new one"
    );
}

#[test]
fn i3_3_requester_pending_ledger_rejects_the_65th_unresolved_source_request_without_eviction() {
    const OUTBOUND_OWNER_REQUEST_PENDING_CAPACITY: usize = 64;

    let project = build_once(CANONICAL_SOURCE);
    let deployment = two_nonempty_slots(&project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut cohort = single_coordinator_cohort(&project, &deployment);
    let mut requester = decode_and_start_private_image(&codec, &mut cohort, REQUESTER_SLOT);
    let mut request_identities = BTreeSet::new();

    for expected_pending_count in 1..=OUTBOUND_OWNER_REQUEST_PENDING_CAPACITY {
        let request = requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect("each bounded unresolved requester operation is source-generated");
        assert!(
            request_identities.insert(request.semantic_request_identity_ref().to_string()),
            "each unresolved source-generated request retains a distinct semantic identity"
        );
        assert_eq!(
            requester.observer_safe_pending_owner_request_count(),
            expected_pending_count,
            "the requester retains each unresolved original operation without eviction"
        );
    }

    let occurrences_at_capacity = requester.observer_safe_semantic_occurrences();
    let outbox_at_capacity = requester.observer_safe_outbox_summary();
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        OUTBOUND_OWNER_REQUEST_PENDING_CAPACITY
    );

    assert_eq!(
        requester
            .emit_generated_owner_request("init_avatar_hp")
            .expect_err("the 65th unresolved source request must fail before a new source action")
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::OutboundRequestLedgerExhausted
    );
    assert_eq!(
        requester.observer_safe_pending_owner_request_count(),
        OUTBOUND_OWNER_REQUEST_PENDING_CAPACITY,
        "requester capacity rejection retains every unresolved pending operation"
    );
    assert_eq!(
        requester.observer_safe_semantic_occurrences(),
        occurrences_at_capacity,
        "requester capacity rejection occurs before a new accepted semantic occurrence"
    );
    assert_eq!(
        requester.observer_safe_outbox_summary(),
        outbox_at_capacity,
        "requester capacity rejection occurs before a new outbound source carrier is created"
    );
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

#[test]
fn i3_owner_admission_budget_sys5_default_owner_entry_rejects_before_owner_effects_and_keeps_the_unannotated_control()
 {
    let budgeted_source = owner_admission_budget_owner_only_source(1);
    let budgeted_project = build_once(&budgeted_source);
    let budgeted_deployment = owner_only_two_slot_deployment(&budgeted_project);
    let (mut budgeted_requester, mut budgeted_owner) =
        start_runtime_pair_directly(&budgeted_project, &budgeted_deployment);
    let budgeted_owner_summary_before = budgeted_owner.observer_safe_runtime_summary();
    let budgeted_owner_outbox_before = budgeted_owner.observer_safe_outbox_summary();

    let budgeted_request = budgeted_requester
        .emit_generated_owner_request("init_avatar_hp")
        .expect("the annotated checked source still generates only its ordinary owner request");
    assert_eq!(
        budgeted_requester.observer_safe_pending_owner_request_count(),
        1,
        "the source-selected requester retains its original pending operation before the owner entry rejects"
    );
    assert_eq!(
        budgeted_owner
            .accept_inbound(budgeted_request)
            .expect_err(
                "the default SYS5 owner entry must not bypass a checked owner-admission condition",
            )
            .kind(),
        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
        "SYS5 may quarantine the lower M8 diagnostic at its carrier boundary, but must fail closed"
    );
    assert_eq!(
        budgeted_owner.observer_safe_runtime_summary(),
        budgeted_owner_summary_before,
        "the default owner entry must not record a serve, write, reply, or receipt on admission-authorization rejection"
    );
    assert_eq!(
        budgeted_owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        0,
        "the rejected annotated request must have no owner serve occurrence"
    );
    assert_eq!(
        budgeted_owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        0,
        "the rejected annotated request must have no owner write occurrence"
    );
    assert_eq!(
        budgeted_owner.observer_safe_outbox_summary(),
        budgeted_owner_outbox_before,
        "the rejected annotated request must not mint an owner reply carrier"
    );
    assert_eq!(
        budgeted_requester.observer_safe_pending_owner_request_count(),
        1,
        "owner rejection must retain the original requester operation rather than manufacture a receipt"
    );
    assert_eq!(
        budgeted_requester
            .observer_safe_runtime_summary()
            .accepted_inbound_receipt_count(),
        0,
        "the requester must not claim a local receipt for the rejected owner admission"
    );

    let ordinary_project = build_once(OWNER_ONLY_SOURCE);
    let ordinary_deployment = owner_only_two_slot_deployment(&ordinary_project);
    let (mut ordinary_requester, mut ordinary_owner) =
        start_runtime_pair_directly(&ordinary_project, &ordinary_deployment);
    let ordinary_reply = ordinary_owner
        .accept_inbound(
            ordinary_requester
                .emit_generated_owner_request("init_avatar_hp")
                .expect(
                    "the unannotated checked source keeps its ordinary generated owner request",
                ),
        )
        .expect("the unannotated SYS5 owner entry remains admitted")
        .expect("ordinary owner execution returns one typed reply");
    let ordinary_receipt = ordinary_requester
        .accept_inbound(ordinary_reply)
        .expect("the ordinary requester admits the typed reply")
        .expect("ordinary reply admission produces one local receipt");
    assert!(ordinary_receipt.has_no_transportable_carrier());
    assert_eq!(
        ordinary_owner
            .observer_safe_runtime_summary()
            .served_owner_request_count(),
        1,
        "unannotated source retains its ordinary one-owner-serve behavior"
    );
    assert_eq!(
        ordinary_owner
            .observer_safe_runtime_summary()
            .actual_owner_write_count(),
        1,
        "unannotated source retains its ordinary one-owner-write behavior"
    );
}

#[test]
fn i3_owner_admission_budget_changed_private_m8_component_cannot_replace_the_sealed_sys5_image_binding()
 {
    let budget_one_project = build_once(&owner_admission_budget_owner_only_source(1));
    let budget_one_deployment = owner_only_two_slot_deployment(&budget_one_project);
    let codec = Sys5I3PrivateProcessCodec::private_provisional_v1();
    let mut budget_one_cohort =
        single_coordinator_cohort(&budget_one_project, &budget_one_deployment);
    let budget_one_binding = budget_one_cohort
        .parent_held_expected_start_binding(OWNER_SLOT)
        .expect("the parent retains the exact budget-one owner image binding");
    let budget_one_image = take_process_image(&mut budget_one_cohort, OWNER_SLOT);
    let budget_one_bytes = codec
        .encode_image(budget_one_image)
        .expect("the genuine budget-one owner image encodes through the private codec");
    let budget_one_candidate = codec
        .decode_untrusted_image(&budget_one_bytes)
        .expect("the untouched budget-one image remains an untrusted candidate");
    codec
        .validate_and_start_image(budget_one_candidate, budget_one_binding)
        .expect("only the exact parent-held budget-one binding starts its genuine image");

    let budget_two_project = build_once(&owner_admission_budget_owner_only_source(2));
    let budget_two_deployment = owner_only_two_slot_deployment(&budget_two_project);
    let mut budget_two_cohort =
        single_coordinator_cohort(&budget_two_project, &budget_two_deployment);
    let budget_two_bytes = codec
        .encode_image(take_process_image(&mut budget_two_cohort, OWNER_SLOT))
        .expect("the independently checked budget-two owner image encodes for the explicit component-copy falsifier");
    let replacement = private_process_json(&budget_two_bytes, PRIVATE_PROCESS_IMAGE_ROOT)
        .pointer(PRIVATE_PROCESS_IMAGE_OWNER_ADMISSION_BUDGET_PATH)
        .cloned()
        .expect("the independently checked budget-two image retains its M8 budget component");
    let mut changed_budget_one =
        private_process_json(&budget_one_bytes, PRIVATE_PROCESS_IMAGE_ROOT);
    *changed_budget_one
        .pointer_mut(PRIVATE_PROCESS_IMAGE_OWNER_ADMISSION_BUDGET_PATH)
        .expect("the budget-one image retains the M8 budget component selected for this narrow falsifier") = replacement;
    let changed_budget_one_bytes = private_process_json_frame(&changed_budget_one);

    assert_eq!(
        codec
            .decode_untrusted_image(&changed_budget_one_bytes)
            .expect_err(
                "a valid-looking budget-two component cannot replace the budget-one M8 snapshot under its sealed image binding",
            )
            .kind(),
        Sys5I3PrivateProcessCodecErrorKind::Malformed,
        "private image decode must reject the changed M8 component before an untrusted child candidate can reach start"
    );
}
