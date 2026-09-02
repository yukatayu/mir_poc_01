//! Private, fail-closed process-image snapshots for an already checked SYS-3
//! projection.
//!
//! This is an internal I3 bridge, never a public wire, package, or
//! compatibility format.  It serializes a projected value exactly; it neither
//! parses source nor checks/lower/adopts any new semantic fact on restore.

use std::collections::{BTreeMap, BTreeSet};

use mir_semantics::{
    evaluation_materialization::{
        InputFrontier, ObservationPolicy, OccurrenceId as MaterializationOccurrenceId, PolicyStamp,
        StaticRetryContractKind,
    },
    shared_model::{
        BindingActivationFrontier, OccurrenceId as SharedOccurrenceId, ResultFrontier, ResultKey,
        ResultVersion, SourceRef,
    },
    surface_v0_pipeline::{
        ResidualObligationKind,
        private_snapshot::{
            SnapshotCheckedEvaluationSignature, SnapshotCheckedIndexedStateSchema,
            SnapshotCheckedProgramIdentity, SnapshotDesignatedCheckedCore,
            SnapshotDesignatedRemoteInputDependency, SnapshotDesignatedResultConsumerCore,
            SnapshotEffectKind, SnapshotFailureRow, SnapshotGeneratedObligationKind,
            SnapshotOwnerRmwCheckedCore, SnapshotRelationCheckedCore,
            SnapshotRelationTransformCore, SnapshotSourceRef,
        },
    },
};
use serde::{Deserialize, Serialize};

use crate::sys3_projection::model::*;

/// The private schema version for the complete projection snapshot.
pub(crate) const I3_PRIVATE_PROJECTION_SNAPSHOT_VERSION: u32 = 1;

/// Fail-closed rejection of a private SYS-3 projection snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum I3PrivateProjectionSnapshotError {
    UnsupportedVersion { found: u32 },
    DuplicateMapKey { map: &'static str, key: String },
    DuplicateSetMember { set: &'static str, member: String },
    UnsupportedVariant { kind: &'static str },
    StructuralMismatch { reason: &'static str },
    SemanticSnapshot,
}

impl From<mir_semantics::surface_v0_pipeline::private_snapshot::SnapshotError>
    for I3PrivateProjectionSnapshotError
{
    fn from(_: mir_semantics::surface_v0_pipeline::private_snapshot::SnapshotError) -> Self {
        Self::SemanticSnapshot
    }
}

/// Complete, source-free snapshot of an already restricted projection.
///
/// JSON object maps are intentionally represented as ordered entry vectors so
/// decoding can reject duplicate keys rather than accepting a last-write-wins
/// interpretation.  All nested records deny unknown fields.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct I3PrivateProjectionSnapshot {
    version: u32,
    checked_program_identity: SnapshotCheckedProgramIdentity,
    projection_identity: SnapshotProjectionIdentity,
    locus_programs: Vec<SnapshotLocusProgramEntry>,
    communication_plan: SnapshotCommunicationPlan,
    effect_handler_plan: SnapshotEffectHandlerPlan,
    relation_graph: SnapshotProjectionRelationGraph,
    observation_plan: SnapshotObservationPlan,
    persistence_plan: SnapshotPersistencePlan,
    projected_source_map: Vec<SnapshotCorrespondenceEntryMapEntry>,
    static_readiness: SnapshotStaticProjectionReadiness,
    runtime_admission_status: SnapshotRuntimeAdmissionStatus,
    backend_requirements: SnapshotBackendRequirements,
    static_conflict_policy: Vec<SnapshotConflictPolicyEntry>,
}

impl I3PrivateProjectionSnapshot {
    fn from_projection(
        value: &GlobalProjectionResult,
    ) -> Result<Self, I3PrivateProjectionSnapshotError> {
        Ok(Self {
            version: I3_PRIVATE_PROJECTION_SNAPSHOT_VERSION,
            checked_program_identity: SnapshotCheckedProgramIdentity::from_checked(
                &value.checked_program_identity,
            ),
            projection_identity: SnapshotProjectionIdentity::from_projection(
                &value.projection_identity,
            ),
            locus_programs: value
                .locus_programs
                .iter()
                .map(|(key, program)| SnapshotLocusProgramEntry {
                    key: key.clone(),
                    value: SnapshotLocusProgram::from_program(program),
                })
                .collect(),
            communication_plan: SnapshotCommunicationPlan::from_plan(&value.communication_plan),
            effect_handler_plan: SnapshotEffectHandlerPlan::from_plan(&value.effect_handler_plan),
            relation_graph: SnapshotProjectionRelationGraph::from_graph(&value.relation_graph)?,
            observation_plan: SnapshotObservationPlan::from_plan(&value.observation_plan),
            persistence_plan: SnapshotPersistencePlan::from_plan(&value.persistence_plan),
            projected_source_map: value
                .projected_source_map
                .entries
                .iter()
                .map(|(key, entry)| SnapshotCorrespondenceEntryMapEntry {
                    key: key.clone(),
                    value: SnapshotCorrespondenceEntry::from_entry(entry),
                })
                .collect(),
            static_readiness: value.static_readiness.into(),
            runtime_admission_status: value.runtime_admission_status.into(),
            backend_requirements: SnapshotBackendRequirements::from_requirements(
                &value.backend_requirements,
            ),
            static_conflict_policy: value
                .static_conflict_policy
                .designated_result_consumers
                .iter()
                .map(|(key, policy)| SnapshotConflictPolicyEntry {
                    key: key.clone(),
                    value: SnapshotDesignatedResultConsumerConflictPolicy::from_policy(policy),
                })
                .collect(),
        })
    }

    fn into_projection(self) -> Result<GlobalProjectionResult, I3PrivateProjectionSnapshotError> {
        if self.version != I3_PRIVATE_PROJECTION_SNAPSHOT_VERSION {
            return Err(I3PrivateProjectionSnapshotError::UnsupportedVersion {
                found: self.version,
            });
        }
        let checked_program_identity = self.checked_program_identity.into_checked()?;
        let projection_identity = self.projection_identity.into_projection()?;
        if projection_identity.checked_program_identity != checked_program_identity {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "projection identity must retain the checked program identity",
            });
        }
        let locus_programs = collect_entries(self.locus_programs, "locus_programs", |entry| {
            Ok((entry.key, entry.value.into_program()?))
        })?;
        let communication_plan = self.communication_plan.into_plan()?;
        let effect_handler_plan = self.effect_handler_plan.into_plan()?;
        let relation_graph = self.relation_graph.into_graph()?;
        let observation_plan = self.observation_plan.into_plan()?;
        let persistence_plan = self.persistence_plan.into_plan()?;
        let projected_source_map = ProjectedSourceMap {
            entries: collect_entries(self.projected_source_map, "projected_source_map", |entry| {
                Ok((entry.key, entry.value.into_entry()?))
            })?,
        };
        let static_conflict_policy = StaticConflictPolicy {
            designated_result_consumers: collect_entries(
                self.static_conflict_policy,
                "static_conflict_policy",
                |entry| Ok((entry.key, entry.value.into_policy()?)),
            )?,
        };
        let restored = GlobalProjectionResult {
            checked_program_identity,
            projection_identity,
            locus_programs,
            communication_plan,
            effect_handler_plan,
            relation_graph,
            observation_plan,
            persistence_plan,
            projected_source_map,
            static_readiness: self.static_readiness.into(),
            runtime_admission_status: self.runtime_admission_status.into(),
            backend_requirements: self.backend_requirements.into_requirements()?,
            static_conflict_policy,
        };
        validate_logical_source_paths(&restored)?;
        Ok(restored)
    }
}

fn validate_logical_source_paths(
    projection: &GlobalProjectionResult,
) -> Result<(), I3PrivateProjectionSnapshotError> {
    let is_logical = |path: &str| {
        !path.is_empty()
            && !path.starts_with('/')
            && !path.starts_with('\\')
            && path.as_bytes().get(1).is_none_or(|byte| *byte != b':')
    };
    let check = |source_ref: &SourceRef| {
        if is_logical(&source_ref.path) {
            Ok(())
        } else {
            Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "private projection snapshot rejects host source paths",
            })
        }
    };
    if !is_logical(projection.checked_program_identity.source_file()) {
        return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "private projection snapshot rejects a host source file path",
        });
    }
    check(projection.checked_program_identity.root_source_ref())?;
    check(
        projection
            .projection_identity
            .checked_program_identity
            .root_source_ref(),
    )?;
    for program in projection.locus_programs.values() {
        for fragment in &program.operations.entries {
            check(&fragment.source_ref)?;
            check(&fragment.checked_core_identity.source_ref)?;
            if let PlacementSpecificCore::RelationConsumer { descriptor } = &fragment.placement {
                check(&descriptor.source_ref)?;
            }
        }
    }
    for edge in &projection.communication_plan.edges {
        check(&edge.source_ref)?;
        check(&edge.checked_core_identity.source_ref)?;
        check(&edge.carrier_contract.source_ref)?;
        check(&edge.carrier_contract.request_identity_template.source_ref)?;
    }
    for handler in &projection.effect_handler_plan.handlers {
        check(&handler.source_ref)?;
        check(&handler.checked_core_identity.source_ref)?;
    }
    for relation in projection.relation_graph.relations.values() {
        check(&relation.primary_anchor.source_ref)?;
        check(&relation.fallback_anchor.source_ref)?;
        if let Some(source_ref) = &relation.primary_anchor.anchor_locus_source_ref {
            check(source_ref)?;
        }
        if let Some(source_ref) = &relation.fallback_anchor.anchor_locus_source_ref {
            check(source_ref)?;
        }
        for (_, source_ref) in &relation.residual_source_refs {
            check(source_ref)?;
        }
    }
    for edge in &projection.relation_graph.typed_dependency_edges {
        check(&edge.from.source_ref)?;
        check(&edge.to.source_ref)?;
    }
    for row in &projection.observation_plan.rows {
        check(&row.source_ref)?;
    }
    for entry in projection.projected_source_map.entries.values() {
        check(&entry.source_ref)?;
        check(&entry.checked_core_identity.source_ref)?;
    }
    Ok(())
}

impl GlobalProjectionResult {
    /// Export an already selected private projection process image.  This does
    /// not parse source, compute a topology, restrict loci, or issue authority.
    pub(crate) fn to_i3_private_snapshot(
        &self,
    ) -> Result<I3PrivateProjectionSnapshot, I3PrivateProjectionSnapshotError> {
        I3PrivateProjectionSnapshot::from_projection(self)
    }

    /// Restore only a previously projected private process image.  Callers
    /// must separately validate its admission/provenance boundary.
    pub(crate) fn from_i3_private_snapshot(
        snapshot: I3PrivateProjectionSnapshot,
    ) -> Result<Self, I3PrivateProjectionSnapshotError> {
        snapshot.into_projection()
    }
}

fn collect_entries<T, V, F>(
    entries: Vec<T>,
    map: &'static str,
    mut convert: F,
) -> Result<BTreeMap<String, V>, I3PrivateProjectionSnapshotError>
where
    F: FnMut(T) -> Result<(String, V), I3PrivateProjectionSnapshotError>,
{
    let mut restored = BTreeMap::new();
    for entry in entries {
        let (key, value) = convert(entry)?;
        if restored.insert(key.clone(), value).is_some() {
            return Err(I3PrivateProjectionSnapshotError::DuplicateMapKey { map, key });
        }
    }
    Ok(restored)
}

fn collect_set<T, F>(
    entries: Vec<T>,
    set: &'static str,
    mut render: F,
) -> Result<BTreeSet<T>, I3PrivateProjectionSnapshotError>
where
    T: Ord + Clone,
    F: FnMut(&T) -> String,
{
    let mut restored = BTreeSet::new();
    for entry in entries {
        if !restored.insert(entry.clone()) {
            return Err(I3PrivateProjectionSnapshotError::DuplicateSetMember {
                set,
                member: render(&entry),
            });
        }
    }
    Ok(restored)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotLocusProgramEntry {
    key: String,
    value: SnapshotLocusProgram,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotLocusProgram {
    locus: String,
    locus_tag: String,
    operations: Vec<SnapshotProjectedOperationFragment>,
    checked_owner_operations: Vec<String>,
    checked_local_state_schemas: Vec<SnapshotCheckedIndexedStateSchema>,
    declared_failures: Vec<SnapshotFailureMapEntry>,
    generated_failures: Vec<SnapshotFailureMapEntry>,
}

impl SnapshotLocusProgram {
    fn from_program(value: &LocusProgram) -> Self {
        Self {
            locus: value.locus.clone(),
            locus_tag: value.locus_tag.name.clone(),
            operations: value
                .operations
                .entries
                .iter()
                .map(SnapshotProjectedOperationFragment::from_fragment)
                .collect(),
            checked_owner_operations: value.checked_fragments.owner_operations.clone(),
            checked_local_state_schemas: value
                .checked_fragments
                .local_state_schemas
                .iter()
                .map(SnapshotCheckedIndexedStateSchema::from_checked)
                .collect(),
            declared_failures: value
                .declared_failures
                .iter()
                .map(|(key, value)| SnapshotFailureMapEntry {
                    key: key.clone(),
                    value: SnapshotFailureRow::from_checked(value),
                })
                .collect(),
            generated_failures: value
                .generated_failures
                .iter()
                .map(|(key, value)| SnapshotFailureMapEntry {
                    key: key.clone(),
                    value: SnapshotFailureRow::from_checked(value),
                })
                .collect(),
        }
    }

    fn into_program(self) -> Result<LocusProgram, I3PrivateProjectionSnapshotError> {
        let declared_failures =
            collect_entries(self.declared_failures, "declared_failures", |entry| {
                Ok((entry.key, entry.value.into_checked()?))
            })?;
        let generated_failures =
            collect_entries(self.generated_failures, "generated_failures", |entry| {
                Ok((entry.key, entry.value.into_checked()?))
            })?;
        let locus_tag = LocusTag {
            name: self.locus_tag,
        };
        if self.locus != locus_tag.name {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "locus program key and locus tag must agree",
            });
        }
        Ok(LocusProgram {
            locus: self.locus,
            locus_tag,
            operations: ProjectedOperationFragments {
                entries: self
                    .operations
                    .into_iter()
                    .map(SnapshotProjectedOperationFragment::into_fragment)
                    .collect::<Result<_, _>>()?,
            },
            checked_fragments: ProjectedCheckedFragments {
                owner_operations: self.checked_owner_operations,
                local_state_schemas: self
                    .checked_local_state_schemas
                    .into_iter()
                    .map(SnapshotCheckedIndexedStateSchema::into_checked)
                    .collect::<Result<_, _>>()?,
            },
            declared_failures,
            generated_failures,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotFailureMapEntry {
    key: String,
    value: SnapshotFailureRow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectedOperationFragment {
    operation_id: String,
    kind: SnapshotProjectedOperationFragmentKind,
    source_ref: SnapshotSourceRef,
    core_ref: String,
    artifact_ref: String,
    authority_requirements: SnapshotAuthorityRequirements,
    declared_failure_row: SnapshotFailureRow,
    generated_failure_row: SnapshotFailureRow,
    placement: SnapshotPlacementSpecificCore,
    locus_tag: String,
    fragment_ref: String,
    checked_core_identity: SnapshotCheckedCoreIdentity,
    semantic_obligations: Vec<SnapshotSemanticObligation>,
    runtime_seam_requirements: SnapshotRuntimeSeamRequirements,
    designated_result_consumer_expression_leakage: bool,
}

impl SnapshotProjectedOperationFragment {
    fn from_fragment(value: &ProjectedOperationFragment) -> Self {
        Self {
            operation_id: value.operation_id.clone(),
            kind: value.kind.into(),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            core_ref: value.core_ref.clone(),
            artifact_ref: value.artifact_ref.clone(),
            authority_requirements: SnapshotAuthorityRequirements::from_requirements(
                &value.authority_requirements,
            ),
            declared_failure_row: SnapshotFailureRow::from_checked(&value.declared_failure_row),
            generated_failure_row: SnapshotFailureRow::from_checked(&value.generated_failure_row),
            placement: SnapshotPlacementSpecificCore::from_placement(&value.placement),
            locus_tag: value.locus_tag.name.clone(),
            fragment_ref: value.fragment_ref.clone(),
            checked_core_identity: SnapshotCheckedCoreIdentity::from_identity(
                &value.checked_core_identity,
            ),
            semantic_obligations: value
                .semantic_obligations
                .rows
                .iter()
                .map(|(kind, source_ref)| SnapshotSemanticObligation {
                    kind: SnapshotGeneratedObligationKind::from_checked(kind),
                    source_ref: SnapshotSourceRef::from_checked(source_ref),
                })
                .collect(),
            runtime_seam_requirements: SnapshotRuntimeSeamRequirements::from_requirements(
                &value.runtime_seam_requirements,
            ),
            designated_result_consumer_expression_leakage: value
                .designated_result_consumer_expression_leakage,
        }
    }

    fn into_fragment(self) -> Result<ProjectedOperationFragment, I3PrivateProjectionSnapshotError> {
        Ok(ProjectedOperationFragment {
            operation_id: self.operation_id,
            kind: self.kind.into(),
            source_ref: self.source_ref.into_checked()?,
            core_ref: self.core_ref,
            artifact_ref: self.artifact_ref,
            authority_requirements: self.authority_requirements.into_requirements()?,
            declared_failure_row: self.declared_failure_row.into_checked()?,
            generated_failure_row: self.generated_failure_row.into_checked()?,
            placement: self.placement.into_placement()?,
            locus_tag: LocusTag {
                name: self.locus_tag,
            },
            fragment_ref: self.fragment_ref,
            checked_core_identity: self.checked_core_identity.into_identity()?,
            semantic_obligations: SemanticObligations {
                rows: self
                    .semantic_obligations
                    .into_iter()
                    .map(|row| Ok((row.kind.into_checked(), row.source_ref.into_checked()?)))
                    .collect::<Result<_, I3PrivateProjectionSnapshotError>>()?,
            },
            runtime_seam_requirements: self.runtime_seam_requirements.into_requirements()?,
            designated_result_consumer_expression_leakage: self
                .designated_result_consumer_expression_leakage,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotSemanticObligation {
    kind: SnapshotGeneratedObligationKind,
    source_ref: SnapshotSourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "fields",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum SnapshotPlacementSpecificCore {
    OwnerRequest {
        signature: SnapshotCheckedEvaluationSignature,
        origin_locus: String,
        target_owner_locus: String,
    },
    OwnerRmw {
        core: SnapshotOwnerRmwCheckedCore,
        local_state_schemas: Vec<SnapshotCheckedIndexedStateSchema>,
    },
    RelationOwner {
        core: SnapshotRelationCheckedCore,
    },
    RelationConsumer {
        descriptor: SnapshotConsumerRelationProjectionDescriptor,
    },
    DesignatedSource {
        dependency: SnapshotDesignatedRemoteInputDependency,
        local_state_schemas: Vec<SnapshotCheckedIndexedStateSchema>,
    },
    DesignatedEvaluator {
        core: SnapshotDesignatedCheckedCore,
    },
    DesignatedResultConsumer {
        core: SnapshotDesignatedResultConsumerCore,
    },
}

impl SnapshotPlacementSpecificCore {
    fn from_placement(value: &PlacementSpecificCore) -> Self {
        match value {
            PlacementSpecificCore::OwnerRequest {
                signature,
                origin_locus,
                target_owner_locus,
            } => Self::OwnerRequest {
                signature: SnapshotCheckedEvaluationSignature::from_checked(signature),
                origin_locus: origin_locus.clone(),
                target_owner_locus: target_owner_locus.clone(),
            },
            PlacementSpecificCore::OwnerRmw {
                core,
                local_state_schemas,
            } => Self::OwnerRmw {
                core: SnapshotOwnerRmwCheckedCore::from_checked(core),
                local_state_schemas: local_state_schemas
                    .iter()
                    .map(SnapshotCheckedIndexedStateSchema::from_checked)
                    .collect(),
            },
            PlacementSpecificCore::RelationOwner { core } => Self::RelationOwner {
                core: SnapshotRelationCheckedCore::from_checked(core),
            },
            PlacementSpecificCore::RelationConsumer { descriptor } => Self::RelationConsumer {
                descriptor: SnapshotConsumerRelationProjectionDescriptor::from_descriptor(
                    descriptor,
                ),
            },
            PlacementSpecificCore::DesignatedSource {
                dependency,
                local_state_schemas,
            } => Self::DesignatedSource {
                dependency: SnapshotDesignatedRemoteInputDependency::from_checked(dependency),
                local_state_schemas: local_state_schemas
                    .iter()
                    .map(SnapshotCheckedIndexedStateSchema::from_checked)
                    .collect(),
            },
            PlacementSpecificCore::DesignatedEvaluator { core } => Self::DesignatedEvaluator {
                core: SnapshotDesignatedCheckedCore::from_checked(core),
            },
            PlacementSpecificCore::DesignatedResultConsumer { core } => {
                Self::DesignatedResultConsumer {
                    core: SnapshotDesignatedResultConsumerCore::from_checked(core),
                }
            }
        }
    }

    fn into_placement(self) -> Result<PlacementSpecificCore, I3PrivateProjectionSnapshotError> {
        Ok(match self {
            Self::OwnerRequest {
                signature,
                origin_locus,
                target_owner_locus,
            } => PlacementSpecificCore::OwnerRequest {
                signature: signature.into_checked()?,
                origin_locus,
                target_owner_locus,
            },
            Self::OwnerRmw {
                core,
                local_state_schemas,
            } => PlacementSpecificCore::OwnerRmw {
                core: core.into_checked()?,
                local_state_schemas: local_state_schemas
                    .into_iter()
                    .map(SnapshotCheckedIndexedStateSchema::into_checked)
                    .collect::<Result<_, _>>()?,
            },
            Self::RelationOwner { core } => PlacementSpecificCore::RelationOwner {
                core: core.into_checked()?,
            },
            Self::RelationConsumer { descriptor } => PlacementSpecificCore::RelationConsumer {
                descriptor: descriptor.into_descriptor()?,
            },
            Self::DesignatedSource {
                dependency,
                local_state_schemas,
            } => PlacementSpecificCore::DesignatedSource {
                dependency: dependency.into_checked()?,
                local_state_schemas: local_state_schemas
                    .into_iter()
                    .map(SnapshotCheckedIndexedStateSchema::into_checked)
                    .collect::<Result<_, _>>()?,
            },
            Self::DesignatedEvaluator { core } => PlacementSpecificCore::DesignatedEvaluator {
                core: core.into_checked()?,
            },
            Self::DesignatedResultConsumer { core } => {
                PlacementSpecificCore::DesignatedResultConsumer {
                    core: core.into_checked()?,
                }
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotConsumerRelationProjectionDescriptor {
    source_relation: String,
    owner_locus: String,
    consumer_locus: String,
    source_ref: SnapshotSourceRef,
}

impl SnapshotConsumerRelationProjectionDescriptor {
    fn from_descriptor(value: &ConsumerRelationProjectionDescriptor) -> Self {
        Self {
            source_relation: value.source_relation.clone(),
            owner_locus: value.owner_locus.clone(),
            consumer_locus: value.consumer_locus.clone(),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
        }
    }

    fn into_descriptor(
        self,
    ) -> Result<ConsumerRelationProjectionDescriptor, I3PrivateProjectionSnapshotError> {
        Ok(ConsumerRelationProjectionDescriptor {
            source_relation: self.source_relation,
            owner_locus: self.owner_locus,
            consumer_locus: self.consumer_locus,
            source_ref: self.source_ref.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCheckedCoreIdentity {
    checked_program_identity: SnapshotCheckedProgramIdentity,
    operation_id: String,
    fragment_kind: Option<SnapshotProjectedOperationFragmentKind>,
    edge_kind: Option<SnapshotCommunicationEdgeKind>,
    source_ref: SnapshotSourceRef,
    dependency_ordinal: Option<usize>,
    designated_dependency: Option<SnapshotDesignatedRemoteInputDependency>,
}

impl SnapshotCheckedCoreIdentity {
    fn from_identity(value: &CheckedCoreIdentity) -> Self {
        Self {
            checked_program_identity: SnapshotCheckedProgramIdentity::from_checked(
                &value.checked_program_identity,
            ),
            operation_id: value.operation_id.clone(),
            fragment_kind: value.fragment_kind.map(Into::into),
            edge_kind: value.edge_kind.map(Into::into),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            dependency_ordinal: value.dependency_ordinal,
            designated_dependency: value
                .designated_dependency
                .as_ref()
                .map(SnapshotDesignatedRemoteInputDependency::from_checked),
        }
    }

    fn into_identity(self) -> Result<CheckedCoreIdentity, I3PrivateProjectionSnapshotError> {
        let fragment_kind = self.fragment_kind.map(Into::into);
        let edge_kind = self.edge_kind.map(Into::into);
        if fragment_kind.is_some() == edge_kind.is_some() {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "checked core identity must name exactly one fragment or edge kind",
            });
        }
        Ok(CheckedCoreIdentity {
            checked_program_identity: self.checked_program_identity.into_checked()?,
            operation_id: self.operation_id,
            fragment_kind,
            edge_kind,
            source_ref: self.source_ref.into_checked()?,
            dependency_ordinal: self.dependency_ordinal,
            designated_dependency: self
                .designated_dependency
                .map(SnapshotDesignatedRemoteInputDependency::into_checked)
                .transpose()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotAuthorityRequirements {
    requirements: SnapshotRuntimeSeamRequirements,
}

impl SnapshotAuthorityRequirements {
    fn from_requirements(value: &AuthorityRequirements) -> Self {
        Self {
            requirements: SnapshotRuntimeSeamRequirements::from_requirements(&value.requirements),
        }
    }

    fn into_requirements(self) -> Result<AuthorityRequirements, I3PrivateProjectionSnapshotError> {
        Ok(AuthorityRequirements {
            requirements: self.requirements.into_requirements()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotRuntimeSeamRequirements {
    rows: Vec<SnapshotRuntimeSeamRequirementRow>,
}

impl SnapshotRuntimeSeamRequirements {
    fn from_requirements(value: &RuntimeSeamRequirements) -> Self {
        Self {
            rows: value
                .rows
                .iter()
                .map(|(kind, obligation, provenance, authority)| {
                    SnapshotRuntimeSeamRequirementRow {
                        kind: (*kind).into(),
                        obligation: obligation
                            .as_ref()
                            .map(SnapshotGeneratedObligationKind::from_checked),
                        provenance: (*provenance).into(),
                        authority: authority.map(Into::into),
                    }
                })
                .collect(),
        }
    }

    fn into_requirements(
        self,
    ) -> Result<RuntimeSeamRequirements, I3PrivateProjectionSnapshotError> {
        Ok(RuntimeSeamRequirements {
            rows: self
                .rows
                .into_iter()
                .map(|row| {
                    Ok((
                        row.kind.into(),
                        row.obligation
                            .map(SnapshotGeneratedObligationKind::into_checked),
                        row.provenance.into(),
                        row.authority.map(Into::into),
                    ))
                })
                .collect::<Result<_, I3PrivateProjectionSnapshotError>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotRuntimeSeamRequirementRow {
    kind: SnapshotRuntimeSeamRequirementKind,
    obligation: Option<SnapshotGeneratedObligationKind>,
    provenance: SnapshotCarrierProvenanceKind,
    authority: Option<SnapshotSeamAuthorityKind>,
}

macro_rules! snapshot_unit_enum {
    ($snapshot:ident, $source:ident, { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum $snapshot { $($variant),+ }
        impl From<$source> for $snapshot {
            fn from(value: $source) -> Self {
                match value { $( $source::$variant => Self::$variant ),+ }
            }
        }
        impl From<$snapshot> for $source {
            fn from(value: $snapshot) -> Self {
                match value { $( $snapshot::$variant => Self::$variant ),+ }
            }
        }
    };
}

snapshot_unit_enum!(
    SnapshotProjectedOperationFragmentKind,
    ProjectedOperationFragmentKind,
    {
        OwnerRequestInvocation,
        OwnerRmwExecution,
        RelationPublication,
        ConsumerLocalRelationProjection,
        DesignatedRemoteInputService,
        DesignatedEvaluation,
        DesignatedResultConsumer,
    }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCommunicationPlan {
    edges: Vec<SnapshotCommunicationEdge>,
}

impl SnapshotCommunicationPlan {
    fn from_plan(value: &CommunicationPlan) -> Self {
        Self {
            edges: value
                .edges
                .iter()
                .map(SnapshotCommunicationEdge::from_edge)
                .collect(),
        }
    }

    fn into_plan(self) -> Result<CommunicationPlan, I3PrivateProjectionSnapshotError> {
        Ok(CommunicationPlan {
            edges: self
                .edges
                .into_iter()
                .map(SnapshotCommunicationEdge::into_edge)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCommunicationEdge {
    operation: String,
    kind: SnapshotCommunicationEdgeKind,
    source_locus: String,
    target_locus: String,
    core_ref: Option<String>,
    source_ref: SnapshotSourceRef,
    derived_from_checked_core: bool,
    transfers_authority: bool,
    edge_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    checked_core_identity: SnapshotCheckedCoreIdentity,
    carrier_contract: SnapshotCarrierContract,
    designated_remote_input_requirement: Option<SnapshotProjectedDesignatedRemoteInputRequirement>,
}

impl SnapshotCommunicationEdge {
    fn from_edge(value: &CommunicationEdge) -> Self {
        Self {
            operation: value.operation.clone(),
            kind: value.kind.into(),
            source_locus: value.source_locus.clone(),
            target_locus: value.target_locus.clone(),
            core_ref: value.core_ref.clone(),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            derived_from_checked_core: value.derived_from_checked_core,
            transfers_authority: value.transfers_authority,
            edge_ref: value.edge_ref.clone(),
            source_fragment_ref: value.source_fragment_ref.clone(),
            target_fragment_ref: value.target_fragment_ref.clone(),
            checked_core_identity: SnapshotCheckedCoreIdentity::from_identity(
                &value.checked_core_identity,
            ),
            carrier_contract: SnapshotCarrierContract::from_contract(&value.carrier_contract),
            designated_remote_input_requirement: value
                .designated_remote_input_requirement
                .as_ref()
                .map(SnapshotProjectedDesignatedRemoteInputRequirement::from_requirement),
        }
    }

    fn into_edge(self) -> Result<CommunicationEdge, I3PrivateProjectionSnapshotError> {
        let kind: CommunicationEdgeKind = self.kind.into();
        if kind == CommunicationEdgeKind::AbsoluteValueStream {
            return Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
                kind: "absolute value stream carrier",
            });
        }
        Ok(CommunicationEdge {
            operation: self.operation,
            kind,
            source_locus: self.source_locus,
            target_locus: self.target_locus,
            core_ref: self.core_ref,
            source_ref: self.source_ref.into_checked()?,
            derived_from_checked_core: self.derived_from_checked_core,
            transfers_authority: self.transfers_authority,
            edge_ref: self.edge_ref,
            source_fragment_ref: self.source_fragment_ref,
            target_fragment_ref: self.target_fragment_ref,
            checked_core_identity: self.checked_core_identity.into_identity()?,
            carrier_contract: self.carrier_contract.into_contract()?,
            designated_remote_input_requirement: self
                .designated_remote_input_requirement
                .map(SnapshotProjectedDesignatedRemoteInputRequirement::into_requirement)
                .transpose()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectedDesignatedRemoteInputRequirement {
    producer_locus: String,
    evaluator: String,
    result: String,
    dependency_ordinal: usize,
    trigger_frontier: String,
}

impl SnapshotProjectedDesignatedRemoteInputRequirement {
    fn from_requirement(value: &ProjectedDesignatedRemoteInputRequirement) -> Self {
        Self {
            producer_locus: value.producer_locus.clone(),
            evaluator: value.evaluator.clone(),
            result: value.result.clone(),
            dependency_ordinal: value.dependency_ordinal,
            trigger_frontier: value.trigger_frontier.clone(),
        }
    }

    fn into_requirement(
        self,
    ) -> Result<ProjectedDesignatedRemoteInputRequirement, I3PrivateProjectionSnapshotError> {
        if self.producer_locus.is_empty()
            || self.evaluator.is_empty()
            || self.result.is_empty()
            || self.trigger_frontier.is_empty()
        {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "designated remote input requirement fields must be present",
            });
        }
        Ok(ProjectedDesignatedRemoteInputRequirement {
            producer_locus: self.producer_locus,
            evaluator: self.evaluator,
            result: self.result,
            dependency_ordinal: self.dependency_ordinal,
            trigger_frontier: self.trigger_frontier,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCarrierContract {
    edge_kind: SnapshotCommunicationEdgeKind,
    lifecycle_kind: SnapshotCarrierLifecycleKind,
    operation_identity_template: String,
    request_identity_operation_id: String,
    request_identity_source_ref: SnapshotSourceRef,
    source_ref: SnapshotSourceRef,
    core_ref: Option<String>,
    origin_principal_template: Option<String>,
    origin_locus_template: Option<String>,
    target_owner_locus_template: Option<String>,
    declared_failure_row: SnapshotFailureRow,
    effect_row: Vec<SnapshotEffectKind>,
    authority_requirements: SnapshotAuthorityRequirements,
    occurrence_slots: Vec<SnapshotCarrierOccurrenceSlotKind>,
    frontiers: Vec<SnapshotCarrierFrontierKind>,
    linked_request_identity: bool,
    typed_outcome: bool,
    evaluator_receipt_consumption: bool,
    designated_dependency: Option<SnapshotDesignatedRemoteInputDependency>,
    reference_only_redaction: bool,
    checked_core_bound: bool,
    designated_result_details: Option<SnapshotCarrierDesignatedResultDetails>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCarrierDesignatedResultDetails {
    result_version: u64,
    input_frontier: Vec<String>,
    result_frontier: Vec<String>,
    observation_policy_name: String,
    policy_stamp_evaluation_name: String,
    policy_stamp_evaluation_deterministic: bool,
    policy_stamp_observation_name: String,
    retry_contract: SnapshotStaticRetryContractKind,
}

impl SnapshotCarrierDesignatedResultDetails {
    fn from_details(value: &DesignatedResultCarrierDetails) -> Self {
        Self {
            result_version: value.result_version.value(),
            input_frontier: value
                .input_frontier
                .as_slice()
                .iter()
                .map(|id| id.as_str().to_string())
                .collect(),
            result_frontier: value
                .result_frontier
                .as_slice()
                .iter()
                .map(|id| id.as_str().to_string())
                .collect(),
            observation_policy_name: value.observation_policy.name.clone(),
            policy_stamp_evaluation_name: value.policy_stamp.evaluation_policy.name.clone(),
            policy_stamp_evaluation_deterministic: value
                .policy_stamp
                .evaluation_policy
                .deterministic,
            policy_stamp_observation_name: value.policy_stamp.observation_policy.name.clone(),
            retry_contract: value.retry_contract.into(),
        }
    }

    fn into_details(
        self,
    ) -> Result<DesignatedResultCarrierDetails, I3PrivateProjectionSnapshotError> {
        let input_frontier = InputFrontier::from_ordered_producers(
            self.input_frontier
                .into_iter()
                .map(MaterializationOccurrenceId::new)
                .collect(),
        )
        .map_err(|_| I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "invalid designated carrier input frontier",
        })?;
        let result_frontier = ResultFrontier::from_ordered_results(
            self.result_frontier
                .into_iter()
                .map(ResultKey::new)
                .collect(),
        )
        .map_err(|_| I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "invalid designated carrier result frontier",
        })?;
        let observation_policy = ObservationPolicy {
            name: self.observation_policy_name,
        };
        let policy_stamp = PolicyStamp {
            evaluation_policy: mir_semantics::evaluation_materialization::EvaluationPolicy {
                name: self.policy_stamp_evaluation_name,
                deterministic: self.policy_stamp_evaluation_deterministic,
            },
            observation_policy: ObservationPolicy {
                name: self.policy_stamp_observation_name,
            },
        };
        if policy_stamp.observation_policy != observation_policy {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "designated carrier policy stamp must agree with observation policy",
            });
        }
        Ok(DesignatedResultCarrierDetails {
            result_version: ResultVersion::new(self.result_version),
            input_frontier,
            result_frontier,
            observation_policy,
            policy_stamp,
            retry_contract: self.retry_contract.into(),
        })
    }
}

snapshot_unit_enum!(SnapshotStaticRetryContractKind, StaticRetryContractKind, {
    ReturnExistingNoNewConsumption
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotEffectHandlerPlan {
    handlers: Vec<SnapshotEffectHandlerPlanEntry>,
}

impl SnapshotEffectHandlerPlan {
    fn from_plan(value: &EffectHandlerPlan) -> Self {
        Self {
            handlers: value
                .handlers
                .iter()
                .map(SnapshotEffectHandlerPlanEntry::from_entry)
                .collect(),
        }
    }

    fn into_plan(self) -> Result<EffectHandlerPlan, I3PrivateProjectionSnapshotError> {
        Ok(EffectHandlerPlan {
            handlers: self
                .handlers
                .into_iter()
                .map(SnapshotEffectHandlerPlanEntry::into_entry)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotEffectHandlerPlanEntry {
    operation: String,
    kind: SnapshotEffectHandlerKind,
    locus: String,
    source_ref: SnapshotSourceRef,
    core_ref: Option<String>,
    effect_row: Vec<SnapshotEffectKind>,
    declared_failure_row: SnapshotFailureRow,
    generated_failure_row: SnapshotFailureRow,
    source_bound: bool,
    handler_ref: String,
    checked_core_identity: SnapshotCheckedCoreIdentity,
}

impl SnapshotEffectHandlerPlanEntry {
    fn from_entry(value: &EffectHandlerPlanEntry) -> Self {
        Self {
            operation: value.operation.clone(),
            kind: value.kind.into(),
            locus: value.locus.clone(),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            core_ref: value.core_ref.clone(),
            effect_row: value
                .effect_row
                .kinds
                .iter()
                .copied()
                .map(SnapshotEffectKind::from_checked)
                .collect(),
            declared_failure_row: SnapshotFailureRow::from_checked(&value.declared_failure_row),
            generated_failure_row: SnapshotFailureRow::from_checked(&value.generated_failure_row),
            source_bound: value.source_bound,
            handler_ref: value.handler_ref.clone(),
            checked_core_identity: SnapshotCheckedCoreIdentity::from_identity(
                &value.checked_core_identity,
            ),
        }
    }

    fn into_entry(self) -> Result<EffectHandlerPlanEntry, I3PrivateProjectionSnapshotError> {
        if !self.source_bound {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "effect handler must remain source-bound",
            });
        }
        Ok(EffectHandlerPlanEntry {
            operation: self.operation,
            kind: self.kind.into(),
            locus: self.locus,
            source_ref: self.source_ref.into_checked()?,
            core_ref: self.core_ref,
            effect_row: ProjectedEffectRow {
                kinds: self
                    .effect_row
                    .into_iter()
                    .map(SnapshotEffectKind::into_checked)
                    .collect(),
            },
            declared_failure_row: self.declared_failure_row.into_checked()?,
            generated_failure_row: self.generated_failure_row.into_checked()?,
            source_bound: self.source_bound,
            handler_ref: self.handler_ref,
            checked_core_identity: self.checked_core_identity.into_identity()?,
        })
    }
}

snapshot_unit_enum!(
    SnapshotEffectHandlerKind,
    EffectHandlerKind,
    { OwnerService, DesignatedSourceService, DesignatedEvaluator }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectionRelationGraph {
    claim: SnapshotRelationGraphClaim,
    relations: Vec<SnapshotProjectedRelationEntry>,
    typed_dependency_edges: Vec<SnapshotRelationGraphEdgeSeed>,
    test_only_extension_boundary: bool,
}

impl SnapshotProjectionRelationGraph {
    fn from_graph(
        value: &ProjectionRelationGraph,
    ) -> Result<Self, I3PrivateProjectionSnapshotError> {
        Ok(Self {
            claim: value.claim.into(),
            relations: value
                .relations
                .iter()
                .map(|(key, relation)| {
                    Ok(SnapshotProjectedRelationEntry {
                        key: key.clone(),
                        value: SnapshotProjectedRelation::from_relation(relation),
                    })
                })
                .collect::<Result<_, I3PrivateProjectionSnapshotError>>()?,
            typed_dependency_edges: value
                .typed_dependency_edges
                .iter()
                .map(SnapshotRelationGraphEdgeSeed::from_seed)
                .collect(),
            test_only_extension_boundary: value.test_only_extension_boundary,
        })
    }

    fn into_graph(self) -> Result<ProjectionRelationGraph, I3PrivateProjectionSnapshotError> {
        Ok(ProjectionRelationGraph {
            claim: self.claim.into(),
            relations: collect_entries(self.relations, "relation_graph.relations", |entry| {
                Ok((entry.key, entry.value.into_relation()?))
            })?,
            typed_dependency_edges: self
                .typed_dependency_edges
                .into_iter()
                .map(SnapshotRelationGraphEdgeSeed::into_seed)
                .collect::<Result<_, _>>()?,
            test_only_extension_boundary: self.test_only_extension_boundary,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectedRelationEntry {
    key: String,
    value: SnapshotProjectedRelation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectedRelation {
    name: String,
    owner_locus: String,
    subject: String,
    subject_type: String,
    primary_anchor: SnapshotProjectedRelationAnchor,
    fallback_anchor: SnapshotProjectedRelationAnchor,
    binding_frontier: Vec<String>,
    consumer_locus: Option<String>,
    residual_source_refs: Vec<SnapshotResidualSourceRef>,
}

impl SnapshotProjectedRelation {
    fn from_relation(value: &ProjectedRelation) -> Self {
        Self {
            name: value.name.clone(),
            owner_locus: value.owner_locus.clone(),
            subject: value.subject.clone(),
            subject_type: value.subject_type.clone(),
            primary_anchor: SnapshotProjectedRelationAnchor::from_anchor(&value.primary_anchor),
            fallback_anchor: SnapshotProjectedRelationAnchor::from_anchor(&value.fallback_anchor),
            binding_frontier: value
                .binding_frontier
                .as_slice()
                .iter()
                .map(|id| id.as_str().to_string())
                .collect(),
            consumer_locus: value.consumer_locus.clone(),
            residual_source_refs: value
                .residual_source_refs
                .iter()
                .map(|(kind, source_ref)| SnapshotResidualSourceRef {
                    kind: (*kind).into(),
                    source_ref: SnapshotSourceRef::from_checked(source_ref),
                })
                .collect(),
        }
    }

    fn into_relation(self) -> Result<ProjectedRelation, I3PrivateProjectionSnapshotError> {
        let binding_frontier = BindingActivationFrontier::from_ordered_occurrences(
            self.binding_frontier
                .into_iter()
                .map(SharedOccurrenceId::new)
                .collect(),
        )
        .map_err(|_| I3PrivateProjectionSnapshotError::StructuralMismatch {
            reason: "invalid projected relation binding frontier",
        })?;
        Ok(ProjectedRelation {
            name: self.name,
            owner_locus: self.owner_locus,
            subject: self.subject,
            subject_type: self.subject_type,
            primary_anchor: self.primary_anchor.into_anchor()?,
            fallback_anchor: self.fallback_anchor.into_anchor()?,
            binding_frontier,
            consumer_locus: self.consumer_locus,
            residual_source_refs: self
                .residual_source_refs
                .into_iter()
                .map(|row| Ok((row.kind.into(), row.source_ref.into_checked()?)))
                .collect::<Result<_, I3PrivateProjectionSnapshotError>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectedRelationAnchor {
    anchor: String,
    anchor_locus: Option<String>,
    anchor_locus_source_ref: Option<SnapshotSourceRef>,
    epoch: String,
    transform: SnapshotRelationTransformCore,
    source_ref: SnapshotSourceRef,
}

impl SnapshotProjectedRelationAnchor {
    fn from_anchor(value: &ProjectedRelationAnchor) -> Self {
        Self {
            anchor: value.anchor.clone(),
            anchor_locus: value.anchor_locus.clone(),
            anchor_locus_source_ref: value
                .anchor_locus_source_ref
                .as_ref()
                .map(SnapshotSourceRef::from_checked),
            epoch: value.epoch.clone(),
            transform: SnapshotRelationTransformCore::from_checked(&value.transform),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
        }
    }

    fn into_anchor(self) -> Result<ProjectedRelationAnchor, I3PrivateProjectionSnapshotError> {
        if self.anchor_locus.is_some() != self.anchor_locus_source_ref.is_some() {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "projected relation anchor locus and source reference must co-occur",
            });
        }
        Ok(ProjectedRelationAnchor {
            anchor: self.anchor,
            anchor_locus: self.anchor_locus,
            anchor_locus_source_ref: self
                .anchor_locus_source_ref
                .map(SnapshotSourceRef::into_checked)
                .transpose()?,
            epoch: self.epoch,
            transform: self.transform.into_checked()?,
            source_ref: self.source_ref.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotResidualSourceRef {
    kind: SnapshotResidualObligationKind,
    source_ref: SnapshotSourceRef,
}

snapshot_unit_enum!(SnapshotRelationGraphClaim, RelationGraphClaim, {
    FiniteTypedExtensionBoundary
});
snapshot_unit_enum!(
    SnapshotRelationAnchorRole,
    RelationAnchorRole,
    { Primary, Fallback }
);
snapshot_unit_enum!(
    SnapshotRelationGraphEdgeTag,
    RelationGraphEdgeTag,
    { CheckedTwoAnchorFallback, TestOnlyTypedExtensionPressure }
);
snapshot_unit_enum!(
    SnapshotResidualObligationKind,
    ResidualObligationKind,
    {
        Visibility,
        RelationLifetime,
        FallbackValidity,
        ValueVisibilityRedaction,
        AuthDeferred,
        VerifyDeferred,
    }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotRelationGraphNode {
    relation: String,
    role: SnapshotRelationAnchorRole,
    source_ref: SnapshotSourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotRelationGraphEdgeSeed {
    from: SnapshotRelationGraphNode,
    to: SnapshotRelationGraphNode,
    tag: SnapshotRelationGraphEdgeTag,
}

impl SnapshotRelationGraphEdgeSeed {
    fn from_seed(value: &RelationGraphEdgeSeed) -> Self {
        Self {
            from: SnapshotRelationGraphNode {
                relation: value.from.relation.clone(),
                role: value.from.role.into(),
                source_ref: SnapshotSourceRef::from_checked(&value.from.source_ref),
            },
            to: SnapshotRelationGraphNode {
                relation: value.to.relation.clone(),
                role: value.to.role.into(),
                source_ref: SnapshotSourceRef::from_checked(&value.to.source_ref),
            },
            tag: value.tag.into(),
        }
    }

    fn into_seed(self) -> Result<RelationGraphEdgeSeed, I3PrivateProjectionSnapshotError> {
        Ok(RelationGraphEdgeSeed {
            from: RelationGraphNode {
                relation: self.from.relation,
                role: self.from.role.into(),
                source_ref: self.from.source_ref.into_checked()?,
            },
            to: RelationGraphNode {
                relation: self.to.relation,
                role: self.to.role.into(),
                source_ref: self.to.source_ref.into_checked()?,
            },
            tag: self.tag.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotPersistencePlan {
    by_locus: Vec<SnapshotPersistenceMapEntry>,
    by_relation: Vec<SnapshotPersistenceMapEntry>,
    by_designated: Vec<SnapshotPersistenceMapEntry>,
    global: Vec<SnapshotPersistenceResponsibilityKind>,
}

impl SnapshotPersistencePlan {
    fn from_plan(value: &PersistencePlan) -> Self {
        let entries = |rows: &BTreeMap<String, Vec<PersistenceResponsibilityKind>>| {
            rows.iter()
                .map(|(key, value)| SnapshotPersistenceMapEntry {
                    key: key.clone(),
                    value: value.iter().copied().map(Into::into).collect(),
                })
                .collect()
        };
        Self {
            by_locus: entries(&value.by_locus),
            by_relation: entries(&value.by_relation),
            by_designated: entries(&value.by_designated),
            global: value.global.iter().copied().map(Into::into).collect(),
        }
    }

    fn into_plan(self) -> Result<PersistencePlan, I3PrivateProjectionSnapshotError> {
        let restore = |entries: Vec<SnapshotPersistenceMapEntry>, map| {
            collect_entries(entries, map, |entry| {
                Ok((entry.key, entry.value.into_iter().map(Into::into).collect()))
            })
        };
        Ok(PersistencePlan {
            by_locus: restore(self.by_locus, "persistence.by_locus")?,
            by_relation: restore(self.by_relation, "persistence.by_relation")?,
            by_designated: restore(self.by_designated, "persistence.by_designated")?,
            global: self.global.into_iter().map(Into::into).collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotPersistenceMapEntry {
    key: String,
    value: Vec<SnapshotPersistenceResponsibilityKind>,
}

snapshot_unit_enum!(
    SnapshotPersistenceResponsibilityKind,
    PersistenceResponsibilityKind,
    {
        LocalStore,
        IncomingCarrierState,
        OutgoingCarrierState,
        MembershipCapabilityWitnessRefs,
        RelationBindingFrontier,
        RelationSelectedFallback,
        RelationResidualEvidenceRefs,
        DesignatedResultVersion,
        DesignatedReceiptConsumption,
        DesignatedInputFrontier,
        ResidualObligationState,
        LocalCut,
        PatchBoundary,
        PatchFrontier,
        ReceiptConsumption,
        OwnerQueue,
        DeclaredLocusBoundary,
        ConsumptionIdentity,
        InFlightDeliveryState,
    }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotObservationPlan {
    rows: Vec<SnapshotObservationRow>,
}

impl SnapshotObservationPlan {
    fn from_plan(value: &ObservationPlan) -> Self {
        Self {
            rows: value
                .rows
                .iter()
                .map(SnapshotObservationRow::from_row)
                .collect(),
        }
    }

    fn into_plan(self) -> Result<ObservationPlan, I3PrivateProjectionSnapshotError> {
        Ok(ObservationPlan {
            rows: self
                .rows
                .into_iter()
                .map(SnapshotObservationRow::into_row)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotObservationRow {
    source_ref: SnapshotSourceRef,
    core_ref: String,
    fragment_ref: String,
    redaction: String,
    observation_row_ref: String,
    edge_identity: Option<SnapshotEdgeIdentity>,
    edge_ref: Option<String>,
    operation_id: String,
    occurrence: SnapshotRuntimeOccurrenceKind,
}

impl SnapshotObservationRow {
    fn from_row(value: &ObservationRow) -> Self {
        Self {
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            core_ref: value.core_ref.clone(),
            fragment_ref: value.fragment_ref.clone(),
            redaction: value.redaction.to_string(),
            observation_row_ref: value.observation_row_ref.clone(),
            edge_identity: value
                .edge_identity
                .as_ref()
                .map(|identity| SnapshotEdgeIdentity {
                    operation: identity.0.clone(),
                    kind: identity.1.into(),
                    source_locus: identity.2.clone(),
                    target_locus: identity.3.clone(),
                }),
            edge_ref: value.edge_ref.clone(),
            operation_id: value.operation_id.clone(),
            occurrence: match &value.occurrence {
                RuntimeOccurrenceBinding::Required(kind) => kind.clone().into(),
            },
        }
    }

    fn into_row(self) -> Result<ObservationRow, I3PrivateProjectionSnapshotError> {
        if self.redaction != "reference-only" {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "observation rows must remain reference-only redacted",
            });
        }
        Ok(ObservationRow {
            source_ref: self.source_ref.into_checked()?,
            core_ref: self.core_ref,
            fragment_ref: self.fragment_ref,
            redaction: "reference-only",
            observation_row_ref: self.observation_row_ref,
            edge_identity: self.edge_identity.map(|identity| {
                (
                    identity.operation,
                    identity.kind.into(),
                    identity.source_locus,
                    identity.target_locus,
                )
            }),
            edge_ref: self.edge_ref,
            operation_id: self.operation_id,
            occurrence: RuntimeOccurrenceBinding::Required(self.occurrence.into()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotEdgeIdentity {
    operation: String,
    kind: SnapshotCommunicationEdgeKind,
    source_locus: String,
    target_locus: String,
}

snapshot_unit_enum!(
    SnapshotRuntimeOccurrenceKind,
    RuntimeOccurrenceKind,
    { Request, Serve, Reply, Receive, Publish, Observe, Consume }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCorrespondenceEntryMapEntry {
    key: String,
    value: SnapshotCorrespondenceEntry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCorrespondenceEntry {
    source_ref: SnapshotSourceRef,
    core_ref: Option<String>,
    artifact_ref: Option<String>,
    edge_ref: Option<String>,
    plan_ref: Option<String>,
    source_fragment_ref: Option<String>,
    target_fragment_ref: Option<String>,
    checked_core_identity: SnapshotCheckedCoreIdentity,
}

impl SnapshotCorrespondenceEntry {
    fn from_entry(value: &CorrespondenceEntry) -> Self {
        Self {
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            core_ref: value.core_ref.clone(),
            artifact_ref: value.artifact_ref.clone(),
            edge_ref: value.edge_ref.clone(),
            plan_ref: value.plan_ref.clone(),
            source_fragment_ref: value.source_fragment_ref.clone(),
            target_fragment_ref: value.target_fragment_ref.clone(),
            checked_core_identity: SnapshotCheckedCoreIdentity::from_identity(
                &value.checked_core_identity,
            ),
        }
    }

    fn into_entry(self) -> Result<CorrespondenceEntry, I3PrivateProjectionSnapshotError> {
        Ok(CorrespondenceEntry {
            source_ref: self.source_ref.into_checked()?,
            core_ref: self.core_ref,
            artifact_ref: self.artifact_ref,
            edge_ref: self.edge_ref,
            plan_ref: self.plan_ref,
            source_fragment_ref: self.source_fragment_ref,
            target_fragment_ref: self.target_fragment_ref,
            checked_core_identity: self.checked_core_identity.into_identity()?,
        })
    }
}

snapshot_unit_enum!(
    SnapshotStaticProjectionReadiness,
    StaticProjectionReadiness,
    { Ready }
);
snapshot_unit_enum!(
    SnapshotRuntimeAdmissionStatus,
    RuntimeAdmissionStatus,
    { AwaitingRuntimeSeam, BlockedByResidual }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotBackendRequirements {
    ow1: SnapshotBackendEligibility,
}

impl SnapshotBackendRequirements {
    fn from_requirements(value: &BackendRequirements) -> Self {
        Self {
            ow1: SnapshotBackendEligibility::from_eligibility(&value.ow1),
        }
    }

    fn into_requirements(self) -> Result<BackendRequirements, I3PrivateProjectionSnapshotError> {
        Ok(BackendRequirements {
            ow1: self.ow1.into_eligibility(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "reason",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum SnapshotBackendEligibility {
    Eligible,
    Ineligible(SnapshotBackendIneligibilityReason),
}

impl SnapshotBackendEligibility {
    fn from_eligibility(value: &BackendEligibility) -> Self {
        match value {
            BackendEligibility::Eligible => Self::Eligible,
            BackendEligibility::Ineligible { reason } => {
                Self::Ineligible(SnapshotBackendIneligibilityReason::from_reason(reason))
            }
        }
    }

    fn into_eligibility(self) -> BackendEligibility {
        match self {
            Self::Eligible => BackendEligibility::Eligible,
            Self::Ineligible(reason) => BackendEligibility::Ineligible {
                reason: reason.into_reason(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "count",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum SnapshotBackendIneligibilityReason {
    NoCombinedOwnerSourceOwnerLocus,
    MultipleCombinedOwnerSourceOwnerLoci(usize),
    Ow1WorkerCutDeferred,
}

impl SnapshotBackendIneligibilityReason {
    fn from_reason(value: &BackendIneligibilityReason) -> Self {
        match value {
            BackendIneligibilityReason::NoCombinedOwnerSourceOwnerLocus => {
                Self::NoCombinedOwnerSourceOwnerLocus
            }
            BackendIneligibilityReason::MultipleCombinedOwnerSourceOwnerLoci { count } => {
                Self::MultipleCombinedOwnerSourceOwnerLoci(*count)
            }
            BackendIneligibilityReason::Ow1WorkerCutDeferred => Self::Ow1WorkerCutDeferred,
        }
    }

    fn into_reason(self) -> BackendIneligibilityReason {
        match self {
            Self::NoCombinedOwnerSourceOwnerLocus => {
                BackendIneligibilityReason::NoCombinedOwnerSourceOwnerLocus
            }
            Self::MultipleCombinedOwnerSourceOwnerLoci(count) => {
                BackendIneligibilityReason::MultipleCombinedOwnerSourceOwnerLoci { count }
            }
            Self::Ow1WorkerCutDeferred => BackendIneligibilityReason::Ow1WorkerCutDeferred,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotConflictPolicyEntry {
    key: String,
    value: SnapshotDesignatedResultConsumerConflictPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotDesignatedResultConsumerConflictPolicy {
    kind: SnapshotStaticConflictPolicyKind,
    accepted_consumer_locus: String,
    on_competing_consumer: SnapshotStaticConflictResolution,
}

impl SnapshotDesignatedResultConsumerConflictPolicy {
    fn from_policy(value: &DesignatedResultConsumerConflictPolicy) -> Self {
        Self {
            kind: value.kind.into(),
            accepted_consumer_locus: value.accepted_consumer_locus.clone(),
            on_competing_consumer: value.on_competing_consumer.into(),
        }
    }

    fn into_policy(
        self,
    ) -> Result<DesignatedResultConsumerConflictPolicy, I3PrivateProjectionSnapshotError> {
        Ok(DesignatedResultConsumerConflictPolicy {
            kind: self.kind.into(),
            accepted_consumer_locus: self.accepted_consumer_locus,
            on_competing_consumer: self.on_competing_consumer.into(),
        })
    }
}

snapshot_unit_enum!(
    SnapshotStaticConflictPolicyKind,
    StaticConflictPolicyKind,
    { OneDesignatedResultConsumerFinite }
);
snapshot_unit_enum!(
    SnapshotStaticConflictResolution,
    StaticConflictResolution,
    { RejectCompetingConsumer }
);

impl SnapshotCarrierContract {
    fn from_contract(value: &CarrierContract) -> Self {
        Self {
            edge_kind: value.edge_kind.into(),
            lifecycle_kind: value.lifecycle_kind.into(),
            operation_identity_template: value.operation_identity_template.operation_id.clone(),
            request_identity_operation_id: value.request_identity_template.operation_id.clone(),
            request_identity_source_ref: SnapshotSourceRef::from_checked(
                &value.request_identity_template.source_ref,
            ),
            source_ref: SnapshotSourceRef::from_checked(&value.source_ref),
            core_ref: value.core_ref.clone(),
            origin_principal_template: value.origin_principal_template.clone(),
            origin_locus_template: value.origin_locus_template.clone(),
            target_owner_locus_template: value.target_owner_locus_template.clone(),
            declared_failure_row: SnapshotFailureRow::from_checked(&value.declared_failure_row),
            effect_row: value
                .effect_row
                .kinds
                .iter()
                .copied()
                .map(SnapshotEffectKind::from_checked)
                .collect(),
            authority_requirements: SnapshotAuthorityRequirements::from_requirements(
                &value.authority_requirements,
            ),
            occurrence_slots: value
                .occurrence_slots
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
            frontiers: value.frontiers.iter().copied().map(Into::into).collect(),
            linked_request_identity: value.linked_request_identity,
            typed_outcome: value.typed_outcome,
            evaluator_receipt_consumption: value.evaluator_receipt_consumption,
            designated_dependency: value
                .designated_dependency
                .as_ref()
                .map(SnapshotDesignatedRemoteInputDependency::from_checked),
            reference_only_redaction: value.visibility_policy.is_reference_only(),
            checked_core_bound: value.provenance.is_checked_core_bound(),
            designated_result_details: value
                .designated_result_details
                .as_ref()
                .map(SnapshotCarrierDesignatedResultDetails::from_details),
        }
    }

    fn into_contract(self) -> Result<CarrierContract, I3PrivateProjectionSnapshotError> {
        if !self.reference_only_redaction || !self.checked_core_bound {
            return Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "carrier contract must remain reference-only and checked-Core-bound",
            });
        }
        let edge_kind: CommunicationEdgeKind = self.edge_kind.into();
        if edge_kind == CommunicationEdgeKind::AbsoluteValueStream {
            return Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
                kind: "absolute value stream carrier contract",
            });
        }
        Ok(CarrierContract {
            edge_kind,
            lifecycle_kind: self.lifecycle_kind.into(),
            operation_identity_template: OperationIdentityTemplate {
                operation_id: self.operation_identity_template,
            },
            request_identity_template: RequestIdentityTemplate {
                operation_id: self.request_identity_operation_id,
                source_ref: self.request_identity_source_ref.into_checked()?,
            },
            source_ref: self.source_ref.into_checked()?,
            core_ref: self.core_ref,
            origin_principal_template: self.origin_principal_template,
            origin_locus_template: self.origin_locus_template,
            target_owner_locus_template: self.target_owner_locus_template,
            declared_failure_row: self.declared_failure_row.into_checked()?,
            effect_row: ProjectedEffectRow {
                kinds: self
                    .effect_row
                    .into_iter()
                    .map(SnapshotEffectKind::into_checked)
                    .collect(),
            },
            authority_requirements: self.authority_requirements.into_requirements()?,
            occurrence_slots: self.occurrence_slots.into_iter().map(Into::into).collect(),
            frontiers: collect_set(self.frontiers, "carrier_contract.frontiers", |frontier| {
                format!("{frontier:?}")
            })?
            .into_iter()
            .map(Into::into)
            .collect(),
            linked_request_identity: self.linked_request_identity,
            typed_outcome: self.typed_outcome,
            evaluator_receipt_consumption: self.evaluator_receipt_consumption,
            designated_dependency: self
                .designated_dependency
                .map(SnapshotDesignatedRemoteInputDependency::into_checked)
                .transpose()?,
            visibility_policy: ReferenceOnlyRedactionPolicy,
            provenance: CarrierContractProvenance::CheckedCoreBound,
            designated_result_details: self
                .designated_result_details
                .map(SnapshotCarrierDesignatedResultDetails::into_details)
                .transpose()?,
        })
    }
}
snapshot_unit_enum!(
    SnapshotCommunicationEdgeKind,
    CommunicationEdgeKind,
    {
        OwnerRequest,
        OwnerReplyReceipt,
        RelationProjectionPublication,
        DesignatedInputRequest,
        DesignatedInputReceipt,
        DesignatedResultDelivery,
        AbsoluteValueStream,
    }
);
snapshot_unit_enum!(
    SnapshotCarrierLifecycleKind,
    CarrierLifecycleKind,
    {
        OwnerRequest,
        OwnerReplyReceipt,
        DesignatedInputRequest,
        DesignatedInputReceipt,
        RelationProjectionPublication,
        DesignatedResultDelivery,
    }
);
snapshot_unit_enum!(
    SnapshotCarrierOccurrenceSlotKind,
    CarrierOccurrenceSlotKind,
    { Request, Serve, Reply, Receive, Publish, Observe, Consume }
);
snapshot_unit_enum!(
    SnapshotCarrierFrontierKind,
    CarrierFrontierKind,
    { Input, Result }
);
snapshot_unit_enum!(SnapshotCarrierProvenanceKind, CarrierProvenanceKind, {
    RequiredFromSealedRuntimeSeam
});
snapshot_unit_enum!(
    SnapshotRuntimeSeamRequirementKind,
    RuntimeSeamRequirementKind,
    {
        MembershipEpochIncarnation,
        LiveCapabilityRef,
        LiveWitnessRef,
        ProducerReleaseCapabilitySlot,
        ProducerReleaseWitnessSlot,
        EvaluatorDecisionAuthoritySlot,
        ConsumerMembershipEpochIncarnation,
        ConsumerCapabilityRef,
        ConsumerWitnessRef,
    }
);
snapshot_unit_enum!(
    SnapshotSeamAuthorityKind,
    SeamAuthorityKind,
    {
        MembershipEpochIncarnation,
        OwnerCapabilityRef,
        OwnerWitnessRef,
        ProducerReleaseCapability,
        ProducerReleaseWitness,
        EvaluatorDecisionAuthority,
        DesignatedResultConsumerMembership,
        DesignatedResultConsumerCapability,
        DesignatedResultConsumerWitness,
    }
);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectionIdentity {
    checked_program_identity: SnapshotCheckedProgramIdentity,
    topology_loci: Vec<String>,
    profile: String,
}

impl SnapshotProjectionIdentity {
    fn from_projection(value: &ProjectionIdentity) -> Self {
        Self {
            checked_program_identity: SnapshotCheckedProgramIdentity::from_checked(
                &value.checked_program_identity,
            ),
            topology_loci: value.topology_loci.iter().cloned().collect(),
            profile: value.profile.to_string(),
        }
    }

    fn into_projection(self) -> Result<ProjectionIdentity, I3PrivateProjectionSnapshotError> {
        if self.profile != "i2-internal-projection-v1" {
            return Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
                kind: "projection profile",
            });
        }
        let topology_loci = collect_set(
            self.topology_loci,
            "projection_identity.topology_loci",
            |locus| locus.clone(),
        )?;
        Ok(ProjectionIdentity {
            checked_program_identity: self.checked_program_identity.into_checked()?,
            topology_loci,
            profile: "i2-internal-projection-v1",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys3_projection::project_checked_core;
    use mir_ast::surface_v0::FixtureSource;
    use mir_semantics::surface_v0_pipeline::check_and_elaborate_surface_v0;

    const ACTIVE_I2_SOURCE: &str =
        include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

    fn active_projection() -> GlobalProjectionResult {
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(
            "samples/clean-near-end/mirrorea-i2-local-toy/main.mir",
            ACTIVE_I2_SOURCE,
        ))
        .expect("the canonical finite I2 source must check");
        let topology = DeclaredLogicalTopology::try_new(
            checked.program_identity().clone(),
            ["WorldAuthority", "ParticipantA", "ParticipantB", "ViewerC"],
        )
        .expect("the canonical finite I2 topology must be valid");
        project_checked_core(&checked, &topology).expect("the canonical finite I2 source projects")
    }

    #[test]
    fn private_projection_snapshot_json_round_trips_every_root_field() {
        let projection = active_projection();
        let snapshot = projection
            .to_i3_private_snapshot()
            .expect("the checked projection has a complete private snapshot");
        let encoded = serde_json::to_vec(&snapshot).expect("private snapshot encodes");
        let decoded: I3PrivateProjectionSnapshot =
            serde_json::from_slice(&encoded).expect("private snapshot decodes");
        let restored = GlobalProjectionResult::from_i3_private_snapshot(decoded)
            .expect("private snapshot restores without source parsing or lowering");
        assert_eq!(restored, projection);
    }

    #[test]
    fn private_projection_snapshot_rejects_unknown_fields_and_duplicate_map_keys() {
        let projection = active_projection();
        let snapshot = projection
            .to_i3_private_snapshot()
            .expect("the checked projection has a complete private snapshot");
        let mut json = serde_json::to_value(&snapshot).expect("private snapshot value encodes");
        json.as_object_mut()
            .expect("snapshot is an object")
            .insert("unexpected".to_string(), serde_json::Value::Null);
        assert!(serde_json::from_value::<I3PrivateProjectionSnapshot>(json).is_err());

        let mut duplicate = snapshot;
        duplicate
            .locus_programs
            .push(duplicate.locus_programs[0].clone());
        assert!(matches!(
            GlobalProjectionResult::from_i3_private_snapshot(duplicate),
            Err(I3PrivateProjectionSnapshotError::DuplicateMapKey {
                map: "locus_programs",
                ..
            })
        ));
    }

    #[test]
    fn private_projection_snapshot_rejects_host_source_paths() {
        let projection = active_projection();
        let snapshot = projection
            .to_i3_private_snapshot()
            .expect("the checked projection has a complete private snapshot");
        let mut json = serde_json::to_value(snapshot).expect("private snapshot value encodes");
        let root = json.as_object_mut().expect("snapshot is an object");
        root.get_mut("checked_program_identity")
            .and_then(serde_json::Value::as_object_mut)
            .expect("checked identity is an object")
            .insert(
                "source_file".to_string(),
                serde_json::Value::String("/host/private/main.mir".to_string()),
            );
        root.get_mut("projection_identity")
            .and_then(serde_json::Value::as_object_mut)
            .and_then(|identity| identity.get_mut("checked_program_identity"))
            .and_then(serde_json::Value::as_object_mut)
            .expect("projection checked identity is an object")
            .insert(
                "source_file".to_string(),
                serde_json::Value::String("/host/private/main.mir".to_string()),
            );
        let decoded: I3PrivateProjectionSnapshot =
            serde_json::from_value(json).expect("the syntactically valid private snapshot decodes");
        assert!(matches!(
            GlobalProjectionResult::from_i3_private_snapshot(decoded),
            Err(I3PrivateProjectionSnapshotError::StructuralMismatch {
                reason: "private projection snapshot rejects a host source file path"
            })
        ));
    }

    #[test]
    fn private_projection_snapshot_rejects_inactive_absolute_value_stream_variant() {
        let projection = active_projection();
        let snapshot = projection
            .to_i3_private_snapshot()
            .expect("the checked projection has a complete private snapshot");
        let mut json = serde_json::to_value(snapshot).expect("private snapshot value encodes");
        let edge = json
            .as_object_mut()
            .and_then(|root| root.get_mut("communication_plan"))
            .and_then(serde_json::Value::as_object_mut)
            .and_then(|plan| plan.get_mut("edges"))
            .and_then(serde_json::Value::as_array_mut)
            .and_then(|edges| edges.first_mut())
            .and_then(serde_json::Value::as_object_mut)
            .expect("the canonical projection retains one communication edge");
        edge.insert(
            "kind".to_string(),
            serde_json::Value::String("absolute_value_stream".to_string()),
        );
        let decoded: I3PrivateProjectionSnapshot =
            serde_json::from_value(json).expect("the syntactically known inactive variant decodes");
        assert!(matches!(
            GlobalProjectionResult::from_i3_private_snapshot(decoded),
            Err(I3PrivateProjectionSnapshotError::UnsupportedVariant {
                kind: "absolute value stream carrier"
            })
        ));
    }

    #[test]
    fn private_projection_snapshot_rejects_unknown_schema_version() {
        let projection = active_projection();
        let mut snapshot = projection
            .to_i3_private_snapshot()
            .expect("the checked projection has a complete private snapshot");
        snapshot.version += 1;
        assert!(matches!(
            GlobalProjectionResult::from_i3_private_snapshot(snapshot),
            Err(I3PrivateProjectionSnapshotError::UnsupportedVersion { .. })
        ));
    }

    #[test]
    fn private_projection_snapshot_round_trips_consumer_only_restricted_relation_graph() {
        let projection = active_projection();
        let assigned_loci = ["ParticipantA", "ViewerC"]
            .into_iter()
            .map(str::to_string)
            .collect::<BTreeSet<_>>();
        let restricted = projection.restricted_to_loci(&assigned_loci);
        assert!(restricted.locus_program("WorldAuthority").is_none());
        assert!(
            restricted
                .relation_graph()
                .relation("bird_follow")
                .is_some()
        );
        let snapshot = restricted
            .to_i3_private_snapshot()
            .expect("consumer-only relation graph remains snapshotable");
        let encoded = serde_json::to_vec(&snapshot).expect("restricted snapshot encodes");
        let decoded: I3PrivateProjectionSnapshot =
            serde_json::from_slice(&encoded).expect("restricted snapshot decodes");
        let restored = GlobalProjectionResult::from_i3_private_snapshot(decoded)
            .expect("restricted snapshot restores without an owner-fragment lookup");
        assert_eq!(restored, restricted);
    }
}
