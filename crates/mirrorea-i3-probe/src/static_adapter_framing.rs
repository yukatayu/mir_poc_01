//! Complete-only private framing for the closed I3-1 static adapter algebra.
//!
//! This module is intentionally separate from the retained I3-0
//! `FrameDecoder` / `SemanticCarrier` comparison seam.  Its only decoded
//! value is an untrusted, reference-only static snapshot; source-bound
//! equality admission remains in `SourceBoundAdapterEdge`.

use std::{error::Error, fmt};

use mir_runtime::sys5_local_slice::Sys5I3AdapterWireSnapshot;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{SourceBoundAdapterEdge, UntrustedDecodedStaticAdapterCarrier};

/// Maximum body size for the private/provisional static-adapter codec.  This
/// is a local resource guard, not a public wire-format commitment.
pub const MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES: usize = 256 * 1024;

const MAX_PRIVATE_STATIC_ADAPTER_PUSH_BYTES: usize = 512 * 1024;
const MAX_PRIVATE_STATIC_ADAPTER_DECODED_EVENTS_PER_PUSH: usize = 16;
const PRIVATE_STATIC_ADAPTER_WIRE_MARKER: &str = "mirrorea-i3-static-adapter";
const PRIVATE_STATIC_ADAPTER_WIRE_VERSION: u16 = 1;
const PRIVATE_STATIC_ADAPTER_REFERENCE_DOMAIN: &[u8] =
    b"mirrorea/i3-1/static-adapter/private-reference/v1\0";
const PRIVATE_STATIC_ADAPTER_REFERENCE_PREFIX: &str =
    "mirrorea-i3-static-adapter-private-ref-sha256-v1:";

/// Compatibility classification for the private static-adapter framing.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticAdapterWireCompatibility {
    /// Private evidence that may change without any public compatibility
    /// obligation.
    PrivateProvisional,
}

/// Non-authoritative private static-adapter framing limits and compatibility
/// facts, exposed only for executable evidence.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivateStaticAdapterWireContract {
    max_frame_bytes: usize,
    max_push_bytes: usize,
    max_decoded_events_per_push: usize,
    compatibility: StaticAdapterWireCompatibility,
}

impl PrivateStaticAdapterWireContract {
    pub const fn max_frame_bytes(&self) -> usize {
        self.max_frame_bytes
    }

    pub const fn max_push_bytes(&self) -> usize {
        self.max_push_bytes
    }

    pub const fn max_decoded_events_per_push(&self) -> usize {
        self.max_decoded_events_per_push
    }

    pub const fn compatibility(&self) -> StaticAdapterWireCompatibility {
        self.compatibility
    }
}

/// Returns the private/provisional static-adapter framing contract.
#[doc(hidden)]
pub const fn private_static_adapter_wire_contract() -> PrivateStaticAdapterWireContract {
    PrivateStaticAdapterWireContract {
        max_frame_bytes: MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES,
        max_push_bytes: MAX_PRIVATE_STATIC_ADAPTER_PUSH_BYTES,
        max_decoded_events_per_push: MAX_PRIVATE_STATIC_ADAPTER_DECODED_EVENTS_PER_PUSH,
        compatibility: StaticAdapterWireCompatibility::PrivateProvisional,
    }
}

/// Per-push resource limits for one private static-adapter decoder.  All
/// limits are local operational guards and carry no semantic authority.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticAdapterFrameLimits {
    max_frame_bytes: usize,
    max_push_bytes: usize,
    max_decoded_events_per_push: usize,
}

impl StaticAdapterFrameLimits {
    /// Returns no limit set for zero or prefix-unrepresentable bounds.
    pub const fn new(
        max_frame_bytes: usize,
        max_push_bytes: usize,
        max_decoded_events_per_push: usize,
    ) -> Option<Self> {
        if max_frame_bytes == 0
            || max_frame_bytes > u32::MAX as usize
            || max_push_bytes == 0
            || max_decoded_events_per_push == 0
        {
            return None;
        }
        Some(Self {
            max_frame_bytes,
            max_push_bytes,
            max_decoded_events_per_push,
        })
    }
}

const PRIVATE_STATIC_ADAPTER_DEFAULT_LIMITS: StaticAdapterFrameLimits = StaticAdapterFrameLimits {
    max_frame_bytes: MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES,
    max_push_bytes: MAX_PRIVATE_STATIC_ADAPTER_PUSH_BYTES,
    max_decoded_events_per_push: MAX_PRIVATE_STATIC_ADAPTER_DECODED_EVENTS_PER_PUSH,
};

/// A typed private static-adapter decoding failure.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticAdapterFrameDecodeErrorKind {
    OversizedFrame,
    PushByteLimitExceeded,
    PushDecodedEventLimitExceeded,
    TruncatedPrefix,
    TruncatedBody,
    MalformedPayload,
    MarkerMismatch,
    UnknownVersion,
    BodyAllocationFailed,
    DecoderRejected,
}

/// One complete decoded candidate or terminal rejection.  Earlier decoded
/// candidates are retained in this event list if later coalesced bytes fail.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticAdapterFrameDecodeEvent {
    Decoded(Box<UntrustedDecodedStaticAdapterCarrier>),
    Rejected(StaticAdapterFrameDecodeErrorKind),
}

/// A terminal decoder-state error.  Content errors are event values so an
/// earlier complete candidate cannot be erased by later bytes in one push.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticAdapterFrameDecoderStateError {
    kind: StaticAdapterFrameDecodeErrorKind,
}

impl StaticAdapterFrameDecoderStateError {
    const fn new(kind: StaticAdapterFrameDecodeErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> StaticAdapterFrameDecodeErrorKind {
        self.kind
    }
}

impl fmt::Display for StaticAdapterFrameDecoderStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("private static adapter decoder is terminal")
    }
}

impl Error for StaticAdapterFrameDecoderStateError {}

/// A non-secret private static-adapter encoding failure.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaticAdapterFrameEncodeError {
    OversizedFrame,
    SerializationFailed,
}

impl fmt::Display for StaticAdapterFrameEncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::OversizedFrame => "private static adapter frame exceeds its limit",
            Self::SerializationFailed => "private static adapter snapshot could not serialize",
        })
    }
}

impl Error for StaticAdapterFrameEncodeError {}

/// Encodes exactly one source-bound static-adapter handoff.  No encoder exists
/// for untrusted decoded input.
#[doc(hidden)]
pub fn encode_static_adapter_frame(
    edge: &SourceBoundAdapterEdge,
) -> Result<Vec<u8>, StaticAdapterFrameEncodeError> {
    let body = serde_json::to_vec(&PrivateStaticAdapterWireEnvelopeOut {
        marker: PRIVATE_STATIC_ADAPTER_WIRE_MARKER,
        version: PRIVATE_STATIC_ADAPTER_WIRE_VERSION,
        carrier: edge.retained_static_contract(),
    })
    .map_err(|_| StaticAdapterFrameEncodeError::SerializationFailed)?;
    if body.len() > MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES {
        return Err(StaticAdapterFrameEncodeError::OversizedFrame);
    }
    let length =
        u32::try_from(body.len()).map_err(|_| StaticAdapterFrameEncodeError::OversizedFrame)?;
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&body);
    Ok(frame)
}

/// Observer-safe reference to exact private static-adapter frame bytes.  The
/// bytes themselves remain inside the private adapter boundary.
#[doc(hidden)]
pub fn private_static_adapter_frame_reference(frame: &[u8]) -> String {
    private_static_adapter_reference(b"frame", frame)
}

/// Observer-safe reference to the complete retained source-owned snapshot.
/// This is static equality evidence, never runtime or transport identity.
#[doc(hidden)]
pub fn private_static_adapter_snapshot_reference(edge: &SourceBoundAdapterEdge) -> String {
    private_static_adapter_snapshot_reference_for_snapshot(edge.retained_static_contract())
}

pub(crate) fn private_static_adapter_snapshot_reference_for_snapshot(
    snapshot: &Sys5I3AdapterWireSnapshot,
) -> String {
    let snapshot = serde_json::to_vec(snapshot)
        .expect("private static adapter snapshot must remain serializable");
    private_static_adapter_reference(b"snapshot", &snapshot)
}

pub(crate) fn tamper_private_static_adapter_target_locus(
    frame: &[u8],
) -> Result<Vec<u8>, StaticAdapterFrameEncodeError> {
    let (prefix, body) = frame
        .split_at_checked(4)
        .ok_or(StaticAdapterFrameEncodeError::SerializationFailed)?;
    let declared = usize::try_from(u32::from_be_bytes(
        prefix
            .try_into()
            .map_err(|_| StaticAdapterFrameEncodeError::SerializationFailed)?,
    ))
    .map_err(|_| StaticAdapterFrameEncodeError::SerializationFailed)?;
    if declared != body.len() {
        return Err(StaticAdapterFrameEncodeError::SerializationFailed);
    }
    // This is deliberately only a pre-send falsifier constructor.  Decoder
    // admission never traverses `Value`; it uses the closed strict structs.
    let mut envelope: serde_json::Value = serde_json::from_slice(body)
        .map_err(|_| StaticAdapterFrameEncodeError::SerializationFailed)?;
    let changed = replace_first_target_locus(&mut envelope);
    if !changed {
        return Err(StaticAdapterFrameEncodeError::SerializationFailed);
    }
    let body = serde_json::to_vec(&envelope)
        .map_err(|_| StaticAdapterFrameEncodeError::SerializationFailed)?;
    if body.len() > MAX_PRIVATE_STATIC_ADAPTER_FRAME_BYTES {
        return Err(StaticAdapterFrameEncodeError::OversizedFrame);
    }
    let length =
        u32::try_from(body.len()).map_err(|_| StaticAdapterFrameEncodeError::OversizedFrame)?;
    let mut output = Vec::with_capacity(4 + body.len());
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(&body);
    Ok(output)
}

fn replace_first_target_locus(value: &mut serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(values) => {
            if let Some(serde_json::Value::String(target)) = values.get_mut("target_locus") {
                target.push_str("-i3-private-tampered");
                return true;
            }
            values.values_mut().any(replace_first_target_locus)
        }
        serde_json::Value::Array(values) => values.iter_mut().any(replace_first_target_locus),
        serde_json::Value::Null
        | serde_json::Value::Bool(_)
        | serde_json::Value::Number(_)
        | serde_json::Value::String(_) => false,
    }
}

fn private_static_adapter_reference(label: &[u8], bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(PRIVATE_STATIC_ADAPTER_REFERENCE_DOMAIN);
    hasher.update(
        u64::try_from(label.len())
            .expect("static reference label fits u64")
            .to_le_bytes(),
    );
    hasher.update(label);
    hasher.update(
        u64::try_from(bytes.len())
            .expect("static reference bytes fit u64")
            .to_le_bytes(),
    );
    hasher.update(bytes);
    format!(
        "{PRIVATE_STATIC_ADAPTER_REFERENCE_PREFIX}{:x}",
        hasher.finalize()
    )
}

/// Incremental complete-only decoder for private static-adapter frames.
#[doc(hidden)]
#[derive(Debug)]
pub struct StaticAdapterFrameDecoder {
    limits: StaticAdapterFrameLimits,
    prefix: Vec<u8>,
    declared_body_length: Option<usize>,
    body: Vec<u8>,
    terminal: bool,
}

impl Default for StaticAdapterFrameDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticAdapterFrameDecoder {
    pub const fn new() -> Self {
        Self {
            limits: PRIVATE_STATIC_ADAPTER_DEFAULT_LIMITS,
            prefix: Vec::new(),
            declared_body_length: None,
            body: Vec::new(),
            terminal: false,
        }
    }

    pub const fn with_private_limits(limits: StaticAdapterFrameLimits) -> Self {
        Self {
            limits,
            prefix: Vec::new(),
            declared_body_length: None,
            body: Vec::new(),
            terminal: false,
        }
    }

    /// Emits only complete untrusted snapshots.  Each invocation gets a fresh
    /// byte/event budget; incomplete prefix/body state may span invocations.
    pub fn push_events(
        &mut self,
        mut bytes: &[u8],
    ) -> Result<Vec<StaticAdapterFrameDecodeEvent>, StaticAdapterFrameDecoderStateError> {
        if self.terminal {
            return Err(StaticAdapterFrameDecoderStateError::new(
                StaticAdapterFrameDecodeErrorKind::DecoderRejected,
            ));
        }
        if bytes.len() > self.limits.max_push_bytes {
            return Ok(self.terminal_event(
                Vec::new(),
                StaticAdapterFrameDecodeErrorKind::PushByteLimitExceeded,
            ));
        }

        let mut events = Vec::new();
        let mut decoded_events = 0_usize;
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
                        .expect("static adapter prefix is exactly four bytes"),
                );
                let declared = usize::try_from(declared)
                    .expect("u32 static adapter frame length always fits usize");
                if declared > self.limits.max_frame_bytes {
                    return Ok(self.terminal_event(
                        events,
                        StaticAdapterFrameDecodeErrorKind::OversizedFrame,
                    ));
                }
                self.declared_body_length = Some(declared);
                self.body.clear();
                if self.body.try_reserve_exact(declared).is_err() {
                    return Ok(self.terminal_event(
                        events,
                        StaticAdapterFrameDecodeErrorKind::BodyAllocationFailed,
                    ));
                }
            }

            let declared = self
                .declared_body_length
                .expect("complete static prefix establishes body length");
            let remaining = declared - self.body.len();
            let take = remaining.min(bytes.len());
            self.body.extend_from_slice(&bytes[..take]);
            bytes = &bytes[take..];
            if self.body.len() != declared {
                continue;
            }
            if decoded_events == self.limits.max_decoded_events_per_push {
                return Ok(self.terminal_event(
                    events,
                    StaticAdapterFrameDecodeErrorKind::PushDecodedEventLimitExceeded,
                ));
            }
            match decode_static_adapter_body(&self.body) {
                Ok(candidate) => {
                    events.push(StaticAdapterFrameDecodeEvent::Decoded(Box::new(candidate)));
                    decoded_events += 1;
                }
                Err(kind) => return Ok(self.terminal_event(events, kind)),
            }
            self.prefix.clear();
            self.declared_body_length = None;
            self.body.clear();
        }
        Ok(events)
    }

    /// Declares end-of-input and turns the decoder terminal.  Incomplete data
    /// becomes a typed event; a clean finish returns no event.
    pub fn finish_event(
        &mut self,
    ) -> Result<Option<StaticAdapterFrameDecodeEvent>, StaticAdapterFrameDecoderStateError> {
        if self.terminal {
            return Err(StaticAdapterFrameDecoderStateError::new(
                StaticAdapterFrameDecodeErrorKind::DecoderRejected,
            ));
        }
        self.terminal = true;
        if self.declared_body_length.is_some() {
            return Ok(Some(StaticAdapterFrameDecodeEvent::Rejected(
                StaticAdapterFrameDecodeErrorKind::TruncatedBody,
            )));
        }
        if !self.prefix.is_empty() {
            return Ok(Some(StaticAdapterFrameDecodeEvent::Rejected(
                StaticAdapterFrameDecodeErrorKind::TruncatedPrefix,
            )));
        }
        Ok(None)
    }

    fn terminal_event(
        &mut self,
        mut events: Vec<StaticAdapterFrameDecodeEvent>,
        kind: StaticAdapterFrameDecodeErrorKind,
    ) -> Vec<StaticAdapterFrameDecodeEvent> {
        self.terminal = true;
        events.push(StaticAdapterFrameDecodeEvent::Rejected(kind));
        events
    }
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct PrivateStaticAdapterWireEnvelopeOut<'a> {
    marker: &'static str,
    version: u16,
    carrier: &'a Sys5I3AdapterWireSnapshot,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateStaticAdapterWireHeaderIn {
    marker: String,
    version: u16,
    carrier: Box<serde_json::value::RawValue>,
}

fn decode_static_adapter_body(
    body: &[u8],
) -> Result<UntrustedDecodedStaticAdapterCarrier, StaticAdapterFrameDecodeErrorKind> {
    let header = serde_json::from_slice::<PrivateStaticAdapterWireHeaderIn>(body)
        .map_err(|_| StaticAdapterFrameDecodeErrorKind::MalformedPayload)?;
    if header.marker != PRIVATE_STATIC_ADAPTER_WIRE_MARKER {
        return Err(StaticAdapterFrameDecodeErrorKind::MarkerMismatch);
    }
    if header.version != PRIVATE_STATIC_ADAPTER_WIRE_VERSION {
        return Err(StaticAdapterFrameDecodeErrorKind::UnknownVersion);
    }
    let carrier = serde_json::from_str::<Sys5I3AdapterWireSnapshot>(header.carrier.get())
        .map_err(|_| StaticAdapterFrameDecodeErrorKind::MalformedPayload)?;
    Ok(UntrustedDecodedStaticAdapterCarrier::from_wire_snapshot(
        carrier,
    ))
}
