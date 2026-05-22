use std::path::Path;

use mir_semantics::full_system_v1::{
    FullSystemV1Obligation, ProjectionDiagnostic, ProjectionIr, ProjectionPreservationReport,
    ProjectionTargetManifest, project_textual_mir_module_path,
};
use serde::{Deserialize, Serialize};

pub const FULL_SYSTEM_V1_PROJECTION_SURFACE_KIND: &str = "full_system_v1_projection_report";
pub const FULL_SYSTEM_V1_PROJECTION_SCOPE: &str = "full-system-v1-projection-v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ProjectionRuntimeReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "projection_scope")]
    pub projection_scope: String,
    pub accepted: bool,
    pub projection_id: String,
    pub source_path: String,
    pub request_path: String,
    pub projection_ir: ProjectionIr,
    pub target_manifests: Vec<ProjectionTargetManifest>,
    pub preservation_report: ProjectionPreservationReport,
    pub diagnostics: Vec<ProjectionDiagnostic>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub final_public_api_frozen: bool,
}

pub fn project_full_system_v1_path(
    source_path: impl AsRef<Path>,
    request_path: impl AsRef<Path>,
) -> FullSystemV1ProjectionRuntimeReport {
    let report = project_textual_mir_module_path(source_path, request_path);
    FullSystemV1ProjectionRuntimeReport {
        surface_kind: surface_kind(),
        projection_scope: projection_scope(),
        accepted: report.accepted,
        projection_id: report.projection_id,
        source_path: report.source_path,
        request_path: report.request_path,
        projection_ir: report.projection_ir,
        target_manifests: report.target_manifests,
        preservation_report: report.preservation_report,
        diagnostics: report.diagnostics,
        residual_obligations: report.residual_obligations,
        final_public_api_frozen: report.final_public_api_frozen,
    }
}

fn surface_kind() -> String {
    FULL_SYSTEM_V1_PROJECTION_SURFACE_KIND.to_string()
}

fn projection_scope() -> String {
    FULL_SYSTEM_V1_PROJECTION_SCOPE.to_string()
}
