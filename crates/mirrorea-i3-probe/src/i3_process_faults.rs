//! Private I3-3 actual-process delivery-fault observations.
//!
//! These types describe only bounded harness scheduling and observer-safe
//! evidence emitted by actual children.  They never carry an expected result,
//! carrier bytes, credentials, semantic authority, or a retry decision.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::i3_process_localnet::{
    I3LocalnetChildSlot, I3LocalnetChildTerminalOutcome, I3LocalnetObserverSafeDeliveryRecord,
    I3LocalnetOwnerAdmissionExpiryEvidence,
};

/// The three bounded actual adapter-delivery schedules. They select only one
/// concrete local transport action after ordinary source has generated the
/// request; they carry no carrier bytes, authority, retry, or expected result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetAdapterDeliveryProfile {
    EndpointClosedBeforeConnect,
    CompleteGeneratedRequestInTwoWrites,
    TruncateGeneratedRequestAfterBodyPrefix,
}

/// Actual adapter failure observed by a bounded delivery schedule. This is
/// distinct from peer binding or semantic admission; it never asserts owner
/// mutation or a remote request record.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetAdapterDeliveryFailure {
    EndpointUnavailable,
    FrameRejected,
}

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

/// Opts into one fixed requester-local monotonic wait after A has observed
/// the existing post-admission reply loss. It supplies no duration, retry,
/// session, carrier, or semantic outcome to the caller.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRequesterLocalWaitProfile {
    WaitLocallyAfterObservedLoss,
}

/// Observer-only corruptions applied after A has completed the real local
/// wait. They cannot alter the source request, QUIC traffic, owner result, or
/// requester runtime state.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetRequesterLocalWaitFalsifier {
    SetObservedElapsedBelowMinimum,
    ClearPostWaitPendingObservation,
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

/// The two finite controls for one complete owner-request frame received on
/// session one and admitted, if at all, only after both children reconnect on
/// session two.  They do not select a resend, source operation, authority
/// value, or expected semantic result.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetLateIngressProfile {
    HoldSessionOneIngressAcrossVerifiedReconnect,
    PrestageOwnerCapabilityRevocationBeforeLateIngressAdmission,
}

/// Bounded probe-only negative controls for the genuine pre-staged owner
/// lifecycle route.  They never expose a candidate, installed receipt, ACK
/// record, publisher, or source operation to the caller.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetLateIngressFalsifier {
    GenuineCandidateBindingTamper,
    ClearRetainedIngressCandidateCommitment,
    MutateRetainedIngressCandidateCommitment,
    /// After the real session-one frame is retained, acquire it a second time
    /// from the same B session. The adapter must reject locally before I/O;
    /// the original opaque ingress remains the only one later admitted.
    RepeatRetainedIngressAcquisition,
    /// B suppresses its receipt-gated registered-FD emission. A instead emits
    /// one bounded, candidate-shaped ACK record on its actual stdout route;
    /// parent may decode it only as tainted and must not publish it.
    RouteTaintedAckCandidateFromActualAStdout,
    DropBRegisteredAck,
    ReplayBRegisteredAck,
    /// Mutates only B's already-produced terminal observer record after the
    /// actual late-admission attempt. It never changes a frame, runtime
    /// admission, or owner mutation.
    SetOwnerLateIngressTerminalUnauthenticatedAdmissionAndClearTrustedControl,
    /// Mutates only A's already-produced terminal observer record after the
    /// actual pending outcome. It never changes A's source request or runtime
    /// pending state.
    ClearRequesterLateIngressTerminalTrustedControl,
}

/// Why the adapter-only session-one ingress record could not be joined to the
/// independently retained source sender record.  This rejects observer
/// evidence only; it does not recast the original requester operation or
/// publish an owner outcome.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetLateIngressEvidenceRejection {
    RetainedIngressCommitmentMissing,
    RetainedIngressCommitmentMismatch,
}

/// Parent observation of one bounded ACK-shaped input delivered on the
/// requester's actual stdout route. It is never a completion route: decoding
/// remains tainted and the parent does not offer it to the registered-B reader
/// or cohort publisher.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetLateIngressNonregisteredAckInputDisposition {
    NotObserved,
    RequesterStdoutTaintedCandidateIgnored,
}

/// Owner-local outcome when the opaque session-one ingress is finally offered
/// to the runtime on session two.  A rejection is not a reply delivered to
/// the requester.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum I3LocalnetLateIngressOwnerOutcome {
    Admitted {
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
    CarrierAdmissionRejected {
        owner_serve_count: usize,
        owner_mutation_count: usize,
    },
}

impl I3LocalnetLateIngressOwnerOutcome {
    pub const fn owner_serve_count(&self) -> usize {
        match self {
            Self::Admitted {
                owner_serve_count, ..
            }
            | Self::CarrierAdmissionRejected {
                owner_serve_count, ..
            } => *owner_serve_count,
        }
    }

    pub const fn owner_mutation_count(&self) -> usize {
        match self {
            Self::Admitted {
                owner_mutation_count,
                ..
            }
            | Self::CarrierAdmissionRejected {
                owner_mutation_count,
                ..
            } => *owner_mutation_count,
        }
    }
}

/// Requester-local disposition of the one original request after the
/// session-two late-admission attempt.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetLateIngressRequesterOutcome {
    ReceiptConsumed,
    PendingReplyOrReceiptNotObserved,
}

/// Provenance of the optional genuine M9 lifecycle installation.  It is
/// intentionally separate from the parent publication state.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I3LocalnetLateIngressLifecycleProvenance {
    NotSelected,
    M9AdmittedLifecycle,
    CandidateBindingRejected,
}

/// Classification made by the parent-owned reader for the dedicated,
/// registered owner-child lifecycle ACK route.  No variant contains an FD,
/// owner label, decoded ACK, or detachable origin token.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetLateIngressAckReaderOutcome {
    NotSelected,
    AcceptedRegisteredOwnerChildFd,
    UntrustedRouteIgnored,
    LostBeforeParentAcceptance,
    ReplayRejected,
}

/// Parent-local disposition of the one pre-staged G1-to-G2 publication.
/// This makes no durability, rollback, or all-child-generation claim.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I3LocalnetLateIngressParentPublication {
    NoPrestageSelected,
    G2Published,
    PublicationIncomplete,
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
    /// The first session produced and retained a declared owner expiry, then
    /// the exact original request reached the existing session-two duplicate
    /// guard. This remains owner-local evidence, never a reply to A.
    DuplicateRequestRejectedAfterDeclaredDeadlineExpired {
        candidate_commitment_ref: String,
        network_occurrence_ref: String,
        owner_expired_count: usize,
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
    initial_owner_expiry: Option<I3LocalnetOwnerAdmissionExpiryEvidence>,
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
        initial_owner_expiry: Option<I3LocalnetOwnerAdmissionExpiryEvidence>,
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
            initial_owner_expiry,
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

    /// Present only when B's first session actually produced a declared
    /// expiry and retained it before the reply was intentionally lost. It is
    /// distinct from a successful owner admission.
    pub fn initial_owner_expiry(&self) -> Option<&I3LocalnetOwnerAdmissionExpiryEvidence> {
        self.initial_owner_expiry.as_ref()
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
    requester_local_wait: Option<I3LocalnetRequesterLocalWaitAudit>,
}

/// A's monotonic local observation after a real reply loss. It deliberately
/// has no remote completion, timeout, driver, retry, or semantic-result fact.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct I3LocalnetRequesterLocalWaitAudit {
    observed_elapsed: Duration,
    required_minimum_elapsed: Duration,
    pending_before: usize,
    pending_after: usize,
    local_receipt_before: usize,
    local_receipt_after: usize,
    terminal_failure_before: usize,
    terminal_failure_after: usize,
}

impl I3LocalnetRequesterLocalWaitAudit {
    #[expect(
        clippy::too_many_arguments,
        reason = "the eight named arguments preserve independently observed elapsed and before/after runtime counts without an anonymous tuple or test-only wrapper"
    )]
    pub(crate) const fn from_actual_observation(
        observed_elapsed: Duration,
        required_minimum_elapsed: Duration,
        pending_before: usize,
        pending_after: usize,
        local_receipt_before: usize,
        local_receipt_after: usize,
        terminal_failure_before: usize,
        terminal_failure_after: usize,
    ) -> Self {
        Self {
            observed_elapsed,
            required_minimum_elapsed,
            pending_before,
            pending_after,
            local_receipt_before,
            local_receipt_after,
            terminal_failure_before,
            terminal_failure_after,
        }
    }

    pub const fn observed_elapsed(&self) -> Duration {
        self.observed_elapsed
    }

    pub const fn required_minimum_elapsed(&self) -> Duration {
        self.required_minimum_elapsed
    }

    pub const fn pending_before(&self) -> usize {
        self.pending_before
    }

    pub const fn pending_after(&self) -> usize {
        self.pending_after
    }

    pub const fn local_receipt_before(&self) -> usize {
        self.local_receipt_before
    }

    pub const fn local_receipt_after(&self) -> usize {
        self.local_receipt_after
    }

    pub const fn terminal_failure_before(&self) -> usize {
        self.terminal_failure_before
    }

    pub const fn terminal_failure_after(&self) -> usize {
        self.terminal_failure_after
    }
}

impl I3LocalnetFaultAudit {
    pub(crate) fn from_actual_child_events(
        profile: I3LocalnetFaultProfile,
        request_identity_ref: String,
        requester_observation: I3LocalnetRequesterFaultObservation,
        remote_admission: Option<I3LocalnetRemoteAdmissionEvidence>,
        remote_evidence_rejection: Option<I3LocalnetRemoteEvidenceRejection>,
        requester_local_wait: Option<I3LocalnetRequesterLocalWaitAudit>,
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
            requester_local_wait,
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

    /// Present only after the parent validated A's actual post-loss local
    /// wait together with the selected remote-admission fault evidence.
    pub fn requester_local_wait(&self) -> Option<&I3LocalnetRequesterLocalWaitAudit> {
        self.requester_local_wait.as_ref()
    }
}
