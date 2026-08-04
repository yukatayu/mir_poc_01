//! Finite, parser-free M4 reference model for maintained relations and late projection.
//!
//! A relation stores an owner-held binding, activation frontier, and required anchor epochs.
//! It does not store an absolute transform for its subject.  A consumer may derive that
//! transform only from released samples in one coherent presentation context.

use std::collections::{BTreeMap, BTreeSet};

use crate::evaluation_materialization::{
    AuthorityOrigin, EvalPlan, EvaluationPolicy, EvaluationSite, InputFrontier, Locus,
    Materialization, MaterializationPlan, ObservationPolicy, OperationKey, PolicyStamp, Principal,
    SemanticForm, TriggerClock,
};

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
    };
}

named_id!(EntityId);
named_id!(RelationId);
named_id!(WitnessId);

/// An anchor incarnation or lease epoch bound into one maintained relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnchorEpoch(u64);

impl AnchorEpoch {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Finite translation-only transform used by this executable reference model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform2 {
    x: i64,
    y: i64,
    debug_coordinate_fragment: String,
}

impl Transform2 {
    pub fn identity() -> Self {
        Self::translation(0, 0)
    }

    pub fn translation(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            debug_coordinate_fragment: format!("({x}, {y})"),
        }
    }

    /// Convenience composition for finite, representable test values.
    /// Projection itself uses [`Self::checked_compose`] and returns a typed diagnostic instead of
    /// saturating on overflow.
    pub fn compose(&self, offset: &Self) -> Self {
        self.checked_compose(offset)
            .expect("Transform2::compose requires representable finite coordinates")
    }

    pub fn checked_compose(&self, offset: &Self) -> Option<Self> {
        Some(Self::translation(
            self.x.checked_add(offset.x)?,
            self.y.checked_add(offset.y)?,
        ))
    }

    /// Test-visible only; M4 traces never contain this string or a transform.
    pub fn debug_coordinate_fragment(&self) -> &str {
        &self.debug_coordinate_fragment
    }
}

/// A finite observation lattice.  A derived relation label is the maximum of its declared
/// label and every released anchor sample admitted into the projection context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisibilityLabel {
    Public,
    Restricted,
    Private,
}

impl VisibilityLabel {
    fn join(self, other: Self) -> Self {
        self.max(other)
    }

    fn policy_name(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Restricted => "restricted",
            Self::Private => "private",
        }
    }
}

/// An M4-owned relation activation frontier.  It remains distinct from M3's
/// designated-result `EvalPlan.input_frontier` coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RelationActivationFrontier(InputFrontier);

impl RelationActivationFrontier {
    fn new(frontier: InputFrontier) -> Self {
        Self(frontier)
    }

    fn as_input_frontier(&self) -> &InputFrontier {
        &self.0
    }
}

/// An explicit dependency used only for finite relation-DAG admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationDependency {
    Relation(RelationId),
}

impl RelationDependency {
    fn relation_id(&self) -> &RelationId {
        match self {
            Self::Relation(relation) => relation,
        }
    }
}

/// Generic maintained relation definition with a primary and fallback anchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintainedRelationSpec {
    owner: Locus,
    subject: EntityId,
    primary_anchor: Locus,
    primary_offset: Transform2,
    fallback_anchor: Locus,
    fallback_offset: Transform2,
    activation_frontier: Option<InputFrontier>,
    required_anchor_epochs: BTreeMap<Locus, AnchorEpoch>,
    visibility_label: VisibilityLabel,
    relation_id: Option<RelationId>,
    dependencies: Vec<RelationDependency>,
}

impl MaintainedRelationSpec {
    pub fn follow_with_fallback(
        owner: Locus,
        subject: EntityId,
        primary_anchor: Locus,
        primary_offset: Transform2,
        fallback_anchor: Locus,
        fallback_offset: Transform2,
    ) -> Self {
        Self {
            owner,
            subject,
            primary_anchor,
            primary_offset,
            fallback_anchor,
            fallback_offset,
            activation_frontier: None,
            required_anchor_epochs: BTreeMap::new(),
            visibility_label: VisibilityLabel::Public,
            relation_id: None,
            dependencies: Vec::new(),
        }
    }

    pub fn with_activation_frontier(mut self, frontier: InputFrontier) -> Self {
        self.activation_frontier = Some(frontier);
        self
    }

    pub fn with_required_anchor_epoch(mut self, anchor: Locus, epoch: AnchorEpoch) -> Self {
        self.required_anchor_epochs.insert(anchor, epoch);
        self
    }

    pub fn with_visibility_label(mut self, visibility_label: VisibilityLabel) -> Self {
        self.visibility_label = visibility_label;
        self
    }

    pub fn with_relation_id(mut self, relation_id: RelationId) -> Self {
        self.relation_id = Some(relation_id);
        self
    }

    pub fn with_dependency(mut self, dependency: RelationDependency) -> Self {
        self.dependencies.push(dependency);
        self
    }
}

/// Owner-held semantic binding state.  It contains no anchor pose or subject transform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationState {
    pub owner: Locus,
    pub subject: EntityId,
    pub current_option_index: usize,
    pub current_anchor: Locus,
    pub lineage_step: u64,
    pub lineage_epoch: AnchorEpoch,
    activation_frontier: RelationActivationFrontier,
    required_anchor_epochs: BTreeMap<Locus, AnchorEpoch>,
}

/// A released presentation sample.  Its transform is not read until release, context,
/// frontier, epoch, and label checks all succeed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationSample {
    anchor: Locus,
    snapshot: String,
    binding_activation_frontier: InputFrontier,
    transform: Transform2,
    visibility_label: VisibilityLabel,
    anchor_epoch: AnchorEpoch,
    released_to: BTreeSet<Principal>,
}

impl PresentationSample {
    pub fn new(
        anchor: Locus,
        snapshot: impl Into<String>,
        binding_activation_frontier: InputFrontier,
        transform: Transform2,
        visibility_label: VisibilityLabel,
    ) -> Self {
        Self {
            anchor,
            snapshot: snapshot.into(),
            binding_activation_frontier,
            transform,
            visibility_label,
            anchor_epoch: AnchorEpoch::new(0),
            released_to: BTreeSet::new(),
        }
    }

    pub fn with_anchor_epoch(mut self, anchor_epoch: AnchorEpoch) -> Self {
        self.anchor_epoch = anchor_epoch;
        self
    }

    pub fn with_release_to(mut self, consumer: Principal) -> Self {
        self.released_to.insert(consumer);
        self
    }
}

/// Explicit consumer-local fallback used when presentation samples are absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationFallback {
    subject: EntityId,
    transform: Transform2,
}

impl PresentationFallback {
    pub fn hold_last_local(subject: EntityId, transform: Transform2) -> Self {
        Self { subject, transform }
    }
}

/// One consumer's coherent presentation context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationProjectionContext {
    consumer: Principal,
    snapshot: String,
    binding_activation_frontier: Option<InputFrontier>,
    samples: Vec<PresentationSample>,
    presentation_fallback: Option<PresentationFallback>,
}

impl RelationProjectionContext {
    pub fn for_consumer(consumer: Principal, snapshot: impl Into<String>) -> Self {
        Self {
            consumer,
            snapshot: snapshot.into(),
            binding_activation_frontier: None,
            samples: Vec::new(),
            presentation_fallback: None,
        }
    }

    pub fn with_binding_activation_frontier(mut self, frontier: InputFrontier) -> Self {
        self.binding_activation_frontier = Some(frontier);
        self
    }

    pub fn with_sample(mut self, sample: PresentationSample) -> Self {
        self.samples.push(sample);
        self
    }

    pub fn with_presentation_gap(mut self, fallback: PresentationFallback) -> Self {
        self.presentation_fallback = Some(fallback);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackDomain {
    Semantic,
    Presentation,
}

/// Semantic invalidation evidence.  It changes only owner-held binding state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticInvalidation {
    MembershipLost {
        occurrence: crate::evaluation_materialization::OccurrenceId,
        frontier: InputFrontier,
    },
    LeaseExpired {
        occurrence: crate::evaluation_materialization::OccurrenceId,
        frontier: InputFrontier,
    },
    AuthorityLost {
        occurrence: crate::evaluation_materialization::OccurrenceId,
        frontier: InputFrontier,
    },
}

impl SemanticInvalidation {
    fn frontier(&self) -> &InputFrontier {
        match self {
            Self::MembershipLost { frontier, .. }
            | Self::LeaseExpired { frontier, .. }
            | Self::AuthorityLost { frontier, .. } => frontier,
        }
    }
}

/// Observable semantic fallback/reacquire transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFallbackEvent {
    pub domain: FallbackDomain,
    pub previous_option_index: usize,
    pub current_option_index: usize,
    pub current_anchor: Locus,
    pub lineage_step: u64,
    pub lineage_epoch: AnchorEpoch,
    pub reacquired_epoch: Option<AnchorEpoch>,
    pub current_owner_authority: RelationOwnerAuthority,
}

/// Opaque authority for one exact current owner-held relation binding.  It cannot be minted
/// from a relation id: activation issues the first authority and reacquire issues a successor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationOwnerAuthority {
    relation: RelationId,
    owner: Locus,
    binding_epoch: AnchorEpoch,
    binding_witness: WitnessId,
}

impl RelationOwnerAuthority {
    fn for_binding(
        relation: RelationId,
        owner: Locus,
        binding_epoch: AnchorEpoch,
        binding_witness: WitnessId,
    ) -> Self {
        Self {
            relation,
            owner,
            binding_epoch,
            binding_witness,
        }
    }
}

/// Relation activation result with the sole current authority for its initial binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationActivation {
    pub relation: RelationId,
    pub current_owner_authority: RelationOwnerAuthority,
}

/// M4 diagnostics.  Their context intentionally omits raw transforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationDiagnosticCode {
    DuplicateRelation,
    MissingActivationFrontier,
    MissingRequiredAnchorEpoch,
    UnknownRelation,
    UnknownRelationDependency,
    RelationCycle,
    RelationAuthorityDenied,
    NoLaterSemanticFallback,
    StaleRelationWitness,
    BindingActivationFrontierMismatch,
    BindingAnchorEpochMismatch,
    SplitFrameProjection,
    MissingPresentationSample,
    PresentationSampleReleaseDenied,
    ProjectionRedactionDenied,
    MismatchedPresentationFallback,
    TransformOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationDiagnostic {
    pub code: RelationDiagnosticCode,
    pub derived_label: Option<VisibilityLabel>,
}

impl RelationDiagnostic {
    fn new(code: RelationDiagnosticCode, derived_label: Option<VisibilityLabel>) -> Self {
        Self {
            code,
            derived_label,
        }
    }

    pub fn raw_transform(&self) -> Option<&Transform2> {
        None
    }
}

/// Consumer-local projection output.  Its input frontier is M4 presentation/binding evidence;
/// both attached M3 plans keep `input_frontier: None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionOutcome {
    pub semantic_owner: Locus,
    pub consumer: Principal,
    pub subject: EntityId,
    pub subject_transform: Transform2,
    pub input_frontier: InputFrontier,
    pub derived_label: VisibilityLabel,
    pub owner_plan: EvalPlan,
    pub consumer_plan: EvalPlan,
    pub fallback_domain: Option<FallbackDomain>,
    pub presentation_only: bool,
    binding_activation_frontier: InputFrontier,
    required_anchor_epochs: BTreeMap<Locus, AnchorEpoch>,
}

impl ProjectionOutcome {
    pub fn binding_activation_frontier(&self) -> &InputFrontier {
        &self.binding_activation_frontier
    }

    pub fn required_anchor_epoch(&self, anchor: &Locus) -> Option<AnchorEpoch> {
        self.required_anchor_epochs.get(anchor).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelationTraceKind {
    Activation,
    SemanticFallback,
    Reacquire,
    Projection,
    PresentationGap,
    ProjectionDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RelationTraceEntry {
    operation_key: OperationKey,
    kind: RelationTraceKind,
    eval_plan: EvalPlan,
}

#[derive(Debug, Clone)]
struct StoredRelation {
    spec: MaintainedRelationSpec,
    state: RelationState,
    owner_plan: EvalPlan,
    current_owner_authority: Option<RelationOwnerAuthority>,
}

#[derive(Debug, Clone)]
struct ContextMetadata {
    frontier: InputFrontier,
    derived_label: VisibilityLabel,
}

/// Finite in-memory M4 reference harness.  It is not a runtime, transport, or public wire
/// contract.
#[derive(Debug, Default)]
pub struct RelationProjectionHarness {
    relations: BTreeMap<RelationId, StoredRelation>,
    consumer_authorizations: BTreeMap<Principal, VisibilityLabel>,
    accepted_witnesses: BTreeMap<RelationId, BTreeSet<WitnessId>>,
    semantic_fallback_events: BTreeMap<RelationId, Vec<SemanticFallbackEvent>>,
    owner_mutations: BTreeMap<Locus, Vec<RelationId>>,
    trace: Vec<RelationTraceEntry>,
    next_relation: u64,
    next_operation: u64,
}

impl RelationProjectionHarness {
    pub fn define_relation(
        &mut self,
        spec: MaintainedRelationSpec,
    ) -> Result<RelationId, RelationDiagnostic> {
        let generated = spec.relation_id.is_none();
        let relation = spec
            .relation_id
            .clone()
            .unwrap_or_else(|| RelationId::new(format!("relation-{}", self.next_relation)));
        self.validate_relation_graph(&[(relation.clone(), spec.clone())])?;
        self.insert_relation(relation.clone(), spec)?;
        if generated {
            self.next_relation += 1;
        }
        Ok(relation)
    }

    /// Activates one relation and returns the sole current authority for its initial binding.
    pub fn activate_relation(
        &mut self,
        spec: MaintainedRelationSpec,
    ) -> Result<RelationActivation, RelationDiagnostic> {
        let relation = self.define_relation(spec)?;
        let authority = {
            let stored = &self.relations[&relation];
            RelationOwnerAuthority::for_binding(
                relation.clone(),
                stored.state.owner.clone(),
                stored.state.lineage_epoch,
                WitnessId::new(format!("activation-witness-{}", relation.as_str())),
            )
        };
        self.relations
            .get_mut(&relation)
            .expect("relation was defined above")
            .current_owner_authority = Some(authority.clone());
        Ok(RelationActivation {
            relation,
            current_owner_authority: authority,
        })
    }

    pub fn define_relation_batch(
        &mut self,
        specs: impl IntoIterator<Item = MaintainedRelationSpec>,
    ) -> Result<Vec<RelationId>, RelationDiagnostic> {
        let mut next_relation = self.next_relation;
        let mut candidates = Vec::new();
        for spec in specs {
            let relation = match &spec.relation_id {
                Some(relation) => relation.clone(),
                None => {
                    let generated = RelationId::new(format!("relation-{next_relation}"));
                    next_relation += 1;
                    generated
                }
            };
            candidates.push((relation, spec));
        }
        self.validate_relation_graph(&candidates)?;
        let mut relations = Vec::with_capacity(candidates.len());
        for (relation, spec) in candidates {
            self.insert_relation(relation.clone(), spec)?;
            relations.push(relation);
        }
        self.next_relation = next_relation;
        Ok(relations)
    }

    pub fn authorize_consumer(
        &mut self,
        consumer: Principal,
        visibility_label: VisibilityLabel,
    ) -> Result<(), RelationDiagnostic> {
        self.consumer_authorizations
            .insert(consumer, visibility_label);
        Ok(())
    }

    pub fn relation_state(&self, relation: &RelationId) -> &RelationState {
        &self
            .relations
            .get(relation)
            .expect("test-facing relation ids are created by define_relation")
            .state
    }

    pub fn advance_semantic_fallback(
        &mut self,
        relation: &RelationId,
        authority: RelationOwnerAuthority,
        invalidated_anchor: Locus,
        invalidation: SemanticInvalidation,
    ) -> Result<SemanticFallbackEvent, RelationDiagnostic> {
        let stored = self.relations.get(relation).ok_or_else(|| {
            RelationDiagnostic::new(RelationDiagnosticCode::UnknownRelation, None)
        })?;
        if stored.current_owner_authority.as_ref() != Some(&authority) {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::RelationAuthorityDenied,
                None,
            ));
        }
        if stored.state.current_option_index != 0
            || stored.state.current_anchor != invalidated_anchor
        {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::NoLaterSemanticFallback,
                None,
            ));
        }

        let fallback_event = {
            let stored = self
                .relations
                .get_mut(relation)
                .expect("relation checked above");
            stored.state.current_option_index = 1;
            stored.state.current_anchor = stored.spec.fallback_anchor.clone();
            stored.state.lineage_step += 1;
            stored.state.activation_frontier =
                RelationActivationFrontier::new(invalidation.frontier().clone());
            SemanticFallbackEvent {
                domain: FallbackDomain::Semantic,
                previous_option_index: 0,
                current_option_index: 1,
                current_anchor: stored.spec.fallback_anchor.clone(),
                lineage_step: stored.state.lineage_step,
                lineage_epoch: stored.state.lineage_epoch,
                reacquired_epoch: None,
                current_owner_authority: stored
                    .current_owner_authority
                    .clone()
                    .expect("authority was validated before mutation"),
            }
        };
        let owner = self.relations[relation].state.owner.clone();
        self.owner_mutations
            .entry(owner)
            .or_default()
            .push(relation.clone());
        self.semantic_fallback_events
            .entry(relation.clone())
            .or_default()
            .push(fallback_event.clone());
        let operation_key = self.next_operation_key("semantic-fallback");
        let owner_plan = self.owner_plan_for(relation, operation_key.clone());
        self.trace.push(RelationTraceEntry {
            operation_key,
            kind: RelationTraceKind::SemanticFallback,
            eval_plan: owner_plan,
        });
        Ok(fallback_event)
    }

    pub fn reacquire_anchor(
        &mut self,
        relation: &RelationId,
        authority: RelationOwnerAuthority,
        anchor: Locus,
        witness: WitnessId,
        epoch: AnchorEpoch,
        frontier: InputFrontier,
    ) -> Result<SemanticFallbackEvent, RelationDiagnostic> {
        let stored = self.relations.get(relation).ok_or_else(|| {
            RelationDiagnostic::new(RelationDiagnosticCode::UnknownRelation, None)
        })?;
        if stored.current_owner_authority.as_ref() != Some(&authority) {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::RelationAuthorityDenied,
                None,
            ));
        }
        if witness.as_str().is_empty()
            || self
                .accepted_witnesses
                .get(relation)
                .is_some_and(|witnesses| witnesses.contains(&witness))
            || anchor != stored.spec.primary_anchor
            || epoch <= stored.state.lineage_epoch
        {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::StaleRelationWitness,
                None,
            ));
        }

        let reacquired = {
            let stored = self
                .relations
                .get_mut(relation)
                .expect("relation checked above");
            let previous_option_index = stored.state.current_option_index;
            stored.state.current_option_index = 0;
            stored.state.current_anchor = stored.spec.primary_anchor.clone();
            stored.state.lineage_step += 1;
            stored.state.lineage_epoch = epoch;
            stored.state.activation_frontier = RelationActivationFrontier::new(frontier);
            let successor_authority = RelationOwnerAuthority::for_binding(
                relation.clone(),
                stored.state.owner.clone(),
                epoch,
                witness.clone(),
            );
            stored.current_owner_authority = Some(successor_authority.clone());
            SemanticFallbackEvent {
                domain: FallbackDomain::Semantic,
                previous_option_index,
                current_option_index: 0,
                current_anchor: stored.spec.primary_anchor.clone(),
                lineage_step: stored.state.lineage_step,
                lineage_epoch: stored.state.lineage_epoch,
                reacquired_epoch: Some(epoch),
                current_owner_authority: successor_authority,
            }
        };
        let owner = self.relations[relation].state.owner.clone();
        self.owner_mutations
            .entry(owner)
            .or_default()
            .push(relation.clone());
        self.accepted_witnesses
            .entry(relation.clone())
            .or_default()
            .insert(witness);
        let operation_key = self.next_operation_key("reacquire");
        let owner_plan = self.owner_plan_for(relation, operation_key.clone());
        self.trace.push(RelationTraceEntry {
            operation_key,
            kind: RelationTraceKind::Reacquire,
            eval_plan: owner_plan,
        });
        Ok(reacquired)
    }

    pub fn project_for_consumer(
        &mut self,
        relation: &RelationId,
        context: RelationProjectionContext,
    ) -> Result<ProjectionOutcome, RelationDiagnostic> {
        let stored = self.relations.get(relation).ok_or_else(|| {
            RelationDiagnostic::new(RelationDiagnosticCode::UnknownRelation, None)
        })?;
        let metadata = Self::validate_context(stored, &context)?;
        let authorization = self
            .consumer_authorizations
            .get(&context.consumer)
            .copied()
            .unwrap_or(VisibilityLabel::Public);
        if authorization < metadata.derived_label {
            let operation_key = self.next_operation_key("projection-denied");
            let consumer_plan = Self::consumer_plan(
                context.consumer.clone(),
                metadata.derived_label,
                operation_key.clone(),
            );
            self.trace.push(RelationTraceEntry {
                operation_key,
                kind: RelationTraceKind::ProjectionDenied,
                eval_plan: consumer_plan,
            });
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::ProjectionRedactionDenied,
                Some(metadata.derived_label),
            ));
        }

        let selected_anchor = stored.state.current_anchor.clone();
        let selected_offset = if stored.state.current_option_index == 0 {
            stored.spec.primary_offset.clone()
        } else {
            stored.spec.fallback_offset.clone()
        };
        let owner = stored.state.owner.clone();
        let subject = stored.state.subject.clone();
        let owner_plan = stored.owner_plan.clone();
        let binding_activation_frontier =
            stored.state.activation_frontier.as_input_frontier().clone();
        let required_anchor_epochs = stored.state.required_anchor_epochs.clone();

        let (subject_transform, fallback_domain, presentation_only, trace_kind) = match context
            .samples
            .iter()
            .find(|sample| sample.anchor == selected_anchor)
        {
            Some(sample) => (
                sample
                    .transform
                    .checked_compose(&selected_offset)
                    .ok_or_else(|| {
                        RelationDiagnostic::new(RelationDiagnosticCode::TransformOverflow, None)
                    })?,
                None,
                false,
                RelationTraceKind::Projection,
            ),
            None => {
                let fallback = context.presentation_fallback.as_ref().ok_or_else(|| {
                    RelationDiagnostic::new(RelationDiagnosticCode::MissingPresentationSample, None)
                })?;
                if fallback.subject != subject {
                    return Err(RelationDiagnostic::new(
                        RelationDiagnosticCode::MismatchedPresentationFallback,
                        None,
                    ));
                }
                (
                    fallback.transform.clone(),
                    Some(FallbackDomain::Presentation),
                    true,
                    RelationTraceKind::PresentationGap,
                )
            }
        };

        let operation_key = self.next_operation_key("consumer-projection");
        let consumer_plan = Self::consumer_plan(
            context.consumer.clone(),
            metadata.derived_label,
            operation_key.clone(),
        );
        self.trace.push(RelationTraceEntry {
            operation_key,
            kind: trace_kind,
            eval_plan: consumer_plan.clone(),
        });
        Ok(ProjectionOutcome {
            semantic_owner: owner,
            consumer: context.consumer,
            subject,
            subject_transform,
            input_frontier: metadata.frontier,
            derived_label: metadata.derived_label,
            owner_plan,
            consumer_plan,
            fallback_domain,
            presentation_only,
            binding_activation_frontier,
            required_anchor_epochs,
        })
    }

    pub fn project_then_evaluate(
        &mut self,
        relation: &RelationId,
        context: RelationProjectionContext,
    ) -> Result<ProjectionOutcome, RelationDiagnostic> {
        self.project_for_consumer(relation, context)
    }

    pub fn evaluate_then_project(
        &mut self,
        relation: &RelationId,
        context: RelationProjectionContext,
    ) -> Result<ProjectionOutcome, RelationDiagnostic> {
        self.project_for_consumer(relation, context)
    }

    pub fn absolute_value_stream_for(&self, _subject: &EntityId) -> &[Transform2] {
        &[]
    }

    pub fn adapter_stream_for_subject(&self, _subject: &EntityId) -> &[Transform2] {
        &[]
    }

    pub fn semantic_fallback_events_for(&self, relation: &RelationId) -> &[SemanticFallbackEvent] {
        self.semantic_fallback_events
            .get(relation)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn owner_mutations_for(&self, owner: &Locus) -> &[RelationId] {
        self.owner_mutations
            .get(owner)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn trace_exposes_no_raw_transforms(&self) -> bool {
        self.trace
            .iter()
            .all(|entry| entry.eval_plan.semantic_form == SemanticForm::Relation)
    }

    pub fn trace_redacted_text(&self) -> String {
        self.trace
            .iter()
            .map(|entry| format!("{}:{:?}", entry.operation_key.as_str(), entry.kind))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn insert_relation(
        &mut self,
        relation: RelationId,
        spec: MaintainedRelationSpec,
    ) -> Result<(), RelationDiagnostic> {
        let activation_frontier = spec.activation_frontier.clone().ok_or_else(|| {
            RelationDiagnostic::new(RelationDiagnosticCode::MissingActivationFrontier, None)
        })?;
        for anchor in [&spec.primary_anchor, &spec.fallback_anchor] {
            if !spec.required_anchor_epochs.contains_key(anchor) {
                return Err(RelationDiagnostic::new(
                    RelationDiagnosticCode::MissingRequiredAnchorEpoch,
                    None,
                ));
            }
        }
        let operation_key = self.next_operation_key("activation");
        let owner_plan = Self::owner_plan(&spec, operation_key.clone());
        let state = RelationState {
            owner: spec.owner.clone(),
            subject: spec.subject.clone(),
            current_option_index: 0,
            current_anchor: spec.primary_anchor.clone(),
            lineage_step: 0,
            lineage_epoch: AnchorEpoch::new(1),
            activation_frontier: RelationActivationFrontier::new(activation_frontier),
            required_anchor_epochs: spec.required_anchor_epochs.clone(),
        };
        self.relations.insert(
            relation,
            StoredRelation {
                spec,
                state,
                owner_plan: owner_plan.clone(),
                current_owner_authority: None,
            },
        );
        self.trace.push(RelationTraceEntry {
            operation_key,
            kind: RelationTraceKind::Activation,
            eval_plan: owner_plan,
        });
        Ok(())
    }

    fn validate_relation_graph(
        &self,
        candidates: &[(RelationId, MaintainedRelationSpec)],
    ) -> Result<(), RelationDiagnostic> {
        let mut graph: BTreeMap<RelationId, Vec<RelationId>> = self
            .relations
            .iter()
            .map(|(relation, stored)| {
                (
                    relation.clone(),
                    stored
                        .spec
                        .dependencies
                        .iter()
                        .map(|dependency| dependency.relation_id().clone())
                        .collect(),
                )
            })
            .collect();
        for (relation, spec) in candidates {
            if graph.contains_key(relation) {
                return Err(RelationDiagnostic::new(
                    RelationDiagnosticCode::DuplicateRelation,
                    None,
                ));
            }
            graph.insert(
                relation.clone(),
                spec.dependencies
                    .iter()
                    .map(|dependency| dependency.relation_id().clone())
                    .collect(),
            );
        }
        if graph
            .values()
            .flatten()
            .any(|dependency| !graph.contains_key(dependency))
        {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::UnknownRelationDependency,
                None,
            ));
        }
        if graph_has_cycle(&graph) {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::RelationCycle,
                None,
            ));
        }
        Ok(())
    }

    fn validate_context(
        stored: &StoredRelation,
        context: &RelationProjectionContext,
    ) -> Result<ContextMetadata, RelationDiagnostic> {
        let frontier = context
            .binding_activation_frontier
            .as_ref()
            .ok_or_else(|| {
                RelationDiagnostic::new(
                    RelationDiagnosticCode::BindingActivationFrontierMismatch,
                    None,
                )
            })?;
        if frontier != stored.state.activation_frontier.as_input_frontier() {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::BindingActivationFrontierMismatch,
                None,
            ));
        }
        if !context.samples.is_empty()
            && stored.state.required_anchor_epochs.keys().any(|anchor| {
                !context
                    .samples
                    .iter()
                    .any(|sample| sample.anchor == *anchor)
            })
        {
            return Err(RelationDiagnostic::new(
                RelationDiagnosticCode::MissingPresentationSample,
                None,
            ));
        }

        let mut derived_label = stored.spec.visibility_label;
        for sample in &context.samples {
            if sample.snapshot != context.snapshot
                || sample.binding_activation_frontier != *frontier
            {
                return Err(RelationDiagnostic::new(
                    RelationDiagnosticCode::SplitFrameProjection,
                    None,
                ));
            }
            if !sample.released_to.contains(&context.consumer) {
                return Err(RelationDiagnostic::new(
                    RelationDiagnosticCode::PresentationSampleReleaseDenied,
                    None,
                ));
            }
            if let Some(required_epoch) = stored.state.required_anchor_epochs.get(&sample.anchor)
                && sample.anchor_epoch != *required_epoch
            {
                return Err(RelationDiagnostic::new(
                    RelationDiagnosticCode::BindingAnchorEpochMismatch,
                    None,
                ));
            }
            derived_label = derived_label.join(sample.visibility_label);
        }
        Ok(ContextMetadata {
            frontier: frontier.clone(),
            derived_label,
        })
    }

    fn owner_plan_for(&self, relation: &RelationId, operation_key: OperationKey) -> EvalPlan {
        Self::owner_plan(&self.relations[relation].spec, operation_key)
    }

    fn owner_plan(spec: &MaintainedRelationSpec, operation_key: OperationKey) -> EvalPlan {
        Self::relation_plan(
            operation_key,
            EvaluationSite::Owner(spec.owner.clone()),
            TriggerClock::FrontierAdvance,
            AuthorityOrigin::OwnerTransition(spec.owner.clone()),
            relation_store_materialization(),
            spec.visibility_label,
        )
    }

    fn consumer_plan(
        consumer: Principal,
        visibility_label: VisibilityLabel,
        operation_key: OperationKey,
    ) -> EvalPlan {
        Self::relation_plan(
            operation_key,
            EvaluationSite::Consumer(consumer.clone()),
            TriggerClock::PresentationFrame,
            AuthorityOrigin::Caller(consumer),
            local_only_materialization(),
            visibility_label,
        )
    }

    fn relation_plan(
        operation_key: OperationKey,
        evaluation_site: EvaluationSite,
        trigger: TriggerClock,
        authority_origin: AuthorityOrigin,
        materialization: MaterializationPlan,
        visibility_label: VisibilityLabel,
    ) -> EvalPlan {
        let evaluation_policy = EvaluationPolicy::declared_deterministic("m4-maintained-relation");
        let observation_policy = ObservationPolicy::declared(visibility_label.policy_name());
        let policy_stamp: PolicyStamp = evaluation_policy.stamp_with(&observation_policy);
        EvalPlan {
            operation_key,
            semantic_form: SemanticForm::Relation,
            evaluation_site,
            trigger,
            authority_origin,
            materialization,
            input_frontier: None,
            remote_receipt: None,
            evaluation_policy: Some(evaluation_policy),
            observation_policy: Some(observation_policy),
            policy_stamp: Some(policy_stamp),
            requires_explicit_receipt: false,
        }
    }

    fn next_operation_key(&mut self, purpose: &str) -> OperationKey {
        let operation_key = OperationKey::new(format!("m4-{purpose}-{}", self.next_operation));
        self.next_operation += 1;
        operation_key
    }
}

fn graph_has_cycle(graph: &BTreeMap<RelationId, Vec<RelationId>>) -> bool {
    fn visit(
        relation: &RelationId,
        graph: &BTreeMap<RelationId, Vec<RelationId>>,
        visiting: &mut BTreeSet<RelationId>,
        visited: &mut BTreeSet<RelationId>,
    ) -> bool {
        if visited.contains(relation) {
            return false;
        }
        if !visiting.insert(relation.clone()) {
            return true;
        }
        let cyclic = graph[relation]
            .iter()
            .any(|dependency| visit(dependency, graph, visiting, visited));
        visiting.remove(relation);
        if !cyclic {
            visited.insert(relation.clone());
        }
        cyclic
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    graph
        .keys()
        .any(|relation| visit(relation, graph, &mut visiting, &mut visited))
}

fn relation_store_materialization() -> MaterializationPlan {
    MaterializationPlan::canonical([Materialization::Store, Materialization::PublishRelation])
        .expect("the fixed M4 relation materialization is M3-admissible")
}

fn local_only_materialization() -> MaterializationPlan {
    MaterializationPlan::canonical([Materialization::LocalOnly])
        .expect("the fixed M4 local projection materialization is M3-admissible")
}
