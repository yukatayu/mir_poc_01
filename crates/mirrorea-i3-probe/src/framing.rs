//! Complete-only private framing for I3-0 transport comparison evidence.

use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{SemanticCarrier, UntrustedDecodedCarrier};

/// Maximum private probe frame body size. This is an I3-0 resource guard,
/// not a public wire-format commitment.
pub const MAX_PRIVATE_FRAME_BYTES: usize = 64 * 1024;

const PRIVATE_WIRE_VERSION: u16 = 1;
const PRIVATE_WIRE_MARKER: &str = "mirrorea-i3-probe";

/// Compatibility classification for this private probe framing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireCompatibility {
    /// Private comparison evidence that I3-1 may replace without compatibility
    /// obligations.
    PrivateProvisional,
}

/// Non-authoritative framing contract facts exposed for test evidence only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivateWireContract {
    max_frame_bytes: usize,
    compatibility: WireCompatibility,
}

impl PrivateWireContract {
    /// The maximum checked before allocating a frame body.
    pub const fn max_frame_bytes(&self) -> usize {
        self.max_frame_bytes
    }

    /// The framing compatibility classification.
    pub const fn compatibility(&self) -> WireCompatibility {
        self.compatibility
    }
}

/// Returns the private, provisional I3-0 framing contract.
pub const fn private_wire_contract() -> PrivateWireContract {
    PrivateWireContract {
        max_frame_bytes: MAX_PRIVATE_FRAME_BYTES,
        compatibility: WireCompatibility::PrivateProvisional,
    }
}

/// A typed private framing failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameDecodeErrorKind {
    /// The declared body length exceeds the private probe maximum.
    OversizedFrame,
    /// End of input occurred before all four prefix bytes arrived.
    TruncatedPrefix,
    /// End of input occurred before the declared body completed.
    TruncatedBody,
    /// The complete body did not contain valid private JSON.
    MalformedPayload,
    /// The complete body used a private wire version this probe does not understand.
    UnknownVersion,
    /// A terminal decoder cannot admit more input.
    DecoderRejected,
}

/// An event emitted from one complete frame or terminal frame rejection.
/// Completed decoded candidates are preserved even if later coalesced bytes
/// reject the stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrameDecodeEvent {
    /// Complete JSON decoded only to an untrusted candidate.
    Decoded(Box<UntrustedDecodedCarrier>),
    /// A typed terminal framing rejection with no candidate for that frame.
    Rejected(FrameDecodeErrorKind),
}

/// A typed decoder state error. Frame-content failures are emitted as events
/// so they cannot erase earlier complete events in the same byte chunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameDecoderStateError {
    kind: FrameDecodeErrorKind,
}

impl FrameDecoderStateError {
    const fn new(kind: FrameDecodeErrorKind) -> Self {
        Self { kind }
    }

    /// The terminal decoder-state classification.
    pub const fn kind(&self) -> FrameDecodeErrorKind {
        self.kind
    }
}

impl fmt::Display for FrameDecoderStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("private I3-0 decoder is terminal")
    }
}

impl Error for FrameDecoderStateError {}

/// A non-secret encoding failure for the private comparison wire.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameEncodeError {
    /// Serialized JSON exceeds the private maximum.
    OversizedFrame,
    /// JSON serialization failed before any frame bytes were produced.
    SerializationFailed,
}

impl fmt::Display for FrameEncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::OversizedFrame => "private I3-0 frame exceeds its limit",
            Self::SerializationFailed => "private I3-0 carrier could not serialize",
        })
    }
}

impl Error for FrameEncodeError {}

/// Encodes one locally bound private carrier as a big-endian u32
/// length-prefixed JSON frame. There is no encoder for decoded untrusted data.
pub fn encode_frame(carrier: &SemanticCarrier) -> Result<Vec<u8>, FrameEncodeError> {
    let body = serde_json::to_vec(&PrivateWireEnvelopeOut {
        marker: PRIVATE_WIRE_MARKER,
        version: PRIVATE_WIRE_VERSION,
        carrier,
    })
    .map_err(|_| FrameEncodeError::SerializationFailed)?;
    if body.len() > MAX_PRIVATE_FRAME_BYTES {
        return Err(FrameEncodeError::OversizedFrame);
    }
    let length = u32::try_from(body.len()).map_err(|_| FrameEncodeError::OversizedFrame)?;
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&body);
    Ok(frame)
}

/// Incremental complete-only decoder for private probe frames.
#[derive(Debug, Default)]
pub struct FrameDecoder {
    prefix: Vec<u8>,
    declared_body_length: Option<usize>,
    body: Vec<u8>,
    terminal: bool,
}

impl FrameDecoder {
    /// Starts a private decoder with no pending prefix or body.
    pub const fn new() -> Self {
        Self {
            prefix: Vec::new(),
            declared_body_length: None,
            body: Vec::new(),
            terminal: false,
        }
    }

    /// Receives bytes and emits complete untrusted candidates plus, if needed,
    /// one terminal rejection event. It never returns a frame-content error as
    /// `Err`, preserving earlier decoded events for coalesced and split input.
    pub fn push_events(
        &mut self,
        mut bytes: &[u8],
    ) -> Result<Vec<FrameDecodeEvent>, FrameDecoderStateError> {
        if self.terminal {
            return Err(FrameDecoderStateError::new(
                FrameDecodeErrorKind::DecoderRejected,
            ));
        }

        let mut events = Vec::new();
        while !bytes.is_empty() {
            if self.declared_body_length.is_none() {
                let required = 4 - self.prefix.len();
                let take = required.min(bytes.len());
                self.prefix.extend_from_slice(&bytes[..take]);
                bytes = &bytes[take..];
                if self.prefix.len() != 4 {
                    continue;
                }
                let declared = u32::from_be_bytes(
                    self.prefix
                        .as_slice()
                        .try_into()
                        .expect("private prefix is exactly four bytes"),
                );
                let declared = usize::try_from(declared)
                    .expect("a u32 private frame length always fits usize");
                if declared > MAX_PRIVATE_FRAME_BYTES {
                    return Ok(self.terminal_event(events, FrameDecodeErrorKind::OversizedFrame));
                }
                self.declared_body_length = Some(declared);
                self.body.clear();
                self.body.reserve(declared);
            }

            let declared = self
                .declared_body_length
                .expect("complete prefix establishes a body length");
            let remaining = declared - self.body.len();
            let take = remaining.min(bytes.len());
            self.body.extend_from_slice(&bytes[..take]);
            bytes = &bytes[take..];
            if self.body.len() != declared {
                continue;
            }

            match decode_body(&self.body) {
                Ok(candidate) => events.push(FrameDecodeEvent::Decoded(Box::new(candidate))),
                Err(kind) => return Ok(self.terminal_event(events, kind)),
            }
            self.prefix.clear();
            self.declared_body_length = None;
            self.body.clear();
        }
        Ok(events)
    }

    /// Declares end-of-input, returning a typed rejection event for an
    /// incomplete prefix/body while preserving the no-admission boundary.
    pub fn finish_event(&mut self) -> Result<Option<FrameDecodeEvent>, FrameDecoderStateError> {
        if self.terminal {
            return Err(FrameDecoderStateError::new(
                FrameDecodeErrorKind::DecoderRejected,
            ));
        }
        if self.declared_body_length.is_some() {
            self.terminal = true;
            return Ok(Some(FrameDecodeEvent::Rejected(
                FrameDecodeErrorKind::TruncatedBody,
            )));
        }
        if !self.prefix.is_empty() {
            self.terminal = true;
            return Ok(Some(FrameDecodeEvent::Rejected(
                FrameDecodeErrorKind::TruncatedPrefix,
            )));
        }
        self.terminal = true;
        Ok(None)
    }

    fn terminal_event(
        &mut self,
        mut events: Vec<FrameDecodeEvent>,
        kind: FrameDecodeErrorKind,
    ) -> Vec<FrameDecodeEvent> {
        self.terminal = true;
        events.push(FrameDecodeEvent::Rejected(kind));
        events
    }
}

#[derive(Serialize)]
struct PrivateWireEnvelopeOut<'a> {
    marker: &'static str,
    version: u16,
    carrier: &'a SemanticCarrier,
}

#[derive(Deserialize)]
struct PrivateWireEnvelopeIn {
    marker: String,
    version: u16,
    carrier: serde_json::Value,
}

fn decode_body(body: &[u8]) -> Result<UntrustedDecodedCarrier, FrameDecodeErrorKind> {
    let value = serde_json::from_slice::<serde_json::Value>(body)
        .map_err(|_| FrameDecodeErrorKind::MalformedPayload)?;
    let Some(version) = value.get("version").and_then(serde_json::Value::as_u64) else {
        return Err(FrameDecodeErrorKind::MalformedPayload);
    };
    if version != u64::from(PRIVATE_WIRE_VERSION) {
        return Err(FrameDecodeErrorKind::UnknownVersion);
    }
    if !has_exact_keys(&value, &["marker", "version", "carrier"]) {
        return Err(FrameDecodeErrorKind::MalformedPayload);
    }
    let envelope = serde_json::from_value::<PrivateWireEnvelopeIn>(value)
        .map_err(|_| FrameDecodeErrorKind::MalformedPayload)?;
    if envelope.marker != PRIVATE_WIRE_MARKER || envelope.version != PRIVATE_WIRE_VERSION {
        return Err(FrameDecodeErrorKind::UnknownVersion);
    }
    if !has_exact_keys(&envelope.carrier, CARRIER_FIELDS) {
        return Err(FrameDecodeErrorKind::MalformedPayload);
    }
    let carrier_bytes = serde_json::to_vec(&envelope.carrier)
        .map_err(|_| FrameDecodeErrorKind::MalformedPayload)?;
    UntrustedDecodedCarrier::from_json(&carrier_bytes)
        .map_err(|_| FrameDecodeErrorKind::MalformedPayload)
}

const CARRIER_FIELDS: &[&str] = &[
    "retained_contract_fingerprint",
    "checked_program_ref",
    "operation",
    "edge_kind",
    "lifecycle_kind",
    "source_locus",
    "target_locus",
    "logical_source_path",
    "source_start",
    "source_end",
    "source_start_line",
    "source_start_column",
    "source_end_line",
    "source_end_column",
    "source_ref",
    "core_ref",
    "source_artifact_ref",
    "target_artifact_ref",
    "edge_ref",
    "declared_failure_names",
    "effect_kind_names",
    "required_occurrence_slot_names",
    "linked_request_identity",
    "typed_outcome",
    "authority_category_names",
    "requires_membership_epoch_and_incarnation",
    "requires_capability_and_witness_refs",
    "reference_only_redaction",
    "checked_core_bound",
    "transfers_authority",
    "semantic_request_seed",
    "request_identity",
];

fn has_exact_keys(value: &serde_json::Value, expected: &[&str]) -> bool {
    let Some(object) = value.as_object() else {
        return false;
    };
    object.len() == expected.len() && expected.iter().all(|key| object.contains_key(*key))
}
