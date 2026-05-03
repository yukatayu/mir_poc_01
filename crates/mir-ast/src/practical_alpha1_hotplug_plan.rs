use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::practical_alpha1::{
    PracticalAlpha1AttachProfile, PracticalAlpha1Package, PracticalAlpha1RuntimeMembershipAdvance,
    load_practical_alpha1_package_path,
};

pub const PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE: &str = "practical-alpha1-hotplug-plan-floor";
pub const PRACTICAL_ALPHA1_HOTPLUG_PLAN_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_hotplug_plan";
pub const PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_HOTPLUG_PLAN_RETAINED_LATER_REFS: &[&str] = &[
    "object_package_attach",
    "stale_membership_attach_reject",
    "missing_witness_attach_reject",
    "detach_minimal_contract",
    "docker_transport_execution",
    "local_save_load_execution",
    "final_public_hotplug_api",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1HotPlugPlanErrorKind {
    FrontDoor,
    MissingManifest,
    MissingHotPlugSection,
    MalformedHotPlugInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1HotPlugPlanError {
    pub kind: PracticalAlpha1HotPlugPlanErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1HotPlugPlanError {
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

impl std::error::Error for PracticalAlpha1HotPlugPlanError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1HotPlugPlan {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "hotplug_plan_scope")]
    pub hotplug_plan_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub package_kind: String,
    pub manifest_version: String,
    pub attach_profile: String,
    pub attachpoint_ref: String,
    pub requesting_principal: String,
    pub requesting_participant_place: String,
    pub capability_refs: Vec<String>,
    pub membership_epoch: u64,
    pub member_incarnation: u64,
    pub witness_refs: Vec<String>,
    pub required_witness_refs: Vec<String>,
    #[serde(default)]
    pub pre_attach_membership_advances: Vec<PracticalAlpha1RuntimeMembershipAdvance>,
    #[serde(default)]
    pub activation_cut_ref: Option<String>,
    #[serde(default)]
    pub contract_update_ref: Option<String>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
}

pub fn build_practical_alpha1_hotplug_plan_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1HotPlugPlan, PracticalAlpha1HotPlugPlanError> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha1HotPlugPlanError {
            kind: PracticalAlpha1HotPlugPlanErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    build_practical_alpha1_hotplug_plan_at_path(&package, &path)
}

pub fn build_practical_alpha1_hotplug_plan(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1HotPlugPlan, PracticalAlpha1HotPlugPlanError> {
    build_practical_alpha1_hotplug_plan_at_path(package, Path::new("<inline>"))
}

fn build_practical_alpha1_hotplug_plan_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha1HotPlugPlan, PracticalAlpha1HotPlugPlanError> {
    if !matches!(package.package_kind.as_str(), "layer" | "object") {
        return Err(PracticalAlpha1HotPlugPlanError {
            kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
            path: path.to_path_buf(),
            detail: format!(
                "current practical hotplug plan floor only admits layer or object packages, found `{}`",
                package.package_kind
            ),
        });
    }

    let manifest = package
        .manifest
        .as_ref()
        .ok_or_else(|| PracticalAlpha1HotPlugPlanError {
            kind: PracticalAlpha1HotPlugPlanErrorKind::MissingManifest,
            path: path.to_path_buf(),
            detail: "practical hotplug plan requires `manifest`".to_string(),
        })?;
    let hotplug = package.alpha_local_hotplug_input.as_ref().ok_or_else(|| {
        PracticalAlpha1HotPlugPlanError {
            kind: PracticalAlpha1HotPlugPlanErrorKind::MissingHotPlugSection,
            path: path.to_path_buf(),
            detail: "package does not declare `alpha_local_hotplug_input`".to_string(),
        }
    })?;
    let attach_profile =
        manifest
            .attach_profile
            .ok_or_else(|| PracticalAlpha1HotPlugPlanError {
                kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
                path: path.to_path_buf(),
                detail: "practical hotplug plan requires manifest.attach_profile".to_string(),
            })?;

    match (package.package_kind.as_str(), attach_profile) {
        ("layer", PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage) => {
            return Err(PracticalAlpha1HotPlugPlanError {
                kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
                path: path.to_path_buf(),
                detail: "object attach profile is not admitted for layer packages".to_string(),
            });
        }
        ("object", PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage) => {}
        ("object", _) => {
            return Err(PracticalAlpha1HotPlugPlanError {
                kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
                path: path.to_path_buf(),
                detail: "current practical object attach preview requires placeholder_avatar_object_package profile".to_string(),
            });
        }
        _ => {}
    }

    if hotplug.sample_id.is_empty()
        || hotplug.attachpoint_ref.is_empty()
        || hotplug.requesting_principal.is_empty()
        || hotplug.requesting_participant_place.is_empty()
        || hotplug.capability_refs.is_empty()
        || hotplug.witness_refs.is_empty()
        || hotplug.required_witness_refs.is_empty()
    {
        return Err(PracticalAlpha1HotPlugPlanError {
            kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
            path: path.to_path_buf(),
            detail:
                "practical hotplug plan requires non-empty request/sample/capability/witness fields"
                    .to_string(),
        });
    }

    if attach_profile == PracticalAlpha1AttachProfile::AuthGateLayer {
        let contract_update_ref = hotplug.contract_update_ref.as_ref().ok_or_else(|| {
            PracticalAlpha1HotPlugPlanError {
                kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
                path: path.to_path_buf(),
                detail: "auth_gate_layer requires contract_update_ref".to_string(),
            }
        })?;
        let activation_cut_ref =
            hotplug
                .activation_cut_ref
                .as_ref()
                .ok_or_else(|| PracticalAlpha1HotPlugPlanError {
                    kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
                    path: path.to_path_buf(),
                    detail: "auth_gate_layer requires activation_cut_ref".to_string(),
                })?;
        if contract_update_ref != activation_cut_ref {
            return Err(PracticalAlpha1HotPlugPlanError {
                kind: PracticalAlpha1HotPlugPlanErrorKind::MalformedHotPlugInput,
                path: path.to_path_buf(),
                detail: "auth_gate_layer requires contract_update_ref to match activation_cut_ref"
                    .to_string(),
            });
        }
    }

    Ok(PracticalAlpha1HotPlugPlan {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        hotplug_plan_scope: hotplug_plan_scope(),
        sample_id: hotplug.sample_id.clone(),
        package_id: package.package_id.clone(),
        package_kind: package.package_kind.clone(),
        manifest_version: manifest.version.clone(),
        attach_profile: attach_profile_name(attach_profile).to_string(),
        attachpoint_ref: hotplug.attachpoint_ref.clone(),
        requesting_principal: hotplug.requesting_principal.clone(),
        requesting_participant_place: hotplug.requesting_participant_place.clone(),
        capability_refs: hotplug.capability_refs.clone(),
        membership_epoch: hotplug.membership_epoch,
        member_incarnation: hotplug.member_incarnation,
        witness_refs: hotplug.witness_refs.clone(),
        required_witness_refs: hotplug.required_witness_refs.clone(),
        pre_attach_membership_advances: hotplug.pre_attach_membership_advances.clone(),
        activation_cut_ref: hotplug.activation_cut_ref.clone(),
        contract_update_ref: hotplug.contract_update_ref.clone(),
        retained_later_refs: retained_later_refs_default(),
    })
}

fn attach_profile_name(profile: PracticalAlpha1AttachProfile) -> &'static str {
    match profile {
        PracticalAlpha1AttachProfile::DebugTraceLayer => "debug_trace_layer",
        PracticalAlpha1AttachProfile::AuthGateLayer => "auth_gate_layer",
        PracticalAlpha1AttachProfile::RateLimitLayer => "rate_limit_layer",
        PracticalAlpha1AttachProfile::UnsafeDebugShadowLayer => "unsafe_debug_shadow_layer",
        PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage => {
            "placeholder_avatar_object_package"
        }
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_HOTPLUG_PLAN_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE_KIND.to_string()
}

fn hotplug_plan_scope() -> String {
    PRACTICAL_ALPHA1_HOTPLUG_PLAN_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_HOTPLUG_PLAN_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
