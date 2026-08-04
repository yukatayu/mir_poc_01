//! Finite, parser-free reference calculus for M3 evaluation and materialization.
//!
//! This module is deliberately independent of the legacy AST and interpreter
//! paths.  It models only the bounded M3 carrier: a typed [`EvalPlan`], the
//! owner-serial mutable transition, explicit remote receipts, and stable
//! designated-value materialization.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

macro_rules! named_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

named_id!(Locus);
named_id!(Principal);
named_id!(Provider);
named_id!(CapabilityRef);
named_id!(OccurrenceId);
named_id!(OperationKey);

/// The semantic kind of an evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticForm {
    Value,
    State,
    Relation,
    Computation,
}

/// The locus at which a plan evaluates, independently of its authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationSite {
    Owner(Locus),
    Locus(Locus),
    DesignatedEvaluator(Locus),
    Consumer(Principal),
    Provider(Provider),
}

/// The plan's explicit clock or trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerClock {
    OnRequest,
    OnEvent,
    OnChange,
    LogicalTick,
    FrontierAdvance,
    PresentationFrame,
    Explicit,
}

/// The origin of authority, which is intentionally distinct from evaluation site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityOrigin {
    Caller(Principal),
    OwnerTransition(Locus),
    AdmittedEvaluator(Locus),
    AdmittedProvider(Provider),
}

/// A finite M3 materialization target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Materialization {
    LocalOnly,
    Store,
    PublishValue,
    PublishRelation,
    AdapterStream,
    Persist,
}

impl Materialization {
    const fn canonical_rank(self) -> u8 {
        match self {
            Self::LocalOnly => 0,
            Self::Store => 1,
            Self::PublishValue => 2,
            Self::PublishRelation => 3,
            Self::AdapterStream => 4,
            Self::Persist => 5,
        }
    }
}

/// A finite declared policy that fixes how a designated evaluator decides a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationPolicy {
    pub name: String,
    pub deterministic: bool,
}

impl EvaluationPolicy {
    pub fn declared_deterministic(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            deterministic: true,
        }
    }

    pub fn stamp_with(&self, observation_policy: &ObservationPolicy) -> PolicyStamp {
        PolicyStamp {
            evaluation_policy: self.clone(),
            observation_policy: observation_policy.clone(),
        }
    }
}

/// A finite declared policy that fixes the admitted observation of a result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationPolicy {
    pub name: String,
}

impl ObservationPolicy {
    pub fn declared(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Immutable evidence of the evaluation and observation policies used for a result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyStamp {
    pub evaluation_policy: EvaluationPolicy,
    pub observation_policy: ObservationPolicy,
}

/// A canonical finite producer set that supports a result.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputFrontier {
    producers: Vec<OccurrenceId>,
}

impl InputFrontier {
    /// Canonicalizes producer order while rejecting an empty or duplicate frontier.
    pub fn from_ordered_producers(mut producers: Vec<OccurrenceId>) -> Result<Self, Diagnostic> {
        if producers.is_empty() {
            return Err(Diagnostic::new(
                DiagnosticCode::EmptyInputFrontier,
                None,
                None,
            ));
        }

        producers.sort();
        for producers_pair in producers.windows(2) {
            if producers_pair[0] == producers_pair[1] {
                return Err(Diagnostic::new(
                    DiagnosticCode::DuplicateInputProducer,
                    None,
                    None,
                ));
            }
        }

        Ok(Self { producers })
    }

    pub fn as_slice(&self) -> &[OccurrenceId] {
        &self.producers
    }

    fn key_fragment(&self) -> String {
        self.producers
            .iter()
            .map(OccurrenceId::as_str)
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// A canonical finite materialization set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializationPlan {
    targets: Vec<Materialization>,
}

impl MaterializationPlan {
    /// Canonicalizes duplicates and rejects the M3-exclusive target combinations.
    pub fn canonical(
        targets: impl IntoIterator<Item = Materialization>,
    ) -> Result<Self, Diagnostic> {
        let mut targets: Vec<_> = targets.into_iter().collect();
        targets.sort_by_key(|target| target.canonical_rank());
        targets.dedup();

        if targets.is_empty() {
            return Err(Diagnostic::new(
                DiagnosticCode::MaterializationConflict,
                None,
                None,
            ));
        }
        if targets.contains(&Materialization::LocalOnly) && targets.len() > 1 {
            return Err(Diagnostic::new(
                DiagnosticCode::MaterializationConflict,
                None,
                None,
            ));
        }
        if targets.contains(&Materialization::PublishValue)
            && targets.contains(&Materialization::PublishRelation)
        {
            return Err(Diagnostic::new(
                DiagnosticCode::MaterializationConflict,
                None,
                None,
            ));
        }
        if targets.contains(&Materialization::AdapterStream)
            && targets.contains(&Materialization::Persist)
        {
            return Err(Diagnostic::new(
                DiagnosticCode::MaterializationConflict,
                None,
                None,
            ));
        }

        Ok(Self { targets })
    }

    pub fn as_slice(&self) -> &[Materialization] {
        &self.targets
    }
}

/// A source-free typed receipt produced for one target locus before a dependent eval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReceipt {
    pub id: String,
    pub producer: Locus,
    pub target: Locus,
    pub label: String,
    pub input_frontier: InputFrontier,
    pub type_name: String,
    outcome: ReceiptOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReceiptOutcome {
    I64(i64),
    Failure(DiagnosticCode),
}

impl RemoteReceipt {
    pub fn typed_i64(
        id: impl Into<String>,
        producer: Locus,
        target: Locus,
        label: impl Into<String>,
        input_frontier: InputFrontier,
        type_name: impl Into<String>,
        value: i64,
    ) -> Self {
        Self {
            id: id.into(),
            producer,
            target,
            label: label.into(),
            input_frontier,
            type_name: type_name.into(),
            outcome: ReceiptOutcome::I64(value),
        }
    }

    /// Builds a typed failure receipt.  It remains non-serviceable as an operand.
    pub fn typed_failure(
        id: impl Into<String>,
        producer: Locus,
        target: Locus,
        label: impl Into<String>,
        input_frontier: InputFrontier,
        type_name: impl Into<String>,
        code: DiagnosticCode,
    ) -> Self {
        Self {
            id: id.into(),
            producer,
            target,
            label: label.into(),
            input_frontier,
            type_name: type_name.into(),
            outcome: ReceiptOutcome::Failure(code),
        }
    }

    pub fn value_i64(&self) -> Option<i64> {
        match self.outcome {
            ReceiptOutcome::I64(value) => Some(value),
            ReceiptOutcome::Failure(_) => None,
        }
    }

    pub fn failure_code(&self) -> Option<DiagnosticCode> {
        match self.outcome {
            ReceiptOutcome::I64(_) => None,
            ReceiptOutcome::Failure(code) => Some(code),
        }
    }
}

/// An admitted but not yet served remote receipt request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReceiptRequest {
    operation_key: OperationKey,
    producer: Locus,
    target: Locus,
    id: String,
    label: String,
    input_frontier: InputFrontier,
    type_name: String,
}

/// A producer-served remote receipt request that has not yet been replied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServedRemoteReceiptRequest {
    request: RemoteReceiptRequest,
    value_i64: i64,
}

/// A typed reply that has not yet been received at its target locus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReceiptReply {
    receipt: RemoteReceipt,
    operation_key: OperationKey,
}

/// The finite typed carrier emitted by M3 elaboration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalPlan {
    pub operation_key: OperationKey,
    pub semantic_form: SemanticForm,
    pub evaluation_site: EvaluationSite,
    pub trigger: TriggerClock,
    pub authority_origin: AuthorityOrigin,
    pub materialization: MaterializationPlan,
    pub input_frontier: Option<InputFrontier>,
    pub remote_receipt: Option<RemoteReceipt>,
    pub evaluation_policy: Option<EvaluationPolicy>,
    pub observation_policy: Option<ObservationPolicy>,
    pub policy_stamp: Option<PolicyStamp>,
    pub requires_explicit_receipt: bool,
}

/// The finite diagnostics emitted during M3 elaboration or service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    EmptyInputFrontier,
    DuplicateInputProducer,
    MaterializationConflict,
    CrossOwnerOperand,
    AmbiguousEvaluation,
    MissingFrontier,
    ProviderMutation,
    ConsumerMutation,
    MissingReceipt,
    MissingCapability,
    MissingState,
    FailedReceipt,
    ReceiptProducerMismatch,
    ReceiptTargetMismatch,
    ReceiptReleaseDenied,
    MissingDesignatedValue,
    ConsumerConflict,
    UnsupportedOperation,
    DesignatedEvaluationFailed,
}

/// Typed diagnostic context.  It contains no source span or requester-private value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub evaluation_site: Option<EvaluationSite>,
    pub authority_origin: Option<AuthorityOrigin>,
}

impl Diagnostic {
    fn new(
        code: DiagnosticCode,
        evaluation_site: Option<EvaluationSite>,
        authority_origin: Option<AuthorityOrigin>,
    ) -> Self {
        Self {
            code,
            evaluation_site,
            authority_origin,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationTarget {
    owner: Locus,
    collection: String,
    target: Principal,
    field: String,
}

/// Syntax-free operation forms accepted by the bounded M3 reference model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    SameOwnerRmw {
        caller: Principal,
        requester_locus: Locus,
        target: MutationTarget,
        delta_i64: i64,
        capability: CapabilityRef,
    },
    OwnerRmwUsingRemoteReceipt {
        caller: Principal,
        requester_locus: Locus,
        target: MutationTarget,
        receipt: RemoteReceipt,
        capability: CapabilityRef,
    },
    ExplicitRemoteReceiptResult {
        caller: Principal,
        requester_locus: Locus,
        receipt_owner: Locus,
        result_key: String,
        target: Principal,
        field: String,
        capability: CapabilityRef,
        receipt: RemoteReceipt,
        trigger: TriggerClock,
    },
    DesignatedEvaluation {
        authority_origin: AuthorityOrigin,
        evaluator: Locus,
        key: String,
        input_frontier: InputFrontier,
        value_i64: i64,
        evaluation_policy: EvaluationPolicy,
        observation_policy: ObservationPolicy,
        trigger: TriggerClock,
    },
    FailingDesignatedEvaluation {
        authority_origin: AuthorityOrigin,
        evaluator: Locus,
        key: String,
        input_frontier: InputFrontier,
        failure: DiagnosticCode,
        evaluation_policy: EvaluationPolicy,
        observation_policy: ObservationPolicy,
        trigger: TriggerClock,
    },
    DesignatedEvaluationWithoutSite {
        caller: Principal,
        key: String,
        input_frontier: InputFrontier,
        evaluation_policy: EvaluationPolicy,
        observation_policy: ObservationPolicy,
        trigger: TriggerClock,
    },
    UnannotatedCrossOwnerOperand {
        caller: Principal,
        requester_locus: Locus,
        target: MutationTarget,
        operand_owner: Locus,
        delta_i64: i64,
        capability: CapabilityRef,
    },
}

impl Operation {
    #[allow(clippy::too_many_arguments)]
    pub fn same_owner_rmw(
        caller: Principal,
        requester_locus: Locus,
        owner: Locus,
        collection: impl Into<String>,
        target: Principal,
        field: impl Into<String>,
        delta_i64: i64,
        capability: CapabilityRef,
    ) -> Self {
        Self::SameOwnerRmw {
            caller,
            requester_locus,
            target: MutationTarget {
                owner,
                collection: collection.into(),
                target,
                field: field.into(),
            },
            delta_i64,
            capability,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn owner_rmw_using_remote_receipt(
        caller: Principal,
        requester_locus: Locus,
        owner: Locus,
        collection: impl Into<String>,
        target: Principal,
        field: impl Into<String>,
        receipt: RemoteReceipt,
        capability: CapabilityRef,
    ) -> Self {
        Self::OwnerRmwUsingRemoteReceipt {
            caller,
            requester_locus,
            target: MutationTarget {
                owner,
                collection: collection.into(),
                target,
                field: field.into(),
            },
            receipt,
            capability,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn explicit_remote_receipt_result(
        caller: Principal,
        requester_locus: Locus,
        receipt_owner: Locus,
        result_key: impl Into<String>,
        target: Principal,
        field: impl Into<String>,
        capability: CapabilityRef,
        receipt: RemoteReceipt,
        trigger: TriggerClock,
    ) -> Self {
        Self::ExplicitRemoteReceiptResult {
            caller,
            requester_locus,
            receipt_owner,
            result_key: result_key.into(),
            target,
            field: field.into(),
            capability,
            receipt,
            trigger,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn designated_evaluation(
        authority_origin: AuthorityOrigin,
        evaluator: Locus,
        key: impl Into<String>,
        input_frontier: InputFrontier,
        value_i64: i64,
        evaluation_policy: EvaluationPolicy,
        observation_policy: ObservationPolicy,
        trigger: TriggerClock,
    ) -> Self {
        Self::DesignatedEvaluation {
            authority_origin,
            evaluator,
            key: key.into(),
            input_frontier,
            value_i64,
            evaluation_policy,
            observation_policy,
            trigger,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn failing_designated_evaluation(
        authority_origin: AuthorityOrigin,
        evaluator: Locus,
        key: impl Into<String>,
        input_frontier: InputFrontier,
        failure: DiagnosticCode,
        evaluation_policy: EvaluationPolicy,
        observation_policy: ObservationPolicy,
        trigger: TriggerClock,
    ) -> Self {
        Self::FailingDesignatedEvaluation {
            authority_origin,
            evaluator,
            key: key.into(),
            input_frontier,
            failure,
            evaluation_policy,
            observation_policy,
            trigger,
        }
    }

    pub fn designated_evaluation_without_site(
        caller: Principal,
        key: impl Into<String>,
        input_frontier: InputFrontier,
        evaluation_policy: EvaluationPolicy,
        observation_policy: ObservationPolicy,
        trigger: TriggerClock,
    ) -> Self {
        Self::DesignatedEvaluationWithoutSite {
            caller,
            key: key.into(),
            input_frontier,
            evaluation_policy,
            observation_policy,
            trigger,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn unannotated_cross_owner_operand(
        caller: Principal,
        requester_locus: Locus,
        owner: Locus,
        operand_owner: Locus,
        collection: impl Into<String>,
        target: Principal,
        field: impl Into<String>,
        delta_i64: i64,
        capability: CapabilityRef,
    ) -> Self {
        Self::UnannotatedCrossOwnerOperand {
            caller,
            requester_locus,
            target: MutationTarget {
                owner,
                collection: collection.into(),
                target,
                field: field.into(),
            },
            operand_owner,
            delta_i64,
            capability,
        }
    }

    fn operation_key(&self) -> OperationKey {
        let key = match self {
            Self::SameOwnerRmw {
                caller,
                target,
                delta_i64,
                capability,
                ..
            } => format!(
                "owner-rmw|{caller}|{}|{}|{}|{}|{delta_i64}|{capability}",
                target.owner, target.collection, target.target, target.field
            ),
            Self::OwnerRmwUsingRemoteReceipt {
                caller,
                target,
                receipt,
                capability,
                ..
            } => format!(
                "owner-rmw-receipt|{caller}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{capability}",
                target.owner,
                target.collection,
                target.target,
                target.field,
                receipt.id,
                receipt.producer,
                receipt.target,
                receipt.label,
                receipt.input_frontier.key_fragment(),
                receipt.type_name,
            ),
            Self::ExplicitRemoteReceiptResult {
                caller,
                requester_locus,
                receipt_owner,
                result_key,
                target,
                field,
                capability,
                receipt,
                trigger,
            } => format!(
                "remote-result|{caller}|{requester_locus}|{receipt_owner}|{result_key}|{target}|{field}|{capability}|{}|{}|{}|{}|{}|{}|{trigger:?}",
                receipt.id,
                receipt.producer,
                receipt.target,
                receipt.label,
                receipt.input_frontier.key_fragment(),
                receipt.type_name,
            ),
            Self::DesignatedEvaluation {
                evaluator,
                key,
                input_frontier,
                value_i64,
                evaluation_policy,
                observation_policy,
                trigger,
                ..
            } => format!(
                "designated|{evaluator}|{key}|{}|{value_i64}|{}|{}|{trigger:?}",
                input_frontier.key_fragment(),
                evaluation_policy.name,
                observation_policy.name,
            ),
            Self::FailingDesignatedEvaluation {
                evaluator,
                key,
                input_frontier,
                failure,
                evaluation_policy,
                observation_policy,
                trigger,
                ..
            } => format!(
                "designated-failure|{evaluator}|{key}|{}|{failure:?}|{}|{}|{trigger:?}",
                input_frontier.key_fragment(),
                evaluation_policy.name,
                observation_policy.name,
            ),
            Self::DesignatedEvaluationWithoutSite {
                caller,
                key,
                input_frontier,
                evaluation_policy,
                observation_policy,
                trigger,
            } => format!(
                "designated-unbound|{caller}|{key}|{}|{}|{}|{trigger:?}",
                input_frontier.key_fragment(),
                evaluation_policy.name,
                observation_policy.name,
            ),
            Self::UnannotatedCrossOwnerOperand {
                caller,
                target,
                operand_owner,
                delta_i64,
                capability,
                ..
            } => format!(
                "cross-owner|{caller}|{}|{}|{}|{}|{operand_owner}|{delta_i64}|{capability}",
                target.owner, target.collection, target.target, target.field
            ),
        };
        OperationKey::new(key)
    }
}

fn checked_materialization(target: Materialization) -> MaterializationPlan {
    MaterializationPlan::canonical([target])
        .expect("a single finite M3 materialization target is always valid")
}

fn owner_transition_plan(
    operation_key: OperationKey,
    caller: &Principal,
    owner: &Locus,
    receipt: Option<RemoteReceipt>,
) -> EvalPlan {
    let input_frontier = receipt
        .as_ref()
        .map(|receipt| receipt.input_frontier.clone());
    EvalPlan {
        operation_key,
        semantic_form: SemanticForm::State,
        evaluation_site: EvaluationSite::Owner(owner.clone()),
        trigger: TriggerClock::OnRequest,
        authority_origin: AuthorityOrigin::Caller(caller.clone()),
        materialization: checked_materialization(Materialization::Store),
        input_frontier,
        remote_receipt: receipt.clone(),
        evaluation_policy: None,
        observation_policy: None,
        policy_stamp: None,
        requires_explicit_receipt: receipt.is_some(),
    }
}

fn designated_plan(
    operation_key: OperationKey,
    authority_origin: &AuthorityOrigin,
    evaluator: &Locus,
    input_frontier: &InputFrontier,
    evaluation_policy: &EvaluationPolicy,
    observation_policy: &ObservationPolicy,
    trigger: TriggerClock,
) -> EvalPlan {
    EvalPlan {
        operation_key,
        semantic_form: SemanticForm::Value,
        evaluation_site: EvaluationSite::DesignatedEvaluator(evaluator.clone()),
        trigger,
        authority_origin: authority_origin.clone(),
        materialization: checked_materialization(Materialization::PublishValue),
        input_frontier: Some(input_frontier.clone()),
        remote_receipt: None,
        evaluation_policy: Some(evaluation_policy.clone()),
        observation_policy: Some(observation_policy.clone()),
        policy_stamp: Some(evaluation_policy.stamp_with(observation_policy)),
        requires_explicit_receipt: false,
    }
}

/// Deterministically elaborates the finite operation forms supported in M3.
pub fn infer_plan(operation: &Operation) -> Result<EvalPlan, Diagnostic> {
    match operation {
        Operation::SameOwnerRmw { caller, target, .. } => Ok(owner_transition_plan(
            operation.operation_key(),
            caller,
            &target.owner,
            None,
        )),
        Operation::OwnerRmwUsingRemoteReceipt {
            caller,
            target,
            receipt,
            ..
        } => Ok(owner_transition_plan(
            operation.operation_key(),
            caller,
            &target.owner,
            Some(receipt.clone()),
        )),
        Operation::ExplicitRemoteReceiptResult {
            caller,
            requester_locus,
            receipt_owner,
            receipt,
            trigger,
            ..
        } => {
            if receipt.producer != *receipt_owner {
                return Err(Diagnostic::new(
                    DiagnosticCode::ReceiptProducerMismatch,
                    Some(EvaluationSite::Owner(receipt_owner.clone())),
                    Some(AuthorityOrigin::Caller(caller.clone())),
                ));
            }
            if receipt.target != *requester_locus {
                return Err(Diagnostic::new(
                    DiagnosticCode::ReceiptTargetMismatch,
                    Some(EvaluationSite::Owner(receipt_owner.clone())),
                    Some(AuthorityOrigin::Caller(caller.clone())),
                ));
            }
            Ok(EvalPlan {
                operation_key: operation.operation_key(),
                semantic_form: SemanticForm::Value,
                evaluation_site: EvaluationSite::Owner(receipt_owner.clone()),
                trigger: *trigger,
                authority_origin: AuthorityOrigin::Caller(caller.clone()),
                materialization: checked_materialization(Materialization::PublishValue),
                input_frontier: Some(receipt.input_frontier.clone()),
                remote_receipt: Some(receipt.clone()),
                evaluation_policy: None,
                observation_policy: None,
                policy_stamp: None,
                requires_explicit_receipt: true,
            })
        }
        Operation::DesignatedEvaluation {
            authority_origin,
            evaluator,
            input_frontier,
            evaluation_policy,
            observation_policy,
            trigger,
            ..
        }
        | Operation::FailingDesignatedEvaluation {
            authority_origin,
            evaluator,
            input_frontier,
            evaluation_policy,
            observation_policy,
            trigger,
            ..
        } => match authority_origin {
            AuthorityOrigin::AdmittedEvaluator(admitted) if admitted == evaluator => {
                Ok(designated_plan(
                    operation.operation_key(),
                    authority_origin,
                    evaluator,
                    input_frontier,
                    evaluation_policy,
                    observation_policy,
                    *trigger,
                ))
            }
            _ => Err(Diagnostic::new(
                DiagnosticCode::AmbiguousEvaluation,
                None,
                Some(authority_origin.clone()),
            )),
        },
        Operation::DesignatedEvaluationWithoutSite { caller, .. } => Err(Diagnostic::new(
            DiagnosticCode::AmbiguousEvaluation,
            None,
            Some(AuthorityOrigin::Caller(caller.clone())),
        )),
        Operation::UnannotatedCrossOwnerOperand { caller, target, .. } => Err(Diagnostic::new(
            DiagnosticCode::CrossOwnerOperand,
            Some(EvaluationSite::Owner(target.owner.clone())),
            Some(AuthorityOrigin::Caller(caller.clone())),
        )),
    }
}

/// Trace row kind for the finite reference model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceEntryKind {
    OwnerRequestQueued,
    OwnerServiceAccepted,
    OwnerServiceRejected(DiagnosticCode),
    RemoteReceiptRequested,
    RemoteReceiptServed,
    RemoteReceiptReplied,
    RemoteReceiptReceived,
    ValuePublished,
    SemanticConsumption,
    DuplicatePublicationObserved,
    DesignatedEvaluationRejected(DiagnosticCode),
    Diagnostic(DiagnosticCode),
}

/// Typed, source-free information attached to a trace row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceDetail {
    None,
    OwnerMutation {
        owner: Locus,
        collection: String,
        target: Principal,
        field: String,
    },
    Receipt(RemoteReceipt),
    PublishedValue(DecidedValue),
    SemanticConsumption(DesignatedConsumption),
    Failure(Diagnostic),
}

/// A single observation in the bounded execution trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEntry {
    pub operation_key: OperationKey,
    pub kind: TraceEntryKind,
    pub eval_plan: Option<EvalPlan>,
    pub detail: TraceDetail,
    pub receipt_producer: Option<Locus>,
    pub receipt_target: Option<Locus>,
    pub receipt_label: Option<String>,
    /// M3 deliberately never releases an owner-private mutable operand here.
    pub requester_private_value_result: Option<i64>,
}

impl TraceEntry {
    fn planned(kind: TraceEntryKind, eval_plan: EvalPlan, detail: TraceDetail) -> Self {
        Self {
            operation_key: eval_plan.operation_key.clone(),
            kind,
            eval_plan: Some(eval_plan),
            detail,
            receipt_producer: None,
            receipt_target: None,
            receipt_label: None,
            requester_private_value_result: None,
        }
    }

    fn diagnostic(
        operation_key: OperationKey,
        diagnostic: Diagnostic,
        eval_plan: Option<EvalPlan>,
    ) -> Self {
        Self {
            operation_key,
            kind: TraceEntryKind::Diagnostic(diagnostic.code),
            eval_plan,
            detail: TraceDetail::Failure(diagnostic),
            receipt_producer: None,
            receipt_target: None,
            receipt_label: None,
            requester_private_value_result: None,
        }
    }

    fn receipt_metadata(
        kind: TraceEntryKind,
        operation_key: OperationKey,
        receipt_producer: Locus,
        receipt_target: Locus,
        receipt_label: String,
        detail: TraceDetail,
    ) -> Self {
        Self {
            operation_key,
            kind,
            eval_plan: None,
            detail,
            receipt_producer: Some(receipt_producer),
            receipt_target: Some(receipt_target),
            receipt_label: Some(receipt_label),
            requester_private_value_result: None,
        }
    }
}

fn receipt_stage_key(operation_key: &OperationKey, stage: &str) -> OperationKey {
    OperationKey::new(format!("{}|{stage}", operation_key.as_str()))
}

/// A stable designated result at one evaluator/key/frontier tuple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecidedValue {
    pub key: String,
    pub input_frontier: InputFrontier,
    pub version: u64,
    pub value_i64: i64,
    pub evaluation_policy: EvaluationPolicy,
    pub observation_policy: ObservationPolicy,
    pub policy_stamp: PolicyStamp,
}

/// One explicit semantic consumption under the finite single-consumer M3 profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedConsumption {
    pub identity: OperationKey,
    pub consumer: Principal,
    pub key: String,
    pub input_frontier: InputFrontier,
    pub policy_stamp: PolicyStamp,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StateKey {
    owner: Locus,
    collection: String,
    target: Principal,
    field: String,
}

impl From<&MutationTarget> for StateKey {
    fn from(target: &MutationTarget) -> Self {
        Self {
            owner: target.owner.clone(),
            collection: target.collection.clone(),
            target: target.target.clone(),
            field: target.field.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapabilityGrant {
    caller: Principal,
    owner: Locus,
    collection: String,
    field: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ReceiptReleaseGrant {
    caller: Principal,
    producer: Locus,
    target: Locus,
    label: String,
}

#[derive(Debug, Clone)]
struct QueuedOperation {
    operation: Operation,
    plan: EvalPlan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DesignatedKey {
    evaluator: Locus,
    key: String,
    input_frontier: InputFrontier,
}

/// In-memory execution evidence for the finite M3 reference calculus.
#[derive(Debug, Default)]
pub struct EvaluationMaterializationHarness {
    state: BTreeMap<StateKey, i64>,
    capabilities: BTreeMap<CapabilityRef, CapabilityGrant>,
    receipt_releases: BTreeSet<ReceiptReleaseGrant>,
    queues: BTreeMap<Locus, VecDeque<QueuedOperation>>,
    pending_receipt_requests: BTreeMap<OperationKey, RemoteReceiptRequest>,
    served_receipt_requests: BTreeMap<OperationKey, ServedRemoteReceiptRequest>,
    pending_receipt_replies: BTreeMap<OperationKey, RemoteReceiptReply>,
    received_receipts: BTreeMap<String, RemoteReceipt>,
    decisions: BTreeMap<DesignatedKey, DecidedValue>,
    consumptions: BTreeMap<DesignatedKey, DesignatedConsumption>,
    trace: Vec<TraceEntry>,
    next_capability: u64,
    next_version: u64,
}

impl EvaluationMaterializationHarness {
    pub fn set_i64_state(
        &mut self,
        owner: Locus,
        collection: impl Into<String>,
        target: Principal,
        field: impl Into<String>,
        value: i64,
    ) {
        self.state.insert(
            StateKey {
                owner,
                collection: collection.into(),
                target,
                field: field.into(),
            },
            value,
        );
    }

    pub fn i64_state(
        &self,
        owner: &Locus,
        collection: &str,
        target: &Principal,
        field: &str,
    ) -> Option<i64> {
        self.state
            .get(&StateKey {
                owner: owner.clone(),
                collection: collection.to_owned(),
                target: target.clone(),
                field: field.to_owned(),
            })
            .copied()
    }

    pub fn grant_capability(
        &mut self,
        caller: Principal,
        owner: Locus,
        collection: impl Into<String>,
        field: impl Into<String>,
    ) -> CapabilityRef {
        self.next_capability += 1;
        let capability = CapabilityRef::new(format!("m3-capability-{}", self.next_capability));
        self.capabilities.insert(
            capability.clone(),
            CapabilityGrant {
                caller,
                owner,
                collection: collection.into(),
                field: field.into(),
            },
        );
        capability
    }

    /// Admits release of one labelled receipt independently of write capability.
    pub fn grant_receipt_release(
        &mut self,
        caller: Principal,
        producer: Locus,
        target: Locus,
        label: impl Into<String>,
    ) {
        self.receipt_releases.insert(ReceiptReleaseGrant {
            caller,
            producer,
            target,
            label: label.into(),
        });
    }

    /// Admits the first, request-side stage of a typed remote receipt.
    #[allow(clippy::too_many_arguments)]
    pub fn request_remote_i64_receipt(
        &mut self,
        caller: Principal,
        producer: Locus,
        target: Locus,
        id: impl Into<String>,
        label: impl Into<String>,
        input_frontier: InputFrontier,
        type_name: impl Into<String>,
    ) -> Result<RemoteReceiptRequest, Diagnostic> {
        let id = id.into();
        let label = label.into();
        let type_name = type_name.into();
        if !self.receipt_releases.contains(&ReceiptReleaseGrant {
            caller: caller.clone(),
            producer: producer.clone(),
            target: target.clone(),
            label: label.clone(),
        }) {
            return Err(Diagnostic::new(
                DiagnosticCode::ReceiptReleaseDenied,
                None,
                None,
            ));
        }
        let operation_key = OperationKey::new(format!(
            "remote-receipt-request|{caller}|{producer}|{target}|{id}|{label}|{}|{type_name}",
            input_frontier.key_fragment()
        ));
        if self.pending_receipt_requests.contains_key(&operation_key)
            || self.served_receipt_requests.contains_key(&operation_key)
            || self.pending_receipt_replies.contains_key(&operation_key)
        {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        }

        let request = RemoteReceiptRequest {
            operation_key: operation_key.clone(),
            producer: producer.clone(),
            target: target.clone(),
            id,
            label: label.clone(),
            input_frontier,
            type_name,
        };
        self.pending_receipt_requests
            .insert(operation_key.clone(), request.clone());
        self.trace.push(TraceEntry::receipt_metadata(
            TraceEntryKind::RemoteReceiptRequested,
            operation_key,
            producer,
            target,
            label,
            TraceDetail::None,
        ));
        Ok(request)
    }

    /// Serves one admitted remote receipt request at its named producer.
    pub fn serve_remote_i64_receipt_request(
        &mut self,
        request: RemoteReceiptRequest,
        value_i64: i64,
    ) -> Result<ServedRemoteReceiptRequest, Diagnostic> {
        let operation_key = request.operation_key.clone();
        let Some(expected) = self.pending_receipt_requests.remove(&operation_key) else {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        };
        if expected != request {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        }

        let served = ServedRemoteReceiptRequest { request, value_i64 };
        self.served_receipt_requests
            .insert(operation_key.clone(), served.clone());
        self.trace.push(TraceEntry::receipt_metadata(
            TraceEntryKind::RemoteReceiptServed,
            receipt_stage_key(&operation_key, "served"),
            served.request.producer.clone(),
            served.request.target.clone(),
            served.request.label.clone(),
            TraceDetail::None,
        ));
        Ok(served)
    }

    /// Turns one producer-served request into a typed reply.
    pub fn reply_remote_i64_receipt(
        &mut self,
        served: ServedRemoteReceiptRequest,
    ) -> Result<RemoteReceiptReply, Diagnostic> {
        let operation_key = served.request.operation_key.clone();
        let Some(expected) = self.served_receipt_requests.remove(&operation_key) else {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        };
        if expected != served {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        }

        let receipt = RemoteReceipt::typed_i64(
            served.request.id.clone(),
            served.request.producer.clone(),
            served.request.target.clone(),
            served.request.label.clone(),
            served.request.input_frontier.clone(),
            served.request.type_name.clone(),
            served.value_i64,
        );
        let reply = RemoteReceiptReply {
            receipt: receipt.clone(),
            operation_key: operation_key.clone(),
        };
        self.pending_receipt_replies
            .insert(operation_key.clone(), reply.clone());
        self.trace.push(TraceEntry::receipt_metadata(
            TraceEntryKind::RemoteReceiptReplied,
            receipt_stage_key(&operation_key, "replied"),
            receipt.producer.clone(),
            receipt.target.clone(),
            receipt.label.clone(),
            TraceDetail::Receipt(receipt),
        ));
        Ok(reply)
    }

    /// Receives a reply and makes it available to a later target-owned transition.
    pub fn receive_remote_receipt(
        &mut self,
        reply: RemoteReceiptReply,
    ) -> Result<RemoteReceipt, Diagnostic> {
        let operation_key = reply.operation_key.clone();
        let Some(expected) = self.pending_receipt_replies.remove(&operation_key) else {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        };
        if expected != reply || self.received_receipts.contains_key(&reply.receipt.id) {
            return Err(Diagnostic::new(DiagnosticCode::MissingReceipt, None, None));
        }

        let receipt = reply.receipt;
        self.received_receipts
            .insert(receipt.id.clone(), receipt.clone());
        self.trace.push(TraceEntry::receipt_metadata(
            TraceEntryKind::RemoteReceiptReceived,
            receipt_stage_key(&operation_key, "received"),
            receipt.producer.clone(),
            receipt.target.clone(),
            receipt.label.clone(),
            TraceDetail::Receipt(receipt.clone()),
        ));
        Ok(receipt)
    }

    /// A manually supplied receipt never enters the causal receipt store.
    pub fn record_remote_receipt(&mut self, _receipt: RemoteReceipt) -> bool {
        false
    }

    /// Elaborates and, for owner transitions, appends an operation to that owner's FIFO.
    pub fn enqueue(&mut self, operation: Operation) -> Result<(), Diagnostic> {
        let operation_key = operation.operation_key();
        let plan = match infer_plan(&operation) {
            Ok(plan) => plan,
            Err(diagnostic) => {
                self.trace.push(TraceEntry::diagnostic(
                    operation_key,
                    diagnostic.clone(),
                    None,
                ));
                return Err(diagnostic);
            }
        };

        let owner = match &operation {
            Operation::SameOwnerRmw { target, .. }
            | Operation::OwnerRmwUsingRemoteReceipt { target, .. } => target.owner.clone(),
            _ => {
                let diagnostic = Diagnostic::new(
                    DiagnosticCode::UnsupportedOperation,
                    Some(plan.evaluation_site.clone()),
                    Some(plan.authority_origin.clone()),
                );
                self.trace.push(TraceEntry::diagnostic(
                    plan.operation_key.clone(),
                    diagnostic.clone(),
                    Some(plan),
                ));
                return Err(diagnostic);
            }
        };

        if let Operation::OwnerRmwUsingRemoteReceipt { receipt, .. } = &operation {
            if self.received_receipts.get(&receipt.id) != Some(receipt) {
                let diagnostic = Diagnostic::new(
                    DiagnosticCode::MissingReceipt,
                    Some(plan.evaluation_site.clone()),
                    Some(plan.authority_origin.clone()),
                );
                self.trace.push(TraceEntry::diagnostic(
                    plan.operation_key.clone(),
                    diagnostic.clone(),
                    Some(plan),
                ));
                return Err(diagnostic);
            }
            if receipt.target != owner {
                let diagnostic = Diagnostic::new(
                    DiagnosticCode::ReceiptTargetMismatch,
                    Some(plan.evaluation_site.clone()),
                    Some(plan.authority_origin.clone()),
                );
                self.trace.push(TraceEntry::diagnostic(
                    plan.operation_key.clone(),
                    diagnostic.clone(),
                    Some(plan),
                ));
                return Err(diagnostic);
            }
            if receipt.value_i64().is_none() {
                let diagnostic = Diagnostic::new(
                    DiagnosticCode::FailedReceipt,
                    Some(plan.evaluation_site.clone()),
                    Some(plan.authority_origin.clone()),
                );
                self.trace.push(TraceEntry::diagnostic(
                    plan.operation_key.clone(),
                    diagnostic.clone(),
                    Some(plan),
                ));
                return Err(diagnostic);
            }
        }

        self.queues
            .entry(owner)
            .or_default()
            .push_back(QueuedOperation {
                operation,
                plan: plan.clone(),
            });
        self.trace.push(TraceEntry::planned(
            TraceEntryKind::OwnerRequestQueued,
            plan,
            TraceDetail::None,
        ));
        Ok(())
    }

    /// Services every currently queued transition for `owner` in FIFO order.
    pub fn service_owner_queue(&mut self, owner: &Locus) {
        loop {
            let queued = self.queues.get_mut(owner).and_then(VecDeque::pop_front);
            let Some(queued) = queued else {
                break;
            };
            self.service_queued_owner_operation(queued);
        }
    }

    pub fn trace(&self) -> &[TraceEntry] {
        &self.trace
    }

    /// Decides a designated value once for each evaluator/key/frontier tuple.
    pub fn evaluate_designated(
        &mut self,
        operation: Operation,
    ) -> Result<DecidedValue, Diagnostic> {
        let operation_key = operation.operation_key();
        let plan = match infer_plan(&operation) {
            Ok(plan) => plan,
            Err(diagnostic) => {
                self.trace.push(TraceEntry::diagnostic(
                    operation_key,
                    diagnostic.clone(),
                    None,
                ));
                return Err(diagnostic);
            }
        };

        let (evaluator, key, input_frontier, value_i64, failure) = match operation {
            Operation::DesignatedEvaluation {
                evaluator,
                key,
                input_frontier,
                value_i64,
                ..
            } => (evaluator, key, input_frontier, Some(value_i64), None),
            Operation::FailingDesignatedEvaluation {
                evaluator,
                key,
                input_frontier,
                failure,
                ..
            } => (evaluator, key, input_frontier, None, Some(failure)),
            _ => {
                let diagnostic = Diagnostic::new(
                    DiagnosticCode::UnsupportedOperation,
                    Some(plan.evaluation_site.clone()),
                    Some(plan.authority_origin.clone()),
                );
                self.trace.push(TraceEntry::diagnostic(
                    plan.operation_key.clone(),
                    diagnostic.clone(),
                    Some(plan),
                ));
                return Err(diagnostic);
            }
        };

        let decision_key = DesignatedKey {
            evaluator,
            key: key.clone(),
            input_frontier: input_frontier.clone(),
        };
        if let Some(existing) = self.decisions.get(&decision_key) {
            let existing = existing.clone();
            self.trace.push(TraceEntry::planned(
                TraceEntryKind::DuplicatePublicationObserved,
                plan,
                TraceDetail::PublishedValue(existing.clone()),
            ));
            return Ok(existing);
        }

        if let Some(failure) = failure {
            let diagnostic = Diagnostic::new(
                failure,
                Some(plan.evaluation_site.clone()),
                Some(plan.authority_origin.clone()),
            );
            self.trace.push(TraceEntry::planned(
                TraceEntryKind::DesignatedEvaluationRejected(failure),
                plan,
                TraceDetail::Failure(diagnostic.clone()),
            ));
            return Err(diagnostic);
        }

        self.next_version += 1;
        let decided = DecidedValue {
            key,
            input_frontier,
            version: self.next_version,
            value_i64: value_i64.expect("successful designated operation contains a value"),
            evaluation_policy: plan
                .evaluation_policy
                .clone()
                .expect("designated plans retain an evaluation policy"),
            observation_policy: plan
                .observation_policy
                .clone()
                .expect("designated plans retain an observation policy"),
            policy_stamp: plan
                .policy_stamp
                .clone()
                .expect("designated plans retain a policy stamp"),
        };
        self.decisions.insert(decision_key, decided.clone());
        self.trace.push(TraceEntry::planned(
            TraceEntryKind::ValuePublished,
            plan,
            TraceDetail::PublishedValue(decided.clone()),
        ));
        Ok(decided)
    }

    /// Records the one permitted consumer of a designated M3 value.
    pub fn consume_designated(
        &mut self,
        evaluator: Locus,
        key: impl Into<String>,
        input_frontier: InputFrontier,
        consumer: Principal,
    ) -> Result<DesignatedConsumption, Diagnostic> {
        let key = key.into();
        let decision_key = DesignatedKey {
            evaluator: evaluator.clone(),
            key: key.clone(),
            input_frontier: input_frontier.clone(),
        };
        let operation_key = OperationKey::new(format!(
            "designated-consume|{evaluator}|{key}|{}|{consumer}",
            input_frontier.key_fragment()
        ));

        if let Some(existing) = self.consumptions.get(&decision_key) {
            if existing.consumer == consumer {
                return Ok(existing.clone());
            }
            let diagnostic = Diagnostic::new(DiagnosticCode::ConsumerConflict, None, None);
            self.trace.push(TraceEntry::diagnostic(
                operation_key,
                diagnostic.clone(),
                None,
            ));
            return Err(diagnostic);
        }

        let Some(decision) = self.decisions.get(&decision_key) else {
            let diagnostic = Diagnostic::new(DiagnosticCode::MissingDesignatedValue, None, None);
            self.trace.push(TraceEntry::diagnostic(
                operation_key,
                diagnostic.clone(),
                None,
            ));
            return Err(diagnostic);
        };
        let consumption = DesignatedConsumption {
            identity: operation_key.clone(),
            consumer,
            key,
            input_frontier,
            policy_stamp: decision.policy_stamp.clone(),
        };
        self.consumptions.insert(decision_key, consumption.clone());
        self.trace.push(TraceEntry {
            operation_key,
            kind: TraceEntryKind::SemanticConsumption,
            eval_plan: None,
            detail: TraceDetail::SemanticConsumption(consumption.clone()),
            receipt_producer: None,
            receipt_target: None,
            receipt_label: None,
            requester_private_value_result: None,
        });
        Ok(consumption)
    }

    pub fn published_values_for(
        &self,
        key: &str,
        input_frontier: &InputFrontier,
    ) -> Vec<DecidedValue> {
        self.decisions
            .values()
            .filter(|decision| decision.key == key && &decision.input_frontier == input_frontier)
            .cloned()
            .collect()
    }

    pub fn semantic_consumption_count(&self, key: &str, input_frontier: &InputFrontier) -> usize {
        self.trace
            .iter()
            .filter(|entry| {
                entry.kind == TraceEntryKind::SemanticConsumption
                    && matches!(
                        &entry.detail,
                        TraceDetail::SemanticConsumption(consumption)
                            if consumption.key == key
                                && &consumption.input_frontier == input_frontier
                    )
            })
            .count()
    }

    fn service_queued_owner_operation(&mut self, queued: QueuedOperation) {
        let (caller, target, capability, receipt_delta) = match queued.operation {
            Operation::SameOwnerRmw {
                caller,
                target,
                delta_i64,
                capability,
                ..
            } => (caller, target, capability, delta_i64),
            Operation::OwnerRmwUsingRemoteReceipt {
                caller,
                target,
                receipt,
                capability,
                ..
            } => (
                caller,
                target,
                capability,
                receipt
                    .value_i64()
                    .expect("only serviceable typed receipt transitions enter the owner queue"),
            ),
            _ => unreachable!("only owner transitions can enter an owner queue"),
        };

        if !self.has_capability(&capability, &caller, &target) {
            self.record_owner_rejection(queued.plan, DiagnosticCode::MissingCapability);
            return;
        }

        let state_key = StateKey::from(&target);
        let Some(previous) = self.state.get(&state_key).copied() else {
            self.record_owner_rejection(queued.plan, DiagnosticCode::MissingState);
            return;
        };
        let next = previous.saturating_add(receipt_delta);
        self.state.insert(state_key, next);
        self.trace.push(TraceEntry::planned(
            TraceEntryKind::OwnerServiceAccepted,
            queued.plan,
            TraceDetail::OwnerMutation {
                owner: target.owner,
                collection: target.collection,
                target: target.target,
                field: target.field,
            },
        ));
    }

    fn has_capability(
        &self,
        capability: &CapabilityRef,
        caller: &Principal,
        target: &MutationTarget,
    ) -> bool {
        self.capabilities.get(capability).is_some_and(|grant| {
            grant.caller == *caller
                && grant.owner == target.owner
                && grant.collection == target.collection
                && grant.field == target.field
        })
    }

    fn record_owner_rejection(&mut self, plan: EvalPlan, code: DiagnosticCode) {
        let diagnostic = Diagnostic::new(
            code,
            Some(plan.evaluation_site.clone()),
            Some(plan.authority_origin.clone()),
        );
        self.trace.push(TraceEntry::planned(
            TraceEntryKind::OwnerServiceRejected(code),
            plan,
            TraceDetail::Failure(diagnostic),
        ));
    }
}
