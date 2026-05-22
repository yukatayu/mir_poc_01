use std::{collections::BTreeSet, fs, path::Path};

use mir_semantics::full_system_v1::{FullSystemV1Obligation, ProjectionBoundarySchema};
use serde::{Deserialize, Serialize};

use crate::{
    full_system_v1_local_split::{
        FullSystemV1LocalSplitRuntimeReport, run_full_system_v1_local_split_path,
    },
    full_system_v1_projection::project_full_system_v1_path,
};

pub const FULL_SYSTEM_V1_PROVIDER_ADMISSION_SURFACE_KIND: &str =
    "full_system_v1_provider_admission_report";
pub const FULL_SYSTEM_V1_PROVIDER_ADMISSION_SCOPE: &str = "full-system-v1-provider-admission-v0";

const PROVIDER_MANIFEST_SCHEMA_VERSION: &str = "full-system-v1-provider-manifest-v0";
const WORLD_SEMANTICS_OWNER: &str = "mir_mirrorea";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ProviderAdmissionDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ProviderAdmissionReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "admission_scope")]
    pub admission_scope: String,
    pub accepted: bool,
    pub projection_id: String,
    pub source_path: String,
    pub request_path: String,
    pub provider_manifest_path: String,
    pub provider_id: String,
    pub provider_kind: String,
    pub target_id: String,
    pub target_provider_policy: String,
    pub input_schema: String,
    pub output_schema: String,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub matched_packet_schema_refs: Vec<String>,
    pub matched_ffi_schema_refs: Vec<String>,
    pub terminal_outcome: String,
    pub execution_admitted: bool,
    pub local_split_report: Option<FullSystemV1LocalSplitRuntimeReport>,
    pub diagnostics: Vec<FullSystemV1ProviderAdmissionDiagnostic>,
    pub rejected_rows: Vec<String>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProviderManifest {
    schema_version: String,
    provider_id: String,
    provider_kind: String,
    target_id: String,
    input_schema: String,
    output_schema: String,
    #[serde(default)]
    effect_row: Vec<String>,
    #[serde(default)]
    failure_row: Vec<String>,
    #[serde(default)]
    required_capabilities: Vec<String>,
    authority_policy: ProviderAuthorityPolicy,
    resource_limits: ProviderResourceLimits,
    sandbox_policy: ProviderSandboxPolicy,
    observation_policy: ProviderObservationPolicy,
    redaction_policy: ProviderRedactionPolicy,
    retention_policy: String,
    packet_boundary: Option<String>,
    ffi_boundary: Option<String>,
    native_execution_policy: String,
    wasm_execution_policy: String,
    rollback_replay_cut_policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ProviderAuthorityPolicy {
    semantic_authority_owner: String,
    provider_may_grant_authority: bool,
    provider_may_mutate_world_state: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProviderResourceLimits {
    max_memory_mb: u64,
    max_cpu_ms: u64,
    semantic_state_owner: String,
    provider_handles_are_nonsemantic: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ProviderSandboxPolicy {
    sandbox_required: bool,
    current_status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProviderObservationPolicy {
    provider_receives_redacted_observation_only: bool,
    retention_owner: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ProviderRedactionPolicy {
    provider_may_emit_unredacted_debug: bool,
    redaction_label_required: bool,
}

#[derive(Debug, Clone, Copy)]
struct ProviderKindSpec {
    provider_kind: &'static str,
    expected_input_schema: &'static str,
    expected_output_schema: &'static str,
    admitted_target_policies: &'static [&'static str],
    accepted_terminal_outcome: &'static str,
}

pub fn run_full_system_v1_provider_admission_path(
    source_path: impl AsRef<Path>,
    request_path: impl AsRef<Path>,
    provider_manifest_path: impl AsRef<Path>,
    input: i64,
) -> FullSystemV1ProviderAdmissionReport {
    let source_path = source_path.as_ref();
    let request_path = request_path.as_ref();
    let provider_manifest_path = provider_manifest_path.as_ref();
    let source_path_text = source_path.display().to_string();
    let request_path_text = request_path.display().to_string();
    let provider_manifest_path_text = provider_manifest_path.display().to_string();

    let projection = project_full_system_v1_path(source_path, request_path);
    let local_split_report = if projection.accepted {
        Some(run_full_system_v1_local_split_path(
            source_path,
            request_path,
            input,
            None,
            None,
        ))
    } else {
        None
    };
    let manifest = match load_provider_manifest(provider_manifest_path) {
        Ok(manifest) => manifest,
        Err(diagnostic) => {
            return rejected_report(
                &projection.projection_id,
                &source_path_text,
                &request_path_text,
                &provider_manifest_path_text,
                "",
                "",
                "",
                "",
                local_split_report,
                vec![diagnostic],
                Vec::new(),
                projection.residual_obligations.clone(),
                "rejected",
            );
        }
    };

    let mut diagnostics = Vec::new();
    let mut rejected_rows = Vec::new();

    if !projection.accepted {
        diagnostics.extend(projection.diagnostics.iter().map(|row| {
            FullSystemV1ProviderAdmissionDiagnostic {
                code: row.code.clone(),
                message: row.message.clone(),
            }
        }));
        rejected_rows.extend(projection.preservation_report.rejected_rows.clone());
    }

    let split_residuals = local_split_report
        .as_ref()
        .map(|report| report.residual_obligations.clone())
        .unwrap_or_else(|| projection.residual_obligations.clone());
    let mut residual_obligations = split_residuals
        .into_iter()
        .filter(|row| row.code != "provider_admission_deferred")
        .collect::<Vec<_>>();

    if let Some(split) = &local_split_report {
        if !split.accepted {
            diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
                code: "local_split_rejected".to_string(),
                message: "provider admission requires an accepted local role-split floor"
                    .to_string(),
            });
        }
    }

    let Some(target_manifest) = projection
        .target_manifests
        .iter()
        .find(|target| target.target_id == manifest.target_id)
    else {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "unknown_target_id".to_string(),
            message: format!(
                "projection `{}` does not declare provider target `{}`",
                projection.projection_id, manifest.target_id
            ),
        });
        rejected_rows.push(format!("{}:unknown_target_id", manifest.provider_id));
        return rejected_report(
            &projection.projection_id,
            &source_path_text,
            &request_path_text,
            &provider_manifest_path_text,
            &manifest.provider_id,
            &manifest.provider_kind,
            &manifest.target_id,
            "",
            local_split_report,
            diagnostics,
            rejected_rows,
            residual_obligations,
            "rejected",
        );
    };

    let target_provider_policy = target_manifest.provider_policy.clone();
    let Some(provider_spec) = provider_kind_spec(&manifest.provider_kind) else {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "unknown_provider_kind".to_string(),
            message: format!("unsupported provider kind `{}`", manifest.provider_kind),
        });
        rejected_rows.push(format!("{}:unknown_provider_kind", manifest.provider_id));
        return rejected_report(
            &projection.projection_id,
            &source_path_text,
            &request_path_text,
            &provider_manifest_path_text,
            &manifest.provider_id,
            &manifest.provider_kind,
            &manifest.target_id,
            "",
            local_split_report,
            diagnostics,
            rejected_rows,
            residual_obligations,
            "rejected",
        );
    };

    if manifest.schema_version != PROVIDER_MANIFEST_SCHEMA_VERSION {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_manifest_schema_version_mismatch".to_string(),
            message: format!(
                "expected provider manifest schema `{PROVIDER_MANIFEST_SCHEMA_VERSION}`, found `{}`",
                manifest.schema_version
            ),
        });
    }

    if manifest.input_schema != provider_spec.expected_input_schema
        || manifest.output_schema != provider_spec.expected_output_schema
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_schema_name_mismatch".to_string(),
            message: format!(
                "provider `{}` expects `{}` -> `{}`, found `{}` -> `{}`",
                provider_spec.provider_kind,
                provider_spec.expected_input_schema,
                provider_spec.expected_output_schema,
                manifest.input_schema,
                manifest.output_schema
            ),
        });
    }

    if !provider_spec
        .admitted_target_policies
        .iter()
        .any(|policy| *policy == target_provider_policy)
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_policy_not_admitted".to_string(),
            message: format!(
                "target `{}` provider policy `{}` does not admit provider kind `{}`",
                manifest.target_id, target_provider_policy, manifest.provider_kind
            ),
        });
    }

    let matched_packet_schemas = projection
        .packet_schemas
        .iter()
        .filter(|schema| schema_matches_manifest(schema, &manifest))
        .cloned()
        .collect::<Vec<_>>();
    let matched_ffi_schemas = projection
        .ffi_schemas
        .iter()
        .filter(|schema| schema_matches_manifest(schema, &manifest))
        .cloned()
        .collect::<Vec<_>>();
    if matched_packet_schemas.is_empty() && matched_ffi_schemas.is_empty() {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_boundary_unmatched".to_string(),
            message: format!(
                "provider `{}` did not match any packet/FFI boundary admitted for target `{}`",
                manifest.provider_id, manifest.target_id
            ),
        });
    }

    let matched_effects = matched_packet_schemas
        .iter()
        .chain(matched_ffi_schemas.iter())
        .flat_map(|schema| schema.effect_names.iter().cloned())
        .collect::<BTreeSet<_>>();
    let matched_failures = matched_packet_schemas
        .iter()
        .chain(matched_ffi_schemas.iter())
        .flat_map(|schema| schema.failure_row.iter().cloned())
        .collect::<BTreeSet<_>>();
    let matched_capabilities = matched_packet_schemas
        .iter()
        .chain(matched_ffi_schemas.iter())
        .flat_map(|schema| schema.capability_row.iter().cloned())
        .collect::<BTreeSet<_>>();

    if manifest
        .effect_row
        .iter()
        .any(|effect| !matched_effects.contains(effect))
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_effect_row_outside_projection_boundary".to_string(),
            message: "provider effect row must stay within matched projection boundary effect rows"
                .to_string(),
        });
    }
    if manifest
        .failure_row
        .iter()
        .any(|failure| !matched_failures.contains(failure))
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_failure_row_outside_projection_boundary".to_string(),
            message:
                "provider failure row must stay within matched projection boundary failure rows"
                    .to_string(),
        });
    }
    if manifest
        .required_capabilities
        .iter()
        .any(|capability| !matched_capabilities.contains(capability))
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_over_capability".to_string(),
            message:
                "provider capability requirement exceeds the capability row admitted by the projection boundary"
                    .to_string(),
        });
    }

    if manifest.authority_policy.semantic_authority_owner != WORLD_SEMANTICS_OWNER
        || manifest.authority_policy.provider_may_grant_authority
        || manifest.authority_policy.provider_may_mutate_world_state
        || manifest.resource_limits.semantic_state_owner != WORLD_SEMANTICS_OWNER
        || !manifest.resource_limits.provider_handles_are_nonsemantic
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_over_capability".to_string(),
            message:
                "provider policies must preserve Mir/Mirrorea as the semantic owner of authority and state"
                    .to_string(),
        });
    }

    if !manifest
        .observation_policy
        .provider_receives_redacted_observation_only
        || manifest.observation_policy.retention_owner != WORLD_SEMANTICS_OWNER
        || manifest.redaction_policy.provider_may_emit_unredacted_debug
        || !manifest.redaction_policy.redaction_label_required
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_observation_policy_violation".to_string(),
            message: "provider observation/redaction policy must stay observer-safe and Mir-owned"
                .to_string(),
        });
    }

    if manifest.resource_limits.max_memory_mb == 0 || manifest.resource_limits.max_cpu_ms == 0 {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_resource_limits_missing".to_string(),
            message: "provider admission requires explicit non-zero resource limits".to_string(),
        });
    }

    if manifest.provider_kind == "wasm-sandbox" && manifest.wasm_execution_policy != "InventoryOnly"
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "wasm_execution_policy_mismatch".to_string(),
            message:
                "WASM provider admission currently requires `wasm_execution_policy = InventoryOnly`"
                    .to_string(),
        });
    }
    if matches!(
        manifest.provider_kind.as_str(),
        "wasm-sandbox" | "native-library-bridge"
    ) && !manifest.sandbox_policy.sandbox_required
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_sandbox_policy_violation".to_string(),
            message: "native and WASM provider rows must declare explicit sandbox requirement"
                .to_string(),
        });
    }

    if manifest.sandbox_policy.current_status.is_empty() {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_sandbox_policy_missing".to_string(),
            message: "provider admission requires an explicit sandbox policy status".to_string(),
        });
    }

    if manifest.retention_policy.is_empty() {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_retention_policy_missing".to_string(),
            message: "provider admission requires explicit retention policy wording".to_string(),
        });
    }

    match manifest.rollback_replay_cut_policy.as_deref() {
        Some(policy) if !policy.trim().is_empty() => {
            if policy == "Replayable"
                && matched_packet_schemas
                    .iter()
                    .chain(matched_ffi_schemas.iter())
                    .any(|schema| !schema.replay_compatible)
            {
                diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
                    code: "rollback_replay_cut_policy_incompatible".to_string(),
                    message:
                        "provider manifest claims replayability outside the matched boundary replay contract"
                            .to_string(),
                });
            }
        }
        _ => diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "missing_rollback_replay_cut_policy".to_string(),
            message:
                "provider admission requires an explicit rollback/replay/cut policy for completion claims"
                    .to_string(),
        }),
    }

    if manifest.provider_kind == "native-library-bridge"
        && manifest.native_execution_policy != "Disabled"
    {
        diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
            code: "native_execution_disabled_by_default".to_string(),
            message:
                "native provider execution remains disabled by default and cannot be admitted in Full System V1"
                    .to_string(),
        });
    }

    let terminal_outcome = if diagnostics.iter().any(|row| {
        matches!(
            row.code.as_str(),
            "provider_over_capability"
                | "missing_rollback_replay_cut_policy"
                | "provider_boundary_unmatched"
                | "provider_policy_not_admitted"
                | "provider_manifest_schema_version_mismatch"
                | "provider_schema_name_mismatch"
                | "provider_effect_row_outside_projection_boundary"
                | "provider_failure_row_outside_projection_boundary"
                | "provider_resource_limits_missing"
                | "provider_sandbox_policy_missing"
                | "provider_retention_policy_missing"
                | "provider_observation_policy_violation"
                | "rollback_replay_cut_policy_incompatible"
                | "local_split_rejected"
                | "native_execution_disabled_by_default"
        )
    }) {
        if diagnostics
            .iter()
            .any(|row| row.code == "native_execution_disabled_by_default")
        {
            "native_execution_disabled".to_string()
        } else {
            "rejected".to_string()
        }
    } else {
        provider_spec.accepted_terminal_outcome.to_string()
    };

    let execution_admitted = false;
    if terminal_outcome == "rejected" || terminal_outcome == "native_execution_disabled" {
        rejected_rows.push(format!(
            "{}:{}",
            manifest.provider_id,
            first_code(&diagnostics)
        ));
        return rejected_report(
            &projection.projection_id,
            &source_path_text,
            &request_path_text,
            &provider_manifest_path_text,
            &manifest.provider_id,
            &manifest.provider_kind,
            &manifest.target_id,
            &target_provider_policy,
            local_split_report,
            diagnostics,
            rejected_rows,
            residual_obligations,
            &terminal_outcome,
        );
    }

    match terminal_outcome.as_str() {
        "inventory_admitted" => residual_obligations.push(FullSystemV1Obligation {
            code: "provider_execution_runtime_deferred".to_string(),
            message:
                "provider contract admission is actualized, but executable provider runtime remains later work"
                    .to_string(),
        }),
        "wasm_inventory_only" => {
            diagnostics.push(FullSystemV1ProviderAdmissionDiagnostic {
                code: "wasm_inventory_only".to_string(),
                message:
                    "WASM provider rows remain inventory-only until a sandboxed admission package proves execution safety"
                        .to_string(),
            });
            residual_obligations.push(FullSystemV1Obligation {
                code: "sandboxed_wasm_execution_deferred".to_string(),
                message:
                    "WASM provider execution remains deferred until explicit sandbox admission exists"
                        .to_string(),
            });
        }
        _ => {}
    }

    FullSystemV1ProviderAdmissionReport {
        surface_kind: surface_kind(),
        admission_scope: admission_scope(),
        accepted: true,
        projection_id: projection.projection_id,
        source_path: source_path_text,
        request_path: request_path_text,
        provider_manifest_path: provider_manifest_path_text,
        provider_id: manifest.provider_id,
        provider_kind: manifest.provider_kind,
        target_id: manifest.target_id,
        target_provider_policy,
        input_schema: manifest.input_schema,
        output_schema: manifest.output_schema,
        effect_row: manifest.effect_row,
        failure_row: manifest.failure_row,
        required_capabilities: manifest.required_capabilities,
        matched_packet_schema_refs: sorted_vec(
            matched_packet_schemas
                .into_iter()
                .map(|schema| schema.schema_ref)
                .collect::<Vec<_>>(),
        ),
        matched_ffi_schema_refs: sorted_vec(
            matched_ffi_schemas
                .into_iter()
                .map(|schema| schema.schema_ref)
                .collect::<Vec<_>>(),
        ),
        terminal_outcome,
        execution_admitted,
        local_split_report,
        diagnostics,
        rejected_rows: Vec::new(),
        residual_obligations,
        final_public_api_frozen: false,
    }
}

fn load_provider_manifest(
    path: &Path,
) -> Result<ProviderManifest, FullSystemV1ProviderAdmissionDiagnostic> {
    let text =
        fs::read_to_string(path).map_err(|error| FullSystemV1ProviderAdmissionDiagnostic {
            code: "provider_manifest_io_error".to_string(),
            message: format!(
                "unable to read provider manifest `{}`: {error}",
                path.display()
            ),
        })?;
    serde_json::from_str(&text).map_err(|error| FullSystemV1ProviderAdmissionDiagnostic {
        code: "provider_manifest_parse_error".to_string(),
        message: format!(
            "invalid provider manifest JSON `{}`: {error}",
            path.display()
        ),
    })
}

fn schema_matches_manifest(schema: &ProjectionBoundarySchema, manifest: &ProviderManifest) -> bool {
    if schema.from_target != manifest.target_id && schema.to_target != manifest.target_id {
        return false;
    }
    let packet_match = manifest
        .packet_boundary
        .as_deref()
        .map(|value| value == schema.schema_ref || value == schema.boundary_ref)
        .unwrap_or(false);
    let ffi_match = manifest
        .ffi_boundary
        .as_deref()
        .map(|value| value == schema.schema_ref || value == schema.boundary_ref)
        .unwrap_or(false);
    packet_match || ffi_match
}

fn provider_kind_spec(provider_kind: &str) -> Option<ProviderKindSpec> {
    match provider_kind {
        "viewer-diagnostic-exporter" => Some(ProviderKindSpec {
            provider_kind: "viewer-diagnostic-exporter",
            expected_input_schema: "diagnostic_export_request",
            expected_output_schema: "diagnostic_export_receipt",
            admitted_target_policies: &["provider_inventory_only"],
            accepted_terminal_outcome: "inventory_admitted",
        }),
        "native-library-bridge" => Some(ProviderKindSpec {
            provider_kind: "native-library-bridge",
            expected_input_schema: "native_bridge_request",
            expected_output_schema: "native_bridge_receipt",
            admitted_target_policies: &["native_disabled"],
            accepted_terminal_outcome: "native_execution_disabled",
        }),
        "wasm-sandbox" => Some(ProviderKindSpec {
            provider_kind: "wasm-sandbox",
            expected_input_schema: "wasm_adapter_request",
            expected_output_schema: "wasm_adapter_receipt",
            admitted_target_policies: &["wasm_inventory_only"],
            accepted_terminal_outcome: "wasm_inventory_only",
        }),
        _ => None,
    }
}

fn rejected_report(
    projection_id: &str,
    source_path: &str,
    request_path: &str,
    provider_manifest_path: &str,
    provider_id: &str,
    provider_kind: &str,
    target_id: &str,
    target_provider_policy: &str,
    local_split_report: Option<FullSystemV1LocalSplitRuntimeReport>,
    diagnostics: Vec<FullSystemV1ProviderAdmissionDiagnostic>,
    rejected_rows: Vec<String>,
    residual_obligations: Vec<FullSystemV1Obligation>,
    terminal_outcome: &str,
) -> FullSystemV1ProviderAdmissionReport {
    FullSystemV1ProviderAdmissionReport {
        surface_kind: surface_kind(),
        admission_scope: admission_scope(),
        accepted: false,
        projection_id: projection_id.to_string(),
        source_path: source_path.to_string(),
        request_path: request_path.to_string(),
        provider_manifest_path: provider_manifest_path.to_string(),
        provider_id: provider_id.to_string(),
        provider_kind: provider_kind.to_string(),
        target_id: target_id.to_string(),
        target_provider_policy: target_provider_policy.to_string(),
        input_schema: String::new(),
        output_schema: String::new(),
        effect_row: Vec::new(),
        failure_row: Vec::new(),
        required_capabilities: Vec::new(),
        matched_packet_schema_refs: Vec::new(),
        matched_ffi_schema_refs: Vec::new(),
        terminal_outcome: terminal_outcome.to_string(),
        execution_admitted: false,
        local_split_report,
        diagnostics,
        rejected_rows: sorted_vec(rejected_rows),
        residual_obligations,
        final_public_api_frozen: false,
    }
}

fn first_code(diagnostics: &[FullSystemV1ProviderAdmissionDiagnostic]) -> String {
    diagnostics
        .first()
        .map(|row| row.code.clone())
        .unwrap_or_else(|| "provider_admission_rejected".to_string())
}

fn surface_kind() -> String {
    FULL_SYSTEM_V1_PROVIDER_ADMISSION_SURFACE_KIND.to_string()
}

fn admission_scope() -> String {
    FULL_SYSTEM_V1_PROVIDER_ADMISSION_SCOPE.to_string()
}

fn sorted_vec(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values
}
