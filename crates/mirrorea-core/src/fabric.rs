use serde::{Deserialize, Serialize};

use crate::error::{MirroreaCoreError, require_non_empty, require_non_empty_items};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrincipalClaim {
    pub principal: String,
    pub participant_place: String,
    pub claimed_authority: String,
    pub claimed_capabilities: Vec<String>,
}

impl PrincipalClaim {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("PrincipalClaim", "principal", &self.principal)?;
        require_non_empty(
            "PrincipalClaim",
            "participant_place",
            &self.participant_place,
        )?;
        require_non_empty(
            "PrincipalClaim",
            "claimed_authority",
            &self.claimed_authority,
        )?;
        require_non_empty_items(
            "PrincipalClaim",
            "claimed_capabilities",
            &self.claimed_capabilities,
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthEvidence {
    pub kind: String,
    pub subject: String,
    pub issuer: String,
    pub bindings: Vec<String>,
    pub notes: Vec<String>,
}

impl AuthEvidence {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("AuthEvidence", "kind", &self.kind)?;
        require_non_empty("AuthEvidence", "subject", &self.subject)?;
        require_non_empty("AuthEvidence", "issuer", &self.issuer)?;
        require_non_empty_items("AuthEvidence", "bindings", &self.bindings)?;
        require_non_empty_items("AuthEvidence", "notes", &self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageEnvelope {
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub transport: String,
    pub transport_medium: Option<String>,
    pub transport_seam: String,
    pub payload_kind: String,
    pub payload_ref: String,
    pub principal_claim: PrincipalClaim,
    pub auth_evidence: Option<AuthEvidence>,
    pub emitter_principal: String,
    pub membership_epoch: u64,
    pub member_incarnation: u64,
    pub freshness_checks: Vec<String>,
    pub capability_requirements: Vec<String>,
    pub authorization_checks: Vec<String>,
    pub witness_refs: Vec<String>,
    pub dispatch_outcome: String,
    pub notes: Vec<String>,
}

impl MessageEnvelope {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("MessageEnvelope", "envelope_id", &self.envelope_id)?;
        require_non_empty("MessageEnvelope", "from_place", &self.from_place)?;
        require_non_empty("MessageEnvelope", "to_place", &self.to_place)?;
        require_non_empty("MessageEnvelope", "transport", &self.transport)?;
        if let Some(transport_medium) = &self.transport_medium {
            require_non_empty("MessageEnvelope", "transport_medium", transport_medium)?;
        }
        require_non_empty("MessageEnvelope", "transport_seam", &self.transport_seam)?;
        if self.transport != self.transport_seam {
            return Err(MirroreaCoreError::new(
                "MessageEnvelope field `transport` must match `transport_seam` because the legacy alias is compatibility-only",
            ));
        }
        require_non_empty("MessageEnvelope", "payload_kind", &self.payload_kind)?;
        require_non_empty("MessageEnvelope", "payload_ref", &self.payload_ref)?;
        self.principal_claim.validate()?;
        if let Some(auth_evidence) = &self.auth_evidence {
            auth_evidence.validate()?;
        }
        require_non_empty(
            "MessageEnvelope",
            "emitter_principal",
            &self.emitter_principal,
        )?;
        require_non_empty_items(
            "MessageEnvelope",
            "freshness_checks",
            &self.freshness_checks,
        )?;
        require_non_empty_items(
            "MessageEnvelope",
            "capability_requirements",
            &self.capability_requirements,
        )?;
        require_non_empty_items(
            "MessageEnvelope",
            "authorization_checks",
            &self.authorization_checks,
        )?;
        require_non_empty_items("MessageEnvelope", "witness_refs", &self.witness_refs)?;
        require_non_empty(
            "MessageEnvelope",
            "dispatch_outcome",
            &self.dispatch_outcome,
        )?;
        require_non_empty_items("MessageEnvelope", "notes", &self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HotPlugRequest {
    pub request_id: String,
    pub attachpoint_ref: String,
    pub patch_ref: String,
    pub operation_kind: String,
    pub requesting_principal: String,
    pub requesting_participant_place: String,
    pub message_envelope_ref: String,
    pub auth_evidence_ref: Option<String>,
    pub capability_refs: Vec<String>,
    pub witness_refs: Vec<String>,
    pub notes: Vec<String>,
}

impl HotPlugRequest {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("HotPlugRequest", "request_id", &self.request_id)?;
        require_non_empty("HotPlugRequest", "attachpoint_ref", &self.attachpoint_ref)?;
        require_non_empty("HotPlugRequest", "patch_ref", &self.patch_ref)?;
        require_non_empty("HotPlugRequest", "operation_kind", &self.operation_kind)?;
        require_non_empty(
            "HotPlugRequest",
            "requesting_principal",
            &self.requesting_principal,
        )?;
        require_non_empty(
            "HotPlugRequest",
            "requesting_participant_place",
            &self.requesting_participant_place,
        )?;
        require_non_empty(
            "HotPlugRequest",
            "message_envelope_ref",
            &self.message_envelope_ref,
        )?;
        if let Some(auth_evidence_ref) = &self.auth_evidence_ref {
            require_non_empty("HotPlugRequest", "auth_evidence_ref", auth_evidence_ref)?;
        }
        require_non_empty_items("HotPlugRequest", "capability_refs", &self.capability_refs)?;
        require_non_empty_items("HotPlugRequest", "witness_refs", &self.witness_refs)?;
        require_non_empty_items("HotPlugRequest", "notes", &self.notes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HotPlugVerdict {
    pub request_ref: String,
    pub verdict_kind: String,
    pub compatibility_reason_refs: Vec<String>,
    pub authorization_reason_refs: Vec<String>,
    pub membership_freshness_reason_refs: Vec<String>,
    pub witness_reason_refs: Vec<String>,
    pub notes: Vec<String>,
}

impl HotPlugVerdict {
    pub fn validate(&self) -> Result<(), MirroreaCoreError> {
        require_non_empty("HotPlugVerdict", "request_ref", &self.request_ref)?;
        require_non_empty("HotPlugVerdict", "verdict_kind", &self.verdict_kind)?;
        if !matches!(
            self.verdict_kind.as_str(),
            "accepted" | "rejected" | "deferred"
        ) {
            return Err(MirroreaCoreError::new(
                "HotPlugVerdict field `verdict_kind` must be one of `accepted`, `rejected`, or `deferred`",
            ));
        }
        require_non_empty_items(
            "HotPlugVerdict",
            "compatibility_reason_refs",
            &self.compatibility_reason_refs,
        )?;
        require_non_empty_items(
            "HotPlugVerdict",
            "authorization_reason_refs",
            &self.authorization_reason_refs,
        )?;
        require_non_empty_items(
            "HotPlugVerdict",
            "membership_freshness_reason_refs",
            &self.membership_freshness_reason_refs,
        )?;
        require_non_empty_items(
            "HotPlugVerdict",
            "witness_reason_refs",
            &self.witness_reason_refs,
        )?;
        require_non_empty_items("HotPlugVerdict", "notes", &self.notes)?;
        Ok(())
    }
}

pub fn message_envelope_lanes() -> Vec<String> {
    [
        "envelope_id",
        "from_place",
        "to_place",
        "transport",
        "transport_medium",
        "transport_seam",
        "payload_kind",
        "payload_ref",
        "principal_claim",
        "auth_evidence",
        "emitter_principal",
        "membership_epoch",
        "member_incarnation",
        "freshness_checks",
        "capability_requirements",
        "authorization_checks",
        "witness_refs",
        "dispatch_outcome",
        "notes",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

pub fn auth_evidence_lanes() -> Vec<String> {
    ["kind", "subject", "issuer", "bindings", "notes"]
        .into_iter()
        .map(|lane| lane.to_string())
        .collect()
}

pub fn hotplug_request_lanes() -> Vec<String> {
    [
        "request_id",
        "attachpoint_ref",
        "patch_ref",
        "operation_kind",
        "requesting_principal",
        "requesting_participant_place",
        "message_envelope_ref",
        "auth_evidence_ref",
        "capability_refs",
        "witness_refs",
        "notes",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

pub fn hotplug_verdict_lanes() -> Vec<String> {
    [
        "request_ref",
        "verdict_kind",
        "compatibility_reason_refs",
        "authorization_reason_refs",
        "membership_freshness_reason_refs",
        "witness_reason_refs",
        "notes",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}
