//! Typed, redacted observer projections over an M8 local semantic session.
//!
//! Observer export is read-only.  It derives bounded rows from the same local
//! session used for owner, relation, and designated execution and never turns
//! debug metadata into authority or exposes raw semantic/authority payloads.

use std::collections::BTreeMap;

use mir_semantics::shared_model::SourceRef;

use crate::{
    m8_runtime_admission::{
        EvidenceRedaction, EvidenceSecurityLabel, M8RuntimeInstance, M8SecurityClass,
    },
    m8_runtime_authority::M8AuthorityState,
    m8_runtime_designated_value::{
        M8DesignatedDiagnostics, M8DesignatedEvaluationRequest, M8InputReceiptSet,
    },
    m8_runtime_local_cut::{
        M8LeaseRecord, M8LocalRuntime, M8LocalRuntimeSeed, M8LocalSavePayload, M8LocalTrace,
        M8LocalTraceKind, M8LocalTraceObservation,
    },
    m8_runtime_owner_queue::{
        M8EnqueueDiagnostics, M8Occurrence, M8OwnerRequest, M8ServeDiagnostics, M8ServeOutcome,
        M8StateKey,
    },
    m8_runtime_relation_projection::{
        M8BindingInvalidation, M8RelationAuthorityUse, M8RelationDiagnostics, M8RelationTransition,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverAuthorityGrant {
    reference: String,
    admitted: bool,
    principal: Option<String>,
    max_security_class: M8SecurityClass,
    epoch: Option<String>,
}

impl M8ObserverAuthorityGrant {
    pub fn already_admitted(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            admitted: true,
            principal: None,
            max_security_class: M8SecurityClass::Public,
            epoch: None,
        }
    }

    pub fn for_principal(mut self, principal: impl Into<String>) -> Self {
        self.principal = Some(principal.into());
        self
    }

    pub fn with_max_security_class(mut self, max_security_class: M8SecurityClass) -> Self {
        self.max_security_class = max_security_class;
        self
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }

    fn admits(&self, policy: &M8ObserverPolicy) -> bool {
        self.admitted
            && self.principal.as_deref() == Some(policy.observer_principal())
            && policy.authority_ref() == Some(self.reference.as_str())
            && self
                .max_security_class
                .is_at_least(policy.label().security_class())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverRetention {
    name: String,
    row_limit: usize,
}

impl M8ObserverRetention {
    pub fn bounded(name: impl Into<String>, row_limit: usize) -> Self {
        Self {
            name: name.into(),
            row_limit,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn permits_rows(&self) -> bool {
        self.row_limit > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverPolicy {
    observer_principal: String,
    authority_ref: Option<String>,
    label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
    retention: M8ObserverRetention,
    source_ref: SourceRef,
    reason_ref: Option<String>,
    proof_ref: Option<String>,
    relation_label_overrides: BTreeMap<String, EvidenceSecurityLabel>,
    relation_input_labels: BTreeMap<String, EvidenceSecurityLabel>,
    debug_provider_name: Option<String>,
    package_name: Option<String>,
}

impl M8ObserverPolicy {
    pub fn for_principal(observer_principal: impl Into<String>) -> Self {
        Self {
            observer_principal: observer_principal.into(),
            authority_ref: None,
            label: EvidenceSecurityLabel::new("observer:unspecified"),
            redaction: EvidenceRedaction::new("unspecified"),
            retention: M8ObserverRetention::bounded("unspecified", 0),
            source_ref: SourceRef::new("<m8-observer>", 1, 1, 1, 1),
            reason_ref: None,
            proof_ref: None,
            relation_label_overrides: BTreeMap::new(),
            relation_input_labels: BTreeMap::new(),
            debug_provider_name: None,
            package_name: None,
        }
    }

    pub fn with_authority_ref(mut self, authority_ref: impl Into<String>) -> Self {
        self.authority_ref = Some(authority_ref.into());
        self
    }

    pub fn with_label(mut self, label: EvidenceSecurityLabel) -> Self {
        self.label = label;
        self
    }

    pub fn with_redaction(mut self, redaction: EvidenceRedaction) -> Self {
        self.redaction = redaction;
        self
    }

    pub fn with_retention(mut self, retention: M8ObserverRetention) -> Self {
        self.retention = retention;
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

    pub fn with_proof_ref(mut self, proof_ref: impl Into<String>) -> Self {
        self.proof_ref = Some(proof_ref.into());
        self
    }

    pub fn with_relation_label_override(
        mut self,
        relation: impl Into<String>,
        label: EvidenceSecurityLabel,
    ) -> Self {
        self.relation_label_overrides.insert(relation.into(), label);
        self
    }

    pub fn with_relation_input_label(
        mut self,
        relation: impl Into<String>,
        label: EvidenceSecurityLabel,
    ) -> Self {
        self.relation_input_labels.insert(relation.into(), label);
        self
    }

    pub fn with_debug_provider_name(mut self, debug_provider_name: impl Into<String>) -> Self {
        self.debug_provider_name = Some(debug_provider_name.into());
        self
    }

    pub fn with_package_name(mut self, package_name: impl Into<String>) -> Self {
        self.package_name = Some(package_name.into());
        self
    }

    pub fn observer_principal(&self) -> &str {
        &self.observer_principal
    }

    pub fn authority_ref(&self) -> Option<&str> {
        self.authority_ref.as_deref()
    }

    pub fn label(&self) -> &EvidenceSecurityLabel {
        &self.label
    }

    pub fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }

    pub fn retention(&self) -> &M8ObserverRetention {
        &self.retention
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn reason_ref(&self) -> Option<&str> {
        self.reason_ref.as_deref()
    }

    pub fn proof_ref(&self) -> Option<&str> {
        self.proof_ref.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverRuntimeSeed {
    owner_ints: BTreeMap<M8StateKey, i64>,
    authority_state: M8AuthorityState,
    live_leases: Vec<M8LeaseRecord>,
    observer_authorities: Vec<M8ObserverAuthorityGrant>,
    designated_input_receipts: M8InputReceiptSet,
}

impl M8ObserverRuntimeSeed {
    pub fn new() -> Self {
        Self {
            owner_ints: BTreeMap::new(),
            authority_state: M8AuthorityState::new(),
            live_leases: Vec::new(),
            observer_authorities: Vec::new(),
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

    pub fn with_observer_authority(mut self, authority: M8ObserverAuthorityGrant) -> Self {
        self.observer_authorities.push(authority);
        self
    }

    pub fn with_designated_input_receipts(
        mut self,
        designated_input_receipts: M8InputReceiptSet,
    ) -> Self {
        self.designated_input_receipts = designated_input_receipts;
        self
    }

    fn into_parts(self) -> (M8LocalRuntimeSeed, Vec<M8ObserverAuthorityGrant>) {
        let mut local = M8LocalRuntimeSeed::new()
            .with_authority_state(self.authority_state)
            .with_designated_input_receipts(self.designated_input_receipts);
        for (key, value) in self.owner_ints {
            local = local.with_owner_int(key, value);
        }
        for lease in self.live_leases {
            local = local.with_live_lease(lease);
        }
        (local, self.observer_authorities)
    }
}

impl Default for M8ObserverRuntimeSeed {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8ObserverDiagnosticKind {
    MissingObserverAuthority,
    MissingTypedPolicy,
    RelationLabelWouldWeakenInputJoin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverDiagnostic {
    kind: M8ObserverDiagnosticKind,
    source_ref: SourceRef,
    reason_ref: Option<String>,
    proof_ref: Option<String>,
}

impl M8ObserverDiagnostic {
    pub const fn kind(&self) -> M8ObserverDiagnosticKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn reason_ref(&self) -> Option<&str> {
        self.reason_ref.as_deref()
    }

    pub fn proof_ref(&self) -> Option<&str> {
        self.proof_ref.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverDiagnostics {
    entries: Vec<M8ObserverDiagnostic>,
    secret_fields: Vec<String>,
}

impl M8ObserverDiagnostics {
    fn one(policy: &M8ObserverPolicy, kind: M8ObserverDiagnosticKind) -> Self {
        Self {
            entries: vec![M8ObserverDiagnostic {
                kind,
                source_ref: policy.source_ref.clone(),
                reason_ref: policy.reason_ref.clone(),
                proof_ref: policy.proof_ref.clone(),
            }],
            secret_fields: Vec::new(),
        }
    }

    pub fn primary(&self) -> &M8ObserverDiagnostic {
        self.entries
            .first()
            .expect("M8 observer diagnostics have a primary entry")
    }

    pub fn contains_raw_value_for(&self, _value_name: &str) -> bool {
        false
    }

    pub fn contains_raw_value_for_state_key(&self, _state_key: &M8StateKey) -> bool {
        false
    }

    pub fn secret_fields(&self) -> &[String] {
        &self.secret_fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M8ObserverRowKind {
    OwnerWrite,
    RelationLineage,
    DesignatedValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverRow {
    kind: M8ObserverRowKind,
    subject: String,
    occurrence_id: Option<String>,
    dependency_ids: Vec<String>,
    source_ref: SourceRef,
    label: EvidenceSecurityLabel,
    redaction: EvidenceRedaction,
}

impl M8ObserverRow {
    pub fn occurrence_id(&self) -> Option<&str> {
        self.occurrence_id.as_deref()
    }

    pub fn dependency_ids(&self) -> &[String] {
        &self.dependency_ids
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn label(&self) -> &EvidenceSecurityLabel {
        &self.label
    }

    pub fn redaction(&self) -> &EvidenceRedaction {
        &self.redaction
    }

    pub fn corresponds_to(&self, trace: M8LocalTrace) -> bool {
        self.corresponds_to_exact_trace(&trace)
    }

    pub fn corresponds_to_exact_trace(&self, trace: &M8LocalTrace) -> bool {
        let Some(occurrence_id) = self.occurrence_id() else {
            return false;
        };
        !self.dependency_ids.is_empty()
            && trace.contains_node_id(occurrence_id)
            && trace.source_ref_for_node_id(occurrence_id) == Some(&self.source_ref)
            && self
                .dependency_ids
                .iter()
                .all(|dependency_id| trace.contains_edge(dependency_id, occurrence_id))
    }

    pub fn contains_secret_field(&self, _field: &str) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8ObserverRows {
    rows: Vec<M8ObserverRow>,
}

impl M8ObserverRows {
    pub fn contains_kind(&self, kind: M8ObserverRowKind) -> bool {
        self.rows.iter().any(|row| row.kind == kind)
    }

    pub fn all_have_occurrence_dependency_correspondence(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.occurrence_id.is_some() && !row.dependency_ids.is_empty())
    }

    pub fn contains_redacted_subject(&self, subject: &str) -> bool {
        self.redacted_subject(subject).is_some()
    }

    pub fn redacted_subject(&self, subject: &str) -> Option<&M8ObserverRow> {
        self.rows.iter().find(|row| row.subject == subject)
    }

    pub fn contains_raw_value_for(&self, _value_name: &str) -> bool {
        false
    }

    pub fn contains_raw_value_for_state_key(&self, _state_key: &M8StateKey) -> bool {
        false
    }

    pub fn all_correspond_to_exact_trace(&self, trace: &M8LocalTrace) -> bool {
        self.rows
            .iter()
            .all(|row| row.corresponds_to_exact_trace(trace))
    }

    pub fn all_source_refs_match_runtime_trace(&self, trace: &M8LocalTrace) -> bool {
        self.rows.iter().all(|row| {
            row.occurrence_id()
                .and_then(|node_id| trace.source_ref_for_node_id(node_id))
                == Some(row.source_ref())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverView {
    policy: M8ObserverPolicy,
    rows: M8ObserverRows,
}

impl M8ObserverView {
    pub fn policy(&self) -> &M8ObserverPolicy {
        &self.policy
    }

    pub fn rows(&self) -> &M8ObserverRows {
        &self.rows
    }
}

/// A read-only observer facade over the M8 local session.  The only retained
/// observer state is the pre-admitted authority inventory; exporting a view
/// does not append trace rows, change semantics, or persist a presentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8ObserverRuntime {
    session: M8LocalRuntime,
    observer_authorities: Vec<M8ObserverAuthorityGrant>,
}

impl M8ObserverRuntime {
    pub fn from_admitted(instance: M8RuntimeInstance, seed: M8ObserverRuntimeSeed) -> Self {
        let (local_seed, observer_authorities) = seed.into_parts();
        Self {
            session: M8LocalRuntime::from_admitted(instance, local_seed),
            observer_authorities,
        }
    }

    pub fn enqueue_owner(
        &mut self,
        request: M8OwnerRequest,
    ) -> Result<M8Occurrence, M8EnqueueDiagnostics> {
        self.session.enqueue_owner(request)
    }

    pub fn serve_next_owner(
        &mut self,
        owner_locus: &str,
    ) -> Result<M8ServeOutcome, M8ServeDiagnostics> {
        self.session.serve_next_owner(owner_locus)
    }

    pub fn invalidate_primary(
        &mut self,
        relation: &str,
        authority: M8RelationAuthorityUse,
        invalidation: M8BindingInvalidation,
    ) -> Result<M8RelationTransition, M8RelationDiagnostics> {
        self.session
            .invalidate_primary(relation, authority, invalidation)
    }

    pub fn evaluate_designated(
        &mut self,
        request: M8DesignatedEvaluationRequest,
    ) -> Result<
        crate::m8_runtime_designated_value::M8PublishedDesignatedValue,
        M8DesignatedDiagnostics,
    > {
        self.session.evaluate_designated(request)
    }

    pub fn save_relevant_payload(&self) -> M8LocalSavePayload {
        self.session.save_relevant_payload()
    }

    pub fn trace(&self) -> M8LocalTrace {
        self.session.trace()
    }

    pub fn export_observer_view(
        &self,
        policy: M8ObserverPolicy,
    ) -> Result<M8ObserverView, M8ObserverDiagnostics> {
        if !self
            .observer_authorities
            .iter()
            .any(|authority| authority.admits(&policy))
        {
            return Err(M8ObserverDiagnostics::one(
                &policy,
                M8ObserverDiagnosticKind::MissingObserverAuthority,
            ));
        }
        if !policy.retention.permits_rows()
            || policy.reason_ref.is_none()
            || policy.proof_ref.is_none()
        {
            return Err(M8ObserverDiagnostics::one(
                &policy,
                M8ObserverDiagnosticKind::MissingTypedPolicy,
            ));
        }
        if policy
            .relation_input_labels
            .iter()
            .any(|(relation, input_label)| {
                let derived_label = policy
                    .relation_label_overrides
                    .get(relation)
                    .unwrap_or(&policy.label);
                !derived_label
                    .security_class()
                    .is_at_least(input_label.security_class())
            })
        {
            return Err(M8ObserverDiagnostics::one(
                &policy,
                M8ObserverDiagnosticKind::RelationLabelWouldWeakenInputJoin,
            ));
        }
        Ok(M8ObserverView {
            rows: self.redacted_rows(&policy),
            policy,
        })
    }

    fn redacted_rows(&self, policy: &M8ObserverPolicy) -> M8ObserverRows {
        let trace = self.session.trace();
        let mut rows = Vec::new();
        if let Some(observation) = trace.latest_observation(M8LocalTraceKind::OwnerWrite) {
            rows.push(redacted_row(
                M8ObserverRowKind::OwnerWrite,
                "owner-write",
                policy,
                observation,
            ));
        }
        if let Some(observation) =
            trace.latest_observation(M8LocalTraceKind::RelationOptionAdvanced)
        {
            rows.push(redacted_row(
                M8ObserverRowKind::RelationLineage,
                "relation-lineage",
                policy,
                observation,
            ));
        }
        for publication in self
            .session
            .designated_result_store()
            .published_value_observations()
        {
            if let Some(observation) = trace.latest_observation_for_occurrence(
                M8LocalTraceKind::DesignatedValuePublished,
                &publication.occurrence_id,
            ) {
                rows.push(redacted_designated_row(
                    &publication.value_name,
                    policy,
                    observation,
                ));
            }
        }
        rows.truncate(policy.retention.row_limit);
        M8ObserverRows { rows }
    }
}

fn redacted_row(
    kind: M8ObserverRowKind,
    subject: &str,
    policy: &M8ObserverPolicy,
    observation: M8LocalTraceObservation,
) -> M8ObserverRow {
    M8ObserverRow {
        kind,
        subject: subject.to_string(),
        occurrence_id: Some(observation.node_id),
        dependency_ids: observation.dependencies,
        source_ref: observation.source_ref,
        label: policy.label.clone(),
        redaction: policy.redaction.clone(),
    }
}

fn redacted_designated_row(
    value_name: &str,
    policy: &M8ObserverPolicy,
    observation: M8LocalTraceObservation,
) -> M8ObserverRow {
    redacted_row(
        M8ObserverRowKind::DesignatedValue,
        value_name,
        policy,
        observation,
    )
}
