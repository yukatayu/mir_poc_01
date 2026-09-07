//! Private I3-3 actual-process delivery-fault observations.
//!
//! These types describe only bounded harness scheduling and observer-safe
//! evidence emitted by actual children.  They never carry an expected result,
//! carrier bytes, credentials, semantic authority, or a retry decision.

use serde::{Deserialize, Serialize};

use super::i3_process_localnet::{
    I3LocalnetChildSlot, I3LocalnetChildTerminalOutcome, I3LocalnetObserverSafeDeliveryRecord,
};

/// The first finite delivery controls on the existing two-process route.
/// They select a real adapter scheduling point; they do not select a route,
/// operation, authority, or semantic result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetFaultProfile {
    DisconnectBeforeRequestCarrierWrite,
    DisconnectAfterRemoteAdmission,
    DisconnectAfterRemoteAdmissionSuppressOwnerAudit,
}

/// Private observer-record falsifiers for the bounded post-admission profile.
///
/// These mutate only the child event emitted to the supervisor after actual
/// admission; they do not alter the source-derived request, carrier traffic,
/// owner dispatch, or mutation.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetFaultAuditFalsifier {
    ReplaceOwnerFaultWithRejectedTerminal,
    MutateOwnerRequestReceiveCoreRef,
    SetOwnerRequestReceiveLinkedRequestIdentity,
    ClearOwnerRequestReceiveCarrierRef,
    ClearOwnerRequestReceiveNetworkOccurrenceRef,
}

/// Why an owner event was not accepted as remote admission evidence.
///
/// This is deliberately distinct from absent remote evidence.  In
/// particular, a malformed observer record cannot be silently reclassified
/// as a no-admission result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRemoteEvidenceRejection {
    ProvenanceMismatch,
}

/// The two finite actual-session controls for the original source-emitted
/// owner request.  They select no caller retry policy, new source operation,
/// request identity, authority, or wire result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRetryProfile {
    ReconnectBeforeInitialCarrierWrite,
    ReconnectAfterOwnerAdmissionBeforeReply,
}

/// Negative-only session misuse control.  It asks the existing adapter to use
/// the reconnect-send API on the fully verified first session; it carries no
/// peer, carrier, identity, or expected error input.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRetryFalsifier {
    RetryOnInitialVerifiedSession,
}

/// Post-send observer-only corruptions of reconnect delivery or attempt
/// observations. They run only after the adapter made the actual session-two
/// frame attempt; no carrier bytes, runtime admission, or owner ledger state
/// is changed.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRetryAuditFalsifier {
    MutateReconnectSenderCandidateCommitment,
    ClearReconnectSenderCandidateCommitment,
    UseInitialSessionCandidateCommitmentForReconnect,
    MutateReconnectRequesterBindingRef,
}

/// Why an actual reconnect sender/receiver evidence join was not accepted.
/// This is distinct from the original request's runtime outcome: the
/// requester remains pending and no owner duplicate conclusion is published.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRetryEvidenceRejection {
    ReconnectAttemptCommitmentMissing,
    ReconnectAttemptCommitmentMismatch,
}

/// Requester-local disposition of the one retained original operation.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRetryRequesterOutcome {
    ReceiptConsumed,
    PendingReplyOrReceiptNotObserved,
}

/// Runtime-owned reason retained for one begun original-request attempt.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRetryAttemptReason {
    InitialDelivery,
    ReconnectRetry,
}

/// Observer-safe snapshot copied from the runtime's retained original-request
/// attempt summary before a later reply can consume its pending record.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRetryAttemptAudit {
    semantic_request_identity_ref: String,
    source_requester_locus: String,
    requester_binding_ref: String,
    attempt_generation: u8,
    reason: I3LocalnetRetryAttemptReason,
}

impl I3LocalnetRetryAttemptAudit {
    pub(crate) fn from_runtime_summary(
        semantic_request_identity_ref: String,
        source_requester_locus: String,
        requester_binding_ref: String,
        attempt_generation: u8,
        reason: I3LocalnetRetryAttemptReason,
    ) -> Self {
        Self {
            semantic_request_identity_ref,
            source_requester_locus,
            requester_binding_ref,
            attempt_generation,
            reason,
        }
    }

    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }

    pub fn source_requester_locus(&self) -> &str {
        &self.source_requester_locus
    }

    pub fn requester_binding_ref(&self) -> &str {
        &self.requester_binding_ref
    }

    pub const fn attempt_generation(&self) -> u8 {
        self.attempt_generation
    }

    pub const fn reason(&self) -> I3LocalnetRetryAttemptReason {
        self.reason
    }
}

/// Exact two-session evidence reported by one actual exec child.  `run_ref`
/// is an observer-safe run binding, not a transport credential or source
/// value; session values are the adapter's fixed first/second generations.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRetryChildAudit {
    slot: I3LocalnetChildSlot,
    run_ref: String,
    first_session_generation: u8,
    reconnect_session_generation: u8,
    first_session_peer_spki_verified: bool,
    first_session_reciprocal_preface_verified: bool,
    reconnect_session_peer_spki_verified: bool,
    reconnect_session_reciprocal_preface_verified: bool,
    first_session_runtime_attempt: Option<I3LocalnetRetryAttemptAudit>,
    // Only the requester runtime owns the retained original request.  The
    // owner child therefore reports `None` here rather than copying a
    // requester-owned runtime summary into its own actual child evidence.
    reconnect_session_runtime_attempt: Option<I3LocalnetRetryAttemptAudit>,
    terminal_outcome: I3LocalnetChildTerminalOutcome,
}

/// Complete observer-safe session evidence emitted by one retry child before
/// the supervisor attaches its child slot and terminal outcome.
#[derive(Clone, Debug)]
pub(crate) struct I3LocalnetRetryChildAuditEvidence {
    pub(crate) run_ref: String,
    pub(crate) first_session_generation: u8,
    pub(crate) reconnect_session_generation: u8,
    pub(crate) first_session_peer_spki_verified: bool,
    pub(crate) first_session_reciprocal_preface_verified: bool,
    pub(crate) reconnect_session_peer_spki_verified: bool,
    pub(crate) reconnect_session_reciprocal_preface_verified: bool,
    pub(crate) first_session_runtime_attempt: Option<I3LocalnetRetryAttemptAudit>,
    pub(crate) reconnect_session_runtime_attempt: Option<I3LocalnetRetryAttemptAudit>,
}

impl I3LocalnetRetryChildAudit {
    pub(crate) fn from_actual_child_event(
        slot: I3LocalnetChildSlot,
        terminal_outcome: I3LocalnetChildTerminalOutcome,
        evidence: I3LocalnetRetryChildAuditEvidence,
    ) -> Self {
        let I3LocalnetRetryChildAuditEvidence {
            run_ref,
            first_session_generation,
            reconnect_session_generation,
            first_session_peer_spki_verified,
            first_session_reciprocal_preface_verified,
            reconnect_session_peer_spki_verified,
            reconnect_session_reciprocal_preface_verified,
            first_session_runtime_attempt,
            reconnect_session_runtime_attempt,
        } = evidence;
        Self {
            slot,
            run_ref,
            first_session_generation,
            reconnect_session_generation,
            first_session_peer_spki_verified,
            first_session_reciprocal_preface_verified,
            reconnect_session_peer_spki_verified,
            reconnect_session_reciprocal_preface_verified,
            first_session_runtime_attempt,
            reconnect_session_runtime_attempt,
            terminal_outcome,
        }
    }

    pub const fn slot(&self) -> I3LocalnetChildSlot {
        self.slot
    }

    pub fn run_ref(&self) -> &str {
        &self.run_ref
    }

    pub const fn first_session_generation(&self) -> u8 {
        self.first_session_generation
    }

    pub const fn reconnect_session_generation(&self) -> u8 {
        self.reconnect_session_generation
    }

    pub const fn first_session_peer_spki_verified(&self) -> bool {
        self.first_session_peer_spki_verified
    }

    pub const fn first_session_reciprocal_preface_verified(&self) -> bool {
        self.first_session_reciprocal_preface_verified
    }

    pub const fn reconnect_session_peer_spki_verified(&self) -> bool {
        self.reconnect_session_peer_spki_verified
    }

    pub const fn reconnect_session_reciprocal_preface_verified(&self) -> bool {
        self.reconnect_session_reciprocal_preface_verified
    }

    pub fn first_session_runtime_attempt(&self) -> Option<&I3LocalnetRetryAttemptAudit> {
        self.first_session_runtime_attempt.as_ref()
    }

    /// The requester child owns the runtime's retained original-request
    /// attempt summary.  Callers must not use this accessor for the owner
    /// child, which has no such pending request.
    pub fn reconnect_session_runtime_attempt(&self) -> &I3LocalnetRetryAttemptAudit {
        self.reconnect_session_runtime_attempt
            .as_ref()
            .expect("only the requester child owns a reconnect runtime attempt")
    }

    pub const fn terminal_outcome(&self) -> I3LocalnetChildTerminalOutcome {
        self.terminal_outcome
    }
}

/// Actual owner result of the session-two attempt.  A duplicate rejection is
/// owner-local evidence only; it is not a reply delivered to the requester.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum I3LocalnetReconnectOwnerOutcome {
    Replied {
        owner_serve_count: usize,
        owner_mutation_count: usize,
        owner_serve_occurrence_ref: String,
        owner_write_occurrence_ref: Option<String>,
    },
    DuplicateRequestRejected {
        candidate_commitment_ref: String,
        network_occurrence_ref: String,
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
}

/// Joined observer-safe evidence for one selected actual two-session control.
/// A successful pre-write control may consume a real local receipt; the
/// post-admission control intentionally leaves the original requester pending.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRetryAudit {
    profile: I3LocalnetRetryProfile,
    request_identity_ref: String,
    original_request_succeeded: bool,
    requester_outcome: I3LocalnetRetryRequesterOutcome,
    first_session_carrier_write_attempted: bool,
    initial_request_delivery: Option<I3LocalnetObserverSafeDeliveryRecord>,
    reconnect_request_delivery: I3LocalnetObserverSafeDeliveryRecord,
    initial_owner_admission: Option<I3LocalnetRemoteAdmissionEvidence>,
    reconnect_owner_outcome: Option<I3LocalnetReconnectOwnerOutcome>,
    evidence_rejection: Option<I3LocalnetRetryEvidenceRejection>,
    requester_child: I3LocalnetRetryChildAudit,
    owner_child: I3LocalnetRetryChildAudit,
}

impl I3LocalnetRetryAudit {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_actual_child_events(
        profile: I3LocalnetRetryProfile,
        request_identity_ref: String,
        requester_outcome: I3LocalnetRetryRequesterOutcome,
        first_session_carrier_write_attempted: bool,
        initial_request_delivery: Option<I3LocalnetObserverSafeDeliveryRecord>,
        reconnect_request_delivery: I3LocalnetObserverSafeDeliveryRecord,
        initial_owner_admission: Option<I3LocalnetRemoteAdmissionEvidence>,
        reconnect_owner_outcome: Option<I3LocalnetReconnectOwnerOutcome>,
        evidence_rejection: Option<I3LocalnetRetryEvidenceRejection>,
        requester_child: I3LocalnetRetryChildAudit,
        owner_child: I3LocalnetRetryChildAudit,
    ) -> Self {
        Self {
            profile,
            request_identity_ref,
            original_request_succeeded: matches!(
                requester_outcome,
                I3LocalnetRetryRequesterOutcome::ReceiptConsumed
            ),
            requester_outcome,
            first_session_carrier_write_attempted,
            initial_request_delivery,
            reconnect_request_delivery,
            initial_owner_admission,
            reconnect_owner_outcome,
            evidence_rejection,
            requester_child,
            owner_child,
        }
    }

    pub const fn profile(&self) -> I3LocalnetRetryProfile {
        self.profile
    }

    pub fn request_identity_ref(&self) -> &str {
        &self.request_identity_ref
    }

    pub const fn original_request_succeeded(&self) -> bool {
        self.original_request_succeeded
    }

    pub const fn requester_outcome(&self) -> I3LocalnetRetryRequesterOutcome {
        self.requester_outcome
    }

    pub const fn first_session_carrier_write_attempted(&self) -> bool {
        self.first_session_carrier_write_attempted
    }

    pub fn initial_request_delivery(&self) -> Option<&I3LocalnetObserverSafeDeliveryRecord> {
        self.initial_request_delivery.as_ref()
    }

    pub fn reconnect_request_delivery(&self) -> &I3LocalnetObserverSafeDeliveryRecord {
        &self.reconnect_request_delivery
    }

    pub fn initial_owner_admission(&self) -> Option<&I3LocalnetRemoteAdmissionEvidence> {
        self.initial_owner_admission.as_ref()
    }

    pub fn reconnect_owner_outcome(&self) -> Option<&I3LocalnetReconnectOwnerOutcome> {
        self.reconnect_owner_outcome.as_ref()
    }

    pub const fn evidence_rejection(&self) -> Option<I3LocalnetRetryEvidenceRejection> {
        self.evidence_rejection
    }

    pub fn requester_child(&self) -> &I3LocalnetRetryChildAudit {
        &self.requester_child
    }

    pub fn owner_child(&self) -> &I3LocalnetRetryChildAudit {
        &self.owner_child
    }
}

/// A requester observation emitted from an actual child event.  It is not a
/// conclusion about remote admission or mutation.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRequesterFaultObservation {
    RequestCarrierWriteNotAttempted,
    ReplyOrReceiptNotObserved,
}

/// Actual owner-side evidence optionally joined by the supervisor.
///
/// `None` at the enclosing audit means that the requester has no remote
/// evidence; it must not be rendered as a zero count.  `NoAdmission` has no
/// request identity because the owner received no request carrier.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum I3LocalnetRemoteAdmissionEvidence {
    NoAdmission {
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
    Admitted {
        request_receive: Box<I3LocalnetObserverSafeDeliveryRecord>,
        owner_serve_count: usize,
        owner_mutation_count: usize,
        owner_serve_occurrence_ref: String,
        owner_write_occurrence_ref: Option<String>,
    },
}

/// Joined observer-safe evidence for one handled delivery fault.
///
/// The supervisor copies this only from the terminal events reported by its
/// two exec children, validating the child slots and matching request
/// identities where a remote carrier was actually admitted.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetFaultAudit {
    profile: I3LocalnetFaultProfile,
    semantic_success: bool,
    request_identity_ref: String,
    requester_observation: I3LocalnetRequesterFaultObservation,
    remote_admission: Option<I3LocalnetRemoteAdmissionEvidence>,
    remote_evidence_rejection: Option<I3LocalnetRemoteEvidenceRejection>,
}

impl I3LocalnetFaultAudit {
    pub(crate) fn from_actual_child_events(
        profile: I3LocalnetFaultProfile,
        request_identity_ref: String,
        requester_observation: I3LocalnetRequesterFaultObservation,
        remote_admission: Option<I3LocalnetRemoteAdmissionEvidence>,
        remote_evidence_rejection: Option<I3LocalnetRemoteEvidenceRejection>,
    ) -> Self {
        Self {
            profile,
            // A delivery fault is never the positive I3-2 semantic round
            // trip, including when both child processes exit naturally.
            semantic_success: false,
            request_identity_ref,
            requester_observation,
            remote_admission,
            remote_evidence_rejection,
        }
    }

    pub const fn profile(&self) -> I3LocalnetFaultProfile {
        self.profile
    }

    pub const fn semantic_success(&self) -> bool {
        self.semantic_success
    }

    pub fn request_identity_ref(&self) -> &str {
        &self.request_identity_ref
    }

    pub const fn requester_observation(&self) -> I3LocalnetRequesterFaultObservation {
        self.requester_observation
    }

    pub fn remote_admission(&self) -> Option<&I3LocalnetRemoteAdmissionEvidence> {
        self.remote_admission.as_ref()
    }

    pub const fn remote_evidence_rejection(&self) -> Option<I3LocalnetRemoteEvidenceRejection> {
        self.remote_evidence_rejection
    }
}
