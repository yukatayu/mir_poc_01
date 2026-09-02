//! Private I3-2 process-image and pre-socket process-runtime seam.
//!
//! This module deliberately has no public wire, socket, process launcher, or
//! authority issuer.  It lowers one already checked/projected/admitted local
//! program into per-slot images and transfers exact generated carriers by
//! value only so the subsequent transport milestone has a narrow boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

// A logical activation occurrence is local runtime evidence, not a process
// identifier, endpoint, session, or transport attempt.  It prevents two
// independently derived cohorts of the same checked source from sharing a
// local-store or request identity before I3-3 retry semantics exist.
static NEXT_PROCESS_COHORT_OCCURRENCE: AtomicU64 = AtomicU64::new(1);

use crate::{
    sys3_projection::{BackendProfile, CommunicationEdgeKind},
    sys4_dispatch::{
        FabricProgram, LocalFabric, ObserverSafeM9SemanticRowSets, SealedFabricAdmission,
        SourceAction, Sys4ProcessCarrier,
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
        for slot in &deployment.slots {
            let image = Sys5I3ProcessImage::from_coordinator_parts(
                project,
                deployment,
                &slot.slot_name,
                &coordinator_program,
                &coordinator_admission,
                &cohort_occurrence_ref,
            )?;
            if images.insert(slot.slot_name.clone(), Some(image)).is_some() {
                return Err(Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::DuplicateDeploymentSlot,
                ));
            }
        }

        Ok(Self {
            images,
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
            .map(|edge| Sys5I3RetainedEdgeContract {
                source_locus: edge.from_locus.clone(),
                target_locus: edge.to_locus.clone(),
                edge_ref: edge.edge_ref.clone(),
                operation_id: edge.operation_id.clone(),
                kind: edge.kind.clone(),
                core_ref: edge.core_ref.clone().unwrap_or_default(),
                source_artifact_ref: edge.source_fragment_ref.clone(),
                target_artifact_ref: edge.target_fragment_ref.clone(),
                parent_checked_program_ref: edge.checked_program_identity.clone(),
            })
            .collect::<Vec<_>>();

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
    logical_origin_ref: String,
    next_logical_ordinal: usize,
    fabric: LocalFabric,
    local_authoritative_mutation_count: usize,
    served_owner_request_count: usize,
    accepted_inbound_receipt_count: usize,
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
            logical_origin_ref,
            next_logical_ordinal: 1,
            fabric,
            local_authoritative_mutation_count: 0,
            served_owner_request_count: 0,
            accepted_inbound_receipt_count: 0,
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
        Ok(Sys5I3ProcessMessage {
            kind: Sys5I3ProcessMessageKind::Request,
            carrier: Some(carrier),
            semantic_request_identity_ref: self
                .semantic_request_identity_ref(submission.request_id()),
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
                if !step.m8_serve_node_id().is_empty() {
                    self.local_authoritative_mutation_count = self
                        .local_authoritative_mutation_count
                        .checked_add(1)
                        .ok_or_else(|| {
                            Sys5I3ProcessRuntimeError::new(
                                Sys5I3ProcessRuntimeErrorKind::CarrierAdmissionRejected,
                            )
                        })?;
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

impl Sys5I3ProcessRuntime {
    fn semantic_request_identity_ref(&mut self, request_id: &str) -> String {
        let ordinal = self.next_logical_ordinal;
        self.next_logical_ordinal = self.next_logical_ordinal.checked_add(1).expect(
            "bounded I3-2 logical request ordinal must not overflow before typed runtime shutdown",
        );
        let mut hasher = Sha256::new();
        hasher.update(b"mirrorea/sys5/i3/semantic-request/v2\\0");
        hasher.update(&self.parent_checked_program_ref);
        hasher.update(&self.projection_ref);
        hasher.update(&self.cohort_ref);
        hasher.update(&self.logical_origin_ref);
        hasher.update(ordinal.to_le_bytes());
        hasher.update(request_id);
        format!("sys5-i3-semantic-request-sha256-v2:{:x}", hasher.finalize())
    }
}
