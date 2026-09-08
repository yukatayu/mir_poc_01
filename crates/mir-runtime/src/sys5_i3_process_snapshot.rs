//! Private, versioned I3-2 child-image snapshot records.
//!
//! This module owns DTO-shaped metadata only. It never checks source, lowers
//! a projection, admits authority, creates an issuer, or starts a fabric.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    m8_runtime_admission::M8I3PrivateProviderComponentSnapshot,
    m9_auth_verification::M9I3PrivateAuthorityGenerationSnapshot,
    sys3_i3_private_snapshot::{
        I3PrivateProjectionSnapshot, I3PrivateProviderStaticProjectionSnapshot,
    },
    sys4_dispatch::{
        FabricProgram, SealedFabricAdmission,
        Sys4I3PrivateRestrictedOwnerCapabilitySuccessorSnapshot,
        Sys4I3PrivateRestrictedSourceDeclaredOwnerMembershipSuccessorSnapshot,
        Sys4I3PrivateSealedAdmissionSnapshot,
    },
};

use super::{
    Sys5I3DesignatedRemoteInputClosure, Sys5I3InactiveProviderImageSeed,
    Sys5I3InactiveProviderRuntimeSeed, Sys5I3ObserverSafeChildSeed,
    Sys5I3OrdinaryPrivateRuntimeSeed, Sys5I3PrivateRuntimeSeed, Sys5I3ProcessArtifact,
    Sys5I3ProcessImage, Sys5I3ProcessRuntimeError, Sys5I3ProcessRuntimeErrorKind,
    Sys5I3RetainedEdgeContract, private_runtime_seed_binding_ref,
};

pub(super) const PRIVATE_PROCESS_SNAPSHOT_VERSION: u64 = 1;

/// Private complete process-image body.  The outer codec adds the fixed
/// length frame.  It contains an exact already restricted projection and
/// admission snapshot, never ordinary source or a coordinator's full values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum PrivateProcessImageSnapshot {
    /// Preserve the ordinary outer private-body shape. Its nested M8 snapshot
    /// owns a required, versioned scope discriminator and may evolve without
    /// giving this ordinary image a provider tag.
    Ordinary(Box<PrivateOrdinaryProcessImageSnapshot>),
    /// Provider images use a distinct tagged internal seed and contain no
    /// ordinary executable projection/admission snapshot.
    InactiveProvider(Box<PrivateInactiveProviderProcessImageSnapshot>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateOrdinaryProcessImageSnapshot {
    pub(super) version: u64,
    pub(super) image: PrivateImageMetadataSnapshot,
    projection: I3PrivateProjectionSnapshot,
    admission: Sys4I3PrivateSealedAdmissionSnapshot,
    private_snapshot_binding_ref: String,
    prestaged_owner_capability_lifecycle: Option<PrivatePrestagedOwnerCapabilityLifecycleSnapshot>,
    /// A missing field from a legacy v1 private image means no membership
    /// lifecycle. A staged membership candidate always has this explicit tag;
    /// it is never inferred from the capability shape.
    #[serde(default)]
    prestaged_source_declared_owner_membership_lifecycle:
        Option<PrivatePrestagedSourceDeclaredOwnerMembershipLifecycleSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivatePrestagedOwnerCapabilityLifecycleSnapshot {
    target_slot_name: String,
    stage_identity_binding_ref: String,
    candidate: Sys4I3PrivateRestrictedOwnerCapabilitySuccessorSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivatePrestagedSourceDeclaredOwnerMembershipLifecycleSnapshot {
    target_slot_name: String,
    stage_identity_binding_ref: String,
    candidate: Sys4I3PrivateRestrictedSourceDeclaredOwnerMembershipSuccessorSnapshot,
}

impl PrivateOrdinaryProcessImageSnapshot {
    /// Consume the one process image and snapshot only its already reduced
    /// executable values.  No call here can check source, lower a projection,
    /// run M8 admission, or generate M9 authority.
    pub(super) fn from_image(image: Sys5I3ProcessImage) -> Result<Self, ()> {
        let seed = image.private_runtime_seed.ordinary().ok_or(())?;
        let metadata = PrivateImageMetadataSnapshot::from_image(&image);
        let projection = seed.program.i3_private_projection_snapshot()?;
        let admission = seed.admission.i3_private_snapshot().map_err(|_| ())?;
        let private_snapshot_binding_ref = seed.private_snapshot_binding_ref.clone();
        let prestaged_owner_capability_lifecycle = seed
            .prestaged_owner_capability_lifecycle
            .as_ref()
            .map(|lifecycle| {
                lifecycle.candidate.i3_private_snapshot().map(|candidate| {
                    PrivatePrestagedOwnerCapabilityLifecycleSnapshot {
                        target_slot_name: lifecycle.target_slot_name.clone(),
                        stage_identity_binding_ref: lifecycle.stage_identity_binding_ref.clone(),
                        candidate,
                    }
                })
            })
            .transpose()
            .map_err(|_| ())?;
        let prestaged_source_declared_owner_membership_lifecycle = seed
            .prestaged_source_declared_owner_membership_lifecycle
            .as_ref()
            .map(|lifecycle| {
                lifecycle.candidate.i3_private_snapshot().map(|candidate| {
                    PrivatePrestagedSourceDeclaredOwnerMembershipLifecycleSnapshot {
                        target_slot_name: lifecycle.target_slot_name.clone(),
                        stage_identity_binding_ref: lifecycle.stage_identity_binding_ref.clone(),
                        candidate,
                    }
                })
            })
            .transpose()
            .map_err(|_| ())?;
        if prestaged_owner_capability_lifecycle.is_some()
            && prestaged_source_declared_owner_membership_lifecycle.is_some()
        {
            return Err(());
        }
        if private_snapshot_binding_ref.is_empty()
            || private_snapshot_binding_ref
                != private_runtime_seed_binding_ref(&seed.program, &seed.admission)?
        {
            return Err(());
        }
        Ok(Self {
            version: PRIVATE_PROCESS_SNAPSHOT_VERSION,
            image: metadata,
            projection,
            admission,
            private_snapshot_binding_ref,
            prestaged_owner_capability_lifecycle,
            prestaged_source_declared_owner_membership_lifecycle,
        })
    }

    /// Restore the exact restricted executable seed.  This is intentionally
    /// not `Sys5I3ProcessRuntime::start`: the returned image remains an
    /// untrusted candidate until a separately held coordinator binding is
    /// matched by the codec boundary.
    pub(super) fn into_untrusted_image(
        self,
    ) -> Result<Sys5I3ProcessImage, Sys5I3ProcessRuntimeError> {
        if self.version != PRIVATE_PROCESS_SNAPSHOT_VERSION {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
            ));
        }
        self.image.validate_collections().map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        let program =
            FabricProgram::from_i3_private_projection_snapshot(self.projection).map_err(|_| {
                Sys5I3ProcessRuntimeError::new(
                    Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
                )
            })?;
        let admission = SealedFabricAdmission::from_i3_private_snapshot(self.admission, &program)
            .map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::RuntimeBootstrapRejected)
        })?;
        if self.private_snapshot_binding_ref.is_empty()
            || self.private_snapshot_binding_ref
                != private_runtime_seed_binding_ref(&program, &admission).map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                    )
                })?
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        let (
            slot_name,
            endpoint,
            assigned_loci,
            executable_artifacts,
            required_edge_contracts,
            designated_remote_input_closure,
            child_seed,
            private_integrity_ref,
        ) = self.image.into_image_fields().map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        let prestaged_owner_capability_lifecycle = self
            .prestaged_owner_capability_lifecycle
            .map(|lifecycle| {
                if lifecycle.target_slot_name.is_empty()
                    || lifecycle.stage_identity_binding_ref.is_empty()
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                    ));
                }
                let candidate =
                    crate::sys4_dispatch::Sys4I3RestrictedOwnerCapabilitySuccessor::from_i3_private_snapshot(
                        lifecycle.candidate,
                        &program,
                    )
                    .map_err(|_| {
                        Sys5I3ProcessRuntimeError::new(
                            Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                        )
                    })?;
                Ok(super::Sys5I3PrestagedOwnerCapabilityLifecycle {
                    target_slot_name: lifecycle.target_slot_name,
                    stage_identity_binding_ref: lifecycle.stage_identity_binding_ref,
                    candidate,
                })
            })
            .transpose()?;
        let prestaged_source_declared_owner_membership_lifecycle = self
            .prestaged_source_declared_owner_membership_lifecycle
            .map(|lifecycle| {
                if lifecycle.target_slot_name.is_empty()
                    || lifecycle.stage_identity_binding_ref.is_empty()
                {
                    return Err(Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                    ));
                }
                let candidate = crate::sys4_dispatch::Sys4I3RestrictedSourceDeclaredOwnerMembershipSuccessor::from_i3_private_snapshot(
                    lifecycle.candidate,
                    &program,
                    &admission,
                )
                .map_err(|_| {
                    Sys5I3ProcessRuntimeError::new(
                        Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
                    )
                })?;
                Ok(super::Sys5I3PrestagedSourceDeclaredOwnerMembershipLifecycle {
                    target_slot_name: lifecycle.target_slot_name,
                    stage_identity_binding_ref: lifecycle.stage_identity_binding_ref,
                    candidate,
                })
            })
            .transpose()?;
        if prestaged_owner_capability_lifecycle.is_some()
            && prestaged_source_declared_owner_membership_lifecycle.is_some()
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        let private_runtime_seed =
            Sys5I3PrivateRuntimeSeed::Ordinary(Box::new(Sys5I3OrdinaryPrivateRuntimeSeed {
                parent_checked_program_ref: child_seed.parent_checked_program_ref.clone(),
                projection_ref: child_seed.projection_ref.clone(),
                m9_generation_ref: child_seed.m9_generation_ref.clone(),
                cohort_occurrence_ref: child_seed.cohort_occurrence_ref.clone(),
                private_snapshot_binding_ref: self.private_snapshot_binding_ref,
                program,
                admission,
                prestaged_owner_capability_lifecycle,
                prestaged_source_declared_owner_membership_lifecycle,
            }));
        Ok(Sys5I3ProcessImage {
            slot_name,
            endpoint,
            assigned_loci,
            executable_artifacts,
            required_edge_contracts,
            designated_remote_input_closure,
            child_seed,
            private_runtime_seed,
            inactive_provider: None,
            private_integrity_ref,
        })
    }
}

/// A distinct private body for an inactive provider image. It retains actual
/// restricted M8/M9 structure for the future child consumer, but none of the
/// ordinary executable `FabricProgram` or `SealedFabricAdmission` values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateInactiveProviderProcessImageSnapshot {
    version: u64,
    image: PrivateImageMetadataSnapshot,
    static_snapshot: I3PrivateProviderStaticProjectionSnapshot,
    component_snapshot: M8I3PrivateProviderComponentSnapshot,
    legacy_authority_snapshot: M9I3PrivateAuthorityGenerationSnapshot,
    component_binding_ref: String,
}

impl PrivateProcessImageSnapshot {
    pub(super) fn from_image(image: Sys5I3ProcessImage) -> Result<Self, ()> {
        if image.inactive_provider.is_some()
            || image.private_runtime_seed.inactive_provider().is_some()
        {
            return PrivateInactiveProviderProcessImageSnapshot::from_image(image)
                .map(|snapshot| Self::InactiveProvider(Box::new(snapshot)));
        }
        PrivateOrdinaryProcessImageSnapshot::from_image(image)
            .map(|snapshot| Self::Ordinary(Box::new(snapshot)))
    }

    pub(super) fn into_untrusted_image(
        self,
    ) -> Result<Sys5I3ProcessImage, Sys5I3ProcessRuntimeError> {
        match self {
            Self::Ordinary(snapshot) => (*snapshot).into_untrusted_image(),
            Self::InactiveProvider(snapshot) => (*snapshot).into_untrusted_image(),
        }
    }
}

impl PrivateInactiveProviderProcessImageSnapshot {
    fn from_image(image: Sys5I3ProcessImage) -> Result<Self, ()> {
        let provider = image.inactive_provider.as_ref().ok_or(())?;
        let seed = image.private_runtime_seed.inactive_provider().ok_or(())?;
        if provider.assigned_loci != image.assigned_loci
            || provider.parent_checked_program_ref.is_empty()
            || provider.component_binding_ref.is_empty()
            || provider.component_binding_ref != seed.component_binding_ref
            || provider.static_snapshot != seed.static_snapshot
            || provider.component_snapshot != seed.component_snapshot
            || provider.legacy_authority_snapshot != seed.legacy_authority_snapshot
            || provider.component_snapshot.component_binding_ref() != provider.component_binding_ref
            || provider
                .component_snapshot
                .declared_provider_lowering_count()
                != 4
            || !image.executable_artifacts.is_empty()
            || !image.required_edge_contracts.is_empty()
        {
            return Err(());
        }
        Ok(Self {
            version: PRIVATE_PROCESS_SNAPSHOT_VERSION,
            image: PrivateImageMetadataSnapshot::from_image(&image),
            static_snapshot: provider.static_snapshot.clone(),
            component_snapshot: provider.component_snapshot.clone(),
            legacy_authority_snapshot: provider.legacy_authority_snapshot.clone(),
            component_binding_ref: provider.component_binding_ref.clone(),
        })
    }

    fn into_untrusted_image(self) -> Result<Sys5I3ProcessImage, Sys5I3ProcessRuntimeError> {
        if self.version != PRIVATE_PROCESS_SNAPSHOT_VERSION {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ProgramProjectionMismatch,
            ));
        }
        let (
            slot_name,
            endpoint,
            assigned_loci,
            executable_artifacts,
            required_edge_contracts,
            designated_remote_input_closure,
            child_seed,
            private_integrity_ref,
        ) = self.image.into_image_fields().map_err(|_| {
            Sys5I3ProcessRuntimeError::new(Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch)
        })?;
        if assigned_loci.is_empty()
            || child_seed.parent_checked_program_ref.is_empty()
            || !executable_artifacts.is_empty()
            || !required_edge_contracts.is_empty()
            || self.component_binding_ref.is_empty()
            || self.component_snapshot.component_binding_ref() != self.component_binding_ref
            || self.component_snapshot.declared_provider_lowering_count() != 4
        {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        let provider = Sys5I3InactiveProviderImageSeed {
            assigned_loci: assigned_loci.clone(),
            parent_checked_program_ref: child_seed.parent_checked_program_ref.clone(),
            static_snapshot: self.static_snapshot.clone(),
            component_snapshot: self.component_snapshot.clone(),
            legacy_authority_snapshot: self.legacy_authority_snapshot.clone(),
            component_binding_ref: self.component_binding_ref.clone(),
        };
        let private_runtime_seed = Sys5I3PrivateRuntimeSeed::InactiveProvider(Box::new(
            Sys5I3InactiveProviderRuntimeSeed {
                parent_checked_program_ref: provider.parent_checked_program_ref.clone(),
                static_snapshot: provider.static_snapshot.clone(),
                component_snapshot: provider.component_snapshot.clone(),
                legacy_authority_snapshot: provider.legacy_authority_snapshot.clone(),
                component_binding_ref: provider.component_binding_ref.clone(),
            },
        ));
        let image = Sys5I3ProcessImage {
            slot_name,
            endpoint,
            assigned_loci,
            executable_artifacts,
            required_edge_contracts,
            designated_remote_input_closure,
            child_seed,
            private_runtime_seed,
            inactive_provider: Some(provider),
            private_integrity_ref,
        };
        if image.private_integrity_ref != image.recomputed_private_integrity() {
            return Err(Sys5I3ProcessRuntimeError::new(
                Sys5I3ProcessRuntimeErrorKind::ImageIntegrityMismatch,
            ));
        }
        Ok(image)
    }
}

/// Complete source-free image metadata consumed by the existing image
/// integrity and inventory gates. Executable SYS-3/M8/M9 values are added by
/// their owning snapshot layers, never synthesized from this metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateImageMetadataSnapshot {
    pub(super) slot_name: String,
    pub(super) endpoint: String,
    // Keep untrusted JSON arrays as vectors until explicit duplicate checks
    // complete.  Deserializing straight into BTreeSet would normalize a
    // duplicate assigned locus before the image integrity boundary saw it.
    pub(super) assigned_loci: Vec<String>,
    pub(super) executable_artifacts: Vec<PrivateProcessArtifactSnapshot>,
    pub(super) required_edge_contracts: Vec<PrivateRetainedEdgeContractSnapshot>,
    pub(super) designated_remote_input_closure: PrivateDesignatedClosureSnapshot,
    pub(super) child_seed: PrivateChildSeedSnapshot,
    pub(super) private_integrity_ref: String,
}

impl PrivateImageMetadataSnapshot {
    pub(super) fn from_image(image: &Sys5I3ProcessImage) -> Self {
        Self {
            slot_name: image.slot_name.clone(),
            endpoint: image.endpoint.clone(),
            assigned_loci: image.assigned_loci.iter().cloned().collect(),
            executable_artifacts: image
                .executable_artifacts
                .iter()
                .map(PrivateProcessArtifactSnapshot::from_artifact)
                .collect(),
            required_edge_contracts: image
                .required_edge_contracts
                .iter()
                .map(PrivateRetainedEdgeContractSnapshot::from_contract)
                .collect(),
            designated_remote_input_closure: PrivateDesignatedClosureSnapshot::from_closure(
                &image.designated_remote_input_closure,
            ),
            child_seed: PrivateChildSeedSnapshot::from_seed(&image.child_seed),
            private_integrity_ref: image.private_integrity_ref.clone(),
        }
    }

    fn validate_collections(&self) -> Result<(), ()> {
        if has_duplicate(&self.assigned_loci)
            || has_duplicate(&self.executable_artifacts)
            || has_duplicate(&self.required_edge_contracts)
        {
            return Err(());
        }
        self.child_seed.validate_collections()
    }

    #[allow(clippy::type_complexity)]
    pub(super) fn into_image_fields(
        self,
    ) -> Result<
        (
            String,
            String,
            BTreeSet<String>,
            Vec<Sys5I3ProcessArtifact>,
            Vec<Sys5I3RetainedEdgeContract>,
            Sys5I3DesignatedRemoteInputClosure,
            Sys5I3ObserverSafeChildSeed,
            String,
        ),
        (),
    > {
        self.validate_collections()?;
        Ok((
            self.slot_name,
            self.endpoint,
            into_unique_set(self.assigned_loci)?,
            self.executable_artifacts
                .into_iter()
                .map(PrivateProcessArtifactSnapshot::into_artifact)
                .collect(),
            self.required_edge_contracts
                .into_iter()
                .map(PrivateRetainedEdgeContractSnapshot::into_contract)
                .collect(),
            self.designated_remote_input_closure.into_closure(),
            self.child_seed.into_seed()?,
            self.private_integrity_ref,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateProcessArtifactSnapshot {
    locus: String,
    operation_id: String,
    kind: String,
    core_ref: String,
    fragment_ref: String,
    parent_checked_program_ref: String,
}

impl PrivateProcessArtifactSnapshot {
    fn from_artifact(artifact: &Sys5I3ProcessArtifact) -> Self {
        Self {
            locus: artifact.locus.clone(),
            operation_id: artifact.operation_id.clone(),
            kind: artifact.kind.clone(),
            core_ref: artifact.core_ref.clone(),
            fragment_ref: artifact.fragment_ref.clone(),
            parent_checked_program_ref: artifact.parent_checked_program_ref.clone(),
        }
    }

    fn into_artifact(self) -> Sys5I3ProcessArtifact {
        Sys5I3ProcessArtifact {
            locus: self.locus,
            operation_id: self.operation_id,
            kind: self.kind,
            core_ref: self.core_ref,
            fragment_ref: self.fragment_ref,
            parent_checked_program_ref: self.parent_checked_program_ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateRetainedEdgeContractSnapshot {
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

impl PrivateRetainedEdgeContractSnapshot {
    fn from_contract(contract: &Sys5I3RetainedEdgeContract) -> Self {
        Self {
            source_locus: contract.source_locus.clone(),
            target_locus: contract.target_locus.clone(),
            edge_ref: contract.edge_ref.clone(),
            operation_id: contract.operation_id.clone(),
            kind: contract.kind.clone(),
            core_ref: contract.core_ref.clone(),
            source_artifact_ref: contract.source_artifact_ref.clone(),
            target_artifact_ref: contract.target_artifact_ref.clone(),
            parent_checked_program_ref: contract.parent_checked_program_ref.clone(),
        }
    }

    fn into_contract(self) -> Sys5I3RetainedEdgeContract {
        Sys5I3RetainedEdgeContract {
            source_locus: self.source_locus,
            target_locus: self.target_locus,
            edge_ref: self.edge_ref,
            operation_id: self.operation_id,
            kind: self.kind,
            core_ref: self.core_ref,
            source_artifact_ref: self.source_artifact_ref,
            target_artifact_ref: self.target_artifact_ref,
            parent_checked_program_ref: self.parent_checked_program_ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateDesignatedClosureSnapshot {
    request_receipt_pair_count: usize,
    distinct_operation_count: usize,
    pairs_are_distinguished_beyond_operation: bool,
    opaque_digest_ref: String,
}

impl PrivateDesignatedClosureSnapshot {
    fn from_closure(closure: &Sys5I3DesignatedRemoteInputClosure) -> Self {
        Self {
            request_receipt_pair_count: closure.request_receipt_pair_count,
            distinct_operation_count: closure.distinct_operation_count,
            pairs_are_distinguished_beyond_operation: closure
                .pairs_are_distinguished_beyond_operation,
            opaque_digest_ref: closure.opaque_digest_ref.clone(),
        }
    }

    fn into_closure(self) -> Sys5I3DesignatedRemoteInputClosure {
        Sys5I3DesignatedRemoteInputClosure {
            request_receipt_pair_count: self.request_receipt_pair_count,
            distinct_operation_count: self.distinct_operation_count,
            pairs_are_distinguished_beyond_operation: self.pairs_are_distinguished_beyond_operation,
            opaque_digest_ref: self.opaque_digest_ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrivateChildSeedSnapshot {
    parent_checked_program_ref: String,
    projection_ref: String,
    m9_generation_ref: String,
    cohort_occurrence_ref: String,
    required_local_authority_closure: PrivateAuthorityClosureSnapshot,
}

impl PrivateChildSeedSnapshot {
    fn from_seed(seed: &Sys5I3ObserverSafeChildSeed) -> Self {
        Self {
            parent_checked_program_ref: seed.parent_checked_program_ref.clone(),
            projection_ref: seed.projection_ref.clone(),
            m9_generation_ref: seed.m9_generation_ref.clone(),
            cohort_occurrence_ref: seed.cohort_occurrence_ref.clone(),
            required_local_authority_closure: PrivateAuthorityClosureSnapshot::from_closure(
                &seed.required_local_authority_closure,
            ),
        }
    }

    fn validate_collections(&self) -> Result<(), ()> {
        self.required_local_authority_closure.validate_collections()
    }

    fn into_seed(self) -> Result<Sys5I3ObserverSafeChildSeed, ()> {
        Ok(Sys5I3ObserverSafeChildSeed {
            parent_checked_program_ref: self.parent_checked_program_ref,
            projection_ref: self.projection_ref,
            m9_generation_ref: self.m9_generation_ref,
            cohort_occurrence_ref: self.cohort_occurrence_ref,
            required_local_authority_closure: self
                .required_local_authority_closure
                .into_closure()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "row", content = "fields", deny_unknown_fields)]
enum PrivateSemanticRowSnapshot {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateAuthorityClosureSnapshot {
    assigned_loci: Vec<String>,
    rows: Vec<PrivateSemanticRowSnapshot>,
    semantic_row_digest_ref: String,
    opaque_digest_ref: String,
    opaque_cohort_ref: String,
}

impl PrivateAuthorityClosureSnapshot {
    fn from_closure(closure: &super::Sys5I3RequiredLocalAuthorityClosure) -> Self {
        Self {
            assigned_loci: closure.assigned_loci.iter().cloned().collect(),
            rows: closure
                .rows
                .iter()
                .map(|row| match row {
                    super::Sys5I3SemanticRow::Artifact {
                        locus,
                        fragment_ref,
                    } => PrivateSemanticRowSnapshot::Artifact {
                        locus: locus.clone(),
                        fragment_ref: fragment_ref.clone(),
                    },
                    super::Sys5I3SemanticRow::IncidentEdge {
                        source_locus,
                        target_locus,
                        edge_ref,
                    } => PrivateSemanticRowSnapshot::IncidentEdge {
                        source_locus: source_locus.clone(),
                        target_locus: target_locus.clone(),
                        edge_ref: edge_ref.clone(),
                    },
                })
                .collect(),
            semantic_row_digest_ref: closure.semantic_row_digest_ref.clone(),
            opaque_digest_ref: closure.opaque_digest_ref.clone(),
            opaque_cohort_ref: closure.opaque_cohort_ref.clone(),
        }
    }

    fn validate_collections(&self) -> Result<(), ()> {
        if has_duplicate(&self.assigned_loci) || has_duplicate(&self.rows) {
            return Err(());
        }
        Ok(())
    }

    fn into_closure(self) -> Result<super::Sys5I3RequiredLocalAuthorityClosure, ()> {
        self.validate_collections()?;
        Ok(super::Sys5I3RequiredLocalAuthorityClosure {
            assigned_loci: into_unique_set(self.assigned_loci)?,
            rows: self
                .rows
                .into_iter()
                .map(|row| match row {
                    PrivateSemanticRowSnapshot::Artifact {
                        locus,
                        fragment_ref,
                    } => super::Sys5I3SemanticRow::Artifact {
                        locus,
                        fragment_ref,
                    },
                    PrivateSemanticRowSnapshot::IncidentEdge {
                        source_locus,
                        target_locus,
                        edge_ref,
                    } => super::Sys5I3SemanticRow::IncidentEdge {
                        source_locus,
                        target_locus,
                        edge_ref,
                    },
                })
                .collect(),
            semantic_row_digest_ref: self.semantic_row_digest_ref,
            opaque_digest_ref: self.opaque_digest_ref,
            opaque_cohort_ref: self.opaque_cohort_ref,
        })
    }
}

fn has_duplicate<T>(values: &[T]) -> bool
where
    T: Ord,
{
    let mut seen = BTreeSet::new();
    values.iter().any(|value| !seen.insert(value))
}

fn into_unique_set(values: Vec<String>) -> Result<BTreeSet<String>, ()> {
    let value_count = values.len();
    let unique = values.into_iter().collect::<BTreeSet<_>>();
    if unique.len() != value_count {
        return Err(());
    }
    Ok(unique)
}
