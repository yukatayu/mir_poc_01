#![doc = r#"
Private I3 transport comparison and static-adapter codec evidence.

The retained I3-0 comparison seam keeps its own `FrameDecoder` and
`SemanticCarrier` boundary.  A separate I3-1 static-adapter codec carries only
closed, source-derived, reference-only snapshots before exact source-bound
static admission.  Neither surface is a public API, wire, package format,
process protocol, transport, certificate representation, or compatibility
promise.
"#]
#![allow(unused_crate_dependencies)]

mod candidates;
mod framing;
mod i3_process_localnet;
mod model;
mod observer;
mod process_harness;
mod quic_static_adapter;
mod receiver_canary;
mod source_fixture;
mod static_adapter_framing;

pub use framing::{
    FrameDecodeErrorKind, FrameDecodeEvent, FrameDecoder, FrameDecoderStateError, FrameEncodeError,
    MAX_PRIVATE_FRAME_BYTES, WireCompatibility, encode_frame, private_wire_contract,
};
pub use i3_process_localnet::{
    I3LocalnetAdapterRejectionKind, I3LocalnetChildSlot, I3LocalnetChildTerminalEvent,
    I3LocalnetChildTerminalOutcome, I3LocalnetControlDelivery, I3LocalnetDeliveryPhase,
    I3LocalnetFailureStage, I3LocalnetFalsifier, I3LocalnetImageDelivery,
    I3LocalnetLifecycleRejectionCause, I3LocalnetObserverSafeDeliveryRecord,
    I3LocalnetRejectionAudit, I3LocalnetRunError, I3LocalnetRunErrorKind, I3ProcessLocalnetRequest,
    I3ProcessLocalnetRun, run_i3_process_localnet,
};
pub use model::{
    RequestIdentity, SemanticAdmissionError, SemanticAdmissionErrorKind, SemanticCarrier,
    SemanticRequestBindingError, SemanticRequestBindingErrorKind, SemanticRequestSeed,
    SourceBoundAdapterEdge, SourceBoundEdge, SourceBoundProbe, StaticAdapterAdmissionError,
    StaticAdapterAdmissionErrorKind, UntrustedDecodedCarrier, UntrustedDecodedStaticAdapterCarrier,
};
pub use observer::{
    ObserverEvidence, ObserverEvidenceError, ObserverEvidenceErrorKind,
    render_observer_safe_evidence, validate_observer_safe_evidence,
};
pub use process_harness::{
    CandidateCase, CandidateRun, CandidateRunError, CandidateRunErrorKind, CandidateRunRequest,
    CredentialDelivery, EvidenceRow, NormalizedEvidenceRow, ProcessLifecycle,
    SemanticFalsifierOrigin, SupervisorCleanupBreachDimension, SupervisorCleanupFailureKind,
    SupervisorFaultDisposition, SupervisorFaultProbeOutcome, SupervisorTestFault,
    TransportCandidate, TransportCaptureOrigin, TransportFeatures,
    run_candidate_inventory_in_child_processes, run_supervisor_fault_probe,
};
pub use quic_static_adapter::{
    StaticAdapterQuicAdmissionOutcome, StaticAdapterQuicFalsifier, StaticAdapterQuicIngressEvent,
    StaticAdapterQuicObserverError, StaticAdapterQuicObserverErrorKind,
    StaticAdapterQuicPlatformClaim, StaticAdapterQuicRun, StaticAdapterQuicRunError,
    StaticAdapterQuicRunErrorKind, StaticAdapterQuicTransportEvent,
    StaticAdapterQuicTransportEventKind, StaticAdapterQuicTransportFeatures,
    encode_private_static_adapter_quic_ingress_for_test, run_static_adapter_quic_loopback,
    run_static_adapter_quic_loopback_from_private_ingress,
    run_static_adapter_quic_loopback_with_falsifier,
    validate_static_adapter_quic_observer_evidence,
};
pub use receiver_canary::{
    ClientChildProbeReplyReceipt, ReceiverChildCanaryEvent, ReceiverChildCanaryEventKind,
};
pub use source_fixture::{SourceBoundProbeError, build_source_bound_probe};
pub use static_adapter_framing::{
    MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES, StaticAdapterFrameDecodeErrorKind,
    StaticAdapterFrameDecodeEvent, StaticAdapterFrameDecoder, StaticAdapterFrameDecoderStateError,
    StaticAdapterFrameEncodeError, StaticAdapterFrameLimits, StaticAdapterWireCompatibility,
    encode_static_adapter_frame, private_static_adapter_frame_reference,
    private_static_adapter_snapshot_reference, private_static_adapter_wire_contract,
};

/// Private binary bridge only. This unpublished crate does not offer a stable
/// command surface; the wrapper prevents the binary target from reaching into
/// non-public process-control implementation details.
#[doc(hidden)]
pub fn run_private_child_process(args: impl IntoIterator<Item = String>) -> bool {
    let args = args.into_iter().collect::<Vec<_>>();
    if let Some(result) = i3_process_localnet::run_private_localnet_child_from_args(args.clone()) {
        return result;
    }
    if let Some(result) = process_harness::run_private_supervisor_fault_from_args(args.clone()) {
        return result;
    }
    let Some(role) = process_harness::child_role_from_args(args) else {
        return false;
    };
    process_harness::run_child_role_from_stdio(role).is_ok()
}
