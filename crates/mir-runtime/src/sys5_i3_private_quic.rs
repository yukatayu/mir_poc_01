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

use quinn::{Connection, RecvStream, SendStream};
use sha2::{Digest, Sha256};

use super::sys5_i3_process_runtime::{
    Sys5I3LocalnetControlErrorKind, Sys5I3LocalnetPeerPreface,
    Sys5I3OriginalOwnerRequestAttemptKind, Sys5I3OriginalOwnerRequestPending,
    Sys5I3PrivateProcessCodec, Sys5I3ProcessMessage, Sys5I3ProcessRuntime,
    Sys5I3ProcessRuntimeError, Sys5I3TrustedLocalnetControl, Sys5I3UntrustedProcessMessage,
    strict_json_value,
};

const MAX_PRIVATE_QUIC_BLOB_BYTES: usize = 64 * 1024;
const MAX_PRIVATE_QUIC_SESSION_ATTEMPTS: u8 = 2;

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
}

/// One inspected connection and its adapter-owned bidi stream.  Neither the
/// connection nor streams are exposed to semantic callers, preventing an
/// external decoded candidate from bypassing the delivery-origin gate.
#[doc(hidden)]
pub struct Sys5I3PrivateQuicSession {
    connection: Connection,
    send: SendStream,
    receive: RecvStream,
    control: Sys5I3TrustedLocalnetControl,
    peer_spki_verified: bool,
    peer_preface_verified: bool,
    session_attempt_generation: u8,
    next_network_occurrence: u64,
    pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit,
}

/// A consuming private reconnect capability.  It transfers the sole retained
/// peer control and checked occurrence counter into at most one second
/// adapter session; it is neither cloneable nor an authority credential.
#[doc(hidden)]
pub struct Sys5I3PrivateQuicReconnect {
    control: Sys5I3TrustedLocalnetControl,
    prior_session_attempt_generation: u8,
    next_network_occurrence: u64,
    pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit,
}

/// A reply result that either consumes the original pending handle exactly
/// once after local receipt admission, or returns it intact after a delivery
/// failure/rejection.  A retry rejection therefore cannot complete the
/// original semantic operation.
#[doc(hidden)]
pub enum Sys5I3PrivateQuicOriginalOwnerReplyOutcome {
    Consumed {
        receipt: Box<Sys5I3ProcessMessage>,
        delivery: Sys5I3PrivateQuicDeliveryEvidence,
    },
    Pending {
        pending: Sys5I3OriginalOwnerRequestPending,
        error: Sys5I3PrivateQuicError,
    },
}

impl Sys5I3PrivateQuicSession {
    /// Owns and inspects a client connection, then opens the profile's only
    /// bidi stream.  The constructor never accepts a stream supplied by an
    /// external caller.
    pub async fn connect(
        connection: Connection,
        control: Sys5I3TrustedLocalnetControl,
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
            control,
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
        })
    }

    /// Owns and inspects a server connection, then accepts the profile's one
    /// bidi stream.  No external process code receives an ingress stream.
    pub async fn accept(
        connection: Connection,
        control: Sys5I3TrustedLocalnetControl,
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
            control,
            peer_spki_verified: true,
            peer_preface_verified: false,
            session_attempt_generation: 1,
            next_network_occurrence: 0,
            pending_ingress_permit: Sys5I3PrivateQuicPendingIngressPermit::Unacquired,
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

    pub async fn send_local_preface(&mut self) -> Result<(), Sys5I3PrivateQuicError> {
        let body = serde_json::to_vec(&self.control.localnet_preface())
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.write_blob(&body).await
    }

    /// Private negative-only delivery test.  It remains inside the adapter so
    /// no caller can mint an ingress token or submit a decoded candidate.
    pub async fn send_unbound_preface_for_private_falsifier(
        &mut self,
    ) -> Result<(), Sys5I3PrivateQuicError> {
        let mut value = serde_json::to_value(self.control.localnet_preface())
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
        let preface: Sys5I3LocalnetPeerPreface =
            serde_json::from_value(value).map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.control
            .validate_peer_preface(&preface)
            .map_err(|error| match error.kind() {
                Sys5I3LocalnetControlErrorKind::PeerBindingRejected => {
                    Sys5I3PrivateQuicError::peer_binding_rejected(
                        self.control.expected_peer_spki_ref(),
                    )
                }
                _ => Sys5I3PrivateQuicError::FrameRejected,
            })?;
        self.peer_preface_verified = true;
        Ok(())
    }

    /// Encodes and sends a generated request/reply over this adapter-owned
    /// stream.  The returned references are hashes of the actual private
    /// carrier bytes and run-salted network occurrence, not inferred counts.
    pub async fn send_generated_message(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
        let semantic_request_identity_ref = message.semantic_request_identity_ref().to_string();
        let linked_request_identity_ref = message.linked_request_identity_ref().map(str::to_string);
        let bytes = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .encode_outbound_message(message)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let lineage = carrier_lineage(&bytes)?;
        let carrier_ref = carrier_ref(&bytes);
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&bytes);
        let network_occurrence_ref = self.reserve_network_occurrence_ref("send", &carrier_ref)?;
        self.write_blob(&bytes).await?;
        Ok(Sys5I3PrivateQuicDeliveryEvidence {
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
        })
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
        self.send_original_owner_request_attempt(
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
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        let bytes = self.read_blob().await?;
        let candidate_commitment_ref = self.candidate_commitment_ref_for_frame(&bytes);
        let network_occurrence_ref =
            self.reserve_network_occurrence_ref("receive", &candidate_commitment_ref)?;
        let candidate = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_message(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        // These private parses must succeed before semantic mutation.  Their
        // values are retained only on success; a rejection below reports the
        // run-scoped commitment instead of any candidate provenance.
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
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        if self.session_attempt_generation != 1 {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        // Reserve before the first await so cancellation, partial reads, and
        // codec failure cannot reopen this session-one acquisition.
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
        Ok(Sys5I3PrivateQuicPendingIngress {
            bytes,
            candidate,
            session_attempt_generation: self.session_attempt_generation,
            // This is the existing trusted transport control rendered as its
            // exact preface binding.  It is retained opaquely and compared
            // only by the adapter on session two; it conveys no authority.
            control_binding: self.control.localnet_preface(),
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
        if !self.peer_spki_verified || !self.peer_preface_verified {
            return Err(Sys5I3PrivateQuicError::peer_binding_rejected(
                self.control.expected_peer_spki_ref(),
            ));
        }
        let reconnect_control_binding = self.control.localnet_preface();
        if !retained_ingress_matches_reconnect_binding(
            pending.session_attempt_generation,
            &pending.control_binding,
            self.session_attempt_generation,
            &reconnect_control_binding,
        ) {
            return Err(Sys5I3PrivateQuicError::LocalAttemptRejected);
        }
        // The exact inherited permit is consumed before semantic handoff.
        // A runtime rejection therefore cannot reopen or replace this ingress.
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
        // Keep all decoded lineage local until the runtime has bound this
        // candidate to its current sealed image.  Any rejection below returns
        // neither the manifest nor carrier provenance.
        let admitted = runtime
            .admit_decoded_process_message(candidate)
            .map_err(|error| Sys5I3PrivateQuicError::SemanticRejected {
                error,
                rejected_attempt: Sys5I3PrivateQuicRejectedAttemptEvidence {
                    rejected_candidate_commitment_ref: candidate_commitment_ref.clone(),
                    network_occurrence_ref: network_occurrence_ref.clone(),
                },
            })?;
        // The retained bytes are re-decoded only after successful semantic
        // admission so the pending value never exports candidate lineage.
        // This is not a second ingress or admission path.
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
            },
        ))
    }

    /// Receive one exact owner reply for an opaque original pending handle.
    /// A local receipt consumes the handle by value; every frame, codec, or
    /// semantic rejection returns it intact so a later original reply can
    /// still complete exactly once through the runtime's pending map.
    pub async fn receive_and_admit_original_owner_reply(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: Sys5I3OriginalOwnerRequestPending,
    ) -> Sys5I3PrivateQuicOriginalOwnerReplyOutcome {
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
            Ok((receipt, delivery)) => Sys5I3PrivateQuicOriginalOwnerReplyOutcome::Consumed {
                receipt: Box::new(receipt),
                delivery,
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

    /// Server-side lifecycle coordination only.  This is called after the
    /// reply stream is finished and after the child emitted its observer-safe
    /// completion report, so it cannot create a semantic acknowledgement.
    pub async fn wait_for_peer_close(&self) {
        let _ = self.connection.closed().await;
    }

    async fn write_blob(&mut self, body: &[u8]) -> Result<(), Sys5I3PrivateQuicError> {
        if body.len() > MAX_PRIVATE_QUIC_BLOB_BYTES {
            return Err(Sys5I3PrivateQuicError::FrameRejected);
        }
        self.send
            .write_all(
                &(u32::try_from(body.len()).map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?)
                    .to_be_bytes(),
            )
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        self.send
            .write_all(body)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)
    }

    async fn read_blob(&mut self) -> Result<Vec<u8>, Sys5I3PrivateQuicError> {
        let mut prefix = [0_u8; 4];
        self.receive
            .read_exact(&mut prefix)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        let length = u32::from_be_bytes(prefix) as usize;
        if length > MAX_PRIVATE_QUIC_BLOB_BYTES {
            return Err(Sys5I3PrivateQuicError::FrameRejected);
        }
        let mut body = vec![0; length];
        self.receive
            .read_exact(&mut body)
            .await
            .map_err(|_| Sys5I3PrivateQuicError::FrameRejected)?;
        Ok(body)
    }

    async fn send_original_owner_request_attempt(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
        kind: Sys5I3OriginalOwnerRequestAttemptKind,
        required_session_attempt_generation: u8,
    ) -> Result<Sys5I3PrivateQuicDeliveryEvidence, Sys5I3PrivateQuicError> {
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
        self.write_blob(bytes).await?;
        Ok(Sys5I3PrivateQuicDeliveryEvidence {
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
        })
    }

    async fn receive_and_admit_original_owner_reply_inner(
        &mut self,
        runtime: &mut Sys5I3ProcessRuntime,
        pending: &Sys5I3OriginalOwnerRequestPending,
    ) -> Result<(Sys5I3ProcessMessage, Sys5I3PrivateQuicDeliveryEvidence), Sys5I3PrivateQuicError>
    {
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
        let receipt = admitted
            .filter(Sys5I3ProcessMessage::is_observer_safe_typed_result_or_receipt)
            .ok_or(Sys5I3PrivateQuicError::FrameRejected)?;
        let carrier_ref = carrier_ref(&bytes);
        Ok((
            receipt,
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
