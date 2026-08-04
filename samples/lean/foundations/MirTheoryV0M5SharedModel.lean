/-!
Mir Theory v0 M5 finite shared formal model.

This file is deliberately self-contained.  It uses one concrete finite
universe for a semantic Surface fragment, Core, configuration, transitions,
history, diagnostics, relation projection, and a cut/save/restore interface.
It is evidence for this declared finite profile only.
-/

namespace MirTheoryV0M5SharedModel

inductive SourceRef where
  | ownerRmwSource
  | crossOwnerSource
  | receiptSource
  | designatedSource
  | relationSource
  | consumerMutationSource
  | cutSource
  | saveSource
  | restoreSource
  | patchSource
deriving DecidableEq, Repr

/- The Surface fragment is a semantic tag plus source identity, not M6 grammar. -/
inductive SurfaceFragment where
  | ownerRmw (source : SourceRef)
  | crossOwnerWithoutReceipt (source : SourceRef)
  | explicitReceiptUse (source : SourceRef)
  | designatedEvaluation (source : SourceRef)
  | relationBind (source : SourceRef)
  | relationPublish (source : SourceRef)
  | relationProjection (source : SourceRef)
  | consumerRelationMutation (source : SourceRef)
  | atomicCut (source : SourceRef)
  | save (source : SourceRef)
  | restore (source : SourceRef)
  | patchOperation (source : SourceRef)
deriving DecidableEq, Repr

/- The designated-result producer set and a relation activation frontier cannot unify. -/
inductive ResultFrontier where
  | producerSetOwner
  | producerSetOwnerForeign
deriving DecidableEq, Repr

inductive ResultVersion where
  | version1
deriving DecidableEq, Repr

inductive EvaluationKind where
  | ownerStore
  | receiptUse
  | designated
  | relationProjection
deriving DecidableEq, Repr

inductive EvaluationSite where
  | owner
  | evaluator
  | consumer
deriving DecidableEq, Repr

inductive Materialization where
  | store
  | publishValue
  | publishRelation
  | localOnly
deriving DecidableEq, Repr

structure EvalPlan where
  kind : EvaluationKind
  source : SourceRef
  site : EvaluationSite
  materialization : Materialization
  resultFrontier : Option ResultFrontier
deriving DecidableEq, Repr

def ownerRmwPlan : EvalPlan :=
  { kind := .ownerStore
    source := .ownerRmwSource
    site := .owner
    materialization := .store
    resultFrontier := none }

def relationPublishPlan (source : SourceRef) : EvalPlan :=
  { kind := .relationProjection
    source := source
    site := .owner
    materialization := .publishRelation
    resultFrontier := none }

def consumerRelationProjectionPlan (source : SourceRef) : EvalPlan :=
  { kind := .relationProjection
    source := source
    site := .consumer
    materialization := .localOnly
    resultFrontier := none }

def validPlan (plan : EvalPlan) : Bool :=
  match plan.site, plan.materialization with
  | .owner, .store =>
      (plan.kind == .ownerStore) || (plan.kind == .receiptUse)
  | .owner, .publishRelation =>
      (plan.kind == .relationProjection) && plan.resultFrontier.isNone
  | .evaluator, .publishValue => plan.resultFrontier.isSome
  | .consumer, .localOnly => plan.kind == .relationProjection
  | _, _ => false

inductive Core where
  | ownerRmw (source : SourceRef) (plan : EvalPlan)
  | explicitReceiptUse (source : SourceRef) (plan : EvalPlan)
  | designatedEvaluation (source : SourceRef) (plan : EvalPlan)
  | relationBind (source : SourceRef)
  | relationPublish (source : SourceRef) (plan : EvalPlan)
  | relationProjection (source : SourceRef) (plan : EvalPlan)
  | atomicCut (source : SourceRef)
  | save (source : SourceRef)
  | restore (source : SourceRef)
deriving DecidableEq, Repr

inductive StaticDiagnostic where
  | crossOwnerRequiresReceipt (source : SourceRef)
  | consumerCannotMutateRelation (source : SourceRef)
  | patchOutsideM5Fragment (source : SourceRef)
deriving DecidableEq, Repr

inductive Elaboration where
  | core (value : Core)
  | diagnostic (value : StaticDiagnostic)
deriving DecidableEq, Repr

def elaborate : SurfaceFragment → Elaboration
  | .ownerRmw source => .core (.ownerRmw source
      ({ kind := .ownerStore
         source := source
         site := .owner
         materialization := .store
         resultFrontier := none }))
  | .crossOwnerWithoutReceipt source => .diagnostic (.crossOwnerRequiresReceipt source)
  | .explicitReceiptUse source => .core (.explicitReceiptUse source
      ({ kind := .receiptUse
         source := source
         site := .owner
         materialization := .store
         resultFrontier := none }))
  | .designatedEvaluation source => .core (.designatedEvaluation source
      ({ kind := .designated
         source := source
         site := .evaluator
         materialization := .publishValue
         resultFrontier := some .producerSetOwner }))
  | .relationBind source => .core (.relationBind source)
  | .relationPublish source => .core (.relationPublish source
      (relationPublishPlan source))
  | .relationProjection source => .core (.relationProjection source
      (consumerRelationProjectionPlan source))
  | .consumerRelationMutation source => .diagnostic (.consumerCannotMutateRelation source)
  | .atomicCut source => .core (.atomicCut source)
  | .save source => .core (.save source)
  | .restore source => .core (.restore source)
  | .patchOperation source => .diagnostic (.patchOutsideM5Fragment source)

inductive RelationFrontier where
  | frontier10
  | frontier11
deriving DecidableEq, Repr

inductive Locus where
  | owner
  | evaluator
  | consumer
deriving DecidableEq, Repr

inductive Principal where
  | requester
  | relationOwner
  | relationConsumer
deriving DecidableEq, Repr

inductive RelationId where
  | maintained
deriving DecidableEq, Repr

inductive RefId where
  | subject
  | primaryAnchor
  | fallbackAnchor
deriving DecidableEq, Repr

inductive BindingSlot where
  | primary
  | fallback
deriving DecidableEq, Repr

inductive Epoch where
  | epoch1
  | epoch2
  | epoch3
deriving DecidableEq, Repr

def epochRank : Epoch → Nat
  | .epoch1 => 1
  | .epoch2 => 2
  | .epoch3 => 3

def freshEpoch (old new : Epoch) : Bool :=
  decide (epochRank old < epochRank new)

inductive RestrictionLabel where
  | publicLabel
  | restrictedLabel
  | privateLabel
deriving DecidableEq, Repr

def restrictionRank : RestrictionLabel → Nat
  | .publicLabel => 0
  | .restrictedLabel => 1
  | .privateLabel => 2

def atLeastAsRestricted (candidate baseline : RestrictionLabel) : Bool :=
  decide (restrictionRank baseline ≤ restrictionRank candidate)

def labelJoin (left right : RestrictionLabel) : RestrictionLabel :=
  if restrictionRank left < restrictionRank right then right else left

inductive Coordinate where
  | coordinate0
  | coordinate5
  | coordinate10
  | coordinate15
deriving DecidableEq, Repr

inductive RelativeTransform where
  | zero
  | plus5
  | minus5
deriving DecidableEq, Repr

def checkedTransform : Coordinate → RelativeTransform → Option Coordinate
  | coordinate, .zero => some coordinate
  | .coordinate0, .plus5 => some .coordinate5
  | .coordinate5, .plus5 => some .coordinate10
  | .coordinate10, .plus5 => some .coordinate15
  | .coordinate15, .plus5 => none
  | .coordinate0, .minus5 => none
  | .coordinate5, .minus5 => some .coordinate0
  | .coordinate10, .minus5 => some .coordinate5
  | .coordinate15, .minus5 => some .coordinate10

structure RelationDef where
  relation : RelationId
  owner : Locus
  subject : RefId
  primaryAnchor : RefId
  fallbackAnchor : RefId
  transform : RelativeTransform
  relationLabel : RestrictionLabel
deriving DecidableEq, Repr

def relationAcyclic (relation : RelationDef) : Bool :=
  (relation.subject != relation.primaryAnchor) &&
    (relation.subject != relation.fallbackAnchor)

structure BindingState where
  selected : BindingSlot
  lineage : Nat
  bindingEpoch : Epoch
  witnessEpoch : Epoch
  activationFrontier : RelationFrontier
  primaryAnchorEpoch : Epoch
  fallbackAnchorEpoch : Epoch
deriving DecidableEq, Repr

def selectedAnchor (relation : RelationDef) : BindingSlot → RefId
  | .primary => relation.primaryAnchor
  | .fallback => relation.fallbackAnchor

def selectedAnchorEpoch (binding : BindingState) : BindingSlot → Epoch
  | .primary => binding.primaryAnchorEpoch
  | .fallback => binding.fallbackAnchorEpoch

inductive CredentialKind where
  | capability
  | witness
deriving DecidableEq, Repr

structure RelationCredential where
  kind : CredentialKind
  principal : Principal
  relation : RelationId
  bindingEpoch : Epoch
  membershipEpoch : Epoch
  live : Bool
deriving DecidableEq, Repr

structure PresentationSample where
  consumer : Principal
  anchor : RefId
  epoch : Epoch
  frontier : RelationFrontier
  coordinate : Coordinate
  label : RestrictionLabel
  releaseAdmitted : Bool
deriving DecidableEq, Repr

/- A presentation context is read-side input, distinct from the published binding carrier. -/
structure PresentationContext where
  consumer : Principal
  frontier : RelationFrontier
  primary : Option PresentationSample
  fallback : Option PresentationSample
deriving DecidableEq, Repr

structure ProjectedRelation where
  relation : RelationDef
  binding : BindingState
  selectedAnchor : RefId
  selectedAnchorEpoch : Epoch
  activationFrontier : RelationFrontier
deriving DecidableEq, Repr

def makeProjection (relation : RelationDef) (binding : BindingState) : Option ProjectedRelation :=
  if relation.owner == .owner && relationAcyclic relation then
    some
      { relation
        binding
        selectedAnchor := selectedAnchor relation binding.selected
        selectedAnchorEpoch := selectedAnchorEpoch binding binding.selected
        activationFrontier := binding.activationFrontier }
  else
    none

inductive ProjectionReject where
  | relationCycle
  | missingSample
  | sampleNotAdmitted
  | staleSample
  | splitFrame
  | weakRelease
  | transformOverflow
  | derivedCoordinateOverflow
deriving DecidableEq, Repr

inductive ProjectionOutcome where
  | evaluated (coordinate : Coordinate) (label : RestrictionLabel)
  | rejected (reason : ProjectionReject)
deriving DecidableEq, Repr

def sampleReject (consumer : Principal) (anchor : RefId) (epoch : Epoch)
    (frontier : RelationFrontier) : Option PresentationSample → Option ProjectionReject
  | none => some .missingSample
  | some sample =>
      if sample.consumer != consumer || !sample.releaseAdmitted then some .sampleNotAdmitted
      else if sample.anchor != anchor || sample.epoch != epoch then some .staleSample
      else if sample.frontier != frontier then some .splitFrame
      else none

def contextReject (projection : ProjectedRelation)
    (context : PresentationContext) : Option ProjectionReject :=
  if !relationAcyclic projection.relation then some .relationCycle
  else if context.consumer != .relationConsumer then some .sampleNotAdmitted
  else if context.frontier != projection.activationFrontier then some .splitFrame
  else
    match sampleReject context.consumer projection.relation.primaryAnchor
        projection.binding.primaryAnchorEpoch context.frontier context.primary with
    | some reason => some reason
    | none => sampleReject context.consumer projection.relation.fallbackAnchor
        projection.binding.fallbackAnchorEpoch context.frontier context.fallback

def admittedInputLabel : Option PresentationSample → RestrictionLabel
  | none => .privateLabel
  | some sample => sample.label

def derivedLabel (projection : ProjectedRelation)
    (context : PresentationContext) : RestrictionLabel :=
  labelJoin (labelJoin projection.relation.relationLabel
    (admittedInputLabel context.primary)) (admittedInputLabel context.fallback)

def evaluateProjection (projection : ProjectedRelation) (context : PresentationContext)
    (requested : RestrictionLabel) : ProjectionOutcome :=
  match contextReject projection context with
  | some reason => .rejected reason
  | none =>
      let required := derivedLabel projection context
      if !atLeastAsRestricted requested required then .rejected .weakRelease
      else
        match context.primary, context.fallback with
        | some primary, some fallback =>
            let selected := match projection.binding.selected with
              | .primary => primary
              | .fallback => fallback
            match checkedTransform selected.coordinate projection.relation.transform with
            | some coordinate => .evaluated coordinate required
            | none =>
                match projection.relation.transform with
                | .zero => .rejected .transformOverflow
                | _ => .rejected .derivedCoordinateOverflow
        | _, _ => .rejected .missingSample

def evaluateOwnerHeld (relation : RelationDef) (binding : BindingState)
    (context : PresentationContext) (requested : RestrictionLabel) : ProjectionOutcome :=
  match makeProjection relation binding with
  | some projection => evaluateProjection projection context requested
  | none => .rejected .relationCycle

def canonicalRelation : RelationDef :=
  { relation := .maintained
    owner := .owner
    subject := .subject
    primaryAnchor := .primaryAnchor
    fallbackAnchor := .fallbackAnchor
    transform := .plus5
    relationLabel := .restrictedLabel }

def canonicalBinding : BindingState :=
  { selected := .primary
    lineage := 7
    bindingEpoch := .epoch2
    witnessEpoch := .epoch2
    activationFrontier := .frontier10
    primaryAnchorEpoch := .epoch2
    fallbackAnchorEpoch := .epoch3 }

def canonicalProjection : ProjectedRelation :=
  { relation := canonicalRelation
    binding := canonicalBinding
    selectedAnchor := .primaryAnchor
    selectedAnchorEpoch := .epoch2
    activationFrontier := .frontier10 }

def canonicalPrimary : PresentationSample :=
  { consumer := .relationConsumer
    anchor := .primaryAnchor
    epoch := .epoch2
    frontier := .frontier10
    coordinate := .coordinate10
    label := .restrictedLabel
    releaseAdmitted := true }

def canonicalFallback : PresentationSample :=
  { consumer := .relationConsumer
    anchor := .fallbackAnchor
    epoch := .epoch3
    frontier := .frontier10
    coordinate := .coordinate5
    label := .publicLabel
    releaseAdmitted := true }

def canonicalPresentationContext : PresentationContext :=
  { consumer := .relationConsumer
    frontier := .frontier10
    primary := some canonicalPrimary
    fallback := some canonicalFallback }

def nonAdmittedPrimaryContext : PresentationContext :=
  { canonicalPresentationContext with primary := some { canonicalPrimary with releaseAdmitted := false } }

def overflowingPresentationContext : PresentationContext :=
  { canonicalPresentationContext with primary := some { canonicalPrimary with coordinate := .coordinate15 } }

inductive ReceiptStatus where
  | success (coordinate : Coordinate)
  | failure
deriving DecidableEq, Repr

structure Receipt where
  caller : Principal
  producer : Locus
  target : Locus
  requestOccurrence : Nat
  serveOccurrence : Nat
  replyOccurrence : Nat
  receiveOccurrence : Nat
  released : Bool
  status : ReceiptStatus
deriving DecidableEq, Repr

def usableReceipt (receipt : Receipt) : Bool :=
  (receipt.target == .owner) &&
    decide (0 < receipt.requestOccurrence) &&
    decide (receipt.requestOccurrence < receipt.serveOccurrence) &&
    decide (receipt.serveOccurrence < receipt.replyOccurrence) &&
    decide (receipt.replyOccurrence < receipt.receiveOccurrence) &&
    receipt.released &&
    match receipt.status with
    | .success _ => true
    | .failure => false

inductive RuntimeReject where
  | missingCapability
  | missingWitness
  | invalidReceipt
  | staleRelationCredential
  | nonOwnerRelationMutation
  | consumerCannotMaterialize
  | missingDecidedResult
  | missingPublishedRelation
  | invalidRelationPublication
  | restoreRejected
deriving DecidableEq, Repr

structure ObservationEvent where
  source : SourceRef
  observer : Principal
  label : RestrictionLabel
deriving DecidableEq, Repr

def observeProjection (source : SourceRef) (outcome : ProjectionOutcome) : Option ObservationEvent :=
  match outcome with
  | .evaluated _ label => some { source, observer := .relationConsumer, label }
  | .rejected _ => none

inductive TraceKind where
  | ownerRequest
  | ownerWrite
  | receiptStored
  | designated
  | consumed
  | relationBound
  | relationDegraded
  | relationReacquired
  | relationPublished
  | consumerProjected (outcome : ProjectionOutcome)
  | observed (event : ObservationEvent)
  | atomicCut
  | rejected (reason : RuntimeReject)
deriving DecidableEq, Repr

structure TraceRow where
  source : SourceRef
  kind : TraceKind
deriving DecidableEq, Repr

/- A save is tied to this nominal cut carrier, never to a result or relation frontier. -/
structure AtomicCut where
  history : List TraceRow
deriving DecidableEq, Repr

/- M5 carries only an explicitly inactive patch slot; patch semantics stay deferred. -/
inductive PatchState where
  | inactive
deriving DecidableEq, Repr

structure DecidedResult where
  frontier : ResultFrontier
  version : ResultVersion
  coordinate : Coordinate
  consumed : Bool
deriving DecidableEq, Repr

structure Config where
  history : List TraceRow
  pendingRequests : List SourceRef
  hp : Nat
  membershipEpoch : Epoch
  ownerCapability : RelationCredential
  ownerWitness : RelationCredential
  leaseLive : Bool
  receipt : Option Receipt
  decided : Option DecidedResult
  relation : RelationDef
  binding : BindingState
  publishedRelation : Option ProjectedRelation
  patch : PatchState
deriving DecidableEq, Repr

def credentialMatches (kind : CredentialKind) (credential : RelationCredential)
    (config : Config) : Bool :=
  (credential.kind == kind) &&
    (credential.principal == .relationOwner) &&
    (credential.relation == config.relation.relation) &&
    (credential.bindingEpoch == config.binding.bindingEpoch) &&
    (credential.membershipEpoch == config.membershipEpoch) && credential.live

def currentOwnerCredentials (config : Config) (capability witness : RelationCredential) : Bool :=
  (capability == config.ownerCapability) && (witness == config.ownerWitness) &&
    credentialMatches .capability capability config && credentialMatches .witness witness config

def publishedRelationMatches (config : Config) : Bool :=
  match config.publishedRelation with
  | none => true
  | some projection =>
      (projection.relation == config.relation) && (projection.binding == config.binding) &&
        (projection.activationFrontier == config.binding.activationFrontier)

/- The concrete finite `WellFormed` definition has no opaque premise. -/
def wellFormed (config : Config) : Bool :=
  relationAcyclic config.relation &&
    credentialMatches .capability config.ownerCapability config &&
    credentialMatches .witness config.ownerWitness config && config.leaseLive &&
    publishedRelationMatches config &&
    match config.patch with
    | .inactive => true

def appendTrace (config : Config) (source : SourceRef) (kind : TraceKind) : Config :=
  { config with history := config.history ++ [{ source, kind }] }

def semanticFallback (binding : BindingState) : BindingState :=
  match binding.selected with
  | .primary => { binding with selected := .fallback }
  | .fallback => binding

def advanceFrontier : RelationFrontier → RelationFrontier
  | .frontier10 => .frontier11
  | .frontier11 => .frontier11

inductive SemanticStep where
  | ownerRmw (hasCapability hasWitness : Bool)
  | storeReceipt (receipt : Receipt)
  | decide (frontier : ResultFrontier)
  | consume
  | relationBind (actor : Principal) (capability witness : RelationCredential)
  | relationDegrade (actor : Principal) (capability witness : RelationCredential)
  | relationReacquire (actor : Principal) (currentCapability currentWitness : RelationCredential)
      (nextCapability nextWitness : RelationCredential) (newBindingEpoch newWitnessEpoch : Epoch)
  | relationPublish (actor : Principal) (capability witness : RelationCredential)
  | consumerMaterialize (target : Materialization)
  | consumerProject (context : PresentationContext) (requested : RestrictionLabel)
  | consumerJMutation
  | cut
deriving DecidableEq, Repr

structure StepOutcome where
  config : Config
  rejection : Option RuntimeReject
  projection : Option ProjectionOutcome
deriving DecidableEq, Repr

def rejectedStep (config : Config) (source : SourceRef) (reason : RuntimeReject) : StepOutcome :=
  { config := appendTrace config source (.rejected reason), rejection := some reason, projection := none }

def successfulStep (config : Config) (source : SourceRef) (kind : TraceKind) : StepOutcome :=
  { config := appendTrace config source kind, rejection := none, projection := none }

def projectedStep (config : Config) (source : SourceRef) (outcome : ProjectionOutcome) : StepOutcome :=
  { config := appendTrace config source (.consumerProjected outcome)
    rejection := none
    projection := some outcome }

def applyStep (config : Config) : SemanticStep → StepOutcome
  | .ownerRmw hasCapability hasWitness =>
      if hasCapability && hasWitness && decide (10 ≤ config.hp) then
        successfulStep { config with hp := config.hp - 10 } .ownerRmwSource .ownerWrite
      else if !hasCapability then rejectedStep config .ownerRmwSource .missingCapability
      else rejectedStep config .ownerRmwSource .missingWitness
  | .storeReceipt receipt =>
      if usableReceipt receipt then
        successfulStep { config with receipt := some receipt } .receiptSource .receiptStored
      else rejectedStep config .receiptSource .invalidReceipt
  | .decide frontier =>
      match config.decided with
      | some _ => successfulStep config .designatedSource .designated
      | none =>
          let result : DecidedResult :=
            { frontier := frontier
              version := .version1
              coordinate := .coordinate5
              consumed := false }
          successfulStep
            ({ config with decided := some result })
            .designatedSource .designated
  | .consume =>
      match config.decided with
      | none => rejectedStep config .designatedSource .missingDecidedResult
      | some result =>
          successfulStep { config with decided := some { result with consumed := true } }
            .designatedSource .consumed
  | .relationBind actor capability witness =>
      if actor == .relationOwner && currentOwnerCredentials config capability witness then
        successfulStep config .relationSource .relationBound
      else rejectedStep config .relationSource .nonOwnerRelationMutation
  | .relationDegrade actor capability witness =>
      if actor == .relationOwner then
        if currentOwnerCredentials config capability witness then
          successfulStep
            { config with
              binding := semanticFallback config.binding
              publishedRelation := none }
            .relationSource .relationDegraded
        else rejectedStep config .relationSource .staleRelationCredential
      else rejectedStep config .relationSource .nonOwnerRelationMutation
  | .relationReacquire actor currentCapability currentWitness nextCapability nextWitness
      newBindingEpoch newWitnessEpoch =>
      if actor == .relationOwner && currentOwnerCredentials config currentCapability currentWitness &&
          freshEpoch config.binding.bindingEpoch newBindingEpoch &&
          freshEpoch config.binding.witnessEpoch newWitnessEpoch then
        let nextBinding : BindingState :=
          { config.binding with
            selected := .primary
            lineage := config.binding.lineage + 1
            bindingEpoch := newBindingEpoch
            witnessEpoch := newWitnessEpoch
            activationFrontier := advanceFrontier config.binding.activationFrontier
            primaryAnchorEpoch := newBindingEpoch }
        let candidate : Config :=
          { config with
            binding := nextBinding
            ownerCapability := nextCapability
            ownerWitness := nextWitness
            publishedRelation := none }
        if currentOwnerCredentials candidate nextCapability nextWitness then
          successfulStep candidate .relationSource .relationReacquired
        else rejectedStep config .relationSource .staleRelationCredential
      else rejectedStep config .relationSource .staleRelationCredential
  | .relationPublish actor capability witness =>
      if actor == .relationOwner && currentOwnerCredentials config capability witness then
        match makeProjection config.relation config.binding with
        | some projection =>
            let published : Config := { config with publishedRelation := some projection }
            successfulStep published .relationSource .relationPublished
        | none => rejectedStep config .relationSource .invalidRelationPublication
      else rejectedStep config .relationSource .nonOwnerRelationMutation
  | .consumerMaterialize target =>
      match target with
      | .localOnly => successfulStep config .relationSource .consumed
      | _ => rejectedStep config .relationSource .consumerCannotMaterialize
  | .consumerProject context requested =>
      match config.publishedRelation with
      | none => rejectedStep config .relationSource .missingPublishedRelation
      | some projection => projectedStep config .relationSource (evaluateProjection projection context requested)
  | .consumerJMutation => rejectedStep config .relationSource .nonOwnerRelationMutation
  | .cut => successfulStep config .cutSource .atomicCut

def endsWithCut : List TraceRow → Bool
  | [] => false
  | row :: [] =>
      match row.kind with
      | .atomicCut => true
      | _ => false
  | _ :: rest => endsWithCut rest

structure SaveObject where
  cut : AtomicCut
  history : List TraceRow
  pendingRequests : List SourceRef
  hp : Nat
  membershipEpoch : Epoch
  ownerCapability : RelationCredential
  ownerWitness : RelationCredential
  leaseLive : Bool
  receipt : Option Receipt
  decided : Option DecidedResult
  relation : RelationDef
  binding : BindingState
  publishedRelation : Option ProjectedRelation
  patch : PatchState
deriving DecidableEq, Repr

def consistentCut (saved : SaveObject) : Bool :=
  saved.cut.history == saved.history

def saveObject (config : Config) : Option SaveObject :=
  if wellFormed config && endsWithCut config.history then
    some
      { cut := { history := config.history }
        history := config.history
        pendingRequests := config.pendingRequests
        hp := config.hp
        membershipEpoch := config.membershipEpoch
        ownerCapability := config.ownerCapability
        ownerWitness := config.ownerWitness
        leaseLive := config.leaseLive
        receipt := config.receipt
        decided := config.decided
        relation := config.relation
        binding := config.binding
        publishedRelation := config.publishedRelation
        patch := config.patch }
  else none

def configOfSave (saved : SaveObject) : Config :=
  { history := saved.history
    pendingRequests := saved.pendingRequests
    hp := saved.hp
    membershipEpoch := saved.membershipEpoch
    ownerCapability := saved.ownerCapability
    ownerWitness := saved.ownerWitness
    leaseLive := saved.leaseLive
    receipt := saved.receipt
    decided := saved.decided
    relation := saved.relation
    binding := saved.binding
    publishedRelation := saved.publishedRelation
    patch := saved.patch }

def restore (saved : SaveObject) : Option Config :=
  let config := configOfSave saved
  if consistentCut saved && endsWithCut saved.history && wellFormed config then some config else none

def canonicalCapability : RelationCredential :=
  { kind := .capability
    principal := .relationOwner
    relation := .maintained
    bindingEpoch := .epoch2
    membershipEpoch := .epoch2
    live := true }

def canonicalWitness : RelationCredential :=
  { kind := .witness
    principal := .relationOwner
    relation := .maintained
    bindingEpoch := .epoch2
    membershipEpoch := .epoch2
    live := true }

def staleCapability : RelationCredential :=
  { canonicalCapability with bindingEpoch := .epoch1 }

def staleWitness : RelationCredential :=
  { canonicalWitness with membershipEpoch := .epoch1 }

def canonicalConfig : Config :=
  { history := []
    pendingRequests := []
    hp := 20
    membershipEpoch := .epoch2
    ownerCapability := canonicalCapability
    ownerWitness := canonicalWitness
    leaseLive := true
    receipt := none
    decided := none
    relation := canonicalRelation
    binding := canonicalBinding
    publishedRelation := none
    patch := .inactive }

def canonicalCutConfig : Config :=
  { canonicalConfig with history := [{ source := .cutSource, kind := .atomicCut }] }

example :
    elaborate (.relationBind .relationSource) = .core (.relationBind .relationSource) := by
  rfl

def canonicalReceipt : Receipt :=
  { caller := .requester
    producer := .owner
    target := .owner
    requestOccurrence := 1
    serveOccurrence := 2
    replyOccurrence := 3
    receiveOccurrence := 4
    released := true
    status := .success .coordinate5 }

def failedReceipt : Receipt :=
  { canonicalReceipt with status := .failure }

def nextCapability : RelationCredential :=
  { canonicalCapability with bindingEpoch := .epoch3 }

def nextWitness : RelationCredential :=
  { canonicalWitness with bindingEpoch := .epoch3 }

def mixedOwnerBindingPublishConfig : Config :=
  let bound := applyStep canonicalConfig (.relationBind .relationOwner canonicalCapability canonicalWitness)
  (applyStep bound.config (.relationPublish .relationOwner canonicalCapability canonicalWitness)).config

def mixedOwnerBindingPublishProjectionOutcome : StepOutcome :=
  applyStep mixedOwnerBindingPublishConfig
    (.consumerProject canonicalPresentationContext .restrictedLabel)

def mixedOwnerBindingPublishProjectionConfig : Config :=
  mixedOwnerBindingPublishProjectionOutcome.config

def mixedOwnerBindingPublishProjectionTrace : List TraceKind :=
  mixedOwnerBindingPublishProjectionConfig.history.map TraceRow.kind

def publishedRelationCutConfig : Config :=
  { mixedOwnerBindingPublishConfig with
    history := mixedOwnerBindingPublishConfig.history ++
      [{ source := .cutSource, kind := .atomicCut }] }

def supportedM5Steps : List SemanticStep :=
  [ .ownerRmw true true
  , .storeReceipt canonicalReceipt
  , .decide .producerSetOwner
  , .consume
  , .relationBind .relationOwner canonicalCapability canonicalWitness
  , .relationDegrade .relationOwner canonicalCapability canonicalWitness
  , .relationReacquire .relationOwner canonicalCapability canonicalWitness nextCapability nextWitness
      .epoch3 .epoch3
  , .relationPublish .relationOwner canonicalCapability canonicalWitness
  , .consumerMaterialize .store
  , .consumerMaterialize .publishValue
  , .consumerProject canonicalPresentationContext .restrictedLabel
  , .consumerJMutation
  , .cut ]

def slotRank : BindingSlot → Nat
  | .primary => 0
  | .fallback => 1

theorem elaboration_deterministic {surface : SurfaceFragment} {first second : Elaboration}
    (firstResult : elaborate surface = first)
    (secondResult : elaborate surface = second) :
    first = second := by
  exact firstResult.symm.trans secondResult

theorem owner_rmw_plan_is_valid : validPlan ownerRmwPlan = true := by
  rfl

theorem owner_relation_publish_plan_is_valid :
    validPlan (relationPublishPlan .relationSource) = true := by
  rfl

theorem consumer_relation_projection_plan_is_valid :
    validPlan (consumerRelationProjectionPlan .relationSource) = true := by
  rfl

theorem unannotated_cross_owner_is_static_diagnostic :
    elaborate (.crossOwnerWithoutReceipt .crossOwnerSource) =
      .diagnostic (.crossOwnerRequiresReceipt .crossOwnerSource) := by
  rfl

theorem explicit_receipt_requires_complete_released_success :
    usableReceipt canonicalReceipt = true ∧ usableReceipt failedReceipt = false := by
  exact ⟨rfl, rfl⟩

theorem duplicate_designated_decision_is_stable :
    let first := applyStep canonicalConfig (.decide .producerSetOwner)
    let duplicate := applyStep first.config (.decide .producerSetOwnerForeign)
    duplicate.config.decided = first.config.decided := by
  rfl

theorem project_then_evaluate_matches_owner_held_relation :
    evaluateProjection canonicalProjection canonicalPresentationContext .restrictedLabel =
      evaluateOwnerHeld canonicalRelation canonicalBinding canonicalPresentationContext .restrictedLabel := by
  rfl

theorem semantic_fallback_is_monotone (binding : BindingState) :
    slotRank binding.selected ≤ slotRank (semanticFallback binding).selected := by
  cases binding with
  | mk selected lineage bindingEpoch witnessEpoch activationFrontier primaryAnchorEpoch fallbackAnchorEpoch =>
      cases selected <;> simp [semanticFallback, slotRank]

theorem fresh_reacquire_starts_a_new_relation_lineage :
    let result := applyStep canonicalConfig
      (.relationReacquire .relationOwner canonicalCapability canonicalWitness nextCapability nextWitness
        .epoch3 .epoch3)
    result.rejection = none ∧ result.config.binding.selected = .primary ∧
      result.config.binding.lineage = canonicalBinding.lineage + 1 ∧
      result.config.binding.bindingEpoch = .epoch3 ∧ result.config.binding.witnessEpoch = .epoch3 := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem per_sample_release_is_required :
    evaluateProjection canonicalProjection nonAdmittedPrimaryContext .restrictedLabel =
      .rejected .sampleNotAdmitted := by
  rfl

theorem checked_transform_rejects_derived_coordinate_overflow :
    evaluateProjection canonicalProjection overflowingPresentationContext .restrictedLabel =
      .rejected .derivedCoordinateOverflow := by
  rfl

theorem derived_release_label_dominates_relation_and_each_admitted_input :
    derivedLabel canonicalProjection canonicalPresentationContext = .restrictedLabel ∧
      atLeastAsRestricted .restrictedLabel canonicalRelation.relationLabel = true ∧
      atLeastAsRestricted .restrictedLabel canonicalPrimary.label = true ∧
      atLeastAsRestricted .restrictedLabel canonicalFallback.label = true := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem owner_binding_publish_then_consumer_local_projection :
    mixedOwnerBindingPublishProjectionTrace =
      [.relationBound, .relationPublished,
        .consumerProjected (.evaluated .coordinate15 .restrictedLabel)] ∧
      mixedOwnerBindingPublishConfig.publishedRelation = some canonicalProjection ∧
      mixedOwnerBindingPublishProjectionOutcome.rejection = none ∧
      mixedOwnerBindingPublishProjectionOutcome.projection =
        some (.evaluated .coordinate15 .restrictedLabel) ∧
      mixedOwnerBindingPublishProjectionConfig =
        appendTrace mixedOwnerBindingPublishConfig .relationSource
          (.consumerProjected (.evaluated .coordinate15 .restrictedLabel)) ∧
      elaborate (.relationBind .relationSource) = .core (.relationBind .relationSource) ∧
      elaborate (.relationProjection .relationSource) =
        .core (.relationProjection .relationSource (consumerRelationProjectionPlan .relationSource)) ∧
      validPlan (relationPublishPlan .relationSource) = true ∧
      validPlan (consumerRelationProjectionPlan .relationSource) = true := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem consumer_store_publishvalue_and_j_mutation_reject_without_semantic_mutation :
    let storeAttempt := applyStep canonicalConfig (.consumerMaterialize .store)
    let valueAttempt := applyStep canonicalConfig (.consumerMaterialize .publishValue)
    let relationAttempt := applyStep canonicalConfig .consumerJMutation
    storeAttempt.rejection = some .consumerCannotMaterialize ∧
      storeAttempt.config = appendTrace canonicalConfig .relationSource
        (.rejected .consumerCannotMaterialize) ∧
      storeAttempt.config.hp = canonicalConfig.hp ∧
      storeAttempt.config.binding = canonicalConfig.binding ∧
      valueAttempt.rejection = some .consumerCannotMaterialize ∧
      valueAttempt.config = appendTrace canonicalConfig .relationSource
        (.rejected .consumerCannotMaterialize) ∧
      valueAttempt.config.hp = canonicalConfig.hp ∧
      valueAttempt.config.binding = canonicalConfig.binding ∧
      relationAttempt.rejection = some .nonOwnerRelationMutation ∧
      relationAttempt.config = appendTrace canonicalConfig .relationSource
        (.rejected .nonOwnerRelationMutation) ∧
      relationAttempt.config.hp = canonicalConfig.hp ∧
      relationAttempt.config.binding = canonicalConfig.binding := by
  decide

theorem exact_owner_relation_epoch_witness_validation_is_fail_closed :
    (applyStep canonicalConfig (.relationDegrade .relationOwner staleCapability canonicalWitness)).rejection =
      some .staleRelationCredential ∧
      (applyStep canonicalConfig (.relationDegrade .relationOwner staleCapability canonicalWitness)).config.binding =
        canonicalConfig.binding := by
  exact ⟨rfl, rfl⟩

theorem supported_finite_steps_preserve_wellFormed :
    supportedM5Steps.all (fun step => wellFormed (applyStep canonicalConfig step).config) = true := by
  rfl

theorem cut_backed_save_restore_preserves_wellFormed :
    (saveObject canonicalCutConfig).bind restore = some canonicalCutConfig ∧
      wellFormed canonicalCutConfig = true := by
  exact ⟨rfl, rfl⟩

theorem cut_backed_save_restore_preserves_published_relation :
    (saveObject publishedRelationCutConfig).bind restore = some publishedRelationCutConfig := by
  rfl

theorem stale_witness_restore_is_rejected :
    (saveObject canonicalCutConfig).bind
      (fun saved => restore { saved with ownerWitness := staleWitness }) = none := by
  rfl

theorem observation_uses_the_derived_restriction :
    observeProjection .relationSource
      (evaluateProjection canonicalProjection canonicalPresentationContext .restrictedLabel) =
      some { source := .relationSource, observer := .relationConsumer, label := .restrictedLabel } := by
  rfl

example :
    (applyStep canonicalConfig (SemanticStep.consumerMaterialize .store)).config.hp = canonicalConfig.hp := by
  rfl

example :
    (applyStep canonicalConfig (SemanticStep.consumerMaterialize .publishValue)).config.binding = canonicalConfig.binding := by
  rfl

example :
    (applyStep canonicalConfig SemanticStep.consumerJMutation).config.binding = canonicalConfig.binding := by
  rfl

example :
    mixedOwnerBindingPublishProjectionTrace =
      [.relationBound, .relationPublished,
        .consumerProjected (.evaluated .coordinate15 .restrictedLabel)] := by
  rfl

example : wellFormed canonicalConfig = true := by
  rfl

example :
    (applyStep canonicalConfig (.relationDegrade .relationOwner canonicalCapability canonicalWitness)).config.binding.selected =
      .fallback := by
  rfl

example :
    (applyStep canonicalConfig (.relationDegrade .relationConsumer canonicalCapability canonicalWitness)).config.binding =
      canonicalConfig.binding := by
  rfl

example :
    (applyStep canonicalConfig (.relationDegrade .relationOwner staleCapability canonicalWitness)).config.binding =
      canonicalConfig.binding := by
  rfl

example :
    (saveObject canonicalCutConfig).bind restore = some canonicalCutConfig := by
  rfl

example :
    (saveObject canonicalCutConfig).bind (fun saved => restore { saved with ownerWitness := staleWitness }) = none := by
  rfl

example :
    elaborate (.ownerRmw .ownerRmwSource) =
      .core (.ownerRmw .ownerRmwSource ownerRmwPlan) := by
  rfl

example : canonicalProjection.activationFrontier = .frontier10 := by
  rfl

example :
    evaluateProjection canonicalProjection canonicalPresentationContext .restrictedLabel =
      .evaluated .coordinate15 .restrictedLabel := by
  rfl

example :
    evaluateProjection canonicalProjection nonAdmittedPrimaryContext .restrictedLabel =
      .rejected .sampleNotAdmitted := by
  rfl

example :
    evaluateProjection canonicalProjection overflowingPresentationContext .restrictedLabel =
      .rejected .derivedCoordinateOverflow := by
  rfl

example :
    validPlan
      { kind := .relationProjection
        source := .relationSource
        site := .owner
        materialization := .publishRelation
        resultFrontier := none } = true := by
  rfl

example : mixedOwnerBindingPublishProjectionTrace.length = 3 := by
  rfl

#print axioms elaboration_deterministic
#print axioms project_then_evaluate_matches_owner_held_relation
#print axioms semantic_fallback_is_monotone
#print axioms owner_binding_publish_then_consumer_local_projection
#print axioms consumer_store_publishvalue_and_j_mutation_reject_without_semantic_mutation
#print axioms supported_finite_steps_preserve_wellFormed
#print axioms cut_backed_save_restore_preserves_wellFormed
#print axioms cut_backed_save_restore_preserves_published_relation

end MirTheoryV0M5SharedModel
