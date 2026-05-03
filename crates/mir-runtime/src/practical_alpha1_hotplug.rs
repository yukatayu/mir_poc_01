use std::{
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::{
    practical_alpha1::{
        PracticalAlpha1AttachProfile, PracticalAlpha1Package, PracticalAlpha1PackageManifest,
        load_practical_alpha1_package_path,
    },
    practical_alpha1_hotplug_plan::{
        PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE, build_practical_alpha1_hotplug_plan,
    },
};
use mirrorea_core::PrincipalClaim;
use serde::Serialize;

use crate::alpha_layer_insertion_runtime::{
    LayerAttachRequest, LayerCompatibilityReport, LayerContract, LayerRuntimePreview,
    auth_layer_contract, auth_layer_signature,
    build_layer_insertion_report_over_default_attachpoint, debug_layer_contract,
    debug_layer_signature, incompatible_patch_contract, incompatible_patch_signature,
    rate_limit_layer_contract, rate_limit_layer_signature,
};
use crate::hotplug_runtime::HotPlugRuntimeSkeletonReport;

pub const PRACTICAL_ALPHA1_HOTPLUG_SCOPE: &str = "practical-alpha1-hotplug-floor";
pub const PRACTICAL_ALPHA1_HOTPLUG_SURFACE_KIND: &str = "practical_alpha1_nonfinal_hotplug_report";
pub const PRACTICAL_ALPHA1_HOTPLUG_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_HOTPLUG_RETAINED_LATER_REFS: &[&str] = &[
    "object_package_attach",
    "stale_membership_attach_reject",
    "missing_witness_attach_reject",
    "detach_minimal_contract",
    "docker_transport_execution",
    "local_save_load_execution",
    "final_public_hotplug_api",
];

const PRACTICAL_ALPHA1_HOTPLUG_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 attach command as object/runtime/avatar package completion",
    "do not treat the practical alpha-1 attach command as stale-membership or missing-witness completion",
    "do not treat the practical alpha-1 attach command as Docker transport, save/load, or final public hotplug ABI completion",
    "do not promote samples/practical-alpha1 to an active runnable root in the hotplug package",
];

const PRACTICAL_ALPHA1_HOTPLUG_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical layer-only hotplug first floor only",
    "limited HP-A1-01..05 practical sample families only",
    "no object package attach, stale-membership reject, missing-witness reject, detach-minimal contract, Docker/local TCP, save/load, or final public ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1HotPlugErrorKind {
    FrontDoor,
    HotPlugPlan,
    Runtime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1HotPlugError {
    pub kind: PracticalAlpha1HotPlugErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1HotPlugError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} at {}: {}",
            self.kind,
            self.path.display(),
            self.detail
        )
    }
}

impl std::error::Error for PracticalAlpha1HotPlugError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha1ManifestCheck {
    pub check_name: String,
    pub passed: bool,
    pub observed: String,
    pub reason_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha1HotPlugReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "hotplug_scope")]
    pub hotplug_scope: String,
    pub hotplug_plan_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub package_kind: String,
    pub attach_profile: String,
    pub manifest_checks: Vec<PracticalAlpha1ManifestCheck>,
    pub layer_compatibility: LayerCompatibilityReport,
    pub hotplug_runtime_report: HotPlugRuntimeSkeletonReport,
    pub active_layers_after: Vec<String>,
    #[serde(default)]
    pub runtime_preview: Option<LayerRuntimePreview>,
    #[serde(default)]
    pub activation_cut_ref: Option<String>,
    pub terminal_outcome: String,
    #[serde(default)]
    pub reason_family: Option<String>,
    #[serde(default)]
    pub rejection_reason_refs: Vec<String>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
    #[serde(default)]
    pub public_cli_frozen: bool,
    #[serde(default)]
    pub run_local_claimed: bool,
    #[serde(default)]
    pub run_docker_claimed: bool,
    #[serde(default)]
    pub save_load_claimed: bool,
    #[serde(default)]
    pub object_attach_claimed: bool,
    #[serde(default)]
    pub freshness_negative_complete: bool,
    #[serde(default)]
    pub stage_pa1_4_complete: bool,
}

struct LayerProfileBundle {
    attach_profile: &'static str,
    required_manifest_capability: &'static str,
    provided_manifest_capability: &'static str,
    manifest_effect_ref: &'static str,
    manifest_failure_row: &'static [&'static str],
    runtime_contract: LayerContract,
    runtime_preview: Option<LayerRuntimePreview>,
    layer_signature: mirrorea_core::LayerSignature,
}

pub fn attach_practical_alpha1_package_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1HotPlugReport, PracticalAlpha1HotPlugError> {
    let path = path.as_ref().to_path_buf();
    let package =
        load_practical_alpha1_package_path(&path).map_err(|error| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        })?;
    attach_practical_alpha1_package_at_path(&package, &path)
}

pub fn attach_practical_alpha1_package(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1HotPlugReport, PracticalAlpha1HotPlugError> {
    attach_practical_alpha1_package_at_path(package, Path::new("<inline>"))
}

fn attach_practical_alpha1_package_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha1HotPlugReport, PracticalAlpha1HotPlugError> {
    let plan = build_practical_alpha1_hotplug_plan(package).map_err(|error| {
        PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::HotPlugPlan,
            path: if error.path == Path::new("<inline>") {
                path.to_path_buf()
            } else {
                error.path
            },
            detail: error.detail,
        }
    })?;
    let manifest = package
        .manifest
        .as_ref()
        .ok_or_else(|| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::HotPlugPlan,
            path: path.to_path_buf(),
            detail: "practical hotplug plan requires `manifest`".to_string(),
        })?;
    let attach_profile = manifest
        .attach_profile
        .ok_or_else(|| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::HotPlugPlan,
            path: path.to_path_buf(),
            detail: "practical hotplug plan requires manifest.attach_profile".to_string(),
        })?;

    let profile = layer_profile_bundle(attach_profile);
    let manifest_checks = build_manifest_checks(manifest, &profile);
    let principal_claim = PrincipalClaim {
        principal: plan.requesting_principal.clone(),
        participant_place: plan.requesting_participant_place.clone(),
        claimed_authority: "AttachPointAdministrator".to_string(),
        claimed_capabilities: plan.capability_refs.clone(),
    };
    principal_claim
        .validate()
        .map_err(|error| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::Runtime,
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;

    let layer_report = build_layer_insertion_report_over_default_attachpoint(
        LayerAttachRequest {
            request_id: format!("layer_attach_request#{}", plan.sample_id),
            sample_id: plan.sample_id.clone(),
            attachpoint_ref: plan.attachpoint_ref.clone(),
            principal_claim,
            capability_refs: plan.capability_refs.clone(),
            membership_epoch: 0,
            member_incarnation: 0,
            witness_refs: plan.witness_refs.clone(),
            layer_signature: profile.layer_signature,
            requested_layer: profile.runtime_contract,
            contract_update_ref: plan.contract_update_ref.clone(),
            notes: vec![
                "practical alpha-1 layer-only hotplug first floor".to_string(),
                format!("attach_profile={}", profile.attach_profile),
            ],
        },
        None,
        profile.runtime_preview,
        None,
    )
    .map_err(|error| PracticalAlpha1HotPlugError {
        kind: PracticalAlpha1HotPlugErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;

    let reason_family = if layer_report.terminal_outcome.starts_with("accepted") {
        None
    } else if layer_report
        .hotplug_runtime_report
        .verdict
        .authorization_reason_refs
        .contains(&"attach_capability_missing".to_string())
    {
        Some("authorization".to_string())
    } else {
        Some("compatibility".to_string())
    };

    let rejection_reason_refs = if layer_report.terminal_outcome.starts_with("accepted") {
        Vec::new()
    } else if reason_family.as_deref() == Some("authorization") {
        layer_report
            .hotplug_runtime_report
            .verdict
            .authorization_reason_refs
            .clone()
    } else {
        layer_report
            .hotplug_runtime_report
            .verdict
            .compatibility_reason_refs
            .clone()
    };

    Ok(PracticalAlpha1HotPlugReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        hotplug_scope: hotplug_scope(),
        hotplug_plan_scope: PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE.to_string(),
        sample_id: plan.sample_id.clone(),
        package_id: plan.package_id.clone(),
        package_kind: plan.package_kind.clone(),
        attach_profile: plan.attach_profile.clone(),
        manifest_checks,
        layer_compatibility: layer_report.compatibility,
        hotplug_runtime_report: layer_report.hotplug_runtime_report,
        active_layers_after: layer_report.active_layers_after,
        runtime_preview: layer_report.runtime_preview,
        activation_cut_ref: plan.activation_cut_ref.clone(),
        terminal_outcome: layer_report.terminal_outcome,
        reason_family,
        rejection_reason_refs,
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        public_cli_frozen: false,
        run_local_claimed: false,
        run_docker_claimed: false,
        save_load_claimed: false,
        object_attach_claimed: false,
        freshness_negative_complete: false,
        stage_pa1_4_complete: false,
    })
}

fn build_manifest_checks(
    manifest: &PracticalAlpha1PackageManifest,
    profile: &LayerProfileBundle,
) -> Vec<PracticalAlpha1ManifestCheck> {
    vec![
        PracticalAlpha1ManifestCheck {
            check_name: "attach_profile_supported".to_string(),
            passed: true,
            observed: format!("attach_profile={}", profile.attach_profile),
            reason_refs: vec!["attach_profile_supported".to_string()],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "required_attach_capability_declared".to_string(),
            passed: manifest
                .requires_capabilities
                .contains(&profile.required_manifest_capability.to_string()),
            observed: format!(
                "requires_capabilities contains {}",
                profile.required_manifest_capability
            ),
            reason_refs: vec![if manifest
                .requires_capabilities
                .contains(&profile.required_manifest_capability.to_string())
            {
                "required_attach_capability_declared".to_string()
            } else {
                "required_attach_capability_missing".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "provided_capability_declared".to_string(),
            passed: manifest
                .provided_capabilities
                .contains(&profile.provided_manifest_capability.to_string()),
            observed: format!(
                "provided_capabilities contains {}",
                profile.provided_manifest_capability
            ),
            reason_refs: vec![if manifest
                .provided_capabilities
                .contains(&profile.provided_manifest_capability.to_string())
            {
                "provided_capability_declared".to_string()
            } else {
                "provided_capability_missing".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "effect_row_declared".to_string(),
            passed: manifest
                .effect_row
                .contains(&profile.manifest_effect_ref.to_string()),
            observed: format!("effect_row contains {}", profile.manifest_effect_ref),
            reason_refs: vec![if manifest
                .effect_row
                .contains(&profile.manifest_effect_ref.to_string())
            {
                "effect_row_declared".to_string()
            } else {
                "effect_row_missing".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "failure_row_declared".to_string(),
            passed: manifest.failure_row
                == profile
                    .manifest_failure_row
                    .iter()
                    .map(|value| (*value).to_string())
                    .collect::<Vec<_>>(),
            observed: format!("failure_row={}", manifest.failure_row.join(",")),
            reason_refs: vec![if manifest.failure_row
                == profile
                    .manifest_failure_row
                    .iter()
                    .map(|value| (*value).to_string())
                    .collect::<Vec<_>>()
            {
                "failure_row_declared".to_string()
            } else {
                "failure_row_mismatch".to_string()
            }],
        },
    ]
}

fn layer_profile_bundle(profile: PracticalAlpha1AttachProfile) -> LayerProfileBundle {
    match profile {
        PracticalAlpha1AttachProfile::DebugTraceLayer => LayerProfileBundle {
            attach_profile: "debug_trace_layer",
            required_manifest_capability: "admin.attach_layer",
            provided_manifest_capability: "observe.debug_trace",
            manifest_effect_ref: "emit_typed_debug_trace",
            manifest_failure_row: &[],
            runtime_contract: debug_layer_contract(),
            runtime_preview: None,
            layer_signature: debug_layer_signature(),
        },
        PracticalAlpha1AttachProfile::AuthGateLayer => LayerProfileBundle {
            attach_profile: "auth_gate_layer",
            required_manifest_capability: "admin.attach_layer",
            provided_manifest_capability: "observe.auth_audit",
            manifest_effect_ref: "emit_auth_audit",
            manifest_failure_row: &["AuthFailed"],
            runtime_contract: auth_layer_contract(),
            runtime_preview: Some(LayerRuntimePreview {
                preview_kind: "auth_contract_update_preview".to_string(),
                terminal_outcome: "accepted_contract_update".to_string(),
                reason_refs: vec![
                    "contract_update_activation_cut_present".to_string(),
                    "auth_failed_declared_on_new_contract_version".to_string(),
                ],
                notes: vec![
                    "transparent overlay is intentionally not claimed for auth".to_string(),
                    "runtime auth session semantics remain later".to_string(),
                ],
            }),
            layer_signature: auth_layer_signature(),
        },
        PracticalAlpha1AttachProfile::RateLimitLayer => LayerProfileBundle {
            attach_profile: "rate_limit_layer",
            required_manifest_capability: "admin.attach_layer",
            provided_manifest_capability: "observe.rate_limit_counter",
            manifest_effect_ref: "emit_rate_limit_counter",
            manifest_failure_row: &["RateLimited"],
            runtime_contract: rate_limit_layer_contract(),
            runtime_preview: Some(LayerRuntimePreview {
                preview_kind: "rate_limit_dispatch_preview".to_string(),
                terminal_outcome: "rejected".to_string(),
                reason_refs: vec![
                    "failure_row_within_declared_budget".to_string(),
                    "rate_limit_budget_exhausted".to_string(),
                    "RateLimited".to_string(),
                ],
                notes: vec![
                    "preview proves declared-failure discipline only".to_string(),
                    "real token-bucket runtime accounting remains later".to_string(),
                ],
            }),
            layer_signature: rate_limit_layer_signature(),
        },
        PracticalAlpha1AttachProfile::UnsafeDebugShadowLayer => LayerProfileBundle {
            attach_profile: "unsafe_debug_shadow_layer",
            required_manifest_capability: "admin.attach_layer",
            provided_manifest_capability: "observe.debug_trace.full",
            manifest_effect_ref: "emit_typed_debug_trace",
            manifest_failure_row: &[],
            runtime_contract: incompatible_patch_contract(),
            runtime_preview: None,
            layer_signature: incompatible_patch_signature(),
        },
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_HOTPLUG_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_HOTPLUG_SCOPE_KIND.to_string()
}

fn hotplug_scope() -> String {
    PRACTICAL_ALPHA1_HOTPLUG_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_HOTPLUG_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_HOTPLUG_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_HOTPLUG_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
