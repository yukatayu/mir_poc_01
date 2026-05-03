use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::practical_alpha1::{
    PracticalAlpha1Package, PracticalAlpha1RuntimeBootstrapParticipant,
    PracticalAlpha1RuntimeEnvelope, PracticalAlpha1RuntimeMembershipAdvance,
    PracticalAlpha1RuntimePlaceSeed, PracticalAlpha1TransportSurface,
    load_practical_alpha1_package_path,
};
use crate::practical_alpha1_checker::{PracticalAlpha1CheckError, check_practical_alpha1_package};

pub const PRACTICAL_ALPHA1_TRANSPORT_PLAN_SCOPE: &str = "practical-alpha1-transport-plan-floor";
pub const PRACTICAL_ALPHA1_TRANSPORT_PLAN_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_transport_plan";
pub const PRACTICAL_ALPHA1_TRANSPORT_PLAN_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_TRANSPORT_PLAN_RETAINED_LATER_REFS: &[&str] = &[
    "wan_federation",
    "session_replay_repair",
    "network_partition_recovery",
    "local_save_load_execution",
    "devtools_viewer_execution",
    "final_public_transport_api",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1TransportPlanErrorKind {
    FrontDoor,
    Checker,
    RejectedByChecker,
    MissingTransportSection,
    MalformedTransportInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1TransportPlanError {
    pub kind: PracticalAlpha1TransportPlanErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1TransportPlanError {
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

impl std::error::Error for PracticalAlpha1TransportPlanError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1TransportPlan {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "transport_plan_scope")]
    pub transport_plan_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub world_id: String,
    pub transport_surface: String,
    pub transport_medium: String,
    pub transport_seam: String,
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
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
}

pub fn build_practical_alpha1_transport_plan_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1TransportPlan, PracticalAlpha1TransportPlanError> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha1TransportPlanError {
            kind: PracticalAlpha1TransportPlanErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    build_practical_alpha1_transport_plan_at_path(&package, &path)
}

pub fn build_practical_alpha1_transport_plan(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1TransportPlan, PracticalAlpha1TransportPlanError> {
    build_practical_alpha1_transport_plan_at_path(package, Path::new("<inline>"))
}

fn build_practical_alpha1_transport_plan_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha1TransportPlan, PracticalAlpha1TransportPlanError> {
    if package.package_kind != "world" {
        return Err(PracticalAlpha1TransportPlanError {
            kind: PracticalAlpha1TransportPlanErrorKind::MalformedTransportInput,
            path: path.to_path_buf(),
            detail: format!(
                "practical transport plan currently admits only world packages, found `{}`",
                package.package_kind
            ),
        });
    }

    let checker_present = package.alpha_local_checker_input.is_some();
    if checker_present {
        let checker = check_practical_alpha1_package(package)
            .map_err(|error| wrap_checker_error(path, error))?;
        if checker.verdict != "accepted" || !checker.rejected_rows.is_empty() {
            return Err(PracticalAlpha1TransportPlanError {
                kind: PracticalAlpha1TransportPlanErrorKind::RejectedByChecker,
                path: path.to_path_buf(),
                detail: format!(
                    "checked package verdict must be accepted before transport-plan lowering, observed `{}`",
                    checker.verdict
                ),
            });
        }
    }

    let transport = package
        .alpha_local_transport_input
        .as_ref()
        .ok_or_else(|| PracticalAlpha1TransportPlanError {
            kind: PracticalAlpha1TransportPlanErrorKind::MissingTransportSection,
            path: path.to_path_buf(),
            detail: "package does not declare `alpha_local_transport_input`".to_string(),
        })?;

    if !checker_present {
        return Err(PracticalAlpha1TransportPlanError {
            kind: PracticalAlpha1TransportPlanErrorKind::Checker,
            path: path.to_path_buf(),
            detail:
                "transport-plan floor requires a checked package with `alpha_local_checker_input`"
                    .to_string(),
        });
    }

    let world = package.world.as_ref().ok_or_else(|| {
        malformed_transport_input(
            path,
            "transport-plan floor currently requires `package_kind = world` with `world` section"
                .to_string(),
        )
    })?;

    validate_transport_input(package, transport, path)?;

    Ok(PracticalAlpha1TransportPlan {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        transport_plan_scope: transport_plan_scope(),
        sample_id: transport.sample_id.clone(),
        package_id: package.package_id.clone(),
        world_id: world.id.clone(),
        transport_surface: transport_surface_name(transport.transport_surface).to_string(),
        transport_medium: transport
            .request_envelope
            .transport_medium
            .clone()
            .unwrap_or_default(),
        transport_seam: transport.request_envelope.transport_seam.clone(),
        bridge_kind: transport.bridge_kind.clone(),
        runtime_places: transport.runtime_places.clone(),
        bootstrap_participants: transport.bootstrap_participants.clone(),
        pre_dispatch_membership_advances: transport.pre_dispatch_membership_advances.clone(),
        request_envelope: transport.request_envelope.clone(),
        required_capabilities: transport.required_capabilities.clone(),
        required_witness_refs: transport.required_witness_refs.clone(),
        auth_bindings_required: transport.auth_bindings_required.clone(),
        notes: transport.notes.clone(),
        retained_later_refs: retained_later_refs_default(),
    })
}

fn validate_transport_input(
    package: &PracticalAlpha1Package,
    transport: &crate::practical_alpha1::PracticalAlpha1AlphaLocalTransportInput,
    path: &Path,
) -> Result<(), PracticalAlpha1TransportPlanError> {
    let world = package.world.as_ref().ok_or_else(|| {
        malformed_transport_input(
            path,
            "transport-plan floor currently requires `package_kind = world` with `world` section"
                .to_string(),
        )
    })?;
    if transport.sample_id.trim().is_empty() || transport.bridge_kind.trim().is_empty() {
        return Err(malformed_transport_input(
            path,
            "transport-plan floor requires non-empty sample_id and bridge_kind".to_string(),
        ));
    }
    if transport.runtime_places.is_empty()
        || transport.bootstrap_participants.is_empty()
        || transport.required_capabilities.is_empty()
        || transport.required_witness_refs.is_empty()
    {
        return Err(malformed_transport_input(
            path,
            "transport-plan floor requires runtime_places, bootstrap_participants, required_capabilities, and required_witness_refs".to_string(),
        ));
    }
    let envelope = &transport.request_envelope;
    if envelope.envelope_id.trim().is_empty()
        || envelope.from_place.trim().is_empty()
        || envelope.to_place.trim().is_empty()
        || envelope.transport.trim().is_empty()
        || envelope.transport_seam.trim().is_empty()
        || envelope.payload_kind.trim().is_empty()
        || envelope.payload_ref.trim().is_empty()
        || envelope.principal_claim.principal.trim().is_empty()
        || envelope.principal_claim.participant_place.trim().is_empty()
        || envelope.principal_claim.claimed_authority.trim().is_empty()
        || envelope.principal_claim.claimed_capabilities.is_empty()
        || envelope.emitter_principal.trim().is_empty()
        || envelope.freshness_checks.is_empty()
        || envelope.capability_requirements.is_empty()
        || envelope.authorization_checks.is_empty()
        || envelope.witness_refs.is_empty()
        || envelope.dispatch_outcome.trim().is_empty()
        || envelope
            .transport_medium
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(malformed_transport_input(
            path,
            "transport-plan floor requires a fully-populated request_envelope with transport_medium".to_string(),
        ));
    }

    let package_place_ids: BTreeSet<&str> = package
        .places
        .iter()
        .map(|place| place.id.as_str())
        .collect();
    if !package_place_ids.contains(world.entry_place.as_str()) {
        return Err(malformed_transport_input(
            path,
            format!(
                "world entry_place `{}` must also appear in package `places`",
                world.entry_place
            ),
        ));
    }
    let mut runtime_place_ids = BTreeSet::new();
    for place in &transport.runtime_places {
        if place.place_id.trim().is_empty() || place.kind.trim().is_empty() {
            return Err(malformed_transport_input(
                path,
                "runtime place entries must have non-empty place_id and kind".to_string(),
            ));
        }
        if !package_place_ids.contains(place.place_id.as_str()) {
            return Err(malformed_transport_input(
                path,
                format!(
                    "runtime place `{}` must also appear in package `places`",
                    place.place_id
                ),
            ));
        }
        runtime_place_ids.insert(place.place_id.as_str());
    }
    if !runtime_place_ids.contains(world.entry_place.as_str()) {
        return Err(malformed_transport_input(
            path,
            format!(
                "runtime_places must include world entry_place `{}`",
                world.entry_place
            ),
        ));
    }
    for participant in &transport.bootstrap_participants {
        if participant.principal.trim().is_empty() {
            return Err(malformed_transport_input(
                path,
                "bootstrap participants must have non-empty principal".to_string(),
            ));
        }
    }
    for advance in &transport.pre_dispatch_membership_advances {
        if advance.principal.trim().is_empty() {
            return Err(malformed_transport_input(
                path,
                "membership advances must have non-empty principal".to_string(),
            ));
        }
    }

    let required_capabilities: BTreeSet<_> = transport
        .required_capabilities
        .iter()
        .map(String::as_str)
        .collect();
    let envelope_capabilities: BTreeSet<_> = envelope
        .capability_requirements
        .iter()
        .map(String::as_str)
        .collect();
    if required_capabilities != envelope_capabilities {
        return Err(malformed_transport_input(
            path,
            "required_capabilities must exactly match request_envelope.capability_requirements"
                .to_string(),
        ));
    }

    Ok(())
}

fn transport_surface_name(surface: PracticalAlpha1TransportSurface) -> &'static str {
    match surface {
        PracticalAlpha1TransportSurface::LocalTcp => "local_tcp",
        PracticalAlpha1TransportSurface::DockerComposeTcp => "docker_compose_tcp",
    }
}

fn malformed_transport_input(path: &Path, detail: String) -> PracticalAlpha1TransportPlanError {
    PracticalAlpha1TransportPlanError {
        kind: PracticalAlpha1TransportPlanErrorKind::MalformedTransportInput,
        path: path.to_path_buf(),
        detail,
    }
}

fn wrap_checker_error(
    path: &Path,
    error: PracticalAlpha1CheckError,
) -> PracticalAlpha1TransportPlanError {
    PracticalAlpha1TransportPlanError {
        kind: PracticalAlpha1TransportPlanErrorKind::Checker,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_TRANSPORT_PLAN_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_TRANSPORT_PLAN_SCOPE_KIND.to_string()
}

fn transport_plan_scope() -> String {
    PRACTICAL_ALPHA1_TRANSPORT_PLAN_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_TRANSPORT_PLAN_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
