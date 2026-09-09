//! Private I3-2 QUIC delivery ingress.
//!
//! This module is the only normal-build promotion path from bytes delivered
//! over the finite private QUIC profile into `Sys5I3ProcessRuntime`.  A
//! claimed preface is deliberately insufficient: this adapter owns a live
//! `quinn::Connection`, opens or accepts its one bidi stream itself, checks
//! the peer's leaf SPKI against the separately retained control, then checks
//! the reciprocal preface before crate-private decoded admission is reached.
//! Transport evidence never grants Mir authority; the runtime still performs
//! its sealed cohort/M9/owner/pending-request validation.

use std::any::Any;

#[cfg(feature = "i3-process-test-seams")]
use quinn::ReadExactError;
use quinn::{Connection, RecvStream, SendStream};
use sha2::{Digest, Sha256};

#[cfg(feature = "i3-process-test-seams")]
use super::sys5_i3_process_runtime::{
    Sys5I3FixedProviderNetworkFirstResultReceiveAction,
    Sys5I3FixedProviderNetworkFirstResultSendAction,
    Sys5I3FixedProviderNetworkSecondResultReceiveAction,
    Sys5I3FixedProviderNetworkSecondResultSendAction,
};
use super::sys5_i3_process_runtime::{
    Sys5I3InstalledProviderChildRuntime, Sys5I3LocalnetControlErrorKind, Sys5I3LocalnetPeerPreface,
    Sys5I3OriginalOwnerRequestAttemptKind, Sys5I3OriginalOwnerRequestPending,
    Sys5I3PrivateProcessCodec, Sys5I3PrivateProviderTransportOccurrence,
    Sys5I3PrivateProviderTransportOccurrenceKind, Sys5I3ProcessLocalCutControlRole,
    Sys5I3ProcessMessage, Sys5I3ProcessRuntime, Sys5I3ProcessRuntimeError,
    Sys5I3ProcessRuntimeErrorKind, Sys5I3ProviderConsumeReceipt, Sys5I3ProviderLocalnetPeerPreface,
    Sys5I3ProviderRequestCarrier, Sys5I3ProviderResultCarrier, Sys5I3TrustedLocalnetControl,
    Sys5I3TrustedProviderLocalnetControl, Sys5I3UntrustedProcessMessage, strict_json_value,
};

const MAX_PRIVATE_QUIC_BLOB_BYTES: usize = 64 * 1024;
const MAX_PRIVATE_QUIC_SESSION_ATTEMPTS: u8 = 2;

/// The existing private QUIC session has two control profiles. The provider
/// profile shares the inspected Quinn connection and bounded stream framing,
/// but cannot enter ordinary generated-owner carrier operations or reconnect.
enum PrivateQuicControl {
    Ordinary(Sys5I3TrustedLocalnetControl),
    Provider(Sys5I3TrustedProviderLocalnetControl),
}

impl PrivateQuicControl {
    fn expected_peer_spki_ref(&self) -> &str {
        match self {
            Self::Ordinary(control) => control.expected_peer_spki_ref(),
            Self::Provider(control) => control.expected_peer_spki_ref(),
        }
    }

    fn run_ref(&self) -> &str {
        match self {
            Self::Ordinary(control) => control.run_ref(),
            Self::Provider(control) => control.run_ref(),
        }
    }
}

/// Fail-closed private-adapter outcomes.  They contain no peer address,
/// certificate, key, raw preface, carrier payload, or semantic state.
#[doc(hidden)]
#[derive(Debug)]
pub enum Sys5I3PrivateQuicError {
    /// A TLS-authenticated peer did not match the separately retained exact
    /// SPKI binding. This is delivery-origin evidence, never authority.
    PeerBindingRejected(Sys5I3PrivateQuicPeerBindingEvidence),
    FrameRejected,
    CodecRejected,
    /// A syntactically decoded carrier failed the sealed runtime boundary.
    /// It retains receiver-owned network-attempt evidence only, never the
    /// untrusted candidate's asserted route, source, Core, or identity.
    SemanticRejected {
        error: Sys5I3ProcessRuntimeError,
        rejected_attempt: Sys5I3PrivateQuicRejectedAttemptEvidence,
    },
    /// A provider carrier reached the installed currentness/admission
    /// boundary. Unlike ordinary delivery rejection, this carries no
    /// frame-derived commitment or candidate evidence because provider frames
    /// may contain a raw value.
    ProviderSemanticRejected(Sys5I3ProcessRuntimeError),
    /// The runtime refused a locally retained original-request attempt before
    /// any stream write.  The caller still owns the opaque pending handle.
    OriginalRequestAttemptRejected(Sys5I3ProcessRuntimeError),
    /// A checked per-session occurrence counter cannot advance without
    /// collision, so this adapter refuses the next frame effect.
    NetworkOccurrenceExhausted,
    /// A consuming reconnect token already represents the fixed second
    /// session attempt.  Its retained control is deliberately not returned.
    SessionAttemptExhausted,
    /// A verified private session was asked to perform an operation reserved
    /// for another bounded session generation.  This is local adapter misuse,
    /// not peer-binding evidence.
    LocalAttemptRejected,
}

/// Test-profile control for one source-generated message's private stream
/// framing.  This controls application writes only; it says nothing about
/// QUIC packetization or receiver read boundaries.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub enum Sys5I3PrivateQuicGeneratedFrameWriteControl {
    /// Split the outer bounded-frame prefix across exactly two application
    /// writes, while still sending the complete encoded message.
    CompleteInTwoWrites { first_frame_fragment_len: usize },
    /// Send the outer bounded-frame prefix and a strict proper prefix of the
    /// already encoded message, then finish the send stream.
    TruncateAfterBodyPrefix { body_prefix_len: usize },
}

#[cfg(feature = "i3-process-test-seams")]
impl Sys5I3PrivateQuicGeneratedFrameWriteControl {
    fn validate_encoded_body(&self, body: &[u8]) -> Result<(), Sys5I3PrivateQuicError> {
        let prefix = private_quic_blob_prefix(body)?;
        match self {
            Self::CompleteInTwoWrites {
                first_frame_fragment_len,
            } if *first_frame_fragment_len > 0 && *first_frame_fragment_len < prefix.len() => {
                Ok(())
            }
            Self::TruncateAfterBodyPrefix { body_prefix_len }
                if *body_prefix_len > 0 && *body_prefix_len < body.len() =>
            {
                Ok(())
            }
            Self::CompleteInTwoWrites { .. } | Self::TruncateAfterBodyPrefix { .. } => {
                Err(Sys5I3PrivateQuicError::FrameRejected)
            }
        }
    }
}

/// Reference-only evidence from an exact post-handshake peer-binding check.
/// The leaf reference is populated only after Quinn/Rustls has validated the
/// run CA chain and exposed the peer identity.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sys5I3PrivateQuicPeerBindingEvidence {
    expected_peer_spki_ref: String,
    actual_peer_spki_ref: Option<String>,
    ca_validated_peer_leaf_ref: Option<String>,
}

impl Sys5I3PrivateQuicPeerBindingEvidence {
    pub fn expected_peer_spki_ref(&self) -> &str {
        &self.expected_peer_spki_ref
    }

    pub fn actual_peer_spki_ref(&self) -> Option<&str> {
        self.actual_peer_spki_ref.as_deref()
    }

    pub fn ca_validated_peer_leaf_ref(&self) -> Option<&str> {
        self.ca_validated_peer_leaf_ref.as_deref()
    }
}

impl Sys5I3PrivateQuicError {
    fn peer_binding_rejected(expected_peer_spki_ref: impl Into<String>) -> Self {
        Self::PeerBindingRejected(Sys5I3PrivateQuicPeerBindingEvidence {
            expected_peer_spki_ref: expected_peer_spki_ref.into(),
            actual_peer_spki_ref: None,
            ca_validated_peer_leaf_ref: None,
        })
    }

    /// Returns only the reference evidence attached to the exact peer-binding
    /// rejection variant.
    pub fn peer_binding_evidence(&self) -> Option<&Sys5I3PrivateQuicPeerBindingEvidence> {
        match self {
            Self::PeerBindingRejected(evidence) => Some(evidence),
            Self::FrameRejected
            | Self::CodecRejected
            | Self::SemanticRejected { .. }
            | Self::ProviderSemanticRejected(_)
            | Self::OriginalRequestAttemptRejected(_)
            | Self::NetworkOccurrenceExhausted
            | Self::SessionAttemptExhausted
            | Self::LocalAttemptRejected => None,
        }
    }

    /// Bounded receiver-owned evidence for a decoded rejection.  It is
    /// intentionally absent for framing/codec failures, which never reached
    /// a typed semantic admission boundary.
    pub fn rejected_attempt_evidence(&self) -> Option<&Sys5I3PrivateQuicRejectedAttemptEvidence> {
        match self {
            Self::SemanticRejected {
                rejected_attempt, ..
            } => Some(rejected_attempt),
            _ => None,
        }
    }
}

/// Evidence that an actual received frame reached a typed semantic rejection.
/// The commitment is run-scoped and domain-separated from source or carrier
/// provenance, so a rejected candidate cannot be reported as a validated
/// source/Core/route fact or joined to an original request.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sys5I3PrivateQuicRejectedAttemptEvidence {
    rejected_candidate_commitment_ref: String,
    network_occurrence_ref: String,
}

impl Sys5I3PrivateQuicRejectedAttemptEvidence {
    /// Run-scoped, domain-separated commitment to the actual bounded frame
    /// that reached typed semantic rejection.  It is deliberately not source,
    /// Core, route, carrier, or semantic-request provenance.
    pub fn candidate_commitment_ref(&self) -> &str {
        &self.rejected_candidate_commitment_ref
    }

    /// Compatibility spelling for the rejection-specific commitment getter.
    /// Both names return the exact same adapter-computed reference.
    pub fn rejected_candidate_commitment_ref(&self) -> &str {
        self.candidate_commitment_ref()
    }

    pub fn network_occurrence_ref(&self) -> &str {
        &self.network_occurrence_ref
    }
}

/// Reference-only evidence for one complete session-one frame retained by the
/// adapter before any semantic admission.  It deliberately cannot report a
/// decoded carrier's source, Core, artifact, edge, identity, or authority
/// assertions: those remain unavailable until a later successful runtime
/// admission.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sys5I3PrivateQuicPendingIngressEvidence {
    session_attempt_generation: u8,
    candidate_commitment_ref: String,
    network_occurrence_ref: String,
}

impl Sys5I3PrivateQuicPendingIngressEvidence {
    pub const fn session_attempt_generation(&self) -> u8 {
        self.session_attempt_generation
    }

    pub fn candidate_commitment_ref(&self) -> &str {
        &self.candidate_commitment_ref
    }

    pub fn network_occurrence_ref(&self) -> &str {
        &self.network_occurrence_ref
    }
}

/// One adapter-owned complete frame read on the verified initial session but
/// not offered to the runtime until the one consuming reconnect session.
///
/// This type deliberately implements neither `Clone` nor `Debug`.  Its
/// decoded candidate, frame bytes, and existing trusted-control binding
/// remain private to this adapter.  The only observer-safe view is the
/// reference record returned by `observer_safe_evidence`.
#[doc(hidden)]
pub struct Sys5I3PrivateQuicPendingIngress {
    bytes: Vec<u8>,
    candidate: Sys5I3UntrustedProcessMessage,
    session_attempt_generation: u8,
    control_binding: Sys5I3LocalnetPeerPreface,
    candidate_commitment_ref: String,
    network_occurrence_ref: String,
}

impl Sys5I3PrivateQuicPendingIngress {
    pub fn observer_safe_evidence(&self) -> Sys5I3PrivateQuicPendingIngressEvidence {
        Sys5I3PrivateQuicPendingIngressEvidence {
            session_attempt_generation: self.session_attempt_generation,
            candidate_commitment_ref: self.candidate_commitment_ref.clone(),
            network_occurrence_ref: self.network_occurrence_ref.clone(),
        }
    }
}

/// One session-owned permit for the sole retained session-one ingress.  It is
/// intentionally private: the adapter alone reserves it before a read and
/// consumes it before semantic admission, so a caller cannot manufacture or
/// replay an ingress right.
enum Sys5I3PrivateQuicPendingIngressPermit {
    Unacquired,
    Reserved,
    Completed { network_occurrence_ref: String },
    Consumed,
}

impl Sys5I3PrivateQuicPendingIngressPermit {
    fn reserve_acquisition(&mut self) -> Result<(), Sys5I3PrivateQuicError> {
        if !matches!(self, Self::Unacquired) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        *self = Self::Reserved;
        Ok(())
    }

    fn complete_acquisition(
        &mut self,
        network_occurrence_ref: &str,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        if !matches!(self, Self::Reserved) || network_occurrence_ref.is_empty() {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        *self = Self::Completed {
            network_occurrence_ref: network_occurrence_ref.to_string(),
        };
        Ok(())
    }

    fn consume_completed(
        &mut self,
        network_occurrence_ref: &str,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        if !matches!(
            self,
            Self::Completed {
                network_occurrence_ref: expected,
            } if expected == network_occurrence_ref
        ) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        *self = Self::Consumed;
        Ok(())
    }
}

/// Observer-safe evidence derived from an actual one-stream send or receive.
/// It contains references only; the adapter never exports carrier bytes.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sys5I3PrivateQuicDeliveryEvidence {
    carrier_ref: String,
    candidate_commitment_ref: String,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    network_occurrence_ref: String,
    #[cfg(feature = "i3-process-test-seams")]
    generated_frame_write_observation: Option<Sys5I3PrivateQuicGeneratedFrameWriteObservation>,
}

/// Observer-safe facts from a completed test-profile generated-frame write.
/// It is constructed only after the adapter's actual application writes
/// succeed; it exports neither bytes nor fragment lengths.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sys5I3PrivateQuicGeneratedFrameWriteObservation {
    application_write_count: u8,
    frame_prefix_split_across_writes: bool,
    complete_frame_written: bool,
}

#[cfg(feature = "i3-process-test-seams")]
impl Sys5I3PrivateQuicGeneratedFrameWriteObservation {
    pub const fn application_write_count(&self) -> u8 {
        self.application_write_count
    }

    pub const fn frame_prefix_split_across_writes(&self) -> bool {
        self.frame_prefix_split_across_writes
    }

    pub const fn complete_frame_written(&self) -> bool {
        self.complete_frame_written
    }
}

impl Sys5I3PrivateQuicDeliveryEvidence {
    pub fn carrier_ref(&self) -> &str {
        &self.carrier_ref
    }

    /// Run-scoped, domain-separated commitment to the actual encoded frame.
    /// It permits a local supervisor to compare the same exact sender frame
    /// and receiver rejection only within one run/session, without exporting
    /// bytes or treating rejection as source/Core/route provenance.  A
    /// reconnect has a new session generation, so its commitment differs even
    /// when the retained carrier reference is unchanged.
    pub fn candidate_commitment_ref(&self) -> &str {
        &self.candidate_commitment_ref
    }

    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }

    pub fn linked_request_identity_ref(&self) -> Option<&str> {
        self.linked_request_identity_ref.as_deref()
    }

    /// Source provenance extracted from the exact bounded carrier bytes.  The
    /// adapter returns only the checked logical reference, never source text.
    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }

    /// Checked Core reference extracted from the exact bounded carrier bytes.
    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    /// Source artifact reference extracted from the exact bounded carrier
    /// bytes.
    pub fn source_artifact_ref(&self) -> &str {
        &self.source_artifact_ref
    }

    /// Target artifact reference extracted from the exact bounded carrier
    /// bytes.
    pub fn target_artifact_ref(&self) -> &str {
        &self.target_artifact_ref
    }

    /// Generated communication edge reference extracted from the exact
    /// bounded carrier bytes.
    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub fn network_occurrence_ref(&self) -> &str {
        &self.network_occurrence_ref
    }

    /// Present only for the successful controlled generated-frame test
    /// profile. Normal sends deliberately have no such observation.
    #[cfg(feature = "i3-process-test-seams")]
    pub fn generated_frame_write_observation(
        &self,
    ) -> Option<&Sys5I3PrivateQuicGeneratedFrameWriteObservation> {
        self.generated_frame_write_observation.as_ref()
    }
}

/// One bounded, adapter-retained generated Reply frame that was fully written
/// on the verified initial session.  This type deliberately implements
/// neither `Clone` nor `Debug`: its exact private bytes and control binding
/// never leave this adapter, and consuming it permits only the fixed
/// successor-session replay falsifier.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub struct Sys5I3PrivateQuicGeneratedReplyReplayCandidate {
    body: Vec<u8>,
    session_attempt_generation: u8,
    control_binding: Sys5I3LocalnetPeerPreface,
    original_send_delivery: Sys5I3PrivateQuicDeliveryEvidence,
}

/// One adapter-retained Row20 first Reply. It is created only after the
/// verified owner session has fully written that reply, stays bound to that
/// physical Quinn connection, and is consumed only by the fixed late-reply
/// conformance path. It is neither a generic replay right nor an authority.
#[cfg(feature = "i3-process-test-seams")]
pub(crate) struct Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply {
    body: Vec<u8>,
    first_request_identity_ref: String,
    cohort_provenance_ref: String,
    control_binding: Sys5I3LocalnetPeerPreface,
    connection_stable_id: usize,
    session_attempt_generation: u8,
}

/// Observer-safe evidence from the one retained initial Reply write and its
/// one actual successor-session replay write.  The adapter returns this only
/// after the replay write completes; it exports neither the retained bytes
/// nor the trusted preface/control binding.
#[cfg(feature = "i3-process-test-seams")]
#[doc(hidden)]
pub struct Sys5I3PrivateQuicReplayedGeneratedReplyDelivery {
    replay_delivery: Sys5I3PrivateQuicDeliveryEvidence,
    original_send_delivery: Sys5I3PrivateQuicDeliveryEvidence,
}

#[cfg(feature = "i3-process-test-seams")]
impl Sys5I3PrivateQuicReplayedGeneratedReplyDelivery {
    /// Evidence for the new, actual successor-session write.
    pub fn replay_delivery(&self) -> &Sys5I3PrivateQuicDeliveryEvidence {
        &self.replay_delivery
    }

    /// Evidence retained from the exact initial write that issued the opaque
    /// candidate.  This is a reference-only audit join, not a delivery
    /// acknowledgement or an authority fact.
    pub fn original_send_delivery(&self) -> &Sys5I3PrivateQuicDeliveryEvidence {
        &self.original_send_delivery
    }
}

/// Private result of encoding the existing generated-message path.  It keeps
/// the exact carrier-derived references with the one encoded body until a
/// session reserves its delivery occurrence and writes it.
struct Sys5I3PrivateQuicEncodedGeneratedMessage {
    body: Vec<u8>,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    lineage: PrivateCarrierLineage,
}

/// One inspected connection and its adapter-owned bidi stream.  Neither the
/// connection nor streams are exposed to semantic callers, preventing an
/// external decoded candidate from bypassing the delivery-origin gate.
#[doc(hidden)]
pub struct Sys5I3PrivateQuicSession {
    connection: Connection,
    send: SendStream,
    receive: RecvStream,
    control: PrivateQuicControl,
    peer_spki_verified: bool,
    peer_preface_verified: bool,
    session_attempt_generation: u8,
    next_network_occurrence: u64,
    pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit,
    #[cfg(feature = "i3-process-test-seams")]
    generated_reply_replay_candidate_issued: bool,
}

/// A consuming private reconnect capability.  It transfers the sole retained
/// peer control and checked occurrence counter into at most one second
/// adapter session; it is neither cloneable nor an authority credential.
#[doc(hidden)]
pub struct Sys5I3PrivateQuicReconnect {
    control: PrivateQuicControl,
    prior_session_attempt_generation: u8,
    next_network_occurrence: u64,
    pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit,
}

/// A reply result that either consumes the original pending handle exactly
/// once after local receipt or terminal-failure admission, or returns it
/// intact after a delivery failure/rejection.  A retry rejection therefore
/// cannot complete the original semantic operation.
#[doc(hidden)]
pub enum Sys5I3PrivateQuicOriginalOwnerReplyOutcome {
    Consumed {
        receipt: Box<Sys5I3ProcessMessage>,
        delivery: Sys5I3PrivateQuicDeliveryEvidence,
    },
    TerminalFailureConsumed {
        terminal: Box<Sys5I3ProcessMessage>,
        delivery: Sys5I3PrivateQuicDeliveryEvidence,
    },
    Pending {
        pending: Sys5I3OriginalOwnerRequestPending,
        error: Sys5I3PrivateQuicError,
    },
}

/// An already-admitted requester-local result, classified before the adapter
/// exposes its original-pending outcome.  The classifier owns no decoded
/// candidate or transport input: it only preserves the runtime's existing
/// receipt versus terminal-consumption distinction.
enum Sys5I3PrivateQuicAdmittedOriginalOwnerReply {
    Receipt(Sys5I3ProcessMessage),
    TerminalFailureConsumed(Sys5I3ProcessMessage),
}

fn classify_admitted_original_owner_reply(
    admitted: Option<Sys5I3ProcessMessage>,
) -> Result<Sys5I3PrivateQuicAdmittedOriginalOwnerReply, Sys5I3PrivateQuicError> {
    let admitted = admitted.ok_or(Sys5I3PrivateQuicError::FrameRejected)?;
    if admitted.is_observer_safe_typed_result_or_receipt() {
        Ok(Sys5I3PrivateQuicAdmittedOriginalOwnerReply::Receipt(
            admitted,
        ))
    } else if admitted.is_observer_safe_terminal_failure_consumed()
        && admitted.has_no_transportable_carrier()
    {
        Ok(Sys5I3PrivateQuicAdmittedOriginalOwnerReply::TerminalFailureConsumed(admitted))
    } else {
        Err(Sys5I3PrivateQuicError::FrameRejected)
    }
}

impl Sys5I3PrivateQuicSession {
    /// Owns and inspects a client connection, then opens the profile's only
    /// bidi stream.  The constructor never accepts a stream supplied by an
    /// external caller.
    pub async fn connect(
        connection: Connection,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<Self, Sys5I3PrivateQuicError> {
        if !control.has_ordinary_purpose() {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .open_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Self {
            connection,
            send,
            receive,
            control: PrivateQuicControl::Ordinary(control),
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
            #[cfg(feature = "i3-process-test-seams")]
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Owns and inspects a server connection, then accepts the profile's one
    /// bidi stream.  No external process code receives an ingress stream.
    pub async fn accept(
        connection: Connection,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<Self, Sys5I3PrivateQuicError> {
        if !control.has_ordinary_purpose() {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .accept_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Self {
            connection,
            send,
            receive,
            control: PrivateQuicControl::Ordinary(control),
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
            #[cfg(feature = "i3-process-test-seams")]
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Row20's custody path owns its first stream from bootstrap.  Unlike
    /// the ordinary constructor it accepts only the factory-issued purpose
    /// for the named child role; it still performs the same peer-SPKI check
    /// and opens no caller-supplied stream.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn connect_process_local_cut(
        connection: Connection,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<Self, Sys5I3PrivateQuicError> {
        if !control.has_process_local_cut_purpose(Sys5I3ProcessLocalCutControlRole::Requester) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .open_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Self {
            connection,
            send,
            receive,
            control: PrivateQuicControl::Ordinary(control),
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Server-side counterpart of `connect_process_local_cut`.  The exact
    /// owner purpose is checked before accepting the owned bidi stream.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn accept_process_local_cut(
        connection: Connection,
        control: Sys5I3TrustedLocalnetControl,
    ) -> Result<Self, Sys5I3PrivateQuicError> {
        if !control.has_process_local_cut_purpose(Sys5I3ProcessLocalCutControlRole::Owner) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .accept_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Self {
            connection,
            send,
            receive,
            control: PrivateQuicControl::Ordinary(control),
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Client-side provider session over the existing owned QUIC bidi stream.
    /// The distinct control can be created only by an installed inherited-FD3
    /// provider runtime; it cannot be reconstructed from image or frame data.
    pub async fn connect_provider(
        connection: Connection,
        control: Sys5I3TrustedProviderLocalnetControl,
    ) -> Result<Self, Sys5I3PrivateQuicError> {
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .open_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Self {
            connection,
            send,
            receive,
            control: PrivateQuicControl::Provider(control),
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
            #[cfg(feature = "i3-process-test-seams")]
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Server-side provider counterpart. It reuses the same inspected
    /// connection and owned ingress stream as the ordinary profile.
    pub async fn accept_provider(
        connection: Connection,
        control: Sys5I3TrustedProviderLocalnetControl,
    ) -> Result<Self, Sys5I3PrivateQuicError> {
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .accept_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Self {
            connection,
            send,
            receive,
            control: PrivateQuicControl::Provider(control),
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
            #[cfg(feature = "i3-process-test-seams")]
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Consume this session into the only possible reconnect capability.  The
    /// connection/streams are dropped with the old adapter; the retained
    /// control and occurrence counter move by value and cannot be cloned.
    pub fn into_reconnect(self) -> Sys5I3PrivateQuicReconnect {
        Sys5I3PrivateQuicReconnect {
            control: self.control,
            prior_session_attempt_generation: self.session_attempt_generation,
            next_network_occurrence: self.next_network_occurrence,
            pending_ingress_permit: self.pending_ingress_permit,
        }
    }

    pub const fn peer_spki_verified(&self) -> bool {
        self.peer_spki_verified
    }

    pub const fn peer_preface_verified(&self) -> bool {
        self.peer_preface_verified
    }

    pub const fn reliable_bidi_stream_count(&self) -> usize {
        1
    }

    pub const fn quic_datagrams_enabled(&self) -> bool {
        false
    }

    pub const fn session_attempt_generation(&self) -> u8 {
        self.session_attempt_generation
    }

    fn require_ordinary_control(
        &self,
    ) -> Result<&Sys5I3TrustedLocalnetControl, Sys5I3PrivateQuicError> {
        match &self.control {
            PrivateQuicControl::Ordinary(control) if control.has_ordinary_purpose() => Ok(control),
            PrivateQuicControl::Provider(_) => Err(Sys5I3PrivateQuicError::LocalAttemptRejected),
            PrivateQuicControl::Ordinary(_) => Err(Sys5I3PrivateQuicError::LocalAttemptRejected),
        }
    }

    #[cfg(feature = "i3-process-test-seams")]
    fn require_process_local_cut_control(
        &self,
        role: Sys5I3ProcessLocalCutControlRole,
    ) -> Result<&Sys5I3TrustedLocalnetControl, Sys5I3PrivateQuicError> {
        match &self.control {
            PrivateQuicControl::Ordinary(control)
                if control.has_process_local_cut_purpose(role) =>
            {
                Ok(control)
            }
            PrivateQuicControl::Provider(_) | PrivateQuicControl::Ordinary(_) => {
                Err(Sys5I3PrivateQuicError::LocalAttemptRejected)
            }
        }
    }

    fn require_provider_control(
        &self,
    ) -> Result<&Sys5I3TrustedProviderLocalnetControl, Sys5I3PrivateQuicError> {
        match &self.control {
            PrivateQuicControl::Provider(control) => Ok(control),
            PrivateQuicControl::Ordinary(_) => Err(Sys5I3PrivateQuicError::LocalAttemptRejected),
        }
    }

    fn require_verified_provider_session(&self) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(1)
    }

    /// Internal provider-session validation for a named bounded generation.
    /// It is not exposed to callers; ordinary provider operations remain
    /// fixed to generation one, while the sealed conformance route names the
    /// only permitted second session at its private phase boundary.
    fn require_verified_provider_session_generation(
        &self,
        required_session_attempt_generation: u8,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_provider_control()?;
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        if self.session_attempt_generation != required_session_attempt_generation {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    fn require_verified_session_generation(
        &self,
        required_session_attempt_generation: u8,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        if self.session_attempt_generation != required_session_attempt_generation {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        Ok(())
    }

    #[cfg(feature = "i3-process-test-seams")]
    fn require_verified_process_local_cut_session(
        &self,
        role: Sys5I3ProcessLocalCutControlRole,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_process_local_cut_control(role)?;
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        if self.session_attempt_generation != 1 {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        Ok(())
    }

    pub async fn send_local_preface(&mut self) -> Result<(), Sys5I3PrivateQuicError> {
        let body = match &self.control {
            PrivateQuicControl::Ordinary(control) => {
                serde_json::to_vec(&control.localnet_preface())
            }
            PrivateQuicControl::Provider(control) => serde_json::to_vec(
                &control
                    .localnet_preface()
                    .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?,
            ),
        }
        .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.write_blob(&body).await
    }

    /// Private negative-only delivery test.  It remains inside the adapter so
    /// no caller can mint an ingress token or submit a decoded candidate.
    pub async fn send_unbound_preface_for_private_falsifier(
        &mut self,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        let mut value = serde_json::to_value(self.require_ordinary_control()?.localnet_preface())
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        let object = value
            .as_object_mut()
            .ok_or(Sys5I3PrivateQuicError::FrameRejected)?;
        let Some(serde_json::Value::String(local_spki_ref)) = object.get_mut("local_spki_ref")
        else {
            return Err(Sys5I3PrivateQuicError::FrameRejected);
        };
        local_spki_ref.push_str("-unbound");
        let body = serde_json::to_vec(&value).map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.write_blob(&body).await
    }

    pub async fn receive_and_validate_peer_preface(
        &mut self,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        let body = self.read_blob().await?;
        let value = strict_json_value(&body).map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        match &self.control {
            PrivateQuicControl::Ordinary(control) => {
                let preface: Sys5I3LocalnetPeerPreface = serde_json::from_value(value)
                    .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
                control.validate_peer_preface(&preface)
            }
            PrivateQuicControl::Provider(control) => {
                let preface: Sys5I3ProviderLocalnetPeerPreface = serde_json::from_value(value)
                    .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
                control.validate_peer_preface(&preface)
            }
        }
        .map_err(|error| match error.kind() {
            Sys5I3LocalnetControlErrorKind::PeerBindingRejected => {
                Sys5I3PrivateQuicError::peer_binding_rejected(self.control.expected_peer_spki_ref())
            }
            _ => Sys5I3PrivateQuicError::FrameRejected,
        })?;
        self.peer_preface_verified = true;
        Ok(())
    }

    /// Send one distinct provider request after the existing mTLS and
    /// reciprocal provider-preface checks. The carrier remains opaque to the
    /// probe and never reuses the generated owner-message path.
    pub async fn send_provider_request(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
        request: Sys5I3ProviderRequestCarrier,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_provider_control()?;
        self.require_verified_provider_session()?;
        let bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_provider_request(&request)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let reservation = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendReservation,
        )?;
        let ticket = runtime
            .reserve_provider_request_transport_send(&request, reservation)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
        // Allocate the next opaque adapter-owned occurrence before beginning
        // the frame write, but bind it only after the write completes. This
        // prevents a post-write counter failure from erasing completion while
        // still keeping reservation and completion distinct retained facts.
        let completion = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted,
        )?;
        self.write_blob(&bytes).await?;
        runtime
            .complete_provider_transport_send(ticket, completion)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
    }

    /// Receive exactly one provider request and let the installed executor
    /// perform current admission, reservation, CallStarted, bounded host read
    /// and result retention. Decoded bytes alone never carry authority.
    pub async fn receive_provider_request_and_execute(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
    ) -> Result<Sys5I3ProviderResultCarrier, Sys5I3PrivateQuicError> {
        self.require_provider_control()?;
        self.require_verified_provider_session()?;
        let bytes = self.read_blob().await?;
        let occurrence = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestReceive,
        )?;
        let request = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_provider_request(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        runtime
            .admit_read_only_provider_request_and_execute_from_transport(request, occurrence)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
    }

    /// Send one retained executor outcome in the distinct provider result
    /// direction. A local failed release has no carrier to send.
    pub async fn send_provider_result(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
        result: Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_provider_control()?;
        self.require_verified_provider_session()?;
        let bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_provider_result(&result)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let reservation = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendReservation,
        )?;
        let ticket = runtime
            .reserve_provider_result_transport_send(&result, reservation)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
        let completion = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted,
        )?;
        self.write_blob(&bytes).await?;
        runtime
            .complete_provider_transport_send(ticket, completion)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
    }

    /// Receive and consume one provider result at the requester. The receipt
    /// is local-only and carries no raw result getter.
    pub async fn receive_provider_result_and_consume(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3PrivateQuicError> {
        self.receive_provider_result_and_consume_at_verified_provider_session_generation(runtime, 1)
            .await
    }

    /// Keep the one normal result-consume codec/admission path shared with
    /// the sealed held-result second-session phase. The generation remains a
    /// private adapter parameter: no caller can turn it into a general retry
    /// or decoded-carrier admission operation.
    async fn receive_provider_result_and_consume_at_verified_provider_session_generation(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
        required_session_attempt_generation: u8,
    ) -> Result<Sys5I3ProviderConsumeReceipt, Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(required_session_attempt_generation)?;
        let bytes = self.read_blob().await?;
        let occurrence = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultReceive,
        )?;
        let result = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_provider_result(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        runtime
            .admit_read_only_provider_result_and_consume_from_transport(result, occurrence)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
    }

    /// Execute the executor's sealed first-result phase on session one. The
    /// selected action is retained in inherited control; no transport flag or
    /// caller-supplied outcome can choose it.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn drive_fixed_provider_network_first_result_send(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(1)?;
        match runtime
            .fixed_provider_network_first_result_send_action(result)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?
        {
            Sys5I3FixedProviderNetworkFirstResultSendAction::Withhold => Ok(()),
            Sys5I3FixedProviderNetworkFirstResultSendAction::Send => {
                self.send_provider_result(runtime, result.clone()).await?;
                runtime
                    .record_fixed_provider_network_first_result_sent(result)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
        }
    }

    /// Execute the requester session-one result phase. A withheld frame is
    /// accepted only as a clean FIN before any prefix byte; a truncated or
    /// nonempty frame is still a frame rejection. The sent/unconsumed profile
    /// reads and validates the exact pending frame, then discards it before
    /// semantic admission; it is not labelled as wire loss or a receipt.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn drive_fixed_provider_network_first_result_receive(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(1)?;
        match runtime
            .fixed_provider_network_first_result_receive_action()
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?
        {
            Sys5I3FixedProviderNetworkFirstResultReceiveAction::ExpectFinishedWithoutFrame => {
                if self.read_blob_or_finished_without_frame().await?.is_some() {
                    return Err(Sys5I3PrivateQuicError::FrameRejected);
                }
                runtime
                    .record_fixed_provider_network_first_result_finished_without_frame()
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
            Sys5I3FixedProviderNetworkFirstResultReceiveAction::Consume => {
                let receipt = self.receive_provider_result_and_consume(runtime).await?;
                runtime
                    .record_fixed_provider_network_first_result_consumed(&receipt)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
            Sys5I3FixedProviderNetworkFirstResultReceiveAction::ReadAndDiscardBeforeConsume => {
                let bytes = self.read_blob().await?;
                let _unbound_receive = self.reserve_provider_transport_occurrence(
                    Sys5I3PrivateProviderTransportOccurrenceKind::ResultReceive,
                )?;
                let result = Sys5I3PrivateProcessCodec::private_provisional_v1()
                    .decode_untrusted_provider_result(&bytes)
                    .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
                runtime
                    .record_fixed_provider_network_first_result_received_and_discarded_before_consume(
                        &result,
                    )
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
        }
    }

    /// Replays only the exact already-consumed requester carrier on the
    /// second session. This does not reopen requester admission or create a
    /// second semantic operation; it fills the bounded second send slot.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn drive_fixed_provider_network_second_request_send(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(2)?;
        let Some(replay) = runtime
            .prepare_fixed_provider_network_second_request_replay()
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?
        else {
            return Ok(());
        };
        let request = replay.request();
        let bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_provider_request(request)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let reservation = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendReservation,
        )?;
        let ticket = runtime
            .reserve_fixed_provider_network_second_request_send(&replay, reservation)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
        let completion = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted,
        )?;
        self.write_blob(&bytes).await?;
        runtime
            .complete_provider_transport_send(ticket, completion)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
    }

    /// Receive the exact consumed requester carrier only for the duplicate
    /// conformance profile. The ordinary runtime boundary must reject it as
    /// the same retained semantic request before host invocation.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn drive_fixed_provider_network_second_request_receive(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(2)?;
        if !runtime
            .expects_fixed_provider_network_second_request_replay()
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?
        {
            return Ok(());
        }
        let bytes = self.read_blob().await?;
        let occurrence = self.reserve_provider_transport_occurrence(
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestReceive,
        )?;
        let request = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_provider_request(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        match runtime.admit_read_only_provider_request_and_execute_from_transport(
            request.clone(),
            occurrence,
        ) {
            Err(error)
                if error.kind() == Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected =>
            {
                runtime
                    .record_fixed_provider_network_second_request_duplicate_rejected(&request)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
            Err(error) => Err(Sys5I3PrivateQuicError::ProviderSemanticRejected(error)),
            Ok(_) => Err(Sys5I3PrivateQuicError::LocalAttemptRejected),
        }
    }

    /// Execute the executor's sealed second-result phase. A held retained
    /// result still crosses the normal current M9 release check immediately
    /// before reservation; the post-call retirement profile must therefore
    /// reject before any result-frame write.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn drive_fixed_provider_network_second_result_send(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
        result: &Sys5I3ProviderResultCarrier,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(2)?;
        match runtime
            .fixed_provider_network_second_result_send_action(result)
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?
        {
            Sys5I3FixedProviderNetworkSecondResultSendAction::FinishWithoutFrame => Ok(()),
            Sys5I3FixedProviderNetworkSecondResultSendAction::Send => {
                let replay = runtime
                    .prepare_fixed_provider_network_second_result_replay(result)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
                let bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
                    .encode_provider_result(replay.result())
                    .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
                let reservation = self.reserve_provider_transport_occurrence(
                    Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendReservation,
                )?;
                let ticket = runtime
                    .reserve_fixed_provider_network_result_send(&replay, reservation)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
                let completion = self.reserve_provider_transport_occurrence(
                    Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted,
                )?;
                self.write_blob(&bytes).await?;
                runtime
                    .complete_provider_transport_send(ticket, completion)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
                runtime
                    .record_fixed_provider_network_second_result_sent(&replay)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
            Sys5I3FixedProviderNetworkSecondResultSendAction::ExpectCurrentnessRejection => {
                let replay = runtime
                    .prepare_fixed_provider_network_second_result_replay(result)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?;
                let reservation = self.reserve_provider_transport_occurrence(
                    Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendReservation,
                )?;
                match runtime.reserve_fixed_provider_network_result_send(&replay, reservation) {
                    Err(error)
                        if error.kind() == Sys5I3ProcessRuntimeErrorKind::MissingCapability =>
                    {
                        runtime
                            .record_fixed_provider_network_second_result_currentness_rejected(
                                result,
                            )
                            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
                    }
                    Err(error) => Err(Sys5I3PrivateQuicError::ProviderSemanticRejected(error)),
                    Ok(_) => Err(Sys5I3PrivateQuicError::LocalAttemptRejected),
                }
            }
        }
    }

    /// Execute the requester session-two result phase. Duplicate result
    /// handling is again delegated to the normal current consume boundary;
    /// it must preserve the original consumed receipt rather than creating a
    /// second consume fact.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn drive_fixed_provider_network_second_result_receive(
        &mut self,
        runtime: &mut Sys5I3InstalledProviderChildRuntime,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_provider_session_generation(2)?;
        match runtime
            .fixed_provider_network_second_result_receive_action()
            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)?
        {
            Sys5I3FixedProviderNetworkSecondResultReceiveAction::Consume => {
                let receipt = self
                    .receive_provider_result_and_consume_at_verified_provider_session_generation(
                        runtime, 2,
                    )
                    .await?;
                runtime
                    .record_fixed_provider_network_second_result_consumed(&receipt)
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
            Sys5I3FixedProviderNetworkSecondResultReceiveAction::ExpectFinishedWithoutFrame => {
                if self.read_blob_or_finished_without_frame().await?.is_some() {
                    return Err(Sys5I3PrivateQuicError::FrameRejected);
                }
                runtime
                    .record_fixed_provider_network_second_result_finished_without_frame()
                    .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
            }
            Sys5I3FixedProviderNetworkSecondResultReceiveAction::ExpectDuplicateRejection => {
                let bytes = self.read_blob().await?;
                let occurrence = self.reserve_provider_transport_occurrence(
                    Sys5I3PrivateProviderTransportOccurrenceKind::ResultReceive,
                )?;
                let result = Sys5I3PrivateProcessCodec::private_provisional_v1()
                    .decode_untrusted_provider_result(&bytes)
                    .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
                match runtime.admit_read_only_provider_result_and_consume_from_transport(
                    result.clone(),
                    occurrence,
                ) {
                    Err(error)
                        if error.kind()
                            == Sys5I3ProcessRuntimeErrorKind::DuplicateRequestRejected =>
                    {
                        runtime
                            .record_fixed_provider_network_second_result_duplicate_rejected(&result)
                            .map_err(Sys5I3PrivateQuicError::ProviderSemanticRejected)
                    }
                    Err(error) => Err(Sys5I3PrivateQuicError::ProviderSemanticRejected(error)),
                    Ok(_) => Err(Sys5I3PrivateQuicError::LocalAttemptRejected),
                }
            }
        }
    }

    /// Encodes and sends a generated request/reply over this adapter-owned
    /// stream.  The returned references are hashes of the actual private
    /// carrier bytes and run-salted network occurrence, not inferred counts.
    pub async fn send_generated_message(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        let encoded = Self::encode_generated_message(message)?;
        let (body, evidence) = self.reserve_generated_message_delivery(encoded)?;
        self.write_blob(&body).await?;
        Ok(evidence)
    }

    /// Row20 owner-side reply delivery.  It reuses the ordinary bounded
    /// carrier encoding and write path but is reachable only from the
    /// factory-issued owner custody session.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn send_process_local_cut_generated_message(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        let encoded = Self::encode_generated_message(message)?;
        let (body, evidence) = self.reserve_generated_message_delivery(encoded)?;
        self.write_blob(&body).await?;
        Ok(evidence)
    }

    /// Sends the genuine first owner Reply and retains its exact encoded body
    /// only for the one fixed same-session late-reply conformance path.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn send_process_local_cut_generated_reply_and_retain_for_late_admission(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply, Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        let first_request_identity_ref = message.semantic_request_identity_ref().to_string();
        let cohort_provenance_ref = message.cohort_provenance_ref().to_string();
        if message.linked_request_identity_ref() != Some(first_request_identity_ref.as_str()) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let control_binding = self
            .require_process_local_cut_control(Sys5I3ProcessLocalCutControlRole::Owner)?
            .localnet_preface()
            .clone();
        if control_binding.cohort_provenance_ref() != cohort_provenance_ref.as_str() {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let encoded = Self::encode_generated_message(message)?;
        let (body, _) = self.reserve_generated_message_delivery(encoded)?;
        self.write_blob(&body).await?;
        Ok(Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply {
            body,
            first_request_identity_ref,
            cohort_provenance_ref,
            control_binding,
            connection_stable_id: self.connection.stable_id(),
            session_attempt_generation: self.session_attempt_generation,
        })
    }

    /// Writes the one retained Row20 first Reply only after the same owner
    /// runtime has genuinely admitted and served a distinct later request.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn send_retained_process_local_cut_reply_after_later_admission(
        &mut self,
        runtime: &Sys5I3ProcessRuntime,
        retained: Sys5I3PrivateQuicProcessLocalCutRetainedOwnerReply,
        second_reply: &Sys5I3ProcessMessage,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        let control_binding = self
            .require_process_local_cut_control(Sys5I3ProcessLocalCutControlRole::Owner)?
            .localnet_preface();
        if retained.connection_stable_id != self.connection.stable_id()
            || retained.session_attempt_generation != self.session_attempt_generation
            || retained.control_binding != control_binding.clone()
            || retained.cohort_provenance_ref.as_str() != control_binding.cohort_provenance_ref()
            || retained.cohort_provenance_ref.as_str() != second_reply.cohort_provenance_ref()
            || !runtime.validates_i3_process_local_cut_late_reply_after_second_admission(
                &retained.first_request_identity_ref,
                second_reply,
            )
        {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let carrier_ref = carrier_ref(&retained.body);
        self.reserve_network_occurrence_ref("send", &carrier_ref)?;
        self.write_blob(&retained.body).await
    }

    /// Test-only source-first replay falsifier.  It retains exactly one
    /// generated Reply only after the verified initial session completed the
    /// full bounded stream write.  Callers provide neither bytes nor a
    /// decoded carrier, and a generated Request cannot mint this candidate.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn test_only_send_generated_reply_and_retain_replay_candidate(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<
        (
            Sys5I3PrivateQuicDeliveryEvidence,
            Sys5I3PrivateQuicGeneratedReplyReplayCandidate,
        ),
        Sys5I3PrivateQuicError,
    > {
        self.require_ordinary_control()?;
        self.require_verified_session_generation(1)?;
        if self.generated_reply_replay_candidate_issued {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let encoded = Self::encode_generated_message(message)?;
        if encoded.linked_request_identity_ref.as_deref()
            != Some(encoded.semantic_request_identity_ref.as_str())
        {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let (body, evidence) = self.reserve_generated_message_delivery(encoded)?;
        self.write_blob(&body).await?;
        self.generated_reply_replay_candidate_issued = true;
        let candidate = Sys5I3PrivateQuicGeneratedReplyReplayCandidate {
            body,
            session_attempt_generation: self.session_attempt_generation,
            control_binding: self.require_ordinary_control()?.localnet_preface(),
            original_send_delivery: evidence.clone(),
        };
        Ok((evidence, candidate))
    }

    /// Consume the one opaque Reply replay candidate on the verified exact
    /// successor session.  A wrong session/control rejects before any replay
    /// occurrence is reserved or bytes are written; consuming the candidate
    /// still prevents retrying that rejected local attempt.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn test_only_replay_retained_generated_reply(
        &mut self,
        candidate: Sys5I3PrivateQuicGeneratedReplyReplayCandidate,
    ) -> Result<Sys5I3PrivateQuicReplayedGeneratedReplyDelivery, Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        self.require_verified_session_generation(2)?;
        let control_binding = self.require_ordinary_control()?.localnet_preface();
        if !retained_ingress_matches_reconnect_binding(
            candidate.session_attempt_generation,
            &candidate.control_binding,
            self.session_attempt_generation,
            &control_binding,
        ) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&candidate.body);
        let network_occurrence_ref = self.reserve_network_occurrence_ref(
            "send",
            candidate.original_send_delivery.carrier_ref(),
        )?;
        self.write_blob(&candidate.body).await?;
        let mut replay_delivery = candidate.original_send_delivery.clone();
        replay_delivery.candidate_commitment_ref = candidate_commitment_ref;
        replay_delivery.network_occurrence_ref = network_occurrence_ref;
        Ok(Sys5I3PrivateQuicReplayedGeneratedReplyDelivery {
            replay_delivery,
            original_send_delivery: candidate.original_send_delivery,
        })
    }

    /// Sends one existing source-generated message through a bounded
    /// test-profile stream-write control.  The control never accepts raw
    /// caller bytes or exposes packetization.  Invalid controls reject before
    /// reserving an occurrence or writing the stream.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn test_only_send_generated_message_with_frame_write_control(
        &mut self,
        message: Sys5I3ProcessMessage,
        control: Sys5I3PrivateQuicGeneratedFrameWriteControl,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        let encoded = Self::encode_generated_message(message)?;
        control.validate_encoded_body(&encoded.body)?;
        let (body, mut evidence) = self.reserve_generated_message_delivery(encoded)?;
        match control {
            Sys5I3PrivateQuicGeneratedFrameWriteControl::CompleteInTwoWrites {
                first_frame_fragment_len,
            } => {
                let observation = self
                    .write_complete_blob_in_two_writes(&body, first_frame_fragment_len)
                    .await?;
                evidence.generated_frame_write_observation = Some(observation);
                Ok(evidence)
            }
            Sys5I3PrivateQuicGeneratedFrameWriteControl::TruncateAfterBodyPrefix {
                body_prefix_len,
            } => {
                self.write_truncated_blob_then_finish(&body, body_prefix_len)
                    .await?;
                Err(Sys5I3PrivateQuicError::FrameRejected)
            }
        }
    }

    /// Begin the one source-emitted original request on its first checked
    /// session.  The runtime authorizes the attempt from its retained exact
    /// carrier immediately before this adapter begins a write; a frame error
    /// leaves the original pending while consuming only its attempt budget.
    pub async fn send_initial_original_owner_request(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        self.send_original_owner_request_attempt(
            runtime,
            pending,
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
            1,
        )
        .await
    }

    /// Send one exact source-emitted requester request from the Row20
    /// custody session.  The retained runtime pending handle and the normal
    /// authorization/attempt accounting remain the only write authority.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn send_process_local_cut_original_owner_request(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(
            Sys5I3ProcessLocalCutControlRole::Requester,
        )?;
        self.send_original_owner_request_attempt(
            runtime,
            pending,
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
            1,
        )
        .await
    }

    /// Begins the fixed cancellation conformance request by writing only the
    /// exact authorized frame length.  The normal authorization, attempt,
    /// occurrence, and runtime-commit sequence remains unchanged; only the
    /// physical body write is deliberately left unavailable to the peer.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn send_process_local_cut_original_owner_request_length_prefix_then_hold(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(
            Sys5I3ProcessLocalCutControlRole::Requester,
        )?;
        self.send_original_owner_request_attempt_length_prefix_then_hold(
            runtime,
            pending,
            Sys5I3OriginalOwnerRequestAttemptKind::InitialDelivery,
            1,
        )
        .await
    }

    /// Retry the unchanged original request only after its control has moved
    /// through `Sys5I3PrivateQuicReconnect` into the fixed second session.
    /// The retry is an attempt outcome, never a replacement request/result.
    pub async fn retry_original_owner_request_after_reconnect(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        self.send_original_owner_request_attempt(
            runtime,
            pending,
            Sys5I3OriginalOwnerRequestAttemptKind::ReconnectRetry,
            2,
        )
        .await
    }

    /// Reads exactly one complete bounded carrier frame and invokes the
    /// crate-private decoded admission core.  This method can only be called
    /// after this session has checked mTLS/SPKI and reciprocal preface.
    pub async fn receive_and_admit_generated_message(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
    ) -> Result<
        (
            Option<Sys5I3ProcessMessage>,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        self.require_ordinary_control()?;
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        self.receive_and_admit_generated_message_inner(runtime)
            .await
    }

    /// Direct admission of a later Row20 requester message.  The first
    /// owner ingress uses the retained-ingress path below; this method is
    /// only available after the same owned session has already verified its
    /// factory-issued owner purpose.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn receive_and_admit_process_local_cut_generated_message(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
    ) -> Result<
        (
            Option<Sys5I3ProcessMessage>,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        self.receive_and_admit_generated_message_inner(runtime)
            .await
    }

    async fn receive_and_admit_generated_message_inner(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
    ) -> Result<
        (
            Option<Sys5I3ProcessMessage>,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        let bytes = self.read_blob().await?;
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&bytes);
        let network_occurrence_ref =
            self.reserve_network_occurrence_ref("receive", &candidate_commitment_ref)?;
        let candidate = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_message(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let lineage = carrier_lineage(&bytes)?;
        let manifest = candidate.observer_safe_manifest();
        let admitted = runtime
            .admit_decoded_process_message(candidate)
            .map_err(|error| Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt: Sys5I3PrivateQuicRejectedAttemptEvidence {
                    rejected_candidate_commitment_ref: candidate_commitment_ref.clone(),
                    network_occurrence_ref: network_occurrence_ref.clone(),
                },
            })?;
        let semantic_request_identity_ref = manifest.semantic_request_identity_ref().to_string();
        let linked_request_identity_ref =
            manifest.linked_request_identity_ref().map(str::to_string);
        let carrier_ref = carrier_ref(&bytes);
        Ok((
            admitted,
            Sys5I3PrivateQuicDeliveryEvidence {
                carrier_ref,
                candidate_commitment_ref,
                semantic_request_identity_ref,
                linked_request_identity_ref,
                source_ref: lineage.source_ref,
                core_ref: lineage.core_ref,
                source_artifact_ref: lineage.source_artifact_ref,
                target_artifact_ref: lineage.target_artifact_ref,
                edge_ref: lineage.edge_ref,
                network_occurrence_ref,
                #[cfg(feature = "i3-process-test-seams")]
                generated_frame_write_observation: None,
            },
        ))
    }

    /// Read one complete frame from the verified initial session without
    /// semantic admission.  The returned opaque value is the sole retained
    /// frame; it cannot be cloned, decoded by a caller, or supplied to a
    /// different run/control/session.
    pub async fn receive_complete_pending_ingress(
        &mut self,
    ) -> Result<Sys5I3PrivateQuicPendingIngress, Sys5I3PrivateQuicError> {
        self.require_ordinary_control()?;
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        if self.session_attempt_generation != 1 {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        self.receive_complete_pending_ingress_inner().await
    }

    /// Reserve and retain the Row20 owner's first complete ingress before
    /// semantic admission.  Reservation occurs before the physical read, so
    /// an error or cancellation cannot make the received frame acquirable a
    /// second time.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn receive_complete_process_local_cut_pending_ingress(
        &mut self,
    ) -> Result<Sys5I3PrivateQuicPendingIngress, Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        self.receive_complete_pending_ingress_inner().await
    }

    /// Exercises the fixed cancellation boundary after a real requester has
    /// written an exact frame header.  It reserves ingress before awaiting,
    /// proves that the body read is still pending, then drops that read while
    /// preserving the reserved permit.  No body bytes are decoded, admitted,
    /// or claimed as received.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn cancel_process_local_cut_pending_ingress_after_complete_header(
        &mut self,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        self.pending_ingress_permit.reserve_acquisition()?;
        let body_len = self.read_blob_header().await?;
        self.read_blob_body_once_must_be_pending(body_len).await
    }

    /// True only for a verified factory-issued custody session whose retained
    /// ingress permit has not entered acquisition.  This is a local custody
    /// guard, not a transport-liveness or peer-quiescence claim.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn has_unacquired_verified_process_local_cut_custody(
        &self,
        role: Sys5I3ProcessLocalCutControlRole,
    ) -> bool {
        self.require_verified_process_local_cut_session(role)
            .is_ok()
            && matches!(
                &self.pending_ingress_permit,
                Sys5I3PrivateQuicPendingIngressPermit::Unacquired
            )
    }

    async fn receive_complete_pending_ingress_inner(
        &mut self,
    ) -> Result<Sys5I3PrivateQuicPendingIngress, Sys5I3PrivateQuicError> {
        // Reserve before the first await so cancellation, partial reads, and
        // codec failure cannot reopen this one ingress acquisition.
        self.pending_ingress_permit.reserve_acquisition()?;
        let bytes = self.read_blob().await?;
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&bytes);
        let network_occurrence_ref =
            self.reserve_network_occurrence_ref("receive", &candidate_commitment_ref)?;
        let candidate = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_message(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        self.pending_ingress_permit
            .complete_acquisition(&network_occurrence_ref)?;
        let control_binding = match &self.control {
            PrivateQuicControl::Ordinary(control) => control.localnet_preface(),
            PrivateQuicControl::Provider(_) => {
                return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
            }
        };
        Ok(Sys5I3PrivateQuicPendingIngress {
            bytes,
            candidate,
            session_attempt_generation: self.session_attempt_generation,
            control_binding,
            candidate_commitment_ref,
            network_occurrence_ref,
        })
    }

    /// Consume one retained, verified session-one ingress on the exact
    /// successor session.  It performs no additional read, send, retry, or
    /// occurrence reservation.  A rejected semantic admission exposes only
    /// the receiver-owned session-one commitment/occurrence; full delivery
    /// lineage is constructed solely after the runtime accepts the candidate.
    pub fn admit_retained_pending_ingress(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: Sys5I3PrivateQuicPendingIngress,
    ) -> Result<
        (
            Option<Sys5I3ProcessMessage>,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        self.require_ordinary_control()?;
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        let reconnect_control_binding = self.require_ordinary_control()?.localnet_preface();
        if !retained_ingress_matches_reconnect_binding(
            pending.session_attempt_generation,
            &pending.control_binding,
            self.session_attempt_generation,
            &reconnect_control_binding,
        ) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        self.admit_pending_ingress_inner(runtime, pending)
    }

    /// Consume the one first-session ingress held by the Row20 owner.  This
    /// preserves the adapter's original reservation/complete/consume state
    /// machine but compares the same-session factory-issued control binding
    /// instead of permitting a generic reconnect promotion.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) fn admit_process_local_cut_pending_ingress(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: Sys5I3PrivateQuicPendingIngress,
    ) -> Result<
        (
            Option<Sys5I3ProcessMessage>,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        self.require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Owner)?;
        let control_binding = self
            .require_process_local_cut_control(Sys5I3ProcessLocalCutControlRole::Owner)?
            .localnet_preface();
        if pending.session_attempt_generation != self.session_attempt_generation
            || pending.control_binding != control_binding
        {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        self.admit_pending_ingress_inner(runtime, pending)
    }

    fn admit_pending_ingress_inner(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: Sys5I3PrivateQuicPendingIngress,
    ) -> Result<
        (
            Option<Sys5I3ProcessMessage>,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        self.pending_ingress_permit
            .consume_completed(&pending.network_occurrence_ref)?;
        let Sys5I3PrivateQuicPendingIngress {
            bytes,
            candidate,
            session_attempt_generation: _,
            control_binding: _,
            candidate_commitment_ref,
            network_occurrence_ref,
        } = pending;
        let admitted = runtime
            .admit_decoded_process_message(candidate)
            .map_err(|error| Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt: Sys5I3PrivateQuicRejectedAttemptEvidence {
                    rejected_candidate_commitment_ref: candidate_commitment_ref.clone(),
                    network_occurrence_ref: network_occurrence_ref.clone(),
                },
            })?;
        let lineage = carrier_lineage(&bytes)?;
        let manifest = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_message(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?
            .observer_safe_manifest();
        Ok((
            admitted,
            Sys5I3PrivateQuicDeliveryEvidence {
                carrier_ref: carrier_ref(&bytes),
                candidate_commitment_ref,
                semantic_request_identity_ref: manifest.semantic_request_identity_ref().to_string(),
                linked_request_identity_ref: manifest
                    .linked_request_identity_ref()
                    .map(str::to_string),
                source_ref: lineage.source_ref,
                core_ref: lineage.core_ref,
                source_artifact_ref: lineage.source_artifact_ref,
                target_artifact_ref: lineage.target_artifact_ref,
                edge_ref: lineage.edge_ref,
                network_occurrence_ref,
                #[cfg(feature = "i3-process-test-seams")]
                generated_frame_write_observation: None,
            },
        ))
    }

    /// Receive one exact owner reply for an opaque original pending handle.
    /// A local receipt or terminal consumption consumes the handle by value;
    /// every frame, codec, or semantic rejection returns it intact so a later
    /// original reply can still complete exactly once through the runtime's
    /// pending map.
    pub async fn receive_and_admit_original_owner_reply(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: Sys5I3OriginalOwnerRequestPending,
    ) -> Sys5I3PrivateQuicOriginalOwnerReplyOutcome {
        if self.require_ordinary_control().is_err() {
            return Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
                pending,
                error: Sys5I3PrivateQuicError::LocalAttemptRejected,
            };
        }
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
                pending,
                error: Sys5I3PrivateQuicError::peer_binding_rejected(
                    self.control.expected_peer_spki_ref(),
                ),
            };
        }
        let result = self
            .receive_and_admit_original_owner_reply_inner(runtime, &pending)
            .await;
        match result {
            Ok((admitted, delivery)) => match admitted {
                Sys5I3PrivateQuicAdmittedOriginalOwnerReply::Receipt(receipt) => {
                    Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed {
                        receipt: Box::new(receipt),
                        delivery,
                    }
                }
                Sys5I3PrivateQuicAdmittedOriginalOwnerReply::TerminalFailureConsumed(terminal) => {
                    Sys5I3PrivateQuicOriginalOwnerReplyOutcome::TerminalFailureConsumed {
                        terminal: Box::new(terminal),
                        delivery,
                    }
                }
            },
            Err(error) => Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending { pending, error },
        }
    }

    /// Receive one requester-local reply through the factory-issued Row20
    /// custody session.  It uses the ordinary exact pending/receipt path;
    /// only the control-purpose gate differs.
    #[cfg(feature = "i3-process-test-seams")]
    pub(crate) async fn receive_and_admit_process_local_cut_original_owner_reply(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: Sys5I3OriginalOwnerRequestPending,
    ) -> Sys5I3PrivateQuicOriginalOwnerReplyOutcome {
        if self
            .require_verified_process_local_cut_session(Sys5I3ProcessLocalCutControlRole::Requester)
            .is_err()
        {
            return Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending {
                pending,
                error: Sys5I3PrivateQuicError::LocalAttemptRejected,
            };
        }
        match self
            .receive_and_admit_original_owner_reply_inner(runtime, &pending)
            .await
        {
            Ok((admitted, delivery)) => match admitted {
                Sys5I3PrivateQuicAdmittedOriginalOwnerReply::Receipt(receipt) => {
                    Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed {
                        receipt: Box::new(receipt),
                        delivery,
                    }
                }
                Sys5I3PrivateQuicAdmittedOriginalOwnerReply::TerminalFailureConsumed(terminal) => {
                    Sys5I3PrivateQuicOriginalOwnerReplyOutcome::TerminalFailureConsumed {
                        terminal: Box::new(terminal),
                        delivery,
                    }
                }
            },
            Err(error) => Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Pending { pending, error },
        }
    }

    /// Completes this side's one request/reply direction.  The caller keeps
    /// the connection alive until the opposite half is consumed.
    pub fn finish_send(&mut self) -> Result<(), Sys5I3PrivateQuicError> {
        self.send
            .finish()
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)
    }

    pub fn close(&self) {
        self.connection.close(0_u32.into(), b"i3-private-complete");
    }

    /// Server-side lifecycle coordination only. This waits for physical
    /// connection closure after a finished stream; it is neither a semantic
    /// receipt nor a delivery acknowledgement and may precede a later
    /// sealed-session completion report.
    pub async fn wait_for_peer_close(&self) {
        let _ = self.connection.closed().await;
    }

    fn encode_generated_message(
        message: Sys5I3ProcessMessage,
    ) -> Result<Sys5I3PrivateQuicEncodedGeneratedMessage, Sys5I3PrivateQuicError> {
        let semantic_request_identity_ref = message.semantic_request_identity_ref().to_string();
        let linked_request_identity_ref = message.linked_request_identity_ref().map(str::to_string);
        let body = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_outbound_message(message)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let lineage = carrier_lineage(&body)?;
        Ok(Sys5I3PrivateQuicEncodedGeneratedMessage {
            body,
            semantic_request_identity_ref,
            linked_request_identity_ref,
            lineage,
        })
    }

    fn reserve_generated_message_delivery(
        &mut self,
        encoded: Sys5I3PrivateQuicEncodedGeneratedMessage,
    ) -> Result<(Vec<u8>, Sys5I3PrivateQuicDeliveryEvidence), Sys5I3PrivateQuicError> {
        let Sys5I3PrivateQuicEncodedGeneratedMessage {
            body,
            semantic_request_identity_ref,
            linked_request_identity_ref,
            lineage,
        } = encoded;
        let carrier_ref = carrier_ref(&body);
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&body);
        let network_occurrence_ref = self.reserve_network_occurrence_ref("send", &carrier_ref)?;
        Ok((
            body,
            Sys5I3PrivateQuicDeliveryEvidence {
                carrier_ref,
                candidate_commitment_ref,
                semantic_request_identity_ref,
                linked_request_identity_ref,
                source_ref: lineage.source_ref,
                core_ref: lineage.core_ref,
                source_artifact_ref: lineage.source_artifact_ref,
                target_artifact_ref: lineage.target_artifact_ref,
                edge_ref: lineage.edge_ref,
                network_occurrence_ref,
                #[cfg(feature = "i3-process-test-seams")]
                generated_frame_write_observation: None,
            },
        ))
    }

    async fn write_blob(&mut self, body: &[u8]) -> Result<(), Sys5I3PrivateQuicError> {
        let prefix = private_quic_blob_prefix(body)?;
        self.send
            .write_all(&prefix)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.send
            .write_all(body)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)
    }

    #[cfg(feature = "i3-process-test-seams")]
    async fn write_blob_length_prefix_only(
        &mut self,
        body: &[u8],
    ) -> Result<(), Sys5I3PrivateQuicError> {
        let prefix = private_quic_blob_prefix(body)?;
        self.send
            .write_all(&prefix)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)
    }

    #[cfg(feature = "i3-process-test-seams")]
    async fn write_complete_blob_in_two_writes(
        &mut self,
        body: &[u8],
        first_frame_fragment_len: usize,
    ) -> Result<Sys5I3PrivateQuicGeneratedFrameWriteObservation, Sys5I3PrivateQuicError> {
        let prefix = private_quic_blob_prefix(body)?;
        if first_frame_fragment_len == 0 || first_frame_fragment_len >= prefix.len() {
            return Err(Sys5I3PrivateQuicError::FrameRejected);
        }
        let mut frame = Vec::with_capacity(prefix.len() + body.len());
        frame.extend_from_slice(&prefix);
        frame.extend_from_slice(body);
        let mut application_write_count = 0_u8;
        self.send
            .write_all(&frame[..first_frame_fragment_len])
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        application_write_count = application_write_count
            .checked_add(1)
            .ok_or(Sys5I3PrivateQuicError::FrameRejected)?;
        self.send
            .write_all(&frame[first_frame_fragment_len..])
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        application_write_count = application_write_count
            .checked_add(1)
            .ok_or(Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Sys5I3PrivateQuicGeneratedFrameWriteObservation {
            application_write_count,
            frame_prefix_split_across_writes: true,
            complete_frame_written: true,
        })
    }

    #[cfg(feature = "i3-process-test-seams")]
    async fn write_truncated_blob_then_finish(
        &mut self,
        body: &[u8],
        body_prefix_len: usize,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        let prefix = private_quic_blob_prefix(body)?;
        self.send
            .write_all(&prefix)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.send
            .write_all(&body[..body_prefix_len])
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.finish_send()
    }

    async fn read_blob(&mut self) -> Result<Vec<u8>, Sys5I3PrivateQuicError> {
        let length = self.read_blob_header().await?;
        self.read_blob_body(length).await
    }

    /// Reads and bounds only the wire length prefix.  Callers which do not
    /// subsequently acquire a body must not infer a decoded frame or an
    /// admitted ingress from this physical framing step.
    async fn read_blob_header(&mut self) -> Result<usize, Sys5I3PrivateQuicError> {
        let mut prefix = [0_u8; 4];
        self.receive
            .read_exact(&mut prefix)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        private_quic_blob_body_len(prefix)
    }

    async fn read_blob_body(&mut self, length: usize) -> Result<Vec<u8>, Sys5I3PrivateQuicError> {
        let mut body = vec![0; length];
        self.receive
            .read_exact(&mut body)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(body)
    }

    /// Polls a real body read exactly once after a complete header.  A ready
    /// result, whether success or error, is not cancellation evidence.
    #[cfg(feature = "i3-process-test-seams")]
    async fn read_blob_body_once_must_be_pending(
        &mut self,
        length: usize,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        let was_pending = {
            let mut body = vec![0; length];
            let mut body_read = std::pin::pin!(self.receive.read_exact(&mut body));
            std::future::poll_fn(|context| {
                match std::future::Future::poll(body_read.as_mut(), context) {
                    std::task::Poll::Pending => std::task::Poll::Ready(true),
                    std::task::Poll::Ready(_) => std::task::Poll::Ready(false),
                }
            })
            .await
        };
        if was_pending {
            Ok(())
        } else {
            Err(Sys5I3PrivateQuicError::LocalAttemptRejected)
        }
    }

    /// Read either one complete bounded blob or a clean stream FIN before the
    /// first prefix byte. A partial prefix, declared body, or body read is a
    /// rejected frame rather than an absent result.
    #[cfg(feature = "i3-process-test-seams")]
    async fn read_blob_or_finished_without_frame(
        &mut self,
    ) -> Result<Option<Vec<u8>>, Sys5I3PrivateQuicError> {
        let mut prefix = [0_u8; 4];
        match self.receive.read_exact(&mut prefix).await {
            Ok(()) => {}
            Err(ReadExactError::FinishedEarly(0)) => return Ok(None),
            Err(_) => return Err(Sys5I3PrivateQuicError::FrameRejected),
        }
        let length = private_quic_blob_body_len(prefix)?;
        self.read_blob_body(length).await.map(Some)
    }

    async fn send_original_owner_request_attempt(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
        kind: Sys5I3OriginalOwnerRequestAttemptKind,
        required_session_attempt_generation: u8,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        let (bytes, evidence) = self.prepare_original_owner_request_attempt(
            runtime,
            pending,
            kind,
            required_session_attempt_generation,
        )?;
        self.write_blob(&bytes).await?;
        Ok(evidence)
    }

    async fn send_original_owner_request_attempt_length_prefix_then_hold(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
        kind: Sys5I3OriginalOwnerRequestAttemptKind,
        required_session_attempt_generation: u8,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        let (bytes, evidence) = self.prepare_original_owner_request_attempt(
            runtime,
            pending,
            kind,
            required_session_attempt_generation,
        )?;
        self.write_blob_length_prefix_only(&bytes).await?;
        Ok(evidence)
    }

    fn prepare_original_owner_request_attempt(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
        kind: Sys5I3OriginalOwnerRequestAttemptKind,
        required_session_attempt_generation: u8,
    ) -> Result<(Vec<u8>, Sys5I3PrivateQuicDeliveryEvidence), Sys5I3PrivateQuicError> {
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        if self.session_attempt_generation != required_session_attempt_generation {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let authorization = runtime
            .authorize_original_owner_request_attempt(pending, kind)
            .map_err(Sys5I3PrivateQuicError::OriginalRequestAttemptRejected)?;
        let bytes = authorization.encoded_message_bytes();
        let lineage = carrier_lineage(bytes)?;
        let carrier_ref = carrier_ref(bytes);
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(bytes);
        let network_occurrence_ref = self.reserve_network_occurrence_ref("send", &carrier_ref)?;
        runtime
            .commit_authorized_original_owner_request_attempt(&authorization)
            .map_err(Sys5I3PrivateQuicError::OriginalRequestAttemptRejected)?;
        Ok((
            bytes.to_vec(),
            Sys5I3PrivateQuicDeliveryEvidence {
                carrier_ref,
                candidate_commitment_ref,
                semantic_request_identity_ref: authorization
                    .semantic_request_identity_ref()
                    .to_string(),
                linked_request_identity_ref: None,
                source_ref: lineage.source_ref,
                core_ref: lineage.core_ref,
                source_artifact_ref: lineage.source_artifact_ref,
                target_artifact_ref: lineage.target_artifact_ref,
                edge_ref: lineage.edge_ref,
                network_occurrence_ref,
                #[cfg(feature = "i3-process-test-seams")]
                generated_frame_write_observation: None,
            },
        ))
    }

    async fn receive_and_admit_original_owner_reply_inner(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<
        (
            Sys5I3PrivateQuicAdmittedOriginalOwnerReply,
            Sys5I3PrivateQuicDeliveryEvidence,
        ),
        Sys5I3PrivateQuicError,
    > {
        let bytes = self.read_blob().await?;
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&bytes);
        let network_occurrence_ref =
            self.reserve_network_occurrence_ref("receive", &candidate_commitment_ref)?;
        let candidate = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_message(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let lineage = carrier_lineage(&bytes)?;
        let manifest = candidate.observer_safe_manifest();
        let admitted = runtime
            .admit_decoded_original_owner_reply(candidate, pending)
            .map_err(|error| Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt: Sys5I3PrivateQuicRejectedAttemptEvidence {
                    rejected_candidate_commitment_ref: candidate_commitment_ref.clone(),
                    network_occurrence_ref: network_occurrence_ref.clone(),
                },
            })?;
        let admitted = classify_admitted_original_owner_reply(admitted)?;
        let carrier_ref = carrier_ref(&bytes);
        Ok((
            admitted,
            Sys5I3PrivateQuicDeliveryEvidence {
                carrier_ref,
                candidate_commitment_ref,
                semantic_request_identity_ref: manifest.semantic_request_identity_ref().to_string(),
                linked_request_identity_ref: manifest
                    .linked_request_identity_ref()
                    .map(str::to_string),
                source_ref: lineage.source_ref,
                core_ref: lineage.core_ref,
                source_artifact_ref: lineage.source_artifact_ref,
                target_artifact_ref: lineage.target_artifact_ref,
                edge_ref: lineage.edge_ref,
                network_occurrence_ref,
                #[cfg(feature = "i3-process-test-seams")]
                generated_frame_write_observation: None,
            },
        ))
    }

    fn reserve_network_occurrence_ref(
        &mut self,
        direction: &str,
        attempt_material_ref: &str,
    ) -> Result<String, Sys5I3PrivateQuicError> {
        self.next_network_occurrence = self
            .next_network_occurrence
            .checked_add(1)
            .ok_or(Sys5I3PrivateQuicError::NetworkOccurrenceExhausted)?;
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/i3/private-quic/network-occurrence/v2\0");
        for component in [self.control.run_ref(), direction, attempt_material_ref] {
            hasher.update((component.len() as u64).to_be_bytes());
            hasher.update(component.as_bytes());
        }
        hasher.update(self.session_attempt_generation.to_be_bytes());
        hasher.update(self.next_network_occurrence.to_be_bytes());
        Ok(format!(
            "i3-private-quic-network-occurrence-sha256-v2:{:x}",
            hasher.finalize()
        ))
    }

    /// Allocate one provider adapter occurrence from only the authenticated
    /// run, this bounded session generation, a fixed direction, and a local
    /// ordinal. It never reads a carrier frame, so neither a value nor a
    /// value-derived hash can reach an error, audit, or observer candidate.
    fn reserve_provider_transport_occurrence(
        &mut self,
        kind: Sys5I3PrivateProviderTransportOccurrenceKind,
    ) -> Result<Sys5I3PrivateProviderTransportOccurrence, Sys5I3PrivateQuicError> {
        self.next_network_occurrence = self
            .next_network_occurrence
            .checked_add(1)
            .ok_or(Sys5I3PrivateQuicError::NetworkOccurrenceExhausted)?;
        let direction = match kind {
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendReservation => {
                "provider-request-send-reservation"
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestSendCompleted => {
                "provider-request-send-completed"
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::RequestReceive => {
                "provider-request-complete-receive"
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendReservation => {
                "provider-result-send-reservation"
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultSendCompleted => {
                "provider-result-send-completed"
            }
            Sys5I3PrivateProviderTransportOccurrenceKind::ResultReceive => {
                "provider-result-complete-receive"
            }
        };
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/i3/private-quic/provider-network-occurrence/v1\0");
        for component in [self.control.run_ref(), direction] {
            hasher.update((component.len() as u64).to_be_bytes());
            hasher.update(component.as_bytes());
        }
        hasher.update(self.session_attempt_generation.to_be_bytes());
        hasher.update(self.next_network_occurrence.to_be_bytes());
        Sys5I3PrivateProviderTransportOccurrence::from_private_quic(
            kind,
            format!(
                "i3-provider-network-occurrence-sha256-v1:{:x}",
                hasher.finalize()
            ),
        )
        .map_err(|_| Sys5I3PrivateQuicError::LocalAttemptRejected)
    }

    fn candidate_commitment_ref_for_frame(&self, bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/i3/private-quic/rejected-attempt/v1\0");
        hasher.update((self.control.run_ref().len() as u64).to_be_bytes());
        hasher.update(self.control.run_ref().as_bytes());
        hasher.update(self.session_attempt_generation.to_be_bytes());
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
        format!(
            "i3-private-quic-rejected-attempt-sha256-v1:{:x}",
            hasher.finalize()
        )
    }
}

impl Sys5I3PrivateQuicReconnect {
    /// Open the fixed second client-side session by consuming the retained
    /// control.  A failed attempt intentionally does not return the control:
    /// retry cannot turn one control into an unbounded connection manager.
    pub async fn connect(
        self,
        connection: Connection,
    ) -> Result<Sys5I3PrivateQuicSession, Sys5I3PrivateQuicError> {
        let Self {
            control,
            prior_session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
        } = self;
        if !matches!(&control, PrivateQuicControl::Ordinary(control) if control.has_ordinary_purpose())
        {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let session_attempt_generation =
            next_reconnect_session_attempt_generation(prior_session_attempt_generation)?;
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .open_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Sys5I3PrivateQuicSession {
            connection,
            send,
            receive,
            control,
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
            #[cfg(feature = "i3-process-test-seams")]
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Server-side counterpart to `connect`; it has the same consuming,
    /// two-session bound and transfers the exact occurrence counter.
    pub async fn accept(
        self,
        connection: Connection,
    ) -> Result<Sys5I3PrivateQuicSession, Sys5I3PrivateQuicError> {
        let Self {
            control,
            prior_session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
        } = self;
        if !matches!(&control, PrivateQuicControl::Ordinary(control) if control.has_ordinary_purpose())
        {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let session_attempt_generation =
            next_reconnect_session_attempt_generation(prior_session_attempt_generation)?;
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .accept_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Sys5I3PrivateQuicSession {
            connection,
            send,
            receive,
            control,
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
            #[cfg(feature = "i3-process-test-seams")]
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Consume the sealed provider reconnect control into the one permitted
    /// second client session. This entry is available only to the fixed
    /// conformance profile; ordinary provider delivery remains one session.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn connect_provider(
        self,
        connection: Connection,
    ) -> Result<Sys5I3PrivateQuicSession, Sys5I3PrivateQuicError> {
        let Self {
            control,
            prior_session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
        } = self;
        let PrivateQuicControl::Provider(provider_control) = &control else {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        };
        if !provider_control.permits_fixed_provider_network_conformance() {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let session_attempt_generation =
            next_reconnect_session_attempt_generation(prior_session_attempt_generation)?;
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .open_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Sys5I3PrivateQuicSession {
            connection,
            send,
            receive,
            control,
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
            generated_reply_replay_candidate_issued: false,
        })
    }

    /// Server counterpart to `connect_provider`, preserving the same single
    /// consuming control, occurrence counter, and two-session bound.
    #[cfg(feature = "i3-process-test-seams")]
    #[doc(hidden)]
    pub async fn accept_provider(
        self,
        connection: Connection,
    ) -> Result<Sys5I3PrivateQuicSession, Sys5I3PrivateQuicError> {
        let Self {
            control,
            prior_session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
        } = self;
        let PrivateQuicControl::Provider(provider_control) = &control else {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        };
        if !provider_control.permits_fixed_provider_network_conformance() {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        let session_attempt_generation =
            next_reconnect_session_attempt_generation(prior_session_attempt_generation)?;
        verify_exact_peer_spki(&connection, control.expected_peer_spki_ref())?;
        let (send, receive) = connection
            .accept_bi()
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(Sys5I3PrivateQuicSession {
            connection,
            send,
            receive,
            control,
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation,
            next_network_occurrence,
            pending_ingress_permit,
            generated_reply_replay_candidate_issued: false,
        })
    }
}

fn next_reconnect_session_attempt_generation(
    prior_session_attempt_generation: u8,
) -> Result<u8, Sys5I3PrivateQuicError> {
    prior_session_attempt_generation
        .checked_add(1)
        .filter(|generation| *generation <= MAX_PRIVATE_QUIC_SESSION_ATTEMPTS)
        .ok_or(Sys5I3PrivateQuicError::SessionAttemptExhausted)
}

/// The exact non-semantic transport check that keeps a complete session-one
/// frame tied to the consuming session-two control.  The retained preface is
/// an existing control binding, not an authority, credential, or candidate
/// fact; this predicate creates neither an ingress right nor a semantic
/// admission result.
fn retained_ingress_matches_reconnect_binding(
    retained_session_attempt_generation: u8,
    retained_control_binding: &Sys5I3LocalnetPeerPreface,
    reconnect_session_attempt_generation: u8,
    reconnect_control_binding: &Sys5I3LocalnetPeerPreface,
) -> bool {
    retained_session_attempt_generation == 1
        && reconnect_session_attempt_generation
            == retained_session_attempt_generation.saturating_add(1)
        && retained_control_binding == reconnect_control_binding
}

fn private_quic_blob_prefix(body: &[u8]) -> Result<[u8; 4], Sys5I3PrivateQuicError> {
    if body.len() > MAX_PRIVATE_QUIC_BLOB_BYTES {
        return Err(Sys5I3PrivateQuicError::FrameRejected);
    }
    Ok(u32::try_from(body.len())
        .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?
        .to_be_bytes())
}

fn private_quic_blob_body_len(prefix: [u8; 4]) -> Result<usize, Sys5I3PrivateQuicError> {
    let length = u32::from_be_bytes(prefix) as usize;
    if length > MAX_PRIVATE_QUIC_BLOB_BYTES {
        return Err(Sys5I3PrivateQuicError::FrameRejected);
    }
    Ok(length)
}

fn carrier_ref(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/private-quic/carrier/v1\0");
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
    format!("i3-private-quic-carrier-sha256-v1:{:x}", hasher.finalize())
}

/// Reference-only provenance read from the exact private carrier framing
/// which was just encoded or decoded by this adapter.  This is deliberately
/// not a reconstructed route: semantic admission still compares the decoded
/// candidate with the receiver's sealed projection/M9 state.
struct PrivateCarrierLineage {
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
}

fn carrier_lineage(bytes: &[u8]) -> Result<PrivateCarrierLineage, Sys5I3PrivateQuicError> {
    let body = private_message_body(bytes)?;
    let value = strict_json_value(body).map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
    let carrier = value
        .pointer("/message/carrier")
        .and_then(serde_json::Value::as_object)
        .ok_or(Sys5I3PrivateQuicError::CodecRejected)?;
    let source_path = carrier_text(carrier, "source_ref_path")?;
    let source_start_line = carrier_u32(carrier, "source_ref_start_line")?;
    let source_start_column = carrier_u32(carrier, "source_ref_start_column")?;
    let source_end_line = carrier_u32(carrier, "source_ref_end_line")?;
    let source_end_column = carrier_u32(carrier, "source_ref_end_column")?;
    let core_ref = carrier_text(carrier, "core_ref")?;
    let source_artifact_ref = carrier_text(carrier, "source_fragment_ref")?;
    let target_artifact_ref = carrier_text(carrier, "target_fragment_ref")?;
    let edge_ref = carrier_text(carrier, "edge_ref")?;
    Ok(PrivateCarrierLineage {
        source_ref: format!(
            "{source_path}:{source_start_line}:{source_start_column}-{source_end_line}:{source_end_column}"
        ),
        core_ref,
        source_artifact_ref,
        target_artifact_ref,
        edge_ref,
    })
}

fn private_message_body(bytes: &[u8]) -> Result<&[u8], Sys5I3PrivateQuicError> {
    if bytes.len() < 4 || bytes.len() > MAX_PRIVATE_QUIC_BLOB_BYTES {
        return Err(Sys5I3PrivateQuicError::CodecRejected);
    }
    let declared = u32::from_be_bytes(
        bytes[..4]
            .try_into()
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?,
    ) as usize;
    let body = &bytes[4..];
    if declared != body.len() {
        return Err(Sys5I3PrivateQuicError::CodecRejected);
    }
    Ok(body)
}

fn carrier_text(
    carrier: &serde_json::Map<String, serde_json::Value>,
    name: &str,
) -> Result<String, Sys5I3PrivateQuicError> {
    carrier
        .get(name)
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or(Sys5I3PrivateQuicError::CodecRejected)
}

fn carrier_u32(
    carrier: &serde_json::Map<String, serde_json::Value>,
    name: &str,
) -> Result<u32, Sys5I3PrivateQuicError> {
    carrier
        .get(name)
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(Sys5I3PrivateQuicError::CodecRejected)
}

fn verify_exact_peer_spki(
    connection: &Connection,
    expected_spki_ref: &str,
) -> Result<(), Sys5I3PrivateQuicError> {
    let rejected = || Sys5I3PrivateQuicError::peer_binding_rejected(expected_spki_ref);
    let identity: Box<dyn Any> = connection.peer_identity().ok_or_else(rejected)?;
    let certificates = identity
        .downcast::<Vec<rustls::pki_types::CertificateDer<'static>>>()
        .map_err(|_| rejected())?;
    let certificate = certificates.first().ok_or_else(rejected)?;
    let (_, parsed) =
        x509_parser::parse_x509_certificate(certificate.as_ref()).map_err(|_| rejected())?;
    let actual = spki_ref(parsed.public_key().raw);
    if actual == expected_spki_ref {
        return Ok(());
    }
    Err(Sys5I3PrivateQuicError::PeerBindingRejected(
        Sys5I3PrivateQuicPeerBindingEvidence {
            expected_peer_spki_ref: expected_spki_ref.to_string(),
            actual_peer_spki_ref: Some(actual),
            ca_validated_peer_leaf_ref: Some(peer_leaf_ref(certificate.as_ref())),
        },
    ))
}

fn spki_ref(spki: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/process-localnet/spki/v1\0");
    hasher.update(spki);
    format!("i3-process-localnet-spki-sha256-v1:{:x}", hasher.finalize())
}

fn peer_leaf_ref(certificate: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/i3/process-localnet/ca-validated-peer-leaf/v1\0");
    hasher.update((certificate.len() as u64).to_be_bytes());
    hasher.update(certificate);
    format!(
        "i3-process-localnet-peer-leaf-sha256-v1:{:x}",
        hasher.finalize()
    )
}

#[cfg(test)]
#[path = "sys5_i3_private_quic_tests.rs"]
mod sys5_i3_private_quic_tests;
