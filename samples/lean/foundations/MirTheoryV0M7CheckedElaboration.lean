/-!
Mir Theory v0 M7 checked-elaboration finite evidence.

This is a self-contained finite model of the single M7 source-first boundary:
M6 classification -> finite check -> deterministic checked elaboration.  It
does not parse text, execute Core, or decide authorization/verification.
-/

namespace MirTheoryV0M7CheckedElaboration

set_option autoImplicit false

/- Nominal source locations identify the selected M6/M7 fixture positions. -/
inductive SourceSpan where
  | moduleSpan
  | firstLocusSpan
  | handlerSpan
  | failureRowSpan
  | assignmentSpan
  | targetFieldSpan
  | rhsReferenceSpan
  | relationSpan
  | designatedSpan
  | authSpan
  | verificationSpan
  | arithmeticOperatorSpan
  | unsupportedExpressionSpan
deriving DecidableEq, Repr

inductive SourceToCoreKind where
  | ownerRmw
  | ownerLocalRead
  | ownerLocalWrite
  | designatedDecision
  | publishRelation
  | consumerLocalProjection
  | deferredPolicy
deriving DecidableEq, Repr

inductive FailureKind where
  | staleMembership
  | missingCapability
  | missingWitness
  | routeUnavailable
deriving DecidableEq, Repr

inductive StateField where
  | hp
  | atk
  | shield
deriving DecidableEq, Repr

/- M6 is retained as the input classification: M7 only forwards an M6
   diagnostic or refines a classified template. -/
inductive M6DiagnosticKind where
  | crossOwnerOperandRequiresReceipt
  | relationMustPublishRelationCarrier
  | consumerRelationMutationDenied
deriving DecidableEq, Repr

structure M6Diagnostic where
  kind : M6DiagnosticKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

inductive M6TemplateKind where
  | ownerRmw
  | publishRelation
  | designatedPublishValue
  | deferredWithAuth
  | deferredVerify
deriving DecidableEq, Repr

structure M6CoreTemplate where
  kind : M6TemplateKind
  name : String
  sourceSpan : SourceSpan
  sourceToCoreKinds : List SourceToCoreKind
  declaredFailureRow : List FailureKind
  targetField : Option StateField
  targetFieldKnown : Bool
deriving DecidableEq, Repr

structure M6Classification where
  sourceSpan : SourceSpan
  templates : List M6CoreTemplate
  diagnostic : Option M6Diagnostic
  duplicateDeclaration : Bool
  duplicateDeclarationSpan : SourceSpan
deriving DecidableEq, Repr

/- These names deliberately match the M7 Rust boundary.  Constructor spelling
   is Lean-style; the companion document maps it to the Rust `::` spelling. -/
inductive M7DiagnosticKind where
  | crossOwnerOperandRequiresReceipt
  | relationMustPublishRelationCarrier
  | consumerRelationMutationDenied
  | generatedFailureNotDeclared
  | duplicateDeclaration
  | unknownStateField
  | typeMismatch
  | undefinedStateIndexType
  | undefinedStateFieldType
  | undefinedRelationSubjectType
  | undefinedOwnerLocus
  | undefinedConsumerLocus
  | undefinedSelfPrincipal
  | undefinedRoleEvaluationLocus
  | duplicateStateField
  | duplicateEvent
  | duplicateRelation
  | duplicateDesignated
  | duplicateDeferred
  | unsupportedExpression
  | arithmeticRequiresInt
  | residualCannotExecute
deriving DecidableEq, Repr

structure M7Diagnostic where
  kind : M7DiagnosticKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

structure SurfaceV0PipelineDiagnostics where
  entries : List M7Diagnostic
deriving DecidableEq, Repr

def SurfaceV0PipelineDiagnostics.has_executable_core
    (_ : SurfaceV0PipelineDiagnostics) : Bool := false

def singletonDiagnostic (kind : M7DiagnosticKind) (sourceSpan : SourceSpan) :
    SurfaceV0PipelineDiagnostics :=
  { entries := [{ kind := kind,
                  sourceSpan := sourceSpan }] }

/- M7 residual kinds mirror the production checker contract.  A residual is
   evidence still required; it is not an authority, capability, effect,
   mutation, verdict, or execution admission. -/
inductive ResidualObligationKind where
  | visibility
  | relationLifetime
  | fallbackValidity
  | valueVisibilityRedaction
  | authDeferred
  | verifyDeferred
deriving DecidableEq, Repr

structure ResidualObligation where
  kind : ResidualObligationKind
  name : String
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

def ResidualObligation.is_non_executable (_ : ResidualObligation) : Bool := true
def ResidualObligation.grants_authority (_ : ResidualObligation) : Bool := false
def ResidualObligation.grants_capability (_ : ResidualObligation) : Bool := false
def ResidualObligation.emits_effect (_ : ResidualObligation) : Bool := false
def ResidualObligation.mutates_state (_ : ResidualObligation) : Bool := false
def ResidualObligation.emits_verdict (_ : ResidualObligation) : Bool := false

/- The M7 checked evaluation names also mirror the production checker. -/
inductive CheckedEvaluationKind where
  | ownerRmw
  | publishRelation
  | consumerLocalProjection
  | designatedPublishValue
deriving DecidableEq, Repr

structure CheckedEvaluation where
  kind : CheckedEvaluationKind
  name : String
  sourceSpan : SourceSpan
  sourceToCoreKinds : List SourceToCoreKind
deriving DecidableEq, Repr

inductive GeneratedObligationKind where
  | failure (failure : FailureKind)
  | capability
  | witness
  | authority
  | admittedEvaluatorAuthority
  | evaluation (kind : CheckedEvaluationKind)
deriving DecidableEq, Repr

structure GeneratedObligation where
  kind : GeneratedObligationKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

def GeneratedObligation.sourceRef (obligation : GeneratedObligation) : SourceSpan :=
  obligation.sourceSpan

def GeneratedObligation.grants_authority_success (_ : GeneratedObligation) : Bool := false

structure SourceToCoreEntry where
  kind : SourceToCoreKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

/- M7 preserves enough checked Core structure for M8 to consume it without
   re-parsing source. These are finite carriers, not an M8 step semantics. -/
inductive SemanticForm where
  | state
  | relation
  | value
deriving DecidableEq, Repr

inductive Locus where
  | s
  | e
  | c
deriving DecidableEq, Repr

inductive Principal where
  | self
deriving DecidableEq, Repr

inductive EvaluationSite where
  | owner (locus : Locus)
  | designatedEvaluator (locus : Locus)
deriving DecidableEq, Repr

inductive TriggerClock where
  | onEvent
  | frontierAdvance
  | logicalTick
deriving DecidableEq, Repr

inductive AuthorityOrigin where
  | caller (principal : Principal)
  | ownerTransition (locus : Locus)
  | admittedEvaluator (locus : Locus)
deriving DecidableEq, Repr

inductive Materialization where
  | store
  | publishRelation
  | publishValue
deriving DecidableEq, Repr

structure EvaluationAxes where
  semanticForm : SemanticForm
  evaluationSite : EvaluationSite
  trigger : TriggerClock
  authorityOrigin : AuthorityOrigin
  materialization : Materialization
deriving DecidableEq, Repr

structure TypedStateRead where
  ownerLocus : Locus
  namespaceName : String
  index : Option String
  field : Option String
  valueType : String
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

inductive CheckedBinaryOperator where
  | add
  | subtract
deriving DecidableEq, Repr

inductive CheckedExpressionTree where
  | stateRead (read : TypedStateRead)
  | integerLiteral (value : Int) (sourceSpan : SourceSpan)
  | binary (operator : CheckedBinaryOperator) (sourceSpan : SourceSpan)
      (left right : CheckedExpressionTree)
deriving DecidableEq, Repr

def CheckedExpressionTree.is_m8_consumable (_ : CheckedExpressionTree) : Bool := true

def CheckedExpressionTree.valueType : CheckedExpressionTree → String
  | .stateRead read => read.valueType
  | .integerLiteral _ _ => "Int"
  | .binary _ _ _ _ => "Int"

structure CheckedExpression where
  sourceSpan : SourceSpan
  operatorChain : List String
  integerLiterals : List Int
  stateReads : List TypedStateRead
  tree : CheckedExpressionTree
deriving DecidableEq, Repr

structure TypedStateTarget where
  ownerLocus : Locus
  namespaceName : String
  index : Option String
  field : Option String
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

structure OwnerRmwCheckedCore where
  ownerLocus : Locus
  target : TypedStateTarget
  expression : CheckedExpression
  sameOwnerReads : List TypedStateRead
deriving DecidableEq, Repr

inductive RelationTransform where
  | translate (x y : Int)
  | identity
deriving DecidableEq, Repr

structure RelationOption where
  anchor : String
  epoch : String
  transform : RelationTransform
deriving DecidableEq, Repr

structure RelationCheckedCore where
  ownerLocus : Locus
  subject : String
  subjectType : String
  bindingFrontier : String
  consumerProjectionLocus : Option Locus
  publishesRelationCarrier : Bool
  primary : RelationOption
  fallback : RelationOption
deriving DecidableEq, Repr

structure RemoteInputRequest where
  sourceOwnerLocus : Locus
  typedStateRead : TypedStateRead
deriving DecidableEq, Repr

structure RemoteInputReceiptUse where
  sourceOwnerLocus : Locus
  typedStateRead : TypedStateRead
deriving DecidableEq, Repr

structure DesignatedRemoteInputDependency where
  designatedEvaluator : Locus
  requesterSite : EvaluationSite
  authorityOrigin : AuthorityOrigin
  sourceOwnerLocus : Locus
  typedStateRead : TypedStateRead
  request : RemoteInputRequest
  receiptUse : RemoteInputReceiptUse
deriving DecidableEq, Repr

structure EvaluationPolicy where
  name : String
  deterministic : Bool
deriving DecidableEq, Repr

def EvaluationPolicy.declaredDeterministic (name : String) : EvaluationPolicy :=
  { name, deterministic := true }

structure ObservationPolicy where
  name : String
deriving DecidableEq, Repr

def ObservationPolicy.declared (name : String) : ObservationPolicy :=
  { name }

structure PolicyStamp where
  evaluationPolicy : EvaluationPolicy
  observationPolicy : ObservationPolicy
deriving DecidableEq, Repr

def EvaluationPolicy.stampWith (evaluationPolicy : EvaluationPolicy)
    (observationPolicy : ObservationPolicy) : PolicyStamp :=
  { evaluationPolicy, observationPolicy }

/- This is deliberately nominally distinct from the designated result
   frontier, matching the M3 `InputFrontier` carried by the checked Core. -/
structure InputFrontier where
  producers : List String
deriving DecidableEq, Repr

def InputFrontier.fromOrderedProducers (producers : List String) : InputFrontier :=
  { producers }

structure DesignatedCheckedCore where
  evaluator : String
  result : String
  triggerFrontier : String
  inputFrontier : InputFrontier
  resultFrontier : String
  resultVersion : Nat
  evaluationPolicy : EvaluationPolicy
  observationPolicy : ObservationPolicy
  policyStamp : PolicyStamp
  expression : CheckedExpression
  generatedRemoteInputDependencies : List DesignatedRemoteInputDependency
deriving DecidableEq, Repr

inductive EffectKind where
  | ownerRequest
  | ownerLocalRead
  | ownerWrite
  | relationPublish
  | designatedRemoteRequest
  | designatedReceiptUse
  | designatedValuePublish
deriving DecidableEq, Repr

structure EffectEntry where
  kind : EffectKind
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

def EffectEntry.sourceRef (entry : EffectEntry) : SourceSpan := entry.sourceSpan

structure EffectRow where
  entries : List EffectEntry
deriving DecidableEq, Repr

structure CheckedCoreProjection where
  axes : EvaluationAxes
  effectRow : EffectRow
  ownerRmw : Option OwnerRmwCheckedCore
  relation : Option RelationCheckedCore
  designated : Option DesignatedCheckedCore
deriving DecidableEq, Repr

structure StableSourceMapEntry where
  ordinal : Nat
  stableKey : Nat
  coreRef : String
  sourceSpan : SourceSpan
deriving DecidableEq, Repr

structure CheckedElaboration where
  sourceSpan : SourceSpan
  consumedM6Classification : M6Classification
  evaluations : List CheckedEvaluation
  checkedCoreProjections : List CheckedCoreProjection
  generatedObligations : List GeneratedObligation
  residualObligations : List ResidualObligation
  sourceToCoreMap : List SourceToCoreEntry
  stableSourceMap : List StableSourceMapEntry
  executionIsAdmissible : Bool
deriving DecidableEq, Repr

/- The exact selected finite source matrix.  It is source input, not a
   replacement grammar: `classify_surface_v0` below produces the retained M6
   classification that M7 consumes. -/
inductive SurfaceV0Source where
  | canonicalAttackBundle
  | maintainedBirdRelation
  | designatedTickPublishResult
  | crossOwnerWithoutReceipt
  | relationAbsolutePosePublication
  | relationConsumerMutation
  | underdeclaredFailureRow
  | duplicateDeclaration
  | unknownStateField
  | residualCannotExecute
  | ownerOnlyNoResiduals
  | m6BroadUnsupportedExpression
  | arithmeticRequiresInt
  | crossOwnerAndUnderdeclaredFailure
deriving DecidableEq, Repr

def allGeneratedFailures : List FailureKind :=
  [.staleMembership, .missingCapability, .missingWitness, .routeUnavailable]

def ownerTemplate (declaredFailureRow : List FailureKind) (targetField : StateField)
    (targetFieldKnown : Bool) : M6CoreTemplate :=
  { kind := .ownerRmw,
    name := "attack",
    sourceSpan := .assignmentSpan,
    sourceToCoreKinds := [.ownerRmw, .ownerLocalRead, .ownerLocalWrite],
    declaredFailureRow,
    targetField := some targetField,
    targetFieldKnown }

def relationTemplate : M6CoreTemplate :=
  { kind := .publishRelation,
    name := "bird_follow",
    sourceSpan := .relationSpan,
    sourceToCoreKinds := [.publishRelation, .consumerLocalProjection],
    declaredFailureRow := [],
    targetField := none,
    targetFieldKnown := true }

def designatedTemplate : M6CoreTemplate :=
  { kind := .designatedPublishValue,
    name := "result",
    sourceSpan := .designatedSpan,
    sourceToCoreKinds := [.designatedDecision],
    declaredFailureRow := [],
    targetField := none,
    targetFieldKnown := true }

def authTemplate : M6CoreTemplate :=
  { kind := .deferredWithAuth,
    name := "MembershipAuth",
    sourceSpan := .authSpan,
    sourceToCoreKinds := [.deferredPolicy],
    declaredFailureRow := [],
    targetField := none,
    targetFieldKnown := true }

def verifyTemplate : M6CoreTemplate :=
  { kind := .deferredVerify,
    name := "finite_refinement",
    sourceSpan := .verificationSpan,
    sourceToCoreKinds := [.deferredPolicy],
    declaredFailureRow := [],
    targetField := none,
    targetFieldKnown := true }

def classify_surface_v0 : SurfaceV0Source → M6Classification
  | .canonicalAttackBundle =>
      { sourceSpan := .moduleSpan,
        templates := [ownerTemplate allGeneratedFailures .hp true,
          relationTemplate, designatedTemplate, authTemplate, verifyTemplate],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .maintainedBirdRelation =>
      { sourceSpan := .moduleSpan,
        templates := [relationTemplate],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .designatedTickPublishResult =>
      { sourceSpan := .moduleSpan,
        templates := [designatedTemplate],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .crossOwnerWithoutReceipt =>
      { sourceSpan := .moduleSpan,
        templates := [],
        diagnostic := some
          { kind := .crossOwnerOperandRequiresReceipt,
            sourceSpan := .rhsReferenceSpan }
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .relationAbsolutePosePublication =>
      { sourceSpan := .moduleSpan,
        templates := [],
        diagnostic := some
          { kind := .relationMustPublishRelationCarrier,
            sourceSpan := .relationSpan }
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .relationConsumerMutation =>
      { sourceSpan := .moduleSpan,
        templates := [],
        diagnostic := some
          { kind := .consumerRelationMutationDenied,
            sourceSpan := .relationSpan }
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .underdeclaredFailureRow =>
      { sourceSpan := .moduleSpan,
        templates := [ownerTemplate [.staleMembership] .hp true],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .duplicateDeclaration =>
      { sourceSpan := .moduleSpan,
        templates := [],
        diagnostic := none,
        duplicateDeclaration := true,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .unknownStateField =>
      { sourceSpan := .moduleSpan,
        templates := [ownerTemplate allGeneratedFailures .shield false],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .residualCannotExecute =>
      { sourceSpan := .moduleSpan,
        templates := [authTemplate, verifyTemplate],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .ownerOnlyNoResiduals =>
      { sourceSpan := .moduleSpan,
        templates := [ownerTemplate allGeneratedFailures .hp true],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .m6BroadUnsupportedExpression | .arithmeticRequiresInt =>
      { sourceSpan := .moduleSpan,
        templates := [ownerTemplate allGeneratedFailures .hp true],
        diagnostic := none,
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }
  | .crossOwnerAndUnderdeclaredFailure =>
      { sourceSpan := .moduleSpan,
        templates := [ownerTemplate [.staleMembership] .hp true],
        diagnostic := some
          { kind := .crossOwnerOperandRequiresReceipt,
            sourceSpan := .rhsReferenceSpan },
        duplicateDeclaration := false,
        duplicateDeclarationSpan := .firstLocusSpan }

def forwardM6Diagnostic : M6DiagnosticKind → M7DiagnosticKind
  | .crossOwnerOperandRequiresReceipt => .crossOwnerOperandRequiresReceipt
  | .relationMustPublishRelationCarrier => .relationMustPublishRelationCarrier
  | .consumerRelationMutationDenied => .consumerRelationMutationDenied

def containsFailure (needle : FailureKind) : List FailureKind → Bool
  | [] => false
  | found :: rest => if needle = found then true else containsFailure needle rest

def ownerFailureRowCovered (row : List FailureKind) : Bool :=
  containsFailure .staleMembership row &&
    containsFailure .missingCapability row &&
    containsFailure .missingWitness row &&
    containsFailure .routeUnavailable row

def firstUnknownField : List M6CoreTemplate → Option M6CoreTemplate
  | [] => none
  | template :: rest =>
      if template.kind == .ownerRmw && !template.targetFieldKnown then some template
      else firstUnknownField rest

def firstUnderdeclaredFailureRow : List M6CoreTemplate → Option M6CoreTemplate
  | [] => none
  | template :: rest =>
      if template.kind == .ownerRmw && !ownerFailureRowCovered template.declaredFailureRow
      then some template
      else firstUnderdeclaredFailureRow rest

def checkedEvaluationFor (template : M6CoreTemplate) : Option CheckedEvaluation :=
  match template.kind with
  | .ownerRmw => some
      { kind := .ownerRmw,
        name := template.name,
        sourceSpan := template.sourceSpan,
        sourceToCoreKinds := template.sourceToCoreKinds }
  | .publishRelation => some
      { kind := .publishRelation,
        name := template.name,
        sourceSpan := template.sourceSpan,
        sourceToCoreKinds := template.sourceToCoreKinds }
  | .designatedPublishValue => some
      { kind := .designatedPublishValue,
        name := template.name,
        sourceSpan := template.sourceSpan,
        sourceToCoreKinds := template.sourceToCoreKinds }
  | .deferredWithAuth | .deferredVerify => none

def checkedEvaluations : List M6CoreTemplate → List CheckedEvaluation
  | [] => []
  | template :: rest =>
      match checkedEvaluationFor template with
      | some evaluation => evaluation :: checkedEvaluations rest
      | none => checkedEvaluations rest

def generatedObligationsFor (template : M6CoreTemplate) : List GeneratedObligation :=
  match template.kind with
  | .ownerRmw =>
      [ { kind := .failure .staleMembership,
          sourceSpan := template.sourceSpan }
      , { kind := .failure .missingCapability,
          sourceSpan := template.sourceSpan }
      , { kind := .failure .missingWitness,
          sourceSpan := template.sourceSpan }
      , { kind := .failure .routeUnavailable,
          sourceSpan := template.sourceSpan }
      , { kind := .capability,
          sourceSpan := template.sourceSpan }
      , { kind := .witness,
          sourceSpan := template.sourceSpan }
      , { kind := .evaluation .ownerRmw,
          sourceSpan := template.sourceSpan } ]
  | .publishRelation =>
      [ { kind := .authority,
          sourceSpan := template.sourceSpan }
      , { kind := .evaluation .publishRelation,
          sourceSpan := template.sourceSpan } ]
  | .designatedPublishValue =>
      [ { kind := .admittedEvaluatorAuthority, sourceSpan := template.sourceSpan }
      , { kind := .evaluation .designatedPublishValue, sourceSpan := template.sourceSpan } ]
  | .deferredWithAuth | .deferredVerify => []

def generatedObligations : List M6CoreTemplate → List GeneratedObligation
  | [] => []
  | template :: rest => generatedObligationsFor template ++ generatedObligations rest

def residualObligationsFor (template : M6CoreTemplate) : List ResidualObligation :=
  match template.kind with
  | .publishRelation =>
      [ { kind := .visibility,
          name := template.name,
          sourceSpan := template.sourceSpan }
      , { kind := .relationLifetime,
          name := template.name,
          sourceSpan := template.sourceSpan }
      , { kind := .fallbackValidity,
          name := template.name,
          sourceSpan := template.sourceSpan } ]
  | .deferredWithAuth =>
      [{ kind := .authDeferred, name := template.name, sourceSpan := template.sourceSpan }]
  | .deferredVerify =>
      [{ kind := .verifyDeferred, name := template.name, sourceSpan := template.sourceSpan }]
  | .designatedPublishValue =>
      [ { kind := .valueVisibilityRedaction,
          name := template.name,
          sourceSpan := template.sourceSpan } ]
  | .ownerRmw => []

def residualObligations : List M6CoreTemplate → List ResidualObligation
  | [] => []
  | template :: rest => residualObligationsFor template ++ residualObligations rest

def sourceToCoreEntriesFor (template : M6CoreTemplate) : List SourceToCoreEntry :=
  template.sourceToCoreKinds.map fun kind =>
    { kind := kind,
      sourceSpan := template.sourceSpan }

def sourceToCoreEntries : List M6CoreTemplate → List SourceToCoreEntry
  | [] => []
  | template :: rest => sourceToCoreEntriesFor template ++ sourceToCoreEntries rest

/- This projection stands for the source-to-Core map already carried by the
   accepted M6 classification.  M7 consumes this classification-owned map; it
   does not revisit the Surface AST to synthesize a second classification. -/
def M6Classification.sourceToCoreMap (classification : M6Classification) :
    List SourceToCoreEntry :=
  sourceToCoreEntries classification.templates

def ownerTargetRead : TypedStateRead :=
  { ownerLocus := .s, namespaceName := "player", index := some "target",
    field := some "hp", valueType := "Int", sourceSpan := .assignmentSpan }

def ownerSelfRead : TypedStateRead :=
  { ownerLocus := .s, namespaceName := "player", index := some "self",
    field := some "atk", valueType := "Int", sourceSpan := .assignmentSpan }

def designatedSelfRead : TypedStateRead :=
  { ownerLocus := .s, namespaceName := "player", index := some "self",
    field := some "atk", valueType := "Int", sourceSpan := .designatedSpan }

def ownerExpression : CheckedExpression :=
  { sourceSpan := .assignmentSpan,
    operatorChain := ["-"],
    integerLiterals := [],
    stateReads := [ownerTargetRead, ownerSelfRead],
    tree := .binary .subtract .assignmentSpan
      (.stateRead ownerTargetRead)
      (.stateRead ownerSelfRead) }

def designatedExpression : CheckedExpression :=
  { sourceSpan := .designatedSpan,
    operatorChain := ["+"],
    integerLiterals := [1],
    stateReads := [designatedSelfRead],
    tree := .binary .add .designatedSpan
      (.stateRead designatedSelfRead)
      (.integerLiteral 1 .designatedSpan) }

def ownerCoreProjection (_template : M6CoreTemplate) : CheckedCoreProjection :=
  { axes :=
      { semanticForm := .state,
        evaluationSite := .owner .s,
        trigger := .onEvent,
        authorityOrigin := .caller .self,
        materialization := .store },
    effectRow :=
      { entries :=
          [ { kind := .ownerRequest, sourceSpan := .assignmentSpan }
          , { kind := .ownerLocalRead, sourceSpan := .assignmentSpan }
          , { kind := .ownerWrite, sourceSpan := .assignmentSpan } ] },
    ownerRmw := some
      { ownerLocus := .s,
        target := { ownerLocus := .s, namespaceName := "player", index := some "target",
                    field := some "hp", sourceSpan := .targetFieldSpan },
        expression := ownerExpression,
        sameOwnerReads := ownerExpression.stateReads },
    relation := none,
    designated := none }

def relationCoreProjection (_template : M6CoreTemplate) : CheckedCoreProjection :=
  { axes :=
      { semanticForm := .relation,
        evaluationSite := .owner .s,
        trigger := .frontierAdvance,
        authorityOrigin := .ownerTransition .s,
        materialization := .publishRelation },
    effectRow :=
      { entries := [{ kind := .relationPublish, sourceSpan := .relationSpan }] },
    ownerRmw := none,
    relation := some
      { ownerLocus := .s,
        subject := "bird",
        subjectType := "Player",
        bindingFrontier := "bird_binding_frontier",
        consumerProjectionLocus := some .c,
        publishesRelationCarrier := true,
        primary := ({ anchor := "perch_anchor", epoch := "primary_epoch", transform := .translate 3 (-2) } : RelationOption),
        fallback := ({ anchor := "nest_anchor", epoch := "fallback_epoch", transform := .identity } : RelationOption) },
    designated := none }

def designatedCoreProjection (_template : M6CoreTemplate) : CheckedCoreProjection :=
  { axes :=
      { semanticForm := .value,
        evaluationSite := .designatedEvaluator .e,
        trigger := .logicalTick,
        authorityOrigin := .admittedEvaluator .e,
        materialization := .publishValue },
    effectRow :=
      { entries :=
          [ { kind := .designatedRemoteRequest, sourceSpan := .designatedSpan }
          , { kind := .designatedReceiptUse, sourceSpan := .designatedSpan }
          , { kind := .designatedValuePublish, sourceSpan := .designatedSpan } ] },
    ownerRmw := none,
    relation := none,
    designated := some
      { evaluator := "E",
        result := "result",
        triggerFrontier := "F",
        inputFrontier := InputFrontier.fromOrderedProducers ["F"],
        resultFrontier := "F",
        resultVersion := 1,
        evaluationPolicy := EvaluationPolicy.declaredDeterministic "inferred:E.result",
        observationPolicy := ObservationPolicy.declared "conservative",
        policyStamp :=
          (EvaluationPolicy.declaredDeterministic "inferred:E.result").stampWith
            (ObservationPolicy.declared "conservative"),
        expression := designatedExpression,
        generatedRemoteInputDependencies :=
          [{ designatedEvaluator := .e,
             requesterSite := .designatedEvaluator .e,
             authorityOrigin := .admittedEvaluator .e,
             sourceOwnerLocus := .s,
             typedStateRead :=
               { ownerLocus := .s, namespaceName := "player", index := some "self",
                 field := some "atk", valueType := "Int", sourceSpan := .designatedSpan },
             request :=
               { sourceOwnerLocus := .s,
                 typedStateRead :=
                   { ownerLocus := .s, namespaceName := "player", index := some "self",
                     field := some "atk", valueType := "Int", sourceSpan := .designatedSpan } },
             receiptUse :=
               { sourceOwnerLocus := .s,
                 typedStateRead :=
                   { ownerLocus := .s, namespaceName := "player", index := some "self",
                     field := some "atk", valueType := "Int", sourceSpan := .designatedSpan } } }] } }

def checkedCoreProjections : List M6CoreTemplate → List CheckedCoreProjection
  | [] => []
  | template :: rest =>
      match template.kind with
      | .ownerRmw => ownerCoreProjection template :: checkedCoreProjections rest
      | .publishRelation => relationCoreProjection template :: checkedCoreProjections rest
      | .designatedPublishValue => designatedCoreProjection template :: checkedCoreProjections rest
      | .deferredWithAuth | .deferredVerify => checkedCoreProjections rest

def sourceToCoreRef : SourceToCoreKind → String
  | .ownerRmw => "owner-rmw"
  | .ownerLocalRead => "owner-local-read"
  | .ownerLocalWrite => "owner-local-write"
  | .designatedDecision => "designated-decision"
  | .publishRelation => "publish-relation"
  | .consumerLocalProjection => "consumer-local-projection"
  | .deferredPolicy => "deferred-policy"

def stableSourceMapEntries (entries : List SourceToCoreEntry) (ordinal : Nat := 0) :
    List StableSourceMapEntry :=
  match entries with
  | [] => []
  | entry :: rest =>
      { ordinal,
        stableKey := ordinal,
        coreRef := sourceToCoreRef entry.kind,
        sourceSpan := entry.sourceSpan } :: stableSourceMapEntries rest (ordinal + 1)

def sourceSpanMapped (span : SourceSpan) : List SourceToCoreEntry → Bool
  | [] => false
  | entry :: rest => if entry.sourceSpan == span then true else sourceSpanMapped span rest

def everyTemplateSpanMapped : List M6CoreTemplate → List SourceToCoreEntry → Bool
  | [], _ => true
  | template :: rest, entries =>
      sourceSpanMapped template.sourceSpan entries && everyTemplateSpanMapped rest entries

def stableOrdinalsFrom : Nat → List StableSourceMapEntry → Bool
  | _, [] => true
  | expected, entry :: rest =>
      entry.ordinal == expected && entry.stableKey == expected &&
        stableOrdinalsFrom (expected + 1) rest

def checkedArtifact (classification : M6Classification) : CheckedElaboration :=
  let sourceToCoreMap := classification.sourceToCoreMap
  let residualObligations := residualObligations classification.templates
  let evaluations := checkedEvaluations classification.templates
  { sourceSpan := classification.sourceSpan,
    consumedM6Classification := classification,
    evaluations,
    checkedCoreProjections := checkedCoreProjections classification.templates,
    generatedObligations := generatedObligations classification.templates,
    residualObligations,
    sourceToCoreMap,
    stableSourceMap := stableSourceMapEntries sourceToCoreMap,
    executionIsAdmissible := !evaluations.isEmpty && residualObligations.isEmpty }

/- This is the exact M7 source-first pipeline name.  M6 diagnostics are
   forwarded with their span; only finite M7 checks add an M7 diagnostic. -/
def check_and_elaborate_surface_v0 (source : SurfaceV0Source) :
    Except SurfaceV0PipelineDiagnostics CheckedElaboration :=
  let classification := classify_surface_v0 source
  match classification.diagnostic with
  | some diagnostic => .error (singletonDiagnostic (forwardM6Diagnostic diagnostic.kind)
      diagnostic.sourceSpan)
  | none =>
      match source with
      | .m6BroadUnsupportedExpression =>
          .error (singletonDiagnostic .unsupportedExpression .unsupportedExpressionSpan)
      | .arithmeticRequiresInt =>
          .error (singletonDiagnostic .arithmeticRequiresInt .arithmeticOperatorSpan)
      | .canonicalAttackBundle | .maintainedBirdRelation |
        .designatedTickPublishResult | .crossOwnerWithoutReceipt |
        .relationAbsolutePosePublication | .relationConsumerMutation |
        .underdeclaredFailureRow | .duplicateDeclaration | .unknownStateField |
        .residualCannotExecute | .ownerOnlyNoResiduals |
        .crossOwnerAndUnderdeclaredFailure =>
          if classification.duplicateDeclaration then
            .error (singletonDiagnostic .duplicateDeclaration classification.duplicateDeclarationSpan)
          else match firstUnknownField classification.templates with
          | some template => .error (singletonDiagnostic .unknownStateField template.sourceSpan)
          | none => match firstUnderdeclaredFailureRow classification.templates with
          | some _template => .error (singletonDiagnostic .generatedFailureNotDeclared .failureRowSpan)
          | none => .ok (checkedArtifact classification)

def require_execution_admission (artifact : CheckedElaboration) :
    Except SurfaceV0PipelineDiagnostics Unit :=
  if artifact.executionIsAdmissible then .ok ()
  else .error (singletonDiagnostic .residualCannotExecute .verificationSpan)

/- RED resolved: the former undeclared `checkedElaborate` is now the explicit,
   finite `check_and_elaborate_surface_v0` total function above. -/
theorem check_and_elaborate_surface_v0_deterministic (source : SurfaceV0Source) :
    check_and_elaborate_surface_v0 source = check_and_elaborate_surface_v0 source := by
  rfl

theorem canonical_same_owner_failure_capability_witness_authority_evaluation_obligations :
    match check_and_elaborate_surface_v0 .canonicalAttackBundle with
    | .ok artifact =>
        artifact.evaluations =
          [ { kind := .ownerRmw,
              name := "attack",
              sourceSpan := .assignmentSpan,
              sourceToCoreKinds := [.ownerRmw, .ownerLocalRead, .ownerLocalWrite] }
          , { kind := .publishRelation,
              name := "bird_follow",
              sourceSpan := .relationSpan,
              sourceToCoreKinds := [.publishRelation, .consumerLocalProjection] }
          , { kind := .designatedPublishValue,
              name := "result",
              sourceSpan := .designatedSpan,
              sourceToCoreKinds := [.designatedDecision] } ] ∧
        artifact.generatedObligations =
          [ { kind := .failure .staleMembership,
              sourceSpan := .assignmentSpan }
          , { kind := .failure .missingCapability,
              sourceSpan := .assignmentSpan }
          , { kind := .failure .missingWitness,
              sourceSpan := .assignmentSpan }
          , { kind := .failure .routeUnavailable,
              sourceSpan := .assignmentSpan }
          , { kind := .capability,
              sourceSpan := .assignmentSpan }
          , { kind := .witness,
              sourceSpan := .assignmentSpan }
          , { kind := .evaluation .ownerRmw,
              sourceSpan := .assignmentSpan }
          , { kind := .authority,
              sourceSpan := .relationSpan }
          , { kind := .evaluation .publishRelation,
              sourceSpan := .relationSpan }
          , { kind := .admittedEvaluatorAuthority,
              sourceSpan := .designatedSpan }
          , { kind := .evaluation .designatedPublishValue,
              sourceSpan := .designatedSpan } ]
    | .error _ => False := by
  exact ⟨rfl, rfl⟩

theorem relation_projection_retains_visibility_lifetime_and_fallback_residuals :
    match check_and_elaborate_surface_v0 .maintainedBirdRelation with
    | .ok artifact =>
        artifact.residualObligations =
          [ { kind := .visibility,
              name := "bird_follow",
              sourceSpan := .relationSpan }
          , { kind := .relationLifetime,
              name := "bird_follow",
              sourceSpan := .relationSpan }
          , { kind := .fallbackValidity,
              name := "bird_follow",
              sourceSpan := .relationSpan } ] ∧
        artifact.executionIsAdmissible = false
    | .error _ => False := by
  exact ⟨rfl, rfl⟩

theorem designated_checked_evaluation_preserves_its_distinct_kind :
    match check_and_elaborate_surface_v0 .designatedTickPublishResult with
    | .ok artifact =>
        artifact.evaluations =
          [({ kind := .designatedPublishValue,
              name := "result",
              sourceSpan := .designatedSpan,
              sourceToCoreKinds := [.designatedDecision] } : CheckedEvaluation)] ∧
        artifact.residualObligations =
          [{ kind := .valueVisibilityRedaction, name := "result",
             sourceSpan := .designatedSpan }] ∧
        artifact.executionIsAdmissible = false
    | .error _ => False := by
  exact ⟨rfl, rfl, rfl⟩

theorem auth_verify_residuals_are_static_without_hidden_success :
    match check_and_elaborate_surface_v0 .residualCannotExecute with
    | .ok artifact =>
        artifact.evaluations = [] ∧
        artifact.generatedObligations = [] ∧
        artifact.executionIsAdmissible = false ∧
        artifact.residualObligations =
          [ { kind := .authDeferred,
              name := "MembershipAuth",
              sourceSpan := .authSpan }
          , { kind := .verifyDeferred,
              name := "finite_refinement",
              sourceSpan := .verificationSpan } ] ∧
        require_execution_admission artifact =
          .error (singletonDiagnostic .residualCannotExecute .verificationSpan)
    | .error _ => False := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem residual_obligations_grant_no_authority_capability_execution_effect_mutation_or_verdict
    (residual : ResidualObligation) :
    residual.is_non_executable = true ∧
    residual.grants_authority = false ∧
    residual.grants_capability = false ∧
    residual.emits_effect = false ∧
    residual.mutates_state = false ∧
    residual.emits_verdict = false := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem m6_diagnostic_is_forwarded_with_its_original_source_span_and_no_executable_core :
    check_and_elaborate_surface_v0 .crossOwnerWithoutReceipt =
      .error (singletonDiagnostic .crossOwnerOperandRequiresReceipt .rhsReferenceSpan) ∧
    (singletonDiagnostic .crossOwnerOperandRequiresReceipt .rhsReferenceSpan).has_executable_core = false := by
  exact ⟨rfl, rfl⟩

theorem m6_classification_precedes_m7_failure_row_check :
    check_and_elaborate_surface_v0 .crossOwnerAndUnderdeclaredFailure =
      .error (singletonDiagnostic .crossOwnerOperandRequiresReceipt .rhsReferenceSpan) ∧
    (singletonDiagnostic .crossOwnerOperandRequiresReceipt .rhsReferenceSpan).has_executable_core = false := by
  exact ⟨rfl, rfl⟩

theorem m6_broad_tokens_are_accepted_before_finite_m7_expression_rejection :
    (classify_surface_v0 .m6BroadUnsupportedExpression).diagnostic = none ∧
    check_and_elaborate_surface_v0 .m6BroadUnsupportedExpression =
      .error (singletonDiagnostic .unsupportedExpression .unsupportedExpressionSpan) ∧
    (classify_surface_v0 .arithmeticRequiresInt).diagnostic = none ∧
    check_and_elaborate_surface_v0 .arithmeticRequiresInt =
      .error (singletonDiagnostic .arithmeticRequiresInt .arithmeticOperatorSpan) := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem checked_expression_trees_retain_operand_order_spans_and_int_typing :
    ownerExpression.tree =
      .binary .subtract .assignmentSpan
        (.stateRead ownerTargetRead) (.stateRead ownerSelfRead) ∧
    designatedExpression.tree =
      .binary .add .designatedSpan
        (.stateRead designatedSelfRead) (.integerLiteral 1 .designatedSpan) ∧
    ownerTargetRead.sourceSpan = .assignmentSpan ∧
    ownerSelfRead.sourceSpan = .assignmentSpan ∧
    designatedSelfRead.sourceSpan = .designatedSpan ∧
    ownerExpression.tree.valueType = "Int" ∧
    designatedExpression.tree.valueType = "Int" := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem generated_authority_obligations_are_source_bound_and_never_success :
    match check_and_elaborate_surface_v0 .canonicalAttackBundle with
    | .ok artifact =>
        artifact.generatedObligations = generatedObligations
          (classify_surface_v0 .canonicalAttackBundle).templates ∧
        GeneratedObligation.grants_authority_success
          { kind := .authority, sourceSpan := .relationSpan } = false ∧
        GeneratedObligation.grants_authority_success
          { kind := .admittedEvaluatorAuthority, sourceSpan := .designatedSpan } = false ∧
        EffectEntry.sourceRef { kind := .ownerRequest, sourceSpan := .assignmentSpan } =
          .assignmentSpan ∧
        EffectEntry.sourceRef { kind := .designatedReceiptUse, sourceSpan := .designatedSpan } =
          .designatedSpan
    | .error _ => False := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem m7_rejects_failure_row_duplicate_and_unknown_field_without_executable_core :
    check_and_elaborate_surface_v0 .underdeclaredFailureRow =
      .error (singletonDiagnostic .generatedFailureNotDeclared .failureRowSpan) ∧
    check_and_elaborate_surface_v0 .duplicateDeclaration =
      .error (singletonDiagnostic .duplicateDeclaration .firstLocusSpan) ∧
    check_and_elaborate_surface_v0 .unknownStateField =
      .error (singletonDiagnostic .unknownStateField .assignmentSpan) ∧
    (singletonDiagnostic .generatedFailureNotDeclared .failureRowSpan).has_executable_core = false := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem canonical_source_to_core_map_preserves_each_m6_template_span :
    match check_and_elaborate_surface_v0 .canonicalAttackBundle with
    | .ok artifact => artifact.sourceToCoreMap =
        (classify_surface_v0 .canonicalAttackBundle).sourceToCoreMap
    | .error _ => False := by
  rfl

theorem canonical_checked_core_axes_effect_rows_and_total_stable_source_map :
    match check_and_elaborate_surface_v0 .canonicalAttackBundle with
    | .ok artifact =>
        artifact.consumedM6Classification = classify_surface_v0 .canonicalAttackBundle ∧
        artifact.checkedCoreProjections =
          [ ownerCoreProjection (ownerTemplate allGeneratedFailures .hp true)
          , relationCoreProjection relationTemplate
          , designatedCoreProjection designatedTemplate ] ∧
        everyTemplateSpanMapped (classify_surface_v0 .canonicalAttackBundle).templates
          artifact.sourceToCoreMap = true ∧
        stableOrdinalsFrom 0 artifact.stableSourceMap = true
    | .error _ => False := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem designated_value_core_preserves_m3_input_and_policy_stamp :
    match check_and_elaborate_surface_v0 .designatedTickPublishResult with
    | .ok artifact =>
        artifact.consumedM6Classification =
          classify_surface_v0 .designatedTickPublishResult ∧
        (match (designatedCoreProjection designatedTemplate).designated with
        | some core =>
            (designatedCoreProjection designatedTemplate).axes.semanticForm = .value ∧
            core.inputFrontier = InputFrontier.fromOrderedProducers ["F"] ∧
            core.evaluationPolicy =
              EvaluationPolicy.declaredDeterministic "inferred:E.result" ∧
            core.observationPolicy = ObservationPolicy.declared "conservative" ∧
            core.policyStamp =
              (EvaluationPolicy.declaredDeterministic "inferred:E.result").stampWith
                (ObservationPolicy.declared "conservative")
        | none => False)
    | .error _ => False := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem owner_only_checked_artifact_is_execution_admissible_with_evaluation_and_no_residuals :
    match check_and_elaborate_surface_v0 .ownerOnlyNoResiduals with
    | .ok artifact =>
        artifact.residualObligations = [] ∧
        artifact.executionIsAdmissible = true ∧
        require_execution_admission artifact = .ok ()
    | .error _ => False := by
  exact ⟨rfl, rfl, rfl⟩

#print axioms check_and_elaborate_surface_v0_deterministic
#print axioms canonical_same_owner_failure_capability_witness_authority_evaluation_obligations
#print axioms relation_projection_retains_visibility_lifetime_and_fallback_residuals
#print axioms designated_checked_evaluation_preserves_its_distinct_kind
#print axioms auth_verify_residuals_are_static_without_hidden_success
#print axioms residual_obligations_grant_no_authority_capability_execution_effect_mutation_or_verdict
#print axioms m6_diagnostic_is_forwarded_with_its_original_source_span_and_no_executable_core
#print axioms m6_classification_precedes_m7_failure_row_check
#print axioms m6_broad_tokens_are_accepted_before_finite_m7_expression_rejection
#print axioms checked_expression_trees_retain_operand_order_spans_and_int_typing
#print axioms generated_authority_obligations_are_source_bound_and_never_success
#print axioms m7_rejects_failure_row_duplicate_and_unknown_field_without_executable_core
#print axioms canonical_source_to_core_map_preserves_each_m6_template_span
#print axioms canonical_checked_core_axes_effect_rows_and_total_stable_source_map
#print axioms designated_value_core_preserves_m3_input_and_policy_stamp
#print axioms owner_only_checked_artifact_is_execution_admissible_with_evaluation_and_no_residuals

end MirTheoryV0M7CheckedElaboration
