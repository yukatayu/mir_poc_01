//! Private, linear I3 owner-admission handoff between SYS-5 and M8.
//!
//! This module is deliberately narrower than an authority system.  It does
//! not grant membership, capability, witness, time, or a caller-selected
//! operation.  It only carries one already staged SYS-5 owner request through
//! the live SYS-4 fabric into the exact admitted M8 owner plan.

use std::sync::atomic::{AtomicU64, Ordering};

use mir_semantics::surface_v0_classification::OwnerAdmissionBudgetCondition;
use sha2::{Digest, Sha256};

use crate::{m8_runtime_admission::M8OwnerExecutionPlan, m8_runtime_owner_queue::M8OwnerRequest};

// This is an opaque live-fabric instance discriminator, not an authority
// reference or a persisted runtime identity.  It prevents a non-Clone permit
// moved from one otherwise identical local runtime from being accepted by
// another live fabric in this process.
static NEXT_I3_OWNER_ADMISSION_GATE_INSTANCE: AtomicU64 = AtomicU64::new(1);

/// One live SYS-4 verifier.  Checked-patch candidates intentionally do not
/// copy it, and generic M8 runtime snapshots never contain it.
pub(crate) struct M8I3OwnerAdmissionGate {
    instance: u64,
    runtime_binding_ref: String,
}

/// Exact private material retained in the one SYS-5 ledger entry.  It has no
/// identity-hash shortcut: carrier binding is compared as its canonical bytes
/// at both issue and live-fabric verification.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct M8I3OwnerAdmissionBinding {
    runtime_binding_ref: String,
    semantic_request_identity_ref: String,
    envelope_id: String,
    request_id: String,
    request_carrier_id: String,
    operation_id: String,
    request_edge_ref: String,
    reply_edge_ref: String,
    requester_locus: String,
    owner_locus: String,
    core_ref: String,
    owner_lineage_ref: String,
    carrier_snapshot_binding_bytes: Vec<u8>,
    condition: OwnerAdmissionBudgetCondition,
    generation_ref: String,
}

/// A staged, non-Clone issuance token.  It exists only in an Awaiting ledger
/// sidecar and is consumed exactly once when that entry becomes ServeReserved.
pub(crate) struct M8I3OwnerAdmissionIssuance {
    gate_instance: u64,
    binding: M8I3OwnerAdmissionBinding,
}

/// One unverified linear permit in the SYS-5 Reserved handoff.  M8 never
/// accepts this type directly.
pub(crate) struct M8I3OwnerAdmissionPermit {
    gate_instance: u64,
    binding: M8I3OwnerAdmissionBinding,
}

/// The only type accepted by the lower M8 enqueue path.  It is constructed
/// solely after the live SYS-4 verifier rechecks the actual fabric, binding,
/// and current generation immediately before backend handoff.
pub(crate) struct M8I3VerifiedOwnerAdmissionHandoff {
    binding: M8I3OwnerAdmissionBinding,
}

/// A gate-produced declared expiry.  Unlike a serve permit it cannot enter
/// M8; it can only be consumed once by SYS-4 to construct the existing typed
/// owner-reply carrier after the owner ledger retained `Expired`.
pub(crate) struct M8I3DeclaredOwnerDeadlineExpired {
    binding: M8I3OwnerAdmissionBinding,
    decision_commitment_ref: String,
}

impl M8I3OwnerAdmissionGate {
    pub(crate) fn install_for_runtime(runtime_binding_ref: &str) -> Option<Self> {
        if runtime_binding_ref.is_empty() {
            return None;
        }
        let instance = NEXT_I3_OWNER_ADMISSION_GATE_INSTANCE
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .ok()?;
        (instance != 0).then_some(Self {
            instance,
            runtime_binding_ref: runtime_binding_ref.to_string(),
        })
    }

    pub(crate) fn stage_issuance(
        &self,
        binding: M8I3OwnerAdmissionBinding,
    ) -> Option<M8I3OwnerAdmissionIssuance> {
        binding
            .is_well_formed_for_runtime(&self.runtime_binding_ref)
            .then_some(M8I3OwnerAdmissionIssuance {
                gate_instance: self.instance,
                binding,
            })
    }

    /// Private live-instance discriminator shared with SYS-5's trusted
    /// clock/Awaiting/Reserved controls.  It is not an observer reference,
    /// semantic identity, or persisted capability.
    pub(crate) const fn live_instance(&self) -> u64 {
        self.instance
    }

    fn can_issue_after_current_revalidation(
        &self,
        issuance: &M8I3OwnerAdmissionIssuance,
        current: &M8I3OwnerAdmissionBinding,
    ) -> bool {
        issuance.gate_instance == self.instance
            && issuance.binding.matches_immutable_request_binding(current)
            && current.is_well_formed_for_runtime(&self.runtime_binding_ref)
    }

    fn issue_after_current_revalidation(
        &self,
        issuance: M8I3OwnerAdmissionIssuance,
        current: M8I3OwnerAdmissionBinding,
    ) -> Option<M8I3OwnerAdmissionPermit> {
        self.can_issue_after_current_revalidation(&issuance, &current)
            .then_some(M8I3OwnerAdmissionPermit {
                gate_instance: self.instance,
                binding: current,
            })
    }

    fn can_declare_deadline_expired(
        &self,
        issuance: &M8I3OwnerAdmissionIssuance,
        current: &M8I3OwnerAdmissionBinding,
        start_tick: u64,
        resolution_tick: u64,
    ) -> bool {
        self.can_issue_after_current_revalidation(issuance, current)
            && issuance
                .binding
                .condition
                .budget_ticks()
                .checked_add(start_tick)
                .is_some_and(|deadline_tick| resolution_tick >= deadline_tick)
    }

    fn declare_deadline_expired(
        &self,
        issuance: M8I3OwnerAdmissionIssuance,
        current: M8I3OwnerAdmissionBinding,
        start_tick: u64,
        resolution_tick: u64,
    ) -> Option<M8I3DeclaredOwnerDeadlineExpired> {
        self.can_declare_deadline_expired(&issuance, &current, start_tick, resolution_tick)
            .then(|| {
                let deadline_tick = start_tick
                    .checked_add(issuance.binding.condition.budget_ticks())
                    .expect("checked by can_declare_deadline_expired");
                let decision_commitment_ref = owner_admission_deadline_expiry_commitment_ref(
                    &current,
                    start_tick,
                    deadline_tick,
                    resolution_tick,
                );
                M8I3DeclaredOwnerDeadlineExpired {
                    binding: current,
                    decision_commitment_ref,
                }
            })
    }

    fn verify_for_live_fabric(
        &self,
        permit: M8I3OwnerAdmissionPermit,
        current: M8I3OwnerAdmissionBinding,
    ) -> Option<M8I3VerifiedOwnerAdmissionHandoff> {
        (permit.gate_instance == self.instance
            && permit.binding == current
            && current.is_well_formed_for_runtime(&self.runtime_binding_ref))
        .then_some(M8I3VerifiedOwnerAdmissionHandoff { binding: current })
    }
}

impl M8I3OwnerAdmissionBinding {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        runtime_binding_ref: String,
        semantic_request_identity_ref: String,
        envelope_id: String,
        request_id: String,
        request_carrier_id: String,
        operation_id: String,
        request_edge_ref: String,
        reply_edge_ref: String,
        requester_locus: String,
        owner_locus: String,
        core_ref: String,
        owner_lineage_ref: String,
        carrier_snapshot_binding_bytes: Vec<u8>,
        condition: OwnerAdmissionBudgetCondition,
        generation_ref: String,
    ) -> Self {
        Self {
            runtime_binding_ref,
            semantic_request_identity_ref,
            envelope_id,
            request_id,
            request_carrier_id,
            operation_id,
            request_edge_ref,
            reply_edge_ref,
            requester_locus,
            owner_locus,
            core_ref,
            owner_lineage_ref,
            carrier_snapshot_binding_bytes,
            condition,
            generation_ref,
        }
    }

    fn is_well_formed_for_runtime(&self, runtime_binding_ref: &str) -> bool {
        self.runtime_binding_ref == runtime_binding_ref
            && !self.semantic_request_identity_ref.is_empty()
            && !self.envelope_id.is_empty()
            && !self.request_id.is_empty()
            && !self.request_carrier_id.is_empty()
            && !self.operation_id.is_empty()
            && !self.request_edge_ref.is_empty()
            && !self.reply_edge_ref.is_empty()
            && !self.requester_locus.is_empty()
            && !self.owner_locus.is_empty()
            && !self.core_ref.is_empty()
            && !self.owner_lineage_ref.is_empty()
            && !self.carrier_snapshot_binding_bytes.is_empty()
            && !self.generation_ref.is_empty()
            && self.condition.owner_locus().as_str() == self.owner_locus
    }

    /// Stage generation is intentionally excluded here.  The request is
    /// still bound exactly to its runtime, identity, carrier bytes,
    /// source/Core/edge/owner contract and checked budget; current M9
    /// authority has already been revalidated by SYS-5.  The permit created
    /// after this comparison records the live generation and final SYS-4
    /// handoff requires full equality again.
    fn matches_immutable_request_binding(&self, other: &Self) -> bool {
        self.runtime_binding_ref == other.runtime_binding_ref
            && self.semantic_request_identity_ref == other.semantic_request_identity_ref
            && self.envelope_id == other.envelope_id
            && self.request_id == other.request_id
            && self.request_carrier_id == other.request_carrier_id
            && self.operation_id == other.operation_id
            && self.request_edge_ref == other.request_edge_ref
            && self.reply_edge_ref == other.reply_edge_ref
            && self.requester_locus == other.requester_locus
            && self.owner_locus == other.owner_locus
            && self.core_ref == other.core_ref
            && self.owner_lineage_ref == other.owner_lineage_ref
            && self.carrier_snapshot_binding_bytes == other.carrier_snapshot_binding_bytes
            && self.condition == other.condition
    }
}

impl M8I3OwnerAdmissionIssuance {
    /// Check whether this staged token can become one permit after SYS-5 has
    /// revalidated current M9 authority.  This borrows rather than consumes
    /// the token so known mismatch rejects before ServeReserved is committed.
    pub(crate) fn is_issuable_after_current_revalidation(
        &self,
        gate: &M8I3OwnerAdmissionGate,
        current: &M8I3OwnerAdmissionBinding,
    ) -> bool {
        gate.can_issue_after_current_revalidation(self, current)
    }

    /// Consume this exact staged token against the fresh live binding after
    /// its non-consuming feasibility check.  Only staging generation may
    /// differ; the returned permit records the live generation.
    pub(crate) fn issue_after_current_revalidation(
        self,
        gate: &M8I3OwnerAdmissionGate,
        current: M8I3OwnerAdmissionBinding,
    ) -> Option<M8I3OwnerAdmissionPermit> {
        gate.issue_after_current_revalidation(self, current)
    }

    /// Borrow-only expiry feasibility check used before SYS-5 commits the
    /// retained `Expired` disposition.  It does not consume the sole
    /// issuance or manufacture a reply.
    pub(crate) fn can_declare_deadline_expired(
        &self,
        gate: &M8I3OwnerAdmissionGate,
        current: &M8I3OwnerAdmissionBinding,
        start_tick: u64,
        resolution_tick: u64,
    ) -> bool {
        gate.can_declare_deadline_expired(self, current, start_tick, resolution_tick)
    }

    /// Consume the sole issuance into an opaque expiry decision.  SYS-4 can
    /// consume that decision only to build the existing owner-reply carrier;
    /// it is neither a permit nor a reusable failure factory.
    pub(crate) fn declare_deadline_expired(
        self,
        gate: &M8I3OwnerAdmissionGate,
        current: M8I3OwnerAdmissionBinding,
        start_tick: u64,
        resolution_tick: u64,
    ) -> Option<M8I3DeclaredOwnerDeadlineExpired> {
        gate.declare_deadline_expired(self, current, start_tick, resolution_tick)
    }
}

impl M8I3OwnerAdmissionPermit {
    /// The live SYS-4 fabric owns the only promotion from unverified permit
    /// to the lower M8 handoff.  A direct M8 call cannot use this type.
    pub(crate) fn verify_for_live_fabric(
        self,
        gate: &M8I3OwnerAdmissionGate,
        current: M8I3OwnerAdmissionBinding,
    ) -> Option<M8I3VerifiedOwnerAdmissionHandoff> {
        gate.verify_for_live_fabric(self, current)
    }
}

impl M8I3VerifiedOwnerAdmissionHandoff {
    /// Check the exact envelope dequeued by SYS-4 before allowing the
    /// verified handoff to preserve its sealed historical lineage across an
    /// otherwise-authorized generation change.  The full carrier snapshot was
    /// compared immediately before this envelope was moved into the empty
    /// owner mailbox; this prevents a different queued request from consuming
    /// the handoff while keeping the current generation explicit here.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn certifies_exact_dequeued_owner_envelope(
        &self,
        envelope_id: &str,
        request_id: &str,
        request_carrier_id: &str,
        operation_id: &str,
        request_edge_ref: &str,
        requester_locus: &str,
        owner_locus: &str,
        core_ref: &str,
        owner_lineage_ref: &str,
        current_generation_ref: &str,
    ) -> bool {
        self.binding.envelope_id == envelope_id
            && self.binding.request_id == request_id
            && self.binding.request_carrier_id == request_carrier_id
            && self.binding.operation_id == operation_id
            && self.binding.request_edge_ref == request_edge_ref
            && self.binding.requester_locus == requester_locus
            && self.binding.owner_locus == owner_locus
            && self.binding.core_ref == core_ref
            && self.binding.owner_lineage_ref == owner_lineage_ref
            && self.binding.generation_ref == current_generation_ref
    }

    /// Consume this handoff only for the exact admitted M8 plan selected by
    /// the retained generated carrier.  A plan mismatch drops the handoff and
    /// leaves the SYS-5 reservation terminally ServeReserved.
    pub(crate) fn consumes_exact_owner_plan(
        self,
        plan: &M8OwnerExecutionPlan,
        request: &M8OwnerRequest,
    ) -> bool {
        self.binding.operation_id == request.evaluation()
            && self.binding.operation_id == plan.evaluation()
            && self.binding.owner_locus == plan.owner_locus()
            && plan.owner_admission_budget() == Some(&self.binding.condition)
    }
}

impl M8I3DeclaredOwnerDeadlineExpired {
    /// SYS-4 uses this equality check before constructing the generated reply
    /// carrier from an exact retained request.  It exposes neither the
    /// issuance nor the checked deadline fields as a caller capability.
    pub(crate) fn certifies_exact_current_binding(
        &self,
        current: &M8I3OwnerAdmissionBinding,
    ) -> bool {
        self.binding.eq(current)
    }

    pub(crate) fn decision_generation_ref(&self) -> &str {
        &self.binding.generation_ref
    }

    pub(crate) fn decision_commitment_ref(&self) -> &str {
        &self.decision_commitment_ref
    }
}

fn owner_admission_deadline_expiry_commitment_ref(
    binding: &M8I3OwnerAdmissionBinding,
    start_tick: u64,
    deadline_tick: u64,
    resolution_tick: u64,
) -> String {
    let mut hasher = Sha256::new();
    let condition_source_span = format!("{:?}", binding.condition.source_span());
    let condition_source_ref = format!("{:?}", binding.condition.source_ref());
    hasher.update(b"mirrorea/i3/owner-admission/deadline-expired/v1\0");
    for component in [
        binding.runtime_binding_ref.as_bytes(),
        binding.semantic_request_identity_ref.as_bytes(),
        binding.envelope_id.as_bytes(),
        binding.request_id.as_bytes(),
        binding.request_carrier_id.as_bytes(),
        binding.operation_id.as_bytes(),
        binding.request_edge_ref.as_bytes(),
        binding.reply_edge_ref.as_bytes(),
        binding.requester_locus.as_bytes(),
        binding.owner_locus.as_bytes(),
        binding.core_ref.as_bytes(),
        binding.owner_lineage_ref.as_bytes(),
        &binding.carrier_snapshot_binding_bytes,
        binding.generation_ref.as_bytes(),
        binding.condition.owner_locus().as_str().as_bytes(),
        binding.condition.clock_domain().as_str().as_bytes(),
        condition_source_span.as_bytes(),
        condition_source_ref.as_bytes(),
    ] {
        hasher.update((component.len() as u64).to_be_bytes());
        hasher.update(component);
    }
    // Keep the checked condition exact even if two different starts happen
    // to yield the same deadline coordinate.
    hasher.update(binding.condition.budget_ticks().to_be_bytes());
    for tick in [start_tick, deadline_tick, resolution_tick] {
        hasher.update(tick.to_be_bytes());
    }
    format!(
        "sys5-i3-owner-admission-deadline-expired-sha256-v1:{:x}",
        hasher.finalize()
    )
}
