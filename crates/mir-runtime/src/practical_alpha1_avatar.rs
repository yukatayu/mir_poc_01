use std::{
    fmt,
    path::{Path, PathBuf},
};

use mir_ast::practical_alpha1::{
    PracticalAlpha1AttachProfile, PracticalAlpha1Package, load_practical_alpha1_package_path,
};
use serde::Serialize;

use crate::practical_alpha1_hotplug::{
    PracticalAlpha1HotPlugError, PracticalAlpha1HotPlugReport, attach_practical_alpha1_package,
    attach_practical_alpha1_package_path,
};

pub const PRACTICAL_ALPHA1_AVATAR_SCOPE: &str = "practical-alpha1-avatar-preview-floor";
pub const PRACTICAL_ALPHA1_AVATAR_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_avatar_preview_report";
pub const PRACTICAL_ALPHA1_AVATAR_SCOPE_KIND: &str = "alpha_local";

const PLACEHOLDER_REPRESENTATION: &str = "static_capsule_placeholder";
const CUSTOM_RUNTIME_REPRESENTATION: &str = "mir_humanoid_runtime_preview";
const CUSTOM_RUNTIME_HOST_CAPABILITY: &str = "HostMirAvatarVM";

const PLACEHOLDER_ACTIVE_ROLES: &[&str] = &["EmbodiedPresence", "Renderable"];
const PLACEHOLDER_HOST_CAPABILITIES: &[&str] = &[
    "AttachAvatarRuntime",
    "ManageRuntimePackageRegistry",
    "RenderPlaceholderMesh",
];
const CUSTOM_HOST_CAPABILITIES: &[&str] = &[
    "AttachAvatarRuntime",
    "ManageRuntimePackageRegistry",
    "RenderPlaceholderMesh",
    CUSTOM_RUNTIME_HOST_CAPABILITY,
];

const PRACTICAL_ALPHA1_AVATAR_RETAINED_LATER_REFS: &[&str] = &[
    "native_execution_realization",
    "final_avatar_package_abi",
    "same_session_runtime_attach_execution",
    "vrm_vrchat_unity_compatibility",
    "product_prototype_completion",
];

const PRACTICAL_ALPHA1_AVATAR_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 avatar preview as native execution or final avatar ABI completion",
    "do not treat the practical alpha-1 avatar preview as successful unsupported-runtime execution",
    "do not treat the practical alpha-1 avatar preview as an active runnable-root promotion",
];

const PRACTICAL_ALPHA1_AVATAR_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical avatar preview floor only",
    "limited AV-A1 practical sample families only",
    "no native execution, final avatar ABI, or monolithic product runtime completion",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1AvatarPreviewErrorKind {
    FrontDoor,
    HotPlug,
    MissingManifest,
    UnsupportedAttachProfile,
    UnsupportedSourceHotPlugOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1AvatarPreviewError {
    pub kind: PracticalAlpha1AvatarPreviewErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1AvatarPreviewError {
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

impl std::error::Error for PracticalAlpha1AvatarPreviewError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PracticalAlpha1AvatarPreviewReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "avatar_scope")]
    pub avatar_scope: String,
    pub hotplug_scope: String,
    pub hotplug_plan_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub package_kind: String,
    pub attach_profile: String,
    pub preview_kind: String,
    pub requested_runtime_route: String,
    pub source_hotplug_terminal_outcome: String,
    #[serde(default)]
    pub source_hotplug_reason_family: Option<String>,
    #[serde(default)]
    pub source_hotplug_rejection_reason_refs: Vec<String>,
    pub selected_representation: String,
    pub fallback_representation: String,
    pub fallback_applied: bool,
    #[serde(default)]
    pub fallback_reason: Option<String>,
    pub fallback_visible: bool,
    pub provided_roles: Vec<String>,
    pub active_roles: Vec<String>,
    pub degraded_roles: Vec<String>,
    pub required_host_capabilities: Vec<String>,
    pub available_host_capabilities: Vec<String>,
    pub missing_host_capabilities: Vec<String>,
    pub signature_present: bool,
    pub native_execution_performed: bool,
    pub object_attach_claimed: bool,
    pub final_avatar_abi_frozen: bool,
    pub what_it_proves: Vec<String>,
    pub what_it_does_not_prove: Vec<String>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

enum AvatarPreviewProfile {
    Placeholder,
    CustomAccepted,
    CustomFallback,
}

impl AvatarPreviewProfile {
    fn from_attach_profile(
        profile: PracticalAlpha1AttachProfile,
    ) -> Result<Self, PracticalAlpha1AvatarPreviewErrorKind> {
        match profile {
            PracticalAlpha1AttachProfile::PlaceholderAvatarObjectPackage => Ok(Self::Placeholder),
            PracticalAlpha1AttachProfile::CustomMirAvatarObjectPackage => Ok(Self::CustomAccepted),
            PracticalAlpha1AttachProfile::CustomMirAvatarFallbackObjectPackage => {
                Ok(Self::CustomFallback)
            }
            _ => Err(PracticalAlpha1AvatarPreviewErrorKind::UnsupportedAttachProfile),
        }
    }

    fn attach_profile_name(&self) -> &'static str {
        match self {
            Self::Placeholder => "placeholder_avatar_object_package",
            Self::CustomAccepted => "custom_mir_avatar_object_package",
            Self::CustomFallback => "custom_mir_avatar_fallback_object_package",
        }
    }

    fn preview_kind(&self) -> &'static str {
        match self {
            Self::Placeholder => "placeholder_avatar_runtime_preview",
            Self::CustomAccepted => "custom_mir_avatar_runtime_preview",
            Self::CustomFallback => "unsupported_runtime_visible_fallback_preview",
        }
    }

    fn requested_runtime_route(&self) -> &'static str {
        match self {
            Self::Placeholder => "placeholder_avatar_runtime",
            Self::CustomAccepted | Self::CustomFallback => "custom_mir_avatar_runtime",
        }
    }

    fn selected_representation(&self) -> &'static str {
        match self {
            Self::Placeholder | Self::CustomFallback => PLACEHOLDER_REPRESENTATION,
            Self::CustomAccepted => CUSTOM_RUNTIME_REPRESENTATION,
        }
    }

    fn available_host_capabilities(&self) -> &'static [&'static str] {
        match self {
            Self::Placeholder | Self::CustomFallback => PLACEHOLDER_HOST_CAPABILITIES,
            Self::CustomAccepted => CUSTOM_HOST_CAPABILITIES,
        }
    }
}

pub fn render_practical_alpha1_avatar_preview_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1AvatarPreviewReport, PracticalAlpha1AvatarPreviewError> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha1AvatarPreviewError {
            kind: PracticalAlpha1AvatarPreviewErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    let hotplug_report = attach_practical_alpha1_package_path(&path)
        .map_err(|error| map_hotplug_error(error, &path))?;
    render_practical_alpha1_avatar_preview_at_path(&package, hotplug_report, &path)
}

pub fn render_practical_alpha1_avatar_preview(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1AvatarPreviewReport, PracticalAlpha1AvatarPreviewError> {
    let hotplug_report = attach_practical_alpha1_package(package)
        .map_err(|error| map_hotplug_error(error, Path::new("<inline>")))?;
    render_practical_alpha1_avatar_preview_at_path(package, hotplug_report, Path::new("<inline>"))
}

fn render_practical_alpha1_avatar_preview_at_path(
    package: &PracticalAlpha1Package,
    hotplug_report: PracticalAlpha1HotPlugReport,
    path: &Path,
) -> Result<PracticalAlpha1AvatarPreviewReport, PracticalAlpha1AvatarPreviewError> {
    let manifest = package
        .manifest
        .as_ref()
        .ok_or_else(|| PracticalAlpha1AvatarPreviewError {
            kind: PracticalAlpha1AvatarPreviewErrorKind::MissingManifest,
            path: path.to_path_buf(),
            detail: "practical avatar preview requires `manifest`".to_string(),
        })?;
    let attach_profile =
        manifest
            .attach_profile
            .ok_or_else(|| PracticalAlpha1AvatarPreviewError {
                kind: PracticalAlpha1AvatarPreviewErrorKind::UnsupportedAttachProfile,
                path: path.to_path_buf(),
                detail: "practical avatar preview requires manifest.attach_profile".to_string(),
            })?;
    let profile = AvatarPreviewProfile::from_attach_profile(attach_profile).map_err(|kind| {
        PracticalAlpha1AvatarPreviewError {
            kind,
            path: path.to_path_buf(),
            detail: "practical avatar preview only admits avatar object attach profiles"
                .to_string(),
        }
    })?;

    validate_source_hotplug_outcome(&profile, &hotplug_report, path)?;

    let available_host_capabilities: Vec<String> = profile
        .available_host_capabilities()
        .iter()
        .map(|value| (*value).to_string())
        .collect();
    let missing_host_capabilities: Vec<String> = manifest
        .required_host_capabilities
        .iter()
        .filter(|capability| !available_host_capabilities.contains(capability))
        .cloned()
        .collect();

    let provided_roles = manifest.provided_roles.clone();
    let (active_roles, degraded_roles, fallback_applied, fallback_reason, fallback_visible) =
        match profile {
            AvatarPreviewProfile::Placeholder => {
                (provided_roles.clone(), Vec::new(), false, None, true)
            }
            AvatarPreviewProfile::CustomAccepted => {
                (provided_roles.clone(), Vec::new(), false, None, true)
            }
            AvatarPreviewProfile::CustomFallback => {
                let active_roles: Vec<String> = provided_roles
                    .iter()
                    .filter(|role| PLACEHOLDER_ACTIVE_ROLES.contains(&role.as_str()))
                    .cloned()
                    .collect();
                let degraded_roles: Vec<String> = provided_roles
                    .iter()
                    .filter(|role| !active_roles.contains(role))
                    .cloned()
                    .collect();
                (
                    active_roles,
                    degraded_roles,
                    true,
                    hotplug_report
                        .rejection_reason_refs
                        .iter()
                        .find(|reason| reason.contains(CUSTOM_RUNTIME_HOST_CAPABILITY))
                        .cloned(),
                    true,
                )
            }
        };

    Ok(PracticalAlpha1AvatarPreviewReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        avatar_scope: avatar_scope(),
        hotplug_scope: hotplug_report.hotplug_scope.clone(),
        hotplug_plan_scope: hotplug_report.hotplug_plan_scope.clone(),
        sample_id: hotplug_report.sample_id.clone(),
        package_id: hotplug_report.package_id.clone(),
        package_kind: hotplug_report.package_kind.clone(),
        attach_profile: profile.attach_profile_name().to_string(),
        preview_kind: profile.preview_kind().to_string(),
        requested_runtime_route: profile.requested_runtime_route().to_string(),
        source_hotplug_terminal_outcome: hotplug_report.terminal_outcome.clone(),
        source_hotplug_reason_family: hotplug_report.reason_family.clone(),
        source_hotplug_rejection_reason_refs: hotplug_report.rejection_reason_refs.clone(),
        selected_representation: profile.selected_representation().to_string(),
        fallback_representation: manifest
            .fallback_representation
            .clone()
            .unwrap_or_else(|| PLACEHOLDER_REPRESENTATION.to_string()),
        fallback_applied,
        fallback_reason,
        fallback_visible,
        provided_roles,
        active_roles,
        degraded_roles,
        required_host_capabilities: manifest.required_host_capabilities.clone(),
        available_host_capabilities,
        missing_host_capabilities,
        signature_present: package
            .native
            .as_ref()
            .map(|native| native.signature_present)
            .unwrap_or(false),
        native_execution_performed: false,
        object_attach_claimed: false,
        final_avatar_abi_frozen: false,
        what_it_proves: what_it_proves(&profile),
        what_it_does_not_prove: what_it_does_not_prove(&profile),
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    })
}

fn validate_source_hotplug_outcome(
    profile: &AvatarPreviewProfile,
    hotplug_report: &PracticalAlpha1HotPlugReport,
    path: &Path,
) -> Result<(), PracticalAlpha1AvatarPreviewError> {
    match profile {
        AvatarPreviewProfile::Placeholder | AvatarPreviewProfile::CustomAccepted => {
            if hotplug_report.terminal_outcome != "accepted_object_attach_preview" {
                return Err(PracticalAlpha1AvatarPreviewError {
                    kind: PracticalAlpha1AvatarPreviewErrorKind::UnsupportedSourceHotPlugOutcome,
                    path: path.to_path_buf(),
                    detail: format!(
                        "avatar preview requires accepted_object_attach_preview, found `{}`",
                        hotplug_report.terminal_outcome
                    ),
                });
            }
        }
        AvatarPreviewProfile::CustomFallback => {
            let has_missing_runtime_capability = hotplug_report
                .rejection_reason_refs
                .iter()
                .any(|reason| reason == "missing_host_capability:HostMirAvatarVM");
            if hotplug_report.terminal_outcome != "rejected"
                || hotplug_report.reason_family.as_deref() != Some("compatibility")
                || !has_missing_runtime_capability
            {
                return Err(PracticalAlpha1AvatarPreviewError {
                    kind: PracticalAlpha1AvatarPreviewErrorKind::UnsupportedSourceHotPlugOutcome,
                    path: path.to_path_buf(),
                    detail: "unsupported runtime fallback preview requires a compatibility rejection with missing_host_capability:HostMirAvatarVM".to_string(),
                });
            }
        }
    }
    Ok(())
}

fn what_it_proves(profile: &AvatarPreviewProfile) -> Vec<String> {
    match profile {
        AvatarPreviewProfile::Placeholder => vec![
            "placeholder avatar preview stays explicit and non-final".to_string(),
            "selected representation remains visible to the caller".to_string(),
        ],
        AvatarPreviewProfile::CustomAccepted => vec![
            "custom Mir avatar runtime route is admitted as a non-final preview".to_string(),
            "custom runtime selection stays explicit without claiming native execution".to_string(),
        ],
        AvatarPreviewProfile::CustomFallback => vec![
            "unsupported runtime fallback remains visible and monotone".to_string(),
            "custom runtime rejection and placeholder degradation stay explicit".to_string(),
        ],
    }
}

fn what_it_does_not_prove(profile: &AvatarPreviewProfile) -> Vec<String> {
    let mut lines = vec![
        "native execution".to_string(),
        "final avatar ABI".to_string(),
        "VRM/VRChat/Unity compatibility".to_string(),
        "monolithic same-session product runtime".to_string(),
    ];
    if matches!(profile, AvatarPreviewProfile::CustomFallback) {
        lines.push("successful unsupported-runtime execution".to_string());
    }
    lines
}

fn map_hotplug_error(
    error: PracticalAlpha1HotPlugError,
    path: &Path,
) -> PracticalAlpha1AvatarPreviewError {
    PracticalAlpha1AvatarPreviewError {
        kind: PracticalAlpha1AvatarPreviewErrorKind::HotPlug,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_AVATAR_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_AVATAR_SCOPE_KIND.to_string()
}

fn avatar_scope() -> String {
    PRACTICAL_ALPHA1_AVATAR_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_AVATAR_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_AVATAR_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_AVATAR_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
