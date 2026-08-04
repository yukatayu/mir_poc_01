//! Finite, parser-free shared M5 reference model.
//!
//! This module deliberately owns one explicit configuration for the shared M1--M4
//! carrier.  It does not call the M3 evaluation harness or the M4 projection
//! harness: those are bounded evidence models, while this module exposes the
//! configuration, checked steps, diagnostics, and trace data directly.

use std::{
    any::TypeId,
    collections::{BTreeMap, BTreeSet},
};

macro_rules! string_id {
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
    };
}

macro_rules! epoch_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u64);

        impl $name {
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            pub const fn value(self) -> u64 {
                self.0
            }
        }
    };
}

string_id!(OccurrenceId);
string_id!(ResultKey);
string_id!(ReceiptRef);
string_id!(ReceiptRequestRef);
string_id!(LocusRef);
string_id!(PrincipalRef);
string_id!(EntityRef);
string_id!(RelationKey);
string_id!(CapabilityName);
string_id!(WitnessRef);
string_id!(DesignatedEvaluatorRef);
string_id!(FieldRef);
string_id!(PresentationContextId);
string_id!(CutId);

epoch_id!(AnchorEpoch);
epoch_id!(BindingEpoch);
epoch_id!(MembershipEpoch);
epoch_id!(LeaseEpoch);
epoch_id!(ResultVersion);

/// A reference to an occurrence created by `atomic_cut`.  A save may name this
/// reference only after the corresponding cut exists in the shared history.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomicCutRef {
    pub cut_id: CutId,
}

impl AtomicCutRef {
    pub fn new(cut_id: CutId) -> Self {
        Self { cut_id }
    }
}

/// A source location carried by manually constructed semantic fragments.  It is
/// intentionally not a parser span or a commitment to Surface grammar.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceRef {
    pub path: String,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

impl SourceRef {
    pub fn new(
        path: impl Into<String>,
        start_line: u32,
        start_column: u32,
        end_line: u32,
        end_column: u32,
    ) -> Self {
        Self {
            path: path.into(),
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }
}

/// A finite value carrier sufficient for the owner and designated examples.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

impl Value {
    pub const fn int(value: i64) -> Self {
        Self::Int(value)
    }

    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// An owner-owned storage key.  It is nominally separate from relation and
/// result keys so a projection cannot accidentally target an owner store.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateKey {
    pub namespace: String,
    pub field: FieldRef,
}

impl StateKey {
    pub fn field(namespace: impl Into<String>, field: FieldRef) -> Self {
        Self {
            namespace: namespace.into(),
            field,
        }
    }
}

/// A finite owner command.  The reference model keeps the mutation explicit;
/// it never smuggles a consumer-derived value into this command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnerCommand {
    Add { state: StateKey, amount: Value },
}

impl OwnerCommand {
    pub fn add(state: StateKey, amount: Value) -> Self {
        Self::Add { state, amount }
    }

    fn state(&self) -> &StateKey {
        match self {
            Self::Add { state, .. } => state,
        }
    }
}

/// M3 result publication coordinates.  A `ResultFrontier` is intentionally a
/// distinct nominal type from an M4 `BindingActivationFrontier`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResultFrontier {
    results: Vec<ResultKey>,
}

impl ResultFrontier {
    pub fn from_ordered_results(mut results: Vec<ResultKey>) -> Result<Self, Diagnostic> {
        if results.is_empty() {
            return Err(Diagnostic::simple(DiagnosticCode::EmptyResultFrontier));
        }
        results.sort();
        if results.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(Diagnostic::simple(DiagnosticCode::DuplicateResultFrontier));
        }
        Ok(Self { results })
    }

    pub fn as_slice(&self) -> &[ResultKey] {
        &self.results
    }
}

/// M4 binding activation coordinates.  These use occurrence identifiers rather
/// than M3 result identifiers and cannot be compared for equality by accident.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BindingActivationFrontier {
    occurrences: Vec<OccurrenceId>,
}

impl BindingActivationFrontier {
    pub fn from_ordered_occurrences(
        mut occurrences: Vec<OccurrenceId>,
    ) -> Result<Self, Diagnostic> {
        if occurrences.is_empty() {
            return Err(Diagnostic::simple(
                DiagnosticCode::EmptyBindingActivationFrontier,
            ));
        }
        occurrences.sort();
        if occurrences.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(Diagnostic::simple(
                DiagnosticCode::DuplicateBindingActivationOccurrence,
            ));
        }
        Ok(Self { occurrences })
    }

    pub fn as_slice(&self) -> &[OccurrenceId] {
        &self.occurrences
    }

    /// Documents the deliberate type-level separation from M3 result
    /// frontiers.  There is no conversion or equality relation between them.
    pub fn is_nominally_distinct_from(&self, _: &ResultFrontier) -> bool {
        TypeId::of::<Self>() != TypeId::of::<ResultFrontier>()
    }
}

/// A finite observation-label lattice used by the checked local projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Label {
    Public,
    Restricted,
    Private,
}

impl Label {
    fn join(self, other: Self) -> Self {
        self.max(other)
    }
}

/// Translation-only transform used by the finite M5 reference model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform2 {
    x: i64,
    y: i64,
}

impl Transform2 {
    pub fn identity() -> Self {
        Self::translation(0, 0)
    }

    pub fn translation(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn checked_compose(&self, offset: &Self) -> Option<Self> {
        Some(Self::translation(
            self.x.checked_add(offset.x)?,
            self.y.checked_add(offset.y)?,
        ))
    }
}

/// The Source-to-Core generated graph edges made by this deliberately small
/// elaboration.  The edges are model data, not parser output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratedEdge {
    Request {
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
    },
    ReceiptUse {
        source_ref: SourceRef,
        receipt: ReceiptRef,
    },
    OwnerWrite {
        source_ref: SourceRef,
        owner: LocusRef,
        state: StateKey,
    },
}

impl GeneratedEdge {
    pub fn request(
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
    ) -> Self {
        Self::Request {
            source_ref,
            caller,
            owner,
            capability,
        }
    }

    pub fn receipt_use(source_ref: SourceRef, receipt: ReceiptRef) -> Self {
        Self::ReceiptUse {
            source_ref,
            receipt,
        }
    }

    pub fn owner_write(source_ref: SourceRef, owner: LocusRef, state: StateKey) -> Self {
        Self::OwnerWrite {
            source_ref,
            owner,
            state,
        }
    }
}

/// A proof-facing authority fact generated during elaboration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityObligation {
    Capability {
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
    },
    Witness {
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
    },
    ReceiptRelease {
        source_ref: SourceRef,
        receipt: ReceiptRef,
    },
    ExplicitReceiptRequired {
        source_ref: SourceRef,
        requesting_owner: LocusRef,
        operand_owner: LocusRef,
    },
}

impl AuthorityObligation {
    pub fn capability(
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
    ) -> Self {
        Self::Capability {
            source_ref,
            caller,
            owner,
            capability,
        }
    }

    pub fn witness(source_ref: SourceRef, caller: PrincipalRef, owner: LocusRef) -> Self {
        Self::Witness {
            source_ref,
            caller,
            owner,
        }
    }

    pub fn receipt_release(source_ref: SourceRef, receipt: ReceiptRef) -> Self {
        Self::ReceiptRelease {
            source_ref,
            receipt,
        }
    }

    pub fn explicit_receipt_required(
        source_ref: SourceRef,
        requesting_owner: LocusRef,
        operand_owner: LocusRef,
    ) -> Self {
        Self::ExplicitReceiptRequired {
            source_ref,
            requesting_owner,
            operand_owner,
        }
    }
}

/// The small manually constructed Core subset used for M5 correspondence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreOp {
    OwnerReadModifyWrite {
        owner: LocusRef,
        command: OwnerCommand,
    },
}

impl CoreOp {
    pub fn owner_rmw(owner: LocusRef, command: OwnerCommand) -> Self {
        Self::OwnerReadModifyWrite { owner, command }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Core {
    source_ref: SourceRef,
    ops: Vec<CoreOp>,
    generated_edges: Vec<GeneratedEdge>,
    authority_obligations: Vec<AuthorityObligation>,
}

impl Core {
    /// Conservative M6 lowering factory for an already-local owner RMW.  It
    /// intentionally records no receipt materialization: same-owner reads do
    /// not fabricate a cross-owner release chain.
    pub fn same_owner_rmw(
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        command: OwnerCommand,
        capability: CapabilityName,
    ) -> Self {
        Self {
            source_ref: source_ref.clone(),
            ops: vec![CoreOp::owner_rmw(owner.clone(), command.clone())],
            generated_edges: vec![
                GeneratedEdge::request(
                    source_ref.clone(),
                    caller.clone(),
                    owner.clone(),
                    capability.clone(),
                ),
                GeneratedEdge::owner_write(
                    source_ref.clone(),
                    owner.clone(),
                    command.state().clone(),
                ),
            ],
            authority_obligations: vec![
                AuthorityObligation::capability(
                    source_ref.clone(),
                    caller.clone(),
                    owner.clone(),
                    capability,
                ),
                AuthorityObligation::witness(source_ref, caller, owner),
            ],
        }
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub fn ops(&self) -> &[CoreOp] {
        &self.ops
    }

    pub fn generated_edges(&self) -> &[GeneratedEdge] {
        &self.generated_edges
    }

    pub fn authority_obligations(&self) -> &[AuthorityObligation] {
        &self.authority_obligations
    }
}

/// A finite semantic-category Surface fragment.  This is not a syntax tree and
/// does not make a parser or grammar claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceFragment {
    OwnerRmwWithReceipt {
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        command: OwnerCommand,
        receipt: ReceiptRef,
        capability: CapabilityName,
    },
    CrossOwnerReadWithoutReceipt {
        source_ref: SourceRef,
        caller: PrincipalRef,
        requesting_owner: LocusRef,
        operand_owner: LocusRef,
        state: StateKey,
    },
}

impl SurfaceFragment {
    pub fn owner_rmw_with_receipt(
        source_ref: SourceRef,
        caller: PrincipalRef,
        owner: LocusRef,
        command: OwnerCommand,
        receipt: ReceiptRef,
        capability: CapabilityName,
    ) -> Self {
        Self::OwnerRmwWithReceipt {
            source_ref,
            caller,
            owner,
            command,
            receipt,
            capability,
        }
    }

    pub fn cross_owner_read_without_receipt(
        source_ref: SourceRef,
        caller: PrincipalRef,
        requesting_owner: LocusRef,
        operand_owner: LocusRef,
        state: StateKey,
    ) -> Self {
        Self::CrossOwnerReadWithoutReceipt {
            source_ref,
            caller,
            requesting_owner,
            operand_owner,
            state,
        }
    }

    pub fn elaborate(&self) -> Elaboration {
        match self {
            Self::OwnerRmwWithReceipt {
                source_ref,
                caller,
                owner,
                command,
                receipt,
                capability,
            } => Elaboration::Core(Core {
                source_ref: source_ref.clone(),
                ops: vec![CoreOp::owner_rmw(owner.clone(), command.clone())],
                generated_edges: vec![
                    GeneratedEdge::request(
                        source_ref.clone(),
                        caller.clone(),
                        owner.clone(),
                        capability.clone(),
                    ),
                    GeneratedEdge::receipt_use(source_ref.clone(), receipt.clone()),
                    GeneratedEdge::owner_write(
                        source_ref.clone(),
                        owner.clone(),
                        command.state().clone(),
                    ),
                ],
                authority_obligations: vec![
                    AuthorityObligation::capability(
                        source_ref.clone(),
                        caller.clone(),
                        owner.clone(),
                        capability.clone(),
                    ),
                    AuthorityObligation::witness(source_ref.clone(), caller.clone(), owner.clone()),
                    AuthorityObligation::receipt_release(source_ref.clone(), receipt.clone()),
                ],
            }),
            Self::CrossOwnerReadWithoutReceipt {
                source_ref,
                requesting_owner,
                operand_owner,
                ..
            } => Elaboration::Diagnostic(Diagnostic::with_obligations(
                DiagnosticCode::CrossOwnerOperandRequiresReceipt,
                source_ref.clone(),
                Vec::new(),
                vec![AuthorityObligation::explicit_receipt_required(
                    source_ref.clone(),
                    requesting_owner.clone(),
                    operand_owner.clone(),
                )],
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Elaboration {
    Core(Core),
    Diagnostic(Diagnostic),
}

impl Elaboration {
    pub fn expect_core(self) -> Core {
        match self {
            Self::Core(core) => core,
            Self::Diagnostic(diagnostic) => {
                panic!("expected Core elaboration, got diagnostic: {diagnostic:?}")
            }
        }
    }

    pub fn expect_diagnostic(self) -> Diagnostic {
        match self {
            Self::Diagnostic(diagnostic) => diagnostic,
            Self::Core(core) => panic!("expected diagnostic elaboration, got Core: {core:?}"),
        }
    }
}

/// Typed failure codes shared by elaboration, checked transitions, and bounded
/// counterexamples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticCode {
    OwnerActionLocusMismatch,
    CrossOwnerWriteTargetOutsideActionLocus,
    FieldlessAssignmentTarget,
    CrossOwnerOperandRequiresReceipt,
    EmptyResultFrontier,
    DuplicateResultFrontier,
    EmptyBindingActivationFrontier,
    DuplicateBindingActivationOccurrence,
    OwnerAuthorityDenied,
    OwnerCommandValueMismatch,
    OwnerCommandOverflow,
    RelationAuthorityDenied,
    RelationAlreadyDefined,
    RelationMissing,
    RelationHasNoFallback,
    RelationAnchorMissing,
    BindingEpochDidNotAdvance,
    ConsumerProjectionNotAdmitted,
    BindingActivationFrontierMismatch,
    PresentationSampleReleaseDenied,
    PresentationSampleEpochMismatch,
    PresentationSampleMissing,
    SplitFrameProjection,
    ProjectionTransformOverflow,
    ConsumerProjectionMaterializationDenied,
    StaleWitness,
    StaleMembership,
    ExpiredLease,
    StaleRelationLineage,
    RelationPublicationRequired,
    MissingAtomicCut,
    BadRelationship,
    ReceiptRequestMissing,
    ReceiptReleaseChainInvalid,
    DesignatedResultMissing,
    DesignatedResultAlreadyConsumed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityFailure {
    OwnerMismatch,
    RelationMismatch,
    StaleBindingEpochOrWitness,
    UnknownAuthority,
}

/// A typed diagnostic carries all model-facing evidence needed by the tests;
/// it does not use a stringly opaque error channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    code: DiagnosticCode,
    source_ref: Option<SourceRef>,
    generated_edges: Vec<GeneratedEdge>,
    authority_obligations: Vec<AuthorityObligation>,
    rejected_materialization: Option<MaterializationRequest>,
    rejected_mutation_target: Option<MutationTarget>,
    authority_failure: Option<AuthorityFailure>,
}

impl Diagnostic {
    fn simple(code: DiagnosticCode) -> Self {
        Self {
            code,
            source_ref: None,
            generated_edges: Vec::new(),
            authority_obligations: Vec::new(),
            rejected_materialization: None,
            rejected_mutation_target: None,
            authority_failure: None,
        }
    }

    fn at(code: DiagnosticCode, source_ref: SourceRef) -> Self {
        let mut diagnostic = Self::simple(code);
        diagnostic.source_ref = Some(source_ref);
        diagnostic
    }

    fn with_obligations(
        code: DiagnosticCode,
        source_ref: SourceRef,
        generated_edges: Vec<GeneratedEdge>,
        authority_obligations: Vec<AuthorityObligation>,
    ) -> Self {
        Self {
            code,
            source_ref: Some(source_ref),
            generated_edges,
            authority_obligations,
            rejected_materialization: None,
            rejected_mutation_target: None,
            authority_failure: None,
        }
    }

    fn with_authority_failure(
        source_ref: SourceRef,
        code: DiagnosticCode,
        failure: AuthorityFailure,
    ) -> Self {
        let mut diagnostic = Self::at(code, source_ref);
        diagnostic.authority_failure = Some(failure);
        diagnostic
    }

    fn with_projection_request(source_ref: SourceRef, request: ConsumerProjectionRequest) -> Self {
        let mut diagnostic = Self::at(
            DiagnosticCode::ConsumerProjectionMaterializationDenied,
            source_ref,
        );
        diagnostic.rejected_materialization = Some(request.materialization);
        diagnostic.rejected_mutation_target = Some(request.mutation_target);
        diagnostic
    }

    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    pub fn source_ref(&self) -> &SourceRef {
        self.source_ref
            .as_ref()
            .expect("source_ref is required by this diagnostic use")
    }

    pub fn generated_edges(&self) -> &[GeneratedEdge] {
        &self.generated_edges
    }

    pub fn authority_obligations(&self) -> &[AuthorityObligation] {
        &self.authority_obligations
    }

    pub const fn rejected_materialization(&self) -> Option<MaterializationRequest> {
        self.rejected_materialization
    }

    pub const fn rejected_mutation_target(&self) -> Option<MutationTarget> {
        self.rejected_mutation_target
    }

    pub const fn authority_failure(&self) -> Option<AuthorityFailure> {
        self.authority_failure
    }
}

/// A direct, non-opaque authorization record.  Relation authority is exact
/// when `relation`, `binding_epoch`, and `witness` are populated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnerAuthority {
    pub principal: PrincipalRef,
    pub owner: LocusRef,
    pub capability: CapabilityName,
    pub membership_epoch: MembershipEpoch,
    pub lease_epoch: LeaseEpoch,
    pub relation: Option<RelationKey>,
    pub binding_epoch: Option<BindingEpoch>,
    pub witness: WitnessRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationOption {
    pub anchor: LocusRef,
    pub offset: Transform2,
    pub anchor_epoch: AnchorEpoch,
}

impl RelationOption {
    pub fn anchor(anchor: LocusRef, offset: Transform2, anchor_epoch: AnchorEpoch) -> Self {
        Self {
            anchor,
            offset,
            anchor_epoch,
        }
    }
}

/// An explicit maintained relation definition.  The first option is primary;
/// later options are semantic fallbacks, not consumer-local presentation gaps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationDef {
    pub relation: RelationKey,
    pub owner: LocusRef,
    pub subject: EntityRef,
    pub options: Vec<RelationOption>,
    pub declared_label: Label,
}

impl RelationDef {
    pub fn follow_with_fallback(
        relation: RelationKey,
        owner: LocusRef,
        subject: EntityRef,
        primary: RelationOption,
        fallback: RelationOption,
        declared_label: Label,
    ) -> Self {
        Self {
            relation,
            owner,
            subject,
            options: vec![primary, fallback],
            declared_label,
        }
    }
}

/// Projection-facing immutable relation data.  It contains anchor requirements
/// and result publication provenance but no anchor pose or subject absolute pose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedRelation {
    definition: RelationDef,
    result_frontier: ResultFrontier,
}

impl ProjectedRelation {
    pub fn required_anchor_epoch(&self, anchor: &LocusRef) -> Option<AnchorEpoch> {
        self.definition
            .options
            .iter()
            .find(|option| &option.anchor == anchor)
            .map(|option| option.anchor_epoch)
    }

    pub fn result_frontier(&self) -> &ResultFrontier {
        &self.result_frontier
    }
}

/// The owner-held M4 binding.  No consumer presentation state is stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationBinding {
    pub definition: RelationDef,
    pub current_option_index: usize,
    pub lineage: Vec<OccurrenceId>,
    pub current_owner_authority: OwnerAuthority,
    activation_frontier: BindingActivationFrontier,
    binding_epoch: BindingEpoch,
}

impl RelationBinding {
    pub fn activation_frontier(&self) -> &BindingActivationFrontier {
        &self.activation_frontier
    }

    pub const fn binding_epoch(&self) -> BindingEpoch {
        self.binding_epoch
    }

    pub fn relation(&self) -> &RelationKey {
        &self.definition.relation
    }
}

/// The owner-published immutable carrier that consumers may project.  It is a
/// snapshot of an exact J binding, not a result materialization or a mutable J
/// reference; therefore it deliberately has no `ResultFrontier` coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedRelation {
    definition: RelationDef,
    selected_option_index: usize,
    binding_epoch: BindingEpoch,
    activation_frontier: BindingActivationFrontier,
    lineage: Vec<OccurrenceId>,
}

impl PublishedRelation {
    fn from_binding(binding: &RelationBinding) -> Self {
        Self {
            definition: binding.definition.clone(),
            selected_option_index: binding.current_option_index,
            binding_epoch: binding.binding_epoch,
            activation_frontier: binding.activation_frontier.clone(),
            lineage: binding.lineage.clone(),
        }
    }

    pub fn relation(&self) -> &RelationKey {
        &self.definition.relation
    }

    pub const fn selected_option_index(&self) -> usize {
        self.selected_option_index
    }

    pub const fn binding_epoch(&self) -> BindingEpoch {
        self.binding_epoch
    }

    pub fn activation_frontier(&self) -> &BindingActivationFrontier {
        &self.activation_frontier
    }

    pub fn required_anchor_epoch(&self, anchor: &LocusRef) -> Option<AnchorEpoch> {
        self.definition
            .options
            .iter()
            .find(|option| &option.anchor == anchor)
            .map(|option| option.anchor_epoch)
    }

    pub const fn label(&self) -> Label {
        self.definition.declared_label
    }

    pub const fn uses_result_frontier(&self) -> bool {
        false
    }

    fn current_option(&self) -> &RelationOption {
        &self.definition.options[self.selected_option_index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationSample {
    pub anchor: LocusRef,
    pub released_to: PrincipalRef,
    pub frontier: BindingActivationFrontier,
    pub anchor_epoch: AnchorEpoch,
    pub transform: Transform2,
    pub label: Label,
}

impl PresentationSample {
    pub fn released(
        anchor: LocusRef,
        released_to: PrincipalRef,
        frontier: BindingActivationFrontier,
        anchor_epoch: AnchorEpoch,
        transform: Transform2,
        label: Label,
    ) -> Self {
        Self {
            anchor,
            released_to,
            frontier,
            anchor_epoch,
            transform,
            label,
        }
    }
}

/// Ephemeral consumer input.  `SharedConfig` never owns one of these contexts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationContext {
    pub id: PresentationContextId,
    pub consumer: PrincipalRef,
    pub frontier: BindingActivationFrontier,
    pub samples: Vec<PresentationSample>,
    pub fallback: Option<PresentationFallback>,
}

impl PresentationContext {
    pub fn for_consumer(
        id: PresentationContextId,
        consumer: PrincipalRef,
        frontier: BindingActivationFrontier,
    ) -> Self {
        Self {
            id,
            consumer,
            frontier,
            samples: Vec::new(),
            fallback: None,
        }
    }

    pub fn with_sample(mut self, sample: PresentationSample) -> Self {
        self.samples.push(sample);
        self
    }

    pub fn with_presentation_gap(mut self, fallback: PresentationFallback) -> Self {
        self.fallback = Some(fallback);
        self
    }

    fn sample_for(&self, anchor: &LocusRef) -> Option<&PresentationSample> {
        self.samples.iter().find(|sample| &sample.anchor == anchor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresentationFallback {
    HoldLastLocal {
        subject: EntityRef,
        transform: Transform2,
    },
}

impl PresentationFallback {
    pub fn hold_last_local(subject: EntityRef, transform: Transform2) -> Self {
        Self::HoldLastLocal { subject, transform }
    }

    fn transform(&self) -> &Transform2 {
        match self {
            Self::HoldLastLocal { transform, .. } => transform,
        }
    }
}

/// Consumer requests must remain local-only and never target shared semantic
/// state.  The explicit request makes mixed materialization falsifiable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterializationRequest {
    LocalOnly,
    Store,
    PublishValue,
    StorePublishValue,
}

impl MaterializationRequest {
    pub const fn store_publish_value() -> Self {
        Self::StorePublishValue
    }

    const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnly)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationTarget {
    None,
    OwnerStore,
    ReceiptStoreR,
    DesignatedStoreD,
    RelationStoreJ,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsumerProjectionRequest {
    materialization: MaterializationRequest,
    mutation_target: MutationTarget,
}

impl ConsumerProjectionRequest {
    pub const fn new() -> Self {
        Self {
            materialization: MaterializationRequest::LocalOnly,
            mutation_target: MutationTarget::None,
        }
    }

    pub const fn with_materialization(mut self, materialization: MaterializationRequest) -> Self {
        self.materialization = materialization;
        self
    }

    pub const fn with_mutation_target(mut self, mutation_target: MutationTarget) -> Self {
        self.mutation_target = mutation_target;
        self
    }

    fn is_local_only(self) -> bool {
        self.materialization.is_local_only() && self.mutation_target == MutationTarget::None
    }
}

impl Default for ConsumerProjectionRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Projection {
    pub subject_transform: Transform2,
    pub derived_label: Label,
    pub presentation_only: bool,
    absolute_stream_entries: Vec<AbsoluteStreamEntry>,
}

impl Projection {
    pub fn absolute_stream_entries(&self) -> &[AbsoluteStreamEntry] {
        &self.absolute_stream_entries
    }
}

/// Present only to make the absence of an absolute stream testable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsoluteStreamEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRecord {
    pub request: ReceiptRequestRef,
    pub receipt: ReceiptRef,
    pub caller: PrincipalRef,
    pub owner: LocusRef,
    pub result: ResultKey,
    pub frontier: ResultFrontier,
    pub label: Label,
    release_chain: [TraceKind; 4],
}

impl ReceiptRecord {
    pub fn release_chain(&self) -> &[TraceKind] {
        &self.release_chain
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptExchange {
    request: ReceiptRequestRef,
    caller: PrincipalRef,
    owner: LocusRef,
    result: ResultKey,
    frontier: ResultFrontier,
    requested_label: Label,
    served: bool,
    reply: Option<(ReceiptRef, Value, Label)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedResult {
    pub evaluator: DesignatedEvaluatorRef,
    pub result: ResultKey,
    pub frontier: ResultFrontier,
    pub value: Value,
    pub label: Label,
    pub version: ResultVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DesignatedResultIdentity {
    pub evaluator: DesignatedEvaluatorRef,
    pub result: ResultKey,
    pub frontier: ResultFrontier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipRecord {
    pub principal: PrincipalRef,
    pub capability: CapabilityName,
    pub membership_epoch: MembershipEpoch,
    pub lease_epoch: LeaseEpoch,
    pub witness: WitnessRef,
}

/// Named components of the one shared configuration, including M3 R/D and M4
/// J rather than a cartesian product of two opaque harness states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigComponent {
    OccurrenceHistory,
    AuthorityStore,
    ObservationLog,
    ReceiptStoreR,
    DesignatedStoreD,
    RelationStoreJ,
    InactivePatchSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedComponentLayout {
    components: BTreeSet<ConfigComponent>,
}

impl SharedComponentLayout {
    pub fn one_config_with(components: impl IntoIterator<Item = ConfigComponent>) -> Self {
        Self {
            components: components.into_iter().collect(),
        }
    }

    pub fn has_no_cartesian_m3_m4_wrappers(&self) -> bool {
        self.components
            == BTreeSet::from([
                ConfigComponent::OccurrenceHistory,
                ConfigComponent::AuthorityStore,
                ConfigComponent::ObservationLog,
                ConfigComponent::ReceiptStoreR,
                ConfigComponent::DesignatedStoreD,
                ConfigComponent::RelationStoreJ,
                ConfigComponent::InactivePatchSlot,
            ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WellFormed {
    Ok,
    Violation(DiagnosticCode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceKind {
    MembershipAdmitted,
    CapabilityIssued,
    WitnessIssued,
    OwnerReadModifyWrite,
    ReceiptRecorded,
    ReceiptRequested,
    ReceiptServed,
    ReceiptReplied,
    ReceiptReceived,
    ReceiptConsumedByOwner,
    DesignatedResultDecided,
    DesignatedResultDuplicate,
    DesignatedResultConsumed,
    RelationActivated,
    RelationPublished,
    ConsumerProjectionAdmitted,
    ConsumerProjection,
    PresentationGap,
    ConsumerProjectionRejected,
    RelationAuthorityRejected,
    RelationAdvanced,
    RelationReacquired,
    SaveObjectCreated,
    SaveRejected,
    AtomicCut,
    RestoreRejected,
    RestoreAccepted,
}

/// The current proof-facing patch slot is explicit and inactive.  It remains
/// part of saved semantic provenance instead of being erased by a restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchSlot {
    Inactive,
}

/// An intentionally malformed direct relationship used to exercise
/// `WellFormed` without introducing an opaque test harness state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadRelationship {
    AuthorityStoreToJ {
        relation: RelationKey,
        owner: LocusRef,
        authority_binding_epoch: BindingEpoch,
        j_binding_epoch: BindingEpoch,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicCut {
    pub cut_ref: AtomicCutRef,
    pub occurrence: OccurrenceId,
    pub owner: LocusRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutProvenance {
    occurrences: Vec<OccurrenceId>,
}

impl CutProvenance {
    pub fn ends_with(&self, occurrence: &OccurrenceId) -> bool {
        self.occurrences.last() == Some(occurrence)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEntry {
    pub kind: TraceKind,
    pub occurrence: OccurrenceId,
    pub causal_predecessor: Option<OccurrenceId>,
    pub source_ref: Option<SourceRef>,
}

/// All shared semantic state.  Presentation contexts and transformed projection
/// values stay outside this structure, making local read-side non-influence
/// inspectable rather than an implementation convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedConfig {
    pub occurrence_history: Vec<OccurrenceId>,
    pub authority_store: BTreeSet<OwnerAuthority>,
    pub observation_log: Vec<TraceEntry>,
    pub memberships: BTreeMap<LocusRef, MembershipRecord>,
    pub owner_stores: BTreeMap<LocusRef, BTreeMap<StateKey, Value>>,
    pub receipts_r: BTreeMap<ReceiptRef, ReceiptRecord>,
    receipt_requests_r: BTreeMap<ReceiptRequestRef, ReceiptExchange>,
    pub designated_results_d: BTreeMap<DesignatedResultIdentity, DesignatedResult>,
    designated_consumptions: BTreeSet<(DesignatedResultIdentity, ResultVersion)>,
    pub relation_store_j: BTreeMap<RelationKey, RelationBinding>,
    published_relations: BTreeMap<RelationKey, PublishedRelation>,
    atomic_cuts: BTreeMap<AtomicCutRef, AtomicCut>,
    patch_slot: PatchSlot,
    projection_admissions: BTreeMap<PrincipalRef, Label>,
}

impl SharedConfig {
    pub fn empty() -> Self {
        Self {
            occurrence_history: Vec::new(),
            authority_store: BTreeSet::new(),
            observation_log: Vec::new(),
            memberships: BTreeMap::new(),
            owner_stores: BTreeMap::new(),
            receipts_r: BTreeMap::new(),
            receipt_requests_r: BTreeMap::new(),
            designated_results_d: BTreeMap::new(),
            designated_consumptions: BTreeSet::new(),
            relation_store_j: BTreeMap::new(),
            published_relations: BTreeMap::new(),
            atomic_cuts: BTreeMap::new(),
            patch_slot: PatchSlot::Inactive,
            projection_admissions: BTreeMap::new(),
        }
    }

    pub fn component_layout(&self) -> SharedComponentLayout {
        SharedComponentLayout::one_config_with([
            ConfigComponent::OccurrenceHistory,
            ConfigComponent::AuthorityStore,
            ConfigComponent::ObservationLog,
            ConfigComponent::ReceiptStoreR,
            ConfigComponent::DesignatedStoreD,
            ConfigComponent::RelationStoreJ,
            ConfigComponent::InactivePatchSlot,
        ])
    }

    pub fn check_well_formed(&self) -> WellFormed {
        for authority in &self.authority_store {
            let Some(membership) = self.memberships.get(&authority.owner) else {
                return WellFormed::Violation(DiagnosticCode::StaleMembership);
            };
            if membership.principal != authority.principal
                || membership.capability != authority.capability
            {
                return WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied);
            }
            if membership.membership_epoch != authority.membership_epoch {
                return WellFormed::Violation(DiagnosticCode::StaleMembership);
            }
            if membership.lease_epoch.value() == 0 || authority.lease_epoch.value() == 0 {
                return WellFormed::Violation(DiagnosticCode::ExpiredLease);
            }
            if membership.lease_epoch != authority.lease_epoch {
                return WellFormed::Violation(DiagnosticCode::OwnerAuthorityDenied);
            }
            if authority.relation.is_none() && membership.witness != authority.witness {
                return WellFormed::Violation(DiagnosticCode::StaleWitness);
            }
        }
        for (relation, binding) in &self.relation_store_j {
            if &binding.definition.relation != relation
                || binding.current_option_index >= binding.definition.options.len()
            {
                return WellFormed::Violation(DiagnosticCode::StaleRelationLineage);
            }
            if binding.current_owner_authority.owner != binding.definition.owner
                || binding.current_owner_authority.relation.as_ref() != Some(relation)
            {
                return WellFormed::Violation(DiagnosticCode::BadRelationship);
            }
            if binding.current_owner_authority.binding_epoch != Some(binding.binding_epoch) {
                return WellFormed::Violation(DiagnosticCode::StaleRelationLineage);
            }
            let Some(membership) = self.memberships.get(&binding.definition.owner) else {
                return WellFormed::Violation(DiagnosticCode::StaleMembership);
            };
            if membership.membership_epoch != binding.current_owner_authority.membership_epoch {
                return WellFormed::Violation(DiagnosticCode::StaleMembership);
            }
            if membership.lease_epoch.value() == 0 {
                return WellFormed::Violation(DiagnosticCode::ExpiredLease);
            }
            if !self
                .authority_store
                .contains(&binding.current_owner_authority)
            {
                let related_authority = self.authority_store.iter().find(|authority| {
                    authority.principal == binding.current_owner_authority.principal
                        && authority.owner == binding.current_owner_authority.owner
                        && authority.capability == binding.current_owner_authority.capability
                        && authority.membership_epoch
                            == binding.current_owner_authority.membership_epoch
                        && authority.relation.as_ref() == Some(relation)
                        && authority.binding_epoch == Some(binding.binding_epoch)
                });
                if let Some(authority) = related_authority {
                    if authority.witness != binding.current_owner_authority.witness {
                        return WellFormed::Violation(DiagnosticCode::StaleWitness);
                    }
                    if authority.lease_epoch.value() == 0 {
                        return WellFormed::Violation(DiagnosticCode::ExpiredLease);
                    }
                }
                return WellFormed::Violation(DiagnosticCode::BadRelationship);
            }
        }
        for (relation, published) in &self.published_relations {
            let Some(binding) = self.relation_store_j.get(relation) else {
                return WellFormed::Violation(DiagnosticCode::BadRelationship);
            };
            if published.relation() != relation
                || published.binding_epoch() != binding.binding_epoch()
                || published.activation_frontier() != binding.activation_frontier()
                || published.selected_option_index() != binding.current_option_index
            {
                return WellFormed::Violation(DiagnosticCode::BadRelationship);
            }
        }
        for receipt in self.receipts_r.values() {
            if receipt.release_chain
                != [
                    TraceKind::ReceiptRequested,
                    TraceKind::ReceiptServed,
                    TraceKind::ReceiptReplied,
                    TraceKind::ReceiptReceived,
                ]
            {
                return WellFormed::Violation(DiagnosticCode::ReceiptReleaseChainInvalid);
            }
        }
        for (identity, version) in &self.designated_consumptions {
            if self
                .designated_results_d
                .get(identity)
                .is_none_or(|result| result.version != *version)
            {
                return WellFormed::Violation(DiagnosticCode::DesignatedResultMissing);
            }
        }
        for atomic_cut in self.atomic_cuts.values() {
            if !self.occurrence_history.contains(&atomic_cut.occurrence) {
                return WellFormed::Violation(DiagnosticCode::MissingAtomicCut);
            }
        }
        WellFormed::Ok
    }

    pub fn trace_len(&self) -> usize {
        self.observation_log.len()
    }

    pub fn trace_kinds_since(&self, offset: usize) -> Vec<TraceKind> {
        self.observation_log[offset..]
            .iter()
            .map(|entry| entry.kind)
            .collect()
    }

    pub fn relation_binding(&self, relation: &RelationKey) -> &RelationBinding {
        self.relation_store_j
            .get(relation)
            .expect("requested relation must be defined in SharedConfig")
    }

    pub fn published_relation(&self, relation: &RelationKey) -> Option<&PublishedRelation> {
        self.published_relations.get(relation)
    }

    pub const fn patch_slot(&self) -> PatchSlot {
        self.patch_slot
    }

    pub fn designated_consumption_count(
        &self,
        result: &ResultKey,
        frontier: &ResultFrontier,
    ) -> usize {
        self.designated_consumptions
            .iter()
            .filter(|(identity, _)| &identity.result == result && &identity.frontier == frontier)
            .count()
    }

    pub fn unchecked_with_bad_relationship(bad: BadRelationship) -> Self {
        match bad {
            BadRelationship::AuthorityStoreToJ {
                relation,
                owner,
                authority_binding_epoch,
                j_binding_epoch,
            } => {
                let mut config = Self::empty();
                let stored_authority = OwnerAuthority {
                    principal: PrincipalRef::new("unchecked-principal"),
                    owner: owner.clone(),
                    capability: CapabilityName::new("unchecked-capability"),
                    membership_epoch: MembershipEpoch::new(1),
                    lease_epoch: LeaseEpoch::new(1),
                    relation: Some(relation.clone()),
                    binding_epoch: Some(authority_binding_epoch),
                    witness: WitnessRef::new("unchecked-witness"),
                };
                let current_owner_authority = OwnerAuthority {
                    binding_epoch: Some(j_binding_epoch),
                    ..stored_authority.clone()
                };
                let definition = RelationDef::follow_with_fallback(
                    relation.clone(),
                    owner.clone(),
                    EntityRef::new("unchecked-subject"),
                    RelationOption::anchor(
                        LocusRef::new("unchecked-primary"),
                        Transform2::identity(),
                        AnchorEpoch::new(1),
                    ),
                    RelationOption::anchor(
                        LocusRef::new("unchecked-fallback"),
                        Transform2::identity(),
                        AnchorEpoch::new(1),
                    ),
                    Label::Public,
                );
                config.memberships.insert(
                    owner.clone(),
                    MembershipRecord {
                        principal: stored_authority.principal.clone(),
                        capability: stored_authority.capability.clone(),
                        membership_epoch: stored_authority.membership_epoch,
                        lease_epoch: stored_authority.lease_epoch,
                        witness: stored_authority.witness.clone(),
                    },
                );
                config.authority_store.insert(stored_authority);
                config.relation_store_j.insert(
                    relation,
                    RelationBinding {
                        definition,
                        current_option_index: 0,
                        lineage: vec![OccurrenceId::new("unchecked-binding")],
                        current_owner_authority,
                        activation_frontier: BindingActivationFrontier {
                            occurrences: vec![OccurrenceId::new("unchecked-frontier")],
                        },
                        binding_epoch: j_binding_epoch,
                    },
                );
                config
            }
        }
    }

    pub fn bad_relationship(&self) -> Option<BadRelationship> {
        self.relation_store_j
            .iter()
            .find_map(|(relation, binding)| {
                let authority_binding_epoch = binding.current_owner_authority.binding_epoch?;
                let mismatched_store_authority = self.authority_store.iter().find(|authority| {
                    authority.principal == binding.current_owner_authority.principal
                        && authority.owner == binding.current_owner_authority.owner
                        && authority.capability == binding.current_owner_authority.capability
                        && authority.membership_epoch
                            == binding.current_owner_authority.membership_epoch
                        && authority.relation.as_ref() == Some(relation)
                        && authority.binding_epoch != Some(binding.binding_epoch)
                });
                let mismatched_epoch = mismatched_store_authority
                    .and_then(|authority| authority.binding_epoch)
                    .or((authority_binding_epoch != binding.binding_epoch)
                        .then_some(authority_binding_epoch));
                mismatched_epoch.map(
                    |authority_binding_epoch| BadRelationship::AuthorityStoreToJ {
                        relation: relation.clone(),
                        owner: binding.definition.owner.clone(),
                        authority_binding_epoch,
                        j_binding_epoch: binding.binding_epoch,
                    },
                )
            })
    }

    pub fn absolute_stream_for(&self, _: &EntityRef) -> &[AbsoluteStreamEntry] {
        &[]
    }

    /// A semantic-state snapshot deliberately excludes the observational trace.
    /// Rejections append evidence but cannot mutate semantic stores.
    pub fn snapshot(&self) -> ConfigSnapshot {
        ConfigSnapshot {
            authority_store: self.authority_store.clone(),
            memberships: self.memberships.clone(),
            owner_stores: self.owner_stores.clone(),
            receipts_r: self.receipts_r.clone(),
            receipt_requests_r: self.receipt_requests_r.clone(),
            designated_results_d: self.designated_results_d.clone(),
            designated_consumptions: self.designated_consumptions.clone(),
            relation_store_j: self.relation_store_j.clone(),
            published_relations: self.published_relations.clone(),
            atomic_cuts: self.atomic_cuts.clone(),
            patch_slot: self.patch_slot,
            projection_admissions: self.projection_admissions.clone(),
        }
    }

    pub fn step(&mut self, step: Step) -> Result<StepOutcome, Diagnostic> {
        match step {
            Step::AdmitOwner {
                source_ref,
                principal,
                owner,
                capability,
                membership_epoch,
                lease_epoch,
            } => self.admit_owner(
                source_ref,
                principal,
                owner,
                capability,
                membership_epoch,
                lease_epoch,
            ),
            Step::OwnerRmw {
                source_ref,
                owner,
                authority,
                command,
            } => self.owner_rmw(source_ref, owner, authority, command),
            Step::RecordReceipt {
                source_ref,
                receipt,
                caller,
                owner,
                result,
                frontier,
                label,
            } => self.record_receipt(source_ref, receipt, caller, owner, result, frontier, label),
            Step::RequestReceipt {
                source_ref,
                request,
                caller,
                owner,
                result,
                frontier,
                label,
            } => self.request_receipt(source_ref, request, caller, owner, result, frontier, label),
            Step::ServeReceipt {
                source_ref,
                request,
                owner,
                authority,
            } => self.serve_receipt(source_ref, request, owner, authority),
            Step::ReplyReceipt {
                source_ref,
                request,
                receipt,
                value,
                label,
            } => self.reply_receipt(source_ref, request, receipt, value, label),
            Step::ReceiveReceipt {
                source_ref,
                request,
                receipt,
                caller,
                owner,
                label,
            } => self.receive_receipt(source_ref, request, receipt, caller, owner, label),
            Step::OwnerRmwWithReceipt {
                source_ref,
                owner,
                authority,
                receipt,
                command,
            } => self.owner_rmw_with_receipt(source_ref, owner, authority, receipt, command),
            Step::DesignatedDecide {
                source_ref,
                evaluator,
                result,
                frontier,
                value,
                label,
            } => self.designated_decide(source_ref, evaluator, result, frontier, value, label),
            Step::ConsumeDesignatedResult {
                source_ref,
                consumer,
                evaluator,
                result,
                frontier,
                version,
            } => self.consume_designated_result(
                source_ref, consumer, evaluator, result, frontier, version,
            ),
            Step::ActivateRelation {
                source_ref,
                definition,
                authority,
                activation_frontier,
            } => self.activate_relation(source_ref, definition, authority, activation_frontier),
            Step::PublishRelation {
                source_ref,
                relation,
                authority,
            } => self.publish_relation(source_ref, relation, authority),
            Step::AuthorizeProjection {
                source_ref,
                consumer,
                maximum_label,
            } => self.authorize_projection(source_ref, consumer, maximum_label),
            Step::ProjectRelation {
                source_ref,
                relation,
                context,
                request,
            } => self.project_relation(source_ref, relation, context, request),
            Step::ProjectPublishedRelation {
                source_ref,
                published_relation,
                context,
                request,
            } => self.project_published_relation(source_ref, published_relation, context, request),
            Step::AdvanceRelationBinding {
                source_ref,
                relation,
                authority,
                invalidation,
            } => self.advance_relation_binding(source_ref, relation, authority, invalidation),
            Step::ReacquireRelationBinding {
                source_ref,
                relation,
                authority,
                anchor,
                fresh_witness,
                binding_epoch,
                activation_frontier,
            } => self.reacquire_relation_binding(
                source_ref,
                relation,
                authority,
                anchor,
                fresh_witness,
                binding_epoch,
                activation_frontier,
            ),
            Step::AtomicCut {
                source_ref,
                owner,
                cut,
            } => self.atomic_cut(source_ref, owner, cut),
            Step::SaveFromCut {
                source_ref,
                cut_ref,
            } => self.save_from_cut(source_ref, cut_ref),
            Step::SaveAtCut { source_ref, cut } => self.save_at_cut(source_ref, cut),
        }
    }

    pub fn restore(&mut self, save: SaveObject) -> Result<(), Diagnostic> {
        if !save.is_consistent_with_cut() {
            return self.reject_restore(DiagnosticCode::MissingAtomicCut);
        }
        let reconstructed = save.reconstructed_config();
        if let WellFormed::Violation(code) = reconstructed.check_well_formed() {
            let restore_code = if code == DiagnosticCode::BadRelationship {
                DiagnosticCode::RelationAuthorityDenied
            } else {
                code
            };
            return self.reject_restore(restore_code);
        }

        self.authority_store = reconstructed.authority_store;
        self.memberships = reconstructed.memberships;
        self.owner_stores = reconstructed.owner_stores;
        self.receipts_r = reconstructed.receipts_r;
        self.receipt_requests_r = reconstructed.receipt_requests_r;
        self.designated_results_d = reconstructed.designated_results_d;
        self.designated_consumptions = reconstructed.designated_consumptions;
        self.relation_store_j = reconstructed.relation_store_j;
        self.published_relations = reconstructed.published_relations;
        self.atomic_cuts = reconstructed.atomic_cuts;
        self.patch_slot = reconstructed.patch_slot;
        self.projection_admissions = reconstructed.projection_admissions;
        self.occurrence_history = reconstructed.occurrence_history;
        self.record_trace(TraceKind::RestoreAccepted, None);
        Ok(())
    }

    fn admit_owner(
        &mut self,
        source_ref: SourceRef,
        principal: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
        membership_epoch: MembershipEpoch,
        lease_epoch: LeaseEpoch,
    ) -> Result<StepOutcome, Diagnostic> {
        let witness = WitnessRef::new(format!(
            "membership-witness:{}:{}:{}",
            principal.as_str(),
            owner.as_str(),
            membership_epoch.value()
        ));
        let authority = OwnerAuthority {
            principal: principal.clone(),
            owner: owner.clone(),
            capability: capability.clone(),
            membership_epoch,
            lease_epoch,
            relation: None,
            binding_epoch: None,
            witness: witness.clone(),
        };
        self.memberships.insert(
            owner,
            MembershipRecord {
                principal,
                capability,
                membership_epoch,
                lease_epoch,
                witness,
            },
        );
        self.authority_store.insert(authority.clone());
        self.record_trace(TraceKind::MembershipAdmitted, Some(&source_ref));
        self.record_trace(TraceKind::CapabilityIssued, Some(&source_ref));
        self.record_trace(TraceKind::WitnessIssued, Some(&source_ref));
        Ok(StepOutcome::OwnerAdmission(OwnerAdmission {
            owner_authority: authority,
        }))
    }

    fn owner_rmw(
        &mut self,
        source_ref: SourceRef,
        owner: LocusRef,
        authority: OwnerAuthority,
        command: OwnerCommand,
    ) -> Result<StepOutcome, Diagnostic> {
        if let Err(diagnostic) = self.validate_owner_authority(&owner, &authority, &source_ref) {
            return self.reject(TraceKind::RelationAuthorityRejected, source_ref, diagnostic);
        }
        let OwnerCommand::Add { state, amount } = &command;
        let Value::Int(amount) = amount else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::OwnerCommandValueMismatch, source_ref),
            );
        };
        let current = self
            .owner_stores
            .get(&owner)
            .and_then(|store| store.get(state))
            .cloned()
            .unwrap_or(Value::Int(0));
        let Value::Int(current) = current else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::OwnerCommandValueMismatch, source_ref),
            );
        };
        let Some(next) = current.checked_add(*amount) else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::OwnerCommandOverflow, source_ref),
            );
        };
        let store = self.owner_stores.entry(owner).or_default();
        store.insert(state.clone(), Value::Int(next));
        let store = store.clone();
        self.record_trace(TraceKind::OwnerReadModifyWrite, Some(&source_ref));
        Ok(StepOutcome::OwnerRmw(OwnerRmw { store }))
    }

    #[allow(clippy::too_many_arguments)]
    fn record_receipt(
        &mut self,
        source_ref: SourceRef,
        _receipt: ReceiptRef,
        _caller: PrincipalRef,
        _owner: LocusRef,
        _result: ResultKey,
        _frontier: ResultFrontier,
        _label: Label,
    ) -> Result<StepOutcome, Diagnostic> {
        self.reject(
            TraceKind::ReceiptRecorded,
            source_ref.clone(),
            Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn request_receipt(
        &mut self,
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        caller: PrincipalRef,
        owner: LocusRef,
        result: ResultKey,
        frontier: ResultFrontier,
        label: Label,
    ) -> Result<StepOutcome, Diagnostic> {
        if self.receipt_requests_r.contains_key(&request) {
            return self.reject(
                TraceKind::ReceiptRequested,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
            );
        }
        self.receipt_requests_r.insert(
            request.clone(),
            ReceiptExchange {
                request: request.clone(),
                caller,
                owner,
                result,
                frontier,
                requested_label: label,
                served: false,
                reply: None,
            },
        );
        self.record_trace(TraceKind::ReceiptRequested, Some(&source_ref));
        Ok(StepOutcome::ReceiptRequest(ReceiptRequest { request }))
    }

    fn serve_receipt(
        &mut self,
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        owner: LocusRef,
        authority: OwnerAuthority,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(exchange) = self.receipt_requests_r.get(&request).cloned() else {
            return self.reject(
                TraceKind::ReceiptServed,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptRequestMissing, source_ref),
            );
        };
        if exchange.owner != owner {
            return self.reject(
                TraceKind::ReceiptServed,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
            );
        }
        if let Err(diagnostic) = self.validate_owner_authority(&owner, &authority, &source_ref) {
            return self.reject(TraceKind::RelationAuthorityRejected, source_ref, diagnostic);
        }
        let Some(exchange) = self.receipt_requests_r.get_mut(&request) else {
            unreachable!("checked receipt request remains present before mutation");
        };
        exchange.served = true;
        self.record_trace(TraceKind::ReceiptServed, Some(&source_ref));
        Ok(StepOutcome::ReceiptServe(ReceiptServe { request }))
    }

    fn reply_receipt(
        &mut self,
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        receipt: ReceiptRef,
        value: Value,
        label: Label,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(exchange) = self.receipt_requests_r.get(&request).cloned() else {
            return self.reject(
                TraceKind::ReceiptReplied,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptRequestMissing, source_ref),
            );
        };
        if !exchange.served || exchange.reply.is_some() || label > exchange.requested_label {
            return self.reject(
                TraceKind::ReceiptReplied,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
            );
        }
        let Some(exchange) = self.receipt_requests_r.get_mut(&request) else {
            unreachable!("checked receipt request remains present before mutation");
        };
        exchange.reply = Some((receipt, value, label));
        self.record_trace(TraceKind::ReceiptReplied, Some(&source_ref));
        Ok(StepOutcome::ReceiptReply(ReceiptReply { request }))
    }

    fn receive_receipt(
        &mut self,
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        receipt: ReceiptRef,
        caller: PrincipalRef,
        owner: LocusRef,
        label: Label,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(exchange) = self.receipt_requests_r.get(&request).cloned() else {
            return self.reject(
                TraceKind::ReceiptReceived,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptRequestMissing, source_ref),
            );
        };
        let Some((replied_receipt, _value, replied_label)) = exchange.reply else {
            return self.reject(
                TraceKind::ReceiptReceived,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
            );
        };
        if !exchange.served
            || replied_receipt != receipt
            || exchange.caller != caller
            || exchange.owner != owner
            || exchange.requested_label != label
            || replied_label != label
        {
            return self.reject(
                TraceKind::ReceiptReceived,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
            );
        }
        let receipt_record = ReceiptRecord {
            request,
            receipt: receipt.clone(),
            caller,
            owner,
            result: exchange.result,
            frontier: exchange.frontier,
            label,
            release_chain: [
                TraceKind::ReceiptRequested,
                TraceKind::ReceiptServed,
                TraceKind::ReceiptReplied,
                TraceKind::ReceiptReceived,
            ],
        };
        self.receipts_r.insert(receipt, receipt_record.clone());
        self.record_trace(TraceKind::ReceiptReceived, Some(&source_ref));
        Ok(StepOutcome::Receipt(receipt_record))
    }

    fn owner_rmw_with_receipt(
        &mut self,
        source_ref: SourceRef,
        owner: LocusRef,
        authority: OwnerAuthority,
        receipt: ReceiptRef,
        command: OwnerCommand,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(receipt_record) = self.receipts_r.get(&receipt) else {
            return self.reject(
                TraceKind::ReceiptConsumedByOwner,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptRequestMissing, source_ref),
            );
        };
        if receipt_record.owner != owner {
            return self.reject(
                TraceKind::ReceiptConsumedByOwner,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ReceiptReleaseChainInvalid, source_ref),
            );
        }
        self.record_trace(TraceKind::ReceiptConsumedByOwner, Some(&source_ref));
        self.owner_rmw(source_ref, owner, authority, command)
    }

    fn designated_decide(
        &mut self,
        source_ref: SourceRef,
        evaluator: DesignatedEvaluatorRef,
        result: ResultKey,
        frontier: ResultFrontier,
        value: Value,
        label: Label,
    ) -> Result<StepOutcome, Diagnostic> {
        let identity = DesignatedResultIdentity {
            evaluator: evaluator.clone(),
            result: result.clone(),
            frontier: frontier.clone(),
        };
        if let Some(existing) = self.designated_results_d.get(&identity) {
            if existing.value == value && existing.label == label {
                let existing = existing.clone();
                self.record_trace(TraceKind::DesignatedResultDuplicate, Some(&source_ref));
                return Ok(StepOutcome::DesignatedResult(existing));
            }
            return self.reject(
                TraceKind::DesignatedResultDuplicate,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::DesignatedResultAlreadyConsumed, source_ref),
            );
        }
        let designated = DesignatedResult {
            evaluator,
            result,
            frontier,
            value,
            label,
            version: ResultVersion::new(1),
        };
        self.designated_results_d
            .insert(identity, designated.clone());
        self.record_trace(TraceKind::DesignatedResultDecided, Some(&source_ref));
        Ok(StepOutcome::DesignatedResult(designated))
    }

    #[allow(clippy::too_many_arguments)]
    fn consume_designated_result(
        &mut self,
        source_ref: SourceRef,
        consumer: PrincipalRef,
        evaluator: DesignatedEvaluatorRef,
        result: ResultKey,
        frontier: ResultFrontier,
        version: ResultVersion,
    ) -> Result<StepOutcome, Diagnostic> {
        let identity = DesignatedResultIdentity {
            evaluator,
            result,
            frontier,
        };
        let Some(designated) = self.designated_results_d.get(&identity) else {
            return self.reject(
                TraceKind::DesignatedResultConsumed,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::DesignatedResultMissing, source_ref),
            );
        };
        if designated.version != version {
            return self.reject(
                TraceKind::DesignatedResultConsumed,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::DesignatedResultMissing, source_ref),
            );
        }
        let _ = consumer;
        let consumption = (identity, version);
        if !self.designated_consumptions.insert(consumption) {
            return self.reject(
                TraceKind::DesignatedResultConsumed,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::DesignatedResultAlreadyConsumed, source_ref),
            );
        }
        self.record_trace(TraceKind::DesignatedResultConsumed, Some(&source_ref));
        Ok(StepOutcome::DesignatedConsumption(DesignatedConsumption))
    }

    fn activate_relation(
        &mut self,
        source_ref: SourceRef,
        definition: RelationDef,
        authority: OwnerAuthority,
        activation_frontier: BindingActivationFrontier,
    ) -> Result<StepOutcome, Diagnostic> {
        if let Err(diagnostic) =
            self.validate_owner_authority(&definition.owner, &authority, &source_ref)
        {
            return self.reject(TraceKind::RelationAuthorityRejected, source_ref, diagnostic);
        }
        if self.relation_store_j.contains_key(&definition.relation) {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationAlreadyDefined, source_ref),
            );
        }
        if definition.options.is_empty() {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationHasNoFallback, source_ref),
            );
        }

        let current_owner_authority = OwnerAuthority {
            principal: authority.principal,
            owner: definition.owner.clone(),
            capability: authority.capability,
            membership_epoch: authority.membership_epoch,
            lease_epoch: authority.lease_epoch,
            relation: Some(definition.relation.clone()),
            binding_epoch: Some(BindingEpoch::new(1)),
            witness: WitnessRef::new("activation-witness"),
        };
        let binding = RelationBinding {
            definition: definition.clone(),
            current_option_index: 0,
            lineage: activation_frontier.as_slice().to_vec(),
            current_owner_authority: current_owner_authority.clone(),
            activation_frontier,
            binding_epoch: BindingEpoch::new(1),
        };
        let relation = definition.relation.clone();
        self.authority_store.insert(current_owner_authority.clone());
        self.relation_store_j
            .insert(relation.clone(), binding.clone());
        self.record_trace(TraceKind::RelationActivated, Some(&source_ref));
        Ok(StepOutcome::RelationActivation(RelationActivation {
            relation,
            current_owner_authority,
            binding_state: binding,
        }))
    }

    fn publish_relation(
        &mut self,
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(binding) = self.relation_store_j.get(&relation).cloned() else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationMissing, source_ref),
            );
        };
        if let Err(diagnostic) =
            self.validate_relation_authority(&relation, &binding, &authority, &source_ref)
        {
            return self.reject(TraceKind::RelationAuthorityRejected, source_ref, diagnostic);
        }
        let published_relation = PublishedRelation::from_binding(&binding);
        self.published_relations
            .insert(relation.clone(), published_relation.clone());
        self.record_trace(TraceKind::RelationPublished, Some(&source_ref));
        Ok(StepOutcome::RelationPublication(RelationPublication {
            relation,
            published_relation,
        }))
    }

    fn authorize_projection(
        &mut self,
        source_ref: SourceRef,
        consumer: PrincipalRef,
        maximum_label: Label,
    ) -> Result<StepOutcome, Diagnostic> {
        self.projection_admissions
            .insert(consumer.clone(), maximum_label);
        self.record_trace(TraceKind::ConsumerProjectionAdmitted, Some(&source_ref));
        Ok(StepOutcome::ProjectionAdmission(ProjectionAdmission {
            consumer,
            maximum_label,
        }))
    }

    fn project_relation(
        &mut self,
        source_ref: SourceRef,
        _: RelationKey,
        _: PresentationContext,
        _: ConsumerProjectionRequest,
    ) -> Result<StepOutcome, Diagnostic> {
        self.reject(
            TraceKind::ConsumerProjectionRejected,
            source_ref.clone(),
            Diagnostic::at(DiagnosticCode::RelationPublicationRequired, source_ref),
        )
    }

    fn project_published_relation(
        &mut self,
        source_ref: SourceRef,
        published_relation: PublishedRelation,
        context: PresentationContext,
        request: ConsumerProjectionRequest,
    ) -> Result<StepOutcome, Diagnostic> {
        if !request.is_local_only() {
            let diagnostic = Diagnostic::with_projection_request(source_ref.clone(), request);
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref,
                diagnostic,
            );
        }
        let Some(stored_publication) = self.published_relations.get(published_relation.relation())
        else {
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationPublicationRequired, source_ref),
            );
        };
        if stored_publication != &published_relation {
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationPublicationRequired, source_ref),
            );
        }
        let Some(maximum_label) = self.projection_admissions.get(&context.consumer).copied() else {
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ConsumerProjectionNotAdmitted, source_ref),
            );
        };
        if context.frontier != published_relation.activation_frontier {
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(
                    DiagnosticCode::BindingActivationFrontierMismatch,
                    source_ref,
                ),
            );
        }
        if context.samples.is_empty() {
            if let Some(fallback) = context.fallback {
                let projection = Projection {
                    subject_transform: fallback.transform().clone(),
                    derived_label: published_relation.definition.declared_label,
                    presentation_only: true,
                    absolute_stream_entries: Vec::new(),
                };
                self.record_trace(TraceKind::PresentationGap, Some(&source_ref));
                return Ok(StepOutcome::Projection(projection));
            }
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::PresentationSampleMissing, source_ref),
            );
        }

        let mut derived_label = published_relation.definition.declared_label;
        for option in &published_relation.definition.options {
            let Some(sample) = context.sample_for(&option.anchor) else {
                return self.reject(
                    TraceKind::ConsumerProjectionRejected,
                    source_ref.clone(),
                    Diagnostic::at(DiagnosticCode::PresentationSampleMissing, source_ref),
                );
            };
            if sample.released_to != context.consumer {
                return self.reject(
                    TraceKind::ConsumerProjectionRejected,
                    source_ref.clone(),
                    Diagnostic::at(DiagnosticCode::PresentationSampleReleaseDenied, source_ref),
                );
            }
            if sample.frontier != context.frontier {
                return self.reject(
                    TraceKind::ConsumerProjectionRejected,
                    source_ref.clone(),
                    Diagnostic::at(
                        DiagnosticCode::BindingActivationFrontierMismatch,
                        source_ref,
                    ),
                );
            }
            if sample.anchor_epoch != option.anchor_epoch {
                return self.reject(
                    TraceKind::ConsumerProjectionRejected,
                    source_ref.clone(),
                    Diagnostic::at(DiagnosticCode::PresentationSampleEpochMismatch, source_ref),
                );
            }
            derived_label = derived_label.join(sample.label);
        }
        if derived_label > maximum_label {
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ConsumerProjectionNotAdmitted, source_ref),
            );
        }
        let selected = published_relation.current_option();
        let sample = context
            .sample_for(&selected.anchor)
            .expect("checked all relation anchors above");
        let Some(subject_transform) = sample.transform.checked_compose(&selected.offset) else {
            return self.reject(
                TraceKind::ConsumerProjectionRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::ProjectionTransformOverflow, source_ref),
            );
        };
        let projection = Projection {
            subject_transform,
            derived_label,
            presentation_only: false,
            absolute_stream_entries: Vec::new(),
        };
        self.record_trace(TraceKind::ConsumerProjection, Some(&source_ref));
        Ok(StepOutcome::Projection(projection))
    }

    fn advance_relation_binding(
        &mut self,
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
        invalidation: SemanticInvalidation,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(binding) = self.relation_store_j.get(&relation).cloned() else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationMissing, source_ref),
            );
        };
        if let Err(diagnostic) =
            self.validate_relation_authority(&relation, &binding, &authority, &source_ref)
        {
            return self.reject(TraceKind::RelationAuthorityRejected, source_ref, diagnostic);
        }
        let next_option_index = binding.current_option_index + 1;
        if next_option_index >= binding.definition.options.len() {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationHasNoFallback, source_ref),
            );
        }
        let mut next_binding = binding;
        next_binding.current_option_index = next_option_index;
        next_binding.lineage.push(invalidation.occurrence());
        self.published_relations.remove(&relation);
        self.relation_store_j.insert(relation, next_binding.clone());
        self.record_trace(TraceKind::RelationAdvanced, Some(&source_ref));
        Ok(StepOutcome::RelationAdvance(RelationAdvance {
            current_option_index: next_binding.current_option_index,
        }))
    }

    #[allow(clippy::too_many_arguments)]
    fn reacquire_relation_binding(
        &mut self,
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
        anchor: LocusRef,
        fresh_witness: WitnessRef,
        binding_epoch: BindingEpoch,
        activation_frontier: BindingActivationFrontier,
    ) -> Result<StepOutcome, Diagnostic> {
        let Some(binding) = self.relation_store_j.get(&relation).cloned() else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationMissing, source_ref),
            );
        };
        if let Err(diagnostic) =
            self.validate_relation_authority(&relation, &binding, &authority, &source_ref)
        {
            return self.reject(TraceKind::RelationAuthorityRejected, source_ref, diagnostic);
        }
        if binding_epoch <= binding.binding_epoch {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::BindingEpochDidNotAdvance, source_ref),
            );
        }
        let Some(current_option_index) = binding
            .definition
            .options
            .iter()
            .position(|option| option.anchor == anchor)
        else {
            return self.reject(
                TraceKind::RelationAuthorityRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::RelationAnchorMissing, source_ref),
            );
        };
        let mut next_binding = binding;
        let current_owner_authority = OwnerAuthority {
            relation: Some(relation.clone()),
            binding_epoch: Some(binding_epoch),
            witness: fresh_witness,
            ..authority
        };
        next_binding.current_option_index = current_option_index;
        next_binding.binding_epoch = binding_epoch;
        next_binding.activation_frontier = activation_frontier.clone();
        next_binding.current_owner_authority = current_owner_authority.clone();
        next_binding
            .lineage
            .extend(activation_frontier.as_slice().iter().cloned());
        self.authority_store.insert(current_owner_authority);
        self.published_relations.remove(&relation);
        self.relation_store_j.insert(relation, next_binding.clone());
        self.record_trace(TraceKind::RelationReacquired, Some(&source_ref));
        Ok(StepOutcome::RelationReacquire(RelationReacquire {
            current_option_index: next_binding.current_option_index,
        }))
    }

    fn atomic_cut(
        &mut self,
        source_ref: SourceRef,
        owner: LocusRef,
        cut: CutId,
    ) -> Result<StepOutcome, Diagnostic> {
        if !self.memberships.contains_key(&owner) {
            return self.reject(
                TraceKind::AtomicCut,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::MissingAtomicCut, source_ref),
            );
        }
        let cut_ref = AtomicCutRef::new(cut);
        let occurrence = self.record_trace(TraceKind::AtomicCut, Some(&source_ref));
        let atomic_cut = AtomicCut {
            cut_ref: cut_ref.clone(),
            occurrence,
            owner,
        };
        self.atomic_cuts.insert(cut_ref, atomic_cut.clone());
        Ok(StepOutcome::AtomicCut(atomic_cut))
    }

    fn save_from_cut(
        &mut self,
        source_ref: SourceRef,
        cut_ref: AtomicCutRef,
    ) -> Result<StepOutcome, Diagnostic> {
        if let WellFormed::Violation(code) = self.check_well_formed() {
            return self.reject(
                TraceKind::SaveRejected,
                source_ref.clone(),
                Diagnostic::at(code, source_ref),
            );
        }
        let Some(cut) = self.atomic_cuts.get(&cut_ref).cloned() else {
            return self.reject(
                TraceKind::SaveRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::MissingAtomicCut, source_ref),
            );
        };
        if self.occurrence_history.last() != Some(&cut.occurrence) {
            return self.reject(
                TraceKind::SaveRejected,
                source_ref.clone(),
                Diagnostic::at(DiagnosticCode::MissingAtomicCut, source_ref),
            );
        }
        let provenance = SemanticProvenance {
            cut,
            cut_history: CutProvenance {
                occurrences: self.occurrence_history.clone(),
            },
            occurrence_history: self.occurrence_history.clone(),
            authority_store: self.authority_store.clone(),
            memberships: self.memberships.clone(),
            owner_stores: self.owner_stores.clone(),
            receipts_r: self.receipts_r.clone(),
            receipt_requests_r: self.receipt_requests_r.clone(),
            designated_results_d: self.designated_results_d.clone(),
            designated_consumptions: self.designated_consumptions.clone(),
            relation_store_j: self.relation_store_j.clone(),
            published_relations: self.published_relations.clone(),
            atomic_cuts: self.atomic_cuts.clone(),
            patch_slot: self.patch_slot,
            projection_admissions: self.projection_admissions.clone(),
        };
        self.record_trace(TraceKind::SaveObjectCreated, Some(&source_ref));
        Ok(StepOutcome::SaveObject(SaveObject { provenance }))
    }

    fn save_at_cut(
        &mut self,
        source_ref: SourceRef,
        cut: CutId,
    ) -> Result<StepOutcome, Diagnostic> {
        self.save_from_cut(source_ref, AtomicCutRef::new(cut))
    }

    fn validate_owner_authority(
        &self,
        owner: &LocusRef,
        authority: &OwnerAuthority,
        source_ref: &SourceRef,
    ) -> Result<(), Diagnostic> {
        if &authority.owner != owner {
            return Err(Diagnostic::with_authority_failure(
                source_ref.clone(),
                DiagnosticCode::OwnerAuthorityDenied,
                AuthorityFailure::OwnerMismatch,
            ));
        }
        if !self.authority_store.contains(authority) {
            return Err(Diagnostic::with_authority_failure(
                source_ref.clone(),
                DiagnosticCode::OwnerAuthorityDenied,
                AuthorityFailure::UnknownAuthority,
            ));
        }
        let Some(membership) = self.memberships.get(owner) else {
            return Err(Diagnostic::with_authority_failure(
                source_ref.clone(),
                DiagnosticCode::OwnerAuthorityDenied,
                AuthorityFailure::UnknownAuthority,
            ));
        };
        if membership.principal != authority.principal
            || membership.capability != authority.capability
            || membership.membership_epoch != authority.membership_epoch
            || membership.lease_epoch != authority.lease_epoch
            || authority.lease_epoch.value() == 0
            || (authority.relation.is_none() && membership.witness != authority.witness)
        {
            return Err(Diagnostic::with_authority_failure(
                source_ref.clone(),
                DiagnosticCode::OwnerAuthorityDenied,
                AuthorityFailure::UnknownAuthority,
            ));
        }
        Ok(())
    }

    fn validate_relation_authority(
        &self,
        relation: &RelationKey,
        binding: &RelationBinding,
        authority: &OwnerAuthority,
        source_ref: &SourceRef,
    ) -> Result<(), Diagnostic> {
        if authority.relation.as_ref() != Some(relation) {
            return Err(Diagnostic::with_authority_failure(
                source_ref.clone(),
                DiagnosticCode::RelationAuthorityDenied,
                AuthorityFailure::RelationMismatch,
            ));
        }
        if authority.binding_epoch != Some(binding.binding_epoch)
            || authority.witness != binding.current_owner_authority.witness
        {
            return Err(Diagnostic::with_authority_failure(
                source_ref.clone(),
                DiagnosticCode::RelationAuthorityDenied,
                AuthorityFailure::StaleBindingEpochOrWitness,
            ));
        }
        self.validate_owner_authority(&binding.definition.owner, authority, source_ref)
            .map_err(|_| {
                Diagnostic::with_authority_failure(
                    source_ref.clone(),
                    DiagnosticCode::RelationAuthorityDenied,
                    AuthorityFailure::UnknownAuthority,
                )
            })
    }

    fn reject<T>(
        &mut self,
        trace_kind: TraceKind,
        source_ref: SourceRef,
        diagnostic: Diagnostic,
    ) -> Result<T, Diagnostic> {
        self.record_trace(trace_kind, Some(&source_ref));
        Err(diagnostic)
    }

    fn reject_restore<T>(&mut self, code: DiagnosticCode) -> Result<T, Diagnostic> {
        self.record_trace(TraceKind::RestoreRejected, None);
        Err(Diagnostic::simple(code))
    }

    fn record_trace(&mut self, kind: TraceKind, source_ref: Option<&SourceRef>) -> OccurrenceId {
        let occurrence =
            OccurrenceId::new(format!("shared-step-{}", self.occurrence_history.len()));
        let causal_predecessor = self.occurrence_history.last().cloned();
        self.occurrence_history.push(occurrence.clone());
        self.observation_log.push(TraceEntry {
            kind,
            occurrence: occurrence.clone(),
            causal_predecessor,
            source_ref: source_ref.cloned(),
        });
        occurrence
    }
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self::empty()
    }
}

/// Semantic fields only, so a diagnostic trace cannot make a rejected step look
/// like a semantic mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigSnapshot {
    authority_store: BTreeSet<OwnerAuthority>,
    memberships: BTreeMap<LocusRef, MembershipRecord>,
    owner_stores: BTreeMap<LocusRef, BTreeMap<StateKey, Value>>,
    receipts_r: BTreeMap<ReceiptRef, ReceiptRecord>,
    receipt_requests_r: BTreeMap<ReceiptRequestRef, ReceiptExchange>,
    designated_results_d: BTreeMap<DesignatedResultIdentity, DesignatedResult>,
    designated_consumptions: BTreeSet<(DesignatedResultIdentity, ResultVersion)>,
    relation_store_j: BTreeMap<RelationKey, RelationBinding>,
    published_relations: BTreeMap<RelationKey, PublishedRelation>,
    atomic_cuts: BTreeMap<AtomicCutRef, AtomicCut>,
    patch_slot: PatchSlot,
    projection_admissions: BTreeMap<PrincipalRef, Label>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerAdmission {
    pub owner_authority: OwnerAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerRmw {
    store: BTreeMap<StateKey, Value>,
}

impl OwnerRmw {
    pub fn store_value(&self, state: &StateKey) -> Option<&Value> {
        self.store.get(state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationActivation {
    pub relation: RelationKey,
    pub current_owner_authority: OwnerAuthority,
    pub binding_state: RelationBinding,
}

impl RelationActivation {
    pub const fn projected_relation(&self) -> Option<&PublishedRelation> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRequest {
    pub request: ReceiptRequestRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptServe {
    pub request: ReceiptRequestRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptReply {
    pub request: ReceiptRequestRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignatedConsumption;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationPublication {
    pub relation: RelationKey,
    pub published_relation: PublishedRelation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionAdmission {
    pub consumer: PrincipalRef,
    pub maximum_label: Label,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationAdvance {
    pub current_option_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationReacquire {
    pub current_option_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOutcome {
    OwnerAdmission(OwnerAdmission),
    OwnerRmw(OwnerRmw),
    Receipt(ReceiptRecord),
    ReceiptRequest(ReceiptRequest),
    ReceiptServe(ReceiptServe),
    ReceiptReply(ReceiptReply),
    DesignatedResult(DesignatedResult),
    DesignatedConsumption(DesignatedConsumption),
    RelationActivation(RelationActivation),
    RelationPublication(RelationPublication),
    ProjectionAdmission(ProjectionAdmission),
    Projection(Projection),
    RelationAdvance(RelationAdvance),
    RelationReacquire(RelationReacquire),
    SaveObject(SaveObject),
    AtomicCut(AtomicCut),
}

impl StepOutcome {
    pub fn expect_owner_admission(self) -> OwnerAdmission {
        match self {
            Self::OwnerAdmission(value) => value,
            other => panic!("expected owner admission outcome, got {other:?}"),
        }
    }

    pub fn expect_owner_rmw(self) -> OwnerRmw {
        match self {
            Self::OwnerRmw(value) => value,
            other => panic!("expected owner RMW outcome, got {other:?}"),
        }
    }

    pub fn expect_receipt(self) -> ReceiptRecord {
        match self {
            Self::Receipt(value) => value,
            other => panic!("expected receipt outcome, got {other:?}"),
        }
    }

    pub fn expect_receipt_request(self) -> ReceiptRequest {
        match self {
            Self::ReceiptRequest(value) => value,
            other => panic!("expected receipt request outcome, got {other:?}"),
        }
    }

    pub fn expect_receipt_serve(self) -> ReceiptServe {
        match self {
            Self::ReceiptServe(value) => value,
            other => panic!("expected receipt serve outcome, got {other:?}"),
        }
    }

    pub fn expect_receipt_reply(self) -> ReceiptReply {
        match self {
            Self::ReceiptReply(value) => value,
            other => panic!("expected receipt reply outcome, got {other:?}"),
        }
    }

    pub fn expect_designated_result(self) -> DesignatedResult {
        match self {
            Self::DesignatedResult(value) => value,
            other => panic!("expected designated result outcome, got {other:?}"),
        }
    }

    pub fn expect_designated_consumption(self) -> DesignatedConsumption {
        match self {
            Self::DesignatedConsumption(value) => value,
            other => panic!("expected designated consumption outcome, got {other:?}"),
        }
    }

    pub fn expect_relation_activation(self) -> RelationActivation {
        match self {
            Self::RelationActivation(value) => value,
            other => panic!("expected relation activation outcome, got {other:?}"),
        }
    }

    pub fn expect_relation_publication(self) -> RelationPublication {
        match self {
            Self::RelationPublication(value) => value,
            other => panic!("expected relation publication outcome, got {other:?}"),
        }
    }

    pub fn expect_projection_admission(self) -> ProjectionAdmission {
        match self {
            Self::ProjectionAdmission(value) => value,
            other => panic!("expected projection admission outcome, got {other:?}"),
        }
    }

    pub fn expect_projection(self) -> Projection {
        match self {
            Self::Projection(value) => value,
            other => panic!("expected projection outcome, got {other:?}"),
        }
    }

    pub fn expect_relation_advance(self) -> RelationAdvance {
        match self {
            Self::RelationAdvance(value) => value,
            other => panic!("expected relation advance outcome, got {other:?}"),
        }
    }

    pub fn expect_relation_reacquire(self) -> RelationReacquire {
        match self {
            Self::RelationReacquire(value) => value,
            other => panic!("expected relation reacquire outcome, got {other:?}"),
        }
    }

    pub fn expect_save_object(self) -> SaveObject {
        match self {
            Self::SaveObject(value) => value,
            other => panic!("expected save object outcome, got {other:?}"),
        }
    }

    pub fn expect_atomic_cut(self) -> AtomicCut {
        match self {
            Self::AtomicCut(value) => value,
            other => panic!("expected atomic cut outcome, got {other:?}"),
        }
    }
}

/// Checked transitions over the one shared configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    AdmitOwner {
        source_ref: SourceRef,
        principal: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
        membership_epoch: MembershipEpoch,
        lease_epoch: LeaseEpoch,
    },
    OwnerRmw {
        source_ref: SourceRef,
        owner: LocusRef,
        authority: OwnerAuthority,
        command: OwnerCommand,
    },
    OwnerRmwWithReceipt {
        source_ref: SourceRef,
        owner: LocusRef,
        authority: OwnerAuthority,
        receipt: ReceiptRef,
        command: OwnerCommand,
    },
    RecordReceipt {
        source_ref: SourceRef,
        receipt: ReceiptRef,
        caller: PrincipalRef,
        owner: LocusRef,
        result: ResultKey,
        frontier: ResultFrontier,
        label: Label,
    },
    RequestReceipt {
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        caller: PrincipalRef,
        owner: LocusRef,
        result: ResultKey,
        frontier: ResultFrontier,
        label: Label,
    },
    ServeReceipt {
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        owner: LocusRef,
        authority: OwnerAuthority,
    },
    ReplyReceipt {
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        receipt: ReceiptRef,
        value: Value,
        label: Label,
    },
    ReceiveReceipt {
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        receipt: ReceiptRef,
        caller: PrincipalRef,
        owner: LocusRef,
        label: Label,
    },
    DesignatedDecide {
        source_ref: SourceRef,
        evaluator: DesignatedEvaluatorRef,
        result: ResultKey,
        frontier: ResultFrontier,
        value: Value,
        label: Label,
    },
    ConsumeDesignatedResult {
        source_ref: SourceRef,
        consumer: PrincipalRef,
        evaluator: DesignatedEvaluatorRef,
        result: ResultKey,
        frontier: ResultFrontier,
        version: ResultVersion,
    },
    ActivateRelation {
        source_ref: SourceRef,
        definition: RelationDef,
        authority: OwnerAuthority,
        activation_frontier: BindingActivationFrontier,
    },
    PublishRelation {
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
    },
    AuthorizeProjection {
        source_ref: SourceRef,
        consumer: PrincipalRef,
        maximum_label: Label,
    },
    ProjectRelation {
        source_ref: SourceRef,
        relation: RelationKey,
        context: PresentationContext,
        request: ConsumerProjectionRequest,
    },
    ProjectPublishedRelation {
        source_ref: SourceRef,
        published_relation: PublishedRelation,
        context: PresentationContext,
        request: ConsumerProjectionRequest,
    },
    AdvanceRelationBinding {
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
        invalidation: SemanticInvalidation,
    },
    ReacquireRelationBinding {
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
        anchor: LocusRef,
        fresh_witness: WitnessRef,
        binding_epoch: BindingEpoch,
        activation_frontier: BindingActivationFrontier,
    },
    AtomicCut {
        source_ref: SourceRef,
        owner: LocusRef,
        cut: CutId,
    },
    SaveFromCut {
        source_ref: SourceRef,
        cut_ref: AtomicCutRef,
    },
    SaveAtCut {
        source_ref: SourceRef,
        cut: CutId,
    },
}

impl Step {
    pub fn admit_owner(
        source_ref: SourceRef,
        principal: PrincipalRef,
        owner: LocusRef,
        capability: CapabilityName,
        membership_epoch: MembershipEpoch,
        lease_epoch: LeaseEpoch,
    ) -> Self {
        Self::AdmitOwner {
            source_ref,
            principal,
            owner,
            capability,
            membership_epoch,
            lease_epoch,
        }
    }

    pub fn owner_rmw(
        source_ref: SourceRef,
        owner: LocusRef,
        authority: OwnerAuthority,
        command: OwnerCommand,
    ) -> Self {
        Self::OwnerRmw {
            source_ref,
            owner,
            authority,
            command,
        }
    }

    pub fn owner_rmw_with_receipt(
        source_ref: SourceRef,
        owner: LocusRef,
        authority: OwnerAuthority,
        receipt: ReceiptRef,
        command: OwnerCommand,
    ) -> Self {
        Self::OwnerRmwWithReceipt {
            source_ref,
            owner,
            authority,
            receipt,
            command,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_receipt(
        source_ref: SourceRef,
        receipt: ReceiptRef,
        caller: PrincipalRef,
        owner: LocusRef,
        result: ResultKey,
        frontier: ResultFrontier,
        label: Label,
    ) -> Self {
        Self::RecordReceipt {
            source_ref,
            receipt,
            caller,
            owner,
            result,
            frontier,
            label,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn request_receipt(
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        caller: PrincipalRef,
        owner: LocusRef,
        result: ResultKey,
        frontier: ResultFrontier,
        label: Label,
    ) -> Self {
        Self::RequestReceipt {
            source_ref,
            request,
            caller,
            owner,
            result,
            frontier,
            label,
        }
    }

    pub fn serve_receipt(
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        owner: LocusRef,
        authority: OwnerAuthority,
    ) -> Self {
        Self::ServeReceipt {
            source_ref,
            request,
            owner,
            authority,
        }
    }

    pub fn reply_receipt(
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        receipt: ReceiptRef,
        value: Value,
        label: Label,
    ) -> Self {
        Self::ReplyReceipt {
            source_ref,
            request,
            receipt,
            value,
            label,
        }
    }

    pub fn receive_receipt(
        source_ref: SourceRef,
        request: ReceiptRequestRef,
        receipt: ReceiptRef,
        caller: PrincipalRef,
        owner: LocusRef,
        label: Label,
    ) -> Self {
        Self::ReceiveReceipt {
            source_ref,
            request,
            receipt,
            caller,
            owner,
            label,
        }
    }

    pub fn designated_decide(
        source_ref: SourceRef,
        evaluator: DesignatedEvaluatorRef,
        result: ResultKey,
        frontier: ResultFrontier,
        value: Value,
        label: Label,
    ) -> Self {
        Self::DesignatedDecide {
            source_ref,
            evaluator,
            result,
            frontier,
            value,
            label,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn consume_designated_result(
        source_ref: SourceRef,
        consumer: PrincipalRef,
        evaluator: DesignatedEvaluatorRef,
        result: ResultKey,
        frontier: ResultFrontier,
        version: ResultVersion,
    ) -> Self {
        Self::ConsumeDesignatedResult {
            source_ref,
            consumer,
            evaluator,
            result,
            frontier,
            version,
        }
    }

    pub fn activate_relation(
        source_ref: SourceRef,
        definition: RelationDef,
        authority: OwnerAuthority,
        activation_frontier: BindingActivationFrontier,
    ) -> Self {
        Self::ActivateRelation {
            source_ref,
            definition,
            authority,
            activation_frontier,
        }
    }

    pub fn publish_relation(
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
    ) -> Self {
        Self::PublishRelation {
            source_ref,
            relation,
            authority,
        }
    }

    pub fn authorize_projection(
        source_ref: SourceRef,
        consumer: PrincipalRef,
        maximum_label: Label,
    ) -> Self {
        Self::AuthorizeProjection {
            source_ref,
            consumer,
            maximum_label,
        }
    }

    pub fn project_relation(
        source_ref: SourceRef,
        relation: RelationKey,
        context: PresentationContext,
    ) -> Self {
        Self::ProjectRelation {
            source_ref,
            relation,
            context,
            request: ConsumerProjectionRequest::new(),
        }
    }

    pub fn project_relation_with_request(
        source_ref: SourceRef,
        relation: RelationKey,
        context: PresentationContext,
        request: ConsumerProjectionRequest,
    ) -> Self {
        Self::ProjectRelation {
            source_ref,
            relation,
            context,
            request,
        }
    }

    pub fn project_relation_from_j(
        source_ref: SourceRef,
        relation: RelationKey,
        context: PresentationContext,
    ) -> Self {
        Self::ProjectRelation {
            source_ref,
            relation,
            context,
            request: ConsumerProjectionRequest::new(),
        }
    }

    pub fn project_published_relation(
        source_ref: SourceRef,
        published_relation: PublishedRelation,
        context: PresentationContext,
    ) -> Self {
        Self::ProjectPublishedRelation {
            source_ref,
            published_relation,
            context,
            request: ConsumerProjectionRequest::new(),
        }
    }

    pub fn project_published_relation_with_request(
        source_ref: SourceRef,
        published_relation: PublishedRelation,
        context: PresentationContext,
        request: ConsumerProjectionRequest,
    ) -> Self {
        Self::ProjectPublishedRelation {
            source_ref,
            published_relation,
            context,
            request,
        }
    }

    pub fn advance_relation_binding(
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
        invalidation: SemanticInvalidation,
    ) -> Self {
        Self::AdvanceRelationBinding {
            source_ref,
            relation,
            authority,
            invalidation,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reacquire_relation_binding(
        source_ref: SourceRef,
        relation: RelationKey,
        authority: OwnerAuthority,
        anchor: LocusRef,
        fresh_witness: WitnessRef,
        binding_epoch: BindingEpoch,
        activation_frontier: BindingActivationFrontier,
    ) -> Self {
        Self::ReacquireRelationBinding {
            source_ref,
            relation,
            authority,
            anchor,
            fresh_witness,
            binding_epoch,
            activation_frontier,
        }
    }

    pub fn save_at_cut(source_ref: SourceRef, cut: CutId) -> Self {
        Self::SaveAtCut { source_ref, cut }
    }

    pub fn atomic_cut(source_ref: SourceRef, owner: LocusRef, cut: CutId) -> Self {
        Self::AtomicCut {
            source_ref,
            owner,
            cut,
        }
    }

    pub fn save_from_cut(source_ref: SourceRef, cut_ref: AtomicCutRef) -> Self {
        Self::SaveFromCut {
            source_ref,
            cut_ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticInvalidation {
    MembershipLost {
        occurrence: OccurrenceId,
        frontier: BindingActivationFrontier,
    },
}

impl SemanticInvalidation {
    pub fn membership_lost(occurrence: OccurrenceId, frontier: BindingActivationFrontier) -> Self {
        Self::MembershipLost {
            occurrence,
            frontier,
        }
    }

    fn occurrence(self) -> OccurrenceId {
        match self {
            Self::MembershipLost { occurrence, .. } => occurrence,
        }
    }
}

/// Cut-backed semantic state with no presentation context or consumer-local
/// fallback.  Restore validation reconstructs the complete shared state from
/// this provenance before accepting it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveObject {
    provenance: SemanticProvenance,
}

impl SaveObject {
    pub fn semantic_provenance(&self) -> &SemanticProvenance {
        &self.provenance
    }

    pub const fn contains_presentation_context(&self, _: &PresentationContextId) -> bool {
        false
    }

    pub fn cut(&self) -> &CutProvenance {
        &self.provenance.cut_history
    }

    pub fn is_consistent_with_cut(&self) -> bool {
        self.provenance
            .cut_history
            .ends_with(&self.provenance.cut.occurrence)
            && self
                .provenance
                .atomic_cuts
                .get(&self.provenance.cut.cut_ref)
                == Some(&self.provenance.cut)
    }

    pub const fn patch_slot(&self) -> PatchSlot {
        self.provenance.patch_slot
    }

    pub fn with_saved_authority_relation(
        mut self,
        authority: OwnerAuthority,
        relation: RelationKey,
    ) -> Self {
        let _ = self.provenance.authority_store.remove(&authority);
        self.provenance.authority_store.insert(OwnerAuthority {
            relation: Some(relation),
            ..authority
        });
        self
    }

    pub fn with_saved_membership_epoch(
        mut self,
        owner: LocusRef,
        membership_epoch: MembershipEpoch,
    ) -> Self {
        if let Some(membership) = self.provenance.memberships.get_mut(&owner) {
            membership.membership_epoch = membership_epoch;
        }
        self
    }

    pub fn with_saved_membership_principal(
        mut self,
        owner: LocusRef,
        principal: PrincipalRef,
    ) -> Self {
        if let Some(membership) = self.provenance.memberships.get_mut(&owner) {
            membership.principal = principal;
        }
        self
    }

    pub fn with_saved_membership_capability(
        mut self,
        owner: LocusRef,
        capability: CapabilityName,
    ) -> Self {
        if let Some(membership) = self.provenance.memberships.get_mut(&owner) {
            membership.capability = capability;
        }
        self
    }

    pub fn with_saved_membership_witness(mut self, owner: LocusRef, witness: WitnessRef) -> Self {
        if let Some(membership) = self.provenance.memberships.get_mut(&owner) {
            membership.witness = witness;
        }
        self
    }

    pub fn with_saved_membership_lease_epoch(
        mut self,
        owner: LocusRef,
        lease_epoch: LeaseEpoch,
    ) -> Self {
        if let Some(membership) = self.provenance.memberships.get_mut(&owner) {
            membership.lease_epoch = lease_epoch;
        }
        self
    }

    pub fn with_saved_authority_witness(
        mut self,
        authority: OwnerAuthority,
        witness: WitnessRef,
    ) -> Self {
        let _ = self.provenance.authority_store.remove(&authority);
        self.provenance.authority_store.insert(OwnerAuthority {
            witness,
            ..authority
        });
        self
    }

    pub fn with_saved_authority_lease_epoch(
        mut self,
        authority: OwnerAuthority,
        lease_epoch: LeaseEpoch,
    ) -> Self {
        let _ = self.provenance.authority_store.remove(&authority);
        self.provenance.authority_store.insert(OwnerAuthority {
            lease_epoch,
            ..authority
        });
        self
    }

    pub fn with_saved_relation_binding_epoch(
        mut self,
        relation: RelationKey,
        binding_epoch: BindingEpoch,
    ) -> Self {
        if let Some(binding) = self.provenance.relation_store_j.get_mut(&relation) {
            binding.binding_epoch = binding_epoch;
        }
        self
    }

    pub fn with_saved_atomic_cut_occurrence(
        mut self,
        cut_ref: AtomicCutRef,
        occurrence: OccurrenceId,
    ) -> Self {
        if let Some(cut) = self.provenance.atomic_cuts.get_mut(&cut_ref) {
            cut.occurrence = occurrence;
        }
        self
    }

    pub fn reconstructed_config(&self) -> SharedConfig {
        SharedConfig {
            occurrence_history: self.provenance.occurrence_history.clone(),
            authority_store: self.provenance.authority_store.clone(),
            observation_log: Vec::new(),
            memberships: self.provenance.memberships.clone(),
            owner_stores: self.provenance.owner_stores.clone(),
            receipts_r: self.provenance.receipts_r.clone(),
            receipt_requests_r: self.provenance.receipt_requests_r.clone(),
            designated_results_d: self.provenance.designated_results_d.clone(),
            designated_consumptions: self.provenance.designated_consumptions.clone(),
            relation_store_j: self.provenance.relation_store_j.clone(),
            published_relations: self.provenance.published_relations.clone(),
            atomic_cuts: self.provenance.atomic_cuts.clone(),
            patch_slot: self.provenance.patch_slot,
            projection_admissions: self.provenance.projection_admissions.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProvenance {
    pub cut: AtomicCut,
    cut_history: CutProvenance,
    occurrence_history: Vec<OccurrenceId>,
    authority_store: BTreeSet<OwnerAuthority>,
    memberships: BTreeMap<LocusRef, MembershipRecord>,
    owner_stores: BTreeMap<LocusRef, BTreeMap<StateKey, Value>>,
    receipts_r: BTreeMap<ReceiptRef, ReceiptRecord>,
    receipt_requests_r: BTreeMap<ReceiptRequestRef, ReceiptExchange>,
    designated_results_d: BTreeMap<DesignatedResultIdentity, DesignatedResult>,
    designated_consumptions: BTreeSet<(DesignatedResultIdentity, ResultVersion)>,
    relation_store_j: BTreeMap<RelationKey, RelationBinding>,
    published_relations: BTreeMap<RelationKey, PublishedRelation>,
    atomic_cuts: BTreeMap<AtomicCutRef, AtomicCut>,
    patch_slot: PatchSlot,
    projection_admissions: BTreeMap<PrincipalRef, Label>,
}

impl SemanticProvenance {
    pub fn contains_relation(&self, relation: &RelationKey) -> bool {
        self.relation_store_j.contains_key(relation)
    }

    pub fn contains_owner_authority(&self, authority: &OwnerAuthority) -> bool {
        self.authority_store.contains(authority)
    }

    pub fn contains_binding_frontier(&self, frontier: &BindingActivationFrontier) -> bool {
        self.relation_store_j
            .values()
            .any(|binding| binding.activation_frontier() == frontier)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationSampleSpec {
    pub anchor: LocusRef,
    pub anchor_epoch: AnchorEpoch,
    pub label: Label,
}

impl PresentationSampleSpec {
    pub fn required(anchor: LocusRef, anchor_epoch: AnchorEpoch, label: Label) -> Self {
        Self {
            anchor,
            anchor_epoch,
            label,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdverseKind {
    SplitFrontierWithReleasedSamples,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextClassification {
    Coherent {
        derived_label: Label,
    },
    Rejected {
        code: DiagnosticCode,
    },
    AdverseCounterexample {
        kind: AdverseKind,
        code: DiagnosticCode,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedContextCase {
    classification: ContextClassification,
    trace_delta: Vec<TraceKind>,
    semantic_mutation_delta: Vec<ConfigComponent>,
}

impl BoundedContextCase {
    pub const fn classification(&self) -> ContextClassification {
        self.classification
    }

    pub fn trace_delta(&self) -> &[TraceKind] {
        &self.trace_delta
    }

    pub fn semantic_mutation_delta(&self) -> &[ConfigComponent] {
        &self.semantic_mutation_delta
    }
}

/// A deliberately finite enumerator.  Its result is evidence over the listed
/// sample cases, never a claim about arbitrary relation graphs or label lattices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedContextEnumerator {
    relation: RelationDef,
    consumer: PrincipalRef,
    frontier: BindingActivationFrontier,
    required_samples: Vec<PresentationSampleSpec>,
}

impl BoundedContextEnumerator {
    pub fn new(
        relation: RelationDef,
        consumer: PrincipalRef,
        frontier: BindingActivationFrontier,
        required_samples: Vec<PresentationSampleSpec>,
    ) -> Self {
        Self {
            relation,
            consumer,
            frontier,
            required_samples,
        }
    }

    pub fn enumerate(&self) -> Vec<BoundedContextCase> {
        let coherent = self.required_context("bounded-coherent");
        let mut stale_frontier = coherent.clone();
        if let Some(sample) = stale_frontier.samples.first_mut() {
            sample.frontier = BindingActivationFrontier {
                occurrences: vec![OccurrenceId::new("bounded-stale-frontier")],
            };
        }
        let mut unreleased = coherent.clone();
        if let Some(sample) = unreleased.samples.first_mut() {
            sample.released_to = PrincipalRef::new("bounded-unreleased-consumer");
        }
        vec![
            BoundedContextCase {
                classification: self.classify(&coherent),
                trace_delta: Vec::new(),
                semantic_mutation_delta: Vec::new(),
            },
            BoundedContextCase {
                classification: self.classify(&stale_frontier),
                trace_delta: Vec::new(),
                semantic_mutation_delta: Vec::new(),
            },
            BoundedContextCase {
                classification: self.classify(&unreleased),
                trace_delta: Vec::new(),
                semantic_mutation_delta: Vec::new(),
            },
        ]
    }

    pub fn explicit_adverse_counterexample(
        &self,
        kind: AdverseKind,
        context: PresentationContext,
    ) -> BoundedContextCase {
        let split_frontier = context
            .samples
            .iter()
            .any(|sample| sample.frontier != context.frontier);
        let classification = match (kind, split_frontier) {
            (AdverseKind::SplitFrontierWithReleasedSamples, true) => {
                ContextClassification::AdverseCounterexample {
                    kind,
                    code: DiagnosticCode::SplitFrameProjection,
                }
            }
            _ => self.classify(&context),
        };
        BoundedContextCase {
            classification,
            trace_delta: Vec::new(),
            semantic_mutation_delta: Vec::new(),
        }
    }

    fn required_context(&self, id: &str) -> PresentationContext {
        self.required_samples.iter().fold(
            PresentationContext::for_consumer(
                PresentationContextId::new(id),
                self.consumer.clone(),
                self.frontier.clone(),
            ),
            |context, required| {
                context.with_sample(PresentationSample::released(
                    required.anchor.clone(),
                    self.consumer.clone(),
                    self.frontier.clone(),
                    required.anchor_epoch,
                    Transform2::identity(),
                    required.label,
                ))
            },
        )
    }

    fn classify(&self, context: &PresentationContext) -> ContextClassification {
        if context.consumer != self.consumer || context.frontier != self.frontier {
            return ContextClassification::Rejected {
                code: DiagnosticCode::BindingActivationFrontierMismatch,
            };
        }
        let mut derived_label = self.relation.declared_label;
        for required in &self.required_samples {
            let Some(sample) = context.sample_for(&required.anchor) else {
                return ContextClassification::Rejected {
                    code: DiagnosticCode::PresentationSampleMissing,
                };
            };
            if sample.released_to != self.consumer {
                return ContextClassification::Rejected {
                    code: DiagnosticCode::PresentationSampleReleaseDenied,
                };
            }
            if sample.frontier != context.frontier {
                return ContextClassification::Rejected {
                    code: DiagnosticCode::BindingActivationFrontierMismatch,
                };
            }
            if sample.anchor_epoch != required.anchor_epoch {
                return ContextClassification::Rejected {
                    code: DiagnosticCode::PresentationSampleEpochMismatch,
                };
            }
            derived_label = derived_label.join(sample.label);
        }
        ContextClassification::Coherent { derived_label }
    }
}
