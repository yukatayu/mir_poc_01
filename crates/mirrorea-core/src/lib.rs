#![doc = r#"
# mirrorea-core

Minimal Mirrorea runtime substrate carriers.

This crate no longer acts as a pure placeholder. The current first tranche owns
pure carrier shapes and invariant helpers for:

- `LayerSignature` inventory rows
- `PrincipalClaim`
- `AuthEvidence`
- `MessageEnvelope`

These are current repo-local carriers for fabric/report inventory only. They are
not the final public transport ABI, viewer contract, hot-plug runtime, or
projection IR.
"#]

mod error;
mod fabric;
mod layer;

pub use error::MirroreaCoreError;
pub use fabric::{
    AuthEvidence, MessageEnvelope, PrincipalClaim, auth_evidence_lanes, message_envelope_lanes,
};
pub use layer::{LayerSignature, insert_layer_signature, layer_signature_lanes};

pub fn crate_name() -> &'static str {
    "mirrorea_core"
}
