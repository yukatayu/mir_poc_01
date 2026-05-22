use std::{collections::BTreeSet, fs, path::Path};

use mir_semantics::full_system_v1::FullSystemV1Obligation;
use serde::{Deserialize, Serialize};

use crate::{
    full_system_v1_provider_admission::{
        FullSystemV1ProviderAdmissionReport, run_full_system_v1_provider_admission_path,
    },
    posegraph_runtime::{
        PoseGraphRuntimeOutcome, PoseGraphRuntimeReport, PoseGraphTransformNode,
        run_posegraph_runtime_package_path,
    },
};

pub const FULL_SYSTEM_V1_RENDERER_POSE_BACKEND_SURFACE_KIND: &str =
    "full_system_v1_renderer_pose_backend_report";
pub const FULL_SYSTEM_V1_RENDERER_POSE_BACKEND_SCOPE: &str =
    "full-system-v1-renderer-pose-backend-v0";

const WORLD_SEMANTICS_OWNER: &str = "mir_mirrorea";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1RendererPoseBackendDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1RendererPoseBindingContext {
    pub projection_id: String,
    pub source_module_refs: Vec<String>,
    pub from_target_id: String,
    pub to_target_id: String,
    pub boundary_ref: String,
    pub entry_transition: String,
    pub provider_id: String,
}

#[derive(Debug, Deserialize)]
struct RendererPoseBindingPackage {
    #[serde(default)]
    binding_context: Option<FullSystemV1RendererPoseBindingContext>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1RendererPoseBackendReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "renderer_scope")]
    pub renderer_scope: String,
    pub accepted: bool,
    pub delivery_admitted: bool,
    pub terminal_outcome: String,
    pub blocked_reason: Option<String>,
    pub source_path: String,
    pub request_path: String,
    pub provider_manifest_path: String,
    pub posegraph_package_path: String,
    pub projection_id: String,
    pub provider_id: String,
    pub provider_kind: String,
    pub target_id: String,
    pub target_provider_policy: String,
    pub semantic_owner: String,
    pub pose_snapshot_frontier: Option<String>,
    pub delivered_pose_snapshot_ref: Option<String>,
    pub expected_binding_context: Option<FullSystemV1RendererPoseBindingContext>,
    pub posegraph_binding_context: Option<FullSystemV1RendererPoseBindingContext>,
    pub delivered_nodes: Vec<PoseGraphTransformNode>,
    pub matched_packet_schema_refs: Vec<String>,
    pub matched_ffi_schema_refs: Vec<String>,
    pub diagnostics: Vec<FullSystemV1RendererPoseBackendDiagnostic>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub provider_admission_report: FullSystemV1ProviderAdmissionReport,
    pub posegraph_runtime_report: PoseGraphRuntimeReport,
    pub observer_safe_summary: String,
    pub final_public_api_frozen: bool,
}

pub fn run_full_system_v1_renderer_pose_backend_path(
    source_path: impl AsRef<Path>,
    request_path: impl AsRef<Path>,
    provider_manifest_path: impl AsRef<Path>,
    posegraph_package_path: impl AsRef<Path>,
    input: i64,
) -> FullSystemV1RendererPoseBackendReport {
    let source_path = source_path.as_ref();
    let request_path = request_path.as_ref();
    let provider_manifest_path = provider_manifest_path.as_ref();
    let posegraph_package_path = posegraph_package_path.as_ref();

    let provider_report = run_full_system_v1_provider_admission_path(
        source_path,
        request_path,
        provider_manifest_path,
        input,
    );
    let posegraph_report = run_posegraph_runtime_package_path(posegraph_package_path);

    let mut diagnostics = provider_report
        .diagnostics
        .iter()
        .map(|row| FullSystemV1RendererPoseBackendDiagnostic {
            code: row.code.clone(),
            message: row.message.clone(),
        })
        .collect::<Vec<_>>();
    let mut residual_obligations = provider_report.residual_obligations.clone();
    if !residual_obligations
        .iter()
        .any(|row| row.code == "renderer_vendor_execution_deferred")
    {
        residual_obligations.push(FullSystemV1Obligation {
            code: "renderer_vendor_execution_deferred".to_string(),
            message:
                "renderer pose backend demo proves bounded pose snapshot delivery only, not arbitrary renderer execution"
                    .to_string(),
        });
    }
    if !residual_obligations
        .iter()
        .any(|row| row.code == "posegraph_binding_attestation_deferred")
    {
        residual_obligations.push(FullSystemV1Obligation {
            code: "posegraph_binding_attestation_deferred".to_string(),
            message:
                "renderer pose backend matches binding context and snapshot frontier structurally, but attested PoseGraph package provenance remains later work"
                    .to_string(),
        });
    }

    let pose_snapshot_frontier =
        optional_string(&posegraph_report.runtime_state.pose_snapshot_frontier);
    let delivered_pose_snapshot_ref = extract_pose_snapshot_ref(&provider_report);
    let expected_binding_context = extract_expected_binding_context(&provider_report);
    let posegraph_binding_context = load_posegraph_binding_context(posegraph_package_path);
    let delivered_nodes = posegraph_report.runtime_state.nodes.clone();

    let (accepted, delivery_admitted, terminal_outcome, blocked_reason, observer_safe_summary) =
        if !provider_report.accepted {
            let code = first_provider_code(&provider_report);
            diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                code: "provider_admission_rejected".to_string(),
                message: format!(
                    "renderer pose backend requires admitted provider inventory row; first provider diagnostic `{code}`"
                ),
            });
            (
                false,
                false,
                "provider_admission_rejected".to_string(),
                Some(code.clone()),
                format!("renderer pose backend blocked by provider admission: {code}"),
            )
        } else if provider_report.provider_kind != "renderer" {
            diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                code: "provider_kind_not_renderer".to_string(),
                message: format!(
                    "renderer pose backend requires provider kind `renderer`, found `{}`",
                    provider_report.provider_kind
                ),
            });
            (
                false,
                false,
                "provider_admission_rejected".to_string(),
                Some("provider_kind_not_renderer".to_string()),
                "renderer pose backend blocked by non-renderer provider kind".to_string(),
            )
        } else {
            match posegraph_report.terminal_outcome {
                PoseGraphRuntimeOutcome::ViolationExport => {
                    let reason = posegraph_report
                        .violation
                        .as_ref()
                        .map(|row| row.violation_kind.clone())
                        .unwrap_or_else(|| "posegraph_violation_export".to_string());
                    diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                        code: "posegraph_violation_export".to_string(),
                        message: format!(
                            "renderer pose delivery blocked by posegraph violation export `{reason}`"
                        ),
                    });
                    (
                        false,
                        false,
                        "blocked_posegraph_violation_export".to_string(),
                        Some(reason.clone()),
                        format!(
                            "renderer pose delivery blocked by posegraph violation export: {reason}"
                        ),
                    )
                }
                PoseGraphRuntimeOutcome::RuntimeRejection => {
                    let reason = posegraph_report
                        .rejection
                        .as_ref()
                        .map(|row| row.code.clone())
                        .unwrap_or_else(|| "posegraph_runtime_rejection".to_string());
                    diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                        code: "posegraph_runtime_rejection".to_string(),
                        message: format!(
                            "renderer pose delivery blocked by posegraph runtime rejection `{reason}`"
                        ),
                    });
                    (
                        false,
                        false,
                        "blocked_posegraph_runtime_rejection".to_string(),
                        Some(reason.clone()),
                        format!(
                            "renderer pose delivery blocked by posegraph runtime rejection: {reason}"
                        ),
                    )
                }
                PoseGraphRuntimeOutcome::Accepted => match delivered_pose_snapshot_ref.clone() {
                    None => {
                        diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                            code: "missing_renderer_pose_snapshot_ref".to_string(),
                            message:
                                "renderer pose backend could not find a `pose_snapshot_ref` binding in the authoritative runtime trace"
                                    .to_string(),
                        });
                        (
                            false,
                            false,
                            "blocked_pose_snapshot_binding_missing".to_string(),
                            Some("missing_renderer_pose_snapshot_ref".to_string()),
                            "renderer pose delivery blocked because the runtime trace did not preserve a pose_snapshot_ref binding".to_string(),
                        )
                    }
                    Some(snapshot_ref) => {
                        if expected_binding_context.is_none() {
                            diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                                code: "missing_expected_binding_context".to_string(),
                                message:
                                    "renderer pose backend could not derive a source/provider binding context from the admitted local split report"
                                        .to_string(),
                            });
                            (
                                false,
                                false,
                                "blocked_expected_binding_context_missing".to_string(),
                                Some("missing_expected_binding_context".to_string()),
                                "renderer pose delivery blocked because the admitted source/provider flow did not retain a binding context".to_string(),
                            )
                        } else if posegraph_binding_context.is_none() {
                            diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                                code: "missing_posegraph_binding_context".to_string(),
                                message:
                                    "renderer pose backend requires the PoseGraph package to declare a matching binding_context"
                                        .to_string(),
                            });
                            (
                                false,
                                false,
                                "blocked_posegraph_binding_context_missing".to_string(),
                                Some("missing_posegraph_binding_context".to_string()),
                                "renderer pose delivery blocked because the PoseGraph package did not declare a binding_context".to_string(),
                            )
                        } else if expected_binding_context.as_ref()
                            != posegraph_binding_context.as_ref()
                        {
                            let expected = binding_context_summary(
                                expected_binding_context
                                    .as_ref()
                                    .expect("checked is_some above"),
                            );
                            let actual = binding_context_summary(
                                posegraph_binding_context
                                    .as_ref()
                                    .expect("checked is_some above"),
                            );
                            diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                                code: "posegraph_binding_context_mismatch".to_string(),
                                message: format!(
                                    "renderer pose backend expected binding context `{expected}` but PoseGraph package declared `{actual}`"
                                ),
                            });
                            (
                                false,
                                false,
                                "blocked_posegraph_binding_context_mismatch".to_string(),
                                Some("posegraph_binding_context_mismatch".to_string()),
                                "renderer pose delivery blocked because the PoseGraph package binding_context did not match the admitted source/provider flow".to_string(),
                            )
                        } else if pose_snapshot_frontier.as_deref() != Some(snapshot_ref.as_str()) {
                            diagnostics.push(FullSystemV1RendererPoseBackendDiagnostic {
                                code: "pose_snapshot_frontier_mismatch".to_string(),
                                message: format!(
                                    "renderer requested snapshot `{snapshot_ref}` but posegraph frontier is `{}`",
                                    pose_snapshot_frontier
                                        .clone()
                                        .unwrap_or_else(|| "<none>".to_string())
                                ),
                            });
                            (
                                false,
                                false,
                                "blocked_pose_snapshot_frontier_mismatch".to_string(),
                                Some("pose_snapshot_frontier_mismatch".to_string()),
                                "renderer pose delivery blocked because the requested snapshot does not match the accepted posegraph frontier".to_string(),
                            )
                        } else {
                            (
                                true,
                                true,
                                "delivery_admitted".to_string(),
                                None,
                                format!(
                                    "accepted renderer pose backend delivery for snapshot `{snapshot_ref}` without transferring semantic ownership"
                                ),
                            )
                        }
                    }
                },
            }
        };

    FullSystemV1RendererPoseBackendReport {
        surface_kind: surface_kind(),
        renderer_scope: renderer_scope(),
        accepted,
        delivery_admitted,
        terminal_outcome,
        blocked_reason,
        source_path: source_path.display().to_string(),
        request_path: request_path.display().to_string(),
        provider_manifest_path: provider_manifest_path.display().to_string(),
        posegraph_package_path: posegraph_package_path.display().to_string(),
        projection_id: provider_report.projection_id.clone(),
        provider_id: provider_report.provider_id.clone(),
        provider_kind: provider_report.provider_kind.clone(),
        target_id: provider_report.target_id.clone(),
        target_provider_policy: provider_report.target_provider_policy.clone(),
        semantic_owner: WORLD_SEMANTICS_OWNER.to_string(),
        pose_snapshot_frontier,
        delivered_pose_snapshot_ref,
        expected_binding_context,
        posegraph_binding_context,
        delivered_nodes,
        matched_packet_schema_refs: provider_report.matched_packet_schema_refs.clone(),
        matched_ffi_schema_refs: provider_report.matched_ffi_schema_refs.clone(),
        diagnostics,
        residual_obligations,
        provider_admission_report: provider_report,
        posegraph_runtime_report: posegraph_report,
        observer_safe_summary,
        final_public_api_frozen: false,
    }
}

fn extract_pose_snapshot_ref(report: &FullSystemV1ProviderAdmissionReport) -> Option<String> {
    let split = report.local_split_report.as_ref()?;
    for target in &split.target_reports {
        for session in &target.runtime_sessions {
            for trace in &session.runtime.compute_trace {
                for binding in &trace.local_bindings_summary {
                    if binding.name == "pose_snapshot_ref" && binding.type_name == "Text" {
                        if let Some(text) = parse_text_summary(&binding.summary) {
                            return Some(text);
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_expected_binding_context(
    report: &FullSystemV1ProviderAdmissionReport,
) -> Option<FullSystemV1RendererPoseBindingContext> {
    let split = report.local_split_report.as_ref()?;
    let matched_packet_schema_refs = report
        .matched_packet_schema_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let matched_ffi_schema_refs = report
        .matched_ffi_schema_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    for target in &split.target_reports {
        let matches_provider_lane = target
            .outbound_packet_schema_refs
            .iter()
            .any(|row| matched_packet_schema_refs.contains(row))
            || target
                .outbound_ffi_schema_refs
                .iter()
                .any(|row| matched_ffi_schema_refs.contains(row));
        if !matches_provider_lane {
            continue;
        }
        let entry_transition = target.runtime_sessions.first()?.entry_function.clone();
        let boundary_ref = if target.outbound_boundary_refs.len() == 1 {
            target.outbound_boundary_refs[0].clone()
        } else {
            return None;
        };
        let source_module_refs = target
            .runtime_sessions
            .iter()
            .flat_map(|session| session.runtime.program_module_paths.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        if source_module_refs.is_empty() {
            return None;
        }
        return Some(FullSystemV1RendererPoseBindingContext {
            projection_id: report.projection_id.clone(),
            source_module_refs,
            from_target_id: target.target_id.clone(),
            to_target_id: report.target_id.clone(),
            boundary_ref,
            entry_transition,
            provider_id: report.provider_id.clone(),
        });
    }
    None
}

fn load_posegraph_binding_context(
    posegraph_package_path: &Path,
) -> Option<FullSystemV1RendererPoseBindingContext> {
    let payload = fs::read_to_string(posegraph_package_path).ok()?;
    serde_json::from_str::<RendererPoseBindingPackage>(&payload)
        .ok()?
        .binding_context
}

fn binding_context_summary(context: &FullSystemV1RendererPoseBindingContext) -> String {
    format!(
        "projection={} modules={} {} -> {} via {} entry={} provider={}",
        context.projection_id,
        context.source_module_refs.join(","),
        context.from_target_id,
        context.to_target_id,
        context.boundary_ref,
        context.entry_transition,
        context.provider_id
    )
}

fn parse_text_summary(summary: &str) -> Option<String> {
    let inner = summary
        .strip_prefix("Text(")
        .and_then(|value| value.strip_suffix(')'))?;
    serde_json::from_str(inner).ok()
}

fn first_provider_code(report: &FullSystemV1ProviderAdmissionReport) -> String {
    report
        .diagnostics
        .first()
        .map(|row| row.code.clone())
        .unwrap_or_else(|| "provider_admission_rejected".to_string())
}

fn optional_string(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn surface_kind() -> String {
    FULL_SYSTEM_V1_RENDERER_POSE_BACKEND_SURFACE_KIND.to_string()
}

fn renderer_scope() -> String {
    FULL_SYSTEM_V1_RENDERER_POSE_BACKEND_SCOPE.to_string()
}
