use std::{collections::BTreeMap, fs, path::Path};

use serde::{Deserialize, Serialize};

pub const POSEGRAPH_RUNTIME_SURFACE_KIND: &str = "posegraph_runtime_report";
pub const POSEGRAPH_RUNTIME_SCOPE: &str = "full-system-v1-posegraph-runtime-v0";
pub const POSEGRAPH_DEVTOOLS_SURFACE_KIND: &str = "posegraph_runtime_devtools_export";
pub const POSEGRAPH_DEVTOOLS_SCOPE: &str = "full-system-v1-posegraph-devtools-v0";
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
pub struct PoseGraphSaveLoadState {
    pub savepoint_ref: String,
    pub saved_pose_snapshot_frontier: String,
    pub restored_pose_snapshot_frontier: String,
    pub saved_membership_epoch: u64,
    pub restored_membership_epoch: Option<u64>,
    pub saved_owner_epoch: u64,
    pub restored_owner_epoch: Option<u64>,
    pub saved_anchor_switch_sequence: u64,
    pub restored_anchor_switch_sequence: Option<u64>,
    pub saved_anchor_witness: Option<String>,
    pub restored_anchor_witness: Option<String>,
    pub saved_active_anchor: Option<String>,
    pub restored_active_anchor: Option<String>,
    pub load_admissible: bool,
    pub state_roundtrip_equal: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsPanel {
    pub panel_id: String,
    pub panel_kind: String,
    pub label: String,
    pub authority: String,
    pub redaction: String,
    pub retention_scope: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsNodeRow {
    pub entity_ref: String,
    pub pose_version: u64,
    pub pose_snapshot_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsAnchorEdge {
    pub entity_ref: String,
    pub anchor_ref: String,
    pub binding_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsSnapshotEntry {
    pub snapshot_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsNoSplitFrameRow {
    pub outcome: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsFallbackEntry {
    pub entity_ref: String,
    pub active_anchor: String,
    pub fallback_targets: Vec<String>,
    pub binding_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsStaleReacquireEvent {
    pub entity_ref: String,
    pub event_kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsSections {
    pub node_list: Vec<PoseGraphDevtoolsNodeRow>,
    pub anchor_edges: Vec<PoseGraphDevtoolsAnchorEdge>,
    pub pose_snapshot_timeline: Vec<PoseGraphDevtoolsSnapshotEntry>,
    pub pose_versions: Vec<PoseGraphPoseVersion>,
    pub no_split_frame_rows: Vec<PoseGraphDevtoolsNoSplitFrameRow>,
    pub fallback_degradation: Vec<PoseGraphDevtoolsFallbackEntry>,
    pub stale_reacquire_events: Vec<PoseGraphDevtoolsStaleReacquireEvent>,
    pub redacted_transform_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoseGraphDevtoolsExport {
    #[serde(default = "posegraph_devtools_surface_kind")]
    pub surface_kind: String,
    #[serde(default = "posegraph_devtools_scope")]
    pub devtools_scope: String,
    pub observer_authority: String,
    pub redaction_policy: String,
    pub retention_scope: String,
    pub panel_ids: Vec<String>,
    pub panels: Vec<PoseGraphDevtoolsPanel>,
    pub sections: PoseGraphDevtoolsSections,
    pub final_public_viewer_frozen: bool,
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
    pub save_load_state: Option<PoseGraphSaveLoadState>,
    pub devtools_export: PoseGraphDevtoolsExport,
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
    #[serde(default)]
    save_load: Option<PoseGraphSaveLoadInput>,
}

#[derive(Debug, Clone, Deserialize)]
struct PoseGraphSaveLoadInput {
    savepoint_ref: String,
    saved_pose_snapshot_frontier: String,
    saved_membership_epoch: u64,
    saved_owner_epoch: u64,
    saved_anchor_switch_sequence: u64,
    #[serde(default)]
    saved_anchor_witness: Option<String>,
    #[serde(default)]
    saved_active_anchor: Option<String>,
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
                None,
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
                None,
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
            None,
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
            None,
            "unsupported_posegraph_package_kind",
            format!(
                "expected package kind `{PACKAGE_KIND}`, found `{}`",
                package.package_kind
            ),
        );
    }

    let mut runtime_state = build_runtime_state(&package.runtime_input.posegraph);
    let save_load_state = build_save_load_state(&package.runtime_input.posegraph, &runtime_state);

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
                    save_load_state,
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
                    save_load_state,
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
                    save_load_state.clone(),
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
                    save_load_state.clone(),
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
                save_load_state.clone(),
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
                save_load_state.clone(),
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
            let devtools_export = build_devtools_export(
                &runtime_state,
                save_load_state.as_ref(),
                "violation_export",
                Some("no_split_frame"),
                None,
            );
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
                save_load_state: save_load_state.clone(),
                devtools_export,
                observer_safe_summary: "posegraph violation export: no_split_frame".to_string(),
                final_public_api_frozen: false,
            };
        }
    }

    if let Some(save_load_state) = save_load_state.as_ref() {
        if !save_load_state.load_admissible {
            return rejection_report(
                &package_path_text,
                &package.package_id,
                &package.module_id,
                &package.transition_id,
                runtime_state,
                Some(save_load_state.clone()),
                "save_load_inadmissible",
                save_load_inadmissibility_detail(save_load_state),
            );
        }
    }

    let observer_safe_summary = format!(
        "accepted posegraph runtime {}.{}",
        package.module_id, package.transition_id
    );
    let devtools_export = build_devtools_export(
        &runtime_state,
        save_load_state.as_ref(),
        "accepted",
        None,
        None,
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
        save_load_state: save_load_state.clone(),
        devtools_export,
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

fn build_save_load_state(
    input: &PoseGraphInput,
    runtime_state: &PoseGraphRuntimeState,
) -> Option<PoseGraphSaveLoadState> {
    let save_load = input.save_load.as_ref()?;
    let restored_membership_epoch = input.current_membership_epoch.or_else(|| {
        input
            .anchored_pose
            .as_ref()
            .and_then(|anchored_pose| anchored_pose.membership_epoch)
    });
    let restored_owner_epoch = input.current_owner_epoch.or_else(|| {
        input
            .anchored_pose
            .as_ref()
            .and_then(|anchored_pose| anchored_pose.owner_epoch)
    });
    let restored_anchor_switch_sequence = input.anchor_switch_log.last().map(|row| row.sequence);
    let restored_anchor_witness = input.current_anchor_witness.clone();
    let restored_active_anchor = input
        .anchored_pose
        .as_ref()
        .map(|anchored_pose| anchored_pose.anchor_ref.clone());
    let mut mismatch_notes = Vec::new();
    if save_load.saved_pose_snapshot_frontier != runtime_state.pose_snapshot_frontier {
        mismatch_notes.push(format!(
            "saved_pose_snapshot_frontier {} does not match restored {}",
            save_load.saved_pose_snapshot_frontier, runtime_state.pose_snapshot_frontier
        ));
    }
    if Some(save_load.saved_membership_epoch) != restored_membership_epoch {
        mismatch_notes.push(format!(
            "saved_membership_epoch {} does not match restored {:?}",
            save_load.saved_membership_epoch, restored_membership_epoch
        ));
    }
    if Some(save_load.saved_owner_epoch) != restored_owner_epoch {
        mismatch_notes.push(format!(
            "saved_owner_epoch {} does not match restored {:?}",
            save_load.saved_owner_epoch, restored_owner_epoch
        ));
    }
    if Some(save_load.saved_anchor_switch_sequence) != restored_anchor_switch_sequence {
        mismatch_notes.push(format!(
            "saved_anchor_switch_sequence {} does not match restored {:?}",
            save_load.saved_anchor_switch_sequence, restored_anchor_switch_sequence
        ));
    }
    if save_load.saved_anchor_witness != restored_anchor_witness {
        mismatch_notes.push(format!(
            "saved_anchor_witness {:?} does not match restored {:?}",
            save_load.saved_anchor_witness, restored_anchor_witness
        ));
    }
    if save_load.saved_active_anchor != restored_active_anchor {
        mismatch_notes.push(format!(
            "saved_active_anchor {:?} does not match restored {:?}",
            save_load.saved_active_anchor, restored_active_anchor
        ));
    }
    let state_roundtrip_equal = mismatch_notes.is_empty();
    let load_admissible = mismatch_notes.is_empty();
    let mut notes = vec![
        "posegraph save/load state is bounded alpha evidence only".to_string(),
        "distributed durable pose save/load remains out of scope".to_string(),
    ];
    notes.extend(mismatch_notes);

    Some(PoseGraphSaveLoadState {
        savepoint_ref: save_load.savepoint_ref.clone(),
        saved_pose_snapshot_frontier: save_load.saved_pose_snapshot_frontier.clone(),
        restored_pose_snapshot_frontier: runtime_state.pose_snapshot_frontier.clone(),
        saved_membership_epoch: save_load.saved_membership_epoch,
        restored_membership_epoch,
        saved_owner_epoch: save_load.saved_owner_epoch,
        restored_owner_epoch,
        saved_anchor_switch_sequence: save_load.saved_anchor_switch_sequence,
        restored_anchor_switch_sequence,
        saved_anchor_witness: save_load.saved_anchor_witness.clone(),
        restored_anchor_witness,
        saved_active_anchor: save_load.saved_active_anchor.clone(),
        restored_active_anchor,
        load_admissible,
        state_roundtrip_equal,
        notes,
    })
}

fn rejected_save_load_state(
    save_load_state: Option<PoseGraphSaveLoadState>,
    reason: String,
) -> Option<PoseGraphSaveLoadState> {
    save_load_state.map(|mut save_load_state| {
        save_load_state.load_admissible = false;
        save_load_state.state_roundtrip_equal = false;
        save_load_state.notes.push(reason);
        save_load_state
    })
}

fn build_devtools_export(
    runtime_state: &PoseGraphRuntimeState,
    save_load_state: Option<&PoseGraphSaveLoadState>,
    outcome: &str,
    violation_kind: Option<&str>,
    rejection_code: Option<&str>,
) -> PoseGraphDevtoolsExport {
    let panel_ids = vec![
        "posegraph_node_list".to_string(),
        "anchor_edges".to_string(),
        "pose_snapshot_timeline".to_string(),
        "pose_version_view".to_string(),
        "no_split_frame_rows".to_string(),
        "fallback_degradation".to_string(),
        "stale_reacquire_events".to_string(),
        "redacted_transform_summary".to_string(),
    ];
    let panels = vec![
        ("posegraph_node_list", "node_list", "PoseGraph node list"),
        ("anchor_edges", "anchor_edges", "anchor edges"),
        (
            "pose_snapshot_timeline",
            "pose_snapshot_timeline",
            "pose snapshot timeline",
        ),
        ("pose_version_view", "pose_versions", "pose version view"),
        (
            "no_split_frame_rows",
            "no_split_frame",
            "no-split-frame rows",
        ),
        (
            "fallback_degradation",
            "fallback_degradation",
            "fallback degradation",
        ),
        (
            "stale_reacquire_events",
            "stale_reacquire_events",
            "stale/reacquire events",
        ),
        (
            "redacted_transform_summary",
            "redacted_transform_summary",
            "redacted transform summary",
        ),
    ]
    .into_iter()
    .map(|(panel_id, panel_kind, label)| PoseGraphDevtoolsPanel {
        panel_id: panel_id.to_string(),
        panel_kind: panel_kind.to_string(),
        label: label.to_string(),
        authority: "observer_safe_posegraph_runtime".to_string(),
        redaction: "redacted_transform_summary".to_string(),
        retention_scope: "report_local_inventory".to_string(),
        notes: vec!["bounded PoseGraph devtools alpha evidence only".to_string()],
    })
    .collect::<Vec<_>>();

    let mut pose_snapshot_frontiers = BTreeMap::new();
    pose_snapshot_frontiers.insert(runtime_state.pose_snapshot_frontier.clone(), ());
    for node in &runtime_state.nodes {
        pose_snapshot_frontiers.insert(node.pose_snapshot_ref.clone(), ());
    }
    for anchor_switch in &runtime_state.anchor_switch_log {
        pose_snapshot_frontiers.insert(anchor_switch.pose_snapshot_frontier.clone(), ());
    }
    if let Some(save_load_state) = save_load_state {
        pose_snapshot_frontiers.insert(save_load_state.saved_pose_snapshot_frontier.clone(), ());
        pose_snapshot_frontiers.insert(save_load_state.restored_pose_snapshot_frontier.clone(), ());
    }

    let stale_reacquire_events = if !runtime_state.reacquire_required.is_empty() {
        runtime_state
            .reacquire_required
            .iter()
            .map(|entity_ref| PoseGraphDevtoolsStaleReacquireEvent {
                entity_ref: entity_ref.clone(),
                event_kind: "reacquire_required".to_string(),
                detail: rejection_code.unwrap_or("reacquire_required").to_string(),
            })
            .collect::<Vec<_>>()
    } else if let Some(rejection_code) = rejection_code {
        if rejection_code.contains("stale") {
            vec![PoseGraphDevtoolsStaleReacquireEvent {
                entity_ref: "posegraph#stale".to_string(),
                event_kind: "stale_anchor".to_string(),
                detail: rejection_code.to_string(),
            }]
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    PoseGraphDevtoolsExport {
        surface_kind: posegraph_devtools_surface_kind(),
        devtools_scope: posegraph_devtools_scope(),
        observer_authority: "observer_safe_posegraph_runtime".to_string(),
        redaction_policy: "redacted_transform_summary".to_string(),
        retention_scope: "report_local_inventory".to_string(),
        panel_ids,
        panels,
        sections: PoseGraphDevtoolsSections {
            node_list: runtime_state
                .nodes
                .iter()
                .map(|node| PoseGraphDevtoolsNodeRow {
                    entity_ref: node.entity_ref.clone(),
                    pose_version: node.pose_version,
                    pose_snapshot_ref: node.pose_snapshot_ref.clone(),
                })
                .collect::<Vec<_>>(),
            anchor_edges: runtime_state
                .anchor_bindings
                .iter()
                .map(|binding| PoseGraphDevtoolsAnchorEdge {
                    entity_ref: binding.entity_ref.clone(),
                    anchor_ref: binding.anchor_ref.clone(),
                    binding_state: binding.state.clone(),
                })
                .collect::<Vec<_>>(),
            pose_snapshot_timeline: pose_snapshot_frontiers
                .into_keys()
                .map(|snapshot_ref| PoseGraphDevtoolsSnapshotEntry { snapshot_ref })
                .collect::<Vec<_>>(),
            pose_versions: runtime_state.pose_versions.clone(),
            no_split_frame_rows: vec![PoseGraphDevtoolsNoSplitFrameRow {
                outcome: outcome.to_string(),
                detail: violation_kind
                    .or(rejection_code)
                    .unwrap_or("same_snapshot_coherent")
                    .to_string(),
            }],
            fallback_degradation: runtime_state
                .fallback_state
                .iter()
                .map(|fallback_state| PoseGraphDevtoolsFallbackEntry {
                    entity_ref: fallback_state.entity_ref.clone(),
                    active_anchor: fallback_state.active_anchor.clone(),
                    fallback_targets: fallback_state
                        .fallback_chain
                        .iter()
                        .map(|fallback_anchor| fallback_anchor.anchor_ref.clone())
                        .collect::<Vec<_>>(),
                    binding_state: fallback_state.binding_state.clone(),
                })
                .collect::<Vec<_>>(),
            stale_reacquire_events,
            redacted_transform_summary: runtime_state
                .nodes
                .iter()
                .map(|node| {
                    format!(
                        "node:{}@v{} snapshot:{}",
                        node.entity_ref, node.pose_version, node.pose_snapshot_ref
                    )
                })
                .collect::<Vec<_>>(),
        },
        final_public_viewer_frozen: false,
    }
}

fn rejection_report(
    package_path: &str,
    package_id: &str,
    module_id: &str,
    transition_id: &str,
    runtime_state: PoseGraphRuntimeState,
    save_load_state: Option<PoseGraphSaveLoadState>,
    code: &str,
    message: String,
) -> PoseGraphRuntimeReport {
    let save_load_state = rejected_save_load_state(save_load_state, code.to_string());
    let devtools_export = build_devtools_export(
        &runtime_state,
        save_load_state.as_ref(),
        "runtime_rejection",
        None,
        Some(code),
    );
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
        save_load_state: save_load_state.clone(),
        devtools_export,
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

fn posegraph_devtools_surface_kind() -> String {
    POSEGRAPH_DEVTOOLS_SURFACE_KIND.to_string()
}

fn posegraph_devtools_scope() -> String {
    POSEGRAPH_DEVTOOLS_SCOPE.to_string()
}

fn save_load_inadmissibility_detail(save_load_state: &PoseGraphSaveLoadState) -> String {
    let mismatch_notes = save_load_state
        .notes
        .iter()
        .filter(|note| {
            note.as_str() != "posegraph save/load state is bounded alpha evidence only"
                && note.as_str() != "distributed durable pose save/load remains out of scope"
                && note.as_str() != "save_load_inadmissible"
        })
        .cloned()
        .collect::<Vec<_>>();
    if mismatch_notes.is_empty() {
        "posegraph save/load state is not admissible".to_string()
    } else {
        format!(
            "posegraph save/load state is not admissible: {}",
            mismatch_notes.join("; ")
        )
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
