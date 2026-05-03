use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::practical_alpha1::{
    PracticalAlpha1Package, PracticalAlpha1RuntimeDispatchProgram, PracticalAlpha1RuntimeEnvelope,
    PracticalAlpha1RuntimeMembershipAdvance, PracticalAlpha1SaveLoadScenarioKind,
    load_practical_alpha1_package_path,
};
use crate::practical_alpha1_runtime_plan::{
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE, PracticalAlpha1RuntimePlan,
    PracticalAlpha1RuntimePlanError, build_practical_alpha1_runtime_plan,
};

pub const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SCOPE: &str = "practical-alpha1-save-load-plan-floor";
pub const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_save_load_plan";
pub const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_RETAINED_LATER_REFS: &[&str] = &[
    "distributed_durable_save_load",
    "stale_witness_nonresurrection",
    "stale_lease_nonresurrection",
    "docker_transport_save_load",
    "hotplug_lifecycle_persistence",
    "final_public_save_load_api",
];

const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 save-load plan as a runtime report or persisted ABI",
    "do not treat the practical alpha-1 save-load plan as distributed or durable save-load completion",
    "do not treat CHK-CUT-01 guard reuse as full consistent-cut or Z-cycle completion",
];

const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_LIMITATIONS: &[&str] = &[
    "alpha-local non-final save-load-plan floor only",
    "limited SL practical sample families only",
    "depends on one exact practical local-runtime frontier before save/load execution",
    "no distributed durable save/load, stale witness/lease completion, or final public ABI",
];

const PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_CHECKER_GUARD_REFS: &[&str] = &["CHK-CUT-01"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1SaveLoadPlanErrorKind {
    FrontDoor,
    Checker,
    RejectedByChecker,
    MissingSaveLoadSection,
    MalformedSaveLoadInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1SaveLoadPlanError {
    pub kind: PracticalAlpha1SaveLoadPlanErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1SaveLoadPlanError {
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

impl std::error::Error for PracticalAlpha1SaveLoadPlanError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1SaveLoadPlan {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "save_load_plan_scope")]
    pub save_load_plan_scope: String,
    #[serde(default = "runtime_plan_scope")]
    pub runtime_plan_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub package_kind: String,
    pub world_id: String,
    pub entry_place: String,
    pub queue_kind: String,
    pub scenario_kind: String,
    pub required_base_terminal_outcome: String,
    pub required_saved_owner: String,
    pub required_saved_publish_history_entry: String,
    pub required_saved_history_tail: String,
    pub resumed_dispatch_program: PracticalAlpha1RuntimeDispatchProgram,
    pub resumed_envelope: PracticalAlpha1RuntimeEnvelope,
    #[serde(default)]
    pub post_restore_membership_advances: Vec<PracticalAlpha1RuntimeMembershipAdvance>,
    #[serde(default)]
    pub checker_guard_refs: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
}

pub fn build_practical_alpha1_save_load_plan_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1SaveLoadPlan, PracticalAlpha1SaveLoadPlanError> {
    let path = path.as_ref().to_path_buf();
    let package = load_practical_alpha1_package_path(&path).map_err(|error| {
        PracticalAlpha1SaveLoadPlanError {
            kind: PracticalAlpha1SaveLoadPlanErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        }
    })?;
    build_practical_alpha1_save_load_plan_at_path(&package, &path)
}

pub fn build_practical_alpha1_save_load_plan(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1SaveLoadPlan, PracticalAlpha1SaveLoadPlanError> {
    build_practical_alpha1_save_load_plan_at_path(package, Path::new("<inline>"))
}

fn build_practical_alpha1_save_load_plan_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha1SaveLoadPlan, PracticalAlpha1SaveLoadPlanError> {
    if package.package_kind != "world" {
        return Err(PracticalAlpha1SaveLoadPlanError {
            kind: PracticalAlpha1SaveLoadPlanErrorKind::MalformedSaveLoadInput,
            path: path.to_path_buf(),
            detail: format!(
                "practical save-load plan currently admits only world packages, found `{}`",
                package.package_kind
            ),
        });
    }

    let save_load = package
        .alpha_local_save_load_input
        .as_ref()
        .ok_or_else(|| PracticalAlpha1SaveLoadPlanError {
            kind: PracticalAlpha1SaveLoadPlanErrorKind::MissingSaveLoadSection,
            path: path.to_path_buf(),
            detail: "package does not declare `alpha_local_save_load_input`".to_string(),
        })?;

    let runtime_plan = build_practical_alpha1_runtime_plan(package)
        .map_err(|error| wrap_runtime_plan_error(path, error))?;
    validate_save_load_input(&runtime_plan, save_load, path)?;

    Ok(PracticalAlpha1SaveLoadPlan {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        save_load_plan_scope: save_load_plan_scope(),
        runtime_plan_scope: runtime_plan_scope(),
        sample_id: save_load.sample_id.clone(),
        package_id: package.package_id.clone(),
        package_kind: package.package_kind.clone(),
        world_id: runtime_plan.world_id.clone(),
        entry_place: runtime_plan.entry_place.clone(),
        queue_kind: runtime_plan.queue_kind.clone(),
        scenario_kind: scenario_kind_name(save_load.scenario_kind).to_string(),
        required_base_terminal_outcome: save_load.required_base_terminal_outcome.clone(),
        required_saved_owner: save_load.required_saved_owner.clone(),
        required_saved_publish_history_entry: save_load
            .required_saved_publish_history_entry
            .clone(),
        required_saved_history_tail: save_load.required_saved_history_tail.clone(),
        resumed_dispatch_program: save_load.resumed_dispatch_program.clone(),
        resumed_envelope: save_load.resumed_envelope.clone(),
        post_restore_membership_advances: save_load.post_restore_membership_advances.clone(),
        checker_guard_refs: checker_guard_refs_default(),
        notes: save_load.notes.clone(),
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
    })
}

fn validate_save_load_input(
    runtime_plan: &PracticalAlpha1RuntimePlan,
    save_load: &crate::practical_alpha1::PracticalAlpha1AlphaLocalSaveLoadInput,
    path: &Path,
) -> Result<(), PracticalAlpha1SaveLoadPlanError> {
    if save_load.sample_id.trim().is_empty()
        || save_load.required_base_terminal_outcome.trim().is_empty()
        || save_load.required_saved_owner.trim().is_empty()
        || save_load
            .required_saved_publish_history_entry
            .trim()
            .is_empty()
        || save_load.required_saved_history_tail.trim().is_empty()
    {
        return Err(malformed_save_load_input(
            path,
            "save-load plan requires non-empty sample/base-frontier requirement fields".to_string(),
        ));
    }

    if save_load.required_base_terminal_outcome != "accepted" {
        return Err(malformed_save_load_input(
            path,
            "current practical save-load floor requires required_base_terminal_outcome = accepted"
                .to_string(),
        ));
    }

    let envelope = &save_load.resumed_envelope;
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
    {
        return Err(malformed_save_load_input(
            path,
            "save-load plan requires a fully-populated resumed_envelope".to_string(),
        ));
    }

    if envelope.to_place != runtime_plan.entry_place {
        return Err(malformed_save_load_input(
            path,
            format!(
                "resumed_envelope.to_place `{}` must match runtime entry_place `{}`",
                envelope.to_place, runtime_plan.entry_place
            ),
        ));
    }

    match &save_load.resumed_dispatch_program {
        PracticalAlpha1RuntimeDispatchProgram::SugorokuRollHandoff { envelope_id, .. } => {
            if envelope_id != &envelope.envelope_id {
                return Err(malformed_save_load_input(
                    path,
                    "resumed_dispatch_program envelope_id must match resumed_envelope.envelope_id"
                        .to_string(),
                ));
            }
        }
    }

    if envelope.emitter_principal != save_load.required_saved_owner
        || envelope.principal_claim.principal != save_load.required_saved_owner
    {
        return Err(malformed_save_load_input(
            path,
            "current practical save-load floor requires the saved owner to emit the resumed envelope"
                .to_string(),
        ));
    }

    match save_load.scenario_kind {
        PracticalAlpha1SaveLoadScenarioKind::ResumeOneDispatch => {
            if !save_load.post_restore_membership_advances.is_empty() {
                return Err(malformed_save_load_input(
                    path,
                    "resume_one_dispatch does not admit post_restore_membership_advances"
                        .to_string(),
                ));
            }
            if envelope.dispatch_outcome != "queued_local_dispatch" {
                return Err(malformed_save_load_input(
                    path,
                    "resume_one_dispatch requires resumed_envelope.dispatch_outcome = queued_local_dispatch"
                        .to_string(),
                ));
            }
        }
        PracticalAlpha1SaveLoadScenarioKind::RejectStaleMembership => {
            if save_load.post_restore_membership_advances.is_empty() {
                return Err(malformed_save_load_input(
                    path,
                    "reject_stale_membership requires post_restore_membership_advances".to_string(),
                ));
            }
            if envelope.dispatch_outcome != "queued_local_dispatch" {
                return Err(malformed_save_load_input(
                    path,
                    "reject_stale_membership still requires a queued resumed_envelope; rejection is observed after restore-time membership drift"
                        .to_string(),
                ));
            }
        }
    }

    Ok(())
}

fn wrap_runtime_plan_error(
    path: &Path,
    error: PracticalAlpha1RuntimePlanError,
) -> PracticalAlpha1SaveLoadPlanError {
    let kind = match error.kind {
        crate::practical_alpha1_runtime_plan::PracticalAlpha1RuntimePlanErrorKind::Checker => {
            PracticalAlpha1SaveLoadPlanErrorKind::Checker
        }
        crate::practical_alpha1_runtime_plan::PracticalAlpha1RuntimePlanErrorKind::RejectedByChecker => {
            PracticalAlpha1SaveLoadPlanErrorKind::RejectedByChecker
        }
        crate::practical_alpha1_runtime_plan::PracticalAlpha1RuntimePlanErrorKind::FrontDoor => {
            PracticalAlpha1SaveLoadPlanErrorKind::FrontDoor
        }
        crate::practical_alpha1_runtime_plan::PracticalAlpha1RuntimePlanErrorKind::MissingRuntimeSection
        | crate::practical_alpha1_runtime_plan::PracticalAlpha1RuntimePlanErrorKind::MalformedRuntimeInput => {
            PracticalAlpha1SaveLoadPlanErrorKind::MalformedSaveLoadInput
        }
    };
    PracticalAlpha1SaveLoadPlanError {
        kind,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn malformed_save_load_input(path: &Path, detail: String) -> PracticalAlpha1SaveLoadPlanError {
    PracticalAlpha1SaveLoadPlanError {
        kind: PracticalAlpha1SaveLoadPlanErrorKind::MalformedSaveLoadInput,
        path: path.to_path_buf(),
        detail,
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SCOPE_KIND.to_string()
}

fn save_load_plan_scope() -> String {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_SCOPE.to_string()
}

fn runtime_plan_scope() -> String {
    PRACTICAL_ALPHA1_RUNTIME_PLAN_SCOPE.to_string()
}

fn scenario_kind_name(kind: PracticalAlpha1SaveLoadScenarioKind) -> &'static str {
    match kind {
        PracticalAlpha1SaveLoadScenarioKind::ResumeOneDispatch => "resume_one_dispatch",
        PracticalAlpha1SaveLoadScenarioKind::RejectStaleMembership => "reject_stale_membership",
    }
}

fn checker_guard_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_CHECKER_GUARD_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_SAVE_LOAD_PLAN_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}
