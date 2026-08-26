//! Total M6 lowering from ordinary Surface v0 syntax into either existing M5
//! evidence, a typed front-end template, or an explicit typed diagnostic.
//!
//! This is deliberately not an M7 checker or an M8 runtime.  In particular,
//! it never turns an unsupported form into an opaque success.

use std::{collections::BTreeSet, fs, path::PathBuf};

use mir_ast::surface_v0::{
    DeferredFormKind, FixtureSource, ParseDiagnostics, ParseErrorKind, RelationPublication,
    SurfaceV0File, SurfaceV0Span, parse_surface_v0,
};

use crate::{
    evaluation_materialization::{
        EvaluationPolicy, InputFrontier, ObservationPolicy, OccurrenceId as M3OccurrenceId,
        PolicyStamp, StaticRetryContractKind,
    },
    shared_model::{
        BindingActivationFrontier, CapabilityName, Core, DiagnosticCode, Elaboration, FieldRef,
        LocusRef, OccurrenceId, OwnerCommand, PrincipalRef, ResultFrontier, ResultKey,
        ResultVersion, SourceRef, StateKey, SurfaceFragment, Value,
    },
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SurfaceV0ClassificationOptions {
    _reserved: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClassificationKind {
    OwnerRmw,
    MaintainedRelationWithFallback,
    DesignatedPublishValue,
    OwnerRmwWithRelationAndDesignated,
    DeferredOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CoreTemplateKind {
    OwnerRmw,
    OwnerLocalRead,
    OwnerLocalWrite,
    DesignatedPublishValue,
    DesignatedResultConsume,
    MaintainedRelation,
    PublishRelation,
    ConsumerLocalProjection,
    DeferredWithAuth,
    DeferredVerify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceToCoreKind {
    OwnerRmw,
    OwnerLocalRead,
    OwnerLocalWrite,
    ObserverPublish,
    DesignatedDecision,
    DesignatedResultConsume,
    PublishRelation,
    ConsumerLocalProjection,
    DeferredPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SurfaceV0DiagnosticKind {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0Diagnostic {
    kind: SurfaceV0DiagnosticKind,
    span: SurfaceV0Span,
    source_ref: SourceRef,
    m5_code: DiagnosticCode,
}

impl SurfaceV0Diagnostic {
    fn new(kind: SurfaceV0DiagnosticKind, span: SurfaceV0Span, m5_code: DiagnosticCode) -> Self {
        let source_ref = source_ref_from_span(&span);
        Self {
            kind,
            span,
            source_ref,
            m5_code,
        }
    }

    pub const fn kind(&self) -> SurfaceV0DiagnosticKind {
        self.kind
    }

    pub fn span(&self) -> &SurfaceV0Span {
        &self.span
    }

    pub fn source_ref(&self) -> &SourceRef {
        &self.source_ref
    }

    pub const fn m5_code(&self) -> DiagnosticCode {
        self.m5_code
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0Diagnostics {
    entries: Vec<SurfaceV0Diagnostic>,
}

impl SurfaceV0Diagnostics {
    pub fn primary(&self) -> &SurfaceV0Diagnostic {
        self.entries
            .first()
            .expect("SurfaceV0Diagnostics always has a primary diagnostic")
    }

    pub fn by_kind(&self, kind: SurfaceV0DiagnosticKind) -> Option<&SurfaceV0Diagnostic> {
        self.entries
            .iter()
            .find(|diagnostic| diagnostic.kind == kind)
    }

    pub fn entries(&self) -> &[SurfaceV0Diagnostic] {
        &self.entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreTemplate {
    kind: CoreTemplateKind,
    name: String,
    result_name: Option<String>,
    source_span: SurfaceV0Span,
    m5_core: Option<Core>,
    result_frontier: Option<ResultFrontier>,
    input_frontier: Option<InputFrontier>,
    result_version: Option<ResultVersion>,
    observation_policy: Option<ObservationPolicy>,
    policy_stamp: Option<PolicyStamp>,
    static_retry_contract: Option<StaticRetryContractKind>,
    binding_frontier: Option<BindingActivationFrontier>,
    owner_publication_kind: Option<CoreTemplateKind>,
    published_relation_carrier: bool,
    consumer_projection_locus: Option<String>,
    consumer_projection_kind: Option<CoreTemplateKind>,
    authority_requirement: Option<String>,
    consumer_locus: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DesignatedResultMetadata {
    result_frontier: ResultFrontier,
    input_frontier: InputFrontier,
    result_version: ResultVersion,
    observation_policy: ObservationPolicy,
    policy_stamp: PolicyStamp,
}

impl CoreTemplate {
    fn owner_rmw(name: String, source_span: SurfaceV0Span, m5_core: Core) -> Self {
        Self {
            kind: CoreTemplateKind::OwnerRmw,
            name,
            result_name: None,
            source_span,
            m5_core: Some(m5_core),
            result_frontier: None,
            input_frontier: None,
            result_version: None,
            observation_policy: None,
            policy_stamp: None,
            static_retry_contract: None,
            binding_frontier: None,
            owner_publication_kind: None,
            published_relation_carrier: false,
            consumer_projection_locus: None,
            consumer_projection_kind: None,
            authority_requirement: None,
            consumer_locus: None,
        }
    }

    fn designated(
        evaluator: String,
        result: String,
        source_span: SurfaceV0Span,
        metadata: DesignatedResultMetadata,
    ) -> Self {
        Self {
            kind: CoreTemplateKind::DesignatedPublishValue,
            name: evaluator,
            result_name: Some(result),
            source_span,
            m5_core: None,
            result_frontier: Some(metadata.result_frontier),
            input_frontier: Some(metadata.input_frontier),
            result_version: Some(metadata.result_version),
            observation_policy: Some(metadata.observation_policy),
            policy_stamp: Some(metadata.policy_stamp),
            static_retry_contract: None,
            binding_frontier: None,
            owner_publication_kind: None,
            published_relation_carrier: false,
            consumer_projection_locus: None,
            consumer_projection_kind: None,
            authority_requirement: None,
            consumer_locus: None,
        }
    }

    fn relation(
        name: String,
        source_span: SurfaceV0Span,
        binding_frontier: BindingActivationFrontier,
        consumer_projection_locus: Option<String>,
    ) -> Self {
        Self {
            kind: CoreTemplateKind::MaintainedRelation,
            name,
            result_name: None,
            source_span,
            m5_core: None,
            result_frontier: None,
            input_frontier: None,
            result_version: None,
            observation_policy: None,
            policy_stamp: None,
            static_retry_contract: None,
            binding_frontier: Some(binding_frontier),
            owner_publication_kind: Some(CoreTemplateKind::PublishRelation),
            published_relation_carrier: true,
            consumer_projection_kind: consumer_projection_locus
                .as_ref()
                .map(|_| CoreTemplateKind::ConsumerLocalProjection),
            consumer_projection_locus,
            authority_requirement: None,
            consumer_locus: None,
        }
    }

    fn deferred(kind: CoreTemplateKind, name: String, source_span: SurfaceV0Span) -> Self {
        let authority_requirement =
            (kind == CoreTemplateKind::DeferredWithAuth).then_some(name.clone());
        Self {
            kind,
            name,
            result_name: None,
            source_span,
            m5_core: None,
            result_frontier: None,
            input_frontier: None,
            result_version: None,
            observation_policy: None,
            policy_stamp: None,
            static_retry_contract: None,
            binding_frontier: None,
            owner_publication_kind: None,
            published_relation_carrier: false,
            consumer_projection_locus: None,
            consumer_projection_kind: None,
            authority_requirement,
            consumer_locus: None,
        }
    }

    fn designated_result_consume(
        evaluator: String,
        result: String,
        consumer_locus: String,
        source_span: SurfaceV0Span,
        producer: &CoreTemplate,
    ) -> Self {
        Self {
            kind: CoreTemplateKind::DesignatedResultConsume,
            name: evaluator,
            result_name: Some(result),
            source_span,
            m5_core: None,
            result_frontier: Some(producer.result_frontier().clone()),
            input_frontier: Some(producer.input_frontier().clone()),
            result_version: Some(producer.result_version()),
            observation_policy: Some(producer.observation_policy().clone()),
            policy_stamp: Some(producer.policy_stamp().clone()),
            static_retry_contract: Some(StaticRetryContractKind::ReturnExistingNoNewConsumption),
            binding_frontier: None,
            owner_publication_kind: None,
            published_relation_carrier: false,
            consumer_projection_locus: None,
            consumer_projection_kind: None,
            authority_requirement: None,
            consumer_locus: Some(consumer_locus),
        }
    }

    pub const fn kind(&self) -> CoreTemplateKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source_span(&self) -> &SurfaceV0Span {
        &self.source_span
    }

    pub fn to_m5_core(&self) -> Option<Core> {
        self.m5_core.clone()
    }

    pub const fn is_non_executable(&self) -> bool {
        matches!(
            self.kind,
            CoreTemplateKind::DeferredWithAuth | CoreTemplateKind::DeferredVerify
        )
    }

    pub fn authority_requirement(&self) -> Option<&str> {
        self.authority_requirement.as_deref()
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

    pub fn evaluator(&self) -> &str {
        &self.name
    }

    pub fn result(&self) -> &str {
        self.result_name
            .as_deref()
            .expect("only designated templates retain a result name")
    }

    pub fn consumer_locus(&self) -> &str {
        self.consumer_locus
            .as_deref()
            .expect("only designated consumers retain a consumer locus")
    }

    pub fn consumes_designated_result(&self) -> bool {
        self.kind == CoreTemplateKind::DesignatedResultConsume
    }

    pub const fn emits_evaluator_expression(&self) -> bool {
        false
    }

    pub fn result_frontier(&self) -> &ResultFrontier {
        self.result_frontier
            .as_ref()
            .expect("only designated templates carry a result frontier")
    }

    pub fn input_frontier(&self) -> &InputFrontier {
        self.input_frontier
            .as_ref()
            .expect("valid designated templates carry an input frontier")
    }

    pub fn result_version(&self) -> ResultVersion {
        self.result_version
            .expect("only designated templates carry a result version")
    }

    pub fn observation_policy(&self) -> &ObservationPolicy {
        self.observation_policy
            .as_ref()
            .expect("valid designated templates carry an observation policy")
    }

    pub fn policy_stamp(&self) -> &PolicyStamp {
        self.policy_stamp
            .as_ref()
            .expect("valid designated templates carry a policy stamp")
    }

    pub fn static_retry_contract(&self) -> StaticRetryContractKind {
        self.static_retry_contract
            .expect("valid designated result consumer carries a retry contract")
    }

    pub const fn preserves_duplicate_version(&self) -> bool {
        self.result_version.is_some()
    }

    pub fn binding_frontier(&self) -> &BindingActivationFrontier {
        self.binding_frontier
            .as_ref()
            .expect("only relation templates carry a binding frontier")
    }

    pub fn owner_publication_kind(&self) -> CoreTemplateKind {
        self.owner_publication_kind
            .expect("only relation templates have owner publication")
    }

    pub fn published_relation_carrier(&self) -> Option<()> {
        self.published_relation_carrier.then_some(())
    }

    pub fn consumer_projection_locus(&self) -> Option<&str> {
        self.consumer_projection_locus.as_deref()
    }

    pub fn consumer_projection_kind(&self) -> CoreTemplateKind {
        self.consumer_projection_kind
            .expect("only relations with a consumer projection have its template")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceToCoreEntry {
    source_span: SurfaceV0Span,
    kind: SourceToCoreKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceToCoreMap {
    entries: Vec<SourceToCoreEntry>,
}

impl SourceToCoreMap {
    fn add(&mut self, source_span: SurfaceV0Span, kind: SourceToCoreKind) {
        self.entries.push(SourceToCoreEntry { source_span, kind });
    }

    pub fn entries_for_span(&self, span: &SurfaceV0Span) -> SourceToCoreEntries {
        SourceToCoreEntries {
            entries: self
                .entries
                .iter()
                .filter(|entry| &entry.source_span == span)
                .cloned()
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceToCoreEntries {
    entries: Vec<SourceToCoreEntry>,
}

impl SourceToCoreEntries {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn kinds(&self) -> Vec<SourceToCoreKind> {
        self.entries.iter().map(|entry| entry.kind).collect()
    }

    pub fn all_source_spans_equal(&self, span: &SurfaceV0Span) -> bool {
        self.entries.iter().all(|entry| &entry.source_span == span)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerAtLocus {
    locus: String,
    mints_authority: bool,
}

impl InnerAtLocus {
    pub const fn does_not_mint_authority(&self) -> bool {
        !self.mints_authority
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityAudit {
    event: String,
    inner_at_loci: Vec<InnerAtLocus>,
    required_authority: BTreeSet<String>,
}

impl AuthorityAudit {
    pub fn inner_at_locus(&self, locus: &str) -> &InnerAtLocus {
        self.inner_at_loci
            .iter()
            .find(|entry| entry.locus == locus)
            .expect("classification records every nested at evaluation locus")
    }

    pub fn required_authority(&self) -> &BTreeSet<String> {
        &self.required_authority
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0Classification {
    kind: ClassificationKind,
    template_names: Vec<String>,
    root_source_ref: SourceRef,
    source_refs: Vec<(SurfaceV0Span, SourceRef)>,
    source_to_core_map: SourceToCoreMap,
    core_templates: Vec<CoreTemplate>,
    designated_templates: Vec<CoreTemplate>,
    designated_result_consumer_templates: Vec<CoreTemplate>,
    relation_templates: Vec<CoreTemplate>,
    deferred_templates: Vec<CoreTemplate>,
    authority_audits: Vec<AuthorityAudit>,
}

impl SurfaceV0Classification {
    pub const fn kind(&self) -> ClassificationKind {
        self.kind
    }

    /// Stable names derived from the accepted Core templates, rather than
    /// re-parsed Surface syntax.
    pub fn template_names(&self) -> &[String] {
        &self.template_names
    }

    pub fn root_source_ref(&self) -> &SourceRef {
        &self.root_source_ref
    }

    pub fn source_ref_for_span(&self, span: &SurfaceV0Span) -> Option<&SourceRef> {
        self.source_refs
            .iter()
            .find_map(|(candidate, source_ref)| (candidate == span).then_some(source_ref))
    }

    pub fn source_to_core_map(&self) -> &SourceToCoreMap {
        &self.source_to_core_map
    }

    pub fn core_template(&self, name: &str) -> Option<&CoreTemplate> {
        self.core_templates
            .iter()
            .find(|template| template.name == name)
    }

    pub fn designated_template(&self, evaluator: &str, result: &str) -> Option<&CoreTemplate> {
        self.designated_templates.iter().find(|template| {
            template.name == evaluator && template.result_name.as_deref() == Some(result)
        })
    }

    pub fn designated_result_consumer_template(
        &self,
        evaluator: &str,
        result: &str,
        consumer_locus: &str,
    ) -> Option<&CoreTemplate> {
        self.designated_result_consumer_templates
            .iter()
            .find(|template| {
                template.evaluator() == evaluator
                    && template.result() == result
                    && template.consumer_locus() == consumer_locus
            })
    }

    pub fn relation_template(&self, name: &str) -> Option<&CoreTemplate> {
        self.relation_templates
            .iter()
            .find(|template| template.name == name)
    }

    pub fn deferred_template(&self, kind: CoreTemplateKind, name: &str) -> Option<&CoreTemplate> {
        self.deferred_templates
            .iter()
            .find(|template| template.kind == kind && template.name == name)
    }

    pub fn authority_audit(&self, event: &str) -> Option<&AuthorityAudit> {
        self.authority_audits
            .iter()
            .find(|audit| audit.event == event)
    }
}

/// Classifies every M6 constructor.  Successful classification is always a
/// concrete M5 Core or a named, typed CoreTemplate; no branch is a generic
/// acceptance token.
pub fn classify_surface_v0(
    ast: &SurfaceV0File,
    _: SurfaceV0ClassificationOptions,
) -> Result<SurfaceV0Classification, SurfaceV0Diagnostics> {
    let assignment_target_diagnostics = assignment_target_diagnostics(ast);
    if !assignment_target_diagnostics.is_empty() {
        return Err(SurfaceV0Diagnostics {
            entries: assignment_target_diagnostics,
        });
    }
    let mut diagnostics = name_resolution_diagnostics(ast);
    diagnostics.extend(relation_diagnostics(ast));
    diagnostics.extend(cross_owner_diagnostics(ast));
    if !diagnostics.is_empty() {
        return Err(SurfaceV0Diagnostics {
            entries: diagnostics,
        });
    }

    let root_source_ref = source_ref_from_span(ast.root().span());
    let mut source_refs = vec![(ast.root().span().clone(), root_source_ref.clone())];
    let mut source_to_core_map = SourceToCoreMap::default();
    let required_authority = ast
        .deferred_forms()
        .entries()
        .iter()
        .filter(|form| form.kind() == DeferredFormKind::WithAuth)
        .map(|form| form.name().to_string())
        .collect::<BTreeSet<_>>();

    let capability = CapabilityName::new(
        required_authority
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| "MembershipAuth".to_string()),
    );
    let mut core_templates = Vec::new();
    let mut authority_audits = Vec::new();
    for assignment in ast.assignments() {
        let source_ref = source_ref_from_span(assignment.span());
        source_refs.push((assignment.span().clone(), source_ref.clone()));
        let field = assignment
            .target()
            .field()
            .expect("M6 assignment grammar requires a field target");
        let command = OwnerCommand::add(
            StateKey::field(assignment.target().base(), FieldRef::new(field)),
            Value::int(-1),
        );
        let core = Core::same_owner_rmw(
            source_ref,
            PrincipalRef::new(assignment.actor()),
            LocusRef::new(assignment.owner_locus()),
            command,
            capability.clone(),
        );
        source_to_core_map.add(assignment.span().clone(), SourceToCoreKind::OwnerRmw);
        source_to_core_map.add(assignment.span().clone(), SourceToCoreKind::OwnerLocalRead);
        source_to_core_map.add(assignment.span().clone(), SourceToCoreKind::OwnerLocalWrite);
        core_templates.push(CoreTemplate::owner_rmw(
            assignment.event().to_string(),
            assignment.span().clone(),
            core,
        ));
        authority_audits.push(AuthorityAudit {
            event: assignment.event().to_string(),
            inner_at_loci: vec![InnerAtLocus {
                locus: assignment.owner_locus().to_string(),
                mints_authority: false,
            }],
            required_authority: required_authority.clone(),
        });
    }

    let mut designated_templates = Vec::new();
    for designated in ast.designated_results() {
        let span = designated.span().clone();
        source_refs.push((span.clone(), source_ref_from_span(&span)));
        let frontier =
            ResultFrontier::from_ordered_results(vec![ResultKey::new(designated.tick_frontier())])
                .expect("one designated tick frontier is finite and nonempty");
        let input_frontier = InputFrontier::from_ordered_producers(vec![M3OccurrenceId::new(
            designated.tick_frontier(),
        )])
        .expect("one designated input frontier is finite and nonempty");
        let evaluation_policy = EvaluationPolicy::declared_deterministic(format!(
            "inferred:{}.{}",
            designated.evaluator(),
            designated.result()
        ));
        let observation_policy = ObservationPolicy::declared("conservative");
        let policy_stamp = evaluation_policy.stamp_with(&observation_policy);
        source_to_core_map.add(span.clone(), SourceToCoreKind::DesignatedDecision);
        designated_templates.push(CoreTemplate::designated(
            designated.evaluator().to_string(),
            designated.result().to_string(),
            span,
            DesignatedResultMetadata {
                result_frontier: frontier,
                input_frontier,
                result_version: ResultVersion::new(1),
                observation_policy,
                policy_stamp,
            },
        ));
    }

    let mut designated_result_consumer_templates = Vec::new();
    for consumer in ast.designated_result_consumers() {
        let span = consumer.span().clone();
        source_refs.push((span.clone(), source_ref_from_span(&span)));
        source_to_core_map.add(span.clone(), SourceToCoreKind::DesignatedResultConsume);
        let Some(producer) = designated_templates.iter().find(|template| {
            template.evaluator() == consumer.evaluator() && template.result() == consumer.result()
        }) else {
            return Err(SurfaceV0Diagnostics {
                entries: vec![SurfaceV0Diagnostic::new(
                    SurfaceV0DiagnosticKind::UnresolvedName,
                    consumer.result_ref_span().clone(),
                    DiagnosticCode::BadRelationship,
                )],
            });
        };
        designated_result_consumer_templates.push(CoreTemplate::designated_result_consume(
            consumer.evaluator().to_string(),
            consumer.result().to_string(),
            consumer.consumer_locus().to_string(),
            span,
            producer,
        ));
    }

    let mut relation_templates = Vec::new();
    for relation in ast.relations() {
        let span = relation.span().clone();
        source_refs.push((span.clone(), source_ref_from_span(&span)));
        let frontier =
            BindingActivationFrontier::from_ordered_occurrences(vec![OccurrenceId::new(
                relation.binding_frontier(),
            )])
            .expect("one relation binding frontier is finite and nonempty");
        source_to_core_map.add(span.clone(), SourceToCoreKind::PublishRelation);
        if relation.consumer_projection_locus().is_some() {
            source_to_core_map.add(span.clone(), SourceToCoreKind::ConsumerLocalProjection);
        }
        relation_templates.push(CoreTemplate::relation(
            relation.name().to_string(),
            span,
            frontier,
            relation.consumer_projection_locus().map(str::to_string),
        ));
    }

    let mut deferred_templates = Vec::new();
    for form in ast.deferred_forms().entries() {
        source_refs.push((form.span().clone(), source_ref_from_span(form.span())));
        source_to_core_map.add(form.span().clone(), SourceToCoreKind::DeferredPolicy);
        deferred_templates.push(CoreTemplate::deferred(
            match form.kind() {
                DeferredFormKind::WithAuth => CoreTemplateKind::DeferredWithAuth,
                DeferredFormKind::Verify => CoreTemplateKind::DeferredVerify,
            },
            form.name().to_string(),
            form.span().clone(),
        ));
    }

    let kind = classification_kind(
        !core_templates.is_empty(),
        !relation_templates.is_empty(),
        !designated_templates.is_empty(),
    );
    let mut template_names = core_templates
        .iter()
        .chain(designated_templates.iter())
        .chain(designated_result_consumer_templates.iter())
        .chain(relation_templates.iter())
        .chain(deferred_templates.iter())
        .map(|template| template.name().to_string())
        .collect::<Vec<_>>();
    template_names.sort();
    template_names.dedup();
    Ok(SurfaceV0Classification {
        kind,
        template_names,
        root_source_ref,
        source_refs,
        source_to_core_map,
        core_templates,
        designated_templates,
        designated_result_consumer_templates,
        relation_templates,
        deferred_templates,
        authority_audits,
    })
}

fn classification_kind(
    has_owner_rmw: bool,
    has_relation: bool,
    has_designated: bool,
) -> ClassificationKind {
    match (has_owner_rmw, has_relation, has_designated) {
        (true, true, true) => ClassificationKind::OwnerRmwWithRelationAndDesignated,
        (_, true, _) => ClassificationKind::MaintainedRelationWithFallback,
        (_, _, true) => ClassificationKind::DesignatedPublishValue,
        (true, false, false) => ClassificationKind::OwnerRmw,
        (false, false, false) => ClassificationKind::DeferredOnly,
    }
}

fn name_resolution_diagnostics(ast: &SurfaceV0File) -> Vec<SurfaceV0Diagnostic> {
    let type_names = ast
        .types()
        .iter()
        .map(|declaration| declaration.name())
        .collect::<BTreeSet<_>>();
    let mut diagnostics = Vec::new();
    for role in ast.roles() {
        for when in role.whens() {
            for parameter in when.parameters() {
                if parameter.type_name() != "Int" && !type_names.contains(parameter.type_name()) {
                    diagnostics.push(SurfaceV0Diagnostic::new(
                        SurfaceV0DiagnosticKind::UnresolvedName,
                        parameter.type_span().clone(),
                        DiagnosticCode::BadRelationship,
                    ));
                }
            }
        }
    }
    for locus in ast.loci() {
        if type_names.contains(locus.name()) {
            diagnostics.push(SurfaceV0Diagnostic::new(
                SurfaceV0DiagnosticKind::AmbiguousName,
                locus.name_span().clone(),
                DiagnosticCode::BadRelationship,
            ));
        }
    }
    for assignment in ast.assignments() {
        for reference in assignment.rhs_references() {
            if ast.state(reference.base()).is_none() {
                diagnostics.push(SurfaceV0Diagnostic::new(
                    SurfaceV0DiagnosticKind::UnresolvedName,
                    reference.span().clone(),
                    DiagnosticCode::BadRelationship,
                ));
            }
        }
    }
    diagnostics
}

fn relation_diagnostics(ast: &SurfaceV0File) -> Vec<SurfaceV0Diagnostic> {
    let mut diagnostics = Vec::new();
    for relation in ast.relations() {
        if let RelationPublication::Value { span, .. } = relation.publication() {
            diagnostics.push(SurfaceV0Diagnostic::new(
                SurfaceV0DiagnosticKind::RelationMustPublishRelationCarrier,
                span.clone(),
                DiagnosticCode::RelationPublicationRequired,
            ));
        }
    }
    for mutation in ast.relation_mutations() {
        diagnostics.push(SurfaceV0Diagnostic::new(
            SurfaceV0DiagnosticKind::ConsumerRelationMutationDenied,
            mutation.span().clone(),
            DiagnosticCode::ConsumerProjectionMaterializationDenied,
        ));
    }
    diagnostics
}

fn assignment_target_diagnostics(ast: &SurfaceV0File) -> Vec<SurfaceV0Diagnostic> {
    let mut diagnostics = Vec::new();
    for assignment in ast.assignments() {
        let target = assignment.target();
        if target.field().is_none() {
            diagnostics.push(SurfaceV0Diagnostic::new(
                SurfaceV0DiagnosticKind::FieldlessAssignmentTarget,
                target.span().clone(),
                DiagnosticCode::FieldlessAssignmentTarget,
            ));
            continue;
        }
        let Some(state) = ast.state(target.base()) else {
            diagnostics.push(SurfaceV0Diagnostic::new(
                SurfaceV0DiagnosticKind::UnresolvedName,
                target.span().clone(),
                DiagnosticCode::BadRelationship,
            ));
            continue;
        };
        if state.owner_locus() != assignment.owner_locus() {
            diagnostics.push(SurfaceV0Diagnostic::new(
                SurfaceV0DiagnosticKind::CrossOwnerWriteTargetOutsideActionLocus,
                target.span().clone(),
                DiagnosticCode::CrossOwnerWriteTargetOutsideActionLocus,
            ));
        }
    }
    diagnostics
}

fn cross_owner_diagnostics(ast: &SurfaceV0File) -> Vec<SurfaceV0Diagnostic> {
    let mut diagnostics = Vec::new();
    for assignment in ast.assignments() {
        for reference in assignment.rhs_references() {
            let Some(state) = ast.state(reference.base()) else {
                continue;
            };
            if state.owner_locus() != assignment.owner_locus() {
                let m5_code = match SurfaceFragment::cross_owner_read_without_receipt(
                    source_ref_from_span(reference.span()),
                    PrincipalRef::new(assignment.actor()),
                    LocusRef::new(assignment.owner_locus()),
                    LocusRef::new(state.owner_locus()),
                    StateKey::field(
                        reference.base(),
                        FieldRef::new(
                            reference
                                .field()
                                .expect("M6 cross-owner field references are explicit"),
                        ),
                    ),
                )
                .elaborate()
                {
                    Elaboration::Diagnostic(diagnostic) => diagnostic.code(),
                    Elaboration::Core(_) => {
                        unreachable!("M5 cross-owner receipt-free fragment always rejects")
                    }
                };
                diagnostics.push(SurfaceV0Diagnostic::new(
                    SurfaceV0DiagnosticKind::CrossOwnerOperandRequiresReceipt,
                    reference.span().clone(),
                    m5_code,
                ));
            }
        }
    }
    diagnostics
}

fn source_ref_from_span(span: &SurfaceV0Span) -> SourceRef {
    let data = span.source_ref_data();
    let (start_line, start_column, end_line, end_column) = data.line_columns();
    SourceRef::new(
        data.path().to_string(),
        start_line,
        start_column,
        end_line,
        end_column,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixOutcomeKind {
    Accepted(ClassificationKind),
    Diagnostic(SurfaceV0DiagnosticKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0MatrixRow {
    name: String,
    outcome: MatrixOutcomeKind,
    used_real_parse: bool,
    used_real_classification: bool,
}

impl SurfaceV0MatrixRow {
    pub fn outcome(&self) -> MatrixOutcomeKind {
        self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SurfaceV0MatrixRows {
    rows: Vec<SurfaceV0MatrixRow>,
}

impl SurfaceV0MatrixRows {
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn all_used_real_parse(&self) -> bool {
        self.rows.iter().all(|row| row.used_real_parse)
    }

    pub fn all_used_real_classification(&self) -> bool {
        self.rows.iter().all(|row| row.used_real_classification)
    }

    pub fn row(&self, name: &str) -> Option<&SurfaceV0MatrixRow> {
        self.rows.iter().find(|row| row.name == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0Matrix {
    rows: SurfaceV0MatrixRows,
}

impl SurfaceV0Matrix {
    pub fn rows(&self) -> &SurfaceV0MatrixRows {
        &self.rows
    }

    pub fn row(&self, name: &str) -> Option<&SurfaceV0MatrixRow> {
        self.rows.row(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceV0MatrixSpec {
    root: PathBuf,
    fixture_names: Vec<String>,
}

impl SurfaceV0MatrixSpec {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            fixture_names: Vec::new(),
        }
    }

    pub fn with_fixture_names(
        mut self,
        fixture_names: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.fixture_names = fixture_names
            .into_iter()
            .map(|name| name.as_ref().to_string())
            .collect();
        self
    }
}

pub fn classify_surface_v0_matrix(
    spec: SurfaceV0MatrixSpec,
    options: SurfaceV0ClassificationOptions,
) -> Result<SurfaceV0Matrix, String> {
    let mut rows = Vec::new();
    for fixture_name in spec.fixture_names {
        let path = spec.root.join(&fixture_name);
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        let name = fixture_name
            .strip_suffix(".mir")
            .unwrap_or(&fixture_name)
            .to_string();
        let outcome = match parse_surface_v0(FixtureSource::new(fixture_name, source)) {
            Ok(ast) => match classify_surface_v0(&ast, options) {
                Ok(classification) => MatrixOutcomeKind::Accepted(classification.kind()),
                Err(diagnostics) => MatrixOutcomeKind::Diagnostic(diagnostics.primary().kind()),
            },
            Err(diagnostics) => MatrixOutcomeKind::Diagnostic(parse_error_kind(diagnostics)),
        };
        rows.push(SurfaceV0MatrixRow {
            name,
            outcome,
            used_real_parse: true,
            used_real_classification: true,
        });
    }
    Ok(SurfaceV0Matrix {
        rows: SurfaceV0MatrixRows { rows },
    })
}

fn parse_error_kind(diagnostics: ParseDiagnostics) -> SurfaceV0DiagnosticKind {
    match diagnostics.primary().kind() {
        ParseErrorKind::RoleActorMustBeLiteralSelf => {
            SurfaceV0DiagnosticKind::RoleActorMustBeLiteralSelf
        }
        ParseErrorKind::IntegerLiteralOutOfRange => SurfaceV0DiagnosticKind::UnexpectedSyntax,
        ParseErrorKind::UnsupportedTransportSyntax => {
            SurfaceV0DiagnosticKind::UnsupportedTransportSyntax
        }
        ParseErrorKind::UnsupportedOccurrenceSyntax => {
            SurfaceV0DiagnosticKind::UnsupportedOccurrenceSyntax
        }
        ParseErrorKind::UnsupportedEnvelopeSyntax => {
            SurfaceV0DiagnosticKind::UnsupportedEnvelopeSyntax
        }
        ParseErrorKind::UnexpectedSyntax => SurfaceV0DiagnosticKind::UnexpectedSyntax,
    }
}
