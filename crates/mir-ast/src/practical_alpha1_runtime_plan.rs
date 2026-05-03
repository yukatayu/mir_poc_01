use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::practical_alpha1::{
    PracticalAlpha1AlphaLocalRuntimeInput, PracticalAlpha1LayerAttachment, PracticalAlpha1Package,
    PracticalAlpha1RuntimeBootstrapParticipant, PracticalAlpha1RuntimeDispatchProgram,
    PracticalAlpha1RuntimeEnvelope, PracticalAlpha1RuntimeMembershipAdvance,
    PracticalAlpha1RuntimeMembershipAdvanceKind, PracticalAlpha1RuntimePlaceSeed,
    load_practical_alpha1_package_path,
};
use crate::practical_alpha1_checker::{PracticalAlpha1CheckError, check_practical_alpha1_package};

pub const PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE: &str = "practical-alpha1-runtime-plan-floor";
pub const PRACTICAL_ALPHA1_RUNTIME_PLAN_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_runtime_plan";
pub const PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_RUNTIME_PLAN_RETAINED_LATER_REFS: &[&str] = &[
    "run_local_runtime_execution",
    "docker_transport_execution",
    "package_hotplug_execution",
    "local_save_load_execution",
    "final_public_runtime_api",
];

const PRACTICAL_ALPHA1_RUNTIME_PLAN_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 runtime plan as a local runtime execution report",
    "do not treat the practical alpha-1 runtime plan as a public CLI/API freeze",
    "do not treat the practical alpha-1 runtime plan as Docker, package/hot-plug, or save/load completion",
];

const PRACTICAL_ALPHA1_RUNTIME_PLAN_LIMITATIONS: &[&str] = &[
    "alpha-local non-final runtime-plan floor only",
    "limited RUN practical sample families only",
    "no local runtime execution, Docker execution, package/hot-plug, save/load, or public runtime ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1RuntimePlanErrorKind {
    FrontDoor,
    Checker,
    RejectedByChecker,
    MissingRuntimeSection,
    MalformedRuntimeInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1RuntimePlanError {
    pub kind: PracticalAlpha1RuntimePlanErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1RuntimePlanError {
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

impl std::error::Error for PracticalAlpha1RuntimePlanError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RuntimePlan {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "runtime_plan_scope")]
    pub runtime_plan_scope: String,
    pub package_id: String,
    pub package_kind: String,
    pub world_id: String,
    pub entry_place: String,
    pub queue_kind: String,
    #[serde(default)]
    pub requested_layers: Vec<PracticalAlpha1LayerAttachment>,
    pub runtime_places: Vec<PracticalAlpha1RuntimePlaceSeed>,
    pub bootstrap_participants: Vec<PracticalAlpha1RuntimeBootstrapParticipant>,
    #[serde(default)]
    pub pre_dispatch_membership_advances: Vec<PracticalAlpha1RuntimeMembershipAdvance>,
    pub dispatch_program: PracticalAlpha1RuntimeDispatchProgram,
    pub initial_envelopes: Vec<PracticalAlpha1RuntimeEnvelope>,
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
}

pub fn build_practical_alpha1_runtime_plan_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1RuntimePlan, PracticalAlpha1RuntimePlanError> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha1RuntimePlanError {
            kind: PracticalAlpha1RuntimePlanErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    build_practical_alpha1_runtime_plan_at_path(&package, &path)
}

pub fn build_practical_alpha1_runtime_plan(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1RuntimePlan, PracticalAlpha1RuntimePlanError> {
    build_practical_alpha1_runtime_plan_at_path(package, Path::new("<inline>"))
}

fn build_practical_alpha1_runtime_plan_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha1RuntimePlan, PracticalAlpha1RuntimePlanError> {
    let checker_present = package.alpha_local_checker_input.is_some();
    if checker_present {
        let checker = check_practical_alpha1_package(package)
            .map_err(|error| wrap_checker_error(path, error))?;
        if checker.verdict != "accepted" || !checker.rejected_rows.is_empty() {
            return Err(PracticalAlpha1RuntimePlanError {
                kind: PracticalAlpha1RuntimePlanErrorKind::RejectedByChecker,
                path: path.to_path_buf(),
                detail: format!(
                    "checked package verdict must be accepted before runtime-plan lowering, observed `{}`",
                    checker.verdict
                ),
            });
        }
    }

    let runtime = package.alpha_local_runtime_input.as_ref().ok_or_else(|| {
        PracticalAlpha1RuntimePlanError {
            kind: PracticalAlpha1RuntimePlanErrorKind::MissingRuntimeSection,
            path: path.to_path_buf(),
            detail: "package does not declare `alpha_local_runtime_input`".to_string(),
        }
    })?;

    if !checker_present {
        return Err(PracticalAlpha1RuntimePlanError {
            kind: PracticalAlpha1RuntimePlanErrorKind::Checker,
            path: path.to_path_buf(),
            detail:
                "runtime-plan floor requires a checked package with `alpha_local_checker_input`"
                    .to_string(),
        });
    }

    validate_runtime_input(package, runtime, path)?;
    let world = package.world.as_ref().ok_or_else(|| {
        malformed_runtime_input(
            path,
            "runtime-plan floor currently requires a world package with `world`".to_string(),
        )
    })?;

    Ok(PracticalAlpha1RuntimePlan {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        runtime_plan_scope: runtime_plan_scope(),
        package_id: package.package_id.clone(),
        package_kind: package.package_kind.clone(),
        world_id: world.id.clone(),
        entry_place: world.entry_place.clone(),
        queue_kind: runtime.queue_kind.clone(),
        requested_layers: package.layers.clone(),
        runtime_places: runtime.runtime_places.clone(),
        bootstrap_participants: runtime.bootstrap_participants.clone(),
        pre_dispatch_membership_advances: runtime.pre_dispatch_membership_advances.clone(),
        dispatch_program: runtime.dispatch_program.clone(),
        initial_envelopes: runtime.initial_envelopes.clone(),
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        public_cli_frozen: false,
        run_local_claimed: false,
        run_docker_claimed: false,
    })
}

fn validate_runtime_input(
    package: &PracticalAlpha1Package,
    runtime: &PracticalAlpha1AlphaLocalRuntimeInput,
    path: &Path,
) -> Result<(), PracticalAlpha1RuntimePlanError> {
    let world = package.world.as_ref().ok_or_else(|| {
        malformed_runtime_input(
            path,
            "runtime-plan floor currently requires `package_kind = world` with `world` section"
                .to_string(),
        )
    })?;
    if runtime.queue_kind != "in_process_fifo" {
        return Err(malformed_runtime_input(
            path,
            format!(
                "runtime-plan floor only admits `queue_kind = in_process_fifo`, found `{}`",
                runtime.queue_kind
            ),
        ));
    }
    if runtime.runtime_places.is_empty() {
        return Err(malformed_runtime_input(
            path,
            "runtime-plan floor requires at least one `runtime_places` entry".to_string(),
        ));
    }
    if runtime.bootstrap_participants.is_empty() {
        return Err(malformed_runtime_input(
            path,
            "runtime-plan floor requires at least one bootstrap participant".to_string(),
        ));
    }
    if runtime.initial_envelopes.len() != 1 {
        return Err(malformed_runtime_input(
            path,
            format!(
                "runtime-plan floor currently admits exactly one initial envelope, found {}",
                runtime.initial_envelopes.len()
            ),
        ));
    }

    let package_place_ids: BTreeSet<&str> = package
        .places
        .iter()
        .map(|place| place.id.as_str())
        .collect();
    let mut runtime_place_ids = BTreeSet::new();
    for place in &runtime.runtime_places {
        if place.place_id.trim().is_empty() || place.kind.trim().is_empty() {
            return Err(malformed_runtime_input(
                path,
                "runtime place entries must have non-empty `place_id` and `kind`".to_string(),
            ));
        }
        if !package_place_ids.contains(place.place_id.as_str()) {
            return Err(malformed_runtime_input(
                path,
                format!(
                    "runtime place `{}` must also appear in package `places`",
                    place.place_id
                ),
            ));
        }
        if !runtime_place_ids.insert(place.place_id.as_str()) {
            return Err(malformed_runtime_input(
                path,
                format!("runtime place `{}` appears more than once", place.place_id),
            ));
        }
    }
    if !runtime_place_ids.contains(world.entry_place.as_str()) {
        return Err(malformed_runtime_input(
            path,
            format!(
                "world entry_place `{}` must be present in `runtime_places`",
                world.entry_place
            ),
        ));
    }

    let mut principals = BTreeSet::new();
    for participant in &runtime.bootstrap_participants {
        if participant.principal.trim().is_empty() {
            return Err(malformed_runtime_input(
                path,
                "bootstrap participants must have non-empty `principal`".to_string(),
            ));
        }
        if !principals.insert(participant.principal.as_str()) {
            return Err(malformed_runtime_input(
                path,
                format!(
                    "bootstrap participant `{}` appears more than once",
                    participant.principal
                ),
            ));
        }
    }

    for advance in &runtime.pre_dispatch_membership_advances {
        if advance.principal.trim().is_empty() {
            return Err(malformed_runtime_input(
                path,
                "membership advances must have non-empty `principal`".to_string(),
            ));
        }
        match advance.kind {
            PracticalAlpha1RuntimeMembershipAdvanceKind::AddParticipant => {}
        }
    }

    let envelope = &runtime.initial_envelopes[0];
    if envelope.envelope_id.trim().is_empty() {
        return Err(malformed_runtime_input(
            path,
            "runtime envelope must have non-empty `envelope_id`".to_string(),
        ));
    }
    let expected_from_place = participant_place_for(&envelope.emitter_principal);
    if envelope.from_place != expected_from_place {
        return Err(malformed_runtime_input(
            path,
            format!(
                "runtime envelope from_place `{}` must match participant place `{}`",
                envelope.from_place, expected_from_place
            ),
        ));
    }
    if envelope.principal_claim.participant_place != expected_from_place {
        return Err(malformed_runtime_input(
            path,
            format!(
                "principal_claim participant_place `{}` must match `{}`",
                envelope.principal_claim.participant_place, expected_from_place
            ),
        ));
    }
    if envelope.principal_claim.principal != envelope.emitter_principal {
        return Err(malformed_runtime_input(
            path,
            "principal_claim principal must match emitter_principal".to_string(),
        ));
    }
    if !principals.contains(envelope.emitter_principal.as_str()) {
        return Err(malformed_runtime_input(
            path,
            format!(
                "runtime envelope emitter `{}` must be present in bootstrap participants",
                envelope.emitter_principal
            ),
        ));
    }
    if !runtime_place_ids.contains(envelope.to_place.as_str()) {
        return Err(malformed_runtime_input(
            path,
            format!(
                "runtime envelope destination `{}` must be present in `runtime_places`",
                envelope.to_place
            ),
        ));
    }

    match &runtime.dispatch_program {
        PracticalAlpha1RuntimeDispatchProgram::SugorokuRollHandoff {
            envelope_id,
            handoff_target,
            publication_witness_ref,
            ..
        } => {
            if envelope_id != &envelope.envelope_id {
                return Err(malformed_runtime_input(
                    path,
                    format!(
                        "dispatch program envelope `{}` must match initial envelope `{}`",
                        envelope_id, envelope.envelope_id
                    ),
                ));
            }
            let advance_principals: BTreeSet<&str> = runtime
                .pre_dispatch_membership_advances
                .iter()
                .map(|advance| advance.principal.as_str())
                .collect();
            if !principals.contains(handoff_target.as_str())
                && !advance_principals.contains(handoff_target.as_str())
            {
                return Err(malformed_runtime_input(
                    path,
                    format!(
                        "dispatch program handoff target `{}` must exist in bootstrap or membership advances",
                        handoff_target
                    ),
                ));
            }
            if publication_witness_ref.trim().is_empty() {
                return Err(malformed_runtime_input(
                    path,
                    "dispatch program `publication_witness_ref` must be non-empty".to_string(),
                ));
            }
        }
    }

    Ok(())
}

fn participant_place_for(principal: &str) -> String {
    format!("ParticipantPlace[{principal}]")
}

fn malformed_runtime_input(path: &Path, detail: String) -> PracticalAlpha1RuntimePlanError {
    PracticalAlpha1RuntimePlanError {
        kind: PracticalAlpha1RuntimePlanErrorKind::MalformedRuntimeInput,
        path: path.to_path_buf(),
        detail,
    }
}

fn wrap_checker_error(
    path: &Path,
    error: PracticalAlpha1CheckError,
) -> PracticalAlpha1RuntimePlanError {
    PracticalAlpha1RuntimePlanError {
        kind: PracticalAlpha1RuntimePlanErrorKind::Checker,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE_KIND.to_string()
}

fn runtime_plan_scope() -> String {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
