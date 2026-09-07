//! Private I3-3 actual-process delivery-fault observations.
//!
//! These types describe only bounded harness scheduling and observer-safe
//! evidence emitted by actual children.  They never carry an expected result,
//! carrier bytes, credentials, semantic authority, or a retry decision.

use serde::{Deserialize, Serialize};

use super::i3_process_localnet::I3LocalnetObserverSafeDeliveryRecord;

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
