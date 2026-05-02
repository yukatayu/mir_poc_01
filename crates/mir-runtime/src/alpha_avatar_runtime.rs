use crate::hotplug_runtime::{
    HotPlugRuntimeSkeletonReport, assemble_hotplug_runtime_skeleton_report,
};
use mirrorea_core::{HotPlugRequest, HotPlugVerdict, LogicalPlaceRuntimeShell, MirroreaCoreError};
use serde::Serialize;

const AVATAR_ATTACHPOINT: &str = "AttachPoint[AlphaRoom#1::AvatarRuntime]";
const ADMIN_PRINCIPAL: &str = "ExampleAdmin";
const ADMIN_PLACE: &str = "ParticipantPlace[ExampleAdmin]";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RuntimePackageManifest {
    pub package_id: String,
    pub package_kind: String,
    pub version: String,
    pub provider: String,
    pub signature_or_provenance: String,
    pub required_host_capabilities: Vec<String>,
    pub provided_roles: Vec<String>,
    pub provided_effects: Vec<String>,
    pub required_effects: Vec<String>,
    pub failure_row: Vec<String>,
    pub resource_budget: String,
    pub observation_labels: Vec<String>,
    pub retention_policy: String,
    pub sandbox_requirement: String,
    pub native_code_policy: String,
    pub adapter_targets: Vec<String>,
    pub compatibility_claims: Vec<String>,
    pub fallback_representation: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AvatarHostSupport {
    pub available_host_capabilities: Vec<String>,
    pub missing_host_capabilities: Vec<String>,
    pub native_execution_available: bool,
    pub sandbox_runtime_available: bool,
    pub renderer_profile: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PackageAdmissionCheck {
    pub check_name: String,
    pub passed: bool,
    pub observed: String,
    pub reason_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AvatarRepresentationState {
    pub requested_package_id: String,
    pub selected_package_id: Option<String>,
    pub selected_representation: Option<String>,
    pub fallback_applied: bool,
    pub fallback_reason: Option<String>,
    pub active_roles: Vec<String>,
    pub degraded_roles: Vec<String>,
    pub native_execution_performed: bool,
    pub limited_capabilities: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AlphaAvatarRuntimeReport {
    pub runtime_scope: String,
    pub sample_id: String,
    pub sample_name: String,
    pub manifest_field_lanes: Vec<String>,
    pub manifest: RuntimePackageManifest,
    pub abstract_role_inventory: Vec<String>,
    pub host_support: AvatarHostSupport,
    pub hotplug_skeleton: HotPlugRuntimeSkeletonReport,
    pub admission_checks: Vec<PackageAdmissionCheck>,
    pub representation_state: AvatarRepresentationState,
    pub terminal_outcome: String,
    pub reason_family: Option<String>,
    pub rejection_reason_refs: Vec<String>,
    pub retained_later_refs: Vec<String>,
    pub what_it_proves: Vec<String>,
    pub what_it_does_not_prove: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AvatarScenario {
    Av01,
    Av02,
    Av06,
    Av08,
    Av09,
    Hp11,
    Hp12,
    Hp15,
}

impl AvatarScenario {
    fn parse(sample_id: &str) -> Result<Self, MirroreaCoreError> {
        match sample_id {
            "AV-01" => Ok(Self::Av01),
            "AV-02" => Ok(Self::Av02),
            "AV-06" => Ok(Self::Av06),
            "AV-08" => Ok(Self::Av08),
            "AV-09" => Ok(Self::Av09),
            "HP-11" => Ok(Self::Hp11),
            "HP-12" => Ok(Self::Hp12),
            "HP-15" => Ok(Self::Hp15),
            _ => Err(MirroreaCoreError::new(format!(
                "unknown alpha avatar/package sample `{sample_id}`"
            ))),
        }
    }

    fn sample_id(self) -> &'static str {
        match self {
            Self::Av01 => "AV-01",
            Self::Av02 => "AV-02",
            Self::Av06 => "AV-06",
            Self::Av08 => "AV-08",
            Self::Av09 => "AV-09",
            Self::Hp11 => "HP-11",
            Self::Hp12 => "HP-12",
            Self::Hp15 => "HP-15",
        }
    }

    fn sample_name(self) -> &'static str {
        match self {
            Self::Av01 => "placeholder_avatar_runtime",
            Self::Av02 => "custom_mir_avatar_runtime",
            Self::Av06 => "untrusted_native_avatar_rejected",
            Self::Av08 => "runtime_unavailable_placeholder",
            Self::Av09 => "adapter_attempts_undeclared_effect",
            Self::Hp11 => "unsigned_native_package_rejected",
            Self::Hp12 => "signed_overcapability_package_rejected",
            Self::Hp15 => "revoked_signed_package_rejected",
        }
    }

    fn requested_package_id(self) -> &'static str {
        match self {
            Self::Av01 => "RuntimePackage[avatar.placeholder.basic@1.0.0]",
            Self::Av02 => "RuntimePackage[avatar.custom.mir_humanoid@1.0.0]",
            Self::Av06 => "RuntimePackage[avatar.native.untrusted@1.0.0]",
            Self::Av08 => "RuntimePackage[avatar.custom.runtime_missing@1.0.0]",
            Self::Av09 => "RuntimePackage[avatar.adapter.undeclared_effect@1.0.0]",
            Self::Hp11 => "RuntimePackage[native.avatar.unsigned@1.0.0]",
            Self::Hp12 => "RuntimePackage[native.avatar.overcapability@1.2.0]",
            Self::Hp15 => "RuntimePackage[native.avatar.revoked@2.0.0]",
        }
    }

    fn expected_terminal_outcome(self) -> &'static str {
        match self {
            Self::Av01 | Self::Av02 | Self::Av08 => "accepted",
            Self::Av06 | Self::Av09 | Self::Hp11 | Self::Hp12 | Self::Hp15 => "rejected",
        }
    }

    fn requested_package_verdict_kind(self) -> &'static str {
        match self {
            Self::Av08 => "rejected",
            _ => self.expected_terminal_outcome(),
        }
    }

    fn reason_family_if_rejected(self) -> Option<&'static str> {
        match self {
            Self::Av01 | Self::Av02 | Self::Av08 => None,
            Self::Av06 | Self::Hp11 | Self::Hp15 => Some("provenance_policy"),
            Self::Av09 => Some("effect_manifest"),
            Self::Hp12 => Some("capability_policy"),
        }
    }
}

pub fn runtime_package_manifest_lanes() -> Vec<String> {
    [
        "package_id",
        "package_kind",
        "version",
        "provider",
        "signature_or_provenance",
        "required_host_capabilities",
        "provided_roles",
        "provided_effects",
        "required_effects",
        "failure_row",
        "resource_budget",
        "observation_labels",
        "retention_policy",
        "sandbox_requirement",
        "native_code_policy",
        "adapter_targets",
        "compatibility_claims",
        "fallback_representation",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

pub fn build_placeholder_avatar_runtime_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Av01)
}

pub fn build_custom_mir_avatar_runtime_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Av02)
}

pub fn build_untrusted_native_avatar_rejected_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Av06)
}

pub fn build_runtime_unavailable_placeholder_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Av08)
}

pub fn build_adapter_undeclared_effect_rejected_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Av09)
}

pub fn build_unsigned_native_package_rejected_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Hp11)
}

pub fn build_signed_overcapability_package_rejected_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Hp12)
}

pub fn build_revoked_signed_package_rejected_report()
-> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::Hp15)
}

pub fn build_closeout_reports() -> Result<Vec<AlphaAvatarRuntimeReport>, MirroreaCoreError> {
    [
        AvatarScenario::Av01,
        AvatarScenario::Av02,
        AvatarScenario::Av06,
        AvatarScenario::Av08,
        AvatarScenario::Av09,
        AvatarScenario::Hp11,
        AvatarScenario::Hp12,
        AvatarScenario::Hp15,
    ]
    .into_iter()
    .map(build_report_for_scenario)
    .collect()
}

pub fn build_report_by_sample_id(
    sample_id: &str,
) -> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    build_report_for_scenario(AvatarScenario::parse(sample_id)?)
}

fn build_report_for_scenario(
    scenario: AvatarScenario,
) -> Result<AlphaAvatarRuntimeReport, MirroreaCoreError> {
    let shell = build_runtime_shell()?;
    let manifest = manifest_for_scenario(scenario);
    let host_support = host_support_for_scenario(scenario);
    let admission_checks = admission_checks_for_scenario(scenario);
    let request = hotplug_request_for_scenario(scenario);
    let verdict = hotplug_verdict_for_scenario(scenario, &admission_checks);
    let skeleton = assemble_hotplug_runtime_skeleton_report(&shell, request, verdict)?;
    let representation_state = representation_state_for_scenario(scenario);

    Ok(AlphaAvatarRuntimeReport {
        runtime_scope: "alpha_runtime_package_avatar_stage_f_bridge".to_string(),
        sample_id: scenario.sample_id().to_string(),
        sample_name: scenario.sample_name().to_string(),
        manifest_field_lanes: runtime_package_manifest_lanes(),
        manifest,
        abstract_role_inventory: abstract_role_inventory(),
        host_support,
        hotplug_skeleton: skeleton,
        admission_checks,
        representation_state,
        terminal_outcome: scenario.expected_terminal_outcome().to_string(),
        reason_family: scenario.reason_family_if_rejected().map(str::to_string),
        rejection_reason_refs: rejection_reason_refs_for_scenario(scenario),
        retained_later_refs: retained_later_refs(),
        what_it_proves: what_it_proves_for_scenario(scenario),
        what_it_does_not_prove: what_it_does_not_prove(),
        notes: scenario_notes(scenario),
    })
}

fn build_runtime_shell() -> Result<LogicalPlaceRuntimeShell, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place(AVATAR_ATTACHPOINT, "AttachPoint")?;
    shell.add_initial_participant(ADMIN_PRINCIPAL)?;
    Ok(shell)
}

fn manifest_for_scenario(scenario: AvatarScenario) -> RuntimePackageManifest {
    match scenario {
        AvatarScenario::Av01 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "placeholder_avatar_runtime".to_string(),
            version: "1.0.0".to_string(),
            provider: "OpenMirroreaSamples".to_string(),
            signature_or_provenance: "mir_source_bundle".to_string(),
            required_host_capabilities: vec![
                "AttachAvatarRuntime".to_string(),
                "RenderPlaceholderMesh".to_string(),
            ],
            provided_roles: vec![
                "EmbodiedPresence".to_string(),
                "Renderable".to_string(),
                "InteractionTarget".to_string(),
            ],
            provided_effects: vec!["render_placeholder_presence".to_string()],
            required_effects: vec!["publish_presence_delta".to_string()],
            failure_row: vec!["RuntimeUnavailable".to_string()],
            resource_budget: "cpu=low;memory=small".to_string(),
            observation_labels: vec!["avatar.presence.public".to_string()],
            retention_policy: "helper_local_ephemeral".to_string(),
            sandbox_requirement: "none".to_string(),
            native_code_policy: "no_native".to_string(),
            adapter_targets: vec!["placeholder_avatar".to_string()],
            compatibility_claims: vec![
                "alpha_local_only".to_string(),
                "no_final_avatar_abi".to_string(),
            ],
            fallback_representation: "static_capsule_placeholder".to_string(),
        },
        AvatarScenario::Av02 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "custom_mir_avatar_runtime".to_string(),
            version: "1.0.0".to_string(),
            provider: "OpenMirroreaSamples".to_string(),
            signature_or_provenance: "mir_source_bundle".to_string(),
            required_host_capabilities: vec![
                "AttachAvatarRuntime".to_string(),
                "HostMirAvatarVM".to_string(),
            ],
            provided_roles: vec![
                "EmbodiedPresence".to_string(),
                "Renderable".to_string(),
                "Animatable".to_string(),
                "AttachmentProvider".to_string(),
                "ExpressionProvider".to_string(),
                "NetworkedStateParticipant".to_string(),
            ],
            provided_effects: vec![
                "publish_pose_delta".to_string(),
                "emit_avatar_expression".to_string(),
            ],
            required_effects: vec!["publish_pose_delta".to_string()],
            failure_row: vec!["RuntimeUnavailable".to_string(), "AssetMissing".to_string()],
            resource_budget: "cpu=medium;memory=small".to_string(),
            observation_labels: vec![
                "avatar.pose.owner_visible".to_string(),
                "avatar.presence.public".to_string(),
            ],
            retention_policy: "helper_local_ephemeral".to_string(),
            sandbox_requirement: "none".to_string(),
            native_code_policy: "no_native".to_string(),
            adapter_targets: vec!["custom_mir_avatar".to_string()],
            compatibility_claims: vec![
                "alpha_local_only".to_string(),
                "no_vrm_claim".to_string(),
                "no_vrchat_claim".to_string(),
            ],
            fallback_representation: "static_capsule_placeholder".to_string(),
        },
        AvatarScenario::Av06 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "native_avatar_runtime".to_string(),
            version: "1.0.0".to_string(),
            provider: "UnknownBinaryVendor".to_string(),
            signature_or_provenance: "unsigned_native_binary".to_string(),
            required_host_capabilities: vec![
                "AttachAvatarRuntime".to_string(),
                "SpawnSandboxedNativeHelper".to_string(),
            ],
            provided_roles: vec![
                "EmbodiedPresence".to_string(),
                "Renderable".to_string(),
                "Animatable".to_string(),
            ],
            provided_effects: vec!["publish_pose_delta".to_string()],
            required_effects: vec!["publish_pose_delta".to_string()],
            failure_row: vec!["ProviderRejected".to_string()],
            resource_budget: "cpu=medium;memory=medium".to_string(),
            observation_labels: vec!["avatar.presence.public".to_string()],
            retention_policy: "audit_log_required".to_string(),
            sandbox_requirement: "external_process_isolation".to_string(),
            native_code_policy: "trusted_signer_required".to_string(),
            adapter_targets: vec!["native_avatar_runtime".to_string()],
            compatibility_claims: vec!["alpha_local_only".to_string()],
            fallback_representation: "static_capsule_placeholder".to_string(),
        },
        AvatarScenario::Av08 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "custom_mir_avatar_runtime".to_string(),
            version: "1.0.0".to_string(),
            provider: "OpenMirroreaSamples".to_string(),
            signature_or_provenance: "mir_source_bundle".to_string(),
            required_host_capabilities: vec![
                "AttachAvatarRuntime".to_string(),
                "HostMirAvatarVM".to_string(),
            ],
            provided_roles: vec![
                "EmbodiedPresence".to_string(),
                "Renderable".to_string(),
                "Animatable".to_string(),
                "InputConsumer".to_string(),
            ],
            provided_effects: vec!["publish_pose_delta".to_string()],
            required_effects: vec!["publish_pose_delta".to_string()],
            failure_row: vec!["RuntimeUnavailable".to_string()],
            resource_budget: "cpu=medium;memory=small".to_string(),
            observation_labels: vec!["avatar.presence.public".to_string()],
            retention_policy: "helper_local_ephemeral".to_string(),
            sandbox_requirement: "none".to_string(),
            native_code_policy: "no_native".to_string(),
            adapter_targets: vec!["custom_mir_avatar".to_string()],
            compatibility_claims: vec!["alpha_local_only".to_string()],
            fallback_representation: "static_capsule_placeholder".to_string(),
        },
        AvatarScenario::Av09 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "avatar_adapter".to_string(),
            version: "1.0.0".to_string(),
            provider: "OpenMirroreaSamples".to_string(),
            signature_or_provenance: "mir_source_bundle".to_string(),
            required_host_capabilities: vec!["AttachAvatarRuntime".to_string()],
            provided_roles: vec!["EmbodiedPresence".to_string(), "Renderable".to_string()],
            provided_effects: vec!["render_avatar_presence".to_string()],
            required_effects: vec!["emit_external_avatar_telemetry".to_string()],
            failure_row: vec!["ProviderRejected".to_string()],
            resource_budget: "cpu=low;memory=small".to_string(),
            observation_labels: vec!["avatar.presence.public".to_string()],
            retention_policy: "helper_local_ephemeral".to_string(),
            sandbox_requirement: "none".to_string(),
            native_code_policy: "no_native".to_string(),
            adapter_targets: vec!["custom_avatar_adapter".to_string()],
            compatibility_claims: vec!["alpha_local_only".to_string()],
            fallback_representation: "static_capsule_placeholder".to_string(),
        },
        AvatarScenario::Hp11 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "runtime_package".to_string(),
            version: "1.0.0".to_string(),
            provider: "UnknownBinaryVendor".to_string(),
            signature_or_provenance: "unsigned_native_binary".to_string(),
            required_host_capabilities: vec![
                "ManageRuntimePackageRegistry".to_string(),
                "SpawnSandboxedNativeHelper".to_string(),
            ],
            provided_roles: vec!["EffectProvider".to_string()],
            provided_effects: vec!["emit_avatar_audit".to_string()],
            required_effects: vec!["emit_avatar_audit".to_string()],
            failure_row: vec!["ProviderRejected".to_string()],
            resource_budget: "cpu=medium;memory=medium".to_string(),
            observation_labels: vec!["avatar.audit.redacted".to_string()],
            retention_policy: "audit_log_required".to_string(),
            sandbox_requirement: "external_process_isolation".to_string(),
            native_code_policy: "trusted_signer_required".to_string(),
            adapter_targets: vec!["native_runtime_package".to_string()],
            compatibility_claims: vec!["alpha_local_only".to_string()],
            fallback_representation: "reject_without_attach".to_string(),
        },
        AvatarScenario::Hp12 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "runtime_package".to_string(),
            version: "1.2.0".to_string(),
            provider: "TrustedPluginVendor".to_string(),
            signature_or_provenance: "trusted_signer:TrustedPluginVendor".to_string(),
            required_host_capabilities: vec![
                "ManageRuntimePackageRegistry".to_string(),
                "SpawnSandboxedNativeHelper".to_string(),
            ],
            provided_roles: vec!["EffectProvider".to_string()],
            provided_effects: vec!["emit_avatar_audit".to_string()],
            required_effects: vec!["emit_avatar_audit".to_string()],
            failure_row: vec!["ProviderRejected".to_string()],
            resource_budget: "cpu=medium;memory=medium".to_string(),
            observation_labels: vec!["avatar.audit.redacted".to_string()],
            retention_policy: "audit_log_required".to_string(),
            sandbox_requirement: "external_process_isolation".to_string(),
            native_code_policy: "sandbox_limited".to_string(),
            adapter_targets: vec!["native_runtime_package".to_string()],
            compatibility_claims: vec!["alpha_local_only".to_string()],
            fallback_representation: "reject_without_attach".to_string(),
        },
        AvatarScenario::Hp15 => RuntimePackageManifest {
            package_id: scenario.requested_package_id().to_string(),
            package_kind: "runtime_package".to_string(),
            version: "2.0.0".to_string(),
            provider: "TrustedPluginVendor".to_string(),
            signature_or_provenance: "trusted_signer:TrustedPluginVendor revoked=true".to_string(),
            required_host_capabilities: vec![
                "ManageRuntimePackageRegistry".to_string(),
                "SpawnSandboxedNativeHelper".to_string(),
            ],
            provided_roles: vec!["EffectProvider".to_string()],
            provided_effects: vec!["emit_avatar_audit".to_string()],
            required_effects: vec!["emit_avatar_audit".to_string()],
            failure_row: vec!["ProviderRejected".to_string()],
            resource_budget: "cpu=medium;memory=medium".to_string(),
            observation_labels: vec!["avatar.audit.redacted".to_string()],
            retention_policy: "audit_log_required".to_string(),
            sandbox_requirement: "external_process_isolation".to_string(),
            native_code_policy: "sandbox_limited".to_string(),
            adapter_targets: vec!["native_runtime_package".to_string()],
            compatibility_claims: vec!["alpha_local_only".to_string()],
            fallback_representation: "reject_without_attach".to_string(),
        },
    }
}

fn host_support_for_scenario(scenario: AvatarScenario) -> AvatarHostSupport {
    let mut available_host_capabilities = vec![
        "AttachAvatarRuntime".to_string(),
        "ManageRuntimePackageRegistry".to_string(),
        "RenderPlaceholderMesh".to_string(),
    ];
    if matches!(
        scenario,
        AvatarScenario::Av02 | AvatarScenario::Av06 | AvatarScenario::Av09
    ) {
        available_host_capabilities.push("HostMirAvatarVM".to_string());
    }

    let missing_host_capabilities = if scenario == AvatarScenario::Av08 {
        vec!["HostMirAvatarVM".to_string()]
    } else {
        Vec::new()
    };

    AvatarHostSupport {
        available_host_capabilities,
        missing_host_capabilities,
        native_execution_available: false,
        sandbox_runtime_available: false,
        renderer_profile: "alpha_placeholder_renderer".to_string(),
        notes: vec![
            "current alpha floor does not execute native avatar code".to_string(),
            "placeholder/custom role admission is report-local, not final public API".to_string(),
        ],
    }
}

fn hotplug_request_for_scenario(scenario: AvatarScenario) -> HotPlugRequest {
    HotPlugRequest {
        request_id: format!(
            "avatar_package_request#{}",
            scenario.sample_id().to_lowercase()
        ),
        attachpoint_ref: AVATAR_ATTACHPOINT.to_string(),
        patch_ref: scenario.requested_package_id().to_string(),
        operation_kind: "attach".to_string(),
        requesting_principal: ADMIN_PRINCIPAL.to_string(),
        requesting_participant_place: ADMIN_PLACE.to_string(),
        message_envelope_ref: format!("message_envelope#{}", scenario.sample_id().to_lowercase()),
        auth_evidence_ref: None,
        capability_refs: vec![
            "ManageAttachPoint(AlphaRoom#1::AvatarRuntime)".to_string(),
            "InstallRuntimePackage(AlphaRoom#1)".to_string(),
        ],
        witness_refs: vec!["membership_frontier_snapshot#avatar_runtime".to_string()],
        notes: vec![
            "non-public runtime package/avatar admission floor".to_string(),
            "avatar format remains non-core".to_string(),
        ],
    }
}

fn hotplug_verdict_for_scenario(
    scenario: AvatarScenario,
    admission_checks: &[PackageAdmissionCheck],
) -> HotPlugVerdict {
    let compatibility_reason_refs = admission_checks
        .iter()
        .filter(|check| !check.passed)
        .flat_map(|check| check.reason_refs.clone())
        .collect::<Vec<_>>();

    let accepted_refs = vec![
        "attachpoint_registered".to_string(),
        "manifest_checked".to_string(),
        "fallback_visibility_preserved".to_string(),
    ];

    HotPlugVerdict {
        request_ref: format!(
            "avatar_package_request#{}",
            scenario.sample_id().to_lowercase()
        ),
        verdict_kind: scenario.requested_package_verdict_kind().to_string(),
        compatibility_reason_refs: if compatibility_reason_refs.is_empty() {
            accepted_refs
        } else {
            compatibility_reason_refs
        },
        authorization_reason_refs: vec![
            "install_runtime_package_capability_present".to_string(),
            "avatar_runtime_attach_authorized".to_string(),
        ],
        membership_freshness_reason_refs: vec!["membership_frontier_current".to_string()],
        notes: vec![
            "runtime package admission remains runtime-private and non-public".to_string(),
            "activation/detach lifecycle completion remains later".to_string(),
        ],
    }
}

fn admission_checks_for_scenario(scenario: AvatarScenario) -> Vec<PackageAdmissionCheck> {
    let mut checks = vec![
        PackageAdmissionCheck {
            check_name: "manifest_well_formed".to_string(),
            passed: true,
            observed: "required manifest families are present".to_string(),
            reason_refs: Vec::new(),
        },
        PackageAdmissionCheck {
            check_name: "provenance_policy_satisfied".to_string(),
            passed: !matches!(
                scenario,
                AvatarScenario::Av06 | AvatarScenario::Hp11 | AvatarScenario::Hp15
            ),
            observed: match scenario {
                AvatarScenario::Av06 | AvatarScenario::Hp11 => {
                    "unsigned native package".to_string()
                }
                AvatarScenario::Hp15 => "trusted signer present but revoked".to_string(),
                _ => "provenance policy satisfied for current alpha floor".to_string(),
            },
            reason_refs: match scenario {
                AvatarScenario::Av06 => vec!["avatar_native_unsigned".to_string()],
                AvatarScenario::Hp11 => vec!["unsigned_native_package".to_string()],
                AvatarScenario::Hp15 => vec!["revoked_or_stale_signature".to_string()],
                _ => Vec::new(),
            },
        },
        PackageAdmissionCheck {
            check_name: "role_compatibility".to_string(),
            passed: true,
            observed: "provided roles stay within abstract avatar role inventory".to_string(),
            reason_refs: Vec::new(),
        },
        PackageAdmissionCheck {
            check_name: "provided_effects_declared".to_string(),
            passed: scenario != AvatarScenario::Av09,
            observed: match scenario {
                AvatarScenario::Av09 => {
                    "attempted effect `emit_external_avatar_telemetry` is absent from provided_effects"
                        .to_string()
                }
                _ => "provided effects match declared manifest rows".to_string(),
            },
            reason_refs: if scenario == AvatarScenario::Av09 {
                vec!["undeclared_effect:emit_external_avatar_telemetry".to_string()]
            } else {
                Vec::new()
            },
        },
        PackageAdmissionCheck {
            check_name: "native_policy_satisfied".to_string(),
            passed: !matches!(scenario, AvatarScenario::Av06 | AvatarScenario::Hp11),
            observed: match scenario {
                AvatarScenario::Av06 | AvatarScenario::Hp11 => {
                    "trusted signer policy not satisfied".to_string()
                }
                _ => "native policy row does not grant full native execution".to_string(),
            },
            reason_refs: match scenario {
                AvatarScenario::Av06 => vec!["native_policy_requires_trusted_signer".to_string()],
                AvatarScenario::Hp11 => vec!["native_policy_requires_trusted_signer".to_string()],
                _ => Vec::new(),
            },
        },
        PackageAdmissionCheck {
            check_name: "host_support_satisfied".to_string(),
            passed: scenario != AvatarScenario::Av08,
            observed: match scenario {
                AvatarScenario::Av08 => "missing HostMirAvatarVM; fallback selected".to_string(),
                _ => "host support adequate for current alpha floor".to_string(),
            },
            reason_refs: if scenario == AvatarScenario::Av08 {
                vec!["host_capability_missing:HostMirAvatarVM".to_string()]
            } else {
                Vec::new()
            },
        },
        PackageAdmissionCheck {
            check_name: "capability_budget_contained".to_string(),
            passed: scenario != AvatarScenario::Hp12,
            observed: match scenario {
                AvatarScenario::Hp12 => {
                    "manifest requests undeclared capability `ReadRawCameraBuffer`".to_string()
                }
                _ => "capability budget stays within declared alpha host policy".to_string(),
            },
            reason_refs: if scenario == AvatarScenario::Hp12 {
                vec!["over_capability:ReadRawCameraBuffer".to_string()]
            } else {
                Vec::new()
            },
        },
        PackageAdmissionCheck {
            check_name: "membership_freshness_current".to_string(),
            passed: true,
            observed: "membership_epoch=0, member_incarnation=0".to_string(),
            reason_refs: Vec::new(),
        },
        PackageAdmissionCheck {
            check_name: "authorization_satisfied".to_string(),
            passed: true,
            observed: format!("{ADMIN_PRINCIPAL} may install runtime packages at {AVATAR_ATTACHPOINT}"),
            reason_refs: Vec::new(),
        },
        PackageAdmissionCheck {
            check_name: "fallback_representation_explicit".to_string(),
            passed: true,
            observed: "fallback representation is declared explicitly".to_string(),
            reason_refs: Vec::new(),
        },
    ];

    if scenario == AvatarScenario::Hp15 {
        checks.push(PackageAdmissionCheck {
            check_name: "revocation_status_current".to_string(),
            passed: false,
            observed: "revocation ledger marks signer token as stale".to_string(),
            reason_refs: vec!["revoked_or_stale_signature".to_string()],
        });
    }

    checks
}

fn representation_state_for_scenario(scenario: AvatarScenario) -> AvatarRepresentationState {
    match scenario {
        AvatarScenario::Av01 => AvatarRepresentationState {
            requested_package_id: scenario.requested_package_id().to_string(),
            selected_package_id: Some(scenario.requested_package_id().to_string()),
            selected_representation: Some("static_capsule_placeholder".to_string()),
            fallback_applied: false,
            fallback_reason: None,
            active_roles: vec![
                "EmbodiedPresence".to_string(),
                "Renderable".to_string(),
                "InteractionTarget".to_string(),
            ],
            degraded_roles: Vec::new(),
            native_execution_performed: false,
            limited_capabilities: Vec::new(),
            notes: vec![
                "placeholder runtime remains explicit rather than pretending to be a richer format"
                    .to_string(),
            ],
        },
        AvatarScenario::Av02 => AvatarRepresentationState {
            requested_package_id: scenario.requested_package_id().to_string(),
            selected_package_id: Some(scenario.requested_package_id().to_string()),
            selected_representation: Some("mir_script_avatar_runtime".to_string()),
            fallback_applied: false,
            fallback_reason: None,
            active_roles: vec![
                "EmbodiedPresence".to_string(),
                "Renderable".to_string(),
                "Animatable".to_string(),
                "AttachmentProvider".to_string(),
                "ExpressionProvider".to_string(),
                "NetworkedStateParticipant".to_string(),
            ],
            degraded_roles: Vec::new(),
            native_execution_performed: false,
            limited_capabilities: Vec::new(),
            notes: vec!["custom Mir avatar runtime remains alpha-local and non-public".to_string()],
        },
        AvatarScenario::Av08 => AvatarRepresentationState {
            requested_package_id: scenario.requested_package_id().to_string(),
            selected_package_id: Some("RuntimePackage[avatar.placeholder.basic@1.0.0]".to_string()),
            selected_representation: Some("static_capsule_placeholder".to_string()),
            fallback_applied: true,
            fallback_reason: Some("host_capability_missing:HostMirAvatarVM".to_string()),
            active_roles: vec!["EmbodiedPresence".to_string(), "Renderable".to_string()],
            degraded_roles: vec![
                "Animatable".to_string(),
                "InputConsumer".to_string(),
                "ExpressionProvider".to_string(),
            ],
            native_execution_performed: false,
            limited_capabilities: vec!["placeholder_only".to_string()],
            notes: vec![
                "fallback is visible and monotone".to_string(),
                "requested runtime package is rejected before placeholder fallback is attached"
                    .to_string(),
                "no silent re-promotion to full runtime occurs in this cut".to_string(),
            ],
        },
        AvatarScenario::Av06
        | AvatarScenario::Av09
        | AvatarScenario::Hp11
        | AvatarScenario::Hp12
        | AvatarScenario::Hp15 => AvatarRepresentationState {
            requested_package_id: scenario.requested_package_id().to_string(),
            selected_package_id: None,
            selected_representation: None,
            fallback_applied: false,
            fallback_reason: None,
            active_roles: Vec::new(),
            degraded_roles: Vec::new(),
            native_execution_performed: false,
            limited_capabilities: Vec::new(),
            notes: vec![
                "rejected packages never silently attach".to_string(),
                "no native execution occurs in the current alpha cut".to_string(),
            ],
        },
    }
}

fn rejection_reason_refs_for_scenario(scenario: AvatarScenario) -> Vec<String> {
    match scenario {
        AvatarScenario::Av01 | AvatarScenario::Av02 | AvatarScenario::Av08 => Vec::new(),
        AvatarScenario::Av06 => vec![
            "avatar_native_unsigned".to_string(),
            "native_policy_requires_trusted_signer".to_string(),
        ],
        AvatarScenario::Av09 => {
            vec!["undeclared_effect:emit_external_avatar_telemetry".to_string()]
        }
        AvatarScenario::Hp11 => vec![
            "unsigned_native_package".to_string(),
            "native_policy_requires_trusted_signer".to_string(),
        ],
        AvatarScenario::Hp12 => vec!["over_capability:ReadRawCameraBuffer".to_string()],
        AvatarScenario::Hp15 => vec!["revoked_or_stale_signature".to_string()],
    }
}

fn what_it_proves_for_scenario(scenario: AvatarScenario) -> Vec<String> {
    match scenario {
        AvatarScenario::Av01 => vec![
            "placeholder avatar runtime can attach without importing concrete avatar formats into core"
                .to_string(),
            "abstract role inventory remains explicit".to_string(),
        ],
        AvatarScenario::Av02 => vec![
            "custom Mir avatar runtime can attach as a runtime package".to_string(),
            "custom avatar support stays outside core semantics".to_string(),
        ],
        AvatarScenario::Av06 => vec![
            "an untrusted native avatar package is rejected before activation".to_string(),
            "unsigned native provenance is not treated as semantic safety".to_string(),
        ],
        AvatarScenario::Av08 => vec![
            "requested package rejection and placeholder fallback remain explicit".to_string(),
            "fallback remains monotone and does not silently claim full support".to_string(),
        ],
        AvatarScenario::Av09 => vec![
            "undeclared effect widening is rejected at package admission".to_string(),
            "underdeclared adapter metadata does not slip through as implicit behavior".to_string(),
        ],
        AvatarScenario::Hp11 => vec![
            "unsigned native runtime packages are rejected by provenance policy".to_string(),
            "native admission remains explicit and auditable".to_string(),
        ],
        AvatarScenario::Hp12 => vec![
            "signed packages can still be rejected for over-capability".to_string(),
            "signature is provenance only, not blanket permission".to_string(),
        ],
        AvatarScenario::Hp15 => vec![
            "revoked or stale-signed packages are rejected before activation".to_string(),
            "revocation state remains part of the admission policy".to_string(),
        ],
    }
}

fn what_it_does_not_prove() -> Vec<String> {
    vec![
        "no final avatar API".to_string(),
        "no full VRM / VRChat / Unity compatibility".to_string(),
        "no native binary production execution".to_string(),
        "no completed detach / migration lifecycle".to_string(),
        "no final public runtime package ABI".to_string(),
    ]
}

fn scenario_notes(scenario: AvatarScenario) -> Vec<String> {
    let mut notes = vec![
        "non-public runtime package/avatar admission floor".to_string(),
        "avatar formats and engine concepts remain non-core".to_string(),
    ];
    if scenario == AvatarScenario::Av08 {
        notes.push(
            "fallback extends representation availability, not the lifetime of a richer runtime"
                .to_string(),
        );
        notes.push(
            "requested package admission stays rejected even when a placeholder remains available"
                .to_string(),
        );
    }
    if matches!(
        scenario,
        AvatarScenario::Av06 | AvatarScenario::Hp11 | AvatarScenario::Hp12 | AvatarScenario::Hp15
    ) {
        notes.push("native execution is not performed in this alpha cut".to_string());
    }
    notes
}

fn abstract_role_inventory() -> Vec<String> {
    [
        "EmbodiedPresence",
        "Renderable",
        "Animatable",
        "InputConsumer",
        "InteractionTarget",
        "AttachmentProvider",
        "AnchorProvider",
        "ExpressionProvider",
        "EffectProvider",
        "InspectableObject",
        "NetworkedStateParticipant",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn retained_later_refs() -> Vec<String> {
    vec![
        "vrm_full_compatibility".to_string(),
        "vrchat_full_compatibility".to_string(),
        "unity_runtime_integration".to_string(),
        "native_execution_sandbox_realization".to_string(),
        "dependent_detach_runtime".to_string(),
        "final_public_avatar_api".to_string(),
        "final_public_runtime_package_abi".to_string(),
    ]
}
