//! Provisional SYS-5 local-slice build/project facade.
//!
//! The facade accepts an ordinary Surface v0 source, checks it once, derives
//! the exact declared logical-locus inventory, and summarizes the resulting
//! SYS-3 projection.  It deliberately does not start SYS-4 dispatch, grant
//! authority, or turn deferred auth/verification obligations into admissions.
//! A separate CLI may consume this module through Rust visibility during the
//! current profile, but that is not a compatibility, public ABI, or wire-format
//! commitment.

use std::{error::Error, fmt};

use mir_ast::surface_v0::FixtureSource;
use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{ResidualObligationKind, check_and_elaborate_surface_v0},
};
use serde::Serialize;

use crate::sys3_projection::{
    CommunicationEdgeKind, DeclaredLogicalTopology, ProjectedOperationFragmentKind,
    project_checked_core,
};

const PROFILE_NAME: &str = "sys5-local-slice";
const PROFILE_STATUS: &str = "provisional-no-compatibility-promise";
const OBSERVER_SAFETY: &str = "observer-safe-no-raw-authority-capability-witness-payload";
const CHECKED_PROGRAM_REF_DOMAIN: &[u8] = b"mirrorea/sys5/checked-program-ref/v1\0";
const FNV1A64_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV1A64_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Ordinary source supplied directly to the provisional build/project facade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5SourceInput {
    logical_source_path: String,
    source_text: String,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sys5LocalProject {
    semantic_summary: Sys5SemanticSummary,
    observer_safe_view: Sys5ObserverSafeView,
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

/// Returns an opaque, deterministic reference for the exact checked-program
/// identity.  The FNV-1a-64 input is the fixed, NUL-terminated domain above,
/// then the stable-key byte length as an eight-byte little-endian integer, and
/// then `CheckedProgramIdentity::stable_key()` bytes.  Only the fixed
/// lower-case hexadecimal digest is serialized.  This is an observer-safe
/// reference, not a collision-resistant proof or a public identity format.
fn checked_program_identity_ref(stable_key: &str) -> String {
    let mut hash = FNV1A64_OFFSET_BASIS;
    for byte in CHECKED_PROGRAM_REF_DOMAIN
        .iter()
        .copied()
        .chain(
            u64::try_from(stable_key.len())
                .expect("logical source input length fits u64")
                .to_le_bytes(),
        )
        .chain(stable_key.bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV1A64_PRIME);
    }
    format!("sys5-checked-program-ref-v1:{hash:016x}")
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
