use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};

pub const POSEGRAPH_RUNTIME_SURFACE_KIND: &str = "posegraph_runtime_report";
pub const POSEGRAPH_RUNTIME_SCOPE: &str = "full-system-v1-posegraph-runtime-v0";
const PACKAGE_SCHEMA_VERSION: &str = "posegraph-runtime-package-v0";
const PACKAGE_KIND: &str = "posegraph_runtime";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoseGraphRuntimeOutcome {
    Accepted,
    ViolationExport,
    RuntimeRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphRuntimeViolation {
    pub violation_kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphRuntimeRejection {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphTransformNode {
    pub entity_ref: String,
    pub pose_version: u64,
    pub pose_snapshot_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphPoseVersion {
    pub entity_ref: String,
    pub pose_version: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphFallbackAnchor {
    pub anchor_ref: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphAnchorBinding {
    pub entity_ref: String,
    pub anchor_ref: String,
    pub pose_version: u64,
    pub pose_snapshot_ref: String,
    pub membership_epoch: Option<u64>,
    pub owner_epoch: Option<u64>,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphAnchorSwitch {
    pub from_anchor: String,
    pub to_anchor: String,
    pub reason: String,
    pub required_capability: String,
    pub membership_epoch: u64,
    pub owner_epoch: u64,
    pub sequence: u64,
    pub pose_snapshot_frontier: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphFallbackState {
    pub entity_ref: String,
    pub active_anchor: String,
    pub fallback_chain: Vec<PoseGraphFallbackAnchor>,
    pub binding_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphRuntimeState {
    pub nodes: Vec<PoseGraphTransformNode>,
    pub anchor_bindings: Vec<PoseGraphAnchorBinding>,
    pub pose_snapshot_frontier: String,
    pub pose_versions: Vec<PoseGraphPoseVersion>,
    pub anchor_switch_log: Vec<PoseGraphAnchorSwitch>,
    pub reacquire_required: Vec<String>,
    pub fallback_state: Vec<PoseGraphFallbackState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphRuntimeReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "runtime_scope")]
    pub runtime_scope: String,
    pub package_path: String,
    pub package_id: String,
    pub module_id: String,
    pub transition_id: String,
    pub accepted: bool,
    pub terminal_outcome: PoseGraphRuntimeOutcome,
    pub violation: Option<PoseGraphRuntimeViolation>,
    pub rejection: Option<PoseGraphRuntimeRejection>,
    pub runtime_state: PoseGraphRuntimeState,
    pub observer_safe_summary: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct PoseGraphRuntimePackage {
    schema_version: String,
    package_id: String,
    package_kind: String,
    module_id: String,
    transition_id: String,
    runtime_input: PoseGraphRuntimeInput,
}

#[derive(Debug, Clone, Deserialize)]
struct PoseGraphRuntimeInput {
    posegraph: PoseGraphInput,
}

#[derive(Debug, Clone, Deserialize)]
struct PoseGraphInput {
    #[serde(default)]
    pose_snapshot_frontier: Option<String>,
    #[serde(default)]
    target_pose: Option<PoseGraphPoseInput>,
    #[serde(default)]
    anchored_pose: Option<PoseGraphAnchoredPoseInput>,
    #[serde(default)]
    fallback_chain: Vec<PoseGraphFallbackAnchor>,
    #[serde(default)]
    anchor_switch_log: Vec<PoseGraphAnchorSwitch>,
    #[serde(default)]
    current_membership_epoch: Option<u64>,
    #[serde(default)]
    current_owner_epoch: Option<u64>,
    #[serde(default)]
    last_anchor_switch_sequence: Option<u64>,
    #[serde(default)]
    fresh_anchor_witness: Option<String>,
    #[serde(default)]
    current_anchor_witness: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PoseGraphPoseInput {
    entity_ref: String,
    pose_version: u64,
    pose_snapshot_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PoseGraphAnchoredPoseInput {
    entity_ref: String,
    anchor_ref: String,
    pose_version: u64,
    pose_snapshot_ref: String,
    #[serde(default)]
    membership_epoch: Option<u64>,
    #[serde(default)]
    owner_epoch: Option<u64>,
    #[serde(default = "stable_binding_state")]
    state: String,
}

pub fn run_posegraph_runtime_package_path(path: impl AsRef<Path>) -> PoseGraphRuntimeReport {
    let package_path = path.as_ref();
    let package_path_text = package_path.display().to_string();
    let payload = match fs::read_to_string(package_path) {
        Ok(payload) => payload,
        Err(error) => {
            return rejection_report(
                &package_path_text,
                "package#unknown",
                "PoseGraph.InvalidPackage",
                "load_posegraph_runtime_package",
                empty_runtime_state(),
                "package_read_failed",
                format!("failed to read posegraph runtime package: {error}"),
            );
        }
    };
    let package = match serde_json::from_str::<PoseGraphRuntimePackage>(&payload) {
        Ok(package) => package,
        Err(error) => {
            return rejection_report(
                &package_path_text,
                "package#unknown",
                "PoseGraph.InvalidPackage",
                "parse_posegraph_runtime_package",
                empty_runtime_state(),
                "invalid_posegraph_package",
                format!("failed to parse posegraph runtime package: {error}"),
            );
        }
    };

    if package.schema_version != PACKAGE_SCHEMA_VERSION {
        return rejection_report(
            &package_path_text,
            &package.package_id,
            &package.module_id,
            &package.transition_id,
            empty_runtime_state(),
            "unsupported_posegraph_package_schema",
            format!(
                "expected schema `{PACKAGE_SCHEMA_VERSION}`, found `{}`",
                package.schema_version
            ),
        );
    }
    if package.package_kind != PACKAGE_KIND {
        return rejection_report(
            &package_path_text,
            &package.package_id,
            &package.module_id,
            &package.transition_id,
            empty_runtime_state(),
            "unsupported_posegraph_package_kind",
            format!(
                "expected package kind `{PACKAGE_KIND}`, found `{}`",
                package.package_kind
            ),
        );
    }

    let mut runtime_state = build_runtime_state(&package.runtime_input.posegraph);

    if let Some(anchored_pose) = package.runtime_input.posegraph.anchored_pose.as_ref() {
        if let Some(current_membership_epoch) =
            package.runtime_input.posegraph.current_membership_epoch
        {
            if anchored_pose.membership_epoch != Some(current_membership_epoch) {
                return rejection_report(
                    &package_path_text,
                    &package.package_id,
                    &package.module_id,
                    &package.transition_id,
                    runtime_state,
                    "stale_anchor_membership_epoch",
                    format!(
                        "anchor binding membership_epoch {:?} does not match current {}",
                        anchored_pose.membership_epoch, current_membership_epoch
                    ),
                );
            }
        }
    }

    if let Some(current_membership_epoch) = package.runtime_input.posegraph.current_membership_epoch
    {
        for anchor_switch in &package.runtime_input.posegraph.anchor_switch_log {
            if anchor_switch.membership_epoch != current_membership_epoch {
                return rejection_report(
                    &package_path_text,
                    &package.package_id,
                    &package.module_id,
                    &package.transition_id,
                    runtime_state,
                    "stale_anchor_membership_epoch",
                    format!(
                        "anchor switch membership_epoch {} does not match current {}",
                        anchor_switch.membership_epoch, current_membership_epoch
                    ),
                );
            }
        }
    }

    let runtime_pose_snapshot_frontier = runtime_state.pose_snapshot_frontier.clone();
    let mut previous_anchor_switch_sequence =
        package.runtime_input.posegraph.last_anchor_switch_sequence;
    for anchor_switch in &package.runtime_input.posegraph.anchor_switch_log {
        if let Some(current_owner_epoch) = package.runtime_input.posegraph.current_owner_epoch {
            if anchor_switch.owner_epoch != current_owner_epoch {
                return rejection_report(
                    &package_path_text,
                    &package.package_id,
                    &package.module_id,
                    &package.transition_id,
                    runtime_state,
                    "anchor_switch_owner_epoch_stale",
                    format!(
                        "anchor switch owner_epoch {} does not match current {}",
                        anchor_switch.owner_epoch, current_owner_epoch
                    ),
                );
            }
        }
        if let Some(previous_sequence) = previous_anchor_switch_sequence {
            if anchor_switch.sequence <= previous_sequence {
                return rejection_report(
                    &package_path_text,
                    &package.package_id,
                    &package.module_id,
                    &package.transition_id,
                    runtime_state,
                    "anchor_switch_frontier_regression",
                    format!(
                        "anchor switch sequence {} must advance beyond {}",
                        anchor_switch.sequence, previous_sequence
                    ),
                );
            }
        }
        if anchor_switch.pose_snapshot_frontier != runtime_pose_snapshot_frontier {
            return rejection_report(
                &package_path_text,
                &package.package_id,
                &package.module_id,
                &package.transition_id,
                runtime_state,
                "anchor_switch_frontier_regression",
                format!(
                    "anchor switch frontier {} does not match runtime {}",
                    anchor_switch.pose_snapshot_frontier, runtime_pose_snapshot_frontier
                ),
            );
        }
        previous_anchor_switch_sequence = Some(anchor_switch.sequence);
    }

    if let Some(anchored_pose) = package.runtime_input.posegraph.anchored_pose.as_ref() {
        if anchored_pose.state == "fallback_only" {
            let fresh_anchor_witness = package
                .runtime_input
                .posegraph
                .fresh_anchor_witness
                .clone()
                .unwrap_or_default();
            let current_anchor_witness = package
                .runtime_input
                .posegraph
                .current_anchor_witness
                .clone()
                .unwrap_or_default();
            runtime_state
                .reacquire_required
                .push(anchored_pose.entity_ref.clone());
            let detail = if fresh_anchor_witness.is_empty() || current_anchor_witness.is_empty() {
                format!(
                    "fallback-only anchor for `{}` requires explicit reacquire with fresh witness",
                    anchored_pose.entity_ref
                )
            } else if fresh_anchor_witness != current_anchor_witness {
                format!(
                    "fallback-only anchor for `{}` requires explicit reacquire",
                    anchored_pose.entity_ref
                )
            } else {
                format!(
                    "fallback-only anchor for `{}` requires an explicit reacquire transition",
                    anchored_pose.entity_ref
                )
            };
            return rejection_report(
                &package_path_text,
                &package.package_id,
                &package.module_id,
                &package.transition_id,
                runtime_state,
                "reacquire_required",
                detail,
            );
        }
    }

    if let (Some(target_pose), Some(anchored_pose)) = (
        package.runtime_input.posegraph.target_pose.as_ref(),
        package.runtime_input.posegraph.anchored_pose.as_ref(),
    ) {
        let mut detail = Vec::new();
        if target_pose.pose_snapshot_ref != anchored_pose.pose_snapshot_ref {
            detail.push(format!(
                "snapshot mismatch: target={}, anchored={}",
                target_pose.pose_snapshot_ref, anchored_pose.pose_snapshot_ref
            ));
        }
        if target_pose.pose_version != anchored_pose.pose_version {
            detail.push(format!(
                "pose version mismatch: target={}, anchored={}",
                target_pose.pose_version, anchored_pose.pose_version
            ));
        }
        if !detail.is_empty() {
            return PoseGraphRuntimeReport {
                surface_kind: surface_kind(),
                runtime_scope: runtime_scope(),
                package_path: package_path_text,
                package_id: package.package_id,
                module_id: package.module_id,
                transition_id: package.transition_id,
                accepted: false,
                terminal_outcome: PoseGraphRuntimeOutcome::ViolationExport,
                violation: Some(PoseGraphRuntimeViolation {
                    violation_kind: "no_split_frame".to_string(),
                    detail: detail.join("; "),
                }),
                rejection: None,
                runtime_state,
                observer_safe_summary: "posegraph violation export: no_split_frame".to_string(),
                final_public_api_frozen: false,
            };
        }
    }

    let observer_safe_summary = format!(
        "accepted posegraph runtime {}.{}",
        package.module_id, package.transition_id
    );

    PoseGraphRuntimeReport {
        surface_kind: surface_kind(),
        runtime_scope: runtime_scope(),
        package_path: package_path_text,
        package_id: package.package_id,
        module_id: package.module_id,
        transition_id: package.transition_id,
        accepted: true,
        terminal_outcome: PoseGraphRuntimeOutcome::Accepted,
        violation: None,
        rejection: None,
        runtime_state,
        observer_safe_summary,
        final_public_api_frozen: false,
    }
}

fn build_runtime_state(input: &PoseGraphInput) -> PoseGraphRuntimeState {
    let mut nodes = BTreeMap::new();
    if let Some(target_pose) = input.target_pose.as_ref() {
        nodes.insert(
            target_pose.entity_ref.clone(),
            PoseGraphTransformNode {
                entity_ref: target_pose.entity_ref.clone(),
                pose_version: target_pose.pose_version,
                pose_snapshot_ref: target_pose.pose_snapshot_ref.clone(),
            },
        );
    }
    if let Some(anchored_pose) = input.anchored_pose.as_ref() {
        nodes.insert(
            anchored_pose.entity_ref.clone(),
            PoseGraphTransformNode {
                entity_ref: anchored_pose.entity_ref.clone(),
                pose_version: anchored_pose.pose_version,
                pose_snapshot_ref: anchored_pose.pose_snapshot_ref.clone(),
            },
        );
    }
    let nodes = nodes.into_values().collect::<Vec<_>>();
    let pose_versions = nodes
        .iter()
        .map(|node| PoseGraphPoseVersion {
            entity_ref: node.entity_ref.clone(),
            pose_version: node.pose_version,
        })
        .collect::<Vec<_>>();

    let anchor_bindings = input
        .anchored_pose
        .as_ref()
        .map(|anchored_pose| {
            vec![PoseGraphAnchorBinding {
                entity_ref: anchored_pose.entity_ref.clone(),
                anchor_ref: anchored_pose.anchor_ref.clone(),
                pose_version: anchored_pose.pose_version,
                pose_snapshot_ref: anchored_pose.pose_snapshot_ref.clone(),
                membership_epoch: anchored_pose.membership_epoch,
                owner_epoch: anchored_pose.owner_epoch,
                state: anchored_pose.state.clone(),
            }]
        })
        .unwrap_or_default();
    let fallback_state = input
        .anchored_pose
        .as_ref()
        .filter(|_| !input.fallback_chain.is_empty())
        .map(|anchored_pose| {
            vec![PoseGraphFallbackState {
                entity_ref: anchored_pose.entity_ref.clone(),
                active_anchor: anchored_pose.anchor_ref.clone(),
                fallback_chain: input.fallback_chain.clone(),
                binding_state: anchored_pose.state.clone(),
            }]
        })
        .unwrap_or_default();

    PoseGraphRuntimeState {
        nodes,
        anchor_bindings,
        pose_snapshot_frontier: resolve_pose_snapshot_frontier(input),
        pose_versions,
        anchor_switch_log: input.anchor_switch_log.clone(),
        reacquire_required: Vec::new(),
        fallback_state,
    }
}

fn resolve_pose_snapshot_frontier(input: &PoseGraphInput) -> String {
    input
        .pose_snapshot_frontier
        .clone()
        .or_else(|| {
            input
                .target_pose
                .as_ref()
                .map(|target_pose| target_pose.pose_snapshot_ref.clone())
        })
        .or_else(|| {
            input
                .anchored_pose
                .as_ref()
                .map(|anchored_pose| anchored_pose.pose_snapshot_ref.clone())
        })
        .or_else(|| {
            input
                .anchor_switch_log
                .last()
                .map(|anchor_switch| anchor_switch.pose_snapshot_frontier.clone())
        })
        .unwrap_or_else(|| "pose_snapshot_frontier#unknown".to_string())
}

fn rejection_report(
    package_path: &str,
    package_id: &str,
    module_id: &str,
    transition_id: &str,
    runtime_state: PoseGraphRuntimeState,
    code: &str,
    message: String,
) -> PoseGraphRuntimeReport {
    PoseGraphRuntimeReport {
        surface_kind: surface_kind(),
        runtime_scope: runtime_scope(),
        package_path: package_path.to_string(),
        package_id: package_id.to_string(),
        module_id: module_id.to_string(),
        transition_id: transition_id.to_string(),
        accepted: false,
        terminal_outcome: PoseGraphRuntimeOutcome::RuntimeRejection,
        violation: None,
        rejection: Some(PoseGraphRuntimeRejection {
            code: code.to_string(),
            message: message.clone(),
        }),
        runtime_state,
        observer_safe_summary: format!("posegraph runtime rejection: {code}"),
        final_public_api_frozen: false,
    }
}

fn empty_runtime_state() -> PoseGraphRuntimeState {
    PoseGraphRuntimeState {
        nodes: Vec::new(),
        anchor_bindings: Vec::new(),
        pose_snapshot_frontier: "pose_snapshot_frontier#unknown".to_string(),
        pose_versions: Vec::new(),
        anchor_switch_log: Vec::new(),
        reacquire_required: Vec::new(),
        fallback_state: Vec::new(),
    }
}

fn stable_binding_state() -> String {
    "stable".to_string()
}

fn surface_kind() -> String {
    POSEGRAPH_RUNTIME_SURFACE_KIND.to_string()
}

fn runtime_scope() -> String {
    POSEGRAPH_RUNTIME_SCOPE.to_string()
}
