//! M6's bounded ordinary Surface v0 grammar.
//!
//! This parser deliberately owns only syntax and canonical source spans.  It
//! does not import semantic crates or assign runtime behaviour; the M6
//! classification boundary performs the one-way conversion into M5 evidence.

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureSource {
    file: String,
    text: String,
}

impl FixtureSource {
    pub fn new(file: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            text: text.into(),
        }
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    fn span(&self, byte_start: usize, byte_end: usize) -> SurfaceV0Span {
        let (start_line, start_column) = line_column(&self.text, byte_start);
        let (end_line, end_column) = line_column(&self.text, byte_end);
        SurfaceV0Span {
            file: self.file.clone(),
            byte_start,
            byte_end,
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }
}

/// Canonical, file-qualified byte span.  This remains parser-owned and is
/// intentionally independent from the M5 semantic source-location carrier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceV0Span {
    file: String,
    byte_start: usize,
    byte_end: usize,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

impl SurfaceV0Span {
    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn byte_range(&self) -> Range<usize> {
        self.byte_start..self.byte_end
    }

    pub const fn start_line_column(&self) -> (u32, u32) {
        (self.start_line, self.start_column)
    }

    pub const fn end_line_column(&self) -> (u32, u32) {
        (self.end_line, self.end_column)
    }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.byte_start..self.byte_end]
    }

    pub fn is_child_of(&self, parent: &Self) -> bool {
        self.file == parent.file
            && parent.byte_start <= self.byte_start
            && self.byte_end <= parent.byte_end
    }

    pub fn source_ref_data(&self) -> SourceRefData {
        SourceRefData {
            path: self.file.clone(),
            start_line: self.start_line,
            start_column: self.start_column,
            end_line: self.end_line,
            end_column: self.end_column,
        }
    }
}

/// Structural data used by the semantic lowering boundary to construct M5's
/// `SourceRef` without making this AST crate depend on `mir-semantics`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceRefData {
    path: String,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

impl SourceRefData {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn line_columns(&self) -> (u32, u32, u32, u32) {
        (
            self.start_line,
            self.start_column,
            self.end_line,
            self.end_column,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyntaxKind {
    Module,
    Locus,
    Principal,
    Type,
    State,
    RoleInstance,
    When,
    Assignment,
    Relation,
    DesignatedResult,
    DesignatedResultConsumer,
    WithAuth,
    Verify,
    RelationMutation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode {
    kind: SyntaxKind,
    label: String,
    span: SurfaceV0Span,
    children: Vec<SyntaxNode>,
}

impl SyntaxNode {
    fn new(
        kind: SyntaxKind,
        label: impl Into<String>,
        span: SurfaceV0Span,
        children: Vec<SyntaxNode>,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            span,
            children,
        }
    }

    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }

    pub fn children(&self) -> &[SyntaxNode] {
        &self.children
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeferredFormKind {
    WithAuth,
    Verify,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeferredForms {
    entries: Vec<DeferredForm>,
}

impl DeferredForms {
    pub fn contains(&self, kind: DeferredFormKind, name: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == kind && entry.name == name)
    }

    pub fn entries(&self) -> &[DeferredForm] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredForm {
    kind: DeferredFormKind,
    name: String,
    node: SyntaxNode,
}

impl DeferredForm {
    pub const fn kind(&self) -> DeferredFormKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseErrorKind {
    RoleActorMustBeLiteralSelf,
    IntegerLiteralOutOfRange,
    UnsupportedTransportSyntax,
    UnsupportedOccurrenceSyntax,
    UnsupportedEnvelopeSyntax,
    UnexpectedSyntax,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    kind: ParseErrorKind,
    span: SurfaceV0Span,
}

impl ParseError {
    pub const fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDiagnostics {
    diagnostics: Vec<ParseError>,
}

impl ParseDiagnostics {
    fn one(kind: ParseErrorKind, span: SurfaceV0Span) -> Self {
        Self {
            diagnostics: vec![ParseError { kind, span }],
        }
    }

    pub fn primary(&self) -> &ParseError {
        self.diagnostics
            .first()
            .expect("a ParseDiagnostics value always has a primary diagnostic")
    }

    pub fn entries(&self) -> &[ParseError] {
        &self.diagnostics
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0File {
    root: SyntaxNode,
    module: ModuleDecl,
    loci: Vec<LocusDecl>,
    principals: Vec<PrincipalDecl>,
    types: Vec<TypeDecl>,
    states: Vec<StateDecl>,
    roles: Vec<RoleInstance>,
    assignments: Vec<Assignment>,
    relations: Vec<MaintainedRelation>,
    designated_results: Vec<DesignatedResultDecl>,
    designated_result_consumers: Vec<DesignatedResultConsumerDecl>,
    deferred_forms: DeferredForms,
    relation_mutations: Vec<RelationMutation>,
}

impl SurfaceV0File {
    pub fn root(&self) -> &SyntaxNode {
        &self.root
    }

    pub fn module(&self) -> &ModuleDecl {
        &self.module
    }

    pub fn locus(&self, name: &str) -> Option<&LocusDecl> {
        self.loci.iter().find(|decl| decl.name == name)
    }

    pub fn loci(&self) -> &[LocusDecl] {
        &self.loci
    }

    pub fn principal(&self, name: &str) -> Option<&PrincipalDecl> {
        self.principals.iter().find(|decl| decl.name == name)
    }

    pub fn principals(&self) -> &[PrincipalDecl] {
        &self.principals
    }

    pub fn types(&self) -> &[TypeDecl] {
        &self.types
    }

    pub fn state(&self, name: &str) -> Option<&StateDecl> {
        self.states.iter().find(|decl| decl.name == name)
    }

    pub fn states(&self) -> &[StateDecl] {
        &self.states
    }

    pub fn roles(&self) -> &[RoleInstance] {
        &self.roles
    }

    pub fn when(&self, event: &str) -> Option<&WhenDecl> {
        self.roles
            .iter()
            .flat_map(|role| role.whens.iter())
            .find(|when| when.event == event)
    }

    pub fn assignment(&self, target: &str) -> Option<&Assignment> {
        self.assignments
            .iter()
            .find(|assignment| assignment.target.text == target)
    }

    pub fn assignments(&self) -> &[Assignment] {
        &self.assignments
    }

    pub fn relation(&self, name: &str) -> Option<&MaintainedRelation> {
        self.relations.iter().find(|relation| relation.name == name)
    }

    pub fn relations(&self) -> &[MaintainedRelation] {
        &self.relations
    }

    pub fn designated_result(
        &self,
        evaluator: &str,
        result: &str,
    ) -> Option<&DesignatedResultDecl> {
        self.designated_results
            .iter()
            .find(|decl| decl.evaluator == evaluator && decl.result == result)
    }

    pub fn designated_results(&self) -> &[DesignatedResultDecl] {
        &self.designated_results
    }

    pub fn designated_result_consumer(
        &self,
        evaluator: &str,
        result: &str,
        consumer_locus: &str,
    ) -> Option<&DesignatedResultConsumerDecl> {
        self.designated_result_consumers.iter().find(|decl| {
            decl.evaluator == evaluator
                && decl.result == result
                && decl.consumer_locus == consumer_locus
        })
    }

    pub fn designated_result_consumers(&self) -> &[DesignatedResultConsumerDecl] {
        &self.designated_result_consumers
    }

    pub fn deferred_forms(&self) -> &DeferredForms {
        &self.deferred_forms
    }

    pub fn relation_mutations(&self) -> &[RelationMutation] {
        &self.relation_mutations
    }

    pub fn find_node(&self, kind: SyntaxKind, label: &str) -> Option<&SyntaxNode> {
        if self.root.kind == kind && self.root.label == label {
            return Some(&self.root);
        }
        self.roles
            .iter()
            .map(|role| &role.node)
            .chain(self.relations.iter().map(|relation| &relation.node))
            .chain(
                self.designated_results
                    .iter()
                    .map(|designated| &designated.node),
            )
            .chain(
                self.designated_result_consumers
                    .iter()
                    .map(|consumer| &consumer.node),
            )
            .find(|node| node.kind == kind && node.label == label)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDecl {
    name: String,
    node: SyntaxNode,
}

impl ModuleDecl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocusDecl {
    name: String,
    name_span: SurfaceV0Span,
    node: SyntaxNode,
}

impl LocusDecl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }

    pub fn name_span(&self) -> &SurfaceV0Span {
        &self.name_span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalDecl {
    name: String,
    node: SyntaxNode,
}

impl PrincipalDecl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDecl {
    name: String,
    name_span: SurfaceV0Span,
    node: SyntaxNode,
}

impl TypeDecl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }

    pub fn name_span(&self) -> &SurfaceV0Span {
        &self.name_span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateDecl {
    name: String,
    index_name: String,
    index_type: String,
    index_type_span: SurfaceV0Span,
    owner_locus: String,
    owner_locus_span: SurfaceV0Span,
    fields: Vec<StateField>,
    visibility: Option<StateVisibility>,
    node: SyntaxNode,
}

impl StateDecl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    pub fn index_type(&self) -> &str {
        &self.index_type
    }

    pub fn index_type_span(&self) -> &SurfaceV0Span {
        &self.index_type_span
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn owner_locus_span(&self) -> &SurfaceV0Span {
        &self.owner_locus_span
    }

    pub fn fields(&self) -> &[StateField] {
        &self.fields
    }

    /// The optional source-declared observer visibility for this state block.
    /// Fields absent from this declaration deliberately remain unspecified at
    /// the syntax layer; M7 gives them their private-by-default meaning.
    pub fn visibility(&self) -> Option<&StateVisibility> {
        self.visibility.as_ref()
    }

    pub fn field(&self, name: &str) -> Option<&StateField> {
        self.fields.iter().find(|field| field.name == name)
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateField {
    name: String,
    type_name: String,
    span: SurfaceV0Span,
    type_span: SurfaceV0Span,
    visibility: Option<StateFieldVisibility>,
}

impl StateField {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }

    pub fn type_span(&self) -> &SurfaceV0Span {
        &self.type_span
    }

    pub fn visibility(&self) -> Option<&StateFieldVisibility> {
        self.visibility.as_ref()
    }
}

/// A field name retained directly from a `visible … fields (…)` declaration.
/// It is separate from `StateField` so M7 can reject unknown and duplicate
/// names at their declaration spans without losing the original syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateVisibilityField {
    name: String,
    span: SurfaceV0Span,
}

impl StateVisibilityField {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

/// The bounded M6 state-block visibility declaration.  M6 retains the
/// channel and exact named fields; it does not grant observation authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateVisibility {
    channel: String,
    fields: Vec<StateVisibilityField>,
    span: SurfaceV0Span,
}

impl StateVisibility {
    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn fields(&self) -> &[StateVisibilityField] {
        &self.fields
    }

    pub fn field(&self, name: &str) -> Option<&StateVisibilityField> {
        self.fields.iter().find(|field| field.name == name)
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

/// State-field-local lookup convenience.  Its presence records only the
/// source declaration; authority and observer emission remain M7 concerns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateFieldVisibility {
    channel: String,
    declaration_span: SurfaceV0Span,
}

impl StateFieldVisibility {
    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn declaration_span(&self) -> &SurfaceV0Span {
        &self.declaration_span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleInstance {
    actor: String,
    actor_span: SurfaceV0Span,
    evaluation_locus: String,
    evaluation_locus_span: SurfaceV0Span,
    whens: Vec<WhenDecl>,
    node: SyntaxNode,
}

impl RoleInstance {
    pub fn actor(&self) -> &str {
        &self.actor
    }

    pub fn actor_span(&self) -> &SurfaceV0Span {
        &self.actor_span
    }

    pub fn evaluation_locus(&self) -> &str {
        &self.evaluation_locus
    }

    pub fn evaluation_locus_span(&self) -> &SurfaceV0Span {
        &self.evaluation_locus_span
    }

    pub fn whens(&self) -> &[WhenDecl] {
        &self.whens
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhenDecl {
    event: String,
    parameters: Vec<Parameter>,
    failures: Vec<String>,
    actor: String,
    role_locus: String,
    node: SyntaxNode,
}

impl WhenDecl {
    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn parameters(&self) -> &[Parameter] {
        &self.parameters
    }

    pub fn failures(&self) -> &[String] {
        &self.failures
    }

    pub fn actor(&self) -> &str {
        &self.actor
    }

    pub fn role_locus(&self) -> &str {
        &self.role_locus
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    name: String,
    type_name: String,
    span: SurfaceV0Span,
    type_span: SurfaceV0Span,
}

impl Parameter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }

    pub fn type_span(&self) -> &SurfaceV0Span {
        &self.type_span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceReference {
    text: String,
    base: String,
    index: Option<String>,
    field: Option<String>,
    span: SurfaceV0Span,
}

impl SurfaceReference {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn index(&self) -> Option<&str> {
        self.index.as_deref()
    }

    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

/// The deliberately bounded arithmetic operators retained by Surface v0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedBinaryOperator {
    Add,
    Subtract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedBinaryOperation {
    operator: BoundedBinaryOperator,
    span: SurfaceV0Span,
}

impl BoundedBinaryOperation {
    pub const fn operator(&self) -> BoundedBinaryOperator {
        self.operator
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedIntegerLiteral {
    value: i64,
    span: SurfaceV0Span,
}

impl BoundedIntegerLiteral {
    pub const fn value(&self) -> i64 {
        self.value
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedExpressionToken {
    span: SurfaceV0Span,
}

impl BoundedExpressionToken {
    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedExpressionTree {
    StateReference(SurfaceReference),
    Identifier {
        name: String,
        span: SurfaceV0Span,
    },
    IntegerLiteral(BoundedIntegerLiteral),
    Binary {
        operator: BoundedBinaryOperator,
        span: SurfaceV0Span,
        left: Box<Self>,
        right: Box<Self>,
    },
    Opaque {
        span: SurfaceV0Span,
    },
}

impl BoundedExpressionTree {
    pub fn source_lexeme<'a>(&self, source: &'a str) -> &'a str {
        self.span().lexeme(source)
    }

    pub fn span(&self) -> &SurfaceV0Span {
        match self {
            Self::StateReference(reference) => reference.span(),
            Self::IntegerLiteral(literal) => literal.span(),
            Self::Identifier { span, .. } | Self::Binary { span, .. } | Self::Opaque { span } => {
                span
            }
        }
    }

    pub const fn operator(&self) -> Option<BoundedBinaryOperator> {
        match self {
            Self::Binary { operator, .. } => Some(*operator),
            Self::StateReference(_)
            | Self::Identifier { .. }
            | Self::IntegerLiteral(_)
            | Self::Opaque { .. } => None,
        }
    }

    pub fn left(&self) -> &Self {
        match self {
            Self::Binary { left, .. } => left,
            Self::StateReference(_)
            | Self::Identifier { .. }
            | Self::IntegerLiteral(_)
            | Self::Opaque { .. } => {
                panic!("only bounded binary expression trees have a left child")
            }
        }
    }

    pub fn right(&self) -> &Self {
        match self {
            Self::Binary { right, .. } => right,
            Self::StateReference(_)
            | Self::Identifier { .. }
            | Self::IntegerLiteral(_)
            | Self::Opaque { .. } => {
                panic!("only bounded binary expression trees have a right child")
            }
        }
    }

    pub fn int_literal(&self) -> Option<&BoundedIntegerLiteral> {
        match self {
            Self::IntegerLiteral(literal) => Some(literal),
            Self::StateReference(_)
            | Self::Identifier { .. }
            | Self::Binary { .. }
            | Self::Opaque { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoundedExpressionPart {
    StateReference(SurfaceReference),
    Identifier { name: String, span: SurfaceV0Span },
    IntegerLiteral(BoundedIntegerLiteral),
    BinaryOperation(BoundedBinaryOperation),
    Opaque,
}

/// A parser-retained, finite expression surface for the M6/M7 path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedExpression {
    span: SurfaceV0Span,
    state_refs: Vec<SurfaceReference>,
    int_literals: Vec<BoundedIntegerLiteral>,
    binary_ops: Vec<BoundedBinaryOperation>,
    tokens: Vec<BoundedExpressionToken>,
    tree: BoundedExpressionTree,
}

impl BoundedExpression {
    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }

    pub fn state_refs(&self) -> &[SurfaceReference] {
        &self.state_refs
    }

    pub fn int_literals(&self) -> &[BoundedIntegerLiteral] {
        &self.int_literals
    }

    pub fn binary_ops(&self) -> &[BoundedBinaryOperation] {
        &self.binary_ops
    }

    pub fn tokens(&self) -> &[BoundedExpressionToken] {
        &self.tokens
    }

    pub fn tree(&self) -> &BoundedExpressionTree {
        &self.tree
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    target: SurfaceReference,
    rhs_references: Vec<SurfaceReference>,
    expression: BoundedExpression,
    role_locus: String,
    owner_locus: String,
    owner_locus_span: SurfaceV0Span,
    event: String,
    actor: String,
    expression_span: SurfaceV0Span,
    node: SyntaxNode,
}

impl Assignment {
    pub fn target(&self) -> &SurfaceReference {
        &self.target
    }

    pub fn rhs_references(&self) -> &[SurfaceReference] {
        &self.rhs_references
    }

    pub fn expression(&self) -> &BoundedExpression {
        &self.expression
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn role_locus(&self) -> &str {
        &self.role_locus
    }

    pub fn owner_locus_span(&self) -> &SurfaceV0Span {
        &self.owner_locus_span
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn actor(&self) -> &str {
        &self.actor
    }

    pub fn expression_span(&self) -> &SurfaceV0Span {
        &self.expression_span
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationTransform {
    Translate { x: i64, y: i64 },
    Identity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationAnchor {
    anchor: String,
    anchor_locus: Option<String>,
    anchor_locus_span: Option<SurfaceV0Span>,
    epoch: String,
    transform: RelationTransform,
    transform_span: SurfaceV0Span,
    span: SurfaceV0Span,
}

impl RelationAnchor {
    pub fn anchor(&self) -> &str {
        &self.anchor
    }

    /// Optional internal locus binding for this relation anchor.
    ///
    /// Legacy accepted source does not carry this field; the checker must not
    /// infer it from the relation owner or any transport concern.
    pub fn anchor_locus(&self) -> Option<&str> {
        self.anchor_locus.as_deref()
    }

    pub fn anchor_locus_span(&self) -> Option<&SurfaceV0Span> {
        self.anchor_locus_span.as_ref()
    }

    pub fn epoch(&self) -> &str {
        &self.epoch
    }

    pub fn transform(&self) -> &RelationTransform {
        &self.transform
    }

    pub fn transform_span(&self) -> &SurfaceV0Span {
        &self.transform_span
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationPublication {
    Relation { span: SurfaceV0Span },
    Value { value: String, span: SurfaceV0Span },
}

impl RelationPublication {
    pub fn span(&self) -> &SurfaceV0Span {
        match self {
            Self::Relation { span } | Self::Value { span, .. } => span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintainedRelation {
    name: String,
    owner_locus: String,
    owner_locus_span: SurfaceV0Span,
    subject: String,
    subject_type: String,
    subject_type_span: SurfaceV0Span,
    primary: RelationAnchor,
    fallback: RelationAnchor,
    binding_frontier: String,
    publication: RelationPublication,
    consumer_projection_locus: Option<String>,
    consumer_projection_locus_span: Option<SurfaceV0Span>,
    node: SyntaxNode,
}

impl MaintainedRelation {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn owner_locus(&self) -> &str {
        &self.owner_locus
    }

    pub fn owner_locus_span(&self) -> &SurfaceV0Span {
        &self.owner_locus_span
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn subject_type(&self) -> &str {
        &self.subject_type
    }

    pub fn subject_type_span(&self) -> &SurfaceV0Span {
        &self.subject_type_span
    }

    pub fn primary(&self) -> &RelationAnchor {
        &self.primary
    }

    pub fn fallback(&self) -> &RelationAnchor {
        &self.fallback
    }

    pub fn binding_frontier(&self) -> &str {
        &self.binding_frontier
    }

    pub fn publish_materialization(&self) -> &str {
        match &self.publication {
            RelationPublication::Relation { .. } => "publish-relation",
            RelationPublication::Value { .. } => "publish-value",
        }
    }

    pub fn publication(&self) -> &RelationPublication {
        &self.publication
    }

    pub fn consumer_projection_locus(&self) -> Option<&str> {
        self.consumer_projection_locus.as_deref()
    }

    pub fn consumer_projection_locus_span(&self) -> Option<&SurfaceV0Span> {
        self.consumer_projection_locus_span.as_ref()
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedResultDecl {
    evaluator: String,
    tick_frontier: String,
    result: String,
    expression: BoundedExpression,
    expression_span: SurfaceV0Span,
    node: SyntaxNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedResultConsumerDecl {
    evaluator: String,
    result: String,
    consumer_locus: String,
    result_ref_span: SurfaceV0Span,
    consumer_locus_span: SurfaceV0Span,
    node: SyntaxNode,
}

impl DesignatedResultConsumerDecl {
    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub fn consumer_locus(&self) -> &str {
        &self.consumer_locus
    }

    pub fn result_ref_span(&self) -> &SurfaceV0Span {
        &self.result_ref_span
    }

    pub fn consumer_locus_span(&self) -> &SurfaceV0Span {
        &self.consumer_locus_span
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

impl DesignatedResultDecl {
    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    pub fn tick_frontier(&self) -> &str {
        &self.tick_frontier
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub const fn materialization(&self) -> &str {
        "publish-value"
    }

    pub fn expression(&self) -> &BoundedExpression {
        &self.expression
    }

    pub fn expression_span(&self) -> &SurfaceV0Span {
        &self.expression_span
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationMutation {
    relation: String,
    locus: String,
    node: SyntaxNode,
}

impl RelationMutation {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn span(&self) -> &SurfaceV0Span {
        self.node.span()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    text: String,
    span: SurfaceV0Span,
}

struct Parser {
    source: FixtureSource,
    tokens: Vec<Token>,
    index: usize,
    loci: Vec<LocusDecl>,
    principals: Vec<PrincipalDecl>,
    types: Vec<TypeDecl>,
    states: Vec<StateDecl>,
    roles: Vec<RoleInstance>,
    assignments: Vec<Assignment>,
    relations: Vec<MaintainedRelation>,
    designated_results: Vec<DesignatedResultDecl>,
    designated_result_consumers: Vec<DesignatedResultConsumerDecl>,
    deferred_forms: DeferredForms,
    relation_mutations: Vec<RelationMutation>,
}

impl Parser {
    fn new(source: FixtureSource, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            index: 0,
            loci: Vec::new(),
            principals: Vec::new(),
            types: Vec::new(),
            states: Vec::new(),
            roles: Vec::new(),
            assignments: Vec::new(),
            relations: Vec::new(),
            designated_results: Vec::new(),
            designated_result_consumers: Vec::new(),
            deferred_forms: DeferredForms::default(),
            relation_mutations: Vec::new(),
        }
    }

    fn parse(mut self) -> Result<SurfaceV0File, ParseDiagnostics> {
        let mut module = None;
        while !self.is_eof() {
            match self.peek_text() {
                "module" => module = Some(self.parse_module()?),
                "locus" => {
                    let declaration = self.parse_locus()?;
                    self.loci.push(declaration);
                }
                "principal" => {
                    let declaration = self.parse_principal()?;
                    self.principals.push(declaration);
                }
                "type" => {
                    let declaration = self.parse_type()?;
                    self.types.push(declaration);
                }
                "state" => {
                    let declaration = self.parse_state()?;
                    self.states.push(declaration);
                }
                "Role" => self.parse_role()?,
                "relation" => self.parse_relation()?,
                "designated" => self.parse_designated()?,
                "with" => self.parse_with_auth()?,
                "verify" => self.parse_verify()?,
                _ => return Err(self.unexpected()),
            }
        }
        let module = module.ok_or_else(|| self.unexpected())?;
        let root_span = self.source.span(0, self.source.text.len());
        let children = self
            .loci
            .iter()
            .map(|decl| decl.node.clone())
            .chain(self.principals.iter().map(|decl| decl.node.clone()))
            .chain(self.types.iter().map(|decl| decl.node.clone()))
            .chain(self.states.iter().map(|decl| decl.node.clone()))
            .chain(self.roles.iter().map(|decl| decl.node.clone()))
            .chain(self.relations.iter().map(|decl| decl.node.clone()))
            .chain(self.designated_results.iter().map(|decl| decl.node.clone()))
            .chain(
                self.designated_result_consumers
                    .iter()
                    .map(|decl| decl.node.clone()),
            )
            .collect();
        Ok(SurfaceV0File {
            root: SyntaxNode::new(SyntaxKind::Module, module.name.clone(), root_span, children),
            module,
            loci: self.loci,
            principals: self.principals,
            types: self.types,
            states: self.states,
            roles: self.roles,
            assignments: self.assignments,
            relations: self.relations,
            designated_results: self.designated_results,
            designated_result_consumers: self.designated_result_consumers,
            deferred_forms: self.deferred_forms,
            relation_mutations: self.relation_mutations,
        })
    }

    fn parse_module(&mut self) -> Result<ModuleDecl, ParseDiagnostics> {
        let start = self.expect("module")?.span;
        let (first, mut end) = self.identifier()?;
        let mut name = first;
        while self.consume(".") {
            let (part, span) = self.identifier()?;
            name.push('.');
            name.push_str(&part);
            end = span;
        }
        let span = joined_span(&start, &end);
        Ok(ModuleDecl {
            name: name.clone(),
            node: SyntaxNode::new(SyntaxKind::Module, name, span, Vec::new()),
        })
    }

    fn parse_locus(&mut self) -> Result<LocusDecl, ParseDiagnostics> {
        let start = self.expect("locus")?.span;
        let (name, end) = self.identifier()?;
        let span = joined_span(&start, &end);
        Ok(LocusDecl {
            name: name.clone(),
            name_span: end.clone(),
            node: SyntaxNode::new(SyntaxKind::Locus, name, span, Vec::new()),
        })
    }

    fn parse_principal(&mut self) -> Result<PrincipalDecl, ParseDiagnostics> {
        let start = self.expect("principal")?.span;
        let (name, end) = self.identifier()?;
        let span = joined_span(&start, &end);
        Ok(PrincipalDecl {
            name: name.clone(),
            node: SyntaxNode::new(SyntaxKind::Principal, name, span, Vec::new()),
        })
    }

    fn parse_type(&mut self) -> Result<TypeDecl, ParseDiagnostics> {
        let start = self.expect("type")?.span;
        let (name, end) = self.identifier()?;
        let span = joined_span(&start, &end);
        Ok(TypeDecl {
            name: name.clone(),
            name_span: end.clone(),
            node: SyntaxNode::new(SyntaxKind::Type, name, span, Vec::new()),
        })
    }

    fn parse_state(&mut self) -> Result<StateDecl, ParseDiagnostics> {
        let start = self.expect("state")?.span;
        let (name, _) = self.identifier()?;
        self.expect("[")?;
        let (index_name, _) = self.identifier()?;
        self.expect(":")?;
        let (index_type, index_type_span) = self.identifier()?;
        self.expect("]")?;
        self.expect("at")?;
        let (owner_locus, owner_locus_span) = self.identifier()?;
        self.expect("{")?;
        let mut fields = Vec::new();
        let mut visibility = None;
        while !self.check("}") {
            if self.check("visible") {
                if visibility.is_some() {
                    return Err(self.unexpected());
                }
                visibility = Some(self.parse_state_visibility()?);
                // The bounded M6 grammar permits at most one visibility
                // declaration, and only after the ordinary field schema.
                // Keeping this terminal makes a second `visible` (or a late
                // field) a deterministic typed parse rejection rather than
                // silently changing the declaration's scope.
                if !self.check("}") {
                    return Err(self.unexpected());
                }
                continue;
            }
            let (field_name, field_start) = self.identifier()?;
            self.expect(":")?;
            let (type_name, field_end) = self.identifier()?;
            fields.push(StateField {
                name: field_name,
                type_name,
                span: joined_span(&field_start, &field_end),
                type_span: field_end,
                visibility: None,
            });
        }
        if let Some(declaration) = &visibility {
            for declared in declaration.fields() {
                if let Some(field) = fields.iter_mut().find(|field| field.name == declared.name) {
                    field.visibility = Some(StateFieldVisibility {
                        channel: declaration.channel.clone(),
                        declaration_span: declared.span.clone(),
                    });
                }
            }
        }
        let end = self.expect("}")?.span;
        let span = joined_span(&start, &end);
        Ok(StateDecl {
            name: name.clone(),
            index_name,
            index_type,
            index_type_span,
            owner_locus,
            owner_locus_span,
            fields,
            visibility,
            node: SyntaxNode::new(SyntaxKind::State, name, span, Vec::new()),
        })
    }

    fn parse_state_visibility(&mut self) -> Result<StateVisibility, ParseDiagnostics> {
        let start = self.expect("visible")?.span;
        let (channel, _) = self.identifier()?;
        self.expect("fields")?;
        self.expect("(")?;
        let mut fields = Vec::new();
        if self.check(")") {
            return Err(self.unexpected());
        }
        while !self.check(")") {
            let (name, span) = self.identifier()?;
            fields.push(StateVisibilityField { name, span });
            if !self.consume(",") {
                break;
            }
        }
        let end = self.expect(")")?.span;
        Ok(StateVisibility {
            channel,
            fields,
            span: joined_span(&start, &end),
        })
    }

    fn parse_role(&mut self) -> Result<(), ParseDiagnostics> {
        let start = self.expect("Role")?.span;
        self.expect("[")?;
        let (actor, actor_span) = self.identifier()?;
        if actor != "self" {
            return Err(ParseDiagnostics::one(
                ParseErrorKind::RoleActorMustBeLiteralSelf,
                actor_span,
            ));
        }
        self.expect("]")?;
        self.expect("at")?;
        let (evaluation_locus, end) = self.identifier()?;
        let header_span = joined_span(&start, &end);
        self.expect("{")?;
        let mut whens = Vec::new();
        while !self.check("}") {
            whens.push(self.parse_when(&actor, &evaluation_locus)?);
        }
        self.expect("}")?;
        let children = whens.iter().map(|when| when.node.clone()).collect();
        let label = format!("Role[{actor}] at {evaluation_locus}");
        let node = SyntaxNode::new(SyntaxKind::RoleInstance, label, header_span, children);
        self.roles.push(RoleInstance {
            actor,
            actor_span,
            evaluation_locus,
            evaluation_locus_span: end,
            whens,
            node,
        });
        Ok(())
    }

    fn parse_when(&mut self, actor: &str, role_locus: &str) -> Result<WhenDecl, ParseDiagnostics> {
        let start = self.expect("when")?.span;
        let (event, _) = self.identifier()?;
        self.expect("(")?;
        let mut parameters = Vec::new();
        while !self.check(")") {
            let (name, parameter_start) = self.identifier()?;
            self.expect(":")?;
            let (type_name, parameter_end) = self.identifier()?;
            parameters.push(Parameter {
                name,
                type_name,
                span: joined_span(&parameter_start, &parameter_end),
                type_span: parameter_end,
            });
            if !self.consume(",") {
                break;
            }
        }
        self.expect(")")?;
        self.expect("fails")?;
        self.expect("(")?;
        let mut failures = Vec::new();
        while !self.check(")") {
            let (failure, _) = self.identifier()?;
            failures.push(failure);
            if !self.consume(",") {
                break;
            }
        }
        let header_end = self.expect(")")?.span;
        let header_span = joined_span(&start, &header_end);
        self.expect("{")?;
        while !self.check("}") {
            self.parse_at_block(&event, actor, role_locus)?;
        }
        self.expect("}")?;
        Ok(WhenDecl {
            event: event.clone(),
            parameters,
            failures,
            actor: actor.to_string(),
            role_locus: role_locus.to_string(),
            node: SyntaxNode::new(SyntaxKind::When, event, header_span, Vec::new()),
        })
    }

    fn parse_at_block(
        &mut self,
        event: &str,
        actor: &str,
        role_locus: &str,
    ) -> Result<(), ParseDiagnostics> {
        let at = self.expect("at")?.span;
        let (locus, locus_end) = self.identifier()?;
        let owner_locus_span = joined_span(&at, &locus_end);
        self.expect("{")?;
        while !self.check("}") {
            if self.check("relation") {
                self.parse_relation_mutation(&locus)?;
            } else {
                self.parse_assignment(event, actor, role_locus, &locus, &owner_locus_span)?;
            }
        }
        self.expect("}")?;
        Ok(())
    }

    fn parse_assignment(
        &mut self,
        event: &str,
        actor: &str,
        role_locus: &str,
        locus: &str,
        owner_locus_span: &SurfaceV0Span,
    ) -> Result<(), ParseDiagnostics> {
        let target = self.parse_reference()?;
        let start = target.span.clone();
        self.expect("=")?;
        let expression = self.parse_bounded_expression_until(&["}"])?;
        let expression_span = expression.span().clone();
        let span = joined_span(&start, expression.span());
        let label = target.text.clone();
        self.assignments.push(Assignment {
            target,
            rhs_references: expression.state_refs.clone(),
            expression,
            role_locus: role_locus.to_string(),
            owner_locus: locus.to_string(),
            owner_locus_span: owner_locus_span.clone(),
            event: event.to_string(),
            actor: actor.to_string(),
            expression_span,
            node: SyntaxNode::new(SyntaxKind::Assignment, label, span, Vec::new()),
        });
        Ok(())
    }

    fn parse_relation_mutation(&mut self, locus: &str) -> Result<(), ParseDiagnostics> {
        let start = self.expect("relation")?.span;
        let (relation, _) = self.identifier()?;
        self.expect("mutate")?;
        let (_, end) = self.identifier()?;
        let span = joined_span(&start, &end);
        self.relation_mutations.push(RelationMutation {
            relation: relation.clone(),
            locus: locus.to_string(),
            node: SyntaxNode::new(SyntaxKind::RelationMutation, relation, span, Vec::new()),
        });
        Ok(())
    }

    fn parse_relation(&mut self) -> Result<(), ParseDiagnostics> {
        let start = self.expect("relation")?.span;
        let (name, _) = self.identifier()?;
        self.expect("at")?;
        let (owner_locus, owner_locus_span) = self.identifier()?;
        self.expect("{")?;

        self.expect("subject")?;
        let (subject, _) = self.identifier()?;
        self.expect(":")?;
        let (subject_type, subject_type_span) = self.identifier()?;

        let primary = self.parse_relation_anchor("primary")?;
        let fallback = self.parse_relation_anchor("fallback")?;
        self.expect("bind")?;
        self.expect("frontier")?;
        let (binding_frontier, _) = self.identifier()?;

        let publication_start = self.expect("publish")?.span;
        let publication = if self.consume("relation") {
            RelationPublication::Relation {
                span: joined_span(&publication_start, &self.previous().span),
            }
        } else {
            self.expect("value")?;
            let (value, value_span) = self.identifier()?;
            RelationPublication::Value {
                value,
                span: joined_span(&publication_start, &value_span),
            }
        };

        let (consumer_projection_locus, consumer_projection_locus_span) = if self.consume("project")
        {
            self.expect("at")?;
            let (locus, span) = self.identifier()?;
            self.expect("local")?;
            (Some(locus), Some(span))
        } else {
            (None, None)
        };
        let end = self.expect("}")?.span;
        let span = joined_span(&start, &end);
        self.relations.push(MaintainedRelation {
            name: name.clone(),
            owner_locus,
            owner_locus_span,
            subject,
            subject_type,
            subject_type_span,
            primary,
            fallback,
            binding_frontier,
            publication,
            consumer_projection_locus,
            consumer_projection_locus_span,
            node: SyntaxNode::new(SyntaxKind::Relation, name, span, Vec::new()),
        });
        Ok(())
    }

    fn parse_relation_anchor(
        &mut self,
        expected: &str,
    ) -> Result<RelationAnchor, ParseDiagnostics> {
        let start = self.expect(expected)?.span;
        let (anchor, _) = self.identifier()?;
        let (anchor_locus, anchor_locus_span) = if self.consume("at") {
            let (locus, span) = self.identifier()?;
            (Some(locus), Some(span))
        } else {
            (None, None)
        };
        self.expect("epoch")?;
        let (epoch, _) = self.identifier()?;
        self.expect("transform")?;
        let (transform, transform_span) = self.parse_transform()?;
        let end = transform_span.clone();
        Ok(RelationAnchor {
            anchor,
            anchor_locus,
            anchor_locus_span,
            epoch,
            transform,
            transform_span,
            span: joined_span(&start, &end),
        })
    }

    fn parse_transform(&mut self) -> Result<(RelationTransform, SurfaceV0Span), ParseDiagnostics> {
        if self.check("identity") {
            let span = self.advance().span;
            return Ok((RelationTransform::Identity, span));
        }
        let start = self.expect("translate")?.span;
        self.expect("(")?;
        let x = self.parse_signed_integer()?;
        self.expect(",")?;
        let y = self.parse_signed_integer()?;
        let end = self.expect(")")?.span;
        Ok((
            RelationTransform::Translate { x, y },
            joined_span(&start, &end),
        ))
    }

    fn parse_signed_integer(&mut self) -> Result<i64, ParseDiagnostics> {
        let is_negative = self.consume("-");
        let value = self.parse_bounded_i64()?;
        Ok(if is_negative { -value } else { value })
    }

    fn parse_designated(&mut self) -> Result<(), ParseDiagnostics> {
        let start = self.expect("designated")?.span;
        if self.consume("consume") {
            let (evaluator, result_start) = self.identifier()?;
            self.expect(".")?;
            let (result, result_end) = self.identifier()?;
            let result_ref_span = joined_span(&result_start, &result_end);
            self.expect("at")?;
            let (consumer_locus, consumer_locus_span) = self.identifier()?;
            let span = joined_span(&start, &consumer_locus_span);
            self.designated_result_consumers
                .push(DesignatedResultConsumerDecl {
                    evaluator: evaluator.clone(),
                    result: result.clone(),
                    consumer_locus: consumer_locus.clone(),
                    result_ref_span,
                    consumer_locus_span,
                    node: SyntaxNode::new(
                        SyntaxKind::DesignatedResultConsumer,
                        format!("{evaluator}.{result}@{consumer_locus}"),
                        span,
                        Vec::new(),
                    ),
                });
            return Ok(());
        }
        self.expect("evaluate")?;
        let (evaluator, _) = self.identifier()?;
        self.expect("on")?;
        self.expect("tick")?;
        let (tick_frontier, _) = self.identifier()?;
        self.expect("publish")?;
        let (result, _) = self.identifier()?;
        self.expect("=")?;
        let expression = self.parse_bounded_expression_until(&["with", "verify", "designated"])?;
        let expression_span = expression.span().clone();
        let span = joined_span(&start, expression.span());
        self.designated_results.push(DesignatedResultDecl {
            evaluator: evaluator.clone(),
            tick_frontier,
            result: result.clone(),
            expression,
            expression_span,
            node: SyntaxNode::new(SyntaxKind::DesignatedResult, evaluator, span, Vec::new()),
        });
        Ok(())
    }

    fn parse_bounded_expression_until(
        &mut self,
        terminators: &[&str],
    ) -> Result<BoundedExpression, ParseDiagnostics> {
        if self.is_eof() || terminators.iter().any(|terminator| self.check(terminator)) {
            return Err(self.unexpected());
        }
        let token_start = self.index;
        let start = self.current().span.clone();
        let mut last = start.clone();
        let mut state_refs = Vec::new();
        let mut int_literals = Vec::new();
        let mut binary_ops = Vec::new();
        let mut parts = Vec::new();

        while !self.is_eof() && !terminators.iter().any(|terminator| self.check(terminator)) {
            if self.current_is_identifier() && self.reference_starts_here() {
                let reference = self.parse_reference()?;
                last = reference.span.clone();
                state_refs.push(reference.clone());
                parts.push(BoundedExpressionPart::StateReference(reference));
                continue;
            }

            if self
                .current()
                .text
                .bytes()
                .all(|byte| byte.is_ascii_digit())
            {
                let span = self.current().span.clone();
                let value = self.parse_bounded_i64()?;
                last = span.clone();
                let literal = BoundedIntegerLiteral { value, span };
                int_literals.push(literal.clone());
                parts.push(BoundedExpressionPart::IntegerLiteral(literal));
                continue;
            }

            match self.current().text.as_str() {
                "+" | "-" => {
                    let operator = if self.check("+") {
                        BoundedBinaryOperator::Add
                    } else {
                        BoundedBinaryOperator::Subtract
                    };
                    let span = self.advance().span;
                    last = span.clone();
                    let operation = BoundedBinaryOperation { operator, span };
                    binary_ops.push(operation.clone());
                    parts.push(BoundedExpressionPart::BinaryOperation(operation));
                }
                // M6 deliberately retains its broad expression-token
                // collector. `}` remains the surrounding owner-block
                // terminator, while the other grammar tokens are retained as
                // opaque input for M7's finite expression check.
                "{" | "[" | "]" | "(" | ")" | ":" | "," | "." | "=" => {
                    last = self.advance().span;
                    parts.push(BoundedExpressionPart::Opaque);
                }
                _ if self.current_is_identifier() => {
                    let token = self.advance();
                    last = token.span.clone();
                    parts.push(BoundedExpressionPart::Identifier {
                        name: token.text,
                        span: token.span,
                    });
                }
                _ => return Err(self.unexpected()),
            }
        }

        let span = joined_span(&start, &last);
        let tokens = self.tokens[token_start..self.index]
            .iter()
            .map(|token| BoundedExpressionToken {
                span: token.span.clone(),
            })
            .collect();

        Ok(BoundedExpression {
            span: span.clone(),
            state_refs,
            int_literals,
            binary_ops,
            tokens,
            tree: bounded_expression_tree(parts, span),
        })
    }

    fn parse_with_auth(&mut self) -> Result<(), ParseDiagnostics> {
        let start = self.expect("with")?.span;
        self.expect("auth")?;
        let (name, end) = self.identifier()?;
        let span = joined_span(&start, &end);
        self.deferred_forms.entries.push(DeferredForm {
            kind: DeferredFormKind::WithAuth,
            name: name.clone(),
            node: SyntaxNode::new(SyntaxKind::WithAuth, name, span, Vec::new()),
        });
        Ok(())
    }

    fn parse_verify(&mut self) -> Result<(), ParseDiagnostics> {
        let start = self.expect("verify")?.span;
        let (name, end) = self.identifier()?;
        let span = joined_span(&start, &end);
        self.deferred_forms.entries.push(DeferredForm {
            kind: DeferredFormKind::Verify,
            name: name.clone(),
            node: SyntaxNode::new(SyntaxKind::Verify, name, span, Vec::new()),
        });
        Ok(())
    }

    fn parse_reference(&mut self) -> Result<SurfaceReference, ParseDiagnostics> {
        let (base, start) = self.identifier()?;
        let index = if self.consume("[") {
            let (index, _) = self.identifier()?;
            self.expect("]")?;
            Some(index)
        } else {
            None
        };
        let field = if self.consume(".") {
            let (field, _) = self.identifier()?;
            Some(field)
        } else {
            None
        };
        let end = self.previous().span.clone();
        let span = joined_span(&start, &end);
        let text = self.source.text[span.byte_range()].to_string();
        Ok(SurfaceReference {
            text,
            base,
            index,
            field,
            span,
        })
    }

    fn reference_starts_here(&self) -> bool {
        self.peek_n(1).is_some_and(|next| next.text == "[")
            || self.peek_n(1).is_some_and(|next| next.text == ".")
    }

    fn identifier(&mut self) -> Result<(String, SurfaceV0Span), ParseDiagnostics> {
        if self.current_is_identifier() {
            let token = self.advance();
            Ok((token.text, token.span))
        } else {
            Err(self.unexpected())
        }
    }

    fn integer(&mut self) -> Result<(String, SurfaceV0Span), ParseDiagnostics> {
        let token = self.current();
        if token.text.bytes().all(|byte| byte.is_ascii_digit()) {
            let token = self.advance();
            Ok((token.text, token.span))
        } else {
            Err(self.unexpected())
        }
    }

    fn parse_bounded_i64(&mut self) -> Result<i64, ParseDiagnostics> {
        let (value, span) = self.integer()?;
        value
            .parse::<i64>()
            .map_err(|_| ParseDiagnostics::one(ParseErrorKind::IntegerLiteralOutOfRange, span))
    }

    fn expect(&mut self, expected: &str) -> Result<Token, ParseDiagnostics> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(self.unexpected())
        }
    }

    fn consume(&mut self, expected: &str) -> bool {
        if self.check(expected) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &str) -> bool {
        self.current().text == expected
    }

    fn current_is_identifier(&self) -> bool {
        self.current()
            .text
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
    }

    fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.index.saturating_sub(1)]
    }

    fn peek_n(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.index + offset)
    }

    fn peek_text(&self) -> &str {
        &self.current().text
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if !self.is_eof() {
            self.index += 1;
        }
        token
    }

    fn is_eof(&self) -> bool {
        self.current().text.is_empty()
    }

    fn unexpected(&self) -> ParseDiagnostics {
        ParseDiagnostics::one(
            ParseErrorKind::UnexpectedSyntax,
            self.current().span.clone(),
        )
    }
}

fn bounded_expression_tree(
    parts: Vec<BoundedExpressionPart>,
    fallback_span: SurfaceV0Span,
) -> BoundedExpressionTree {
    let mut parts = parts.into_iter();
    let Some(first) = parts.next() else {
        return BoundedExpressionTree::Opaque {
            span: fallback_span,
        };
    };
    let mut tree = match first {
        BoundedExpressionPart::StateReference(reference) => {
            BoundedExpressionTree::StateReference(reference)
        }
        BoundedExpressionPart::Identifier { name, span } => {
            BoundedExpressionTree::Identifier { name, span }
        }
        BoundedExpressionPart::IntegerLiteral(literal) => {
            BoundedExpressionTree::IntegerLiteral(literal)
        }
        BoundedExpressionPart::BinaryOperation(_) | BoundedExpressionPart::Opaque => {
            return BoundedExpressionTree::Opaque {
                span: fallback_span,
            };
        }
    };

    loop {
        let Some(part) = parts.next() else {
            return tree;
        };
        let BoundedExpressionPart::BinaryOperation(operation) = part else {
            return BoundedExpressionTree::Opaque {
                span: fallback_span,
            };
        };
        let Some(next) = parts.next() else {
            return BoundedExpressionTree::Opaque {
                span: fallback_span,
            };
        };
        let right = match next {
            BoundedExpressionPart::StateReference(reference) => {
                BoundedExpressionTree::StateReference(reference)
            }
            BoundedExpressionPart::Identifier { name, span } => {
                BoundedExpressionTree::Identifier { name, span }
            }
            BoundedExpressionPart::IntegerLiteral(literal) => {
                BoundedExpressionTree::IntegerLiteral(literal)
            }
            BoundedExpressionPart::BinaryOperation(_) | BoundedExpressionPart::Opaque => {
                return BoundedExpressionTree::Opaque {
                    span: fallback_span,
                };
            }
        };
        let span = joined_span(tree.span(), right.span());
        tree = BoundedExpressionTree::Binary {
            operator: operation.operator,
            span,
            left: Box::new(tree),
            right: Box::new(right),
        };
    }
}

/// Parses M6 Surface v0 without importing the historical Surface-alpha parser.
pub fn parse_surface_v0(source: FixtureSource) -> Result<SurfaceV0File, ParseDiagnostics> {
    let tokens = lex(&source)?;
    for token in &tokens {
        let kind = match token.text.as_str() {
            "send" | "receive" => Some(ParseErrorKind::UnsupportedTransportSyntax),
            "occurrence" => Some(ParseErrorKind::UnsupportedOccurrenceSyntax),
            "envelope" => Some(ParseErrorKind::UnsupportedEnvelopeSyntax),
            _ => None,
        };
        if let Some(kind) = kind {
            return Err(ParseDiagnostics::one(kind, token.span.clone()));
        }
    }
    Parser::new(source, tokens).parse()
}

fn lex(source: &FixtureSource) -> Result<Vec<Token>, ParseDiagnostics> {
    let bytes = source.text.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b' ' | b'\t' | b'\r' | b'\n' => index += 1,
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                index += 2;
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            byte if byte.is_ascii_alphabetic() || byte == b'_' => {
                let start = index;
                index += 1;
                while index < bytes.len()
                    && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
                {
                    index += 1;
                }
                tokens.push(Token {
                    text: source.text[start..index].to_string(),
                    span: source.span(start, index),
                });
            }
            byte if byte.is_ascii_digit() => {
                let start = index;
                index += 1;
                while index < bytes.len() && bytes[index].is_ascii_digit() {
                    index += 1;
                }
                tokens.push(Token {
                    text: source.text[start..index].to_string(),
                    span: source.span(start, index),
                });
            }
            byte @ (b'{' | b'}' | b'[' | b']' | b'(' | b')' | b':' | b',' | b'.' | b'=' | b'+'
            | b'-') => {
                let start = index;
                index += 1;
                tokens.push(Token {
                    text: char::from(byte).to_string(),
                    span: source.span(start, index),
                });
            }
            _ => {
                return Err(ParseDiagnostics::one(
                    ParseErrorKind::UnexpectedSyntax,
                    source.span(index, index + 1),
                ));
            }
        }
    }
    tokens.push(Token {
        text: String::new(),
        span: source.span(bytes.len(), bytes.len()),
    });
    Ok(tokens)
}

fn joined_span(start: &SurfaceV0Span, end: &SurfaceV0Span) -> SurfaceV0Span {
    debug_assert_eq!(start.file, end.file);
    SurfaceV0Span {
        file: start.file.clone(),
        byte_start: start.byte_start,
        byte_end: end.byte_end,
        start_line: start.start_line,
        start_column: start.start_column,
        end_line: end.end_line,
        end_column: end.end_column,
    }
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
