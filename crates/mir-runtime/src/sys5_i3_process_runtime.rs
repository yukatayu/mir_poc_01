//! Private I3-2 process-image and pre-socket process-runtime seam.
//!
//! This module deliberately has no public wire, socket, process launcher, or
//! authority issuer.  It lowers one already checked/projected/admitted local
//! program into per-slot images and transfers exact generated carriers by
//! value only so the subsequent transport milestone has a narrow boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{
    Deserialize, Serialize,
    de::{self, Error as _, MapAccess, SeqAccess, Visitor},
};
use sha2::{Digest, Sha256};

#[path = "sys5_i3_process_snapshot.rs"]
mod process_snapshot;

// A logical activation occurrence is local runtime evidence, not a process
// identifier, endpoint, session, or transport attempt.  It prevents two
// independently derived cohorts of the same checked source from sharing a
// local-store or request identity before I3-3 retry semantics exist.
static NEXT_PROCESS_COHORT_OCCURRENCE: AtomicU64 = AtomicU64::new(1);

use crate::{
    sys3_projection::{BackendProfile, CommunicationEdgeKind},
    sys4_dispatch::{
        FabricProgram, LocalFabric, ObserverSafeM9SemanticRowSets, SealedFabricAdmission,
        SourceAction, Sys4I3PendingOwnerRequestBinding, Sys4I3PrivateProcessCarrierSnapshot,
        Sys4ProcessCarrier,
    },
    sys5_local_slice::Sys5LocalProject,
};

/// Typed, fail-closed outcomes for the provisional I3-2 process seam.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3ProcessRuntimeErrorKind {
    MissingLocusAssignment,
    ExtraLocusAssignment,
    DuplicateLocusAssignment,
    DuplicateDeploymentSlot,
    EmptyDeploymentSlot,
    InsufficientDeploymentSlots,
    UnknownDeploymentSlot,
    ProcessImageAlreadyTaken,
    ForeignArtifact,
    ForeignEdgeContract,
    ImageIntegrityMismatch,
    ImageInventoryProvenanceMismatch,
    ProgramProjectionMismatch,
    MissingRequiredAuthorityEvidence,
    ForeignAuthorityEvidence,
    CohortParentProgramMismatch,
    CohortProjectionMismatch,
    CohortM9GenerationMismatch,
    CohortProvenanceMismatch,
    AuthorityClosureDigestMismatch,
    RuntimeBootstrapRejected,
    NoGeneratedOwnerRequest,
    NonOwnerServe,
    DirectRemoteStore,
    CarrierAdmissionRejected,
    MissingAuthoritativeState,
    OutboundExtractionRejected,
}

/// One observer-safe process-seam failure.  No raw M8/M9 material or source
/// text is included in the diagnostic surface.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessRuntimeError {
    kind: Sys5I3ProcessRuntimeErrorKind,
}

impl Sys5I3ProcessRuntimeError {
    fn new(kind: Sys5I3ProcessRuntimeErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3ProcessRuntimeErrorKind {
        self.kind
    }
}

/// Deployment input for one logical process slot.  It is intentionally only
/// a locus-to-slot-to-endpoint assignment, never a route, Core, authority,
/// state, or expected result.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3DeploymentSlot {
    slot_name: String,
    endpoint: String,
    // Preserve raw construction rows until deployment validation.  Collapsing
    // this to a set here would hide duplicate locus assignment before the
    // fail-closed boundary gets a chance to reject it.
    loci: Vec<String>,
}

impl Sys5I3DeploymentSlot {
    pub fn new<I, S>(slot_name: impl Into<String>, endpoint: impl Into<String>, loci: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            slot_name: slot_name.into(),
            endpoint: endpoint.into(),
            loci: loci.into_iter().map(Into::into).collect(),
        }
    }
}

/// Observer-only deployment view proving that deployment did not choose
/// semantic operations or authority.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3DeploymentManifest;

impl Sys5I3DeploymentManifest {
    pub const fn has_only_locus_slot_endpoint_assignments(&self) -> bool {
        true
    }
}

/// A complete checked deployment assignment.  Its only retained information
/// is the slot/endpoint/locus map plus observer-safe parent provenance refs.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3Deployment {
    slots: Vec<Sys5I3DeploymentSlot>,
    parent_checked_program_ref: String,
    parent_projection_ref: String,
}

impl Sys5I3Deployment {
    pub fn from_checked_project<I>(
        project: &Sys5LocalProject,
        slots: I,
    ) -> Result<Self, Sys5I3ProcessRuntimeError>
    where
        I: IntoIterator<Item = Sys5I3DeploymentSlot>,
    {
        let expected = project
            .semantic_summary()
            .loci
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let slots = slots.into_iter().collect::<Vec<_>>();
        if slots.len() < 2 {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::InsufficientDeploymentSlots,
            ));
        }
        if slots.iter().any(|slot| slot.loci.is_empty()) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::EmptyDeploymentSlot,
            ));
        }
        let mut slot_names = BTreeSet::new();
        if slots
            .iter()
            .any(|slot| !slot_names.insert(slot.slot_name.clone()))
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
            ));
        }
        let mut assigned = BTreeSet::new();
        for locus in slots.iter().flat_map(|slot| slot.loci.iter()) {
            if !expected.contains(locus) {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ExtraLocusAssignment,
                ));
            }
            if !assigned.insert(locus.clone()) {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateLocusAssignment,
                ));
            }
        }
        if assigned != expected {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::MissingLocusAssignment,
            ));
        }
        Ok(Self {
            slots,
            parent_checked_program_ref: project.checked_program_identity_ref().to_string(),
            parent_projection_ref: project.i3_parent_projection_ref(),
        })
    }

    pub fn observer_safe_manifest(&self) -> Sys5I3DeploymentManifest {
        Sys5I3DeploymentManifest
    }

    fn slot(&self, slot_name: &str) -> Option<&Sys5I3DeploymentSlot> {
        self.slots.iter().find(|slot| slot.slot_name == slot_name)
    }
}

/// A compact source-free description of one executable artifact retained in
/// a process image.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessArtifact {
    locus: String,
    operation_id: String,
    kind: String,
    core_ref: String,
    fragment_ref: String,
    parent_checked_program_ref: String,
}

impl Sys5I3ProcessArtifact {
    pub fn locus(&self) -> &str {
        &self.locus
    }
}

/// One reference-only generated edge incident to a process image.  It has no
/// payload, witness, capability, or transport/session data.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3RetainedEdgeContract {
    source_locus: String,
    target_locus: String,
    edge_ref: String,
    operation_id: String,
    kind: String,
    core_ref: String,
    source_artifact_ref: String,
    target_artifact_ref: String,
    parent_checked_program_ref: String,
}

impl Sys5I3RetainedEdgeContract {
    pub fn source_locus(&self) -> &str {
        &self.source_locus
    }

    pub fn target_locus(&self) -> &str {
        &self.target_locus
    }

    pub fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub const fn is_reference_only(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Sys5I3SemanticRow {
    Artifact {
        locus: String,
        fragment_ref: String,
    },
    IncidentEdge {
        source_locus: String,
        target_locus: String,
        edge_ref: String,
    },
}

/// Observer-safe proof that the child has exactly the semantic-row boundary
/// required by its assigned artifacts and incident generated edges.  Raw M9
/// authority is retained only in the private SYS-4/M8 seed.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3RequiredLocalAuthorityClosure {
    assigned_loci: BTreeSet<String>,
    rows: Vec<Sys5I3SemanticRow>,
    semantic_row_digest_ref: String,
    opaque_digest_ref: String,
    opaque_cohort_ref: String,
}

impl Sys5I3RequiredLocalAuthorityClosure {
    pub const fn is_reference_only(&self) -> bool {
        true
    }

    pub fn is_exact_for_image(&self) -> bool {
        !self.rows.is_empty()
            && !self.semantic_row_digest_ref.is_empty()
            && self.opaque_digest_ref == self.recomputed_digest()
    }

    pub fn has_no_unassigned_semantic_rows(&self) -> bool {
        self.rows.iter().all(|row| match row {
            Sys5I3SemanticRow::Artifact { locus, .. } => self.assigned_loci.contains(locus),
            Sys5I3SemanticRow::IncidentEdge {
                source_locus,
                target_locus,
                ..
            } => {
                self.assigned_loci.contains(source_locus)
                    || self.assigned_loci.contains(target_locus)
            }
        })
    }

    pub fn opaque_digest_ref(&self) -> &str {
        &self.opaque_digest_ref
    }

    pub fn opaque_cohort_ref(&self) -> &str {
        &self.opaque_cohort_ref
    }

    fn recomputed_digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/sys5/i3/process-authority-closure/v1\\0");
        hasher.update(format!(
            "{:?}{:?}{}",
            self.assigned_loci, self.rows, self.semantic_row_digest_ref
        ));
        format!(
            "sys5-i3-process-authority-closure-sha256-v1:{:x}",
            hasher.finalize()
        )
    }
}

/// Observer-only child-seed view.  The executable seed stays private and is
/// limited to selected M8/M9 plans; this view makes the no-publisher and
/// no-full-program properties explicit without exposing those plans.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeChildSeed {
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_occurrence_ref: String,
    required_local_authority_closure: Sys5I3RequiredLocalAuthorityClosure,
}

impl Sys5I3ObserverSafeChildSeed {
    pub const fn carries_authority_publisher_or_issuer(&self) -> bool {
        false
    }

    pub const fn carries_full_prepared_admission(&self) -> bool {
        false
    }

    pub const fn carries_full_fabric_program(&self) -> bool {
        false
    }

    pub fn required_local_authority_closure(&self) -> &Sys5I3RequiredLocalAuthorityClosure {
        &self.required_local_authority_closure
    }

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub fn projection_ref(&self) -> &str {
        &self.projection_ref
    }

    pub fn m9_generation_ref(&self) -> &str {
        &self.m9_generation_ref
    }

    fn cohort_occurrence_ref(&self) -> &str {
        &self.cohort_occurrence_ref
    }
}

/// Observer-safe designated remote-input closure for one child image.  The
/// typed request/receipt tuples remain in the sealed SYS-4 projection; this
/// view exposes only exact-pair facts and an opaque identity.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3DesignatedRemoteInputClosure {
    request_receipt_pair_count: usize,
    distinct_operation_count: usize,
    pairs_are_distinguished_beyond_operation: bool,
    opaque_digest_ref: String,
}

impl Sys5I3DesignatedRemoteInputClosure {
    pub const fn is_reference_only(&self) -> bool {
        true
    }

    pub fn is_exact_for_image(&self) -> bool {
        !self.opaque_digest_ref.is_empty()
    }

    pub const fn is_derived_from_request_receipt_edges(&self) -> bool {
        true
    }

    pub const fn request_receipt_pair_count(&self) -> usize {
        self.request_receipt_pair_count
    }

    pub const fn distinct_operation_count(&self) -> usize {
        self.distinct_operation_count
    }

    pub const fn pairs_are_distinguished_beyond_operation(&self) -> bool {
        self.pairs_are_distinguished_beyond_operation
    }

    pub const fn is_symmetric_empty_for_image(&self) -> bool {
        self.request_receipt_pair_count == 0
            && self.distinct_operation_count == 0
            && !self.pairs_are_distinguished_beyond_operation
    }
}

/// Process-image-only tamper choices used by the I3-2 RED conformance test.
/// They mutate a detached candidate and cannot provide a production input.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sys5I3ProcessImageTamper {
    AppendForeignArtifact(Sys5I3ProcessArtifact),
    AppendForeignEdgeContract(Sys5I3RetainedEdgeContract),
    CorruptImageIntegrity,
    RemoveProjectedDesignatedRemoteInputRequirement,
    MismatchProjectedDesignatedRemoteInputRequestReceipt,
    RemoveOneRequiredSemanticBinding,
    AppendSemanticRowForUnassignedLocus,
    MismatchedParentCheckedProgramRef,
    MismatchedProjectionRef,
    MismatchedM9GenerationRef,
    MismatchedAuthorityClosureDigest,
    RemoveActualRestrictedOwnerBindingFromPrivateSeed,
    RemoveActualDesignatedRemoteInputLineageFromPrivateSeed,
    SubstituteSameLocusArtifactAndRecomputeIntegrity(Sys5I3ProcessArtifact),
    SubstituteSameIncidentEdgeAndRecomputeIntegrity(Sys5I3RetainedEdgeContract),
    DuplicateArtifactRowAndRecomputeIntegrity,
    DuplicateEdgeContractRowAndRecomputeIntegrity,
}

impl Sys5I3ProcessImageTamper {
    pub fn append_foreign_artifact(artifact: Sys5I3ProcessArtifact) -> Self {
        Self::AppendForeignArtifact(artifact)
    }

    pub fn append_foreign_edge_contract(contract: Sys5I3RetainedEdgeContract) -> Self {
        Self::AppendForeignEdgeContract(contract)
    }

    pub fn corrupt_image_integrity() -> Self {
        Self::CorruptImageIntegrity
    }

    pub fn remove_projected_designated_remote_input_requirement() -> Self {
        Self::RemoveProjectedDesignatedRemoteInputRequirement
    }

    pub fn mismatch_projected_designated_remote_input_request_receipt() -> Self {
        Self::MismatchProjectedDesignatedRemoteInputRequestReceipt
    }

    pub fn remove_one_required_semantic_binding() -> Self {
        Self::RemoveOneRequiredSemanticBinding
    }

    pub fn append_semantic_row_for_unassigned_locus() -> Self {
        Self::AppendSemanticRowForUnassignedLocus
    }

    pub fn mismatched_parent_checked_program_ref() -> Self {
        Self::MismatchedParentCheckedProgramRef
    }

    pub fn mismatched_projection_ref() -> Self {
        Self::MismatchedProjectionRef
    }

    pub fn mismatched_m9_generation_ref() -> Self {
        Self::MismatchedM9GenerationRef
    }

    pub fn mismatched_authority_closure_digest() -> Self {
        Self::MismatchedAuthorityClosureDigest
    }

    pub const fn remove_actual_restricted_owner_binding_from_private_seed() -> Self {
        Self::RemoveActualRestrictedOwnerBindingFromPrivateSeed
    }

    pub const fn remove_actual_designated_remote_input_lineage_from_private_seed() -> Self {
        Self::RemoveActualDesignatedRemoteInputLineageFromPrivateSeed
    }

    pub fn substitute_same_locus_artifact_and_recompute_integrity(
        artifact: Sys5I3ProcessArtifact,
    ) -> Self {
        Self::SubstituteSameLocusArtifactAndRecomputeIntegrity(artifact)
    }

    pub fn substitute_same_incident_edge_and_recompute_integrity(
        contract: Sys5I3RetainedEdgeContract,
    ) -> Self {
        Self::SubstituteSameIncidentEdgeAndRecomputeIntegrity(contract)
    }

    pub const fn duplicate_artifact_row_and_recompute_integrity() -> Self {
        Self::DuplicateArtifactRowAndRecomputeIntegrity
    }

    pub const fn duplicate_edge_contract_row_and_recompute_integrity() -> Self {
        Self::DuplicateEdgeContractRowAndRecomputeIntegrity
    }
}

struct Sys5I3PrivateRuntimeSeed {
    program: FabricProgram,
    admission: SealedFabricAdmission,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_occurrence_ref: String,
    private_snapshot_binding_ref: String,
}

/// An immutable process image.  Its private runtime seed is already reduced
/// to assigned loci and incident generated edges; the full project admission
/// and global program have been dropped before this value is returned.
#[doc(hidden)]
pub struct Sys5I3ProcessImage {
    slot_name: String,
    endpoint: String,
    assigned_loci: BTreeSet<String>,
    executable_artifacts: Vec<Sys5I3ProcessArtifact>,
    required_edge_contracts: Vec<Sys5I3RetainedEdgeContract>,
    designated_remote_input_closure: Sys5I3DesignatedRemoteInputClosure,
    child_seed: Sys5I3ObserverSafeChildSeed,
    private_runtime_seed: Sys5I3PrivateRuntimeSeed,
    private_integrity_ref: String,
}

impl std::fmt::Debug for Sys5I3ProcessImage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3ProcessImage")
            .field("slot_name", &self.slot_name)
            .field("assigned_loci", &self.assigned_loci)
            .field(
                "executable_artifact_count",
                &self.executable_artifacts.len(),
            )
            .field(
                "incident_edge_contract_count",
                &self.required_edge_contracts.len(),
            )
            .finish_non_exhaustive()
    }
}

/// Observer-safe construction facts for one process cohort.  The counts are
/// fixed by the only constructor below: the coordinator performs one full
/// prepare/admission and one M9 generation, then drops those full values
/// after it has derived every child image.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ProcessCohortSummary {
    full_admission_count: usize,
    authority_generation_count: usize,
    parent_checked_program_ref: String,
    projection_ref: String,
    activation_occurrence_ref: String,
    cohort_occurrence_ref: String,
}

/// A coordinator-retained, observer-safe expectation for exactly one child
/// bootstrap.  It contains no executable program, authority generation,
/// issuer, publisher, store, or source text.  The private child snapshot is
/// therefore never self-authorizing merely because it decodes successfully.
#[doc(hidden)]
#[derive(Debug, PartialEq, Eq)]
pub struct Sys5I3ExpectedStartBinding {
    slot_name: String,
    assigned_loci: BTreeSet<String>,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_provenance_ref: String,
    image_integrity_ref: String,
    private_snapshot_binding_ref: String,
}

impl Sys5I3ExpectedStartBinding {
    fn for_image(image: &Sys5I3ProcessImage) -> Self {
        Self {
            slot_name: image.slot_name.clone(),
            assigned_loci: image.assigned_loci.clone(),
            parent_checked_program_ref: image.child_seed.parent_checked_program_ref.clone(),
            projection_ref: image.child_seed.projection_ref.clone(),
            m9_generation_ref: image.child_seed.m9_generation_ref.clone(),
            cohort_provenance_ref: image
                .child_seed
                .required_local_authority_closure
                .opaque_cohort_ref()
                .to_string(),
            image_integrity_ref: image.private_integrity_ref.clone(),
            private_snapshot_binding_ref: image
                .private_runtime_seed
                .private_snapshot_binding_ref
                .clone(),
        }
    }

    fn validate_image(&self, image: &Sys5I3ProcessImage) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.cohort_provenance_ref
            != image
                .child_seed
                .required_local_authority_closure
                .opaque_cohort_ref()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
            ));
        }
        if self.parent_checked_program_ref != image.child_seed.parent_checked_program_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch,
            ));
        }
        if self.projection_ref != image.child_seed.projection_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch,
            ));
        }
        if self.m9_generation_ref != image.child_seed.m9_generation_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch,
            ));
        }
        if self.slot_name != image.slot_name
            || self.assigned_loci != image.assigned_loci
            || self.image_integrity_ref != image.private_integrity_ref
            || self.private_snapshot_binding_ref
                != image.private_runtime_seed.private_snapshot_binding_ref
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        Ok(())
    }
}

impl Sys5I3ProcessCohortSummary {
    pub const fn full_admission_count(&self) -> usize {
        self.full_admission_count
    }

    pub const fn authority_generation_count(&self) -> usize {
        self.authority_generation_count
    }

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub fn projection_ref(&self) -> &str {
        &self.projection_ref
    }

    pub fn activation_occurrence_ref(&self) -> &str {
        &self.activation_occurrence_ref
    }

    pub fn cohort_occurrence_ref(&self) -> &str {
        &self.cohort_occurrence_ref
    }
}

/// A one-shot coordinator result.  It holds only derived child images and
/// observer-safe construction facts; it never retains a full projected
/// program, prepared admission, or M9 authority publisher after construction.
#[doc(hidden)]
pub struct Sys5I3ProcessCohort {
    images: BTreeMap<String, Option<Sys5I3ProcessImage>>,
    expected_start_bindings: BTreeMap<String, Option<Sys5I3ExpectedStartBinding>>,
    summary: Sys5I3ProcessCohortSummary,
}

impl Sys5I3ProcessCohort {
    pub fn from_checked_project(
        project: &Sys5LocalProject,
        deployment: &Sys5I3Deployment,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        if deployment.parent_checked_program_ref != project.checked_program_identity_ref() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch,
            ));
        }
        if deployment.parent_projection_ref != project.i3_parent_projection_ref() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch,
            ));
        }
        let activation_occurrence_ref = fresh_occurrence_ref(
            "activation",
            &deployment.parent_checked_program_ref,
            &deployment.parent_projection_ref,
        );
        let cohort_occurrence_ref = fresh_occurrence_ref(
            "cohort",
            &deployment.parent_checked_program_ref,
            &deployment.parent_projection_ref,
        );

        // This is the sole full prepare/admission call for the cohort.  The
        // values are borrowed only while images are sealed below and are not
        // moved into `Self` or any child image.
        let prepared = project
            .prepare_canonical_local_st_admission()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let (coordinator_program, coordinator_admission) = prepared.into_parts_for_sys4();
        let mut images = BTreeMap::new();
        let mut expected_start_bindings = BTreeMap::new();
        for slot in &deployment.slots {
            let image = Sys5I3ProcessImage::from_coordinator_parts(
                project,
                deployment,
                &slot.slot_name,
                &coordinator_program,
                &coordinator_admission,
                &cohort_occurrence_ref,
            )?;
            let expected_start_binding = Sys5I3ExpectedStartBinding::for_image(&image);
            if images.insert(slot.slot_name.clone(), Some(image)).is_some() {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
                ));
            }
            if expected_start_bindings
                .insert(slot.slot_name.clone(), Some(expected_start_binding))
                .is_some()
            {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
                ));
            }
        }

        Ok(Self {
            images,
            expected_start_bindings,
            summary: Sys5I3ProcessCohortSummary {
                full_admission_count: 1,
                authority_generation_count: 1,
                parent_checked_program_ref: deployment.parent_checked_program_ref.clone(),
                projection_ref: deployment.parent_projection_ref.clone(),
                activation_occurrence_ref,
                cohort_occurrence_ref,
            },
        })
    }

    pub fn observer_safe_summary(&self) -> Sys5I3ProcessCohortSummary {
        self.summary.clone()
    }

    /// Retain the supervisor-controlled expectation before the child image
    /// is released.  It is deliberately derived from the already sealed
    /// image and contains only binding/provenance facts, never an authority
    /// seed or replacement admission path.
    pub fn parent_held_expected_start_binding(
        &mut self,
        slot_name: &str,
    ) -> Result<Sys5I3ExpectedStartBinding, Sys5I3ProcessRuntimeError> {
        let binding = self
            .expected_start_bindings
            .get_mut(slot_name)
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
            })?;
        binding.take().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken)
        })
    }

    /// Consume the one sealed image for a deployment slot.  Neither a cohort
    /// nor an image implements a duplication path, so a caller cannot start
    /// two owner runtimes from one derived local authority seed.
    pub fn take_process_image(
        &mut self,
        slot_name: &str,
    ) -> Result<Sys5I3ProcessImage, Sys5I3ProcessRuntimeError> {
        let image = self.images.get_mut(slot_name).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
        })?;
        image.take().ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ProcessImageAlreadyTaken)
        })
    }
}

impl Sys5I3ProcessImage {
    /// Construct one child image from the one coordinator-owned checked
    /// admission.  This is intentionally private: an image must not trigger
    /// a second full check/projection/admission or authority generation.
    fn from_coordinator_parts(
        project: &Sys5LocalProject,
        deployment: &Sys5I3Deployment,
        slot_name: &str,
        coordinator_program: &FabricProgram,
        coordinator_admission: &SealedFabricAdmission,
        cohort_occurrence_ref: &str,
    ) -> Result<Self, Sys5I3ProcessRuntimeError> {
        let slot = deployment.slot(slot_name).ok_or_else(|| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::UnknownDeploymentSlot)
        })?;
        let assigned_loci = slot.loci.iter().cloned().collect::<BTreeSet<_>>();
        let executable_artifacts = project
            .semantic_summary()
            .artifacts
            .iter()
            .filter(|artifact| assigned_loci.contains(&artifact.locus))
            .map(|artifact| Sys5I3ProcessArtifact {
                locus: artifact.locus.clone(),
                operation_id: artifact.operation_id.clone(),
                kind: artifact.kind.clone(),
                core_ref: artifact.core_ref.clone(),
                fragment_ref: artifact.fragment_ref.clone(),
                parent_checked_program_ref: artifact.checked_program_identity.clone(),
            })
            .collect::<Vec<_>>();
        // Retain every incident generated edge, including inbound contracts.
        // The receiver will still resolve an inbound carrier independently
        // from its sealed local projection; this inventory is provenance
        // evidence, never a sender-selected route.
        let required_edge_contracts = project
            .semantic_summary()
            .generated_communication
            .iter()
            .filter(|edge| {
                assigned_loci.contains(&edge.from_locus) || assigned_loci.contains(&edge.to_locus)
            })
            .map(|edge| {
                let core_ref = edge.core_ref.clone().ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                    )
                })?;
                Ok(Sys5I3RetainedEdgeContract {
                    source_locus: edge.from_locus.clone(),
                    target_locus: edge.to_locus.clone(),
                    edge_ref: edge.edge_ref.clone(),
                    operation_id: edge.operation_id.clone(),
                    kind: edge.kind.clone(),
                    core_ref,
                    source_artifact_ref: edge.source_fragment_ref.clone(),
                    target_artifact_ref: edge.target_fragment_ref.clone(),
                    parent_checked_program_ref: edge.checked_program_identity.clone(),
                })
            })
            .collect::<Result<Vec<_>, Sys5I3ProcessRuntimeError>>()?;

        // The full program/admission belong solely to the coordinator.  A
        // child receives the layer-owned subset produced from them, never a
        // clone of the global FabricProgram or Sys5PreparedAdmission.
        let program = coordinator_program
            .restricted_to_loci(&assigned_loci)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let admission = coordinator_admission
            .restricted_to_process_program(&program)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected,
                )
            })?;
        let parent_checked_program_ref = deployment.parent_checked_program_ref.clone();
        let projection_ref = deployment.parent_projection_ref.clone();
        let m9_generation_ref = admission.m9_generation_ref().to_string();
        let private_snapshot_binding_ref = private_runtime_seed_binding_ref(&program, &admission)
            .map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        let designated_inventory = program
            .i3_process_designated_remote_input_inventory()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                )
            })?;
        let designated_remote_input_closure = Sys5I3DesignatedRemoteInputClosure {
            request_receipt_pair_count: designated_inventory.request_receipt_pair_count(),
            distinct_operation_count: designated_inventory.distinct_operation_count(),
            pairs_are_distinguished_beyond_operation: designated_inventory
                .pairs_are_distinguished_beyond_operation(),
            opaque_digest_ref: designated_inventory.opaque_digest_ref(),
        };
        let closure = authority_closure_for_image(
            &assigned_loci,
            &executable_artifacts,
            &required_edge_contracts,
            &parent_checked_program_ref,
            &projection_ref,
            &m9_generation_ref,
            cohort_occurrence_ref,
            &admission.observer_safe_m9_semantic_row_sets_clone(),
        );
        let child_seed = Sys5I3ObserverSafeChildSeed {
            parent_checked_program_ref: parent_checked_program_ref.clone(),
            projection_ref: projection_ref.clone(),
            m9_generation_ref: m9_generation_ref.clone(),
            cohort_occurrence_ref: cohort_occurrence_ref.to_string(),
            required_local_authority_closure: closure,
        };
        let private_runtime_seed = Sys5I3PrivateRuntimeSeed {
            program,
            admission,
            parent_checked_program_ref,
            projection_ref,
            m9_generation_ref,
            cohort_occurrence_ref: cohort_occurrence_ref.to_string(),
            private_snapshot_binding_ref,
        };
        let mut image = Self {
            slot_name: slot.slot_name.clone(),
            endpoint: slot.endpoint.clone(),
            assigned_loci,
            executable_artifacts,
            required_edge_contracts,
            designated_remote_input_closure,
            child_seed,
            private_runtime_seed,
            private_integrity_ref: String::new(),
        };
        image.refresh_private_integrity();
        Ok(image)
    }

    pub fn assigned_loci(&self) -> Vec<String> {
        self.assigned_loci.iter().cloned().collect()
    }

    pub fn executable_artifacts(&self) -> &[Sys5I3ProcessArtifact] {
        &self.executable_artifacts
    }

    pub fn required_edge_contracts(&self) -> &[Sys5I3RetainedEdgeContract] {
        &self.required_edge_contracts
    }

    pub fn observer_safe_manifest(&self) -> Sys5I3ProcessImageManifest {
        Sys5I3ProcessImageManifest
    }

    pub fn observer_safe_child_seed(&self) -> &Sys5I3ObserverSafeChildSeed {
        &self.child_seed
    }

    pub fn observer_safe_designated_remote_input_closure(
        &self,
    ) -> &Sys5I3DesignatedRemoteInputClosure {
        &self.designated_remote_input_closure
    }

    pub fn into_test_only_tamper(mut self, tamper: Sys5I3ProcessImageTamper) -> Self {
        // The production type deliberately has no clone path.  The detached
        // falsifier consumes the sole image, so an invalid candidate cannot
        // leave a second startable owner image behind.
        match tamper {
            Sys5I3ProcessImageTamper::AppendForeignArtifact(artifact) => {
                self.executable_artifacts.push(artifact);
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::AppendForeignEdgeContract(contract) => {
                self.required_edge_contracts.push(contract);
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::CorruptImageIntegrity => {
                self.private_integrity_ref = "sys5-i3-corrupt-image-integrity".to_string();
            }
            Sys5I3ProcessImageTamper::RemoveProjectedDesignatedRemoteInputRequirement => {
                let _ = self
                    .private_runtime_seed
                    .program
                    .remove_i3_process_designated_requirement_for_test();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchProjectedDesignatedRemoteInputRequestReceipt => {
                let _ = self
                    .private_runtime_seed
                    .program
                    .mismatch_i3_process_designated_requirement_for_test();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::RemoveOneRequiredSemanticBinding => {
                self.child_seed.required_local_authority_closure.rows.pop();
                self.child_seed
                    .required_local_authority_closure
                    .opaque_digest_ref = self
                    .child_seed
                    .required_local_authority_closure
                    .recomputed_digest();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::AppendSemanticRowForUnassignedLocus => {
                self.child_seed.required_local_authority_closure.rows.push(
                    Sys5I3SemanticRow::Artifact {
                        locus: "i3-unassigned-locus".to_string(),
                        fragment_ref: "i3-foreign-semantic-row".to_string(),
                    },
                );
                self.child_seed
                    .required_local_authority_closure
                    .opaque_digest_ref = self
                    .child_seed
                    .required_local_authority_closure
                    .recomputed_digest();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedParentCheckedProgramRef => {
                self.child_seed.parent_checked_program_ref =
                    "sys5-i3-mismatched-parent-program".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedProjectionRef => {
                self.child_seed.projection_ref = "sys5-i3-mismatched-projection".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedM9GenerationRef => {
                self.child_seed.m9_generation_ref = "sys5-i3-mismatched-m9-generation".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::MismatchedAuthorityClosureDigest => {
                self.child_seed
                    .required_local_authority_closure
                    .opaque_digest_ref = "sys5-i3-mismatched-authority-closure".to_string();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::SubstituteSameLocusArtifactAndRecomputeIntegrity(
                artifact,
            ) => {
                if let Some(row) = self
                    .executable_artifacts
                    .iter_mut()
                    .find(|row| row.locus == artifact.locus)
                {
                    *row = artifact;
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::SubstituteSameIncidentEdgeAndRecomputeIntegrity(contract) => {
                if let Some(row) = self.required_edge_contracts.iter_mut().find(|row| {
                    row.source_locus == contract.source_locus
                        && row.target_locus == contract.target_locus
                }) {
                    *row = contract;
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::DuplicateArtifactRowAndRecomputeIntegrity => {
                if let Some(row) = self.executable_artifacts.first().cloned() {
                    self.executable_artifacts.push(row);
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::DuplicateEdgeContractRowAndRecomputeIntegrity => {
                if let Some(row) = self.required_edge_contracts.first().cloned() {
                    self.required_edge_contracts.push(row);
                }
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::RemoveActualRestrictedOwnerBindingFromPrivateSeed => {
                let _ = self
                    .private_runtime_seed
                    .admission
                    .remove_actual_restricted_owner_binding_for_i3_process_test();
                self.refresh_private_integrity();
            }
            Sys5I3ProcessImageTamper::RemoveActualDesignatedRemoteInputLineageFromPrivateSeed => {
                let _ = self
                    .private_runtime_seed
                    .admission
                    .remove_actual_designated_remote_input_lineage_for_i3_process_test();
                self.refresh_private_integrity();
            }
        }
        self
    }

    fn refresh_private_integrity(&mut self) {
        self.private_integrity_ref = self.recomputed_private_integrity();
    }

    fn recomputed_private_integrity(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/sys5/i3/process-image-integrity/v1\\0");
        hasher.update(format!(
            "{}{}{:?}{:?}{:?}{:?}{:?}",
            self.slot_name,
            self.endpoint,
            self.assigned_loci,
            self.executable_artifacts,
            self.required_edge_contracts,
            self.designated_remote_input_closure,
            self.child_seed,
        ));
        format!("sys5-i3-process-image-sha256-v1:{:x}", hasher.finalize())
    }

    fn validate_before_start(&self) -> Result<(), Sys5I3ProcessRuntimeError> {
        if self.private_integrity_ref != self.recomputed_private_integrity() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        if self
            .private_runtime_seed
            .program
            .validate_i3_process_designated_requirements()
            .is_err()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
            ));
        }
        if self
            .executable_artifacts
            .iter()
            .any(|artifact| !self.assigned_loci.contains(&artifact.locus))
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ForeignArtifact,
            ));
        }
        if self.required_edge_contracts.iter().any(|contract| {
            !self.assigned_loci.contains(&contract.source_locus)
                && !self.assigned_loci.contains(&contract.target_locus)
        }) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ForeignEdgeContract,
            ));
        }
        if self.child_seed.parent_checked_program_ref
            != self.private_runtime_seed.parent_checked_program_ref
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortParentProgramMismatch,
            ));
        }
        if self.child_seed.projection_ref != self.private_runtime_seed.projection_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProjectionMismatch,
            ));
        }
        if self.child_seed.m9_generation_ref != self.private_runtime_seed.m9_generation_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch,
            ));
        }
        if self.child_seed.cohort_occurrence_ref()
            != self.private_runtime_seed.cohort_occurrence_ref
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortM9GenerationMismatch,
            ));
        }
        let (expected_artifacts, expected_edges) = self
            .private_runtime_seed
            .program
            .i3_process_normalized_inventory_refs();
        let actual_artifacts = self
            .executable_artifacts
            .iter()
            .map(normalized_artifact_inventory_ref)
            .collect::<BTreeSet<_>>();
        let actual_edges = self
            .required_edge_contracts
            .iter()
            .map(normalized_edge_inventory_ref)
            .collect::<BTreeSet<_>>();
        let artifact_parent_matches = self.executable_artifacts.iter().all(|artifact| {
            artifact.parent_checked_program_ref == self.child_seed.parent_checked_program_ref
        });
        let edge_parent_matches = self.required_edge_contracts.iter().all(|contract| {
            contract.parent_checked_program_ref == self.child_seed.parent_checked_program_ref
        });
        if !artifact_parent_matches
            || !edge_parent_matches
            || actual_artifacts.len() != self.executable_artifacts.len()
            || actual_edges.len() != self.required_edge_contracts.len()
            || actual_artifacts != expected_artifacts
            || actual_edges != expected_edges
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageInventoryProvenanceMismatch,
            ));
        }
        let designated_inventory = self
            .private_runtime_seed
            .program
            .i3_process_designated_remote_input_inventory()
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                )
            })?;
        let expected_designated_closure = Sys5I3DesignatedRemoteInputClosure {
            request_receipt_pair_count: designated_inventory.request_receipt_pair_count(),
            distinct_operation_count: designated_inventory.distinct_operation_count(),
            pairs_are_distinguished_beyond_operation: designated_inventory
                .pairs_are_distinguished_beyond_operation(),
            opaque_digest_ref: designated_inventory.opaque_digest_ref(),
        };
        if self.designated_remote_input_closure != expected_designated_closure {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
            ));
        }
        let closure = &self.child_seed.required_local_authority_closure;
        if !closure.has_no_unassigned_semantic_rows() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ForeignAuthorityEvidence,
            ));
        }
        if closure.opaque_digest_ref != closure.recomputed_digest() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::AuthorityClosureDigestMismatch,
            ));
        }
        let expected = authority_closure_for_image(
            &self.assigned_loci,
            &self.executable_artifacts,
            &self.required_edge_contracts,
            &self.child_seed.parent_checked_program_ref,
            &self.child_seed.projection_ref,
            &self.child_seed.m9_generation_ref,
            self.child_seed.cohort_occurrence_ref(),
            &self
                .private_runtime_seed
                .admission
                .observer_safe_m9_semantic_row_sets_clone(),
        );
        if closure != &expected {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::MissingRequiredAuthorityEvidence,
            ));
        }
        Ok(())
    }
}

/// Observer-safe image manifest.  It intentionally makes the non-carriage
/// guarantees explicit rather than exporting image internals.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ProcessImageManifest;

impl Sys5I3ProcessImageManifest {
    pub const fn carries_source_text(&self) -> bool {
        false
    }

    pub const fn carries_host_path(&self) -> bool {
        false
    }

    pub const fn carries_expected_result(&self) -> bool {
        false
    }
}

fn normalized_artifact_inventory_ref(artifact: &Sys5I3ProcessArtifact) -> String {
    format!(
        "artifact|{}|{}|{}|{}|{}",
        artifact.locus,
        artifact.operation_id,
        artifact.kind,
        artifact.core_ref,
        artifact.fragment_ref
    )
}

fn normalized_edge_inventory_ref(contract: &Sys5I3RetainedEdgeContract) -> String {
    format!(
        "edge|{}|{}|{}|{}|{}|{}|{}|{}",
        contract.source_locus,
        contract.target_locus,
        contract.operation_id,
        contract.kind,
        contract.edge_ref,
        contract.core_ref,
        contract.source_artifact_ref,
        contract.target_artifact_ref,
    )
}

#[allow(clippy::too_many_arguments)]
fn authority_closure_for_image(
    assigned_loci: &BTreeSet<String>,
    artifacts: &[Sys5I3ProcessArtifact],
    contracts: &[Sys5I3RetainedEdgeContract],
    parent_checked_program_ref: &str,
    projection_ref: &str,
    m9_generation_ref: &str,
    cohort_occurrence_ref: &str,
    semantic_rows: &ObserverSafeM9SemanticRowSets,
) -> Sys5I3RequiredLocalAuthorityClosure {
    let mut rows = artifacts
        .iter()
        .map(|artifact| Sys5I3SemanticRow::Artifact {
            locus: artifact.locus.clone(),
            fragment_ref: artifact.fragment_ref.clone(),
        })
        .chain(
            contracts
                .iter()
                .map(|contract| Sys5I3SemanticRow::IncidentEdge {
                    source_locus: contract.source_locus.clone(),
                    target_locus: contract.target_locus.clone(),
                    edge_ref: contract.edge_ref.clone(),
                }),
        )
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| format!("{row:?}"));
    let mut cohort_hasher = Sha256::new();
    cohort_hasher.update(b"mirrorea/sys5/i3/process-cohort/v1\\0");
    cohort_hasher.update(parent_checked_program_ref);
    cohort_hasher.update(projection_ref);
    cohort_hasher.update(m9_generation_ref);
    cohort_hasher.update(cohort_occurrence_ref);
    let opaque_cohort_ref = format!(
        "sys5-i3-process-cohort-sha256-v1:{:x}",
        cohort_hasher.finalize()
    );
    let mut closure = Sys5I3RequiredLocalAuthorityClosure {
        assigned_loci: assigned_loci.clone(),
        rows,
        semantic_row_digest_ref: observer_safe_semantic_rows_digest(semantic_rows),
        opaque_digest_ref: String::new(),
        opaque_cohort_ref,
    };
    closure.opaque_digest_ref = closure.recomputed_digest();
    closure
}

fn fresh_occurrence_ref(
    domain: &str,
    parent_checked_program_ref: &str,
    projection_ref: &str,
) -> String {
    let ordinal = NEXT_PROCESS_COHORT_OCCURRENCE.fetch_add(1, Ordering::Relaxed);
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/local-occurrence/v1\\0");
    hasher.update(domain);
    hasher.update(parent_checked_program_ref);
    hasher.update(projection_ref);
    hasher.update(ordinal.to_le_bytes());
    format!(
        "sys5-i3-{domain}-occurrence-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn observer_safe_semantic_rows_digest(rows: &ObserverSafeM9SemanticRowSets) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/process-authority-semantic-rows/v1\\0");
    hasher.update(format!("{rows:?}"));
    format!(
        "sys5-i3-process-authority-semantic-rows-sha256-v1:{:x}",
        hasher.finalize()
    )
}

/// Bind both independently restored private roots.  The sealed admission
/// commitment covers M8/M9/state facts; the projection commitment covers the
/// complete restricted executable structure rather than merely its route
/// fingerprint.  No source text, transport identity, or new authority enters
/// this value.
fn private_runtime_seed_binding_ref(
    program: &FabricProgram,
    admission: &SealedFabricAdmission,
) -> Result<String, ()> {
    let projection_binding = program.i3_private_projection_binding_ref()?;
    let admission_binding = admission.i3_private_snapshot_binding_ref();
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/private-runtime-seed-binding/v1\0");
    hasher.update(projection_binding);
    hasher.update(admission_binding);
    Ok(format!(
        "sys5-i3-private-runtime-seed-binding-sha256-v1:{:x}",
        hasher.finalize()
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sys5I3ProcessMessageKind {
    Request,
    Reply,
    Receipt,
}

/// One generated carrier moving by value between two process runtimes.  It
/// has no socket, connection, session, or certificate binding in G1.
#[doc(hidden)]
#[derive(Debug)]
pub struct Sys5I3ProcessMessage {
    kind: Sys5I3ProcessMessageKind,
    carrier: Option<Sys4ProcessCarrier>,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    // Private admission provenance.  This binds a generated carrier to the
    // cohort occurrence which sealed its process images; it is neither an
    // authority fact nor a substitute for M9 validation.
    cohort_provenance_ref: String,
    identity_basis: Sys5I3ObserverSafeIdentityBasis,
}

impl Sys5I3ProcessMessage {
    pub const fn transport_binding(&self) -> Option<()> {
        None
    }

    pub fn semantic_request_identity_ref(&self) -> &str {
        &self.semantic_request_identity_ref
    }

    pub fn linked_request_identity_ref(&self) -> Option<&str> {
        self.linked_request_identity_ref.as_deref()
    }

    pub const fn is_observer_safe_typed_result_or_receipt(&self) -> bool {
        matches!(self.kind, Sys5I3ProcessMessageKind::Receipt)
    }

    pub fn has_no_transportable_carrier(&self) -> bool {
        matches!(self.kind, Sys5I3ProcessMessageKind::Receipt) && self.carrier.is_none()
    }

    pub const fn observer_safe_identity_basis(&self) -> Sys5I3ObserverSafeIdentityBasis {
        self.identity_basis
    }
}

/// Declared bounded limits for the private G2 codec.  They are an internal
/// fail-closed resource boundary, not a public wire-format commitment.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3PrivateProcessCodecLimits {
    max_image_bytes: usize,
    max_message_bytes: usize,
}

impl Sys5I3PrivateProcessCodecLimits {
    pub const fn max_image_bytes(&self) -> usize {
        self.max_image_bytes
    }

    pub const fn max_message_bytes(&self) -> usize {
        self.max_message_bytes
    }
}

/// Typed rejection classes for the private I3-2 codec.  These names and the
/// JSON representation are provisional implementation detail only.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys5I3PrivateProcessCodecErrorKind {
    Malformed,
    Incomplete,
    Oversized,
    UnknownVersion,
    MissingRequiredCoreProvenance,
    ReceiptIsLocalOnly,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3PrivateProcessCodecError {
    kind: Sys5I3PrivateProcessCodecErrorKind,
}

impl Sys5I3PrivateProcessCodecError {
    fn new(kind: Sys5I3PrivateProcessCodecErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> Sys5I3PrivateProcessCodecErrorKind {
        self.kind
    }
}

/// A length-prefixed, private codec for the one-shot I3 child image and the
/// generated owner request/reply carrier.  It accepts only exact restricted
/// snapshots; decode returns untrusted candidates that cannot start a child
/// or enter a mailbox without a receiver-owned validation boundary.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sys5I3PrivateProcessCodec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProcessMessageEnvelope {
    version: u64,
    message: PrivateProcessMessageSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateProcessMessageSnapshot {
    kind: PrivateProcessMessageKind,
    carrier: Sys4I3PrivateProcessCarrierSnapshot,
    semantic_request_identity_ref: String,
    linked_request_identity_ref: Option<String>,
    cohort_provenance_ref: String,
}

/// Parse one untrusted JSON value without normalizing duplicate object keys.
/// `serde_json::Value` otherwise accepts duplicate members with last-write
/// wins semantics, which would erase the malformed input before the strict
/// private DTO boundary can reject it.
struct StrictJsonValue(serde_json::Value);

impl<'de> Deserialize<'de> for StrictJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictJsonValueVisitor)
    }
}

struct StrictJsonValueVisitor;

impl<'de> Visitor<'de> for StrictJsonValueVisitor {
    type Value = StrictJsonValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("strict JSON without duplicate object members")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Bool(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Number(value.into())))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .map(StrictJsonValue)
            .ok_or_else(|| E::custom("JSON number must be finite"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::String(
            value.to_string(),
        )))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::String(value)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(StrictJsonValue(serde_json::Value::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StrictJsonValue::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictJsonValue>()? {
            values.push(value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Array(values)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = serde_json::Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(A::Error::custom("duplicate JSON object member"));
            }
            let value = map.next_value::<StrictJsonValue>()?;
            values.insert(key, value.0);
        }
        Ok(StrictJsonValue(serde_json::Value::Object(values)))
    }
}

fn strict_json_value(bytes: &[u8]) -> Result<serde_json::Value, ()> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictJsonValue::deserialize(&mut deserializer).map_err(|_| ())?;
    deserializer.end().map_err(|_| ())?;
    Ok(value.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PrivateProcessMessageKind {
    Request,
    Reply,
}

/// Codec-decoded image that has no direct runtime constructor.  Its only
/// promotion path is `validate_and_start_image`, which requires the T0
/// coordinator's separately retained expected binding.
#[doc(hidden)]
pub struct Sys5I3UntrustedProcessImage {
    image: Sys5I3ProcessImage,
}

impl std::fmt::Debug for Sys5I3UntrustedProcessImage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3UntrustedProcessImage")
            .field("assigned_loci", &self.image.assigned_loci)
            .field(
                "executable_artifact_count",
                &self.image.executable_artifacts.len(),
            )
            .field(
                "incident_edge_contract_count",
                &self.image.required_edge_contracts.len(),
            )
            .finish_non_exhaustive()
    }
}

/// Observer-safe facts from a decoded image.  It deliberately omits the
/// executable projection/admission values and every raw M8/M9 datum.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3UntrustedProcessImageManifest {
    assigned_loci: Vec<String>,
    has_assigned_artifacts_only: bool,
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_provenance_ref: String,
}

impl Sys5I3UntrustedProcessImage {
    pub fn observer_safe_manifest(&self) -> Sys5I3UntrustedProcessImageManifest {
        Sys5I3UntrustedProcessImageManifest {
            assigned_loci: self.image.assigned_loci(),
            has_assigned_artifacts_only: self
                .image
                .executable_artifacts
                .iter()
                .all(|artifact| self.image.assigned_loci.contains(&artifact.locus)),
            parent_checked_program_ref: self.image.child_seed.parent_checked_program_ref.clone(),
            projection_ref: self.image.child_seed.projection_ref.clone(),
            m9_generation_ref: self.image.child_seed.m9_generation_ref.clone(),
            cohort_provenance_ref: self
                .image
                .child_seed
                .required_local_authority_closure
                .opaque_cohort_ref()
                .to_string(),
        }
    }
}

impl Sys5I3UntrustedProcessImageManifest {
    pub fn assigned_loci(&self) -> Vec<String> {
        self.assigned_loci.clone()
    }

    pub const fn has_assigned_artifacts_only(&self) -> bool {
        self.has_assigned_artifacts_only
    }

    pub fn parent_checked_program_ref(&self) -> &str {
        &self.parent_checked_program_ref
    }

    pub fn projection_ref(&self) -> &str {
        &self.projection_ref
    }

    pub fn m9_generation_ref(&self) -> &str {
        &self.m9_generation_ref
    }

    pub fn cohort_provenance_ref(&self) -> &str {
        &self.cohort_provenance_ref
    }

    pub const fn carries_source_text(&self) -> bool {
        false
    }

    pub const fn carries_host_path(&self) -> bool {
        false
    }

    pub const fn carries_expected_result(&self) -> bool {
        false
    }
}

/// Codec-decoded carrier candidate.  It contains private bytes-derived facts
/// only and cannot call `accept_inbound` until the target runtime resolves
/// them against its local image.
#[doc(hidden)]
pub struct Sys5I3UntrustedProcessMessage {
    message: PrivateProcessMessageSnapshot,
}

impl std::fmt::Debug for Sys5I3UntrustedProcessMessage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3UntrustedProcessMessage")
            .field("kind", &self.message.kind)
            .finish_non_exhaustive()
    }
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3UntrustedProcessMessageManifest {
    is_request: bool,
    is_reply: bool,
}

impl Sys5I3UntrustedProcessMessage {
    pub const fn observer_safe_manifest(&self) -> Sys5I3UntrustedProcessMessageManifest {
        Sys5I3UntrustedProcessMessageManifest {
            is_request: matches!(self.message.kind, PrivateProcessMessageKind::Request),
            is_reply: matches!(self.message.kind, PrivateProcessMessageKind::Reply),
        }
    }
}

impl Sys5I3UntrustedProcessMessageManifest {
    pub const fn is_request(&self) -> bool {
        self.is_request
    }

    pub const fn is_reply(&self) -> bool {
        self.is_reply
    }
}

impl Sys5I3PrivateProcessCodec {
    // The exact restricted M8/M9 closure is intentionally carried rather
    // than recomputed in the child.  Keep a finite private bound large enough
    // for the accepted four-locus profile while still rejecting allocation
    // growth before JSON decoding.
    const MAX_IMAGE_BYTES: usize = 8 << 20;
    const MAX_MESSAGE_BYTES: usize = 1 << 16;

    pub const fn private_provisional_v1() -> Self {
        Self
    }

    pub const fn limits(&self) -> Sys5I3PrivateProcessCodecLimits {
        Sys5I3PrivateProcessCodecLimits {
            max_image_bytes: Self::MAX_IMAGE_BYTES,
            max_message_bytes: Self::MAX_MESSAGE_BYTES,
        }
    }

    /// Consume the only startable image.  The returned bytes are untrusted
    /// delivery material; no image value remains available to start directly.
    pub fn encode_image(
        &self,
        image: Sys5I3ProcessImage,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let snapshot =
            process_snapshot::PrivateProcessImageSnapshot::from_image(image).map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        let body = serde_json::to_vec(&snapshot).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_IMAGE_BYTES)
    }

    pub fn decode_untrusted_image(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3UntrustedProcessImage, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_IMAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.validate_image_json_shape(&value)?;
        let snapshot: process_snapshot::PrivateProcessImageSnapshot = serde_json::from_value(value)
            .map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        let image = snapshot.into_untrusted_image().map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        Ok(Sys5I3UntrustedProcessImage { image })
    }

    pub fn validate_and_start_image(
        &self,
        candidate: Sys5I3UntrustedProcessImage,
        expected: Sys5I3ExpectedStartBinding,
    ) -> Result<Sys5I3ProcessRuntime, Sys5I3ProcessRuntimeError> {
        expected.validate_image(&candidate.image)?;
        Sys5I3ProcessRuntime::start(candidate.image)
    }

    pub fn encode_outbound_message(
        &self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let kind = match message.kind {
            Sys5I3ProcessMessageKind::Request => PrivateProcessMessageKind::Request,
            Sys5I3ProcessMessageKind::Reply => PrivateProcessMessageKind::Reply,
            Sys5I3ProcessMessageKind::Receipt => {
                return Err(Sys5I3PrivateProcessCodecError::new(
                    Sys5I3PrivateProcessCodecErrorKind::ReceiptIsLocalOnly,
                ));
            }
        };
        let carrier = message.carrier.as_ref().ok_or_else(|| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let carrier = carrier.i3_private_process_snapshot().map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        let envelope = PrivateProcessMessageEnvelope {
            version: process_snapshot::PRIVATE_PROCESS_SNAPSHOT_VERSION,
            message: PrivateProcessMessageSnapshot {
                kind,
                carrier,
                semantic_request_identity_ref: message.semantic_request_identity_ref,
                linked_request_identity_ref: message.linked_request_identity_ref,
                cohort_provenance_ref: message.cohort_provenance_ref,
            },
        };
        let body = serde_json::to_vec(&envelope).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.frame_body(body, Self::MAX_MESSAGE_BYTES)
    }

    pub fn decode_untrusted_message(
        &self,
        bytes: &[u8],
    ) -> Result<Sys5I3UntrustedProcessMessage, Sys5I3PrivateProcessCodecError> {
        let body = self.unframe_body(bytes, Self::MAX_MESSAGE_BYTES)?;
        let value = strict_json_value(body).map_err(|_| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
        })?;
        self.validate_version(&value)?;
        let envelope: PrivateProcessMessageEnvelope =
            serde_json::from_value(value).map_err(|_| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if envelope.message.semantic_request_identity_ref.is_empty()
            || envelope.message.cohort_provenance_ref.is_empty()
        {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        Ok(Sys5I3UntrustedProcessMessage {
            message: envelope.message,
        })
    }

    fn frame_body(
        &self,
        body: Vec<u8>,
        limit: usize,
    ) -> Result<Vec<u8>, Sys5I3PrivateProcessCodecError> {
        let total = body.len().checked_add(4).ok_or_else(|| {
            Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Oversized)
        })?;
        if total > limit || body.len() > u32::MAX as usize {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Oversized,
            ));
        }
        let mut framed = (body.len() as u32).to_be_bytes().to_vec();
        framed.extend(body);
        Ok(framed)
    }

    fn unframe_body<'a>(
        &self,
        bytes: &'a [u8],
        limit: usize,
    ) -> Result<&'a [u8], Sys5I3PrivateProcessCodecError> {
        if bytes.len() > limit {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Oversized,
            ));
        }
        if bytes.len() < 4 {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Incomplete,
            ));
        }
        let declared = u32::from_be_bytes(bytes[..4].try_into().expect("fixed prefix")) as usize;
        let available = bytes.len() - 4;
        if declared > limit.saturating_sub(4) {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        if available < declared {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Incomplete,
            ));
        }
        if available != declared {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        }
        Ok(&bytes[4..])
    }

    fn validate_version(
        &self,
        value: &serde_json::Value,
    ) -> Result<(), Sys5I3PrivateProcessCodecError> {
        let version = value
            .get("version")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| {
                Sys5I3PrivateProcessCodecError::new(Sys5I3PrivateProcessCodecErrorKind::Malformed)
            })?;
        if version != process_snapshot::PRIVATE_PROCESS_SNAPSHOT_VERSION {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::UnknownVersion,
            ));
        }
        Ok(())
    }

    fn validate_image_json_shape(
        &self,
        value: &serde_json::Value,
    ) -> Result<(), Sys5I3PrivateProcessCodecError> {
        self.validate_version(value)?;
        let Some(edges) = value
            .pointer("/image/required_edge_contracts")
            .and_then(serde_json::Value::as_array)
        else {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::Malformed,
            ));
        };
        if edges.iter().any(|edge| {
            edge.get("core_ref")
                .and_then(serde_json::Value::as_str)
                .is_none_or(str::is_empty)
        }) {
            return Err(Sys5I3PrivateProcessCodecError::new(
                Sys5I3PrivateProcessCodecErrorKind::MissingRequiredCoreProvenance,
            ));
        }
        Ok(())
    }
}

/// The explicit semantic identity basis for local images and generated
/// carrier lineage.  Process IDs, addresses, sessions, and future transport
/// attempts deliberately do not participate.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeIdentityBasis;

impl Sys5I3ObserverSafeIdentityBasis {
    pub const fn includes_checked_program_ref(&self) -> bool {
        true
    }
    pub const fn includes_projection_ref(&self) -> bool {
        true
    }
    pub const fn includes_cohort_ref(&self) -> bool {
        true
    }
    pub const fn includes_logical_origin_ref(&self) -> bool {
        true
    }
    pub const fn includes_ordinal(&self) -> bool {
        true
    }
    pub const fn includes_process_id(&self) -> bool {
        false
    }
    pub const fn includes_network_identity(&self) -> bool {
        false
    }
}

/// Observer-safe pending-carrier facts for the pre-transport runtime.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeOutboxSummary {
    pending_carrier_count: usize,
    generated_owner_operations: BTreeSet<String>,
}

impl Sys5I3ObserverSafeOutboxSummary {
    pub const fn pending_carrier_count(&self) -> usize {
        self.pending_carrier_count
    }

    pub fn contains_generated_owner_request(&self, operation_id: &str) -> bool {
        self.generated_owner_operations.contains(operation_id)
    }
}

/// Observer-safe proof that a started child retains no coordinator-only
/// authority publisher, full admission, or global FabricProgram.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sys5I3ObserverSafeRuntimeSummary {
    served_owner_request_count: usize,
    actual_owner_write_count: usize,
    accepted_inbound_receipt_count: usize,
}

/// Exact, observer-safe semantic occurrence references retained by the
/// process runtime.  These are recorded from accepted SYS-4 steps; they are
/// deliberately not inferred from the aggregate counters above.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sys5I3ObserverSafeSemanticOccurrences {
    owner_serve_linearizations: BTreeMap<String, String>,
    actual_owner_writes: BTreeMap<String, String>,
    requester_local_receipts: BTreeMap<String, String>,
}

impl Sys5I3ObserverSafeSemanticOccurrences {
    pub fn owner_serve_linearization_occurrence_ref(
        &self,
        request_identity_ref: &str,
    ) -> Option<&str> {
        self.owner_serve_linearizations
            .get(request_identity_ref)
            .map(String::as_str)
    }

    pub fn actual_owner_write_occurrence_ref(&self, request_identity_ref: &str) -> Option<&str> {
        self.actual_owner_writes
            .get(request_identity_ref)
            .map(String::as_str)
    }

    pub fn requester_local_receipt_occurrence_ref(
        &self,
        request_identity_ref: &str,
    ) -> Option<&str> {
        self.requester_local_receipts
            .get(request_identity_ref)
            .map(String::as_str)
    }
}

impl Sys5I3ObserverSafeRuntimeSummary {
    pub const fn carries_authority_publisher_or_issuer(&self) -> bool {
        false
    }

    pub const fn carries_full_admission_or_fabric_program(&self) -> bool {
        false
    }

    pub const fn served_owner_request_count(&self) -> usize {
        self.served_owner_request_count
    }

    pub const fn actual_owner_write_count(&self) -> usize {
        self.actual_owner_write_count
    }

    pub const fn accepted_inbound_receipt_count(&self) -> usize {
        self.accepted_inbound_receipt_count
    }
}

/// Started process-local runtime.  It owns only selected loci and an
/// independent local fabric/store; it has no access to another image's
/// stores, artifacts, publisher, or full projection.
#[doc(hidden)]
pub struct Sys5I3ProcessRuntime {
    assigned_loci: BTreeSet<String>,
    local_store_identity_ref: String,
    identity_basis: Sys5I3ObserverSafeIdentityBasis,
    parent_checked_program_ref: String,
    projection_ref: String,
    cohort_ref: String,
    fabric: LocalFabric,
    local_authoritative_mutation_count: usize,
    served_owner_request_count: usize,
    accepted_inbound_receipt_count: usize,
    semantic_occurrences: Sys5I3ObserverSafeSemanticOccurrences,
    // A requester-local claim for one emitted owner request.  It contains
    // only receiver-owned, source-derived route/provenance facts; it is not
    // a transport session, credential, or mutable remote-store handle.
    pending_outbound_owner_requests: BTreeMap<String, Sys4I3PendingOwnerRequestBinding>,
    reject_next_outbound_extraction: bool,
}

impl std::fmt::Debug for Sys5I3ProcessRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Sys5I3ProcessRuntime")
            .field("assigned_loci", &self.assigned_loci)
            .field("local_store_identity_ref", &self.local_store_identity_ref)
            .field(
                "local_authoritative_mutation_count",
                &self.local_authoritative_mutation_count,
            )
            .field(
                "served_owner_request_count",
                &self.served_owner_request_count,
            )
            .finish_non_exhaustive()
    }
}

impl Sys5I3ProcessRuntime {
    pub fn start(image: Sys5I3ProcessImage) -> Result<Self, Sys5I3ProcessRuntimeError> {
        image.validate_before_start()?;
        let parent_checked_program_ref = image.child_seed.parent_checked_program_ref.clone();
        let projection_ref = image.child_seed.projection_ref.clone();
        let cohort_ref = image
            .child_seed
            .required_local_authority_closure
            .opaque_cohort_ref()
            .to_string();
        let logical_origin_ref = logical_origin_ref(
            &image.slot_name,
            &image.assigned_loci,
            &parent_checked_program_ref,
            &projection_ref,
            &cohort_ref,
        );
        let local_store_identity_ref = process_store_identity_ref(
            &parent_checked_program_ref,
            &projection_ref,
            &cohort_ref,
            &logical_origin_ref,
            0,
        );
        let fabric = LocalFabric::bootstrap(
            image.private_runtime_seed.program,
            image.private_runtime_seed.admission,
            BackendProfile::St,
        )
        .map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        Ok(Self {
            assigned_loci: image.assigned_loci,
            local_store_identity_ref,
            identity_basis: Sys5I3ObserverSafeIdentityBasis,
            parent_checked_program_ref,
            projection_ref,
            cohort_ref,
            fabric,
            local_authoritative_mutation_count: 0,
            served_owner_request_count: 0,
            accepted_inbound_receipt_count: 0,
            semantic_occurrences: Sys5I3ObserverSafeSemanticOccurrences::default(),
            pending_outbound_owner_requests: BTreeMap::new(),
            reject_next_outbound_extraction: false,
        })
    }

    pub fn local_store_identity_ref(&self) -> &str {
        &self.local_store_identity_ref
    }

    pub const fn observer_safe_runtime_summary(&self) -> Sys5I3ObserverSafeRuntimeSummary {
        Sys5I3ObserverSafeRuntimeSummary {
            served_owner_request_count: self.served_owner_request_count,
            actual_owner_write_count: self.local_authoritative_mutation_count,
            accepted_inbound_receipt_count: self.accepted_inbound_receipt_count,
        }
    }

    pub const fn local_authoritative_mutation_count(&self) -> usize {
        self.local_authoritative_mutation_count
    }

    pub const fn observer_safe_store_identity_basis(&self) -> Sys5I3ObserverSafeIdentityBasis {
        self.identity_basis
    }

    pub fn observer_safe_semantic_occurrences(&self) -> Sys5I3ObserverSafeSemanticOccurrences {
        self.semantic_occurrences.clone()
    }

    pub fn observer_safe_outbox_summary(&self) -> Sys5I3ObserverSafeOutboxSummary {
        let summary = self.fabric.i3_process_outbox_summary();
        Sys5I3ObserverSafeOutboxSummary {
            pending_carrier_count: summary.pending_carrier_count(),
            generated_owner_operations: summary.generated_owner_operations_for_sys5(),
        }
    }

    pub fn test_only_reject_next_outbound_extraction(&mut self) {
        self.reject_next_outbound_extraction = true;
    }

    pub fn emit_generated_owner_request(
        &mut self,
        operation_id: &str,
    ) -> Result<Sys5I3ProcessMessage, Sys5I3ProcessRuntimeError> {
        let submission = self
            .fabric
            .submit_source_action(SourceAction::owner_operation(operation_id))
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        self.fabric
            .validate_outbound_process_carrier(submission.origin_locus(), submission.envelope_id())
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        if self.reject_next_outbound_extraction {
            self.reject_next_outbound_extraction = false;
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::OutboundExtractionRejected,
            ));
        }
        let carrier = self
            .fabric
            .take_outbound_process_carrier(submission.origin_locus(), submission.envelope_id())
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        if carrier.edge_kind() != CommunicationEdgeKind::OwnerRequest {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
            ));
        }
        let pending = self
            .fabric
            .i3_pending_owner_request_binding(&carrier)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::NoGeneratedOwnerRequest,
                )
            })?;
        let request_identity_ref = pending.semantic_request_identity_ref(
            &self.parent_checked_program_ref,
            &self.projection_ref,
            &self.cohort_ref,
        );
        if self
            .pending_outbound_owner_requests
            .insert(request_identity_ref.clone(), pending)
            .is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
            ));
        }
        Ok(Sys5I3ProcessMessage {
            kind: Sys5I3ProcessMessageKind::Request,
            carrier: Some(carrier),
            semantic_request_identity_ref: request_identity_ref,
            linked_request_identity_ref: None,
            cohort_provenance_ref: self.cohort_ref.clone(),
            identity_basis: self.identity_basis,
        })
    }

    pub fn accept_inbound(
        &mut self,
        message: Sys5I3ProcessMessage,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        // Fail before carrier/M9 admission.  A transport-independent cohort
        // provenance namespace prevents two same-source local activations
        // from crossing their sealed child images, but grants no authority.
        if message.cohort_provenance_ref != self.cohort_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
            ));
        }
        match message.kind {
            Sys5I3ProcessMessageKind::Request => {
                let carrier = message.carrier.ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
                if carrier.edge_kind() != CommunicationEdgeKind::OwnerRequest {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let expected_request_identity_ref = self
                    .fabric
                    .i3_pending_owner_request_binding(&carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?
                    .semantic_request_identity_ref(
                        &self.parent_checked_program_ref,
                        &self.projection_ref,
                        &self.cohort_ref,
                    );
                if message.semantic_request_identity_ref != expected_request_identity_ref
                    || message.linked_request_identity_ref.is_some()
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                if !self.assigned_loci.contains(carrier.target_locus()) {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::NonOwnerServe,
                    ));
                }
                let target_locus = carrier.target_locus().to_string();
                let step = self
                    .fabric
                    .accept_inbound_process_carrier(carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                self.served_owner_request_count = self
                    .served_owner_request_count
                    .checked_add(1)
                    .ok_or_else(|| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                let reply_envelope_id = step.reply_envelope_id().to_string();
                if reply_envelope_id.is_empty() {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let reply = self
                    .fabric
                    .take_outbound_process_carrier(&target_locus, &reply_envelope_id)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                if reply.edge_kind() != CommunicationEdgeKind::OwnerReplyReceipt {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let request_identity_ref = message.semantic_request_identity_ref.clone();
                let serve_occurrence = observer_safe_process_occurrence_ref(
                    "owner-serve-linearization",
                    &request_identity_ref,
                    step.m8_serve_node_id(),
                    step.consumed_envelope_id(),
                );
                self.semantic_occurrences
                    .owner_serve_linearizations
                    .insert(request_identity_ref.clone(), serve_occurrence);
                if let Some(actual_owner_write_occurrence_id) =
                    step.actual_owner_write_occurrence_id()
                {
                    self.local_authoritative_mutation_count = self
                        .local_authoritative_mutation_count
                        .checked_add(1)
                        .ok_or_else(|| {
                            Sys5I3ProcessRuntimeError::new(
                                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                            )
                        })?;
                    let write_occurrence = observer_safe_process_occurrence_ref(
                        "owner-actual-write",
                        &request_identity_ref,
                        actual_owner_write_occurrence_id,
                        step.locus_dequeue_occurrence_id(),
                    );
                    self.semantic_occurrences
                        .actual_owner_writes
                        .insert(request_identity_ref, write_occurrence);
                }
                Ok(Some(Sys5I3ProcessMessage {
                    kind: Sys5I3ProcessMessageKind::Reply,
                    carrier: Some(reply),
                    semantic_request_identity_ref: message.semantic_request_identity_ref.clone(),
                    linked_request_identity_ref: Some(message.semantic_request_identity_ref),
                    cohort_provenance_ref: self.cohort_ref.clone(),
                    identity_basis: self.identity_basis,
                }))
            }
            Sys5I3ProcessMessageKind::Reply => {
                let carrier = message.carrier.ok_or_else(|| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    )
                })?;
                if carrier.edge_kind() != CommunicationEdgeKind::OwnerReplyReceipt {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let request_identity_ref = message.semantic_request_identity_ref.clone();
                if message.linked_request_identity_ref.as_deref()
                    != Some(request_identity_ref.as_str())
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let pending = self
                    .pending_outbound_owner_requests
                    .get(&request_identity_ref)
                    .cloned()
                    .ok_or_else(|| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                self.fabric
                    .validate_i3_pending_owner_reply(&pending, &carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                if !self.assigned_loci.contains(carrier.target_locus()) {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let step = self
                    .fabric
                    .accept_inbound_process_carrier(carrier)
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                if step.receipt().is_none() {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                    ));
                }
                let receipt_occurrence = observer_safe_process_occurrence_ref(
                    "requester-local-receipt",
                    &request_identity_ref,
                    step.consumed_envelope_id(),
                    step.locus_dequeue_occurrence_id(),
                );
                self.semantic_occurrences
                    .requester_local_receipts
                    .insert(request_identity_ref.clone(), receipt_occurrence);
                self.accepted_inbound_receipt_count = self
                    .accepted_inbound_receipt_count
                    .checked_add(1)
                    .ok_or_else(|| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                        )
                    })?;
                // A reply is consumed only after the receiver-owned SYS-4
                // admission completed and produced its local receipt.
                self.pending_outbound_owner_requests
                    .remove(&request_identity_ref);
                Ok(Some(Sys5I3ProcessMessage {
                    kind: Sys5I3ProcessMessageKind::Receipt,
                    carrier: None,
                    semantic_request_identity_ref: message.semantic_request_identity_ref.clone(),
                    linked_request_identity_ref: Some(message.semantic_request_identity_ref),
                    cohort_provenance_ref: self.cohort_ref.clone(),
                    identity_basis: self.identity_basis,
                }))
            }
            Sys5I3ProcessMessageKind::Receipt => {
                // A consumed owner reply completes requester-locally.  There
                // is no invented third carrier back to the owner and no
                // receipt admission count to inflate.
                let _ = message;
                Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                ))
            }
        }
    }

    /// Receiver-owned admission for a private decoded request/reply.  It
    /// validates cohort provenance before touching SYS-4, then binds every
    /// static carrier field and the owner-request M9 lineage to this local
    /// sealed image before any mailbox/store mutation can occur.
    pub fn admit_untrusted_message(
        &mut self,
        candidate: Sys5I3UntrustedProcessMessage,
    ) -> Result<Option<Sys5I3ProcessMessage>, Sys5I3ProcessRuntimeError> {
        let message = candidate.message;
        if message.cohort_provenance_ref != self.cohort_ref {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::CohortProvenanceMismatch,
            ));
        }
        let (kind, expected_edge_kind) = match message.kind {
            PrivateProcessMessageKind::Request => (
                Sys5I3ProcessMessageKind::Request,
                CommunicationEdgeKind::OwnerRequest,
            ),
            PrivateProcessMessageKind::Reply => (
                Sys5I3ProcessMessageKind::Reply,
                CommunicationEdgeKind::OwnerReplyReceipt,
            ),
        };
        let carrier = self
            .fabric
            .bind_i3_untrusted_process_carrier(message.carrier, expected_edge_kind)
            .map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                )
            })?;
        self.accept_inbound(Sys5I3ProcessMessage {
            kind,
            carrier: Some(carrier),
            semantic_request_identity_ref: message.semantic_request_identity_ref,
            linked_request_identity_ref: message.linked_request_identity_ref,
            cohort_provenance_ref: message.cohort_provenance_ref,
            identity_basis: self.identity_basis,
        })
    }

    pub fn attempt_owner_serve(
        &mut self,
        request: &Sys5I3ProcessMessage,
    ) -> Result<(), Sys5I3ProcessRuntimeError> {
        let target = request
            .carrier
            .as_ref()
            .map(Sys4ProcessCarrier::target_locus)
            .unwrap_or_default();
        if !self.assigned_loci.contains(target) {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::NonOwnerServe,
            ));
        }
        Err(Sys5I3ProcessRuntimeError::new(
            Sys5I3ProcessRuntimeErrorKind::DirectRemoteStore,
        ))
    }

    pub fn authoritative_i64_state(
        &self,
        state: &str,
        index: &str,
        field: &str,
    ) -> Result<i64, Sys5I3ProcessRuntimeError> {
        self.assigned_loci
            .iter()
            .find_map(|locus| {
                self.fabric
                    .semantic_snapshot()
                    .int(locus, state, index, field)
            })
            .ok_or_else(|| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::MissingAuthoritativeState,
                )
            })
    }
}

fn observer_safe_process_occurrence_ref(
    kind: &str,
    request_identity_ref: &str,
    source_occurrence_ref: &str,
    admission_occurrence_ref: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/process-semantic-occurrence/v1\\0");
    hasher.update(kind);
    hasher.update(request_identity_ref);
    hasher.update(source_occurrence_ref);
    hasher.update(admission_occurrence_ref);
    format!(
        "sys5-i3-process-semantic-occurrence-sha256-v1:{:x}",
        hasher.finalize()
    )
}

fn logical_origin_ref(
    slot_name: &str,
    assigned_loci: &BTreeSet<String>,
    parent_checked_program_ref: &str,
    projection_ref: &str,
    cohort_ref: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/logical-origin/v1\\0");
    hasher.update(slot_name);
    hasher.update(format!("{assigned_loci:?}"));
    hasher.update(parent_checked_program_ref);
    hasher.update(projection_ref);
    hasher.update(cohort_ref);
    format!("sys5-i3-logical-origin-sha256-v1:{:x}", hasher.finalize())
}

fn process_store_identity_ref(
    parent_checked_program_ref: &str,
    projection_ref: &str,
    cohort_ref: &str,
    logical_origin_ref: &str,
    ordinal: usize,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"mirrorea/sys5/i3/process-local-store/v2\\0");
    hasher.update(parent_checked_program_ref);
    hasher.update(projection_ref);
    hasher.update(cohort_ref);
    hasher.update(logical_origin_ref);
    hasher.update(ordinal.to_le_bytes());
    format!("sys5-i3-local-store-sha256-v2:{:x}", hasher.finalize())
}
