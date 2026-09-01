//! Candidate-neutral process supervision and evidence normalization.
//!
//! This module is deliberately the only place that can turn source-bound
//! carrier facts into evidence rows. Candidate modules may transport private
//! bytes and report a transport disposition, but they cannot mint source/Core
//! references, semantic authority, retry policy, handler counts, or observer
//! claims.

use std::{
    error::Error,
    fmt,
    io::{self, BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    process::{Child, ChildStderr, ChildStdout, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use rcgen::generate_simple_self_signed;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    ClientChildProbeReplyReceipt, FrameDecodeErrorKind, FrameDecodeEvent, FrameDecoder,
    ReceiverChildCanaryEvent, ReceiverChildCanaryEventKind, SemanticAdmissionErrorKind,
    SemanticCarrier, SourceBoundEdge,
    candidates::{
        CandidateCaseInput, CandidateExecutionErrorKind, CandidateTransportDisposition,
        CandidateTransportObservation, ChildReceiveKind,
    },
    receiver_canary::{ReceiverChildCanary, ReceiverChildCanaryReport},
};

const TRANSPORT_CAPTURE_REF_DOMAIN: &[u8] = b"mirrorea/i3-0/transport-capture/v1\0";

/// The fixed, ordered I3-0 comparison inventory. It is intentionally private
/// probe evidence, not a public transport API or conformance wire.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CandidateCase {
    /// A successful transport connection must not itself admit a semantic request.
    ConnectWithoutSemanticAdmission,
    /// One valid frame arrives in deterministic fragments and is admitted once.
    DeterministicFragmentedRoundTrip,
    /// A partial private frame fails closed before semantic admission.
    TruncatedFrame,
    /// An over-limit frame prefix fails closed before allocation/admission.
    OversizedFrame,
    /// Loss before remote admission has no handler effect.
    DisconnectBeforeAdmission,
    /// Loss after one admission but before a result is an explicit ambiguity.
    DisconnectAfterAdmissionBeforeResult,
    /// An explicit reconnect occurrence cannot silently re-run the handler.
    DuplicateAcrossReconnect,
    /// Valid transport bytes whose retained Core reference was tampered fail admission.
    TamperedSemanticAdmissionReference,
    /// Reference-only structured evidence remains safe for an observer.
    ObserverSafeEvidence,
}

impl CandidateCase {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::ConnectWithoutSemanticAdmission => "connect-without-semantic-admission",
            Self::DeterministicFragmentedRoundTrip => "deterministic-fragmented-round-trip",
            Self::TruncatedFrame => "truncated-frame",
            Self::OversizedFrame => "oversized-frame",
            Self::DisconnectBeforeAdmission => "disconnect-before-admission",
            Self::DisconnectAfterAdmissionBeforeResult => {
                "disconnect-after-admission-before-result"
            }
            Self::DuplicateAcrossReconnect => "duplicate-across-reconnect",
            Self::TamperedSemanticAdmissionReference => "tampered-semantic-admission-reference",
            Self::ObserverSafeEvidence => "observer-safe-evidence",
        }
    }
}

pub(crate) const CANONICAL_CASES: [CandidateCase; 9] = [
    CandidateCase::ConnectWithoutSemanticAdmission,
    CandidateCase::DeterministicFragmentedRoundTrip,
    CandidateCase::TruncatedFrame,
    CandidateCase::OversizedFrame,
    CandidateCase::DisconnectBeforeAdmission,
    CandidateCase::DisconnectAfterAdmissionBeforeResult,
    CandidateCase::DuplicateAcrossReconnect,
    CandidateCase::TamperedSemanticAdmissionReference,
    CandidateCase::ObserverSafeEvidence,
];

/// The two and only two transport candidates for I3-0.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TransportCandidate {
    /// TLS over a framed reliable TCP byte stream.
    TlsOverTcpFramedReliableStream,
    /// QUIC using reliable bidirectional streams only.
    QuicReliableBidirectionalStream,
}

/// Transport feature declarations used to prevent the QUIC candidate from
/// silently expanding into datagrams or 0-RTT.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransportFeatures {
    reliable_bidirectional_streams_only: bool,
    datagram_enabled: bool,
    zero_rtt_enabled: bool,
}

impl TransportFeatures {
    const fn for_candidate(candidate: TransportCandidate) -> Self {
        match candidate {
            TransportCandidate::TlsOverTcpFramedReliableStream => Self {
                reliable_bidirectional_streams_only: false,
                datagram_enabled: false,
                zero_rtt_enabled: false,
            },
            TransportCandidate::QuicReliableBidirectionalStream => Self {
                reliable_bidirectional_streams_only: true,
                datagram_enabled: false,
                zero_rtt_enabled: false,
            },
        }
    }

    /// Whether this candidate is restricted to QUIC reliable bidirectional streams.
    pub const fn reliable_bidirectional_streams_only(&self) -> bool {
        self.reliable_bidirectional_streams_only
    }

    /// Whether QUIC datagrams are enabled. They must stay disabled for I3-0.
    pub const fn datagram_enabled(&self) -> bool {
        self.datagram_enabled
    }

    /// Whether 0-RTT is enabled. It must stay disabled for I3-0.
    pub const fn zero_rtt_enabled(&self) -> bool {
        self.zero_rtt_enabled
    }
}

/// The only credential delivery classification accepted by this probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialDelivery {
    /// Ephemeral credential bytes remain in memory and travel to children only
    /// through their inherited standard-input pipe.
    InMemoryOrPrivatePipe,
}

/// Provenance for the byte count and digest carried in one evidence row.
/// Candidate coordinators never get to assign this: the only admitted source is
/// the receiving child-process event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum TransportCaptureOrigin {
    /// Raw private bytes were reported by the child that received them through
    /// the candidate transport. The row exposes only a digest and total size.
    ChildProcessReceive,
}

/// Source of a semantic-falsifier frame. I3-0 permits only the common harness
/// to construct it from the source-bound private carrier; candidate modules
/// may transport the already prepared bytes but cannot alter their semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum SemanticFalsifierOrigin {
    /// The fixed common harness produced the one deliberate private frame.
    CommonHarness,
    /// No semantic falsifier applies to this case.
    NotApplicable,
}

/// Aggregate lifecycle evidence created only by the common child supervisor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessLifecycle {
    server_pids: Vec<u32>,
    client_pids: Vec<u32>,
    deadline_enforced: bool,
    kill_wait_cleanup_enforced: bool,
    orphan_cleanup_complete: bool,
    credential_delivery: CredentialDelivery,
    secret_exposed_via_cli_environment_file_or_log: bool,
    permissive_certificate_verifier_used: bool,
    os_trust_store_used: bool,
    cleanup_policy_declared: bool,
    kill_attempted: bool,
    wait_completed: bool,
}

impl ProcessLifecycle {
    fn new() -> Self {
        Self {
            server_pids: Vec::new(),
            client_pids: Vec::new(),
            deadline_enforced: false,
            kill_wait_cleanup_enforced: false,
            orphan_cleanup_complete: false,
            credential_delivery: CredentialDelivery::InMemoryOrPrivatePipe,
            secret_exposed_via_cli_environment_file_or_log: false,
            permissive_certificate_verifier_used: false,
            os_trust_store_used: false,
            cleanup_policy_declared: false,
            kill_attempted: false,
            wait_completed: false,
        }
    }

    fn merge(&mut self, other: Self) {
        if self.server_pids.is_empty() && self.client_pids.is_empty() {
            *self = other;
            return;
        }
        self.server_pids.extend(other.server_pids);
        self.client_pids.extend(other.client_pids);
        self.deadline_enforced &= other.deadline_enforced;
        self.kill_wait_cleanup_enforced &= other.kill_wait_cleanup_enforced;
        self.orphan_cleanup_complete &= other.orphan_cleanup_complete;
        self.secret_exposed_via_cli_environment_file_or_log |=
            other.secret_exposed_via_cli_environment_file_or_log;
        self.permissive_certificate_verifier_used |= other.permissive_certificate_verifier_used;
        self.os_trust_store_used |= other.os_trust_store_used;
        self.cleanup_policy_declared &= other.cleanup_policy_declared;
        self.kill_attempted |= other.kill_attempted;
        self.wait_completed &= other.wait_completed;
    }

    /// Confirms that each participating server/client pair were separate child
    /// OS processes. The coordinator process never contributes a PID.
    pub fn server_and_client_are_distinct_children(&self) -> bool {
        !self.server_pids.is_empty()
            && !self.client_pids.is_empty()
            && self
                .server_pids
                .iter()
                .zip(&self.client_pids)
                .all(|(server, client)| server != client)
    }

    /// Whether every child operation received the common deadline guard.
    pub const fn deadline_enforced(&self) -> bool {
        self.deadline_enforced
    }

    /// Whether timeout cleanup is kill followed by wait, not detached abandon.
    pub const fn kill_wait_cleanup_enforced(&self) -> bool {
        self.kill_wait_cleanup_enforced
    }

    /// Whether this completed lifecycle actually applied the common cleanup
    /// policy. Normal runs may reap already-exited children without a kill;
    /// forced falsifiers separately expose whether a kill was attempted.
    pub const fn cleanup_policy_declared(&self) -> bool {
        self.cleanup_policy_declared
    }

    /// Whether every tracked child had been reaped at case completion.
    pub const fn orphan_cleanup_complete(&self) -> bool {
        self.orphan_cleanup_complete
    }

    /// How ephemeral server credentials reached children.
    pub const fn credential_delivery(&self) -> CredentialDelivery {
        self.credential_delivery
    }

    /// Whether the common harness put generated secrets in command-line,
    /// environment, file, or observer log channels.
    pub const fn secret_exposed_via_cli_environment_file_or_log(&self) -> bool {
        self.secret_exposed_via_cli_environment_file_or_log
    }

    /// Whether a candidate used an accept-any certificate verifier.
    pub const fn permissive_certificate_verifier_used(&self) -> bool {
        self.permissive_certificate_verifier_used
    }

    /// Whether a candidate delegated test trust to an operating-system store.
    pub const fn os_trust_store_used(&self) -> bool {
        self.os_trust_store_used
    }
}

/// Request for the complete, ordered candidate inventory.
#[derive(Clone, Debug)]
pub struct CandidateRunRequest {
    candidate: TransportCandidate,
    carrier: SemanticCarrier,
    target_admission_edge: Option<SourceBoundEdge>,
    cases: Vec<CandidateCase>,
    deadline: Duration,
}

impl CandidateRunRequest {
    /// Begins a comparison request with a locally bound semantic carrier.
    pub fn new(candidate: TransportCandidate, carrier: SemanticCarrier) -> Self {
        Self {
            candidate,
            carrier,
            target_admission_edge: None,
            cases: CANONICAL_CASES.to_vec(),
            deadline: Duration::from_secs(15),
        }
    }

    /// Supplies the independently retained target edge used for every decoded
    /// carrier admission. No candidate receives authority to replace it.
    pub fn with_target_admission_edge(mut self, edge: SourceBoundEdge) -> Self {
        self.target_admission_edge = Some(edge);
        self
    }

    /// Replaces the requested inventory; the runner fails closed unless it is
    /// exactly the fixed nine-case order.
    pub fn with_cases(mut self, cases: impl IntoIterator<Item = CandidateCase>) -> Self {
        self.cases = cases.into_iter().collect();
        self
    }

    /// Sets one nonzero common deadline for every child operation in this run.
    pub fn with_deadline(mut self, deadline: Duration) -> Self {
        self.deadline = deadline;
        self
    }
}

/// A common evidence row. All provenance and semantic fields are constructed
/// from the request's carrier/retained target edge, never candidate output.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvidenceRow {
    candidate: TransportCandidate,
    case: CandidateCase,
    mechanism: String,
    program_ref: String,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    request_ref: String,
    retained_contract_fingerprint: String,
    semantic_falsifier_origin: SemanticFalsifierOrigin,
    semantic_falsifier_frame_ref: String,
    occurrence_refs: Vec<String>,
    transport_capture_origin: TransportCaptureOrigin,
    transport_capture_ref: String,
    transport_observed_octets: usize,
    transport_capture_count: usize,
    distinct_os_processes: bool,
    semantic_admission_count: usize,
    handler_count: usize,
    stored_decision_returned: bool,
    typed_outcome: String,
    retry_initiated: bool,
    transport_metadata_used_as_authority: bool,
    observer_safe: bool,
    cleanup_complete: bool,
    target_contract_authority_revalidation_count: usize,
    receiver_child_canary_events: Vec<ReceiverChildCanaryEvent>,
    client_child_probe_reply_receipts: Vec<ClientChildProbeReplyReceipt>,
}

impl EvidenceRow {
    /// Candidate identity; it is omitted only from normalized comparisons.
    pub const fn candidate(&self) -> TransportCandidate {
        self.candidate
    }

    /// Ordered common comparison case.
    pub const fn case(&self) -> CandidateCase {
        self.case
    }

    /// Candidate-specific transport mechanism label.
    pub fn mechanism(&self) -> &str {
        &self.mechanism
    }

    /// Checked-program provenance retained from the independent target edge.
    pub fn program_ref(&self) -> &str {
        &self.program_ref
    }

    /// Reference-only source provenance.
    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }

    /// Checked Core provenance.
    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    /// Generated source-artifact provenance.
    pub fn source_artifact_ref(&self) -> &str {
        &self.source_artifact_ref
    }

    /// Generated target-artifact provenance.
    pub fn target_artifact_ref(&self) -> &str {
        &self.target_artifact_ref
    }

    /// Generated communication-edge provenance.
    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    /// Semantic request identity, intentionally not a network occurrence.
    pub fn request_ref(&self) -> &str {
        &self.request_ref
    }

    /// Full retained contract fingerprint bound into this request and checked
    /// by the receiver child before any cache or handler access.
    pub fn retained_contract_fingerprint(&self) -> &str {
        &self.retained_contract_fingerprint
    }

    /// Origin of the one deliberate semantic falsifier frame.
    pub const fn semantic_falsifier_origin(&self) -> SemanticFalsifierOrigin {
        self.semantic_falsifier_origin
    }

    /// Observer-safe reference to the common falsifier frame. It is empty for
    /// cases that have no semantic falsifier.
    pub fn semantic_falsifier_frame_ref(&self) -> &str {
        &self.semantic_falsifier_frame_ref
    }

    /// Locally generated adapter/process occurrence evidence.
    pub fn occurrence_refs(&self) -> &[String] {
        &self.occurrence_refs
    }

    /// The origin of private transport capture used to compute the row's
    /// digest/count. The raw bytes are intentionally not observable here.
    pub const fn transport_capture_origin(&self) -> TransportCaptureOrigin {
        self.transport_capture_origin
    }

    /// Domain-separated SHA-256 reference over exact child-reported captures,
    /// including deterministic capture boundaries but never raw bytes.
    pub fn transport_capture_ref(&self) -> &str {
        &self.transport_capture_ref
    }

    /// Total byte count over exact child-reported transport captures.
    pub const fn transport_observed_octets(&self) -> usize {
        self.transport_observed_octets
    }

    /// Number of deterministic child-reported capture boundaries used in the
    /// reference. It distinguishes no bytes from one empty receive capture.
    pub const fn transport_capture_count(&self) -> usize {
        self.transport_capture_count
    }

    /// Whether server/client were distinct child processes for this case.
    pub const fn distinct_os_processes(&self) -> bool {
        self.distinct_os_processes
    }

    /// Number of target retained-contract semantic admissions.
    pub const fn semantic_admission_count(&self) -> usize {
        self.semantic_admission_count
    }

    /// Number of owner handler linearizations for the bounded case.
    pub const fn handler_count(&self) -> usize {
        self.handler_count
    }

    /// Whether an explicit duplicate returned a stored result rather than rerunning.
    pub const fn stored_decision_returned(&self) -> bool {
        self.stored_decision_returned
    }

    /// Typed semantic result/failure after common normalization.
    pub fn typed_outcome(&self) -> &str {
        &self.typed_outcome
    }

    /// Whether an application/semantic retry occurred. I3-0 requires false.
    pub const fn retry_initiated(&self) -> bool {
        self.retry_initiated
    }

    /// Whether connection/session/certificate metadata became authority.
    pub const fn transport_metadata_used_as_authority(&self) -> bool {
        self.transport_metadata_used_as_authority
    }

    /// Whether this row contains reference-only observer-safe facts.
    pub const fn observer_safe(&self) -> bool {
        self.observer_safe
    }

    /// Whether the child pair was reaped before this row was emitted.
    pub const fn cleanup_complete(&self) -> bool {
        self.cleanup_complete
    }

    /// Number of actual receiver-child retained-contract revalidations.
    pub const fn target_contract_authority_revalidation_count(&self) -> usize {
        self.target_contract_authority_revalidation_count
    }

    /// Ordered, reference-only receiver-child semantic facts.
    pub fn receiver_child_canary_events(&self) -> &[ReceiverChildCanaryEvent] {
        &self.receiver_child_canary_events
    }

    /// Actual client-child private reply receipts, represented only by safe
    /// refs and stored-decision refs.
    pub fn client_child_probe_reply_receipts(&self) -> &[ClientChildProbeReplyReceipt] {
        &self.client_child_probe_reply_receipts
    }

    pub(crate) fn normalized(&self) -> NormalizedEvidenceRow {
        NormalizedEvidenceRow {
            case: self.case,
            program_ref: self.program_ref.clone(),
            source_ref: self.source_ref.clone(),
            core_ref: self.core_ref.clone(),
            source_artifact_ref: self.source_artifact_ref.clone(),
            target_artifact_ref: self.target_artifact_ref.clone(),
            edge_ref: self.edge_ref.clone(),
            request_ref: self.request_ref.clone(),
            retained_contract_fingerprint: self.retained_contract_fingerprint.clone(),
            semantic_falsifier_origin: self.semantic_falsifier_origin,
            semantic_falsifier_frame_ref: self.semantic_falsifier_frame_ref.clone(),
            occurrence_count: self.occurrence_refs.len(),
            transport_capture_origin: self.transport_capture_origin,
            transport_capture_ref: self.transport_capture_ref.clone(),
            transport_observed_octets: self.transport_observed_octets,
            transport_capture_count: self.transport_capture_count,
            semantic_admission_count: self.semantic_admission_count,
            handler_count: self.handler_count,
            stored_decision_returned: self.stored_decision_returned,
            typed_outcome: self.typed_outcome.clone(),
            retry_initiated: self.retry_initiated,
            transport_metadata_used_as_authority: self.transport_metadata_used_as_authority,
            observer_safe: self.observer_safe,
            cleanup_complete: self.cleanup_complete,
            target_contract_authority_revalidation_count: self
                .target_contract_authority_revalidation_count,
        }
    }
}

/// Candidate-independent portion of an evidence row. Mechanism and candidate
/// identity are deliberately excluded; timing/addresses are never recorded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedEvidenceRow {
    case: CandidateCase,
    program_ref: String,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    request_ref: String,
    retained_contract_fingerprint: String,
    semantic_falsifier_origin: SemanticFalsifierOrigin,
    semantic_falsifier_frame_ref: String,
    occurrence_count: usize,
    transport_capture_origin: TransportCaptureOrigin,
    transport_capture_ref: String,
    transport_observed_octets: usize,
    transport_capture_count: usize,
    semantic_admission_count: usize,
    handler_count: usize,
    stored_decision_returned: bool,
    typed_outcome: String,
    retry_initiated: bool,
    transport_metadata_used_as_authority: bool,
    observer_safe: bool,
    cleanup_complete: bool,
    target_contract_authority_revalidation_count: usize,
}

impl NormalizedEvidenceRow {
    /// The common inventory row identity.
    pub const fn case(&self) -> CandidateCase {
        self.case
    }
}

/// Completed candidate evidence plus common lifecycle facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateRun {
    rows: Vec<EvidenceRow>,
    lifecycle: ProcessLifecycle,
    transport_features: TransportFeatures,
}

impl CandidateRun {
    /// Complete ordered evidence rows.
    pub fn rows(&self) -> &[EvidenceRow] {
        &self.rows
    }

    /// Returns one row by common case identity.
    pub fn row(&self, case: CandidateCase) -> Option<&EvidenceRow> {
        self.rows.iter().find(|row| row.case == case)
    }

    /// Common child process lifecycle evidence.
    pub const fn process_lifecycle(&self) -> &ProcessLifecycle {
        &self.lifecycle
    }

    /// Candidate feature declarations.
    pub const fn transport_features(&self) -> &TransportFeatures {
        &self.transport_features
    }

    /// Evidence comparable across candidate mechanism and timing differences.
    pub fn normalized_rows(&self) -> Vec<NormalizedEvidenceRow> {
        self.rows.iter().map(EvidenceRow::normalized).collect()
    }
}

/// Typed candidate-run failure with no raw input, source text, credential, or
/// endpoint embedded in its text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandidateRunErrorKind {
    /// A candidate stub has not yet provided actual transport behavior.
    CandidateUnavailable,
    /// The caller omitted the independently retained target edge.
    MissingTargetAdmissionEdge,
    /// The requested case order differs from the mandatory equal inventory.
    InvalidCaseInventory,
    /// The requested deadline was zero.
    InvalidDeadline,
    /// Ephemeral private control material could not be constructed.
    CredentialSetupFailed,
    /// Candidate bytes/disposition could not satisfy the common case contract.
    CandidateEvidenceMismatch,
    /// Child process control, exit, timeout, or cleanup failed.
    ChildProcessFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CandidateRunError {
    kind: CandidateRunErrorKind,
}

impl CandidateRunError {
    const fn new(kind: CandidateRunErrorKind) -> Self {
        Self { kind }
    }

    /// Returns a typed, observer-safe failure classification.
    pub const fn kind(&self) -> CandidateRunErrorKind {
        self.kind
    }
}

impl fmt::Display for CandidateRunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            CandidateRunErrorKind::CandidateUnavailable => {
                "I3-0 transport candidate is not implemented"
            }
            CandidateRunErrorKind::MissingTargetAdmissionEdge => {
                "I3-0 retained target admission edge is required"
            }
            CandidateRunErrorKind::InvalidCaseInventory => {
                "I3-0 candidate case inventory is not the fixed common order"
            }
            CandidateRunErrorKind::InvalidDeadline => "I3-0 candidate deadline is invalid",
            CandidateRunErrorKind::CredentialSetupFailed => {
                "I3-0 ephemeral private control setup failed"
            }
            CandidateRunErrorKind::CandidateEvidenceMismatch => {
                "I3-0 candidate transport evidence violated the common contract"
            }
            CandidateRunErrorKind::ChildProcessFailed => "I3-0 child process supervision failed",
        })
    }
}

impl Error for CandidateRunError {}

/// Doc-hidden supervisor fault used only by the finite I3-0 falsifier tests.
/// It is not a candidate transport option or a production child protocol.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SupervisorTestFault {
    EmitNonLoopbackReady,
    FailPostSpawnSetup,
    ExpireDeadline,
}

/// Observer-safe disposition of a forced common-supervisor fault.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupervisorFaultDisposition {
    NonLoopbackReadyRejected,
    PostSpawnSetupFailure,
    DeadlineExpired,
}

/// Bounded supervisor fault evidence. It names no endpoint, credential,
/// source, payload, or child PID.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SupervisorFaultProbeOutcome {
    fault: SupervisorTestFault,
    disposition: SupervisorFaultDisposition,
    actual_child_spawned: bool,
    deadline_elapsed_before_cleanup: bool,
    kill_attempted: bool,
    wait_completed: bool,
    no_orphan_remains: bool,
}

impl SupervisorFaultProbeOutcome {
    pub const fn fault(&self) -> SupervisorTestFault {
        self.fault
    }

    pub const fn disposition(&self) -> SupervisorFaultDisposition {
        self.disposition
    }

    pub const fn actual_child_spawned(&self) -> bool {
        self.actual_child_spawned
    }

    pub const fn deadline_elapsed_before_cleanup(&self) -> bool {
        self.deadline_elapsed_before_cleanup
    }

    pub const fn kill_attempted(&self) -> bool {
        self.kill_attempted
    }

    pub const fn wait_completed(&self) -> bool {
        self.wait_completed
    }

    pub const fn no_orphan_remains(&self) -> bool {
        self.no_orphan_remains
    }
}

/// Runs the fixed inventory. Candidate implementations remain deliberately
/// unavailable until their separately owned real-network modules are added.
pub fn run_candidate_inventory_in_child_processes(
    request: CandidateRunRequest,
) -> Result<CandidateRun, CandidateRunError> {
    if request.cases.as_slice() != CANONICAL_CASES {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::InvalidCaseInventory,
        ));
    }
    if request.deadline.is_zero() {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::InvalidDeadline,
        ));
    }
    let target_edge = request.target_admission_edge.ok_or(CandidateRunError::new(
        CandidateRunErrorKind::MissingTargetAdmissionEdge,
    ))?;
    if request.carrier.edge_ref() != target_edge.edge_ref()
        || request.carrier.target_locus() != target_edge.target_locus()
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }

    let encoded_frame = crate::encode_frame(&request.carrier)
        .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch))?;
    let prepared_cases = CANONICAL_CASES
        .into_iter()
        .map(|case| {
            PreparedCase::new(
                case,
                &encoded_frame,
                target_edge.retained_contract_fingerprint(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut lifecycle = ProcessLifecycle::new();
    let mut rows = Vec::with_capacity(CANONICAL_CASES.len());
    for prepared in prepared_cases {
        let mut harness = ChildProcessHarness::new(
            request.candidate,
            prepared.case,
            request.deadline,
            prepared.frame.clone(),
            target_edge.clone(),
        )
        .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CredentialSetupFailed))?;
        let observation = crate::candidates::execute_case(
            request.candidate,
            CandidateCaseInput::new(prepared.case, prepared.frame.clone()),
            &mut harness,
        );
        let case_lifecycle = harness.cleanup();
        lifecycle.merge(case_lifecycle.clone());
        let observation = observation.map_err(map_candidate_error)?;
        let row = normalize_observation(
            request.candidate,
            prepared.case,
            &request.carrier,
            &target_edge,
            &prepared,
            observation,
            &case_lifecycle,
        )?;
        rows.push(row);
    }
    Ok(CandidateRun {
        rows,
        lifecycle,
        transport_features: TransportFeatures::for_candidate(request.candidate),
    })
}

fn map_candidate_error(error: crate::candidates::CandidateExecutionError) -> CandidateRunError {
    CandidateRunError::new(match error.kind() {
        CandidateExecutionErrorKind::CandidateUnavailable => {
            CandidateRunErrorKind::CandidateUnavailable
        }
        CandidateExecutionErrorKind::ChildProtocolRejected
        | CandidateExecutionErrorKind::ChildLifecycleFailed
        | CandidateExecutionErrorKind::DeadlineExceeded => {
            CandidateRunErrorKind::ChildProcessFailed
        }
        CandidateExecutionErrorKind::TransportFailed => {
            CandidateRunErrorKind::CandidateEvidenceMismatch
        }
    })
}

const SEMANTIC_FALSIFIER_REF_DOMAIN: &[u8] = b"mirrorea/i3-0/common-semantic-falsifier/v1\0";
const NETWORK_OCCURRENCE_REF_DOMAIN: &[u8] = b"mirrorea/i3-0/network-occurrence/v1\0";

/// Immutable case bytes prepared once by the source-first common harness and
/// then handed unchanged to both candidate transports.
struct PreparedCase {
    case: CandidateCase,
    frame: Vec<u8>,
    semantic_falsifier_frame_ref: String,
}

impl PreparedCase {
    fn new(
        case: CandidateCase,
        ordinary_frame: &[u8],
        retained_contract_fingerprint: &str,
    ) -> Result<Self, CandidateRunError> {
        let frame = if case == CandidateCase::TamperedSemanticAdmissionReference {
            tampered_retained_contract_fingerprint_frame(ordinary_frame)?
        } else {
            ordinary_frame.to_vec()
        };
        let semantic_falsifier_frame_ref =
            if case == CandidateCase::TamperedSemanticAdmissionReference {
                semantic_falsifier_reference(&frame, retained_contract_fingerprint)
            } else {
                String::new()
            };
        Ok(Self {
            case,
            frame,
            semantic_falsifier_frame_ref,
        })
    }
}

fn tampered_retained_contract_fingerprint_frame(
    ordinary_frame: &[u8],
) -> Result<Vec<u8>, CandidateRunError> {
    let body = ordinary_frame.get(4..).ok_or(CandidateRunError::new(
        CandidateRunErrorKind::CandidateEvidenceMismatch,
    ))?;
    let mut envelope = serde_json::from_slice::<serde_json::Value>(body)
        .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch))?;
    let carrier = envelope
        .get_mut("carrier")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ))?;
    carrier.insert(
        "retained_contract_fingerprint".to_string(),
        serde_json::Value::String(
            "sys5-i3-probe-carrier-contract-sha256-v1:0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
        ),
    );
    let body = serde_json::to_vec(&envelope)
        .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch))?;
    if body.len() > crate::MAX_PRIVATE_FRAME_BYTES {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    let length = u32::try_from(body.len())
        .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch))?;
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&body);
    Ok(frame)
}

fn semantic_falsifier_reference(frame: &[u8], fingerprint: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(SEMANTIC_FALSIFIER_REF_DOMAIN);
    hasher.update(
        u64::try_from(fingerprint.len())
            .expect("private retained contract fingerprint length fits u64")
            .to_le_bytes(),
    );
    hasher.update(fingerprint.as_bytes());
    hasher.update(
        u64::try_from(frame.len())
            .expect("private falsifier frame length fits u64")
            .to_le_bytes(),
    );
    hasher.update(frame);
    format!(
        "i3-0-common-semantic-falsifier-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn normalize_observation(
    candidate: TransportCandidate,
    case: CandidateCase,
    carrier: &SemanticCarrier,
    target_edge: &SourceBoundEdge,
    prepared: &PreparedCase,
    observation: CandidateTransportObservation,
    lifecycle: &ProcessLifecycle,
) -> Result<EvidenceRow, CandidateRunError> {
    if observation.mechanism().is_empty()
        || observation.disposition() != expected_transport_disposition(case)
        || !lifecycle.server_and_client_are_distinct_children()
        || !lifecycle.deadline_enforced()
        || !lifecycle.cleanup_policy_declared()
        || !lifecycle.orphan_cleanup_complete()
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    let captures = observation.received_captures();
    validate_received_captures(case, observation.child_receive_kind(), captures, prepared)?;
    let canary_events = observation.receiver_child_canary_events();
    validate_receiver_child_trace(case, canary_events)?;
    let reply_receipts = observation.client_child_probe_reply_receipts();
    validate_client_child_reply_receipts(case, canary_events, reply_receipts)?;
    let (transport_capture_ref, transport_observed_octets) = capture_reference(captures)?;
    let occurrences = occurrence_references(case, canary_events, &transport_capture_ref)?;
    if occurrences
        .iter()
        .any(|occurrence| occurrence == carrier.request_identity().as_str())
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    let semantic_admission_count = canary_events
        .iter()
        .filter(|event| event.kind() == ReceiverChildCanaryEventKind::TargetAdmissionAccepted)
        .count();
    let handler_count = canary_events
        .iter()
        .filter(|event| event.kind() == ReceiverChildCanaryEventKind::ProbeHandlerLinearized)
        .count();
    let target_contract_authority_revalidation_count = canary_events
        .iter()
        .filter(|event| {
            matches!(
                event.kind(),
                ReceiverChildCanaryEventKind::TargetAdmissionAccepted
                    | ReceiverChildCanaryEventKind::TargetAdmissionRevalidated
            )
        })
        .count();
    let stored_decision_returned = canary_events
        .iter()
        .any(|event| event.kind() == ReceiverChildCanaryEventKind::StoredDecisionHit)
        && !reply_receipts.is_empty();
    Ok(EvidenceRow {
        candidate,
        case,
        mechanism: observation.mechanism().to_string(),
        program_ref: target_edge.program_ref().to_string(),
        source_ref: target_edge.source_ref().to_string(),
        core_ref: target_edge.core_ref().to_string(),
        source_artifact_ref: target_edge.source_artifact_ref().to_string(),
        target_artifact_ref: target_edge.target_artifact_ref().to_string(),
        edge_ref: target_edge.edge_ref().to_string(),
        request_ref: carrier.request_identity().as_str().to_string(),
        retained_contract_fingerprint: target_edge.retained_contract_fingerprint().to_string(),
        semantic_falsifier_origin: if case == CandidateCase::TamperedSemanticAdmissionReference {
            SemanticFalsifierOrigin::CommonHarness
        } else {
            SemanticFalsifierOrigin::NotApplicable
        },
        semantic_falsifier_frame_ref: prepared.semantic_falsifier_frame_ref.clone(),
        occurrence_refs: occurrences,
        transport_capture_origin: TransportCaptureOrigin::ChildProcessReceive,
        transport_capture_ref,
        transport_observed_octets,
        transport_capture_count: captures.len(),
        distinct_os_processes: lifecycle.server_and_client_are_distinct_children(),
        semantic_admission_count,
        handler_count,
        stored_decision_returned,
        typed_outcome: typed_outcome(case, observation.child_receive_kind(), canary_events),
        retry_initiated: false,
        transport_metadata_used_as_authority: false,
        observer_safe: true,
        cleanup_complete: lifecycle.orphan_cleanup_complete(),
        target_contract_authority_revalidation_count,
        receiver_child_canary_events: canary_events.to_vec(),
        client_child_probe_reply_receipts: reply_receipts.to_vec(),
    })
}

/// This table is a falsifier only: it checks that a candidate's transport
/// event belongs to the requested case. It never supplies semantic counts,
/// outcomes, reply facts, or retained provenance.
const fn expected_transport_disposition(case: CandidateCase) -> CandidateTransportDisposition {
    match case {
        CandidateCase::ConnectWithoutSemanticAdmission => CandidateTransportDisposition::Connected,
        CandidateCase::DeterministicFragmentedRoundTrip => {
            CandidateTransportDisposition::CompleteFrame
        }
        CandidateCase::TruncatedFrame => CandidateTransportDisposition::TruncatedFrame,
        CandidateCase::OversizedFrame => CandidateTransportDisposition::OversizedFrame,
        CandidateCase::DisconnectBeforeAdmission => {
            CandidateTransportDisposition::DisconnectBeforeAdmission
        }
        CandidateCase::DisconnectAfterAdmissionBeforeResult => {
            CandidateTransportDisposition::DisconnectAfterAdmissionBeforeResult
        }
        CandidateCase::DuplicateAcrossReconnect => {
            CandidateTransportDisposition::DuplicateAcrossReconnect
        }
        CandidateCase::TamperedSemanticAdmissionReference => {
            CandidateTransportDisposition::TamperedSemanticAdmissionReference
        }
        CandidateCase::ObserverSafeEvidence => CandidateTransportDisposition::ObserverSafeEvidence,
    }
}

fn validate_received_captures(
    case: CandidateCase,
    child_receive_kind: ChildReceiveKind,
    captures: &[Vec<u8>],
    prepared: &PreparedCase,
) -> Result<(), CandidateRunError> {
    match case {
        CandidateCase::ConnectWithoutSemanticAdmission => {
            if child_receive_kind == ChildReceiveKind::Complete && captures.is_empty() {
                Ok(())
            } else {
                Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ))
            }
        }
        CandidateCase::TruncatedFrame => {
            if child_receive_kind
                != ChildReceiveKind::Failure(ChildTransportFailureClass::TruncatedFrame)
                || captures.len() != 1
                || captures[0].len() <= 4
                || captures[0].len() >= prepared.frame.len()
                || !prepared.frame.starts_with(&captures[0])
                || !is_terminal_rejection(&captures[0], FrameDecodeErrorKind::TruncatedBody)
            {
                return Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ));
            }
            Ok(())
        }
        CandidateCase::OversizedFrame => {
            if child_receive_kind
                != ChildReceiveKind::Failure(ChildTransportFailureClass::OversizedFrame)
                || captures.len() != 1
                || captures[0].len() != 4
                || !is_terminal_rejection(&captures[0], FrameDecodeErrorKind::OversizedFrame)
            {
                return Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ));
            }
            Ok(())
        }
        CandidateCase::DisconnectBeforeAdmission => {
            if child_receive_kind
                == ChildReceiveKind::Failure(ChildTransportFailureClass::DisconnectBeforeAdmission)
                && captures.len() == 1
                && captures[0].is_empty()
            {
                Ok(())
            } else {
                Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ))
            }
        }
        CandidateCase::DeterministicFragmentedRoundTrip
        | CandidateCase::DisconnectAfterAdmissionBeforeResult
        | CandidateCase::ObserverSafeEvidence => {
            if child_receive_kind == ChildReceiveKind::Complete
                && captures.len() == 1
                && captures[0] == prepared.frame
            {
                Ok(())
            } else {
                Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ))
            }
        }
        CandidateCase::DuplicateAcrossReconnect => {
            if child_receive_kind == ChildReceiveKind::Complete
                && captures.len() == 2
                && captures.iter().all(|capture| capture == &prepared.frame)
            {
                Ok(())
            } else {
                Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ))
            }
        }
        CandidateCase::TamperedSemanticAdmissionReference => {
            if child_receive_kind == ChildReceiveKind::Complete
                && captures.len() == 1
                && captures[0] == prepared.frame
            {
                Ok(())
            } else {
                Err(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ))
            }
        }
    }
}

fn validate_receiver_child_trace(
    case: CandidateCase,
    events: &[ReceiverChildCanaryEvent],
) -> Result<(), CandidateRunError> {
    if events
        .iter()
        .enumerate()
        .any(|(index, event)| event.sequence() != index + 1)
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    let expected = match case {
        CandidateCase::ConnectWithoutSemanticAdmission
        | CandidateCase::TruncatedFrame
        | CandidateCase::OversizedFrame
        | CandidateCase::DisconnectBeforeAdmission => &[][..],
        CandidateCase::DeterministicFragmentedRoundTrip | CandidateCase::ObserverSafeEvidence => &[
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionAccepted,
            ReceiverChildCanaryEventKind::ProbeHandlerLinearized,
            ReceiverChildCanaryEventKind::DecisionStored,
        ][..],
        CandidateCase::DisconnectAfterAdmissionBeforeResult => &[
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionAccepted,
            ReceiverChildCanaryEventKind::ProbeHandlerLinearized,
            ReceiverChildCanaryEventKind::DecisionStored,
            ReceiverChildCanaryEventKind::ResultPathLost,
            ReceiverChildCanaryEventKind::AmbiguousDelivery,
        ][..],
        CandidateCase::DuplicateAcrossReconnect => &[
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionAccepted,
            ReceiverChildCanaryEventKind::ProbeHandlerLinearized,
            ReceiverChildCanaryEventKind::DecisionStored,
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::TargetAdmissionRevalidated,
            ReceiverChildCanaryEventKind::StoredDecisionHit,
        ][..],
        CandidateCase::TamperedSemanticAdmissionReference => &[
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            ReceiverChildCanaryEventKind::SemanticAdmissionRejected,
        ][..],
    };
    if events
        .iter()
        .map(ReceiverChildCanaryEvent::kind)
        .ne(expected.iter().copied())
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    if case == CandidateCase::TamperedSemanticAdmissionReference
        && events
            .last()
            .and_then(ReceiverChildCanaryEvent::rejection_kind)
            != Some(SemanticAdmissionErrorKind::RetainedContractFingerprintMismatch)
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    Ok(())
}

fn validate_client_child_reply_receipts(
    case: CandidateCase,
    events: &[ReceiverChildCanaryEvent],
    receipts: &[ClientChildProbeReplyReceipt],
) -> Result<(), CandidateRunError> {
    let expected_receipt_count = match case {
        CandidateCase::DeterministicFragmentedRoundTrip | CandidateCase::ObserverSafeEvidence => 1,
        CandidateCase::DuplicateAcrossReconnect => 2,
        CandidateCase::ConnectWithoutSemanticAdmission
        | CandidateCase::TruncatedFrame
        | CandidateCase::OversizedFrame
        | CandidateCase::DisconnectBeforeAdmission
        | CandidateCase::DisconnectAfterAdmissionBeforeResult
        | CandidateCase::TamperedSemanticAdmissionReference => 0,
    };
    if receipts.len() != expected_receipt_count
        || receipts.iter().enumerate().any(|(index, receipt)| {
            receipt.sequence() != index + 1 || receipt.receipt_ref().is_empty()
        })
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::CandidateEvidenceMismatch,
        ));
    }
    if case == CandidateCase::DuplicateAcrossReconnect {
        let Some(stored_ref) = events
            .iter()
            .find(|event| event.kind() == ReceiverChildCanaryEventKind::StoredDecisionHit)
            .and_then(ReceiverChildCanaryEvent::stored_decision_ref)
        else {
            return Err(CandidateRunError::new(
                CandidateRunErrorKind::CandidateEvidenceMismatch,
            ));
        };
        if receipts
            .iter()
            .any(|receipt| receipt.stored_decision_ref() != stored_ref)
        {
            return Err(CandidateRunError::new(
                CandidateRunErrorKind::CandidateEvidenceMismatch,
            ));
        }
    }
    Ok(())
}

fn occurrence_references(
    case: CandidateCase,
    events: &[ReceiverChildCanaryEvent],
    capture_ref: &str,
) -> Result<Vec<String>, CandidateRunError> {
    let event_occurrences = events
        .iter()
        .filter(|event| event.kind() == ReceiverChildCanaryEventKind::ReceiverChildFrameReceived)
        .count();
    let count = event_occurrences.max(1);
    (1..=count)
        .map(|index| {
            let mut hasher = Sha256::new();
            hasher.update(NETWORK_OCCURRENCE_REF_DOMAIN);
            hasher.update(case.label().as_bytes());
            hasher.update(
                u64::try_from(index)
                    .map_err(|_| {
                        CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch)
                    })?
                    .to_le_bytes(),
            );
            hasher.update(capture_ref.as_bytes());
            Ok(format!(
                "i3-0-network-occurrence-sha256-v1:{:x}",
                hasher.finalize()
            ))
        })
        .collect()
}

fn typed_outcome(
    case: CandidateCase,
    receive_kind: ChildReceiveKind,
    events: &[ReceiverChildCanaryEvent],
) -> String {
    match receive_kind {
        ChildReceiveKind::Failure(ChildTransportFailureClass::TruncatedFrame) => {
            "TruncatedFrame".to_string()
        }
        ChildReceiveKind::Failure(ChildTransportFailureClass::OversizedFrame) => {
            "OversizedFrame".to_string()
        }
        ChildReceiveKind::Failure(ChildTransportFailureClass::DisconnectBeforeAdmission) => {
            "DisconnectBeforeAdmission".to_string()
        }
        ChildReceiveKind::Failure(_) => "TransportFailure".to_string(),
        ChildReceiveKind::Complete => {
            if let Some(kind) = events
                .iter()
                .find(|event| {
                    event.kind() == ReceiverChildCanaryEventKind::SemanticAdmissionRejected
                })
                .and_then(ReceiverChildCanaryEvent::rejection_kind)
            {
                return format!("SemanticAdmissionRejected:{kind:?}");
            }
            if events
                .iter()
                .any(|event| event.kind() == ReceiverChildCanaryEventKind::AmbiguousDelivery)
            {
                return "AmbiguousDelivery".to_string();
            }
            if events
                .iter()
                .any(|event| event.kind() == ReceiverChildCanaryEventKind::StoredDecisionHit)
            {
                return "StoredDecisionReturned".to_string();
            }
            if case == CandidateCase::ConnectWithoutSemanticAdmission {
                return "ConnectedWithoutSemanticAdmission".to_string();
            }
            if case == CandidateCase::ObserverSafeEvidence {
                return "ObserverSafeEvidence".to_string();
            }
            "Accepted".to_string()
        }
    }
}

fn capture_reference(captures: &[Vec<u8>]) -> Result<(String, usize), CandidateRunError> {
    let mut hasher = Sha256::new();
    hasher.update(TRANSPORT_CAPTURE_REF_DOMAIN);
    hasher.update(
        u64::try_from(captures.len())
            .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch))?
            .to_le_bytes(),
    );
    let mut observed_octets = 0_usize;
    for capture in captures {
        observed_octets =
            observed_octets
                .checked_add(capture.len())
                .ok_or(CandidateRunError::new(
                    CandidateRunErrorKind::CandidateEvidenceMismatch,
                ))?;
        hasher.update(
            u64::try_from(capture.len())
                .map_err(|_| {
                    CandidateRunError::new(CandidateRunErrorKind::CandidateEvidenceMismatch)
                })?
                .to_le_bytes(),
        );
        hasher.update(capture);
    }
    Ok((
        format!("i3-0-transport-capture-sha256-v1:{:x}", hasher.finalize()),
        observed_octets,
    ))
}

fn is_terminal_rejection(frame: &[u8], expected: FrameDecodeErrorKind) -> bool {
    let mut decoder = FrameDecoder::new();
    let Ok(events) = decoder.push_events(frame) else {
        return false;
    };
    if events
        .iter()
        .any(|event| matches!(event, FrameDecodeEvent::Decoded(_)))
    {
        return false;
    }
    events
        .iter()
        .any(|event| matches!(event, FrameDecodeEvent::Rejected(kind) if *kind == expected))
        || decoder.finish_event().ok().flatten().is_some_and(
            |event| matches!(event, FrameDecodeEvent::Rejected(kind) if kind == expected),
        )
}

/// Private server/client process role. Only the role token may appear on the
/// command line; candidate, carrier, endpoint, and credential material travel
/// in the inherited standard-input pipe.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) enum ChildRole {
    Server,
    Client,
}

/// A private child handle; only the common supervisor records process IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ChildProcessHandle(usize);

/// Candidate-only child control. `Debug` is deliberately not implemented so
/// ephemeral key bytes cannot enter a formatter or error path.
#[derive(Deserialize, Serialize)]
pub(crate) struct ChildProcessControl {
    candidate: TransportCandidate,
    case: CandidateCase,
    role: ChildRole,
    frame: Vec<u8>,
    endpoint: Option<String>,
    target_admission_edge: Option<SourceBoundEdge>,
    credential: ChildCredential,
    supervisor_fault: Option<SupervisorTestFault>,
}

#[allow(dead_code)]
impl ChildProcessControl {
    pub(crate) const fn candidate(&self) -> TransportCandidate {
        self.candidate
    }

    pub(crate) const fn case(&self) -> CandidateCase {
        self.case
    }

    pub(crate) const fn role(&self) -> ChildRole {
        self.role
    }

    pub(crate) fn frame(&self) -> &[u8] {
        &self.frame
    }

    pub(crate) fn endpoint(&self) -> Option<&str> {
        self.endpoint.as_deref()
    }

    /// Reconstructs the fixed-capacity receiver-child canary solely from the
    /// independently retained verifier serialized into this server's private
    /// stdin control. It never reparses source or accepts a coordinator
    /// semantic verdict.
    pub(crate) fn receiver_child_canary(&self) -> Result<ReceiverChildCanary, CandidateChildError> {
        if self.role != ChildRole::Server {
            return Err(CandidateChildError::Protocol);
        }
        let target_edge = self
            .target_admission_edge
            .clone()
            .ok_or(CandidateChildError::Protocol)?;
        Ok(ReceiverChildCanary::new(self.case, target_edge))
    }

    pub(crate) fn certificate_der(&self) -> &[u8] {
        self.credential.certificate_der()
    }

    pub(crate) fn server_private_key_der(&self) -> Option<&[u8]> {
        self.credential.server_private_key_der()
    }
}

/// Secret child credential material. It is encoded only to private stdin and
/// has no `Debug`, `Display`, file, argv, or environment representation.
#[derive(Deserialize, Serialize)]
enum ChildCredential {
    Server {
        certificate_der: Vec<u8>,
        private_key_der: Vec<u8>,
    },
    ClientTrustRoot {
        certificate_der: Vec<u8>,
    },
}

#[allow(dead_code)]
impl ChildCredential {
    fn certificate_der(&self) -> &[u8] {
        match self {
            Self::Server {
                certificate_der, ..
            }
            | Self::ClientTrustRoot { certificate_der } => certificate_der,
        }
    }

    fn server_private_key_der(&self) -> Option<&[u8]> {
        match self {
            Self::Server {
                private_key_der, ..
            } => Some(private_key_der),
            Self::ClientTrustRoot { .. } => None,
        }
    }
}

struct EphemeralCredentials {
    certificate_der: Vec<u8>,
    private_key_der: Vec<u8>,
}

impl EphemeralCredentials {
    fn generate() -> Result<Self, ()> {
        let generated =
            generate_simple_self_signed(vec!["localhost".to_string()]).map_err(|_| ())?;
        Ok(Self {
            certificate_der: generated.cert.der().to_vec(),
            private_key_der: generated.signing_key.serialize_der(),
        })
    }

    fn for_role(&self, role: ChildRole) -> ChildCredential {
        match role {
            ChildRole::Server => ChildCredential::Server {
                certificate_der: self.certificate_der.clone(),
                private_key_der: self.private_key_der.clone(),
            },
            ChildRole::Client => ChildCredential::ClientTrustRoot {
                certificate_der: self.certificate_der.clone(),
            },
        }
    }
}

/// A candidate-facing bounded child supervisor. Its public-to-candidate API is
/// intentionally narrow; candidates cannot control lifecycle evidence values.
#[allow(dead_code)]
pub(crate) struct ChildProcessHarness {
    candidate: TransportCandidate,
    case: CandidateCase,
    deadline: Instant,
    frame: Vec<u8>,
    target_admission_edge: Option<SourceBoundEdge>,
    credentials: EphemeralCredentials,
    children: Vec<TrackedChild>,
    lifecycle: ProcessLifecycle,
}

#[allow(dead_code)]
impl ChildProcessHarness {
    fn new(
        candidate: TransportCandidate,
        case: CandidateCase,
        deadline: Duration,
        frame: Vec<u8>,
        target_admission_edge: SourceBoundEdge,
    ) -> Result<Self, ()> {
        let mut lifecycle = ProcessLifecycle::new();
        lifecycle.deadline_enforced = true;
        Ok(Self {
            candidate,
            case,
            deadline: Instant::now().checked_add(deadline).ok_or(())?,
            frame,
            target_admission_edge: Some(target_admission_edge),
            credentials: EphemeralCredentials::generate()?,
            children: Vec::new(),
            lifecycle,
        })
    }

    fn new_supervisor_probe(
        candidate: TransportCandidate,
        deadline: Duration,
    ) -> Result<Self, CandidateChildError> {
        let deadline = Instant::now()
            .checked_add(deadline)
            .ok_or(CandidateChildError::Deadline)?;
        let mut lifecycle = ProcessLifecycle::new();
        lifecycle.deadline_enforced = true;
        Ok(Self {
            candidate,
            case: CandidateCase::ConnectWithoutSemanticAdmission,
            deadline,
            frame: Vec::new(),
            target_admission_edge: None,
            credentials: EphemeralCredentials::generate()
                .map_err(|_| CandidateChildError::Lifecycle)?,
            children: Vec::new(),
            lifecycle,
        })
    }

    /// Spawns the server role with private control on stdin. The endpoint is
    /// intentionally absent until it emits a safe readiness event.
    pub(crate) fn spawn_server(&mut self) -> Result<ChildProcessHandle, CandidateChildError> {
        self.spawn(ChildRole::Server, None)
    }

    /// Spawns the client role only after a server readiness event supplied an
    /// endpoint. The endpoint is control data, never observer evidence.
    pub(crate) fn spawn_client(
        &mut self,
        endpoint: String,
    ) -> Result<ChildProcessHandle, CandidateChildError> {
        self.spawn(ChildRole::Client, Some(endpoint))
    }

    fn spawn_supervisor_fault_probe(
        &mut self,
        fault: SupervisorTestFault,
    ) -> Result<ChildProcessHandle, CandidateChildError> {
        if let Err(error) = self.remaining() {
            let _ = self.cleanup();
            return Err(error);
        }
        let executable = probe_binary_path().ok_or(CandidateChildError::Lifecycle)?;
        let control = ChildProcessControl {
            candidate: self.candidate,
            case: self.case,
            role: ChildRole::Server,
            frame: Vec::new(),
            endpoint: None,
            target_admission_edge: None,
            credential: self.credentials.for_role(ChildRole::Server),
            supervisor_fault: Some(fault),
        };
        self.spawn_control(
            executable,
            control,
            fault == SupervisorTestFault::FailPostSpawnSetup,
        )
    }

    /// Waits for one structured child event until the common deadline. A
    /// timeout immediately kills and waits every tracked child.
    pub(crate) fn next_event(
        &mut self,
        handle: ChildProcessHandle,
    ) -> Result<ChildProcessEvent, CandidateChildError> {
        let remaining = match self.remaining() {
            Ok(remaining) => remaining,
            Err(error) => {
                let _ = self.cleanup();
                return Err(error);
            }
        };
        let received = self
            .children
            .get_mut(handle.0)
            .and_then(|child| child.events.as_ref())
            .ok_or(CandidateChildError::Protocol)?
            .recv_timeout(remaining);
        match received {
            Ok(Ok(event)) => {
                if let ChildProcessEvent::Ready { endpoint } = &event
                    && !is_numeric_loopback_endpoint(endpoint)
                {
                    let _ = self.cleanup();
                    return Err(CandidateChildError::Protocol);
                }
                Ok(event)
            }
            Ok(Err(())) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                let _ = self.cleanup();
                Err(CandidateChildError::Protocol)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.lifecycle.deadline_enforced = true;
                let _ = self.cleanup();
                Err(CandidateChildError::Deadline)
            }
        }
    }

    /// Remaining common child-operation time. Candidates must use this rather
    /// than inventing retries or an independent timeout policy.
    pub(crate) fn remaining(&self) -> Result<Duration, CandidateChildError> {
        self.deadline
            .checked_duration_since(Instant::now())
            .filter(|duration| !duration.is_zero())
            .ok_or(CandidateChildError::Deadline)
    }

    fn spawn(
        &mut self,
        role: ChildRole,
        endpoint: Option<String>,
    ) -> Result<ChildProcessHandle, CandidateChildError> {
        if let Err(error) = self.remaining() {
            let _ = self.cleanup();
            return Err(error);
        }
        let executable = probe_binary_path().ok_or(CandidateChildError::Lifecycle)?;
        let control = ChildProcessControl {
            candidate: self.candidate,
            case: self.case,
            role,
            frame: self.frame.clone(),
            endpoint,
            target_admission_edge: (role == ChildRole::Server)
                .then(|| self.target_admission_edge.clone())
                .flatten(),
            credential: self.credentials.for_role(role),
            supervisor_fault: None,
        };
        self.spawn_control(executable, control, false)
    }

    fn spawn_control(
        &mut self,
        executable: PathBuf,
        control: ChildProcessControl,
        force_post_spawn_setup_failure: bool,
    ) -> Result<ChildProcessHandle, CandidateChildError> {
        let role = control.role;
        let encoded = serde_json::to_vec(&control).map_err(|_| CandidateChildError::Protocol)?;
        let child = Command::new(executable)
            .env_clear()
            .arg(match role {
                ChildRole::Server => "--i3-0-child-role=server",
                ChildRole::Client => "--i3-0-child-role=client",
            })
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| CandidateChildError::Lifecycle)?;
        let pid = child.id();
        match role {
            ChildRole::Server => self.lifecycle.server_pids.push(pid),
            ChildRole::Client => self.lifecycle.client_pids.push(pid),
        }
        // Registration happens immediately after OS spawn and before any
        // fallible pipe extraction or private-control delivery.
        self.children.push(TrackedChild {
            child,
            events: None,
            event_reader: None,
            stderr_reader: None,
            control_writer: None,
            reaped: false,
        });
        let handle = ChildProcessHandle(self.children.len() - 1);
        if force_post_spawn_setup_failure {
            let _ = self.cleanup();
            return Err(CandidateChildError::Lifecycle);
        }
        let (stdout, stderr, stdin) = match self.children.get_mut(handle.0) {
            Some(child) => match (
                child.child.stdout.take(),
                child.child.stderr.take(),
                child.child.stdin.take(),
            ) {
                (Some(stdout), Some(stderr), Some(stdin)) => (stdout, stderr, stdin),
                _ => {
                    let _ = self.cleanup();
                    return Err(CandidateChildError::Lifecycle);
                }
            },
            None => {
                let _ = self.cleanup();
                return Err(CandidateChildError::Lifecycle);
            }
        };
        let (events, event_reader) = spawn_event_reader(stdout);
        let stderr_reader = spawn_discard_reader(stderr);
        if let Some(child) = self.children.get_mut(handle.0) {
            child.events = Some(events);
            child.event_reader = Some(event_reader);
            child.stderr_reader = Some(stderr_reader);
        }
        if let Err(error) = self.deliver_private_control(handle, stdin, encoded) {
            let _ = self.cleanup();
            return Err(error);
        }
        Ok(handle)
    }

    fn deliver_private_control(
        &mut self,
        handle: ChildProcessHandle,
        mut stdin: std::process::ChildStdin,
        encoded: Vec<u8>,
    ) -> Result<(), CandidateChildError> {
        let remaining = self.remaining()?;
        let (sender, receiver) = mpsc::channel();
        let writer = thread::spawn(move || {
            let result = stdin.write_all(&encoded).and_then(|_| stdin.flush());
            let _ = sender.send(result.map_err(|_| ()));
        });
        let child = self
            .children
            .get_mut(handle.0)
            .ok_or(CandidateChildError::Lifecycle)?;
        child.control_writer = Some(writer);
        match receiver.recv_timeout(remaining) {
            Ok(Ok(())) => child
                .control_writer
                .take()
                .ok_or(CandidateChildError::Lifecycle)?
                .join()
                .map_err(|_| CandidateChildError::Lifecycle),
            Ok(Err(())) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(CandidateChildError::Lifecycle)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => Err(CandidateChildError::Deadline),
        }
    }

    fn cleanup(&mut self) -> ProcessLifecycle {
        let mut every_child_reaped = !self.children.is_empty();
        let mut every_wait_completed = !self.children.is_empty();
        for child in &mut self.children {
            if child.reaped {
                continue;
            }
            match child.child.try_wait() {
                Ok(Some(_)) => {}
                Ok(None) => {
                    self.lifecycle.kill_attempted = true;
                    if child.child.kill().is_err() {
                        every_child_reaped = false;
                    }
                }
                Err(_) => every_child_reaped = false,
            }
            if child.child.wait().is_err() {
                every_child_reaped = false;
                every_wait_completed = false;
            } else {
                child.reaped = true;
            }
            if let Some(reader) = child.event_reader.take()
                && reader.join().is_err()
            {
                every_child_reaped = false;
            }
            if let Some(reader) = child.stderr_reader.take()
                && reader.join().is_err()
            {
                every_child_reaped = false;
            }
            if let Some(writer) = child.control_writer.take()
                && writer.join().is_err()
            {
                every_child_reaped = false;
            }
        }
        self.lifecycle.cleanup_policy_declared = !self.children.is_empty();
        self.lifecycle.kill_wait_cleanup_enforced =
            self.lifecycle.cleanup_policy_declared && every_wait_completed;
        self.lifecycle.wait_completed = every_wait_completed;
        self.lifecycle.orphan_cleanup_complete =
            every_child_reaped && self.children.iter().all(|child| child.reaped);
        self.lifecycle.clone()
    }
}

impl Drop for ChildProcessHarness {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

/// Runs a real private child through one forced supervisor failure. This
/// doc-hidden seam exists solely to demonstrate that every post-spawn failure
/// kills, waits, and reaps rather than abandoning a child process.
#[doc(hidden)]
pub fn run_supervisor_fault_probe(
    candidate: TransportCandidate,
    fault: SupervisorTestFault,
) -> Result<SupervisorFaultProbeOutcome, CandidateRunError> {
    let deadline = match fault {
        SupervisorTestFault::ExpireDeadline => Duration::from_millis(25),
        SupervisorTestFault::EmitNonLoopbackReady | SupervisorTestFault::FailPostSpawnSetup => {
            Duration::from_secs(2)
        }
    };
    let mut harness = ChildProcessHarness::new_supervisor_probe(candidate, deadline)
        .map_err(|_| CandidateRunError::new(CandidateRunErrorKind::CredentialSetupFailed))?;
    let result = harness.spawn_supervisor_fault_probe(fault);
    let deadline_elapsed_before_cleanup = match (fault, result) {
        (SupervisorTestFault::EmitNonLoopbackReady, Ok(handle)) => {
            matches!(
                harness.next_event(handle),
                Err(CandidateChildError::Protocol)
            )
        }
        (SupervisorTestFault::FailPostSpawnSetup, Err(CandidateChildError::Lifecycle)) => false,
        (SupervisorTestFault::ExpireDeadline, Ok(handle)) => {
            matches!(
                harness.next_event(handle),
                Err(CandidateChildError::Deadline)
            )
        }
        _ => {
            return Err(CandidateRunError::new(
                CandidateRunErrorKind::ChildProcessFailed,
            ));
        }
    };
    let actual_child_spawned = !harness.children.is_empty();
    let lifecycle = harness.cleanup();
    if !actual_child_spawned
        || !lifecycle.kill_attempted
        || !lifecycle.wait_completed
        || !lifecycle.orphan_cleanup_complete
    {
        return Err(CandidateRunError::new(
            CandidateRunErrorKind::ChildProcessFailed,
        ));
    }
    let disposition = match fault {
        SupervisorTestFault::EmitNonLoopbackReady => {
            SupervisorFaultDisposition::NonLoopbackReadyRejected
        }
        SupervisorTestFault::FailPostSpawnSetup => {
            SupervisorFaultDisposition::PostSpawnSetupFailure
        }
        SupervisorTestFault::ExpireDeadline => SupervisorFaultDisposition::DeadlineExpired,
    };
    Ok(SupervisorFaultProbeOutcome {
        fault,
        disposition,
        actual_child_spawned,
        deadline_elapsed_before_cleanup,
        kill_attempted: lifecycle.kill_attempted,
        wait_completed: lifecycle.wait_completed,
        no_orphan_remains: lifecycle.orphan_cleanup_complete,
    })
}

struct TrackedChild {
    child: Child,
    events: Option<Receiver<Result<ChildProcessEvent, ()>>>,
    event_reader: Option<JoinHandle<()>>,
    stderr_reader: Option<JoinHandle<()>>,
    control_writer: Option<JoinHandle<()>>,
    reaped: bool,
}

fn spawn_event_reader(
    stdout: ChildStdout,
) -> (Receiver<Result<ChildProcessEvent, ()>>, JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let reader = thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let event = serde_json::from_str(line.trim_end()).map_err(|_| ());
                    if sender.send(event).is_err() {
                        break;
                    }
                }
                Err(_) => {
                    let _ = sender.send(Err(()));
                    break;
                }
            }
        }
    });
    (receiver, reader)
}

fn spawn_discard_reader(mut stderr: ChildStderr) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        while stderr.read(&mut buffer).is_ok_and(|read| read != 0) {}
    })
}

fn probe_binary_path() -> Option<PathBuf> {
    let current = std::env::current_exe().ok()?;
    if current
        .file_name()
        .is_some_and(|name| name == "mirrorea-i3-probe")
    {
        return Some(current);
    }
    let parent = current.parent()?;
    let debug_dir = if parent.file_name().is_some_and(|name| name == "deps") {
        parent.parent()?
    } else {
        parent
    };
    let candidate = debug_dir.join("mirrorea-i3-probe");
    candidate.is_file().then_some(candidate)
}

fn is_numeric_loopback_endpoint(endpoint: &str) -> bool {
    endpoint
        .parse::<SocketAddr>()
        .ok()
        .is_some_and(|address| matches!(address.ip(), IpAddr::V4(ip) if ip == Ipv4Addr::LOCALHOST))
}

/// Private child-to-coordinator event. It contains endpoint/transport data only
/// and must never contain semantic verdicts or private credential/source data.
/// Raw receive captures stay inside this private process-control channel; the
/// common row records only their digest and byte count.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) enum ChildProcessEvent {
    Ready {
        endpoint: String,
    },
    TransportComplete {
        received_frames: Vec<Vec<u8>>,
    },
    /// Server-child transport receipt plus its independently executed
    /// receiver canary facts. The coordinator may fold these facts into
    /// evidence but cannot manufacture them.
    ReceiverChildReport {
        received_frames: Vec<Vec<u8>>,
        report: ReceiverChildCanaryReport,
    },
    /// Actual client-child probe replies decoded after receiver-child result
    /// writes. Only reference-only receipt facts leave the client child.
    ClientProbeReplyReceipts {
        receipts: Vec<ClientChildProbeReplyReceipt>,
    },
    TransportFailure {
        class: ChildTransportFailureClass,
        received_capture: Vec<u8>,
    },
}

/// Typed child transport status, mapped by candidate implementations to the
/// common disposition table rather than directly to semantic outcomes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) enum ChildTransportFailureClass {
    TruncatedFrame,
    OversizedFrame,
    DisconnectBeforeAdmission,
    DisconnectAfterAdmissionBeforeResult,
    TransportFailure,
}

/// Non-secret candidate-facing child supervision failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CandidateChildError {
    Protocol,
    Lifecycle,
    Deadline,
    CandidateUnavailable,
}

/// Dispatches a private child role from the probe binary. Until a candidate
/// writer replaces its stub, this returns a typed unavailable error and emits
/// no control/data log.
pub(crate) fn run_child_role_from_stdio(role: ChildRole) -> Result<(), CandidateChildError> {
    let mut input = Vec::new();
    io::stdin()
        .take(512 * 1024)
        .read_to_end(&mut input)
        .map_err(|_| CandidateChildError::Protocol)?;
    let control = serde_json::from_slice::<ChildProcessControl>(&input)
        .map_err(|_| CandidateChildError::Protocol)?;
    if control.role != role {
        return Err(CandidateChildError::Protocol);
    }
    if control.supervisor_fault.is_some() {
        return run_supervisor_fault_child(control);
    }
    let event = crate::candidates::execute_child(control).map_err(|error| match error.kind() {
        CandidateExecutionErrorKind::CandidateUnavailable => {
            CandidateChildError::CandidateUnavailable
        }
        CandidateExecutionErrorKind::DeadlineExceeded => CandidateChildError::Deadline,
        CandidateExecutionErrorKind::ChildProtocolRejected => CandidateChildError::Protocol,
        CandidateExecutionErrorKind::ChildLifecycleFailed
        | CandidateExecutionErrorKind::TransportFailed => CandidateChildError::Lifecycle,
    })?;
    serde_json::to_writer(io::stdout(), &event).map_err(|_| CandidateChildError::Protocol)?;
    io::stdout()
        .write_all(b"\n")
        .map_err(|_| CandidateChildError::Protocol)
}

fn run_supervisor_fault_child(control: ChildProcessControl) -> Result<(), CandidateChildError> {
    match control
        .supervisor_fault
        .ok_or(CandidateChildError::Protocol)?
    {
        SupervisorTestFault::EmitNonLoopbackReady => {
            serde_json::to_writer(
                io::stdout(),
                &ChildProcessEvent::Ready {
                    endpoint: "192.0.2.1:1".to_string(),
                },
            )
            .map_err(|_| CandidateChildError::Protocol)?;
            io::stdout()
                .write_all(b"\n")
                .and_then(|_| io::stdout().flush())
                .map_err(|_| CandidateChildError::Protocol)?;
            thread::sleep(Duration::from_secs(60));
            Ok(())
        }
        SupervisorTestFault::ExpireDeadline => {
            thread::sleep(Duration::from_secs(60));
            Ok(())
        }
        SupervisorTestFault::FailPostSpawnSetup => Err(CandidateChildError::Protocol),
    }
}

/// Parses the intentionally minimal child role argument. Candidate identity
/// and all private material are rejected on argv and must arrive through stdin.
pub(crate) fn child_role_from_args(args: impl IntoIterator<Item = String>) -> Option<ChildRole> {
    let values = args.into_iter().collect::<Vec<_>>();
    match values.as_slice() {
        [value] if value == "--i3-0-child-role=server" => Some(ChildRole::Server),
        [value] if value == "--i3-0-child-role=client" => Some(ChildRole::Client),
        _ => None,
    }
}
