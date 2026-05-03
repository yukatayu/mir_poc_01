use std::collections::BTreeMap;

use mirrorea_core::{
    AuthEvidence, HotPlugRequest, HotPlugVerdict, LayerSignature, MessageEnvelope, PrincipalClaim,
    auth_evidence_lanes, hotplug_request_lanes, hotplug_verdict_lanes, insert_layer_signature,
    layer_signature_lanes, message_envelope_lanes,
};

fn sample_layer_signature(obligations: &[&str]) -> LayerSignature {
    LayerSignature {
        name: "transport_provider_boundary".to_string(),
        requires: vec!["provider_request".to_string()],
        provides: vec!["provider_receipt".to_string()],
        transforms: vec!["request_to_receipt".to_string()],
        checks: vec!["freshness_check".to_string()],
        emits: vec!["message_envelopes".to_string()],
        obligations: obligations
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        laws: vec!["no_hidden_transport".to_string()],
    }
}

fn sample_principal_claim() -> PrincipalClaim {
    PrincipalClaim {
        principal: "Alice".to_string(),
        participant_place: "ParticipantPlace[Alice]".to_string(),
        claimed_authority: "RollAuthority".to_string(),
        claimed_capabilities: vec!["RequestDelegatedRngRoll".to_string()],
    }
}

fn sample_auth_evidence() -> AuthEvidence {
    AuthEvidence {
        kind: "none".to_string(),
        subject: "Alice".to_string(),
        issuer: "AuthorityRng".to_string(),
        bindings: vec!["draw_pub#1".to_string()],
        notes: vec!["baseline none evidence row".to_string()],
    }
}

fn sample_message_envelope() -> MessageEnvelope {
    MessageEnvelope {
        envelope_id: "provider_request#1".to_string(),
        from_place: "ParticipantPlace[Alice]".to_string(),
        to_place: "ProviderPlace[AuthorityRng]".to_string(),
        transport: "provider_boundary".to_string(),
        transport_medium: None,
        transport_seam: "provider_boundary".to_string(),
        payload_kind: "effect_request".to_string(),
        payload_ref: "delegated_rng_roll".to_string(),
        principal_claim: sample_principal_claim(),
        auth_evidence: Some(sample_auth_evidence()),
        emitter_principal: "Alice".to_string(),
        membership_epoch: 0,
        member_incarnation: 0,
        freshness_checks: vec![
            "membership_epoch matches request frontier".to_string(),
            "member_incarnation matches live participant".to_string(),
        ],
        capability_requirements: vec!["RequestDelegatedRngRoll".to_string()],
        authorization_checks: vec!["auth none baseline".to_string()],
        witness_refs: vec!["provider_receipt".to_string()],
        dispatch_outcome: "accepted".to_string(),
        notes: vec!["helper-local preview only".to_string()],
    }
}

fn sample_hotplug_request() -> HotPlugRequest {
    HotPlugRequest {
        request_id: "hotplug_request#canonical".to_string(),
        attachpoint_ref: "AttachPoint[ExampleRoom#1]".to_string(),
        patch_ref: "Patch[ExamplePackage@runtime]".to_string(),
        operation_kind: "attach".to_string(),
        requesting_principal: "ExampleAdmin".to_string(),
        requesting_participant_place: "ParticipantPlace[ExampleAdmin]".to_string(),
        message_envelope_ref: "hotplug_request_envelope#1".to_string(),
        auth_evidence_ref: None,
        capability_refs: vec!["AttachComponent(ExamplePackage)".to_string()],
        witness_refs: Vec::new(),
        notes: vec!["engine-neutral request carrier".to_string()],
    }
}

fn sample_hotplug_verdict() -> HotPlugVerdict {
    HotPlugVerdict {
        request_ref: "hotplug_request#canonical".to_string(),
        verdict_kind: "accepted".to_string(),
        compatibility_reason_refs: vec!["attachpoint_compatible".to_string()],
        authorization_reason_refs: vec!["request_authority_verified".to_string()],
        membership_freshness_reason_refs: vec!["membership_frontier_verified".to_string()],
        witness_reason_refs: vec!["required_witnesses_present".to_string()],
        notes: vec!["engine-neutral verdict carrier".to_string()],
    }
}

#[test]
fn layer_signature_lanes_match_current_shape() {
    assert_eq!(
        layer_signature_lanes(),
        vec![
            "name".to_string(),
            "requires".to_string(),
            "provides".to_string(),
            "transforms".to_string(),
            "checks".to_string(),
            "emits".to_string(),
            "obligations".to_string(),
            "laws".to_string(),
        ]
    );
}

#[test]
fn insert_layer_signature_accepts_identical_duplicate() {
    let mut signatures = BTreeMap::new();
    insert_layer_signature(
        &mut signatures,
        sample_layer_signature(&["provider_result_requires_named_witness"]),
    )
    .expect("first insert");
    insert_layer_signature(
        &mut signatures,
        sample_layer_signature(&["provider_result_requires_named_witness"]),
    )
    .expect("identical duplicate");
    assert_eq!(signatures.len(), 1);
}

#[test]
fn insert_layer_signature_rejects_conflicting_duplicate() {
    let mut signatures = BTreeMap::new();
    insert_layer_signature(
        &mut signatures,
        sample_layer_signature(&["provider_result_requires_named_witness"]),
    )
    .expect("first insert");
    let err = insert_layer_signature(
        &mut signatures,
        sample_layer_signature(&["different_obligation"]),
    )
    .expect_err("conflicting duplicate should fail");
    assert!(err.to_string().contains("transport_provider_boundary"));
}

#[test]
fn auth_and_message_lanes_match_current_shape() {
    assert_eq!(
        auth_evidence_lanes(),
        vec![
            "kind".to_string(),
            "subject".to_string(),
            "issuer".to_string(),
            "bindings".to_string(),
            "notes".to_string(),
        ]
    );
    assert_eq!(
        message_envelope_lanes(),
        vec![
            "envelope_id".to_string(),
            "from_place".to_string(),
            "to_place".to_string(),
            "transport".to_string(),
            "transport_medium".to_string(),
            "transport_seam".to_string(),
            "payload_kind".to_string(),
            "payload_ref".to_string(),
            "principal_claim".to_string(),
            "auth_evidence".to_string(),
            "emitter_principal".to_string(),
            "membership_epoch".to_string(),
            "member_incarnation".to_string(),
            "freshness_checks".to_string(),
            "capability_requirements".to_string(),
            "authorization_checks".to_string(),
            "witness_refs".to_string(),
            "dispatch_outcome".to_string(),
            "notes".to_string(),
        ]
    );
}

#[test]
fn hotplug_carrier_lanes_match_current_shape() {
    assert_eq!(
        hotplug_request_lanes(),
        vec![
            "request_id".to_string(),
            "attachpoint_ref".to_string(),
            "patch_ref".to_string(),
            "operation_kind".to_string(),
            "requesting_principal".to_string(),
            "requesting_participant_place".to_string(),
            "message_envelope_ref".to_string(),
            "auth_evidence_ref".to_string(),
            "capability_refs".to_string(),
            "witness_refs".to_string(),
            "notes".to_string(),
        ]
    );
    assert_eq!(
        hotplug_verdict_lanes(),
        vec![
            "request_ref".to_string(),
            "verdict_kind".to_string(),
            "compatibility_reason_refs".to_string(),
            "authorization_reason_refs".to_string(),
            "membership_freshness_reason_refs".to_string(),
            "witness_reason_refs".to_string(),
            "notes".to_string(),
        ]
    );
}

#[test]
fn message_envelope_validate_accepts_current_row_shape() {
    sample_message_envelope()
        .validate()
        .expect("current row shape should validate");
}

#[test]
fn message_envelope_validate_rejects_blank_transport_seam() {
    let mut envelope = sample_message_envelope();
    envelope.transport_seam = " ".to_string();
    let err = envelope
        .validate()
        .expect_err("blank transport seam should fail");
    assert!(err.to_string().contains("transport_seam"));
}

#[test]
fn message_envelope_validate_rejects_transport_alias_drift() {
    let mut envelope = sample_message_envelope();
    envelope.transport = "legacy_alias_drift".to_string();
    let err = envelope
        .validate()
        .expect_err("transport alias drift should fail");
    assert!(err.to_string().contains("transport"));
    assert!(err.to_string().contains("transport_seam"));
}

#[test]
fn hotplug_request_validate_accepts_current_row_shape() {
    sample_hotplug_request()
        .validate()
        .expect("current hot-plug request shape should validate");
}

#[test]
fn hotplug_request_validate_rejects_blank_message_envelope_ref() {
    let mut request = sample_hotplug_request();
    request.message_envelope_ref = " ".to_string();
    let err = request
        .validate()
        .expect_err("blank message envelope ref should fail");
    assert!(err.to_string().contains("message_envelope_ref"));
}

#[test]
fn hotplug_verdict_validate_accepts_current_row_shape() {
    sample_hotplug_verdict()
        .validate()
        .expect("current hot-plug verdict shape should validate");
}

#[test]
fn hotplug_verdict_validate_rejects_unknown_verdict_kind() {
    let mut verdict = sample_hotplug_verdict();
    verdict.verdict_kind = "needs_runtime_engine".to_string();
    let err = verdict
        .validate()
        .expect_err("unknown verdict kind should fail");
    assert!(err.to_string().contains("verdict_kind"));
}
