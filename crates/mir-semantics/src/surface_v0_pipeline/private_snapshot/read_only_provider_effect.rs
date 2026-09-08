use super::{SnapshotError, SnapshotSourceRef};
use crate::surface_v0_provider_effect::CheckedReadOnlyProviderEffectCore;

/// Private process-image DTO for the fixed checked provider-effect Core.
/// This is neither a transport message nor an effect-use authority.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotReadOnlyProviderEffectCore {
    pub operation: String,
    pub requester_principal: String,
    pub requester_locus: String,
    pub executor_locus: String,
    pub result_consumer_locus: String,
    pub adapter_profile: String,
    pub logical_resource_slot: String,
    pub result_type: String,
    pub observation_label: String,
    pub max_bytes: u64,
    pub max_calls: u64,
    pub declared_failures: Vec<String>,
    pub source_ref: SnapshotSourceRef,
}

impl SnapshotReadOnlyProviderEffectCore {
    pub fn from_checked(core: &CheckedReadOnlyProviderEffectCore) -> Self {
        Self {
            operation: core.operation().to_string(),
            requester_principal: core.requester_principal().to_string(),
            requester_locus: core.requester_locus().to_string(),
            executor_locus: core.executor_locus().to_string(),
            result_consumer_locus: core.result_consumer_locus().to_string(),
            adapter_profile: core.adapter_profile().as_str().to_string(),
            logical_resource_slot: core.logical_resource_slot().to_string(),
            result_type: core.result_type().to_string(),
            observation_label: core.observation_label().to_string(),
            max_bytes: core.allowance().max_bytes(),
            max_calls: core.allowance().max_calls(),
            declared_failures: core.declared_failures().to_vec(),
            source_ref: SnapshotSourceRef::from_checked(core.source_ref()),
        }
    }

    pub fn into_checked(self) -> Result<CheckedReadOnlyProviderEffectCore, SnapshotError> {
        CheckedReadOnlyProviderEffectCore::from_private_snapshot(
            self.operation,
            self.requester_principal,
            self.requester_locus,
            self.executor_locus,
            self.result_consumer_locus,
            self.adapter_profile,
            self.logical_resource_slot,
            self.result_type,
            self.observation_label,
            self.max_bytes,
            self.max_calls,
            self.declared_failures,
            self.source_ref.into_checked()?,
        )
        .ok_or(SnapshotError::StructuralMismatch {
            reason: "invalid checked read-only provider effect core",
        })
    }
}
