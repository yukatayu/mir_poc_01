//! Candidate-only transport seams for the private I3-0 comparison.
//!
//! The common harness owns source/Core/artifact/edge/request facts, semantic
//! admission, retry policy, and normalized evidence. A candidate may report
//! only bytes received through its actual transport plus a transport-local
//! disposition and mechanism name. These stubs deliberately remain unavailable
//! until the separate TLS/TCP and QUIC writers install real child-process
//! implementations.

use std::{error::Error, fmt, sync::Arc};

use crate::{
    CandidateCase, ClientChildProbeReplyReceipt, ReceiverChildCanaryEvent, TransportCandidate,
    process_harness::{
        ChildProcessControl, ChildProcessEvent, ChildProcessHarness, ChildTransportFailureClass,
    },
};

mod quic;
mod tls_tcp;

/// Candidate-visible input whose semantic facts were fixed by the common
/// harness. It deliberately exposes frame bytes, not mutable source/Core or
/// authority fields.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct CandidateCaseInput {
    case: CandidateCase,
    frame: Arc<[u8]>,
}

#[allow(dead_code)]
impl CandidateCaseInput {
    pub(crate) fn new(case: CandidateCase, frame: Vec<u8>) -> Self {
        Self {
            case,
            frame: Arc::from(frame),
        }
    }

    pub(crate) const fn case(&self) -> CandidateCase {
        self.case
    }

    pub(crate) fn frame(&self) -> &[u8] {
        &self.frame
    }
}

/// Candidate-local outcome before the common layer performs framing and exact
/// retained-contract admission. It must never contain a semantic verdict.
#[derive(Clone, Debug)]
pub(crate) struct CandidateTransportObservation {
    mechanism: &'static str,
    disposition: CandidateTransportDisposition,
    received_captures: Vec<Vec<u8>>,
    child_receive_kind: ChildReceiveKind,
    receiver_child_canary_events: Vec<ReceiverChildCanaryEvent>,
    client_child_probe_reply_receipts: Vec<ClientChildProbeReplyReceipt>,
}

impl CandidateTransportObservation {
    /// Constructs an observation only from a child event. Candidate
    /// coordinators cannot reconstruct a successful capture from their local
    /// `CandidateCaseInput`; the child-reported bytes are the sole source.
    pub(crate) fn from_child_process_events(
        mechanism: &'static str,
        disposition: CandidateTransportDisposition,
        server_event: ChildProcessEvent,
        client_event: ChildProcessEvent,
    ) -> Result<Self, CandidateExecutionError> {
        let (received_captures, child_receive_kind, receiver_child_canary_events) =
            match server_event {
                ChildProcessEvent::ReceiverChildReport {
                    received_frames,
                    report,
                } => (
                    received_frames,
                    ChildReceiveKind::Complete,
                    report.events().to_vec(),
                ),
                ChildProcessEvent::TransportComplete { received_frames } => {
                    (received_frames, ChildReceiveKind::Complete, Vec::new())
                }
                ChildProcessEvent::TransportFailure {
                    class,
                    received_capture,
                } => (
                    vec![received_capture],
                    ChildReceiveKind::Failure(class),
                    Vec::new(),
                ),
                ChildProcessEvent::Ready { .. }
                | ChildProcessEvent::ClientProbeReplyReceipts { .. } => {
                    return Err(CandidateExecutionError::new(
                        CandidateExecutionErrorKind::ChildProtocolRejected,
                    ));
                }
            };
        let client_child_probe_reply_receipts = match client_event {
            ChildProcessEvent::ClientProbeReplyReceipts { receipts } => receipts,
            ChildProcessEvent::TransportComplete { received_frames }
                if received_frames.is_empty() =>
            {
                Vec::new()
            }
            ChildProcessEvent::Ready { .. }
            | ChildProcessEvent::ReceiverChildReport { .. }
            | ChildProcessEvent::TransportComplete { .. }
            | ChildProcessEvent::TransportFailure { .. } => {
                return Err(CandidateExecutionError::new(
                    CandidateExecutionErrorKind::ChildProtocolRejected,
                ));
            }
        };
        Ok(Self {
            mechanism,
            disposition,
            received_captures,
            child_receive_kind,
            receiver_child_canary_events,
            client_child_probe_reply_receipts,
        })
    }

    pub(crate) const fn mechanism(&self) -> &'static str {
        self.mechanism
    }

    pub(crate) const fn disposition(&self) -> CandidateTransportDisposition {
        self.disposition
    }

    pub(crate) fn received_captures(&self) -> &[Vec<u8>] {
        &self.received_captures
    }

    pub(crate) const fn child_receive_kind(&self) -> ChildReceiveKind {
        self.child_receive_kind
    }

    pub(crate) fn receiver_child_canary_events(&self) -> &[ReceiverChildCanaryEvent] {
        &self.receiver_child_canary_events
    }

    pub(crate) fn client_child_probe_reply_receipts(&self) -> &[ClientChildProbeReplyReceipt] {
        &self.client_child_probe_reply_receipts
    }
}

/// A direct classification of the child event that supplied raw capture bytes.
/// It is not a semantic outcome and cannot be supplied by a coordinator alone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChildReceiveKind {
    Complete,
    Failure(ChildTransportFailureClass),
}

/// Transport facts that candidates may report. The common layer maps only a
/// matching disposition to the fixed semantic case table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CandidateTransportDisposition {
    Connected,
    CompleteFrame,
    TruncatedFrame,
    OversizedFrame,
    DisconnectBeforeAdmission,
    DisconnectAfterAdmissionBeforeResult,
    DuplicateAcrossReconnect,
    TamperedSemanticAdmissionReference,
    ObserverSafeEvidence,
}

/// A typed candidate execution failure that contains no certificate, key,
/// source text, raw payload, or host endpoint.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CandidateExecutionErrorKind {
    CandidateUnavailable,
    ChildProtocolRejected,
    ChildLifecycleFailed,
    DeadlineExceeded,
    TransportFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CandidateExecutionError {
    kind: CandidateExecutionErrorKind,
}

#[allow(dead_code)]
impl CandidateExecutionError {
    pub(crate) const fn unavailable() -> Self {
        Self {
            kind: CandidateExecutionErrorKind::CandidateUnavailable,
        }
    }

    pub(crate) const fn new(kind: CandidateExecutionErrorKind) -> Self {
        Self { kind }
    }

    pub(crate) const fn kind(&self) -> CandidateExecutionErrorKind {
        self.kind
    }
}

impl fmt::Display for CandidateExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            CandidateExecutionErrorKind::CandidateUnavailable => {
                "I3-0 transport candidate is not implemented"
            }
            CandidateExecutionErrorKind::ChildProtocolRejected => {
                "I3-0 child process protocol was rejected"
            }
            CandidateExecutionErrorKind::ChildLifecycleFailed => {
                "I3-0 child process lifecycle failed"
            }
            CandidateExecutionErrorKind::DeadlineExceeded => {
                "I3-0 child operation exceeded deadline"
            }
            CandidateExecutionErrorKind::TransportFailed => "I3-0 candidate transport failed",
        })
    }
}

impl Error for CandidateExecutionError {}

/// Runs one candidate case. Candidate writers may use only the common
/// harness's child-process/control API and must return raw transport evidence;
/// they cannot construct rows or semantic admissions.
pub(crate) fn execute_case(
    candidate: TransportCandidate,
    input: CandidateCaseInput,
    harness: &mut ChildProcessHarness,
) -> Result<CandidateTransportObservation, CandidateExecutionError> {
    match candidate {
        TransportCandidate::TlsOverTcpFramedReliableStream => tls_tcp::execute(input, harness),
        TransportCandidate::QuicReliableBidirectionalStream => quic::execute(input, harness),
    }
}

/// Dispatches one already-supervised child role. Candidate source owns all
/// candidate-specific socket behavior, while the common binary owns the
/// private stdin protocol and never writes credentials to output.
pub(crate) fn execute_child(
    control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    match control.candidate() {
        TransportCandidate::TlsOverTcpFramedReliableStream => tls_tcp::execute_child(control),
        TransportCandidate::QuicReliableBidirectionalStream => quic::execute_child(control),
    }
}
