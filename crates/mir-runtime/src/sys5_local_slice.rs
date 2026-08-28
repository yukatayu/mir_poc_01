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
        FabricProgram, LocalFabric, ObserverSafeM9SemanticRowSets, ObserverSafeM9Summary,
        RelationPublicationFailureDisposition, RuntimeValue, SealedFabricAdmission, SourceAction,
        Sys4CheckedPatchCandidate, Sys4DispatchDiagnostics, Sys4InitialStateSeed, Sys4LocalCut,
        Sys4PatchDiagnosticKind, Sys4PatchOutcome, Sys4PatchVerdict, Sys4RelationEndpointReceipt,
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

/// An experimental, non-public checked/projected local slice.  It can start
/// the bounded in-process runtime through `Sys5PreparedAdmission`; this type
/// itself retains only checked/projected state, not a live fabric.
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

    /// A serializable, observer-safe causal index for this checked/projected
    /// build. Runtime occurrence joins are exposed only by an admitted live
    /// slice, never by this projection summary alone.
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
            "cut={};program={};admission={};artifact={};startup={};bindings={};principal={};prefix={:?};shadows={:?};sys4={};next_lifecycle_occurrence={}",
            self.cut_id_ref,
            self.checked_program_identity_ref,
            self.sealed_admission_attestation_ref,
            self.artifact_projection_ref,
            self.startup_plan_ref,
            self.bindings_ref,
            self.source_principal_ref,
            self.joined_prefix,
            self.relation_shadows,
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
    next_lifecycle_occurrence: u64,
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
        )
    }

    pub const fn rejected_before_m9_authority_use(&self) -> bool {
        matches!(
            self.kind,
            Sys5VerticalDiagnosticKind::UnknownSourceOperation
                | Sys5VerticalDiagnosticKind::UnknownSourceValue
                | Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed
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
    performed_m8_semantic_consumption: bool,
    returned_from_designated_cache_after_authority_revalidation: bool,
    relation_shadow: Option<Sys5RelationObserverShadow>,
    no_direct_cross_locus_store_mutation: bool,
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
}

/// One compact observer-safe causal view.  The values and private M9 material
/// remain in the runtime; this report carries only typed references and
/// status rows needed to join source through local occurrences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5VerticalJoinedReport {
    // This is an event prefix, rather than a set of labels.  A set made a
    // restored cut look complete while losing the order in which its actual
    // source→runtime evidence was observed.  Retain insertion order and
    // deduplicate exact repeated rows at the recording boundary.
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
        if !self.rows.contains(&row) {
            self.rows.push(row);
        }
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
        let outcome = sys5_patch_outcome(&sys4_outcome);
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
        let receipt = self
            .fabric
            .fresh_reacquire_relation_primary(relation)
            .map_err(|_| {
                Sys5VerticalSliceError::new(Sys5VerticalDiagnosticKind::RelationTransitionRejected)
            })?;
        self.relation_receipt_from_sys4(relation, receipt)
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
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: false,
            relation_shadow: Some(relation_shadow),
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
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: false,
            relation_shadow: None,
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
            performed_m8_semantic_consumption: receipt.performed_m8_semantic_consumption(),
            returned_from_designated_cache_after_authority_revalidation: receipt
                .returned_from_designated_cache_after_authority_revalidation(),
            relation_shadow: context.relation_shadow,
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
            performed_m8_semantic_consumption: false,
            returned_from_designated_cache_after_authority_revalidation: true,
            relation_shadow: None,
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

fn sys5_patch_outcome(outcome: &Sys4PatchOutcome) -> Sys5PatchOutcome {
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
