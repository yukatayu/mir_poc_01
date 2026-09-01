//! Private receiver-child semantic canary for the finite I3-0 comparison.
//!
//! The canary is deliberately not an actual Mir owner runtime. It has no
//! irreversible external effect: it only proves that a receiving OS child can
//! decode and revalidate a retained contract before a bounded, no-eviction
//! stored-decision lookup. Candidate transports own socket mechanics only.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    CandidateCase, FrameDecodeEvent, FrameDecoder, SemanticAdmissionErrorKind, SourceBoundEdge,
};

const DECISION_REF_DOMAIN: &[u8] = b"mirrorea/i3-0/receiver-canary-decision/v1\0";
const FRAME_REF_DOMAIN: &[u8] = b"mirrorea/i3-0/receiver-canary-frame/v1\0";
const RECEIPT_REF_DOMAIN: &[u8] = b"mirrorea/i3-0/client-child-reply-receipt/v1\0";
const RECEIVER_CANARY_CACHE_CAPACITY: usize = 8;

/// Ordered semantic facts emitted by the receiver child. These are private
/// I3-0 evidence names, not a public event vocabulary.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ReceiverChildCanaryEventKind {
    ReceiverChildFrameReceived,
    TargetAdmissionAccepted,
    TargetAdmissionRevalidated,
    ProbeHandlerLinearized,
    DecisionStored,
    StoredDecisionHit,
    SemanticAdmissionRejected,
    ResultPathLost,
    AmbiguousDelivery,
}

/// One reference-only fact from a receiver child. The raw frame, source,
/// principal, authority, capability, and witness never leave child control.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReceiverChildCanaryEvent {
    sequence: usize,
    kind: ReceiverChildCanaryEventKind,
    frame_ref: Option<String>,
    request_ref: Option<String>,
    retained_contract_fingerprint: Option<String>,
    stored_decision_ref: Option<String>,
    rejection_kind: Option<SemanticAdmissionErrorKind>,
}

impl ReceiverChildCanaryEvent {
    /// Monotonic, receiver-child-local event sequence.
    pub const fn sequence(&self) -> usize {
        self.sequence
    }

    /// Private canary event kind.
    pub const fn kind(&self) -> ReceiverChildCanaryEventKind {
        self.kind
    }

    pub(crate) const fn rejection_kind(&self) -> Option<SemanticAdmissionErrorKind> {
        self.rejection_kind
    }

    pub(crate) fn stored_decision_ref(&self) -> Option<&str> {
        self.stored_decision_ref.as_deref()
    }
}

/// Observer-safe receipt observed by the actual client child after it decodes
/// a private receiver reply. Raw reply bytes never enter an evidence row.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClientChildProbeReplyReceipt {
    sequence: usize,
    receipt_ref: String,
    stored_decision_ref: String,
    received_by_client_child: bool,
}

impl ClientChildProbeReplyReceipt {
    /// Confirms this provenance object was created by the private client-child
    /// reply parser before crossing the private child-control channel. The
    /// coordinator has no constructor for this type and cannot derive it from
    /// case expectations or sent input bytes.
    pub const fn received_by_client_child(&self) -> bool {
        self.received_by_client_child
    }

    /// Monotonic client-child receipt sequence for this case.
    pub const fn sequence(&self) -> usize {
        self.sequence
    }

    /// Reference-only digest of the exact reply observed by the client child.
    pub fn receipt_ref(&self) -> &str {
        &self.receipt_ref
    }

    /// The stored-decision reference returned by the receiver child.
    pub fn stored_decision_ref(&self) -> &str {
        &self.stored_decision_ref
    }

    /// Parses one actual private reply received by the client child. This does
    /// not reconstruct a reply from a frame or case table.
    pub(crate) fn from_received_reply(sequence: usize, received_reply: &[u8]) -> Option<Self> {
        let reply = serde_json::from_slice::<ReceiverChildReplyEnvelope>(received_reply).ok()?;
        if reply.version != 1 || reply.stored_decision_ref.is_empty() {
            return None;
        }
        let mut hasher = Sha256::new();
        hasher.update(RECEIPT_REF_DOMAIN);
        hasher.update(u64::try_from(sequence).ok()?.to_le_bytes());
        hasher.update(
            u64::try_from(reply.stored_decision_ref.len())
                .ok()?
                .to_le_bytes(),
        );
        hasher.update(reply.stored_decision_ref.as_bytes());
        hasher.update(u64::try_from(received_reply.len()).ok()?.to_le_bytes());
        hasher.update(received_reply);
        Some(Self {
            sequence,
            receipt_ref: format!(
                "i3-0-client-child-reply-receipt-sha256-v1:{:x}",
                hasher.finalize()
            ),
            stored_decision_ref: reply.stored_decision_ref,
            received_by_client_child: true,
        })
    }
}

/// Private server-child report folded by the coordinator into an evidence
/// row. It has no coordinator-supplied semantic counts or outcome fields.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ReceiverChildCanaryReport {
    events: Vec<ReceiverChildCanaryEvent>,
}

impl ReceiverChildCanaryReport {
    pub(crate) fn new(events: Vec<ReceiverChildCanaryEvent>) -> Self {
        Self { events }
    }

    pub(crate) fn events(&self) -> &[ReceiverChildCanaryEvent] {
        &self.events
    }
}

/// Receiver response bytes held only inside the server child until a
/// candidate transport writes them. It intentionally has no `Debug`.
pub(crate) struct ReceiverChildProbeReply(Vec<u8>);

impl ReceiverChildProbeReply {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Result of one complete ingress frame after receiver-side decode and exact
/// target admission.
pub(crate) enum ReceiverChildIngress {
    Reply(ReceiverChildProbeReply),
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ReceiverChildReplyEnvelope {
    version: u8,
    stored_decision_ref: String,
}

struct StoredDecision {
    request_ref: String,
    retained_contract_fingerprint: String,
    decision_ref: String,
}

/// Fixed-capacity, no-eviction receiver child canary. It receives an already
/// serialized retained verifier through private stdin; it never reparses
/// source and never receives network/session/certificate identity as a cache
/// key or authority input.
pub(crate) struct ReceiverChildCanary {
    case: CandidateCase,
    target_edge: SourceBoundEdge,
    stored_decisions: Vec<StoredDecision>,
    events: Vec<ReceiverChildCanaryEvent>,
}

impl ReceiverChildCanary {
    pub(crate) fn new(case: CandidateCase, target_edge: SourceBoundEdge) -> Self {
        Self {
            case,
            target_edge,
            stored_decisions: Vec::with_capacity(RECEIVER_CANARY_CACHE_CAPACITY),
            events: Vec::new(),
        }
    }

    /// Full-decodes and revalidates a complete ingress frame before checking
    /// the fixed cache. The reply is returned only after `DecisionStored` was
    /// recorded, so callers cannot write application reply bytes first.
    pub(crate) fn receive_complete_frame(&mut self, frame: &[u8]) -> ReceiverChildIngress {
        self.push_untrusted_frame_received(frame);
        let Some(candidate) = decode_exactly_one(frame) else {
            self.push_event(
                ReceiverChildCanaryEventKind::SemanticAdmissionRejected,
                None,
                None,
                None,
                None,
            );
            return ReceiverChildIngress::Rejected;
        };
        let admitted = match self.target_edge.admit_untrusted_candidate(candidate) {
            Ok(admitted) => admitted,
            Err(error) => {
                self.push_event(
                    ReceiverChildCanaryEventKind::SemanticAdmissionRejected,
                    None,
                    None,
                    None,
                    Some(error.kind()),
                );
                return ReceiverChildIngress::Rejected;
            }
        };
        let request_ref = admitted.request_identity().as_str().to_string();
        let fingerprint = admitted.retained_contract_fingerprint().to_string();
        if let Some((stored_request_ref, stored_fingerprint, stored_decision_ref)) = self
            .stored_decisions
            .iter()
            .find(|stored| {
                stored.request_ref == request_ref
                    && stored.retained_contract_fingerprint == fingerprint
            })
            .map(|stored| {
                (
                    stored.request_ref.clone(),
                    stored.retained_contract_fingerprint.clone(),
                    stored.decision_ref.clone(),
                )
            })
        {
            self.push_event(
                ReceiverChildCanaryEventKind::TargetAdmissionRevalidated,
                Some(request_ref),
                Some(fingerprint),
                Some(stored_decision_ref.clone()),
                None,
            );
            self.push_event(
                ReceiverChildCanaryEventKind::StoredDecisionHit,
                Some(stored_request_ref),
                Some(stored_fingerprint),
                Some(stored_decision_ref.clone()),
                None,
            );
            return ReceiverChildIngress::Reply(private_reply(&stored_decision_ref));
        }
        self.push_event(
            ReceiverChildCanaryEventKind::TargetAdmissionAccepted,
            Some(request_ref.clone()),
            Some(fingerprint.clone()),
            None,
            None,
        );
        if self.stored_decisions.len() == RECEIVER_CANARY_CACHE_CAPACITY {
            // The bounded canary has no eviction policy. It rejects a new
            // decision rather than replacing a previously linearized one.
            self.push_event(
                ReceiverChildCanaryEventKind::SemanticAdmissionRejected,
                None,
                None,
                None,
                None,
            );
            return ReceiverChildIngress::Rejected;
        }
        self.push_event(
            ReceiverChildCanaryEventKind::ProbeHandlerLinearized,
            Some(request_ref.clone()),
            Some(fingerprint.clone()),
            None,
            None,
        );
        let decision_ref = decision_reference(&request_ref, &fingerprint);
        self.stored_decisions.push(StoredDecision {
            request_ref: request_ref.clone(),
            retained_contract_fingerprint: fingerprint.clone(),
            decision_ref: decision_ref.clone(),
        });
        self.push_event(
            ReceiverChildCanaryEventKind::DecisionStored,
            Some(request_ref),
            Some(fingerprint),
            Some(decision_ref.clone()),
            None,
        );
        ReceiverChildIngress::Reply(private_reply(&decision_ref))
    }

    /// Records the explicit common result-path cut after an already stored
    /// decision. It is never a resend or semantic retry.
    pub(crate) fn cut_result_path_before_reply(&mut self) {
        let Some((request_ref, fingerprint, decision_ref)) =
            self.stored_decisions.last().map(|stored| {
                (
                    stored.request_ref.clone(),
                    stored.retained_contract_fingerprint.clone(),
                    stored.decision_ref.clone(),
                )
            })
        else {
            return;
        };
        self.push_event(
            ReceiverChildCanaryEventKind::ResultPathLost,
            Some(request_ref.clone()),
            Some(fingerprint.clone()),
            Some(decision_ref.clone()),
            None,
        );
        self.push_event(
            ReceiverChildCanaryEventKind::AmbiguousDelivery,
            Some(request_ref),
            Some(fingerprint),
            Some(decision_ref),
            None,
        );
    }

    pub(crate) fn report(self) -> ReceiverChildCanaryReport {
        ReceiverChildCanaryReport::new(self.events)
    }

    fn push_untrusted_frame_received(&mut self, frame: &[u8]) {
        let mut hasher = Sha256::new();
        hasher.update(FRAME_REF_DOMAIN);
        hasher.update(self.case.label().as_bytes());
        hasher.update(
            u64::try_from(self.events.len() + 1)
                .expect("bounded canary sequence fits u64")
                .to_le_bytes(),
        );
        hasher.update(
            u64::try_from(frame.len())
                .expect("private frame length fits u64")
                .to_le_bytes(),
        );
        hasher.update(frame);
        self.push_event(
            ReceiverChildCanaryEventKind::ReceiverChildFrameReceived,
            None,
            None,
            Some(format!(
                "i3-0-receiver-child-frame-sha256-v1:{:x}",
                hasher.finalize()
            )),
            None,
        );
    }

    fn push_event(
        &mut self,
        kind: ReceiverChildCanaryEventKind,
        request_ref: Option<String>,
        retained_contract_fingerprint: Option<String>,
        stored_decision_ref: Option<String>,
        rejection_kind: Option<SemanticAdmissionErrorKind>,
    ) {
        self.events.push(ReceiverChildCanaryEvent {
            sequence: self.events.len() + 1,
            kind,
            frame_ref: (kind == ReceiverChildCanaryEventKind::ReceiverChildFrameReceived)
                .then_some(stored_decision_ref.clone())
                .flatten(),
            request_ref,
            retained_contract_fingerprint,
            stored_decision_ref: (kind != ReceiverChildCanaryEventKind::ReceiverChildFrameReceived)
                .then_some(stored_decision_ref)
                .flatten(),
            rejection_kind,
        });
    }
}

fn decode_exactly_one(frame: &[u8]) -> Option<crate::UntrustedDecodedCarrier> {
    let mut decoder = FrameDecoder::new();
    let events = decoder.push_events(frame).ok()?;
    if decoder.finish_event().ok()?.is_some() {
        return None;
    }
    match events.as_slice() {
        [FrameDecodeEvent::Decoded(candidate)] => Some((**candidate).clone()),
        _ => None,
    }
}

fn decision_reference(request_ref: &str, fingerprint: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(DECISION_REF_DOMAIN);
    hasher.update(
        u64::try_from(request_ref.len())
            .expect("private request reference length fits u64")
            .to_le_bytes(),
    );
    hasher.update(request_ref.as_bytes());
    hasher.update(
        u64::try_from(fingerprint.len())
            .expect("private retained contract fingerprint length fits u64")
            .to_le_bytes(),
    );
    hasher.update(fingerprint.as_bytes());
    format!(
        "i3-0-receiver-child-decision-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn private_reply(stored_decision_ref: &str) -> ReceiverChildProbeReply {
    ReceiverChildProbeReply(
        serde_json::to_vec(&ReceiverChildReplyEnvelope {
            version: 1,
            stored_decision_ref: stored_decision_ref.to_string(),
        })
        .expect("fixed private receiver reply serializes"),
    )
}
