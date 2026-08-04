//! Typed M8 patch admission and one-session activation cuts.
//!
//! Candidates are bound to checked and admitted structure.  This boundary
//! deliberately accepts neither raw evaluations nor reconstructed source.

use std::collections::BTreeMap;

use mir_semantics::{
    shared_model::SourceRef,
    surface_v0_pipeline::{CheckedProgramIdentity, CheckedSurfaceV0},
};

use crate::{
    m8_runtime_admission::{M8Runtime, M8RuntimeAdmission, M8RuntimeInstance},
    m8_runtime_authority::{M8AuthorityState, M8PatchActivationAuthorityLookup},
    m8_runtime_designated_value::M8InputReceiptSet,
    m8_runtime_designated_value::{
        M8DesignatedDiagnostics, M8DesignatedEvaluationRequest, M8PublishedDesignatedValue,
    },
    m8_runtime_local_cut::{
        M8LiveFloor, M8LocalCut, M8LocalRestoreDiagnostics, M8LocalRuntime, M8LocalRuntimeSeed,
        M8LocalSavePayload, M8LocalSemanticPayload,
    },
    m8_runtime_owner_queue::{M8EnqueueDiagnostics, M8Occurrence, M8OwnerRequest, M8StateKey},
};

pub use crate::m8_runtime_local_cut::M8LeaseRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchRuntimeSeed {
    owner_ints: BTreeMap<M8StateKey, i64>,
    authority_state: M8AuthorityState,
    live_leases: Vec<M8LeaseRecord>,
    designated_input_receipts: M8InputReceiptSet,
}

impl M8PatchRuntimeSeed {
    pub fn new() -> Self {
        Self {
            owner_ints: BTreeMap::new(),
            authority_state: M8AuthorityState::new(),
            live_leases: Vec::new(),
            designated_input_receipts: M8InputReceiptSet::new(),
        }
    }

    pub fn with_owner_int(mut self, key: M8StateKey, value: i64) -> Self {
        self.owner_ints.insert(key, value);
        self
    }

    pub fn with_authority_state(mut self, authority_state: M8AuthorityState) -> Self {
        self.authority_state = authority_state;
        self
    }

    pub fn with_live_lease(mut self, lease: M8LeaseRecord) -> Self {
        self.live_leases.push(lease);
        self
    }

    pub fn with_designated_input_receipts(
        mut self,
        designated_input_receipts: M8InputReceiptSet,
    ) -> Self {
        self.designated_input_receipts = designated_input_receipts;
        self
    }

    fn into_local_seed(self) -> M8LocalRuntimeSeed {
        let mut seed = M8LocalRuntimeSeed::new()
            .with_authority_state(self.authority_state)
            .with_designated_input_receipts(self.designated_input_receipts);
        for (key, value) in self.owner_ints {
            seed = seed.with_owner_int(key, value);
        }
        for lease in self.live_leases {
            seed = seed.with_live_lease(lease);
        }
        seed
    }
}

impl Default for M8PatchRuntimeSeed {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum M8PatchCandidateState {
    Bound,
    Unknown,
    Stale,
    Unadmitted,
    DeferredToM9,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchCandidate {
    patch_id: String,
    state: M8PatchCandidateState,
    checked: Option<CheckedSurfaceV0>,
    admission: Option<M8RuntimeAdmission>,
    base_program_identity: Option<CheckedProgramIdentity>,
    base_admission: Option<M8RuntimeAdmission>,
    patch_authority: Option<M8PatchAuthorityUse>,
    designated_input_receipts: Option<M8InputReceiptSet>,
    source_ref: SourceRef,
    reason_ref: Option<String>,
}

impl M8PatchCandidate {
    pub fn from_checked_admitted(
        patch_id: impl Into<String>,
        checked: CheckedSurfaceV0,
        admission: M8RuntimeAdmission,
    ) -> Self {
        Self {
            patch_id: patch_id.into(),
            state: M8PatchCandidateState::Bound,
            source_ref: checked.program_identity().root_source_ref().clone(),
            checked: Some(checked),
            admission: Some(admission),
            base_program_identity: None,
            base_admission: None,
            patch_authority: None,
            designated_input_receipts: None,
            reason_ref: None,
        }
    }

    pub fn from_checked_unadmitted(
        patch_id: impl Into<String>,
        checked: CheckedSurfaceV0,
        source_ref: SourceRef,
        reason_ref: impl Into<String>,
    ) -> Self {
        Self {
            patch_id: patch_id.into(),
            state: M8PatchCandidateState::Unadmitted,
            checked: Some(checked),
            admission: None,
            base_program_identity: None,
            base_admission: None,
            patch_authority: None,
            designated_input_receipts: None,
            source_ref,
            reason_ref: Some(reason_ref.into()),
        }
    }

    pub fn from_checked_deferred_to_m9(
        patch_id: impl Into<String>,
        checked: CheckedSurfaceV0,
        admission: M8RuntimeAdmission,
    ) -> Self {
        Self {
            patch_id: patch_id.into(),
            state: M8PatchCandidateState::DeferredToM9,
            source_ref: checked.program_identity().root_source_ref().clone(),
            checked: Some(checked),
            admission: Some(admission),
            base_program_identity: None,
            base_admission: None,
            patch_authority: None,
            designated_input_receipts: None,
            reason_ref: None,
        }
    }

    pub fn unknown_reference(
        patch_id: impl Into<String>,
        source_ref: SourceRef,
        reason_ref: impl Into<String>,
    ) -> Self {
        Self {
            patch_id: patch_id.into(),
            state: M8PatchCandidateState::Unknown,
            checked: None,
            admission: None,
            base_program_identity: None,
            base_admission: None,
            patch_authority: None,
            designated_input_receipts: None,
            source_ref,
            reason_ref: Some(reason_ref.into()),
        }
    }

    pub fn with_base_program_identity(
        mut self,
        base_program_identity: CheckedProgramIdentity,
    ) -> Self {
        self.base_program_identity = Some(base_program_identity);
        self
    }

    pub fn with_base_admission(mut self, base_admission: M8RuntimeAdmission) -> Self {
        self.base_admission = Some(base_admission);
        self
    }

    pub fn with_patch_authority(mut self, patch_authority: M8PatchAuthorityUse) -> Self {
        self.patch_authority = Some(patch_authority);
        self
    }

    pub fn with_designated_input_receipts(
        mut self,
        designated_input_receipts: M8InputReceiptSet,
    ) -> Self {
        self.designated_input_receipts = Some(designated_input_receipts);
        self
    }

    pub fn with_source_ref(mut self, source_ref: SourceRef) -> Self {
        self.source_ref = source_ref;
        self
    }

    pub fn with_reason_ref(mut self, reason_ref: impl Into<String>) -> Self {
        self.reason_ref = Some(reason_ref.into());
        self
    }

    pub fn mark_stale_against_current_identity(mut self) -> Self {
        self.state = M8PatchCandidateState::Stale;
        self
    }

    pub const fn accepts_raw_eval(&self) -> bool {
        false
    }

    pub fn checked_program_identity(&self) -> &CheckedProgramIdentity {
        self.checked
            .as_ref()
            .expect("only checked M8 patch candidates have a checked identity")
            .program_identity()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

/// References to already-admitted authority required to cross a patch
/// activation cut.  Debug/provider/package metadata is retained only to make
/// clear that it is not consulted as authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchAuthorityUse {
    patch_program: String,
    owner_locus: Option<String>,
    principal: Option<String>,
    membership_ref: Option<String>,
    capability_ref: Option<String>,
    witness_ref: Option<String>,
    provider_name: Option<String>,
    package_name: Option<String>,
}

impl M8PatchAuthorityUse {
    pub fn for_patch_program(patch_program: impl Into<String>) -> Self {
        Self {
            patch_program: patch_program.into(),
            owner_locus: None,
            principal: None,
            membership_ref: None,
            capability_ref: None,
            witness_ref: None,
            provider_name: None,
            package_name: None,
        }
    }

    pub fn with_owner_locus(mut self, owner_locus: impl Into<String>) -> Self {
        self.owner_locus = Some(owner_locus.into());
        self
    }

    pub fn with_principal(mut self, principal: impl Into<String>) -> Self {
        self.principal = Some(principal.into());
        self
    }

    pub fn with_membership_ref(mut self, membership_ref: impl Into<String>) -> Self {
        self.membership_ref = Some(membership_ref.into());
        self
    }

    pub fn with_capability_ref(mut self, capability_ref: impl Into<String>) -> Self {
        self.capability_ref = Some(capability_ref.into());
        self
    }

    pub fn with_witness_ref(mut self, witness_ref: impl Into<String>) -> Self {
        self.witness_ref = Some(witness_ref.into());
        self
    }

    pub fn with_provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.provider_name = Some(provider_name.into());
        self
    }

    pub fn with_package_name(mut self, package_name: impl Into<String>) -> Self {
        self.package_name = Some(package_name.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8PatchLifecycleKind {
    CandidateBound,
    Parsed,
    Checked,
    Elaborated,
    Compatible,
    RuntimeAdmitted,
    ActivationCut,
    Rejected,
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8PatchDiagnosticKind {
    UnknownCandidate,
    StaleCandidate,
    UnadmittedCandidate,
    DeferredToM9,
    StructuralIdentityMismatch,
    AdmissionProvenanceMismatch,
    MissingPatchAuthority,
    NonQuiescentSession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchDiagnostic {
    kind: M8PatchDiagnosticKind,
    source_ref: SourceRef,
    reason_ref: Option<String>,
}

impl M8PatchDiagnostic {
    pub const fn kind(&self) -> M8PatchDiagnosticKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct M8PatchLifecycleRow {
    kind: M8PatchLifecycleKind,
    diagnostic: Option<M8PatchDiagnostic>,
    source_ref: SourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8PatchLifecycle {
    rows: Vec<M8PatchLifecycleRow>,
}

impl M8PatchLifecycle {
    pub fn kinds(&self) -> Vec<M8PatchLifecycleKind> {
        self.rows.iter().map(|row| row.kind).collect()
    }

    pub const fn contains_raw_eval(&self) -> bool {
        false
    }

    pub fn contains(&self, kind: M8PatchLifecycleKind) -> bool {
        self.rows.iter().any(|row| row.kind == kind)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn last_diagnostic_kind(&self) -> Option<M8PatchDiagnosticKind> {
        self.rows
            .last()
            .and_then(|row| row.diagnostic.as_ref())
            .map(M8PatchDiagnostic::kind)
    }

    pub fn last_source_ref(&self) -> &SourceRef {
        &self
            .rows
            .last()
            .expect("M8 patch lifecycle has a row before its source is queried")
            .source_ref
    }

    fn push_event(&mut self, kind: M8PatchLifecycleKind, source_ref: SourceRef) {
        self.rows.push(M8PatchLifecycleRow {
            kind,
            diagnostic: None,
            source_ref,
        });
    }

    fn push_diagnostic(&mut self, kind: M8PatchLifecycleKind, diagnostic: M8PatchDiagnostic) {
        self.rows.push(M8PatchLifecycleRow {
            kind,
            source_ref: diagnostic.source_ref.clone(),
            diagnostic: Some(diagnostic),
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8PatchVerdictKind {
    Accepted,
    Rejected,
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8PatchActivationCutKind {
    SingleSession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchActivationCut {
    kind: M8PatchActivationCutKind,
    patch_id: String,
    base_program_identity: CheckedProgramIdentity,
    activated_program_identity: CheckedProgramIdentity,
}

impl M8PatchActivationCut {
    pub const fn kind(&self) -> M8PatchActivationCutKind {
        self.kind
    }

    pub fn is_the_only_semantic_change_between(
        &self,
        before: &M8PatchSavePayload,
        after: &M8PatchSavePayload,
    ) -> bool {
        before.active_program_identity == self.base_program_identity
            && after.active_program_identity == self.activated_program_identity
            && before.activated_patch.is_none()
            && after.activated_patch.as_deref() == Some(self.patch_id.as_str())
            && before
                .session_semantic_payload
                .equivalent_for_activation(&after.session_semantic_payload)
            && after.session_patch_lifecycle_rows
                == [
                    before.session_patch_lifecycle_rows.as_slice(),
                    &[format!("activated:{}", self.patch_id)],
                ]
                .concat()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchSavePayload {
    session_payload: M8LocalSavePayload,
    session_semantic_payload: M8LocalSemanticPayload,
    session_patch_lifecycle_rows: Vec<String>,
    active_program_identity: CheckedProgramIdentity,
    activated_patch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchSemanticPayload {
    session_payload: M8LocalSemanticPayload,
    active_program_identity: CheckedProgramIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchOutcome {
    verdict: M8PatchVerdictKind,
    lifecycle: M8PatchLifecycle,
    diagnostic: Option<M8PatchDiagnostic>,
    activation_cut: Option<M8PatchActivationCut>,
    source_ref: SourceRef,
    reason_ref: Option<String>,
}

impl M8PatchOutcome {
    pub const fn verdict(&self) -> M8PatchVerdictKind {
        self.verdict
    }

    pub fn lifecycle(&self) -> &M8PatchLifecycle {
        &self.lifecycle
    }

    pub fn primary_diagnostic(&self) -> &M8PatchDiagnostic {
        self.diagnostic
            .as_ref()
            .expect("only non-accepted M8 patch outcomes have a diagnostic")
    }

    pub fn activation_cut(&self) -> Option<&M8PatchActivationCut> {
        self.activation_cut.as_ref()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn reason_ref(&self) -> Option<&str> {
        self.reason_ref.as_deref()
    }

    pub const fn has_runtime_success(&self) -> bool {
        matches!(self.verdict, M8PatchVerdictKind::Accepted)
    }

    pub const fn grants_authority(&self) -> bool {
        false
    }

    pub const fn emits_verdict(&self) -> bool {
        false
    }
}

/// Patch lifecycle rows are retained beside, not instead of, the one local
/// semantic session.  Only an accepted activation changes that session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8PatchRuntime {
    session: M8LocalRuntime,
    lifecycle: M8PatchLifecycle,
}

impl M8PatchRuntime {
    pub fn from_admitted(instance: M8RuntimeInstance, seed: M8PatchRuntimeSeed) -> Self {
        Self {
            session: M8LocalRuntime::from_admitted(instance, seed.into_local_seed()),
            lifecycle: M8PatchLifecycle::default(),
        }
    }

    pub fn active_program_identity(&self) -> &CheckedProgramIdentity {
        self.session.active_program_identity()
    }

    pub fn active_admission(&self) -> &M8RuntimeAdmission {
        self.session.active_admission()
    }

    pub fn patch_lifecycle(&self) -> &M8PatchLifecycle {
        &self.lifecycle
    }

    pub fn enqueue_owner(
        &mut self,
        request: M8OwnerRequest,
    ) -> Result<M8Occurrence, M8EnqueueDiagnostics> {
        self.session.enqueue_owner(request)
    }

    pub fn pending_owner_fifo(&self, owner_locus: &str) -> Vec<String> {
        self.session.pending_owner_fifo(owner_locus)
    }

    pub fn evaluate_designated(
        &mut self,
        request: M8DesignatedEvaluationRequest,
    ) -> Result<M8PublishedDesignatedValue, M8DesignatedDiagnostics> {
        self.session.evaluate_designated(request)
    }

    pub fn save_local_cut(&self, cut_id: impl Into<String>) -> M8LocalCut {
        self.session.save_local_cut(cut_id)
    }

    pub fn try_restore_local_cut(
        &mut self,
        cut: &M8LocalCut,
        floor: &M8LiveFloor,
    ) -> Result<(), M8LocalRestoreDiagnostics> {
        let mut candidate_session = self.session.clone();
        candidate_session.install_admitted_from_cut(cut);
        candidate_session.try_restore_local_cut(cut, floor)?;
        self.session = candidate_session;
        Ok(())
    }

    pub fn save_relevant_payload(&self) -> M8PatchSavePayload {
        M8PatchSavePayload {
            session_payload: self.session.save_relevant_payload(),
            session_semantic_payload: self.session.semantic_payload_without_patch_lifecycle(),
            session_patch_lifecycle_rows: self.session.patch_lifecycle_rows(),
            active_program_identity: self.active_program_identity().clone(),
            activated_patch: self.session.last_activated_patch().map(ToOwned::to_owned),
        }
    }

    pub fn semantic_payload_without_patch_lifecycle(&self) -> M8PatchSemanticPayload {
        M8PatchSemanticPayload {
            session_payload: self.session.semantic_payload_without_patch_lifecycle(),
            active_program_identity: self.active_program_identity().clone(),
        }
    }

    pub fn activate_patch(&mut self, candidate: M8PatchCandidate) -> M8PatchOutcome {
        if let Some((verdict, diagnostic_kind, lifecycle_kind)) =
            self.pre_activation_failure(&candidate)
        {
            let diagnostic = diagnostic_for(&candidate, diagnostic_kind);
            let mut attempt = M8PatchLifecycle::default();
            attempt.push_diagnostic(lifecycle_kind, diagnostic.clone());
            self.lifecycle.rows.extend(attempt.rows.iter().cloned());
            return M8PatchOutcome {
                verdict,
                lifecycle: attempt,
                diagnostic: Some(diagnostic),
                activation_cut: None,
                source_ref: candidate.source_ref.clone(),
                reason_ref: candidate.reason_ref.clone(),
            };
        }

        let mut attempt = M8PatchLifecycle::default();
        for kind in [
            M8PatchLifecycleKind::CandidateBound,
            M8PatchLifecycleKind::Parsed,
            M8PatchLifecycleKind::Checked,
            M8PatchLifecycleKind::Elaborated,
            M8PatchLifecycleKind::Compatible,
            M8PatchLifecycleKind::RuntimeAdmitted,
        ] {
            attempt.push_event(kind, candidate.source_ref.clone());
        }
        let activation_cut = M8PatchActivationCut {
            kind: M8PatchActivationCutKind::SingleSession,
            patch_id: candidate.patch_id.clone(),
            base_program_identity: self.active_program_identity().clone(),
            activated_program_identity: candidate.checked_program_identity().clone(),
        };
        let admitted = M8Runtime::default()
            .admit(
                candidate
                    .checked
                    .as_ref()
                    .expect("bound candidate retains checked structure")
                    .clone(),
                candidate
                    .admission
                    .as_ref()
                    .expect("bound candidate retains M8 admission")
                    .clone(),
            )
            .expect("pre-activation validation admitted the checked candidate");
        self.session.install_admitted_patch(
            admitted,
            candidate.designated_input_receipts.clone(),
            &candidate.patch_id,
        );
        attempt.push_event(
            M8PatchLifecycleKind::ActivationCut,
            candidate.source_ref.clone(),
        );
        self.lifecycle.rows.extend(attempt.rows.iter().cloned());
        M8PatchOutcome {
            verdict: M8PatchVerdictKind::Accepted,
            lifecycle: attempt,
            diagnostic: None,
            activation_cut: Some(activation_cut),
            source_ref: candidate.source_ref,
            reason_ref: candidate.reason_ref,
        }
    }

    fn pre_activation_failure(
        &self,
        candidate: &M8PatchCandidate,
    ) -> Option<(
        M8PatchVerdictKind,
        M8PatchDiagnosticKind,
        M8PatchLifecycleKind,
    )> {
        match candidate.state {
            M8PatchCandidateState::Unknown => {
                return Some((
                    M8PatchVerdictKind::Rejected,
                    M8PatchDiagnosticKind::UnknownCandidate,
                    M8PatchLifecycleKind::Rejected,
                ));
            }
            M8PatchCandidateState::Stale => {
                return Some((
                    M8PatchVerdictKind::Rejected,
                    M8PatchDiagnosticKind::StaleCandidate,
                    M8PatchLifecycleKind::Rejected,
                ));
            }
            M8PatchCandidateState::Unadmitted => {
                return Some((
                    M8PatchVerdictKind::Rejected,
                    M8PatchDiagnosticKind::UnadmittedCandidate,
                    M8PatchLifecycleKind::Rejected,
                ));
            }
            M8PatchCandidateState::DeferredToM9 => {
                return Some((
                    M8PatchVerdictKind::Deferred,
                    M8PatchDiagnosticKind::DeferredToM9,
                    M8PatchLifecycleKind::Deferred,
                ));
            }
            M8PatchCandidateState::Bound => {}
        }
        if candidate.base_program_identity.as_ref() != Some(self.active_program_identity()) {
            return Some((
                M8PatchVerdictKind::Rejected,
                M8PatchDiagnosticKind::StructuralIdentityMismatch,
                M8PatchLifecycleKind::Rejected,
            ));
        }
        if candidate.base_admission.as_ref() != Some(self.active_admission())
            || candidate.admission.as_ref().is_none_or(|admission| {
                admission.program_identity() != candidate.checked_program_identity()
            })
            || M8Runtime::default()
                .admit(
                    candidate
                        .checked
                        .as_ref()
                        .expect("bound candidate retains checked structure")
                        .clone(),
                    candidate
                        .admission
                        .as_ref()
                        .expect("bound candidate retains M8 admission")
                        .clone(),
                )
                .is_err()
        {
            return Some((
                M8PatchVerdictKind::Rejected,
                M8PatchDiagnosticKind::AdmissionProvenanceMismatch,
                M8PatchLifecycleKind::Rejected,
            ));
        }
        if !self.validates_patch_authority(candidate) {
            return Some((
                M8PatchVerdictKind::Rejected,
                M8PatchDiagnosticKind::MissingPatchAuthority,
                M8PatchLifecycleKind::Rejected,
            ));
        }
        if self.session.has_pending_owner_requests() {
            return Some((
                M8PatchVerdictKind::Rejected,
                M8PatchDiagnosticKind::NonQuiescentSession,
                M8PatchLifecycleKind::Rejected,
            ));
        }
        None
    }

    fn validates_patch_authority(&self, candidate: &M8PatchCandidate) -> bool {
        let Some(authority) = candidate.patch_authority.as_ref() else {
            return false;
        };
        let (Some(owner_locus), Some(principal)) = (
            authority.owner_locus.as_deref(),
            authority.principal.as_deref(),
        ) else {
            return false;
        };
        if authority.patch_program != candidate.checked_program_identity().module() {
            return false;
        }
        self.session
            .owner_state()
            .authority_state()
            .validates_patch_activation_use(M8PatchActivationAuthorityLookup {
                program_identity: authority.patch_program.as_str(),
                owner_locus,
                principal,
                membership_ref: authority.membership_ref.as_deref(),
                capability_ref: authority.capability_ref.as_deref(),
                witness_ref: authority.witness_ref.as_deref(),
            })
    }
}

fn diagnostic_for(candidate: &M8PatchCandidate, kind: M8PatchDiagnosticKind) -> M8PatchDiagnostic {
    M8PatchDiagnostic {
        kind,
        source_ref: candidate.source_ref.clone(),
        reason_ref: candidate.reason_ref.clone(),
    }
}
