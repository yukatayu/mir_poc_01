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
    Sys5I3LocalnetControlErrorKind, Sys5I3LocalnetPeerPreface, Sys5I3PrivateProcessCodec,
    Sys5I3ProcessMessage, Sys5I3ProcessRuntime, Sys5I3ProcessRuntimeError,
    Sys5I3TrustedLocalnetControl, strict_json_value,
};

const MAX_PRIVATE_QUIC_BLOB_BYTES: usize = 64 * 1024;

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
    SemanticRejected(Sys5I3ProcessRuntimeError),
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
            Self::FrameRejected | Self::CodecRejected | Self::SemanticRejected(_) => None,
        }
    }
}

/// Observer-safe evidence derived from an actual one-stream send or receive.
/// It contains references only; the adapter never exports carrier bytes.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sys5I3PrivateQuicDeliveryEvidence {
    carrier_ref: String,
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
    next_network_occurrence: u64,
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
            next_network_occurrence: 0,
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
            next_network_occurrence: 0,
        })
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
        self.write_blob(&bytes).await?;
        let network_occurrence_ref =
            self.next_network_occurrence_ref("send", &carrier_ref, &semantic_request_identity_ref);
        Ok(Sys5I3PrivateQuicDeliveryEvidence {
            carrier_ref,
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
        let candidate = Sys5I3PrivateProcessCodec::private_provisional_v1()
            .decode_untrusted_message(&bytes)
            .map_err(|_| Sys5I3PrivateQuicError::CodecRejected)?;
        let lineage = carrier_lineage(&bytes)?;
        let manifest = candidate.observer_safe_manifest();
        let semantic_request_identity_ref = manifest.semantic_request_identity_ref().to_string();
        let linked_request_identity_ref =
            manifest.linked_request_identity_ref().map(str::to_string);
        let carrier_ref = carrier_ref(&bytes);
        let admitted = runtime
            .admit_decoded_process_message(candidate)
            .map_err(Sys5I3PrivateQuicError::SemanticRejected)?;
        let network_occurrence_ref = self.next_network_occurrence_ref(
            "receive",
            &carrier_ref,
            &semantic_request_identity_ref,
        );
        Ok((
            admitted,
            Sys5I3PrivateQuicDeliveryEvidence {
                carrier_ref,
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

    fn next_network_occurrence_ref(
        &mut self,
        direction: &str,
        carrier_ref: &str,
        request_identity_ref: &str,
    ) -> String {
        self.next_network_occurrence = self.next_network_occurrence.saturating_add(1);
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/i3/private-quic/network-occurrence/v1\0");
        for component in [
            self.control.run_ref(),
            direction,
            carrier_ref,
            request_identity_ref,
        ] {
            hasher.update((component.len() as u64).to_be_bytes());
            hasher.update(component.as_bytes());
        }
        hasher.update(self.next_network_occurrence.to_be_bytes());
        format!(
            "i3-private-quic-network-occurrence-sha256-v1:{:x}",
            hasher.finalize()
        )
    }
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
