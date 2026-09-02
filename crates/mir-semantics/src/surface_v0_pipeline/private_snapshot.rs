//! Private, versioned snapshots for already checked M7 semantic facts.
//!
//! This module is deliberately not a public wire or package format. Its only
//! direct consumer is the I3 process-image bridge, which must carry checked
//! facts without parsing source, re-checking Core, or granting authority.

use serde::{Deserialize, Serialize};

use crate::{
    evaluation_materialization::{
        AuthorityOrigin, EvaluationPolicy, EvaluationSite, InputFrontier, Locus, ObservationPolicy,
        OccurrenceId as MaterializationOccurrenceId, PolicyStamp, Principal, Provider,
        StaticRetryContractKind,
    },
    shared_model::{
        BindingActivationFrontier, OccurrenceId as SharedOccurrenceId, ResultFrontier, ResultKey,
        ResultVersion, SourceRef,
    },
};

use super::{
    CheckedBinaryOperator, CheckedEvaluationKind, CheckedEvaluationParameter,
    CheckedEvaluationSignature, CheckedExpressionTree, CheckedIndexedStateSchema,
    CheckedIntegerLiteral, CheckedProgramIdentity, CheckedStateFieldSchema, DesignatedCheckedCore,
    DesignatedInputReceiptUse, DesignatedInputRequest, DesignatedMaterializationCore,
    DesignatedRemoteInputDependency, DesignatedResultConsumerCore, DesignatedTriggerCore,
    EffectKind, FailureRow, GeneratedObligationKind, OwnerRmwCheckedCore, PipelineSourceSpan,
    RelationAnchorCore, RelationCheckedCore, RelationTransformCore, TypedExpression,
    TypedStateRead,
};

/// Private snapshot schema version. A receiver rejects every other version.
pub const SNAPSHOT_VERSION: u32 = 1;

/// Typed, fail-closed rejection of a private checked-artifact snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    UnsupportedVersion { found: u32 },
    InvalidFrontier { kind: &'static str },
    InvalidExpression { reason: &'static str },
    StructuralMismatch { reason: &'static str },
    InconsistentPolicyStamp,
}

/// Version envelope for a private process-image component.
///
/// A caller must wrap each independently decoded component (or its enclosing
/// one-locus DTO) in this envelope.  The schema is intentionally provisional:
/// it is neither a public compatibility claim nor a transport wire version.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotEnvelope<T> {
    pub version: u32,
    pub payload: T,
}

impl<T> SnapshotEnvelope<T> {
    pub fn from_checked(payload: T) -> Self {
        Self {
            version: SNAPSHOT_VERSION,
            payload,
        }
    }

    pub fn into_checked(self) -> Result<T, SnapshotError> {
        if self.version != SNAPSHOT_VERSION {
            return Err(SnapshotError::UnsupportedVersion {
                found: self.version,
            });
        }
        Ok(self.payload)
    }
}

/// An exact SourceRef DTO used only inside the private process-image snapshot.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotSourceRef {
    pub version: u32,
    pub path: String,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotPipelineSourceSpan {
    file: String,
    byte_start: usize,
    byte_end: usize,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

impl SnapshotPipelineSourceSpan {
    fn from_checked(span: &PipelineSourceSpan) -> Self {
        Self {
            file: span.file.clone(),
            byte_start: span.byte_start,
            byte_end: span.byte_end,
            start_line: span.start_line,
            start_column: span.start_column,
            end_line: span.end_line,
            end_column: span.end_column,
        }
    }

    fn into_checked(self) -> PipelineSourceSpan {
        PipelineSourceSpan {
            file: self.file,
            byte_start: self.byte_start,
            byte_end: self.byte_end,
            start_line: self.start_line,
            start_column: self.start_column,
            end_line: self.end_line,
            end_column: self.end_column,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCheckedProgramIdentity {
    pub module: String,
    pub source_file: String,
    pub root_source_ref: SnapshotSourceRef,
    pub structural_entries: Vec<String>,
}

impl SnapshotCheckedProgramIdentity {
    pub fn from_checked(identity: &CheckedProgramIdentity) -> Self {
        Self {
            module: identity.module.clone(),
            source_file: identity.source_file.clone(),
            root_source_ref: SnapshotSourceRef::from_checked(&identity.root_source_ref),
            structural_entries: identity.structural_entries.clone(),
        }
    }

    pub fn into_checked(self) -> Result<CheckedProgramIdentity, SnapshotError> {
        Ok(CheckedProgramIdentity::new(
            self.module,
            self.source_file,
            self.root_source_ref.into_checked()?,
        )
        .with_structural_entries(self.structural_entries))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCheckedStateFieldSchema {
    name: String,
    type_name: String,
    visibility_channel: Option<String>,
    source_ref: SnapshotSourceRef,
}

impl SnapshotCheckedStateFieldSchema {
    pub fn from_checked(field: &CheckedStateFieldSchema) -> Self {
        Self {
            name: field.name.clone(),
            type_name: field.type_name.clone(),
            visibility_channel: field.visibility_channel.clone(),
            source_ref: SnapshotSourceRef::from_checked(&field.source_ref),
        }
    }

    pub fn into_checked(self) -> Result<CheckedStateFieldSchema, SnapshotError> {
        Ok(CheckedStateFieldSchema {
            name: self.name,
            type_name: self.type_name,
            visibility_channel: self.visibility_channel,
            source_ref: self.source_ref.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCheckedIndexedStateSchema {
    pub name: String,
    pub index_name: String,
    pub index_type: String,
    pub owner_locus: String,
    pub fields: Vec<SnapshotCheckedStateFieldSchema>,
    pub source_ref: SnapshotSourceRef,
}

impl SnapshotCheckedIndexedStateSchema {
    pub fn from_checked(schema: &CheckedIndexedStateSchema) -> Self {
        Self {
            name: schema.name.clone(),
            index_name: schema.index_name.clone(),
            index_type: schema.index_type.clone(),
            owner_locus: schema.owner_locus.clone(),
            fields: schema
                .fields
                .iter()
                .map(SnapshotCheckedStateFieldSchema::from_checked)
                .collect(),
            source_ref: SnapshotSourceRef::from_checked(&schema.source_ref),
        }
    }

    pub fn into_checked(self) -> Result<CheckedIndexedStateSchema, SnapshotError> {
        Ok(CheckedIndexedStateSchema {
            name: self.name,
            index_name: self.index_name,
            index_type: self.index_type,
            owner_locus: self.owner_locus,
            fields: self
                .fields
                .into_iter()
                .map(SnapshotCheckedStateFieldSchema::into_checked)
                .collect::<Result<_, _>>()?,
            source_ref: self.source_ref.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotCheckedEvaluationKind {
    OwnerRmw,
    DesignatedPublishValue,
    PublishRelation,
    ConsumerLocalProjection,
    DesignatedResultConsume,
}

impl SnapshotCheckedEvaluationKind {
    pub fn from_checked(kind: CheckedEvaluationKind) -> Self {
        match kind {
            CheckedEvaluationKind::OwnerRmw => Self::OwnerRmw,
            CheckedEvaluationKind::DesignatedPublishValue => Self::DesignatedPublishValue,
            CheckedEvaluationKind::PublishRelation => Self::PublishRelation,
            CheckedEvaluationKind::ConsumerLocalProjection => Self::ConsumerLocalProjection,
            CheckedEvaluationKind::DesignatedResultConsume => Self::DesignatedResultConsume,
        }
    }

    pub fn into_checked(self) -> CheckedEvaluationKind {
        match self {
            Self::OwnerRmw => CheckedEvaluationKind::OwnerRmw,
            Self::DesignatedPublishValue => CheckedEvaluationKind::DesignatedPublishValue,
            Self::PublishRelation => CheckedEvaluationKind::PublishRelation,
            Self::ConsumerLocalProjection => CheckedEvaluationKind::ConsumerLocalProjection,
            Self::DesignatedResultConsume => CheckedEvaluationKind::DesignatedResultConsume,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCheckedEvaluationParameter {
    name: String,
    type_name: String,
    source_ref: SnapshotSourceRef,
}

impl SnapshotCheckedEvaluationParameter {
    fn from_checked(parameter: &CheckedEvaluationParameter) -> Self {
        Self {
            name: parameter.name.clone(),
            type_name: parameter.type_name.clone(),
            source_ref: SnapshotSourceRef::from_checked(&parameter.source_ref),
        }
    }

    fn into_checked(self) -> Result<CheckedEvaluationParameter, SnapshotError> {
        Ok(CheckedEvaluationParameter {
            name: self.name,
            type_name: self.type_name,
            source_ref: self.source_ref.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCheckedEvaluationSignature {
    pub name: String,
    pub kind: SnapshotCheckedEvaluationKind,
    pub actor: Option<String>,
    pub owner_locus: Option<String>,
    pub parameters: Vec<SnapshotCheckedEvaluationParameter>,
    pub source_ref: SnapshotSourceRef,
}

impl SnapshotCheckedEvaluationSignature {
    pub fn from_checked(signature: &CheckedEvaluationSignature) -> Self {
        Self {
            name: signature.name.clone(),
            kind: SnapshotCheckedEvaluationKind::from_checked(signature.kind),
            actor: signature.actor.clone(),
            owner_locus: signature.owner_locus.clone(),
            parameters: signature
                .parameters
                .iter()
                .map(SnapshotCheckedEvaluationParameter::from_checked)
                .collect(),
            source_ref: SnapshotSourceRef::from_checked(&signature.source_ref),
        }
    }

    pub fn into_checked(self) -> Result<CheckedEvaluationSignature, SnapshotError> {
        Ok(CheckedEvaluationSignature {
            name: self.name,
            kind: self.kind.into_checked(),
            actor: self.actor,
            owner_locus: self.owner_locus,
            parameters: self
                .parameters
                .into_iter()
                .map(SnapshotCheckedEvaluationParameter::into_checked)
                .collect::<Result<_, _>>()?,
            source_ref: self.source_ref.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotFailureRow {
    pub names: Vec<String>,
}

impl SnapshotFailureRow {
    pub fn from_checked(row: &FailureRow) -> Self {
        Self {
            names: row.names.clone(),
        }
    }

    pub fn into_checked(self) -> Result<FailureRow, SnapshotError> {
        Ok(FailureRow { names: self.names })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotTypedStateRead {
    namespace: String,
    index: Option<String>,
    field: Option<String>,
    owner_locus: String,
    value_type: String,
    span: SnapshotPipelineSourceSpan,
}

impl SnapshotTypedStateRead {
    pub fn from_checked(read: &TypedStateRead) -> Self {
        Self {
            namespace: read.namespace.clone(),
            index: read.index.clone(),
            field: read.field.clone(),
            owner_locus: read.owner_locus.clone(),
            value_type: read.value_type.clone(),
            span: SnapshotPipelineSourceSpan::from_checked(&read.span),
        }
    }

    pub fn into_checked(self) -> Result<TypedStateRead, SnapshotError> {
        Ok(TypedStateRead {
            namespace: self.namespace,
            index: self.index,
            field: self.field,
            owner_locus: self.owner_locus,
            value_type: self.value_type,
            span: self.span.into_checked(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotCheckedBinaryOperator {
    Add,
    Subtract,
}

impl SnapshotCheckedBinaryOperator {
    fn from_checked(operator: CheckedBinaryOperator) -> Self {
        match operator {
            CheckedBinaryOperator::Add => Self::Add,
            CheckedBinaryOperator::Subtract => Self::Subtract,
        }
    }

    fn into_checked(self) -> CheckedBinaryOperator {
        match self {
            Self::Add => CheckedBinaryOperator::Add,
            Self::Subtract => CheckedBinaryOperator::Subtract,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SnapshotCheckedExpressionTree {
    StateRead {
        read: SnapshotTypedStateRead,
    },
    ParameterRead {
        name: String,
        span: SnapshotPipelineSourceSpan,
    },
    IntegerLiteral {
        value: i64,
        span: SnapshotPipelineSourceSpan,
    },
    Binary {
        operator: SnapshotCheckedBinaryOperator,
        span: SnapshotPipelineSourceSpan,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl SnapshotCheckedExpressionTree {
    fn from_checked(tree: &CheckedExpressionTree) -> Self {
        match tree {
            CheckedExpressionTree::StateRead(read) => Self::StateRead {
                read: SnapshotTypedStateRead::from_checked(read),
            },
            CheckedExpressionTree::ParameterRead { name, span } => Self::ParameterRead {
                name: name.clone(),
                span: SnapshotPipelineSourceSpan::from_checked(span),
            },
            CheckedExpressionTree::IntegerLiteral(literal) => Self::IntegerLiteral {
                value: literal.value,
                span: SnapshotPipelineSourceSpan::from_checked(&literal.span),
            },
            CheckedExpressionTree::Binary {
                operator,
                span,
                left,
                right,
            } => Self::Binary {
                operator: SnapshotCheckedBinaryOperator::from_checked(*operator),
                span: SnapshotPipelineSourceSpan::from_checked(span),
                left: Box::new(Self::from_checked(left)),
                right: Box::new(Self::from_checked(right)),
            },
        }
    }

    fn into_checked(self) -> Result<CheckedExpressionTree, SnapshotError> {
        Ok(match self {
            Self::StateRead { read } => CheckedExpressionTree::StateRead(read.into_checked()?),
            Self::ParameterRead { name, span } => CheckedExpressionTree::ParameterRead {
                name,
                span: span.into_checked(),
            },
            Self::IntegerLiteral { value, span } => {
                CheckedExpressionTree::IntegerLiteral(CheckedIntegerLiteral {
                    value,
                    span: span.into_checked(),
                })
            }
            Self::Binary {
                operator,
                span,
                left,
                right,
            } => CheckedExpressionTree::Binary {
                operator: operator.into_checked(),
                span: span.into_checked(),
                left: Box::new(left.into_checked()?),
                right: Box::new(right.into_checked()?),
            },
        })
    }

    fn collect_facts(
        &self,
        reads: &mut Vec<SnapshotTypedStateRead>,
        literals: &mut Vec<i64>,
        operators: &mut Vec<String>,
    ) {
        match self {
            Self::StateRead { read } => reads.push(read.clone()),
            Self::ParameterRead { .. } => {}
            Self::IntegerLiteral { value, .. } => literals.push(*value),
            Self::Binary {
                operator,
                left,
                right,
                ..
            } => {
                left.collect_facts(reads, literals, operators);
                operators.push(match operator {
                    SnapshotCheckedBinaryOperator::Add => "+".to_string(),
                    SnapshotCheckedBinaryOperator::Subtract => "-".to_string(),
                });
                right.collect_facts(reads, literals, operators);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotTypedExpression {
    span: SnapshotPipelineSourceSpan,
    state_reads: Vec<SnapshotTypedStateRead>,
    int_literals: Vec<i64>,
    operator_chain: Vec<String>,
    tree: SnapshotCheckedExpressionTree,
}

impl SnapshotTypedExpression {
    pub fn from_checked(expression: &TypedExpression) -> Self {
        Self {
            span: SnapshotPipelineSourceSpan::from_checked(&expression.span),
            state_reads: expression
                .state_reads
                .iter()
                .map(SnapshotTypedStateRead::from_checked)
                .collect(),
            int_literals: expression.int_literals.clone(),
            operator_chain: expression.operator_chain.clone(),
            tree: SnapshotCheckedExpressionTree::from_checked(&expression.tree),
        }
    }

    pub fn into_checked(self) -> Result<TypedExpression, SnapshotError> {
        let mut expected_reads = Vec::new();
        let mut expected_literals = Vec::new();
        let mut expected_operators = Vec::new();
        self.tree.collect_facts(
            &mut expected_reads,
            &mut expected_literals,
            &mut expected_operators,
        );
        if expected_reads != self.state_reads
            || expected_literals != self.int_literals
            || expected_operators != self.operator_chain
        {
            return Err(SnapshotError::InvalidExpression {
                reason: "tree facts do not match checked expression metadata",
            });
        }

        Ok(TypedExpression {
            span: self.span.into_checked(),
            state_reads: self
                .state_reads
                .into_iter()
                .map(SnapshotTypedStateRead::into_checked)
                .collect::<Result<_, _>>()?,
            int_literals: self.int_literals,
            operator_chain: self.operator_chain,
            tree: self.tree.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotOwnerRmwCheckedCore {
    pub authority_origin_locus: String,
    pub owner_locus: String,
    pub target: SnapshotTypedStateRead,
    pub expression: SnapshotTypedExpression,
}

impl SnapshotOwnerRmwCheckedCore {
    pub fn from_checked(core: &OwnerRmwCheckedCore) -> Self {
        Self {
            authority_origin_locus: core.authority_origin_locus.clone(),
            owner_locus: core.owner_locus.clone(),
            target: SnapshotTypedStateRead::from_checked(&core.target),
            expression: SnapshotTypedExpression::from_checked(&core.expression),
        }
    }

    pub fn into_checked(self) -> Result<OwnerRmwCheckedCore, SnapshotError> {
        let target = self.target.into_checked()?;
        if target.field.is_none() || target.owner_locus != self.owner_locus {
            return Err(SnapshotError::InvalidExpression {
                reason: "owner RMW target is not an owner-owned state field",
            });
        }
        Ok(OwnerRmwCheckedCore {
            authority_origin_locus: self.authority_origin_locus,
            owner_locus: self.owner_locus,
            target,
            expression: self.expression.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRelationTransformCore {
    kind: String,
    translation: Option<(i64, i64)>,
}

impl SnapshotRelationTransformCore {
    /// Copies the checked transform for a private consumer-restricted image.
    pub fn from_checked(transform: &RelationTransformCore) -> Self {
        Self {
            kind: transform.kind.clone(),
            translation: transform.translation,
        }
    }

    /// Restores only the finite checked transform forms accepted by M7.
    pub fn into_checked(self) -> Result<RelationTransformCore, SnapshotError> {
        match (self.kind.as_str(), self.translation) {
            ("translate", Some(_)) | ("identity", Some((0, 0))) => Ok(RelationTransformCore {
                kind: self.kind,
                translation: self.translation,
            }),
            _ => Err(SnapshotError::StructuralMismatch {
                reason: "unsupported or noncanonical relation transform",
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRelationAnchorCore {
    anchor: String,
    anchor_locus: Option<String>,
    anchor_locus_source_ref: Option<SnapshotSourceRef>,
    epoch: String,
    transform: SnapshotRelationTransformCore,
}

impl SnapshotRelationAnchorCore {
    fn from_checked(anchor: &RelationAnchorCore) -> Self {
        Self {
            anchor: anchor.anchor.clone(),
            anchor_locus: anchor.anchor_locus.clone(),
            anchor_locus_source_ref: anchor
                .anchor_locus_source_ref
                .as_ref()
                .map(SnapshotSourceRef::from_checked),
            epoch: anchor.epoch.clone(),
            transform: SnapshotRelationTransformCore::from_checked(&anchor.transform),
        }
    }

    fn into_checked(self) -> Result<RelationAnchorCore, SnapshotError> {
        if self.anchor_locus.is_some() != self.anchor_locus_source_ref.is_some() {
            return Err(SnapshotError::StructuralMismatch {
                reason: "relation anchor locus and source reference must co-occur",
            });
        }
        Ok(RelationAnchorCore {
            anchor: self.anchor,
            anchor_locus: self.anchor_locus,
            anchor_locus_source_ref: self
                .anchor_locus_source_ref
                .map(SnapshotSourceRef::into_checked)
                .transpose()?,
            epoch: self.epoch,
            transform: self.transform.into_checked()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotBindingActivationFrontier {
    occurrences: Vec<String>,
}

impl SnapshotBindingActivationFrontier {
    fn from_checked(frontier: &BindingActivationFrontier) -> Self {
        Self {
            occurrences: frontier
                .as_slice()
                .iter()
                .map(|occurrence| occurrence.as_str().to_string())
                .collect(),
        }
    }

    fn into_checked(self) -> Result<BindingActivationFrontier, SnapshotError> {
        BindingActivationFrontier::from_ordered_occurrences(
            self.occurrences
                .into_iter()
                .map(SharedOccurrenceId::new)
                .collect(),
        )
        .map_err(|_| SnapshotError::InvalidFrontier {
            kind: "binding activation",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRelationCheckedCore {
    pub owner_locus: String,
    pub subject: String,
    pub subject_type: String,
    primary: SnapshotRelationAnchorCore,
    fallback: SnapshotRelationAnchorCore,
    binding_frontier: SnapshotBindingActivationFrontier,
    pub consumer_projection_locus: Option<String>,
}

impl SnapshotRelationCheckedCore {
    pub fn from_checked(core: &RelationCheckedCore) -> Self {
        Self {
            owner_locus: core.owner_locus.clone(),
            subject: core.subject.clone(),
            subject_type: core.subject_type.clone(),
            primary: SnapshotRelationAnchorCore::from_checked(&core.primary),
            fallback: SnapshotRelationAnchorCore::from_checked(&core.fallback),
            binding_frontier: SnapshotBindingActivationFrontier::from_checked(
                &core.binding_frontier,
            ),
            consumer_projection_locus: core.consumer_projection_locus.clone(),
        }
    }

    pub fn into_checked(self) -> Result<RelationCheckedCore, SnapshotError> {
        Ok(RelationCheckedCore {
            owner_locus: self.owner_locus,
            subject: self.subject,
            subject_type: self.subject_type,
            primary: self.primary.into_checked()?,
            fallback: self.fallback.into_checked()?,
            binding_frontier: self.binding_frontier.into_checked()?,
            consumer_projection_locus: self.consumer_projection_locus,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SnapshotEvaluationSite {
    Owner(String),
    Locus(String),
    DesignatedEvaluator(String),
    Consumer(String),
    Provider(String),
}

impl SnapshotEvaluationSite {
    fn from_checked(site: &EvaluationSite) -> Self {
        match site {
            EvaluationSite::Owner(locus) => Self::Owner(locus.as_str().to_string()),
            EvaluationSite::Locus(locus) => Self::Locus(locus.as_str().to_string()),
            EvaluationSite::DesignatedEvaluator(locus) => {
                Self::DesignatedEvaluator(locus.as_str().to_string())
            }
            EvaluationSite::Consumer(principal) => Self::Consumer(principal.as_str().to_string()),
            EvaluationSite::Provider(provider) => Self::Provider(provider.as_str().to_string()),
        }
    }

    fn into_checked(self) -> EvaluationSite {
        match self {
            Self::Owner(value) => EvaluationSite::Owner(Locus::new(value)),
            Self::Locus(value) => EvaluationSite::Locus(Locus::new(value)),
            Self::DesignatedEvaluator(value) => {
                EvaluationSite::DesignatedEvaluator(Locus::new(value))
            }
            Self::Consumer(value) => EvaluationSite::Consumer(Principal::new(value)),
            Self::Provider(value) => EvaluationSite::Provider(Provider::new(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SnapshotAuthorityOrigin {
    Caller(String),
    OwnerTransition(String),
    AdmittedEvaluator(String),
    AdmittedProvider(String),
}

impl SnapshotAuthorityOrigin {
    fn from_checked(origin: &AuthorityOrigin) -> Self {
        match origin {
            AuthorityOrigin::Caller(principal) => Self::Caller(principal.as_str().to_string()),
            AuthorityOrigin::OwnerTransition(locus) => {
                Self::OwnerTransition(locus.as_str().to_string())
            }
            AuthorityOrigin::AdmittedEvaluator(locus) => {
                Self::AdmittedEvaluator(locus.as_str().to_string())
            }
            AuthorityOrigin::AdmittedProvider(provider) => {
                Self::AdmittedProvider(provider.as_str().to_string())
            }
        }
    }

    fn into_checked(self) -> AuthorityOrigin {
        match self {
            Self::Caller(value) => AuthorityOrigin::Caller(Principal::new(value)),
            Self::OwnerTransition(value) => AuthorityOrigin::OwnerTransition(Locus::new(value)),
            Self::AdmittedEvaluator(value) => AuthorityOrigin::AdmittedEvaluator(Locus::new(value)),
            Self::AdmittedProvider(value) => {
                AuthorityOrigin::AdmittedProvider(Provider::new(value))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotInputFrontier {
    producers: Vec<String>,
}

impl SnapshotInputFrontier {
    fn from_checked(frontier: &InputFrontier) -> Self {
        Self {
            producers: frontier
                .as_slice()
                .iter()
                .map(|occurrence| occurrence.as_str().to_string())
                .collect(),
        }
    }

    fn into_checked(self) -> Result<InputFrontier, SnapshotError> {
        InputFrontier::from_ordered_producers(
            self.producers
                .into_iter()
                .map(MaterializationOccurrenceId::new)
                .collect(),
        )
        .map_err(|_| SnapshotError::InvalidFrontier { kind: "input" })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotResultFrontier {
    results: Vec<String>,
}

impl SnapshotResultFrontier {
    fn from_checked(frontier: &ResultFrontier) -> Self {
        Self {
            results: frontier
                .as_slice()
                .iter()
                .map(|result| result.as_str().to_string())
                .collect(),
        }
    }

    fn into_checked(self) -> Result<ResultFrontier, SnapshotError> {
        ResultFrontier::from_ordered_results(self.results.into_iter().map(ResultKey::new).collect())
            .map_err(|_| SnapshotError::InvalidFrontier { kind: "result" })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotEvaluationPolicy {
    name: String,
    deterministic: bool,
}

impl SnapshotEvaluationPolicy {
    fn from_checked(policy: &EvaluationPolicy) -> Self {
        Self {
            name: policy.name.clone(),
            deterministic: policy.deterministic,
        }
    }

    fn into_checked(self) -> EvaluationPolicy {
        EvaluationPolicy {
            name: self.name,
            deterministic: self.deterministic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotObservationPolicy {
    name: String,
}

impl SnapshotObservationPolicy {
    fn from_checked(policy: &ObservationPolicy) -> Self {
        Self {
            name: policy.name.clone(),
        }
    }

    fn into_checked(self) -> ObservationPolicy {
        ObservationPolicy { name: self.name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotPolicyStamp {
    evaluation_policy: SnapshotEvaluationPolicy,
    observation_policy: SnapshotObservationPolicy,
}

impl SnapshotPolicyStamp {
    fn from_checked(stamp: &PolicyStamp) -> Self {
        Self {
            evaluation_policy: SnapshotEvaluationPolicy::from_checked(&stamp.evaluation_policy),
            observation_policy: SnapshotObservationPolicy::from_checked(&stamp.observation_policy),
        }
    }

    fn into_checked(self) -> PolicyStamp {
        PolicyStamp {
            evaluation_policy: self.evaluation_policy.into_checked(),
            observation_policy: self.observation_policy.into_checked(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDesignatedRemoteInputDependency {
    designated_evaluator: String,
    requester_site: SnapshotEvaluationSite,
    authority_origin: SnapshotAuthorityOrigin,
    source_owner_locus: String,
    typed_state_read: SnapshotTypedStateRead,
    request_source_owner_locus: String,
    request_typed_state_read: SnapshotTypedStateRead,
    receipt_use_source_owner_locus: String,
    receipt_use_typed_state_read: SnapshotTypedStateRead,
}

impl SnapshotDesignatedRemoteInputDependency {
    /// Copies an already checked remote-input dependency into the private DTO.
    pub fn from_checked(dependency: &DesignatedRemoteInputDependency) -> Self {
        Self {
            designated_evaluator: dependency.designated_evaluator.clone(),
            requester_site: SnapshotEvaluationSite::from_checked(&dependency.requester_site),
            authority_origin: SnapshotAuthorityOrigin::from_checked(&dependency.authority_origin),
            source_owner_locus: dependency.source_owner_locus.clone(),
            typed_state_read: SnapshotTypedStateRead::from_checked(&dependency.typed_state_read),
            request_source_owner_locus: dependency.request.source_owner_locus.clone(),
            request_typed_state_read: SnapshotTypedStateRead::from_checked(
                &dependency.request.typed_state_read,
            ),
            receipt_use_source_owner_locus: dependency.receipt_use.source_owner_locus.clone(),
            receipt_use_typed_state_read: SnapshotTypedStateRead::from_checked(
                &dependency.receipt_use.typed_state_read,
            ),
        }
    }

    /// Restores the dependency only when request, receipt, and checked read
    /// retain their exact source-derived identity.
    pub fn into_checked(self) -> Result<DesignatedRemoteInputDependency, SnapshotError> {
        let typed_state_read = self.typed_state_read.into_checked()?;
        let request_typed_state_read = self.request_typed_state_read.into_checked()?;
        let receipt_use_typed_state_read = self.receipt_use_typed_state_read.into_checked()?;
        if self.source_owner_locus != self.request_source_owner_locus
            || self.source_owner_locus != self.receipt_use_source_owner_locus
            || typed_state_read != request_typed_state_read
            || typed_state_read != receipt_use_typed_state_read
        {
            return Err(SnapshotError::StructuralMismatch {
                reason: "designated remote input request and receipt must retain the exact read",
            });
        }
        Ok(DesignatedRemoteInputDependency {
            designated_evaluator: self.designated_evaluator,
            requester_site: self.requester_site.into_checked(),
            authority_origin: self.authority_origin.into_checked(),
            source_owner_locus: self.source_owner_locus.clone(),
            typed_state_read: typed_state_read.clone(),
            request: DesignatedInputRequest {
                source_owner_locus: self.request_source_owner_locus,
                typed_state_read: request_typed_state_read,
            },
            receipt_use: DesignatedInputReceiptUse {
                source_owner_locus: self.receipt_use_source_owner_locus,
                typed_state_read: receipt_use_typed_state_read,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDesignatedCheckedCore {
    pub evaluator: String,
    pub result: String,
    trigger_frontier: String,
    result_frontier: SnapshotResultFrontier,
    input_frontier: SnapshotInputFrontier,
    result_version: u64,
    evaluation_policy: SnapshotEvaluationPolicy,
    observation_policy: SnapshotObservationPolicy,
    policy_stamp: SnapshotPolicyStamp,
    expression: SnapshotTypedExpression,
    generated_remote_input_dependencies: Vec<SnapshotDesignatedRemoteInputDependency>,
}

impl SnapshotDesignatedCheckedCore {
    pub fn from_checked(core: &DesignatedCheckedCore) -> Self {
        Self {
            evaluator: core.evaluator.clone(),
            result: core.result.clone(),
            trigger_frontier: core.trigger.frontier.clone(),
            result_frontier: SnapshotResultFrontier::from_checked(&core.result_frontier),
            input_frontier: SnapshotInputFrontier::from_checked(&core.input_frontier),
            result_version: core.result_version.value(),
            evaluation_policy: SnapshotEvaluationPolicy::from_checked(&core.evaluation_policy),
            observation_policy: SnapshotObservationPolicy::from_checked(&core.observation_policy),
            policy_stamp: SnapshotPolicyStamp::from_checked(&core.policy_stamp),
            expression: SnapshotTypedExpression::from_checked(&core.expression),
            generated_remote_input_dependencies: core
                .generated_remote_input_dependencies
                .iter()
                .map(SnapshotDesignatedRemoteInputDependency::from_checked)
                .collect(),
        }
    }

    pub fn into_checked(self) -> Result<DesignatedCheckedCore, SnapshotError> {
        let evaluation_policy = self.evaluation_policy.into_checked();
        let observation_policy = self.observation_policy.into_checked();
        let policy_stamp = self.policy_stamp.into_checked();
        if policy_stamp.evaluation_policy != evaluation_policy
            || policy_stamp.observation_policy != observation_policy
        {
            return Err(SnapshotError::InconsistentPolicyStamp);
        }
        Ok(DesignatedCheckedCore {
            evaluator: self.evaluator,
            result: self.result,
            trigger: DesignatedTriggerCore {
                frontier: self.trigger_frontier,
            },
            result_frontier: self.result_frontier.into_checked()?,
            input_frontier: self.input_frontier.into_checked()?,
            result_version: ResultVersion::new(self.result_version),
            evaluation_policy,
            observation_policy,
            policy_stamp,
            materialization: DesignatedMaterializationCore,
            expression: self.expression.into_checked()?,
            generated_remote_input_dependencies: self
                .generated_remote_input_dependencies
                .into_iter()
                .map(SnapshotDesignatedRemoteInputDependency::into_checked)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotStaticRetryContractKind {
    ReturnExistingNoNewConsumption,
}

impl SnapshotStaticRetryContractKind {
    fn from_checked(contract: StaticRetryContractKind) -> Self {
        match contract {
            StaticRetryContractKind::ReturnExistingNoNewConsumption => {
                Self::ReturnExistingNoNewConsumption
            }
        }
    }

    fn into_checked(self) -> StaticRetryContractKind {
        match self {
            Self::ReturnExistingNoNewConsumption => {
                StaticRetryContractKind::ReturnExistingNoNewConsumption
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDesignatedResultConsumerCore {
    pub evaluator: String,
    pub result: String,
    pub consumer_locus: String,
    source_ref: SnapshotSourceRef,
    result_ref_source_ref: SnapshotSourceRef,
    result_frontier: SnapshotResultFrontier,
    input_frontier: SnapshotInputFrontier,
    result_version: u64,
    observation_policy: SnapshotObservationPolicy,
    policy_stamp: SnapshotPolicyStamp,
    retry_contract: SnapshotStaticRetryContractKind,
}

impl SnapshotDesignatedResultConsumerCore {
    pub fn from_checked(core: &DesignatedResultConsumerCore) -> Self {
        Self {
            evaluator: core.evaluator.clone(),
            result: core.result.clone(),
            consumer_locus: core.consumer_locus.clone(),
            source_ref: SnapshotSourceRef::from_checked(&core.source_ref),
            result_ref_source_ref: SnapshotSourceRef::from_checked(&core.result_ref_source_ref),
            result_frontier: SnapshotResultFrontier::from_checked(&core.result_frontier),
            input_frontier: SnapshotInputFrontier::from_checked(&core.input_frontier),
            result_version: core.result_version.value(),
            observation_policy: SnapshotObservationPolicy::from_checked(&core.observation_policy),
            policy_stamp: SnapshotPolicyStamp::from_checked(&core.policy_stamp),
            retry_contract: SnapshotStaticRetryContractKind::from_checked(core.retry_contract),
        }
    }

    pub fn into_checked(self) -> Result<DesignatedResultConsumerCore, SnapshotError> {
        let observation_policy = self.observation_policy.into_checked();
        let policy_stamp = self.policy_stamp.into_checked();
        if policy_stamp.observation_policy != observation_policy {
            return Err(SnapshotError::InconsistentPolicyStamp);
        }
        Ok(DesignatedResultConsumerCore {
            evaluator: self.evaluator,
            result: self.result,
            consumer_locus: self.consumer_locus,
            source_ref: self.source_ref.into_checked()?,
            result_ref_source_ref: self.result_ref_source_ref.into_checked()?,
            result_frontier: self.result_frontier.into_checked()?,
            input_frontier: self.input_frontier.into_checked()?,
            result_version: ResultVersion::new(self.result_version),
            observation_policy,
            policy_stamp,
            retry_contract: self.retry_contract.into_checked(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotEffectKind {
    OwnerRequest,
    OwnerLocalRead,
    OwnerWrite,
    ActorReadReply,
    ObserverPublish,
    RelationPublish,
    DesignatedRemoteRequest,
    DesignatedReceiptUse,
    DesignatedValuePublish,
    DesignatedResultDelivery,
    DesignatedResultConsume,
}

impl SnapshotEffectKind {
    pub fn from_checked(kind: EffectKind) -> Self {
        match kind {
            EffectKind::OwnerRequest => Self::OwnerRequest,
            EffectKind::OwnerLocalRead => Self::OwnerLocalRead,
            EffectKind::OwnerWrite => Self::OwnerWrite,
            EffectKind::ActorReadReply => Self::ActorReadReply,
            EffectKind::ObserverPublish => Self::ObserverPublish,
            EffectKind::RelationPublish => Self::RelationPublish,
            EffectKind::DesignatedRemoteRequest => Self::DesignatedRemoteRequest,
            EffectKind::DesignatedReceiptUse => Self::DesignatedReceiptUse,
            EffectKind::DesignatedValuePublish => Self::DesignatedValuePublish,
            EffectKind::DesignatedResultDelivery => Self::DesignatedResultDelivery,
            EffectKind::DesignatedResultConsume => Self::DesignatedResultConsume,
        }
    }

    pub fn into_checked(self) -> EffectKind {
        match self {
            Self::OwnerRequest => EffectKind::OwnerRequest,
            Self::OwnerLocalRead => EffectKind::OwnerLocalRead,
            Self::OwnerWrite => EffectKind::OwnerWrite,
            Self::ActorReadReply => EffectKind::ActorReadReply,
            Self::ObserverPublish => EffectKind::ObserverPublish,
            Self::RelationPublish => EffectKind::RelationPublish,
            Self::DesignatedRemoteRequest => EffectKind::DesignatedRemoteRequest,
            Self::DesignatedReceiptUse => EffectKind::DesignatedReceiptUse,
            Self::DesignatedValuePublish => EffectKind::DesignatedValuePublish,
            Self::DesignatedResultDelivery => EffectKind::DesignatedResultDelivery,
            Self::DesignatedResultConsume => EffectKind::DesignatedResultConsume,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SnapshotGeneratedObligationKind {
    Failure(String),
    Capability,
    Witness,
    Authority,
    AdmittedEvaluatorAuthority,
    DesignatedResultConsumerAuthority,
    Evaluation(SnapshotCheckedEvaluationKind),
}

impl SnapshotGeneratedObligationKind {
    pub fn from_checked(kind: &GeneratedObligationKind) -> Self {
        match kind {
            GeneratedObligationKind::Failure(name) => Self::Failure(name.clone()),
            GeneratedObligationKind::Capability => Self::Capability,
            GeneratedObligationKind::Witness => Self::Witness,
            GeneratedObligationKind::Authority => Self::Authority,
            GeneratedObligationKind::AdmittedEvaluatorAuthority => Self::AdmittedEvaluatorAuthority,
            GeneratedObligationKind::DesignatedResultConsumerAuthority => {
                Self::DesignatedResultConsumerAuthority
            }
            GeneratedObligationKind::Evaluation(kind) => {
                Self::Evaluation(SnapshotCheckedEvaluationKind::from_checked(*kind))
            }
        }
    }

    pub fn into_checked(self) -> GeneratedObligationKind {
        match self {
            Self::Failure(name) => GeneratedObligationKind::Failure(name),
            Self::Capability => GeneratedObligationKind::Capability,
            Self::Witness => GeneratedObligationKind::Witness,
            Self::Authority => GeneratedObligationKind::Authority,
            Self::AdmittedEvaluatorAuthority => GeneratedObligationKind::AdmittedEvaluatorAuthority,
            Self::DesignatedResultConsumerAuthority => {
                GeneratedObligationKind::DesignatedResultConsumerAuthority
            }
            Self::Evaluation(kind) => GeneratedObligationKind::Evaluation(kind.into_checked()),
        }
    }
}

impl SnapshotSourceRef {
    pub fn from_checked(source_ref: &SourceRef) -> Self {
        Self {
            version: SNAPSHOT_VERSION,
            path: source_ref.path.clone(),
            start_line: source_ref.start_line,
            start_column: source_ref.start_column,
            end_line: source_ref.end_line,
            end_column: source_ref.end_column,
        }
    }

    pub fn into_checked(self) -> Result<SourceRef, SnapshotError> {
        if self.version != SNAPSHOT_VERSION {
            return Err(SnapshotError::UnsupportedVersion {
                found: self.version,
            });
        }
        Ok(SourceRef::new(
            self.path,
            self.start_line,
            self.start_column,
            self.end_line,
            self.end_column,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SNAPSHOT_VERSION, SnapshotCheckedEvaluationSignature, SnapshotCheckedIndexedStateSchema,
        SnapshotDesignatedCheckedCore, SnapshotDesignatedResultConsumerCore, SnapshotEnvelope,
        SnapshotError, SnapshotOwnerRmwCheckedCore, SnapshotRelationCheckedCore, SnapshotSourceRef,
    };
    use crate::{shared_model::SourceRef, surface_v0_pipeline::check_and_elaborate_surface_v0};
    use mir_ast::surface_v0::FixtureSource;

    const ACTIVE_I2_SOURCE: &str =
        include_str!("../../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");

    #[test]
    fn source_ref_snapshot_restores_the_exact_checked_location() {
        let source_ref = SourceRef::new("samples/example.mir", 2, 3, 4, 5);
        let snapshot = SnapshotSourceRef::from_checked(&source_ref);

        assert_eq!(snapshot.version, SNAPSHOT_VERSION);
        assert_eq!(
            snapshot
                .clone()
                .into_checked()
                .expect("valid private snapshot"),
            source_ref
        );
        let envelope = SnapshotEnvelope::from_checked(snapshot);
        assert_eq!(
            envelope
                .into_checked()
                .expect("matching version unwraps")
                .version,
            SNAPSHOT_VERSION
        );
        let mut unsupported =
            SnapshotEnvelope::from_checked(SnapshotSourceRef::from_checked(&source_ref));
        unsupported.version += 1;
        assert_eq!(
            unsupported.into_checked(),
            Err(SnapshotError::UnsupportedVersion {
                found: SNAPSHOT_VERSION + 1
            })
        );
    }

    #[test]
    fn active_checked_cores_round_trip_through_private_json_without_rechecking_source() {
        let checked = check_and_elaborate_surface_v0(FixtureSource::new(
            "samples/clean-near-end/mirrorea-i2-local-toy/main.mir",
            ACTIVE_I2_SOURCE,
        ))
        .expect("the active ordinary source checks before snapshotting");

        let identity =
            super::SnapshotCheckedProgramIdentity::from_checked(checked.program_identity());
        let identity_json = serde_json::to_string(&identity).expect("private identity serializes");
        let restored_identity: super::SnapshotCheckedProgramIdentity =
            serde_json::from_str(&identity_json).expect("private identity deserializes");
        assert_eq!(
            restored_identity
                .into_checked()
                .expect("private identity restores"),
            *checked.program_identity()
        );

        for schema in checked.static_environment().indexed_state_schemas() {
            let snapshot = SnapshotCheckedIndexedStateSchema::from_checked(schema);
            let json = serde_json::to_string(&snapshot).expect("schema serializes");
            let restored: SnapshotCheckedIndexedStateSchema =
                serde_json::from_str(&json).expect("schema deserializes");
            assert_eq!(restored.into_checked().expect("schema restores"), *schema);
        }
        for signature in checked.static_environment().evaluation_signatures() {
            let snapshot = SnapshotCheckedEvaluationSignature::from_checked(signature);
            let json = serde_json::to_string(&snapshot).expect("signature serializes");
            let restored: SnapshotCheckedEvaluationSignature =
                serde_json::from_str(&json).expect("signature deserializes");
            assert_eq!(
                restored.into_checked().expect("signature restores"),
                *signature
            );
        }

        for evaluation in checked.evaluations() {
            if let Some(core) = evaluation.owner_rmw_core() {
                let snapshot = SnapshotOwnerRmwCheckedCore::from_checked(core);
                let json = serde_json::to_string(&snapshot).expect("owner core serializes");
                let restored: SnapshotOwnerRmwCheckedCore =
                    serde_json::from_str(&json).expect("owner core deserializes");
                assert_eq!(restored.into_checked().expect("owner core restores"), *core);
            }
            if let Some(core) = evaluation.relation_core() {
                let snapshot = SnapshotRelationCheckedCore::from_checked(core);
                let json = serde_json::to_string(&snapshot).expect("relation core serializes");
                let restored: SnapshotRelationCheckedCore =
                    serde_json::from_str(&json).expect("relation core deserializes");
                assert_eq!(
                    restored.into_checked().expect("relation core restores"),
                    *core
                );
            }
            if let Some(core) = evaluation.designated_core() {
                let snapshot = SnapshotDesignatedCheckedCore::from_checked(core);
                let json = serde_json::to_string(&snapshot).expect("designated core serializes");
                let restored: SnapshotDesignatedCheckedCore =
                    serde_json::from_str(&json).expect("designated core deserializes");
                assert_eq!(
                    restored.into_checked().expect("designated core restores"),
                    *core
                );
            }
            if let Some(core) = evaluation.designated_result_consumer_core() {
                let snapshot = SnapshotDesignatedResultConsumerCore::from_checked(core);
                let json = serde_json::to_string(&snapshot).expect("consumer core serializes");
                let restored: SnapshotDesignatedResultConsumerCore =
                    serde_json::from_str(&json).expect("consumer core deserializes");
                assert_eq!(
                    restored.into_checked().expect("consumer core restores"),
                    *core
                );
            }
        }

        let unknown = identity_json.replacen('}', ",\"unexpected\":true}", 1);
        assert!(serde_json::from_str::<super::SnapshotCheckedProgramIdentity>(&unknown).is_err());
    }
}
