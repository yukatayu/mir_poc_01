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
    BackendEligibility, BackendIneligibilityReason, BackendProfile, CarrierFrontierKind,
    CarrierLifecycleKind, CarrierOccurrenceSlotKind, CarrierProvenanceKind, CheckedCoreIdentity,
    CommunicationEdgeKind, DeclaredLogicalTopology, EffectHandlerKind, GlobalProjectionResult,
    LocusOperationKind, LocusTag, PersistenceResponsibilityKind, ProjectedOperationFragmentKind,
    ProjectedRelationAnchor, ProjectionDiagnosticKind, ProjectionDiagnostics,
    ProjectionRelationGraph, RelationAnchorRole, RelationGraphClaim, RelationGraphEdgeSeed,
    RelationGraphEdgeTag, RuntimeAdmissionStatus, RuntimeOccurrenceBinding, RuntimeOccurrenceKind,
    RuntimeSeamRequirementKind, SeamAuthorityKind, StaticProjectionReadiness,
};
pub(crate) use validate::verify_projection;
