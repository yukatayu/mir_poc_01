//! Selected I3-1 QUIC static-adapter evidence.
//!
//! This is a Linux x86_64 localhost-only, private/provisional seam.  It moves
//! complete static-adapter frames over QUIC reliable bidirectional streams and
//! immediately returns to independently retained source-bound equality
//! admission.  It is not a process runtime, retry mechanism, public wire, or
//! transport-authority boundary.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use quinn::{
    Endpoint,
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
};
use rcgen::generate_simple_self_signed;
use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::{
    SourceBoundAdapterEdge, StaticAdapterAdmissionErrorKind, StaticAdapterFrameDecodeEvent,
    StaticAdapterFrameDecoder, UntrustedDecodedStaticAdapterCarrier, encode_static_adapter_frame,
    private_static_adapter_frame_reference, private_static_adapter_snapshot_reference,
    static_adapter_framing::{
        private_static_adapter_snapshot_reference_for_snapshot,
        tamper_private_static_adapter_target_locus,
    },
};

const PRIVATE_QUIC_REFERENCE_DOMAIN: &[u8] = b"mirrorea/i3-1/static-adapter/quic-evidence/v1\0";
const PRIVATE_QUIC_REFERENCE_PREFIX: &str = "mirrorea-i3-static-adapter-private-ref-sha256-v1:";
const PRIVATE_QUIC_ALPN: &[u8] = b"mirrorea-i3-static-adapter-quic-v1";
const MAX_COMPLETE_FRAME_BYTES: usize = crate::MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES + 4;
const MAX_PRIVATE_INGRESS_BYTES: usize = MAX_COMPLETE_FRAME_BYTES + 4 * 1024;
/// Closed I3-1 evidence profile cardinality, not a public or extensible wire
/// cardinality. A future profile must use a new private observer schema.
const PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT: usize = 12;
static NEXT_RUN_NONCE: AtomicU64 = AtomicU64::new(1);

/// The intentionally narrow platform claim for the selected evidence seam.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StaticAdapterQuicPlatformClaim {
    LinuxX86_64LocalhostOnly,
}

/// A pre-send static falsifier.  Neither case creates a runtime occurrence.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticAdapterQuicFalsifier {
    WrongRetainedReferenceHint,
    TamperedTargetLocus,
}

/// Exact branch-derived transport evidence kinds.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StaticAdapterQuicTransportEventKind {
    UdpSocketBound,
    QuicHandshakeCompleted,
    CertificateEvidence,
    SessionEvidence,
    ServerAcceptedBidirectionalStream,
}

/// A reference-only fact created at a real selected-transport branch.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StaticAdapterQuicTransportEvent {
    kind: StaticAdapterQuicTransportEventKind,
    evidence_ref: String,
}

impl StaticAdapterQuicTransportEvent {
    pub const fn kind(&self) -> StaticAdapterQuicTransportEventKind {
        self.kind
    }

    pub fn evidence_ref(&self) -> &str {
        &self.evidence_ref
    }
}

/// Strictly excluded QUIC capabilities for this static seam.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticAdapterQuicTransportFeatures {
    reliable_bidirectional_streams_only: bool,
    datagram_enabled: bool,
    zero_rtt_enabled: bool,
}

impl StaticAdapterQuicTransportFeatures {
    pub const fn reliable_bidirectional_streams_only(&self) -> bool {
        self.reliable_bidirectional_streams_only
    }

    pub const fn datagram_enabled(&self) -> bool {
        self.datagram_enabled
    }

    pub const fn zero_rtt_enabled(&self) -> bool {
        self.zero_rtt_enabled
    }
}

/// The source-bound result of one actual server ingress.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StaticAdapterQuicIngressEvent {
    sender_edge_ref: String,
    untrusted_reference_hint: String,
    canonical_source_frame_ref: String,
    transmitted_frame_ref: String,
    server_received_frame_ref: String,
    decoded_full_snapshot_ref: String,
    selected_receiver_retained_edge_ref: String,
    selected_receiver_full_snapshot_ref: String,
    admission_outcome: StaticAdapterQuicAdmissionOutcome,
}

impl StaticAdapterQuicIngressEvent {
    pub fn sender_edge_ref(&self) -> &str {
        &self.sender_edge_ref
    }
    pub fn untrusted_reference_hint(&self) -> &str {
        &self.untrusted_reference_hint
    }
    pub fn canonical_source_frame_ref(&self) -> &str {
        &self.canonical_source_frame_ref
    }
    pub fn transmitted_frame_ref(&self) -> &str {
        &self.transmitted_frame_ref
    }
    pub fn server_received_frame_ref(&self) -> &str {
        &self.server_received_frame_ref
    }
    pub fn decoded_full_snapshot_ref(&self) -> &str {
        &self.decoded_full_snapshot_ref
    }
    pub fn selected_receiver_retained_edge_ref(&self) -> &str {
        &self.selected_receiver_retained_edge_ref
    }
    pub fn selected_receiver_full_snapshot_ref(&self) -> &str {
        &self.selected_receiver_full_snapshot_ref
    }
    pub const fn admission_outcome(&self) -> StaticAdapterQuicAdmissionOutcome {
        self.admission_outcome
    }
}

/// Source-bound equality admission only; no semantic action occurs here.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StaticAdapterQuicAdmissionOutcome {
    Admitted,
    Rejected(StaticAdapterAdmissionErrorKind),
}

/// Typed selected-seam failure that deliberately omits endpoints, raw bytes,
/// certificates, keys, and source text.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticAdapterQuicRunErrorKind {
    UnsupportedPlatform,
    EmptyInventory,
    InvalidFiniteProfileInventory,
    DuplicateRetainedReference,
    ReceiverReferenceNotFound,
    InvalidCompleteFrame,
    TransportFailure,
    ConfigurationFailure,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticAdapterQuicRunError {
    kind: StaticAdapterQuicRunErrorKind,
}

impl StaticAdapterQuicRunError {
    const fn new(kind: StaticAdapterQuicRunErrorKind) -> Self {
        Self { kind }
    }
    pub const fn kind(&self) -> StaticAdapterQuicRunErrorKind {
        self.kind
    }
}

impl fmt::Display for StaticAdapterQuicRunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("private selected QUIC static-adapter seam failed")
    }
}

impl Error for StaticAdapterQuicRunError {}

/// Strict observer-schema validation failure.  It never repeats untrusted
/// content in its display text.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticAdapterQuicObserverErrorKind {
    MalformedEvidence,
    UnexpectedField,
    InvalidPrivateReference,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticAdapterQuicObserverError {
    kind: StaticAdapterQuicObserverErrorKind,
}

impl StaticAdapterQuicObserverError {
    const fn new(kind: StaticAdapterQuicObserverErrorKind) -> Self {
        Self { kind }
    }
    pub const fn kind(&self) -> StaticAdapterQuicObserverErrorKind {
        self.kind
    }
}

impl fmt::Display for StaticAdapterQuicObserverError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("private selected QUIC observer evidence is invalid")
    }
}

impl Error for StaticAdapterQuicObserverError {}

/// Completed private selected-QUIC evidence.  Admitted values are static
/// source-bound handoffs; no runtime request/action is constructed.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticAdapterQuicRun {
    platform_claim: StaticAdapterQuicPlatformClaim,
    transport_features: StaticAdapterQuicTransportFeatures,
    transport_events: Vec<StaticAdapterQuicTransportEvent>,
    ingress_events: Vec<StaticAdapterQuicIngressEvent>,
    admitted_handoffs: Vec<SourceBoundAdapterEdge>,
    source_sender_inventory_count: usize,
    receiver_retained_inventory_count: usize,
    observer_safe_evidence: String,
}

impl StaticAdapterQuicRun {
    pub const fn platform_claim(&self) -> StaticAdapterQuicPlatformClaim {
        self.platform_claim
    }
    pub const fn transport_features(&self) -> &StaticAdapterQuicTransportFeatures {
        &self.transport_features
    }
    pub fn transport_events(&self) -> &[StaticAdapterQuicTransportEvent] {
        &self.transport_events
    }
    pub fn ingress_events(&self) -> &[StaticAdapterQuicIngressEvent] {
        &self.ingress_events
    }
    pub fn admitted_handoffs(&self) -> &[SourceBoundAdapterEdge] {
        &self.admitted_handoffs
    }
    pub const fn source_sender_inventory_count(&self) -> usize {
        self.source_sender_inventory_count
    }
    pub const fn receiver_retained_inventory_count(&self) -> usize {
        self.receiver_retained_inventory_count
    }

    pub fn admitted_family_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for edge in &self.admitted_handoffs {
            *counts.entry(edge.edge_kind().to_string()).or_insert(0) += 1;
        }
        counts
    }

    pub fn observer_safe_evidence(&self) -> &str {
        &self.observer_safe_evidence
    }
}

/// Runs the normal selected-QUIC source-bound static inventory.
#[doc(hidden)]
pub fn run_static_adapter_quic_loopback(
    sender_inventory: &[SourceBoundAdapterEdge],
    receiver_inventory: &[SourceBoundAdapterEdge],
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    run_static_adapter_quic_loopback_inner(sender_inventory, receiver_inventory, None)
}

/// Runs one representative pre-admission falsifier over the actual selected
/// QUIC ingress.  The first source edge alone is enough to falsify equality.
#[doc(hidden)]
pub fn run_static_adapter_quic_loopback_with_falsifier(
    sender_inventory: &[SourceBoundAdapterEdge],
    receiver_inventory: &[SourceBoundAdapterEdge],
    falsifier: StaticAdapterQuicFalsifier,
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    run_static_adapter_quic_loopback_inner(sender_inventory, receiver_inventory, Some(falsifier))
}

/// Constructs one complete private selected-QUIC ingress envelope for focused
/// evidence tests.  It is not a public wire constructor or runtime API.
#[doc(hidden)]
pub fn encode_private_static_adapter_quic_ingress_for_test(
    untrusted_reference_hint: &str,
    frame: &[u8],
) -> Result<Vec<u8>, StaticAdapterQuicRunError> {
    encode_private_ingress(untrusted_reference_hint, frame)
}

/// Sends one caller-supplied private ingress through the same actual QUIC
/// receiver/decode/admission path used by the normal loopback run.  The
/// receiver accepts no falsifier state; its retained-edge selection is solely
/// a function of the received bytes.
#[doc(hidden)]
pub fn run_static_adapter_quic_loopback_from_private_ingress(
    sender_edge: &SourceBoundAdapterEdge,
    receiver_inventory: &[SourceBoundAdapterEdge],
    private_ingress: &[u8],
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        run_static_adapter_quic_loopback_from_private_ingress_linux(
            sender_edge,
            receiver_inventory,
            private_ingress,
        )
    }
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
        let _ = (sender_edge, receiver_inventory, private_ingress);
        Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::UnsupportedPlatform,
        ))
    }
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn run_static_adapter_quic_loopback_inner(
    _sender_inventory: &[SourceBoundAdapterEdge],
    _receiver_inventory: &[SourceBoundAdapterEdge],
    _falsifier: Option<StaticAdapterQuicFalsifier>,
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    Err(StaticAdapterQuicRunError::new(
        StaticAdapterQuicRunErrorKind::UnsupportedPlatform,
    ))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn run_static_adapter_quic_loopback_from_private_ingress_linux(
    sender_edge: &SourceBoundAdapterEdge,
    receiver_inventory: &[SourceBoundAdapterEdge],
    private_ingress: &[u8],
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    if receiver_inventory.is_empty() {
        return Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::EmptyInventory,
        ));
    }
    if receiver_inventory.len() != PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT {
        return Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::InvalidFiniteProfileInventory,
        ));
    }
    let receiver_by_ref = receiver_inventory_by_ref(receiver_inventory)?;
    let canonical_frame = encode_static_adapter_frame(sender_edge).map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    let canonical_source_frame_ref = private_static_adapter_frame_reference(&canonical_frame);
    let (_, transmitted_frame) = decode_private_ingress(private_ingress)?;
    let transmitted_frame_ref = private_static_adapter_frame_reference(&transmitted_frame);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure)
        })?;
    runtime.block_on(run_actual_quic_from_received_private_ingress(
        sender_edge,
        receiver_inventory,
        &receiver_by_ref,
        private_ingress,
        canonical_source_frame_ref,
        transmitted_frame_ref,
    ))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn run_static_adapter_quic_loopback_inner(
    sender_inventory: &[SourceBoundAdapterEdge],
    receiver_inventory: &[SourceBoundAdapterEdge],
    falsifier: Option<StaticAdapterQuicFalsifier>,
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    if sender_inventory.is_empty() || receiver_inventory.is_empty() {
        return Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::EmptyInventory,
        ));
    }
    if sender_inventory.len() != PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT
        || receiver_inventory.len() != PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT
    {
        return Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::InvalidFiniteProfileInventory,
        ));
    }
    let receiver_by_ref = receiver_inventory_by_ref(receiver_inventory)?;
    let sender = match falsifier {
        Some(_) => &sender_inventory[..1],
        None => sender_inventory,
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure)
        })?;
    runtime.block_on(run_actual_quic(
        sender,
        sender_inventory.len(),
        receiver_inventory,
        &receiver_by_ref,
        falsifier,
    ))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
async fn run_actual_quic(
    sender_inventory: &[SourceBoundAdapterEdge],
    source_sender_inventory_count: usize,
    receiver_inventory: &[SourceBoundAdapterEdge],
    receiver_by_ref: &BTreeMap<String, &SourceBoundAdapterEdge>,
    falsifier: Option<StaticAdapterQuicFalsifier>,
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let credentials = StaticAdapterQuicCredentials::generate()?;
    let run_nonce = NEXT_RUN_NONCE.fetch_add(1, Ordering::Relaxed);
    let mut transport_events = Vec::new();
    let server =
        Endpoint::server(server_config(&credentials)?, loopback_unspecified()).map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
    let server_address = server.local_addr().map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::UdpSocketBound,
        &credentials.certificate_der,
        run_nonce,
    ));
    let mut client = Endpoint::client(loopback_unspecified()).map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    client.set_default_client_config(client_config(&credentials)?);

    let connecting = client.connect(server_address, "localhost").map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    let incoming = server.accept().await.ok_or_else(|| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    let (server_connection, client_connection) =
        tokio::try_join!(incoming, connecting).map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::QuicHandshakeCompleted,
        &credentials.certificate_der,
        run_nonce,
    ));
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::CertificateEvidence,
        &credentials.certificate_der,
        run_nonce,
    ));
    let mut session_material = credentials.certificate_der.clone();
    session_material.extend_from_slice(&run_nonce.to_le_bytes());
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::SessionEvidence,
        &session_material,
        run_nonce,
    ));

    let mut ingress_events = Vec::new();
    let mut admitted_handoffs = Vec::new();
    for sender_edge in sender_inventory {
        let canonical_frame = encode_static_adapter_frame(sender_edge).map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
        })?;
        let canonical_source_frame_ref = private_static_adapter_frame_reference(&canonical_frame);
        let (hint, transmitted) = match falsifier {
            Some(StaticAdapterQuicFalsifier::WrongRetainedReferenceHint) => {
                let wrong = receiver_inventory
                    .iter()
                    .find(|edge| edge.edge_ref() != sender_edge.edge_ref())
                    .ok_or_else(|| {
                        StaticAdapterQuicRunError::new(
                            StaticAdapterQuicRunErrorKind::ReceiverReferenceNotFound,
                        )
                    })?;
                (wrong.edge_ref().to_string(), canonical_frame.clone())
            }
            Some(StaticAdapterQuicFalsifier::TamperedTargetLocus) => (
                sender_edge.edge_ref().to_string(),
                tamper_private_static_adapter_target_locus(&canonical_frame).map_err(|_| {
                    StaticAdapterQuicRunError::new(
                        StaticAdapterQuicRunErrorKind::InvalidCompleteFrame,
                    )
                })?,
            ),
            None => (sender_edge.edge_ref().to_string(), canonical_frame.clone()),
        };
        let transmitted_frame_ref = private_static_adapter_frame_reference(&transmitted);
        let (mut send, _receive) = client_connection.open_bi().await.map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
        // The selected stream carries an untrusted lookup hint plus one
        // complete static frame.  The hint is evidence only: receiver-owned
        // exact snapshot admission follows it and cannot be retargeted by it.
        let private_ingress = encode_private_ingress(&hint, &transmitted)?;
        send.write_all(&private_ingress).await.map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
        send.finish().map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;

        let (_reply, mut receive) = server_connection.accept_bi().await.map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
        transport_events.push(transport_event(
            StaticAdapterQuicTransportEventKind::ServerAcceptedBidirectionalStream,
            &private_ingress,
            run_nonce,
        ));
        let received = receive
            .read_to_end(MAX_PRIVATE_INGRESS_BYTES)
            .await
            .map_err(|_| {
                StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
            })?;
        let (event, admitted) = receive_private_static_adapter_ingress(
            sender_edge,
            canonical_source_frame_ref,
            transmitted_frame_ref,
            &received,
            receiver_by_ref,
        )?;
        if let Some(admitted) = admitted {
            admitted_handoffs.push(admitted);
        }
        ingress_events.push(event);
    }
    client.close(0_u32.into(), b"private static adapter complete");
    server.close(0_u32.into(), b"private static adapter complete");
    let mut run = StaticAdapterQuicRun {
        platform_claim: StaticAdapterQuicPlatformClaim::LinuxX86_64LocalhostOnly,
        transport_features: StaticAdapterQuicTransportFeatures {
            reliable_bidirectional_streams_only: true,
            datagram_enabled: false,
            zero_rtt_enabled: false,
        },
        transport_events,
        ingress_events,
        admitted_handoffs,
        source_sender_inventory_count,
        receiver_retained_inventory_count: receiver_inventory.len(),
        observer_safe_evidence: String::new(),
    };
    run.observer_safe_evidence =
        serde_json::to_string(&StaticAdapterQuicObserverEnvelope::from(&run))
            .expect("strict selected QUIC observer evidence serializes");
    Ok(run)
}

/// One actual selected-QUIC send/accept/decode/admission run for a supplied
/// private ingress.  It deliberately calls `receive_private_static_adapter_ingress`
/// above rather than rebuilding a test-only interpretation path.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
async fn run_actual_quic_from_received_private_ingress(
    sender_edge: &SourceBoundAdapterEdge,
    receiver_inventory: &[SourceBoundAdapterEdge],
    receiver_by_ref: &BTreeMap<String, &SourceBoundAdapterEdge>,
    private_ingress: &[u8],
    canonical_source_frame_ref: String,
    transmitted_frame_ref: String,
) -> Result<StaticAdapterQuicRun, StaticAdapterQuicRunError> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let credentials = StaticAdapterQuicCredentials::generate()?;
    let run_nonce = NEXT_RUN_NONCE.fetch_add(1, Ordering::Relaxed);
    let mut transport_events = Vec::new();
    let server =
        Endpoint::server(server_config(&credentials)?, loopback_unspecified()).map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
    let server_address = server.local_addr().map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::UdpSocketBound,
        &credentials.certificate_der,
        run_nonce,
    ));
    let mut client = Endpoint::client(loopback_unspecified()).map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    client.set_default_client_config(client_config(&credentials)?);
    let connecting = client.connect(server_address, "localhost").map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    let incoming = server.accept().await.ok_or_else(|| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    let (server_connection, client_connection) =
        tokio::try_join!(incoming, connecting).map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::QuicHandshakeCompleted,
        &credentials.certificate_der,
        run_nonce,
    ));
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::CertificateEvidence,
        &credentials.certificate_der,
        run_nonce,
    ));
    let mut session_material = credentials.certificate_der.clone();
    session_material.extend_from_slice(&run_nonce.to_le_bytes());
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::SessionEvidence,
        &session_material,
        run_nonce,
    ));
    let (mut send, _receive) = client_connection.open_bi().await.map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    send.write_all(private_ingress).await.map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    send.finish().map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    let (_reply, mut receive) = server_connection.accept_bi().await.map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
    })?;
    transport_events.push(transport_event(
        StaticAdapterQuicTransportEventKind::ServerAcceptedBidirectionalStream,
        private_ingress,
        run_nonce,
    ));
    let received = receive
        .read_to_end(MAX_PRIVATE_INGRESS_BYTES)
        .await
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::TransportFailure)
        })?;
    let (event, admitted) = receive_private_static_adapter_ingress(
        sender_edge,
        canonical_source_frame_ref,
        transmitted_frame_ref,
        &received,
        receiver_by_ref,
    )?;
    client.close(0_u32.into(), b"private static adapter complete");
    server.close(0_u32.into(), b"private static adapter complete");
    let mut run = StaticAdapterQuicRun {
        platform_claim: StaticAdapterQuicPlatformClaim::LinuxX86_64LocalhostOnly,
        transport_features: StaticAdapterQuicTransportFeatures {
            reliable_bidirectional_streams_only: true,
            datagram_enabled: false,
            zero_rtt_enabled: false,
        },
        transport_events,
        ingress_events: vec![event],
        admitted_handoffs: admitted.into_iter().collect(),
        // This doc-hidden route transmits exactly one supplied source edge;
        // receiver inventory remains independently retained and may be wider.
        source_sender_inventory_count: 1,
        receiver_retained_inventory_count: receiver_inventory.len(),
        observer_safe_evidence: String::new(),
    };
    run.observer_safe_evidence =
        serde_json::to_string(&StaticAdapterQuicObserverEnvelope::from(&run))
            .expect("strict selected QUIC observer evidence serializes");
    Ok(run)
}

/// The single selected-QUIC receiver path.  The lookup key comes only from
/// the bytes actually received on the private QUIC stream; source-edge
/// equality is then checked separately before a handoff is admitted.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn receive_private_static_adapter_ingress(
    sender_edge: &SourceBoundAdapterEdge,
    canonical_source_frame_ref: String,
    transmitted_frame_ref: String,
    received: &[u8],
    receiver_by_ref: &BTreeMap<String, &SourceBoundAdapterEdge>,
) -> Result<
    (
        StaticAdapterQuicIngressEvent,
        Option<SourceBoundAdapterEdge>,
    ),
    StaticAdapterQuicRunError,
> {
    let (received_hint, received_frame) = decode_private_ingress(received)?;
    let server_received_frame_ref = private_static_adapter_frame_reference(&received_frame);
    let decoded = decode_one_complete_frame(&received_frame)?;
    let decoded_full_snapshot_ref = private_static_adapter_snapshot_reference_for_decoded(&decoded);
    let selected = receiver_by_ref
        .get(&received_hint)
        .copied()
        .ok_or_else(|| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ReceiverReferenceNotFound)
        })?;
    let selected_receiver_retained_edge_ref = selected.edge_ref().to_string();
    let selected_receiver_full_snapshot_ref = private_static_adapter_snapshot_reference(selected);
    let exact_source_sender_binding = sender_edge.edge_ref() == selected.edge_ref()
        && canonical_source_frame_ref == transmitted_frame_ref
        && transmitted_frame_ref == server_received_frame_ref
        && decoded_full_snapshot_ref == selected_receiver_full_snapshot_ref;
    let (admission_outcome, admitted) =
        match selected.admit_untrusted_static_adapter_candidate(decoded) {
            Ok(admitted) if exact_source_sender_binding => {
                (StaticAdapterQuicAdmissionOutcome::Admitted, Some(admitted))
            }
            Ok(_) => (
                StaticAdapterQuicAdmissionOutcome::Rejected(
                    StaticAdapterAdmissionErrorKind::RetainedStaticContractMismatch,
                ),
                None,
            ),
            Err(error) => (
                StaticAdapterQuicAdmissionOutcome::Rejected(error.kind()),
                None,
            ),
        };
    Ok((
        StaticAdapterQuicIngressEvent {
            sender_edge_ref: sender_edge.edge_ref().to_string(),
            untrusted_reference_hint: received_hint,
            canonical_source_frame_ref,
            transmitted_frame_ref,
            server_received_frame_ref,
            decoded_full_snapshot_ref,
            selected_receiver_retained_edge_ref,
            selected_receiver_full_snapshot_ref,
            admission_outcome,
        },
        admitted,
    ))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn receiver_inventory_by_ref(
    receiver_inventory: &[SourceBoundAdapterEdge],
) -> Result<BTreeMap<String, &SourceBoundAdapterEdge>, StaticAdapterQuicRunError> {
    let mut result = BTreeMap::new();
    for edge in receiver_inventory {
        if result.insert(edge.edge_ref().to_string(), edge).is_some() {
            return Err(StaticAdapterQuicRunError::new(
                StaticAdapterQuicRunErrorKind::DuplicateRetainedReference,
            ));
        }
    }
    Ok(result)
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn decode_one_complete_frame(
    received: &[u8],
) -> Result<UntrustedDecodedStaticAdapterCarrier, StaticAdapterQuicRunError> {
    let mut decoder = StaticAdapterFrameDecoder::new();
    let events = decoder.push_events(received).map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    if decoder
        .finish_event()
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
        })?
        .is_some()
    {
        return Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::InvalidCompleteFrame,
        ));
    }
    match events.as_slice() {
        [StaticAdapterFrameDecodeEvent::Decoded(candidate)] => Ok((**candidate).clone()),
        _ => Err(StaticAdapterQuicRunError::new(
            StaticAdapterQuicRunErrorKind::InvalidCompleteFrame,
        )),
    }
}

fn encode_private_ingress(hint: &str, frame: &[u8]) -> Result<Vec<u8>, StaticAdapterQuicRunError> {
    let hint_length = u32::try_from(hint.len()).map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    let total = 4_usize
        .checked_add(hint.len())
        .and_then(|size| size.checked_add(frame.len()))
        .filter(|size| *size <= MAX_PRIVATE_INGRESS_BYTES)
        .ok_or_else(|| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
        })?;
    let mut output = Vec::with_capacity(total);
    output.extend_from_slice(&hint_length.to_be_bytes());
    output.extend_from_slice(hint.as_bytes());
    output.extend_from_slice(frame);
    Ok(output)
}

fn decode_private_ingress(bytes: &[u8]) -> Result<(String, Vec<u8>), StaticAdapterQuicRunError> {
    let prefix = bytes.get(..4).ok_or_else(|| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    let length = usize::try_from(u32::from_be_bytes(prefix.try_into().map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?))
    .map_err(|_| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    let hint_end = 4_usize.checked_add(length).ok_or_else(|| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    let hint = bytes.get(4..hint_end).ok_or_else(|| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
    })?;
    let frame = bytes
        .get(hint_end..)
        .filter(|frame| !frame.is_empty())
        .ok_or_else(|| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
        })?;
    let hint = std::str::from_utf8(hint)
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::InvalidCompleteFrame)
        })?
        .to_string();
    Ok((hint, frame.to_vec()))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
struct StaticAdapterQuicCredentials {
    certificate_der: Vec<u8>,
    private_key_der: Zeroizing<Vec<u8>>,
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
impl StaticAdapterQuicCredentials {
    fn generate() -> Result<Self, StaticAdapterQuicRunError> {
        let generated =
            generate_simple_self_signed(vec!["localhost".to_string()]).map_err(|_| {
                StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure)
            })?;
        Ok(Self {
            certificate_der: generated.cert.der().to_vec(),
            private_key_der: Zeroizing::new(generated.signing_key.serialize_der()),
        })
    }
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn server_config(
    credentials: &StaticAdapterQuicCredentials,
) -> Result<quinn::ServerConfig, StaticAdapterQuicRunError> {
    // This conversion makes the bounded handoff from the probe-owned
    // `Zeroizing<Vec<u8>>` to rustls' key owner.  `to_vec()` is an API-required
    // temporary and is not itself a zeroization claim.  This seam claims only
    // its probe-owned owner; it explicitly makes no claim about rcgen, rustls,
    // Quinn, allocator, or OS-internal copies.
    let private_key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
        credentials.private_key_der.to_vec(),
    ));
    let mut crypto = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(credentials.certificate_der.clone())],
            private_key,
        )
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure)
        })?;
    crypto.alpn_protocols = vec![PRIVATE_QUIC_ALPN.to_vec()];
    let mut configuration =
        quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(crypto).map_err(
            |_| StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure),
        )?));
    let transport = Arc::get_mut(&mut configuration.transport).ok_or_else(|| {
        StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure)
    })?;
    transport
        .max_concurrent_bidi_streams(1_u32.into())
        .max_concurrent_uni_streams(0_u32.into())
        .datagram_receive_buffer_size(None)
        .datagram_send_buffer_size(0);
    Ok(configuration)
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn client_config(
    credentials: &StaticAdapterQuicCredentials,
) -> Result<quinn::ClientConfig, StaticAdapterQuicRunError> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(credentials.certificate_der.clone()))
        .map_err(|_| {
            StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure)
        })?;
    let mut crypto = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    crypto.alpn_protocols = vec![PRIVATE_QUIC_ALPN.to_vec()];
    let mut configuration =
        quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(crypto).map_err(
            |_| StaticAdapterQuicRunError::new(StaticAdapterQuicRunErrorKind::ConfigurationFailure),
        )?));
    let mut transport = quinn::TransportConfig::default();
    transport
        .max_concurrent_bidi_streams(0_u32.into())
        .max_concurrent_uni_streams(0_u32.into())
        .datagram_receive_buffer_size(None)
        .datagram_send_buffer_size(0);
    configuration.transport_config(Arc::new(transport));
    Ok(configuration)
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn loopback_unspecified() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 0))
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn transport_event(
    kind: StaticAdapterQuicTransportEventKind,
    actual_artifact: &[u8],
    run_nonce: u64,
) -> StaticAdapterQuicTransportEvent {
    let mut hasher = Sha256::new();
    hasher.update(PRIVATE_QUIC_REFERENCE_DOMAIN);
    hasher.update(format!("{kind:?}").as_bytes());
    hasher.update(run_nonce.to_le_bytes());
    hasher.update(actual_artifact);
    StaticAdapterQuicTransportEvent {
        kind,
        evidence_ref: format!("{PRIVATE_QUIC_REFERENCE_PREFIX}{:x}", hasher.finalize()),
    }
}

fn private_static_adapter_snapshot_reference_for_decoded(
    decoded: &UntrustedDecodedStaticAdapterCarrier,
) -> String {
    private_static_adapter_snapshot_reference_for_snapshot(decoded.retained_static_contract())
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct StaticAdapterQuicObserverEnvelope {
    schema: &'static str,
    platform_claim: StaticAdapterQuicPlatformClaim,
    transport_events: Vec<StaticAdapterQuicTransportEvent>,
    ingress_events: Vec<StaticAdapterQuicIngressEvent>,
    summary: StaticAdapterQuicObserverSummary,
}

impl From<&StaticAdapterQuicRun> for StaticAdapterQuicObserverEnvelope {
    fn from(run: &StaticAdapterQuicRun) -> Self {
        Self {
            schema: "mirrorea-i3-static-adapter-quic-observer-v1",
            platform_claim: run.platform_claim,
            transport_events: run.transport_events.clone(),
            ingress_events: run.ingress_events.clone(),
            summary: StaticAdapterQuicObserverSummary {
                source_sender_inventory_count: run.source_sender_inventory_count,
                receiver_retained_inventory_count: run.receiver_retained_inventory_count,
                ingress_event_count: run.ingress_events.len(),
                admitted_handoff_count: run.admitted_handoffs.len(),
                admitted_family_counts: run.admitted_family_counts(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StaticAdapterQuicObserverSummary {
    source_sender_inventory_count: usize,
    receiver_retained_inventory_count: usize,
    ingress_event_count: usize,
    admitted_handoff_count: usize,
    admitted_family_counts: BTreeMap<String, usize>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StaticAdapterQuicObserverEnvelopeIn {
    schema: String,
    platform_claim: StaticAdapterQuicPlatformClaim,
    transport_events: Vec<StaticAdapterQuicTransportEvent>,
    ingress_events: Vec<StaticAdapterQuicIngressEvent>,
    summary: StaticAdapterQuicObserverSummary,
}

/// Validates the exact private selected-QUIC observer schema after the common
/// redaction scan.  Unknown nested fields, raw endpoint text, and raw
/// credential encodings all fail closed.
#[doc(hidden)]
pub fn validate_static_adapter_quic_observer_evidence(
    input: &str,
) -> Result<(), StaticAdapterQuicObserverError> {
    crate::validate_observer_safe_evidence(input).map_err(|_| {
        StaticAdapterQuicObserverError::new(StaticAdapterQuicObserverErrorKind::MalformedEvidence)
    })?;
    let envelope =
        serde_json::from_str::<StaticAdapterQuicObserverEnvelopeIn>(input).map_err(|_| {
            StaticAdapterQuicObserverError::new(StaticAdapterQuicObserverErrorKind::UnexpectedField)
        })?;
    if envelope.schema != "mirrorea-i3-static-adapter-quic-observer-v1" {
        return Err(StaticAdapterQuicObserverError::new(
            StaticAdapterQuicObserverErrorKind::MalformedEvidence,
        ));
    }
    if envelope.platform_claim != StaticAdapterQuicPlatformClaim::LinuxX86_64LocalhostOnly
        || envelope.ingress_events.is_empty()
    {
        return Err(StaticAdapterQuicObserverError::new(
            StaticAdapterQuicObserverErrorKind::MalformedEvidence,
        ));
    }

    let mut udp_socket_binds = 0_usize;
    let mut handshakes = 0_usize;
    let mut certificates = 0_usize;
    let mut sessions = 0_usize;
    let mut accepted_bidi_streams = 0_usize;
    for event in &envelope.transport_events {
        if !is_private_reference(&event.evidence_ref) {
            return Err(StaticAdapterQuicObserverError::new(
                StaticAdapterQuicObserverErrorKind::InvalidPrivateReference,
            ));
        }
        match event.kind {
            StaticAdapterQuicTransportEventKind::UdpSocketBound => udp_socket_binds += 1,
            StaticAdapterQuicTransportEventKind::QuicHandshakeCompleted => handshakes += 1,
            StaticAdapterQuicTransportEventKind::CertificateEvidence => certificates += 1,
            StaticAdapterQuicTransportEventKind::SessionEvidence => sessions += 1,
            StaticAdapterQuicTransportEventKind::ServerAcceptedBidirectionalStream => {
                accepted_bidi_streams += 1;
            }
        }
    }
    if udp_socket_binds != 1
        || handshakes != 1
        || certificates != 1
        || sessions != 1
        || accepted_bidi_streams != envelope.ingress_events.len()
        || envelope.transport_events.len() != 4 + envelope.ingress_events.len()
    {
        return Err(StaticAdapterQuicObserverError::new(
            StaticAdapterQuicObserverErrorKind::MalformedEvidence,
        ));
    }

    let mut admitted_handoff_count = 0_usize;
    let mut admitted_family_counts = BTreeMap::new();
    let mut selected_receiver_references = BTreeSet::new();
    for event in &envelope.ingress_events {
        let _sender_family =
            retained_edge_reference_family(&event.sender_edge_ref).ok_or_else(|| {
                StaticAdapterQuicObserverError::new(
                    StaticAdapterQuicObserverErrorKind::MalformedEvidence,
                )
            })?;
        let hint_family = retained_edge_reference_family(&event.untrusted_reference_hint)
            .ok_or_else(|| {
                StaticAdapterQuicObserverError::new(
                    StaticAdapterQuicObserverErrorKind::MalformedEvidence,
                )
            })?;
        let selected_family = retained_edge_reference_family(
            &event.selected_receiver_retained_edge_ref,
        )
        .ok_or_else(|| {
            StaticAdapterQuicObserverError::new(
                StaticAdapterQuicObserverErrorKind::MalformedEvidence,
            )
        })?;
        if event.untrusted_reference_hint != event.selected_receiver_retained_edge_ref
            || hint_family != selected_family
            || event.transmitted_frame_ref != event.server_received_frame_ref
            || (event.admission_outcome == StaticAdapterQuicAdmissionOutcome::Admitted
                && (event.sender_edge_ref != event.selected_receiver_retained_edge_ref
                    || event.canonical_source_frame_ref != event.transmitted_frame_ref
                    || event.decoded_full_snapshot_ref
                        != event.selected_receiver_full_snapshot_ref))
        {
            return Err(StaticAdapterQuicObserverError::new(
                StaticAdapterQuicObserverErrorKind::MalformedEvidence,
            ));
        }
        selected_receiver_references.insert(event.selected_receiver_retained_edge_ref.as_str());
        for reference in [
            event.canonical_source_frame_ref.as_str(),
            event.transmitted_frame_ref.as_str(),
            event.server_received_frame_ref.as_str(),
            event.decoded_full_snapshot_ref.as_str(),
            event.selected_receiver_full_snapshot_ref.as_str(),
        ] {
            if !is_private_reference(reference) {
                return Err(StaticAdapterQuicObserverError::new(
                    StaticAdapterQuicObserverErrorKind::InvalidPrivateReference,
                ));
            }
        }
        if event.admission_outcome == StaticAdapterQuicAdmissionOutcome::Admitted {
            admitted_handoff_count += 1;
            *admitted_family_counts
                .entry(selected_family.to_string())
                .or_insert(0) += 1;
        }
    }
    let summary = &envelope.summary;
    let ingress_event_count = envelope.ingress_events.len();
    // Observer schema v1 is only the closed finite profile: the complete
    // twelve-edge inventory, its one-edge falsifier, or the doc-hidden
    // one-sender ingress seam.  These are not public/general cardinalities.
    let supported_finite_census = matches!(
        (summary.source_sender_inventory_count, ingress_event_count),
        (1, 1)
            | (PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT, 1)
            | (
                PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT,
                PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT
            )
    );
    if !supported_finite_census
        || summary.ingress_event_count != ingress_event_count
        || summary.receiver_retained_inventory_count != PRIVATE_FINITE_RECEIVER_INVENTORY_COUNT
        || summary.receiver_retained_inventory_count < selected_receiver_references.len()
        || summary.admitted_handoff_count != admitted_handoff_count
        || summary.admitted_family_counts != admitted_family_counts
    {
        return Err(StaticAdapterQuicObserverError::new(
            StaticAdapterQuicObserverErrorKind::MalformedEvidence,
        ));
    }
    Ok(())
}

/// Checks only the exact, observer-safe shape emitted by the retained static
/// adapter's private edge-reference generator.  This is a grammar check, not
/// a claim that any observer-supplied reference has authority.
fn retained_edge_reference_family(reference: &str) -> Option<&'static str> {
    let mut fields = reference.split(':');
    if fields.next()? != "edge" {
        return None;
    }
    let operation = fields.next()?;
    let debug_kind = fields.next()?;
    let source_locus = fields.next()?;
    let target_locus = fields.next()?;
    let dependency = fields.next()?;
    let valid_dependency = dependency == "dependency-None"
        || dependency
            .strip_prefix("dependency-Some(")
            .and_then(|ordinal| ordinal.strip_suffix(')'))
            .is_some_and(|ordinal| {
                !ordinal.is_empty() && ordinal.bytes().all(|byte| byte.is_ascii_digit())
            });
    if fields.next().is_some()
        || !is_private_retained_edge_component(operation)
        || !is_private_retained_edge_component(source_locus)
        || !is_private_retained_edge_component(target_locus)
        || !valid_dependency
    {
        return None;
    }
    match debug_kind {
        "OwnerRequest" => Some("owner-request"),
        "OwnerReplyReceipt" => Some("owner-reply-receipt"),
        "RelationProjectionPublication" => Some("relation-projection-publication"),
        "DesignatedInputRequest" => Some("designated-input-request"),
        "DesignatedInputReceipt" => Some("designated-input-receipt"),
        "DesignatedResultDelivery" => Some("designated-result-delivery"),
        _ => None,
    }
}

fn is_private_retained_edge_component(component: &str) -> bool {
    !component.is_empty()
        && component.len() <= 256
        && component
            .bytes()
            .all(|byte| byte.is_ascii_graphic() && byte != b':')
}

fn is_private_reference(reference: &str) -> bool {
    reference
        .strip_prefix(PRIVATE_QUIC_REFERENCE_PREFIX)
        .is_some_and(|digest| {
            digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
}
