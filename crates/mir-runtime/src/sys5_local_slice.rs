//! Provisional SYS-5 local-slice build/project facade.
//!
//! The facade accepts an ordinary Surface v0 source, checks it once, derives
//! the exact declared logical-locus inventory, and summarizes the resulting
//! SYS-3 projection.  It deliberately does not start SYS-4 dispatch, grant
//! authority, or turn deferred auth/verification obligations into admissions.
//! A separate CLI may consume this module through Rust visibility during the
//! current profile, but that is not a compatibility, public ABI, or wire-format
//! commitment.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt,
};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{
        CheckedSurfaceV0, ResidualObligationKind, check_and_elaborate_surface_v0,
    },
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{
    m9_auth_verification::{
        M9FiniteLocalAdmissionCandidate, M9FiniteLocalAdmissionFact, M9RuntimeExecutionSeam,
    },
    sys3_projection::{
        BackendEligibility, BackendProfile, CommunicationEdgeKind, DeclaredLogicalTopology,
        GlobalProjectionResult, ProjectedOperationFragmentKind, project_checked_core,
    },
    sys4_dispatch::{
        FabricProgram, ObserverSafeM9SemanticRowSets, ObserverSafeM9Summary, SealedFabricAdmission,
        Sys4InitialStateSeed,
    },
};

const PROFILE_NAME: &str = "sys5-local-slice";
const PROFILE_STATUS: &str = "provisional-no-compatibility-promise";
const OBSERVER_SAFETY: &str = "observer-safe-no-raw-authority-capability-witness-payload";
const CHECKED_PROGRAM_REF_DOMAIN: &[u8] = b"mirrorea/sys5/checked-program-ref/v1\0";
const SEALED_INVENTORY_REF_DOMAIN: &[u8] = b"mirrorea/sys5/sealed-inventory-ref/v1\0";
const DEBUG_PATH_REF_DOMAIN: &[u8] = b"mirrorea/sys5/debug-logical-path-ref/v1\0";

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

/// A non-executing checked-and-projected local slice.
#[derive(Clone, PartialEq, Eq)]
pub struct Sys5LocalProject {
    checked: CheckedSurfaceV0,
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

    /// A serializable, observer-safe causal index for this non-executing build.
    pub fn observer_safe_view(&self) -> &Sys5ObserverSafeView {
        &self.observer_safe_view
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
        let seed =
            Sys4InitialStateSeed::for_checked_program(self.checked.program_identity().clone());
        let admission = SealedFabricAdmission::from_m9_execution_seam(&program, seam, seed)
            .map_err(|_| {
                Sys5LocalAdmissionError::new(
                    Sys5LocalAdmissionErrorKind::IncompleteSourceDerivedInventory,
                )
            })?;
        let inventory = Sys5AdmissionInventory::from_checked(&self.checked);
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

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn into_parts_for_sys4(self) -> (FabricProgram, SealedFabricAdmission) {
        (self.program, self.admission)
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
    pub source_path: String,
    pub source_span: Sys5SourceSpan,
    pub operation_id: String,
    pub core_kind: String,
    pub core_ref: String,
    pub artifact_locus: String,
    pub artifact_kind: String,
    pub fragment_ref: String,
    pub checked_program_identity: String,
}

/// A source position with no source text.  The logical source path remains in
/// the containing summary row so a viewer cannot recover host paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Sys5SourceSpan {
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
                source_path,
                source_span,
                operation_id: fragment.operation_id().to_string(),
                core_kind: core_kind_name(fragment.fragment_kind()).to_string(),
                core_ref,
                artifact_locus: locus.to_string(),
                artifact_kind,
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
        projection,
        semantic_summary: summary,
        observer_safe_view,
    })
}

fn normalize_logical_source_path(path: &str) -> Result<String, Sys5LocalSliceError> {
    if path.trim().is_empty()
        || path.starts_with('/')
        || path.starts_with('\\')
        || path.contains(':')
        || path.contains('\\')
        || path
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(Sys5LocalSliceError::InvalidLogicalSourcePath);
    }
    Ok(path.to_string())
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

fn summary_source_span(source_ref: &SourceRef) -> Sys5SourceSpan {
    Sys5SourceSpan {
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
