use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::error::Category;

pub const PRACTICAL_ALPHA1_FORMAT_VERSION: &str = "mirrorea-practical-alpha1-v0";
pub const PRACTICAL_ALPHA1_PACKAGE_FILE_NAME: &str = "package.mir.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1FrontDoorManifest {
    pub carrier_kind: &'static str,
    pub accepted_surface_refs: &'static [&'static str],
    pub code_anchor_refs: &'static [&'static str],
    pub retained_later_refs: &'static [&'static str],
}

const PRACTICAL_ALPHA1_FRONT_DOOR_ACCEPTED_SURFACE_REFS: &[&str] = &[
    "package.mir.json",
    "world",
    "places",
    "fallback_chains",
    "layers",
    "manifest",
    "native",
    "alpha_local_checker_input",
    "alpha_local_runtime_input",
    "alpha_local_hotplug_input",
    "alpha_local_transport_input",
    "alpha_local_save_load_input",
];

const PRACTICAL_ALPHA1_FRONT_DOOR_CODE_ANCHOR_REFS: &[&str] = &[
    "mir_ast_practical_alpha1_module",
    "practical_alpha1_front_door_tests",
];

const PRACTICAL_ALPHA1_FRONT_DOOR_RETAINED_LATER_REFS: &[&str] = &[
    "final_textual_alpha_source_grammar",
    "typed_ir_checker_integration",
    "runtime_plan_execution",
];

pub const PRACTICAL_ALPHA1_FRONT_DOOR_MANIFEST: PracticalAlpha1FrontDoorManifest =
    PracticalAlpha1FrontDoorManifest {
        carrier_kind: "practical_alpha1_nonfinal_package_json_front_door",
        accepted_surface_refs: PRACTICAL_ALPHA1_FRONT_DOOR_ACCEPTED_SURFACE_REFS,
        code_anchor_refs: PRACTICAL_ALPHA1_FRONT_DOOR_CODE_ANCHOR_REFS,
        retained_later_refs: PRACTICAL_ALPHA1_FRONT_DOOR_RETAINED_LATER_REFS,
    };

pub fn practical_alpha1_front_door_manifest() -> &'static PracticalAlpha1FrontDoorManifest {
    &PRACTICAL_ALPHA1_FRONT_DOOR_MANIFEST
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1FrontDoorErrorKind {
    MissingPackageFile,
    Io,
    JsonParse,
    SchemaDecode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1FrontDoorError {
    pub kind: PracticalAlpha1FrontDoorErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1FrontDoorError {
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

impl std::error::Error for PracticalAlpha1FrontDoorError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1Package {
    pub format_version: String,
    pub package_id: String,
    pub package_kind: String,
    #[serde(default)]
    pub world: Option<PracticalAlpha1World>,
    #[serde(default)]
    pub places: Vec<PracticalAlpha1Place>,
    #[serde(default)]
    pub fallback_chains: Vec<PracticalAlpha1FallbackChain>,
    #[serde(default)]
    pub layers: Vec<PracticalAlpha1LayerAttachment>,
    #[serde(default)]
    pub manifest: Option<PracticalAlpha1PackageManifest>,
    #[serde(default)]
    pub native: Option<PracticalAlpha1NativeManifest>,
    #[serde(default, alias = "checker")]
    pub alpha_local_checker_input: Option<PracticalAlpha1AlphaLocalCheckerInput>,
    #[serde(default, alias = "runtime")]
    pub alpha_local_runtime_input: Option<PracticalAlpha1AlphaLocalRuntimeInput>,
    #[serde(default, alias = "hotplug")]
    pub alpha_local_hotplug_input: Option<PracticalAlpha1AlphaLocalHotPlugInput>,
    #[serde(default, alias = "transport")]
    pub alpha_local_transport_input: Option<PracticalAlpha1AlphaLocalTransportInput>,
    #[serde(default, alias = "save_load")]
    pub alpha_local_save_load_input: Option<PracticalAlpha1AlphaLocalSaveLoadInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1World {
    pub id: String,
    pub entry_place: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1Place {
    pub id: String,
    pub authority: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1FallbackChain {
    pub id: String,
    pub capability: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1LayerAttachment {
    pub id: String,
    pub kind: String,
    pub attach_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1PackageManifest {
    pub version: String,
    #[serde(default)]
    pub attach_profile: Option<PracticalAlpha1AttachProfile>,
    #[serde(default)]
    pub requires_capabilities: Vec<String>,
    #[serde(default)]
    pub provided_capabilities: Vec<String>,
    #[serde(default)]
    pub provided_roles: Vec<String>,
    #[serde(default)]
    pub required_host_capabilities: Vec<String>,
    #[serde(default)]
    pub effect_row: Vec<String>,
    #[serde(default)]
    pub failure_row: Vec<String>,
    #[serde(default)]
    pub observation_labels: Vec<String>,
    #[serde(default)]
    pub fallback_representation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticalAlpha1AttachProfile {
    DebugTraceLayer,
    AuthGateLayer,
    RateLimitLayer,
    UnsafeDebugShadowLayer,
    PlaceholderAvatarObjectPackage,
    CustomMirAvatarObjectPackage,
    CustomMirAvatarFallbackObjectPackage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1NativeManifest {
    pub entry_ref: String,
    pub signature_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1AlphaLocalCheckerInput {
    pub family: PracticalAlpha1AlphaLocalCheckerFamily,
    pub case: PracticalAlpha1AlphaLocalCheckerCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticalAlpha1AlphaLocalCheckerFamily {
    LifetimeFallback,
    ContractVariance,
    CutPredicate,
    PackageAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PracticalAlpha1AlphaLocalCheckerCase {
    RawDanglingReference {
        sample_id: String,
        source_ref: String,
        stored_into: String,
    },
    FallbackChainValid {
        sample_id: String,
        role: String,
        capability: String,
        options: Vec<String>,
        monotone_degradation: bool,
    },
    InheritedChainValid {
        sample_id: String,
        source_chain: Vec<String>,
        appended_fallback: String,
        lineage_edges: Vec<String>,
        implicit_propagation: bool,
    },
    SnapshotSelectedDistinction {
        sample_id: String,
        source_chain: Vec<String>,
        selected_option: String,
        appended_fallback: String,
        excluded_options: Vec<String>,
    },
    TransparentObserveOnlyLayer {
        sample_id: String,
        layer_kind: String,
        effect_delta: Vec<String>,
        failure_delta: Vec<String>,
        precondition_strengthened: bool,
        postcondition_weakened: bool,
    },
    PreconditionStrengthening {
        sample_id: String,
        base_precondition: String,
        layer_precondition: String,
    },
    MutableCovariance {
        sample_id: String,
        base_capability: String,
        widened_capability: String,
    },
    OrphanReceive {
        sample_id: String,
        receive_event: String,
        missing_predecessor: String,
    },
    UnsignedNativePackage {
        sample_id: String,
        required_signature: bool,
    },
    OverCapabilityPackage {
        sample_id: String,
        allowed_capability_prefixes: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1AlphaLocalRuntimeInput {
    pub queue_kind: String,
    pub runtime_places: Vec<PracticalAlpha1RuntimePlaceSeed>,
    pub bootstrap_participants: Vec<PracticalAlpha1RuntimeBootstrapParticipant>,
    #[serde(default)]
    pub pre_dispatch_membership_advances: Vec<PracticalAlpha1RuntimeMembershipAdvance>,
    pub dispatch_program: PracticalAlpha1RuntimeDispatchProgram,
    pub initial_envelopes: Vec<PracticalAlpha1RuntimeEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1AlphaLocalHotPlugInput {
    pub sample_id: String,
    #[serde(default = "default_hotplug_operation_kind")]
    pub operation_kind: PracticalAlpha1HotPlugOperationKind,
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
    pub detach_boundary_ref: Option<String>,
    #[serde(default)]
    pub contract_update_ref: Option<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1AlphaLocalTransportInput {
    pub sample_id: String,
    pub transport_surface: PracticalAlpha1TransportSurface,
    pub bridge_kind: String,
    pub runtime_places: Vec<PracticalAlpha1RuntimePlaceSeed>,
    pub bootstrap_participants: Vec<PracticalAlpha1RuntimeBootstrapParticipant>,
    #[serde(default)]
    pub pre_dispatch_membership_advances: Vec<PracticalAlpha1RuntimeMembershipAdvance>,
    pub request_envelope: PracticalAlpha1RuntimeEnvelope,
    pub required_capabilities: Vec<String>,
    pub required_witness_refs: Vec<String>,
    #[serde(default)]
    pub auth_bindings_required: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1AlphaLocalSaveLoadInput {
    pub sample_id: String,
    pub scenario_kind: PracticalAlpha1SaveLoadScenarioKind,
    pub required_base_terminal_outcome: String,
    pub required_saved_owner: String,
    pub required_saved_publish_history_entry: String,
    pub required_saved_history_tail: String,
    pub resumed_dispatch_program: PracticalAlpha1RuntimeDispatchProgram,
    pub resumed_envelope: PracticalAlpha1RuntimeEnvelope,
    #[serde(default)]
    pub post_restore_membership_advances: Vec<PracticalAlpha1RuntimeMembershipAdvance>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticalAlpha1HotPlugOperationKind {
    Attach,
    Detach,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticalAlpha1TransportSurface {
    LocalTcp,
    DockerComposeTcp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticalAlpha1SaveLoadScenarioKind {
    ResumeOneDispatch,
    RejectStaleMembership,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimePlaceSeed {
    pub place_id: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimeBootstrapParticipant {
    pub principal: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimeMembershipAdvance {
    pub kind: PracticalAlpha1RuntimeMembershipAdvanceKind,
    pub principal: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticalAlpha1RuntimeMembershipAdvanceKind {
    AddParticipant,
}

fn default_hotplug_operation_kind() -> PracticalAlpha1HotPlugOperationKind {
    PracticalAlpha1HotPlugOperationKind::Attach
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PracticalAlpha1RuntimeDispatchProgram {
    SugorokuRollHandoff {
        envelope_id: String,
        roll_value: u64,
        handoff_target: String,
        publication_witness_ref: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimeEnvelope {
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub transport: String,
    #[serde(default)]
    pub transport_medium: Option<String>,
    pub transport_seam: String,
    pub payload_kind: String,
    pub payload_ref: String,
    pub principal_claim: PracticalAlpha1RuntimePrincipalClaim,
    #[serde(default)]
    pub auth_evidence: Option<PracticalAlpha1RuntimeAuthEvidence>,
    pub emitter_principal: String,
    pub membership_epoch: u64,
    pub member_incarnation: u64,
    pub freshness_checks: Vec<String>,
    pub capability_requirements: Vec<String>,
    pub authorization_checks: Vec<String>,
    pub witness_refs: Vec<String>,
    pub dispatch_outcome: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimePrincipalClaim {
    pub principal: String,
    pub participant_place: String,
    pub claimed_authority: String,
    pub claimed_capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimeAuthEvidence {
    pub kind: String,
    pub subject: String,
    pub issuer: String,
    pub bindings: Vec<String>,
    pub notes: Vec<String>,
}

pub fn load_practical_alpha1_package_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1Package, PracticalAlpha1FrontDoorError> {
    let resolved_path = resolve_package_path(path.as_ref())?;
    let text =
        fs::read_to_string(&resolved_path).map_err(|error| PracticalAlpha1FrontDoorError {
            kind: PracticalAlpha1FrontDoorErrorKind::Io,
            path: resolved_path.clone(),
            detail: error.to_string(),
        })?;

    parse_practical_alpha1_package_text_at_path(&text, &resolved_path)
}

pub fn parse_practical_alpha1_package_text(
    text: &str,
) -> Result<PracticalAlpha1Package, PracticalAlpha1FrontDoorError> {
    parse_practical_alpha1_package_text_at_path(text, Path::new("<inline>"))
}

fn parse_practical_alpha1_package_text_at_path(
    text: &str,
    path: &Path,
) -> Result<PracticalAlpha1Package, PracticalAlpha1FrontDoorError> {
    let package = serde_json::from_str::<PracticalAlpha1Package>(text).map_err(|error| {
        let kind = match error.classify() {
            Category::Syntax | Category::Eof => PracticalAlpha1FrontDoorErrorKind::JsonParse,
            Category::Data => PracticalAlpha1FrontDoorErrorKind::SchemaDecode,
            Category::Io => PracticalAlpha1FrontDoorErrorKind::Io,
        };
        PracticalAlpha1FrontDoorError {
            kind,
            path: path.to_path_buf(),
            detail: error.to_string(),
        }
    })?;

    validate_package_shape(&package, path)?;
    Ok(package)
}

fn resolve_package_path(path: &Path) -> Result<PathBuf, PracticalAlpha1FrontDoorError> {
    if path.is_dir() {
        let candidate = path.join(PRACTICAL_ALPHA1_PACKAGE_FILE_NAME);
        if candidate.exists() {
            return Ok(candidate);
        }

        return Err(PracticalAlpha1FrontDoorError {
            kind: PracticalAlpha1FrontDoorErrorKind::MissingPackageFile,
            path: candidate,
            detail: format!(
                "package directory is missing {}",
                PRACTICAL_ALPHA1_PACKAGE_FILE_NAME
            ),
        });
    }

    if path.file_name().and_then(|name| name.to_str()) != Some(PRACTICAL_ALPHA1_PACKAGE_FILE_NAME) {
        return Err(PracticalAlpha1FrontDoorError {
            kind: PracticalAlpha1FrontDoorErrorKind::MissingPackageFile,
            path: path.to_path_buf(),
            detail: format!(
                "practical alpha-1 front-door only accepts {}",
                PRACTICAL_ALPHA1_PACKAGE_FILE_NAME
            ),
        });
    }

    if path.exists() {
        return Ok(path.to_path_buf());
    }

    Err(PracticalAlpha1FrontDoorError {
        kind: PracticalAlpha1FrontDoorErrorKind::MissingPackageFile,
        path: path.to_path_buf(),
        detail: "package file does not exist".to_string(),
    })
}

fn validate_package_shape(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<(), PracticalAlpha1FrontDoorError> {
    if package.format_version != PRACTICAL_ALPHA1_FORMAT_VERSION {
        return Err(schema_error(
            path,
            format!(
                "unsupported format_version `{}`; expected `{}`",
                package.format_version, PRACTICAL_ALPHA1_FORMAT_VERSION
            ),
        ));
    }

    match package.package_kind.as_str() {
        "world" => {
            if package.world.is_none() {
                return Err(schema_error(
                    path,
                    "world package must include `world`".to_string(),
                ));
            }
        }
        "layer" | "object" | "runtime" | "avatar" | "adapter" => {}
        other => {
            return Err(schema_error(
                path,
                format!("unsupported package_kind `{other}`"),
            ));
        }
    }

    Ok(())
}

fn schema_error(path: &Path, detail: String) -> PracticalAlpha1FrontDoorError {
    PracticalAlpha1FrontDoorError {
        kind: PracticalAlpha1FrontDoorErrorKind::SchemaDecode,
        path: path.to_path_buf(),
        detail,
    }
}
