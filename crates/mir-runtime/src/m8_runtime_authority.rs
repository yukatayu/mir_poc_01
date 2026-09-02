//! Already-admitted authority records retained by M8 semantic state.
//!
//! M8 validates references to these finite records but never creates a grant,
//! refreshes a witness, or acts as an authentication provider.

use std::collections::{BTreeMap, BTreeSet};

use mir_semantics::shared_model::ResultVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8MembershipRecord {
    reference: String,
    principal: Option<String>,
    locus: Option<String>,
    epoch: Option<String>,
}

impl M8MembershipRecord {
    pub fn already_admitted(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            principal: None,
            locus: None,
            epoch: None,
        }
    }

    pub fn with_principal(mut self, principal: impl Into<String>) -> Self {
        self.principal = Some(principal.into());
        self
    }

    pub fn with_locus(mut self, locus: impl Into<String>) -> Self {
        self.locus = Some(locus.into());
        self
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum M8CapabilityScope {
    OwnerEvaluation {
        evaluation: String,
    },
    RelationTransition {
        relation: String,
        transition: String,
    },
    DesignatedEvaluation {
        evaluator: String,
        result: String,
    },
    DesignatedConsumption {
        consumer: String,
        value_name: String,
    },
    PatchActivation {
        program_identity: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8CapabilityGrant {
    reference: String,
    active: bool,
    scope: Option<M8CapabilityScope>,
    owner_locus: Option<String>,
    principal: Option<String>,
    membership_ref: Option<String>,
    epoch: Option<String>,
    binding_epoch: Option<String>,
    evaluator_locus: Option<String>,
    consumer_locus: Option<String>,
    input_frontier: Option<String>,
    result_version: Option<ResultVersion>,
}

impl M8CapabilityGrant {
    pub fn already_admitted(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            active: true,
            scope: None,
            owner_locus: None,
            principal: None,
            membership_ref: None,
            epoch: None,
            binding_epoch: None,
            evaluator_locus: None,
            consumer_locus: None,
            input_frontier: None,
            result_version: None,
        }
    }

    pub fn revoked(reference: impl Into<String>) -> Self {
        Self {
            active: false,
            ..Self::already_admitted(reference)
        }
    }

    pub fn for_owner_evaluation(mut self, evaluation: impl Into<String>) -> Self {
        self.scope = Some(M8CapabilityScope::OwnerEvaluation {
            evaluation: evaluation.into(),
        });
        self
    }

    pub fn for_relation_transition(
        mut self,
        relation: impl Into<String>,
        transition: impl Into<String>,
    ) -> Self {
        self.scope = Some(M8CapabilityScope::RelationTransition {
            relation: relation.into(),
            transition: transition.into(),
        });
        self
    }

    pub fn for_designated_evaluation(
        mut self,
        evaluator: impl Into<String>,
        result: impl Into<String>,
    ) -> Self {
        self.scope = Some(M8CapabilityScope::DesignatedEvaluation {
            evaluator: evaluator.into(),
            result: result.into(),
        });
        self
    }

    pub fn for_designated_consumption(
        mut self,
        consumer: impl Into<String>,
        value_name: impl Into<String>,
    ) -> Self {
        self.scope = Some(M8CapabilityScope::DesignatedConsumption {
            consumer: consumer.into(),
            value_name: value_name.into(),
        });
        self
    }

    pub fn for_patch_activation(mut self, program_identity: impl Into<String>) -> Self {
        self.scope = Some(M8CapabilityScope::PatchActivation {
            program_identity: program_identity.into(),
        });
        self
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

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }

    pub fn with_binding_epoch(mut self, binding_epoch: impl Into<String>) -> Self {
        self.binding_epoch = Some(binding_epoch.into());
        self
    }

    pub fn with_evaluator_locus(mut self, evaluator_locus: impl Into<String>) -> Self {
        self.evaluator_locus = Some(evaluator_locus.into());
        self
    }

    pub fn with_consumer_locus(mut self, consumer_locus: impl Into<String>) -> Self {
        self.consumer_locus = Some(consumer_locus.into());
        self
    }

    pub fn with_input_frontier(mut self, input_frontier: impl Into<String>) -> Self {
        self.input_frontier = Some(input_frontier.into());
        self
    }

    pub fn with_result_version(mut self, result_version: ResultVersion) -> Self {
        self.result_version = Some(result_version);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M8WitnessRecord {
    reference: String,
    live: bool,
    capability_ref: Option<String>,
    membership_ref: Option<String>,
    epoch: Option<String>,
}

impl M8WitnessRecord {
    pub fn live(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
            live: true,
            capability_ref: None,
            membership_ref: None,
            epoch: None,
        }
    }

    pub fn stale(reference: impl Into<String>) -> Self {
        Self {
            live: false,
            ..Self::live(reference)
        }
    }

    pub fn for_capability(mut self, capability_ref: impl Into<String>) -> Self {
        self.capability_ref = Some(capability_ref.into());
        self
    }

    pub fn with_membership_ref(mut self, membership_ref: impl Into<String>) -> Self {
        self.membership_ref = Some(membership_ref.into());
        self
    }

    pub fn with_epoch(mut self, epoch: impl Into<String>) -> Self {
        self.epoch = Some(epoch.into());
        self
    }
}

/// Finite already-admitted authority inventory.  Mutation of this inventory is
/// deliberately not part of the M8 owner or relation transition APIs.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct M8AuthorityState {
    memberships: BTreeMap<String, M8MembershipRecord>,
    capability_grants: BTreeMap<String, M8CapabilityGrant>,
    witness_records: BTreeMap<String, M8WitnessRecord>,
}

/// Strict private process-image snapshot of already-admitted M8 authority
/// records.  It has no issuer, refresh, or transport-facing constructor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct M8I3PrivateAuthorityStateSnapshot {
    memberships: Vec<PrivateMembershipRecordSnapshot>,
    capability_grants: Vec<PrivateCapabilityGrantSnapshot>,
    witness_records: Vec<PrivateWitnessRecordSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateMembershipRecordSnapshot {
    reference: String,
    principal: Option<String>,
    locus: Option<String>,
    epoch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum PrivateCapabilityScopeSnapshot {
    OwnerEvaluation {
        evaluation: String,
    },
    RelationTransition {
        relation: String,
        transition: String,
    },
    DesignatedEvaluation {
        evaluator: String,
        result: String,
    },
    DesignatedConsumption {
        consumer: String,
        value_name: String,
    },
    PatchActivation {
        program_identity: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateCapabilityGrantSnapshot {
    reference: String,
    active: bool,
    scope: Option<PrivateCapabilityScopeSnapshot>,
    owner_locus: Option<String>,
    principal: Option<String>,
    membership_ref: Option<String>,
    epoch: Option<String>,
    binding_epoch: Option<String>,
    evaluator_locus: Option<String>,
    consumer_locus: Option<String>,
    input_frontier: Option<String>,
    result_version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivateWitnessRecordSnapshot {
    reference: String,
    live: bool,
    capability_ref: Option<String>,
    membership_ref: Option<String>,
    epoch: Option<String>,
}

impl M8AuthorityState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_membership_record(mut self, record: M8MembershipRecord) -> Self {
        self.memberships.insert(record.reference.clone(), record);
        self
    }

    pub fn with_capability_grant(mut self, grant: M8CapabilityGrant) -> Self {
        self.capability_grants
            .insert(grant.reference.clone(), grant);
        self
    }

    pub fn with_witness_record(mut self, record: M8WitnessRecord) -> Self {
        self.witness_records
            .insert(record.reference.clone(), record);
        self
    }

    pub(crate) fn validate_owner_use(
        &self,
        principal: &str,
        membership_ref: Option<&str>,
        capability_ref: Option<&str>,
        witness_ref: Option<&str>,
        owner_locus: &str,
        evaluation: &str,
    ) -> Result<(), M8AuthorityValidationFailure> {
        let membership_ref = membership_ref.ok_or(M8AuthorityValidationFailure::StaleMembership)?;
        let membership = self
            .memberships
            .get(membership_ref)
            .ok_or(M8AuthorityValidationFailure::StaleMembership)?;
        if membership.principal.as_deref() != Some(principal)
            || membership.locus.as_deref() != Some(owner_locus)
            || membership.epoch.is_none()
        {
            return Err(M8AuthorityValidationFailure::StaleMembership);
        }

        let capability_ref =
            capability_ref.ok_or(M8AuthorityValidationFailure::MissingCapability)?;
        let grant = self
            .capability_grants
            .get(capability_ref)
            .filter(|grant| grant.active)
            .ok_or(M8AuthorityValidationFailure::MissingCapability)?;
        if !matches!(
            &grant.scope,
            Some(M8CapabilityScope::OwnerEvaluation { evaluation: granted }) if granted == evaluation
        ) || grant.owner_locus.as_deref() != Some(owner_locus)
            || grant.principal.as_deref() != Some(principal)
            || grant.membership_ref.as_deref() != Some(membership_ref)
            || grant.epoch.as_deref() != membership.epoch.as_deref()
        {
            return Err(M8AuthorityValidationFailure::MissingCapability);
        }

        let witness_ref = witness_ref.ok_or(M8AuthorityValidationFailure::MissingWitness)?;
        let witness = self
            .witness_records
            .get(witness_ref)
            .filter(|witness| witness.live)
            .ok_or(M8AuthorityValidationFailure::MissingWitness)?;
        if witness.capability_ref.as_deref() != Some(capability_ref)
            || witness.membership_ref.as_deref() != Some(membership_ref)
            || witness.epoch.as_deref() != grant.epoch.as_deref()
        {
            return Err(M8AuthorityValidationFailure::MissingWitness);
        }
        Ok(())
    }

    pub(crate) fn validates_relation_use(&self, use_refs: M8RelationAuthorityLookup<'_>) -> bool {
        let (
            Some(membership_ref),
            Some(capability_ref),
            Some(binding_epoch),
            Some(witness_ref),
            Some(witness_epoch),
        ) = (
            use_refs.membership_ref,
            use_refs.capability_ref,
            use_refs.binding_epoch,
            use_refs.witness_ref,
            use_refs.witness_epoch,
        )
        else {
            return false;
        };
        let Some(membership) = self.memberships.get(membership_ref) else {
            return false;
        };
        let membership_epoch = use_refs.membership_epoch.unwrap_or(binding_epoch);
        if membership.principal.as_deref() != Some(use_refs.principal)
            || membership.locus.as_deref() != Some(use_refs.owner_locus)
            || membership.epoch.as_deref() != Some(membership_epoch)
        {
            return false;
        }
        let Some(grant) = self.capability_grants.get(capability_ref) else {
            return false;
        };
        if !grant.active
            || !matches!(
                &grant.scope,
                Some(M8CapabilityScope::RelationTransition { relation: granted_relation, transition: granted_transition })
                    if granted_relation == use_refs.relation && granted_transition == use_refs.transition
            )
            || grant.owner_locus.as_deref() != Some(use_refs.owner_locus)
            || grant.principal.as_deref() != Some(use_refs.principal)
            || grant.membership_ref.as_deref() != Some(membership_ref)
            || grant.binding_epoch.as_deref() != Some(binding_epoch)
        {
            return false;
        }
        let Some(witness) = self.witness_records.get(witness_ref) else {
            return false;
        };
        witness.live
            && witness.capability_ref.as_deref() == Some(capability_ref)
            && witness.membership_ref.as_deref() == Some(membership_ref)
            && witness.epoch.as_deref() == Some(witness_epoch)
    }

    pub(crate) fn validates_designated_evaluation_use(
        &self,
        use_refs: M8DesignatedEvaluationAuthorityLookup<'_>,
    ) -> bool {
        let (Some(membership_ref), Some(capability_ref), Some(witness_ref)) = (
            use_refs.membership_ref,
            use_refs.capability_ref,
            use_refs.witness_ref,
        ) else {
            return false;
        };
        let Some(membership) = self.memberships.get(membership_ref) else {
            return false;
        };
        if membership.principal.as_deref() != Some(use_refs.principal)
            || membership.locus.as_deref() != Some(use_refs.evaluator)
            || membership.epoch.is_none()
        {
            return false;
        }
        let Some(grant) = self.capability_grants.get(capability_ref) else {
            return false;
        };
        if !grant.active
            || !matches!(
                &grant.scope,
                Some(M8CapabilityScope::DesignatedEvaluation { evaluator, result })
                    if evaluator == use_refs.evaluator && result == use_refs.result
            )
            || grant.evaluator_locus.as_deref() != Some(use_refs.evaluator)
            || grant.principal.as_deref() != Some(use_refs.principal)
            || grant.membership_ref.as_deref() != Some(membership_ref)
            || grant.input_frontier.as_deref() != Some(use_refs.input_frontier)
            || grant.epoch.as_deref() != membership.epoch.as_deref()
        {
            return false;
        }
        let Some(witness) = self.witness_records.get(witness_ref) else {
            return false;
        };
        witness.live
            && witness.capability_ref.as_deref() == Some(capability_ref)
            && witness.membership_ref.as_deref() == Some(membership_ref)
            && witness.epoch.as_deref() == grant.epoch.as_deref()
    }

    pub(crate) fn validates_designated_consumption_use(
        &self,
        use_refs: M8DesignatedConsumptionAuthorityLookup<'_>,
    ) -> bool {
        let (Some(membership_ref), Some(capability_ref), Some(witness_ref)) = (
            use_refs.membership_ref,
            use_refs.capability_ref,
            use_refs.witness_ref,
        ) else {
            return false;
        };
        let Some(membership) = self.memberships.get(membership_ref) else {
            return false;
        };
        if membership.principal.as_deref() != Some(use_refs.principal)
            || membership.locus.as_deref() != Some(use_refs.consumer)
            || membership.epoch.is_none()
        {
            return false;
        }
        let Some(grant) = self.capability_grants.get(capability_ref) else {
            return false;
        };
        if !grant.active
            || !matches!(
                &grant.scope,
                Some(M8CapabilityScope::DesignatedConsumption { consumer, value_name })
                    if consumer == use_refs.consumer && value_name == use_refs.value_name
            )
            || grant.consumer_locus.as_deref() != Some(use_refs.consumer)
            || grant.principal.as_deref() != Some(use_refs.principal)
            || grant.membership_ref.as_deref() != Some(membership_ref)
            || grant.result_version != Some(use_refs.result_version)
            || grant.epoch.as_deref() != membership.epoch.as_deref()
        {
            return false;
        }
        let Some(witness) = self.witness_records.get(witness_ref) else {
            return false;
        };
        witness.live
            && witness.capability_ref.as_deref() == Some(capability_ref)
            && witness.membership_ref.as_deref() == Some(membership_ref)
            && witness.epoch.as_deref() == grant.epoch.as_deref()
    }

    pub(crate) fn validates_patch_activation_use(
        &self,
        use_refs: M8PatchActivationAuthorityLookup<'_>,
    ) -> bool {
        let (Some(membership_ref), Some(capability_ref), Some(witness_ref)) = (
            use_refs.membership_ref,
            use_refs.capability_ref,
            use_refs.witness_ref,
        ) else {
            return false;
        };
        let Some(membership) = self.memberships.get(membership_ref) else {
            return false;
        };
        if membership.principal.as_deref() != Some(use_refs.principal)
            || membership.locus.as_deref() != Some(use_refs.owner_locus)
            || membership.epoch.is_none()
        {
            return false;
        }
        let Some(grant) = self.capability_grants.get(capability_ref) else {
            return false;
        };
        if !grant.active
            || !matches!(
                &grant.scope,
                Some(M8CapabilityScope::PatchActivation { program_identity })
                    if program_identity == use_refs.program_identity
            )
            || grant.owner_locus.as_deref() != Some(use_refs.owner_locus)
            || grant.principal.as_deref() != Some(use_refs.principal)
            || grant.membership_ref.as_deref() != Some(membership_ref)
            || grant.epoch.as_deref() != membership.epoch.as_deref()
        {
            return false;
        }
        let Some(witness) = self.witness_records.get(witness_ref) else {
            return false;
        };
        witness.live
            && witness.capability_ref.as_deref() == Some(capability_ref)
            && witness.membership_ref.as_deref() == Some(membership_ref)
            && witness.epoch.as_deref() == grant.epoch.as_deref()
    }

    pub fn issued_by_m8(&self) -> &[String] {
        &[]
    }

    pub fn contains_membership(&self, reference: &str) -> bool {
        self.memberships.contains_key(reference)
    }

    pub fn contains_capability(&self, reference: &str) -> bool {
        self.capability_grants.contains_key(reference)
    }

    pub fn contains_witness(&self, reference: &str) -> bool {
        self.witness_records.contains_key(reference)
    }

    /// Restrict already-admitted authority records to the exact references
    /// consumed by a sealed downstream execution image.  This is a pure
    /// filter: it never issues, refreshes, or rewrites a membership,
    /// capability, or witness record.
    pub(crate) fn restricted_to_references(
        &self,
        membership_refs: &BTreeSet<String>,
        capability_refs: &BTreeSet<String>,
        witness_refs: &BTreeSet<String>,
    ) -> Self {
        Self {
            memberships: self
                .memberships
                .iter()
                .filter(|(reference, _)| membership_refs.contains(*reference))
                .map(|(reference, record)| (reference.clone(), record.clone()))
                .collect(),
            capability_grants: self
                .capability_grants
                .iter()
                .filter(|(reference, _)| capability_refs.contains(*reference))
                .map(|(reference, grant)| (reference.clone(), grant.clone()))
                .collect(),
            witness_records: self
                .witness_records
                .iter()
                .filter(|(reference, _)| witness_refs.contains(*reference))
                .map(|(reference, record)| (reference.clone(), record.clone()))
                .collect(),
        }
    }

    /// Copy an exact restricted authority inventory into the private I3 image
    /// schema.  This does not widen the selection or produce authority.
    pub(crate) fn i3_private_snapshot(&self) -> M8I3PrivateAuthorityStateSnapshot {
        M8I3PrivateAuthorityStateSnapshot {
            memberships: self
                .memberships
                .values()
                .map(PrivateMembershipRecordSnapshot::from_record)
                .collect(),
            capability_grants: self
                .capability_grants
                .values()
                .map(PrivateCapabilityGrantSnapshot::from_grant)
                .collect(),
            witness_records: self
                .witness_records
                .values()
                .map(PrivateWitnessRecordSnapshot::from_record)
                .collect(),
        }
    }

    /// Restore only the already-selected records contained in a private I3
    /// process image.  Duplicate references fail closed before a runtime can
    /// validate or consume an authority use.
    pub(crate) fn from_i3_private_snapshot(
        snapshot: M8I3PrivateAuthorityStateSnapshot,
    ) -> Result<Self, ()> {
        let mut memberships = BTreeMap::new();
        for snapshot in snapshot.memberships {
            let record = snapshot.into_record();
            if memberships
                .insert(record.reference.clone(), record)
                .is_some()
            {
                return Err(());
            }
        }
        let mut capability_grants = BTreeMap::new();
        for snapshot in snapshot.capability_grants {
            let grant = snapshot.into_grant();
            if capability_grants
                .insert(grant.reference.clone(), grant)
                .is_some()
            {
                return Err(());
            }
        }
        let mut witness_records = BTreeMap::new();
        for snapshot in snapshot.witness_records {
            let record = snapshot.into_record();
            if witness_records
                .insert(record.reference.clone(), record)
                .is_some()
            {
                return Err(());
            }
        }
        Ok(Self {
            memberships,
            capability_grants,
            witness_records,
        })
    }
}

impl PrivateMembershipRecordSnapshot {
    fn from_record(record: &M8MembershipRecord) -> Self {
        Self {
            reference: record.reference.clone(),
            principal: record.principal.clone(),
            locus: record.locus.clone(),
            epoch: record.epoch.clone(),
        }
    }

    fn into_record(self) -> M8MembershipRecord {
        M8MembershipRecord {
            reference: self.reference,
            principal: self.principal,
            locus: self.locus,
            epoch: self.epoch,
        }
    }
}

impl PrivateCapabilityGrantSnapshot {
    fn from_grant(grant: &M8CapabilityGrant) -> Self {
        Self {
            reference: grant.reference.clone(),
            active: grant.active,
            scope: grant
                .scope
                .as_ref()
                .map(PrivateCapabilityScopeSnapshot::from_scope),
            owner_locus: grant.owner_locus.clone(),
            principal: grant.principal.clone(),
            membership_ref: grant.membership_ref.clone(),
            epoch: grant.epoch.clone(),
            binding_epoch: grant.binding_epoch.clone(),
            evaluator_locus: grant.evaluator_locus.clone(),
            consumer_locus: grant.consumer_locus.clone(),
            input_frontier: grant.input_frontier.clone(),
            result_version: grant.result_version.map(ResultVersion::value),
        }
    }

    fn into_grant(self) -> M8CapabilityGrant {
        M8CapabilityGrant {
            reference: self.reference,
            active: self.active,
            scope: self.scope.map(PrivateCapabilityScopeSnapshot::into_scope),
            owner_locus: self.owner_locus,
            principal: self.principal,
            membership_ref: self.membership_ref,
            epoch: self.epoch,
            binding_epoch: self.binding_epoch,
            evaluator_locus: self.evaluator_locus,
            consumer_locus: self.consumer_locus,
            input_frontier: self.input_frontier,
            result_version: self.result_version.map(ResultVersion::new),
        }
    }
}

impl PrivateCapabilityScopeSnapshot {
    fn from_scope(scope: &M8CapabilityScope) -> Self {
        match scope {
            M8CapabilityScope::OwnerEvaluation { evaluation } => Self::OwnerEvaluation {
                evaluation: evaluation.clone(),
            },
            M8CapabilityScope::RelationTransition {
                relation,
                transition,
            } => Self::RelationTransition {
                relation: relation.clone(),
                transition: transition.clone(),
            },
            M8CapabilityScope::DesignatedEvaluation { evaluator, result } => {
                Self::DesignatedEvaluation {
                    evaluator: evaluator.clone(),
                    result: result.clone(),
                }
            }
            M8CapabilityScope::DesignatedConsumption {
                consumer,
                value_name,
            } => Self::DesignatedConsumption {
                consumer: consumer.clone(),
                value_name: value_name.clone(),
            },
            M8CapabilityScope::PatchActivation { program_identity } => Self::PatchActivation {
                program_identity: program_identity.clone(),
            },
        }
    }

    fn into_scope(self) -> M8CapabilityScope {
        match self {
            Self::OwnerEvaluation { evaluation } => {
                M8CapabilityScope::OwnerEvaluation { evaluation }
            }
            Self::RelationTransition {
                relation,
                transition,
            } => M8CapabilityScope::RelationTransition {
                relation,
                transition,
            },
            Self::DesignatedEvaluation { evaluator, result } => {
                M8CapabilityScope::DesignatedEvaluation { evaluator, result }
            }
            Self::DesignatedConsumption {
                consumer,
                value_name,
            } => M8CapabilityScope::DesignatedConsumption {
                consumer,
                value_name,
            },
            Self::PatchActivation { program_identity } => {
                M8CapabilityScope::PatchActivation { program_identity }
            }
        }
    }
}

impl PrivateWitnessRecordSnapshot {
    fn from_record(record: &M8WitnessRecord) -> Self {
        Self {
            reference: record.reference.clone(),
            live: record.live,
            capability_ref: record.capability_ref.clone(),
            membership_ref: record.membership_ref.clone(),
            epoch: record.epoch.clone(),
        }
    }

    fn into_record(self) -> M8WitnessRecord {
        M8WitnessRecord {
            reference: self.reference,
            live: self.live,
            capability_ref: self.capability_ref,
            membership_ref: self.membership_ref,
            epoch: self.epoch,
        }
    }
}

/// Borrowed relation-authority references supplied by a typed M8 transition.
/// This is a validation request only, not an authority carrier or issuer.
pub(crate) struct M8RelationAuthorityLookup<'a> {
    pub(crate) relation: &'a str,
    pub(crate) transition: &'a str,
    pub(crate) owner_locus: &'a str,
    pub(crate) principal: &'a str,
    pub(crate) membership_ref: Option<&'a str>,
    pub(crate) capability_ref: Option<&'a str>,
    pub(crate) membership_epoch: Option<&'a str>,
    pub(crate) binding_epoch: Option<&'a str>,
    pub(crate) witness_ref: Option<&'a str>,
    pub(crate) witness_epoch: Option<&'a str>,
}

/// Borrowed evaluator authority use resolved against admitted records.
pub(crate) struct M8DesignatedEvaluationAuthorityLookup<'a> {
    pub(crate) evaluator: &'a str,
    pub(crate) result: &'a str,
    pub(crate) input_frontier: &'a str,
    pub(crate) principal: &'a str,
    pub(crate) membership_ref: Option<&'a str>,
    pub(crate) capability_ref: Option<&'a str>,
    pub(crate) witness_ref: Option<&'a str>,
}

/// Borrowed consumer authority use resolved against admitted records.
pub(crate) struct M8DesignatedConsumptionAuthorityLookup<'a> {
    pub(crate) consumer: &'a str,
    pub(crate) value_name: &'a str,
    pub(crate) result_version: ResultVersion,
    pub(crate) principal: &'a str,
    pub(crate) membership_ref: Option<&'a str>,
    pub(crate) capability_ref: Option<&'a str>,
    pub(crate) witness_ref: Option<&'a str>,
}

/// Borrowed patch-activation authority resolved against already-admitted M8
/// records.  Provider/package strings are intentionally absent.
pub(crate) struct M8PatchActivationAuthorityLookup<'a> {
    pub(crate) program_identity: &'a str,
    pub(crate) owner_locus: &'a str,
    pub(crate) principal: &'a str,
    pub(crate) membership_ref: Option<&'a str>,
    pub(crate) capability_ref: Option<&'a str>,
    pub(crate) witness_ref: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum M8AuthorityValidationFailure {
    StaleMembership,
    MissingCapability,
    MissingWitness,
}
