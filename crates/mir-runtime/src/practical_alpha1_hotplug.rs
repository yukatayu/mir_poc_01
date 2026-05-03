use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::{
    practical_alpha1::{
        PracticalAlpha1AttachProfile, PracticalAlpha1Package, PracticalAlpha1PackageManifest,
        PracticalAlpha1RuntimeMembershipAdvance, PracticalAlpha1RuntimeMembershipAdvanceKind,
        load_practical_alpha1_package_path,
    },
    practical_alpha1_hotplug_plan::{
        PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE, PracticalAlpha1HotPlugPlan,
        build_practical_alpha1_hotplug_plan,
    },
};
use mirrorea_core::{
    HotPlugRequest, HotPlugVerdict, LogicalPlaceRuntimeShell, MirroreaCoreError, PrincipalClaim,
};
use serde::Serialize;

use crate::{
    alpha_avatar_runtime::PackageAdmissionCheck,
    alpha_layer_insertion_runtime::{
        LayerAttachRequest, LayerCompatibilityReport, LayerContract, LayerRuntimePreview,
        auth_layer_contract, auth_layer_signature, build_default_layer_attach_shell,
        build_layer_insertion_report_over_default_attachpoint, debug_layer_contract,
        debug_layer_signature, incompatible_patch_contract, incompatible_patch_signature,
        rate_limit_layer_contract, rate_limit_layer_signature,
    },
    hotplug_runtime::{HotPlugRuntimeSkeletonReport, assemble_hotplug_runtime_skeleton_report},
};

pub const PRACTICAL_ALPHA1_HOTPLUG_SCOPE: &str = "practical-alpha1-hotplug-floor";
pub const PRACTICAL_ALPHA1_HOTPLUG_SURFACE_KIND: &str = "practical_alpha1_nonfinal_hotplug_report";
pub const PRACTICAL_ALPHA1_HOTPLUG_SCOPE_KIND: &str = "alpha_local";

const OBJECT_ATTACHPOINT: &str = "AttachPoint[AlphaRoom#1::AvatarRuntime]";
const OBJECT_ATTACHPOINT_CAPABILITY: &str = "ManageAttachPoint(AlphaRoom#1::AvatarRuntime)";
const OBJECT_INSTALL_CAPABILITY: &str = "InstallRuntimePackage(AlphaRoom#1)";
const OBJECT_REQUIRED_ATTACH_CAPABILITY: &str = "admin.attach_object_package";
const OBJECT_RENDER_CAPABILITY: &str = "render.placeholder_avatar";
const OBJECT_FALLBACK_REPRESENTATION: &str = "static_capsule_placeholder";

const PRACTICAL_ALPHA1_HOTPLUG_RETAINED_LATER_REFS: &[&str] = &[
    "detach_minimal_contract",
    "docker_transport_execution",
    "local_save_load_execution",
    "final_public_hotplug_api",
];

const PRACTICAL_ALPHA1_HOTPLUG_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 attach command as final avatar/runtime/package completion",
    "do not treat the practical alpha-1 attach command as Docker transport, save/load, or final public hotplug ABI completion",
    "do not promote samples/practical-alpha1 to an active runnable root in the hotplug package",
];

const PRACTICAL_ALPHA1_HOTPLUG_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical hotplug floor only",
    "limited HP-A1 practical sample families only",
    "no detach-minimal contract, Docker/local TCP, save/load, or final public ABI",
];

const OBJECT_HOST_CAPABILITIES: &[&str] = &[
    "AttachAvatarRuntime",
    "ManageRuntimePackageRegistry",
    "RenderPlaceholderMesh",
];

const ABSTRACT_AVATAR_ROLES: &[&str] = &["EmbodiedPresence", "Renderable"];

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
pub struct PracticalAlpha1ObjectAttachPreview {
    pub preview_kind: String,
    pub selected_representation: String,
    pub provided_roles: Vec<String>,
    pub required_host_capabilities: Vec<String>,
    pub available_host_capabilities: Vec<String>,
    pub signature_present: bool,
    pub fallback_representation: String,
    pub native_execution_performed: bool,
    pub notes: Vec<String>,
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
    pub offered_membership_epoch: u64,
    pub offered_member_incarnation: u64,
    pub required_witness_refs: Vec<String>,
    pub manifest_checks: Vec<PracticalAlpha1ManifestCheck>,
    #[serde(default)]
    pub package_admission_checks: Vec<PackageAdmissionCheck>,
    #[serde(default)]
    pub layer_compatibility: Option<LayerCompatibilityReport>,
    pub hotplug_runtime_report: HotPlugRuntimeSkeletonReport,
    #[serde(default)]
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
    #[serde(default)]
    pub object_attach_preview: Option<PracticalAlpha1ObjectAttachPreview>,
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

struct HotPlugGateEvaluation {
    compatibility_reason_refs: Vec<String>,
    authorization_reason_refs: Vec<String>,
    membership_freshness_reason_refs: Vec<String>,
    witness_reason_refs: Vec<String>,
    accepted: bool,
    reason_family: Option<String>,
    rejection_reason_refs: Vec<String>,
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

    match attach_profile {
        PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage => {
            attach_object_package_preview(package, manifest, &plan, path)
        }
        _ => attach_layer_package(package, manifest, attach_profile, &plan, path),
    }
}

fn attach_layer_package(
    _package: &PracticalAlpha1Package,
    manifest: &PracticalAlpha1PackageManifest,
    attach_profile: PracticalAlpha1AttachProfile,
    plan: &PracticalAlpha1HotPlugPlan,
    path: &Path,
) -> Result<PracticalAlpha1HotPlugReport, PracticalAlpha1HotPlugError> {
    let profile = layer_profile_bundle(attach_profile);
    let manifest_checks = build_layer_manifest_checks(manifest, &profile);
    let principal_claim = build_principal_claim(plan, "AttachPointAdministrator")?;
    let request = LayerAttachRequest {
        request_id: format!("layer_attach_request#{}", plan.sample_id),
        sample_id: plan.sample_id.clone(),
        attachpoint_ref: plan.attachpoint_ref.clone(),
        principal_claim: principal_claim.clone(),
        capability_refs: plan.capability_refs.clone(),
        membership_epoch: plan.membership_epoch,
        member_incarnation: plan.member_incarnation,
        witness_refs: plan.witness_refs.clone(),
        layer_signature: profile.layer_signature,
        requested_layer: profile.runtime_contract.clone(),
        contract_update_ref: plan.contract_update_ref.clone(),
        notes: vec![
            "practical alpha-1 hotplug floor".to_string(),
            format!("attach_profile={}", profile.attach_profile),
        ],
    };
    let base_report = build_layer_insertion_report_over_default_attachpoint(
        request.clone(),
        None,
        profile.runtime_preview.clone(),
        None,
    )
    .map_err(|error| PracticalAlpha1HotPlugError {
        kind: PracticalAlpha1HotPlugErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;

    let mut shell =
        build_default_layer_attach_shell().map_err(|error| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::Runtime,
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;
    apply_pre_attach_membership_advances(&mut shell, &plan.pre_attach_membership_advances)
        .map_err(|error| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::Runtime,
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;
    let gate = evaluate_hotplug_gates(
        &shell,
        plan,
        &principal_claim,
        &manifest_checks,
        compatibility_reason_refs(&base_report.compatibility),
        base_report.compatibility.failed_reason_refs.is_empty(),
        required_attach_capability_refs(&[profile.runtime_contract.attach_capability.as_str()]),
    );
    let hotplug_runtime_report = build_hotplug_runtime_report(
        &shell,
        plan,
        &principal_claim,
        &gate,
        format!("LayerPatch[{}]", profile.runtime_contract.layer_name),
        "current verdict is computed from manifest, freshness, witness, authorization, and layer compatibility checks",
    )
    .map_err(|error| PracticalAlpha1HotPlugError {
        kind: PracticalAlpha1HotPlugErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;

    let terminal_outcome = if gate.accepted {
        base_report.terminal_outcome
    } else {
        "rejected".to_string()
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
        offered_membership_epoch: plan.membership_epoch,
        offered_member_incarnation: plan.member_incarnation,
        required_witness_refs: plan.required_witness_refs.clone(),
        manifest_checks,
        package_admission_checks: Vec::new(),
        layer_compatibility: Some(base_report.compatibility),
        hotplug_runtime_report,
        active_layers_after: if gate.accepted {
            base_report.active_layers_after
        } else {
            Vec::new()
        },
        runtime_preview: base_report.runtime_preview,
        activation_cut_ref: plan.activation_cut_ref.clone(),
        terminal_outcome,
        reason_family: gate.reason_family,
        rejection_reason_refs: gate.rejection_reason_refs,
        object_attach_preview: None,
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

fn attach_object_package_preview(
    package: &PracticalAlpha1Package,
    manifest: &PracticalAlpha1PackageManifest,
    plan: &PracticalAlpha1HotPlugPlan,
    path: &Path,
) -> Result<PracticalAlpha1HotPlugReport, PracticalAlpha1HotPlugError> {
    let manifest_checks = build_object_manifest_checks(package, manifest);
    let package_admission_checks = build_object_package_admission_checks(package, manifest);
    let principal_claim = build_principal_claim(plan, "AttachPointAdministrator")?;
    let mut shell = build_object_attach_shell().map_err(|error| PracticalAlpha1HotPlugError {
        kind: PracticalAlpha1HotPlugErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;
    apply_pre_attach_membership_advances(&mut shell, &plan.pre_attach_membership_advances)
        .map_err(|error| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::Runtime,
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;

    let compatibility_reason_refs = package_admission_compatibility_refs(&package_admission_checks);
    let gate = evaluate_hotplug_gates(
        &shell,
        plan,
        &principal_claim,
        &manifest_checks,
        compatibility_reason_refs,
        package_admission_checks.iter().all(|check| check.passed),
        required_attach_capability_refs(&[
            OBJECT_ATTACHPOINT_CAPABILITY,
            OBJECT_INSTALL_CAPABILITY,
        ]),
    );
    let hotplug_runtime_report = build_hotplug_runtime_report(
        &shell,
        plan,
        &principal_claim,
        &gate,
        plan.package_id.clone(),
        "current verdict is computed from manifest, package-admission preview checks, freshness, witness, and authorization",
    )
    .map_err(|error| PracticalAlpha1HotPlugError {
        kind: PracticalAlpha1HotPlugErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;

    let object_attach_preview = if gate.accepted {
        Some(build_object_attach_preview(package, manifest))
    } else {
        None
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
        offered_membership_epoch: plan.membership_epoch,
        offered_member_incarnation: plan.member_incarnation,
        required_witness_refs: plan.required_witness_refs.clone(),
        manifest_checks,
        package_admission_checks,
        layer_compatibility: None,
        hotplug_runtime_report,
        active_layers_after: Vec::new(),
        runtime_preview: None,
        activation_cut_ref: plan.activation_cut_ref.clone(),
        terminal_outcome: if gate.accepted {
            "accepted_object_attach_preview".to_string()
        } else {
            "rejected".to_string()
        },
        reason_family: gate.reason_family,
        rejection_reason_refs: gate.rejection_reason_refs,
        object_attach_preview,
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

fn build_principal_claim(
    plan: &PracticalAlpha1HotPlugPlan,
    claimed_authority: &str,
) -> Result<PrincipalClaim, PracticalAlpha1HotPlugError> {
    let principal_claim = PrincipalClaim {
        principal: plan.requesting_principal.clone(),
        participant_place: plan.requesting_participant_place.clone(),
        claimed_authority: claimed_authority.to_string(),
        claimed_capabilities: plan.capability_refs.clone(),
    };
    principal_claim
        .validate()
        .map_err(|error| PracticalAlpha1HotPlugError {
            kind: PracticalAlpha1HotPlugErrorKind::Runtime,
            path: PathBuf::from("<inline>"),
            detail: error.to_string(),
        })?;
    Ok(principal_claim)
}

fn apply_pre_attach_membership_advances(
    shell: &mut LogicalPlaceRuntimeShell,
    advances: &[PracticalAlpha1RuntimeMembershipAdvance],
) -> Result<(), MirroreaCoreError> {
    for advance in advances {
        match advance.kind {
            PracticalAlpha1RuntimeMembershipAdvanceKind::AddParticipant => {
                shell.add_participant(&advance.principal)?;
            }
        }
    }
    Ok(())
}

fn build_hotplug_runtime_report(
    shell: &LogicalPlaceRuntimeShell,
    plan: &PracticalAlpha1HotPlugPlan,
    principal_claim: &PrincipalClaim,
    gate: &HotPlugGateEvaluation,
    patch_ref: String,
    note: &str,
) -> Result<HotPlugRuntimeSkeletonReport, MirroreaCoreError> {
    let request = HotPlugRequest {
        request_id: format!(
            "practical_hotplug_request#{}",
            plan.sample_id.to_lowercase()
        ),
        attachpoint_ref: plan.attachpoint_ref.clone(),
        patch_ref,
        operation_kind: "attach".to_string(),
        requesting_principal: principal_claim.principal.clone(),
        requesting_participant_place: principal_claim.participant_place.clone(),
        message_envelope_ref: format!(
            "practical_hotplug_envelope#{}",
            plan.sample_id.to_lowercase()
        ),
        auth_evidence_ref: plan.contract_update_ref.clone(),
        capability_refs: plan.capability_refs.clone(),
        witness_refs: plan.witness_refs.clone(),
        notes: vec![format!("sample_id={}", plan.sample_id), note.to_string()],
    };
    let verdict = HotPlugVerdict {
        request_ref: request.request_id.clone(),
        verdict_kind: if gate.accepted {
            "accepted".to_string()
        } else {
            "rejected".to_string()
        },
        compatibility_reason_refs: gate.compatibility_reason_refs.clone(),
        authorization_reason_refs: gate.authorization_reason_refs.clone(),
        membership_freshness_reason_refs: gate.membership_freshness_reason_refs.clone(),
        witness_reason_refs: gate.witness_reason_refs.clone(),
        notes: vec![
            "practical hotplug verdict stays non-final and alpha-local".to_string(),
            "detach/minimal lifecycle contract remains later".to_string(),
        ],
    };
    assemble_hotplug_runtime_skeleton_report(shell, request, verdict)
}

fn evaluate_hotplug_gates(
    shell: &LogicalPlaceRuntimeShell,
    plan: &PracticalAlpha1HotPlugPlan,
    principal_claim: &PrincipalClaim,
    manifest_checks: &[PracticalAlpha1ManifestCheck],
    compatibility_reason_refs: Vec<String>,
    compatibility_accepted: bool,
    required_capability_refs: Vec<String>,
) -> HotPlugGateEvaluation {
    let snapshot = shell.snapshot();
    let manifest_rejection_refs = failed_manifest_reason_refs(manifest_checks);
    let (membership_refs, membership_accepted) =
        membership_reason_refs(&snapshot, principal_claim, plan);
    let (witness_refs, witness_accepted) =
        witness_reason_refs(&plan.witness_refs, &plan.required_witness_refs);
    let (authorization_refs, authorization_accepted) =
        authorization_reason_refs(&plan.capability_refs, &required_capability_refs);

    let accepted = manifest_rejection_refs.is_empty()
        && membership_accepted
        && witness_accepted
        && authorization_accepted
        && compatibility_accepted;

    let (reason_family, rejection_reason_refs) = if accepted {
        (None, Vec::new())
    } else if !manifest_rejection_refs.is_empty() {
        (Some("manifest".to_string()), manifest_rejection_refs)
    } else if !witness_accepted {
        (
            Some("witness".to_string()),
            reject_only(&witness_refs, "required_witnesses_present"),
        )
    } else if !membership_accepted {
        (
            Some("membership_freshness".to_string()),
            reject_only(&membership_refs, "membership_frontier_current"),
        )
    } else if !authorization_accepted {
        (
            Some("authorization".to_string()),
            reject_only(&authorization_refs, "attach_capability_present"),
        )
    } else {
        (
            Some("compatibility".to_string()),
            compatibility_reason_refs.clone(),
        )
    };

    HotPlugGateEvaluation {
        compatibility_reason_refs,
        authorization_reason_refs: authorization_refs,
        membership_freshness_reason_refs: membership_refs,
        witness_reason_refs: witness_refs,
        accepted,
        reason_family,
        rejection_reason_refs,
    }
}

fn membership_reason_refs(
    snapshot: &mirrorea_core::LogicalPlaceRuntimeSnapshot,
    principal_claim: &PrincipalClaim,
    plan: &PracticalAlpha1HotPlugPlan,
) -> (Vec<String>, bool) {
    let Some(member) = snapshot.membership.members.get(&principal_claim.principal) else {
        return (vec!["missing_requesting_membership".to_string()], false);
    };
    if !member.active {
        return (vec!["inactive_requesting_membership".to_string()], false);
    }
    if member.place != principal_claim.participant_place {
        return (vec!["participant_place_drift".to_string()], false);
    }
    if snapshot.membership.membership_epoch != plan.membership_epoch {
        return (vec!["membership_epoch_drift".to_string()], false);
    }
    if member.incarnation != plan.member_incarnation {
        return (vec!["member_incarnation_drift".to_string()], false);
    }
    (
        vec![
            "membership_frontier_current".to_string(),
            "member_incarnation_current".to_string(),
        ],
        true,
    )
}

fn witness_reason_refs(
    actual_witness_refs: &[String],
    required_witness_refs: &[String],
) -> (Vec<String>, bool) {
    let actual: BTreeSet<_> = actual_witness_refs.iter().cloned().collect();
    let required: BTreeSet<_> = required_witness_refs.iter().cloned().collect();
    let missing: Vec<String> = required.difference(&actual).cloned().collect();
    if missing.is_empty() {
        (vec!["required_witnesses_present".to_string()], true)
    } else {
        (
            missing
                .into_iter()
                .map(|witness| format!("missing_witness:{witness}"))
                .collect(),
            false,
        )
    }
}

fn authorization_reason_refs(
    offered_capability_refs: &[String],
    required_capability_refs: &[String],
) -> (Vec<String>, bool) {
    let offered: BTreeSet<_> = offered_capability_refs.iter().cloned().collect();
    let required: BTreeSet<_> = required_capability_refs.iter().cloned().collect();
    let missing: Vec<String> = required.difference(&offered).cloned().collect();
    if missing.is_empty() {
        (
            vec![
                "attach_capability_present".to_string(),
                "admin_attach_authority_confirmed".to_string(),
            ],
            true,
        )
    } else {
        (
            missing
                .into_iter()
                .map(|capability| format!("attach_capability_missing:{capability}"))
                .collect(),
            false,
        )
    }
}

fn failed_manifest_reason_refs(checks: &[PracticalAlpha1ManifestCheck]) -> Vec<String> {
    checks
        .iter()
        .filter(|check| !check.passed)
        .flat_map(|check| check.reason_refs.clone())
        .collect()
}

fn compatibility_reason_refs(report: &LayerCompatibilityReport) -> Vec<String> {
    if report.failed_reason_refs.is_empty() {
        report.passed_reason_refs.iter().take(3).cloned().collect()
    } else {
        report.failed_reason_refs.clone()
    }
}

fn package_admission_compatibility_refs(checks: &[PackageAdmissionCheck]) -> Vec<String> {
    let failed: Vec<String> = checks
        .iter()
        .filter(|check| !check.passed)
        .flat_map(|check| check.reason_refs.clone())
        .collect();
    if failed.is_empty() {
        vec![
            "manifest_checked".to_string(),
            "object_preview_route_selected".to_string(),
            "fallback_visibility_preserved".to_string(),
        ]
    } else {
        failed
    }
}

fn reject_only(reason_refs: &[String], success_reason: &str) -> Vec<String> {
    reason_refs
        .iter()
        .filter(|reason| reason.as_str() != success_reason)
        .cloned()
        .collect()
}

fn build_layer_manifest_checks(
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

fn build_object_manifest_checks(
    package: &PracticalAlpha1Package,
    manifest: &PracticalAlpha1PackageManifest,
) -> Vec<PracticalAlpha1ManifestCheck> {
    vec![
        PracticalAlpha1ManifestCheck {
            check_name: "attach_profile_supported".to_string(),
            passed: manifest.attach_profile
                == Some(PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage),
            observed: format!(
                "attach_profile={}",
                attach_profile_name(manifest.attach_profile)
            ),
            reason_refs: vec![if manifest.attach_profile
                == Some(PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage)
            {
                "attach_profile_supported".to_string()
            } else {
                "attach_profile_unsupported".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "required_attach_capability_declared".to_string(),
            passed: manifest
                .requires_capabilities
                .contains(&OBJECT_REQUIRED_ATTACH_CAPABILITY.to_string()),
            observed: format!(
                "requires_capabilities contains {}",
                OBJECT_REQUIRED_ATTACH_CAPABILITY
            ),
            reason_refs: vec![if manifest
                .requires_capabilities
                .contains(&OBJECT_REQUIRED_ATTACH_CAPABILITY.to_string())
            {
                "required_attach_capability_declared".to_string()
            } else {
                "required_attach_capability_missing".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "placeholder_render_capability_declared".to_string(),
            passed: manifest
                .provided_capabilities
                .contains(&OBJECT_RENDER_CAPABILITY.to_string()),
            observed: format!(
                "provided_capabilities contains {}",
                OBJECT_RENDER_CAPABILITY
            ),
            reason_refs: vec![if manifest
                .provided_capabilities
                .contains(&OBJECT_RENDER_CAPABILITY.to_string())
            {
                "placeholder_render_capability_declared".to_string()
            } else {
                "placeholder_render_capability_missing".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "signature_present".to_string(),
            passed: package
                .native
                .as_ref()
                .map(|native| native.signature_present)
                .unwrap_or(false),
            observed: format!(
                "signature_present={}",
                package
                    .native
                    .as_ref()
                    .map(|native| native.signature_present)
                    .unwrap_or(false)
            ),
            reason_refs: vec![if package
                .native
                .as_ref()
                .map(|native| native.signature_present)
                .unwrap_or(false)
            {
                "signature_present".to_string()
            } else {
                "signature_missing".to_string()
            }],
        },
        PracticalAlpha1ManifestCheck {
            check_name: "fallback_representation_explicit".to_string(),
            passed: manifest.fallback_representation.as_deref()
                == Some(OBJECT_FALLBACK_REPRESENTATION),
            observed: format!(
                "fallback_representation={}",
                manifest
                    .fallback_representation
                    .as_deref()
                    .unwrap_or("<missing>")
            ),
            reason_refs: vec![if manifest.fallback_representation.as_deref()
                == Some(OBJECT_FALLBACK_REPRESENTATION)
            {
                "fallback_representation_explicit".to_string()
            } else {
                "fallback_representation_missing".to_string()
            }],
        },
    ]
}

fn build_object_package_admission_checks(
    package: &PracticalAlpha1Package,
    manifest: &PracticalAlpha1PackageManifest,
) -> Vec<PackageAdmissionCheck> {
    let available_host_capabilities: Vec<String> = OBJECT_HOST_CAPABILITIES
        .iter()
        .map(|value| (*value).to_string())
        .collect();
    let required_host_capabilities = manifest.required_host_capabilities.clone();
    let missing_host_capabilities: Vec<String> = required_host_capabilities
        .iter()
        .filter(|capability| !available_host_capabilities.contains(capability))
        .cloned()
        .collect();
    let provided_roles = manifest.provided_roles.clone();
    let unsupported_roles: Vec<String> = provided_roles
        .iter()
        .filter(|role| !ABSTRACT_AVATAR_ROLES.contains(&role.as_str()))
        .cloned()
        .collect();

    vec![
        PackageAdmissionCheck {
            check_name: "manifest_well_formed".to_string(),
            passed: package.native.is_some()
                && !manifest.provided_roles.is_empty()
                && !manifest.required_host_capabilities.is_empty(),
            observed: "native/provided_roles/required_host_capabilities must be present"
                .to_string(),
            reason_refs: if package.native.is_some()
                && !manifest.provided_roles.is_empty()
                && !manifest.required_host_capabilities.is_empty()
            {
                Vec::new()
            } else {
                vec!["object_manifest_incomplete".to_string()]
            },
        },
        PackageAdmissionCheck {
            check_name: "provenance_policy_satisfied".to_string(),
            passed: package
                .native
                .as_ref()
                .map(|native| native.signature_present)
                .unwrap_or(false),
            observed: format!(
                "signature_present={}",
                package
                    .native
                    .as_ref()
                    .map(|native| native.signature_present)
                    .unwrap_or(false)
            ),
            reason_refs: if package
                .native
                .as_ref()
                .map(|native| native.signature_present)
                .unwrap_or(false)
            {
                Vec::new()
            } else {
                vec!["native_policy_requires_trusted_signer".to_string()]
            },
        },
        PackageAdmissionCheck {
            check_name: "role_compatibility".to_string(),
            passed: unsupported_roles.is_empty(),
            observed: format!("provided_roles={provided_roles:?}"),
            reason_refs: unsupported_roles
                .into_iter()
                .map(|role| format!("unsupported_role:{role}"))
                .collect(),
        },
        PackageAdmissionCheck {
            check_name: "host_support_satisfied".to_string(),
            passed: missing_host_capabilities.is_empty(),
            observed: format!(
                "available={available_host_capabilities:?} required={required_host_capabilities:?}"
            ),
            reason_refs: missing_host_capabilities
                .into_iter()
                .map(|capability| format!("missing_host_capability:{capability}"))
                .collect(),
        },
        PackageAdmissionCheck {
            check_name: "fallback_representation_explicit".to_string(),
            passed: manifest.fallback_representation.as_deref()
                == Some(OBJECT_FALLBACK_REPRESENTATION),
            observed: format!(
                "fallback_representation={}",
                manifest
                    .fallback_representation
                    .as_deref()
                    .unwrap_or("<missing>")
            ),
            reason_refs: if manifest.fallback_representation.as_deref()
                == Some(OBJECT_FALLBACK_REPRESENTATION)
            {
                Vec::new()
            } else {
                vec!["fallback_representation_missing".to_string()]
            },
        },
    ]
}

fn build_object_attach_preview(
    package: &PracticalAlpha1Package,
    manifest: &PracticalAlpha1PackageManifest,
) -> PracticalAlpha1ObjectAttachPreview {
    PracticalAlpha1ObjectAttachPreview {
        preview_kind: "runtime_package_avatar_preview".to_string(),
        selected_representation: OBJECT_FALLBACK_REPRESENTATION.to_string(),
        provided_roles: manifest.provided_roles.clone(),
        required_host_capabilities: manifest.required_host_capabilities.clone(),
        available_host_capabilities: OBJECT_HOST_CAPABILITIES
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        signature_present: package
            .native
            .as_ref()
            .map(|native| native.signature_present)
            .unwrap_or(false),
        fallback_representation: manifest
            .fallback_representation
            .clone()
            .unwrap_or_else(|| OBJECT_FALLBACK_REPRESENTATION.to_string()),
        native_execution_performed: false,
        notes: vec![
            "object package attach remains a non-final preview over runtime package admission"
                .to_string(),
            "placeholder representation is explicit and visible".to_string(),
            "no final avatar ABI or native execution is claimed".to_string(),
        ],
    }
}

fn build_object_attach_shell() -> Result<LogicalPlaceRuntimeShell, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place(OBJECT_ATTACHPOINT, "AttachPoint")?;
    shell.add_initial_participant("ExampleAdmin")?;
    Ok(shell)
}

fn required_attach_capability_refs(required: &[&str]) -> Vec<String> {
    required.iter().map(|value| (*value).to_string()).collect()
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
        PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage => {
            panic!("object package attach profile is not a layer profile")
        }
    }
}

fn attach_profile_name(profile: Option<PracticalAlpha1AttachProfile>) -> &'static str {
    match profile {
        Some(PracticalAlpha1AttachProfile::DebugTraceLayer) => "debug_trace_layer",
        Some(PracticalAlpha1AttachProfile::AuthGateLayer) => "auth_gate_layer",
        Some(PracticalAlpha1AttachProfile::RateLimitLayer) => "rate_limit_layer",
        Some(PracticalAlpha1AttachProfile::UnsafeDebugShadowLayer) => "unsafe_debug_shadow_layer",
        Some(PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage) => {
            "placeholder_avatar_object_package"
        }
        None => "<missing>",
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
