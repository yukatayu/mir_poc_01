//! Reference-only observer rendering and defensive evidence scanning.

use std::{error::Error, fmt};

use serde::Serialize;

use crate::EvidenceRow;

/// A structured, reference-only observer rendering. It intentionally keeps the
/// JSON string opaque so callers cannot mutate rows after common validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObserverEvidence(String);

impl ObserverEvidence {
    /// Returns the structured private evidence text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Confirms the renderer produced the fixed private object shape.
    pub fn is_structured(&self) -> bool {
        serde_json::from_str::<serde_json::Value>(&self.0)
            .ok()
            .and_then(|value| {
                value.as_object().and_then(|object| {
                    (object.get("schema")
                        == Some(&serde_json::Value::String(
                            "mirrorea-i3-probe-observer-evidence-v1".to_string(),
                        ))
                        && object.get("rows").is_some_and(serde_json::Value::is_array))
                    .then_some(())
                })
            })
            .is_some()
    }
}

/// Observer-safety rejection kinds. Each one is expressed without repeating
/// the unsafe material in an error string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObserverEvidenceErrorKind {
    /// Private key material or a private-key field is present.
    RawKeyMaterial,
    /// Certificate PEM/DER material or a certificate field is present.
    RawCertificateMaterial,
    /// Raw source text is present.
    RawSourceText,
    /// A host-local source path is present.
    HostSourcePath,
    /// Raw private payload bytes or payload field are present.
    RawPayload,
    /// Capability material is present.
    CapabilityMaterial,
    /// Witness material is present.
    WitnessMaterial,
    /// Private state material is present.
    PrivateState,
    /// Transport metadata was asserted to be semantic authority.
    TransportAuthorityClaim,
    /// The input is not a structured JSON evidence object.
    MalformedEvidence,
}

/// A typed observer-evidence error that does not echo unsafe content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObserverEvidenceError {
    kind: ObserverEvidenceErrorKind,
}

impl ObserverEvidenceError {
    const fn new(kind: ObserverEvidenceErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the evidence policy violation class.
    pub const fn kind(&self) -> ObserverEvidenceErrorKind {
        self.kind
    }
}

impl fmt::Display for ObserverEvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I3-0 observer evidence violates its redaction policy")
    }
}

impl Error for ObserverEvidenceError {}

#[derive(Serialize)]
struct ObserverEnvelope<'a> {
    schema: &'static str,
    rows: &'a [EvidenceRow],
}

/// Renders common rows as structured reference-only evidence and scans the
/// rendered bytes before exposing them to an observer.
pub fn render_observer_safe_evidence(
    rows: &[EvidenceRow],
) -> Result<ObserverEvidence, ObserverEvidenceError> {
    let rendered = serde_json::to_string(&ObserverEnvelope {
        schema: "mirrorea-i3-probe-observer-evidence-v1",
        rows,
    })
    .map_err(|_| ObserverEvidenceError::new(ObserverEvidenceErrorKind::MalformedEvidence))?;
    validate_observer_safe_evidence(&rendered)?;
    Ok(ObserverEvidence(rendered))
}

/// Rejects raw secret/source/state material and transport-as-authority claims
/// from a prospective observer rendering.
pub fn validate_observer_safe_evidence(input: &str) -> Result<(), ObserverEvidenceError> {
    let value = serde_json::from_str::<serde_json::Value>(input)
        .map_err(|_| ObserverEvidenceError::new(ObserverEvidenceErrorKind::MalformedEvidence))?;
    scan_value(&value)
}

fn scan_value(value: &serde_json::Value) -> Result<(), ObserverEvidenceError> {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                scan_value(value)?;
            }
        }
        serde_json::Value::Object(values) => {
            for (key, value) in values {
                reject_key_value(key, value)?;
                scan_value(value)?;
            }
        }
        serde_json::Value::String(value) => reject_string(value)?,
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {}
    }
    Ok(())
}

fn reject_key_value(key: &str, value: &serde_json::Value) -> Result<(), ObserverEvidenceError> {
    let normalized = key.to_ascii_lowercase();
    let kind = match normalized.as_str() {
        "privatekey" | "private_key" | "private_key_der" | "key_der" => {
            Some(ObserverEvidenceErrorKind::RawKeyMaterial)
        }
        "certificate" | "certificate_der" | "cert_der" => {
            Some(ObserverEvidenceErrorKind::RawCertificateMaterial)
        }
        "source_text" => Some(ObserverEvidenceErrorKind::RawSourceText),
        "source_path" | "host_source_path" => Some(ObserverEvidenceErrorKind::HostSourcePath),
        "payload" | "raw_payload" => Some(ObserverEvidenceErrorKind::RawPayload),
        "capability" | "capability_ref" => Some(ObserverEvidenceErrorKind::CapabilityMaterial),
        "witness" | "witness_ref" => Some(ObserverEvidenceErrorKind::WitnessMaterial),
        "private_state" | "state_snapshot" => Some(ObserverEvidenceErrorKind::PrivateState),
        "transport_metadata_used_as_authority" if value == &serde_json::Value::Bool(true) => {
            Some(ObserverEvidenceErrorKind::TransportAuthorityClaim)
        }
        _ => None,
    };
    kind.map_or(Ok(()), |kind| Err(ObserverEvidenceError::new(kind)))
}

fn reject_string(value: &str) -> Result<(), ObserverEvidenceError> {
    let normalized = value.to_ascii_lowercase();
    let kind = if normalized.contains("-----begin certificate-----") {
        Some(ObserverEvidenceErrorKind::RawCertificateMaterial)
    } else if normalized.contains("-----begin private key-----") {
        Some(ObserverEvidenceErrorKind::RawKeyMaterial)
    } else {
        None
    };
    kind.map_or(Ok(()), |kind| Err(ObserverEvidenceError::new(kind)))
}
