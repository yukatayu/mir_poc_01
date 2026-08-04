/-!
Mir Theory v0 M6 bounded Surface classifier evidence.

The finite classifier below will model only the selected M6 source forms and
their source-span-preserving classification.  It is not a parser, checker,
runtime, public grammar, or general elaboration theorem.
-/

namespace MirTheoryV0M6Surface

/- `SourceSpan` is a nominal source carrier, not an operation key or a runtime
   occurrence.  The concrete finite values identify the selected grammar forms. -/
inductive SourceSpan where
  | moduleSpan
  | locusSpan
  | principalSpan
  | typeSpan
  | stateSpan
  | actorSpan
  | roleActorSpan
  | handlerSpan
  | ownerAssignmentSpan
  | assignmentTargetSpan
  | rhsReferenceSpan
  | relationSpan
  | relationPublicationSpan
  | relationMutationSpan
  | designatedEvaluationSpan
  | authSpan
  | verificationSpan
deriving DecidableEq, Repr

inductive LocusName where
  | ownerS
  | evaluatorE
deriving DecidableEq, Repr

inductive PrincipalName where
  | role
  | participant
deriving DecidableEq, Repr

inductive StateName where
  | playerHp
  | maintainedSubject
  | primaryAnchor
  | fallbackAnchor
deriving DecidableEq, Repr

inductive TypeName where
  | player
  | bird
deriving DecidableEq, Repr

/- These are intentionally distinct nominal references, even in this finite
   profile.  A source `tick F` cannot become a relation activation frontier. -/
inductive ResultFrontierRef where
  | tickF
deriving DecidableEq, Repr

inductive RelationFrontierRef where
  | birdBindingFrontier
deriving DecidableEq, Repr

inductive Materialization where
  | store
  | publishValue
  | publishRelation
deriving DecidableEq, Repr

/- These are M6-owned references to named M5 fragment cases, not an amendment
   or import of M5's `SurfaceFragment` carrier. -/
inductive M5Fragment where
  | ownerRmw
  | designatedEvaluation
  | relationBind
  | relationPublish
  | relationProjection
deriving DecidableEq, Repr

inductive AuthorityOrigin where
  | roleSelf
deriving DecidableEq, Repr

inductive TemplateKind where
  | moduleDeclaration
  | locusDeclaration
  | principalDeclaration
  | typeDeclaration
  | stateDeclaration
  | actorDeclaration
  | handlerDeclaration
  | ownerMutation
  | maintainedRelation
  | designatedResult
  | deferredWithAuth
  | deferredVerify
deriving DecidableEq, Repr

inductive ObligationKind where
  | ownerCapability
  | ownerWitness
  | relationOwnerAuthority
  | resultFrontierBinding
deriving DecidableEq, Repr

structure GeneratedObligation where
  kind : ObligationKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

inductive GeneratedEdgeKind where
  | requestToOwner
  | ownerWrite
deriving DecidableEq, Repr

structure GeneratedEdge where
  kind : GeneratedEdgeKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

/- The classifier's source-to-Core map is separate from generated M5 edges.
   In particular, an owner-local RHS dependency is map evidence, not an edge. -/
inductive SourceToCoreKind where
  | ownerRmw
  | ownerLocalRead
  | ownerLocalWrite
  | designatedDecision
  | publishRelation
  | consumerLocalProjection
  | deferredPolicy
deriving DecidableEq, Repr

/- Receipt facts are output-only metadata.  M6 has no corresponding source
   syntax, and the same-owner assignment lowering emits an empty list. -/
inductive ReceiptFact where
  | receipt
  | receiptRelease
deriving DecidableEq, Repr

/- The M6 CoreTemplate is an inspectable non-wire elaboration target.  The two
   frontier fields stay distinct even when one is absent. -/
structure CoreTemplate where
  kind : TemplateKind
  m5Fragments : List M5Fragment
  authorityOrigin : Option AuthorityOrigin
  evaluationSite : Option LocusName
  materialization : Option Materialization
  resultFrontier : Option ResultFrontierRef
  relationFrontier : Option RelationFrontierRef
  consumerProjectionSite : Option LocusName
  generatedEdges : List GeneratedEdge
  receiptFacts : List ReceiptFact
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

/- Lean case spellings map one-to-one to the Rust diagnostic names
   `RoleActorMustBeLiteralSelf`, `OwnerActionLocusMismatch`,
   `CrossOwnerWriteTargetOutsideActionLocus`, `FieldlessAssignmentTarget`, and
   `CrossOwnerOperandRequiresReceipt`. -/
inductive DiagnosticKind where
  | roleActorMustBeLiteralSelf
  | ownerActionLocusMismatch
  | crossOwnerWriteTargetOutsideActionLocus
  | fieldlessAssignmentTarget
  | crossOwnerReceiptRequired
  | relationMustPublishRelationCarrier
  | consumerRelationMutationDenied
deriving DecidableEq, Repr

structure StaticDiagnostic where
  kind : DiagnosticKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

/- This is the finite source-result domain. It includes rejected source shapes
   only to state their parser/classifier result and canonical span; it does not
   claim that such a shape builds a successful parser AST. It deliberately
   contains no send, receive, envelope, occurrence, or witness constructor. -/
inductive SurfaceForm where
  | moduleDecl (sourceSpan : SourceSpan)
  | locusDecl (sourceSpan : SourceSpan) (locus : LocusName)
  | principalDecl (sourceSpan : SourceSpan) (principal : PrincipalName)
  | typeDecl (sourceSpan : SourceSpan) (type : TypeName)
  | stateDecl (sourceSpan : SourceSpan) (state : StateName) (owner : LocusName)
      (indexPrincipal : PrincipalName)
  | actorBlock (sourceSpan : SourceSpan) (actor : PrincipalName)
      (actorTokenSpan : SourceSpan) (site : LocusName)
  | handler (sourceSpan : SourceSpan) (actor : PrincipalName) (site : LocusName)
  | ownerAssignment (sourceSpan : SourceSpan) (actor : PrincipalName)
      (actorSite actionSite targetOwner rhsOwner : LocusName) (targetHasField : Bool)
      (targetSpan rhsRefSpan : SourceSpan)
      (state : StateName)
  | maintainedRelation (sourceSpan : SourceSpan) (owner : LocusName)
      (consumerProjectionSite : Option LocusName) (subject primary fallback : StateName)
  | relationValuePublication (sourceSpan : SourceSpan)
  | consumerRelationMutation (sourceSpan : SourceSpan)
  | designatedEvaluation (sourceSpan : SourceSpan) (evaluator : LocusName)
      (frontier : ResultFrontierRef)
  | withAuth (sourceSpan : SourceSpan)
  | verifyFiniteRefinement (sourceSpan : SourceSpan)
deriving DecidableEq, Repr

def SurfaceForm.sourceSpan : SurfaceForm → SourceSpan
  | .moduleDecl span => span
  | .locusDecl span _ => span
  | .principalDecl span _ => span
  | .typeDecl span _ => span
  | .stateDecl span _ _ _ => span
  | .actorBlock span _ _ _ => span
  | .handler span _ _ => span
  | .ownerAssignment span _ _ _ _ _ _ _ _ _ => span
  | .maintainedRelation span _ _ _ _ _ => span
  | .relationValuePublication span => span
  | .consumerRelationMutation span => span
  | .designatedEvaluation span _ _ => span
  | .withAuth span => span
  | .verifyFiniteRefinement span => span

structure CoreClassification where
  parseSpan : SourceSpan
  template : CoreTemplate
  obligations : List GeneratedObligation
  sourceToCoreKinds : List SourceToCoreKind
deriving DecidableEq, Repr

inductive Classification where
  | core (value : CoreClassification)
  | diagnostic (parseSpan : SourceSpan) (value : StaticDiagnostic)
deriving DecidableEq, Repr

def Classification.sourceSpan : Classification → SourceSpan
  | .core value => value.parseSpan
  | .diagnostic span _ => span

def declarationTemplate (kind : TemplateKind) (span : SourceSpan) : CoreTemplate :=
  { kind
    m5Fragments := []
    authorityOrigin := none
    evaluationSite := none
    materialization := none
    resultFrontier := none
    relationFrontier := none
    consumerProjectionSite := none
    generatedEdges := []
    receiptFacts := []
    sourceSpan := span }

def coreResultWithSourceToCore (span : SourceSpan) (template : CoreTemplate)
    (obligationKinds : List ObligationKind)
    (sourceToCoreKinds : List SourceToCoreKind) : Classification :=
  .core
    { parseSpan := span
      template
      obligations := obligationKinds.map (fun kind => { kind, sourceSpan := span })
      sourceToCoreKinds }

def coreResult (span : SourceSpan) (template : CoreTemplate)
    (obligationKinds : List ObligationKind) : Classification :=
  coreResultWithSourceToCore span template obligationKinds []

def deferred (kind : DiagnosticKind) (parseSpan diagnosticSpan : SourceSpan) : Classification :=
  .diagnostic parseSpan { kind, sourceSpan := diagnosticSpan }

/- This is a total classifier for the finite parsed-form domain.  It is not a
   parser nor a claim that M7 has implemented this source grammar. -/
def classify : SurfaceForm → Classification
  | .moduleDecl span => coreResult span (declarationTemplate .moduleDeclaration span) []
  | .locusDecl span _ => coreResult span (declarationTemplate .locusDeclaration span) []
  | .principalDecl span _ => coreResult span (declarationTemplate .principalDeclaration span) []
  | .typeDecl span _ => coreResult span (declarationTemplate .typeDeclaration span) []
  | .stateDecl span _ _ _ => coreResult span (declarationTemplate .stateDeclaration span) []
  | .actorBlock span actor actorTokenSpan site =>
      if actor != .role then
        deferred .roleActorMustBeLiteralSelf span actorTokenSpan
      else
        coreResult span
          { kind := .actorDeclaration
            m5Fragments := []
            authorityOrigin := some .roleSelf
            evaluationSite := some site
            materialization := none
            resultFrontier := none
            relationFrontier := none
            consumerProjectionSite := none
            generatedEdges := []
            receiptFacts := []
            sourceSpan := span }
          []
  | .handler span _ site =>
      coreResult span
        { kind := .handlerDeclaration
          m5Fragments := []
          authorityOrigin := some .roleSelf
          evaluationSite := some site
          materialization := none
          resultFrontier := none
          relationFrontier := none
          consumerProjectionSite := none
          generatedEdges := []
          receiptFacts := []
          sourceSpan := span }
        []
  | .ownerAssignment span _ actorSite actionSite targetOwner rhsOwner targetHasField targetSpan rhsRefSpan _ =>
      if actorSite != actionSite then
        deferred .ownerActionLocusMismatch span span
      else if targetHasField = false then
        deferred .fieldlessAssignmentTarget span targetSpan
      else if targetOwner != actionSite then
        deferred .crossOwnerWriteTargetOutsideActionLocus span targetSpan
      else if rhsOwner != actionSite then
        deferred .crossOwnerReceiptRequired span rhsRefSpan
      else
        coreResultWithSourceToCore span
          { kind := .ownerMutation
            m5Fragments := [.ownerRmw]
            authorityOrigin := some .roleSelf
            evaluationSite := some actionSite
            materialization := some .store
            resultFrontier := none
            relationFrontier := none
            consumerProjectionSite := none
            generatedEdges :=
              [ { kind := .requestToOwner, sourceSpan := span }
              , { kind := .ownerWrite, sourceSpan := span } ]
            receiptFacts := []
            sourceSpan := span }
          [.ownerCapability, .ownerWitness]
          [.ownerRmw, .ownerLocalRead, .ownerLocalWrite]
  | .maintainedRelation span owner consumerProjectionSite _ _ _ =>
      coreResultWithSourceToCore span
        { kind := .maintainedRelation
          m5Fragments := [.relationBind, .relationPublish, .relationProjection]
          authorityOrigin := none
          evaluationSite := some owner
          materialization := some .publishRelation
          resultFrontier := none
          relationFrontier := some .birdBindingFrontier
          consumerProjectionSite
          generatedEdges := []
          receiptFacts := []
          sourceSpan := span }
        [.relationOwnerAuthority]
        (match consumerProjectionSite with
        | none => [.publishRelation]
        | some _ => [.publishRelation, .consumerLocalProjection])
  | .relationValuePublication span => deferred .relationMustPublishRelationCarrier span span
  | .consumerRelationMutation span => deferred .consumerRelationMutationDenied span span
  | .designatedEvaluation span evaluator frontier =>
      coreResultWithSourceToCore span
        { kind := .designatedResult
          m5Fragments := [.designatedEvaluation]
          authorityOrigin := none
          evaluationSite := some evaluator
          materialization := some .publishValue
          resultFrontier := some frontier
          relationFrontier := none
          consumerProjectionSite := none
          generatedEdges := []
          receiptFacts := []
          sourceSpan := span }
        [.resultFrontierBinding]
        [.designatedDecision]
  | .withAuth span =>
      coreResultWithSourceToCore span (declarationTemplate .deferredWithAuth span) [] [.deferredPolicy]
  | .verifyFiniteRefinement span =>
      coreResultWithSourceToCore span (declarationTemplate .deferredVerify span) [] [.deferredPolicy]

def coreTemplateSpanPreserved (form : SurfaceForm) : Bool :=
  match classify form with
  | .core result => result.template.sourceSpan == form.sourceSpan
  | .diagnostic _ _ => true

def generatedObligationSpansPreserved (form : SurfaceForm) : Bool :=
  match classify form with
  | .core result => result.obligations.all (fun obligation => obligation.sourceSpan == form.sourceSpan)
  | .diagnostic _ _ => true

def generatedEdgeSpansPreserved (form : SurfaceForm) : Bool :=
  match classify form with
  | .core result => result.template.generatedEdges.all (fun edge => edge.sourceSpan == form.sourceSpan)
  | .diagnostic _ _ => true

def diagnosticSpanPreserved (form : SurfaceForm) : Bool :=
  match form, classify form with
  | .actorBlock span _ actorTokenSpan _, .diagnostic _ diagnostic =>
      diagnostic.sourceSpan == span || diagnostic.sourceSpan == actorTokenSpan
  | .ownerAssignment span _ _ _ _ _ _ targetSpan rhsRefSpan _, .diagnostic _ diagnostic =>
      diagnostic.sourceSpan == span || diagnostic.sourceSpan == targetSpan ||
        diagnostic.sourceSpan == rhsRefSpan
  | _, .core _ => true
  | _, .diagnostic _ diagnostic => diagnostic.sourceSpan == form.sourceSpan

/- RED example, now green: the selected M6 module form retains its canonical
   source span through parsing/classification. -/
example :
    (classify (.moduleDecl .moduleSpan)).sourceSpan = .moduleSpan := by
  rfl

/- RED: the real M6 parser accepts a non-braced `with auth MembershipAuth`
   form as a successful, non-executable typed deferred template. -/
example :
    match classify (.withAuth .authSpan) with
    | .core result => result.template.kind = .deferredWithAuth
    | .diagnostic _ _ => False := by
  rfl

/- RED: the actual classifier records a local RHS dependency in its
   source-to-Core map; it is not a third generated M5 edge. -/
example :
    match classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .ownerS
      .ownerS .ownerS true .assignmentTargetSpan .rhsReferenceSpan .playerHp) with
    | .core result => result.sourceToCoreKinds =
        [.ownerRmw, .ownerLocalRead, .ownerLocalWrite]
    | .diagnostic _ _ => False := by
  rfl

/- RED: a syntactically valid field-bearing write target owned outside the
   action locus must reject at that target, before any CoreTemplate exists. -/
theorem target_owner_outside_action_locus_is_typed_diagnostic :
    classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .ownerS
      .evaluatorE .ownerS true .assignmentTargetSpan .rhsReferenceSpan .playerHp) =
      .diagnostic .ownerAssignmentSpan
        { kind := .crossOwnerWriteTargetOutsideActionLocus,
          sourceSpan := .assignmentTargetSpan } := by
  rfl

theorem classifier_deterministic {form : SurfaceForm} {first second : Classification}
    (firstResult : classify form = first)
    (secondResult : classify form = second) :
    first = second := by
  exact firstResult.symm.trans secondResult

theorem classification_preserves_parse_span (form : SurfaceForm) :
    (classify form).sourceSpan = form.sourceSpan := by
  cases form
  all_goals try rfl
  case actorBlock span actor actorTokenSpan site =>
    by_cases literalSelf : actor = .role <;>
      simp [classify, coreResult, coreResultWithSourceToCore, deferred, literalSelf,
        Classification.sourceSpan, SurfaceForm.sourceSpan]
  case ownerAssignment span actor actorSite actionSite targetOwner rhsOwner targetHasField targetSpan rhsRefSpan state =>
    by_cases sameActorSite : actorSite = actionSite <;>
      cases targetHasField <;>
      by_cases sameTargetOwner : targetOwner = actionSite <;>
      by_cases sameRhsOwner : rhsOwner = actionSite <;>
      simp [classify, coreResultWithSourceToCore, deferred, sameActorSite, sameTargetOwner, sameRhsOwner,
        Classification.sourceSpan, SurfaceForm.sourceSpan]

theorem core_template_preserves_canonical_source_span (form : SurfaceForm) :
    coreTemplateSpanPreserved form = true := by
  cases form <;>
    simp [classify, coreResult, coreResultWithSourceToCore, deferred, declarationTemplate,
      coreTemplateSpanPreserved, SurfaceForm.sourceSpan]
  case actorBlock span actor actorTokenSpan site =>
    by_cases literalSelf : actor = .role <;>
      simp [literalSelf]
  case ownerAssignment span actor actorSite actionSite targetOwner rhsOwner targetHasField targetSpan rhsRefSpan state =>
    by_cases sameActorSite : actorSite = actionSite <;>
      cases targetHasField <;>
      by_cases sameTargetOwner : targetOwner = actionSite <;>
      by_cases sameRhsOwner : rhsOwner = actionSite <;>
      simp [sameActorSite, sameTargetOwner, sameRhsOwner]

theorem generated_obligations_preserve_canonical_source_span (form : SurfaceForm) :
    generatedObligationSpansPreserved form = true := by
  cases form <;>
    simp [classify, coreResult, coreResultWithSourceToCore, deferred, declarationTemplate,
      generatedObligationSpansPreserved, SurfaceForm.sourceSpan]
  case actorBlock span actor actorTokenSpan site =>
    by_cases literalSelf : actor = .role <;>
      simp [literalSelf]
  case ownerAssignment span actor actorSite actionSite targetOwner rhsOwner targetHasField targetSpan rhsRefSpan state =>
    by_cases sameActorSite : actorSite = actionSite <;>
      cases targetHasField <;>
      by_cases sameTargetOwner : targetOwner = actionSite <;>
      by_cases sameRhsOwner : rhsOwner = actionSite <;>
      simp [sameActorSite, sameTargetOwner, sameRhsOwner]

theorem generated_edges_preserve_canonical_source_span (form : SurfaceForm) :
    generatedEdgeSpansPreserved form = true := by
  cases form <;>
    simp [classify, coreResult, coreResultWithSourceToCore, deferred, declarationTemplate,
      generatedEdgeSpansPreserved, SurfaceForm.sourceSpan]
  case actorBlock span actor actorTokenSpan site =>
    by_cases literalSelf : actor = .role <;>
      simp [literalSelf]
  case ownerAssignment span actor actorSite actionSite targetOwner rhsOwner targetHasField targetSpan rhsRefSpan state =>
    by_cases sameActorSite : actorSite = actionSite <;>
      cases targetHasField <;>
      by_cases sameTargetOwner : targetOwner = actionSite <;>
      by_cases sameRhsOwner : rhsOwner = actionSite <;>
      simp [sameActorSite, sameTargetOwner, sameRhsOwner]

theorem static_diagnostic_preserves_canonical_source_span (form : SurfaceForm) :
    diagnosticSpanPreserved form = true := by
  cases form <;>
    simp [classify, coreResult, coreResultWithSourceToCore, deferred, declarationTemplate,
      diagnosticSpanPreserved, SurfaceForm.sourceSpan]
  case actorBlock span actor actorTokenSpan site =>
    by_cases literalSelf : actor = .role <;>
      simp [literalSelf]
  case ownerAssignment span actor actorSite actionSite targetOwner rhsOwner targetHasField targetSpan rhsRefSpan state =>
    by_cases sameActorSite : actorSite = actionSite <;>
      cases targetHasField <;>
      by_cases sameTargetOwner : targetOwner = actionSite <;>
      by_cases sameRhsOwner : rhsOwner = actionSite <;>
      simp [sameActorSite, sameTargetOwner, sameRhsOwner]

theorem owner_assignment_keeps_role_self_authority_and_owner_site :
    classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .ownerS .ownerS .ownerS
      true .assignmentTargetSpan .rhsReferenceSpan .playerHp) =
      .core
        { parseSpan := .ownerAssignmentSpan
          template :=
            { kind := .ownerMutation
              m5Fragments := [.ownerRmw]
              authorityOrigin := some .roleSelf
              evaluationSite := some .ownerS
              materialization := some .store
              resultFrontier := none
              relationFrontier := none
              consumerProjectionSite := none
              generatedEdges :=
                [ { kind := .requestToOwner, sourceSpan := .ownerAssignmentSpan }
                , { kind := .ownerWrite, sourceSpan := .ownerAssignmentSpan } ]
              receiptFacts := []
              sourceSpan := .ownerAssignmentSpan }
          obligations :=
            [ { kind := .ownerCapability, sourceSpan := .ownerAssignmentSpan }
            , { kind := .ownerWitness, sourceSpan := .ownerAssignmentSpan } ]
          sourceToCoreKinds := [.ownerRmw, .ownerLocalRead, .ownerLocalWrite] } := by
  rfl

theorem mismatched_owner_action_site_is_static_diagnostic :
    classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .evaluatorE .ownerS .ownerS
      true .assignmentTargetSpan .rhsReferenceSpan .playerHp) =
      .diagnostic .ownerAssignmentSpan
        { kind := .ownerActionLocusMismatch, sourceSpan := .ownerAssignmentSpan } := by
  rfl

theorem non_literal_role_actor_is_typed_parser_diagnostic :
    classify (.actorBlock .actorSpan .participant .roleActorSpan .ownerS) =
      .diagnostic .actorSpan
        { kind := .roleActorMustBeLiteralSelf, sourceSpan := .roleActorSpan } := by
  rfl

theorem fieldless_assignment_target_is_typed_diagnostic :
    classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .ownerS .ownerS .ownerS
      false .assignmentTargetSpan .rhsReferenceSpan .playerHp) =
      .diagnostic .ownerAssignmentSpan
        { kind := .fieldlessAssignmentTarget, sourceSpan := .assignmentTargetSpan } := by
  rfl

theorem cross_owner_rhs_requires_explicit_receipt_diagnostic :
    classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .ownerS .ownerS .evaluatorE
      true .assignmentTargetSpan .rhsReferenceSpan .playerHp) =
      .diagnostic .ownerAssignmentSpan
        { kind := .crossOwnerReceiptRequired, sourceSpan := .rhsReferenceSpan } := by
  rfl

theorem maintained_relation_uses_relation_publication_not_value_publication :
    classify (.maintainedRelation .relationSpan .ownerS (some .evaluatorE)
      .maintainedSubject .primaryAnchor .fallbackAnchor) =
      .core
        { parseSpan := .relationSpan
          template :=
            { kind := .maintainedRelation
              m5Fragments := [.relationBind, .relationPublish, .relationProjection]
              authorityOrigin := none
              evaluationSite := some .ownerS
              materialization := some .publishRelation
              resultFrontier := none
              relationFrontier := some .birdBindingFrontier
              consumerProjectionSite := some .evaluatorE
              generatedEdges := []
              receiptFacts := []
              sourceSpan := .relationSpan }
          obligations := [{ kind := .relationOwnerAuthority, sourceSpan := .relationSpan }]
          sourceToCoreKinds := [.publishRelation, .consumerLocalProjection] } := by
  rfl

theorem designated_evaluation_keeps_only_result_frontier :
    classify (.designatedEvaluation .designatedEvaluationSpan .evaluatorE .tickF) =
      .core
        { parseSpan := .designatedEvaluationSpan
          template :=
            { kind := .designatedResult
              m5Fragments := [.designatedEvaluation]
              authorityOrigin := none
              evaluationSite := some .evaluatorE
              materialization := some .publishValue
              resultFrontier := some .tickF
              relationFrontier := none
              consumerProjectionSite := none
              generatedEdges := []
              receiptFacts := []
              sourceSpan := .designatedEvaluationSpan }
          obligations := [{ kind := .resultFrontierBinding, sourceSpan := .designatedEvaluationSpan }]
          sourceToCoreKinds := [.designatedDecision] } := by
  rfl

theorem auth_and_finite_refinement_are_non_executable_deferred_templates :
    (match classify (.withAuth .authSpan) with
      | .core result => result.template.kind = .deferredWithAuth ∧
          result.template.m5Fragments = [] ∧
          result.template.materialization = none ∧
          result.sourceToCoreKinds = [.deferredPolicy]
      | .diagnostic _ _ => False) ∧
    (match classify (.verifyFiniteRefinement .verificationSpan) with
      | .core result => result.template.kind = .deferredVerify ∧
          result.template.m5Fragments = [] ∧
          result.template.materialization = none ∧
          result.sourceToCoreKinds = [.deferredPolicy]
      | .diagnostic _ _ => False) := by
  exact ⟨⟨rfl, rfl, rfl, rfl⟩, ⟨rfl, rfl, rfl, rfl⟩⟩

theorem relation_value_and_consumer_mutation_are_typed_diagnostics :
    classify (.relationValuePublication .relationPublicationSpan) =
      .diagnostic .relationPublicationSpan
        { kind := .relationMustPublishRelationCarrier,
          sourceSpan := .relationPublicationSpan } ∧
    classify (.consumerRelationMutation .relationMutationSpan) =
      .diagnostic .relationMutationSpan
        { kind := .consumerRelationMutationDenied,
          sourceSpan := .relationMutationSpan } := by
  exact ⟨rfl, rfl⟩

/- RED: a same-owner M6 lowering must expose its non-receipt CoreTemplate
   boundary rather than synthesize an M5 receipt fact. -/
example :
    match classify (.ownerAssignment .ownerAssignmentSpan .role .ownerS .ownerS .ownerS .ownerS
      true .assignmentTargetSpan .rhsReferenceSpan .playerHp) with
    | .core result => result.template.receiptFacts = []
    | .diagnostic _ _ => False := by
  rfl

#print axioms classifier_deterministic
#print axioms classification_preserves_parse_span
#print axioms core_template_preserves_canonical_source_span
#print axioms generated_obligations_preserve_canonical_source_span
#print axioms generated_edges_preserve_canonical_source_span
#print axioms static_diagnostic_preserves_canonical_source_span
#print axioms owner_assignment_keeps_role_self_authority_and_owner_site
#print axioms non_literal_role_actor_is_typed_parser_diagnostic
#print axioms target_owner_outside_action_locus_is_typed_diagnostic
#print axioms fieldless_assignment_target_is_typed_diagnostic
#print axioms cross_owner_rhs_requires_explicit_receipt_diagnostic
#print axioms maintained_relation_uses_relation_publication_not_value_publication
#print axioms auth_and_finite_refinement_are_non_executable_deferred_templates
#print axioms relation_value_and_consumer_mutation_are_typed_diagnostics

end MirTheoryV0M6Surface
