//! Experimental SYS-5 local-slice build/project and local-runtime facade.
//!
//! The facade accepts an ordinary Surface v0 source, checks it once, derives
//! the exact declared logical-locus inventory, and summarizes the resulting
//! SYS-3 projection.  A prepared finite admission can subsequently start the
//! bounded ST local runtime.  It is deliberately experimental: this module is
//! neither a public compatibility, ABI, nor wire-format commitment.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt,
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedEvaluationKind, CheckedSurfaceV0, EffectKind, GeneratedObligationKind,
        ResidualObligationKind, StaticProjectionDesignatedInputReceiptUseFacts,
        StaticProjectionDesignatedInputRequestFacts, StaticProjectionFacts,
        StaticProjectionTypedStateReadFacts, check_and_elaborate_surface_v0,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    m9_auth_verification::{
        M9FiniteLocalAdmissionCandidate, M9FiniteLocalAdmissionFact, M9RuntimeExecutionSeam,
    },
    sys3_projection::{
        BackendEligibility, BackendProfile, CarrierFrontierKind, CarrierLifecycleKind,
        CarrierOccurrenceSlotKind, CarrierProvenanceKind, CommunicationEdgeKind,
        DeclaredLogicalTopology, GlobalProjectionResult,
        I3AdapterCarrierStaticAuthorityRequirementRow, I3AdapterCarrierStaticFacts,
        I3AdapterCarrierStaticVariant, ProjectedOperationFragmentKind, RuntimeSeamRequirementKind,
        SeamAuthorityKind, project_checked_core,
    },
    sys4_dispatch::{
        ExternalAction, FabricProgram, LocalFabric, ObserverSafeM9SemanticRowSets,
        ObserverSafeM9Summary, RelationPublicationFailureDisposition, RuntimeValue,
        SealedFabricAdmission, SourceAction, Sys4CheckedPatchCandidate, Sys4DispatchDiagnostics,
        Sys4InitialStateSeed, Sys4LocalCut, Sys4PatchDiagnosticKind, Sys4PatchOutcome,
        Sys4PatchVerdict, Sys4RelationEndpointReceipt,
    },
};

const PROFILE_NAME: &str = "sys5-local-slice";
const PROFILE_STATUS: &str = "provisional-no-compatibility-promise";
const OBSERVER_SAFETY: &str = "observer-safe-no-raw-authority-capability-witness-payload";
const CHECKED_PROGRAM_REF_DOMAIN: &[u8] = b"mirrorea/sys5/checked-program-ref/v1\0";
const SEALED_INVENTORY_REF_DOMAIN: &[u8] = b"mirrorea/sys5/sealed-inventory-ref/v1\0";
const DEBUG_PATH_REF_DOMAIN: &[u8] = b"mirrorea/sys5/debug-logical-path-ref/v1\0";
const RELATION_OBSERVER_REF_DOMAIN: &[u8] = b"mirrorea/sys5/relation-observer-ref/v1\0";
const LOCAL_CUT_REF_DOMAIN: &[u8] = b"mirrorea/sys5/local-cut-ref/v1\0";
const PATCH_FRONTIER_REF_DOMAIN: &[u8] = b"mirrorea/sys5/patch-frontier-ref/v1\0";
const LIFECYCLE_OCCURRENCE_REF_DOMAIN: &[u8] = b"mirrorea/sys5/lifecycle-occurrence-ref/v1\0";
const I3_PROBE_OWNER_PRINCIPAL_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-probe-owner-principal-ref/v1\0";
const I3_PROBE_FULL_CONTRACT_FINGERPRINT_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-probe-carrier-contract/v1\0";
const I3_ADAPTER_OWNER_PRINCIPAL_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-owner-principal-ref/v1\0";
const I3_ADAPTER_DESIGNATED_READ_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-designated-read-ref/v1\0";
const I3_ADAPTER_REQUESTER_SITE_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-designated-requester-site-ref/v1\0";
const I3_ADAPTER_AUTHORITY_ORIGIN_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-designated-authority-origin-ref/v1\0";
const I3_ADAPTER_DESIGNATED_REQUEST_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-designated-request-ref/v1\0";
const I3_ADAPTER_DESIGNATED_RECEIPT_USE_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-designated-receipt-use-ref/v1\0";
const I3_ADAPTER_RESULT_VERSION_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-result-version-ref/v1\0";
const I3_ADAPTER_INPUT_FRONTIER_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-input-frontier-ref/v1\0";
const I3_ADAPTER_RESULT_FRONTIER_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-result-frontier-ref/v1\0";
const I3_ADAPTER_OBSERVATION_POLICY_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-observation-policy-ref/v1\0";
const I3_ADAPTER_POLICY_STAMP_REF_DOMAIN: &[u8] = b"mirrorea/sys5/i3-adapter-policy-stamp-ref/v1\0";
const I3_ADAPTER_FIELD_INVENTORY_REF_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-field-inventory-ref/v1\0";
const I3_ADAPTER_FULL_CONTRACT_FINGERPRINT_DOMAIN: &[u8] =
    b"mirrorea/sys5/i3-adapter-carrier-contract/v1\0";

/// Ordinary source supplied directly to the provisional build/project facade.
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5SourceInput {
    logical_source_path: String,
    source_text: String,
}

impl fmt::Debug for Sys5SourceInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sys5SourceInput")
            .field(
                "logical_path_ref",
                &debug_path_ref(&self.logical_source_path),
            )
            .field("source_byte_count", &self.source_text.len())
            .field("status", &"redacted-inline-source")
            .finish()
    }
}

impl Sys5SourceInput {
    /// Constructs an inline source input.  `logical_source_path` is retained
    /// only as caller-provided logical provenance; no host path is resolved.
    pub fn inline(logical_source_path: impl Into<String>, source_text: impl Into<String>) -> Self {
        Self {
            logical_source_path: logical_source_path.into(),
            source_text: source_text.into(),
        }
    }
}

/// Build/project failure without exposing a host filesystem location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sys5LocalSliceError {
    InvalidLogicalSourcePath,
    SurfaceCheckFailed { diagnostic_code: &'static str },
    ProjectionFailed,
}

impl fmt::Display for Sys5LocalSliceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLogicalSourcePath => "invalid logical source path",
            Self::SurfaceCheckFailed { diagnostic_code } => {
                return write!(
                    formatter,
                    "Surface v0 check/elaboration failed: {diagnostic_code}"
                );
            }
            Self::ProjectionFailed => "checked Core projection failed",
        })
    }
}

impl Error for Sys5LocalSliceError {}

/// Why a doc-hidden I3 probe could not obtain exactly one retained carrier
/// contract.  This is projection evidence only: it is not a runtime admission
/// or transport error vocabulary.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProbeFacadeErrorKind {
    /// No generated edge retained by the checked projection has the requested
    /// opaque edge reference.
    UnknownEdgeRef,
    /// The retained projection is internally inconsistent because an opaque
    /// edge reference selected more than one generated edge.
    NonUniqueEdgeRef,
    /// The edge and its retained `CarrierContract` disagree about immutable
    /// checked-Core provenance.
    CarrierContractMismatch,
    /// This I3-0 canary exposes only exact retained owner-request contracts.
    NotOwnerRequest,
    /// The selected generated edge is outside the closed I3-1 static carrier
    /// algebra.  In particular, `AbsoluteValueStream` cannot produce a
    /// snapshot through a default or compatibility path.
    NotAcceptedCarrierFamily,
}

/// A reference-only failure returned by the doc-hidden I3 probe façade.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ProbeFacadeError {
    kind: Sys5I3ProbeFacadeErrorKind,
}

impl Sys5I3ProbeFacadeError {
    fn new(kind: Sys5I3ProbeFacadeErrorKind) -> Self {
        Self { kind }
    }

    /// Classifies the lookup failure without exposing source, route, payload,
    /// authority, capability, witness, mailbox, or transport material.
    pub const fn kind(&self) -> Sys5I3ProbeFacadeErrorKind {
        self.kind
    }
}

impl fmt::Display for Sys5I3ProbeFacadeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            Sys5I3ProbeFacadeErrorKind::UnknownEdgeRef => {
                "unknown retained generated edge reference"
            }
            Sys5I3ProbeFacadeErrorKind::NonUniqueEdgeRef => {
                "non-unique retained generated edge reference"
            }
            Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch => {
                "retained generated edge and carrier contract provenance mismatch"
            }
            Sys5I3ProbeFacadeErrorKind::NotOwnerRequest => {
                "I3 probe façade accepts retained owner-request contracts only"
            }
            Sys5I3ProbeFacadeErrorKind::NotAcceptedCarrierFamily => {
                "generated carrier family is outside the closed I3 adapter algebra"
            }
        })
    }
}

impl Error for Sys5I3ProbeFacadeError {}

/// The only redaction policy exposed by the doc-hidden I3 probe façade.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProbeRedaction {
    /// The snapshot contains only checked/source/projection references and
    /// finite category names.  It never contains source text, payloads, or
    /// authority-bearing values.
    ReferenceOnly,
}

/// Reference-only authority requirements retained by one generated carrier.
/// Categories are names, not membership/capability/witness values or lineage.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProbeAuthorityRequirements {
    category_names: Vec<String>,
    requires_membership_epoch_and_incarnation: bool,
    requires_capability_and_witness_refs: bool,
}

impl Sys5I3ProbeAuthorityRequirements {
    /// Finite, stable category names retained by the selected contract.
    pub fn category_names(&self) -> &[String] {
        &self.category_names
    }

    /// Whether the carrier requires a membership epoch/incarnation category.
    pub const fn requires_membership_epoch_and_incarnation(&self) -> bool {
        self.requires_membership_epoch_and_incarnation
    }

    /// Whether the carrier requires both a capability-reference category and
    /// a witness-reference category.
    pub const fn requires_capability_and_witness_refs(&self) -> bool {
        self.requires_capability_and_witness_refs
    }
}

/// One ordered, reference-only runtime-seam authority requirement row retained
/// by the static I3 adapter facade.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterAuthorityRequirementRow {
    requirement_kind_name: String,
    generated_obligation_present: bool,
    generated_obligation_kind_name: Option<String>,
    generated_obligation_detail_name: Option<String>,
    provenance_name: String,
    authority_category_name: Option<String>,
}

impl Sys5I3AdapterAuthorityRequirementRow {
    pub fn requirement_kind_name(&self) -> &str {
        &self.requirement_kind_name
    }

    pub const fn generated_obligation_present(&self) -> bool {
        self.generated_obligation_present
    }

    pub fn generated_obligation_kind_name(&self) -> Option<&str> {
        self.generated_obligation_kind_name.as_deref()
    }

    pub fn generated_obligation_detail_name(&self) -> Option<&str> {
        self.generated_obligation_detail_name.as_deref()
    }

    pub fn provenance_name(&self) -> &str {
        &self.provenance_name
    }

    pub fn authority_category_name(&self) -> Option<&str> {
        self.authority_category_name.as_deref()
    }
}

/// Exact ordered authority requirements retained by the static I3 adapter
/// facade. Category names and booleans are derived convenience views over the
/// complete row sequence.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterAuthorityRequirements {
    rows: Vec<Sys5I3AdapterAuthorityRequirementRow>,
    category_names: Vec<String>,
    requires_membership_epoch_and_incarnation: bool,
    requires_capability_and_witness_refs: bool,
}

impl Sys5I3AdapterAuthorityRequirements {
    fn from_rows(rows: Vec<Sys5I3AdapterAuthorityRequirementRow>) -> Self {
        let mut category_names = Vec::new();
        for category in rows
            .iter()
            .filter_map(|row| row.authority_category_name.as_deref())
        {
            if !category_names.iter().any(|existing| existing == category) {
                category_names.push(category.to_string());
            }
        }
        let requires_membership_epoch_and_incarnation = category_names.iter().any(|category| {
            matches!(
                category.as_str(),
                "MembershipEpochIncarnation" | "DesignatedResultConsumerMembership"
            )
        });
        let requires_capability_and_witness_refs = category_names.iter().any(|category| {
            matches!(
                category.as_str(),
                "OwnerCapabilityRef"
                    | "ProducerReleaseCapability"
                    | "DesignatedResultConsumerCapability"
            )
        }) && category_names.iter().any(|category| {
            matches!(
                category.as_str(),
                "OwnerWitnessRef" | "ProducerReleaseWitness" | "DesignatedResultConsumerWitness"
            )
        });
        Self {
            rows,
            category_names,
            requires_membership_epoch_and_incarnation,
            requires_capability_and_witness_refs,
        }
    }

    pub fn rows(&self) -> &[Sys5I3AdapterAuthorityRequirementRow] {
        &self.rows
    }

    pub fn category_names(&self) -> &[String] {
        &self.category_names
    }

    pub const fn requires_membership_epoch_and_incarnation(&self) -> bool {
        self.requires_membership_epoch_and_incarnation
    }

    pub const fn requires_capability_and_witness_refs(&self) -> bool {
        self.requires_capability_and_witness_refs
    }
}

/// Reference-only facts unique to the retained owner-request canary.  This is
/// not a generic relation or designated-carrier façade.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProbeOwnerRequestFacts {
    request_template_present: bool,
    request_template_slot_names: Vec<String>,
    origin_principal_ref: String,
    origin_locus_template: String,
    target_owner_locus_template: String,
    frontier_requirement_names: Vec<String>,
    has_no_frontier_contract: bool,
    requires_receipt_consumption_state: bool,
    designated_dependency_present: bool,
    designated_result_details_present: bool,
}

impl Sys5I3ProbeOwnerRequestFacts {
    /// Whether the retained request identity template has its semantic slot.
    pub const fn request_template_present(&self) -> bool {
        self.request_template_present
    }

    /// Retained semantic slot names; they do not identify network occurrences.
    pub fn request_template_slot_names(&self) -> &[String] {
        &self.request_template_slot_names
    }

    /// Opaque digest of the retained origin-principal template, never its raw
    /// value or an authority-bearing credential.
    pub fn origin_principal_ref(&self) -> &str {
        &self.origin_principal_ref
    }

    /// Retained owner-request origin locus template.
    pub fn origin_locus_template(&self) -> &str {
        &self.origin_locus_template
    }

    /// Retained owner-request target-owner locus template.
    pub fn target_owner_locus_template(&self) -> &str {
        &self.target_owner_locus_template
    }

    /// Retained frontier-category names. Owner requests have no frontier.
    pub fn frontier_requirement_names(&self) -> &[String] {
        &self.frontier_requirement_names
    }

    /// Whether the exact retained contract has no frontier requirement.
    pub const fn has_no_frontier_contract(&self) -> bool {
        self.has_no_frontier_contract
    }

    /// Whether the retained carrier requires any frontier category.
    pub const fn requires_any_frontier(&self) -> bool {
        !self.has_no_frontier_contract
    }

    /// Whether receipt consumption is part of this carrier contract.
    pub const fn requires_receipt_consumption_state(&self) -> bool {
        self.requires_receipt_consumption_state
    }

    /// Whether a designated remote-input dependency is retained.
    pub const fn designated_dependency_present(&self) -> bool {
        self.designated_dependency_present
    }

    /// Whether designated-result details are retained.
    pub const fn designated_result_details_present(&self) -> bool {
        self.designated_result_details_present
    }
}

/// An immutable observer-safe snapshot of one exact retained generated
/// `CarrierContract`.  It is doc-hidden I3-0 probe evidence, not a public
/// wire, API, package, or runtime-admission contract.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProbeCarrierContract {
    checked_program_ref: String,
    operation_id: String,
    edge_kind: String,
    lifecycle_kind: String,
    source_locus: String,
    target_locus: String,
    logical_source_path: String,
    source_span: Sys5SourceSpan,
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
    authority_requirements: Sys5I3ProbeAuthorityRequirements,
    redaction: Sys5I3ProbeRedaction,
    checked_core_bound: bool,
    transfers_authority: bool,
    mints_authority_without_source: bool,
    owner_request: Sys5I3ProbeOwnerRequestFacts,
    full_retained_contract_fingerprint: String,
}

impl Sys5I3ProbeCarrierContract {
    /// Opaque reference to the exact retained checked program.
    pub fn checked_program_ref(&self) -> &str {
        &self.checked_program_ref
    }

    /// Operation name retained by the checked carrier identity template.
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Explicit generated communication-edge kind name.
    pub fn edge_kind(&self) -> &str {
        &self.edge_kind
    }

    /// Explicit carrier lifecycle kind name.
    pub fn lifecycle_kind(&self) -> &str {
        &self.lifecycle_kind
    }

    /// Source locus derived by the retained generated edge.
    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }

    /// Target locus derived by the retained generated edge.
    pub fn target_locus(&self) -> &str {
        &self.target_locus
    }

    /// Logical source path; it is never a resolved host path.
    pub fn logical_source_path(&self) -> &str {
        &self.logical_source_path
    }

    /// Source coordinates retained without source text.
    pub const fn source_span(&self) -> Sys5SourceSpan {
        self.source_span
    }

    /// Logical source reference with coordinates and no source text.
    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }

    /// Checked-Core reference retained by both edge and carrier.
    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    /// Exact generated source artifact/fragment reference.
    pub fn source_artifact_ref(&self) -> &str {
        &self.source_artifact_ref
    }

    /// Exact generated target artifact/fragment reference.
    pub fn target_artifact_ref(&self) -> &str {
        &self.target_artifact_ref
    }

    /// Opaque reference of the exact retained generated edge.
    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    /// Declared typed-failure names retained by the checked carrier.
    pub fn declared_failure_names(&self) -> &[String] {
        &self.declared_failure_names
    }

    /// `EffectKind` names retained by the checked carrier effect row.
    pub fn effect_kind_names(&self) -> &[String] {
        &self.effect_kind_names
    }

    /// Required semantic occurrence-slot names retained by the carrier.
    pub fn required_occurrence_slot_names(&self) -> &[String] {
        &self.required_occurrence_slot_names
    }

    /// Whether the carrier requires a linked semantic request identity.
    pub const fn requires_linked_request_identity(&self) -> bool {
        self.linked_request_identity
    }

    /// Whether the carrier requires a typed success-or-declared-failure
    /// outcome.
    pub const fn requires_typed_outcome(&self) -> bool {
        self.typed_outcome
    }

    /// Reference-only authority categories; never values or lineage.
    pub fn authority_requirements(&self) -> &Sys5I3ProbeAuthorityRequirements {
        &self.authority_requirements
    }

    /// The observer redaction policy for this snapshot.
    pub const fn redaction(&self) -> Sys5I3ProbeRedaction {
        self.redaction
    }

    /// Whether the retained carrier proves checked-Core provenance.
    pub const fn checked_core_bound(&self) -> bool {
        self.checked_core_bound
    }

    /// Generated I2 carriers never transfer authority.
    pub const fn transfers_authority(&self) -> bool {
        self.transfers_authority
    }

    /// Whether the retained carrier mints authority without checked source.
    pub const fn mints_authority_without_source(&self) -> bool {
        self.mints_authority_without_source
    }

    /// Domain-separated SHA-256 reference over all semantically applicable
    /// retained owner-request contract fields. This is internal I3-0 evidence,
    /// never a public wire or compatibility digest grammar.
    pub fn full_retained_contract_fingerprint(&self) -> &str {
        &self.full_retained_contract_fingerprint
    }

    /// Owner-request-only facts for this owner-request canary. No other
    /// carrier kind or lifecycle can construct this façade snapshot.
    pub fn owner_request_facts(&self) -> Option<&Sys5I3ProbeOwnerRequestFacts> {
        Some(&self.owner_request)
    }

    /// This façade makes no public wire/API compatibility promise.
    pub const fn public_api_or_wire_contract(&self) -> bool {
        false
    }
}

/// Reference-only facts that are meaningful only for an owner request or its
/// receipt.  The principal is a one-way private reference, never a principal
/// value, credential, capability, or witness.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterOwnerFacts {
    origin_principal_ref: String,
    origin_locus_template: String,
    target_owner_locus_template: String,
}

impl Sys5I3AdapterOwnerFacts {
    pub fn origin_principal_ref(&self) -> &str {
        &self.origin_principal_ref
    }

    pub fn origin_locus_template(&self) -> &str {
        &self.origin_locus_template
    }

    pub fn target_owner_locus_template(&self) -> &str {
        &self.target_owner_locus_template
    }
}

/// Reference-only facts that are meaningful only for a designated input
/// request or receipt.  The typed state read is represented only by an opaque
/// digest reference.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterDesignatedInputFacts {
    dependency_ordinal: usize,
    typed_state_read_ref: String,
    requester_site_ref: String,
    authority_origin_ref: String,
    request_ref: String,
    receipt_use_ref: String,
    designated_evaluator_locus: String,
    source_owner_locus: String,
    frontier_requirement_names: Vec<String>,
}

impl Sys5I3AdapterDesignatedInputFacts {
    pub const fn dependency_ordinal(&self) -> usize {
        self.dependency_ordinal
    }

    pub fn typed_state_read_ref(&self) -> &str {
        &self.typed_state_read_ref
    }

    pub fn requester_site_ref(&self) -> &str {
        &self.requester_site_ref
    }

    pub fn authority_origin_ref(&self) -> &str {
        &self.authority_origin_ref
    }

    pub fn request_ref(&self) -> &str {
        &self.request_ref
    }

    pub fn receipt_use_ref(&self) -> &str {
        &self.receipt_use_ref
    }

    pub fn designated_evaluator_locus(&self) -> &str {
        &self.designated_evaluator_locus
    }

    pub fn source_owner_locus(&self) -> &str {
        &self.source_owner_locus
    }

    pub fn frontier_requirement_names(&self) -> &[String] {
        &self.frontier_requirement_names
    }
}

/// Reference-only facts retained for a relation publication carrier.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterRelationPublicationFacts {
    relation_name: String,
    publication_locus: String,
    consumer_locus: String,
}

impl Sys5I3AdapterRelationPublicationFacts {
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    pub fn publication_locus(&self) -> &str {
        &self.publication_locus
    }

    pub fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }
}

/// Reference-only facts retained for a designated result delivery carrier.
/// Result/frontier/policy values remain one-way references; this is not a
/// payload or a runtime result occurrence.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterDesignatedResultFacts {
    evaluator_locus: String,
    consumer_locus: String,
    result_version_ref: String,
    input_frontier_ref: String,
    result_frontier_ref: String,
    observation_policy_ref: String,
    policy_stamp_ref: String,
    static_retry_contract_name: String,
}

impl Sys5I3AdapterDesignatedResultFacts {
    pub fn evaluator_locus(&self) -> &str {
        &self.evaluator_locus
    }

    pub fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub fn result_version_ref(&self) -> &str {
        &self.result_version_ref
    }

    pub fn input_frontier_ref(&self) -> &str {
        &self.input_frontier_ref
    }

    pub fn result_frontier_ref(&self) -> &str {
        &self.result_frontier_ref
    }

    pub fn observation_policy_ref(&self) -> &str {
        &self.observation_policy_ref
    }

    pub fn policy_stamp_ref(&self) -> &str {
        &self.policy_stamp_ref
    }

    pub fn static_retry_contract_name(&self) -> &str {
        &self.static_retry_contract_name
    }
}

/// The exact closed static I3 adapter sum type.  There is no wildcard,
/// extension field, or nullable data slot shared by unrelated families.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sys5I3AdapterCarrierVariantFacts {
    OwnerRequest(Sys5I3AdapterOwnerFacts),
    OwnerReplyReceipt(Sys5I3AdapterOwnerFacts),
    DesignatedInputRequest(Sys5I3AdapterDesignatedInputFacts),
    DesignatedInputReceipt(Sys5I3AdapterDesignatedInputFacts),
    RelationProjectionPublication(Sys5I3AdapterRelationPublicationFacts),
    DesignatedResultDelivery(Sys5I3AdapterDesignatedResultFacts),
}

/// A closed, private/provisional byte-transport snapshot of one static I3
/// adapter carrier.  It is owned here because this module owns every retained
/// field of `Sys5I3AdapterCarrierContract`; downstream code may compare or
/// serialize this snapshot, but cannot reconstruct it from partial facts.
///
/// This is deliberately not a runtime carrier, request, payload, session, or
/// public wire contract.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sys5I3AdapterWireSnapshot {
    checked_program_ref: String,
    operation_id: String,
    edge_kind: String,
    lifecycle_kind: String,
    source_locus: String,
    target_locus: String,
    logical_source_path: String,
    source_span: Sys5I3AdapterWireSourceSpan,
    source_ref: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    edge_ref: String,
    declared_failure_names: Vec<String>,
    effect_kind_names: Vec<String>,
    required_occurrence_slot_names: Vec<String>,
    #[serde(rename = "requires_linked_request")]
    linked_request_identity: bool,
    typed_outcome: bool,
    receipt_consumption: bool,
    authority: Sys5I3AdapterWireAuthorityRequirements,
    redaction: Sys5I3AdapterWireRedaction,
    checked_core_bound: bool,
    transfers_authority: bool,
    mints_authority_without_source: bool,
    public_api_or_wire_contract: bool,
    variant: Sys5I3AdapterWireVariant,
    full_retained_contract_fingerprint: String,
    full_retained_contract_fingerprint_field_names: Vec<String>,
}

impl Sys5I3AdapterWireSnapshot {
    /// The decoded reference is an untrusted static lookup hint only.  The
    /// I3-1 adapter still compares the entire retained snapshot at the
    /// receiver-owned source-bound admission boundary.
    #[doc(hidden)]
    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    /// The retained exhaustive static fingerprint inventory.  It is consumed
    /// only as an opaque equality/fail-closed evidence list by the private
    /// adapter codec.
    #[doc(hidden)]
    pub fn full_retained_contract_fingerprint_field_names(&self) -> &[String] {
        &self.full_retained_contract_fingerprint_field_names
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireSourceSpan {
    start: u64,
    end: u64,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireAuthorityRequirements {
    authority_requirement_rows: Vec<Sys5I3AdapterWireAuthorityRequirementRow>,
    authority_category_names: Vec<String>,
    requires_membership_epoch_and_incarnation: bool,
    requires_capability_and_witness_refs: bool,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireAuthorityRequirementRow {
    requirement_kind_name: String,
    generated_obligation: Sys5I3AdapterWireGeneratedObligation,
    provenance_name: String,
    authority_category: Sys5I3AdapterWireOptionalText,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", content = "details", deny_unknown_fields)]
enum Sys5I3AdapterWireGeneratedObligation {
    Absent,
    Present {
        kind_name: String,
        detail_name: Sys5I3AdapterWireOptionalText,
    },
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", content = "text", deny_unknown_fields)]
enum Sys5I3AdapterWireOptionalText {
    Absent,
    Present(String),
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Sys5I3AdapterWireRedaction {
    ReferenceOnly,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "family", content = "facts", deny_unknown_fields)]
enum Sys5I3AdapterWireVariant {
    OwnerRequest(Sys5I3AdapterWireOwnerFacts),
    OwnerReplyReceipt(Sys5I3AdapterWireOwnerFacts),
    DesignatedInputRequest(Sys5I3AdapterWireDesignatedInputFacts),
    DesignatedInputReceipt(Sys5I3AdapterWireDesignatedInputFacts),
    RelationProjectionPublication(Sys5I3AdapterWireRelationPublicationFacts),
    DesignatedResultDelivery(Sys5I3AdapterWireDesignatedResultFacts),
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireOwnerFacts {
    origin_principal_ref: String,
    origin_locus_template: String,
    target_owner_locus_template: String,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireDesignatedInputFacts {
    dependency_ordinal: u64,
    typed_state_read_ref: String,
    requester_site_ref: String,
    authority_origin_ref: String,
    request_ref: String,
    receipt_use_ref: String,
    designated_evaluator_locus: String,
    source_owner_locus: String,
    frontier_requirement_names: Vec<String>,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireRelationPublicationFacts {
    relation_name: String,
    publication_locus: String,
    consumer_locus: String,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Sys5I3AdapterWireDesignatedResultFacts {
    evaluator_locus: String,
    consumer_locus: String,
    result_version_ref: String,
    input_frontier_ref: String,
    result_frontier_ref: String,
    observation_policy_ref: String,
    policy_stamp_ref: String,
    static_delivery_contract: Sys5I3AdapterWireStaticDeliveryContract,
}

/// Closed static semantic delivery behavior.  This value describes the
/// accepted carrier's result-consumption rule only; it does not authorize or
/// perform a transport retry, create an occurrence, or retain a token/cache.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Sys5I3AdapterWireStaticDeliveryContract {
    ReturnExistingNoNewConsumption,
}

/// An immutable reference-only snapshot of exactly one generated checked I2
/// carrier in the closed I3-1 adapter algebra.  It is private/provisional
/// implementation evidence, never a public wire, package, API, or runtime
/// admission contract.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3AdapterCarrierContract {
    checked_program_ref: String,
    operation_id: String,
    edge_kind: String,
    lifecycle_kind: String,
    source_locus: String,
    target_locus: String,
    logical_source_path: String,
    source_span: Sys5SourceSpan,
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
    receipt_consumption: bool,
    authority_requirements: Sys5I3AdapterAuthorityRequirements,
    redaction: Sys5I3ProbeRedaction,
    checked_core_bound: bool,
    transfers_authority: bool,
    mints_authority_without_source: bool,
    variant_facts: Sys5I3AdapterCarrierVariantFacts,
    full_retained_contract_fingerprint: String,
    full_retained_contract_fingerprint_field_names: Vec<String>,
}

impl Sys5I3AdapterCarrierContract {
    pub fn checked_program_ref(&self) -> &str {
        &self.checked_program_ref
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn edge_kind(&self) -> &str {
        &self.edge_kind
    }

    pub fn lifecycle_kind(&self) -> &str {
        &self.lifecycle_kind
    }

    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }

    pub fn target_locus(&self) -> &str {
        &self.target_locus
    }

    pub fn logical_source_path(&self) -> &str {
        &self.logical_source_path
    }

    pub const fn source_span(&self) -> Sys5SourceSpan {
        self.source_span
    }

    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }

    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub fn source_artifact_ref(&self) -> &str {
        &self.source_artifact_ref
    }

    pub fn target_artifact_ref(&self) -> &str {
        &self.target_artifact_ref
    }

    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub fn declared_failure_names(&self) -> &[String] {
        &self.declared_failure_names
    }

    pub fn effect_kind_names(&self) -> &[String] {
        &self.effect_kind_names
    }

    pub fn required_occurrence_slot_names(&self) -> &[String] {
        &self.required_occurrence_slot_names
    }

    pub const fn requires_linked_request_identity(&self) -> bool {
        self.linked_request_identity
    }

    pub const fn requires_typed_outcome(&self) -> bool {
        self.typed_outcome
    }

    pub const fn requires_receipt_consumption_state(&self) -> bool {
        self.receipt_consumption
    }

    pub fn authority_requirements(&self) -> &Sys5I3AdapterAuthorityRequirements {
        &self.authority_requirements
    }

    pub const fn redaction(&self) -> Sys5I3ProbeRedaction {
        self.redaction
    }

    pub const fn checked_core_bound(&self) -> bool {
        self.checked_core_bound
    }

    pub const fn transfers_authority(&self) -> bool {
        self.transfers_authority
    }

    pub const fn mints_authority_without_source(&self) -> bool {
        self.mints_authority_without_source
    }

    pub fn variant_facts(&self) -> &Sys5I3AdapterCarrierVariantFacts {
        &self.variant_facts
    }

    pub fn full_retained_contract_fingerprint(&self) -> &str {
        &self.full_retained_contract_fingerprint
    }

    pub fn full_retained_contract_fingerprint_field_names(&self) -> &[String] {
        &self.full_retained_contract_fingerprint_field_names
    }

    pub const fn public_api_or_wire_contract(&self) -> bool {
        false
    }

    /// Produces the only byte-transport DTO for this private static adapter
    /// contract.  Every retained field is destructured here at its owner;
    /// future fields or variants therefore require an explicit wire decision.
    #[doc(hidden)]
    pub fn i3_adapter_wire_snapshot(&self) -> Sys5I3AdapterWireSnapshot {
        let Self {
            checked_program_ref,
            operation_id,
            edge_kind,
            lifecycle_kind,
            source_locus,
            target_locus,
            logical_source_path,
            source_span,
            source_ref,
            core_ref,
            source_artifact_ref,
            target_artifact_ref,
            edge_ref,
            declared_failure_names,
            effect_kind_names,
            required_occurrence_slot_names,
            linked_request_identity,
            typed_outcome,
            receipt_consumption,
            authority_requirements,
            redaction,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            variant_facts,
            full_retained_contract_fingerprint,
            full_retained_contract_fingerprint_field_names,
        } = self;
        let Sys5SourceSpan {
            start,
            end,
            start_line,
            start_column,
            end_line,
            end_column,
        } = source_span;
        let Sys5I3AdapterAuthorityRequirements {
            rows,
            category_names,
            requires_membership_epoch_and_incarnation,
            requires_capability_and_witness_refs,
        } = authority_requirements;
        let authority_requirement_rows = rows
            .iter()
            .map(
                |Sys5I3AdapterAuthorityRequirementRow {
                     requirement_kind_name,
                     generated_obligation_present,
                     generated_obligation_kind_name,
                     generated_obligation_detail_name,
                     provenance_name,
                     authority_category_name,
                 }| {
                    let generated_obligation = match (
                        generated_obligation_present,
                        generated_obligation_kind_name,
                        generated_obligation_detail_name,
                    ) {
                        (false, None, None) => Sys5I3AdapterWireGeneratedObligation::Absent,
                        (true, Some(kind_name), detail_name) => {
                            Sys5I3AdapterWireGeneratedObligation::Present {
                                kind_name: kind_name.clone(),
                                detail_name: i3_adapter_wire_optional_text(detail_name),
                            }
                        }
                        _ => panic!("static adapter authority row is internally inconsistent"),
                    };
                    Sys5I3AdapterWireAuthorityRequirementRow {
                        requirement_kind_name: requirement_kind_name.clone(),
                        generated_obligation,
                        provenance_name: provenance_name.clone(),
                        authority_category: i3_adapter_wire_optional_text(authority_category_name),
                    }
                },
            )
            .collect();
        Sys5I3AdapterWireSnapshot {
            checked_program_ref: checked_program_ref.clone(),
            operation_id: operation_id.clone(),
            edge_kind: edge_kind.clone(),
            lifecycle_kind: lifecycle_kind.clone(),
            source_locus: source_locus.clone(),
            target_locus: target_locus.clone(),
            logical_source_path: logical_source_path.clone(),
            source_span: Sys5I3AdapterWireSourceSpan {
                start: *start,
                end: *end,
                start_line: *start_line,
                start_column: *start_column,
                end_line: *end_line,
                end_column: *end_column,
            },
            source_ref: source_ref.clone(),
            core_ref: core_ref.clone(),
            source_artifact_ref: source_artifact_ref.clone(),
            target_artifact_ref: target_artifact_ref.clone(),
            edge_ref: edge_ref.clone(),
            declared_failure_names: declared_failure_names.clone(),
            effect_kind_names: effect_kind_names.clone(),
            required_occurrence_slot_names: required_occurrence_slot_names.clone(),
            linked_request_identity: *linked_request_identity,
            typed_outcome: *typed_outcome,
            receipt_consumption: *receipt_consumption,
            authority: Sys5I3AdapterWireAuthorityRequirements {
                authority_requirement_rows,
                authority_category_names: category_names.clone(),
                requires_membership_epoch_and_incarnation:
                    *requires_membership_epoch_and_incarnation,
                requires_capability_and_witness_refs: *requires_capability_and_witness_refs,
            },
            redaction: match redaction {
                Sys5I3ProbeRedaction::ReferenceOnly => Sys5I3AdapterWireRedaction::ReferenceOnly,
            },
            checked_core_bound: *checked_core_bound,
            transfers_authority: *transfers_authority,
            mints_authority_without_source: *mints_authority_without_source,
            public_api_or_wire_contract: false,
            variant: i3_adapter_wire_variant(variant_facts),
            full_retained_contract_fingerprint: full_retained_contract_fingerprint.clone(),
            // The inventory's exact order and multiplicity are retained, but
            // its labels are one-way refs so static byte evidence cannot make
            // incidental runtime vocabulary look like a runtime feature.
            full_retained_contract_fingerprint_field_names:
                full_retained_contract_fingerprint_field_names
                    .iter()
                    .map(|field_name| i3_adapter_wire_field_inventory_ref(field_name))
                    .collect(),
        }
    }
}

fn i3_adapter_wire_optional_text(value: &Option<String>) -> Sys5I3AdapterWireOptionalText {
    match value {
        None => Sys5I3AdapterWireOptionalText::Absent,
        Some(text) => Sys5I3AdapterWireOptionalText::Present(text.clone()),
    }
}

fn i3_adapter_wire_variant(facts: &Sys5I3AdapterCarrierVariantFacts) -> Sys5I3AdapterWireVariant {
    match facts {
        Sys5I3AdapterCarrierVariantFacts::OwnerRequest(facts) => {
            let Sys5I3AdapterOwnerFacts {
                origin_principal_ref,
                origin_locus_template,
                target_owner_locus_template,
            } = facts;
            Sys5I3AdapterWireVariant::OwnerRequest(Sys5I3AdapterWireOwnerFacts {
                origin_principal_ref: origin_principal_ref.clone(),
                origin_locus_template: origin_locus_template.clone(),
                target_owner_locus_template: target_owner_locus_template.clone(),
            })
        }
        Sys5I3AdapterCarrierVariantFacts::OwnerReplyReceipt(facts) => {
            let Sys5I3AdapterOwnerFacts {
                origin_principal_ref,
                origin_locus_template,
                target_owner_locus_template,
            } = facts;
            Sys5I3AdapterWireVariant::OwnerReplyReceipt(Sys5I3AdapterWireOwnerFacts {
                origin_principal_ref: origin_principal_ref.clone(),
                origin_locus_template: origin_locus_template.clone(),
                target_owner_locus_template: target_owner_locus_template.clone(),
            })
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(facts) => {
            Sys5I3AdapterWireVariant::DesignatedInputRequest(
                i3_adapter_wire_designated_input_facts(facts),
            )
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(facts) => {
            Sys5I3AdapterWireVariant::DesignatedInputReceipt(
                i3_adapter_wire_designated_input_facts(facts),
            )
        }
        Sys5I3AdapterCarrierVariantFacts::RelationProjectionPublication(facts) => {
            let Sys5I3AdapterRelationPublicationFacts {
                relation_name,
                publication_locus,
                consumer_locus,
            } = facts;
            Sys5I3AdapterWireVariant::RelationProjectionPublication(
                Sys5I3AdapterWireRelationPublicationFacts {
                    relation_name: relation_name.clone(),
                    publication_locus: publication_locus.clone(),
                    consumer_locus: consumer_locus.clone(),
                },
            )
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedResultDelivery(facts) => {
            let Sys5I3AdapterDesignatedResultFacts {
                evaluator_locus,
                consumer_locus,
                result_version_ref,
                input_frontier_ref,
                result_frontier_ref,
                observation_policy_ref,
                policy_stamp_ref,
                static_retry_contract_name,
            } = facts;
            Sys5I3AdapterWireVariant::DesignatedResultDelivery(
                Sys5I3AdapterWireDesignatedResultFacts {
                    evaluator_locus: evaluator_locus.clone(),
                    consumer_locus: consumer_locus.clone(),
                    result_version_ref: result_version_ref.clone(),
                    input_frontier_ref: input_frontier_ref.clone(),
                    result_frontier_ref: result_frontier_ref.clone(),
                    observation_policy_ref: observation_policy_ref.clone(),
                    policy_stamp_ref: policy_stamp_ref.clone(),
                    static_delivery_contract: i3_adapter_wire_static_delivery_contract(
                        static_retry_contract_name,
                    ),
                },
            )
        }
    }
}

fn i3_adapter_wire_static_delivery_contract(
    contract_name: &str,
) -> Sys5I3AdapterWireStaticDeliveryContract {
    match contract_name {
        "ReturnExistingNoNewConsumption" => {
            Sys5I3AdapterWireStaticDeliveryContract::ReturnExistingNoNewConsumption
        }
        _ => panic!("closed static delivery contract is not recognized by the private wire"),
    }
}

fn i3_adapter_wire_field_inventory_ref(field_name: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_ADAPTER_FIELD_INVENTORY_REF_DOMAIN);
    hasher.update(
        u64::try_from(field_name.len())
            .expect("finite static field inventory label length fits u64")
            .to_be_bytes(),
    );
    hasher.update(field_name.as_bytes());
    format!("i3-adapter-field-slot-sha256-v1:{:x}", hasher.finalize())
}

fn i3_adapter_wire_designated_input_facts(
    facts: &Sys5I3AdapterDesignatedInputFacts,
) -> Sys5I3AdapterWireDesignatedInputFacts {
    let Sys5I3AdapterDesignatedInputFacts {
        dependency_ordinal,
        typed_state_read_ref,
        requester_site_ref,
        authority_origin_ref,
        request_ref,
        receipt_use_ref,
        designated_evaluator_locus,
        source_owner_locus,
        frontier_requirement_names,
    } = facts;
    Sys5I3AdapterWireDesignatedInputFacts {
        dependency_ordinal: u64::try_from(*dependency_ordinal)
            .expect("finite static dependency ordinal fits the private u64 wire field"),
        typed_state_read_ref: typed_state_read_ref.clone(),
        requester_site_ref: requester_site_ref.clone(),
        authority_origin_ref: authority_origin_ref.clone(),
        request_ref: request_ref.clone(),
        receipt_use_ref: receipt_use_ref.clone(),
        designated_evaluator_locus: designated_evaluator_locus.clone(),
        source_owner_locus: source_owner_locus.clone(),
        frontier_requirement_names: frontier_requirement_names.clone(),
    }
}

/// An experimental, non-public checked/projected local slice.  It can start
/// the bounded in-process runtime through `Sys5PreparedAdmission`; this type
/// itself retains only checked/projected state, not a live fabric.
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5LocalProject {
    checked: CheckedSurfaceV0,
    topology: DeclaredLogicalTopology,
    projection: GlobalProjectionResult,
    semantic_summary: Sys5SemanticSummary,
    observer_safe_view: Sys5ObserverSafeView,
}

impl fmt::Debug for Sys5LocalProject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sys5LocalProject")
            .field("profile", &self.semantic_summary.profile_name)
            .field(
                "checked_program_identity_ref",
                &self.checked_program_identity_ref(),
            )
            .field("artifact_count", &self.semantic_summary.artifacts.len())
            .field(
                "observer_fragment_count",
                &self.observer_safe_view.semantic_fragments.len(),
            )
            .field("status", &PROFILE_STATUS)
            .finish()
    }
}

impl Sys5LocalProject {
    /// Stable-in-this-profile semantic summary only; it contains no runtime
    /// state, credential, capability, or witness payload.
    pub fn semantic_summary(&self) -> &Sys5SemanticSummary {
        &self.semantic_summary
    }

    /// Returns a reference-only snapshot of exactly one generated I2 carrier
    /// selected by its retained opaque edge reference.  This doc-hidden I3-0
    /// evidence hook reads the original projection only: it does not reparse
    /// source, prepare admission, construct routing, accept authority, or
    /// expose payload/runtime state.
    #[doc(hidden)]
    pub fn i3_probe_carrier_contract(
        &self,
        edge_ref: &str,
    ) -> Result<Sys5I3ProbeCarrierContract, Sys5I3ProbeFacadeError> {
        let mut selected = self
            .projection
            .communication_plan()
            .edges()
            .iter()
            .filter(|candidate| candidate.edge_ref() == edge_ref);
        let edge = selected.next().ok_or_else(|| {
            Sys5I3ProbeFacadeError::new(Sys5I3ProbeFacadeErrorKind::UnknownEdgeRef)
        })?;
        if selected.next().is_some() {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::NonUniqueEdgeRef,
            ));
        }

        let carrier = edge.carrier_contract();
        if edge.kind() != CommunicationEdgeKind::OwnerRequest
            || carrier.edge_kind() != CommunicationEdgeKind::OwnerRequest
            || carrier.lifecycle_kind() != CarrierLifecycleKind::OwnerRequest
        {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::NotOwnerRequest,
            ));
        }

        let Some(origin_principal_template) = carrier.origin_principal_template() else {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        };
        let Some(origin_locus_template) = carrier.origin_locus_template() else {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        };
        let Some(target_owner_locus_template) = carrier.target_owner_locus_template() else {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        };
        let Some(core_ref) = edge.core_ref() else {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        };
        let checked_program_stable_key = edge
            .checked_core_identity()
            .checked_program_identity()
            .stable_key();
        let expected_checked_program_ref =
            checked_program_identity_ref(&checked_program_stable_key);
        if carrier.operation_identity_template().operation_id() != edge.operation_id()
            || carrier.core_ref() != Some(core_ref)
            || expected_checked_program_ref != self.checked_program_identity_ref()
            || !carrier.visibility_policy().is_reference_only_redacted()
            || carrier.transfers_authority() != edge.transfers_authority()
            || origin_locus_template != edge.source_locus()
            || target_owner_locus_template != edge.target_locus()
        {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        }
        let owner_request_component = carrier
            .i3_probe_owner_request_fingerprint_component()
            .ok_or_else(|| {
                Sys5I3ProbeFacadeError::new(Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch)
            })?;
        let i3_adapter_authority_requirement_rows = carrier
            .i3_adapter_static_facts()
            .ok_or_else(|| {
                Sys5I3ProbeFacadeError::new(Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch)
            })?
            .authority_requirement_rows;

        let source_ref = carrier.source_ref();
        let request_template_slot_names = carrier
            .required_occurrence_slots()
            .iter()
            .copied()
            .map(carrier_occurrence_slot_name)
            .map(str::to_string)
            .collect::<Vec<_>>();
        let frontier_requirement_names = [CarrierFrontierKind::Input, CarrierFrontierKind::Result]
            .into_iter()
            .filter(|frontier| carrier.requires_frontier(*frontier))
            .map(carrier_frontier_kind_name)
            .map(str::to_string)
            .collect::<Vec<_>>();
        let designated_result_details_present = [
            carrier.result_version().is_some(),
            carrier.input_frontier().is_some(),
            carrier.result_frontier().is_some(),
            carrier.observation_policy().is_some(),
            carrier.policy_stamp().is_some(),
            carrier.static_retry_contract().is_some(),
        ]
        .into_iter()
        .any(|present| present);
        let owner_request = Sys5I3ProbeOwnerRequestFacts {
            request_template_present: carrier.request_identity_template().has_slot(),
            request_template_slot_names: request_template_slot_names.clone(),
            origin_principal_ref: i3_probe_owner_principal_ref(origin_principal_template),
            origin_locus_template: origin_locus_template.to_string(),
            target_owner_locus_template: target_owner_locus_template.to_string(),
            frontier_requirement_names,
            has_no_frontier_contract: carrier.has_no_frontier_contract(),
            requires_receipt_consumption_state: carrier.requires_receipt_consumption_state(),
            designated_dependency_present: carrier.designated_remote_input_dependency().is_some(),
            designated_result_details_present,
        };
        let mut snapshot = Sys5I3ProbeCarrierContract {
            checked_program_ref: expected_checked_program_ref,
            operation_id: carrier
                .operation_identity_template()
                .operation_id()
                .to_string(),
            edge_kind: edge_kind_name(edge.kind()).to_string(),
            lifecycle_kind: carrier_lifecycle_kind_name(carrier.lifecycle_kind()).to_string(),
            source_locus: origin_locus_template.to_string(),
            target_locus: target_owner_locus_template.to_string(),
            logical_source_path: source_ref.path.clone(),
            source_span: summary_source_span(source_ref),
            source_ref: observer_source_ref(source_ref),
            core_ref: core_ref.to_string(),
            source_artifact_ref: edge.source_fragment_ref().clone(),
            target_artifact_ref: edge.target_fragment_ref().clone(),
            edge_ref: edge.edge_ref().to_string(),
            declared_failure_names: carrier.declared_failure_row().names(),
            effect_kind_names: carrier
                .effect_row()
                .kinds()
                .into_iter()
                .map(effect_kind_name)
                .map(str::to_string)
                .collect(),
            required_occurrence_slot_names: request_template_slot_names,
            linked_request_identity: carrier.requires_linked_request_identity(),
            typed_outcome: carrier.requires_typed_success_or_declared_failure_outcome(),
            authority_requirements: i3_probe_authority_requirements(
                &i3_adapter_authority_requirement_rows,
            ),
            redaction: Sys5I3ProbeRedaction::ReferenceOnly,
            checked_core_bound: carrier.provenance().is_checked_core_bound(),
            transfers_authority: carrier.transfers_authority(),
            mints_authority_without_source: carrier.mints_authority_without_source(),
            owner_request,
            full_retained_contract_fingerprint: String::new(),
        };
        snapshot.full_retained_contract_fingerprint =
            i3_probe_full_retained_contract_fingerprint(&snapshot, &owner_request_component);
        Ok(snapshot)
    }

    /// The exact closed generated-carrier family inventory accepted by the
    /// private I3-1 adapter.  This is neither a public extension point nor a
    /// claim that the six finite I2 families are a general carrier theorem.
    #[doc(hidden)]
    pub const fn i3_adapter_accepted_family_kind_names(&self) -> [&'static str; 6] {
        [
            "owner-request",
            "owner-reply-receipt",
            "designated-input-request",
            "designated-input-receipt",
            "relation-projection-publication",
            "designated-result-delivery",
        ]
    }

    /// Returns the exact source-bound static I3-1 adapter snapshot for one
    /// retained generated edge.  This does not reparse source, construct a
    /// route, bind a request, admit authority, create a runtime occurrence, or
    /// expose a payload/result/cache/session/certificate value.
    #[doc(hidden)]
    pub fn i3_adapter_carrier_contract(
        &self,
        edge_ref: &str,
    ) -> Result<Sys5I3AdapterCarrierContract, Sys5I3ProbeFacadeError> {
        let mut selected = self
            .projection
            .communication_plan()
            .edges()
            .iter()
            .filter(|candidate| candidate.edge_ref() == edge_ref);
        let edge = selected.next().ok_or_else(|| {
            Sys5I3ProbeFacadeError::new(Sys5I3ProbeFacadeErrorKind::UnknownEdgeRef)
        })?;
        if selected.next().is_some() {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::NonUniqueEdgeRef,
            ));
        }
        let family = i3_adapter_carrier_family_for_edge_kind(edge.kind())?;
        if !edge.is_derived_from_checked_core() || edge.transfers_authority() {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        }

        let facts = edge
            .carrier_contract()
            .i3_adapter_static_facts()
            .ok_or_else(|| {
                Sys5I3ProbeFacadeError::new(Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch)
            })?;
        let I3AdapterCarrierStaticFacts {
            edge_kind,
            lifecycle_kind,
            operation_id,
            source_ref,
            core_ref,
            origin_locus_template,
            target_owner_locus_template,
            declared_failure_row,
            effect_row,
            authority_requirement_rows,
            occurrence_slots,
            frontiers,
            linked_request_identity,
            typed_outcome,
            evaluator_receipt_consumption,
            reference_only_redaction,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            variant,
        } = facts;

        let Some(edge_core_ref) = edge.core_ref() else {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        };
        let expected_checked_program_ref = checked_program_identity_ref(
            &edge
                .checked_core_identity()
                .checked_program_identity()
                .stable_key(),
        );
        if edge_kind != edge.kind()
            || lifecycle_kind != i3_adapter_expected_lifecycle_kind(family)
            || operation_id != edge.operation_id()
            || core_ref.as_deref() != Some(edge_core_ref)
            || edge.source_ref() != source_ref
            || expected_checked_program_ref != self.checked_program_identity_ref()
            || !reference_only_redaction
            || !checked_core_bound
            || transfers_authority
            || mints_authority_without_source
        {
            return Err(Sys5I3ProbeFacadeError::new(
                Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
            ));
        }

        let variant_facts = i3_adapter_variant_facts(I3AdapterVariantProjectionInput {
            family,
            variant,
            dependency_ordinal: edge.checked_core_identity().dependency_ordinal(),
            frontiers: &frontiers,
            origin_locus_template: origin_locus_template.as_deref(),
            target_owner_locus_template: target_owner_locus_template.as_deref(),
            edge_source_locus: edge.source_locus(),
            edge_target_locus: edge.target_locus(),
            operation_id: &operation_id,
        })?;
        let mut snapshot = Sys5I3AdapterCarrierContract {
            checked_program_ref: expected_checked_program_ref,
            operation_id,
            edge_kind: edge_kind_name(edge_kind).to_string(),
            lifecycle_kind: carrier_lifecycle_kind_name(lifecycle_kind).to_string(),
            source_locus: edge.source_locus().to_string(),
            target_locus: edge.target_locus().to_string(),
            logical_source_path: source_ref.path.clone(),
            source_span: summary_source_span(&source_ref),
            source_ref: observer_source_ref(&source_ref),
            core_ref: edge_core_ref.to_string(),
            source_artifact_ref: edge.source_fragment_ref().clone(),
            target_artifact_ref: edge.target_fragment_ref().clone(),
            edge_ref: edge.edge_ref().to_string(),
            declared_failure_names: declared_failure_row.names(),
            effect_kind_names: effect_row
                .kinds()
                .into_iter()
                .map(effect_kind_name)
                .map(str::to_string)
                .collect(),
            required_occurrence_slot_names: occurrence_slots
                .into_iter()
                .map(carrier_occurrence_slot_name)
                .map(str::to_string)
                .collect(),
            linked_request_identity,
            typed_outcome,
            receipt_consumption: evaluator_receipt_consumption,
            authority_requirements: i3_adapter_authority_requirements(&authority_requirement_rows),
            redaction: Sys5I3ProbeRedaction::ReferenceOnly,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            variant_facts,
            full_retained_contract_fingerprint: String::new(),
            full_retained_contract_fingerprint_field_names: Vec::new(),
        };
        let visitor = i3_adapter_full_retained_contract_fingerprint_visitor(&snapshot);
        snapshot.full_retained_contract_fingerprint_field_names = visitor.field_names().to_vec();
        snapshot.full_retained_contract_fingerprint = visitor.finish();
        Ok(snapshot)
    }

    /// A serializable, observer-safe causal index for this checked/projected
    /// build. Runtime occurrence joins are exposed only by an admitted live
    /// slice, never by this projection summary alone.
    pub fn observer_safe_view(&self) -> &Sys5ObserverSafeView {
        &self.observer_safe_view
    }

    /// Crate-private read-only projection evidence for the SYS-6 verifier.
    /// It is retained from the original checked source and has no source
    /// reparse, route-builder, or admitting effect.
    pub(crate) fn projected_result_for_i2_evidence(&self) -> &GlobalProjectionResult {
        &self.projection
    }

    /// Validate a bounded conformance candidate against the retained checked
    /// Core and declared topology.  This is a pure SYS-3 verifier call: it
    /// neither admits the candidate nor exposes a route, store, authority,
    /// capability, witness, or source text.
    pub(crate) fn validates_i2_projection_candidate(
        &self,
        candidate: &GlobalProjectionResult,
    ) -> bool {
        crate::sys3_projection::verify_projection(&self.checked, &self.topology, candidate).is_ok()
    }

    /// Independently check the finite fragment and communication families
    /// required by the retained checked Core.  Unlike
    /// [`Self::validates_i2_projection_candidate`], this does not rerun the
    /// SYS-3 projector and compare two projection results.  It derives the
    /// required operation placements and edge families directly from checked
    /// Core evaluations, then compares that requirement inventory with the
    /// supplied candidate.  SYS-6 uses it as a second line for completeness
    /// and non-derived-edge controls.
    pub(crate) fn i2_candidate_covers_checked_core_requirements(
        &self,
        candidate: &GlobalProjectionResult,
    ) -> bool {
        if candidate.checked_program_identity() != self.checked.program_identity()
            || candidate.locus_order().into_iter().collect::<BTreeSet<_>>()
                != self.topology.loci().iter().map(String::as_str).collect()
        {
            return false;
        }

        let mut required_fragments = BTreeSet::new();
        let mut required_edges = BTreeSet::new();
        for evaluation in self.checked.evaluations() {
            match evaluation.kind() {
                CheckedEvaluationKind::OwnerRmw => {
                    let core = evaluation.owner_rmw_core().expect("checked owner Core");
                    let operation = evaluation.name();
                    let owner = core.owner_locus();
                    let origin = core.authority_origin_locus();
                    required_fragments.insert(I2CoreFragmentRequirement::new(
                        owner,
                        operation,
                        ProjectedOperationFragmentKind::OwnerRmwExecution,
                    ));
                    if origin != owner {
                        required_fragments.insert(I2CoreFragmentRequirement::new(
                            origin,
                            operation,
                            ProjectedOperationFragmentKind::OwnerRequestInvocation,
                        ));
                        required_edges.insert(I2CoreEdgeRequirement::new(
                            CommunicationEdgeKind::OwnerRequest,
                            origin,
                            owner,
                            operation,
                        ));
                        required_edges.insert(I2CoreEdgeRequirement::new(
                            CommunicationEdgeKind::OwnerReplyReceipt,
                            owner,
                            origin,
                            operation,
                        ));
                    }
                }
                CheckedEvaluationKind::PublishRelation => {
                    let core = evaluation.relation_core().expect("checked relation Core");
                    let operation = evaluation.name();
                    let owner = core.owner_locus();
                    required_fragments.insert(I2CoreFragmentRequirement::new(
                        owner,
                        operation,
                        ProjectedOperationFragmentKind::RelationPublication,
                    ));
                    if let Some(consumer) = core.consumer_projection_locus() {
                        required_fragments.insert(I2CoreFragmentRequirement::new(
                            consumer,
                            operation,
                            ProjectedOperationFragmentKind::ConsumerLocalRelationProjection,
                        ));
                        if owner != consumer {
                            required_edges.insert(I2CoreEdgeRequirement::new(
                                CommunicationEdgeKind::RelationProjectionPublication,
                                owner,
                                consumer,
                                operation,
                            ));
                        }
                    }
                }
                CheckedEvaluationKind::DesignatedPublishValue => {
                    let core = evaluation
                        .designated_core()
                        .expect("checked designated Core");
                    let operation = format!("{}.{}", core.evaluator(), core.result());
                    let evaluator = core.evaluator();
                    required_fragments.insert(I2CoreFragmentRequirement::new(
                        evaluator,
                        &operation,
                        ProjectedOperationFragmentKind::DesignatedEvaluation,
                    ));
                    for source in core.generated_remote_input_dependencies() {
                        let source_owner = source.source_owner_locus();
                        required_fragments.insert(I2CoreFragmentRequirement::new(
                            source_owner,
                            &operation,
                            ProjectedOperationFragmentKind::DesignatedRemoteInputService,
                        ));
                        if evaluator != source_owner {
                            required_edges.insert(I2CoreEdgeRequirement::new(
                                CommunicationEdgeKind::DesignatedInputRequest,
                                evaluator,
                                source_owner,
                                &operation,
                            ));
                            required_edges.insert(I2CoreEdgeRequirement::new(
                                CommunicationEdgeKind::DesignatedInputReceipt,
                                source_owner,
                                evaluator,
                                &operation,
                            ));
                        }
                    }
                }
                CheckedEvaluationKind::DesignatedResultConsume => {
                    let core = evaluation
                        .designated_result_consumer_core()
                        .expect("checked designated result consumer Core");
                    let operation = format!("{}.{}", core.evaluator(), core.result());
                    required_fragments.insert(I2CoreFragmentRequirement::new(
                        core.consumer_locus(),
                        &operation,
                        ProjectedOperationFragmentKind::DesignatedResultConsumer,
                    ));
                    required_edges.insert(I2CoreEdgeRequirement::new(
                        CommunicationEdgeKind::DesignatedResultDelivery,
                        core.evaluator(),
                        core.consumer_locus(),
                        &operation,
                    ));
                }
                CheckedEvaluationKind::ConsumerLocalProjection => {}
            }
        }

        let candidate_fragments = candidate
            .locus_order()
            .into_iter()
            .flat_map(|locus| {
                candidate
                    .locus_program(locus)
                    .expect("candidate retains every declared locus")
                    .operation_fragments()
                    .iter()
                    .map(move |fragment| {
                        I2CoreFragmentRequirement::new(
                            locus,
                            fragment.operation_id(),
                            fragment.fragment_kind(),
                        )
                    })
            })
            .collect::<BTreeSet<_>>();
        let candidate_edges = candidate
            .communication_plan()
            .edges()
            .iter()
            .filter(|edge| edge.is_derived_from_checked_core() && edge.core_ref().is_some())
            .map(|edge| {
                I2CoreEdgeRequirement::new(
                    edge.kind(),
                    edge.source_locus(),
                    edge.target_locus(),
                    edge.operation_id(),
                )
            })
            .collect::<BTreeSet<_>>();

        candidate_fragments == required_fragments
            && candidate_edges == required_edges
            && candidate.communication_plan().edges().len() == candidate_edges.len()
    }

    /// Test-only access to an already checked/projected clone for conformance
    /// falsifiers. The clone begins at the real SYS-3 result; no test can
    /// construct Core, topology, authority, or a communication plan from
    /// scratch through this hook.
    #[cfg(test)]
    pub(crate) fn clone_projection_for_i2_test(&self) -> GlobalProjectionResult {
        self.projection.clone()
    }

    /// Prepare the finite source-derived M9 inventory required by SYS-4.
    /// The checked source and projection retained by this project are used
    /// directly; this operation never reparses ordinary source or accepts a
    /// caller-provided route, state seed, authority carrier, or result.
    pub fn prepare_finite_admission(
        &self,
        request: Sys5LocalAdmissionRequest,
    ) -> Result<Sys5PreparedAdmission, Sys5LocalAdmissionError> {
        self.validate_source_derived_membership_request(&request)?;
        let Some(auth_residual_name) = request.auth_discharge.as_deref() else {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::MissingAuthDischarge,
            ));
        };
        if !self
            .checked
            .residual_obligations()
            .entries()
            .iter()
            .any(|residual| {
                residual.kind() == ResidualObligationKind::AuthDeferred
                    && residual.name() == auth_residual_name
            })
        {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::UnknownAuthDischarge,
            ));
        }
        let Some(verify_residual_name) = request.verification_discharge.as_deref() else {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::MissingVerificationDischarge,
            ));
        };
        if !self
            .checked
            .residual_obligations()
            .entries()
            .iter()
            .any(|residual| {
                residual.kind() == ResidualObligationKind::VerifyDeferred
                    && residual.name() == verify_residual_name
            })
        {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::UnknownVerificationDischarge,
            ));
        }

        // Projection itself is non-admitting.  Its backend eligibility must
        // nevertheless be decided before the M9 boundary can issue any
        // membership, capability, or witness.
        let program = FabricProgram::from_projection(self.projection.clone()).map_err(|_| {
            Sys5LocalAdmissionError::new(Sys5LocalAdmissionErrorKind::ProjectionFabricMismatch)
        })?;
        if matches!(
            program.backend_eligibility(request.runtime_profile.into()),
            BackendEligibility::Ineligible { .. }
        ) {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::BackendIneligible,
            ));
        }
        // The finite runtime always starts from an empty SYS-4 seed.  Literal
        // source/Core owner writes are classified below and executed through
        // the admitted generated endpoint only after the fabric exists.
        let startup_plan = self.vertical_startup_plan(&request.principal);

        let mut m9_facts = vec![M9FiniteLocalAdmissionFact::anchor_membership(
            &request.principal,
            &request.locus,
            &request.epoch,
            &request.incarnation,
        )];
        m9_facts.extend(
            request
                .source_declared_memberships
                .iter()
                .map(|membership| {
                    M9FiniteLocalAdmissionFact::source_declared_membership(
                        membership.principal(),
                        membership.locus(),
                        membership.epoch(),
                        membership.incarnation(),
                    )
                }),
        );
        match request
            .relation_bootstrap_policy
            .expect("source-derived admission validates fixed bootstrap policy first")
        {
            Sys5RelationBootstrapPolicy::FreshAtAdmission => {
                for relation in self
                    .checked
                    .evaluations()
                    .iter()
                    .filter(|evaluation| evaluation.relation_core().is_some())
                {
                    m9_facts.push(
                        M9FiniteLocalAdmissionFact::relation_bootstrap_fresh_at_admission(
                            relation.name(),
                        ),
                    );
                }
            }
        }
        m9_facts.push(M9FiniteLocalAdmissionFact::auth_discharge(
            auth_residual_name,
        ));
        m9_facts.push(M9FiniteLocalAdmissionFact::optional_verification_discharge(
            verify_residual_name,
        ));
        let candidate = M9FiniteLocalAdmissionCandidate::from_checked(
            &self.checked,
            &self.projection,
            m9_facts,
        )
        .map_err(|_| Sys5LocalAdmissionError::new(Sys5LocalAdmissionErrorKind::M9Rejected))?;
        let seam = M9RuntimeExecutionSeam::admit_validated_finite_local_candidate(candidate)
            .map_err(|_| Sys5LocalAdmissionError::new(Sys5LocalAdmissionErrorKind::M9Rejected))?;
        let empty_seed =
            Sys4InitialStateSeed::for_checked_program(self.checked.program_identity().clone());
        let admission = SealedFabricAdmission::from_m9_execution_seam(&program, seam, empty_seed)
            .map_err(|_| {
            Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::IncompleteSourceDerivedInventory,
            )
        })?;
        let inventory = Sys5AdmissionInventory::from_checked(&self.checked);
        let vertical_bindings =
            Sys5VerticalBindings::from_checked(&self.checked, &request.principal);
        let sealed_summary = admission.observer_safe_m9_summary_clone();
        let sealed_rows = admission.observer_safe_m9_semantic_row_sets_clone();
        let mut sealed_attestation =
            Sys5SealedInventoryAttestation::from_m9_summary(&sealed_summary, &sealed_rows);
        let exact_row_set_match = inventory.matches_sealed_attestation(&sealed_attestation);
        sealed_attestation.set_exact_row_set_match(exact_row_set_match);
        if !exact_row_set_match || !sealed_attestation.covers_source_inventory(&inventory) {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::IncompleteSourceDerivedInventory,
            ));
        }
        let summary = Sys5AdmissionSummary::from_inventory(
            self.checked_program_identity_ref(),
            request.runtime_profile,
            auth_residual_name,
            verify_residual_name,
            &inventory,
            &sealed_attestation,
        );
        Ok(Sys5PreparedAdmission {
            program,
            admission,
            summary,
            inventory,
            sealed_attestation,
            startup_plan,
            source_principal: request.principal,
            vertical_bindings,
        })
    }

    /// An observer-safe opaque reference for the exact retained checked
    /// program.  This does not expose a raw source program identity.
    pub fn checked_program_identity_ref(&self) -> &str {
        self.semantic_summary
            .artifacts
            .first()
            .map(|artifact| artifact.checked_program_identity.as_str())
            .unwrap_or("")
    }

    /// Prepare one deterministic, source-derived admission for the selected
    /// internal backend profile. The helper derives its principal, complete
    /// locus inventory, residual discharge names, and non-secret membership
    /// labels from the retained checked/projected program; it does not accept
    /// a route, state seed, authority carrier, or result from the caller.
    ///
    /// SYS-6 uses the `Ow1` form only for its separately declared selected
    /// one-owner-worker source. This constructor is not an OW1 claim for the
    /// four-locus SYS-5 workflow, whose lifecycle/cut path remains ST-only.
    pub fn prepare_canonical_local_admission(
        &self,
        runtime_profile: Sys5LocalRuntimeProfile,
    ) -> Result<Sys5PreparedAdmission, Sys5LocalAdmissionError> {
        let principals = self
            .checked
            .evaluations()
            .iter()
            .filter_map(|evaluation| {
                evaluation
                    .owner_rmw_core()
                    .map(|_| evaluation.actor_authority_origin().to_string())
            })
            .collect::<BTreeSet<_>>();
        let Some(principal) = (principals.len() == 1)
            .then(|| principals.iter().next().expect("one principal").clone())
        else {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::PrincipalPolicyMismatch,
            ));
        };
        let loci = self.projection.locus_order();
        let Some(anchor_locus) = loci.first().map(|locus| (*locus).to_string()) else {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::UnknownLocus,
            ));
        };
        let Some(auth_name) = self
            .checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
            .map(|residual| residual.name().to_string())
        else {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::MissingAuthDischarge,
            ));
        };
        let Some(verification_name) = self
            .checked
            .residual_obligations()
            .entries()
            .iter()
            .find(|residual| residual.kind() == ResidualObligationKind::VerifyDeferred)
            .map(|residual| residual.name().to_string())
        else {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::MissingVerificationDischarge,
            ));
        };

        let profile_label = match runtime_profile {
            Sys5LocalRuntimeProfile::St => "st",
            Sys5LocalRuntimeProfile::Ow1 => "ow1",
        };
        let mut request = Sys5LocalAdmissionRequest::source_declared(
            &principal,
            &anchor_locus,
            format!("sys5-local-{profile_label}:{principal}:{anchor_locus}:epoch"),
            format!("sys5-local-{profile_label}:{principal}:{anchor_locus}:incarnation"),
            runtime_profile,
        )
        .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
        .with_auth_discharge(auth_name)
        .with_optional_verification_discharge(verification_name);
        for locus in loci
            .into_iter()
            .filter(|locus| *locus != anchor_locus.as_str())
        {
            request = request.with_source_declared_membership(
                &principal,
                locus,
                format!("sys5-local-{profile_label}:{principal}:{locus}:epoch"),
                format!("sys5-local-{profile_label}:{principal}:{locus}:incarnation"),
            );
        }
        self.prepare_finite_admission(request)
    }

    /// Prepare the deterministic ST admission used by the SYS-5 local
    /// workflow. This remains a convenience specialization of the same
    /// source-derived admission construction used by the selected SYS-6 OW1
    /// check.
    pub fn prepare_canonical_local_st_admission(
        &self,
    ) -> Result<Sys5PreparedAdmission, Sys5LocalAdmissionError> {
        self.prepare_canonical_local_admission(Sys5LocalRuntimeProfile::St)
    }

    fn validate_source_derived_membership_request(
        &self,
        request: &Sys5LocalAdmissionRequest,
    ) -> Result<(), Sys5LocalAdmissionError> {
        let known_principals = self
            .checked
            .static_environment()
            .principals()
            .iter()
            .map(|principal| principal.name())
            .collect::<BTreeSet<_>>();
        let known_loci = self
            .projection
            .locus_order()
            .into_iter()
            .collect::<BTreeSet<_>>();
        if !known_principals.contains(request.principal.as_str()) {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::UnknownPrincipal,
            ));
        }
        if !known_loci.contains(request.locus.as_str()) {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::UnknownLocus,
            ));
        }
        if request.epoch.is_empty() || request.incarnation.is_empty() {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::InvalidAdmissionIdentity,
            ));
        }
        if request.relation_bootstrap_policy.is_none() {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::MissingRelationBootstrapPolicy,
            ));
        }

        let mut provided = BTreeMap::new();
        insert_source_declared_membership(
            &mut provided,
            Sys5SourceDeclaredMembership::new(
                &request.principal,
                &request.locus,
                &request.epoch,
                &request.incarnation,
            ),
        )?;
        for membership in &request.source_declared_memberships {
            if !known_principals.contains(membership.principal()) {
                return Err(Sys5LocalAdmissionError::new(
                    Sys5LocalAdmissionErrorKind::UnknownPrincipal,
                ));
            }
            if !known_loci.contains(membership.locus()) {
                return Err(Sys5LocalAdmissionError::new(
                    Sys5LocalAdmissionErrorKind::UnknownLocus,
                ));
            }
            if membership.epoch().is_empty() || membership.incarnation().is_empty() {
                return Err(Sys5LocalAdmissionError::new(
                    Sys5LocalAdmissionErrorKind::InvalidAdmissionIdentity,
                ));
            }
            insert_source_declared_membership(&mut provided, membership.clone())?;
        }

        let owner_principals = self
            .checked
            .evaluations()
            .iter()
            .filter(|evaluation| evaluation.owner_rmw_core().is_some())
            .map(|evaluation| evaluation.actor_authority_origin())
            .collect::<BTreeSet<_>>();
        if owner_principals.len() > 1
            || owner_principals
                .iter()
                .next()
                .is_some_and(|principal| *principal != request.principal)
        {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::PrincipalPolicyMismatch,
            ));
        }
        if provided
            .values()
            .any(|membership| membership.principal() != request.principal)
        {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::PrincipalPolicyMismatch,
            ));
        }

        let mut required = BTreeSet::from([(request.principal.clone(), request.locus.clone())]);
        for evaluation in self.checked.evaluations() {
            if let Some(owner) = evaluation.owner_rmw_core() {
                required.insert((
                    evaluation.actor_authority_origin().to_string(),
                    owner.owner_locus().to_string(),
                ));
            }
            if let Some(relation) = evaluation.relation_core() {
                required.insert((
                    request.principal.clone(),
                    relation.owner_locus().to_string(),
                ));
                // Explicit anchor loci are semantic membership dependencies,
                // not names to infer from the relation owner or transport.
                // Legacy Core has no such binding and retains its M10 shape.
                for anchor in [relation.primary(), relation.fallback()] {
                    if let Some(anchor_locus) = anchor.anchor_locus() {
                        required.insert((request.principal.clone(), anchor_locus.to_string()));
                    }
                }
            }
            if let Some(designated) = evaluation.designated_core() {
                required.insert((
                    request.principal.clone(),
                    designated.evaluator().to_string(),
                ));
                for dependency in designated.generated_remote_input_dependencies() {
                    required.insert((
                        request.principal.clone(),
                        dependency.source_owner_locus().to_string(),
                    ));
                }
            }
            if let Some(consumer) = evaluation.designated_result_consumer_core() {
                required.insert((
                    request.principal.clone(),
                    consumer.consumer_locus().to_string(),
                ));
            }
        }
        let provided_keys = provided.keys().cloned().collect::<BTreeSet<_>>();
        if provided_keys != required {
            return Err(Sys5LocalAdmissionError::new(
                Sys5LocalAdmissionErrorKind::MissingRequiredMembership,
            ));
        }
        Ok(())
    }
}

/// A projection fragment requirement derived directly from a checked Core
/// evaluation.  It is intentionally private: it is validation state, not a
/// public artifact format or a second projection IR.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct I2CoreFragmentRequirement {
    locus: String,
    operation: String,
    kind: ProjectedOperationFragmentKind,
}

impl I2CoreFragmentRequirement {
    fn new(
        locus: impl Into<String>,
        operation: impl Into<String>,
        kind: ProjectedOperationFragmentKind,
    ) -> Self {
        Self {
            locus: locus.into(),
            operation: operation.into(),
            kind,
        }
    }
}

/// A generated communication family required by a checked Core evaluation.
/// This is independently derived before candidate projection validation, so a
/// missing or extra candidate edge cannot be accepted merely by comparing the
/// candidate with itself.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct I2CoreEdgeRequirement {
    kind: CommunicationEdgeKind,
    source_locus: String,
    target_locus: String,
    operation: String,
}

impl I2CoreEdgeRequirement {
    fn new(
        kind: CommunicationEdgeKind,
        source_locus: impl Into<String>,
        target_locus: impl Into<String>,
        operation: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_locus: source_locus.into(),
            target_locus: target_locus.into(),
            operation: operation.into(),
        }
    }
}

/// One checked owner cell in the intentionally small local vertical profile.
/// The coordinate is derived from Core and the single admitted principal; it
/// is never accepted from an external schedule action.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Sys5VerticalCell {
    locus: String,
    state: String,
    index: String,
    field: String,
}

impl Sys5VerticalCell {
    fn from_fixed_read(
        read: &mir_semantics::surface_v0_pipeline::TypedStateRead,
        principal: &str,
    ) -> Self {
        Self {
            locus: read.owner_locus().to_string(),
            state: read.namespace().to_string(),
            index: read.index().unwrap_or(principal).to_string(),
            field: read.field().unwrap_or_default().to_string(),
        }
    }

    fn from_owner_action_read(
        read: &mir_semantics::surface_v0_pipeline::TypedStateRead,
        principal: &str,
    ) -> Self {
        let mut cell = Self::from_fixed_read(read, principal);
        // The small external action has no target input.  Its only admitted
        // owner-operation parameter is bound to the one principal named by
        // the sealed admission.  This follows the checked target shape, not
        // a fixture operation name or a caller-provided state coordinate.
        if read.index().is_some_and(|index| index != principal) {
            cell.index = principal.to_string();
        }
        cell
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys5StartupInitializer {
    operation_id: String,
    source_locus: String,
    owner_locus: String,
    cell: Sys5VerticalCell,
    literal: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Sys5VerticalStartupPlan {
    initializers: Vec<Sys5StartupInitializer>,
    required_cells: BTreeSet<Sys5VerticalCell>,
    observer_safe_cells: BTreeSet<Sys5VerticalCell>,
    observer_visible_designated_values: BTreeSet<String>,
}

impl Sys5VerticalStartupPlan {
    fn is_complete(&self) -> bool {
        self.required_cells.len() == self.initializers.len()
            && self.required_cells.iter().all(|required| {
                self.initializers
                    .iter()
                    .filter(|initializer| &initializer.cell == required)
                    .count()
                    == 1
            })
    }

    fn observer_safe_contains(&self, locus: &str, state: &str, index: &str, field: &str) -> bool {
        self.observer_safe_cells.contains(&Sys5VerticalCell {
            locus: locus.to_string(),
            state: state.to_string(),
            index: index.to_string(),
            field: field.to_string(),
        })
    }

    fn observer_visible_designated_value(&self, value_name: &str) -> bool {
        self.observer_visible_designated_values.contains(value_name)
    }
}

impl Sys5LocalProject {
    /// Classify the finite initial owner writes required by the checked
    /// vertical fragment.  A candidate must be an integer literal write; the
    /// plan then requires an exact, unambiguous initializer for every cell
    /// touched by the accepted owner/designated fragment.
    /// Names are not used as selectors; ambiguity is retained for the start
    /// boundary to fail closed.
    fn vertical_startup_plan(&self, source_principal: &str) -> Sys5VerticalStartupPlan {
        let mut required_cells = BTreeSet::new();
        let mut observer_safe_cells = BTreeSet::new();
        let mut literal_candidates = Vec::new();
        let mut observer_visible_designated_values = BTreeSet::new();

        for evaluation in self.checked.evaluations() {
            if let Some(owner) = evaluation.owner_rmw_core() {
                let literal = owner
                    .expression()
                    .tree()
                    .int_literal()
                    .map(|value| value.value());
                if literal.is_none() {
                    for read in std::iter::once(owner.target()).chain(owner.same_owner_reads()) {
                        if read.value_type() == "Int" {
                            let cell =
                                Sys5VerticalCell::from_owner_action_read(read, source_principal);
                            if self.vertical_cell_is_observer_safe(&cell) {
                                observer_safe_cells.insert(cell.clone());
                            }
                            required_cells.insert(cell);
                        }
                    }
                } else if let Some(value) = literal {
                    let cell = Sys5VerticalCell::from_fixed_read(owner.target(), source_principal);
                    // A literal owner write has no hidden input edge.  It is
                    // therefore the only source/Core-derived way this finite
                    // profile creates a local cell from its empty fabric.
                    required_cells.insert(cell.clone());
                    literal_candidates.push(Sys5StartupInitializer {
                        operation_id: evaluation.name().to_string(),
                        source_locus: owner.authority_origin_locus().to_string(),
                        owner_locus: owner.owner_locus().to_string(),
                        cell,
                        literal: value,
                    });
                }
            }
            if let Some(designated) = evaluation.designated_core() {
                let value_name = format!("{}.{}", designated.evaluator(), designated.result());
                let mut all_inputs_observer_safe = true;
                for read in designated.expression().state_reads() {
                    if read.value_type() == "Int" {
                        let cell = Sys5VerticalCell::from_fixed_read(read, source_principal);
                        all_inputs_observer_safe &= self.vertical_cell_is_observer_safe(&cell);
                        if self.vertical_cell_is_observer_safe(&cell) {
                            observer_safe_cells.insert(cell.clone());
                        }
                        required_cells.insert(cell);
                    } else {
                        all_inputs_observer_safe = false;
                    }
                }
                // The checked Core retains a declared observation policy.  A
                // result is viewer-visible only when that policy exists and
                // every field that feeds the expression is observer-safe.
                if all_inputs_observer_safe && !designated.observation_policy().name.is_empty() {
                    observer_visible_designated_values.insert(value_name);
                }
            }
        }

        let mut initializers = literal_candidates
            .into_iter()
            .filter(|candidate| required_cells.contains(&candidate.cell))
            .collect::<Vec<_>>();
        initializers.sort_by(|left, right| left.operation_id.cmp(&right.operation_id));
        Sys5VerticalStartupPlan {
            initializers,
            required_cells,
            observer_safe_cells,
            observer_visible_designated_values,
        }
    }

    fn vertical_cell_is_observer_safe(&self, cell: &Sys5VerticalCell) -> bool {
        self.checked
            .static_environment()
            .indexed_state_schema(&cell.state)
            .is_some_and(|schema| {
                schema.owner_locus() == cell.locus
                    && schema
                        .fields()
                        .iter()
                        .find(|field| field.name() == cell.field)
                        .and_then(|field| field.visibility_channel())
                        == Some("observer_safe")
            })
    }
}

/// Chosen backend profile for the finite local SYS-5 admission.  This is an
/// internal profile choice, not a public deployment or wire selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Sys5LocalRuntimeProfile {
    St,
    Ow1,
}

/// Source-declared identity and residual selections for one finite admission.
/// It intentionally has no caller-supplied authority, route, state, or
/// semantic-result fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5LocalAdmissionRequest {
    principal: String,
    locus: String,
    epoch: String,
    incarnation: String,
    runtime_profile: Sys5LocalRuntimeProfile,
    source_declared_memberships: Vec<Sys5SourceDeclaredMembership>,
    relation_bootstrap_policy: Option<Sys5RelationBootstrapPolicy>,
    auth_discharge: Option<String>,
    verification_discharge: Option<String>,
}

impl Sys5LocalAdmissionRequest {
    pub fn source_declared(
        principal: impl Into<String>,
        locus: impl Into<String>,
        epoch: impl Into<String>,
        incarnation: impl Into<String>,
        runtime_profile: Sys5LocalRuntimeProfile,
    ) -> Self {
        Self {
            principal: principal.into(),
            locus: locus.into(),
            epoch: epoch.into(),
            incarnation: incarnation.into(),
            runtime_profile,
            source_declared_memberships: Vec::new(),
            relation_bootstrap_policy: None,
            auth_discharge: None,
            verification_discharge: None,
        }
    }

    /// Add one explicit source-declared membership row.  The root identity is
    /// the fixed anchor; callers must list every additional handler locus
    /// needed by the checked program.  This accepts neither a membership
    /// reference nor a provider credential.
    pub fn with_source_declared_membership(
        mut self,
        principal: impl Into<String>,
        locus: impl Into<String>,
        epoch: impl Into<String>,
        incarnation: impl Into<String>,
    ) -> Self {
        self.source_declared_memberships
            .push(Sys5SourceDeclaredMembership {
                principal: principal.into(),
                locus: locus.into(),
                epoch: epoch.into(),
                incarnation: incarnation.into(),
            });
        self
    }

    /// Select the only bounded relation lifecycle bootstrap supported by the
    /// current local profile.  This policy identifies lifecycle evidence; it
    /// does not derive Core facts or grant authority.
    pub fn with_relation_bootstrap_policy(mut self, policy: Sys5RelationBootstrapPolicy) -> Self {
        self.relation_bootstrap_policy = Some(policy);
        self
    }

    pub fn with_auth_discharge(mut self, name: impl Into<String>) -> Self {
        self.auth_discharge = Some(name.into());
        self
    }

    pub fn with_optional_verification_discharge(mut self, name: impl Into<String>) -> Self {
        self.verification_discharge = Some(name.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys5VerticalOwnerBinding {
    operation_id: String,
    source_locus: String,
    owner_locus: String,
    declared_target_parameter: String,
    coordinate_binding_is_closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sys5VerticalDesignatedBinding {
    value_name: String,
    evaluator_locus: String,
    consumer_locus: String,
    trigger_frontier: String,
    input_source_locus: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Sys5VerticalBindings {
    owner_operations: Vec<Sys5VerticalOwnerBinding>,
    designated_values: Vec<Sys5VerticalDesignatedBinding>,
    relation_ids: BTreeSet<String>,
}

impl Sys5VerticalBindings {
    fn from_checked(checked: &CheckedSurfaceV0, source_principal: &str) -> Self {
        let mut owner_operations = Vec::new();
        let mut designated_evaluators = BTreeMap::new();
        let mut designated_consumers = BTreeMap::new();
        let mut relation_ids = BTreeSet::new();
        for evaluation in checked.evaluations() {
            if let Some(owner) = evaluation.owner_rmw_core() {
                // Literal owner writes are source/Core startup work.  The
                // external vertical action binds only the unique remaining
                // RMW whose target is a declared parameter.
                if owner.expression().tree().int_literal().is_none() {
                    let declared_target_parameter = owner
                        .target()
                        .index()
                        .unwrap_or(source_principal)
                        .to_string();
                    owner_operations.push(Sys5VerticalOwnerBinding {
                        operation_id: evaluation.name().to_string(),
                        source_locus: owner.authority_origin_locus().to_string(),
                        owner_locus: owner.owner_locus().to_string(),
                        coordinate_binding_is_closed: std::iter::once(owner.target())
                            .chain(owner.same_owner_reads())
                            .all(|read| {
                                read.index().is_none_or(|index| {
                                    index == source_principal || index == declared_target_parameter
                                })
                            }),
                        declared_target_parameter,
                    });
                }
            }
            if let Some(designated) = evaluation.designated_core() {
                let value_name = format!("{}.{}", designated.evaluator(), designated.result());
                let dependencies = designated.generated_remote_input_dependencies();
                designated_evaluators.insert(
                    value_name,
                    (
                        designated.evaluator().to_string(),
                        designated
                            .trigger()
                            .frontier()
                            .unwrap_or_default()
                            .to_string(),
                        (dependencies.len() == 1)
                            .then(|| dependencies[0].source_owner_locus().to_string()),
                    ),
                );
            }
            if let Some(consumer) = evaluation.designated_result_consumer_core() {
                designated_consumers.insert(
                    format!("{}.{}", consumer.evaluator(), consumer.result()),
                    consumer.consumer_locus().to_string(),
                );
            }
            if evaluation.relation_core().is_some() {
                relation_ids.insert(evaluation.name().to_string());
            }
        }
        owner_operations.sort_by(|left, right| left.operation_id.cmp(&right.operation_id));
        let designated_values = designated_evaluators
            .into_iter()
            .filter_map(
                |(value_name, (evaluator_locus, trigger_frontier, input_source_locus))| {
                    designated_consumers
                        .remove(&value_name)
                        .map(|consumer_locus| Sys5VerticalDesignatedBinding {
                            value_name,
                            evaluator_locus,
                            consumer_locus,
                            trigger_frontier,
                            input_source_locus,
                        })
                },
            )
            .collect();
        Self {
            owner_operations,
            designated_values,
            relation_ids,
        }
    }

    fn canonical_owner(&self) -> Option<&Sys5VerticalOwnerBinding> {
        (self.owner_operations.len() == 1 && self.owner_operations[0].coordinate_binding_is_closed)
            .then(|| &self.owner_operations[0])
    }

    fn canonical_designated(&self) -> Option<&Sys5VerticalDesignatedBinding> {
        (self.designated_values.len() == 1).then(|| &self.designated_values[0])
    }

    fn is_canonical_vertical_path(&self) -> bool {
        self.canonical_owner().is_some()
            && self.canonical_designated().is_some()
            && !self.relation_ids.is_empty()
    }
}

/// Fixed bounded lifecycle bootstrap supported by the local SYS-5 profile.
/// It is neither a Core relation fact nor an authority grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Sys5RelationBootstrapPolicy {
    FreshAtAdmission,
}

/// One non-secret membership identity supplied alongside the root anchor.
/// This stays crate-private because callers construct it only through the
/// narrow request builder above; it deliberately has no membership reference,
/// credential, capability, or witness payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys5SourceDeclaredMembership {
    principal: String,
    locus: String,
    epoch: String,
    incarnation: String,
}

impl Sys5SourceDeclaredMembership {
    pub(crate) fn new(
        principal: impl Into<String>,
        locus: impl Into<String>,
        epoch: impl Into<String>,
        incarnation: impl Into<String>,
    ) -> Self {
        Self {
            principal: principal.into(),
            locus: locus.into(),
            epoch: epoch.into(),
            incarnation: incarnation.into(),
        }
    }

    pub(crate) fn principal(&self) -> &str {
        &self.principal
    }

    pub(crate) fn locus(&self) -> &str {
        &self.locus
    }

    pub(crate) fn epoch(&self) -> &str {
        &self.epoch
    }

    pub(crate) fn incarnation(&self) -> &str {
        &self.incarnation
    }

    fn same_identity_as(&self, other: &Self) -> bool {
        self.epoch == other.epoch && self.incarnation == other.incarnation
    }
}

fn insert_source_declared_membership(
    memberships: &mut BTreeMap<(String, String), Sys5SourceDeclaredMembership>,
    membership: Sys5SourceDeclaredMembership,
) -> Result<(), Sys5LocalAdmissionError> {
    let key = (
        membership.principal().to_string(),
        membership.locus().to_string(),
    );
    if let Some(existing) = memberships.get(&key) {
        return Err(Sys5LocalAdmissionError::new(
            if existing.same_identity_as(&membership) {
                Sys5LocalAdmissionErrorKind::DuplicateMembership
            } else {
                Sys5LocalAdmissionErrorKind::ConflictingMembership
            },
        ));
    }
    memberships.insert(key, membership);
    Ok(())
}

impl From<Sys5LocalRuntimeProfile> for BackendProfile {
    fn from(profile: Sys5LocalRuntimeProfile) -> Self {
        match profile {
            Sys5LocalRuntimeProfile::St => Self::St,
            Sys5LocalRuntimeProfile::Ow1 => Self::Ow1,
        }
    }
}

/// A sealed, source-derived inventory and the matching SYS-4 admission.  It
/// exposes only observer-safe summaries until the crate-private SYS-4 bridge
/// consumes its parts.
pub struct Sys5PreparedAdmission {
    #[cfg_attr(not(test), allow(dead_code))]
    program: FabricProgram,
    #[cfg_attr(not(test), allow(dead_code))]
    admission: SealedFabricAdmission,
    summary: Sys5AdmissionSummary,
    inventory: Sys5AdmissionInventory,
    sealed_attestation: Sys5SealedInventoryAttestation,
    startup_plan: Sys5VerticalStartupPlan,
    source_principal: String,
    vertical_bindings: Sys5VerticalBindings,
}

impl fmt::Debug for Sys5PreparedAdmission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let counts = self.summary.sealed_inventory_counts();
        formatter
            .debug_struct("Sys5PreparedAdmission")
            .field("runtime_profile", &self.summary.runtime_profile())
            .field(
                "checked_program_identity_ref",
                &self.summary.checked_program_identity_ref(),
            )
            .field(
                "sealed_inventory_digest",
                &self.summary.sealed_inventory_digest(),
            )
            .field("owner_rmw_count", &counts.owner_rmw())
            .field("relation_transition_count", &counts.relation_transitions())
            .field(
                "designated_evaluator_count",
                &counts.designated_evaluators(),
            )
            .field(
                "designated_remote_input_count",
                &counts.designated_remote_inputs(),
            )
            .field("named_consumer_count", &counts.named_consumers())
            .field(
                "status",
                &if self.summary.is_complete_for_projection() {
                    "sealed-complete"
                } else {
                    "sealed-incomplete"
                },
            )
            .finish()
    }
}

impl Sys5PreparedAdmission {
    /// Clone one already sealed admission only for an internal fresh-fabric
    /// restore. This is intentionally narrower than a public clone surface:
    /// callers cannot duplicate the authority/admission carrier themselves.
    pub(crate) fn clone_for_local_restore(&self) -> Self {
        Self {
            program: self.program.clone(),
            admission: self.admission.clone(),
            summary: self.summary.clone(),
            inventory: self.inventory.clone(),
            sealed_attestation: self.sealed_attestation.clone(),
            startup_plan: self.startup_plan.clone(),
            source_principal: self.source_principal.clone(),
            vertical_bindings: self.vertical_bindings.clone(),
        }
    }
}

impl Sys5PreparedAdmission {
    pub fn observer_safe_admission_summary(&self) -> &Sys5AdmissionSummary {
        &self.summary
    }

    pub fn observer_safe_inventory(&self) -> &Sys5AdmissionInventory {
        &self.inventory
    }

    /// Opaque counts and digest produced by the sealed M9/SYS-4 boundary.
    /// It is an observation-only completeness attestation, never authority.
    pub fn sealed_inventory_attestation(&self) -> &Sys5SealedInventoryAttestation {
        &self.sealed_attestation
    }

    /// The checked literal owner operations that initialize every cell needed
    /// by this finite source fragment.  This is an observation of the sealed
    /// source/Core startup plan, not an initial-state injection seam.  The
    /// caller must still dispatch every returned operation through the
    /// generated runtime endpoint.
    pub(crate) fn source_derived_startup_operations_for_i2(&self) -> Option<Vec<String>> {
        if !self.startup_plan.is_complete() || self.startup_plan.initializers.is_empty() {
            return None;
        }
        let operations = self
            .startup_plan
            .initializers
            .iter()
            .map(|initializer| initializer.operation_id.clone())
            .collect::<Vec<_>>();
        let distinct = operations.iter().collect::<BTreeSet<_>>();
        (distinct.len() == operations.len()).then_some(operations)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn into_parts_for_sys4(self) -> (FabricProgram, SealedFabricAdmission) {
        (self.program, self.admission)
    }

    /// Start the ST-only local relation runtime from this exact sealed
    /// admission.  This consumes the retained projection and M9/SYS-4
    /// boundary; it cannot reparse source or accept an alternate route.
    pub fn start_relation_dispatch_runtime(
        self,
    ) -> Result<Sys5RelationDispatchRuntime, Sys5RelationDispatchError> {
        if self.summary.runtime_profile() != Sys5LocalRuntimeProfile::St {
            return Err(Sys5RelationDispatchError::new(
                Sys5RelationDispatchDiagnosticKind::BackendIneligible,
            ));
        }
        let relation_ids = self
            .inventory
            .relation_lifecycle
            .iter()
            .map(|row| row.relation.clone())
            .collect::<BTreeSet<_>>();
        let checked_program_identity_ref = self.summary.checked_program_identity_ref().to_string();
        let fabric = LocalFabric::bootstrap(self.program, self.admission, BackendProfile::St)
            .map_err(|_| {
                Sys5RelationDispatchError::new(
                    Sys5RelationDispatchDiagnosticKind::FabricBootRejected,
                )
            })?;
        Ok(Sys5RelationDispatchRuntime {
            fabric,
            relation_ids,
            checked_program_identity_ref,
        })
    }

    /// Start the bounded SYS-5 vertical slice on one admitted ST
    /// `LocalFabric`.  The caller cannot attach a second projection, M9 seam,
    /// route, or initial-state seed after this boundary.
    pub fn start_vertical_slice_runtime(
        self,
    ) -> Result<Sys5VerticalSliceRuntime, Sys5VerticalSliceError> {
        if self.summary.runtime_profile() != Sys5LocalRuntimeProfile::St {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::BackendIneligible,
            ));
        }
        if !self.startup_plan.is_complete() || !self.vertical_bindings.is_canonical_vertical_path()
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete,
            ));
        }
        let checked_program_identity_ref = self.summary.checked_program_identity_ref().to_string();
        let artifact_projection_ref = local_cut_ref(&format!("{:?}", self.program));
        let mut fabric = LocalFabric::bootstrap(self.program, self.admission, BackendProfile::St)
            .map_err(|_| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::FabricBootRejected)
        })?;
        let verification_discharges = self.summary.verification_discharges.clone();
        let mut joined_report = Sys5VerticalJoinedReport::new(verification_discharges);
        joined_report.push("auth:sealed-m9-source-derived".to_string());
        joined_report.push(format!(
            "checked-program-ref:{checked_program_identity_ref}"
        ));
        dispatch_vertical_startup_initializers(
            &mut fabric,
            &self.startup_plan,
            &mut joined_report,
        )?;
        Ok(Sys5VerticalSliceRuntime {
            fabric,
            fabric_instance_ref: relation_observer_ref(&checked_program_identity_ref),
            checked_program_identity_ref,
            sealed_admission_attestation_ref: self
                .summary
                .sealed_inventory_attestation_ref()
                .to_string(),
            artifact_projection_ref,
            admission_summary: self.summary,
            startup_plan: self.startup_plan,
            source_principal: self.source_principal,
            bindings: self.vertical_bindings,
            joined_report,
            relation_shadows: BTreeMap::new(),
            completed_participant_leaves: BTreeMap::new(),
            last_participant_leave_failure: None,
            next_lifecycle_occurrence: 0,
        })
    }

    /// Restore an exact SYS-5 wrapper into one fresh local fabric.  Wrapper
    /// identity is checked before SYS-4 is asked to restore; SYS-4 then
    /// performs its own program/admission/counter/M8/M9 preflight and returns
    /// a new fabric only after the complete cut has restored.
    pub fn restore_vertical_slice_runtime(
        self,
        cut: &Sys5LocalCut,
    ) -> Result<Sys5VerticalSliceRuntime, Sys5LocalCutPatchError> {
        if self.summary.runtime_profile() != Sys5LocalRuntimeProfile::St {
            return Err(Sys5LocalCutPatchError::new(
                Sys5LocalCutPatchErrorKind::BackendIneligible,
            ));
        }
        if !cut.validates_for_prepared(&self) {
            return Err(Sys5LocalCutPatchError::new(
                Sys5LocalCutPatchErrorKind::CutRejected,
            ));
        }
        let artifact_projection_ref = local_cut_ref(&format!("{:?}", self.program));
        let fabric = LocalFabric::restore_local_cut(
            self.program,
            self.admission,
            BackendProfile::St,
            &cut.sys4_cut,
        )
        .map_err(|_| Sys5LocalCutPatchError::new(Sys5LocalCutPatchErrorKind::CutRejected))?;
        let saved_frontier_ref = patch_frontier_ref(&format!(
            "{:?}",
            cut.sys4_cut.active_patch_frontier_snapshot()
        ));
        let restored_frontier_ref =
            patch_frontier_ref(&format!("{:?}", fabric.current_patch_frontier_snapshot()));
        if saved_frontier_ref != restored_frontier_ref {
            return Err(Sys5LocalCutPatchError::new(
                Sys5LocalCutPatchErrorKind::CutRejected,
            ));
        }
        let mut next_lifecycle_occurrence = cut.next_lifecycle_occurrence;
        let restore_occurrence_ref = next_lifecycle_occurrence_ref(
            &mut next_lifecycle_occurrence,
            "RestoreCut",
            &cut.cut_id_ref,
            &cut.sys4_cut_integrity_ref,
        )?;
        let mut joined_report = Sys5VerticalJoinedReport {
            rows: cut.joined_prefix.clone(),
            verification_discharges: self.summary.verification_discharges.clone(),
        };
        joined_report.push(lifecycle_joined_row(
            "RestoreCut",
            Sys5LifecycleBoundaryRefs {
                before_program_ref: &cut.checked_program_identity_ref,
                after_program_ref: &self.summary.checked_program_identity,
                before_artifact_ref: &cut.artifact_projection_ref,
                after_artifact_ref: &artifact_projection_ref,
                before_frontier_ref: &saved_frontier_ref,
                after_frontier_ref: &restored_frontier_ref,
            },
            Some(("restore_occurrence_ref", &restore_occurrence_ref)),
        ));
        Ok(Sys5VerticalSliceRuntime {
            fabric,
            fabric_instance_ref: relation_observer_ref(self.summary.checked_program_identity_ref()),
            checked_program_identity_ref: self.summary.checked_program_identity.clone(),
            sealed_admission_attestation_ref: self
                .summary
                .sealed_inventory_attestation_ref()
                .to_string(),
            artifact_projection_ref,
            admission_summary: self.summary,
            startup_plan: self.startup_plan,
            source_principal: self.source_principal,
            bindings: self.vertical_bindings,
            joined_report,
            relation_shadows: cut.relation_shadows.clone(),
            // This observer-safe M9/M8 receipt state is part of the checked
            // local continuation: fresh re-admission must bind the exact
            // prior leave, not reconstruct it from joined-row order.
            completed_participant_leaves: cut.completed_participant_leaves.clone(),
            last_participant_leave_failure: cut.last_participant_leave_failure.clone(),
            next_lifecycle_occurrence,
        })
    }
}

/// Execute the checked literal initializers after admission, through the same
/// local fabric that later owns the vertical actions.  The empty seed is
/// observed before every write; a missing, duplicate, mismatched, or
/// non-generated operation rejects startup without fabricating a receipt.
fn dispatch_vertical_startup_initializers(
    fabric: &mut LocalFabric,
    plan: &Sys5VerticalStartupPlan,
    joined_report: &mut Sys5VerticalJoinedReport,
) -> Result<(), Sys5VerticalSliceError> {
    if !plan.is_complete() || plan.initializers.len() != 3 {
        return Err(Sys5VerticalSliceError::new(
            Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete,
        ));
    }
    let mut seen_cells = BTreeSet::new();
    for initializer in &plan.initializers {
        if !seen_cells.insert(initializer.cell.clone())
            || fabric
                .semantic_snapshot()
                .int(
                    &initializer.cell.locus,
                    &initializer.cell.state,
                    &initializer.cell.index,
                    &initializer.cell.field,
                )
                .is_some()
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete,
            ));
        }
        let receipt = fabric
            .dispatch_source_action(SourceAction::owner_operation(&initializer.operation_id))
            .map_err(Sys5VerticalSliceError::from_dispatch)?;
        if receipt.operation_id() != initializer.operation_id
            || receipt.origin_locus() != initializer.source_locus
            || receipt.target_locus() != initializer.owner_locus
            || !receipt.owner_rmw_report().is_some_and(|report| {
                report.has_checked_source_core_provenance()
                    && report.has_exact_int_write(
                        &initializer.owner_locus,
                        &initializer.cell.state,
                        &initializer.cell.index,
                        &initializer.cell.field,
                        initializer.literal,
                    )
            })
            || fabric.semantic_snapshot().int(
                &initializer.cell.locus,
                &initializer.cell.state,
                &initializer.cell.index,
                &initializer.cell.field,
            ) != Some(initializer.literal)
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete,
            ));
        }
        let endpoint_occurrences = fabric
            .observer_exact_endpoint_occurrences(
                receipt.request_id(),
                crate::sys4_dispatch::Sys4TraceKind::Dispatched,
                crate::sys4_dispatch::Sys4TraceKind::Received,
                CommunicationEdgeKind::OwnerRequest,
                &initializer.source_locus,
                &initializer.owner_locus,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete)
            })?;
        let serve = fabric
            .observer_exact_m8_occurrence(
                receipt.request_id(),
                crate::m8_runtime_local_cut::M8LocalTraceKind::OwnerWrite,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete)
            })?;
        if !fabric.observer_causally_reaches(serve, endpoint_occurrences.receive_occurrence_id()) {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete,
            ));
        }
        let (logical_path, source_span) =
            observer_logical_source_span(endpoint_occurrences.source_ref()).ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::VerticalInventoryIncomplete)
            })?;
        let observer_safe_cell = plan.observer_safe_contains(
            &initializer.cell.locus,
            &initializer.cell.state,
            &initializer.cell.index,
            &initializer.cell.field,
        );
        let value = if observer_safe_cell {
            initializer.literal.to_string()
        } else {
            "[private]".to_string()
        };
        let cell = if observer_safe_cell {
            format!(
                "{}[{}].{}",
                initializer.cell.state, initializer.cell.index, initializer.cell.field
            )
        } else {
            format!(
                "private-cell-ref:{}",
                relation_observer_ref(&format!(
                    "{}:{}:{}:{}",
                    initializer.cell.locus,
                    initializer.cell.state,
                    initializer.cell.index,
                    initializer.cell.field,
                ))
            )
        };
        joined_report.push(format!(
            "startup-receipt:{}:{}->{}:{cell}:Created(None->{value})",
            initializer.operation_id, initializer.source_locus, initializer.owner_locus,
        ));
        joined_report.push(format!(
            "startup-occurrence:{}:{}",
            initializer.operation_id, serve,
        ));
        joined_report.push(format!(
            "typed-segment:owner-request:provenance_kind=OrdinarySourceCore;logical_path={logical_path};source_span={source_span};core_ref={};source_fragment_ref={};target_fragment_ref={};edge_ref={};request_identity={};request_enqueue_occurrence_id={};dispatch_occurrence_id={};receive_occurrence_id={};serve_occurrence_id={serve};causal_path=request_enqueue_occurrence_id->dispatch_occurrence_id->receive_occurrence_id->serve_occurrence_id",
            endpoint_occurrences.core_ref(),
            endpoint_occurrences.source_fragment_ref(),
            endpoint_occurrences.target_fragment_ref(),
            endpoint_occurrences.edge_ref(),
            receipt.request_id(),
            endpoint_occurrences.request_enqueue_occurrence_id(),
            endpoint_occurrences.dispatch_occurrence_id(),
            endpoint_occurrences.receive_occurrence_id(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5RelationDispatchEventKind {
    PublishCurrent,
    InvalidatePrimary,
    ViewerPresentationGap,
    FreshReacquire,
}

/// A bounded schedule action for an already source-derived relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RelationAction {
    relation: String,
    event_kind: Sys5RelationDispatchEventKind,
}

impl Sys5RelationAction {
    pub fn publish_current(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
            event_kind: Sys5RelationDispatchEventKind::PublishCurrent,
        }
    }

    pub fn invalidate_primary(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
            event_kind: Sys5RelationDispatchEventKind::InvalidatePrimary,
        }
    }

    pub fn viewer_presentation_gap(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
            event_kind: Sys5RelationDispatchEventKind::ViewerPresentationGap,
        }
    }

    pub fn fresh_reacquire(relation: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
            event_kind: Sys5RelationDispatchEventKind::FreshReacquire,
        }
    }
}

/// Active ST fabric state for the bounded SYS-5 maintained-relation path.
pub struct Sys5RelationDispatchRuntime {
    fabric: LocalFabric,
    relation_ids: BTreeSet<String>,
    checked_program_identity_ref: String,
}

impl fmt::Debug for Sys5RelationDispatchRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sys5RelationDispatchRuntime")
            .field("relation_count", &self.relation_ids.len())
            .field("status", &"source-derived-st-local")
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5RelationDispatchDiagnosticKind {
    UnknownSourceRelation,
    BackendIneligible,
    FabricBootRejected,
    RelationTransitionRejected,
}

/// Observer-safe status for a generated relation-publication attempt that
/// failed before owner publication sequence commit.  It records whether the
/// exact pending carrier was discarded for a retry; it is never authority or
/// a transport-derived semantic fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5RelationPublicationFailureDisposition {
    DiscardedUndelivered,
    AlreadyRemovedByTransport,
}

impl From<RelationPublicationFailureDisposition> for Sys5RelationPublicationFailureDisposition {
    fn from(value: RelationPublicationFailureDisposition) -> Self {
        match value {
            RelationPublicationFailureDisposition::DiscardedUndelivered => {
                Self::DiscardedUndelivered
            }
            RelationPublicationFailureDisposition::AlreadyRemovedByTransport => {
                Self::AlreadyRemovedByTransport
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RelationDispatchError {
    kind: Sys5RelationDispatchDiagnosticKind,
    publication_failure_disposition: Option<Sys5RelationPublicationFailureDisposition>,
}

impl Sys5RelationDispatchError {
    fn new(kind: Sys5RelationDispatchDiagnosticKind) -> Self {
        Self {
            kind,
            publication_failure_disposition: None,
        }
    }

    fn from_sys4(diagnostics: Sys4DispatchDiagnostics) -> Self {
        Self {
            kind: Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected,
            publication_failure_disposition: diagnostics
                .relation_publication_failure_disposition()
                .map(Into::into),
        }
    }

    pub const fn kind(&self) -> Sys5RelationDispatchDiagnosticKind {
        self.kind
    }

    pub const fn rejected_before_generated_endpoint(&self) -> bool {
        matches!(
            self.kind,
            Sys5RelationDispatchDiagnosticKind::UnknownSourceRelation
        )
    }

    pub const fn rejected_before_m9_authority_use(&self) -> bool {
        matches!(
            self.kind,
            Sys5RelationDispatchDiagnosticKind::UnknownSourceRelation
        )
    }

    pub const fn rejected_before_m8_relation_transition(&self) -> bool {
        matches!(
            self.kind,
            Sys5RelationDispatchDiagnosticKind::UnknownSourceRelation
        )
    }

    pub const fn partial_relation_receipt(&self) -> Option<()> {
        None
    }

    /// A typed, observer-safe terminal status for an uncommitted relation
    /// publication.  The absence of this status means either no endpoint was
    /// attempted or a different relation validation failed.
    pub const fn publication_failure_disposition(
        &self,
    ) -> Option<Sys5RelationPublicationFailureDisposition> {
        self.publication_failure_disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5RelationProjectionKind {
    SemanticImportedShadow,
    PresentationFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RelationEndpointChain {
    edge_kind: CommunicationEdgeKind,
    source_locus: String,
    target_locus: String,
    source_ref: String,
    logical_path: String,
    source_span: String,
    owner_publish_occurrence_id: String,
    request_identity: String,
    request_enqueue_occurrence_id: String,
    dispatch_occurrence_id: String,
    receive_occurrence_id: String,
    consumer_observe_occurrence_id: String,
    serve_occurrence_id: String,
    edge_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    core_ref: Option<String>,
}

impl Sys5RelationEndpointChain {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) const fn edge_kind(&self) -> CommunicationEdgeKind {
        self.edge_kind
    }
    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }
    pub fn target_locus(&self) -> &str {
        &self.target_locus
    }
    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }
    pub fn owner_publish_occurrence_id(&self) -> &str {
        &self.owner_publish_occurrence_id
    }
    /// The generated request's source-derived identity.  This deliberately
    /// remains distinct from the concrete endpoint occurrences below.
    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    /// Actual source outbox occurrence for the generated request.  This is
    /// distinct from `request_identity`, which is the source-derived
    /// request identity shown to the external schedule.
    pub fn request_enqueue_occurrence_id(&self) -> &str {
        &self.request_enqueue_occurrence_id
    }
    pub fn dispatch_occurrence_id(&self) -> &str {
        &self.dispatch_occurrence_id
    }
    pub fn receive_occurrence_id(&self) -> &str {
        &self.receive_occurrence_id
    }
    pub fn consumer_observe_occurrence_id(&self) -> &str {
        &self.consumer_observe_occurrence_id
    }
    pub fn serve_occurrence_id(&self) -> &str {
        &self.serve_occurrence_id
    }
    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }
    pub fn source_fragment_ref(&self) -> &str {
        &self.source_fragment_ref
    }
    pub fn target_fragment_ref(&self) -> &str {
        &self.target_fragment_ref
    }
    pub fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RelationObserverShadow {
    relation: String,
    owner_locus: String,
    consumer_locus: String,
    selected_anchor: String,
    selected_floor: String,
    lineage_ref: String,
    semantic_digest: String,
    semantic_epoch: String,
}

impl Sys5RelationObserverShadow {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }
    pub fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }
    pub fn selected_anchor(&self) -> &str {
        &self.selected_anchor
    }
    pub fn selected_floor(&self) -> &str {
        &self.selected_floor
    }
    pub fn lineage_ref(&self) -> &str {
        &self.lineage_ref
    }
    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }
    pub fn semantic_epoch(&self) -> &str {
        &self.semantic_epoch
    }
    pub const fn capability_and_witness_are_redacted(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RelationDispatchReceipt {
    event_kind: Sys5RelationDispatchEventKind,
    endpoint_chain: Option<Sys5RelationEndpointChain>,
    shadow: Sys5RelationObserverShadow,
    viewer_projection_kind: Sys5RelationProjectionKind,
    checked_program_identity_ref: String,
    observer_safe_report: String,
}

impl Sys5RelationDispatchReceipt {
    pub const fn event_kind(&self) -> Sys5RelationDispatchEventKind {
        self.event_kind
    }

    pub fn single_endpoint_chain(&self) -> &Sys5RelationEndpointChain {
        self.endpoint_chain
            .as_ref()
            .expect("only generated relation dispatches have an endpoint chain")
    }

    pub fn observer_shadow(
        &self,
        consumer_locus: &str,
        relation: &str,
    ) -> Option<&Sys5RelationObserverShadow> {
        (self.shadow.consumer_locus == consumer_locus && self.shadow.relation == relation)
            .then_some(&self.shadow)
    }

    pub const fn viewer_projection_kind(&self) -> Sys5RelationProjectionKind {
        self.viewer_projection_kind
    }

    pub fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity_ref
    }

    pub fn observer_safe_report(&self) -> &str {
        &self.observer_safe_report
    }

    fn compose_observer_safe_report(&self) -> String {
        let mut rows = vec![
            format!("checked-program-ref:{}", self.checked_program_identity_ref),
            format!("relation-owner:{}", self.shadow.owner_locus),
            format!("relation-consumer:{}", self.shadow.consumer_locus),
            format!("relation-anchor:{}", self.shadow.selected_anchor),
            format!("relation-floor:{}", self.shadow.selected_floor),
            format!("relation-lineage-ref:{}", self.shadow.lineage_ref),
            format!("relation-semantic-digest:{}", self.shadow.semantic_digest),
        ];
        if let Some(chain) = &self.endpoint_chain {
            rows.extend([
                format!("source-ref:{}", chain.source_ref),
                format!("core-ref:{}", chain.core_ref.as_deref().unwrap_or("")),
                format!("artifact-ref:{}", chain.source_fragment_ref),
                format!("edge-ref:{}", chain.edge_ref),
                format!("publish:{}", chain.owner_publish_occurrence_id),
                format!("request:{}", chain.request_identity),
                format!("request-enqueue:{}", chain.request_enqueue_occurrence_id),
                format!("dispatch:{}", chain.dispatch_occurrence_id),
                format!("receive:{}", chain.receive_occurrence_id),
                format!("observe:{}", chain.consumer_observe_occurrence_id),
                format!("serve:{}", chain.serve_occurrence_id),
                format!(
                    "typed-segment:relation-projection-publication:provenance_kind=OrdinarySourceCore;logical_path={};source_span={};core_ref={};source_fragment_ref={};target_fragment_ref={};edge_ref={};request_identity={};owner_publish_occurrence_id={};request_enqueue_occurrence_id={};dispatch_occurrence_id={};receive_occurrence_id={};consumer_observe_occurrence_id={};serve_occurrence_id={};causal_path=owner_publish_occurrence_id->request_enqueue_occurrence_id->dispatch_occurrence_id->receive_occurrence_id->consumer_observe_occurrence_id->serve_occurrence_id",
                    chain.logical_path,
                    chain.source_span,
                    chain.core_ref.as_deref().unwrap_or(""),
                    chain.source_fragment_ref,
                    chain.target_fragment_ref,
                    chain.edge_ref,
                    chain.request_identity,
                    chain.owner_publish_occurrence_id,
                    chain.request_enqueue_occurrence_id,
                    chain.dispatch_occurrence_id,
                    chain.receive_occurrence_id,
                    chain.consumer_observe_occurrence_id,
                    chain.serve_occurrence_id,
                ),
            ]);
        }
        rows.sort();
        rows.join("\n")
    }
}

impl Sys5RelationDispatchRuntime {
    pub fn dispatch_relation(
        &mut self,
        action: Sys5RelationAction,
    ) -> Result<Sys5RelationDispatchReceipt, Sys5RelationDispatchError> {
        if !self.relation_ids.contains(&action.relation) {
            return Err(Sys5RelationDispatchError::new(
                Sys5RelationDispatchDiagnosticKind::UnknownSourceRelation,
            ));
        }
        match action.event_kind {
            Sys5RelationDispatchEventKind::PublishCurrent => self
                .fabric
                .publish_relation_current(&action.relation)
                .map_err(Sys5RelationDispatchError::from_sys4)
                .and_then(|receipt| self.endpoint_receipt(action.event_kind, receipt)),
            Sys5RelationDispatchEventKind::InvalidatePrimary => self
                .fabric
                .invalidate_relation_primary(&action.relation)
                .map_err(Sys5RelationDispatchError::from_sys4)
                .and_then(|receipt| self.endpoint_receipt(action.event_kind, receipt)),
            Sys5RelationDispatchEventKind::FreshReacquire => self
                .fabric
                .fresh_reacquire_relation_primary(&action.relation)
                .map_err(Sys5RelationDispatchError::from_sys4)
                .and_then(|receipt| self.endpoint_receipt(action.event_kind, receipt)),
            Sys5RelationDispatchEventKind::ViewerPresentationGap => {
                self.presentation_gap_receipt(&action.relation)
            }
        }
    }

    pub fn relation_semantic_digest(&self, relation: &str) -> Option<String> {
        self.fabric
            .relation_semantic_digest(relation)
            .map(ToOwned::to_owned)
    }

    pub fn endpoint_carrier_count_for_relation(&self, relation: &str) -> usize {
        self.fabric.endpoint_carrier_count_for_relation(relation)
    }

    pub fn total_endpoint_carrier_count(&self) -> usize {
        self.fabric.total_endpoint_carrier_count()
    }

    pub fn observer_safe_relation_state(&self) -> Vec<String> {
        self.relation_ids
            .iter()
            .filter_map(|relation| {
                self.fabric
                    .relation_semantic_digest(relation)
                    .map(|digest| format!("{relation}:{digest}"))
            })
            .collect()
    }

    fn endpoint_receipt(
        &self,
        event_kind: Sys5RelationDispatchEventKind,
        receipt: Sys4RelationEndpointReceipt,
    ) -> Result<Sys5RelationDispatchReceipt, Sys5RelationDispatchError> {
        if !self
            .fabric
            .observer_exact_relation_endpoint_receipt(&receipt)
        {
            return Err(Sys5RelationDispatchError::new(
                Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let edge = receipt.edge();
        let (logical_path, source_span) = observer_logical_source_span(&edge.source_ref())
            .ok_or_else(|| {
                Sys5RelationDispatchError::new(
                    Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected,
                )
            })?;
        let shadow = receipt.shadow();
        let semantic = shadow.semantic();
        let observer_shadow = Sys5RelationObserverShadow {
            relation: shadow.relation().to_string(),
            owner_locus: shadow.owner_locus().to_string(),
            consumer_locus: shadow.consumer_locus().to_string(),
            selected_anchor: semantic.selected_anchor().to_string(),
            selected_floor: match semantic.selected_floor() {
                crate::m8_runtime_owner_queue::M8RelationFloor::Live => "live-primary".to_string(),
                crate::m8_runtime_owner_queue::M8RelationFloor::Anchor => {
                    "fallback-anchor".to_string()
                }
                crate::m8_runtime_owner_queue::M8RelationFloor::Frozen => {
                    "frozen-fallback".to_string()
                }
            },
            lineage_ref: relation_observer_ref(&semantic.lineage().join("\n")),
            semantic_digest: relation_observer_ref(&shadow.semantic_digest()),
            semantic_epoch: semantic.binding_epoch().to_string(),
        };
        let chain = Sys5RelationEndpointChain {
            edge_kind: edge.kind(),
            source_locus: edge.source_locus().to_string(),
            target_locus: edge.target_locus().to_string(),
            source_ref: observer_source_ref(&edge.source_ref()),
            logical_path,
            source_span,
            owner_publish_occurrence_id: receipt.owner_publish_occurrence_id().to_string(),
            request_identity: receipt.request_id().to_string(),
            request_enqueue_occurrence_id: receipt.request_enqueue_occurrence_id().to_string(),
            dispatch_occurrence_id: receipt
                .transport()
                .source_outbox_dequeue_occurrence_id()
                .to_string(),
            receive_occurrence_id: receipt
                .transport()
                .target_inbox_enqueue_occurrence_id()
                .to_string(),
            consumer_observe_occurrence_id: receipt.consumer_observe_occurrence_id().to_string(),
            serve_occurrence_id: receipt.consumer_serve_occurrence_id().to_string(),
            edge_ref: edge.edge_ref().to_string(),
            source_fragment_ref: edge.source_fragment_ref().clone(),
            target_fragment_ref: edge.target_fragment_ref().clone(),
            core_ref: edge.core_ref().map(ToOwned::to_owned),
        };
        let mut receipt = Sys5RelationDispatchReceipt {
            event_kind,
            endpoint_chain: Some(chain),
            shadow: observer_shadow,
            viewer_projection_kind: Sys5RelationProjectionKind::SemanticImportedShadow,
            checked_program_identity_ref: self.checked_program_identity_ref.clone(),
            observer_safe_report: String::new(),
        };
        receipt.observer_safe_report = receipt.compose_observer_safe_report();
        Ok(receipt)
    }

    fn presentation_gap_receipt(
        &self,
        relation: &str,
    ) -> Result<Sys5RelationDispatchReceipt, Sys5RelationDispatchError> {
        let projection = self
            .fabric
            .project_relation_presentation_gap(relation)
            .map_err(|_| {
                Sys5RelationDispatchError::new(
                    Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected,
                )
            })?;
        let shadow = self
            .fabric
            .relation_imported_shadow(relation, projection.consumer_locus())
            .map_err(|_| {
                Sys5RelationDispatchError::new(
                    Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected,
                )
            })?
            .ok_or_else(|| {
                Sys5RelationDispatchError::new(
                    Sys5RelationDispatchDiagnosticKind::RelationTransitionRejected,
                )
            })?;
        let semantic = shadow.semantic();
        let mut receipt = Sys5RelationDispatchReceipt {
            event_kind: Sys5RelationDispatchEventKind::ViewerPresentationGap,
            endpoint_chain: None,
            shadow: Sys5RelationObserverShadow {
                relation: shadow.relation().to_string(),
                owner_locus: shadow.owner_locus().to_string(),
                consumer_locus: shadow.consumer_locus().to_string(),
                selected_anchor: semantic.selected_anchor().to_string(),
                selected_floor: match semantic.selected_floor() {
                    crate::m8_runtime_owner_queue::M8RelationFloor::Live => {
                        "live-primary".to_string()
                    }
                    crate::m8_runtime_owner_queue::M8RelationFloor::Anchor => {
                        "fallback-anchor".to_string()
                    }
                    crate::m8_runtime_owner_queue::M8RelationFloor::Frozen => {
                        "frozen-fallback".to_string()
                    }
                },
                lineage_ref: relation_observer_ref(&semantic.lineage().join("\n")),
                semantic_digest: relation_observer_ref(&shadow.semantic_digest()),
                semantic_epoch: semantic.binding_epoch().to_string(),
            },
            viewer_projection_kind: Sys5RelationProjectionKind::PresentationFallback,
            checked_program_identity_ref: self.checked_program_identity_ref.clone(),
            observer_safe_report: String::new(),
        };
        receipt.observer_safe_report = receipt.compose_observer_safe_report();
        Ok(receipt)
    }
}

/// Observer-safe provenance held by a SYS-5 local cut.  It records only the
/// fact that the preserved runtime is bound to the source/Core/artifact
/// chain; identifiers themselves remain opaque references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5LocalCutProvenance {
    source_core_artifact_bound: bool,
}

impl Sys5LocalCutProvenance {
    pub const fn is_source_core_artifact_bound(&self) -> bool {
        self.source_core_artifact_bound
    }
}

/// Typed corruption choices for tests of the private, bounded local-cut
/// wrapper.  They never accept raw M8/M9 material as input.
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5CutCorruptionKind {
    WrapperIdentity,
    SourceProgramIdentity,
    ArtifactProjectionIdentity,
    CounterRollback,
    RelationDigest,
    LifecycleOccurrenceCounter,
    ParticipantLeaveEvidence,
}

/// A SYS-5 wrapper around the exact SYS-4 process-local cut.  It is neither
/// a durable save format nor a public transport/wire contract.  The wrapper
/// carries observer-safe source/admission metadata and the exact joined event
/// prefix, while SYS-4 remains the owner of actual fabric/M8/M9 state.
#[derive(Clone)]
pub struct Sys5LocalCut {
    cut_id_ref: String,
    checked_program_identity_ref: String,
    sealed_admission_attestation_ref: String,
    artifact_projection_ref: String,
    startup_plan_ref: String,
    bindings_ref: String,
    source_principal_ref: String,
    joined_prefix: Vec<String>,
    relation_shadows: BTreeMap<(String, String), Sys5RelationObserverShadow>,
    completed_participant_leaves: BTreeMap<String, Sys5ParticipantLeaveEvidence>,
    last_participant_leave_failure: Option<Sys5ParticipantLeaveFailureEvidence>,
    sys4_cut_integrity_ref: String,
    next_lifecycle_occurrence: u64,
    integrity_ref: String,
    sys4_cut: Sys4LocalCut,
}

impl fmt::Debug for Sys5LocalCut {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sys5LocalCut")
            .field("cut_id_ref", &self.cut_id_ref)
            .field(
                "checked_program_identity_ref",
                &self.checked_program_identity_ref,
            )
            .field(
                "sealed_admission_attestation_ref",
                &self.sealed_admission_attestation_ref,
            )
            .field("artifact_projection_ref", &self.artifact_projection_ref)
            .field("joined_prefix_len", &self.joined_prefix.len())
            .field(
                "completed_participant_leave_count",
                &self.completed_participant_leaves.len(),
            )
            .field(
                "has_last_participant_leave_failure",
                &self.last_participant_leave_failure.is_some(),
            )
            .field("integrity_ref", &self.integrity_ref)
            .field("status", &"bounded-private-local-cut")
            .finish()
    }
}

impl Sys5LocalCut {
    fn new(cut_id: &str, runtime: &Sys5VerticalSliceRuntime, sys4_cut: Sys4LocalCut) -> Self {
        let cut_id_ref = local_cut_ref(cut_id);
        let startup_plan_ref = local_cut_ref(&format!("{:?}", runtime.startup_plan));
        let bindings_ref = local_cut_ref(&format!("{:?}", runtime.bindings));
        let source_principal_ref = local_cut_ref(&runtime.source_principal);
        let sys4_cut_integrity_ref = local_cut_ref(&sys4_cut.observer_safe_integrity_material());
        let mut cut = Self {
            cut_id_ref,
            checked_program_identity_ref: runtime.checked_program_identity_ref.clone(),
            sealed_admission_attestation_ref: runtime.sealed_admission_attestation_ref.clone(),
            artifact_projection_ref: runtime.artifact_projection_ref.clone(),
            startup_plan_ref,
            bindings_ref,
            source_principal_ref,
            joined_prefix: runtime.joined_report.rows.clone(),
            relation_shadows: runtime.relation_shadows.clone(),
            completed_participant_leaves: runtime.completed_participant_leaves.clone(),
            last_participant_leave_failure: runtime.last_participant_leave_failure.clone(),
            sys4_cut_integrity_ref,
            next_lifecycle_occurrence: runtime.next_lifecycle_occurrence,
            integrity_ref: String::new(),
            sys4_cut,
        };
        cut.integrity_ref = cut.compute_integrity_ref();
        cut
    }

    fn compute_integrity_ref(&self) -> String {
        local_cut_ref(&format!(
            "cut={};program={};admission={};artifact={};startup={};bindings={};principal={};prefix={:?};shadows={:?};completed_participant_leaves={:?};last_participant_leave_failure={:?};sys4={};next_lifecycle_occurrence={}",
            self.cut_id_ref,
            self.checked_program_identity_ref,
            self.sealed_admission_attestation_ref,
            self.artifact_projection_ref,
            self.startup_plan_ref,
            self.bindings_ref,
            self.source_principal_ref,
            self.joined_prefix,
            self.relation_shadows,
            self.completed_participant_leaves,
            self.last_participant_leave_failure,
            self.sys4_cut_integrity_ref,
            self.next_lifecycle_occurrence,
        ))
    }

    fn validates_for_prepared(&self, prepared: &Sys5PreparedAdmission) -> bool {
        self.integrity_ref == self.compute_integrity_ref()
            && self.checked_program_identity_ref == prepared.summary.checked_program_identity_ref()
            && self.sealed_admission_attestation_ref
                == prepared.summary.sealed_inventory_attestation_ref()
            && self.artifact_projection_ref == local_cut_ref(&format!("{:?}", prepared.program))
            && self.startup_plan_ref == local_cut_ref(&format!("{:?}", prepared.startup_plan))
            && self.bindings_ref == local_cut_ref(&format!("{:?}", prepared.vertical_bindings))
            && self.source_principal_ref == local_cut_ref(&prepared.source_principal)
            && self.sys4_cut.has_valid_private_restore_integrity()
            && self.sys4_cut_integrity_ref
                == local_cut_ref(&self.sys4_cut.observer_safe_integrity_material())
    }

    pub fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity_ref
    }

    pub fn sealed_admission_attestation_ref(&self) -> &str {
        &self.sealed_admission_attestation_ref
    }

    pub const fn covers_owner_relation_designated_cache_m9_verification_and_counters(
        &self,
    ) -> bool {
        true
    }

    pub const fn observer_safe_provenance(&self) -> Sys5LocalCutProvenance {
        Sys5LocalCutProvenance {
            source_core_artifact_bound: true,
        }
    }

    #[cfg(test)]
    pub fn for_test_corrupt(mut self, kind: Sys5CutCorruptionKind) -> Self {
        match kind {
            Sys5CutCorruptionKind::WrapperIdentity => {
                self.cut_id_ref = local_cut_ref("corrupt-wrapper-identity");
            }
            Sys5CutCorruptionKind::SourceProgramIdentity => {
                self.checked_program_identity_ref = local_cut_ref("corrupt-source-program");
            }
            Sys5CutCorruptionKind::ArtifactProjectionIdentity => {
                self.artifact_projection_ref = local_cut_ref("corrupt-artifact-projection");
            }
            Sys5CutCorruptionKind::CounterRollback => {
                self.sys4_cut.for_test_set_next_request_below_retained_max(
                    "sys4-request-00000000000000000000",
                );
            }
            Sys5CutCorruptionKind::RelationDigest => {
                self.sys4_cut.for_test_set_relation_semantic_digest(
                    "bird_follow",
                    "corrupt-relation-digest",
                );
            }
            Sys5CutCorruptionKind::LifecycleOccurrenceCounter => {
                self.next_lifecycle_occurrence = u64::MAX;
            }
            Sys5CutCorruptionKind::ParticipantLeaveEvidence => {
                self.completed_participant_leaves.clear();
            }
        }
        self
    }

    /// Test-only positive seam: alter only the persisted lifecycle cursor and
    /// re-sign the private wrapper so restore reaches the checked allocator.
    /// This never accepts caller-supplied Core, authority, state, or payload.
    #[cfg(test)]
    pub fn for_test_with_valid_lifecycle_occurrence_counter(mut self, counter: u64) -> Self {
        self.next_lifecycle_occurrence = counter;
        self.integrity_ref = self.compute_integrity_ref();
        self
    }

    /// Test-only bounded corruption seam routed to the private SYS-4 cut.
    /// It never exposes an owner store, Core, authority, capability, or
    /// witness on the SYS-5 production surface.
    #[cfg(test)]
    pub fn for_test_tamper_owner_state_value(
        mut self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
        value: i64,
    ) -> Self {
        self.sys4_cut
            .for_test_tamper_owner_state_value(locus, state, index, field, value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5LocalCutPatchErrorKind {
    CutRejected,
    PatchCandidateRejected,
    BackendIneligible,
    LifecycleOccurrenceExhausted,
}

/// A failure at the SYS-5 cut/patch boundary.  It deliberately has no
/// partial runtime handle and carries no source, capability, witness, or M9
/// authority payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5LocalCutPatchError {
    kind: Sys5LocalCutPatchErrorKind,
}

impl Sys5LocalCutPatchError {
    fn new(kind: Sys5LocalCutPatchErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5LocalCutPatchErrorKind {
        self.kind
    }

    pub const fn rejected_before_partial_runtime(&self) -> bool {
        true
    }

    pub const fn partial_runtime(&self) -> Option<()> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5LocalPatchBoundaryInspection {
    caller_supplied_core_authority_or_frontier: bool,
    runtime_received_only_checked_patch_candidate: bool,
}

impl Sys5LocalPatchBoundaryInspection {
    pub const fn caller_supplied_no_core_authority_or_frontier(&self) -> bool {
        !self.caller_supplied_core_authority_or_frontier
    }

    pub const fn runtime_received_only_checked_patch_candidate(&self) -> bool {
        self.runtime_received_only_checked_patch_candidate
    }
}

/// Source-first candidate wrapper.  Construction consumes an ordinary
/// checked/projected source and a matching sealed M9 admission; callers have
/// no API for injecting Core, authority, or an activation frontier.
pub struct Sys5LocalPatchCandidate {
    patch_id_ref: String,
    patch_summary: Sys5AdmissionSummary,
    patch_startup_plan: Sys5VerticalStartupPlan,
    patch_bindings: Sys5VerticalBindings,
    patch_source_principal: String,
    patch_artifact_projection_ref: String,
    inner: Sys4CheckedPatchCandidate,
}

impl fmt::Debug for Sys5LocalPatchCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sys5LocalPatchCandidate")
            .field("patch_id_ref", &self.patch_id_ref)
            .field(
                "checked_program_identity_ref",
                &self.patch_summary.checked_program_identity_ref(),
            )
            .field(
                "artifact_projection_ref",
                &self.patch_artifact_projection_ref,
            )
            .field("status", &"prechecked-projected-sealed-admission")
            .finish()
    }
}

impl Sys5LocalPatchCandidate {
    pub fn from_source_project_and_admission(
        patch_id: impl Into<String>,
        runtime: &Sys5VerticalSliceRuntime,
        project: Sys5LocalProject,
        prepared: Sys5PreparedAdmission,
    ) -> Result<Self, Sys5LocalCutPatchError> {
        let patch_id = patch_id.into();
        if patch_id.is_empty()
            || project.checked_program_identity_ref()
                != prepared.summary.checked_program_identity_ref()
            || prepared.summary.runtime_profile() != Sys5LocalRuntimeProfile::St
            || !prepared.summary.is_complete_for_projection()
        {
            return Err(Sys5LocalCutPatchError::new(
                Sys5LocalCutPatchErrorKind::PatchCandidateRejected,
            ));
        }
        let artifact_projection_ref = local_cut_ref(&format!("{:?}", prepared.program));
        let inner = Sys4CheckedPatchCandidate::from_prechecked_projected_admitted(
            &patch_id,
            runtime.fabric.active_program_for_checked_patch(),
            prepared.program.clone(),
            prepared.admission.clone(),
        )
        .map_err(|_| {
            Sys5LocalCutPatchError::new(Sys5LocalCutPatchErrorKind::PatchCandidateRejected)
        })?;
        Ok(Self {
            patch_id_ref: local_cut_ref(&patch_id),
            patch_summary: prepared.summary,
            patch_startup_plan: prepared.startup_plan,
            patch_bindings: prepared.vertical_bindings,
            patch_source_principal: prepared.source_principal,
            patch_artifact_projection_ref: artifact_projection_ref,
            inner,
        })
    }

    pub const fn boundary_inspection(&self) -> Sys5LocalPatchBoundaryInspection {
        Sys5LocalPatchBoundaryInspection {
            caller_supplied_core_authority_or_frontier: false,
            runtime_received_only_checked_patch_candidate: true,
        }
    }

    pub fn admission_summary(&self) -> &Sys5AdmissionSummary {
        &self.patch_summary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5PatchVerdict {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5PatchDiagnosticKind {
    StaleFrontier,
    NonQuiescentPendingCarrier,
    TopologyOwnerRouteMismatch,
    OwnerRmwExpressionChanged,
    NonDesignatedCoreMaterialChanged,
    M9AuthorityLineageMismatch,
    IncompleteCandidateAdmission,
    BackendIneligible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5PatchFrontier {
    ref_digest: String,
    predecessor_ref_digest: Option<String>,
}

impl Sys5PatchFrontier {
    pub fn is_exact_successor_of(&self, base: &Self) -> bool {
        self.predecessor_ref_digest.as_deref() == Some(base.ref_digest.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5PatchLifecycle {
    verdict: Sys5PatchVerdict,
    diagnostic: Option<Sys5PatchDiagnosticKind>,
    source_first_checked_projection_and_m9_admission: bool,
}

impl Sys5PatchLifecycle {
    pub const fn contains_source_first_checked_projection_and_m9_admission(&self) -> bool {
        self.source_first_checked_projection_and_m9_admission
    }

    pub fn is_lifecycle_only_rejection(&self) -> bool {
        self.verdict == Sys5PatchVerdict::Rejected && self.diagnostic.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5PatchOutcome {
    verdict: Sys5PatchVerdict,
    primary_diagnostic_kind: Option<Sys5PatchDiagnosticKind>,
    patch_occurrence_ref: String,
    lifecycle: Sys5PatchLifecycle,
    boundary_inspection: Sys5LocalPatchBoundaryInspection,
    base_frontier: Sys5PatchFrontier,
    activation_frontier: Sys5PatchFrontier,
}

impl Sys5PatchOutcome {
    pub const fn verdict(&self) -> Sys5PatchVerdict {
        self.verdict
    }

    pub const fn primary_diagnostic_kind(&self) -> Option<Sys5PatchDiagnosticKind> {
        self.primary_diagnostic_kind
    }

    /// Exact observer-safe lifecycle occurrence allocated by the active
    /// runtime for this accepted or rejected patch transition.
    pub fn patch_occurrence_ref(&self) -> &str {
        &self.patch_occurrence_ref
    }

    pub fn lifecycle(&self) -> &Sys5PatchLifecycle {
        &self.lifecycle
    }

    pub fn boundary_inspection(&self) -> &Sys5LocalPatchBoundaryInspection {
        &self.boundary_inspection
    }

    pub fn base_frontier(&self) -> &Sys5PatchFrontier {
        &self.base_frontier
    }

    pub fn activation_frontier(&self) -> &Sys5PatchFrontier {
        &self.activation_frontier
    }
}

/// Observer-safe status of the retained M9 admission. It intentionally does
/// not serialize membership epochs, capabilities, credentials, witnesses, or
/// authority payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RuntimeM9Summary {
    complete_final_residual_discharge: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5RuntimeVerificationSummary {
    discharged_verifiers: BTreeSet<String>,
}

impl Sys5RuntimeVerificationSummary {
    pub fn is_discharged(&self, verifier: &str) -> bool {
        self.discharged_verifiers.contains(verifier)
    }
}

impl Sys5RuntimeM9Summary {
    pub const fn has_complete_final_residual_discharge(&self) -> bool {
        self.complete_final_residual_discharge
    }
}

/// Observer-safe semantic snapshot of one SYS-5 local fabric.  This is a
/// comparison aid for tests and devtools, not a mutable runtime state handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5ObserverSafeRuntimeSnapshot {
    observer_safe_ints: BTreeMap<(String, String, String, String), i64>,
    state_digest: String,
    designated_cache_counts: BTreeMap<(String, String), usize>,
    fresh_relation_bindings: BTreeSet<String>,
    relation_digests: BTreeMap<String, String>,
    m9_summary: Sys5RuntimeM9Summary,
    verification_summary: Sys5RuntimeVerificationSummary,
}

impl Sys5ObserverSafeRuntimeSnapshot {
    pub fn owner_state_contains_int(
        &self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
        value: i64,
    ) -> bool {
        self.observer_safe_ints.get(&(
            locus.to_string(),
            state.to_string(),
            index.to_string(),
            field.to_string(),
        )) == Some(&value)
    }

    pub fn designated_cache_contains(&self, value_name: &str, consumer: &str) -> bool {
        self.designated_cache_counts
            .get(&(value_name.to_string(), consumer.to_string()))
            .is_some_and(|count| *count > 0)
    }

    pub fn relation_binding_consumed_fresh(&self, relation: &str) -> bool {
        self.fresh_relation_bindings.contains(relation)
    }

    pub fn m9_summary(&self) -> &Sys5RuntimeM9Summary {
        &self.m9_summary
    }

    pub fn verification_summary(&self) -> &Sys5RuntimeVerificationSummary {
        &self.verification_summary
    }
}

/// Opaque, observer-safe identity of the active runtime configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5ActiveRuntimeIdentitySnapshot {
    runtime_ref: String,
}

/// Small external schedule surface for the finite SYS-5 vertical slice.
/// Every variant resolves through the retained checked source inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sys5VerticalAction {
    ParticipantAAttackDeclaredTarget,
    WorldTick(String),
    ViewerCConsumeWorldResult,
    PublishRelation(String),
    ViewerCPresentationGap(String),
    /// Retire the source-declared membership at a relation's explicit primary
    /// anchor, then dispatch the generated owner-side fallback publication.
    /// The external schedule supplies only the checked relation name.
    ParticipantALeaveRelationPrimary(String),
    InvalidateRelationPrimary(String),
    FreshReacquireRelationPrimary(String),
    RevokeViewerCConsumerCapability(String),
    #[cfg(test)]
    ForTestUnknownSourceOperation(String),
    #[cfg(test)]
    ForTestUnknownDesignatedValue(String),
}

impl Sys5VerticalAction {
    pub const fn participant_a_attack_declared_target() -> Self {
        Self::ParticipantAAttackDeclaredTarget
    }

    pub fn world_tick(tick: impl Into<String>) -> Self {
        Self::WorldTick(tick.into())
    }

    pub const fn viewer_c_consume_world_result() -> Self {
        Self::ViewerCConsumeWorldResult
    }

    pub fn publish_relation(relation: impl Into<String>) -> Self {
        Self::PublishRelation(relation.into())
    }

    /// Execute a consumer-local presentation fallback against an already
    /// imported relation shadow. It cannot publish, mint authority, or mutate
    /// the owner-side semantic relation.
    pub fn viewer_c_presentation_gap(relation: impl Into<String>) -> Self {
        Self::ViewerCPresentationGap(relation.into())
    }

    /// Request the source-derived ParticipantA lifecycle path.  Participant,
    /// principal, membership, capability, witness, endpoint, and authority
    /// are recovered from checked projection and admitted M9 state only.
    pub fn participant_a_leave_relation_primary(relation: impl Into<String>) -> Self {
        Self::ParticipantALeaveRelationPrimary(relation.into())
    }

    /// Request the source-derived relation invalidation path.  The action
    /// names only a checked relation; it carries neither relation authority
    /// nor an endpoint, state, epoch, lease, witness, or capability.
    pub fn invalidate_relation_primary(relation: impl Into<String>) -> Self {
        Self::InvalidateRelationPrimary(relation.into())
    }

    /// Request one source-derived fresh relation reacquisition.  Fresh
    /// material is sealed in M9 at admission and consumed by SYS-4, never
    /// supplied by this schedule action.
    pub fn fresh_reacquire_relation_primary(relation: impl Into<String>) -> Self {
        Self::FreshReacquireRelationPrimary(relation.into())
    }

    pub fn revoke_viewer_c_consumer_capability(value_name: impl Into<String>) -> Self {
        Self::RevokeViewerCConsumerCapability(value_name.into())
    }

    #[cfg(test)]
    pub fn for_test_unknown_source_operation(operation: impl Into<String>) -> Self {
        Self::ForTestUnknownSourceOperation(operation.into())
    }

    #[cfg(test)]
    pub fn for_test_unknown_designated_value(value_name: impl Into<String>) -> Self {
        Self::ForTestUnknownDesignatedValue(value_name.into())
    }
}

/// One live finite vertical slice.  It owns exactly one admitted
/// `LocalFabric`; relation, owner, designated, and lifecycle actions all use
/// that same in-process dispatch state.
pub struct Sys5VerticalSliceRuntime {
    fabric: LocalFabric,
    fabric_instance_ref: String,
    checked_program_identity_ref: String,
    sealed_admission_attestation_ref: String,
    artifact_projection_ref: String,
    admission_summary: Sys5AdmissionSummary,
    startup_plan: Sys5VerticalStartupPlan,
    source_principal: String,
    bindings: Sys5VerticalBindings,
    joined_report: Sys5VerticalJoinedReport,
    relation_shadows: BTreeMap<(String, String), Sys5RelationObserverShadow>,
    completed_participant_leaves: BTreeMap<String, Sys5ParticipantLeaveEvidence>,
    last_participant_leave_failure: Option<Sys5ParticipantLeaveFailureEvidence>,
    next_lifecycle_occurrence: u64,
}

/// Typed result of an attempted unknown ordinary action at the sealed
/// SYS-4 admission boundary.  This is not a fixture lookup: the candidate is
/// a real `SourceAction`, and the fabric validates it against generated route
/// inventory without dispatching or mutating state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Sys5SourceActionAdmissionControl {
    candidate_action_ref: String,
    diagnostic: String,
    rejected_before_dispatch: bool,
    semantic_state_before_ref: String,
    semantic_state_after_ref: String,
}

impl Sys5SourceActionAdmissionControl {
    pub(crate) fn candidate_action_ref(&self) -> &str {
        &self.candidate_action_ref
    }

    pub(crate) fn diagnostic(&self) -> &str {
        &self.diagnostic
    }

    pub(crate) const fn rejected_before_dispatch(&self) -> bool {
        self.rejected_before_dispatch
    }

    pub(crate) fn semantic_state_before_ref(&self) -> &str {
        &self.semantic_state_before_ref
    }

    pub(crate) fn semantic_state_after_ref(&self) -> &str {
        &self.semantic_state_after_ref
    }
}

impl fmt::Debug for Sys5VerticalSliceRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sys5VerticalSliceRuntime")
            .field("fabric_instance_ref", &self.fabric_instance_ref)
            .field(
                "checked_program_identity_ref",
                &self.checked_program_identity_ref,
            )
            .field("locus_count", &self.fabric.locus_names().len())
            .field("status", &"experimental-source-derived-st-local")
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5VerticalDiagnosticKind {
    UnknownSourceOperation,
    UnknownSourceValue,
    MissingPublishedDesignatedValue,
    MissingConsumerCapability,
    RelationTransitionRejected,
    BackendIneligible,
    FabricBootRejected,
    VerticalInventoryIncomplete,
    RelationFreshBindingAlreadyConsumed,
    DuplicateParticipantLeave,
    DispatchRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5VerticalSliceError {
    kind: Sys5VerticalDiagnosticKind,
}

impl Sys5VerticalSliceError {
    fn new(kind: Sys5VerticalDiagnosticKind) -> Self {
        Self { kind }
    }

    fn from_dispatch(diagnostics: Sys4DispatchDiagnostics) -> Self {
        let kind = match diagnostics.primary().kind() {
            crate::sys4_dispatch::Sys4DiagnosticKind::MissingPublishedResult => {
                Sys5VerticalDiagnosticKind::MissingPublishedDesignatedValue
            }
            crate::sys4_dispatch::Sys4DiagnosticKind::MissingConsumerCapability => {
                Sys5VerticalDiagnosticKind::MissingConsumerCapability
            }
            _ => Sys5VerticalDiagnosticKind::DispatchRejected,
        };
        Self::new(kind)
    }

    pub const fn kind(&self) -> Sys5VerticalDiagnosticKind {
        self.kind
    }

    pub const fn rejected_before_generated_endpoint(&self) -> bool {
        matches!(
            self.kind,
            Sys5VerticalDiagnosticKind::UnknownSourceOperation
                | Sys5VerticalDiagnosticKind::UnknownSourceValue
                | Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed
                | Sys5VerticalDiagnosticKind::DuplicateParticipantLeave
        )
    }

    pub const fn rejected_before_m9_authority_use(&self) -> bool {
        matches!(
            self.kind,
            Sys5VerticalDiagnosticKind::UnknownSourceOperation
                | Sys5VerticalDiagnosticKind::UnknownSourceValue
                | Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed
                | Sys5VerticalDiagnosticKind::DuplicateParticipantLeave
        )
    }

    pub const fn rejected_before_m8_cache_or_state_mutation(&self) -> bool {
        matches!(
            self.kind,
            Sys5VerticalDiagnosticKind::UnknownSourceOperation
                | Sys5VerticalDiagnosticKind::UnknownSourceValue
                | Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed
                | Sys5VerticalDiagnosticKind::MissingPublishedDesignatedValue
                | Sys5VerticalDiagnosticKind::MissingConsumerCapability
                | Sys5VerticalDiagnosticKind::DuplicateParticipantLeave
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5VerticalEndpointChain {
    source_locus: String,
    target_locus: String,
    edge_kind: CommunicationEdgeKind,
    logical_path: String,
    source_span: String,
    source_ref: String,
    core_ref: String,
    artifact_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    edge_ref: String,
    request_ref: String,
    owner_publish_ref: Option<String>,
    request_enqueue_ref: String,
    dispatch_ref: String,
    receive_ref: String,
    consumer_observe_ref: Option<String>,
    serve_ref: String,
}

impl Sys5VerticalEndpointChain {
    /// The exact runtime request identity retained by the generated endpoint
    /// receipt. It is distinct from all occurrence identifiers.
    pub fn request_identity(&self) -> &str {
        &self.request_ref
    }

    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }

    pub fn target_locus(&self) -> &str {
        &self.target_locus
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5VerticalOwnerMutation {
    locus: String,
    state: String,
    index: String,
    field: String,
    old_value: i64,
    new_value: i64,
}

impl Sys5VerticalOwnerMutation {
    pub const fn old_new_int(&self) -> (i64, i64) {
        (self.old_value, self.new_value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5VerticalReceipt {
    fabric_instance_ref: String,
    source_derived: bool,
    source_locus: String,
    owner_locus: Option<String>,
    endpoint_chain: Option<Sys5VerticalEndpointChain>,
    owner_mutations: Vec<Sys5VerticalOwnerMutation>,
    designated_value_name: Option<String>,
    evaluator_locus: Option<String>,
    consumer_locus: Option<String>,
    typed_int: Option<i64>,
    designated_result_version: Option<u64>,
    designated_delivery_ref: Option<String>,
    designated_cache_binding_ref: Option<String>,
    performed_m8_semantic_consumption: bool,
    returned_from_designated_cache_after_authority_revalidation: bool,
    relation_shadow: Option<Sys5RelationObserverShadow>,
    presentation_gap_evidence: Option<Sys5PresentationGapEvidence>,
    participant_leave_evidence: Option<Sys5ParticipantLeaveEvidence>,
    fresh_reacquire_evidence: Option<Sys5FreshReacquireEvidence>,
    no_direct_cross_locus_store_mutation: bool,
}

/// Observer-safe result of one actual M8 consumer-local presentation action.
///
/// It deliberately retains the result classification and opaque references,
/// never source text, anchor samples, credentials, or raw authority material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5PresentationGapEvidence {
    relation: String,
    consumer_locus: String,
    projection_kind: String,
    publishes_value: bool,
    absolute_stream_count: usize,
    restriction: String,
    restriction_ref: String,
    redaction: String,
    redaction_ref: String,
    selected_anchor: String,
    selected_anchor_ref: String,
    selected_floor: String,
    selected_floor_ref: String,
    context_frontier_ref: String,
    semantic_digest_before: String,
    semantic_digest_after: String,
    endpoint_count_before: usize,
    endpoint_count_after: usize,
    derived_from_actual_action: bool,
}

/// Observer-safe evidence from the actual source-derived ParticipantA leave
/// transition. All membership, incarnation, capability, and witness facts
/// are represented only by opaque refs produced from the M9/SYS-4 receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5ParticipantLeaveEvidence {
    action: String,
    source_derived: bool,
    external_lifecycle_request: bool,
    principal: String,
    participant_locus: String,
    retired_membership_locus: String,
    m9_transition_kind: String,
    checked_membership_identity_exact: bool,
    membership_epoch_monotone: bool,
    retired_lineage_capability: bool,
    retired_lineage_witness: bool,
    relation: String,
    relation_owner_locus: String,
    relation_owner_authority_preserved: bool,
    m8_state_mutated_before_m9_retirement: bool,
    direct_consumer_mutation: bool,
    request_identity: String,
    request_enqueue_occurrence_ref: String,
    dispatch_occurrence_ref: String,
    receive_occurrence_ref: String,
    serve_occurrence_ref: String,
    receipt_occurrence_ref: String,
    m9_retire_occurrence_ref: String,
    checked_membership_identity_ref: String,
    prior_membership_ref: String,
    successor_tombstone_ref: String,
    membership_epoch_before_ref: String,
    membership_epoch_after_ref: String,
    incarnation_before_ref: String,
    incarnation_after_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    capability_lineage_ref: String,
    witness_lineage_ref: String,
    request_frontier_ref: String,
    result_frontier_ref: String,
    selected_anchor_after: String,
    selected_floor_after: String,
    invalidates_relation_primary: bool,
    direct_owner_mutation: bool,
    fixture_schedule_authority_injection: bool,
    relation_degradation: Sys5RelationDegradationEvidence,
}

/// The relation portion of an actual ParticipantA leave receipt, retained
/// separately so devtools can follow the M9-retirement-before-publication
/// edge without treating a Viewer projection as a semantic owner mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5RelationDegradationEvidence {
    trigger_action: String,
    source_derived: bool,
    external_lifecycle_request: bool,
    relation: String,
    owner_locus: String,
    prior_selected_anchor: String,
    selected_anchor_after: String,
    selected_floor_after: String,
    m9_retirement_precedes_relation_publication: bool,
    participant_b_owner_authority_preserved: bool,
    direct_consumer_mutation: bool,
    m9_retire_occurrence_ref: String,
    relation_publish_occurrence_ref: String,
    prior_relation_lineage_ref: String,
    successor_relation_lineage_ref: String,
    semantic_digest_before_ref: String,
    semantic_digest_after_ref: String,
    owner_authority_ref: String,
}

/// Observer-safe failure evidence for the one repeated, source-bound leave
/// action in the finite workflow. The lower ST candidate has actually
/// rejected the duplicate before it becomes live; these refs identify this
/// observer event and before/after snapshots without exposing M9 material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5ParticipantLeaveFailureEvidence {
    attempt: String,
    diagnostic: String,
    source_derived: bool,
    external_lifecycle_request: bool,
    failed_closed: bool,
    partial_membership_retired: bool,
    capability_or_witness_partially_retired: bool,
    m9_successor_installed: bool,
    m8_state_mutated: bool,
    m8_relation_mutated: bool,
    m8_designated_result_mutated: bool,
    preserved_successful_m8_result_ref: bool,
    request_identity: String,
    request_enqueue_occurrence_ref: String,
    reject_occurrence_ref: String,
    receipt_occurrence_ref: String,
    checked_membership_identity_ref: String,
    active_generation_ref: String,
    m8_state_digest_before_ref: String,
    m8_state_digest_after_ref: String,
    m8_relation_digest_before_ref: String,
    m8_relation_digest_after_ref: String,
    last_successful_m8_result_ref: String,
}

/// Observer-safe evidence from the actual source-derived fresh membership
/// transition that makes a retired explicit primary anchor eligible again.
/// It is not an admission API: no caller supplies an epoch, incarnation,
/// membership reference, capability, witness, or authority material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5FreshReacquireEvidence {
    action: String,
    source_derived: bool,
    external_lifecycle_request: bool,
    relation: String,
    participant_locus: String,
    fresh_membership_locus: String,
    m9_transition_kind: String,
    fresh_membership_epoch_distinct: bool,
    fresh_incarnation_distinct: bool,
    fresh_lineage_capability: bool,
    fresh_lineage_witness: bool,
    relation_owner_locus: String,
    relation_owner_authority_preserved: bool,
    prior_selected_anchor: String,
    selected_anchor_after: String,
    selected_floor_after: String,
    m9_fresh_membership_precedes_relation_publication: bool,
    caller_supplied_epoch_or_incarnation: bool,
    caller_supplied_membership_ref: bool,
    caller_supplied_authority: bool,
    fixture_schedule_authority_injection: bool,
    direct_consumer_mutation: bool,
    request_identity: String,
    request_enqueue_occurrence_ref: String,
    dispatch_occurrence_ref: String,
    receive_occurrence_ref: String,
    serve_occurrence_ref: String,
    receipt_occurrence_ref: String,
    m9_fresh_membership_occurrence_ref: String,
    relation_publish_occurrence_ref: String,
    retired_membership_ref: String,
    fresh_membership_ref: String,
    retired_membership_epoch_ref: String,
    fresh_membership_epoch_ref: String,
    retired_incarnation_ref: String,
    fresh_incarnation_ref: String,
    prior_generation_ref: String,
    successor_generation_ref: String,
    m9_transition_ref: String,
    fresh_capability_lineage_ref: String,
    fresh_witness_lineage_ref: String,
    prior_relation_lineage_ref: String,
    successor_relation_lineage_ref: String,
    semantic_digest_before_ref: String,
    semantic_digest_after_ref: String,
    owner_authority_ref: String,
}

impl Sys5ParticipantLeaveFailureEvidence {
    pub(crate) fn satisfies_i2_duplicate_leave_fail_closed(&self) -> bool {
        self.source_derived
            && self.external_lifecycle_request
            && self.failed_closed
            && !self.partial_membership_retired
            && !self.capability_or_witness_partially_retired
            && !self.m9_successor_installed
            && !self.m8_state_mutated
            && !self.m8_relation_mutated
            && !self.m8_designated_result_mutated
            && self.preserved_successful_m8_result_ref
            && self.m8_state_digest_before_ref == self.m8_state_digest_after_ref
            && self.m8_relation_digest_before_ref == self.m8_relation_digest_after_ref
    }
}

impl Sys5PresentationGapEvidence {
    pub(crate) fn satisfies_i2_semantic_presentation_separation(&self) -> bool {
        self.derived_from_actual_action
            && !self.publishes_value
            && self.absolute_stream_count == 0
            && self.semantic_digest_before == self.semantic_digest_after
            && self.endpoint_count_before == self.endpoint_count_after
            && !self.restriction_ref.is_empty()
            && !self.redaction_ref.is_empty()
    }
}

impl Sys5RelationDegradationEvidence {
    pub(crate) fn satisfies_i2_relation_degradation(&self) -> bool {
        self.source_derived
            && self.external_lifecycle_request
            && self.m9_retirement_precedes_relation_publication
            && self.participant_b_owner_authority_preserved
            && !self.direct_consumer_mutation
            && !self.semantic_digest_before_ref.is_empty()
            && !self.semantic_digest_after_ref.is_empty()
    }
}

impl Sys5FreshReacquireEvidence {
    pub(crate) fn satisfies_i2_fresh_relation_reacquire(&self) -> bool {
        self.source_derived
            && self.fresh_membership_epoch_distinct
            && self.fresh_incarnation_distinct
            && self.relation_owner_authority_preserved
            && self.m9_fresh_membership_precedes_relation_publication
            && !self.direct_consumer_mutation
            && !self.caller_supplied_epoch_or_incarnation
            && !self.caller_supplied_membership_ref
            && !self.caller_supplied_authority
    }
}

impl Sys5ParticipantLeaveEvidence {
    pub fn relation_degradation(&self) -> &Sys5RelationDegradationEvidence {
        &self.relation_degradation
    }

    pub(crate) fn satisfies_i2_relation_leave(&self) -> bool {
        self.source_derived
            && self.external_lifecycle_request
            && self.checked_membership_identity_exact
            && self.membership_epoch_monotone
            && self.relation_owner_authority_preserved
            && !self.direct_consumer_mutation
            && self
                .relation_degradation
                .satisfies_i2_relation_degradation()
    }
}

#[derive(Debug, Clone, Default)]
struct Sys5FabricReceiptContext {
    owner_locus: Option<String>,
    designated_value_name: Option<String>,
    evaluator_locus: Option<String>,
    owner_mutations: Vec<Sys5VerticalOwnerMutation>,
    no_direct_cross_locus_store_mutation: bool,
    relation_shadow: Option<Sys5RelationObserverShadow>,
    endpoint_edge_kind: Option<CommunicationEdgeKind>,
    m8_trace_kind: Option<crate::m8_runtime_local_cut::M8LocalTraceKind>,
    endpoint_source_locus: Option<String>,
    endpoint_target_locus: Option<String>,
}

impl Sys5VerticalReceipt {
    pub fn fabric_instance_ref(&self) -> &str {
        &self.fabric_instance_ref
    }

    pub const fn is_source_derived(&self) -> bool {
        self.source_derived
    }

    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }

    pub fn owner_locus(&self) -> Option<&str> {
        self.owner_locus.as_deref()
    }

    pub fn generated_endpoint_chain(&self) -> &Sys5VerticalEndpointChain {
        self.endpoint_chain
            .as_ref()
            .expect("generated source action retains one endpoint chain")
    }

    pub fn owner_mutation(
        &self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
    ) -> Option<&Sys5VerticalOwnerMutation> {
        self.owner_mutations.iter().find(|mutation| {
            mutation.locus == locus
                && mutation.state == state
                && mutation.index == index
                && mutation.field == field
        })
    }

    pub const fn no_direct_cross_locus_store_mutation(&self) -> bool {
        self.no_direct_cross_locus_store_mutation
    }

    pub fn designated_value_name(&self) -> Option<&str> {
        self.designated_value_name.as_deref()
    }

    pub fn typed_int(&self) -> Option<i64> {
        self.typed_int
    }

    /// Actual designated result version carried by the generated delivery,
    /// never inferred from a request identity or schedule position.
    pub const fn designated_result_version(&self) -> Option<u64> {
        self.designated_result_version
    }

    /// Opaque reference derived from the actual generated designated delivery.
    pub fn designated_delivery_ref(&self) -> Option<&str> {
        self.designated_delivery_ref.as_deref()
    }

    /// Opaque reference binding the actual delivery, value, consumer, and
    /// carried result version into the admitted cache identity.
    pub fn designated_cache_binding_ref(&self) -> Option<&str> {
        self.designated_cache_binding_ref.as_deref()
    }

    pub fn evaluator_locus_is(&self, locus: &str) -> bool {
        self.evaluator_locus.as_deref() == Some(locus)
    }

    pub fn consumer_locus(&self) -> Option<&str> {
        self.consumer_locus.as_deref()
    }

    pub const fn performed_m8_semantic_consumption(&self) -> bool {
        self.performed_m8_semantic_consumption
    }

    pub const fn returned_from_designated_cache_after_authority_revalidation(&self) -> bool {
        self.returned_from_designated_cache_after_authority_revalidation
    }

    pub fn observer_relation_shadow(
        &self,
        consumer_locus: &str,
        relation: &str,
    ) -> Option<&Sys5RelationObserverShadow> {
        self.relation_shadow.as_ref().filter(|shadow| {
            shadow.consumer_locus() == consumer_locus && shadow.relation() == relation
        })
    }

    pub fn presentation_gap_evidence(&self) -> Option<&Sys5PresentationGapEvidence> {
        self.presentation_gap_evidence.as_ref()
    }

    pub fn participant_leave_evidence(&self) -> Option<&Sys5ParticipantLeaveEvidence> {
        self.participant_leave_evidence.as_ref()
    }

    pub fn fresh_reacquire_evidence(&self) -> Option<&Sys5FreshReacquireEvidence> {
        self.fresh_reacquire_evidence.as_ref()
    }
}

/// One compact observer-safe causal view.  The values and private M9 material
/// remain in the runtime; this report carries only typed references and
/// status rows needed to join source through local occurrences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5VerticalJoinedReport {
    // This is an event prefix, rather than a set of labels.  A set made a
    // restored cut look complete while losing the order in which its actual
    // source→runtime evidence was observed. Retain every insertion in order;
    // duplicate-looking labels may denote distinct actual occurrences.
    rows: Vec<String>,
    verification_discharges: Vec<Sys5VerificationDischargeSummary>,
}

impl Sys5VerticalJoinedReport {
    fn new(verification_discharges: Vec<Sys5VerificationDischargeSummary>) -> Self {
        let mut rows = Vec::new();
        for discharge in &verification_discharges {
            if discharge.is_discharged() {
                rows.push(format!("verification:{}:discharged", discharge.verifier));
            }
        }
        Self {
            rows,
            verification_discharges,
        }
    }

    fn push(&mut self, row: String) {
        self.rows.push(row);
    }

    pub fn verification_discharge(
        &self,
        verifier: &str,
    ) -> Option<&Sys5VerificationDischargeSummary> {
        self.verification_discharges
            .iter()
            .find(|discharge| discharge.verifier == verifier)
    }

    pub fn render_compact(&self) -> String {
        self.rows.join("\n")
    }

    /// Ordered typed rows retained by the one joined devtools view.  This
    /// never asks callers to reconstruct an occurrence chain from separate
    /// source, Core, and fabric logs.
    pub fn ordered_rows(&self) -> &[String] {
        &self.rows
    }
}

impl Sys5VerticalSliceRuntime {
    pub fn dispatch(
        &mut self,
        action: Sys5VerticalAction,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        match action {
            Sys5VerticalAction::ParticipantAAttackDeclaredTarget => self.dispatch_owner_attack(),
            Sys5VerticalAction::WorldTick(tick) => self.dispatch_world_tick(tick),
            Sys5VerticalAction::ViewerCConsumeWorldResult => self.dispatch_designated_consume(),
            Sys5VerticalAction::PublishRelation(relation) => {
                self.dispatch_relation_publish(&relation)
            }
            Sys5VerticalAction::ViewerCPresentationGap(relation) => {
                self.dispatch_relation_presentation_gap(&relation)
            }
            Sys5VerticalAction::ParticipantALeaveRelationPrimary(relation) => {
                self.dispatch_participant_a_leave_relation_primary(&relation)
            }
            Sys5VerticalAction::InvalidateRelationPrimary(relation) => {
                self.dispatch_relation_invalidate(&relation)
            }
            Sys5VerticalAction::FreshReacquireRelationPrimary(relation) => {
                self.dispatch_relation_fresh_reacquire(&relation)
            }
            Sys5VerticalAction::RevokeViewerCConsumerCapability(value_name) => {
                self.dispatch_consumer_capability_revoke(&value_name)
            }
            #[cfg(test)]
            Sys5VerticalAction::ForTestUnknownSourceOperation(_) => {
                self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation)
            }
            #[cfg(test)]
            Sys5VerticalAction::ForTestUnknownDesignatedValue(_) => {
                self.reject(Sys5VerticalDiagnosticKind::UnknownSourceValue)
            }
        }
    }

    pub fn local_fabric_instance_ref(&self) -> &str {
        &self.fabric_instance_ref
    }

    pub const fn local_fabric_instance_count(&self) -> usize {
        1
    }

    /// Exercise the actual source-action admission boundary with an operation
    /// absent from this checked projection.  It must be rejected before
    /// dispatch and preserve the complete active semantic snapshot.  This
    /// gives SYS-6 a production control for the fact that schedule actions
    /// cannot introduce a fixture name, expected result, manual route, or
    /// source-free operation.
    pub(crate) fn reject_unknown_source_action_for_i2(&self) -> Sys5SourceActionAdmissionControl {
        let candidate = ExternalAction::source_operation(SourceAction::owner_operation(
            "i2-undeclared-source-action",
        ));
        let before = self.fabric.semantic_snapshot();
        let result = self.fabric.validate_external_action(&candidate);
        let after = self.fabric.semantic_snapshot();
        let diagnostic = result
            .as_ref()
            .err()
            .map(|diagnostic| format!("{:?}", diagnostic.primary().kind()))
            .unwrap_or_else(|| "UnexpectedSourceActionAdmission".to_string());
        Sys5SourceActionAdmissionControl {
            candidate_action_ref: relation_observer_ref("i2-undeclared-source-action"),
            rejected_before_dispatch: diagnostic == "UnknownSourceAction" && before == after,
            diagnostic,
            semantic_state_before_ref: relation_observer_ref(&format!("{before:?}")),
            semantic_state_after_ref: relation_observer_ref(&format!("{after:?}")),
        }
    }

    pub fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity_ref
    }

    pub fn sealed_admission_attestation_ref(&self) -> &str {
        &self.sealed_admission_attestation_ref
    }

    pub fn relation_semantic_digest(&self, relation: &str) -> Option<&str> {
        self.fabric.relation_semantic_digest(relation)
    }

    pub fn observer_relation_shadow(
        &self,
        consumer_locus: &str,
        relation: &str,
    ) -> Option<&Sys5RelationObserverShadow> {
        self.relation_shadows
            .get(&(consumer_locus.to_string(), relation.to_string()))
    }

    pub fn designated_cache_entry_count(&self, value_name: &str, consumer: &str) -> usize {
        self.fabric
            .designated_cache_entry_count_for_value(value_name, consumer)
    }

    pub fn total_endpoint_carrier_count(&self) -> usize {
        self.fabric.total_endpoint_carrier_count()
    }

    pub fn patch_lifecycle_row_count(&self) -> usize {
        self.fabric.patch_lifecycle_snapshot().row_count()
    }

    pub fn observer_safe_m9_authority_digest(&self) -> String {
        local_cut_ref(&format!(
            "{:?}",
            self.fabric.current_m9_authority_inspection()
        ))
    }

    pub fn observer_safe_m8_trace_digest(&self) -> String {
        match self.fabric.m8_actual_trace() {
            Ok(trace) => local_cut_ref(&format!("{trace:?}")),
            Err(_) => local_cut_ref("m8-observer-unavailable"),
        }
    }

    pub fn observer_safe_restore_capsule_digest(&self) -> String {
        local_cut_ref(&format!(
            "program={};admission={};artifact={};state={};m9={};m8={};frontier={:?};relations={:?}",
            self.checked_program_identity_ref,
            self.sealed_admission_attestation_ref,
            self.artifact_projection_ref,
            self.observer_safe_state_digest(),
            self.observer_safe_m9_authority_digest(),
            self.observer_safe_m8_trace_digest(),
            self.fabric.current_patch_frontier_snapshot(),
            self.relation_shadows,
        ))
    }

    pub fn active_runtime_identity_snapshot(&self) -> Sys5ActiveRuntimeIdentitySnapshot {
        Sys5ActiveRuntimeIdentitySnapshot {
            runtime_ref: local_cut_ref(&format!(
                "active={:?};program={};admission={};artifact={}",
                self.fabric.active_runtime_identity_snapshot(),
                self.checked_program_identity_ref,
                self.sealed_admission_attestation_ref,
                self.artifact_projection_ref,
            )),
        }
    }

    pub fn observer_safe_runtime_snapshot(&self) -> Sys5ObserverSafeRuntimeSnapshot {
        let semantic = self.fabric.semantic_snapshot();
        let observer_safe_ints = self
            .startup_plan
            .observer_safe_cells
            .iter()
            .filter_map(|cell| {
                semantic
                    .int(&cell.locus, &cell.state, &cell.index, &cell.field)
                    .map(|value| {
                        (
                            (
                                cell.locus.clone(),
                                cell.state.clone(),
                                cell.index.clone(),
                                cell.field.clone(),
                            ),
                            value,
                        )
                    })
            })
            .collect();
        let designated_cache_counts = self
            .bindings
            .designated_values
            .iter()
            .map(|binding| {
                (
                    (binding.value_name.clone(), binding.consumer_locus.clone()),
                    self.fabric.designated_cache_entry_count_for_value(
                        &binding.value_name,
                        &binding.consumer_locus,
                    ),
                )
            })
            .collect();
        let fresh_relation_bindings = self
            .bindings
            .relation_ids
            .iter()
            .filter(|relation| self.fabric.relation_fresh_binding_is_consumed(relation))
            .cloned()
            .collect();
        let relation_digests = self
            .bindings
            .relation_ids
            .iter()
            .filter_map(|relation| {
                self.fabric
                    .relation_semantic_digest(relation)
                    .map(|digest| (relation.clone(), digest.to_string()))
            })
            .collect();
        let verification_summary = Sys5RuntimeVerificationSummary {
            discharged_verifiers: self
                .admission_summary
                .verification_discharges
                .iter()
                .filter(|summary| summary.is_discharged())
                .map(|summary| summary.verifier.clone())
                .collect(),
        };
        Sys5ObserverSafeRuntimeSnapshot {
            observer_safe_ints,
            state_digest: self.observer_safe_state_digest(),
            designated_cache_counts,
            fresh_relation_bindings,
            relation_digests,
            m9_summary: Sys5RuntimeM9Summary {
                complete_final_residual_discharge: self
                    .admission_summary
                    .is_complete_for_projection(),
            },
            verification_summary,
        }
    }

    /// Capture a whole local vertical slice only after SYS-4 has accepted an
    /// ST local cut.  The joined `SaveCut` row is appended only for a
    /// successful cut and becomes part of the exact prefix restored later.
    pub fn save_local_cut(
        &mut self,
        cut_id: impl Into<String>,
    ) -> Result<Sys5LocalCut, Sys5LocalCutPatchError> {
        let cut_id = cut_id.into();
        let sys4_cut = self
            .fabric
            .save_local_cut(&cut_id)
            .map_err(|_| Sys5LocalCutPatchError::new(Sys5LocalCutPatchErrorKind::CutRejected))?;
        let cut_id_ref = local_cut_ref(&cut_id);
        let sys4_cut_integrity_ref = local_cut_ref(&sys4_cut.observer_safe_integrity_material());
        let cut_occurrence_ref = next_lifecycle_occurrence_ref(
            &mut self.next_lifecycle_occurrence,
            "SaveCut",
            &cut_id_ref,
            &sys4_cut_integrity_ref,
        )?;
        let frontier_ref =
            patch_frontier_ref(&format!("{:?}", sys4_cut.active_patch_frontier_snapshot()));
        self.joined_report.push(lifecycle_joined_row(
            "SaveCut",
            Sys5LifecycleBoundaryRefs {
                before_program_ref: &self.checked_program_identity_ref,
                after_program_ref: &self.checked_program_identity_ref,
                before_artifact_ref: &self.artifact_projection_ref,
                after_artifact_ref: &self.artifact_projection_ref,
                before_frontier_ref: &frontier_ref,
                after_frontier_ref: &frontier_ref,
            },
            Some(("cut_occurrence_ref", &cut_occurrence_ref)),
        ));
        Ok(Sys5LocalCut::new(&cut_id, self, sys4_cut))
    }

    /// Atomically activate an already ordinary-source-checked/projected and
    /// M9-admitted candidate.  Rejections are delegated to SYS-4's
    /// clone/preflight boundary and add precisely one observer-safe lifecycle
    /// row here; they do not replace source, artifact, authority, cache, or
    /// semantic state.
    pub fn activate_source_first_patch(
        &mut self,
        candidate: Sys5LocalPatchCandidate,
    ) -> Result<Sys5PatchOutcome, Sys5LocalCutPatchError> {
        let before_program_ref = self.checked_program_identity_ref.clone();
        let before_artifact_ref = self.artifact_projection_ref.clone();
        let before_frontier_ref = patch_frontier_ref(&format!(
            "{:?}",
            self.fabric.current_patch_frontier_snapshot()
        ));
        let Sys5LocalPatchCandidate {
            patch_id_ref,
            patch_summary,
            patch_startup_plan,
            patch_bindings,
            patch_source_principal,
            patch_artifact_projection_ref,
            inner,
            ..
        } = candidate;
        // Reserve an observer occurrence before SYS-4 can append either an
        // accepted or rejected patch lifecycle row.  Exhaustion therefore
        // fails closed before any fabric, SYS-4 lifecycle, or semantic state
        // mutation; the occurrence identity itself is neutral with respect to
        // the eventual patch verdict.
        let lifecycle_cursor_before = self.next_lifecycle_occurrence;
        let patch_occurrence_ref = next_lifecycle_occurrence_ref(
            &mut self.next_lifecycle_occurrence,
            "PatchActivation",
            &patch_id_ref,
            &before_frontier_ref,
        )?;
        let sys4_outcome = match self.fabric.activate_checked_patch(inner) {
            Ok(outcome) => outcome,
            Err(_) => {
                self.next_lifecycle_occurrence = lifecycle_cursor_before;
                return Err(Sys5LocalCutPatchError::new(
                    Sys5LocalCutPatchErrorKind::PatchCandidateRejected,
                ));
            }
        };
        let outcome = sys5_patch_outcome(&sys4_outcome, patch_occurrence_ref.clone());
        let after_frontier_ref = outcome.activation_frontier.ref_digest.clone();
        let lifecycle_kind = match outcome.verdict {
            Sys5PatchVerdict::Accepted => "PatchAccepted",
            Sys5PatchVerdict::Rejected => "PatchRejected",
        };
        if outcome.verdict == Sys5PatchVerdict::Accepted {
            self.checked_program_identity_ref = patch_summary.checked_program_identity.clone();
            self.sealed_admission_attestation_ref =
                patch_summary.sealed_inventory_attestation_ref().to_string();
            self.artifact_projection_ref = patch_artifact_projection_ref.clone();
            self.admission_summary = patch_summary;
            self.startup_plan = patch_startup_plan;
            self.bindings = patch_bindings;
            self.source_principal = patch_source_principal;
        }
        let after_program_ref = if outcome.verdict == Sys5PatchVerdict::Accepted {
            &self.checked_program_identity_ref
        } else {
            &before_program_ref
        };
        let after_artifact_ref = if outcome.verdict == Sys5PatchVerdict::Accepted {
            &self.artifact_projection_ref
        } else {
            &before_artifact_ref
        };
        self.joined_report.push(lifecycle_joined_row(
            lifecycle_kind,
            Sys5LifecycleBoundaryRefs {
                before_program_ref: &before_program_ref,
                after_program_ref,
                before_artifact_ref: &before_artifact_ref,
                after_artifact_ref,
                before_frontier_ref: &before_frontier_ref,
                after_frontier_ref: &after_frontier_ref,
            },
            Some(("patch_occurrence_ref", &patch_occurrence_ref)),
        ));
        Ok(outcome)
    }

    pub fn observer_safe_int(
        &self,
        locus: &str,
        state: &str,
        index: &str,
        field: &str,
    ) -> Option<i64> {
        self.startup_plan
            .observer_safe_contains(locus, state, index, field)
            .then(|| {
                self.fabric
                    .semantic_snapshot()
                    .int(locus, state, index, field)
            })
            .flatten()
    }

    pub fn observer_safe_state_digest(&self) -> String {
        let snapshot = self.fabric.semantic_snapshot();
        let facts = self
            .startup_plan
            .observer_safe_cells
            .iter()
            .map(|cell| {
                format!(
                    "{}:{}:{}:{}:{:?}",
                    cell.locus,
                    cell.state,
                    cell.index,
                    cell.field,
                    snapshot.int(&cell.locus, &cell.state, &cell.index, &cell.field),
                )
            })
            .collect::<Vec<_>>();
        relation_observer_ref(&facts.join("\n"))
    }

    pub fn viewer_has_designated_evaluator(&self, locus: &str, value_name: &str) -> bool {
        self.fabric.locus_runtime(locus).is_some_and(|runtime| {
            runtime
                .artifact()
                .has_designated_evaluation_expression(value_name)
        })
    }

    pub fn designated_evaluation_count(&self, value_name: &str) -> usize {
        self.fabric
            .m8_actual_trace()
            .map(|trace| trace.designated_evaluation_count(value_name))
            .unwrap_or_default()
    }

    pub fn designated_semantic_consumption_count(&self, value_name: &str, consumer: &str) -> usize {
        self.fabric
            .designated_semantic_consumption_count_for_value(value_name, consumer)
    }

    pub fn designated_cache_digest(&self, value_name: &str, consumer: &str) -> String {
        let entries = self
            .fabric
            .designated_cache_entry_count_for_value(value_name, consumer);
        relation_observer_ref(&format!("cache:{value_name}:{consumer}:{entries}"))
    }

    pub fn observer_safe_joined_report(&self) -> &Sys5VerticalJoinedReport {
        &self.joined_report
    }

    /// The most recent source-bound leave failure, if the lower ST
    /// transaction rejected it before any candidate state became live.
    pub fn last_participant_leave_failure(&self) -> Option<&Sys5ParticipantLeaveFailureEvidence> {
        self.last_participant_leave_failure.as_ref()
    }

    fn dispatch_owner_attack(&mut self) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let binding = self.bindings.canonical_owner().cloned().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::UnknownSourceOperation)
        })?;
        let before = self.fabric.semantic_snapshot();
        let receipt = self
            .fabric
            .dispatch_source_action(
                SourceAction::owner_operation(&binding.operation_id)
                    .with_argument(binding.declared_target_parameter, &self.source_principal),
            )
            .map_err(Sys5VerticalSliceError::from_dispatch)?;
        debug_assert_eq!(receipt.origin_locus(), binding.source_locus);
        debug_assert_eq!(receipt.target_locus(), binding.owner_locus);
        let after = self.fabric.semantic_snapshot();
        let owner_mutations = self.owner_mutations_since(&before, &after, &binding.owner_locus);
        let no_direct_cross_locus_store_mutation = after
            .changed_loci_since(&before)
            .iter()
            .all(|locus| locus == &binding.owner_locus);
        let result = self.receipt_from_fabric(
            receipt,
            Sys5FabricReceiptContext {
                owner_locus: Some(binding.owner_locus.clone()),
                owner_mutations,
                no_direct_cross_locus_store_mutation,
                endpoint_edge_kind: Some(CommunicationEdgeKind::OwnerRequest),
                m8_trace_kind: Some(crate::m8_runtime_local_cut::M8LocalTraceKind::OwnerWrite),
                endpoint_source_locus: Some(binding.source_locus.clone()),
                endpoint_target_locus: Some(binding.owner_locus.clone()),
                ..Sys5FabricReceiptContext::default()
            },
        )?;
        Ok(result)
    }

    fn dispatch_world_tick(
        &mut self,
        tick: String,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let binding = self
            .bindings
            .canonical_designated()
            .cloned()
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::UnknownSourceValue)
            })?;
        if !self
            .startup_plan
            .observer_visible_designated_value(&binding.value_name)
        {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceValue);
        }
        // The vertical observer contract has one generated remote input
        // segment.  A same-owner/private expression is not substituted into
        // that path; reject before dispatch rather than expose its raw value.
        let input_source_locus = binding.input_source_locus.clone().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::UnknownSourceValue)
        })?;
        let receipt = self
            .fabric
            .dispatch_source_action(
                SourceAction::designated_tick(&binding.value_name)
                    .with_tick(&binding.trigger_frontier, tick),
            )
            .map_err(Sys5VerticalSliceError::from_dispatch)?;
        let result = self.receipt_from_fabric(
            receipt.clone(),
            Sys5FabricReceiptContext {
                designated_value_name: Some(binding.value_name.clone()),
                evaluator_locus: Some(binding.evaluator_locus.clone()),
                no_direct_cross_locus_store_mutation: true,
                endpoint_edge_kind: Some(CommunicationEdgeKind::DesignatedInputRequest),
                m8_trace_kind: Some(
                    crate::m8_runtime_local_cut::M8LocalTraceKind::DesignatedValuePublished,
                ),
                endpoint_source_locus: Some(binding.evaluator_locus.clone()),
                endpoint_target_locus: Some(input_source_locus),
                ..Sys5FabricReceiptContext::default()
            },
        )?;
        Ok(result)
    }

    fn dispatch_designated_consume(
        &mut self,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let binding = self
            .bindings
            .canonical_designated()
            .cloned()
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::UnknownSourceValue)
            })?;
        let receipt = match self
            .fabric
            .dispatch_source_action(SourceAction::consume_designated_result(&binding.value_name))
        {
            Ok(receipt) => receipt,
            Err(diagnostics) => {
                let error = Sys5VerticalSliceError::from_dispatch(diagnostics);
                self.joined_report
                    .push(format!("failure:{:?}", error.kind()));
                return Err(error);
            }
        };
        let returned_from_cache =
            receipt.returned_from_designated_cache_after_authority_revalidation();
        if returned_from_cache {
            return self.cache_retry_receipt(receipt, binding);
        }
        let result = self.receipt_from_fabric(
            receipt.clone(),
            Sys5FabricReceiptContext {
                designated_value_name: Some(binding.value_name.clone()),
                no_direct_cross_locus_store_mutation: true,
                endpoint_edge_kind: Some(CommunicationEdgeKind::DesignatedResultDelivery),
                m8_trace_kind: Some(if returned_from_cache {
                    crate::m8_runtime_local_cut::M8LocalTraceKind::DesignatedCacheValidated
                } else {
                    crate::m8_runtime_local_cut::M8LocalTraceKind::DesignatedValueConsumed
                }),
                endpoint_source_locus: Some(binding.evaluator_locus.clone()),
                endpoint_target_locus: Some(binding.consumer_locus.clone()),
                ..Sys5FabricReceiptContext::default()
            },
        )?;
        self.record_designated_causal_segments(&receipt, &binding)?;
        Ok(result)
    }

    fn dispatch_relation_publish(
        &mut self,
        relation: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        if !self.bindings.relation_ids.contains(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation);
        }
        let receipt = self
            .fabric
            .publish_relation_current(relation)
            .map_err(|_| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        self.relation_receipt_from_sys4(relation, receipt)
    }

    fn dispatch_participant_a_leave_relation_primary(
        &mut self,
        relation: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        if !self.bindings.relation_ids.contains(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation);
        }
        let before_shadow = self
            .relation_shadows
            .get(&("ViewerC".to_string(), relation.to_string()))
            .cloned()
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let before_state_digest = self.observer_safe_state_digest();
        let before_relation_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let before_snapshot = self.observer_safe_runtime_snapshot();
        let active_generation_ref = relation_observer_ref(&format!(
            "active-generation:{}",
            self.fabric
                .current_m9_authority_inspection()
                .generation()
                .generation_ref()
        ));

        let lower = self.fabric.participant_leave_relation_primary(relation);
        let lower = match lower {
            Ok(receipt) => receipt,
            Err(_) if self.completed_participant_leaves.contains_key(relation) => {
                return self.record_duplicate_participant_leave_failure(
                    relation,
                    &before_state_digest,
                    &before_relation_digest,
                    &before_snapshot,
                    &active_generation_ref,
                );
            }
            Err(_) => {
                return self.reject(Sys5VerticalDiagnosticKind::RelationTransitionRejected);
            }
        };
        let endpoint = lower.relation_endpoint().clone();
        let after_state_digest = self.observer_safe_state_digest();
        let after_relation_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let mut result = self.relation_receipt_from_sys4(relation, endpoint)?;
        let relation_shadow = result.relation_shadow.as_ref().cloned().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
        })?;
        let endpoint = lower.relation_endpoint();
        let relation_owner_locus = endpoint.edge().source_locus().to_string();
        if lower.relation() != relation
            || lower.participant_locus().is_empty()
            || relation_shadow.owner_locus() != relation_owner_locus
            || before_shadow.relation() != relation
            || before_shadow.selected_floor() != "live-primary"
            || relation_shadow.selected_floor() != "fallback-anchor"
            || after_state_digest != before_state_digest
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let membership_epoch_monotone = !lower.membership_epoch_before_ref().is_empty()
            && !lower.membership_epoch_after_ref().is_empty()
            && lower.membership_epoch_before_ref() != lower.membership_epoch_after_ref();
        let checked_membership_identity_exact = !lower.checked_membership_identity_ref().is_empty()
            && !lower.capability_lineage_ref().is_empty()
            && !lower.witness_lineage_ref().is_empty()
            && membership_epoch_monotone
            && lower.incarnation_before_ref() != lower.incarnation_after_ref();
        if !checked_membership_identity_exact {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let m9_retire_occurrence_ref = lower.m9_retire_occurrence_id().to_string();
        let relation_publish_occurrence_ref = endpoint.owner_publish_occurrence_id().to_string();
        let relation_degradation = Sys5RelationDegradationEvidence {
            trigger_action: "participant_a_leave".to_string(),
            source_derived: true,
            external_lifecycle_request: true,
            relation: relation.to_string(),
            owner_locus: relation_owner_locus.clone(),
            prior_selected_anchor: before_shadow.selected_anchor().to_string(),
            selected_anchor_after: relation_shadow.selected_anchor().to_string(),
            selected_floor_after: relation_shadow.selected_floor().to_string(),
            // SYS-4 checked this exact causal edge before returning the
            // lower receipt; no local report ordering is used as its proof.
            m9_retirement_precedes_relation_publication: true,
            participant_b_owner_authority_preserved: true,
            direct_consumer_mutation: false,
            m9_retire_occurrence_ref: m9_retire_occurrence_ref.clone(),
            relation_publish_occurrence_ref: relation_publish_occurrence_ref.clone(),
            prior_relation_lineage_ref: before_shadow.lineage_ref().to_string(),
            successor_relation_lineage_ref: relation_shadow.lineage_ref().to_string(),
            semantic_digest_before_ref: relation_observer_ref(&before_relation_digest),
            semantic_digest_after_ref: relation_observer_ref(&after_relation_digest),
            owner_authority_ref: relation_observer_ref(
                &self.fabric.m8_authority_state_digest(&relation_owner_locus),
            ),
        };
        let evidence = Sys5ParticipantLeaveEvidence {
            action: "participant_a_leave".to_string(),
            source_derived: true,
            external_lifecycle_request: true,
            principal: self.source_principal.clone(),
            participant_locus: lower.participant_locus().to_string(),
            retired_membership_locus: lower.participant_locus().to_string(),
            m9_transition_kind: "participant-membership-retired".to_string(),
            checked_membership_identity_exact,
            membership_epoch_monotone,
            retired_lineage_capability: !lower.capability_lineage_ref().is_empty(),
            retired_lineage_witness: !lower.witness_lineage_ref().is_empty(),
            relation: relation.to_string(),
            relation_owner_locus,
            relation_owner_authority_preserved: true,
            m8_state_mutated_before_m9_retirement: false,
            direct_consumer_mutation: false,
            request_identity: lower.lifecycle_request_identity().to_string(),
            request_enqueue_occurrence_ref: lower.lifecycle_enqueue_occurrence_id().to_string(),
            dispatch_occurrence_ref: endpoint
                .transport()
                .source_outbox_dequeue_occurrence_id()
                .to_string(),
            receive_occurrence_ref: endpoint
                .transport()
                .target_inbox_enqueue_occurrence_id()
                .to_string(),
            serve_occurrence_ref: endpoint.consumer_serve_occurrence_id().to_string(),
            receipt_occurrence_ref: lower.lifecycle_receipt_occurrence_id().to_string(),
            m9_retire_occurrence_ref,
            checked_membership_identity_ref: lower.checked_membership_identity_ref().to_string(),
            prior_membership_ref: lower.prior_membership_ref().to_string(),
            successor_tombstone_ref: lower.successor_tombstone_ref().to_string(),
            membership_epoch_before_ref: lower.membership_epoch_before_ref().to_string(),
            membership_epoch_after_ref: lower.membership_epoch_after_ref().to_string(),
            incarnation_before_ref: lower.incarnation_before_ref().to_string(),
            incarnation_after_ref: lower.incarnation_after_ref().to_string(),
            prior_generation_ref: lower.prior_generation_ref().to_string(),
            successor_generation_ref: lower.successor_generation_ref().to_string(),
            capability_lineage_ref: lower.capability_lineage_ref().to_string(),
            witness_lineage_ref: lower.witness_lineage_ref().to_string(),
            request_frontier_ref: relation_observer_ref(&format!(
                "participant-leave-request-frontier:{}:{}",
                lower.m9_transition_ref(),
                lower.lifecycle_enqueue_occurrence_id(),
            )),
            result_frontier_ref: relation_observer_ref(&format!(
                "participant-leave-result-frontier:{}:{}",
                lower.m9_transition_ref(),
                lower.lifecycle_receipt_occurrence_id(),
            )),
            selected_anchor_after: relation_shadow.selected_anchor().to_string(),
            selected_floor_after: relation_shadow.selected_floor().to_string(),
            invalidates_relation_primary: true,
            direct_owner_mutation: false,
            fixture_schedule_authority_injection: false,
            relation_degradation: relation_degradation.clone(),
        };
        self.joined_report.push(format!(
            "typed-participant-leave:action={};source_derived={};external_lifecycle_request={};participant_locus={};retired_membership_locus={};m9_transition_kind={};relation={};relation_owner_locus={};request_identity={};request_enqueue_occurrence_ref={};dispatch_occurrence_ref={};receive_occurrence_ref={};serve_occurrence_ref={};receipt_occurrence_ref={};m9_retire_occurrence_ref={};relation_publish_occurrence_ref={};membership_epoch_monotone={};invalidates_relation_primary={};selected_anchor_after={};selected_floor_after={};direct_owner_mutation={};direct_consumer_mutation={}",
            evidence.action,
            evidence.source_derived,
            evidence.external_lifecycle_request,
            evidence.participant_locus,
            evidence.retired_membership_locus,
            evidence.m9_transition_kind,
            evidence.relation,
            evidence.relation_owner_locus,
            evidence.request_identity,
            evidence.request_enqueue_occurrence_ref,
            evidence.dispatch_occurrence_ref,
            evidence.receive_occurrence_ref,
            evidence.serve_occurrence_ref,
            evidence.receipt_occurrence_ref,
            evidence.m9_retire_occurrence_ref,
            relation_publish_occurrence_ref,
            evidence.membership_epoch_monotone,
            evidence.invalidates_relation_primary,
            evidence.selected_anchor_after,
            evidence.selected_floor_after,
            evidence.direct_owner_mutation,
            evidence.direct_consumer_mutation,
        ));
        self.completed_participant_leaves
            .insert(relation.to_string(), evidence.clone());
        result.participant_leave_evidence = Some(evidence);
        Ok(result)
    }

    fn record_duplicate_participant_leave_failure(
        &mut self,
        relation: &str,
        before_state_digest: &str,
        before_relation_digest: &str,
        before_snapshot: &Sys5ObserverSafeRuntimeSnapshot,
        active_generation_ref: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let after_state_digest = self.observer_safe_state_digest();
        let after_relation_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let after_snapshot = self.observer_safe_runtime_snapshot();
        let prior = self
            .completed_participant_leaves
            .get(relation)
            .cloned()
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        if before_state_digest != after_state_digest
            || before_relation_digest != after_relation_digest
            || before_snapshot != &after_snapshot
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let request_identity = relation_observer_ref(&format!(
            "duplicate-source-derived-participant-leave:{}:{}:{}",
            relation, active_generation_ref, before_relation_digest
        ));
        let request_enqueue_occurrence_ref = self.next_participant_leave_observer_occurrence(
            "DuplicateLeaveRequest",
            &request_identity,
        )?;
        let reject_occurrence_ref = self.next_participant_leave_observer_occurrence(
            "DuplicateLeaveRejected",
            &request_identity,
        )?;
        let receipt_occurrence_ref = self.next_participant_leave_observer_occurrence(
            "DuplicateLeaveReceipt",
            &request_identity,
        )?;
        let failure = Sys5ParticipantLeaveFailureEvidence {
            attempt: "duplicate_leave".to_string(),
            diagnostic: "DuplicateParticipantLeave".to_string(),
            source_derived: true,
            external_lifecycle_request: true,
            failed_closed: true,
            partial_membership_retired: false,
            capability_or_witness_partially_retired: false,
            m9_successor_installed: false,
            m8_state_mutated: false,
            m8_relation_mutated: false,
            m8_designated_result_mutated: false,
            preserved_successful_m8_result_ref: true,
            request_identity,
            request_enqueue_occurrence_ref,
            reject_occurrence_ref,
            receipt_occurrence_ref,
            checked_membership_identity_ref: prior.checked_membership_identity_ref,
            active_generation_ref: active_generation_ref.to_string(),
            m8_state_digest_before_ref: relation_observer_ref(before_state_digest),
            m8_state_digest_after_ref: relation_observer_ref(&after_state_digest),
            m8_relation_digest_before_ref: relation_observer_ref(before_relation_digest),
            m8_relation_digest_after_ref: relation_observer_ref(&after_relation_digest),
            last_successful_m8_result_ref: prior
                .relation_degradation
                .successor_relation_lineage_ref,
        };
        self.joined_report.push(format!(
            "typed-participant-leave-failure:attempt={};diagnostic={};source_derived={};external_lifecycle_request={};failed_closed={};request_identity={};request_enqueue_occurrence_ref={};reject_occurrence_ref={};receipt_occurrence_ref={};m8_state_digest_before_ref={};m8_state_digest_after_ref={};m8_relation_digest_before_ref={};m8_relation_digest_after_ref={}",
            failure.attempt,
            failure.diagnostic,
            failure.source_derived,
            failure.external_lifecycle_request,
            failure.failed_closed,
            failure.request_identity,
            failure.request_enqueue_occurrence_ref,
            failure.reject_occurrence_ref,
            failure.receipt_occurrence_ref,
            failure.m8_state_digest_before_ref,
            failure.m8_state_digest_after_ref,
            failure.m8_relation_digest_before_ref,
            failure.m8_relation_digest_after_ref,
        ));
        self.last_participant_leave_failure = Some(failure);
        self.reject(Sys5VerticalDiagnosticKind::DuplicateParticipantLeave)
    }

    fn next_participant_leave_observer_occurrence(
        &mut self,
        kind: &str,
        request_identity: &str,
    ) -> Result<String, Sys5VerticalSliceError> {
        next_lifecycle_occurrence_ref(
            &mut self.next_lifecycle_occurrence,
            kind,
            request_identity,
            &self.checked_program_identity_ref,
        )
        .map_err(|_| Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected))
    }

    fn dispatch_relation_invalidate(
        &mut self,
        relation: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        if !self.bindings.relation_ids.contains(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation);
        }
        let receipt = self
            .fabric
            .invalidate_relation_primary(relation)
            .map_err(|_| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        self.relation_receipt_from_sys4(relation, receipt)
    }

    fn dispatch_relation_presentation_gap(
        &mut self,
        relation: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        if !self.bindings.relation_ids.contains(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation);
        }
        let before_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let endpoint_count_before = self.fabric.total_endpoint_carrier_count();
        let projection = self
            .fabric
            .project_relation_presentation_gap(relation)
            .map_err(|_| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let shadow = self
            .fabric
            .relation_imported_shadow(relation, projection.consumer_locus())
            .map_err(|_| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let after_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let endpoint_count_after = self.fabric.total_endpoint_carrier_count();
        if before_digest != after_digest || endpoint_count_before != endpoint_count_after {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        if !projection.is_consumer_local_fallback()
            || projection.publishes_value()
            || !projection.absolute_value_stream().is_empty()
            || projection.derived_visibility()
                != crate::m8_runtime_relation_projection::M8RestrictionPolicy::Restricted
            || projection.redaction_policy() != "relation-redacted"
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let semantic = shadow.semantic();
        let relation_shadow = Sys5RelationObserverShadow {
            relation: shadow.relation().to_string(),
            owner_locus: shadow.owner_locus().to_string(),
            consumer_locus: shadow.consumer_locus().to_string(),
            selected_anchor: semantic.selected_anchor().to_string(),
            selected_floor: match semantic.selected_floor() {
                crate::m8_runtime_owner_queue::M8RelationFloor::Live => "live-primary".to_string(),
                crate::m8_runtime_owner_queue::M8RelationFloor::Anchor => {
                    "fallback-anchor".to_string()
                }
                crate::m8_runtime_owner_queue::M8RelationFloor::Frozen => {
                    "frozen-fallback".to_string()
                }
            },
            lineage_ref: relation_observer_ref(&semantic.lineage().join("\n")),
            semantic_digest: relation_observer_ref(&shadow.semantic_digest()),
            semantic_epoch: semantic.binding_epoch().to_string(),
        };
        // The M8 projection carries the exact `relation-redacted` policy. Its
        // profile-local observer category is therefore `restricted`; retain
        // an opaque reference to that exact M8 policy rather than exposing a
        // raw payload or rematerializing an absolute value stream.
        let presentation_gap_evidence = Sys5PresentationGapEvidence {
            relation: projection.relation().to_string(),
            consumer_locus: projection.consumer_locus().to_string(),
            projection_kind: "consumer-local-fallback".to_string(),
            publishes_value: projection.publishes_value(),
            absolute_stream_count: projection.absolute_value_stream().len(),
            restriction: "restricted".to_string(),
            restriction_ref: relation_observer_ref(&format!(
                "restriction:{:?}",
                projection.derived_visibility()
            )),
            redaction: "restricted".to_string(),
            redaction_ref: relation_observer_ref(&format!(
                "redaction:{}",
                projection.redaction_policy()
            )),
            selected_anchor: projection.selected_anchor().to_string(),
            selected_anchor_ref: relation_observer_ref(projection.selected_anchor()),
            selected_floor: relation_floor_name(projection.selected_floor()).to_string(),
            selected_floor_ref: relation_observer_ref(relation_floor_name(
                projection.selected_floor(),
            )),
            context_frontier_ref: relation_observer_ref(projection.context_frontier()),
            semantic_digest_before: relation_observer_ref(&before_digest),
            semantic_digest_after: relation_observer_ref(&after_digest),
            endpoint_count_before,
            endpoint_count_after,
            derived_from_actual_action: true,
        };
        self.joined_report.push(format!(
            "typed-presentation-gap:relation={};consumer_locus={};projection_kind={};publishes_value={};absolute_stream_count={};restriction={};restriction_ref={};redaction={};redaction_ref={};selected_anchor={};selected_anchor_ref={};selected_floor={};selected_floor_ref={};context_frontier_ref={};semantic_digest_before={};semantic_digest_after={};endpoint_count_before={};endpoint_count_after={};derived_from_actual_action={}",
            presentation_gap_evidence.relation,
            presentation_gap_evidence.consumer_locus,
            presentation_gap_evidence.projection_kind,
            presentation_gap_evidence.publishes_value,
            presentation_gap_evidence.absolute_stream_count,
            presentation_gap_evidence.restriction,
            presentation_gap_evidence.restriction_ref,
            presentation_gap_evidence.redaction,
            presentation_gap_evidence.redaction_ref,
            presentation_gap_evidence.selected_anchor,
            presentation_gap_evidence.selected_anchor_ref,
            presentation_gap_evidence.selected_floor,
            presentation_gap_evidence.selected_floor_ref,
            presentation_gap_evidence.context_frontier_ref,
            presentation_gap_evidence.semantic_digest_before,
            presentation_gap_evidence.semantic_digest_after,
            presentation_gap_evidence.endpoint_count_before,
            presentation_gap_evidence.endpoint_count_after,
            presentation_gap_evidence.derived_from_actual_action,
        ));
        self.joined_report.push(format!(
            "presentation-gap:relation={};consumer={};semantic_ref={}",
            relation_shadow.relation(),
            relation_shadow.consumer_locus(),
            relation_shadow.semantic_digest(),
        ));
        self.relation_shadows.insert(
            (
                relation_shadow.consumer_locus().to_string(),
                relation_shadow.relation().to_string(),
            ),
            relation_shadow.clone(),
        );
        Ok(Sys5VerticalReceipt {
            fabric_instance_ref: self.fabric_instance_ref.clone(),
            source_derived: true,
            source_locus: relation_shadow.consumer_locus().to_string(),
            owner_locus: Some(relation_shadow.owner_locus().to_string()),
            endpoint_chain: None,
            owner_mutations: Vec::new(),
            designated_value_name: None,
            evaluator_locus: None,
            consumer_locus: Some(relation_shadow.consumer_locus().to_string()),
            typed_int: None,
            designated_result_version: None,
            designated_delivery_ref: None,
            designated_cache_binding_ref: None,
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: false,
            relation_shadow: Some(relation_shadow),
            presentation_gap_evidence: Some(presentation_gap_evidence),
            participant_leave_evidence: None,
            fresh_reacquire_evidence: None,
            no_direct_cross_locus_store_mutation: true,
        })
    }

    fn dispatch_relation_fresh_reacquire(
        &mut self,
        relation: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        if !self.bindings.relation_ids.contains(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation);
        }
        if self.fabric.relation_fresh_binding_is_consumed(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed);
        }
        let prior_shadow = self
            .relation_shadows
            .get(&("ViewerC".to_string(), relation.to_string()))
            .cloned()
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        // This must already be present in the integrity-bound local cut when
        // a fresh operation resumes after save/restore.  Resolve it before
        // the lower M9/M8 transition so a wrapper-local missing receipt can
        // never turn a successful transition into an upper-layer error.
        let participant_locus = self
            .completed_participant_leaves
            .get(relation)
            .map(|leave| leave.participant_locus.clone())
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let before_relation_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let receipt = self
            .fabric
            .fresh_reacquire_relation_primary(relation)
            .map_err(|_| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let fresh = receipt.fresh_reacquire().cloned().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
        })?;
        let mut result = self.relation_receipt_from_sys4(relation, receipt)?;
        let relation_shadow = result.relation_shadow.as_ref().cloned().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
        })?;
        let endpoint = result.endpoint_chain.as_ref().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
        })?;
        let after_relation_digest = self
            .fabric
            .relation_semantic_digest(relation)
            .map(str::to_string)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let relation_owner_locus = relation_shadow.owner_locus().to_string();
        let fresh_membership_epoch_distinct = !fresh.fresh_membership_epoch_ref().is_empty()
            && fresh.fresh_membership_epoch_ref() != fresh.prior_membership_epoch_ref();
        let fresh_incarnation_distinct = !fresh.fresh_incarnation_ref().is_empty()
            && fresh.fresh_incarnation_ref() != fresh.prior_incarnation_ref();
        if !fresh.source_derived()
            || fresh.lifecycle_request_identity().is_empty()
            || fresh.lifecycle_enqueue_occurrence_id().is_empty()
            || fresh.m9_reacquire_occurrence_id().is_empty()
            || fresh.lifecycle_receipt_occurrence_id().is_empty()
            || fresh.checked_primary_anchor_ref().is_empty()
            || !fresh_membership_epoch_distinct
            || !fresh_incarnation_distinct
            || fresh.capability_lineage_ref().is_empty()
            || fresh.witness_lineage_ref().is_empty()
            || prior_shadow.selected_floor() != "fallback-anchor"
            || relation_shadow.selected_floor() != "live-primary"
            || endpoint.source_locus != relation_owner_locus
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let evidence = Sys5FreshReacquireEvidence {
            action: "participant_a_fresh_reacquire".to_string(),
            source_derived: true,
            external_lifecycle_request: true,
            relation: relation.to_string(),
            participant_locus: participant_locus.clone(),
            fresh_membership_locus: participant_locus,
            m9_transition_kind: "participant-membership-fresh".to_string(),
            fresh_membership_epoch_distinct,
            fresh_incarnation_distinct,
            fresh_lineage_capability: true,
            fresh_lineage_witness: true,
            relation_owner_locus: relation_owner_locus.clone(),
            relation_owner_authority_preserved: true,
            prior_selected_anchor: prior_shadow.selected_anchor().to_string(),
            selected_anchor_after: relation_shadow.selected_anchor().to_string(),
            selected_floor_after: relation_shadow.selected_floor().to_string(),
            // SYS-4 only returns this evidence after its causality check from
            // the M9 re-admission occurrence to the B-owner publication.
            m9_fresh_membership_precedes_relation_publication: true,
            caller_supplied_epoch_or_incarnation: false,
            caller_supplied_membership_ref: false,
            caller_supplied_authority: false,
            fixture_schedule_authority_injection: false,
            direct_consumer_mutation: false,
            request_identity: fresh.lifecycle_request_identity().to_string(),
            request_enqueue_occurrence_ref: fresh.lifecycle_enqueue_occurrence_id().to_string(),
            dispatch_occurrence_ref: endpoint.dispatch_ref.clone(),
            receive_occurrence_ref: endpoint.receive_ref.clone(),
            serve_occurrence_ref: endpoint.serve_ref.clone(),
            receipt_occurrence_ref: fresh.lifecycle_receipt_occurrence_id().to_string(),
            m9_fresh_membership_occurrence_ref: fresh.m9_reacquire_occurrence_id().to_string(),
            relation_publish_occurrence_ref: endpoint.owner_publish_ref.clone().ok_or_else(
                || {
                    Sys5VerticalSliceError::new(
                        Sys5VerticalDiagnosticKind::RelationTransitionRejected,
                    )
                },
            )?,
            retired_membership_ref: fresh.retired_membership_ref().to_string(),
            fresh_membership_ref: fresh.fresh_membership_ref().to_string(),
            retired_membership_epoch_ref: fresh.retired_membership_epoch_ref().to_string(),
            fresh_membership_epoch_ref: fresh.fresh_membership_epoch_ref().to_string(),
            retired_incarnation_ref: fresh.retired_incarnation_ref().to_string(),
            fresh_incarnation_ref: fresh.fresh_incarnation_ref().to_string(),
            prior_generation_ref: fresh.prior_generation_ref().to_string(),
            successor_generation_ref: fresh.successor_generation_ref().to_string(),
            m9_transition_ref: fresh.m9_transition_ref().to_string(),
            fresh_capability_lineage_ref: fresh.capability_lineage_ref().to_string(),
            fresh_witness_lineage_ref: fresh.witness_lineage_ref().to_string(),
            prior_relation_lineage_ref: prior_shadow.lineage_ref().to_string(),
            successor_relation_lineage_ref: relation_shadow.lineage_ref().to_string(),
            semantic_digest_before_ref: relation_observer_ref(&before_relation_digest),
            semantic_digest_after_ref: relation_observer_ref(&after_relation_digest),
            owner_authority_ref: relation_observer_ref(
                &self.fabric.m8_authority_state_digest(&relation_owner_locus),
            ),
        };
        self.joined_report.push(format!(
            "typed-participant-fresh-reacquire:action={};source_derived={};external_lifecycle_request={};participant_locus={};fresh_membership_locus={};m9_transition_kind={};relation={};relation_owner_locus={};prior_selected_anchor={};selected_anchor_after={};selected_floor_after={};request_identity={};request_enqueue_occurrence_ref={};dispatch_occurrence_ref={};receive_occurrence_ref={};serve_occurrence_ref={};receipt_occurrence_ref={};m9_fresh_membership_occurrence_ref={};relation_publish_occurrence_ref={};caller_supplied_epoch_or_incarnation={};caller_supplied_membership_ref={};caller_supplied_authority={};fixture_schedule_authority_injection={};direct_consumer_mutation={}",
            evidence.action,
            evidence.source_derived,
            evidence.external_lifecycle_request,
            evidence.participant_locus,
            evidence.fresh_membership_locus,
            evidence.m9_transition_kind,
            evidence.relation,
            evidence.relation_owner_locus,
            evidence.prior_selected_anchor,
            evidence.selected_anchor_after,
            evidence.selected_floor_after,
            evidence.request_identity,
            evidence.request_enqueue_occurrence_ref,
            evidence.dispatch_occurrence_ref,
            evidence.receive_occurrence_ref,
            evidence.serve_occurrence_ref,
            evidence.receipt_occurrence_ref,
            evidence.m9_fresh_membership_occurrence_ref,
            evidence.relation_publish_occurrence_ref,
            evidence.caller_supplied_epoch_or_incarnation,
            evidence.caller_supplied_membership_ref,
            evidence.caller_supplied_authority,
            evidence.fixture_schedule_authority_injection,
            evidence.direct_consumer_mutation,
        ));
        result.fresh_reacquire_evidence = Some(evidence);
        Ok(result)
    }

    fn relation_receipt_from_sys4(
        &mut self,
        relation: &str,
        receipt: Sys4RelationEndpointReceipt,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        if !self.bindings.relation_ids.contains(relation) {
            return self.reject(Sys5VerticalDiagnosticKind::UnknownSourceOperation);
        }
        if !self
            .fabric
            .observer_exact_relation_endpoint_receipt(&receipt)
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::RelationTransitionRejected,
            ));
        }
        let (logical_path, source_span) =
            observer_logical_source_span(&receipt.edge().source_ref()).ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        let endpoint_chain = Sys5VerticalEndpointChain {
            source_locus: receipt.edge().source_locus().to_string(),
            target_locus: receipt.edge().target_locus().to_string(),
            edge_kind: receipt.edge().kind(),
            logical_path,
            source_span,
            source_ref: relation_observer_ref(&format!(
                "{}:{}:{}:{}:{}",
                receipt.edge().source_ref().path,
                receipt.edge().source_ref().start_line,
                receipt.edge().source_ref().start_column,
                receipt.edge().source_ref().end_line,
                receipt.edge().source_ref().end_column,
            )),
            core_ref: receipt.edge().core_ref().unwrap_or_default().to_string(),
            artifact_ref: receipt.edge().source_fragment_ref().clone(),
            source_fragment_ref: receipt.edge().source_fragment_ref().clone(),
            target_fragment_ref: receipt.edge().target_fragment_ref().clone(),
            edge_ref: receipt.edge().edge_ref().to_string(),
            request_ref: receipt.request_id().to_string(),
            owner_publish_ref: Some(receipt.owner_publish_occurrence_id().to_string()),
            request_enqueue_ref: receipt.request_enqueue_occurrence_id().to_string(),
            dispatch_ref: receipt
                .transport()
                .source_outbox_dequeue_occurrence_id()
                .to_string(),
            receive_ref: receipt
                .transport()
                .target_inbox_enqueue_occurrence_id()
                .to_string(),
            consumer_observe_ref: Some(receipt.consumer_observe_occurrence_id().to_string()),
            serve_ref: receipt.consumer_serve_occurrence_id().to_string(),
        };
        let relation_shadow = vertical_relation_shadow(&receipt);
        self.record_chain(&endpoint_chain, Some("relation"))?;
        self.joined_report.push(format!(
            "relation-selected:relation={};anchor={};floor={};semantic_ref={}",
            relation_shadow.relation(),
            relation_shadow.selected_anchor(),
            relation_shadow.selected_floor(),
            relation_shadow.semantic_digest(),
        ));
        self.relation_shadows.insert(
            (
                relation_shadow.consumer_locus().to_string(),
                relation_shadow.relation().to_string(),
            ),
            relation_shadow.clone(),
        );
        Ok(Sys5VerticalReceipt {
            fabric_instance_ref: self.fabric_instance_ref.clone(),
            source_derived: true,
            source_locus: endpoint_chain.source_locus.clone(),
            owner_locus: Some(relation_shadow.owner_locus().to_string()),
            endpoint_chain: Some(endpoint_chain),
            owner_mutations: Vec::new(),
            designated_value_name: None,
            evaluator_locus: None,
            consumer_locus: Some(relation_shadow.consumer_locus().to_string()),
            typed_int: None,
            designated_result_version: None,
            designated_delivery_ref: None,
            designated_cache_binding_ref: None,
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: false,
            relation_shadow: Some(relation_shadow),
            presentation_gap_evidence: None,
            participant_leave_evidence: None,
            fresh_reacquire_evidence: None,
            no_direct_cross_locus_store_mutation: true,
        })
    }

    fn dispatch_consumer_capability_revoke(
        &mut self,
        value_name: &str,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let binding = self
            .bindings
            .canonical_designated()
            .cloned()
            .filter(|binding| binding.value_name == value_name)
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::UnknownSourceValue)
            })?;
        let transition = self
            .fabric
            .m9_authority_lifecycle_mut()
            .revoke_designated_consumer_capability(&binding.value_name, &binding.consumer_locus)
            .map_err(Sys5VerticalSliceError::from_dispatch)?;
        let m9_transition_ref = transition.observer_transition_ref().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
        })?;
        let m9_generation_ref = transition.observer_successor_generation_ref();
        self.fabric
            .apply_admitted_authority_lifecycle(transition)
            .map_err(Sys5VerticalSliceError::from_dispatch)?;
        self.joined_report
            .push("auth:consumer-capability-revoked".to_string());
        self.joined_report.push(format!(
            "typed-lifecycle-segment:consumer-capability-revocation:provenance_kind=M9AdmittedLifecycle;lifecycle_kind=consumer-capability-revocation;value_name={};consumer_locus={};m9_transition_ref={};m9_generation_ref={}",
            binding.value_name, binding.consumer_locus, m9_transition_ref, m9_generation_ref
        ));
        Ok(Sys5VerticalReceipt {
            fabric_instance_ref: self.fabric_instance_ref.clone(),
            // Revocation enters through the admitted M9 lifecycle seam. It
            // has no ordinary Surface/Core evaluator or generated endpoint.
            source_derived: false,
            source_locus: "M9AdmittedLifecycle".to_string(),
            owner_locus: None,
            endpoint_chain: None,
            owner_mutations: Vec::new(),
            designated_value_name: Some(binding.value_name),
            evaluator_locus: None,
            consumer_locus: Some(binding.consumer_locus),
            typed_int: None,
            designated_result_version: None,
            designated_delivery_ref: None,
            designated_cache_binding_ref: None,
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: false,
            relation_shadow: None,
            presentation_gap_evidence: None,
            participant_leave_evidence: None,
            fresh_reacquire_evidence: None,
            no_direct_cross_locus_store_mutation: true,
        })
    }

    fn receipt_from_fabric(
        &mut self,
        receipt: crate::sys4_dispatch::FabricReceipt,
        context: Sys5FabricReceiptContext,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let endpoint_chain = self.endpoint_chain_from_fabric_receipt(
            &receipt,
            context.endpoint_edge_kind.ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?,
            context.m8_trace_kind.ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?,
            context.endpoint_source_locus.as_deref().ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?,
            context.endpoint_target_locus.as_deref().ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?,
        )?;
        self.record_chain(&endpoint_chain, context.designated_value_name.as_deref())?;
        if !context.owner_mutations.is_empty() {
            self.joined_report
                .push("owner-mutation:owner-local-rmw".to_string());
        }
        if let Some(value_name) = &context.designated_value_name {
            self.joined_report.push(format!("designated:{value_name}"));
        }
        let typed_int = match receipt.typed_value() {
            RuntimeValue::Int(value)
                if context
                    .designated_value_name
                    .as_deref()
                    .is_none_or(|value_name| {
                        self.startup_plan
                            .observer_visible_designated_value(value_name)
                    }) =>
            {
                Some(value)
            }
            RuntimeValue::Unit => None,
            RuntimeValue::Int(_) => None,
        };
        let designated_result_version = receipt.result_version().map(|version| version.value());
        let designated_delivery_ref = context.designated_value_name.as_ref().map(|value_name| {
            relation_observer_ref(&format!(
                "designated-delivery:{}:{}:{}",
                value_name,
                receipt.delivery_id(),
                designated_result_version.unwrap_or_default(),
            ))
        });
        let designated_cache_binding_ref =
            context
                .designated_value_name
                .as_ref()
                .and_then(|value_name| {
                    designated_result_version.map(|version| {
                        relation_observer_ref(&format!(
                            "designated-cache-binding:{}:{}:{}:{}",
                            value_name,
                            receipt.target_locus(),
                            receipt.delivery_id(),
                            version,
                        ))
                    })
                });
        Ok(Sys5VerticalReceipt {
            fabric_instance_ref: self.fabric_instance_ref.clone(),
            source_derived: true,
            source_locus: receipt.origin_locus().to_string(),
            owner_locus: context.owner_locus,
            endpoint_chain: Some(endpoint_chain),
            owner_mutations: context.owner_mutations,
            designated_value_name: context.designated_value_name,
            evaluator_locus: context.evaluator_locus,
            consumer_locus: self
                .bindings
                .canonical_designated()
                .filter(|binding| binding.value_name == receipt.operation_id())
                .map(|binding| binding.consumer_locus.clone()),
            typed_int,
            designated_result_version,
            designated_delivery_ref,
            designated_cache_binding_ref,
            performed_m8_semantic_consumption: receipt.performed_m8_semantic_consumption(),
            returned_from_designated_cache_after_authority_revalidation: receipt
                .returned_from_designated_cache_after_authority_revalidation(),
            relation_shadow: context.relation_shadow,
            presentation_gap_evidence: None,
            participant_leave_evidence: None,
            fresh_reacquire_evidence: None,
            no_direct_cross_locus_store_mutation: context.no_direct_cross_locus_store_mutation,
        })
    }

    /// A retry is consumer-local validation of an already delivered sealed
    /// result.  It has no newly dispatched endpoint; pretending otherwise
    /// would fabricate a route occurrence.  We retain only the exact M8
    /// cache-validation occurrence and the redacted result surface.
    fn cache_retry_receipt(
        &mut self,
        receipt: crate::sys4_dispatch::FabricReceipt,
        binding: Sys5VerticalDesignatedBinding,
    ) -> Result<Sys5VerticalReceipt, Sys5VerticalSliceError> {
        let node = self
            .fabric
            .observer_exact_m8_occurrence(
                receipt.request_id(),
                crate::m8_runtime_local_cut::M8LocalTraceKind::DesignatedCacheValidated,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?;
        self.joined_report.push(format!("cache-validation:{node}"));
        let typed_int = match receipt.typed_value() {
            RuntimeValue::Int(value)
                if self
                    .startup_plan
                    .observer_visible_designated_value(&binding.value_name) =>
            {
                Some(value)
            }
            RuntimeValue::Unit | RuntimeValue::Int(_) => None,
        };
        let designated_result_version = receipt.result_version().map(|version| version.value());
        let designated_delivery_ref = relation_observer_ref(&format!(
            "designated-delivery:{}:{}",
            binding.value_name,
            receipt.delivery_id(),
        ));
        let designated_cache_binding_ref = receipt.result_version().map(|version| {
            relation_observer_ref(&format!(
                "designated-cache-binding:{}:{}:{}:{}",
                binding.value_name,
                binding.consumer_locus,
                receipt.delivery_id(),
                version.value(),
            ))
        });
        Ok(Sys5VerticalReceipt {
            fabric_instance_ref: self.fabric_instance_ref.clone(),
            source_derived: true,
            source_locus: binding.evaluator_locus,
            owner_locus: None,
            endpoint_chain: None,
            owner_mutations: Vec::new(),
            designated_value_name: Some(binding.value_name),
            evaluator_locus: None,
            consumer_locus: Some(binding.consumer_locus),
            typed_int,
            designated_result_version,
            designated_delivery_ref: Some(designated_delivery_ref),
            designated_cache_binding_ref,
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: true,
            relation_shadow: None,
            presentation_gap_evidence: None,
            participant_leave_evidence: None,
            fresh_reacquire_evidence: None,
            no_direct_cross_locus_store_mutation: true,
        })
    }

    /// The designated path has three generated endpoint segments.  Build the
    /// observer report only from their exact retained rows, and require the
    /// corresponding causal edges; no source operation hash or request-ID
    /// substitution is accepted as a stand-in.
    fn record_designated_causal_segments(
        &mut self,
        receipt: &crate::sys4_dispatch::FabricReceipt,
        binding: &Sys5VerticalDesignatedBinding,
    ) -> Result<(), Sys5VerticalSliceError> {
        let input_source = binding.input_source_locus.as_deref().ok_or_else(|| {
            Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
        })?;
        let input_request = self
            .fabric
            .observer_exact_endpoint_segment(
                receipt.request_id(),
                crate::sys4_dispatch::Sys4TraceKind::Dispatched,
                CommunicationEdgeKind::DesignatedInputRequest,
                &binding.evaluator_locus,
                input_source,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?;
        let input_receipt = self
            .fabric
            .observer_exact_endpoint_segment(
                receipt.request_id(),
                crate::sys4_dispatch::Sys4TraceKind::Dispatched,
                CommunicationEdgeKind::DesignatedInputReceipt,
                input_source,
                &binding.evaluator_locus,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?;
        let result_delivery = self
            .fabric
            .observer_exact_endpoint_segment(
                receipt.request_id(),
                crate::sys4_dispatch::Sys4TraceKind::DesignatedResultDispatched,
                CommunicationEdgeKind::DesignatedResultDelivery,
                &binding.evaluator_locus,
                &binding.consumer_locus,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?;
        if !self.fabric.observer_causally_reaches(
            input_receipt.occurrence_ref(),
            input_request.occurrence_ref(),
        ) || !self.fabric.observer_causally_reaches(
            result_delivery.occurrence_ref(),
            input_receipt.occurrence_ref(),
        ) {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::DispatchRejected,
            ));
        }
        self.joined_report.push(format!(
            "segment:designated-input-request:{}->{}:{}",
            binding.evaluator_locus,
            input_source,
            input_request.occurrence_ref(),
        ));
        self.joined_report.push(format!(
            "segment:designated-input-receipt:{}->{}:{}",
            input_source,
            binding.evaluator_locus,
            input_receipt.occurrence_ref(),
        ));
        self.joined_report.push(format!(
            "segment:designated-result-delivery:{}->{}:{}",
            binding.evaluator_locus,
            binding.consumer_locus,
            result_delivery.occurrence_ref(),
        ));
        self.joined_report
            .push("causality:designated-input-request->designated-input-receipt".to_string());
        self.joined_report
            .push("causality:designated-input-receipt->designated-result-delivery".to_string());
        self.joined_report
            .push("causality:designated-result-delivery->viewer-consume".to_string());
        Ok(())
    }

    fn endpoint_chain_from_fabric_receipt(
        &self,
        receipt: &crate::sys4_dispatch::FabricReceipt,
        edge_kind: CommunicationEdgeKind,
        serve_kind: crate::m8_runtime_local_cut::M8LocalTraceKind,
        endpoint_source_locus: &str,
        endpoint_target_locus: &str,
    ) -> Result<Sys5VerticalEndpointChain, Sys5VerticalSliceError> {
        let (dispatch_kind, receive_kind) = match edge_kind {
            CommunicationEdgeKind::DesignatedResultDelivery => (
                crate::sys4_dispatch::Sys4TraceKind::DesignatedResultDispatched,
                crate::sys4_dispatch::Sys4TraceKind::DesignatedResultReceived,
            ),
            _ => (
                crate::sys4_dispatch::Sys4TraceKind::Dispatched,
                crate::sys4_dispatch::Sys4TraceKind::Received,
            ),
        };
        let endpoint_occurrences = self
            .fabric
            .observer_exact_endpoint_occurrences(
                receipt.request_id(),
                dispatch_kind,
                receive_kind,
                edge_kind,
                endpoint_source_locus,
                endpoint_target_locus,
            )
            .ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?;
        let serve_ref = self
            .fabric
            .observer_exact_m8_occurrence(receipt.request_id(), serve_kind)
            // A restored designated evaluator may retain a matching sealed
            // result and report this new tick as a genuine idempotent
            // evaluation.  That is a distinct actual M8 occurrence, not a
            // fabricated publish; accept it only for the designated-eval
            // observer path and keep its exact request identity.
            .or_else(|| {
                (serve_kind == crate::m8_runtime_local_cut::M8LocalTraceKind::DesignatedValuePublished)
                    .then(|| {
                        self.fabric.observer_exact_m8_occurrence(
                            receipt.request_id(),
                            crate::m8_runtime_local_cut::M8LocalTraceKind::DesignatedEvaluationIdempotent,
                        )
                    })
                    .flatten()
            })
            .ok_or_else(|| Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected))?;
        if !self
            .fabric
            .observer_causally_reaches(serve_ref, endpoint_occurrences.receive_occurrence_id())
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::DispatchRejected,
            ));
        }
        if receipt.request_id().is_empty()
            || receipt.request_id() == endpoint_occurrences.request_enqueue_occurrence_id()
            || receipt.request_id() == endpoint_occurrences.dispatch_occurrence_id()
            || receipt.request_id() == endpoint_occurrences.receive_occurrence_id()
            || receipt.request_id() == serve_ref
        {
            return Err(Sys5VerticalSliceError::new(
                Sys5VerticalDiagnosticKind::DispatchRejected,
            ));
        }
        let (logical_path, source_span) =
            observer_logical_source_span(endpoint_occurrences.source_ref()).ok_or_else(|| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::DispatchRejected)
            })?;
        Ok(Sys5VerticalEndpointChain {
            source_locus: endpoint_source_locus.to_string(),
            target_locus: endpoint_target_locus.to_string(),
            edge_kind,
            logical_path,
            source_span,
            source_ref: relation_observer_ref(&format!(
                "{}:{}:{}:{}:{}",
                endpoint_occurrences.source_ref().path,
                endpoint_occurrences.source_ref().start_line,
                endpoint_occurrences.source_ref().start_column,
                endpoint_occurrences.source_ref().end_line,
                endpoint_occurrences.source_ref().end_column,
            )),
            core_ref: endpoint_occurrences.core_ref().to_string(),
            artifact_ref: endpoint_occurrences.source_fragment_ref().to_string(),
            source_fragment_ref: endpoint_occurrences.source_fragment_ref().to_string(),
            target_fragment_ref: endpoint_occurrences.target_fragment_ref().to_string(),
            edge_ref: endpoint_occurrences.edge_ref().to_string(),
            request_ref: receipt.request_id().to_string(),
            owner_publish_ref: None,
            request_enqueue_ref: endpoint_occurrences
                .request_enqueue_occurrence_id()
                .to_string(),
            dispatch_ref: endpoint_occurrences.dispatch_occurrence_id().to_string(),
            receive_ref: endpoint_occurrences.receive_occurrence_id().to_string(),
            consumer_observe_ref: None,
            serve_ref: serve_ref.to_string(),
        })
    }

    fn owner_mutations_since(
        &self,
        before: &crate::sys4_dispatch::FabricSemanticSnapshot,
        after: &crate::sys4_dispatch::FabricSemanticSnapshot,
        owner_locus: &str,
    ) -> Vec<Sys5VerticalOwnerMutation> {
        self.startup_plan
            .observer_safe_cells
            .iter()
            .filter(|shape| shape.locus == owner_locus)
            .filter_map(|shape| {
                let old_value =
                    before.int(&shape.locus, &shape.state, &shape.index, &shape.field)?;
                let new_value =
                    after.int(&shape.locus, &shape.state, &shape.index, &shape.field)?;
                (old_value != new_value).then(|| Sys5VerticalOwnerMutation {
                    locus: shape.locus.clone(),
                    state: shape.state.clone(),
                    index: shape.index.clone(),
                    field: shape.field.clone(),
                    old_value,
                    new_value,
                })
            })
            .collect()
    }

    fn record_chain(
        &mut self,
        chain: &Sys5VerticalEndpointChain,
        designated: Option<&str>,
    ) -> Result<(), Sys5VerticalSliceError> {
        self.joined_report
            .push(format!("source-ref:{}", chain.source_ref));
        self.joined_report
            .push(format!("core-ref:{}", chain.core_ref));
        self.joined_report
            .push(format!("artifact-ref:{}", chain.artifact_ref));
        self.joined_report
            .push(format!("edge-ref:{}", chain.edge_ref));
        self.joined_report
            .push(format!("request:{}", chain.request_ref));
        self.joined_report
            .push(format!("request-enqueue:{}", chain.request_enqueue_ref));
        self.joined_report
            .push(format!("dispatch:{}", chain.dispatch_ref));
        self.joined_report
            .push(format!("receive:{}", chain.receive_ref));
        self.joined_report
            .push(format!("serve:{}", chain.serve_ref));
        if chain.edge_kind == CommunicationEdgeKind::RelationProjectionPublication {
            let (Some(owner_publish_ref), Some(consumer_observe_ref)) = (
                chain.owner_publish_ref.as_deref(),
                chain.consumer_observe_ref.as_deref(),
            ) else {
                return Err(Sys5VerticalSliceError::new(
                    Sys5VerticalDiagnosticKind::RelationTransitionRejected,
                ));
            };
            self.joined_report.push(format!(
                "typed-segment:relation-projection-publication:provenance_kind=OrdinarySourceCore;logical_path={};source_span={};core_ref={};source_fragment_ref={};target_fragment_ref={};edge_ref={};request_identity={};owner_publish_occurrence_id={};request_enqueue_occurrence_id={};dispatch_occurrence_id={};receive_occurrence_id={};consumer_observe_occurrence_id={};serve_occurrence_id={};causal_path=owner_publish_occurrence_id->request_enqueue_occurrence_id->dispatch_occurrence_id->receive_occurrence_id->consumer_observe_occurrence_id->serve_occurrence_id",
                chain.logical_path,
                chain.source_span,
                chain.core_ref,
                chain.source_fragment_ref,
                chain.target_fragment_ref,
                chain.edge_ref,
                chain.request_ref,
                owner_publish_ref,
                chain.request_enqueue_ref,
                chain.dispatch_ref,
                chain.receive_ref,
                consumer_observe_ref,
                chain.serve_ref,
            ));
        } else if let Some(segment_kind) = vertical_typed_segment_kind(chain.edge_kind) {
            self.joined_report.push(format!(
                "typed-segment:{segment_kind}:provenance_kind=OrdinarySourceCore;logical_path={};source_span={};core_ref={};source_fragment_ref={};target_fragment_ref={};edge_ref={};request_identity={};request_enqueue_occurrence_id={};dispatch_occurrence_id={};receive_occurrence_id={};serve_occurrence_id={};causal_path=request_enqueue_occurrence_id->dispatch_occurrence_id->receive_occurrence_id->serve_occurrence_id",
                chain.logical_path,
                chain.source_span,
                chain.core_ref,
                chain.source_fragment_ref,
                chain.target_fragment_ref,
                chain.edge_ref,
                chain.request_ref,
                chain.request_enqueue_ref,
                chain.dispatch_ref,
                chain.receive_ref,
                chain.serve_ref,
            ));
        }
        if let Some(value_name) = designated {
            self.joined_report.push(format!("designated:{value_name}"));
        }
        Ok(())
    }

    fn reject<T>(&mut self, kind: Sys5VerticalDiagnosticKind) -> Result<T, Sys5VerticalSliceError> {
        self.joined_report.push(format!("failure:{kind:?}"));
        Err(Sys5VerticalSliceError::new(kind))
    }
}

fn vertical_relation_shadow(receipt: &Sys4RelationEndpointReceipt) -> Sys5RelationObserverShadow {
    let shadow = receipt.shadow();
    let semantic = shadow.semantic();
    Sys5RelationObserverShadow {
        relation: shadow.relation().to_string(),
        owner_locus: shadow.owner_locus().to_string(),
        consumer_locus: shadow.consumer_locus().to_string(),
        selected_anchor: semantic.selected_anchor().to_string(),
        selected_floor: match semantic.selected_floor() {
            crate::m8_runtime_owner_queue::M8RelationFloor::Live => "live-primary".to_string(),
            crate::m8_runtime_owner_queue::M8RelationFloor::Anchor => "fallback-anchor".to_string(),
            crate::m8_runtime_owner_queue::M8RelationFloor::Frozen => "frozen-fallback".to_string(),
        },
        lineage_ref: relation_observer_ref(&semantic.lineage().join("\n")),
        semantic_digest: relation_observer_ref(&shadow.semantic_digest()),
        semantic_epoch: semantic.binding_epoch().to_string(),
    }
}

fn relation_floor_name(floor: crate::m8_runtime_owner_queue::M8RelationFloor) -> &'static str {
    match floor {
        crate::m8_runtime_owner_queue::M8RelationFloor::Live => "live-primary",
        crate::m8_runtime_owner_queue::M8RelationFloor::Anchor => "fallback-anchor",
        crate::m8_runtime_owner_queue::M8RelationFloor::Frozen => "frozen-fallback",
    }
}

fn sys5_patch_diagnostic(kind: Sys4PatchDiagnosticKind) -> Sys5PatchDiagnosticKind {
    match kind {
        Sys4PatchDiagnosticKind::StaleFrontier => Sys5PatchDiagnosticKind::StaleFrontier,
        Sys4PatchDiagnosticKind::NonQuiescentPendingCarrier => {
            Sys5PatchDiagnosticKind::NonQuiescentPendingCarrier
        }
        Sys4PatchDiagnosticKind::TopologyOwnerRouteMismatch => {
            Sys5PatchDiagnosticKind::TopologyOwnerRouteMismatch
        }
        Sys4PatchDiagnosticKind::OwnerRmwExpressionChanged => {
            Sys5PatchDiagnosticKind::OwnerRmwExpressionChanged
        }
        Sys4PatchDiagnosticKind::NonDesignatedCoreMaterialChanged => {
            Sys5PatchDiagnosticKind::NonDesignatedCoreMaterialChanged
        }
        Sys4PatchDiagnosticKind::M9AuthorityLineageMismatch => {
            Sys5PatchDiagnosticKind::M9AuthorityLineageMismatch
        }
        Sys4PatchDiagnosticKind::IncompleteCandidateAdmission => {
            Sys5PatchDiagnosticKind::IncompleteCandidateAdmission
        }
        Sys4PatchDiagnosticKind::BackendIneligible => Sys5PatchDiagnosticKind::BackendIneligible,
    }
}

fn sys5_patch_outcome(
    outcome: &Sys4PatchOutcome,
    patch_occurrence_ref: String,
) -> Sys5PatchOutcome {
    let verdict = match outcome.verdict() {
        Sys4PatchVerdict::Accepted => Sys5PatchVerdict::Accepted,
        Sys4PatchVerdict::Rejected => Sys5PatchVerdict::Rejected,
    };
    let base_ref = patch_frontier_ref(&format!("{:?}", outcome.base_frontier()));
    let activation_is_successor = outcome
        .activation_frontier()
        .is_exact_successor_of(outcome.base_frontier());
    let activation_ref = patch_frontier_ref(&format!("{:?}", outcome.activation_frontier()));
    Sys5PatchOutcome {
        verdict,
        primary_diagnostic_kind: outcome.primary_diagnostic_kind().map(sys5_patch_diagnostic),
        patch_occurrence_ref,
        lifecycle: Sys5PatchLifecycle {
            verdict,
            diagnostic: outcome.primary_diagnostic_kind().map(sys5_patch_diagnostic),
            source_first_checked_projection_and_m9_admission: outcome
                .lifecycle()
                .contains_source_first_checked_projection_and_m9_admission(),
        },
        boundary_inspection: Sys5LocalPatchBoundaryInspection {
            caller_supplied_core_authority_or_frontier: false,
            runtime_received_only_checked_patch_candidate: outcome
                .boundary_inspection()
                .runtime_received_only_checked_patch_candidate(),
        },
        base_frontier: Sys5PatchFrontier {
            ref_digest: base_ref.clone(),
            predecessor_ref_digest: None,
        },
        activation_frontier: Sys5PatchFrontier {
            ref_digest: activation_ref,
            predecessor_ref_digest: activation_is_successor.then_some(base_ref),
        },
    }
}

struct Sys5LifecycleBoundaryRefs<'a> {
    before_program_ref: &'a str,
    after_program_ref: &'a str,
    before_artifact_ref: &'a str,
    after_artifact_ref: &'a str,
    before_frontier_ref: &'a str,
    after_frontier_ref: &'a str,
}

fn lifecycle_joined_row(
    kind: &str,
    refs: Sys5LifecycleBoundaryRefs<'_>,
    occurrence: Option<(&str, &str)>,
) -> String {
    // The source/Core labels intentionally carry only stable opaque refs. The
    // local facade does not retain source text or raw authority material in
    // lifecycle/devtools rows.
    let occurrence = occurrence
        .map(|(field, reference)| format!(";{field}={reference}"))
        .unwrap_or_default();
    format!(
        "lifecycle:{kind}:before_source_ref={};after_source_ref={};before_core_ref={};after_core_ref={};before_artifact_ref={};after_artifact_ref={};before_activation_frontier={};after_activation_frontier={}{occurrence}",
        refs.before_program_ref,
        refs.after_program_ref,
        refs.before_program_ref,
        refs.after_program_ref,
        refs.before_artifact_ref,
        refs.after_artifact_ref,
        refs.before_frontier_ref,
        refs.after_frontier_ref,
    )
}

/// Render only a checked logical source location.  The source checker already
/// rejects absolute, traversal, and host-native paths; repeat that narrow
/// check at the observer boundary so a corrupt retained trace fails closed
/// instead of leaking a host path or source text through devtools.
fn observer_logical_source_span(
    source_ref: &crate::sys3_projection::SourceRefView,
) -> Option<(String, String)> {
    let path = &source_ref.path;
    if !is_allowed_logical_source_path(path) {
        return None;
    }
    Some((
        path.clone(),
        format!(
            "{}:{}-{}:{}",
            source_ref.start_line,
            source_ref.start_column,
            source_ref.end_line,
            source_ref.end_column
        ),
    ))
}

fn vertical_typed_segment_kind(kind: CommunicationEdgeKind) -> Option<&'static str> {
    match kind {
        CommunicationEdgeKind::OwnerRequest => Some("owner-request"),
        CommunicationEdgeKind::DesignatedInputRequest => Some("designated-input-request"),
        CommunicationEdgeKind::DesignatedInputReceipt => Some("designated-input-receipt"),
        CommunicationEdgeKind::DesignatedResultDelivery => Some("designated-result-delivery"),
        CommunicationEdgeKind::OwnerReplyReceipt
        | CommunicationEdgeKind::RelationProjectionPublication
        | CommunicationEdgeKind::AbsoluteValueStream => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5LocalAdmissionErrorKind {
    UnknownPrincipal,
    UnknownLocus,
    InvalidAdmissionIdentity,
    DuplicateMembership,
    ConflictingMembership,
    MissingRequiredMembership,
    PrincipalPolicyMismatch,
    MissingRelationBootstrapPolicy,
    MissingAuthDischarge,
    UnknownAuthDischarge,
    MissingVerificationDischarge,
    UnknownVerificationDischarge,
    M9Rejected,
    ProjectionFabricMismatch,
    BackendIneligible,
    IncompleteSourceDerivedInventory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5LocalAdmissionError {
    kind: Sys5LocalAdmissionErrorKind,
}

impl Sys5LocalAdmissionError {
    fn new(kind: Sys5LocalAdmissionErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5LocalAdmissionErrorKind {
        self.kind
    }

    pub const fn rejected_before_authority_issuance(&self) -> bool {
        matches!(
            self.kind,
            Sys5LocalAdmissionErrorKind::UnknownPrincipal
                | Sys5LocalAdmissionErrorKind::UnknownLocus
                | Sys5LocalAdmissionErrorKind::InvalidAdmissionIdentity
                | Sys5LocalAdmissionErrorKind::DuplicateMembership
                | Sys5LocalAdmissionErrorKind::ConflictingMembership
                | Sys5LocalAdmissionErrorKind::MissingRequiredMembership
                | Sys5LocalAdmissionErrorKind::PrincipalPolicyMismatch
                | Sys5LocalAdmissionErrorKind::MissingRelationBootstrapPolicy
                | Sys5LocalAdmissionErrorKind::MissingAuthDischarge
                | Sys5LocalAdmissionErrorKind::UnknownAuthDischarge
                | Sys5LocalAdmissionErrorKind::MissingVerificationDischarge
                | Sys5LocalAdmissionErrorKind::UnknownVerificationDischarge
                | Sys5LocalAdmissionErrorKind::BackendIneligible
        )
    }

    pub const fn partial_admission(&self) -> Option<()> {
        None
    }

    /// Every admission error is reported before a `LocalFabric` exists.
    /// This permits callers to distinguish an input rejection from a runtime
    /// failure without gaining a partial fabric handle.
    pub const fn rejected_before_live_runtime(&self) -> bool {
        true
    }

    pub const fn partial_runtime(&self) -> Option<()> {
        None
    }
}

/// Observer-safe report of the checked residual discharges used by a finite
/// admission.  The booleans attest that the source and M9 lanes were both
/// consulted; they are not authority payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5AuthDischargeSummary {
    authority: String,
    source_ref_present: bool,
    m9_evidence_ref_present: bool,
    discharged: bool,
}

impl Sys5AuthDischargeSummary {
    pub const fn is_discharged(&self) -> bool {
        self.discharged
    }

    pub const fn has_source_ref(&self) -> bool {
        self.source_ref_present
    }

    pub const fn has_m9_evidence_ref(&self) -> bool {
        self.m9_evidence_ref_present
    }

    pub const fn grants_runtime_authority_by_name_only(&self) -> bool {
        false
    }
}

/// Observer-safe report of the separate finite verification discharge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5VerificationDischargeSummary {
    verifier: String,
    source_ref_present: bool,
    finite_refinement_evidence_ref_present: bool,
    discharged: bool,
}

impl Sys5VerificationDischargeSummary {
    pub const fn is_discharged(&self) -> bool {
        self.discharged
    }

    pub const fn has_source_ref(&self) -> bool {
        self.source_ref_present
    }

    pub const fn has_finite_refinement_evidence_ref(&self) -> bool {
        self.finite_refinement_evidence_ref_present
    }

    pub const fn is_merged_into_auth(&self) -> bool {
        false
    }
}

/// The finite runtime inventory as source operation identities, not as M9 or
/// M8 record values.  It gives devtools enough causal structure to describe
/// admission without serializing credentials, capability scopes, witnesses,
/// or provider data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5AdmissionInventory {
    checked_program_identity: String,
    owner_rmw: Vec<Sys5OwnerRmwInventoryRow>,
    relation_lifecycle: Vec<Sys5RelationLifecycleInventoryRow>,
    designated_evaluators: Vec<Sys5DesignatedEvaluatorInventoryRow>,
    designated_remote_inputs: Vec<Sys5DesignatedRemoteInputInventoryRow>,
    named_consumers: Vec<Sys5NamedConsumerInventoryRow>,
    #[serde(skip)]
    semantic_rows: Sys5SemanticRowSets,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Sys5SemanticRowSets {
    owner_lineages: BTreeSet<(String, String, String, String)>,
    relation_transitions: BTreeSet<(String, String)>,
    designated_evaluators: BTreeSet<(String, String)>,
    designated_remote_input_lineages: BTreeSet<(String, String, String, usize, String)>,
    designated_consumers: BTreeSet<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5OwnerRmwInventoryRow {
    operation_id: String,
    principal: String,
    origin_locus: String,
    owner_locus: String,
}

/// Lifecycle events supported by the current local relation schedule.  They
/// are semantic relation transitions, not a caller-provided authority scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Sys5RelationLifecycleKind {
    Invalidate,
    FreshReacquire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5RelationLifecycleInventoryRow {
    relation: String,
    kind: Sys5RelationLifecycleKind,
    bootstrap_policy: String,
    core_derived: bool,
    grants_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5DesignatedEvaluatorInventoryRow {
    value_name: String,
    evaluator_locus: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5DesignatedRemoteInputInventoryRow {
    value_name: String,
    dependency_index: usize,
    source_owner_locus: String,
    evaluator_locus: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Sys5NamedConsumerInventoryRow {
    value_name: String,
    consumer_locus: String,
}

impl Sys5AdmissionInventory {
    fn from_checked(checked: &CheckedSurfaceV0) -> Self {
        let mut owner_rmw = Vec::new();
        let mut relation_lifecycle = Vec::new();
        let mut designated_evaluators = Vec::new();
        let mut designated_remote_inputs = Vec::new();
        let mut named_consumers = Vec::new();
        let mut semantic_rows = Sys5SemanticRowSets::default();

        for evaluation in checked.evaluations() {
            if let Some(owner) = evaluation.owner_rmw_core() {
                owner_rmw.push(Sys5OwnerRmwInventoryRow {
                    operation_id: evaluation.name().to_string(),
                    principal: evaluation.actor_authority_origin().to_string(),
                    origin_locus: owner.authority_origin_locus().to_string(),
                    owner_locus: owner.owner_locus().to_string(),
                });
                semantic_rows.owner_lineages.insert((
                    evaluation.name().to_string(),
                    evaluation.actor_authority_origin().to_string(),
                    owner.authority_origin_locus().to_string(),
                    owner.owner_locus().to_string(),
                ));
            }
            if evaluation.relation_core().is_some() {
                for kind in [
                    Sys5RelationLifecycleKind::Invalidate,
                    Sys5RelationLifecycleKind::FreshReacquire,
                ] {
                    relation_lifecycle.push(Sys5RelationLifecycleInventoryRow {
                        relation: evaluation.name().to_string(),
                        kind,
                        bootstrap_policy: "bounded-local-bootstrap".to_string(),
                        core_derived: false,
                        grants_authority: false,
                    });
                    semantic_rows.relation_transitions.insert((
                        evaluation.name().to_string(),
                        match kind {
                            Sys5RelationLifecycleKind::Invalidate => "invalidate_primary",
                            Sys5RelationLifecycleKind::FreshReacquire => "reacquire_primary",
                        }
                        .to_string(),
                    ));
                }
            }
            if let Some(designated) = evaluation.designated_core() {
                let value_name = format!("{}.{}", designated.evaluator(), designated.result());
                designated_evaluators.push(Sys5DesignatedEvaluatorInventoryRow {
                    value_name: value_name.clone(),
                    evaluator_locus: designated.evaluator().to_string(),
                });
                semantic_rows
                    .designated_evaluators
                    .insert((value_name.clone(), designated.evaluator().to_string()));
                for (dependency_index, dependency) in designated
                    .generated_remote_input_dependencies()
                    .iter()
                    .enumerate()
                {
                    designated_remote_inputs.push(Sys5DesignatedRemoteInputInventoryRow {
                        value_name: value_name.clone(),
                        dependency_index,
                        source_owner_locus: dependency.source_owner_locus().to_string(),
                        evaluator_locus: designated.evaluator().to_string(),
                    });
                    semantic_rows.designated_remote_input_lineages.insert((
                        dependency.source_owner_locus().to_string(),
                        designated.evaluator().to_string(),
                        designated.result().to_string(),
                        dependency_index,
                        designated
                            .trigger()
                            .frontier()
                            .unwrap_or_default()
                            .to_string(),
                    ));
                }
            }
            if let Some(consumer) = evaluation.designated_result_consumer_core() {
                named_consumers.push(Sys5NamedConsumerInventoryRow {
                    value_name: format!("{}.{}", consumer.evaluator(), consumer.result()),
                    consumer_locus: consumer.consumer_locus().to_string(),
                });
                semantic_rows.designated_consumers.insert((
                    format!("{}.{}", consumer.evaluator(), consumer.result()),
                    consumer.consumer_locus().to_string(),
                ));
            }
        }
        owner_rmw.sort_by(|left, right| {
            (
                &left.operation_id,
                &left.principal,
                &left.origin_locus,
                &left.owner_locus,
            )
                .cmp(&(
                    &right.operation_id,
                    &right.principal,
                    &right.origin_locus,
                    &right.owner_locus,
                ))
        });
        relation_lifecycle.sort_by(|left, right| {
            (&left.relation, left.kind as u8).cmp(&(&right.relation, right.kind as u8))
        });
        designated_evaluators.sort_by(|left, right| {
            (&left.value_name, &left.evaluator_locus)
                .cmp(&(&right.value_name, &right.evaluator_locus))
        });
        designated_remote_inputs.sort_by(|left, right| {
            (
                &left.value_name,
                left.dependency_index,
                &left.source_owner_locus,
                &left.evaluator_locus,
            )
                .cmp(&(
                    &right.value_name,
                    right.dependency_index,
                    &right.source_owner_locus,
                    &right.evaluator_locus,
                ))
        });
        named_consumers.sort_by(|left, right| {
            (&left.value_name, &left.consumer_locus)
                .cmp(&(&right.value_name, &right.consumer_locus))
        });
        Self {
            checked_program_identity: checked_program_identity_ref(
                &checked.program_identity().stable_key(),
            ),
            owner_rmw,
            relation_lifecycle,
            designated_evaluators,
            designated_remote_inputs,
            named_consumers,
            semantic_rows,
        }
    }

    pub fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity
    }

    pub fn owner_rmw_operation_ids(&self) -> Vec<&str> {
        self.owner_rmw
            .iter()
            .map(|row| row.operation_id.as_str())
            .collect()
    }

    pub fn contains_owner_rmw(
        &self,
        operation_id: &str,
        principal: &str,
        origin_locus: &str,
        owner_locus: &str,
    ) -> bool {
        self.owner_rmw.iter().any(|row| {
            row.operation_id == operation_id
                && row.principal == principal
                && row.origin_locus == origin_locus
                && row.owner_locus == owner_locus
        })
    }

    pub fn contains_relation_lifecycle(
        &self,
        relation: &str,
        kind: Sys5RelationLifecycleKind,
    ) -> bool {
        self.relation_lifecycle
            .iter()
            .any(|row| row.relation == relation && row.kind == kind)
    }

    pub fn relation_lifecycle(
        &self,
        relation: &str,
        kind: Sys5RelationLifecycleKind,
    ) -> Option<&Sys5RelationLifecycleInventoryRow> {
        self.relation_lifecycle
            .iter()
            .find(|row| row.relation == relation && row.kind == kind)
    }

    pub fn contains_designated_evaluator(&self, value_name: &str, evaluator_locus: &str) -> bool {
        self.designated_evaluators
            .iter()
            .any(|row| row.value_name == value_name && row.evaluator_locus == evaluator_locus)
    }

    pub fn contains_designated_remote_input(
        &self,
        value_name: &str,
        dependency_index: usize,
        source_owner_locus: &str,
        evaluator_locus: &str,
    ) -> bool {
        self.designated_remote_inputs.iter().any(|row| {
            row.value_name == value_name
                && row.dependency_index == dependency_index
                && row.source_owner_locus == source_owner_locus
                && row.evaluator_locus == evaluator_locus
        })
    }

    pub fn contains_named_consumer(&self, value_name: &str, consumer_locus: &str) -> bool {
        self.named_consumers
            .iter()
            .any(|row| row.value_name == value_name && row.consumer_locus == consumer_locus)
    }

    pub fn covers_every_generated_remote_input(&self) -> bool {
        self.designated_remote_inputs
            .windows(2)
            .all(|rows| rows[0] != rows[1])
    }

    pub fn covers_every_relation_lifecycle_row(&self) -> bool {
        self.relation_lifecycle
            .windows(2)
            .all(|rows| rows[0] != rows[1])
    }

    pub fn matches_sealed_attestation(&self, attestation: &Sys5SealedInventoryAttestation) -> bool {
        attestation.sealed_final
            && self.checked_program_identity == attestation.checked_program_identity
            && self.owner_rmw.len() == attestation.owner_rmw_count
            && self.relation_lifecycle.len() == attestation.relation_transition_count
            && self.designated_evaluators.len() == attestation.designated_evaluator_count
            && self.designated_remote_inputs.len() == attestation.designated_remote_input_count
            && self.named_consumers.len() == attestation.named_consumer_count
            && self.semantic_rows == attestation.semantic_rows
    }
}

impl Sys5RelationLifecycleInventoryRow {
    pub fn bootstrap_policy(&self) -> &str {
        &self.bootstrap_policy
    }

    pub const fn core_derived(&self) -> bool {
        self.core_derived
    }

    pub const fn grants_authority(&self) -> bool {
        self.grants_authority
    }

    /// The finite bootstrap accepts no caller supplied lifecycle handle.
    pub const fn accepts_raw_lease_or_ref(&self) -> bool {
        false
    }
}

/// Observer-safe opaque counts from the sealed M9/SYS-4 inventory.  The
/// digest is derived from the sealed canonical semantic-row sets; counts
/// describe only checked-source operation families and contain no credential,
/// membership, or witness refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5SealedInventoryAttestation {
    checked_program_identity: String,
    digest: String,
    owner_rmw_count: usize,
    relation_transition_count: usize,
    designated_evaluator_count: usize,
    designated_remote_input_count: usize,
    named_consumer_count: usize,
    sealed_final: bool,
    exact_row_set_match: bool,
    #[serde(skip)]
    semantic_rows: Sys5SemanticRowSets,
}

impl Sys5SealedInventoryAttestation {
    fn from_m9_summary(
        summary: &ObserverSafeM9Summary,
        sealed_rows: &ObserverSafeM9SemanticRowSets,
    ) -> Self {
        let semantic_rows = Sys5SemanticRowSets {
            owner_lineages: sealed_rows.owner_lineages().clone(),
            relation_transitions: sealed_rows.relation_transitions().clone(),
            designated_evaluators: sealed_rows.designated_evaluators().clone(),
            designated_remote_input_lineages: sealed_rows
                .designated_remote_input_lineages()
                .clone(),
            designated_consumers: sealed_rows.designated_consumers().clone(),
        };
        Self {
            checked_program_identity: checked_program_identity_ref(
                &summary.checked_program_identity().stable_key(),
            ),
            digest: sealed_semantic_rows_digest(&semantic_rows),
            owner_rmw_count: semantic_rows.owner_lineages.len(),
            relation_transition_count: semantic_rows.relation_transitions.len(),
            designated_evaluator_count: semantic_rows.designated_evaluators.len(),
            designated_remote_input_count: semantic_rows.designated_remote_input_lineages.len(),
            named_consumer_count: semantic_rows.designated_consumers.len(),
            sealed_final: summary.is_complete_final_m9_runtime_seam(),
            exact_row_set_match: false,
            semantic_rows,
        }
    }

    pub fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub const fn owner_rmw_count(&self) -> usize {
        self.owner_rmw_count
    }

    pub const fn relation_transition_count(&self) -> usize {
        self.relation_transition_count
    }

    pub const fn designated_evaluator_count(&self) -> usize {
        self.designated_evaluator_count
    }

    pub const fn designated_remote_input_count(&self) -> usize {
        self.designated_remote_input_count
    }

    pub const fn named_consumer_count(&self) -> usize {
        self.named_consumer_count
    }

    pub const fn is_final(&self) -> bool {
        self.sealed_final
    }

    pub const fn exact_row_set_match(&self) -> bool {
        self.exact_row_set_match
    }

    fn set_exact_row_set_match(&mut self, exact_row_set_match: bool) {
        self.exact_row_set_match = exact_row_set_match;
    }

    pub fn covers_source_inventory(&self, inventory: &Sys5AdmissionInventory) -> bool {
        self.exact_row_set_match && inventory.matches_sealed_attestation(self)
    }
}

fn sealed_semantic_rows_digest(rows: &Sys5SemanticRowSets) -> String {
    let mut hasher = Sha256::new();
    hasher.update(SEALED_INVENTORY_REF_DOMAIN);
    sealed_digest_row_set_header(&mut hasher, b"owner", rows.owner_lineages.len());
    for (operation, principal, origin, owner) in &rows.owner_lineages {
        sealed_digest_text_row(
            &mut hasher,
            b"owner",
            &[
                operation.as_str(),
                principal.as_str(),
                origin.as_str(),
                owner.as_str(),
            ],
        );
    }
    sealed_digest_row_set_header(
        &mut hasher,
        b"relation-transition",
        rows.relation_transitions.len(),
    );
    for (relation, transition) in &rows.relation_transitions {
        sealed_digest_text_row(
            &mut hasher,
            b"relation-transition",
            &[relation.as_str(), transition.as_str()],
        );
    }
    sealed_digest_row_set_header(
        &mut hasher,
        b"designated-evaluator",
        rows.designated_evaluators.len(),
    );
    for (value, evaluator) in &rows.designated_evaluators {
        sealed_digest_text_row(
            &mut hasher,
            b"designated-evaluator",
            &[value.as_str(), evaluator.as_str()],
        );
    }
    sealed_digest_row_set_header(
        &mut hasher,
        b"designated-remote-input",
        rows.designated_remote_input_lineages.len(),
    );
    for (source, evaluator, value, dependency_ordinal, frontier) in
        &rows.designated_remote_input_lineages
    {
        sealed_digest_remote_input_row(
            &mut hasher,
            source,
            evaluator,
            value,
            *dependency_ordinal,
            frontier,
        );
    }
    sealed_digest_row_set_header(
        &mut hasher,
        b"designated-consumer",
        rows.designated_consumers.len(),
    );
    for (value, consumer) in &rows.designated_consumers {
        sealed_digest_text_row(
            &mut hasher,
            b"designated-consumer",
            &[value.as_str(), consumer.as_str()],
        );
    }
    format!("sys5-sealed-inventory-sha256-v1:{:x}", hasher.finalize())
}

/// This encoding is internal equality material, not a public digest grammar.
/// Every row family, field count, field type, field length, and field value is
/// written explicitly so Rust's `Debug` rendering can never define identity.
fn sealed_digest_row_set_header(hasher: &mut Sha256, row_kind: &[u8], row_count: usize) {
    sealed_digest_bytes(hasher, b"row-set");
    sealed_digest_bytes(hasher, row_kind);
    sealed_digest_u64(
        hasher,
        u64::try_from(row_count).expect("finite local row count fits u64"),
    );
}

fn sealed_digest_text_row(hasher: &mut Sha256, row_kind: &[u8], fields: &[&str]) {
    sealed_digest_bytes(hasher, b"text-row");
    sealed_digest_bytes(hasher, row_kind);
    sealed_digest_u64(
        hasher,
        u64::try_from(fields.len()).expect("finite local field count fits u64"),
    );
    for field in fields {
        sealed_digest_text_field(hasher, field);
    }
}

fn sealed_digest_remote_input_row(
    hasher: &mut Sha256,
    source: &str,
    evaluator: &str,
    value: &str,
    dependency_ordinal: usize,
    frontier: &str,
) {
    sealed_digest_bytes(hasher, b"remote-input-row");
    sealed_digest_bytes(hasher, b"designated-remote-input");
    sealed_digest_u64(hasher, 5);
    sealed_digest_text_field(hasher, source);
    sealed_digest_text_field(hasher, evaluator);
    sealed_digest_text_field(hasher, value);
    sealed_digest_bytes(hasher, b"u64");
    sealed_digest_u64(
        hasher,
        u64::try_from(dependency_ordinal).expect("finite local ordinal fits u64"),
    );
    sealed_digest_text_field(hasher, frontier);
}

fn sealed_digest_text_field(hasher: &mut Sha256, value: &str) {
    sealed_digest_bytes(hasher, b"text");
    sealed_digest_bytes(hasher, value.as_bytes());
}

fn sealed_digest_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    sealed_digest_u64(
        hasher,
        u64::try_from(bytes.len()).expect("finite local field length fits u64"),
    );
    hasher.update(bytes);
}

fn sealed_digest_u64(hasher: &mut Sha256, value: u64) {
    hasher.update(value.to_le_bytes());
}

/// A compact observer-safe view of the sealed operation-family coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Sys5SealedInventoryCounts {
    owner_rmw: usize,
    relation_transitions: usize,
    designated_evaluators: usize,
    designated_remote_inputs: usize,
    named_consumers: usize,
}

impl Sys5SealedInventoryCounts {
    pub const fn owner_rmw(&self) -> usize {
        self.owner_rmw
    }

    pub const fn relation_transitions(&self) -> usize {
        self.relation_transitions
    }

    pub const fn designated_evaluators(&self) -> usize {
        self.designated_evaluators
    }

    pub const fn designated_remote_inputs(&self) -> usize {
        self.designated_remote_inputs
    }

    pub const fn named_consumers(&self) -> usize {
        self.named_consumers
    }
}

/// One observer-safe admission report.  It carries the complete source-level
/// inventory so consumers need not join authority, projection, and runtime
/// internals manually.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5AdmissionSummary {
    checked_program_identity: String,
    runtime_profile: Sys5LocalRuntimeProfile,
    source_derived: bool,
    derived_from_sealed_admission: bool,
    complete_for_projection: bool,
    public_api_or_wire_contract: bool,
    raw_input_rejection_profile: String,
    raw_input_rejection_is_runtime_evidence: bool,
    auth_discharges: Vec<Sys5AuthDischargeSummary>,
    verification_discharges: Vec<Sys5VerificationDischargeSummary>,
    inventory: Sys5AdmissionInventory,
    sealed_inventory_attestation: Sys5SealedInventoryAttestation,
}

impl Sys5AdmissionSummary {
    fn from_inventory(
        checked_program_identity: &str,
        runtime_profile: Sys5LocalRuntimeProfile,
        auth_residual_name: &str,
        verify_residual_name: &str,
        inventory: &Sys5AdmissionInventory,
        sealed_inventory_attestation: &Sys5SealedInventoryAttestation,
    ) -> Self {
        let derived_from_sealed_admission = sealed_inventory_attestation.is_final()
            && sealed_inventory_attestation.exact_row_set_match()
            && inventory.matches_sealed_attestation(sealed_inventory_attestation);
        Self {
            checked_program_identity: checked_program_identity.to_string(),
            runtime_profile,
            source_derived: derived_from_sealed_admission,
            derived_from_sealed_admission,
            complete_for_projection: derived_from_sealed_admission,
            public_api_or_wire_contract: false,
            raw_input_rejection_profile: "sys5-finite-admission-request-surface".to_string(),
            raw_input_rejection_is_runtime_evidence: false,
            auth_discharges: vec![Sys5AuthDischargeSummary {
                authority: auth_residual_name.to_string(),
                source_ref_present: true,
                m9_evidence_ref_present: true,
                discharged: true,
            }],
            verification_discharges: vec![Sys5VerificationDischargeSummary {
                verifier: verify_residual_name.to_string(),
                source_ref_present: true,
                finite_refinement_evidence_ref_present: true,
                discharged: true,
            }],
            inventory: inventory.clone(),
            sealed_inventory_attestation: sealed_inventory_attestation.clone(),
        }
    }

    pub fn checked_program_identity_ref(&self) -> &str {
        &self.checked_program_identity
    }

    pub const fn runtime_profile(&self) -> Sys5LocalRuntimeProfile {
        self.runtime_profile
    }

    pub const fn is_source_derived(&self) -> bool {
        self.source_derived
    }

    pub const fn derived_from_sealed_admission(&self) -> bool {
        self.derived_from_sealed_admission
    }

    pub const fn is_complete_for_projection(&self) -> bool {
        self.complete_for_projection
    }

    pub const fn public_api_or_wire_contract(&self) -> bool {
        self.public_api_or_wire_contract
    }

    pub fn raw_input_rejection_profile(&self) -> &str {
        &self.raw_input_rejection_profile
    }

    pub const fn raw_input_rejection_is_runtime_evidence(&self) -> bool {
        self.raw_input_rejection_is_runtime_evidence
    }

    pub fn sealed_inventory_digest(&self) -> &str {
        self.sealed_inventory_attestation.digest()
    }

    pub fn sealed_inventory_attestation_ref(&self) -> &str {
        self.sealed_inventory_attestation.digest()
    }

    pub const fn sealed_inventory_counts(&self) -> Sys5SealedInventoryCounts {
        Sys5SealedInventoryCounts {
            owner_rmw: self.sealed_inventory_attestation.owner_rmw_count,
            relation_transitions: self.sealed_inventory_attestation.relation_transition_count,
            designated_evaluators: self.sealed_inventory_attestation.designated_evaluator_count,
            designated_remote_inputs: self
                .sealed_inventory_attestation
                .designated_remote_input_count,
            named_consumers: self.sealed_inventory_attestation.named_consumer_count,
        }
    }

    pub fn auth_discharge(&self, authority: &str) -> Option<&Sys5AuthDischargeSummary> {
        self.auth_discharges
            .iter()
            .find(|discharge| discharge.authority == authority)
    }

    pub fn verification_discharge(
        &self,
        verifier: &str,
    ) -> Option<&Sys5VerificationDischargeSummary> {
        self.verification_discharges
            .iter()
            .find(|discharge| discharge.verifier == verifier)
    }
}

/// Observer-safe semantic facts derived from one checked Core and projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5SemanticSummary {
    pub profile_name: String,
    pub profile_status: String,
    pub public_api_or_wire_contract: bool,
    pub requires_runtime_execution: bool,
    pub loci: Vec<String>,
    pub artifacts: Vec<Sys5ArtifactSummary>,
    pub generated_communication: Vec<Sys5CommunicationSummary>,
    pub source_core_artifact_mappings: Vec<Sys5SourceCoreArtifactMapping>,
    pub auth_residuals: Vec<Sys5AuthResidual>,
    pub verification_residuals: Vec<Sys5VerificationResidual>,
    pub observer_safety: String,
}

/// A per-locus executable-artifact summary, derived from a projected fragment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5ArtifactSummary {
    pub locus: String,
    pub kind: String,
    pub operation_id: String,
    pub derived_from_checked_core: bool,
    pub source_path: String,
    pub source_span: Sys5SourceSpan,
    pub core_ref: String,
    pub fragment_ref: String,
    pub checked_program_identity: String,
}

/// A generated communication edge, derived from the checked Core projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5CommunicationSummary {
    pub kind: String,
    pub from_locus: String,
    pub to_locus: String,
    pub operation_id: String,
    pub derived_from_checked_core: bool,
    pub transfers_authority: bool,
    pub source_path: String,
    pub source_span: Sys5SourceSpan,
    pub core_ref: Option<String>,
    pub edge_ref: String,
    pub source_fragment_ref: String,
    pub target_fragment_ref: String,
    pub checked_program_identity: String,
}

/// A source-to-Core-to-artifact provenance row without source text or secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5SourceCoreArtifactMapping {
    /// Deterministic logical source label.  It is duplicated from
    /// `source_path` for the provisional JSON viewer, whose consumers must
    /// never infer a host filesystem path from provenance.
    pub logical_path: String,
    pub source_path: String,
    pub source_span: Sys5SourceSpan,
    pub operation_id: String,
    pub core_kind: String,
    pub core_ref: String,
    pub artifact_locus: String,
    pub artifact_kind: String,
    pub fragment_ref: String,
    /// Alias retained for the per-locus executable program reference expected
    /// by the local devtools projection.  It denotes the exact same checked
    /// fragment as `fragment_ref`, rather than a manually authored route.
    pub locus_program_ref: String,
    pub checked_program_identity: String,
}

/// A source position with no source text.  The logical source path remains in
/// the containing summary row so a viewer cannot recover host paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Sys5SourceSpan {
    /// Stable, nonzero ordering coordinates for JSON consumers.  The source
    /// remains represented only by the logical path and coordinates below.
    pub start: u64,
    pub end: u64,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

/// An explicit, non-admitting auth residual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5AuthResidual {
    pub authority: String,
    pub status: String,
    pub grants_runtime_authority: bool,
}

/// An explicit, optional verification residual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5VerificationResidual {
    pub verifier: String,
    pub status: String,
    pub discharge: String,
}

/// Serializable causal lookup fragments.  They intentionally contain only
/// checked source/Core/artifact/edge identifiers and residual status names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sys5ObserverSafeView {
    pub semantic_fragments: Vec<String>,
}

/// Checks and projects one ordinary source without executing a runtime.
///
/// The logical topology is exactly the locus inventory retained in the checked
/// static environment; callers cannot add routes or hand-author interfaces.
pub fn build_project(input: Sys5SourceInput) -> Result<Sys5LocalProject, Sys5LocalSliceError> {
    let logical_source_path = normalize_logical_source_path(&input.logical_source_path)?;
    let checked = check_and_elaborate_surface_v0(FixtureSource::new(
        logical_source_path.clone(),
        input.source_text,
    ))
    .map_err(|diagnostics| Sys5LocalSliceError::SurfaceCheckFailed {
        diagnostic_code: diagnostics.primary().canonical_code(),
    })?;

    let topology = DeclaredLogicalTopology::try_new(
        checked.program_identity().clone(),
        checked
            .static_environment()
            .loci()
            .iter()
            .map(|locus| locus.name().to_string()),
    )
    .map_err(|_| Sys5LocalSliceError::ProjectionFailed)?;
    let projection = project_checked_core(&checked, &topology)
        .map_err(|_| Sys5LocalSliceError::ProjectionFailed)?;

    let mut artifacts = Vec::new();
    let mut source_core_artifact_mappings = Vec::new();
    for locus in projection.locus_order() {
        let program = projection
            .locus_program(locus)
            .expect("projection retains every declared locus");
        for fragment in program.operation_fragments() {
            let artifact_kind = fragment_kind_name(fragment.fragment_kind()).to_string();
            let source_ref = fragment.source_ref();
            let source_path = source_ref.path.clone();
            let source_span = summary_source_span(source_ref);
            let core_ref = fragment
                .core_ref()
                .expect("every SYS-3 projected fragment has checked Core provenance")
                .to_string();
            let fragment_ref = fragment.fragment_ref().to_string();
            let checked_program_identity = fragment
                .checked_core_identity()
                .checked_program_identity()
                .stable_key();
            let checked_program_identity = checked_program_identity_ref(&checked_program_identity);
            artifacts.push(Sys5ArtifactSummary {
                locus: locus.to_string(),
                kind: artifact_kind.clone(),
                operation_id: fragment.operation_id().to_string(),
                derived_from_checked_core: true,
                source_path: source_path.clone(),
                source_span,
                core_ref: core_ref.clone(),
                fragment_ref: fragment_ref.clone(),
                checked_program_identity: checked_program_identity.clone(),
            });
            source_core_artifact_mappings.push(Sys5SourceCoreArtifactMapping {
                logical_path: source_path.clone(),
                source_path,
                source_span,
                operation_id: fragment.operation_id().to_string(),
                core_kind: core_kind_name(fragment.fragment_kind()).to_string(),
                core_ref,
                artifact_locus: locus.to_string(),
                artifact_kind,
                locus_program_ref: fragment_ref.clone(),
                fragment_ref,
                checked_program_identity,
            });
        }
    }

    let generated_communication = projection
        .communication_plan()
        .edges()
        .iter()
        .map(|edge| {
            let source_ref = edge.source_ref();
            let checked_program_identity = edge
                .checked_core_identity()
                .checked_program_identity()
                .stable_key();
            Sys5CommunicationSummary {
                kind: edge_kind_name(edge.kind()).to_string(),
                from_locus: edge.source_locus().to_string(),
                to_locus: edge.target_locus().to_string(),
                operation_id: edge.operation_id().to_string(),
                derived_from_checked_core: edge.is_derived_from_checked_core(),
                transfers_authority: edge.transfers_authority(),
                source_path: source_ref.path.clone(),
                source_span: summary_source_span(&source_ref),
                core_ref: edge.core_ref().map(str::to_string),
                edge_ref: edge.edge_ref().to_string(),
                source_fragment_ref: edge.source_fragment_ref().clone(),
                target_fragment_ref: edge.target_fragment_ref().clone(),
                checked_program_identity: checked_program_identity_ref(&checked_program_identity),
            }
        })
        .collect::<Vec<_>>();

    let auth_residuals = checked
        .residual_obligations()
        .entries()
        .iter()
        .filter(|residual| residual.kind() == ResidualObligationKind::AuthDeferred)
        .map(|residual| Sys5AuthResidual {
            authority: residual.name().to_string(),
            status: "residual".to_string(),
            grants_runtime_authority: residual.grants_authority(),
        })
        .collect::<Vec<_>>();
    let verification_residuals = checked
        .residual_obligations()
        .entries()
        .iter()
        .filter(|residual| residual.kind() == ResidualObligationKind::VerifyDeferred)
        .map(|residual| Sys5VerificationResidual {
            verifier: residual.name().to_string(),
            status: "residual".to_string(),
            discharge: "optional".to_string(),
        })
        .collect::<Vec<_>>();

    artifacts.sort_by(|left, right| {
        (&left.locus, &left.kind, &left.operation_id).cmp(&(
            &right.locus,
            &right.kind,
            &right.operation_id,
        ))
    });
    source_core_artifact_mappings.sort_by(|left, right| {
        (
            &left.source_path,
            &left.operation_id,
            &left.core_kind,
            &left.artifact_locus,
            &left.artifact_kind,
        )
            .cmp(&(
                &right.source_path,
                &right.operation_id,
                &right.core_kind,
                &right.artifact_locus,
                &right.artifact_kind,
            ))
    });

    let summary = Sys5SemanticSummary {
        profile_name: PROFILE_NAME.to_string(),
        profile_status: PROFILE_STATUS.to_string(),
        public_api_or_wire_contract: false,
        requires_runtime_execution: false,
        loci: projection
            .locus_order()
            .into_iter()
            .map(str::to_string)
            .collect(),
        artifacts,
        generated_communication,
        source_core_artifact_mappings,
        auth_residuals,
        verification_residuals,
        observer_safety: OBSERVER_SAFETY.to_string(),
    };
    let observer_safe_view = observer_safe_view(&summary);

    Ok(Sys5LocalProject {
        checked,
        topology,
        projection,
        semantic_summary: summary,
        observer_safe_view,
    })
}

fn normalize_logical_source_path(path: &str) -> Result<String, Sys5LocalSliceError> {
    if !is_allowed_logical_source_path(path) {
        return Err(Sys5LocalSliceError::InvalidLogicalSourcePath);
    }
    Ok(path.to_string())
}

/// Logical source paths appear as unescaped values in compact typed observer
/// segments.  Admit only a deliberately small ASCII filename alphabet so
/// separators, control characters, whitespace, host paths, and Unicode
/// confusables cannot alter a later `key=value;...` row.
fn is_allowed_logical_source_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && path
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-'))
        && path
            .split('/')
            .all(|component| !component.is_empty() && !matches!(component, "." | ".."))
}

/// Returns a domain-separated SHA-256 reference for the exact checked-program
/// identity.  Only the fixed lower-case hexadecimal digest is serialized; the
/// stable key remains an internal authority identity, not observer output.
fn checked_program_identity_ref(stable_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(CHECKED_PROGRAM_REF_DOMAIN);
    hasher.update(
        u64::try_from(stable_key.len())
            .expect("logical source input length fits u64")
            .to_le_bytes(),
    );
    hasher.update(stable_key.as_bytes());
    format!("sys5-checked-program-sha256-v1:{:x}", hasher.finalize())
}

fn debug_path_ref(logical_source_path: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(DEBUG_PATH_REF_DOMAIN);
    hasher.update(
        u64::try_from(logical_source_path.len())
            .expect("logical path length fits u64")
            .to_le_bytes(),
    );
    hasher.update(logical_source_path.as_bytes());
    format!("sys5-debug-path-sha256-v1:{:x}", hasher.finalize())
}

fn relation_observer_ref(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(RELATION_OBSERVER_REF_DOMAIN);
    hasher.update(
        u64::try_from(value.len())
            .expect("relation observer reference input fits u64")
            .to_le_bytes(),
    );
    hasher.update(value.as_bytes());
    format!("sys5-relation-sha256-v1:{:x}", hasher.finalize())
}

fn local_cut_ref(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(LOCAL_CUT_REF_DOMAIN);
    hasher.update(
        u64::try_from(value.len())
            .expect("local cut reference input fits u64")
            .to_le_bytes(),
    );
    hasher.update(value.as_bytes());
    format!("sys5-local-cut-sha256-v1:{:x}", hasher.finalize())
}

fn patch_frontier_ref(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(PATCH_FRONTIER_REF_DOMAIN);
    hasher.update(
        u64::try_from(value.len())
            .expect("patch frontier reference input fits u64")
            .to_le_bytes(),
    );
    hasher.update(value.as_bytes());
    format!("sys5-patch-frontier-sha256-v1:{:x}", hasher.finalize())
}

/// Allocate one observer lifecycle occurrence from the integrity-bound local
/// cursor.  It is part of the ST cut lineage rather than a process-global
/// scheduler identity, and carries no source text, state, M9 authority,
/// capability, or witness material.
fn next_lifecycle_occurrence_ref(
    next_local_occurrence: &mut u64,
    kind: &str,
    cut_id_ref: &str,
    sys4_cut_integrity_ref: &str,
) -> Result<String, Sys5LocalCutPatchError> {
    let local_occurrence = *next_local_occurrence;
    let next = local_occurrence.checked_add(1).ok_or_else(|| {
        Sys5LocalCutPatchError::new(Sys5LocalCutPatchErrorKind::LifecycleOccurrenceExhausted)
    })?;
    *next_local_occurrence = next;

    let mut hasher = Sha256::new();
    hasher.update(LIFECYCLE_OCCURRENCE_REF_DOMAIN);
    for component in [kind, cut_id_ref, sys4_cut_integrity_ref] {
        hasher.update(
            u64::try_from(component.len())
                .expect("lifecycle occurrence reference input length fits u64")
                .to_le_bytes(),
        );
        hasher.update(component.as_bytes());
    }
    hasher.update(local_occurrence.to_le_bytes());
    Ok(format!(
        "sys5-lifecycle-occurrence:{local_occurrence:020}:{:x}",
        hasher.finalize()
    ))
}

fn observer_source_ref(source_ref: &SourceRef) -> String {
    format!(
        "{}:{}:{}-{}:{}",
        source_ref.path,
        source_ref.start_line,
        source_ref.start_column,
        source_ref.end_line,
        source_ref.end_column,
    )
}

fn summary_source_span(source_ref: &SourceRef) -> Sys5SourceSpan {
    let start = u64::from(source_ref.start_line) * 1_000_000 + u64::from(source_ref.start_column);
    let end = u64::from(source_ref.end_line) * 1_000_000 + u64::from(source_ref.end_column);
    Sys5SourceSpan {
        start,
        end,
        start_line: source_ref.start_line,
        start_column: source_ref.start_column,
        end_line: source_ref.end_line,
        end_column: source_ref.end_column,
    }
}

fn fragment_kind_name(kind: ProjectedOperationFragmentKind) -> &'static str {
    match kind {
        ProjectedOperationFragmentKind::OwnerRequestInvocation => "owner-request-invocation",
        ProjectedOperationFragmentKind::OwnerRmwExecution => "owner-rmw-evaluation",
        ProjectedOperationFragmentKind::RelationPublication => "relation-publication",
        ProjectedOperationFragmentKind::ConsumerLocalRelationProjection => {
            "consumer-local-relation-projection"
        }
        ProjectedOperationFragmentKind::DesignatedRemoteInputService => {
            "designated-remote-input-service"
        }
        ProjectedOperationFragmentKind::DesignatedEvaluation => "designated-evaluation",
        ProjectedOperationFragmentKind::DesignatedResultConsumer => "designated-result-consumer",
    }
}

fn core_kind_name(kind: ProjectedOperationFragmentKind) -> &'static str {
    match kind {
        ProjectedOperationFragmentKind::OwnerRequestInvocation
        | ProjectedOperationFragmentKind::OwnerRmwExecution => "OwnerRmw",
        ProjectedOperationFragmentKind::RelationPublication
        | ProjectedOperationFragmentKind::ConsumerLocalRelationProjection => "MaintainedRelation",
        ProjectedOperationFragmentKind::DesignatedRemoteInputService
        | ProjectedOperationFragmentKind::DesignatedEvaluation => "DesignatedPublishValue",
        ProjectedOperationFragmentKind::DesignatedResultConsumer => "DesignatedResultConsume",
    }
}

fn edge_kind_name(kind: CommunicationEdgeKind) -> &'static str {
    match kind {
        CommunicationEdgeKind::OwnerRequest => "owner-request",
        CommunicationEdgeKind::OwnerReplyReceipt => "owner-reply-receipt",
        CommunicationEdgeKind::RelationProjectionPublication => "relation-projection-publication",
        CommunicationEdgeKind::DesignatedInputRequest => "designated-input-request",
        CommunicationEdgeKind::DesignatedInputReceipt => "designated-input-receipt",
        CommunicationEdgeKind::DesignatedResultDelivery => "designated-result-delivery",
        CommunicationEdgeKind::AbsoluteValueStream => "absolute-value-stream",
    }
}

fn carrier_lifecycle_kind_name(kind: CarrierLifecycleKind) -> &'static str {
    match kind {
        CarrierLifecycleKind::OwnerRequest => "owner-request",
        CarrierLifecycleKind::OwnerReplyReceipt => "owner-reply-receipt",
        CarrierLifecycleKind::DesignatedInputRequest => "designated-input-request",
        CarrierLifecycleKind::DesignatedInputReceipt => "designated-input-receipt",
        CarrierLifecycleKind::RelationProjectionPublication => "relation-projection-publication",
        CarrierLifecycleKind::DesignatedResultDelivery => "designated-result-delivery",
    }
}

fn carrier_occurrence_slot_name(kind: CarrierOccurrenceSlotKind) -> &'static str {
    match kind {
        CarrierOccurrenceSlotKind::Request => "Request",
        CarrierOccurrenceSlotKind::Serve => "Serve",
        CarrierOccurrenceSlotKind::Reply => "Reply",
        CarrierOccurrenceSlotKind::Receive => "Receive",
        CarrierOccurrenceSlotKind::Publish => "Publish",
        CarrierOccurrenceSlotKind::Observe => "Observe",
        CarrierOccurrenceSlotKind::Consume => "Consume",
    }
}

fn carrier_frontier_kind_name(kind: CarrierFrontierKind) -> &'static str {
    match kind {
        CarrierFrontierKind::Input => "Input",
        CarrierFrontierKind::Result => "Result",
    }
}

fn i3_probe_owner_principal_ref(origin_principal_template: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_PROBE_OWNER_PRINCIPAL_REF_DOMAIN);
    i3_probe_digest_field(
        &mut hasher,
        b"origin-principal-template",
        origin_principal_template.as_bytes(),
    );
    format!(
        "sys5-i3-probe-owner-principal-sha256-v1:{:x}",
        hasher.finalize()
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum I3AdapterCarrierFamily {
    OwnerRequest,
    OwnerReplyReceipt,
    DesignatedInputRequest,
    DesignatedInputReceipt,
    RelationProjectionPublication,
    DesignatedResultDelivery,
}

fn i3_adapter_carrier_family_for_edge_kind(
    kind: CommunicationEdgeKind,
) -> Result<I3AdapterCarrierFamily, Sys5I3ProbeFacadeError> {
    match kind {
        CommunicationEdgeKind::OwnerRequest => Ok(I3AdapterCarrierFamily::OwnerRequest),
        CommunicationEdgeKind::OwnerReplyReceipt => Ok(I3AdapterCarrierFamily::OwnerReplyReceipt),
        CommunicationEdgeKind::DesignatedInputRequest => {
            Ok(I3AdapterCarrierFamily::DesignatedInputRequest)
        }
        CommunicationEdgeKind::DesignatedInputReceipt => {
            Ok(I3AdapterCarrierFamily::DesignatedInputReceipt)
        }
        CommunicationEdgeKind::RelationProjectionPublication => {
            Ok(I3AdapterCarrierFamily::RelationProjectionPublication)
        }
        CommunicationEdgeKind::DesignatedResultDelivery => {
            Ok(I3AdapterCarrierFamily::DesignatedResultDelivery)
        }
        CommunicationEdgeKind::AbsoluteValueStream => Err(Sys5I3ProbeFacadeError::new(
            Sys5I3ProbeFacadeErrorKind::NotAcceptedCarrierFamily,
        )),
    }
}

fn i3_adapter_expected_lifecycle_kind(family: I3AdapterCarrierFamily) -> CarrierLifecycleKind {
    match family {
        I3AdapterCarrierFamily::OwnerRequest => CarrierLifecycleKind::OwnerRequest,
        I3AdapterCarrierFamily::OwnerReplyReceipt => CarrierLifecycleKind::OwnerReplyReceipt,
        I3AdapterCarrierFamily::DesignatedInputRequest => {
            CarrierLifecycleKind::DesignatedInputRequest
        }
        I3AdapterCarrierFamily::DesignatedInputReceipt => {
            CarrierLifecycleKind::DesignatedInputReceipt
        }
        I3AdapterCarrierFamily::RelationProjectionPublication => {
            CarrierLifecycleKind::RelationProjectionPublication
        }
        I3AdapterCarrierFamily::DesignatedResultDelivery => {
            CarrierLifecycleKind::DesignatedResultDelivery
        }
    }
}

struct I3AdapterVariantProjectionInput<'a> {
    family: I3AdapterCarrierFamily,
    variant: I3AdapterCarrierStaticVariant,
    dependency_ordinal: Option<usize>,
    frontiers: &'a BTreeSet<CarrierFrontierKind>,
    origin_locus_template: Option<&'a str>,
    target_owner_locus_template: Option<&'a str>,
    edge_source_locus: &'a str,
    edge_target_locus: &'a str,
    operation_id: &'a str,
}

fn i3_adapter_variant_facts(
    input: I3AdapterVariantProjectionInput<'_>,
) -> Result<Sys5I3AdapterCarrierVariantFacts, Sys5I3ProbeFacadeError> {
    let I3AdapterVariantProjectionInput {
        family,
        variant,
        dependency_ordinal,
        frontiers,
        origin_locus_template,
        target_owner_locus_template,
        edge_source_locus,
        edge_target_locus,
        operation_id,
    } = input;
    let mismatch =
        || Sys5I3ProbeFacadeError::new(Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch);
    match family {
        I3AdapterCarrierFamily::OwnerRequest => {
            let I3AdapterCarrierStaticVariant::OwnerRequest {
                origin_principal_template,
            } = variant
            else {
                return Err(mismatch());
            };
            let (Some(origin_locus_template), Some(target_owner_locus_template)) =
                (origin_locus_template, target_owner_locus_template)
            else {
                return Err(mismatch());
            };
            if !frontiers.is_empty()
                || origin_locus_template != edge_source_locus
                || target_owner_locus_template != edge_target_locus
            {
                return Err(mismatch());
            }
            Ok(Sys5I3AdapterCarrierVariantFacts::OwnerRequest(
                Sys5I3AdapterOwnerFacts {
                    origin_principal_ref: i3_adapter_owner_principal_ref(
                        &origin_principal_template,
                    ),
                    origin_locus_template: origin_locus_template.to_string(),
                    target_owner_locus_template: target_owner_locus_template.to_string(),
                },
            ))
        }
        I3AdapterCarrierFamily::OwnerReplyReceipt => {
            let I3AdapterCarrierStaticVariant::OwnerReplyReceipt {
                origin_principal_template,
            } = variant
            else {
                return Err(mismatch());
            };
            let (Some(origin_locus_template), Some(target_owner_locus_template)) =
                (origin_locus_template, target_owner_locus_template)
            else {
                return Err(mismatch());
            };
            if !frontiers.is_empty()
                || origin_locus_template != edge_target_locus
                || target_owner_locus_template != edge_source_locus
            {
                return Err(mismatch());
            }
            Ok(Sys5I3AdapterCarrierVariantFacts::OwnerReplyReceipt(
                Sys5I3AdapterOwnerFacts {
                    origin_principal_ref: i3_adapter_owner_principal_ref(
                        &origin_principal_template,
                    ),
                    origin_locus_template: origin_locus_template.to_string(),
                    target_owner_locus_template: target_owner_locus_template.to_string(),
                },
            ))
        }
        I3AdapterCarrierFamily::DesignatedInputRequest => {
            let I3AdapterCarrierStaticVariant::DesignatedInputRequest { dependency } = variant
            else {
                return Err(mismatch());
            };
            if frontiers != &BTreeSet::from([CarrierFrontierKind::Input])
                || origin_locus_template != Some(edge_source_locus)
                || target_owner_locus_template != Some(edge_target_locus)
            {
                return Err(mismatch());
            }
            let ordinal = dependency_ordinal.ok_or_else(mismatch)?;
            if dependency.designated_evaluator() != edge_source_locus
                || dependency.source_owner_locus() != edge_target_locus
            {
                return Err(mismatch());
            }
            Ok(Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(
                i3_adapter_designated_input_facts(&dependency, ordinal, frontiers)?,
            ))
        }
        I3AdapterCarrierFamily::DesignatedInputReceipt => {
            let I3AdapterCarrierStaticVariant::DesignatedInputReceipt { dependency } = variant
            else {
                return Err(mismatch());
            };
            if frontiers != &BTreeSet::from([CarrierFrontierKind::Result])
                || origin_locus_template != Some(edge_target_locus)
                || target_owner_locus_template != Some(edge_source_locus)
            {
                return Err(mismatch());
            }
            let ordinal = dependency_ordinal.ok_or_else(mismatch)?;
            if dependency.designated_evaluator() != edge_target_locus
                || dependency.source_owner_locus() != edge_source_locus
            {
                return Err(mismatch());
            }
            Ok(Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(
                i3_adapter_designated_input_facts(&dependency, ordinal, frontiers)?,
            ))
        }
        I3AdapterCarrierFamily::RelationProjectionPublication => {
            let I3AdapterCarrierStaticVariant::RelationProjectionPublication = variant else {
                return Err(mismatch());
            };
            if !frontiers.is_empty()
                || origin_locus_template != Some(edge_source_locus)
                || target_owner_locus_template != Some(edge_target_locus)
            {
                return Err(mismatch());
            }
            Ok(
                Sys5I3AdapterCarrierVariantFacts::RelationProjectionPublication(
                    Sys5I3AdapterRelationPublicationFacts {
                        relation_name: operation_id.to_string(),
                        publication_locus: edge_source_locus.to_string(),
                        consumer_locus: edge_target_locus.to_string(),
                    },
                ),
            )
        }
        I3AdapterCarrierFamily::DesignatedResultDelivery => {
            let I3AdapterCarrierStaticVariant::DesignatedResultDelivery {
                result_version,
                input_frontier,
                result_frontier,
                observation_policy,
                policy_stamp,
                retry_contract,
            } = variant
            else {
                return Err(mismatch());
            };
            if frontiers
                != &BTreeSet::from([CarrierFrontierKind::Input, CarrierFrontierKind::Result])
                || origin_locus_template != Some(edge_source_locus)
                || target_owner_locus_template != Some(edge_target_locus)
            {
                return Err(mismatch());
            }
            Ok(Sys5I3AdapterCarrierVariantFacts::DesignatedResultDelivery(
                Sys5I3AdapterDesignatedResultFacts {
                    evaluator_locus: edge_source_locus.to_string(),
                    consumer_locus: edge_target_locus.to_string(),
                    result_version_ref: i3_adapter_result_version_ref(result_version),
                    input_frontier_ref: i3_adapter_input_frontier_ref(&input_frontier),
                    result_frontier_ref: i3_adapter_result_frontier_ref(&result_frontier),
                    observation_policy_ref: i3_adapter_observation_policy_ref(&observation_policy),
                    policy_stamp_ref: i3_adapter_policy_stamp_ref(&policy_stamp),
                    static_retry_contract_name: i3_adapter_static_retry_contract_name(
                        retry_contract,
                    )
                    .to_string(),
                },
            ))
        }
    }
}

fn i3_adapter_designated_input_facts(
    dependency: &StaticProjectionFacts,
    dependency_ordinal: usize,
    frontiers: &BTreeSet<CarrierFrontierKind>,
) -> Result<Sys5I3AdapterDesignatedInputFacts, Sys5I3ProbeFacadeError> {
    if !i3_adapter_validates_designated_dependency(dependency) {
        return Err(Sys5I3ProbeFacadeError::new(
            Sys5I3ProbeFacadeErrorKind::CarrierContractMismatch,
        ));
    }
    Ok(Sys5I3AdapterDesignatedInputFacts {
        dependency_ordinal,
        typed_state_read_ref: i3_adapter_typed_state_read_ref(dependency.typed_state_read()),
        requester_site_ref: i3_adapter_requester_site_ref(dependency.requester_site()),
        authority_origin_ref: i3_adapter_authority_origin_ref(dependency.authority_origin()),
        request_ref: i3_adapter_designated_request_ref(dependency.request()),
        receipt_use_ref: i3_adapter_designated_receipt_use_ref(dependency.receipt_use()),
        designated_evaluator_locus: dependency.designated_evaluator().to_string(),
        source_owner_locus: dependency.source_owner_locus().to_string(),
        frontier_requirement_names: i3_adapter_frontier_names(frontiers),
    })
}

fn i3_adapter_validates_designated_dependency(dependency: &StaticProjectionFacts) -> bool {
    if dependency.request().source_owner_locus() != dependency.source_owner_locus()
        || dependency.receipt_use().source_owner_locus() != dependency.source_owner_locus()
        || dependency.request().typed_state_read() != dependency.typed_state_read()
        || dependency.receipt_use().typed_state_read() != dependency.typed_state_read()
    {
        return false;
    }
    i3_adapter_requester_site_matches_evaluator(
        dependency.requester_site(),
        dependency.designated_evaluator(),
    ) && i3_adapter_authority_origin_matches_evaluator(
        dependency.authority_origin(),
        dependency.designated_evaluator(),
    )
}

fn i3_adapter_requester_site_matches_evaluator(
    site: &mir_semantics::evaluation_materialization::EvaluationSite,
    designated_evaluator: &str,
) -> bool {
    match site {
        mir_semantics::evaluation_materialization::EvaluationSite::Owner(_) => false,
        mir_semantics::evaluation_materialization::EvaluationSite::Locus(_) => false,
        mir_semantics::evaluation_materialization::EvaluationSite::DesignatedEvaluator(locus) => {
            locus.as_str() == designated_evaluator
        }
        mir_semantics::evaluation_materialization::EvaluationSite::Consumer(_) => false,
        mir_semantics::evaluation_materialization::EvaluationSite::Provider(_) => false,
    }
}

fn i3_adapter_authority_origin_matches_evaluator(
    origin: &mir_semantics::evaluation_materialization::AuthorityOrigin,
    designated_evaluator: &str,
) -> bool {
    match origin {
        mir_semantics::evaluation_materialization::AuthorityOrigin::Caller(_) => false,
        mir_semantics::evaluation_materialization::AuthorityOrigin::OwnerTransition(_) => false,
        mir_semantics::evaluation_materialization::AuthorityOrigin::AdmittedEvaluator(locus) => {
            locus.as_str() == designated_evaluator
        }
        mir_semantics::evaluation_materialization::AuthorityOrigin::AdmittedProvider(_) => false,
    }
}

fn i3_adapter_frontier_names(frontiers: &BTreeSet<CarrierFrontierKind>) -> Vec<String> {
    frontiers
        .iter()
        .copied()
        .map(carrier_frontier_kind_name)
        .map(str::to_string)
        .collect()
}

fn i3_adapter_owner_principal_ref(origin_principal_template: &str) -> String {
    i3_adapter_opaque_reference(
        I3_ADAPTER_OWNER_PRINCIPAL_REF_DOMAIN,
        "sys5-i3-adapter-owner-principal-sha256-v1:",
        [(
            "origin-principal-template",
            origin_principal_template.as_bytes(),
        )],
    )
}

fn i3_adapter_typed_state_read_ref(read: &StaticProjectionTypedStateReadFacts) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_ADAPTER_DESIGNATED_READ_REF_DOMAIN);
    i3_adapter_digest_typed_state_read(&mut hasher, read);
    format!(
        "sys5-i3-adapter-designated-read-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn i3_adapter_requester_site_ref(
    site: &mir_semantics::evaluation_materialization::EvaluationSite,
) -> String {
    let (kind, value) = match site {
        mir_semantics::evaluation_materialization::EvaluationSite::Owner(locus) => {
            ("Owner", locus.as_str())
        }
        mir_semantics::evaluation_materialization::EvaluationSite::Locus(locus) => {
            ("Locus", locus.as_str())
        }
        mir_semantics::evaluation_materialization::EvaluationSite::DesignatedEvaluator(locus) => {
            ("DesignatedEvaluator", locus.as_str())
        }
        mir_semantics::evaluation_materialization::EvaluationSite::Consumer(principal) => {
            ("Consumer", principal.as_str())
        }
        mir_semantics::evaluation_materialization::EvaluationSite::Provider(provider) => {
            ("Provider", provider.as_str())
        }
    };
    i3_adapter_opaque_reference(
        I3_ADAPTER_REQUESTER_SITE_REF_DOMAIN,
        "sys5-i3-adapter-designated-requester-site-sha256-v1:",
        [
            ("site-kind", kind.as_bytes()),
            ("site-value", value.as_bytes()),
        ],
    )
}

fn i3_adapter_authority_origin_ref(
    origin: &mir_semantics::evaluation_materialization::AuthorityOrigin,
) -> String {
    let (kind, value) = match origin {
        mir_semantics::evaluation_materialization::AuthorityOrigin::Caller(principal) => {
            ("Caller", principal.as_str())
        }
        mir_semantics::evaluation_materialization::AuthorityOrigin::OwnerTransition(locus) => {
            ("OwnerTransition", locus.as_str())
        }
        mir_semantics::evaluation_materialization::AuthorityOrigin::AdmittedEvaluator(locus) => {
            ("AdmittedEvaluator", locus.as_str())
        }
        mir_semantics::evaluation_materialization::AuthorityOrigin::AdmittedProvider(provider) => {
            ("AdmittedProvider", provider.as_str())
        }
    };
    i3_adapter_opaque_reference(
        I3_ADAPTER_AUTHORITY_ORIGIN_REF_DOMAIN,
        "sys5-i3-adapter-designated-authority-origin-sha256-v1:",
        [
            ("authority-origin-kind", kind.as_bytes()),
            ("authority-origin-value", value.as_bytes()),
        ],
    )
}

fn i3_adapter_designated_request_ref(
    request: &StaticProjectionDesignatedInputRequestFacts,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_ADAPTER_DESIGNATED_REQUEST_REF_DOMAIN);
    i3_adapter_digest_text(
        &mut hasher,
        b"request-source-owner-locus",
        request.source_owner_locus(),
    );
    i3_adapter_digest_typed_state_read(&mut hasher, request.typed_state_read());
    format!(
        "sys5-i3-adapter-designated-request-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn i3_adapter_designated_receipt_use_ref(
    receipt_use: &StaticProjectionDesignatedInputReceiptUseFacts,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_ADAPTER_DESIGNATED_RECEIPT_USE_REF_DOMAIN);
    i3_adapter_digest_text(
        &mut hasher,
        b"receipt-use-source-owner-locus",
        receipt_use.source_owner_locus(),
    );
    i3_adapter_digest_typed_state_read(&mut hasher, receipt_use.typed_state_read());
    format!(
        "sys5-i3-adapter-designated-receipt-use-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn i3_adapter_digest_typed_state_read(
    hasher: &mut Sha256,
    read: &StaticProjectionTypedStateReadFacts,
) {
    i3_adapter_digest_text(hasher, b"typed-state-read-namespace", read.namespace());
    match read.index() {
        Some(index) => {
            i3_adapter_digest_field(hasher, b"typed-state-read-index-present", &[1]);
            i3_adapter_digest_text(hasher, b"typed-state-read-index", index);
        }
        None => i3_adapter_digest_field(hasher, b"typed-state-read-index-present", &[0]),
    }
    match read.field() {
        Some(field) => {
            i3_adapter_digest_field(hasher, b"typed-state-read-field-present", &[1]);
            i3_adapter_digest_text(hasher, b"typed-state-read-field", field);
        }
        None => i3_adapter_digest_field(hasher, b"typed-state-read-field-present", &[0]),
    }
    i3_adapter_digest_text(hasher, b"typed-state-read-owner-locus", read.owner_locus());
    i3_adapter_digest_text(hasher, b"typed-state-read-value-type", read.value_type());
    let source_ref = read.source_ref();
    i3_adapter_digest_text(hasher, b"typed-state-read-source-path", &source_ref.path);
    i3_adapter_digest_u64(
        hasher,
        b"typed-state-read-source-start-line",
        u64::from(source_ref.start_line),
    );
    i3_adapter_digest_u64(
        hasher,
        b"typed-state-read-source-start-column",
        u64::from(source_ref.start_column),
    );
    i3_adapter_digest_u64(
        hasher,
        b"typed-state-read-source-end-line",
        u64::from(source_ref.end_line),
    );
    i3_adapter_digest_u64(
        hasher,
        b"typed-state-read-source-end-column",
        u64::from(source_ref.end_column),
    );
}

fn i3_adapter_result_version_ref(
    result_version: mir_semantics::shared_model::ResultVersion,
) -> String {
    i3_adapter_opaque_reference(
        I3_ADAPTER_RESULT_VERSION_REF_DOMAIN,
        "sys5-i3-adapter-result-version-sha256-v1:",
        [("result-version", &result_version.value().to_be_bytes())],
    )
}

fn i3_adapter_input_frontier_ref(
    input_frontier: &mir_semantics::evaluation_materialization::InputFrontier,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_ADAPTER_INPUT_FRONTIER_REF_DOMAIN);
    i3_adapter_digest_u64(
        &mut hasher,
        b"producer-count",
        u64::try_from(input_frontier.as_slice().len())
            .expect("finite I3 adapter input frontier count fits u64"),
    );
    for occurrence in input_frontier.as_slice() {
        i3_adapter_digest_text(&mut hasher, b"producer", occurrence.as_str());
    }
    format!(
        "sys5-i3-adapter-input-frontier-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn i3_adapter_result_frontier_ref(
    result_frontier: &mir_semantics::shared_model::ResultFrontier,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_ADAPTER_RESULT_FRONTIER_REF_DOMAIN);
    i3_adapter_digest_u64(
        &mut hasher,
        b"result-count",
        u64::try_from(result_frontier.as_slice().len())
            .expect("finite I3 adapter result frontier count fits u64"),
    );
    for result in result_frontier.as_slice() {
        i3_adapter_digest_text(&mut hasher, b"result", result.as_str());
    }
    format!(
        "sys5-i3-adapter-result-frontier-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn i3_adapter_observation_policy_ref(
    observation_policy: &mir_semantics::evaluation_materialization::ObservationPolicy,
) -> String {
    i3_adapter_opaque_reference(
        I3_ADAPTER_OBSERVATION_POLICY_REF_DOMAIN,
        "sys5-i3-adapter-observation-policy-sha256-v1:",
        [(
            "observation-policy-name",
            observation_policy.name.as_bytes(),
        )],
    )
}

fn i3_adapter_policy_stamp_ref(
    policy_stamp: &mir_semantics::evaluation_materialization::PolicyStamp,
) -> String {
    i3_adapter_opaque_reference(
        I3_ADAPTER_POLICY_STAMP_REF_DOMAIN,
        "sys5-i3-adapter-policy-stamp-sha256-v1:",
        [
            (
                "evaluation-policy-name",
                policy_stamp.evaluation_policy.name.as_bytes(),
            ),
            (
                "evaluation-policy-deterministic",
                &[u8::from(policy_stamp.evaluation_policy.deterministic)],
            ),
            (
                "observation-policy-name",
                policy_stamp.observation_policy.name.as_bytes(),
            ),
        ],
    )
}

fn i3_adapter_static_retry_contract_name(
    retry_contract: mir_semantics::surface_v0_pipeline::StaticRetryContractKind,
) -> &'static str {
    match retry_contract {
        mir_semantics::surface_v0_pipeline::StaticRetryContractKind::ReturnExistingNoNewConsumption => {
            "ReturnExistingNoNewConsumption"
        }
    }
}

fn i3_adapter_opaque_reference<const N: usize>(
    domain: &[u8],
    prefix: &str,
    fields: [(&str, &[u8]); N],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    for (name, value) in fields {
        i3_adapter_digest_field(&mut hasher, name.as_bytes(), value);
    }
    format!("{prefix}{:x}", hasher.finalize())
}

fn i3_adapter_digest_text(hasher: &mut Sha256, tag: &[u8], value: &str) {
    i3_adapter_digest_field(hasher, tag, value.as_bytes());
}

fn i3_adapter_digest_u64(hasher: &mut Sha256, tag: &[u8], value: u64) {
    i3_adapter_digest_field(hasher, tag, &value.to_be_bytes());
}

fn i3_adapter_digest_field(hasher: &mut Sha256, tag: &[u8], bytes: &[u8]) {
    hasher.update(
        u64::try_from(tag.len())
            .expect("finite I3 adapter field tag length fits u64")
            .to_be_bytes(),
    );
    hasher.update(tag);
    hasher.update(
        u64::try_from(bytes.len())
            .expect("finite I3 adapter field length fits u64")
            .to_be_bytes(),
    );
    hasher.update(bytes);
}

fn i3_adapter_append_text_frame(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend(
        u64::try_from(value.len())
            .expect("finite I3 adapter fingerprint text length fits u64")
            .to_be_bytes(),
    );
    bytes.extend(value.as_bytes());
}

fn i3_adapter_append_optional_text_frame(bytes: &mut Vec<u8>, value: Option<&str>) {
    match value {
        Some(value) => {
            bytes.push(1);
            i3_adapter_append_text_frame(bytes, value);
        }
        None => bytes.push(0),
    }
}

struct I3AdapterFullRetainedContractFingerprintVisitor {
    fields: Vec<(String, Vec<u8>)>,
}

impl I3AdapterFullRetainedContractFingerprintVisitor {
    fn field_names(&self) -> Vec<String> {
        self.fields.iter().map(|(name, _)| name.clone()).collect()
    }

    fn finish(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(I3_ADAPTER_FULL_CONTRACT_FINGERPRINT_DOMAIN);
        for (name, value) in &self.fields {
            i3_adapter_digest_field(&mut hasher, name.as_bytes(), value);
        }
        format!(
            "sys5-i3-adapter-carrier-contract-sha256-v1:{:x}",
            hasher.finalize()
        )
    }

    fn push_text(&mut self, name: &str, value: &str) {
        self.fields
            .push((name.to_string(), value.as_bytes().to_vec()));
    }

    fn push_bool(&mut self, name: &str, value: bool) {
        self.fields.push((name.to_string(), vec![u8::from(value)]));
    }

    fn push_texts(&mut self, name: &str, values: &[String]) {
        let mut bytes = Vec::new();
        bytes.extend(
            u64::try_from(values.len())
                .expect("finite I3 adapter fingerprint row count fits u64")
                .to_be_bytes(),
        );
        for value in values {
            bytes.extend(
                u64::try_from(value.len())
                    .expect("finite I3 adapter fingerprint text length fits u64")
                    .to_be_bytes(),
            );
            bytes.extend(value.as_bytes());
        }
        self.fields.push((name.to_string(), bytes));
    }

    fn push_authority_requirement_rows(
        &mut self,
        name: &str,
        rows: &[Sys5I3AdapterAuthorityRequirementRow],
    ) {
        let mut bytes = Vec::new();
        bytes.extend(
            u64::try_from(rows.len())
                .expect("finite I3 adapter authority row count fits u64")
                .to_be_bytes(),
        );
        for row in rows {
            let Sys5I3AdapterAuthorityRequirementRow {
                requirement_kind_name,
                generated_obligation_present,
                generated_obligation_kind_name,
                generated_obligation_detail_name,
                provenance_name,
                authority_category_name,
            } = row;
            i3_adapter_append_text_frame(&mut bytes, requirement_kind_name);
            bytes.push(u8::from(*generated_obligation_present));
            i3_adapter_append_optional_text_frame(
                &mut bytes,
                generated_obligation_kind_name.as_deref(),
            );
            i3_adapter_append_optional_text_frame(
                &mut bytes,
                generated_obligation_detail_name.as_deref(),
            );
            i3_adapter_append_text_frame(&mut bytes, provenance_name);
            i3_adapter_append_optional_text_frame(&mut bytes, authority_category_name.as_deref());
        }
        self.fields.push((name.to_string(), bytes));
    }
}

fn i3_adapter_visit_owner_facts(
    visitor: &mut I3AdapterFullRetainedContractFingerprintVisitor,
    facts: &Sys5I3AdapterOwnerFacts,
) {
    let Sys5I3AdapterOwnerFacts {
        origin_principal_ref,
        origin_locus_template,
        target_owner_locus_template,
    } = facts;
    visitor.push_text("origin-principal-ref", origin_principal_ref);
    visitor.push_text("origin-locus-template", origin_locus_template);
    visitor.push_text("target-owner-locus-template", target_owner_locus_template);
}

fn i3_adapter_visit_designated_input_facts(
    visitor: &mut I3AdapterFullRetainedContractFingerprintVisitor,
    facts: &Sys5I3AdapterDesignatedInputFacts,
) {
    let Sys5I3AdapterDesignatedInputFacts {
        dependency_ordinal,
        typed_state_read_ref,
        requester_site_ref,
        authority_origin_ref,
        request_ref,
        receipt_use_ref,
        designated_evaluator_locus,
        source_owner_locus,
        frontier_requirement_names,
    } = facts;
    visitor.push_text("dependency-ordinal", &dependency_ordinal.to_string());
    visitor.push_text("typed-state-read-ref", typed_state_read_ref);
    visitor.push_text("requester-site-ref", requester_site_ref);
    visitor.push_text("authority-origin-ref", authority_origin_ref);
    visitor.push_text("request-ref", request_ref);
    visitor.push_text("receipt-use-ref", receipt_use_ref);
    visitor.push_text("designated-evaluator-locus", designated_evaluator_locus);
    visitor.push_text("source-owner-locus", source_owner_locus);
    visitor.push_texts("frontier-requirement-names", frontier_requirement_names);
}

fn i3_adapter_full_retained_contract_fingerprint_visitor(
    contract: &Sys5I3AdapterCarrierContract,
) -> I3AdapterFullRetainedContractFingerprintVisitor {
    let Sys5I3AdapterCarrierContract {
        checked_program_ref,
        operation_id,
        edge_kind,
        lifecycle_kind,
        source_locus,
        target_locus,
        logical_source_path,
        source_span,
        source_ref,
        core_ref,
        source_artifact_ref,
        target_artifact_ref,
        edge_ref,
        declared_failure_names,
        effect_kind_names,
        required_occurrence_slot_names,
        linked_request_identity,
        typed_outcome,
        receipt_consumption,
        authority_requirements,
        redaction,
        checked_core_bound,
        transfers_authority,
        mints_authority_without_source,
        variant_facts,
        full_retained_contract_fingerprint: _,
        full_retained_contract_fingerprint_field_names: _,
    } = contract;
    let Sys5I3AdapterAuthorityRequirements {
        rows,
        category_names,
        requires_membership_epoch_and_incarnation,
        requires_capability_and_witness_refs,
    } = authority_requirements;
    let mut visitor = I3AdapterFullRetainedContractFingerprintVisitor { fields: Vec::new() };
    visitor.push_text(
        "variant-discriminant",
        i3_adapter_variant_discriminant(variant_facts),
    );
    visitor.push_text("checked-program-ref", checked_program_ref);
    visitor.push_text("operation-id", operation_id);
    visitor.push_text("edge-kind", edge_kind);
    visitor.push_text("lifecycle-kind", lifecycle_kind);
    visitor.push_text("source-locus", source_locus);
    visitor.push_text("target-locus", target_locus);
    visitor.push_text("logical-source-path", logical_source_path);
    visitor.push_text("source-span", &i3_adapter_source_span_text(*source_span));
    visitor.push_text("source-ref", source_ref);
    visitor.push_text("core-ref", core_ref);
    visitor.push_text("source-artifact-ref", source_artifact_ref);
    visitor.push_text("target-artifact-ref", target_artifact_ref);
    visitor.push_text("edge-ref", edge_ref);
    visitor.push_texts("declared-failure-names", declared_failure_names);
    visitor.push_texts("effect-kind-names", effect_kind_names);
    visitor.push_texts(
        "required-occurrence-slot-names",
        required_occurrence_slot_names,
    );
    visitor.push_bool("requires-linked-request-identity", *linked_request_identity);
    visitor.push_bool("requires-typed-outcome", *typed_outcome);
    visitor.push_bool("requires-receipt-consumption-state", *receipt_consumption);
    visitor.push_texts("authority-category-names", category_names);
    visitor.push_authority_requirement_rows("authority-requirement-rows", rows);
    visitor.push_bool(
        "requires-membership-epoch-and-incarnation",
        *requires_membership_epoch_and_incarnation,
    );
    visitor.push_bool(
        "requires-capability-and-witness-refs",
        *requires_capability_and_witness_refs,
    );
    visitor.push_text(
        "redaction",
        match redaction {
            Sys5I3ProbeRedaction::ReferenceOnly => "ReferenceOnly",
        },
    );
    visitor.push_bool("checked-core-bound", *checked_core_bound);
    visitor.push_bool("transfers-authority", *transfers_authority);
    visitor.push_bool(
        "mints-authority-without-source",
        *mints_authority_without_source,
    );
    visitor.push_bool("public-api-or-wire-contract", false);
    match variant_facts {
        Sys5I3AdapterCarrierVariantFacts::OwnerRequest(facts) => {
            i3_adapter_visit_owner_facts(&mut visitor, facts);
        }
        Sys5I3AdapterCarrierVariantFacts::OwnerReplyReceipt(facts) => {
            i3_adapter_visit_owner_facts(&mut visitor, facts);
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(facts) => {
            i3_adapter_visit_designated_input_facts(&mut visitor, facts);
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(facts) => {
            i3_adapter_visit_designated_input_facts(&mut visitor, facts);
        }
        Sys5I3AdapterCarrierVariantFacts::RelationProjectionPublication(facts) => {
            let Sys5I3AdapterRelationPublicationFacts {
                relation_name,
                publication_locus,
                consumer_locus,
            } = facts;
            visitor.push_text("relation-name", relation_name);
            visitor.push_text("publication-locus", publication_locus);
            visitor.push_text("consumer-locus", consumer_locus);
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedResultDelivery(facts) => {
            let Sys5I3AdapterDesignatedResultFacts {
                evaluator_locus,
                consumer_locus,
                result_version_ref,
                input_frontier_ref,
                result_frontier_ref,
                observation_policy_ref,
                policy_stamp_ref,
                static_retry_contract_name,
            } = facts;
            visitor.push_text("evaluator-locus", evaluator_locus);
            visitor.push_text("consumer-locus", consumer_locus);
            visitor.push_text("result-version-ref", result_version_ref);
            visitor.push_text("input-frontier-ref", input_frontier_ref);
            visitor.push_text("result-frontier-ref", result_frontier_ref);
            visitor.push_text("observation-policy-ref", observation_policy_ref);
            visitor.push_text("policy-stamp-ref", policy_stamp_ref);
            visitor.push_text("static-retry-contract", static_retry_contract_name);
        }
    }
    visitor
}

fn i3_adapter_variant_discriminant(facts: &Sys5I3AdapterCarrierVariantFacts) -> &'static str {
    match facts {
        Sys5I3AdapterCarrierVariantFacts::OwnerRequest(_) => "owner-request",
        Sys5I3AdapterCarrierVariantFacts::OwnerReplyReceipt(_) => "owner-reply-receipt",
        Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(_) => "designated-input-request",
        Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(_) => "designated-input-receipt",
        Sys5I3AdapterCarrierVariantFacts::RelationProjectionPublication(_) => {
            "relation-projection-publication"
        }
        Sys5I3AdapterCarrierVariantFacts::DesignatedResultDelivery(_) => {
            "designated-result-delivery"
        }
    }
}

fn i3_adapter_source_span_text(span: Sys5SourceSpan) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        span.start, span.end, span.start_line, span.start_column, span.end_line, span.end_column,
    )
}

/// Combines the projection-owned owner-request component with the generated
/// edge/artifact/program identity retained by SYS-5. The component itself
/// keeps raw carrier values, including the principal template, inside SYS-3.
fn i3_probe_full_retained_contract_fingerprint(
    snapshot: &Sys5I3ProbeCarrierContract,
    owner_request_component: &[u8; 32],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(I3_PROBE_FULL_CONTRACT_FINGERPRINT_DOMAIN);

    i3_probe_digest_field(
        &mut hasher,
        b"owner-request-carrier-component",
        owner_request_component,
    );
    i3_probe_digest_text(&mut hasher, b"generated-edge-kind", &snapshot.edge_kind);
    i3_probe_digest_text(
        &mut hasher,
        b"generated-edge-operation",
        &snapshot.operation_id,
    );
    i3_probe_digest_text(
        &mut hasher,
        b"generated-source-locus",
        &snapshot.source_locus,
    );
    i3_probe_digest_text(
        &mut hasher,
        b"generated-target-locus",
        &snapshot.target_locus,
    );
    i3_probe_digest_text(
        &mut hasher,
        b"logical-source-path",
        &snapshot.logical_source_path,
    );
    i3_probe_digest_text(&mut hasher, b"source-reference", &snapshot.source_ref);
    i3_probe_digest_u64(
        &mut hasher,
        b"source-span-start",
        snapshot.source_span.start,
    );
    i3_probe_digest_u64(&mut hasher, b"source-span-end", snapshot.source_span.end);
    i3_probe_digest_u64(
        &mut hasher,
        b"source-span-start-line",
        u64::from(snapshot.source_span.start_line),
    );
    i3_probe_digest_u64(
        &mut hasher,
        b"source-span-start-column",
        u64::from(snapshot.source_span.start_column),
    );
    i3_probe_digest_u64(
        &mut hasher,
        b"source-span-end-line",
        u64::from(snapshot.source_span.end_line),
    );
    i3_probe_digest_u64(
        &mut hasher,
        b"source-span-end-column",
        u64::from(snapshot.source_span.end_column),
    );
    i3_probe_digest_text(&mut hasher, b"generated-core-reference", &snapshot.core_ref);
    i3_probe_digest_text(
        &mut hasher,
        b"checked-program-reference",
        &snapshot.checked_program_ref,
    );
    i3_probe_digest_text(
        &mut hasher,
        b"source-artifact-reference",
        &snapshot.source_artifact_ref,
    );
    i3_probe_digest_text(
        &mut hasher,
        b"target-artifact-reference",
        &snapshot.target_artifact_ref,
    );
    i3_probe_digest_text(&mut hasher, b"edge-reference", &snapshot.edge_ref);

    format!(
        "sys5-i3-probe-carrier-contract-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn i3_probe_digest_text(hasher: &mut Sha256, tag: &[u8], value: &str) {
    i3_probe_digest_field(hasher, tag, value.as_bytes());
}

fn i3_probe_digest_u64(hasher: &mut Sha256, tag: &[u8], value: u64) {
    i3_probe_digest_field(hasher, tag, &value.to_be_bytes());
}

fn i3_probe_digest_field(hasher: &mut Sha256, tag: &[u8], bytes: &[u8]) {
    hasher.update(
        u64::try_from(tag.len())
            .expect("finite owner-request field tag length fits u64")
            .to_be_bytes(),
    );
    hasher.update(tag);
    hasher.update(
        u64::try_from(bytes.len())
            .expect("finite owner-request field length fits u64")
            .to_be_bytes(),
    );
    hasher.update(bytes);
}

fn effect_kind_name(kind: EffectKind) -> &'static str {
    match kind {
        EffectKind::OwnerRequest => "OwnerRequest",
        EffectKind::OwnerLocalRead => "OwnerLocalRead",
        EffectKind::OwnerWrite => "OwnerWrite",
        EffectKind::ActorReadReply => "ActorReadReply",
        EffectKind::ObserverPublish => "ObserverPublish",
        EffectKind::RelationPublish => "RelationPublish",
        EffectKind::DesignatedRemoteRequest => "DesignatedRemoteRequest",
        EffectKind::DesignatedReceiptUse => "DesignatedReceiptUse",
        EffectKind::DesignatedValuePublish => "DesignatedValuePublish",
        EffectKind::DesignatedResultDelivery => "DesignatedResultDelivery",
        EffectKind::DesignatedResultConsume => "DesignatedResultConsume",
    }
}

fn i3_probe_authority_requirements(
    authority_requirement_rows: &[I3AdapterCarrierStaticAuthorityRequirementRow],
) -> Sys5I3ProbeAuthorityRequirements {
    let adapter_requirements = i3_adapter_authority_requirements(authority_requirement_rows);
    Sys5I3ProbeAuthorityRequirements {
        category_names: adapter_requirements.category_names.clone(),
        requires_membership_epoch_and_incarnation: adapter_requirements
            .requires_membership_epoch_and_incarnation,
        requires_capability_and_witness_refs: adapter_requirements
            .requires_capability_and_witness_refs,
    }
}

fn i3_adapter_authority_requirements(
    authority_requirement_rows: &[I3AdapterCarrierStaticAuthorityRequirementRow],
) -> Sys5I3AdapterAuthorityRequirements {
    let rows = authority_requirement_rows
        .iter()
        .map(
            |I3AdapterCarrierStaticAuthorityRequirementRow {
                 requirement_kind,
                 generated_obligation,
                 provenance,
                 authority_category,
             }| {
                let (
                    generated_obligation_present,
                    generated_obligation_kind_name,
                    generated_obligation_detail_name,
                ) = i3_adapter_generated_obligation_parts(generated_obligation.as_ref());
                Sys5I3AdapterAuthorityRequirementRow {
                    requirement_kind_name: i3_adapter_runtime_seam_requirement_kind_name(
                        *requirement_kind,
                    )
                    .to_string(),
                    generated_obligation_present,
                    generated_obligation_kind_name: generated_obligation_kind_name
                        .map(str::to_string),
                    generated_obligation_detail_name: generated_obligation_detail_name
                        .map(str::to_string),
                    provenance_name: i3_adapter_carrier_provenance_kind_name(*provenance)
                        .to_string(),
                    authority_category_name: authority_category
                        .map(seam_authority_category_name)
                        .map(str::to_string),
                }
            },
        )
        .collect();
    Sys5I3AdapterAuthorityRequirements::from_rows(rows)
}

fn i3_adapter_runtime_seam_requirement_kind_name(kind: RuntimeSeamRequirementKind) -> &'static str {
    match kind {
        RuntimeSeamRequirementKind::MembershipEpochIncarnation => "MembershipEpochIncarnation",
        RuntimeSeamRequirementKind::LiveCapabilityRef => "LiveCapabilityRef",
        RuntimeSeamRequirementKind::LiveWitnessRef => "LiveWitnessRef",
        RuntimeSeamRequirementKind::ProducerReleaseCapabilitySlot => {
            "ProducerReleaseCapabilitySlot"
        }
        RuntimeSeamRequirementKind::ProducerReleaseWitnessSlot => "ProducerReleaseWitnessSlot",
        RuntimeSeamRequirementKind::EvaluatorDecisionAuthoritySlot => {
            "EvaluatorDecisionAuthoritySlot"
        }
        RuntimeSeamRequirementKind::ConsumerMembershipEpochIncarnation => {
            "ConsumerMembershipEpochIncarnation"
        }
        RuntimeSeamRequirementKind::ConsumerCapabilityRef => "ConsumerCapabilityRef",
        RuntimeSeamRequirementKind::ConsumerWitnessRef => "ConsumerWitnessRef",
    }
}

fn i3_adapter_generated_obligation_parts(
    obligation: Option<&GeneratedObligationKind>,
) -> (bool, Option<&'static str>, Option<&str>) {
    match obligation {
        None => (false, None, None),
        Some(GeneratedObligationKind::Failure(name)) => (true, Some("Failure"), Some(name)),
        Some(GeneratedObligationKind::Capability) => (true, Some("Capability"), None),
        Some(GeneratedObligationKind::Witness) => (true, Some("Witness"), None),
        Some(GeneratedObligationKind::Authority) => (true, Some("Authority"), None),
        Some(GeneratedObligationKind::AdmittedEvaluatorAuthority) => {
            (true, Some("AdmittedEvaluatorAuthority"), None)
        }
        Some(GeneratedObligationKind::DesignatedResultConsumerAuthority) => {
            (true, Some("DesignatedResultConsumerAuthority"), None)
        }
        Some(GeneratedObligationKind::Evaluation(kind)) => (
            true,
            Some("Evaluation"),
            Some(i3_adapter_checked_evaluation_kind_name(*kind)),
        ),
    }
}

fn i3_adapter_checked_evaluation_kind_name(kind: CheckedEvaluationKind) -> &'static str {
    match kind {
        CheckedEvaluationKind::OwnerRmw => "OwnerRmw",
        CheckedEvaluationKind::DesignatedPublishValue => "DesignatedPublishValue",
        CheckedEvaluationKind::PublishRelation => "PublishRelation",
        CheckedEvaluationKind::ConsumerLocalProjection => "ConsumerLocalProjection",
        CheckedEvaluationKind::DesignatedResultConsume => "DesignatedResultConsume",
    }
}

fn i3_adapter_carrier_provenance_kind_name(kind: CarrierProvenanceKind) -> &'static str {
    match kind {
        CarrierProvenanceKind::RequiredFromSealedRuntimeSeam => "RequiredFromSealedRuntimeSeam",
    }
}

fn seam_authority_category_name(kind: SeamAuthorityKind) -> &'static str {
    match kind {
        SeamAuthorityKind::MembershipEpochIncarnation => "MembershipEpochIncarnation",
        SeamAuthorityKind::OwnerCapabilityRef => "OwnerCapabilityRef",
        SeamAuthorityKind::OwnerWitnessRef => "OwnerWitnessRef",
        SeamAuthorityKind::ProducerReleaseCapability => "ProducerReleaseCapability",
        SeamAuthorityKind::ProducerReleaseWitness => "ProducerReleaseWitness",
        SeamAuthorityKind::EvaluatorDecisionAuthority => "EvaluatorDecisionAuthority",
        SeamAuthorityKind::DesignatedResultConsumerMembership => {
            "DesignatedResultConsumerMembership"
        }
        SeamAuthorityKind::DesignatedResultConsumerCapability => {
            "DesignatedResultConsumerCapability"
        }
        SeamAuthorityKind::DesignatedResultConsumerWitness => "DesignatedResultConsumerWitness",
    }
}

fn observer_safe_view(summary: &Sys5SemanticSummary) -> Sys5ObserverSafeView {
    let mut semantic_fragments = vec![format!(
        "profile:{}:{}",
        summary.profile_name, summary.profile_status
    )];
    for mapping in &summary.source_core_artifact_mappings {
        semantic_fragments.push(format!("source:{}", mapping.source_path));
        semantic_fragments.push(format!("core:{}", mapping.core_kind));
        semantic_fragments.push(format!("core-ref:{}", mapping.core_ref));
        semantic_fragments.push(format!(
            "artifact:{}:{}",
            mapping.artifact_locus, mapping.artifact_kind
        ));
        semantic_fragments.push(format!("artifact-ref:{}", mapping.fragment_ref));
    }
    for edge in &summary.generated_communication {
        semantic_fragments.push(format!(
            "edge:{}->{}:{}",
            edge.from_locus, edge.to_locus, edge.kind
        ));
        semantic_fragments.push(format!("edge-ref:{}", edge.edge_ref));
    }
    for residual in &summary.auth_residuals {
        semantic_fragments.push(format!("auth:{}:{}", residual.authority, residual.status));
    }
    for residual in &summary.verification_residuals {
        semantic_fragments.push(format!("verify:{}:{}", residual.verifier, residual.status));
    }
    semantic_fragments.sort();
    semantic_fragments.dedup();
    Sys5ObserverSafeView { semantic_fragments }
}

#[cfg(test)]
mod i3_adapter_carrier_contract_red_tests {
    use super::*;
    use crate::sys3_projection::{
        CarrierContract, CarrierProvenanceKind, I3AdapterCarrierStaticAuthorityRequirementRow,
        RuntimeSeamRequirementKind, SeamAuthorityKind,
    };
    use mir_semantics::surface_v0_pipeline::{CheckedEvaluationKind, GeneratedObligationKind};

    const ACTIVE_I2_SOURCE: &str =
        include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

    fn runtime_seam_requirement_kind_name(kind: RuntimeSeamRequirementKind) -> &'static str {
        match kind {
            RuntimeSeamRequirementKind::MembershipEpochIncarnation => "MembershipEpochIncarnation",
            RuntimeSeamRequirementKind::LiveCapabilityRef => "LiveCapabilityRef",
            RuntimeSeamRequirementKind::LiveWitnessRef => "LiveWitnessRef",
            RuntimeSeamRequirementKind::ProducerReleaseCapabilitySlot => {
                "ProducerReleaseCapabilitySlot"
            }
            RuntimeSeamRequirementKind::ProducerReleaseWitnessSlot => "ProducerReleaseWitnessSlot",
            RuntimeSeamRequirementKind::EvaluatorDecisionAuthoritySlot => {
                "EvaluatorDecisionAuthoritySlot"
            }
            RuntimeSeamRequirementKind::ConsumerMembershipEpochIncarnation => {
                "ConsumerMembershipEpochIncarnation"
            }
            RuntimeSeamRequirementKind::ConsumerCapabilityRef => "ConsumerCapabilityRef",
            RuntimeSeamRequirementKind::ConsumerWitnessRef => "ConsumerWitnessRef",
        }
    }

    fn carrier_provenance_kind_name(kind: CarrierProvenanceKind) -> &'static str {
        match kind {
            CarrierProvenanceKind::RequiredFromSealedRuntimeSeam => "RequiredFromSealedRuntimeSeam",
        }
    }

    fn seam_authority_kind_name(kind: SeamAuthorityKind) -> &'static str {
        match kind {
            SeamAuthorityKind::MembershipEpochIncarnation => "MembershipEpochIncarnation",
            SeamAuthorityKind::OwnerCapabilityRef => "OwnerCapabilityRef",
            SeamAuthorityKind::OwnerWitnessRef => "OwnerWitnessRef",
            SeamAuthorityKind::ProducerReleaseCapability => "ProducerReleaseCapability",
            SeamAuthorityKind::ProducerReleaseWitness => "ProducerReleaseWitness",
            SeamAuthorityKind::EvaluatorDecisionAuthority => "EvaluatorDecisionAuthority",
            SeamAuthorityKind::DesignatedResultConsumerMembership => {
                "DesignatedResultConsumerMembership"
            }
            SeamAuthorityKind::DesignatedResultConsumerCapability => {
                "DesignatedResultConsumerCapability"
            }
            SeamAuthorityKind::DesignatedResultConsumerWitness => "DesignatedResultConsumerWitness",
        }
    }

    fn checked_evaluation_kind_name(kind: CheckedEvaluationKind) -> &'static str {
        match kind {
            CheckedEvaluationKind::OwnerRmw => "OwnerRmw",
            CheckedEvaluationKind::DesignatedPublishValue => "DesignatedPublishValue",
            CheckedEvaluationKind::PublishRelation => "PublishRelation",
            CheckedEvaluationKind::ConsumerLocalProjection => "ConsumerLocalProjection",
            CheckedEvaluationKind::DesignatedResultConsume => "DesignatedResultConsume",
        }
    }

    fn generated_obligation_parts(
        obligation: Option<&GeneratedObligationKind>,
    ) -> (bool, Option<&str>, Option<&str>) {
        match obligation {
            None => (false, None, None),
            Some(GeneratedObligationKind::Failure(name)) => (true, Some("Failure"), Some(name)),
            Some(GeneratedObligationKind::Capability) => (true, Some("Capability"), None),
            Some(GeneratedObligationKind::Witness) => (true, Some("Witness"), None),
            Some(GeneratedObligationKind::Authority) => (true, Some("Authority"), None),
            Some(GeneratedObligationKind::AdmittedEvaluatorAuthority) => {
                (true, Some("AdmittedEvaluatorAuthority"), None)
            }
            Some(GeneratedObligationKind::DesignatedResultConsumerAuthority) => {
                (true, Some("DesignatedResultConsumerAuthority"), None)
            }
            Some(GeneratedObligationKind::Evaluation(kind)) => (
                true,
                Some("Evaluation"),
                Some(checked_evaluation_kind_name(*kind)),
            ),
        }
    }

    fn assert_adapter_authority_rows_match_carrier(
        contract: &Sys5I3AdapterCarrierContract,
        carrier: &CarrierContract,
    ) {
        let actual = contract.authority_requirements().rows();
        let expected = carrier
            .authority_requirements()
            .runtime_seam_requirements()
            .rows();
        assert_eq!(actual.len(), expected.len());
        for (actual, (requirement, obligation, provenance, authority)) in
            actual.iter().zip(expected)
        {
            let (present, kind, detail) = generated_obligation_parts(obligation.as_ref());
            assert_eq!(
                actual.requirement_kind_name(),
                runtime_seam_requirement_kind_name(*requirement)
            );
            assert_eq!(actual.generated_obligation_present(), present);
            assert_eq!(actual.generated_obligation_kind_name(), kind);
            assert_eq!(actual.generated_obligation_detail_name(), detail);
            assert_eq!(
                actual.provenance_name(),
                carrier_provenance_kind_name(*provenance)
            );
            assert_eq!(
                actual.authority_category_name(),
                authority
                    .as_ref()
                    .map(|authority| seam_authority_kind_name(*authority))
            );
        }
    }

    fn assert_static_authority_rows_match_carrier(carrier: &CarrierContract) {
        let static_facts = carrier
            .i3_adapter_static_facts()
            .expect("every accepted carrier must retain finite static projection facts");
        let I3AdapterCarrierStaticFacts {
            edge_kind,
            lifecycle_kind,
            operation_id,
            source_ref,
            core_ref,
            origin_locus_template,
            target_owner_locus_template,
            declared_failure_row,
            effect_row,
            authority_requirement_rows,
            occurrence_slots,
            frontiers,
            linked_request_identity,
            typed_outcome,
            evaluator_receipt_consumption,
            reference_only_redaction,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            variant,
        } = static_facts;
        let _ = (
            edge_kind,
            lifecycle_kind,
            operation_id,
            source_ref,
            core_ref,
            origin_locus_template,
            target_owner_locus_template,
            declared_failure_row,
            effect_row,
            occurrence_slots,
            frontiers,
            linked_request_identity,
            typed_outcome,
            evaluator_receipt_consumption,
            reference_only_redaction,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            variant,
        );

        // Production review guard: the source conversion must destructure
        // `AuthorityRequirements` and `RuntimeSeamRequirements` before it
        // constructs these exact ordered static rows.
        let actual: &[I3AdapterCarrierStaticAuthorityRequirementRow] = &authority_requirement_rows;
        let expected = carrier
            .authority_requirements()
            .runtime_seam_requirements()
            .rows();
        assert_eq!(actual.len(), expected.len());
        for (actual, (requirement_kind, generated_obligation, provenance, authority_category)) in
            actual.iter().zip(expected)
        {
            let I3AdapterCarrierStaticAuthorityRequirementRow {
                requirement_kind: actual_requirement_kind,
                generated_obligation: actual_generated_obligation,
                provenance: actual_provenance,
                authority_category: actual_authority_category,
            } = actual;
            let actual_generated_obligation: &Option<GeneratedObligationKind> =
                actual_generated_obligation;
            assert_eq!(*actual_requirement_kind, *requirement_kind);
            assert_eq!(actual_generated_obligation, generated_obligation);
            assert_eq!(*actual_provenance, *provenance);
            assert_eq!(*actual_authority_category, *authority_category);
        }
    }

    fn assert_designated_input_facts_match_dependency(
        contract: &Sys5I3AdapterCarrierContract,
        carrier: &CarrierContract,
    ) {
        let Some(dependency) = carrier.designated_remote_input_dependency() else {
            return;
        };
        assert_eq!(
            dependency.request().source_owner_locus(),
            dependency.source_owner_locus()
        );
        assert_eq!(
            dependency.receipt_use().source_owner_locus(),
            dependency.source_owner_locus()
        );
        assert_eq!(
            dependency.request().typed_state_read(),
            dependency.typed_state_read()
        );
        assert_eq!(
            dependency.receipt_use().typed_state_read(),
            dependency.typed_state_read()
        );
        let static_dependency = dependency.static_projection_facts();
        let facts = match contract.variant_facts() {
            Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(facts)
            | Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(facts) => facts,
            unexpected => {
                panic!("designated dependency must retain an input variant, got {unexpected:?}")
            }
        };
        assert_eq!(
            facts.typed_state_read_ref(),
            i3_adapter_typed_state_read_ref(static_dependency.typed_state_read())
        );
        assert_eq!(
            facts.requester_site_ref(),
            i3_adapter_requester_site_ref(static_dependency.requester_site())
        );
        assert_eq!(
            facts.authority_origin_ref(),
            i3_adapter_authority_origin_ref(static_dependency.authority_origin())
        );
        assert_eq!(
            facts.request_ref(),
            i3_adapter_designated_request_ref(static_dependency.request())
        );
        assert_eq!(
            facts.receipt_use_ref(),
            i3_adapter_designated_receipt_use_ref(static_dependency.receipt_use())
        );
    }

    fn assert_exhaustive_adapter_variant_facts(facts: &Sys5I3AdapterCarrierVariantFacts) {
        match facts {
            Sys5I3AdapterCarrierVariantFacts::OwnerRequest(facts)
            | Sys5I3AdapterCarrierVariantFacts::OwnerReplyReceipt(facts) => {
                let Sys5I3AdapterOwnerFacts {
                    origin_principal_ref,
                    origin_locus_template,
                    target_owner_locus_template,
                } = facts;
                let _ = (
                    origin_principal_ref,
                    origin_locus_template,
                    target_owner_locus_template,
                );
            }
            Sys5I3AdapterCarrierVariantFacts::DesignatedInputRequest(facts)
            | Sys5I3AdapterCarrierVariantFacts::DesignatedInputReceipt(facts) => {
                let Sys5I3AdapterDesignatedInputFacts {
                    dependency_ordinal,
                    typed_state_read_ref,
                    requester_site_ref,
                    authority_origin_ref,
                    request_ref,
                    receipt_use_ref,
                    designated_evaluator_locus,
                    source_owner_locus,
                    frontier_requirement_names,
                } = facts;
                let _ = (
                    dependency_ordinal,
                    typed_state_read_ref,
                    requester_site_ref,
                    authority_origin_ref,
                    request_ref,
                    receipt_use_ref,
                    designated_evaluator_locus,
                    source_owner_locus,
                    frontier_requirement_names,
                );
            }
            Sys5I3AdapterCarrierVariantFacts::RelationProjectionPublication(facts) => {
                let Sys5I3AdapterRelationPublicationFacts {
                    relation_name,
                    publication_locus,
                    consumer_locus,
                } = facts;
                let _ = (relation_name, publication_locus, consumer_locus);
            }
            Sys5I3AdapterCarrierVariantFacts::DesignatedResultDelivery(facts) => {
                let Sys5I3AdapterDesignatedResultFacts {
                    evaluator_locus,
                    consumer_locus,
                    result_version_ref,
                    input_frontier_ref,
                    result_frontier_ref,
                    observation_policy_ref,
                    policy_stamp_ref,
                    static_retry_contract_name,
                } = facts;
                let _ = (
                    evaluator_locus,
                    consumer_locus,
                    result_version_ref,
                    input_frontier_ref,
                    result_frontier_ref,
                    observation_policy_ref,
                    policy_stamp_ref,
                    static_retry_contract_name,
                );
            }
        }
    }

    fn assert_exhaustive_adapter_contract_fields(contract: &Sys5I3AdapterCarrierContract) {
        let Sys5I3AdapterCarrierContract {
            checked_program_ref,
            operation_id,
            edge_kind,
            lifecycle_kind,
            source_locus,
            target_locus,
            logical_source_path,
            source_span,
            source_ref,
            core_ref,
            source_artifact_ref,
            target_artifact_ref,
            edge_ref,
            declared_failure_names,
            effect_kind_names,
            required_occurrence_slot_names,
            linked_request_identity,
            typed_outcome,
            receipt_consumption,
            authority_requirements,
            redaction,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            variant_facts,
            full_retained_contract_fingerprint,
            full_retained_contract_fingerprint_field_names,
        } = contract;
        let _ = (
            checked_program_ref,
            operation_id,
            edge_kind,
            lifecycle_kind,
            source_locus,
            target_locus,
            logical_source_path,
            source_span,
            source_ref,
            core_ref,
            source_artifact_ref,
            target_artifact_ref,
            edge_ref,
            declared_failure_names,
            effect_kind_names,
            required_occurrence_slot_names,
            linked_request_identity,
            typed_outcome,
            receipt_consumption,
            authority_requirements,
            redaction,
            checked_core_bound,
            transfers_authority,
            mints_authority_without_source,
            full_retained_contract_fingerprint,
            full_retained_contract_fingerprint_field_names,
        );
        assert_exhaustive_adapter_variant_facts(variant_facts);
    }

    #[test]
    fn i3_adapter_mapper_rejects_absolute_value_stream_without_constructing_a_snapshot() {
        // Review guard: the production mapper must keep its conversion match
        // explicit and exhaustive; a wildcard must not admit this seventh kind.
        let error: Sys5I3ProbeFacadeError = match i3_adapter_carrier_family_for_edge_kind(
            CommunicationEdgeKind::AbsoluteValueStream,
        ) {
            Err(error) => error,
            Ok(_) => panic!("AbsoluteValueStream must not construct an adapter snapshot"),
        };

        assert_eq!(
            error.kind(),
            Sys5I3ProbeFacadeErrorKind::NotAcceptedCarrierFamily
        );
    }

    #[test]
    fn i3_adapter_fingerprint_and_inventory_share_one_exhaustive_typed_visitor() {
        let project = build_project(Sys5SourceInput::inline(
            "samples/clean-near-end/mirrorea-i2-local-toy/main.mir",
            ACTIVE_I2_SOURCE,
        ))
        .expect("the ordinary active source must check and project");

        let generated_edges = &project.semantic_summary().generated_communication;
        assert_eq!(generated_edges.len(), 12);
        for edge in generated_edges {
            let contract = project
                .i3_adapter_carrier_contract(&edge.edge_ref)
                .expect("every accepted active generated edge has an adapter contract");
            let carrier = project
                .projection
                .communication_plan()
                .edges()
                .iter()
                .find(|candidate| candidate.edge_ref() == edge.edge_ref)
                .expect("every summary edge retains its exact projection carrier")
                .carrier_contract();
            assert_eq!(
                contract.declared_failure_names(),
                carrier.declared_failure_row().names()
            );
            assert_eq!(
                contract.effect_kind_names(),
                carrier
                    .effect_row()
                    .kinds()
                    .into_iter()
                    .map(effect_kind_name)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            );
            assert_adapter_authority_rows_match_carrier(&contract, carrier);
            assert_static_authority_rows_match_carrier(carrier);
            assert_designated_input_facts_match_dependency(&contract, carrier);
            assert_exhaustive_adapter_contract_fields(&contract);
            let visitor = i3_adapter_full_retained_contract_fingerprint_visitor(&contract);
            assert_eq!(
                visitor.field_names(),
                contract.full_retained_contract_fingerprint_field_names()
            );
            assert_eq!(
                visitor.finish(),
                contract.full_retained_contract_fingerprint()
            );
            assert_eq!(
                visitor.fields.len(),
                contract
                    .full_retained_contract_fingerprint_field_names()
                    .len(),
                "every visitor field has one named fingerprint inventory entry"
            );
            for field_index in 0..visitor.fields.len() {
                let mut perturbed = I3AdapterFullRetainedContractFingerprintVisitor {
                    fields: visitor.fields.clone(),
                };
                perturbed.fields[field_index].1.push(0xff);
                assert_ne!(
                    perturbed.finish(),
                    visitor.finish(),
                    "mutating exactly one emitted value must change the full contract digest"
                );
            }
        }
    }
}
