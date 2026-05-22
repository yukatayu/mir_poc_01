use std::{collections::BTreeSet, path::Path};

use mir_semantics::full_system_v1::FullSystemV1Obligation;
use serde::{Deserialize, Serialize};

use crate::{
    full_system_v1_projection::project_full_system_v1_path,
    full_system_v1_session::{
        FullSystemV1SessionReport, run_full_system_v1_session_with_provider_boundaries_path,
    },
};

pub const FULL_SYSTEM_V1_LOCAL_SPLIT_SURFACE_KIND: &str = "full_system_v1_local_split_report";
pub const FULL_SYSTEM_V1_LOCAL_SPLIT_SCOPE: &str = "full-system-v1-local-role-split-v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullSystemV1LocalRoleExecutionKind {
    AuthoritativeRuntime,
    PassiveEndpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1LocalSplitDiagnostic {
    pub code: String,
    pub message: String,
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1LocalTargetReport {
    pub target_id: String,
    pub role: String,
    pub accepted: bool,
    pub execution_kind: FullSystemV1LocalRoleExecutionKind,
    pub place_refs: Vec<String>,
    pub admitted_entry_transitions: Vec<String>,
    pub launched_entry_transitions: Vec<String>,
    pub inbound_boundary_refs: Vec<String>,
    pub outbound_boundary_refs: Vec<String>,
    pub inbound_packet_schema_refs: Vec<String>,
    pub outbound_packet_schema_refs: Vec<String>,
    pub inbound_ffi_schema_refs: Vec<String>,
    pub outbound_ffi_schema_refs: Vec<String>,
    pub save_load_authority: bool,
    pub prediction_allowed: bool,
    pub observer_safe_summary: String,
    pub runtime_sessions: Vec<FullSystemV1SessionReport>,
    pub diagnostics: Vec<FullSystemV1LocalSplitDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1LocalSplitRuntimeReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "split_scope")]
    pub split_scope: String,
    pub launch_mode: String,
    pub accepted: bool,
    pub projection_id: String,
    pub source_path: String,
    pub request_path: String,
    pub selected_target_id: Option<String>,
    pub entry_override: Option<String>,
    pub target_reports: Vec<FullSystemV1LocalTargetReport>,
    pub rejected_rows: Vec<String>,
    pub diagnostics: Vec<FullSystemV1LocalSplitDiagnostic>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub final_public_api_frozen: bool,
}

pub fn run_full_system_v1_local_split_path(
    source_path: impl AsRef<Path>,
    request_path: impl AsRef<Path>,
    input: i64,
    selected_target_id: Option<&str>,
    entry_override: Option<&str>,
) -> FullSystemV1LocalSplitRuntimeReport {
    let source_path_text = source_path.as_ref().display().to_string();
    let request_path_text = request_path.as_ref().display().to_string();
    let projection = project_full_system_v1_path(&source_path, &request_path);
    let projection_id = projection.projection_id.clone();
    let selected_target_id = selected_target_id.map(str::to_string);
    let entry_override = entry_override.map(str::to_string);

    let mut diagnostics = Vec::new();
    let mut rejected_rows = Vec::new();
    let residual_obligations = projection
        .residual_obligations
        .iter()
        .filter(|row| row.code != "server_client_runtime_split_deferred")
        .cloned()
        .chain(std::iter::once(FullSystemV1Obligation {
            code: "docker_process_carrier_deferred".to_string(),
            message:
                "same-binary local role split is actualized, but Docker/container role carrier remains later work"
                    .to_string(),
        }))
        .collect::<Vec<_>>();

    if !projection.accepted {
        diagnostics.extend(projection.diagnostics.iter().map(|row| {
            FullSystemV1LocalSplitDiagnostic {
                code: row.code.clone(),
                message: row.message.clone(),
                target_id: None,
            }
        }));
        rejected_rows.extend(projection.preservation_report.rejected_rows.clone());
        return FullSystemV1LocalSplitRuntimeReport {
            surface_kind: surface_kind(),
            split_scope: split_scope(),
            launch_mode: "same_binary_local_role_wrapper".to_string(),
            accepted: false,
            projection_id,
            source_path: source_path_text,
            request_path: request_path_text,
            selected_target_id,
            entry_override,
            target_reports: Vec::new(),
            rejected_rows,
            diagnostics,
            residual_obligations,
            final_public_api_frozen: false,
        };
    }

    let manifests = projection
        .target_manifests
        .iter()
        .filter(|target| {
            selected_target_id
                .as_deref()
                .map(|selected| selected == target.target_id)
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();

    if manifests.is_empty() {
        diagnostics.push(FullSystemV1LocalSplitDiagnostic {
            code: "unknown_target_id".to_string(),
            message: format!(
                "projection `{}` does not declare target `{}`",
                projection_id,
                selected_target_id
                    .clone()
                    .unwrap_or_else(|| "<none>".to_string())
            ),
            target_id: selected_target_id.clone(),
        });
        if let Some(target_id) = selected_target_id.clone() {
            rejected_rows.push(format!("{target_id}:unknown_target_id"));
        }
    }

    let target_reports = manifests
        .into_iter()
        .map(|target| {
            build_target_report(
                &source_path,
                &projection,
                target,
                input,
                entry_override.as_deref(),
            )
        })
        .collect::<Vec<_>>();

    for report in &target_reports {
        for diagnostic in &report.diagnostics {
            diagnostics.push(diagnostic.clone());
            rejected_rows.push(format!("{}:{}", report.target_id, diagnostic.code));
        }
    }

    let accepted = diagnostics.is_empty() && target_reports.iter().all(|row| row.accepted);

    FullSystemV1LocalSplitRuntimeReport {
        surface_kind: surface_kind(),
        split_scope: split_scope(),
        launch_mode: "same_binary_local_role_wrapper".to_string(),
        accepted,
        projection_id,
        source_path: source_path_text,
        request_path: request_path_text,
        selected_target_id,
        entry_override,
        target_reports,
        rejected_rows: sorted_vec(rejected_rows.into_iter()),
        diagnostics,
        residual_obligations,
        final_public_api_frozen: false,
    }
}

fn build_target_report(
    source_path: impl AsRef<Path>,
    projection: &crate::full_system_v1_projection::FullSystemV1ProjectionRuntimeReport,
    target: &mir_semantics::full_system_v1::ProjectionTargetManifest,
    input: i64,
    entry_override: Option<&str>,
) -> FullSystemV1LocalTargetReport {
    let inbound_boundaries = projection
        .projection_ir
        .boundaries
        .iter()
        .filter(|boundary| boundary.to_target == target.target_id)
        .collect::<Vec<_>>();
    let outbound_boundaries = projection
        .projection_ir
        .boundaries
        .iter()
        .filter(|boundary| boundary.from_target == target.target_id)
        .collect::<Vec<_>>();

    let admitted_entry_transitions = target.entry_transitions.clone();
    let launched_entry_transitions = entry_override
        .map(|entry| vec![entry.to_string()])
        .unwrap_or_else(|| admitted_entry_transitions.clone());

    let mut diagnostics = Vec::new();
    if let Some(entry) = entry_override {
        if !admitted_entry_transitions.iter().any(|row| row == entry) {
            diagnostics.push(FullSystemV1LocalSplitDiagnostic {
                code: "entry_transition_not_admitted".to_string(),
                message: format!(
                    "target `{}` role `{}` cannot launch undeclared entry transition `{entry}`",
                    target.target_id, target.role
                ),
                target_id: Some(target.target_id.clone()),
            });
        }
    }

    let mut runtime_sessions = Vec::new();
    let admitted_provider_boundaries = outbound_boundaries
        .iter()
        .map(|boundary| boundary.boundary_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if diagnostics.is_empty() {
        for entry in &launched_entry_transitions {
            runtime_sessions.push(run_full_system_v1_session_with_provider_boundaries_path(
                &source_path,
                entry,
                input,
                &admitted_provider_boundaries,
            ));
        }
    }

    for session in &runtime_sessions {
        if session.runtime.accepted {
            continue;
        }
        let (code, message) = session
            .runtime
            .runtime_rejection
            .as_ref()
            .map(|rejection| (rejection.code.clone(), rejection.message.clone()))
            .or_else(|| {
                session
                    .runtime
                    .diagnostics
                    .first()
                    .map(|diagnostic| (diagnostic.code.clone(), diagnostic.message.clone()))
            })
            .unwrap_or_else(|| {
                (
                    "target_runtime_rejection".to_string(),
                    session.runtime.observer_safe_summary.clone(),
                )
            });
        diagnostics.push(FullSystemV1LocalSplitDiagnostic {
            code,
            message,
            target_id: Some(target.target_id.clone()),
        });
    }

    let accepted =
        diagnostics.is_empty() && runtime_sessions.iter().all(|row| row.runtime.accepted);
    let execution_kind = if launched_entry_transitions.is_empty() {
        FullSystemV1LocalRoleExecutionKind::PassiveEndpoint
    } else {
        FullSystemV1LocalRoleExecutionKind::AuthoritativeRuntime
    };
    let observer_safe_summary = if launched_entry_transitions.is_empty() {
        format!(
            "accepted passive {} endpoint with {} inbound boundary refs and {} outbound boundary refs",
            target.role,
            inbound_boundaries.len(),
            outbound_boundaries.len()
        )
    } else {
        runtime_sessions
            .first()
            .map(|row| row.observer_safe_summary.clone())
            .unwrap_or_else(|| {
                format!(
                    "accepted {} endpoint launched {} entry transition(s)",
                    target.role,
                    launched_entry_transitions.len()
                )
            })
    };

    FullSystemV1LocalTargetReport {
        target_id: target.target_id.clone(),
        role: target.role.clone(),
        accepted,
        execution_kind,
        place_refs: target.place_refs.clone(),
        admitted_entry_transitions,
        launched_entry_transitions,
        inbound_boundary_refs: sorted_vec(
            inbound_boundaries
                .iter()
                .map(|boundary| boundary.boundary_ref.clone()),
        ),
        outbound_boundary_refs: sorted_vec(
            outbound_boundaries
                .iter()
                .map(|boundary| boundary.boundary_ref.clone()),
        ),
        inbound_packet_schema_refs: sorted_vec(
            inbound_boundaries
                .iter()
                .filter_map(|boundary| boundary.packet_schema_ref.clone()),
        ),
        outbound_packet_schema_refs: sorted_vec(
            outbound_boundaries
                .iter()
                .filter_map(|boundary| boundary.packet_schema_ref.clone()),
        ),
        inbound_ffi_schema_refs: sorted_vec(
            inbound_boundaries
                .iter()
                .filter_map(|boundary| boundary.ffi_schema_ref.clone()),
        ),
        outbound_ffi_schema_refs: sorted_vec(
            outbound_boundaries
                .iter()
                .filter_map(|boundary| boundary.ffi_schema_ref.clone()),
        ),
        save_load_authority: target.save_load_authority,
        prediction_allowed: target.prediction_allowed,
        observer_safe_summary,
        runtime_sessions,
        diagnostics,
    }
}

fn sorted_vec<T>(values: impl IntoIterator<Item = T>) -> Vec<T>
where
    T: Ord,
{
    let mut rows = values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn surface_kind() -> String {
    FULL_SYSTEM_V1_LOCAL_SPLIT_SURFACE_KIND.to_string()
}

fn split_scope() -> String {
    FULL_SYSTEM_V1_LOCAL_SPLIT_SCOPE.to_string()
}
