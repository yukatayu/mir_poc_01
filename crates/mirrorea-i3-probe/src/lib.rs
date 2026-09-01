#![doc = r#"
Private I3-0 transport comparison probe.

This unpublished crate is executable evidence only. Its Rust surface, process
protocol, framing, codec, limits, diagnostics, certificate representation, and
candidate libraries are not public compatibility promises and do not define the
I3-1 adapter or wire.
"#]
#![allow(unused_crate_dependencies)]

mod candidates;
mod framing;
mod model;
mod observer;
mod process_harness;
mod receiver_canary;
mod source_fixture;

pub use framing::{
    FrameDecodeErrorKind, FrameDecodeEvent, FrameDecoder, FrameDecoderStateError, FrameEncodeError,
    MAX_PRIVATE_FRAME_BYTES, WireCompatibility, encode_frame, private_wire_contract,
};
pub use model::{
    RequestIdentity, SemanticAdmissionError, SemanticAdmissionErrorKind, SemanticCarrier,
    SemanticRequestBindingError, SemanticRequestBindingErrorKind, SemanticRequestSeed,
    SourceBoundEdge, SourceBoundProbe, UntrustedDecodedCarrier,
};
pub use observer::{
    ObserverEvidence, ObserverEvidenceError, ObserverEvidenceErrorKind,
    render_observer_safe_evidence, validate_observer_safe_evidence,
};
pub use process_harness::{
    CandidateCase, CandidateRun, CandidateRunError, CandidateRunErrorKind, CandidateRunRequest,
    CredentialDelivery, EvidenceRow, NormalizedEvidenceRow, ProcessLifecycle,
    SemanticFalsifierOrigin, SupervisorFaultDisposition, SupervisorFaultProbeOutcome,
    SupervisorTestFault, TransportCandidate, TransportCaptureOrigin, TransportFeatures,
    run_candidate_inventory_in_child_processes, run_supervisor_fault_probe,
};
pub use receiver_canary::{
    ClientChildProbeReplyReceipt, ReceiverChildCanaryEvent, ReceiverChildCanaryEventKind,
};
pub use source_fixture::{SourceBoundProbeError, build_source_bound_probe};

/// Private binary bridge only. This unpublished crate does not offer a stable
/// command surface; the wrapper prevents the binary target from reaching into
/// non-public process-control implementation details.
#[doc(hidden)]
pub fn run_private_child_process(args: impl IntoIterator<Item = String>) -> bool {
    let Some(role) = process_harness::child_role_from_args(args) else {
        return false;
    };
    process_harness::run_child_role_from_stdio(role).is_ok()
}
