//! M7's authoritative static route for the bounded M6 Surface v0 corpus.
//!
//! This module composes the M6 parser and classifier with finite M7 checks.
//! Its checked Core records are static evidence for the next layer; they are
//! intentionally not a runtime interpreter or an M5 command surrogate.

use std::collections::BTreeMap;

use mir_ast::surface_v0::{
    BoundedExpression, BoundedExpressionTree, DeferredFormKind, FixtureSource, Parameter,
    ParseErrorKind, RelationTransform, SurfaceReference, SurfaceV0File, SurfaceV0Span,
    parse_surface_v0,
};

use crate::{
    evaluation_materialization::{
        AuthorityOrigin, EvaluationPolicy, EvaluationSite, InputFrontier, Locus, Materialization,
        ObservationPolicy, OccurrenceId as M3OccurrenceId, PolicyStamp, Principal, SemanticForm,
        TriggerClock,
    },
    shared_model::{
        BindingActivationFrontier, OccurrenceId, ResultFrontier, ResultKey, ResultVersion,
        SourceRef,
    },
    surface_v0_classification::{
        CoreTemplateKind, SourceToCoreKind, SurfaceV0Classification,
        SurfaceV0ClassificationOptions, SurfaceV0DiagnosticKind, classify_surface_v0,
    },
};

const OWNER_RMW_FAILURES: [&str; 4] = [
    "StaleMembership",
    "MissingCapability",
    "MissingWitness",
    "RouteUnavailable",
];

const OBSERVER_SAFE_CHANNEL: &str = "observer_safe";
const VISIBILITY_DENIED_FAILURE: &str = "VisibilityDenied";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineSourceSpan {
    file: String,
    byte_start: usize,
    byte_end: usize,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

impl PipelineSourceSpan {
    fn from_surface(span: &SurfaceV0Span) -> Self {
        let (start_line, start_column) = span.start_line_column();
        let (end_line, end_column) = span.end_line_column();
        let range = span.byte_range();
        Self {
            file: span.file().to_string(),
            byte_start: range.start,
            byte_end: range.end,
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    fn from_source_range(source: &FixtureSource, byte_start: usize, byte_end: usize) -> Self {
        let (start_line, start_column) = line_column(source.text(), byte_start);
        let (end_line, end_column) = line_column(source.text(), byte_end);
        Self {
            file: source.file().to_string(),
            byte_start,
            byte_end,
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    pub fn byte_range(&self) -> std::ops::Range<usize> {
        self.byte_start..self.byte_end
    }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.byte_start..self.byte_end]
    }

    fn source_ref(&self) -> SourceRef {
        SourceRef::new(
            self.file.clone(),
            self.start_line,
            self.start_column,
            self.end_line,
            self.end_column,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M7DiagnosticKind {
    RoleActorMustBeLiteralSelf,
    OwnerActionLocusMismatch,
    CrossOwnerWriteTargetOutsideActionLocus,
    FieldlessAssignmentTarget,
    CrossOwnerOperandRequiresReceipt,
    RelationMustPublishRelationCarrier,
    ConsumerRelationMutationDenied,
    UnresolvedName,
    AmbiguousName,
    UnsupportedTransportSyntax,
    UnsupportedOccurrenceSyntax,
    UnsupportedEnvelopeSyntax,
    UnexpectedSyntax,
    UnsupportedExpression,
    ArithmeticRequiresInt,
    GeneratedFailureNotDeclared,
    DuplicateDeclaration,
    UnknownStateField,
    TypeMismatch,
    UndefinedStateIndexType,
    UndefinedStateFieldType,
    UndefinedRelationSubjectType,
    UndefinedOwnerLocus,
    UndefinedConsumerLocus,
    UndefinedSelfPrincipal,
    UndefinedRoleEvaluationLocus,
    DuplicateStateField,
    DuplicateEvent,
    DuplicateRelation,
    DuplicateDesignated,
    DuplicateDeferred,
    ResidualCannotExecute,
}

/// Reason retained for an under-declared generated failure row.
///
/// The kind alone deliberately remains broad so existing M7 consumers can
/// continue to classify the diagnostic.  The missing row member is retained
/// separately for release-code selection and source-bound evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M7GeneratedFailureReason {
    MissingDeclaredFailure(String),
}

impl M7GeneratedFailureReason {
    pub fn missing_failure(&self) -> &str {
        match self {
            Self::MissingDeclaredFailure(name) => name,
        }
    }
}

/// Compatibility name for the accepted M6 evidence retained without
/// reconstructing it from the Surface AST.
pub type ConsumedM6Classification = SurfaceV0Classification;

/// Structural identity for the checked source artifact that M8 may admit.
///
/// This deliberately retains source identity rather than deriving an opaque
/// content hash.  M8 admission therefore remains tied to the checked source
/// artifact and its canonical root location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProgramIdentity {
    module: String,
    source_file: String,
    root_source_ref: SourceRef,
    structural_entries: Vec<String>,
}

impl CheckedProgramIdentity {
    pub fn new(
        module: impl Into<String>,
        source_file: impl Into<String>,
        root_source_ref: SourceRef,
    ) -> Self {
        Self {
            module: module.into(),
            source_file: source_file.into(),
            root_source_ref,
            structural_entries: Vec::new(),
        }
    }

    fn with_structural_entries(mut self, structural_entries: Vec<String>) -> Self {
        self.structural_entries = structural_entries;
        self
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn source_file(&self) -> &str {
        &self.source_file
    }

    pub fn root_source_ref(&self) -> &SourceRef {
        &self.root_source_ref
    }

    pub fn stable_key(&self) -> String {
        let root = &self.root_source_ref;
        let base = format!(
            "{}:{}:{}:{}:{}:{}",
            self.module,
            self.source_file,
            root.start_line,
            root.start_column,
            root.end_line,
            root.end_column
        );
        if self.structural_entries.is_empty() {
            base
        } else {
            format!("{base}|{}", self.structural_entries.join("|"))
        }
    }

    pub fn structural_entries(&self) -> &[String] {
        &self.structural_entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedStaticDeclaration {
    name: String,
    source_ref: SourceRef,
}

impl CheckedStaticDeclaration {
    fn new(name: impl Into<String>, source_ref: SourceRef) -> Self {
        Self {
            name: name.into(),
            source_ref,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedStateFieldSchema {
    name: String,
    type_name: String,
    visibility_channel: Option<String>,
    source_ref: SourceRef,
}

impl CheckedStateFieldSchema {
    fn new(
        name: impl Into<String>,
        type_name: impl Into<String>,
        visibility_channel: Option<String>,
        source_ref: SourceRef,
    ) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            visibility_channel,
            source_ref,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn visibility_channel(&self) -> Option<&str> {
        self.visibility_channel.as_deref()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedIndexedStateSchema {
    name: String,
    index_name: String,
    index_type: String,
    owner_locus: String,
    fields: Vec<CheckedStateFieldSchema>,
    source_ref: SourceRef,
}

impl CheckedIndexedStateSchema {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    pub fn index_type(&self) -> &str {
        &self.index_type
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn fields(&self) -> &[CheckedStateFieldSchema] {
        &self.fields
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedEvaluationParameter {
    name: String,
    type_name: String,
    source_ref: SourceRef,
}

impl CheckedEvaluationParameter {
    fn new(name: impl Into<String>, type_name: impl Into<String>, source_ref: SourceRef) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            source_ref,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedEvaluationSignature {
    name: String,
    kind: CheckedEvaluationKind,
    actor: Option<String>,
    owner_locus: Option<String>,
    parameters: Vec<CheckedEvaluationParameter>,
    source_ref: SourceRef,
}

impl CheckedEvaluationSignature {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn kind(&self) -> CheckedEvaluationKind {
        self.kind
    }

    pub fn actor(&self) -> Option<&str> {
        self.actor.as_deref()
    }

    pub fn owner_locus(&self) -> Option<&str> {
        self.owner_locus.as_deref()
    }

    pub fn parameters(&self) -> &[CheckedEvaluationParameter] {
        &self.parameters
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

/// The finite, read-only declarations retained from a checked M7 artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedStaticEnvironment {
    module: String,
    loci: Vec<CheckedStaticDeclaration>,
    principals: Vec<CheckedStaticDeclaration>,
    types: Vec<CheckedStaticDeclaration>,
    indexed_state_schemas: Vec<CheckedIndexedStateSchema>,
    evaluation_signatures: Vec<CheckedEvaluationSignature>,
}

impl CheckedStaticEnvironment {
    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn loci(&self) -> &[CheckedStaticDeclaration] {
        &self.loci
    }

    pub fn principals(&self) -> &[CheckedStaticDeclaration] {
        &self.principals
    }

    pub fn types(&self) -> &[CheckedStaticDeclaration] {
        &self.types
    }

    pub fn indexed_state_schemas(&self) -> &[CheckedIndexedStateSchema] {
        &self.indexed_state_schemas
    }

    pub fn indexed_state_schema(&self, name: &str) -> Option<&CheckedIndexedStateSchema> {
        self.indexed_state_schemas
            .iter()
            .find(|schema| schema.name == name)
    }

    pub fn evaluation_signatures(&self) -> &[CheckedEvaluationSignature] {
        &self.evaluation_signatures
    }

    pub fn evaluation_signature(&self, name: &str) -> Option<&CheckedEvaluationSignature> {
        self.evaluation_signatures
            .iter()
            .find(|signature| signature.name == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M7Diagnostic {
    kind: M7DiagnosticKind,
    span: PipelineSourceSpan,
    source_ref: SourceRef,
    generated_failure_reason: Option<M7GeneratedFailureReason>,
}

impl M7Diagnostic {
    fn new(kind: M7DiagnosticKind, span: PipelineSourceSpan) -> Self {
        let source_ref = span.source_ref();
        Self {
            kind,
            span,
            source_ref,
            generated_failure_reason: None,
        }
    }

    fn missing_generated_failure(span: PipelineSourceSpan, failure: impl Into<String>) -> Self {
        let source_ref = span.source_ref();
        Self {
            kind: M7DiagnosticKind::GeneratedFailureNotDeclared,
            span,
            source_ref,
            generated_failure_reason: Some(M7GeneratedFailureReason::MissingDeclaredFailure(
                failure.into(),
            )),
        }
    }

    pub const fn kind(&self) -> M7DiagnosticKind {
        self.kind
    }

    pub fn canonical_code(&self) -> &'static str {
        match (&self.kind, &self.generated_failure_reason) {
            (M7DiagnosticKind::GeneratedFailureNotDeclared, Some(reason))
                if reason.missing_failure() == VISIBILITY_DENIED_FAILURE =>
            {
                "E-ROW-002"
            }
            (M7DiagnosticKind::GeneratedFailureNotDeclared, _) => "E-ROW-001",
            _ => "E-M7-PROVISIONAL",
        }
    }

    pub fn generated_failure_reason(&self) -> Option<&M7GeneratedFailureReason> {
        self.generated_failure_reason.as_ref()
    }

    pub fn span(&self) -> &PipelineSourceSpan {
        &self.span
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0PipelineDiagnostics {
    entries: Vec<M7Diagnostic>,
}

impl SurfaceV0PipelineDiagnostics {
    fn one(kind: M7DiagnosticKind, span: PipelineSourceSpan) -> Self {
        Self {
            entries: vec![M7Diagnostic::new(kind, span)],
        }
    }

    fn missing_generated_failure(span: PipelineSourceSpan, failure: impl Into<String>) -> Self {
        Self {
            entries: vec![M7Diagnostic::missing_generated_failure(span, failure)],
        }
    }

    pub fn entries(&self) -> &[M7Diagnostic] {
        &self.entries
    }

    pub fn primary(&self) -> &M7Diagnostic {
        self.entries
            .first()
            .expect("M7 diagnostics always have a primary diagnostic")
    }

    pub const fn has_executable_core(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureRow {
    names: Vec<String>,
}

impl FailureRow {
    fn new(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }

    pub fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    pub fn is_subset_of(&self, declared: &Self) -> bool {
        self.names
            .iter()
            .all(|name| declared.names.iter().any(|candidate| candidate == name))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckedEvaluationKind {
    OwnerRmw,
    DesignatedPublishValue,
    PublishRelation,
    ConsumerLocalProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationAxes {
    semantic_form: SemanticForm,
    evaluation_site: EvaluationSite,
    trigger: TriggerClock,
    authority_origin: AuthorityOrigin,
    materialization: Materialization,
}

impl EvaluationAxes {
    fn new(
        semantic_form: SemanticForm,
        evaluation_site: EvaluationSite,
        trigger: TriggerClock,
        authority_origin: AuthorityOrigin,
        materialization: Materialization,
    ) -> Self {
        Self {
            semantic_form,
            evaluation_site,
            trigger,
            authority_origin,
            materialization,
        }
    }

    pub const fn semantic_form(&self) -> SemanticForm {
        self.semantic_form
    }

    pub fn evaluation_site(&self) -> &EvaluationSite {
        &self.evaluation_site
    }

    pub const fn trigger(&self) -> TriggerClock {
        self.trigger
    }

    pub fn authority_origin(&self) -> &AuthorityOrigin {
        &self.authority_origin
    }

    pub const fn materialization(&self) -> Materialization {
        self.materialization
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedStateRead {
    namespace: String,
    index: Option<String>,
    field: Option<String>,
    owner_locus: String,
    value_type: String,
    span: PipelineSourceSpan,
}

impl TypedStateRead {
    fn from_reference(ast: &SurfaceV0File, reference: &SurfaceReference) -> Self {
        let state = ast
            .state(reference.base())
            .expect("M7 checked state references resolve before Core construction");
        let field = reference
            .field()
            .expect("M7 checked expression references carry a state field");
        let value_type = state
            .fields()
            .iter()
            .find(|candidate| candidate.name() == field)
            .expect("M7 checked state fields resolve before Core construction")
            .type_name()
            .to_string();
        Self {
            namespace: reference.base().to_string(),
            index: reference.index().map(str::to_string),
            field: Some(field.to_string()),
            owner_locus: state.owner_locus().to_string(),
            value_type,
            span: PipelineSourceSpan::from_surface(reference.span()),
        }
    }

    fn from_target(ast: &SurfaceV0File, reference: &SurfaceReference) -> Self {
        Self::from_reference(ast, reference)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn index(&self) -> Option<&str> {
        self.index.as_deref()
    }

    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn value_type(&self) -> &str {
        &self.value_type
    }

    pub fn source_ref(&self) -> SourceRef {
        self.span.source_ref()
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span.lexeme(source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedIntegerLiteral {
    value: i64,
    span: PipelineSourceSpan,
}

impl CheckedIntegerLiteral {
    fn from_surface(literal: &mir_ast::surface_v0::BoundedIntegerLiteral) -> Self {
        Self {
            value: literal.value(),
            span: PipelineSourceSpan::from_surface(literal.span()),
        }
    }

    pub const fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckedBinaryOperator {
    Add,
    Subtract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckedExpressionTree {
    StateRead(TypedStateRead),
    ParameterRead {
        name: String,
        span: PipelineSourceSpan,
    },
    IntegerLiteral(CheckedIntegerLiteral),
    Binary {
        operator: CheckedBinaryOperator,
        span: PipelineSourceSpan,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl CheckedExpressionTree {
    fn from_surface(ast: &SurfaceV0File, tree: &BoundedExpressionTree) -> Option<Self> {
        match tree {
            BoundedExpressionTree::StateReference(reference) => Some(Self::StateRead(
                TypedStateRead::from_reference(ast, reference),
            )),
            BoundedExpressionTree::Identifier { name, span } => Some(Self::ParameterRead {
                name: name.clone(),
                span: PipelineSourceSpan::from_surface(span),
            }),
            BoundedExpressionTree::IntegerLiteral(literal) => Some(Self::IntegerLiteral(
                CheckedIntegerLiteral::from_surface(literal),
            )),
            BoundedExpressionTree::Binary {
                operator,
                span,
                left,
                right,
            } => Some(Self::Binary {
                operator: match operator {
                    mir_ast::surface_v0::BoundedBinaryOperator::Add => CheckedBinaryOperator::Add,
                    mir_ast::surface_v0::BoundedBinaryOperator::Subtract => {
                        CheckedBinaryOperator::Subtract
                    }
                },
                span: PipelineSourceSpan::from_surface(span),
                left: Box::new(Self::from_surface(ast, left)?),
                right: Box::new(Self::from_surface(ast, right)?),
            }),
            BoundedExpressionTree::Opaque { .. } => None,
        }
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span().lexeme(source)
    }

    pub fn operator(&self) -> Option<CheckedBinaryOperator> {
        match self {
            Self::Binary { operator, .. } => Some(*operator),
            Self::StateRead(_) | Self::ParameterRead { .. } | Self::IntegerLiteral(_) => None,
        }
    }

    pub fn left(&self) -> &Self {
        match self {
            Self::Binary { left, .. } => left,
            Self::StateRead(_) | Self::ParameterRead { .. } | Self::IntegerLiteral(_) => {
                panic!("only checked binary expression trees have a left child")
            }
        }
    }

    pub fn right(&self) -> &Self {
        match self {
            Self::Binary { right, .. } => right,
            Self::StateRead(_) | Self::ParameterRead { .. } | Self::IntegerLiteral(_) => {
                panic!("only checked binary expression trees have a right child")
            }
        }
    }

    pub fn int_literal(&self) -> Option<&CheckedIntegerLiteral> {
        match self {
            Self::IntegerLiteral(literal) => Some(literal),
            Self::StateRead(_) | Self::ParameterRead { .. } | Self::Binary { .. } => None,
        }
    }

    pub const fn is_m8_consumable(&self) -> bool {
        true
    }

    fn span(&self) -> &PipelineSourceSpan {
        match self {
            Self::StateRead(read) => &read.span,
            Self::ParameterRead { span, .. } => span,
            Self::IntegerLiteral(literal) => &literal.span,
            Self::Binary { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedExpression {
    span: PipelineSourceSpan,
    state_reads: Vec<TypedStateRead>,
    int_literals: Vec<i64>,
    operator_chain: Vec<String>,
    tree: CheckedExpressionTree,
}

impl TypedExpression {
    fn from_surface(ast: &SurfaceV0File, expression: &BoundedExpression) -> Self {
        Self {
            span: PipelineSourceSpan::from_surface(expression.span()),
            state_reads: expression
                .state_refs()
                .iter()
                .map(|reference| TypedStateRead::from_reference(ast, reference))
                .collect(),
            int_literals: expression
                .int_literals()
                .iter()
                .map(|literal| literal.value())
                .collect(),
            operator_chain: expression
                .binary_ops()
                .iter()
                .map(|operation| match operation.operator() {
                    mir_ast::surface_v0::BoundedBinaryOperator::Add => "+".to_string(),
                    mir_ast::surface_v0::BoundedBinaryOperator::Subtract => "-".to_string(),
                })
                .collect(),
            tree: CheckedExpressionTree::from_surface(ast, expression.tree())
                .expect("M7 only constructs checked Core after finite-expression validation"),
        }
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span.lexeme(source)
    }

    pub fn operator_chain(&self) -> Vec<&str> {
        self.operator_chain.iter().map(String::as_str).collect()
    }

    pub fn int_literals(&self) -> Vec<i64> {
        self.int_literals.clone()
    }

    pub fn state_reads(&self) -> &[TypedStateRead] {
        &self.state_reads
    }

    pub fn tree(&self) -> &CheckedExpressionTree {
        &self.tree
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerRmwCheckedCore {
    authority_origin_locus: String,
    owner_locus: String,
    target: TypedStateRead,
    expression: TypedExpression,
}

impl OwnerRmwCheckedCore {
    pub fn authority_origin_locus(&self) -> &str {
        &self.authority_origin_locus
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn target(&self) -> &TypedStateRead {
        &self.target
    }

    pub fn expression(&self) -> &TypedExpression {
        &self.expression
    }

    pub fn same_owner_reads(&self) -> &[TypedStateRead] {
        self.expression.state_reads()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationTransformCore {
    kind: String,
    translation: Option<(i64, i64)>,
}

impl RelationTransformCore {
    fn from_surface(transform: &RelationTransform) -> Self {
        match transform {
            RelationTransform::Translate { x, y } => Self {
                kind: "translate".to_string(),
                translation: Some((*x, *y)),
            },
            RelationTransform::Identity => Self {
                kind: "identity".to_string(),
                translation: Some((0, 0)),
            },
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub const fn translation(&self) -> Option<(i64, i64)> {
        self.translation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationAnchorCore {
    anchor: String,
    epoch: String,
    transform: RelationTransformCore,
}

impl RelationAnchorCore {
    fn from_surface(anchor: &mir_ast::surface_v0::RelationAnchor) -> Self {
        Self {
            anchor: anchor.anchor().to_string(),
            epoch: anchor.epoch().to_string(),
            transform: RelationTransformCore::from_surface(anchor.transform()),
        }
    }

    pub fn anchor(&self) -> &str {
        &self.anchor
    }

    pub fn epoch(&self) -> &str {
        &self.epoch
    }

    pub fn transform(&self) -> &RelationTransformCore {
        &self.transform
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationCheckedCore {
    owner_locus: String,
    subject: String,
    subject_type: String,
    primary: RelationAnchorCore,
    fallback: RelationAnchorCore,
    binding_frontier: BindingActivationFrontier,
    consumer_projection_locus: Option<String>,
}

impl RelationCheckedCore {
    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn subject_type(&self) -> &str {
        &self.subject_type
    }

    pub fn primary(&self) -> &RelationAnchorCore {
        &self.primary
    }

    pub fn fallback(&self) -> &RelationAnchorCore {
        &self.fallback
    }

    pub fn binding_frontier(&self) -> &BindingActivationFrontier {
        &self.binding_frontier
    }

    pub fn consumer_projection_locus(&self) -> Option<&str> {
        self.consumer_projection_locus.as_deref()
    }

    pub const fn publishes_relation_carrier(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedTriggerCore {
    frontier: String,
}

impl DesignatedTriggerCore {
    pub const fn kind(&self) -> &str {
        "logical-tick"
    }

    pub fn frontier(&self) -> Option<&str> {
        Some(&self.frontier)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedMaterializationCore;

impl DesignatedMaterializationCore {
    pub const fn kind(&self) -> &str {
        "publish-value"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedInputRequest {
    source_owner_locus: String,
    typed_state_read: TypedStateRead,
}

impl DesignatedInputRequest {
    pub fn source_owner_locus(&self) -> &str {
        &self.source_owner_locus
    }

    pub fn typed_state_read(&self) -> &TypedStateRead {
        &self.typed_state_read
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedInputReceiptUse {
    source_owner_locus: String,
    typed_state_read: TypedStateRead,
}

impl DesignatedInputReceiptUse {
    pub fn source_owner_locus(&self) -> &str {
        &self.source_owner_locus
    }

    pub fn typed_state_read(&self) -> &TypedStateRead {
        &self.typed_state_read
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedRemoteInputDependency {
    designated_evaluator: String,
    requester_site: EvaluationSite,
    authority_origin: AuthorityOrigin,
    source_owner_locus: String,
    typed_state_read: TypedStateRead,
    request: DesignatedInputRequest,
    receipt_use: DesignatedInputReceiptUse,
}

impl DesignatedRemoteInputDependency {
    pub fn designated_evaluator(&self) -> &str {
        &self.designated_evaluator
    }

    pub fn requester_site(&self) -> &EvaluationSite {
        &self.requester_site
    }

    pub fn authority_origin(&self) -> &AuthorityOrigin {
        &self.authority_origin
    }

    pub fn source_owner_locus(&self) -> &str {
        &self.source_owner_locus
    }

    pub fn typed_state_read(&self) -> &TypedStateRead {
        &self.typed_state_read
    }

    pub fn request(&self) -> &DesignatedInputRequest {
        &self.request
    }

    pub fn receipt_use(&self) -> &DesignatedInputReceiptUse {
        &self.receipt_use
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedCheckedCore {
    evaluator: String,
    result: String,
    trigger: DesignatedTriggerCore,
    result_frontier: ResultFrontier,
    input_frontier: InputFrontier,
    result_version: ResultVersion,
    evaluation_policy: EvaluationPolicy,
    observation_policy: ObservationPolicy,
    policy_stamp: PolicyStamp,
    materialization: DesignatedMaterializationCore,
    expression: TypedExpression,
    generated_remote_input_dependencies: Vec<DesignatedRemoteInputDependency>,
}

impl DesignatedCheckedCore {
    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub fn trigger(&self) -> &DesignatedTriggerCore {
        &self.trigger
    }

    pub fn result_frontier(&self) -> &ResultFrontier {
        &self.result_frontier
    }

    pub fn input_frontier(&self) -> &InputFrontier {
        &self.input_frontier
    }

    pub const fn result_version(&self) -> ResultVersion {
        self.result_version
    }

    pub fn evaluation_policy(&self) -> &EvaluationPolicy {
        &self.evaluation_policy
    }

    pub fn observation_policy(&self) -> &ObservationPolicy {
        &self.observation_policy
    }

    pub fn policy_stamp(&self) -> &PolicyStamp {
        &self.policy_stamp
    }

    pub const fn materialization(&self) -> &DesignatedMaterializationCore {
        &self.materialization
    }

    pub fn expression(&self) -> &TypedExpression {
        &self.expression
    }

    pub fn generated_remote_input_dependencies(&self) -> &[DesignatedRemoteInputDependency] {
        &self.generated_remote_input_dependencies
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectKind {
    OwnerRequest,
    OwnerLocalRead,
    OwnerWrite,
    /// Present in the typed row vocabulary so consumers can reject an
    /// accidental actor reply; M7 never emits it for owner-local reads.
    ActorReadReply,
    ObserverPublish,
    RelationPublish,
    DesignatedRemoteRequest,
    DesignatedReceiptUse,
    DesignatedValuePublish,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectEntry {
    kind: EffectKind,
    span: PipelineSourceSpan,
    source_ref: SourceRef,
    caller: Option<String>,
    owner: Option<String>,
    namespace: Option<String>,
    field: Option<String>,
    redaction_label: Option<String>,
    failure: Option<String>,
    relation: Option<String>,
    evaluator: Option<String>,
    result: Option<String>,
}

impl EffectEntry {
    fn new(kind: EffectKind, span: PipelineSourceSpan) -> Self {
        let source_ref = span.source_ref();
        Self {
            kind,
            span,
            source_ref,
            caller: None,
            owner: None,
            namespace: None,
            field: None,
            redaction_label: None,
            failure: None,
            relation: None,
            evaluator: None,
            result: None,
        }
    }

    pub const fn kind(&self) -> EffectKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span.lexeme(source)
    }

    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    pub fn redaction_label(&self) -> &str {
        self.redaction_label.as_deref().unwrap_or("")
    }

    pub fn failure(&self) -> Option<&str> {
        self.failure.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EffectRow {
    entries: Vec<EffectEntry>,
}

impl EffectRow {
    pub fn entries(&self) -> &[EffectEntry] {
        &self.entries
    }

    pub fn contains_request_to_owner(&self, caller: &str, owner: &str) -> bool {
        self.entries.iter().any(|entry| {
            entry.kind == EffectKind::OwnerRequest
                && entry.caller.as_deref() == Some(caller)
                && entry.owner.as_deref() == Some(owner)
        })
    }

    pub fn contains_owner_write(&self, owner: &str, namespace: &str, field: &str) -> bool {
        self.entries.iter().any(|entry| {
            entry.kind == EffectKind::OwnerWrite
                && entry.owner.as_deref() == Some(owner)
                && entry.namespace.as_deref() == Some(namespace)
                && entry.field.as_deref() == Some(field)
        })
    }

    pub fn contains_state_write(&self, owner: &str, namespace: &str, field: &str) -> bool {
        self.contains_owner_write(owner, namespace, field)
    }

    pub fn contains_relation_publication(&self, relation: &str) -> bool {
        self.entries.iter().any(|entry| {
            entry.kind == EffectKind::RelationPublish && entry.relation.as_deref() == Some(relation)
        })
    }

    pub fn contains_value_publication(&self, evaluator: &str, result: &str) -> bool {
        self.entries.iter().any(|entry| {
            entry.kind == EffectKind::DesignatedValuePublish
                && entry.evaluator.as_deref() == Some(evaluator)
                && entry.result.as_deref() == Some(result)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratedObligationKind {
    Failure(String),
    Capability,
    Witness,
    Authority,
    AdmittedEvaluatorAuthority,
    Evaluation(CheckedEvaluationKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedObligation {
    kind: GeneratedObligationKind,
    span: PipelineSourceSpan,
    source_ref: SourceRef,
}

impl GeneratedObligation {
    fn new(kind: GeneratedObligationKind, span: PipelineSourceSpan) -> Self {
        let source_ref = span.source_ref();
        Self {
            kind,
            span,
            source_ref,
        }
    }

    pub fn kind(&self) -> &GeneratedObligationKind {
        &self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span.lexeme(source)
    }

    pub const fn grants_authority_success(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedObligations {
    entries: Vec<GeneratedObligation>,
}

impl GeneratedObligations {
    fn owner_rmw(span: PipelineSourceSpan, observer_safe: bool) -> Self {
        let mut entries = owner_rmw_failure_names(observer_safe)
            .iter()
            .map(|failure| {
                GeneratedObligation::new(
                    GeneratedObligationKind::Failure(failure.clone()),
                    span.clone(),
                )
            })
            .collect::<Vec<_>>();
        entries.extend([
            GeneratedObligation::new(GeneratedObligationKind::Capability, span.clone()),
            GeneratedObligation::new(GeneratedObligationKind::Witness, span.clone()),
            GeneratedObligation::new(
                GeneratedObligationKind::Evaluation(CheckedEvaluationKind::OwnerRmw),
                span,
            ),
        ]);
        Self { entries }
    }

    fn relation(span: PipelineSourceSpan) -> Self {
        Self {
            entries: vec![
                GeneratedObligation::new(GeneratedObligationKind::Authority, span.clone()),
                GeneratedObligation::new(
                    GeneratedObligationKind::Evaluation(CheckedEvaluationKind::PublishRelation),
                    span,
                ),
            ],
        }
    }

    fn designated(span: PipelineSourceSpan) -> Self {
        Self {
            entries: vec![
                GeneratedObligation::new(
                    GeneratedObligationKind::AdmittedEvaluatorAuthority,
                    span.clone(),
                ),
                GeneratedObligation::new(
                    GeneratedObligationKind::Evaluation(
                        CheckedEvaluationKind::DesignatedPublishValue,
                    ),
                    span,
                ),
            ],
        }
    }

    pub fn entries(&self) -> &[GeneratedObligation] {
        &self.entries
    }

    pub fn failure_names(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter_map(|entry| match &entry.kind {
                GeneratedObligationKind::Failure(name) => Some(name.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn contains_capability(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == GeneratedObligationKind::Capability)
    }

    pub fn contains_witness(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == GeneratedObligationKind::Witness)
    }

    pub fn contains_authority(&self) -> bool {
        self.entries.iter().any(|entry| {
            matches!(
                entry.kind,
                GeneratedObligationKind::Authority
                    | GeneratedObligationKind::AdmittedEvaluatorAuthority
            )
        })
    }

    pub fn contains_evaluation(&self, kind: CheckedEvaluationKind) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == GeneratedObligationKind::Evaluation(kind))
    }
}

fn owner_rmw_failure_names(observer_safe: bool) -> Vec<String> {
    let mut names = OWNER_RMW_FAILURES
        .iter()
        .map(|failure| (*failure).to_string())
        .collect::<Vec<_>>();
    if observer_safe {
        names.push(VISIBILITY_DENIED_FAILURE.to_string());
    }
    names
}

fn observer_visibility_channel<'a>(
    ast: &'a SurfaceV0File,
    reference: &SurfaceReference,
) -> Option<&'a str> {
    ast.state(reference.base())?
        .field(reference.field()?)?
        .visibility()
        .map(|visibility| visibility.channel())
}

fn designated_effect_entries(
    span: PipelineSourceSpan,
    evaluator: &str,
    result: &str,
    input_count: usize,
) -> Vec<EffectEntry> {
    let mut entries = Vec::with_capacity(input_count.saturating_mul(2) + 1);
    for _ in 0..input_count {
        entries.push(EffectEntry {
            evaluator: Some(evaluator.to_string()),
            result: Some(result.to_string()),
            ..EffectEntry::new(EffectKind::DesignatedRemoteRequest, span.clone())
        });
        entries.push(EffectEntry {
            evaluator: Some(evaluator.to_string()),
            result: Some(result.to_string()),
            ..EffectEntry::new(EffectKind::DesignatedReceiptUse, span.clone())
        });
    }
    entries.push(EffectEntry {
        evaluator: Some(evaluator.to_string()),
        result: Some(result.to_string()),
        ..EffectEntry::new(EffectKind::DesignatedValuePublish, span)
    });
    entries
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedEvaluation {
    kind: CheckedEvaluationKind,
    name: String,
    result_name: Option<String>,
    actor_authority_origin: String,
    authority_origin_locus: String,
    owner_evaluation_locus: String,
    declared_failure_row: FailureRow,
    generated_failure_row: FailureRow,
    owner_rmw_core: Option<OwnerRmwCheckedCore>,
    relation_core: Option<RelationCheckedCore>,
    designated_core: Option<DesignatedCheckedCore>,
    evaluation_axes: EvaluationAxes,
    effect_row: EffectRow,
    generated_obligations: GeneratedObligations,
}

impl CheckedEvaluation {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn result_name(&self) -> Option<&str> {
        self.result_name.as_deref()
    }

    pub const fn kind(&self) -> CheckedEvaluationKind {
        self.kind
    }

    pub fn source_ref(&self) -> &SourceRef {
        self.effect_row
            .entries()
            .first()
            .expect("every M7 checked evaluation has a source-bound effect entry")
            .source_ref()
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.effect_row
            .entries()
            .first()
            .expect("every M7 checked evaluation has a source-bound effect entry")
            .source_lexeme(source)
    }

    pub fn actor_authority_origin(&self) -> &str {
        &self.actor_authority_origin
    }

    pub fn authority_origin_locus(&self) -> &str {
        &self.authority_origin_locus
    }

    pub fn owner_evaluation_locus(&self) -> &str {
        &self.owner_evaluation_locus
    }

    pub fn declared_failure_row(&self) -> &FailureRow {
        &self.declared_failure_row
    }

    pub fn generated_failure_row(&self) -> &FailureRow {
        &self.generated_failure_row
    }

    pub fn owner_rmw_core(&self) -> Option<&OwnerRmwCheckedCore> {
        self.owner_rmw_core.as_ref()
    }

    pub fn relation_core(&self) -> Option<&RelationCheckedCore> {
        self.relation_core.as_ref()
    }

    pub fn designated_core(&self) -> Option<&DesignatedCheckedCore> {
        self.designated_core.as_ref()
    }

    pub fn evaluation_axes(&self) -> &EvaluationAxes {
        &self.evaluation_axes
    }

    pub fn effect_row(&self) -> &EffectRow {
        &self.effect_row
    }

    pub fn generated_obligations(&self) -> &GeneratedObligations {
        &self.generated_obligations
    }

    pub fn binding_frontier(&self) -> &BindingActivationFrontier {
        self.relation_core
            .as_ref()
            .expect("only relation evaluations carry a binding frontier")
            .binding_frontier()
    }

    pub const fn publishes_relation_carrier(&self) -> bool {
        self.relation_core.is_some()
    }

    pub fn consumer_projection_locus(&self) -> Option<&str> {
        self.relation_core
            .as_ref()
            .and_then(RelationCheckedCore::consumer_projection_locus)
    }

    pub fn consumer_projection_kind(&self) -> CheckedEvaluationKind {
        self.consumer_projection_locus()
            .map(|_| CheckedEvaluationKind::ConsumerLocalProjection)
            .expect("only projected relations have a consumer projection kind")
    }

    pub fn result_frontier(&self) -> &ResultFrontier {
        self.designated_core
            .as_ref()
            .expect("only designated evaluations carry a result frontier")
            .result_frontier()
    }

    pub fn result_version(&self) -> ResultVersion {
        self.designated_core
            .as_ref()
            .expect("only designated evaluations carry a result version")
            .result_version()
    }

    pub const fn publishes_value(&self) -> bool {
        self.designated_core.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidualObligationKind {
    Visibility,
    RelationLifetime,
    FallbackValidity,
    ValueVisibilityRedaction,
    AuthDeferred,
    VerifyDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidualObligation {
    kind: ResidualObligationKind,
    name: String,
    span: PipelineSourceSpan,
    source_ref: SourceRef,
    required_authority: Option<String>,
}

impl ResidualObligation {
    fn new(
        kind: ResidualObligationKind,
        name: String,
        span: PipelineSourceSpan,
        required_authority: Option<String>,
    ) -> Self {
        let source_ref = span.source_ref();
        Self {
            kind,
            name,
            span,
            source_ref,
            required_authority,
        }
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span.lexeme(source)
    }

    pub const fn kind(&self) -> ResidualObligationKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn required_authority(&self) -> Option<&str> {
        self.required_authority.as_deref()
    }

    pub const fn is_non_executable(&self) -> bool {
        true
    }

    pub const fn grants_authority(&self) -> bool {
        false
    }

    pub const fn emits_effect(&self) -> bool {
        false
    }

    pub const fn mutates_state(&self) -> bool {
        false
    }

    pub const fn emits_verdict(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResidualObligations {
    entries: Vec<ResidualObligation>,
}

impl ResidualObligations {
    pub fn entries(&self) -> &[ResidualObligation] {
        &self.entries
    }

    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn contains_kind(&self, kind: ResidualObligationKind) -> bool {
        self.entries.iter().any(|entry| entry.kind == kind)
    }

    pub fn for_kind_and_name(
        &self,
        kind: ResidualObligationKind,
        name: &str,
    ) -> Option<&ResidualObligation> {
        self.entries
            .iter()
            .find(|entry| entry.kind == kind && entry.name == name)
    }

    fn execution_blocker(&self) -> Option<&ResidualObligation> {
        self.entries
            .iter()
            .find(|entry| entry.kind == ResidualObligationKind::VerifyDeferred)
            .or_else(|| self.entries.first())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedSourceMapEntry {
    ordinal: usize,
    span: PipelineSourceSpan,
    source_ref: SourceRef,
    kind: SourceToCoreKind,
    core_ref: String,
}

impl CheckedSourceMapEntry {
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn core_ref(&self) -> &str {
        &self.core_ref
    }

    pub const fn kind(&self) -> SourceToCoreKind {
        self.kind
    }

    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span.lexeme(source)
    }

    pub fn stable_key(&self) -> String {
        format!(
            "{:020}:{:020}:{}",
            self.span.byte_start,
            self.kind_rank(),
            self.core_ref
        )
    }

    fn kind_rank(&self) -> usize {
        match self.kind {
            SourceToCoreKind::OwnerRmw => 0,
            SourceToCoreKind::OwnerLocalRead => 1,
            SourceToCoreKind::OwnerLocalWrite => 2,
            SourceToCoreKind::ObserverPublish => 3,
            SourceToCoreKind::DesignatedDecision => 4,
            SourceToCoreKind::PublishRelation => 5,
            SourceToCoreKind::ConsumerLocalProjection => 6,
            SourceToCoreKind::DeferredPolicy => 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CheckedSourceMap {
    entries: Vec<CheckedSourceMapEntry>,
}

impl CheckedSourceMap {
    fn add(&mut self, span: PipelineSourceSpan, kind: SourceToCoreKind, core_ref: String) {
        let source_ref = span.source_ref();
        self.entries.push(CheckedSourceMapEntry {
            ordinal: 0,
            span,
            source_ref,
            kind,
            core_ref,
        });
    }

    fn finalize(&mut self) {
        self.entries.sort_by_key(CheckedSourceMapEntry::stable_key);
        for (ordinal, entry) in self.entries.iter_mut().enumerate() {
            entry.ordinal = ordinal;
        }
    }

    pub fn entries(&self) -> &[CheckedSourceMapEntry] {
        &self.entries
    }

    pub fn entries_for_lexeme(
        &self,
        source: &str,
        lexeme: &str,
    ) -> Option<CheckedSourceMapEntries> {
        let entries = self
            .entries
            .iter()
            .filter(|entry| entry.source_lexeme(source) == lexeme)
            .collect::<Vec<_>>();
        let source_ref = entries.first()?.source_ref.clone();
        Some(CheckedSourceMapEntries {
            kinds: entries.iter().map(|entry| entry.kind).collect(),
            source_ref,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedSourceMapEntries {
    kinds: Vec<SourceToCoreKind>,
    source_ref: SourceRef,
}

impl CheckedSourceMapEntries {
    pub fn kinds(&self) -> Vec<SourceToCoreKind> {
        self.kinds.clone()
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedSurfaceV0 {
    source_file: String,
    root_span: PipelineSourceSpan,
    program_identity: CheckedProgramIdentity,
    static_environment: CheckedStaticEnvironment,
    evaluations: Vec<CheckedEvaluation>,
    source_map: CheckedSourceMap,
    residual_obligations: ResidualObligations,
    consumed_m6_classification: SurfaceV0Classification,
}

impl CheckedSurfaceV0 {
    pub fn source_file(&self) -> &str {
        &self.source_file
    }

    pub fn program_identity(&self) -> &CheckedProgramIdentity {
        &self.program_identity
    }

    pub fn static_environment(&self) -> &CheckedStaticEnvironment {
        &self.static_environment
    }

    pub fn evaluations(&self) -> &[CheckedEvaluation] {
        &self.evaluations
    }

    pub fn evaluation(&self, name: &str) -> Option<&CheckedEvaluation> {
        self.evaluations.iter().find(|evaluation| {
            evaluation.kind == CheckedEvaluationKind::OwnerRmw && evaluation.name == name
        })
    }

    pub fn relation(&self, name: &str) -> Option<&CheckedEvaluation> {
        self.evaluations.iter().find(|evaluation| {
            evaluation.kind == CheckedEvaluationKind::PublishRelation && evaluation.name == name
        })
    }

    pub fn designated_result(&self, evaluator: &str, result: &str) -> Option<&CheckedEvaluation> {
        self.evaluations.iter().find(|evaluation| {
            evaluation.kind == CheckedEvaluationKind::DesignatedPublishValue
                && evaluation.name == evaluator
                && evaluation.result_name.as_deref() == Some(result)
        })
    }

    pub fn source_map(&self) -> &CheckedSourceMap {
        &self.source_map
    }

    pub fn residual_obligations(&self) -> &ResidualObligations {
        &self.residual_obligations
    }

    pub fn consumed_m6_classification(&self) -> &SurfaceV0Classification {
        &self.consumed_m6_classification
    }

    pub fn execution_is_admissible(&self) -> bool {
        !self.evaluations.is_empty() && self.residual_obligations.is_empty()
    }

    pub fn require_execution_admission(&self) -> Result<(), SurfaceV0PipelineDiagnostics> {
        if self.execution_is_admissible() {
            return Ok(());
        }
        let span = self
            .residual_obligations
            .execution_blocker()
            .map(|obligation| obligation.span.clone())
            .unwrap_or_else(|| self.root_span.clone());
        Err(SurfaceV0PipelineDiagnostics::one(
            M7DiagnosticKind::ResidualCannotExecute,
            span,
        ))
    }
}

/// Parses, classifies, checks, and elaborates one bounded Surface v0 source
/// through the M7 static authority route. No legacy runner or fixture report is
/// involved.
pub fn check_and_elaborate_surface_v0(
    source: FixtureSource,
) -> Result<CheckedSurfaceV0, SurfaceV0PipelineDiagnostics> {
    let ast = parse_surface_v0(source.clone()).map_err(forward_parse_diagnostic)?;
    let classification = classify_surface_v0(&ast, SurfaceV0ClassificationOptions::default())
        .map_err(forward_classification_diagnostic)?;
    if let Some(diagnostic) = m7_static_diagnostic(&source, &ast) {
        return Err(diagnostic);
    }
    Ok(build_checked_artifact(source, ast, classification))
}

fn checked_static_environment(
    ast: &SurfaceV0File,
    evaluations: &[CheckedEvaluation],
) -> CheckedStaticEnvironment {
    let evaluation_signatures = evaluations
        .iter()
        .map(|evaluation| match evaluation.kind() {
            CheckedEvaluationKind::OwnerRmw => {
                let when = ast
                    .when(evaluation.name())
                    .expect("checked owner evaluation retains its declared event");
                CheckedEvaluationSignature {
                    name: evaluation.name().to_string(),
                    kind: evaluation.kind(),
                    actor: Some(evaluation.actor_authority_origin().to_string()),
                    owner_locus: Some(evaluation.owner_evaluation_locus().to_string()),
                    parameters: when
                        .parameters()
                        .iter()
                        .map(|parameter| {
                            CheckedEvaluationParameter::new(
                                parameter.name(),
                                parameter.type_name(),
                                PipelineSourceSpan::from_surface(parameter.span()).source_ref(),
                            )
                        })
                        .collect(),
                    source_ref: evaluation.source_ref().clone(),
                }
            }
            CheckedEvaluationKind::PublishRelation => {
                let relation = ast
                    .relation(evaluation.name())
                    .expect("checked relation evaluation retains its declaration");
                CheckedEvaluationSignature {
                    name: evaluation.name().to_string(),
                    kind: evaluation.kind(),
                    actor: None,
                    owner_locus: Some(evaluation.owner_evaluation_locus().to_string()),
                    parameters: vec![CheckedEvaluationParameter::new(
                        relation.subject(),
                        relation.subject_type(),
                        PipelineSourceSpan::from_surface(relation.span()).source_ref(),
                    )],
                    source_ref: evaluation.source_ref().clone(),
                }
            }
            CheckedEvaluationKind::DesignatedPublishValue => CheckedEvaluationSignature {
                name: format!(
                    "{}.{}",
                    evaluation.name(),
                    evaluation
                        .result_name()
                        .expect("checked designated evaluation retains its result name")
                ),
                kind: evaluation.kind(),
                actor: None,
                owner_locus: None,
                parameters: Vec::new(),
                source_ref: evaluation.source_ref().clone(),
            },
            CheckedEvaluationKind::ConsumerLocalProjection => {
                unreachable!("M7 has no standalone consumer-projection evaluation")
            }
        })
        .collect();

    CheckedStaticEnvironment {
        module: ast.module().name().to_string(),
        loci: ast
            .loci()
            .iter()
            .map(|decl| {
                CheckedStaticDeclaration::new(
                    decl.name(),
                    PipelineSourceSpan::from_surface(decl.span()).source_ref(),
                )
            })
            .collect(),
        principals: ast
            .principals()
            .iter()
            .map(|decl| {
                CheckedStaticDeclaration::new(
                    decl.name(),
                    PipelineSourceSpan::from_surface(decl.span()).source_ref(),
                )
            })
            .collect(),
        types: ast
            .types()
            .iter()
            .map(|decl| {
                CheckedStaticDeclaration::new(
                    decl.name(),
                    PipelineSourceSpan::from_surface(decl.span()).source_ref(),
                )
            })
            .collect(),
        indexed_state_schemas: ast
            .states()
            .iter()
            .map(|state| CheckedIndexedStateSchema {
                name: state.name().to_string(),
                index_name: state.index_name().to_string(),
                index_type: state.index_type().to_string(),
                owner_locus: state.owner_locus().to_string(),
                fields: state
                    .fields()
                    .iter()
                    .map(|field| {
                        CheckedStateFieldSchema::new(
                            field.name(),
                            field.type_name(),
                            field
                                .visibility()
                                .map(|visibility| visibility.channel().to_string()),
                            PipelineSourceSpan::from_surface(field.span()).source_ref(),
                        )
                    })
                    .collect(),
                source_ref: PipelineSourceSpan::from_surface(state.span()).source_ref(),
            })
            .collect(),
        evaluation_signatures,
    }
}

fn checked_identity_structure(
    environment: &CheckedStaticEnvironment,
    evaluations: &[CheckedEvaluation],
    source_map: &CheckedSourceMap,
) -> Vec<String> {
    let mut entries = vec![format!("module:{}", environment.module())];
    entries.extend(
        environment
            .loci()
            .iter()
            .map(|decl| format!("locus:{}:{:?}", decl.name(), decl.source_ref())),
    );
    entries.extend(
        environment
            .principals()
            .iter()
            .map(|decl| format!("principal:{}:{:?}", decl.name(), decl.source_ref())),
    );
    entries.extend(
        environment
            .types()
            .iter()
            .map(|decl| format!("type:{}:{:?}", decl.name(), decl.source_ref())),
    );
    entries.extend(environment.indexed_state_schemas().iter().map(|schema| {
        format!(
            "state:{}:{}:{}:{}:{:?}",
            schema.name(),
            schema.index_name(),
            schema.index_type(),
            schema.owner_locus(),
            schema.fields()
        )
    }));
    entries.extend(environment.evaluation_signatures().iter().map(|signature| {
        format!(
            "signature:{}:{:?}:{:?}:{:?}:{:?}:{:?}",
            signature.name(),
            signature.kind(),
            signature.actor(),
            signature.owner_locus(),
            signature.parameters(),
            signature.source_ref()
        )
    }));
    entries.extend(evaluations.iter().map(|evaluation| {
        let core = match evaluation.kind() {
            CheckedEvaluationKind::OwnerRmw => {
                let owner = evaluation
                    .owner_rmw_core()
                    .expect("owner evaluation retains its checked Core");
                format!(
                    "owner:{:?}:{:?}:{:?}",
                    owner.target(),
                    owner.expression(),
                    owner.same_owner_reads()
                )
            }
            CheckedEvaluationKind::PublishRelation => {
                let relation = evaluation
                    .relation_core()
                    .expect("relation evaluation retains its checked Core");
                format!(
                    "relation:{}:{}:{}:{:?}:{:?}:{:?}:{:?}",
                    relation.owner_locus(),
                    relation.subject(),
                    relation.subject_type(),
                    relation.primary(),
                    relation.fallback(),
                    relation.binding_frontier(),
                    relation.consumer_projection_locus()
                )
            }
            CheckedEvaluationKind::DesignatedPublishValue => {
                let designated = evaluation
                    .designated_core()
                    .expect("designated evaluation retains its checked Core");
                format!(
                    "designated:{}:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
                    designated.evaluator(),
                    designated.result(),
                    designated.trigger(),
                    designated.result_frontier(),
                    designated.input_frontier(),
                    designated.result_version(),
                    designated.evaluation_policy(),
                    designated.observation_policy(),
                    designated.expression(),
                )
            }
            CheckedEvaluationKind::ConsumerLocalProjection => {
                unreachable!("M7 has no standalone consumer-projection evaluation")
            }
        };
        format!(
            "evaluation:{}:{:?}:{}:{:?}:{:?}:{:?}:{:?}:{:?}",
            evaluation.name(),
            evaluation.kind(),
            core,
            evaluation.evaluation_axes(),
            evaluation.effect_row().entries(),
            evaluation.generated_obligations().entries(),
            evaluation.declared_failure_row(),
            evaluation.generated_failure_row(),
        )
    }));
    entries.extend(source_map.entries().iter().map(|entry| {
        format!(
            "source-map:{}:{:?}:{}:{:?}",
            entry.ordinal(),
            entry.kind(),
            entry.core_ref(),
            entry.source_ref()
        )
    }));
    entries
}

fn build_checked_artifact(
    source: FixtureSource,
    ast: SurfaceV0File,
    consumed_m6_classification: SurfaceV0Classification,
) -> CheckedSurfaceV0 {
    let mut evaluations = Vec::new();
    let mut source_map = CheckedSourceMap::default();
    for assignment in ast.assignments() {
        let source_span = PipelineSourceSpan::from_surface(
            consumed_m6_classification
                .core_template(assignment.event())
                .expect("accepted M6 classification retains every owner-RMW template")
                .source_span(),
        );
        let target = TypedStateRead::from_target(&ast, assignment.target());
        let expression = TypedExpression::from_surface(&ast, assignment.expression());
        let observer_safe =
            observer_visibility_channel(&ast, assignment.target()) == Some(OBSERVER_SAFE_CHANNEL);
        let owner_core = OwnerRmwCheckedCore {
            authority_origin_locus: assignment.role_locus().to_string(),
            owner_locus: assignment.owner_locus().to_string(),
            target: target.clone(),
            expression,
        };
        let failure_row = ast
            .when(assignment.event())
            .expect("accepted assignment is nested under its declared event");
        let mut request = EffectEntry::new(EffectKind::OwnerRequest, source_span.clone());
        request.caller = Some(assignment.role_locus().to_string());
        request.owner = Some(assignment.owner_locus().to_string());
        let mut local_read = EffectEntry::new(EffectKind::OwnerLocalRead, source_span.clone());
        local_read.owner = Some(assignment.owner_locus().to_string());
        local_read.namespace = Some(target.namespace.clone());
        local_read.field = target.field.clone();
        let mut owner_write = EffectEntry::new(EffectKind::OwnerWrite, source_span.clone());
        owner_write.owner = Some(assignment.owner_locus().to_string());
        owner_write.namespace = Some(target.namespace.clone());
        owner_write.field = target.field.clone();
        let observer_publish = observer_safe.then(|| EffectEntry {
            namespace: Some(target.namespace.clone()),
            field: target.field.clone(),
            redaction_label: Some(OBSERVER_SAFE_CHANNEL.to_string()),
            failure: Some(VISIBILITY_DENIED_FAILURE.to_string()),
            ..EffectEntry::new(EffectKind::ObserverPublish, source_span.clone())
        });
        evaluations.push(CheckedEvaluation {
            kind: CheckedEvaluationKind::OwnerRmw,
            name: assignment.event().to_string(),
            result_name: None,
            actor_authority_origin: assignment.actor().to_string(),
            authority_origin_locus: assignment.role_locus().to_string(),
            owner_evaluation_locus: assignment.owner_locus().to_string(),
            declared_failure_row: FailureRow::new(failure_row.failures().iter().cloned()),
            generated_failure_row: FailureRow::new(owner_rmw_failure_names(observer_safe)),
            owner_rmw_core: Some(owner_core),
            relation_core: None,
            designated_core: None,
            evaluation_axes: EvaluationAxes::new(
                SemanticForm::State,
                EvaluationSite::Owner(Locus::new(assignment.owner_locus())),
                TriggerClock::OnEvent,
                AuthorityOrigin::Caller(Principal::new(assignment.actor())),
                Materialization::Store,
            ),
            effect_row: EffectRow {
                entries: [
                    Some(request),
                    Some(local_read),
                    Some(owner_write),
                    observer_publish,
                ]
                .into_iter()
                .flatten()
                .collect(),
            },
            generated_obligations: GeneratedObligations::owner_rmw(
                source_span.clone(),
                observer_safe,
            ),
        });
        for (kind, suffix) in [
            (SourceToCoreKind::OwnerRmw, "owner-rmw"),
            (SourceToCoreKind::OwnerLocalRead, "owner-local-read"),
            (SourceToCoreKind::OwnerLocalWrite, "owner-local-write"),
        ] {
            source_map.add(
                source_span.clone(),
                kind,
                format!(
                    "{}:{suffix}:principal={}:caller={}:owner={}",
                    assignment.event(),
                    assignment.actor(),
                    assignment.role_locus(),
                    assignment.owner_locus(),
                ),
            );
        }
        if observer_safe {
            source_map.add(
                source_span.clone(),
                SourceToCoreKind::ObserverPublish,
                format!(
                    "{}:observer-publish:principal={}:caller={}:owner={}",
                    assignment.event(),
                    assignment.actor(),
                    assignment.role_locus(),
                    assignment.owner_locus(),
                ),
            );
        }
    }

    let mut residual_entries = Vec::new();
    for relation in ast.relations() {
        let span = PipelineSourceSpan::from_surface(
            consumed_m6_classification
                .relation_template(relation.name())
                .expect("accepted M6 classification retains every relation template")
                .source_span(),
        );
        let binding_frontier =
            BindingActivationFrontier::from_ordered_occurrences(vec![OccurrenceId::new(
                relation.binding_frontier(),
            )])
            .expect("one relation binding frontier is finite and nonempty");
        let relation_core = RelationCheckedCore {
            owner_locus: relation.owner_locus().to_string(),
            subject: relation.subject().to_string(),
            subject_type: relation.subject_type().to_string(),
            primary: RelationAnchorCore::from_surface(relation.primary()),
            fallback: RelationAnchorCore::from_surface(relation.fallback()),
            binding_frontier,
            consumer_projection_locus: relation.consumer_projection_locus().map(str::to_string),
        };
        evaluations.push(CheckedEvaluation {
            kind: CheckedEvaluationKind::PublishRelation,
            name: relation.name().to_string(),
            result_name: None,
            actor_authority_origin: String::new(),
            authority_origin_locus: String::new(),
            owner_evaluation_locus: relation.owner_locus().to_string(),
            declared_failure_row: FailureRow::new([] as [String; 0]),
            generated_failure_row: FailureRow::new([] as [String; 0]),
            owner_rmw_core: None,
            relation_core: Some(relation_core),
            designated_core: None,
            evaluation_axes: EvaluationAxes::new(
                SemanticForm::Relation,
                EvaluationSite::Owner(Locus::new(relation.owner_locus())),
                TriggerClock::FrontierAdvance,
                AuthorityOrigin::OwnerTransition(Locus::new(relation.owner_locus())),
                Materialization::PublishRelation,
            ),
            effect_row: EffectRow {
                entries: vec![EffectEntry {
                    relation: Some(relation.name().to_string()),
                    ..EffectEntry::new(EffectKind::RelationPublish, span.clone())
                }],
            },
            generated_obligations: GeneratedObligations::relation(span.clone()),
        });
        source_map.add(
            span.clone(),
            SourceToCoreKind::PublishRelation,
            format!("{}:publish-relation", relation.name()),
        );
        if relation.consumer_projection_locus().is_some() {
            source_map.add(
                span.clone(),
                SourceToCoreKind::ConsumerLocalProjection,
                format!("{}:consumer-local-projection", relation.name()),
            );
        }
        for kind in [
            ResidualObligationKind::Visibility,
            ResidualObligationKind::RelationLifetime,
            ResidualObligationKind::FallbackValidity,
        ] {
            residual_entries.push(ResidualObligation::new(
                kind,
                relation.name().to_string(),
                span.clone(),
                None,
            ));
        }
    }

    for designated in ast.designated_results() {
        let span = PipelineSourceSpan::from_surface(
            consumed_m6_classification
                .designated_template(designated.evaluator(), designated.result())
                .expect("accepted M6 classification retains every designated template")
                .source_span(),
        );
        let expression = TypedExpression::from_surface(&ast, designated.expression());
        let designated_input_count = expression.state_reads().len();
        let evaluator = designated.evaluator().to_string();
        let evaluator_locus = Locus::new(&evaluator);
        let requester_site = EvaluationSite::DesignatedEvaluator(evaluator_locus.clone());
        let authority_origin = AuthorityOrigin::AdmittedEvaluator(evaluator_locus);
        let generated_remote_input_dependencies = expression
            .state_reads()
            .iter()
            .cloned()
            .map(|typed_state_read| {
                let source_owner_locus = typed_state_read.owner_locus().to_string();
                DesignatedRemoteInputDependency {
                    designated_evaluator: evaluator.clone(),
                    requester_site: requester_site.clone(),
                    authority_origin: authority_origin.clone(),
                    source_owner_locus: source_owner_locus.clone(),
                    request: DesignatedInputRequest {
                        source_owner_locus: source_owner_locus.clone(),
                        typed_state_read: typed_state_read.clone(),
                    },
                    receipt_use: DesignatedInputReceiptUse {
                        source_owner_locus,
                        typed_state_read: typed_state_read.clone(),
                    },
                    typed_state_read,
                }
            })
            .collect();
        let result_frontier =
            ResultFrontier::from_ordered_results(vec![ResultKey::new(designated.tick_frontier())])
                .expect("one designated tick frontier is finite and nonempty");
        let input_frontier = InputFrontier::from_ordered_producers(vec![M3OccurrenceId::new(
            designated.tick_frontier(),
        )])
        .expect("one designated input frontier is finite and nonempty");
        let evaluation_policy = EvaluationPolicy::declared_deterministic(format!(
            "inferred:{evaluator}.{}",
            designated.result()
        ));
        let observation_policy = ObservationPolicy::declared("conservative");
        let policy_stamp = evaluation_policy.stamp_with(&observation_policy);
        let designated_core = DesignatedCheckedCore {
            evaluator: evaluator.clone(),
            result: designated.result().to_string(),
            trigger: DesignatedTriggerCore {
                frontier: designated.tick_frontier().to_string(),
            },
            result_frontier,
            input_frontier,
            result_version: ResultVersion::new(1),
            evaluation_policy,
            observation_policy,
            policy_stamp,
            materialization: DesignatedMaterializationCore,
            expression,
            generated_remote_input_dependencies,
        };
        evaluations.push(CheckedEvaluation {
            kind: CheckedEvaluationKind::DesignatedPublishValue,
            name: evaluator.clone(),
            result_name: Some(designated.result().to_string()),
            actor_authority_origin: String::new(),
            authority_origin_locus: String::new(),
            owner_evaluation_locus: String::new(),
            declared_failure_row: FailureRow::new([] as [String; 0]),
            generated_failure_row: FailureRow::new([] as [String; 0]),
            owner_rmw_core: None,
            relation_core: None,
            designated_core: Some(designated_core),
            evaluation_axes: EvaluationAxes::new(
                SemanticForm::Value,
                requester_site,
                TriggerClock::LogicalTick,
                authority_origin,
                Materialization::PublishValue,
            ),
            effect_row: EffectRow {
                entries: designated_effect_entries(
                    span.clone(),
                    &evaluator,
                    designated.result(),
                    designated_input_count,
                ),
            },
            generated_obligations: GeneratedObligations::designated(span.clone()),
        });
        source_map.add(
            span.clone(),
            SourceToCoreKind::DesignatedDecision,
            format!(
                "{evaluator}:{}:designated-publish-value",
                designated.result()
            ),
        );
        residual_entries.push(ResidualObligation::new(
            ResidualObligationKind::ValueVisibilityRedaction,
            format!("{evaluator}.{}", designated.result()),
            span,
            None,
        ));
    }

    for form in ast.deferred_forms().entries() {
        let (residual_kind, core_ref, template_kind, required_authority) = match form.kind() {
            DeferredFormKind::WithAuth => (
                ResidualObligationKind::AuthDeferred,
                "deferred-auth",
                CoreTemplateKind::DeferredWithAuth,
                Some(form.name().to_string()),
            ),
            DeferredFormKind::Verify => (
                ResidualObligationKind::VerifyDeferred,
                "deferred-verify",
                CoreTemplateKind::DeferredVerify,
                None,
            ),
        };
        let span = PipelineSourceSpan::from_surface(
            consumed_m6_classification
                .deferred_template(template_kind, form.name())
                .expect("accepted M6 classification retains every deferred template")
                .source_span(),
        );
        residual_entries.push(ResidualObligation::new(
            residual_kind,
            form.name().to_string(),
            span.clone(),
            required_authority,
        ));
        source_map.add(
            span,
            SourceToCoreKind::DeferredPolicy,
            format!("{}:{core_ref}", form.name()),
        );
    }

    source_map.finalize();
    let root_span = PipelineSourceSpan::from_surface(ast.root().span());
    let static_environment = checked_static_environment(&ast, &evaluations);
    let program_identity =
        CheckedProgramIdentity::new(ast.module().name(), source.file(), root_span.source_ref())
            .with_structural_entries(checked_identity_structure(
                &static_environment,
                &evaluations,
                &source_map,
            ));
    CheckedSurfaceV0 {
        source_file: source.file().to_string(),
        root_span,
        program_identity,
        static_environment,
        evaluations,
        source_map,
        residual_obligations: ResidualObligations {
            entries: residual_entries,
        },
        consumed_m6_classification,
    }
}

fn m7_static_diagnostic(
    source: &FixtureSource,
    ast: &SurfaceV0File,
) -> Option<SurfaceV0PipelineDiagnostics> {
    duplicate_diagnostic(source, ast)
        .or_else(|| declaration_consistency_diagnostic(ast))
        .or_else(|| generated_failure_diagnostic(source, ast))
        .or_else(|| expression_diagnostic(source, ast))
}

fn duplicate_diagnostic(
    source: &FixtureSource,
    ast: &SurfaceV0File,
) -> Option<SurfaceV0PipelineDiagnostics> {
    let mut names = BTreeMap::new();
    for locus in ast.loci() {
        if names.insert(locus.name(), ()).is_some() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateDeclaration,
                PipelineSourceSpan::from_surface(locus.span()),
            ));
        }
    }
    names.clear();
    for principal in ast.principals() {
        if names.insert(principal.name(), ()).is_some() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateDeclaration,
                PipelineSourceSpan::from_surface(principal.span()),
            ));
        }
    }
    names.clear();
    for type_declaration in ast.types() {
        if names.insert(type_declaration.name(), ()).is_some() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateDeclaration,
                PipelineSourceSpan::from_surface(type_declaration.span()),
            ));
        }
    }
    names.clear();
    for state in ast.states() {
        if names.insert(state.name(), ()).is_some() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateDeclaration,
                PipelineSourceSpan::from_surface(state.span()),
            ));
        }
        let mut fields = BTreeMap::new();
        for field in state.fields() {
            if fields.insert(field.name(), ()).is_some() {
                return Some(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::DuplicateStateField,
                    PipelineSourceSpan::from_surface(field.span()),
                ));
            }
        }
    }

    names.clear();
    for role in ast.roles() {
        for event in role.whens() {
            if names.insert(event.event(), ()).is_some() {
                return Some(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::DuplicateEvent,
                    PipelineSourceSpan::from_surface(event.span()),
                ));
            }
        }
    }
    names.clear();
    for relation in ast.relations() {
        if names.insert(relation.name(), ()).is_some() {
            let header = format!("relation {} at {}", relation.name(), relation.owner_locus());
            let span = find_lexeme_in_span(source, relation.span(), &header)
                .unwrap_or_else(|| PipelineSourceSpan::from_surface(relation.span()));
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateRelation,
                span,
            ));
        }
    }
    let mut designated_names = BTreeMap::new();
    for designated in ast.designated_results() {
        let key = (designated.evaluator(), designated.result());
        if designated_names.insert(key, ()).is_some() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateDesignated,
                PipelineSourceSpan::from_surface(designated.span()),
            ));
        }
    }
    let mut deferred_names = BTreeMap::new();
    for form in ast.deferred_forms().entries() {
        let key = (form.kind(), form.name());
        if deferred_names.insert(key, ()).is_some() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::DuplicateDeferred,
                PipelineSourceSpan::from_surface(form.span()),
            ));
        }
    }
    None
}

fn declaration_consistency_diagnostic(ast: &SurfaceV0File) -> Option<SurfaceV0PipelineDiagnostics> {
    let known_types =
        |name: &str| name == "Int" || ast.types().iter().any(|item| item.name() == name);
    let known_locus = |name: &str| ast.locus(name).is_some();

    for state in ast.states() {
        if !known_types(state.index_type()) {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedStateIndexType,
                PipelineSourceSpan::from_surface(state.index_type_span()),
            ));
        }
        if !known_locus(state.owner_locus()) {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedOwnerLocus,
                PipelineSourceSpan::from_surface(state.owner_locus_span()),
            ));
        }
        if let Some(visibility) = state.visibility() {
            if visibility.channel() != OBSERVER_SAFE_CHANNEL {
                return Some(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::UnexpectedSyntax,
                    PipelineSourceSpan::from_surface(visibility.span()),
                ));
            }
            let mut visible_fields = BTreeMap::new();
            for declared in visibility.fields() {
                if visible_fields.insert(declared.name(), ()).is_some() {
                    return Some(SurfaceV0PipelineDiagnostics::one(
                        M7DiagnosticKind::DuplicateStateField,
                        PipelineSourceSpan::from_surface(declared.span()),
                    ));
                }
                if state.field(declared.name()).is_none() {
                    return Some(SurfaceV0PipelineDiagnostics::one(
                        M7DiagnosticKind::UnknownStateField,
                        PipelineSourceSpan::from_surface(declared.span()),
                    ));
                }
            }
        }
        for field in state.fields() {
            if !known_types(field.type_name()) {
                return Some(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::UndefinedStateFieldType,
                    PipelineSourceSpan::from_surface(field.type_span()),
                ));
            }
        }
    }
    for relation in ast.relations() {
        if !known_locus(relation.owner_locus()) {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedOwnerLocus,
                PipelineSourceSpan::from_surface(relation.owner_locus_span()),
            ));
        }
        if !known_types(relation.subject_type()) {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedRelationSubjectType,
                PipelineSourceSpan::from_surface(relation.subject_type_span()),
            ));
        }
        if let Some(locus) = relation.consumer_projection_locus()
            && !known_locus(locus)
        {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedConsumerLocus,
                PipelineSourceSpan::from_surface(
                    relation
                        .consumer_projection_locus_span()
                        .expect("projection locus has a span"),
                ),
            ));
        }
    }
    for role in ast.roles() {
        if ast.principal(role.actor()).is_none() {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedSelfPrincipal,
                PipelineSourceSpan::from_surface(role.actor_span()),
            ));
        }
        if !known_locus(role.evaluation_locus()) {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::UndefinedRoleEvaluationLocus,
                PipelineSourceSpan::from_surface(role.evaluation_locus_span()),
            ));
        }
    }
    None
}

fn generated_failure_diagnostic(
    source: &FixtureSource,
    ast: &SurfaceV0File,
) -> Option<SurfaceV0PipelineDiagnostics> {
    for assignment in ast.assignments() {
        let when = ast
            .when(assignment.event())
            .expect("accepted assignment has a declared event");
        let declared = FailureRow::new(when.failures().iter().cloned());
        let observer_safe =
            observer_visibility_channel(ast, assignment.target()) == Some(OBSERVER_SAFE_CHANNEL);
        let generated = FailureRow::new(owner_rmw_failure_names(observer_safe));
        if !generated.is_subset_of(&declared) {
            let Some(missing) = generated
                .names()
                .into_iter()
                .find(|name| !declared.names().iter().any(|candidate| candidate == name))
            else {
                continue;
            };
            if missing == VISIBILITY_DENIED_FAILURE {
                return Some(SurfaceV0PipelineDiagnostics::missing_generated_failure(
                    PipelineSourceSpan::from_surface(assignment.span()),
                    missing,
                ));
            }
            let lexeme = format!("fails ({})", when.failures().join(", "));
            let span = find_lexeme_in_span(source, when.span(), &lexeme)
                .unwrap_or_else(|| PipelineSourceSpan::from_surface(when.span()));
            return Some(SurfaceV0PipelineDiagnostics::missing_generated_failure(
                span, missing,
            ));
        }
    }
    None
}

fn expression_diagnostic(
    source: &FixtureSource,
    ast: &SurfaceV0File,
) -> Option<SurfaceV0PipelineDiagnostics> {
    for assignment in ast.assignments() {
        if assignment.target().field().is_none() {
            continue;
        }
        let target_type = match state_reference_type(source, ast, assignment.target()) {
            Ok(value) => value,
            Err(diagnostic) => return Some(diagnostic),
        };
        let parameters = ast
            .when(assignment.event())
            .expect("accepted assignment is nested under its declared event")
            .parameters();
        let expression_type =
            match bounded_expression_type(source, ast, assignment.expression(), parameters) {
                Ok(value) => value,
                Err(diagnostic) => return Some(diagnostic),
            };
        if let Some(expression_type) = expression_type
            && expression_type.0 != target_type.0
        {
            return Some(SurfaceV0PipelineDiagnostics::one(
                M7DiagnosticKind::TypeMismatch,
                expression_type.1,
            ));
        }
    }
    for designated in ast.designated_results() {
        if let Err(diagnostic) = bounded_expression_type(source, ast, designated.expression(), &[])
        {
            return Some(diagnostic);
        }
    }
    None
}

fn state_reference_type(
    source: &FixtureSource,
    ast: &SurfaceV0File,
    reference: &SurfaceReference,
) -> Result<(String, PipelineSourceSpan), SurfaceV0PipelineDiagnostics> {
    let Some(state) = ast.state(reference.base()) else {
        return Err(SurfaceV0PipelineDiagnostics::one(
            M7DiagnosticKind::UnresolvedName,
            PipelineSourceSpan::from_surface(reference.span()),
        ));
    };
    let Some(field) = reference.field() else {
        return Err(SurfaceV0PipelineDiagnostics::one(
            M7DiagnosticKind::UnresolvedName,
            PipelineSourceSpan::from_surface(reference.span()),
        ));
    };
    let Some(candidate) = state
        .fields()
        .iter()
        .find(|candidate| candidate.name() == field)
    else {
        let range = reference.span().byte_range();
        return Err(SurfaceV0PipelineDiagnostics::one(
            M7DiagnosticKind::UnknownStateField,
            PipelineSourceSpan::from_source_range(source, range.end - field.len(), range.end),
        ));
    };
    Ok((
        candidate.type_name().to_string(),
        PipelineSourceSpan::from_surface(reference.span()),
    ))
}

fn bounded_expression_type(
    source: &FixtureSource,
    ast: &SurfaceV0File,
    expression: &BoundedExpression,
    parameters: &[Parameter],
) -> Result<Option<(String, PipelineSourceSpan)>, SurfaceV0PipelineDiagnostics> {
    finite_expression_tree_type(source, ast, expression.tree(), parameters).map(Some)
}

fn finite_expression_tree_type(
    source: &FixtureSource,
    ast: &SurfaceV0File,
    tree: &BoundedExpressionTree,
    parameters: &[Parameter],
) -> Result<(String, PipelineSourceSpan), SurfaceV0PipelineDiagnostics> {
    match tree {
        BoundedExpressionTree::StateReference(reference) => {
            state_reference_type(source, ast, reference)
        }
        BoundedExpressionTree::Identifier { name, span } => {
            let Some(parameter) = parameters.iter().find(|parameter| parameter.name() == name)
            else {
                return Err(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::UnresolvedName,
                    PipelineSourceSpan::from_surface(span),
                ));
            };
            Ok((
                parameter.type_name().to_string(),
                PipelineSourceSpan::from_surface(span),
            ))
        }
        BoundedExpressionTree::IntegerLiteral(literal) => Ok((
            "Int".to_string(),
            PipelineSourceSpan::from_surface(literal.span()),
        )),
        BoundedExpressionTree::Opaque { span } => Err(SurfaceV0PipelineDiagnostics::one(
            M7DiagnosticKind::UnsupportedExpression,
            PipelineSourceSpan::from_surface(span),
        )),
        BoundedExpressionTree::Binary {
            span, left, right, ..
        } => {
            let left_type = finite_expression_tree_type(source, ast, left, parameters)?;
            let right_type = finite_expression_tree_type(source, ast, right, parameters)?;
            if left_type.0 != right_type.0 {
                return Err(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::TypeMismatch,
                    right_type.1,
                ));
            }
            if left_type.0 != "Int" {
                let operator_span = match tree {
                    BoundedExpressionTree::Binary {
                        operator: _,
                        span,
                        left: _,
                        right: _,
                    } => {
                        let source_range = span.byte_range();
                        let operator_start = left.span().byte_range().end;
                        let operator_end = right.span().byte_range().start;
                        source.text()[operator_start..operator_end]
                            .find(['+', '-'])
                            .map(|offset| {
                                PipelineSourceSpan::from_source_range(
                                    source,
                                    operator_start + offset,
                                    operator_start + offset + 1,
                                )
                            })
                            .unwrap_or_else(|| {
                                PipelineSourceSpan::from_source_range(
                                    source,
                                    source_range.start,
                                    source_range.end,
                                )
                            })
                    }
                    _ => unreachable!("matched binary expression tree"),
                };
                return Err(SurfaceV0PipelineDiagnostics::one(
                    M7DiagnosticKind::ArithmeticRequiresInt,
                    operator_span,
                ));
            }
            Ok(("Int".to_string(), PipelineSourceSpan::from_surface(span)))
        }
    }
}

fn forward_parse_diagnostic(
    diagnostics: mir_ast::surface_v0::ParseDiagnostics,
) -> SurfaceV0PipelineDiagnostics {
    let diagnostic = diagnostics.primary();
    SurfaceV0PipelineDiagnostics::one(
        match diagnostic.kind() {
            ParseErrorKind::RoleActorMustBeLiteralSelf => {
                M7DiagnosticKind::RoleActorMustBeLiteralSelf
            }
            ParseErrorKind::IntegerLiteralOutOfRange => M7DiagnosticKind::UnexpectedSyntax,
            ParseErrorKind::UnsupportedTransportSyntax => {
                M7DiagnosticKind::UnsupportedTransportSyntax
            }
            ParseErrorKind::UnsupportedOccurrenceSyntax => {
                M7DiagnosticKind::UnsupportedOccurrenceSyntax
            }
            ParseErrorKind::UnsupportedEnvelopeSyntax => {
                M7DiagnosticKind::UnsupportedEnvelopeSyntax
            }
            ParseErrorKind::UnexpectedSyntax => M7DiagnosticKind::UnexpectedSyntax,
        },
        PipelineSourceSpan::from_surface(diagnostic.span()),
    )
}

fn forward_classification_diagnostic(
    diagnostics: crate::surface_v0_classification::SurfaceV0Diagnostics,
) -> SurfaceV0PipelineDiagnostics {
    let diagnostic = diagnostics.primary();
    SurfaceV0PipelineDiagnostics::one(
        match diagnostic.kind() {
            SurfaceV0DiagnosticKind::RoleActorMustBeLiteralSelf => {
                M7DiagnosticKind::RoleActorMustBeLiteralSelf
            }
            SurfaceV0DiagnosticKind::OwnerActionLocusMismatch => {
                M7DiagnosticKind::OwnerActionLocusMismatch
            }
            SurfaceV0DiagnosticKind::CrossOwnerWriteTargetOutsideActionLocus => {
                M7DiagnosticKind::CrossOwnerWriteTargetOutsideActionLocus
            }
            SurfaceV0DiagnosticKind::FieldlessAssignmentTarget => {
                M7DiagnosticKind::FieldlessAssignmentTarget
            }
            SurfaceV0DiagnosticKind::CrossOwnerOperandRequiresReceipt => {
                M7DiagnosticKind::CrossOwnerOperandRequiresReceipt
            }
            SurfaceV0DiagnosticKind::RelationMustPublishRelationCarrier => {
                M7DiagnosticKind::RelationMustPublishRelationCarrier
            }
            SurfaceV0DiagnosticKind::ConsumerRelationMutationDenied => {
                M7DiagnosticKind::ConsumerRelationMutationDenied
            }
            SurfaceV0DiagnosticKind::UnresolvedName => M7DiagnosticKind::UnresolvedName,
            SurfaceV0DiagnosticKind::AmbiguousName => M7DiagnosticKind::AmbiguousName,
            SurfaceV0DiagnosticKind::UnsupportedTransportSyntax => {
                M7DiagnosticKind::UnsupportedTransportSyntax
            }
            SurfaceV0DiagnosticKind::UnsupportedOccurrenceSyntax => {
                M7DiagnosticKind::UnsupportedOccurrenceSyntax
            }
            SurfaceV0DiagnosticKind::UnsupportedEnvelopeSyntax => {
                M7DiagnosticKind::UnsupportedEnvelopeSyntax
            }
            SurfaceV0DiagnosticKind::UnexpectedSyntax => M7DiagnosticKind::UnexpectedSyntax,
        },
        PipelineSourceSpan::from_surface(diagnostic.span()),
    )
}

fn find_lexeme_in_span(
    source: &FixtureSource,
    container: &SurfaceV0Span,
    lexeme: &str,
) -> Option<PipelineSourceSpan> {
    let range = container.byte_range();
    let start = source.text()[range.clone()].find(lexeme)? + range.start;
    Some(PipelineSourceSpan::from_source_range(
        source,
        start,
        start + lexeme.len(),
    ))
}

fn line_column(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 1_u32;
    let mut column = 1_u32;
    for byte in source[..byte_offset].bytes() {
        if byte == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}
