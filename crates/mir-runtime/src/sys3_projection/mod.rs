#![cfg_attr(not(test), allow(dead_code, unused_imports))]

//! Deterministic, internal projection of checked Surface v0 Core into locus plans.
//!
//! This is a static compiler boundary.  It does not start execution, choose a
//! deployment, or admit a result into a runtime.

mod lowering;
mod model;
mod validate;

pub(crate) use lowering::project_checked_core;
pub(crate) use model::{
    BackendEligibility, BackendIneligibilityReason, BackendProfile, CarrierContract,
    CarrierFrontierKind, CarrierLifecycleKind, CarrierOccurrenceSlotKind, CarrierProvenanceKind,
    CheckedCoreIdentity, CommunicationEdge, CommunicationEdgeKind, CommunicationPlan,
    DeclaredLogicalTopology, EffectHandlerKind, GlobalProjectionResult,
    I3AdapterCarrierStaticAuthorityRequirementRow, I3AdapterCarrierStaticFacts,
    I3AdapterCarrierStaticVariant, LocusOperationKind, LocusProgram, LocusTag,
    PersistenceResponsibilityKind, ProjectedOperationFragment, ProjectedOperationFragmentKind,
    ProjectedRelationAnchor, ProjectionDiagnosticKind, ProjectionDiagnostics,
    ProjectionRelationGraph, ReferenceOnlyRedactionPolicy, RelationAnchorRole, RelationGraphClaim,
    RelationGraphEdgeSeed, RelationGraphEdgeTag, RuntimeAdmissionStatus, RuntimeOccurrenceBinding,
    RuntimeOccurrenceKind, RuntimeSeamRequirementKind, SeamAuthorityKind, SourceRefView,
    StaticConflictPolicyKind, StaticConflictResolution, StaticProjectionReadiness,
};
pub(crate) use validate::verify_projection;
