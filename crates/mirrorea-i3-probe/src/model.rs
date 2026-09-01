//! Exact retained-contract bindings for the private I3-0 probe.

use std::{error::Error, fmt};

use mir_runtime::sys5_local_slice::Sys5I3ProbeCarrierContract;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const REQUEST_ID_DOMAIN: &[u8] = b"mirrorea/i3-0/semantic-request/v2\0";

/// An explicit semantic invocation seed. It is neither a transport occurrence
/// nor an authority-bearing value.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SemanticRequestSeed(String);

impl SemanticRequestSeed {
    /// Creates a candidate semantic invocation seed. The binding boundary
    /// validates its intentionally small observer-safe alphabet.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the seed only for private probe evidence.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_well_formed(&self) -> bool {
        !self.0.is_empty()
            && self.0.len() <= 128
            && self
                .0
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    }
}

/// A semantic request identity derived from an exact retained contract and an
/// explicit invocation seed, never from a network occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestIdentity {
    value: String,
    retained_contract_fingerprint: String,
}

impl Serialize for RequestIdentity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl<'de> Deserialize<'de> for RequestIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|value| Self {
            value,
            // Byte-decoded identities are untrusted. The receiver replaces
            // this with the independently retained fingerprint only after
            // exact target-contract admission succeeds.
            retained_contract_fingerprint: String::new(),
        })
    }
}

impl RequestIdentity {
    /// Returns the private, source-contract-bound semantic request ID.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// The exact retained-contract fingerprint bound into this locally
    /// admitted request identity. It is reference-only evidence, never an
    /// authority-bearing value or a transport occurrence.
    pub fn retained_contract_fingerprint(&self) -> &str {
        &self.retained_contract_fingerprint
    }

    fn from_retained_contract(
        contract: &RetainedCarrierContract,
        seed: &SemanticRequestSeed,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(REQUEST_ID_DOMAIN);
        hasher.update(
            u64::try_from(contract.full_retained_contract_fingerprint.len())
                .expect("private retained contract fingerprint length fits u64")
                .to_le_bytes(),
        );
        hasher.update(contract.full_retained_contract_fingerprint.as_bytes());
        hasher.update(
            u64::try_from(seed.0.len())
                .expect("semantic request seed length fits u64")
                .to_le_bytes(),
        );
        hasher.update(seed.0.as_bytes());
        Self {
            value: format!("i3-0-semantic-request-sha256-v1:{:x}", hasher.finalize()),
            retained_contract_fingerprint: contract.full_retained_contract_fingerprint.clone(),
        }
    }

    fn has_private_shape(&self) -> bool {
        self.value
            .strip_prefix("i3-0-semantic-request-sha256-v1:")
            .is_some_and(|digest| {
                digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
    }
}

/// Binding failure before a semantic carrier exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticRequestBindingErrorKind {
    /// The requested seed could carry arbitrary source, credential, or host
    /// material and is rejected before binding.
    InvalidSemanticRequestSeed,
}

/// A typed private request-binding rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticRequestBindingError {
    kind: SemanticRequestBindingErrorKind,
}

impl SemanticRequestBindingError {
    const fn new(kind: SemanticRequestBindingErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the private rejection classification.
    pub const fn kind(&self) -> SemanticRequestBindingErrorKind {
        self.kind
    }
}

impl fmt::Display for SemanticRequestBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid private semantic request seed")
    }
}

impl Error for SemanticRequestBindingError {}

/// A typed admission failure for a decoded, untrusted private carrier.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SemanticAdmissionErrorKind {
    /// The complete retained owner-request contract fingerprint differs from
    /// the independently retained verifier before cache or handler access.
    RetainedContractFingerprintMismatch,
    /// The checked program does not match the independently retained contract.
    CheckedProgramReferenceMismatch,
    /// The operation identity does not match the independently retained contract.
    OperationMismatch,
    /// The generated edge kind does not match the independently retained contract.
    EdgeKindMismatch,
    /// The carrier lifecycle kind does not match the independently retained contract.
    LifecycleKindMismatch,
    /// The generated source locus does not match the independently retained contract.
    SourceLocusMismatch,
    /// The generated target locus does not match the independently retained contract.
    TargetLocusMismatch,
    /// The source provenance does not match the independently retained contract.
    SourceReferenceMismatch,
    /// Checked Core provenance does not match the independently retained contract.
    CoreReferenceMismatch,
    /// Source artifact provenance does not match the independently retained contract.
    SourceArtifactReferenceMismatch,
    /// Target artifact provenance does not match the independently retained contract.
    TargetArtifactReferenceMismatch,
    /// Generated edge provenance does not match the independently retained contract.
    EdgeReferenceMismatch,
    /// Declared failure names do not match the independently retained contract.
    DeclaredFailureMismatch,
    /// Effect-kind names do not match the independently retained contract.
    EffectKindMismatch,
    /// Required occurrence slots do not match the independently retained contract.
    OccurrenceSlotMismatch,
    /// Linked-request or typed-outcome requirements do not match the retained contract.
    RequestOutcomeRequirementMismatch,
    /// Authority categories or category requirements do not match the retained contract.
    AuthorityRequirementMismatch,
    /// Redaction requirements do not match the retained contract.
    RedactionMismatch,
    /// Checked-Core provenance status does not match the retained contract.
    CheckedCoreBindingMismatch,
    /// An incoming carrier tried to claim a different authority-transfer property.
    AuthorityTransferMismatch,
    /// The seed is not valid private semantic invocation data.
    InvalidSemanticRequestSeed,
    /// The identity does not recompute from the exact retained contract and seed.
    RequestBindingMismatch,
}

/// A typed private receiver admission error. It intentionally contains no
/// untrusted payload, source text, authority, credential, witness, or network
/// occurrence material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticAdmissionError {
    kind: SemanticAdmissionErrorKind,
}

impl SemanticAdmissionError {
    const fn new(kind: SemanticAdmissionErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the fail-closed semantic admission classification.
    pub const fn kind(&self) -> SemanticAdmissionErrorKind {
        self.kind
    }
}

impl fmt::Display for SemanticAdmissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("private I3-0 semantic carrier admission rejected")
    }
}

impl Error for SemanticAdmissionError {}

/// Exactly the reference-only fields supplied by the retained SYS-5 façade.
/// This is intentionally a private duplication for byte transport; every
/// decoded field is compared against an independently retained instance.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct RetainedCarrierContract {
    #[serde(rename = "retained_contract_fingerprint")]
    full_retained_contract_fingerprint: String,
    checked_program_ref: String,
    operation: String,
    edge_kind: String,
    lifecycle_kind: String,
    source_locus: String,
    target_locus: String,
    logical_source_path: String,
    source_start: u64,
    source_end: u64,
    source_start_line: u32,
    source_start_column: u32,
    source_end_line: u32,
    source_end_column: u32,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    declared_failure_names: Vec<String>,
    effect_kind_names: Vec<String>,
    required_occurrence_slot_names: Vec<String>,
    linked_request_identity: bool,
    typed_outcome: bool,
    authority_category_names: Vec<String>,
    requires_membership_epoch_and_incarnation: bool,
    requires_capability_and_witness_refs: bool,
    reference_only_redaction: bool,
    checked_core_bound: bool,
    transfers_authority: bool,
}

impl RetainedCarrierContract {
    fn from_sys5(contract: &Sys5I3ProbeCarrierContract) -> Self {
        let source_span = contract.source_span();
        let authority = contract.authority_requirements();
        Self {
            full_retained_contract_fingerprint: contract
                .full_retained_contract_fingerprint()
                .to_string(),
            checked_program_ref: contract.checked_program_ref().to_string(),
            operation: contract.operation_id().to_string(),
            edge_kind: contract.edge_kind().to_string(),
            lifecycle_kind: contract.lifecycle_kind().to_string(),
            source_locus: contract.source_locus().to_string(),
            target_locus: contract.target_locus().to_string(),
            logical_source_path: contract.logical_source_path().to_string(),
            source_start: source_span.start,
            source_end: source_span.end,
            source_start_line: source_span.start_line,
            source_start_column: source_span.start_column,
            source_end_line: source_span.end_line,
            source_end_column: source_span.end_column,
            source_ref: contract.source_ref().to_string(),
            core_ref: contract.core_ref().to_string(),
            source_artifact_ref: contract.source_artifact_ref().to_string(),
            target_artifact_ref: contract.target_artifact_ref().to_string(),
            edge_ref: contract.edge_ref().to_string(),
            declared_failure_names: contract.declared_failure_names().to_vec(),
            effect_kind_names: contract.effect_kind_names().to_vec(),
            required_occurrence_slot_names: contract.required_occurrence_slot_names().to_vec(),
            linked_request_identity: contract.requires_linked_request_identity(),
            typed_outcome: contract.requires_typed_outcome(),
            authority_category_names: authority.category_names().to_vec(),
            requires_membership_epoch_and_incarnation: authority
                .requires_membership_epoch_and_incarnation(),
            requires_capability_and_witness_refs: authority.requires_capability_and_witness_refs(),
            reference_only_redaction: matches!(
                contract.redaction(),
                mir_runtime::sys5_local_slice::Sys5I3ProbeRedaction::ReferenceOnly
            ),
            checked_core_bound: contract.checked_core_bound(),
            transfers_authority: contract.transfers_authority(),
        }
    }

    fn mismatch_kind(&self, candidate: &Self) -> Option<SemanticAdmissionErrorKind> {
        if candidate.full_retained_contract_fingerprint != self.full_retained_contract_fingerprint {
            return Some(SemanticAdmissionErrorKind::RetainedContractFingerprintMismatch);
        }
        if candidate.checked_program_ref != self.checked_program_ref {
            return Some(SemanticAdmissionErrorKind::CheckedProgramReferenceMismatch);
        }
        if candidate.operation != self.operation {
            return Some(SemanticAdmissionErrorKind::OperationMismatch);
        }
        if candidate.edge_kind != self.edge_kind {
            return Some(SemanticAdmissionErrorKind::EdgeKindMismatch);
        }
        if candidate.lifecycle_kind != self.lifecycle_kind {
            return Some(SemanticAdmissionErrorKind::LifecycleKindMismatch);
        }
        if candidate.source_locus != self.source_locus {
            return Some(SemanticAdmissionErrorKind::SourceLocusMismatch);
        }
        if candidate.target_locus != self.target_locus {
            return Some(SemanticAdmissionErrorKind::TargetLocusMismatch);
        }
        if candidate.logical_source_path != self.logical_source_path
            || candidate.source_start != self.source_start
            || candidate.source_end != self.source_end
            || candidate.source_start_line != self.source_start_line
            || candidate.source_start_column != self.source_start_column
            || candidate.source_end_line != self.source_end_line
            || candidate.source_end_column != self.source_end_column
            || candidate.source_ref != self.source_ref
        {
            return Some(SemanticAdmissionErrorKind::SourceReferenceMismatch);
        }
        if candidate.core_ref != self.core_ref {
            return Some(SemanticAdmissionErrorKind::CoreReferenceMismatch);
        }
        if candidate.source_artifact_ref != self.source_artifact_ref {
            return Some(SemanticAdmissionErrorKind::SourceArtifactReferenceMismatch);
        }
        if candidate.target_artifact_ref != self.target_artifact_ref {
            return Some(SemanticAdmissionErrorKind::TargetArtifactReferenceMismatch);
        }
        if candidate.edge_ref != self.edge_ref {
            return Some(SemanticAdmissionErrorKind::EdgeReferenceMismatch);
        }
        if candidate.declared_failure_names != self.declared_failure_names {
            return Some(SemanticAdmissionErrorKind::DeclaredFailureMismatch);
        }
        if candidate.effect_kind_names != self.effect_kind_names {
            return Some(SemanticAdmissionErrorKind::EffectKindMismatch);
        }
        if candidate.required_occurrence_slot_names != self.required_occurrence_slot_names {
            return Some(SemanticAdmissionErrorKind::OccurrenceSlotMismatch);
        }
        if candidate.linked_request_identity != self.linked_request_identity
            || candidate.typed_outcome != self.typed_outcome
        {
            return Some(SemanticAdmissionErrorKind::RequestOutcomeRequirementMismatch);
        }
        if candidate.authority_category_names != self.authority_category_names
            || candidate.requires_membership_epoch_and_incarnation
                != self.requires_membership_epoch_and_incarnation
            || candidate.requires_capability_and_witness_refs
                != self.requires_capability_and_witness_refs
        {
            return Some(SemanticAdmissionErrorKind::AuthorityRequirementMismatch);
        }
        if candidate.reference_only_redaction != self.reference_only_redaction {
            return Some(SemanticAdmissionErrorKind::RedactionMismatch);
        }
        if candidate.checked_core_bound != self.checked_core_bound {
            return Some(SemanticAdmissionErrorKind::CheckedCoreBindingMismatch);
        }
        if candidate.transfers_authority != self.transfers_authority {
            return Some(SemanticAdmissionErrorKind::AuthorityTransferMismatch);
        }
        None
    }
}

/// A private semantic carrier that has a locally retained-contract binding.
/// It deliberately does not implement `Deserialize`; byte input must become
/// an [`UntrustedDecodedCarrier`] and pass receiver revalidation first.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticCarrier {
    #[serde(flatten)]
    contract: RetainedCarrierContract,
    semantic_request_seed: SemanticRequestSeed,
    request_identity: RequestIdentity,
}

impl SemanticCarrier {
    fn from_binding(contract: RetainedCarrierContract, seed: SemanticRequestSeed) -> Self {
        let request_identity = RequestIdentity::from_retained_contract(&contract, &seed);
        Self {
            contract,
            semantic_request_seed: seed,
            request_identity,
        }
    }

    /// The exact retained generated edge reference.
    pub fn edge_ref(&self) -> &str {
        &self.contract.edge_ref
    }

    /// The exact retained target locus.
    pub fn target_locus(&self) -> &str {
        &self.contract.target_locus
    }

    /// The semantic request identity, independent of any network occurrence.
    pub fn request_identity(&self) -> &RequestIdentity {
        &self.request_identity
    }

    /// The full retained contract fingerprint bound into the request identity.
    pub fn retained_contract_fingerprint(&self) -> &str {
        &self.contract.full_retained_contract_fingerprint
    }
}

/// Byte-decoded private carrier data. This is not semantic admission and has
/// no authority, transport occurrence, or trusted verdict attached.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UntrustedDecodedCarrier {
    #[serde(flatten)]
    contract: RetainedCarrierContract,
    semantic_request_seed: SemanticRequestSeed,
    request_identity: RequestIdentity,
}

impl UntrustedDecodedCarrier {
    pub(crate) fn from_json(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

/// One retained generated edge, used only to make and verify private probe
/// carriers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBoundEdge {
    contract: RetainedCarrierContract,
}

impl SourceBoundEdge {
    pub(crate) fn from_sys5(contract: Sys5I3ProbeCarrierContract) -> Self {
        Self {
            contract: RetainedCarrierContract::from_sys5(&contract),
        }
    }

    /// The checked operation identifier.
    pub fn operation(&self) -> &str {
        &self.contract.operation
    }

    /// The generated source locus.
    pub fn source_locus(&self) -> &str {
        &self.contract.source_locus
    }

    /// The generated target locus.
    pub fn target_locus(&self) -> &str {
        &self.contract.target_locus
    }

    /// The retained generated edge kind.
    pub fn edge_kind(&self) -> &str {
        &self.contract.edge_kind
    }

    /// The retained carrier lifecycle kind.
    pub fn lifecycle_kind(&self) -> &str {
        &self.contract.lifecycle_kind
    }

    /// The retained observer-safe source reference.
    pub fn source_ref(&self) -> &str {
        &self.contract.source_ref
    }

    /// The retained checked Core reference.
    pub fn core_ref(&self) -> &str {
        &self.contract.core_ref
    }

    /// The retained generated source artifact reference.
    pub fn source_artifact_ref(&self) -> &str {
        &self.contract.source_artifact_ref
    }

    /// The retained generated target artifact reference.
    pub fn target_artifact_ref(&self) -> &str {
        &self.contract.target_artifact_ref
    }

    /// The retained generated edge reference.
    pub fn edge_ref(&self) -> &str {
        &self.contract.edge_ref
    }

    /// The retained checked-program reference.
    pub fn program_ref(&self) -> &str {
        &self.contract.checked_program_ref
    }

    /// Domain-separated reference to the complete retained owner-request
    /// contract. It binds a semantic request/cache key but never carries
    /// authority or a transport session identity.
    pub fn retained_contract_fingerprint(&self) -> &str {
        &self.contract.full_retained_contract_fingerprint
    }

    /// Exact checked declared-failure names.
    pub fn declared_failure_names(&self) -> &[String] {
        &self.contract.declared_failure_names
    }

    /// Exact checked effect-kind names.
    pub fn effect_kind_names(&self) -> &[String] {
        &self.contract.effect_kind_names
    }

    /// Exact required semantic occurrence slots.
    pub fn required_occurrence_slot_names(&self) -> &[String] {
        &self.contract.required_occurrence_slot_names
    }

    /// Whether membership epoch/incarnation is required by the retained contract.
    pub const fn requires_membership_epoch_and_incarnation(&self) -> bool {
        self.contract.requires_membership_epoch_and_incarnation
    }

    /// Whether capability and witness references are required by the retained contract.
    pub const fn requires_capability_and_witness_refs(&self) -> bool {
        self.contract.requires_capability_and_witness_refs
    }

    /// Whether the retained contract is checked-Core-bound.
    pub const fn checked_core_bound(&self) -> bool {
        self.contract.checked_core_bound
    }

    /// Whether the retained contract is reference-only redacted.
    pub const fn reference_only_redaction(&self) -> bool {
        self.contract.reference_only_redaction
    }

    /// Whether the retained contract transfers authority.
    pub const fn transfers_authority(&self) -> bool {
        self.contract.transfers_authority
    }

    /// Binds one explicit semantic invocation to this exact retained contract.
    pub fn bind_semantic_request(
        &self,
        seed: SemanticRequestSeed,
    ) -> Result<SemanticCarrier, SemanticRequestBindingError> {
        if !seed.is_well_formed() {
            return Err(SemanticRequestBindingError::new(
                SemanticRequestBindingErrorKind::InvalidSemanticRequestSeed,
            ));
        }
        Ok(SemanticCarrier::from_binding(self.contract.clone(), seed))
    }

    /// Revalidates every byte-decoded snapshot field against the independently
    /// retained checked contract before creating a non-deserializable carrier.
    pub fn admit_untrusted_candidate(
        &self,
        candidate: UntrustedDecodedCarrier,
    ) -> Result<SemanticCarrier, SemanticAdmissionError> {
        if let Some(kind) = self.contract.mismatch_kind(&candidate.contract) {
            return Err(SemanticAdmissionError::new(kind));
        }
        if !candidate.semantic_request_seed.is_well_formed() {
            return Err(SemanticAdmissionError::new(
                SemanticAdmissionErrorKind::InvalidSemanticRequestSeed,
            ));
        }
        let expected = RequestIdentity::from_retained_contract(
            &self.contract,
            &candidate.semantic_request_seed,
        );
        if !candidate.request_identity.has_private_shape()
            || candidate.request_identity.value != expected.value
        {
            return Err(SemanticAdmissionError::new(
                SemanticAdmissionErrorKind::RequestBindingMismatch,
            ));
        }
        Ok(SemanticCarrier {
            contract: candidate.contract,
            semantic_request_seed: candidate.semantic_request_seed,
            request_identity: expected,
        })
    }
}

/// The finite source-derived edge set for a private I3-0 probe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceBoundProbe {
    program_ref: String,
    owner_request_edges: Vec<SourceBoundEdge>,
}

impl SourceBoundProbe {
    pub(crate) fn new(program_ref: String, owner_request_edges: Vec<SourceBoundEdge>) -> Self {
        Self {
            program_ref,
            owner_request_edges,
        }
    }

    /// Checked-program provenance shared by every returned edge.
    pub fn program_ref(&self) -> &str {
        &self.program_ref
    }

    /// Selects one retained owner-request edge by checked operation ID.
    pub fn owner_request_edge(&self, operation: &str) -> Option<&SourceBoundEdge> {
        self.owner_request_edges
            .iter()
            .find(|edge| edge.operation() == operation)
    }
}
