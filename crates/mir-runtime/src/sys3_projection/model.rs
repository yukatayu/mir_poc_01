use std::collections::{BTreeMap, BTreeSet};

use mir_semantics::{
    evaluation_materialization::{InputFrontier, ObservationPolicy, PolicyStamp},
    shared_model::{BindingActivationFrontier, SourceRef},
    shared_model::{ResultFrontier, ResultVersion},
    surface_v0_pipeline::{
        CheckedEvaluation, CheckedEvaluationSignature, CheckedIndexedStateSchema,
        CheckedProgramIdentity, DesignatedCheckedCore, DesignatedRemoteInputDependency,
        DesignatedResultConsumerCore, EffectKind, FailureRow, GeneratedObligationKind,
        OwnerRmwCheckedCore, RelationCheckedCore, RelationTransformCore, ResidualObligationKind,
        StaticRetryContractKind, TypedStateRead,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ProjectionDiagnosticKind {
    DuplicateLocus,
    MissingRequiredLocus,
    UnknownDeclaredLocus,
    CheckedProgramIdentityMismatch,
    MissingDerivedEdge,
    ExtraNonDerivedEdge,
    OwnerOperationMoved,
    SourceMapMismatch,
    RelationGraphCycle,
    ForeignCheckedProgramRelationDependency,
    BackendEligibilityMismatch,
    PersistencePlanMismatch,
    EffectHandlerProvenanceMismatch,
    MissingDerivedFragment,
    DesignatedResultConsumerMoved,
    DesignatedResultConsumerExpressionLeakage,
    StructuralMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectionDiagnostic {
    kind: ProjectionDiagnosticKind,
    detail: String,
}

impl ProjectionDiagnostic {
    pub(crate) fn new(kind: ProjectionDiagnosticKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub(crate) const fn kind(&self) -> ProjectionDiagnosticKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectionDiagnostics {
    entries: Vec<ProjectionDiagnostic>,
}

impl ProjectionDiagnostics {
    pub(crate) fn one(kind: ProjectionDiagnosticKind, detail: impl Into<String>) -> Self {
        Self {
            entries: vec![ProjectionDiagnostic::new(kind, detail)],
        }
    }

    pub(crate) fn primary(&self) -> &ProjectionDiagnostic {
        self.entries
            .first()
            .expect("projection diagnostics always have a primary entry")
    }

    pub(crate) const fn partial_result(&self) -> Option<()> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeclaredLogicalTopology {
    checked_program_identity: CheckedProgramIdentity,
    loci: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LocusTag {
    name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SourceRefView(SourceRef);

impl SourceRefView {
    fn new(source_ref: &SourceRef) -> Self {
        Self(source_ref.clone())
    }
}

impl std::ops::Deref for SourceRefView {
    type Target = SourceRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for SourceRefView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for SourceRefView {}

impl PartialEq<SourceRef> for SourceRefView {
    fn eq(&self, other: &SourceRef) -> bool {
        self.0 == *other
    }
}

impl PartialEq<&SourceRef> for SourceRefView {
    fn eq(&self, other: &&SourceRef) -> bool {
        self.0 == **other
    }
}

impl LocusTag {
    pub(crate) fn checked(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedCoreIdentity {
    checked_program_identity: CheckedProgramIdentity,
    operation_id: String,
    fragment_kind: Option<ProjectedOperationFragmentKind>,
    edge_kind: Option<CommunicationEdgeKind>,
    source_ref: SourceRef,
    dependency_ordinal: Option<usize>,
    designated_dependency: Option<DesignatedRemoteInputDependency>,
}

impl CheckedCoreIdentity {
    pub(super) fn fragment(
        checked_program_identity: CheckedProgramIdentity,
        operation_id: impl Into<String>,
        fragment_kind: ProjectedOperationFragmentKind,
        source_ref: SourceRef,
        dependency_ordinal: Option<usize>,
        designated_dependency: Option<DesignatedRemoteInputDependency>,
    ) -> Self {
        Self {
            checked_program_identity,
            operation_id: operation_id.into(),
            fragment_kind: Some(fragment_kind),
            edge_kind: None,
            source_ref,
            dependency_ordinal,
            designated_dependency,
        }
    }

    pub(super) fn edge(
        checked_program_identity: CheckedProgramIdentity,
        operation_id: impl Into<String>,
        edge_kind: CommunicationEdgeKind,
        source_ref: SourceRef,
        dependency_ordinal: Option<usize>,
        designated_dependency: Option<DesignatedRemoteInputDependency>,
    ) -> Self {
        Self {
            checked_program_identity,
            operation_id: operation_id.into(),
            fragment_kind: None,
            edge_kind: Some(edge_kind),
            source_ref,
            dependency_ordinal,
            designated_dependency,
        }
    }

    pub(crate) fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        &self.checked_program_identity
    }
    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }
    pub(crate) fn fragment_kind(&self) -> Option<ProjectedOperationFragmentKind> {
        self.fragment_kind
    }
    pub(crate) fn edge_kind(&self) -> Option<CommunicationEdgeKind> {
        self.edge_kind
    }
    pub(crate) fn source_ref(&self) -> SourceRefView {
        SourceRefView::new(&self.source_ref)
    }
    pub(crate) fn dependency_ordinal(&self) -> Option<usize> {
        self.dependency_ordinal
    }
    pub(crate) fn designated_dependency(&self) -> Option<&DesignatedRemoteInputDependency> {
        self.designated_dependency.as_ref()
    }
}

impl DeclaredLogicalTopology {
    pub(crate) fn try_new<I, S>(
        checked_program_identity: CheckedProgramIdentity,
        loci: I,
    ) -> Result<Self, ProjectionDiagnostics>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut retained = BTreeSet::new();
        for locus in loci {
            let locus = locus.into();
            if !retained.insert(locus.clone()) {
                return Err(ProjectionDiagnostics::one(
                    ProjectionDiagnosticKind::DuplicateLocus,
                    format!("logical topology repeats locus {locus}"),
                ));
            }
        }
        Ok(Self {
            checked_program_identity,
            loci: retained,
        })
    }

    pub(crate) fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        &self.checked_program_identity
    }

    pub(crate) fn loci(&self) -> &BTreeSet<String> {
        &self.loci
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectionIdentity {
    checked_program_identity: CheckedProgramIdentity,
    topology_loci: BTreeSet<String>,
    profile: &'static str,
}

impl ProjectionIdentity {
    pub(crate) fn new(
        checked_program_identity: CheckedProgramIdentity,
        topology_loci: BTreeSet<String>,
    ) -> Self {
        Self {
            checked_program_identity,
            topology_loci,
            profile: "i2-internal-projection-v1",
        }
    }

    pub(crate) fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        &self.checked_program_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LocusOperationKind {
    OwnerRequestStub,
    OwnerRmwEvaluation,
    RelationPublication,
    ConsumerLocalProjection,
    DesignatedEvaluationExpression,
    DirectStoreMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ProjectedOperationFragmentKind {
    OwnerRequestInvocation,
    OwnerRmwExecution,
    RelationPublication,
    ConsumerLocalRelationProjection,
    DesignatedRemoteInputService,
    DesignatedEvaluation,
    DesignatedResultConsumer,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct AuthorityRequirements {
    requirements: RuntimeSeamRequirements,
}

impl AuthorityRequirements {
    pub(super) fn standard(_operation: &str, _source_ref: &SourceRef) -> Self {
        Self {
            requirements: RuntimeSeamRequirements::owner(),
        }
    }

    pub(super) fn designated(_operation: &str, _source_ref: &SourceRef) -> Self {
        Self {
            requirements: RuntimeSeamRequirements::designated(),
        }
    }

    pub(super) fn designated_result_consumer(_operation: &str, _source_ref: &SourceRef) -> Self {
        Self {
            requirements: RuntimeSeamRequirements::designated_result_consumer(),
        }
    }

    pub(super) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn requires_membership_epoch(&self) -> bool {
        self.has_kind(RuntimeSeamRequirementKind::MembershipEpochIncarnation)
    }

    pub(crate) fn requires_membership_epoch_and_incarnation(&self) -> bool {
        self.requires_membership_epoch()
    }

    pub(crate) fn requires_capability_ref(&self) -> bool {
        self.has_kind(RuntimeSeamRequirementKind::LiveCapabilityRef)
            || self.has_kind(RuntimeSeamRequirementKind::ProducerReleaseCapabilitySlot)
    }

    pub(crate) fn requires_witness_ref(&self) -> bool {
        self.has_kind(RuntimeSeamRequirementKind::LiveWitnessRef)
            || self.has_kind(RuntimeSeamRequirementKind::ProducerReleaseWitnessSlot)
    }

    pub(crate) fn requires_capability_and_witness_refs(&self) -> bool {
        self.requires_capability_ref() && self.requires_witness_ref()
    }

    pub(crate) fn requires_producer_release_tuple_slot(&self) -> bool {
        self.has_kind(RuntimeSeamRequirementKind::ProducerReleaseCapabilitySlot)
            && self.has_kind(RuntimeSeamRequirementKind::ProducerReleaseWitnessSlot)
    }

    pub(crate) fn requires_source_owner_release_authority(&self) -> bool {
        self.has_authority(SeamAuthorityKind::ProducerReleaseCapability)
    }

    pub(crate) fn requires_evaluator_authority(&self) -> bool {
        self.has_authority(SeamAuthorityKind::EvaluatorDecisionAuthority)
    }

    pub(crate) fn source_owner_release_authority_ref(&self) -> Option<SeamAuthorityKind> {
        self.requirements.rows().iter().find_map(|row| {
            (row.3 == Some(SeamAuthorityKind::ProducerReleaseCapability))
                .then_some(SeamAuthorityKind::ProducerReleaseCapability)
        })
    }

    pub(crate) fn evaluator_authority_ref(&self) -> Option<SeamAuthorityKind> {
        self.requirements.rows().iter().find_map(|row| {
            (row.3 == Some(SeamAuthorityKind::EvaluatorDecisionAuthority))
                .then_some(SeamAuthorityKind::EvaluatorDecisionAuthority)
        })
    }

    pub(crate) fn all_requirements_are_checked_core_bound_or(
        &self,
        provenance: CarrierProvenanceKind,
    ) -> bool {
        self.requirements
            .rows()
            .iter()
            .all(|row| row.2 == provenance)
    }

    pub(crate) fn runtime_seam_requirements(&self) -> &RuntimeSeamRequirements {
        &self.requirements
    }

    fn has_kind(&self, kind: RuntimeSeamRequirementKind) -> bool {
        self.requirements.rows().iter().any(|row| row.0 == kind)
    }

    fn has_authority(&self, authority: SeamAuthorityKind) -> bool {
        self.requirements
            .rows()
            .iter()
            .any(|row| row.3 == Some(authority))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConsumerRelationProjectionDescriptor {
    pub(super) source_relation: String,
    pub(super) owner_locus: String,
    pub(super) consumer_locus: String,
    pub(super) source_ref: SourceRef,
}

impl ConsumerRelationProjectionDescriptor {
    pub(crate) fn source_relation(&self) -> &str {
        &self.source_relation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PlacementSpecificCore {
    OwnerRequest {
        signature: CheckedEvaluationSignature,
        origin_locus: String,
        target_owner_locus: String,
    },
    OwnerRmw {
        core: OwnerRmwCheckedCore,
        local_state_schemas: Vec<CheckedIndexedStateSchema>,
    },
    RelationOwner {
        core: RelationCheckedCore,
    },
    RelationConsumer {
        descriptor: ConsumerRelationProjectionDescriptor,
    },
    DesignatedSource {
        dependency: DesignatedRemoteInputDependency,
    },
    DesignatedEvaluator {
        core: DesignatedCheckedCore,
    },
    DesignatedResultConsumer {
        core: DesignatedResultConsumerCore,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectedOperationFragment {
    pub(super) operation_id: String,
    pub(super) kind: ProjectedOperationFragmentKind,
    pub(super) source_ref: SourceRef,
    pub(super) core_ref: String,
    pub(super) artifact_ref: String,
    pub(super) authority_requirements: AuthorityRequirements,
    pub(super) declared_failure_row: FailureRow,
    pub(super) generated_failure_row: FailureRow,
    pub(super) placement: PlacementSpecificCore,
    pub(super) locus_tag: LocusTag,
    pub(super) fragment_ref: String,
    pub(super) checked_core_identity: CheckedCoreIdentity,
    pub(super) semantic_obligations: SemanticObligations,
    pub(super) runtime_seam_requirements: RuntimeSeamRequirements,
    pub(super) designated_result_consumer_expression_leakage: bool,
}

impl ProjectedOperationFragment {
    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub(crate) const fn fragment_kind(&self) -> ProjectedOperationFragmentKind {
        self.kind
    }

    pub(crate) fn origin_locus(&self) -> Option<&str> {
        match &self.placement {
            PlacementSpecificCore::OwnerRequest { origin_locus, .. } => Some(origin_locus),
            _ => None,
        }
    }

    pub(crate) fn target_owner_locus(&self) -> Option<&str> {
        match &self.placement {
            PlacementSpecificCore::OwnerRequest {
                target_owner_locus, ..
            } => Some(target_owner_locus),
            _ => None,
        }
    }

    pub(crate) fn typed_input_signature(&self) -> Option<&CheckedEvaluationSignature> {
        match &self.placement {
            PlacementSpecificCore::OwnerRequest { signature, .. } => Some(signature),
            _ => None,
        }
    }

    pub(crate) fn declared_failure_names(&self) -> Vec<String> {
        self.declared_failure_row.names()
    }

    pub(crate) fn generated_failure_names(&self) -> Vec<String> {
        self.generated_failure_row.names()
    }

    pub(crate) fn authority_requirements(&self) -> &AuthorityRequirements {
        &self.authority_requirements
    }

    pub(crate) fn core_ref(&self) -> Option<&str> {
        Some(&self.core_ref)
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn owner_rmw_checked_core(&self) -> Option<&OwnerRmwCheckedCore> {
        match &self.placement {
            PlacementSpecificCore::OwnerRmw { core, .. } => Some(core),
            _ => None,
        }
    }

    pub(crate) fn local_state_schemas(&self) -> &[CheckedIndexedStateSchema] {
        match &self.placement {
            PlacementSpecificCore::OwnerRmw {
                local_state_schemas,
                ..
            } => local_state_schemas,
            _ => &[],
        }
    }

    pub(crate) fn designated_remote_input_dependency(
        &self,
    ) -> Option<&DesignatedRemoteInputDependency> {
        match &self.placement {
            PlacementSpecificCore::DesignatedSource { dependency } => Some(dependency),
            _ => None,
        }
    }

    pub(crate) fn designated_checked_core(&self) -> Option<&DesignatedCheckedCore> {
        match &self.placement {
            PlacementSpecificCore::DesignatedEvaluator { core } => Some(core),
            _ => None,
        }
    }

    pub(crate) fn designated_result_consumer_core(&self) -> Option<&DesignatedResultConsumerCore> {
        match &self.placement {
            PlacementSpecificCore::DesignatedResultConsumer { core } => Some(core),
            _ => None,
        }
    }

    pub(crate) fn exposes_typed_expression(&self) -> bool {
        self.designated_checked_core().is_some()
            || self.designated_result_consumer_expression_leakage
    }

    pub(crate) const fn exposes_raw_input(&self) -> bool {
        false
    }

    pub(crate) fn static_retry_contract(&self) -> StaticRetryContractKind {
        self.designated_result_consumer_core()
            .expect("only a designated result consumer has a retry contract")
            .retry_contract()
    }

    pub(crate) fn relation_checked_core(&self) -> Option<&RelationCheckedCore> {
        match &self.placement {
            PlacementSpecificCore::RelationOwner { core } => Some(core),
            _ => None,
        }
    }

    pub(crate) fn consumer_relation_projection(
        &self,
    ) -> Option<&ConsumerRelationProjectionDescriptor> {
        match &self.placement {
            PlacementSpecificCore::RelationConsumer { descriptor } => Some(descriptor),
            _ => None,
        }
    }

    pub(crate) fn exposes_owner_rmw_checked_core(&self) -> bool {
        self.owner_rmw_checked_core().is_some()
    }

    pub(crate) const fn exposes_owner_expression(&self) -> bool {
        matches!(self.placement, PlacementSpecificCore::OwnerRmw { .. })
    }

    pub(crate) fn exposes_target_private_state_read(&self, read: &TypedStateRead) -> bool {
        self.owner_rmw_checked_core()
            .is_some_and(|core| core.target() == read || core.same_owner_reads().contains(read))
    }

    pub(crate) const fn exposes_designated_evaluator_expression(&self) -> bool {
        matches!(
            self.placement,
            PlacementSpecificCore::DesignatedEvaluator { .. }
        )
    }

    pub(crate) fn locus_tag(&self) -> &LocusTag {
        &self.locus_tag
    }
    pub(crate) fn locus_program_tag(&self) -> &LocusTag {
        &self.locus_tag
    }
    pub(crate) fn fragment_ref(&self) -> &str {
        &self.fragment_ref
    }
    pub(crate) fn checked_core_identity(&self) -> &CheckedCoreIdentity {
        &self.checked_core_identity
    }
    pub(crate) fn semantic_obligations(&self) -> &SemanticObligations {
        &self.semantic_obligations
    }
    pub(crate) fn runtime_seam_requirements(&self) -> &RuntimeSeamRequirements {
        &self.runtime_seam_requirements
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct SemanticObligations {
    rows: Vec<(GeneratedObligationKind, SourceRef)>,
}

impl SemanticObligations {
    pub(super) fn from_evaluation(evaluation: &CheckedEvaluation) -> Self {
        Self {
            rows: evaluation
                .generated_obligations()
                .entries()
                .iter()
                .map(|row| (row.kind().clone(), row.source_ref().clone()))
                .collect(),
        }
    }
    pub(crate) fn rows(&self) -> &[(GeneratedObligationKind, SourceRef)] {
        &self.rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeSeamRequirementKind {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SeamAuthorityKind {
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

pub(crate) type RuntimeSeamRequirementRow = (
    RuntimeSeamRequirementKind,
    Option<GeneratedObligationKind>,
    CarrierProvenanceKind,
    Option<SeamAuthorityKind>,
);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct RuntimeSeamRequirements {
    rows: Vec<RuntimeSeamRequirementRow>,
}

impl RuntimeSeamRequirements {
    pub(super) fn owner() -> Self {
        Self {
            rows: vec![
                (
                    RuntimeSeamRequirementKind::MembershipEpochIncarnation,
                    None,
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::MembershipEpochIncarnation),
                ),
                (
                    RuntimeSeamRequirementKind::LiveCapabilityRef,
                    Some(GeneratedObligationKind::Capability),
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::OwnerCapabilityRef),
                ),
                (
                    RuntimeSeamRequirementKind::LiveWitnessRef,
                    Some(GeneratedObligationKind::Witness),
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::OwnerWitnessRef),
                ),
            ],
        }
    }
    pub(super) fn designated() -> Self {
        Self {
            rows: vec![
                (
                    RuntimeSeamRequirementKind::MembershipEpochIncarnation,
                    None,
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::MembershipEpochIncarnation),
                ),
                (
                    RuntimeSeamRequirementKind::ProducerReleaseCapabilitySlot,
                    None,
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::ProducerReleaseCapability),
                ),
                (
                    RuntimeSeamRequirementKind::ProducerReleaseWitnessSlot,
                    None,
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::ProducerReleaseWitness),
                ),
                (
                    RuntimeSeamRequirementKind::EvaluatorDecisionAuthoritySlot,
                    Some(GeneratedObligationKind::AdmittedEvaluatorAuthority),
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::EvaluatorDecisionAuthority),
                ),
            ],
        }
    }
    pub(super) fn designated_result_consumer() -> Self {
        Self {
            rows: vec![
                (
                    RuntimeSeamRequirementKind::ConsumerMembershipEpochIncarnation,
                    Some(GeneratedObligationKind::DesignatedResultConsumerAuthority),
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::DesignatedResultConsumerMembership),
                ),
                (
                    RuntimeSeamRequirementKind::ConsumerCapabilityRef,
                    Some(GeneratedObligationKind::DesignatedResultConsumerAuthority),
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::DesignatedResultConsumerCapability),
                ),
                (
                    RuntimeSeamRequirementKind::ConsumerWitnessRef,
                    Some(GeneratedObligationKind::DesignatedResultConsumerAuthority),
                    CarrierProvenanceKind::RequiredFromSealedRuntimeSeam,
                    Some(SeamAuthorityKind::DesignatedResultConsumerWitness),
                ),
            ],
        }
    }
    pub(crate) fn rows(&self) -> &[RuntimeSeamRequirementRow] {
        &self.rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ProjectedOperationFragments {
    entries: Vec<ProjectedOperationFragment>,
}

impl ProjectedOperationFragments {
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub(crate) fn single(
        &self,
        operation: &str,
        kind: ProjectedOperationFragmentKind,
    ) -> Option<&ProjectedOperationFragment> {
        self.entries
            .iter()
            .find(|entry| entry.operation_id == operation && entry.kind == kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Sys4ArtifactFragments {
    entries: Vec<ProjectedOperationFragment>,
}

impl Sys4ArtifactFragments {
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn entries(&self) -> &[ProjectedOperationFragment] {
        &self.entries
    }

    pub(crate) fn single(
        &self,
        operation: &str,
        kind: ProjectedOperationFragmentKind,
    ) -> Option<&ProjectedOperationFragment> {
        self.entries
            .iter()
            .find(|entry| entry.operation_id == operation && entry.kind == kind)
    }

    pub(crate) fn all_fragments_are_self_describing_for_sys4_iteration(&self) -> bool {
        self.entries.iter().all(|fragment| {
            !fragment.operation_id.is_empty()
                && !fragment.artifact_ref.is_empty()
                && !fragment.core_ref.is_empty()
                && !fragment.source_ref.path.is_empty()
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ProjectedCheckedFragments {
    owner_operations: Vec<String>,
    local_state_schemas: Vec<CheckedIndexedStateSchema>,
}

impl ProjectedCheckedFragments {
    pub(crate) fn owner_operations(&self) -> Vec<&str> {
        self.owner_operations.iter().map(String::as_str).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocusProgram {
    locus: String,
    locus_tag: LocusTag,
    operations: ProjectedOperationFragments,
    checked_fragments: ProjectedCheckedFragments,
    declared_failures: BTreeMap<String, FailureRow>,
    generated_failures: BTreeMap<String, FailureRow>,
}

impl LocusProgram {
    pub(crate) fn new(locus_tag: LocusTag) -> Self {
        Self {
            locus: locus_tag.as_str().to_string(),
            locus_tag,
            operations: ProjectedOperationFragments::default(),
            checked_fragments: ProjectedCheckedFragments::default(),
            declared_failures: BTreeMap::new(),
            generated_failures: BTreeMap::new(),
        }
    }

    pub(crate) fn has_operation(&self, operation: &str, kind: LocusOperationKind) -> bool {
        self.operations.entries.iter().any(|item| {
            item.operation_id == operation
                && matches!(
                    (item.kind, kind),
                    (
                        ProjectedOperationFragmentKind::OwnerRequestInvocation,
                        LocusOperationKind::OwnerRequestStub
                    ) | (
                        ProjectedOperationFragmentKind::OwnerRmwExecution,
                        LocusOperationKind::OwnerRmwEvaluation
                    ) | (
                        ProjectedOperationFragmentKind::RelationPublication,
                        LocusOperationKind::RelationPublication
                    ) | (
                        ProjectedOperationFragmentKind::ConsumerLocalRelationProjection,
                        LocusOperationKind::ConsumerLocalProjection
                    ) | (
                        ProjectedOperationFragmentKind::DesignatedEvaluation,
                        LocusOperationKind::DesignatedEvaluationExpression
                    )
                )
        })
    }

    pub(crate) fn is_empty_artifact(&self) -> bool {
        self.operations.entries.is_empty() && self.checked_fragments.local_state_schemas.is_empty()
    }

    pub(crate) fn checked_fragments(&self) -> &ProjectedCheckedFragments {
        &self.checked_fragments
    }

    pub(crate) fn operations(&self) -> &ProjectedOperationFragments {
        &self.operations
    }

    pub(crate) fn locus_tag(&self) -> &LocusTag {
        &self.locus_tag
    }

    pub(crate) fn generated_failures(&self, operation: &str) -> Option<&FailureRow> {
        self.generated_failures.get(operation)
    }

    pub(crate) fn declared_failures(&self, operation: &str) -> Option<&FailureRow> {
        self.declared_failures.get(operation)
    }

    pub(super) fn add_fragment(&mut self, fragment: ProjectedOperationFragment) {
        if fragment.kind == ProjectedOperationFragmentKind::OwnerRmwExecution {
            self.checked_fragments
                .owner_operations
                .push(fragment.operation_id.clone());
            self.checked_fragments
                .local_state_schemas
                .extend(fragment.local_state_schemas().iter().cloned());
        }
        self.operations.entries.push(fragment);
    }

    pub(crate) fn add_failures(
        &mut self,
        operation: impl Into<String>,
        declared: &FailureRow,
        generated: &FailureRow,
    ) {
        let operation = operation.into();
        self.declared_failures
            .insert(operation.clone(), declared.clone());
        self.generated_failures.insert(operation, generated.clone());
    }

    pub(crate) fn sort(&mut self) {
        self.operations.entries.sort_by(|left, right| {
            (
                &left.operation_id,
                left.kind,
                &left.core_ref,
                left.checked_core_identity.dependency_ordinal(),
                &left.fragment_ref,
            )
                .cmp(&(
                    &right.operation_id,
                    right.kind,
                    &right.core_ref,
                    right.checked_core_identity.dependency_ordinal(),
                    &right.fragment_ref,
                ))
        });
        self.checked_fragments.owner_operations.sort();
        self.checked_fragments
            .local_state_schemas
            .sort_by(|left, right| left.name().cmp(right.name()));
    }
}

pub(crate) struct LocusPrograms<'a> {
    programs: &'a BTreeMap<String, LocusProgram>,
}

impl<'a> LocusPrograms<'a> {
    pub(crate) fn locus_tags(&self) -> Vec<LocusTag> {
        self.programs
            .values()
            .map(|program| program.locus_tag.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CommunicationEdgeKind {
    OwnerRequest,
    OwnerReplyReceipt,
    RelationProjectionPublication,
    DesignatedInputRequest,
    DesignatedInputReceipt,
    DesignatedResultDelivery,
    AbsoluteValueStream,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarrierLifecycleKind {
    OwnerRequest,
    OwnerReplyReceipt,
    DesignatedInputRequest,
    DesignatedInputReceipt,
    RelationProjectionPublication,
    DesignatedResultDelivery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CarrierOccurrenceSlotKind {
    Request,
    Serve,
    Reply,
    Receive,
    Publish,
    Observe,
    Consume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CarrierFrontierKind {
    Input,
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarrierProvenanceKind {
    RequiredFromSealedRuntimeSeam,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperationIdentityTemplate {
    operation_id: String,
}

impl OperationIdentityTemplate {
    pub(crate) fn operation_id(&self) -> &str {
        &self.operation_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RequestIdentityTemplate {
    operation_id: String,
    source_ref: SourceRef,
}

impl RequestIdentityTemplate {
    pub(crate) const fn has_slot(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReferenceOnlyRedactionPolicy;

impl ReferenceOnlyRedactionPolicy {
    pub(crate) const fn is_reference_only_redacted(&self) -> bool {
        true
    }

    pub(crate) const fn is_reference_only(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarrierContractProvenance {
    CheckedCoreBound,
}

impl CarrierContractProvenance {
    pub(crate) const fn is_checked_core_bound(&self) -> bool {
        matches!(self, Self::CheckedCoreBound)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DesignatedResultCarrierDetails {
    result_version: ResultVersion,
    input_frontier: InputFrontier,
    result_frontier: ResultFrontier,
    observation_policy: ObservationPolicy,
    policy_stamp: PolicyStamp,
    retry_contract: StaticRetryContractKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CarrierContract {
    edge_kind: CommunicationEdgeKind,
    lifecycle_kind: CarrierLifecycleKind,
    operation_identity_template: OperationIdentityTemplate,
    request_identity_template: RequestIdentityTemplate,
    source_ref: SourceRef,
    core_ref: Option<String>,
    origin_principal_template: Option<String>,
    origin_locus_template: Option<String>,
    target_owner_locus_template: Option<String>,
    declared_failure_row: FailureRow,
    effect_row: ProjectedEffectRow,
    authority_requirements: AuthorityRequirements,
    occurrence_slots: Vec<CarrierOccurrenceSlotKind>,
    frontiers: BTreeSet<CarrierFrontierKind>,
    linked_request_identity: bool,
    typed_outcome: bool,
    evaluator_receipt_consumption: bool,
    designated_dependency: Option<DesignatedRemoteInputDependency>,
    visibility_policy: ReferenceOnlyRedactionPolicy,
    provenance: CarrierContractProvenance,
    designated_result_details: Option<DesignatedResultCarrierDetails>,
}

impl CarrierContract {
    pub(crate) fn edge_kind(&self) -> CommunicationEdgeKind {
        self.edge_kind
    }

    pub(crate) fn lifecycle_kind(&self) -> CarrierLifecycleKind {
        self.lifecycle_kind
    }

    pub(crate) fn operation_identity_template(&self) -> &OperationIdentityTemplate {
        &self.operation_identity_template
    }

    pub(crate) fn request_identity_template(&self) -> &RequestIdentityTemplate {
        &self.request_identity_template
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }

    pub(crate) fn origin_principal_template(&self) -> Option<&str> {
        self.origin_principal_template.as_deref()
    }

    pub(crate) fn origin_locus_template(&self) -> Option<&str> {
        self.origin_locus_template.as_deref()
    }

    pub(crate) fn target_owner_locus_template(&self) -> Option<&str> {
        self.target_owner_locus_template.as_deref()
    }

    pub(crate) fn requires_occurrence_slot(&self, slot: CarrierOccurrenceSlotKind) -> bool {
        self.occurrence_slots.contains(&slot)
    }

    pub(crate) fn required_occurrence_slots(&self) -> &[CarrierOccurrenceSlotKind] {
        &self.occurrence_slots
    }

    pub(crate) fn declared_failure_row(&self) -> &FailureRow {
        &self.declared_failure_row
    }

    pub(crate) fn effect_row(&self) -> &ProjectedEffectRow {
        &self.effect_row
    }

    pub(crate) fn authority_requirements(&self) -> &AuthorityRequirements {
        &self.authority_requirements
    }

    pub(crate) fn runtime_seam_requirements(&self) -> &RuntimeSeamRequirements {
        self.authority_requirements.runtime_seam_requirements()
    }

    pub(crate) fn has_no_frontier_contract(&self) -> bool {
        self.frontiers.is_empty()
    }

    pub(crate) fn requires_any_frontier(&self) -> bool {
        !self.frontiers.is_empty()
    }

    pub(crate) fn requires_frontier(&self, frontier: CarrierFrontierKind) -> bool {
        self.frontiers.contains(&frontier)
    }

    pub(crate) const fn requires_linked_request_identity(&self) -> bool {
        self.linked_request_identity
    }

    pub(crate) const fn requires_typed_success_or_declared_failure_outcome(&self) -> bool {
        self.typed_outcome
    }

    pub(crate) const fn requires_evaluator_receipt_consumption_state(&self) -> bool {
        self.evaluator_receipt_consumption
    }

    pub(crate) const fn requires_receipt_consumption_state(&self) -> bool {
        self.evaluator_receipt_consumption
    }

    pub(crate) fn designated_remote_input_dependency(
        &self,
    ) -> Option<&DesignatedRemoteInputDependency> {
        self.designated_dependency.as_ref()
    }

    pub(crate) fn visibility_policy(&self) -> &ReferenceOnlyRedactionPolicy {
        &self.visibility_policy
    }

    pub(crate) fn provenance(&self) -> &CarrierContractProvenance {
        &self.provenance
    }

    pub(crate) fn result_version(&self) -> Option<ResultVersion> {
        self.designated_result_details
            .as_ref()
            .map(|details| details.result_version)
    }

    pub(crate) fn input_frontier(&self) -> Option<&InputFrontier> {
        self.designated_result_details
            .as_ref()
            .map(|details| &details.input_frontier)
    }

    pub(crate) fn result_frontier(&self) -> Option<&ResultFrontier> {
        self.designated_result_details
            .as_ref()
            .map(|details| &details.result_frontier)
    }

    pub(crate) fn observation_policy(&self) -> Option<&ObservationPolicy> {
        self.designated_result_details
            .as_ref()
            .map(|details| &details.observation_policy)
    }

    pub(crate) fn policy_stamp(&self) -> Option<&PolicyStamp> {
        self.designated_result_details
            .as_ref()
            .map(|details| &details.policy_stamp)
    }

    pub(crate) fn static_retry_contract(&self) -> Option<StaticRetryContractKind> {
        self.designated_result_details
            .as_ref()
            .map(|details| details.retry_contract)
    }

    pub(crate) const fn transfers_authority(&self) -> bool {
        false
    }

    pub(crate) const fn mints_authority_without_source(&self) -> bool {
        false
    }

    pub(super) fn owner_request(evaluation: &CheckedEvaluation) -> Self {
        let core = evaluation
            .owner_rmw_core()
            .expect("owner request carrier comes from owner checked Core");
        Self::new(
            CommunicationEdgeKind::OwnerRequest,
            CarrierLifecycleKind::OwnerRequest,
            evaluation.name(),
            evaluation.source_ref().clone(),
            format!("owner-rmw:{}", evaluation.name()),
            Some(evaluation.actor_authority_origin().to_string()),
            Some(core.authority_origin_locus().to_string()),
            Some(core.owner_locus().to_string()),
            evaluation.declared_failure_row().clone(),
            effect_row_for(evaluation),
            AuthorityRequirements::standard(evaluation.name(), evaluation.source_ref()),
            [CarrierOccurrenceSlotKind::Request],
            [],
            false,
            false,
            false,
            None,
        )
    }

    pub(super) fn owner_reply(evaluation: &CheckedEvaluation) -> Self {
        Self::new(
            CommunicationEdgeKind::OwnerReplyReceipt,
            CarrierLifecycleKind::OwnerReplyReceipt,
            evaluation.name(),
            evaluation.source_ref().clone(),
            format!("owner-rmw:{}", evaluation.name()),
            Some(evaluation.actor_authority_origin().to_string()),
            Some(evaluation.authority_origin_locus().to_string()),
            Some(evaluation.owner_evaluation_locus().to_string()),
            evaluation.declared_failure_row().clone(),
            effect_row_for(evaluation),
            AuthorityRequirements::standard(evaluation.name(), evaluation.source_ref()),
            all_occurrences(),
            [],
            true,
            true,
            false,
            None,
        )
    }

    pub(super) fn designated_request(
        operation: &str,
        evaluation: &CheckedEvaluation,
        dependency: &DesignatedRemoteInputDependency,
    ) -> Self {
        Self::new(
            CommunicationEdgeKind::DesignatedInputRequest,
            CarrierLifecycleKind::DesignatedInputRequest,
            operation,
            dependency.typed_state_read().source_ref(),
            format!(
                "designated-input:{operation}:{}",
                dependency.typed_state_read().namespace()
            ),
            None,
            Some(dependency.designated_evaluator().to_string()),
            Some(dependency.source_owner_locus().to_string()),
            evaluation.declared_failure_row().clone(),
            effect_row_for(evaluation),
            AuthorityRequirements::designated(
                operation,
                &dependency.typed_state_read().source_ref(),
            ),
            [CarrierOccurrenceSlotKind::Request],
            [CarrierFrontierKind::Input],
            false,
            false,
            false,
            Some(dependency.clone()),
        )
    }

    pub(super) fn designated_receipt(
        operation: &str,
        evaluation: &CheckedEvaluation,
        dependency: &DesignatedRemoteInputDependency,
    ) -> Self {
        Self::new(
            CommunicationEdgeKind::DesignatedInputReceipt,
            CarrierLifecycleKind::DesignatedInputReceipt,
            operation,
            dependency.typed_state_read().source_ref(),
            format!(
                "designated-input:{operation}:{}",
                dependency.typed_state_read().namespace()
            ),
            None,
            Some(dependency.designated_evaluator().to_string()),
            Some(dependency.source_owner_locus().to_string()),
            evaluation.declared_failure_row().clone(),
            effect_row_for(evaluation),
            AuthorityRequirements::designated(
                operation,
                &dependency.typed_state_read().source_ref(),
            ),
            all_occurrences(),
            [CarrierFrontierKind::Result],
            true,
            true,
            true,
            Some(dependency.clone()),
        )
    }

    pub(super) fn relation_publication(evaluation: &CheckedEvaluation) -> Self {
        Self::new(
            CommunicationEdgeKind::RelationProjectionPublication,
            CarrierLifecycleKind::RelationProjectionPublication,
            evaluation.name(),
            evaluation.source_ref().clone(),
            format!("relation:{}", evaluation.name()),
            None,
            Some(evaluation.owner_evaluation_locus().to_string()),
            evaluation.consumer_projection_locus().map(str::to_string),
            evaluation.declared_failure_row().clone(),
            effect_row_for(evaluation),
            AuthorityRequirements::empty(),
            [
                CarrierOccurrenceSlotKind::Publish,
                CarrierOccurrenceSlotKind::Observe,
            ],
            [],
            false,
            false,
            false,
            None,
        )
    }

    pub(super) fn designated_result_delivery(evaluation: &CheckedEvaluation) -> Self {
        let core = evaluation
            .designated_result_consumer_core()
            .expect("delivery carrier comes from checked designated result consumer Core");
        let mut contract = Self::new(
            CommunicationEdgeKind::DesignatedResultDelivery,
            CarrierLifecycleKind::DesignatedResultDelivery,
            format!("{}.{}", core.evaluator(), core.result()),
            core.source_ref().clone(),
            format!(
                "designated-consume:{}.{}:{}",
                core.evaluator(),
                core.result(),
                core.consumer_locus()
            ),
            None,
            Some(core.evaluator().to_string()),
            Some(core.consumer_locus().to_string()),
            evaluation.declared_failure_row().clone(),
            effect_row_for(evaluation),
            AuthorityRequirements::designated_result_consumer(
                &format!("{}.{}", core.evaluator(), core.result()),
                core.source_ref(),
            ),
            [
                CarrierOccurrenceSlotKind::Publish,
                CarrierOccurrenceSlotKind::Receive,
                CarrierOccurrenceSlotKind::Consume,
            ],
            [CarrierFrontierKind::Input, CarrierFrontierKind::Result],
            true,
            true,
            true,
            None,
        );
        contract.designated_result_details = Some(DesignatedResultCarrierDetails {
            result_version: core.result_version(),
            input_frontier: core.input_frontier().clone(),
            result_frontier: core.result_frontier().clone(),
            observation_policy: core.observation_policy().clone(),
            policy_stamp: core.policy_stamp().clone(),
            retry_contract: core.retry_contract(),
        });
        contract
    }

    #[allow(clippy::too_many_arguments)]
    fn new<I, F>(
        edge_kind: CommunicationEdgeKind,
        lifecycle_kind: CarrierLifecycleKind,
        operation: impl Into<String>,
        source_ref: SourceRef,
        core_ref: impl Into<String>,
        origin_principal_template: Option<String>,
        origin_locus_template: Option<String>,
        target_owner_locus_template: Option<String>,
        declared_failure_row: FailureRow,
        effect_row: ProjectedEffectRow,
        authority_requirements: AuthorityRequirements,
        occurrence_slots: I,
        frontiers: F,
        linked_request_identity: bool,
        typed_outcome: bool,
        evaluator_receipt_consumption: bool,
        designated_dependency: Option<DesignatedRemoteInputDependency>,
    ) -> Self
    where
        I: IntoIterator<Item = CarrierOccurrenceSlotKind>,
        F: IntoIterator<Item = CarrierFrontierKind>,
    {
        let operation = operation.into();
        Self {
            edge_kind,
            lifecycle_kind,
            operation_identity_template: OperationIdentityTemplate {
                operation_id: operation.clone(),
            },
            request_identity_template: RequestIdentityTemplate {
                operation_id: operation,
                source_ref: source_ref.clone(),
            },
            source_ref,
            core_ref: Some(core_ref.into()),
            origin_principal_template,
            origin_locus_template,
            target_owner_locus_template,
            declared_failure_row,
            effect_row,
            authority_requirements,
            occurrence_slots: occurrence_slots.into_iter().collect(),
            frontiers: frontiers.into_iter().collect(),
            linked_request_identity,
            typed_outcome,
            evaluator_receipt_consumption,
            designated_dependency,
            visibility_policy: ReferenceOnlyRedactionPolicy,
            provenance: CarrierContractProvenance::CheckedCoreBound,
            designated_result_details: None,
        }
    }
}

fn effect_row_for(evaluation: &CheckedEvaluation) -> ProjectedEffectRow {
    ProjectedEffectRow {
        kinds: evaluation
            .effect_row()
            .entries()
            .iter()
            .map(|entry| entry.kind())
            .collect(),
    }
}

fn all_occurrences() -> [CarrierOccurrenceSlotKind; 4] {
    [
        CarrierOccurrenceSlotKind::Request,
        CarrierOccurrenceSlotKind::Serve,
        CarrierOccurrenceSlotKind::Reply,
        CarrierOccurrenceSlotKind::Receive,
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommunicationEdge {
    operation: String,
    kind: CommunicationEdgeKind,
    source_locus: String,
    target_locus: String,
    core_ref: Option<String>,
    source_ref: SourceRef,
    derived_from_checked_core: bool,
    transfers_authority: bool,
    edge_ref: String,
    source_fragment_ref: String,
    target_fragment_ref: String,
    checked_core_identity: CheckedCoreIdentity,
    carrier_contract: CarrierContract,
}

impl CommunicationEdge {
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }

    pub(crate) fn source_ref(&self) -> SourceRefView {
        SourceRefView::new(&self.source_ref)
    }

    pub(crate) const fn is_derived_from_checked_core(&self) -> bool {
        self.derived_from_checked_core
    }

    pub(crate) const fn transfers_authority(&self) -> bool {
        self.transfers_authority
    }

    pub(crate) fn edge_ref(&self) -> &str {
        &self.edge_ref
    }

    pub(crate) fn source_fragment_ref(&self) -> &String {
        &self.source_fragment_ref
    }

    pub(crate) fn target_fragment_ref(&self) -> &String {
        &self.target_fragment_ref
    }

    pub(crate) fn carrier_contract(&self) -> &CarrierContract {
        &self.carrier_contract
    }

    pub(crate) fn checked_core_identity(&self) -> &CheckedCoreIdentity {
        &self.checked_core_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct CommunicationPlan {
    edges: Vec<CommunicationEdge>,
}

pub(super) struct CommunicationEdgeInput {
    pub(super) operation: String,
    pub(super) kind: CommunicationEdgeKind,
    pub(super) source_locus: String,
    pub(super) target_locus: String,
    pub(super) core_ref: String,
    pub(super) source_ref: SourceRef,
    pub(super) carrier_contract: CarrierContract,
    pub(super) checked_core_identity: CheckedCoreIdentity,
    pub(super) source_fragment_ref: String,
    pub(super) target_fragment_ref: String,
}

pub(crate) struct CommunicationEdges<'a> {
    entries: Vec<&'a CommunicationEdge>,
}

pub(crate) struct SelectedCommunicationEdge<'a> {
    edge: &'a CommunicationEdge,
}

impl<'a> SelectedCommunicationEdge<'a> {
    pub(crate) fn carrier_contract(&self) -> &CarrierContract {
        self.edge.carrier_contract()
    }
    pub(crate) fn source_ref(&self) -> SourceRefView {
        self.edge.source_ref()
    }
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.edge.core_ref()
    }
    pub(crate) fn checked_core_identity(&self) -> &CheckedCoreIdentity {
        self.edge.checked_core_identity()
    }
    pub(crate) fn edge_ref(&self) -> &str {
        self.edge.edge_ref()
    }
}

impl<'a> CommunicationEdges<'a> {
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn single_for_designated_dependency(
        &self,
        dependency: &DesignatedRemoteInputDependency,
    ) -> Option<SelectedCommunicationEdge<'a>> {
        self.entries
            .iter()
            .copied()
            .find(|edge| {
                edge.carrier_contract.designated_remote_input_dependency() == Some(dependency)
            })
            .map(|edge| SelectedCommunicationEdge { edge })
    }

    pub(crate) fn edge_refs_union_with(&self, other: &Self) -> EdgeRefs {
        EdgeRefs(
            self.entries
                .iter()
                .chain(other.entries.iter())
                .map(|edge| edge.edge_ref.clone())
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EdgeRefs(BTreeSet<String>);

impl EdgeRefs {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn contains_all(&self, other: &Self) -> bool {
        other.0.iter().all(|entry| self.0.contains(entry))
    }
}

impl CommunicationPlan {
    pub(crate) fn has_edge(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
    ) -> bool {
        self.single_edge(operation, kind, source, target).is_some()
    }

    pub(crate) fn single_edge(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
    ) -> Option<&CommunicationEdge> {
        self.edges.iter().find(|edge| {
            edge.operation == operation
                && edge.kind == kind
                && edge.source_locus == source
                && edge.target_locus == target
        })
    }

    pub(crate) fn count_edges(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
    ) -> usize {
        self.edges
            .iter()
            .filter(|edge| {
                edge.operation == operation
                    && edge.kind == kind
                    && edge.source_locus == source
                    && edge.target_locus == target
            })
            .count()
    }

    pub(crate) fn edges(&self) -> &[CommunicationEdge] {
        &self.edges
    }

    pub(crate) fn edges_for(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
    ) -> CommunicationEdges<'_> {
        CommunicationEdges {
            entries: self
                .edges
                .iter()
                .filter(|edge| {
                    edge.operation == operation
                        && edge.kind == kind
                        && edge.source_locus == source
                        && edge.target_locus == target
                })
                .collect(),
        }
    }

    pub(super) fn add_derived(&mut self, input: CommunicationEdgeInput) {
        let operation = input.operation;
        let source_locus = input.source_locus;
        let target_locus = input.target_locus;
        let kind = input.kind;
        let checked_core_identity = input.checked_core_identity;
        let source_fragment_ref = input.source_fragment_ref;
        let target_fragment_ref = input.target_fragment_ref;
        let ordinal = checked_core_identity.dependency_ordinal();
        self.edges.push(CommunicationEdge {
            edge_ref: format!(
                "edge:{operation}:{kind:?}:{source_locus}:{target_locus}:dependency-{ordinal:?}"
            ),
            operation,
            kind,
            source_locus,
            target_locus,
            core_ref: Some(input.core_ref),
            source_ref: input.source_ref,
            derived_from_checked_core: true,
            transfers_authority: false,
            carrier_contract: input.carrier_contract,
            checked_core_identity,
            source_fragment_ref,
            target_fragment_ref,
        });
    }

    pub(crate) fn sort(&mut self) {
        self.edges.sort_by(|left, right| {
            (
                &left.operation,
                left.kind,
                &left.source_locus,
                &left.target_locus,
                &left.core_ref,
                &left.edge_ref,
            )
                .cmp(&(
                    &right.operation,
                    right.kind,
                    &right.source_locus,
                    &right.target_locus,
                    &right.core_ref,
                    &right.edge_ref,
                ))
        });
    }

    #[cfg(test)]
    pub(crate) fn for_test_remove(&mut self, operation: &str, kind: CommunicationEdgeKind) {
        if let Some(index) = self
            .edges
            .iter()
            .position(|edge| edge.operation == operation && edge.kind == kind)
        {
            self.edges.remove(index);
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_insert_non_derived(
        &mut self,
        id: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
        operation: &str,
    ) {
        let seed = self
            .edges
            .first()
            .expect("canonical projection has a checked carrier before mutation");
        let carrier_contract = seed.carrier_contract.clone();
        let checked_core_identity = seed.checked_core_identity.clone();
        let source_fragment_ref = seed.source_fragment_ref.clone();
        let target_fragment_ref = seed.target_fragment_ref.clone();
        self.edges.push(CommunicationEdge {
            operation: operation.to_string(),
            kind,
            source_locus: source.to_string(),
            target_locus: target.to_string(),
            core_ref: Some(id.to_string()),
            source_ref: SourceRef::new("projection-test", 1, 1, 1, 1),
            derived_from_checked_core: false,
            transfers_authority: false,
            edge_ref: format!("edge:test:{id}"),
            carrier_contract,
            checked_core_identity,
            source_fragment_ref,
            target_fragment_ref,
        });
        self.sort();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EffectHandlerKind {
    OwnerService,
    DesignatedSourceService,
    DesignatedEvaluator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectedEffectRow {
    kinds: Vec<EffectKind>,
}

impl ProjectedEffectRow {
    pub(crate) fn kinds(&self) -> Vec<EffectKind> {
        self.kinds.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectHandlerPlanEntry {
    operation: String,
    kind: EffectHandlerKind,
    locus: String,
    source_ref: SourceRef,
    core_ref: Option<String>,
    effect_row: ProjectedEffectRow,
    declared_failure_row: FailureRow,
    generated_failure_row: FailureRow,
    source_bound: bool,
    handler_ref: String,
    checked_core_identity: CheckedCoreIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EffectHandlerInput {
    pub(super) operation: String,
    pub(super) kind: EffectHandlerKind,
    pub(super) locus: String,
    pub(super) source_ref: SourceRef,
    pub(super) core_ref: String,
    pub(super) effect_kinds: Vec<EffectKind>,
    pub(super) declared_failure_row: FailureRow,
    pub(super) generated_failure_row: FailureRow,
    pub(super) checked_core_identity: CheckedCoreIdentity,
}

impl EffectHandlerPlanEntry {
    pub(crate) const fn is_source_bound(&self) -> bool {
        self.source_bound
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }

    pub(crate) fn effect_row(&self) -> &ProjectedEffectRow {
        &self.effect_row
    }

    pub(crate) fn declared_failure_row(&self) -> &FailureRow {
        &self.declared_failure_row
    }

    pub(crate) fn generated_failure_row(&self) -> &FailureRow {
        &self.generated_failure_row
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct EffectHandlerPlan {
    handlers: Vec<EffectHandlerPlanEntry>,
}

impl EffectHandlerPlan {
    pub(crate) const fn has_generic_provider_registry(&self) -> bool {
        false
    }

    pub(crate) fn single_handler(
        &self,
        operation: &str,
        kind: EffectHandlerKind,
        locus: &str,
    ) -> Option<&EffectHandlerPlanEntry> {
        self.handlers.iter().find(|handler| {
            handler.operation == operation && handler.kind == kind && handler.locus == locus
        })
    }

    pub(crate) fn entries(&self) -> &[EffectHandlerPlanEntry] {
        &self.handlers
    }

    pub(crate) fn all_handlers_for_operation_with_kind_are_at_locus(
        &self,
        operation: &str,
        kind: EffectHandlerKind,
        locus: &str,
    ) -> bool {
        let handlers = self
            .handlers
            .iter()
            .filter(|handler| handler.operation == operation && handler.kind == kind)
            .collect::<Vec<_>>();
        !handlers.is_empty() && handlers.iter().all(|handler| handler.locus == locus)
    }

    pub(super) fn add(&mut self, input: EffectHandlerInput) {
        let handler_ref = format!(
            "handler:{}:{:?}:{}:dependency-{:?}",
            input.operation,
            input.kind,
            input.locus,
            input.checked_core_identity.dependency_ordinal(),
        );
        self.handlers.push(EffectHandlerPlanEntry {
            operation: input.operation,
            kind: input.kind,
            locus: input.locus,
            source_ref: input.source_ref,
            core_ref: Some(input.core_ref),
            effect_row: ProjectedEffectRow {
                kinds: input.effect_kinds,
            },
            declared_failure_row: input.declared_failure_row,
            generated_failure_row: input.generated_failure_row,
            source_bound: true,
            handler_ref,
            checked_core_identity: input.checked_core_identity,
        });
    }

    pub(crate) fn sort(&mut self) {
        self.handlers.sort_by(|left, right| {
            (&left.operation, left.kind, &left.locus, &left.handler_ref).cmp(&(
                &right.operation,
                right.kind,
                &right.locus,
                &right.handler_ref,
            ))
        });
    }

    #[cfg(test)]
    pub(crate) fn for_test_clear_provenance(
        &mut self,
        operation: &str,
        kind: EffectHandlerKind,
        locus: &str,
    ) {
        if let Some(handler) = self.handlers.iter_mut().find(|handler| {
            handler.operation == operation && handler.kind == kind && handler.locus == locus
        }) {
            handler.source_bound = false;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectedRelationAnchor {
    pub(super) anchor: String,
    pub(super) epoch: String,
    pub(super) transform: RelationTransformCore,
    pub(super) source_ref: SourceRef,
}

impl ProjectedRelationAnchor {
    pub(crate) fn anchor(&self) -> &str {
        &self.anchor
    }

    pub(crate) fn epoch(&self) -> &str {
        &self.epoch
    }

    pub(crate) fn transform(&self) -> &RelationTransformCore {
        &self.transform
    }

    pub(crate) fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RelationAnchorRole {
    Primary,
    Fallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectedRelation {
    pub(super) name: String,
    pub(super) owner_locus: String,
    pub(super) subject: String,
    pub(super) subject_type: String,
    pub(super) primary_anchor: ProjectedRelationAnchor,
    pub(super) fallback_anchor: ProjectedRelationAnchor,
    pub(super) binding_frontier: BindingActivationFrontier,
    pub(super) consumer_locus: Option<String>,
    pub(super) residual_source_refs: Vec<(ResidualObligationKind, SourceRef)>,
}

impl ProjectedRelation {
    pub(crate) fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }

    pub(crate) fn subject_type(&self) -> &str {
        &self.subject_type
    }

    pub(crate) fn primary_anchor(&self) -> &ProjectedRelationAnchor {
        &self.primary_anchor
    }

    pub(crate) fn fallback_anchor(&self) -> &ProjectedRelationAnchor {
        &self.fallback_anchor
    }

    pub(crate) fn binding_frontier(&self) -> &BindingActivationFrontier {
        &self.binding_frontier
    }

    pub(crate) fn consumer_locus(&self) -> Option<&str> {
        self.consumer_locus.as_deref()
    }

    pub(crate) fn residual_source_ref(&self, kind: ResidualObligationKind) -> Option<&SourceRef> {
        self.residual_source_refs
            .iter()
            .find(|(candidate, _)| *candidate == kind)
            .map(|(_, source_ref)| source_ref)
    }

    fn anchor(&self, role: RelationAnchorRole) -> &ProjectedRelationAnchor {
        match role {
            RelationAnchorRole::Primary => self.primary_anchor(),
            RelationAnchorRole::Fallback => self.fallback_anchor(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RelationGraphClaim {
    FiniteTypedExtensionBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RelationGraphEdgeTag {
    CheckedTwoAnchorFallback,
    TestOnlyTypedExtensionPressure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelationGraphEdgeProvenance {
    tag: RelationGraphEdgeTag,
}

impl RelationGraphEdgeProvenance {
    pub(crate) fn is_checked_two_anchor_fallback(&self) -> bool {
        self.tag == RelationGraphEdgeTag::CheckedTwoAnchorFallback
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RelationGraphNode {
    relation: String,
    role: RelationAnchorRole,
    source_ref: SourceRef,
}

impl RelationGraphNode {
    fn id(&self) -> String {
        format!(
            "{}:{:?}:{}:{}:{}:{}:{}",
            self.relation,
            self.role,
            self.source_ref.path,
            self.source_ref.start_line,
            self.source_ref.start_column,
            self.source_ref.end_line,
            self.source_ref.end_column
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelationGraphEdgeSeed {
    from: RelationGraphNode,
    to: RelationGraphNode,
    tag: RelationGraphEdgeTag,
}

impl RelationGraphEdgeSeed {
    #[cfg(test)]
    pub(crate) fn typed_extension_dependency_between_checked_anchors(
        from: (&CheckedEvaluation, RelationAnchorRole),
        to: (&CheckedEvaluation, RelationAnchorRole),
    ) -> Self {
        Self {
            from: relation_anchor_node(from.0, from.1),
            to: relation_anchor_node(to.0, to.1),
            tag: RelationGraphEdgeTag::TestOnlyTypedExtensionPressure,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only_typed_checked_anchor_dependency(
        from: (&CheckedEvaluation, RelationAnchorRole),
        to: (&CheckedEvaluation, RelationAnchorRole),
    ) -> Self {
        Self::typed_extension_dependency_between_checked_anchors(from, to)
    }

    pub(crate) fn tag(&self) -> RelationGraphEdgeTag {
        self.tag
    }
    pub(crate) fn provenance(&self) -> RelationGraphEdgeProvenance {
        RelationGraphEdgeProvenance { tag: self.tag }
    }
}

fn relation_anchor_node(
    evaluation: &CheckedEvaluation,
    role: RelationAnchorRole,
) -> RelationGraphNode {
    let _ = evaluation
        .relation_core()
        .expect("typed relation graph endpoints are checked relation Core");
    RelationGraphNode {
        relation: evaluation.name().to_string(),
        role,
        source_ref: SourceRef::new(
            evaluation.source_ref().path.clone(),
            evaluation.source_ref().start_line,
            evaluation.source_ref().start_column,
            evaluation.source_ref().end_line,
            evaluation.source_ref().end_column,
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectionRelationGraph {
    claim: RelationGraphClaim,
    relations: BTreeMap<String, ProjectedRelation>,
    typed_dependency_edges: Vec<RelationGraphEdgeSeed>,
    test_only_extension_boundary: bool,
}

impl Default for ProjectionRelationGraph {
    fn default() -> Self {
        Self {
            claim: RelationGraphClaim::FiniteTypedExtensionBoundary,
            relations: BTreeMap::new(),
            typed_dependency_edges: Vec::new(),
            test_only_extension_boundary: false,
        }
    }
}

impl ProjectionRelationGraph {
    #[cfg(test)]
    pub(crate) fn try_new_for_test<I>(
        claim: RelationGraphClaim,
        edges: I,
    ) -> Result<Self, ProjectionDiagnostics>
    where
        I: IntoIterator<Item = RelationGraphEdgeSeed>,
    {
        let typed_dependency_edges = edges.into_iter().collect::<Vec<_>>();
        if relation_edges_have_cycle(&typed_dependency_edges) {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::RelationGraphCycle,
                "finite typed relation-extension boundary rejects a cycle",
            ));
        }
        Ok(Self {
            claim,
            relations: BTreeMap::new(),
            typed_dependency_edges,
            test_only_extension_boundary: true,
        })
    }

    pub(crate) const fn claim(&self) -> RelationGraphClaim {
        self.claim
    }

    pub(crate) const fn claims_arbitrary_dag_theorem(&self) -> bool {
        false
    }

    pub(crate) const fn claims_ordinary_source_nested_relation_semantics(&self) -> bool {
        false
    }

    pub(crate) const fn can_mint_future_semantic_dependencies(&self) -> bool {
        false
    }

    #[cfg(test)]
    pub(crate) fn try_test_only_extension_boundary<I>(
        expected_identity: CheckedProgramIdentity,
        edges: I,
    ) -> Result<Self, ProjectionDiagnostics>
    where
        I: IntoIterator<Item = RelationGraphEdgeSeed>,
    {
        let typed_dependency_edges = edges.into_iter().collect::<Vec<_>>();
        if typed_dependency_edges.iter().any(|edge| {
            edge.from.source_ref.path != expected_identity.source_file()
                || edge.to.source_ref.path != expected_identity.source_file()
        }) {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::ForeignCheckedProgramRelationDependency,
                "test-only relation-extension pressure mixes checked program identities",
            ));
        }
        if relation_edges_have_cycle(&typed_dependency_edges) {
            return Err(ProjectionDiagnostics::one(
                ProjectionDiagnosticKind::RelationGraphCycle,
                "finite typed relation-extension boundary rejects a cycle",
            ));
        }
        Ok(Self {
            claim: RelationGraphClaim::FiniteTypedExtensionBoundary,
            relations: BTreeMap::new(),
            typed_dependency_edges,
            test_only_extension_boundary: true,
        })
    }

    #[cfg(test)]
    pub(crate) const fn is_test_only_extension_boundary(&self) -> bool {
        self.test_only_extension_boundary
    }

    #[cfg(test)]
    pub(crate) const fn claims_test_only_non_source_semantic_pressure(&self) -> bool {
        self.test_only_extension_boundary
    }

    pub(crate) fn relation(&self, name: &str) -> Option<&ProjectedRelation> {
        self.relations.get(name)
    }

    pub(crate) fn is_acyclic(&self) -> bool {
        !relation_edges_have_cycle(&self.typed_dependency_edges)
    }

    pub(crate) fn typed_dependency_edge_count(&self) -> usize {
        self.typed_dependency_edges.len()
    }

    pub(crate) fn has_typed_dependency_edge(
        &self,
        from: (&str, RelationAnchorRole),
        to: (&str, RelationAnchorRole),
    ) -> bool {
        self.typed_dependency_edges.iter().any(|edge| {
            edge.from.relation == from.0
                && edge.from.role == from.1
                && edge.to.relation == to.0
                && edge.to.role == to.1
        })
    }

    pub(crate) fn single_typed_dependency_edge(
        &self,
        from: (&str, RelationAnchorRole),
        to: (&str, RelationAnchorRole),
    ) -> Option<&RelationGraphEdgeSeed> {
        self.typed_dependency_edges.iter().find(|edge| {
            edge.from.relation == from.0
                && edge.from.role == from.1
                && edge.to.relation == to.0
                && edge.to.role == to.1
        })
    }

    pub(crate) fn node_id_for_relation_anchor(
        &self,
        relation: &str,
        role: RelationAnchorRole,
    ) -> String {
        if let Some(entry) = self.relations.get(relation) {
            let anchor = entry.anchor(role);
            return format!(
                "{}:{:?}:{}:{}:{}:{}:{}",
                relation,
                role,
                anchor.source_ref.path,
                anchor.source_ref.start_line,
                anchor.source_ref.start_column,
                anchor.source_ref.end_line,
                anchor.source_ref.end_column
            );
        }
        self.typed_dependency_edges
            .iter()
            .flat_map(|edge| [&edge.from, &edge.to])
            .find(|node| node.relation == relation && node.role == role)
            .map(RelationGraphNode::id)
            .unwrap_or_else(|| format!("{relation}:{role:?}:absent"))
    }

    pub(crate) fn all_typed_dependency_endpoints_are_checked_core_source_bound_to(
        &self,
        fixture: &str,
    ) -> bool {
        self.typed_dependency_edges.iter().all(|edge| {
            edge.from.source_ref.path.ends_with(fixture)
                && edge.to.source_ref.path.ends_with(fixture)
        })
    }

    pub(crate) fn max_dependency_depth(&self) -> usize {
        let mut adjacency: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for edge in &self.typed_dependency_edges {
            adjacency
                .entry(edge.from.id())
                .or_default()
                .push(edge.to.id());
            adjacency.entry(edge.to.id()).or_default();
        }
        let mut memo = BTreeMap::new();
        adjacency
            .keys()
            .map(|node| relation_depth(node, &adjacency, &mut memo))
            .max()
            .unwrap_or(0)
    }

    pub(crate) fn add_relation(&mut self, relation: ProjectedRelation) {
        let source_ref = relation.primary_anchor.source_ref.clone();
        self.typed_dependency_edges.push(RelationGraphEdgeSeed {
            from: RelationGraphNode {
                relation: relation.name.clone(),
                role: RelationAnchorRole::Primary,
                source_ref: source_ref.clone(),
            },
            to: RelationGraphNode {
                relation: relation.name.clone(),
                role: RelationAnchorRole::Fallback,
                source_ref,
            },
            tag: RelationGraphEdgeTag::CheckedTwoAnchorFallback,
        });
        self.relations.insert(relation.name.clone(), relation);
    }
}

fn relation_edges_have_cycle(edges: &[RelationGraphEdgeSeed]) -> bool {
    let mut adjacency: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for edge in edges {
        adjacency
            .entry(edge.from.id())
            .or_default()
            .push(edge.to.id());
        adjacency.entry(edge.to.id()).or_default();
    }
    fn visit(
        node: &str,
        adjacency: &BTreeMap<String, Vec<String>>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
    ) -> bool {
        if visiting.contains(node) {
            return true;
        }
        if !visited.insert(node.to_string()) {
            return false;
        }
        visiting.insert(node.to_string());
        let has_cycle = adjacency
            .get(node)
            .into_iter()
            .flatten()
            .any(|child| visit(child, adjacency, visiting, visited));
        visiting.remove(node);
        has_cycle
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    adjacency
        .keys()
        .any(|node| visit(node, &adjacency, &mut visiting, &mut visited))
}

fn relation_depth(
    node: &str,
    adjacency: &BTreeMap<String, Vec<String>>,
    memo: &mut BTreeMap<String, usize>,
) -> usize {
    if let Some(depth) = memo.get(node) {
        return *depth;
    }
    let depth = adjacency
        .get(node)
        .map(|children| {
            children
                .iter()
                .map(|child| 1 + relation_depth(child, adjacency, memo))
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0);
    memo.insert(node.to_string(), depth);
    depth
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PersistenceResponsibilityKind {
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PersistencePlan {
    by_locus: BTreeMap<String, Vec<PersistenceResponsibilityKind>>,
    by_relation: BTreeMap<String, Vec<PersistenceResponsibilityKind>>,
    by_designated: BTreeMap<String, Vec<PersistenceResponsibilityKind>>,
    global: Vec<PersistenceResponsibilityKind>,
}

impl PersistencePlan {
    pub(crate) fn responsibilities_for_locus(
        &self,
        locus: &str,
    ) -> Option<&[PersistenceResponsibilityKind]> {
        self.by_locus.get(locus).map(Vec::as_slice)
    }
    pub(crate) fn responsibilities_for_relation(
        &self,
        relation: &str,
    ) -> Option<&[PersistenceResponsibilityKind]> {
        self.by_relation.get(relation).map(Vec::as_slice)
    }

    pub(crate) fn responsibilities_for_designated_result(
        &self,
        result: &str,
    ) -> Option<&[PersistenceResponsibilityKind]> {
        self.by_designated.get(result).map(Vec::as_slice)
    }

    pub(crate) fn responsibilities_for_designated_result_consumer(
        &self,
        _operation: &str,
        locus: &str,
    ) -> Option<&[PersistenceResponsibilityKind]> {
        self.by_locus.get(locus).map(Vec::as_slice)
    }

    pub(crate) fn global_obligations(&self) -> &[PersistenceResponsibilityKind] {
        &self.global
    }

    pub(crate) fn add_relation(&mut self, relation: impl Into<String>) {
        self.by_relation.insert(
            relation.into(),
            vec![
                PersistenceResponsibilityKind::RelationBindingFrontier,
                PersistenceResponsibilityKind::RelationSelectedFallback,
                PersistenceResponsibilityKind::RelationResidualEvidenceRefs,
            ],
        );
    }

    pub(crate) fn add_designated(&mut self, result: impl Into<String>) {
        self.by_designated.insert(
            result.into(),
            vec![
                PersistenceResponsibilityKind::DesignatedResultVersion,
                PersistenceResponsibilityKind::DesignatedReceiptConsumption,
                PersistenceResponsibilityKind::DesignatedInputFrontier,
            ],
        );
    }

    pub(crate) fn finalize(&mut self) {
        self.global = vec![
            PersistenceResponsibilityKind::LocalStore,
            PersistenceResponsibilityKind::IncomingCarrierState,
            PersistenceResponsibilityKind::OutgoingCarrierState,
            PersistenceResponsibilityKind::MembershipCapabilityWitnessRefs,
            PersistenceResponsibilityKind::ResidualObligationState,
            PersistenceResponsibilityKind::LocalCut,
            PersistenceResponsibilityKind::PatchBoundary,
            PersistenceResponsibilityKind::PatchFrontier,
        ];
    }

    pub(super) fn assign_loci(&mut self, loci: &BTreeMap<String, LocusProgram>) {
        for (locus, program) in loci {
            let entry = self
                .by_locus
                .entry(locus.clone())
                .or_insert_with(|| vec![PersistenceResponsibilityKind::DeclaredLocusBoundary]);
            for fragment in &program.operations.entries {
                let responsibilities = match fragment.kind {
                    ProjectedOperationFragmentKind::OwnerRequestInvocation => vec![
                        PersistenceResponsibilityKind::OutgoingCarrierState,
                        PersistenceResponsibilityKind::IncomingCarrierState,
                        PersistenceResponsibilityKind::MembershipCapabilityWitnessRefs,
                        PersistenceResponsibilityKind::ReceiptConsumption,
                    ],
                    ProjectedOperationFragmentKind::OwnerRmwExecution => vec![
                        PersistenceResponsibilityKind::LocalStore,
                        PersistenceResponsibilityKind::OwnerQueue,
                        PersistenceResponsibilityKind::IncomingCarrierState,
                        PersistenceResponsibilityKind::OutgoingCarrierState,
                        PersistenceResponsibilityKind::MembershipCapabilityWitnessRefs,
                    ],
                    ProjectedOperationFragmentKind::RelationPublication => vec![
                        PersistenceResponsibilityKind::LocalStore,
                        PersistenceResponsibilityKind::RelationBindingFrontier,
                        PersistenceResponsibilityKind::IncomingCarrierState,
                        PersistenceResponsibilityKind::OutgoingCarrierState,
                    ],
                    ProjectedOperationFragmentKind::ConsumerLocalRelationProjection => vec![
                        PersistenceResponsibilityKind::RelationBindingFrontier,
                        PersistenceResponsibilityKind::RelationSelectedFallback,
                    ],
                    ProjectedOperationFragmentKind::DesignatedRemoteInputService => vec![
                        PersistenceResponsibilityKind::IncomingCarrierState,
                        PersistenceResponsibilityKind::OutgoingCarrierState,
                        PersistenceResponsibilityKind::MembershipCapabilityWitnessRefs,
                    ],
                    ProjectedOperationFragmentKind::DesignatedEvaluation => vec![
                        PersistenceResponsibilityKind::DesignatedResultVersion,
                        PersistenceResponsibilityKind::DesignatedReceiptConsumption,
                        PersistenceResponsibilityKind::DesignatedInputFrontier,
                    ],
                    ProjectedOperationFragmentKind::DesignatedResultConsumer => vec![
                        PersistenceResponsibilityKind::ConsumptionIdentity,
                        PersistenceResponsibilityKind::InFlightDeliveryState,
                        PersistenceResponsibilityKind::ReceiptConsumption,
                        PersistenceResponsibilityKind::MembershipCapabilityWitnessRefs,
                    ],
                };
                for responsibility in responsibilities {
                    if !entry.contains(&responsibility) {
                        entry.push(responsibility);
                    }
                }
            }
            entry.sort();
        }
    }

    #[cfg(test)]
    pub(super) fn for_test_remove_locus_responsibility(
        &mut self,
        locus: &str,
        responsibility: PersistenceResponsibilityKind,
    ) {
        if let Some(entry) = self.by_locus.get_mut(locus) {
            entry.retain(|item| *item != responsibility);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeOccurrenceKind {
    Request,
    Serve,
    Reply,
    Receive,
    Publish,
    Observe,
    Consume,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeOccurrenceBinding {
    Required(RuntimeOccurrenceKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ObservationRow {
    source_ref: SourceRef,
    core_ref: String,
    fragment_ref: String,
    redaction: &'static str,
    observation_row_ref: String,
    edge_identity: Option<(String, CommunicationEdgeKind, String, String)>,
    edge_ref: Option<String>,
    operation_id: String,
    occurrence: RuntimeOccurrenceBinding,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ObservationPlan {
    rows: Vec<ObservationRow>,
}

impl ObservationPlan {
    pub(crate) fn rows(&self) -> &[ObservationRow] {
        &self.rows
    }

    pub(crate) fn add_local_fragment(
        &mut self,
        source_ref: SourceRef,
        core_ref: impl Into<String>,
        fragment_ref: impl Into<String>,
        kind: ProjectedOperationFragmentKind,
    ) {
        let core_ref = core_ref.into();
        let fragment_ref = fragment_ref.into();
        let Some(occurrence) = (match kind {
            ProjectedOperationFragmentKind::OwnerRmwExecution => Some(RuntimeOccurrenceKind::Serve),
            ProjectedOperationFragmentKind::RelationPublication => {
                Some(RuntimeOccurrenceKind::Publish)
            }
            ProjectedOperationFragmentKind::ConsumerLocalRelationProjection => {
                Some(RuntimeOccurrenceKind::Observe)
            }
            ProjectedOperationFragmentKind::DesignatedRemoteInputService
            | ProjectedOperationFragmentKind::DesignatedEvaluation => {
                Some(RuntimeOccurrenceKind::Serve)
            }
            ProjectedOperationFragmentKind::DesignatedResultConsumer => {
                Some(RuntimeOccurrenceKind::Consume)
            }
            ProjectedOperationFragmentKind::OwnerRequestInvocation => None,
        }) else {
            return;
        };
        self.rows.push(ObservationRow {
            source_ref,
            core_ref: core_ref.clone(),
            fragment_ref,
            redaction: "reference-only",
            observation_row_ref: format!("observation:artifact:{}", self.rows.len()),
            edge_identity: None,
            edge_ref: None,
            operation_id: core_ref,
            occurrence: RuntimeOccurrenceBinding::Required(occurrence),
        });
    }

    pub(crate) fn row_for_edge_occurrence(
        &self,
        operation: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
        occurrence: RuntimeOccurrenceKind,
    ) -> Option<&ObservationRow> {
        self.rows.iter().find(|row| {
            row.edge_identity.as_ref().is_some_and(|identity| {
                identity.0 == operation
                    && identity.1 == kind
                    && identity.2 == source
                    && identity.3 == target
            }) && row.occurrence == RuntimeOccurrenceBinding::Required(occurrence.clone())
        })
    }

    pub(crate) fn row_for_artifact_occurrence(
        &self,
        artifact_ref: &str,
        occurrence: RuntimeOccurrenceKind,
    ) -> Option<&ObservationRow> {
        self.rows.iter().find(|row| {
            row.fragment_ref == artifact_ref
                && row.occurrence == RuntimeOccurrenceBinding::Required(occurrence.clone())
        })
    }

    pub(crate) fn row_for_fragment_occurrence(
        &self,
        fragment_ref: &str,
        occurrence: RuntimeOccurrenceKind,
    ) -> Option<&ObservationRow> {
        self.row_for_artifact_occurrence(fragment_ref, occurrence)
    }

    pub(crate) fn occurrence_kinds_for_edge(&self, edge_ref: &str) -> Vec<RuntimeOccurrenceKind> {
        self.rows
            .iter()
            .filter(|row| row.edge_ref.as_deref() == Some(edge_ref))
            .map(|row| match &row.occurrence {
                RuntimeOccurrenceBinding::Required(kind) => kind.clone(),
            })
            .collect()
    }

    pub(crate) fn all_rows_for_edge_ref_have_no_actual_occurrence(&self, edge_ref: &str) -> bool {
        self.rows
            .iter()
            .filter(|row| row.edge_ref.as_deref() == Some(edge_ref))
            .all(|row| !row.has_actual_runtime_occurrence())
    }

    pub(crate) fn all_planned_rows_have_no_actual_occurrence(&self) -> bool {
        self.rows
            .iter()
            .all(|row| !row.has_actual_runtime_occurrence())
    }

    pub(crate) fn unique_row_ref_count_for_operation(&self, operation: &str) -> usize {
        self.rows
            .iter()
            .filter(|row| row.operation_id == operation)
            .map(|row| row.observation_row_ref.clone())
            .collect::<BTreeSet<_>>()
            .len()
    }

    pub(crate) fn row_count_for_operation(&self, operation: &str) -> usize {
        self.rows
            .iter()
            .filter(|row| row.operation_id == operation)
            .count()
    }

    pub(crate) fn edge_refs_for_operation(&self, operation: &str) -> EdgeRefs {
        EdgeRefs(
            self.rows
                .iter()
                .filter(|row| row.operation_id == operation)
                .filter_map(|row| row.edge_ref.clone())
                .collect(),
        )
    }

    pub(super) fn add_required_rows(&mut self, plan: &CommunicationPlan) {
        for edge in &plan.edges {
            for slot in edge.carrier_contract.required_occurrence_slots() {
                let (occurrence, fragment_ref) = match (edge.kind, slot) {
                    (
                        CommunicationEdgeKind::OwnerRequest
                        | CommunicationEdgeKind::DesignatedInputRequest,
                        CarrierOccurrenceSlotKind::Request,
                    ) => (RuntimeOccurrenceKind::Request, &edge.source_fragment_ref),
                    (
                        CommunicationEdgeKind::OwnerReplyReceipt
                        | CommunicationEdgeKind::DesignatedInputReceipt,
                        CarrierOccurrenceSlotKind::Request | CarrierOccurrenceSlotKind::Receive,
                    ) => (
                        match slot {
                            CarrierOccurrenceSlotKind::Request => RuntimeOccurrenceKind::Request,
                            CarrierOccurrenceSlotKind::Receive => RuntimeOccurrenceKind::Receive,
                            _ => unreachable!("receipt lifecycle restricts occurrence slots"),
                        },
                        &edge.target_fragment_ref,
                    ),
                    (
                        CommunicationEdgeKind::OwnerReplyReceipt
                        | CommunicationEdgeKind::DesignatedInputReceipt,
                        CarrierOccurrenceSlotKind::Serve | CarrierOccurrenceSlotKind::Reply,
                    ) => (
                        match slot {
                            CarrierOccurrenceSlotKind::Serve => RuntimeOccurrenceKind::Serve,
                            CarrierOccurrenceSlotKind::Reply => RuntimeOccurrenceKind::Reply,
                            _ => unreachable!("receipt lifecycle restricts occurrence slots"),
                        },
                        &edge.source_fragment_ref,
                    ),
                    (
                        CommunicationEdgeKind::RelationProjectionPublication,
                        CarrierOccurrenceSlotKind::Publish,
                    ) => (RuntimeOccurrenceKind::Publish, &edge.source_fragment_ref),
                    (
                        CommunicationEdgeKind::RelationProjectionPublication,
                        CarrierOccurrenceSlotKind::Observe,
                    ) => (RuntimeOccurrenceKind::Observe, &edge.target_fragment_ref),
                    (
                        CommunicationEdgeKind::DesignatedResultDelivery,
                        CarrierOccurrenceSlotKind::Publish,
                    ) => (RuntimeOccurrenceKind::Publish, &edge.source_fragment_ref),
                    (
                        CommunicationEdgeKind::DesignatedResultDelivery,
                        CarrierOccurrenceSlotKind::Receive,
                    ) => (RuntimeOccurrenceKind::Receive, &edge.target_fragment_ref),
                    (
                        CommunicationEdgeKind::DesignatedResultDelivery,
                        CarrierOccurrenceSlotKind::Consume,
                    ) => (RuntimeOccurrenceKind::Consume, &edge.target_fragment_ref),
                    _ => continue,
                };
                let row = ObservationRow {
                    source_ref: edge.source_ref.clone(),
                    core_ref: edge.core_ref.clone().unwrap_or_default(),
                    fragment_ref: fragment_ref.clone(),
                    redaction: "reference-only",
                    observation_row_ref: format!("observation:{}:{occurrence:?}", edge.edge_ref),
                    edge_identity: Some((
                        edge.operation.clone(),
                        edge.kind,
                        edge.source_locus.clone(),
                        edge.target_locus.clone(),
                    )),
                    edge_ref: Some(edge.edge_ref.clone()),
                    operation_id: edge.operation.clone(),
                    occurrence: RuntimeOccurrenceBinding::Required(occurrence),
                };
                if !self.rows.contains(&row) {
                    self.rows.push(row);
                }
            }
        }
    }

    pub(crate) fn all_rows_source_core_artifact_bound_observer_safe_and_redacted(&self) -> bool {
        self.rows.iter().all(|row| {
            !row.source_ref.path.is_empty()
                && !row.core_ref.is_empty()
                && !row.fragment_ref.is_empty()
                && row.redaction == "reference-only"
        })
    }
}

impl ObservationRow {
    pub(crate) fn runtime_occurrence_binding(&self) -> RuntimeOccurrenceBinding {
        self.occurrence.clone()
    }

    pub(crate) const fn has_actual_runtime_occurrence(&self) -> bool {
        false
    }

    pub(crate) fn redaction_policy(&self) -> ReferenceOnlyRedactionPolicy {
        ReferenceOnlyRedactionPolicy
    }

    pub(crate) fn observation_row_ref(&self) -> &str {
        &self.observation_row_ref
    }

    pub(crate) fn fragment_ref(&self) -> &str {
        &self.fragment_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ProjectedSourceMap {
    entries: BTreeMap<String, CorrespondenceEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CorrespondenceEntry {
    source_ref: SourceRef,
    core_ref: Option<String>,
    artifact_ref: Option<String>,
    edge_ref: Option<String>,
    plan_ref: Option<String>,
    source_fragment_ref: Option<String>,
    target_fragment_ref: Option<String>,
    checked_core_identity: CheckedCoreIdentity,
}

impl CorrespondenceEntry {
    pub(crate) fn source_ref(&self) -> SourceRefView {
        SourceRefView::new(&self.source_ref)
    }
    pub(crate) fn core_ref(&self) -> Option<&str> {
        self.core_ref.as_deref()
    }
    pub(crate) fn artifact_ref(&self) -> Option<&str> {
        self.artifact_ref.as_deref()
    }
    pub(crate) fn edge_ref(&self) -> Option<&str> {
        self.edge_ref.as_deref()
    }
    pub(crate) fn plan_ref(&self) -> Option<&str> {
        self.plan_ref.as_deref()
    }
    pub(crate) fn checked_core_identity(&self) -> &CheckedCoreIdentity {
        &self.checked_core_identity
    }
    pub(crate) fn source_fragment_ref(&self) -> Option<&String> {
        self.source_fragment_ref.as_ref()
    }
    pub(crate) fn target_fragment_ref(&self) -> Option<&String> {
        self.target_fragment_ref.as_ref()
    }
}

impl ProjectedSourceMap {
    pub(crate) fn all_entries_source_core_artifact_bound_to_source(&self, fixture: &str) -> bool {
        !self.entries.is_empty()
            && self.entries.iter().all(|(artifact, entry)| {
                !artifact.is_empty() && entry.source_ref.path.ends_with(fixture)
            })
    }

    pub(super) fn rebuild(
        &mut self,
        loci: &BTreeMap<String, LocusProgram>,
        edges: &[CommunicationEdge],
        handlers: &[EffectHandlerPlanEntry],
    ) {
        self.entries.clear();
        for program in loci.values() {
            for fragment in &program.operations.entries {
                self.entries.insert(
                    fragment.fragment_ref.clone(),
                    CorrespondenceEntry {
                        source_ref: fragment.source_ref.clone(),
                        core_ref: Some(fragment.core_ref.clone()),
                        artifact_ref: Some(fragment.artifact_ref.clone()),
                        edge_ref: None,
                        plan_ref: Some(format!("plan:{}", fragment.fragment_ref)),
                        source_fragment_ref: None,
                        target_fragment_ref: None,
                        checked_core_identity: fragment.checked_core_identity.clone(),
                    },
                );
            }
        }
        for edge in edges {
            self.entries.insert(
                edge.edge_ref.clone(),
                CorrespondenceEntry {
                    source_ref: edge.source_ref.clone(),
                    core_ref: edge.core_ref.clone(),
                    artifact_ref: Some(edge.source_fragment_ref.clone()),
                    edge_ref: Some(edge.edge_ref.clone()),
                    plan_ref: Some(format!("plan:{}", edge.edge_ref)),
                    source_fragment_ref: Some(edge.source_fragment_ref.clone()),
                    target_fragment_ref: Some(edge.target_fragment_ref.clone()),
                    checked_core_identity: edge.checked_core_identity.clone(),
                },
            );
        }
        for handler in handlers {
            let fragment = loci.get(&handler.locus).and_then(|program| {
                program.operations.entries.iter().find(|fragment| {
                    fragment.operation_id == handler.operation
                        && fragment.source_ref == handler.source_ref
                })
            });
            if let Some(fragment) = fragment {
                self.entries.insert(
                    handler.handler_ref.clone(),
                    CorrespondenceEntry {
                        source_ref: handler.source_ref.clone(),
                        core_ref: handler.core_ref.clone(),
                        artifact_ref: Some(fragment.artifact_ref.clone()),
                        edge_ref: None,
                        plan_ref: Some(handler.handler_ref.clone()),
                        source_fragment_ref: None,
                        target_fragment_ref: None,
                        checked_core_identity: handler.checked_core_identity.clone(),
                    },
                );
            }
        }
    }

    pub(crate) fn entry_for_artifact_ref(
        &self,
        artifact_ref: &str,
    ) -> Option<&CorrespondenceEntry> {
        self.entries
            .values()
            .find(|entry| entry.artifact_ref.as_deref() == Some(artifact_ref))
    }

    pub(crate) fn entry_for_fragment_ref(
        &self,
        fragment_ref: &str,
    ) -> Option<&CorrespondenceEntry> {
        self.entries.get(fragment_ref)
    }

    pub(crate) fn entry_for_edge_ref(&self, edge_ref: &str) -> Option<&CorrespondenceEntry> {
        self.entries.get(edge_ref)
    }

    pub(crate) fn covers_all_operation_fragments(&self, fragments: Sys4ArtifactFragments) -> bool {
        fragments.entries().iter().all(|fragment| {
            self.entry_for_fragment_ref(fragment.fragment_ref())
                .is_some_and(|entry| {
                    entry.source_ref == fragment.source_ref
                        && entry.core_ref.as_deref() == Some(fragment.core_ref.as_str())
                        && entry.artifact_ref.as_deref() == Some(fragment.artifact_ref.as_str())
                        && entry.checked_core_identity == fragment.checked_core_identity
                })
        })
    }

    pub(crate) fn covers_all_effect_handlers(&self, handlers: &EffectHandlerPlan) -> bool {
        handlers.entries().iter().all(|handler| {
            handler.is_source_bound()
                && handler.core_ref.is_some()
                && self.entries.values().any(|entry| {
                    entry.source_ref == handler.source_ref
                        && entry.core_ref == handler.core_ref
                        && entry
                            .plan_ref
                            .as_deref()
                            .is_some_and(|plan_ref| plan_ref.starts_with("handler:"))
                })
        })
    }

    pub(crate) fn count_edge_entries_for_operation(&self, operation: &str) -> usize {
        self.entries
            .values()
            .filter(|entry| {
                entry.edge_ref.is_some() && entry.checked_core_identity.operation_id() == operation
            })
            .count()
    }

    pub(crate) fn covers_all_operations_edges_and_handlers(
        &self,
        result: &GlobalProjectionResult,
    ) -> bool {
        self.covers_all_operation_fragments(result.sys4_artifact_fragments())
            && result.communication_plan.edges().iter().all(|edge| {
                self.entry_for_edge_ref(edge.edge_ref())
                    .is_some_and(|entry| {
                        entry.source_ref == edge.source_ref
                            && entry.core_ref == edge.core_ref
                            && entry.edge_ref.as_deref() == Some(edge.edge_ref())
                            && entry.source_fragment_ref.as_deref()
                                == Some(edge.source_fragment_ref().as_str())
                            && entry.target_fragment_ref.as_deref()
                                == Some(edge.target_fragment_ref().as_str())
                    })
            })
            && self.covers_all_effect_handlers(result.effect_handler_plan())
    }

    pub(super) fn matches(&self, other: &Self) -> bool {
        self.entries == other.entries
    }

    #[cfg(test)]
    pub(crate) fn for_test_rewrite(&mut self, artifact_ref: &str, source_ref: SourceRef) {
        if let Some(entry) = self.entries.get_mut(artifact_ref) {
            entry.source_ref = source_ref;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StaticProjectionReadiness {
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RuntimeAdmissionStatus {
    AwaitingRuntimeSeam,
    BlockedByResidual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BackendProfile {
    St,
    Ow1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BackendIneligibilityReason {
    NoCombinedOwnerSourceOwnerLocus,
    MultipleCombinedOwnerSourceOwnerLoci { count: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BackendEligibility {
    Eligible,
    Ineligible { reason: BackendIneligibilityReason },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackendRequirements {
    ow1: BackendEligibility,
}

impl BackendRequirements {
    pub(crate) const fn supports(&self, profile: BackendProfile) -> bool {
        match profile {
            BackendProfile::St => true,
            BackendProfile::Ow1 => matches!(self.ow1, BackendEligibility::Eligible),
        }
    }

    pub(crate) fn eligibility(&self, profile: BackendProfile) -> BackendEligibility {
        match profile {
            BackendProfile::St => BackendEligibility::Eligible,
            BackendProfile::Ow1 => self.ow1.clone(),
        }
    }

    pub(crate) fn from_combined_owner_source_owner_loci(loci: &BTreeSet<String>) -> Self {
        let ow1 = match loci.len() {
            1 => BackendEligibility::Eligible,
            0 => BackendEligibility::Ineligible {
                reason: BackendIneligibilityReason::NoCombinedOwnerSourceOwnerLocus,
            },
            count => BackendEligibility::Ineligible {
                reason: BackendIneligibilityReason::MultipleCombinedOwnerSourceOwnerLoci { count },
            },
        };
        Self { ow1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GlobalProjectionResult {
    checked_program_identity: CheckedProgramIdentity,
    projection_identity: ProjectionIdentity,
    locus_programs: BTreeMap<String, LocusProgram>,
    communication_plan: CommunicationPlan,
    effect_handler_plan: EffectHandlerPlan,
    relation_graph: ProjectionRelationGraph,
    observation_plan: ObservationPlan,
    persistence_plan: PersistencePlan,
    projected_source_map: ProjectedSourceMap,
    static_readiness: StaticProjectionReadiness,
    runtime_admission_status: RuntimeAdmissionStatus,
    backend_requirements: BackendRequirements,
    static_conflict_policy: StaticConflictPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StaticConflictPolicyKind {
    OneDesignatedResultConsumerFinite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StaticConflictResolution {
    RejectCompetingConsumer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DesignatedResultConsumerConflictPolicy {
    kind: StaticConflictPolicyKind,
    accepted_consumer_locus: String,
    on_competing_consumer: StaticConflictResolution,
}

impl DesignatedResultConsumerConflictPolicy {
    pub(crate) const fn kind(&self) -> StaticConflictPolicyKind {
        self.kind
    }
    pub(crate) fn accepted_consumer_locus(&self) -> &str {
        &self.accepted_consumer_locus
    }
    pub(crate) const fn on_competing_consumer(&self) -> StaticConflictResolution {
        self.on_competing_consumer
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct StaticConflictPolicy {
    designated_result_consumers: BTreeMap<String, DesignatedResultConsumerConflictPolicy>,
}

impl StaticConflictPolicy {
    pub(crate) fn designated_result_consumer(
        &self,
        operation: &str,
    ) -> Option<&DesignatedResultConsumerConflictPolicy> {
        self.designated_result_consumers.get(operation)
    }

    pub(super) fn add_designated_result_consumer(
        &mut self,
        operation: impl Into<String>,
        consumer_locus: impl Into<String>,
    ) {
        self.designated_result_consumers.insert(
            operation.into(),
            DesignatedResultConsumerConflictPolicy {
                kind: StaticConflictPolicyKind::OneDesignatedResultConsumerFinite,
                accepted_consumer_locus: consumer_locus.into(),
                on_competing_consumer: StaticConflictResolution::RejectCompetingConsumer,
            },
        );
    }
}

impl GlobalProjectionResult {
    pub(crate) fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        &self.checked_program_identity
    }

    pub(crate) fn projection_identity(&self) -> &ProjectionIdentity {
        &self.projection_identity
    }

    pub(crate) fn locus_order(&self) -> Vec<&str> {
        self.locus_programs.keys().map(String::as_str).collect()
    }

    pub(crate) fn locus_program(&self, locus: &str) -> Option<&LocusProgram> {
        self.locus_programs.get(locus)
    }

    pub(crate) fn locus_programs(&self) -> LocusPrograms<'_> {
        LocusPrograms {
            programs: &self.locus_programs,
        }
    }

    pub(crate) fn fragment_by_ref(
        &self,
        fragment_ref: &str,
    ) -> Option<&ProjectedOperationFragment> {
        self.locus_programs
            .values()
            .flat_map(|program| program.operations.entries.iter())
            .find(|fragment| fragment.fragment_ref == fragment_ref)
    }

    pub(crate) fn sys4_artifact_fragments(&self) -> Sys4ArtifactFragments {
        Sys4ArtifactFragments {
            entries: self
                .locus_programs
                .values()
                .flat_map(|program| program.operations.entries.iter().cloned())
                .collect(),
        }
    }

    pub(crate) fn communication_plan(&self) -> &CommunicationPlan {
        &self.communication_plan
    }

    pub(crate) fn effect_handler_plan(&self) -> &EffectHandlerPlan {
        &self.effect_handler_plan
    }

    pub(crate) fn relation_graph(&self) -> &ProjectionRelationGraph {
        &self.relation_graph
    }

    pub(crate) fn observation_plan(&self) -> &ObservationPlan {
        &self.observation_plan
    }

    pub(crate) fn persistence_plan(&self) -> &PersistencePlan {
        &self.persistence_plan
    }

    pub(crate) fn projected_source_map(&self) -> &ProjectedSourceMap {
        &self.projected_source_map
    }

    pub(super) fn owner_locus_for_operation(&self, operation: &str) -> Option<&str> {
        self.locus_programs.iter().find_map(|(locus, program)| {
            program
                .has_operation(operation, LocusOperationKind::OwnerRmwEvaluation)
                .then_some(locus.as_str())
        })
    }

    pub(crate) const fn static_readiness(&self) -> StaticProjectionReadiness {
        self.static_readiness
    }

    pub(crate) const fn runtime_admission_status(&self) -> RuntimeAdmissionStatus {
        self.runtime_admission_status
    }

    pub(crate) fn backend_requirements(&self) -> &BackendRequirements {
        &self.backend_requirements
    }

    pub(crate) fn static_conflict_policy(&self) -> &StaticConflictPolicy {
        &self.static_conflict_policy
    }

    pub(crate) fn new(
        checked_program_identity: CheckedProgramIdentity,
        topology_loci: BTreeSet<String>,
        runtime_admission_status: RuntimeAdmissionStatus,
        backend_requirements: BackendRequirements,
    ) -> Self {
        let mut locus_programs = BTreeMap::new();
        for locus in &topology_loci {
            locus_programs.insert(locus.clone(), LocusProgram::new(LocusTag::checked(locus)));
        }
        Self {
            projection_identity: ProjectionIdentity::new(
                checked_program_identity.clone(),
                topology_loci,
            ),
            checked_program_identity,
            locus_programs,
            communication_plan: CommunicationPlan::default(),
            effect_handler_plan: EffectHandlerPlan::default(),
            relation_graph: ProjectionRelationGraph::default(),
            observation_plan: ObservationPlan::default(),
            persistence_plan: PersistencePlan::default(),
            projected_source_map: ProjectedSourceMap::default(),
            static_readiness: StaticProjectionReadiness::Ready,
            runtime_admission_status,
            backend_requirements,
            static_conflict_policy: StaticConflictPolicy::default(),
        }
    }

    pub(crate) fn locus_program_mut(&mut self, locus: &str) -> &mut LocusProgram {
        self.locus_programs
            .get_mut(locus)
            .expect("projection preallocates every validated logical locus")
    }

    pub(crate) fn communication_plan_mut(&mut self) -> &mut CommunicationPlan {
        &mut self.communication_plan
    }

    pub(crate) fn effect_handler_plan_mut(&mut self) -> &mut EffectHandlerPlan {
        &mut self.effect_handler_plan
    }

    pub(crate) fn relation_graph_mut(&mut self) -> &mut ProjectionRelationGraph {
        &mut self.relation_graph
    }

    pub(crate) fn observation_plan_mut(&mut self) -> &mut ObservationPlan {
        &mut self.observation_plan
    }

    pub(crate) fn persistence_plan_mut(&mut self) -> &mut PersistencePlan {
        &mut self.persistence_plan
    }

    pub(crate) fn static_conflict_policy_mut(&mut self) -> &mut StaticConflictPolicy {
        &mut self.static_conflict_policy
    }

    pub(crate) fn finalize(&mut self) {
        for locus in self.locus_programs.values_mut() {
            locus.sort();
        }
        self.communication_plan.sort();
        self.effect_handler_plan.sort();
        self.persistence_plan.assign_loci(&self.locus_programs);
        self.persistence_plan.finalize();
        self.observation_plan
            .add_required_rows(&self.communication_plan);
        self.projected_source_map.rebuild(
            &self.locus_programs,
            &self.communication_plan.edges,
            &self.effect_handler_plan.handlers,
        );
    }

    #[cfg(test)]
    pub(crate) fn for_test_rewrite_fragment_ref(
        &mut self,
        operation: &str,
        kind: ProjectedOperationFragmentKind,
        opaque_ref: &str,
    ) -> Option<&ProjectedOperationFragment> {
        for program in self.locus_programs.values_mut() {
            if let Some(fragment) = program
                .operations
                .entries
                .iter_mut()
                .find(|fragment| fragment.operation_id == operation && fragment.kind == kind)
            {
                fragment.fragment_ref = opaque_ref.to_string();
                return Some(fragment);
            }
        }
        None
    }

    #[cfg(test)]
    pub(crate) fn for_test_remove_derived_edge(
        &mut self,
        operation: &str,
        kind: CommunicationEdgeKind,
    ) {
        self.communication_plan.for_test_remove(operation, kind);
    }

    #[cfg(test)]
    pub(crate) fn for_test_insert_non_derived_edge(
        &mut self,
        id: &str,
        kind: CommunicationEdgeKind,
        source: &str,
        target: &str,
        operation: &str,
    ) {
        self.communication_plan
            .for_test_insert_non_derived(id, kind, source, target, operation);
    }

    #[cfg(test)]
    pub(crate) fn for_test_move_owner_operation(&mut self, operation: &str, from: &str, to: &str) {
        let source = self
            .locus_programs
            .get_mut(from)
            .expect("test source locus exists");
        let index = source.operations.entries.iter().position(|item| {
            item.operation_id == operation
                && item.kind == ProjectedOperationFragmentKind::OwnerRmwExecution
        });
        if let Some(index) = index {
            let moved = source.operations.entries.remove(index);
            source
                .checked_fragments
                .owner_operations
                .retain(|candidate| candidate != operation);
            let target = self
                .locus_programs
                .get_mut(to)
                .expect("test target locus exists");
            target.operations.entries.push(moved);
            target
                .checked_fragments
                .owner_operations
                .push(operation.to_string());
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_remove_designated_result_consumer_fragment(
        &mut self,
        operation: &str,
        locus: &str,
    ) {
        if let Some(program) = self.locus_programs.get_mut(locus) {
            program.operations.entries.retain(|fragment| {
                !(fragment.operation_id == operation
                    && fragment.kind == ProjectedOperationFragmentKind::DesignatedResultConsumer)
            });
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_move_designated_result_consumer_fragment(
        &mut self,
        operation: &str,
        from: &str,
        to: &str,
    ) {
        let moved = self.locus_programs.get_mut(from).and_then(|program| {
            program
                .operations
                .entries
                .iter()
                .position(|fragment| {
                    fragment.operation_id == operation
                        && fragment.kind == ProjectedOperationFragmentKind::DesignatedResultConsumer
                })
                .map(|index| program.operations.entries.remove(index))
        });
        if let Some(fragment) = moved {
            self.locus_programs
                .get_mut(to)
                .expect("test target locus exists")
                .operations
                .entries
                .push(fragment);
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_enable_designated_result_consumer_expression_leakage(
        &mut self,
        operation: &str,
        locus: &str,
    ) {
        if let Some(fragment) = self.locus_programs.get_mut(locus).and_then(|program| {
            program.operations.entries.iter_mut().find(|fragment| {
                fragment.operation_id == operation
                    && fragment.kind == ProjectedOperationFragmentKind::DesignatedResultConsumer
            })
        }) {
            fragment.designated_result_consumer_expression_leakage = true;
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_replace_checked_program_identity(
        &mut self,
        checked_program_identity: CheckedProgramIdentity,
    ) {
        self.checked_program_identity = checked_program_identity;
    }

    #[cfg(test)]
    pub(crate) fn for_test_rewrite_projected_source_ref(
        &mut self,
        artifact_ref: &str,
        source_ref: SourceRef,
    ) {
        self.projected_source_map
            .for_test_rewrite(artifact_ref, source_ref);
    }

    #[cfg(test)]
    pub(crate) fn for_test_replace_backend_eligibility(
        &mut self,
        profile: BackendProfile,
        eligibility: BackendEligibility,
    ) {
        if profile == BackendProfile::Ow1 {
            self.backend_requirements.ow1 = eligibility;
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test_remove_locus_persistence_responsibility(
        &mut self,
        locus: &str,
        responsibility: PersistenceResponsibilityKind,
    ) {
        self.persistence_plan
            .for_test_remove_locus_responsibility(locus, responsibility);
    }

    #[cfg(test)]
    pub(crate) fn for_test_clear_effect_handler_provenance(
        &mut self,
        operation: &str,
        kind: EffectHandlerKind,
        locus: &str,
    ) {
        self.effect_handler_plan
            .for_test_clear_provenance(operation, kind, locus);
    }
}
