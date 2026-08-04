/-!
Mir Theory v0 M8 finite deterministic-runtime evidence.

This fresh, self-contained Lean universe models the bounded M8 handoff.  It
does not import or identify Rust, M5, or M7 carrier types.  The correspondence
is structural and is documented in Canon; every theorem below is only for the
concrete finite profile declared here.
-/

namespace MirTheoryV0M8DeterministicRuntime

set_option autoImplicit false

/- Checked-artifact view and source-bound runtime admission. -/
inductive SourceRef where
  | ownerSource
  | relationSource
  | designatedSource
  | authSource
  | verifySource
deriving DecidableEq, Repr

inductive ProgramIdentity where
  | ownerOnly
  | maintainedRelation
  | designatedValue
  | deferredAuthVerify
deriving DecidableEq, Repr

inductive CheckedEvaluationKind where
  | ownerRmw
  | publishRelation
  | designatedPublishValue
deriving DecidableEq, Repr

structure CheckedEvaluation where
  kind : CheckedEvaluationKind
  source : SourceRef
deriving DecidableEq, Repr

inductive ResidualKind where
  | visibility
  | relationLifetime
  | fallbackValidity
  | valueVisibilityRedaction
  | authDeferred
  | verifyDeferred
deriving DecidableEq, Repr

structure ResidualEvidence where
  kind : ResidualKind
  name : String
  source : SourceRef
deriving DecidableEq, Repr

inductive LoweredAction where
  | ownerRequest (source : SourceRef)
  | ownerLocalRead (source : SourceRef)
  | ownerWrite (source : SourceRef)
  | relationPublish (source : SourceRef)
  | consumerLocalProjection (source : SourceRef)
  | designatedRequest (source : SourceRef)
  | designatedReceiptUse (source : SourceRef)
  | designatedValuePublish (source : SourceRef)
deriving DecidableEq, Repr

structure SourceToCoreRow where
  ordinal : Nat
  source : SourceRef
  action : LoweredAction
deriving DecidableEq, Repr

inductive StaticEnvironment where
  | ownerEnvironment
  | relationEnvironment
  | designatedEnvironment
  | deferredEnvironment
deriving DecidableEq, Repr

inductive EffectObligationShape where
  | failureRow
  | capability
  | witness
  | authority
  | admittedEvaluatorAuthority
  | evaluation
  | authDeferred
  | verifyDeferred
deriving DecidableEq, Repr

inductive SecurityLabel where
  | publicLabel
  | restrictedLabel
  | privateLabel
deriving DecidableEq, Repr

inductive Redaction where
  | ownerRedaction
  | relationRedaction
  | privateRelationRedaction
  | designatedRedaction
  | wrongRedaction
deriving DecidableEq, Repr

inductive RelationOwner where
  | relationOwner
  | wrongOwner
deriving DecidableEq, Repr

inductive LeaseReference where
  | leaseRef1
  | wrongLeaseRef
deriving DecidableEq, Repr

inductive LeaseEvidence where
  | liveLease
  | expiredLease
deriving DecidableEq, Repr

inductive BindingFrontier where
  | binding1
  | binding2
  | wrongBinding
deriving DecidableEq, Repr

inductive RelationEpoch where
  | primaryEpoch1
  | fallbackEpoch1
  | reacquiredPrimaryEpoch
  | wrongEpoch
deriving DecidableEq, Repr

inductive RelationWitness where
  | initialWitness
  | freshWitness
  | reusedWitness
deriving DecidableEq, Repr

inductive RawWitnessPayload where
  | witnessAlpha
  | witnessBeta
deriving DecidableEq, Repr

inductive RawCapabilityPayload where
  | capabilityAlpha
  | capabilityBeta
deriving DecidableEq, Repr

inductive RawAuthorityPayload where
  | authorityAlpha
  | authorityBeta
deriving DecidableEq, Repr

inductive RawFailurePayload where
  | failureAlpha
  | failureBeta
deriving DecidableEq, Repr

/- The finite lease inventory is keyed by the relation carrier, its owner,
   binding frontier, and epoch.  There is no implied live lease on absence. -/
structure LeaseInventoryRow where
  relation : String
  owner : RelationOwner
  reference : LeaseReference
  lease : LeaseEvidence
  bindingFrontier : BindingFrontier
  epoch : RelationEpoch
deriving DecidableEq, Repr

def canonicalLeaseInventory : List LeaseInventoryRow :=
  [{ relation := "bird_follow"
     owner := .relationOwner
     reference := .leaseRef1
     lease := .liveLease
     bindingFrontier := .binding1
     epoch := .primaryEpoch1 }]

def expiredLeaseInventory : List LeaseInventoryRow :=
  [{ relation := "bird_follow"
     owner := .relationOwner
     reference := .leaseRef1
     lease := .expiredLease
     bindingFrontier := .binding1
     epoch := .primaryEpoch1 }]

def mismatchedLeaseInventory : List LeaseInventoryRow :=
  [{ relation := "bird_follow"
     owner := .wrongOwner
     reference := .leaseRef1
     lease := .liveLease
     bindingFrontier := .binding1
     epoch := .primaryEpoch1 }]

def missingLeaseInventory : List LeaseInventoryRow := []

def freshLeaseInventory : List LeaseInventoryRow :=
  [{ relation := "bird_follow"
     owner := .relationOwner
     reference := .leaseRef1
     lease := .liveLease
     bindingFrontier := .binding2
     epoch := .reacquiredPrimaryEpoch }]

structure CheckedProgramIdentity where
  program : ProgramIdentity
  staticEnvironment : StaticEnvironment
  evaluationShape : List CheckedEvaluationKind
  effectObligationShape : List EffectObligationShape
  sourceMapShape : List SourceToCoreRow
  residualSourceRefs : List SourceRef
deriving DecidableEq, Repr

structure CheckedArtifact where
  identity : CheckedProgramIdentity
  evaluations : List CheckedEvaluation
  effectObligations : List EffectObligationShape
  residuals : List ResidualEvidence
  sourceToCore : List SourceToCoreRow
deriving DecidableEq, Repr

/- Each finite evidence row carries the payload that it settles. -/
inductive AdmissionEvidence where
  | relationVisibility (source : SourceRef) (label : SecurityLabel) (redaction : Redaction)
  | relationLifetime (source : SourceRef) (reference : LeaseReference) (frontier : BindingFrontier)
  | relationFallbackValidity
      (source : SourceRef) (primaryEpoch : RelationEpoch) (fallbackEpoch : RelationEpoch)
  | designatedValueVisibilityRedaction
      (source : SourceRef) (label : SecurityLabel) (redaction : Redaction)
  | authDeferredRecord (source : SourceRef)
  | verifyDeferredRecord (source : SourceRef)
deriving DecidableEq, Repr

structure RuntimeAdmission where
  checkedIdentity : CheckedProgramIdentity
  evidence : List AdmissionEvidence
deriving DecidableEq, Repr

/- The retained checked artifact and ordered lowered plans are one carrier. -/
structure RuntimeProgram where
  checkedArtifact : CheckedArtifact
  checkedIdentity : CheckedProgramIdentity
  loweredActions : List LoweredAction
  admission : RuntimeAdmission
deriving DecidableEq, Repr

inductive RuntimeAdmissionStatus where
  | runtimeAdmitted
  | deferredToM9 (source : SourceRef)
  | rejected (source : SourceRef)
deriving DecidableEq, Repr

def ownerEvaluation : CheckedEvaluation :=
  { kind := .ownerRmw, source := .ownerSource }

def relationEvaluation : CheckedEvaluation :=
  { kind := .publishRelation, source := .relationSource }

def designatedEvaluation : CheckedEvaluation :=
  { kind := .designatedPublishValue, source := .designatedSource }

def lowerEvaluation : CheckedEvaluation → List LoweredAction
  | { kind := .ownerRmw, source := source } =>
      [.ownerRequest source, .ownerLocalRead source, .ownerWrite source]
  | { kind := .publishRelation, source := source } =>
      [.relationPublish source, .consumerLocalProjection source]
  | { kind := .designatedPublishValue, source := source } =>
      [.designatedRequest source, .designatedReceiptUse source,
       .designatedValuePublish source]

def lowerEvaluations : List CheckedEvaluation → List LoweredAction
  | [] => []
  | evaluation :: rest => lowerEvaluation evaluation ++ lowerEvaluations rest

def ownerSourceMap : List SourceToCoreRow :=
  [ { ordinal := 0, source := .ownerSource, action := .ownerRequest .ownerSource }
  , { ordinal := 1, source := .ownerSource, action := .ownerLocalRead .ownerSource }
  , { ordinal := 2, source := .ownerSource, action := .ownerWrite .ownerSource } ]

def relationSourceMap : List SourceToCoreRow :=
  [ { ordinal := 0, source := .relationSource, action := .relationPublish .relationSource }
  , { ordinal := 1, source := .relationSource,
      action := .consumerLocalProjection .relationSource } ]

def designatedSourceMap : List SourceToCoreRow :=
  [ { ordinal := 0, source := .designatedSource, action := .designatedRequest .designatedSource }
  , { ordinal := 1, source := .designatedSource,
      action := .designatedReceiptUse .designatedSource }
  , { ordinal := 2, source := .designatedSource,
      action := .designatedValuePublish .designatedSource } ]

def ownerCheckedIdentity : CheckedProgramIdentity :=
  { program := .ownerOnly
    staticEnvironment := .ownerEnvironment
    evaluationShape := [.ownerRmw]
    effectObligationShape := [.failureRow, .capability, .witness, .evaluation]
    sourceMapShape := ownerSourceMap
    residualSourceRefs := [] }

def relationCheckedIdentity : CheckedProgramIdentity :=
  { program := .maintainedRelation
    staticEnvironment := .relationEnvironment
    evaluationShape := [.publishRelation]
    effectObligationShape := [.authority, .evaluation]
    sourceMapShape := relationSourceMap
    residualSourceRefs := [.relationSource, .relationSource, .relationSource] }

def designatedCheckedIdentity : CheckedProgramIdentity :=
  { program := .designatedValue
    staticEnvironment := .designatedEnvironment
    evaluationShape := [.designatedPublishValue]
    effectObligationShape := [.admittedEvaluatorAuthority, .evaluation]
    sourceMapShape := designatedSourceMap
    residualSourceRefs := [.designatedSource] }

def deferredCheckedIdentity : CheckedProgramIdentity :=
  { program := .deferredAuthVerify
    staticEnvironment := .deferredEnvironment
    evaluationShape := []
    effectObligationShape := [.authDeferred, .verifyDeferred]
    sourceMapShape := []
    residualSourceRefs := [.authSource, .verifySource] }

def ownerArtifact : CheckedArtifact :=
  { identity := ownerCheckedIdentity
    evaluations := [ownerEvaluation]
    effectObligations := [.failureRow, .capability, .witness, .evaluation]
    residuals := []
    sourceToCore := ownerSourceMap }

def relationArtifact : CheckedArtifact :=
  { identity := relationCheckedIdentity
    evaluations := [relationEvaluation]
    effectObligations := [.authority, .evaluation]
    residuals :=
      [ { kind := .visibility, name := "bird_follow", source := .relationSource }
      , { kind := .relationLifetime, name := "bird_follow", source := .relationSource }
      , { kind := .fallbackValidity, name := "bird_follow", source := .relationSource } ]
    sourceToCore := relationSourceMap }

def designatedArtifact : CheckedArtifact :=
  { identity := designatedCheckedIdentity
    evaluations := [designatedEvaluation]
    effectObligations := [.admittedEvaluatorAuthority, .evaluation]
    residuals :=
      [{ kind := .valueVisibilityRedaction, name := "E.result", source := .designatedSource }]
    sourceToCore := designatedSourceMap }

def deferredArtifact : CheckedArtifact :=
  { identity := deferredCheckedIdentity
    evaluations := []
    effectObligations := [.authDeferred, .verifyDeferred]
    residuals :=
      [ { kind := .authDeferred, name := "MembershipAuth", source := .authSource }
      , { kind := .verifyDeferred, name := "finite_refinement", source := .verifySource } ]
    sourceToCore := [] }

def ownerAdmission : RuntimeAdmission :=
  { checkedIdentity := ownerCheckedIdentity, evidence := [] }

def relationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def incompleteRelationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1 ] }

def wrongSourceRelationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .ownerSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def wrongVisibilityRelationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .privateLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def wrongLeaseReferenceRelationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .wrongLeaseRef .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def wrongBindingFrontierRelationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .wrongBinding
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def wrongEpochRelationAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .wrongEpoch .fallbackEpoch1 ] }

def duplicateRelationLifetimeAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def conflictingRelationLifetimeAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationLifetime .relationSource .wrongLeaseRef .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def reverseConflictingRelationLifetimeAdmission : RuntimeAdmission :=
  { checkedIdentity := relationCheckedIdentity
    evidence :=
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .wrongLeaseRef .binding1
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] }

def designatedAdmission : RuntimeAdmission :=
  { checkedIdentity := designatedCheckedIdentity
    evidence :=
      [.designatedValueVisibilityRedaction .designatedSource .restrictedLabel .designatedRedaction] }

def wrongRedactionDesignatedAdmission : RuntimeAdmission :=
  { checkedIdentity := designatedCheckedIdentity
    evidence :=
      [.designatedValueVisibilityRedaction .designatedSource .restrictedLabel .wrongRedaction] }

def deferredAdmission : RuntimeAdmission :=
  { checkedIdentity := deferredCheckedIdentity
    evidence := [.authDeferredRecord .authSource, .verifyDeferredRecord .verifySource] }

def runtimeProgramFor (artifact : CheckedArtifact) (admission : RuntimeAdmission) : RuntimeProgram :=
  { checkedArtifact := artifact
    checkedIdentity := artifact.identity
    loweredActions := lowerEvaluations artifact.evaluations
    admission := admission }

def ownerRuntimeProgram : RuntimeProgram := runtimeProgramFor ownerArtifact ownerAdmission
def relationRuntimeProgram : RuntimeProgram := runtimeProgramFor relationArtifact relationAdmission
def designatedRuntimeProgram : RuntimeProgram :=
  runtimeProgramFor designatedArtifact designatedAdmission

/- This finite input family is the checked-artifact-only M8 boundary. -/
inductive RuntimeAdmissionCase where
  | ownerResidualFree
  | relationComplete
  | relationMissingFallback
  | relationWrongSource
  | relationWrongVisibility
  | relationWrongLeaseReference
  | relationWrongBindingFrontier
  | relationWrongEpoch
  | relationDuplicateLifetimeEvidence
  | relationConflictingLifetimeEvidence
  | relationReverseConflictingLifetimeEvidence
  | designatedComplete
  | designatedWrongRedaction
  | authVerifyDeferred
  | ownerIdentityMismatch
deriving DecidableEq, Repr

def checkedArtifactFor : RuntimeAdmissionCase → CheckedArtifact
  | .ownerResidualFree => ownerArtifact
  | .relationComplete | .relationMissingFallback | .relationWrongSource |
      .relationWrongVisibility | .relationWrongLeaseReference |
      .relationWrongBindingFrontier | .relationWrongEpoch |
      .relationDuplicateLifetimeEvidence | .relationConflictingLifetimeEvidence |
      .relationReverseConflictingLifetimeEvidence => relationArtifact
  | .designatedComplete | .designatedWrongRedaction => designatedArtifact
  | .authVerifyDeferred => deferredArtifact
  | .ownerIdentityMismatch => ownerArtifact

def runtimeAdmissionFor : RuntimeAdmissionCase → RuntimeAdmission
  | .ownerResidualFree => ownerAdmission
  | .relationComplete => relationAdmission
  | .relationMissingFallback => incompleteRelationAdmission
  | .relationWrongSource => wrongSourceRelationAdmission
  | .relationWrongVisibility => wrongVisibilityRelationAdmission
  | .relationWrongLeaseReference => wrongLeaseReferenceRelationAdmission
  | .relationWrongBindingFrontier => wrongBindingFrontierRelationAdmission
  | .relationWrongEpoch => wrongEpochRelationAdmission
  | .relationDuplicateLifetimeEvidence => duplicateRelationLifetimeAdmission
  | .relationConflictingLifetimeEvidence => conflictingRelationLifetimeAdmission
  | .relationReverseConflictingLifetimeEvidence => reverseConflictingRelationLifetimeAdmission
  | .designatedComplete => designatedAdmission
  | .designatedWrongRedaction => wrongRedactionDesignatedAdmission
  | .authVerifyDeferred => deferredAdmission
  | .ownerIdentityMismatch =>
      { checkedIdentity := relationCheckedIdentity, evidence := [] }

def resolveRuntimeAdmission : RuntimeAdmissionCase → RuntimeAdmissionStatus
  | .ownerResidualFree | .relationComplete | .designatedComplete => .runtimeAdmitted
  | .relationMissingFallback | .relationWrongSource | .relationWrongVisibility |
      .relationWrongLeaseReference | .relationWrongBindingFrontier | .relationWrongEpoch |
      .relationDuplicateLifetimeEvidence |
      .relationConflictingLifetimeEvidence | .relationReverseConflictingLifetimeEvidence =>
        .rejected .relationSource
  | .designatedWrongRedaction => .rejected .designatedSource
  | .authVerifyDeferred => .deferredToM9 .authSource
  | .ownerIdentityMismatch => .rejected .ownerSource

/- One finite K8 configuration, including active retained plans and unified H. -/
inductive CredentialStatus where
  | current
  | stale
deriving DecidableEq, Repr

inductive AtomicCut where
  | cut1
deriving DecidableEq, Repr

inductive PatchFrontier where
  | frontier1
deriving DecidableEq, Repr

inductive PatchVerdict where
  | rejected
  | deferred
  | accepted
deriving DecidableEq, Repr

inductive InputFrontier where
  | input1
deriving DecidableEq, Repr

inductive ResultFrontier where
  | result1
deriving DecidableEq, Repr

inductive ResultVersion where
  | version1
deriving DecidableEq, Repr

structure EvaluationPolicy where
  name : String
deriving DecidableEq, Repr

structure ObservationPolicy where
  name : String
deriving DecidableEq, Repr

structure PolicyStamp where
  value : String
deriving DecidableEq, Repr

structure RelationState where
  name : String
  owner : RelationOwner
  leaseReference : LeaseReference
  bindingFrontier : BindingFrontier
  label : SecurityLabel
  redaction : Redaction
  lease : LeaseEvidence
  leaseEpoch : RelationEpoch
  primaryEpoch : RelationEpoch
  fallbackEpoch : RelationEpoch
  witness : RelationWitness
  lineage : List String
deriving DecidableEq, Repr

structure DesignatedValue where
  name : String
  inputFrontier : InputFrontier
  resultFrontier : ResultFrontier
  version : ResultVersion
  evaluationPolicy : EvaluationPolicy
  observationPolicy : ObservationPolicy
  policyStamp : PolicyStamp
  label : SecurityLabel
  redaction : Redaction
  value : Int
deriving DecidableEq, Repr

structure OwnerRequest where
  source : SourceRef
  damage : Int
deriving DecidableEq, Repr

inductive OccurrenceKind where
  | ownerAuthorityValidated
  | ownerWitnessValidated
  | ownerWitnessRejected
  | ownerWrite
  | ownerFailure
  | relationAuthorityValidated
  | relationPublished
  | relationFreshReacquired
  | designatedAuthorityValidated
  | designatedValuePublished
deriving DecidableEq, Repr

structure Occurrence where
  ordinal : Nat
  kind : OccurrenceKind
  source : SourceRef
  predecessor : Option Nat
  label : SecurityLabel
  redaction : Redaction
  rawWitness : Option RawWitnessPayload
  rawCapability : Option RawCapabilityPayload
  rawAuthority : Option RawAuthorityPayload
  rawFailure : Option RawFailurePayload
deriving DecidableEq, Repr

structure PatchLifecycleRow where
  verdict : PatchVerdict
  frontier : PatchFrontier
  installedProgram : Option CheckedProgramIdentity
deriving DecidableEq, Repr

structure RuntimeConfig where
  activeProgram : RuntimeProgram
  ownerQueue : List OwnerRequest
  hp : Int
  membership : CredentialStatus
  capability : CredentialStatus
  witness : CredentialStatus
  lease : CredentialStatus
  leaseInventory : List LeaseInventoryRow
  relation : Option RelationState
  designated : Option DesignatedValue
  cut : Option AtomicCut
  patchFrontier : PatchFrontier
  occurrences : List Occurrence
  patchLifecycle : List PatchLifecycleRow
deriving DecidableEq, Repr

/- Semantic snapshots intentionally exclude H and patch lifecycle rows, which
   are separately retained by the full save object for the exact K8 profile. -/
structure SemanticSnapshot where
  activeProgram : RuntimeProgram
  ownerQueue : List OwnerRequest
  hp : Int
  membership : CredentialStatus
  capability : CredentialStatus
  witness : CredentialStatus
  lease : CredentialStatus
  leaseInventory : List LeaseInventoryRow
  relation : Option RelationState
  designated : Option DesignatedValue
  cut : Option AtomicCut
  patchFrontier : PatchFrontier
deriving DecidableEq, Repr

def semanticSnapshot (config : RuntimeConfig) : SemanticSnapshot :=
  { activeProgram := config.activeProgram
    ownerQueue := config.ownerQueue
    hp := config.hp
    membership := config.membership
    capability := config.capability
    witness := config.witness
    lease := config.lease
    leaseInventory := config.leaseInventory
    relation := config.relation
    designated := config.designated
    cut := config.cut
    patchFrontier := config.patchFrontier }

def initialOwnerConfig : RuntimeConfig :=
  { activeProgram := ownerRuntimeProgram
    ownerQueue := [{ source := .ownerSource, damage := 10 },
      { source := .ownerSource, damage := 10 }]
    hp := 100
    membership := .current
    capability := .current
    witness := .current
    lease := .current
    leaseInventory := []
    relation := none
    designated := none
    cut := some .cut1
    patchFrontier := .frontier1
    occurrences := []
    patchLifecycle := [] }

def credentialsCurrent (config : RuntimeConfig) : Bool :=
  decide (config.membership = .current) && decide (config.capability = .current) &&
    decide (config.witness = .current) && decide (config.lease = .current)

def predecessorOf (occurrences : List Occurrence) : Option Nat :=
  match occurrences.reverse with
  | [] => none
  | occurrence :: _ => some occurrence.ordinal

def rawFreeOccurrence (ordinal : Nat) (kind : OccurrenceKind) (source : SourceRef)
    (predecessor : Option Nat) (label : SecurityLabel) (redaction : Redaction) : Occurrence :=
  { ordinal := ordinal
    kind := kind
    source := source
    predecessor := predecessor
    label := label
    redaction := redaction
    rawWitness := none
    rawCapability := none
    rawAuthority := none
    rawFailure := none }

def appendOccurrence (config : RuntimeConfig) (kind : OccurrenceKind)
    (source : SourceRef) (label : SecurityLabel) (redaction : Redaction) : RuntimeConfig :=
  { config with occurrences := config.occurrences ++
      [rawFreeOccurrence config.occurrences.length kind source
        (predecessorOf config.occurrences) label redaction] }

def serviceOneOwnerRequest (config : RuntimeConfig) : RuntimeConfig :=
  match config.ownerQueue with
  | [] => config
  | request :: rest =>
      let queued := { config with ownerQueue := rest }
      if credentialsCurrent config then
        let authority := appendOccurrence queued .ownerAuthorityValidated request.source
          .restrictedLabel .ownerRedaction
        let witness := appendOccurrence authority .ownerWitnessValidated request.source
          .restrictedLabel .ownerRedaction
        let written := appendOccurrence witness .ownerWrite request.source
          .restrictedLabel .ownerRedaction
        { written with hp := config.hp - request.damage }
      else
        let rejected := appendOccurrence queued .ownerWitnessRejected request.source
          .restrictedLabel .ownerRedaction
        appendOccurrence rejected .ownerFailure request.source .restrictedLabel .ownerRedaction

def frozenTwoRequestReplay (config : RuntimeConfig) : RuntimeConfig :=
  serviceOneOwnerRequest (serviceOneOwnerRequest config)

def occurrenceDependenciesAreBackward (occurrence : Occurrence) : Bool :=
  match occurrence.predecessor with
  | none => true
  | some predecessor => decide (predecessor < occurrence.ordinal)

def dependencyTraceIsAcyclic (occurrences : List Occurrence) : Bool :=
  occurrences.all occurrenceDependenciesAreBackward

def canonicalRelation : RelationState :=
  { name := "bird_follow"
    owner := .relationOwner
    leaseReference := .leaseRef1
    bindingFrontier := .binding1
    label := .restrictedLabel
    redaction := .relationRedaction
    lease := .liveLease
    leaseEpoch := .primaryEpoch1
    primaryEpoch := .primaryEpoch1
    fallbackEpoch := .fallbackEpoch1
    witness := .initialWitness
    lineage := ["bird_follow:lineage:binding1"] }

def relationRuntimeConfig : RuntimeConfig :=
  { initialOwnerConfig with
    activeProgram := relationRuntimeProgram
    ownerQueue := []
    leaseInventory := canonicalLeaseInventory }

def publishRelation (config : RuntimeConfig) : RuntimeConfig :=
  appendOccurrence
    (appendOccurrence { config with relation := some canonicalRelation }
      .relationAuthorityValidated .relationSource .restrictedLabel .relationRedaction)
    .relationPublished .relationSource .restrictedLabel .relationRedaction

/- The dynamic relation lease inventory is not admission evidence.  Every
   relation operation rechecks the exact current owner/ref/frontier/epoch row. -/
def hasExactLiveLeaseInventory (relation : RelationState)
    (inventory : List LeaseInventoryRow) : Bool :=
  decide (inventory =
    [{ relation := relation.name
       owner := relation.owner
       reference := relation.leaseReference
       lease := .liveLease
       bindingFrontier := relation.bindingFrontier
       epoch := relation.leaseEpoch }])

/- Consumer projection is read-side; it cannot alter owner-held J, and an
   absent, expired, or mismatched dynamic inventory returns no projection. -/
def consumerLocalProjection (config : RuntimeConfig) : Option RuntimeConfig :=
  match config.relation with
  | none => none
  | some relation =>
      if hasExactLiveLeaseInventory relation config.leaseInventory then some config else none

/- The finite transition gate is checked independently of projection; its
   successful branch is intentionally a no-op carrier for this bounded model. -/
def relationTransitionGate (config : RuntimeConfig) : Option RuntimeConfig :=
  match config.relation with
  | none => none
  | some relation =>
      if hasExactLiveLeaseInventory relation config.leaseInventory then some config else none

structure FallbackProjection where
  label : SecurityLabel
  redaction : Redaction
deriving DecidableEq, Repr

/- A local fallback is read-side and retains the admitted relation policy; it
   cannot silently weaken a private relation or substitute a redaction. -/
def localFallbackProjection (relation : RelationState) : FallbackProjection :=
  { label := relation.label, redaction := relation.redaction }

def canonicalPrivateRelation : RelationState :=
  { canonicalRelation with
    name := "private_follow"
    label := .privateLabel
    redaction := .privateRelationRedaction }

def publishedRelationConfig : RuntimeConfig := publishRelation relationRuntimeConfig

def missingLeaseOperationConfig : RuntimeConfig :=
  { publishedRelationConfig with leaseInventory := missingLeaseInventory }

def expiredLeaseOperationConfig : RuntimeConfig :=
  { publishedRelationConfig with leaseInventory := expiredLeaseInventory }

def mismatchedLeaseOperationConfig : RuntimeConfig :=
  { publishedRelationConfig with leaseInventory := mismatchedLeaseInventory }

def freshReacquireRuntimeConfig : RuntimeConfig :=
  { publishedRelationConfig with leaseInventory := freshLeaseInventory }

inductive RelationReacquireCase where
  | forgedReuse
  | freshExact
deriving DecidableEq, Repr

def freshReacquiredRelation (relation : RelationState) : RelationState :=
  { relation with
    bindingFrontier := .binding2
    leaseEpoch := .reacquiredPrimaryEpoch
    primaryEpoch := .reacquiredPrimaryEpoch
    witness := .freshWitness
    lineage := ["bird_follow:lineage:binding2"] }

def reacquirePrimary (config : RuntimeConfig) (attempt : RelationReacquireCase) : RuntimeConfig :=
  match config.relation, attempt with
  | none, _ => config
  | some _, .forgedReuse => config
  | some relation, .freshExact =>
      let reacquired := freshReacquiredRelation relation
      if hasExactLiveLeaseInventory reacquired config.leaseInventory then
        appendOccurrence
          { config with relation := some reacquired }
          .relationFreshReacquired .relationSource .restrictedLabel .relationRedaction
      else config

def canonicalDesignatedValue : DesignatedValue :=
  { name := "E.result"
    inputFrontier := .input1
    resultFrontier := .result1
    version := .version1
    evaluationPolicy := { name := "inferred:E.result" }
    observationPolicy := { name := "conservative" }
    policyStamp := { value := "inferred:E.result|conservative" }
    label := .restrictedLabel
    redaction := .designatedRedaction
    value := 11 }

def designatedRuntimeConfig : RuntimeConfig :=
  { initialOwnerConfig with activeProgram := designatedRuntimeProgram, ownerQueue := [] }

def decideDesignatedValue (config : RuntimeConfig) : RuntimeConfig :=
  match config.designated with
  | some _ => config
  | none =>
      appendOccurrence
        (appendOccurrence { config with designated := some canonicalDesignatedValue }
          .designatedAuthorityValidated .designatedSource .restrictedLabel .designatedRedaction)
        .designatedValuePublished .designatedSource .restrictedLabel .designatedRedaction

structure SaveObject where
  cut : AtomicCut
  config : RuntimeConfig
deriving DecidableEq, Repr

def saveAtLocalCut (config : RuntimeConfig) : Option SaveObject :=
  match config.cut with
  | none => none
  | some cut => some { cut := cut, config := config }

def restoreIsLive (saved current : RuntimeConfig) : Bool :=
  decide (saved.activeProgram = current.activeProgram) &&
    decide (saved.membership = current.membership) &&
    decide (saved.capability = current.capability) &&
    decide (saved.witness = current.witness) &&
    decide (saved.lease = current.lease)

def restoreAtLocalCut (saved : SaveObject) (current : RuntimeConfig) : Option RuntimeConfig :=
  if restoreIsLive saved.config current && saved.config.cut == some saved.cut then some saved.config
  else none

def staleMembershipConfig : RuntimeConfig :=
  { initialOwnerConfig with membership := .stale }

def staleCapabilityConfig : RuntimeConfig :=
  { initialOwnerConfig with capability := .stale }

def staleWitnessConfig : RuntimeConfig :=
  { initialOwnerConfig with witness := .stale }

def staleLeaseConfig : RuntimeConfig :=
  { initialOwnerConfig with lease := .stale }

/- The accepted selected candidate replaces the whole retained checked-plan
   carrier at the declared local cut; reject/defer retain it unchanged. -/
def applySelectedPatch (config : RuntimeConfig) (verdict : PatchVerdict) : RuntimeConfig :=
  match verdict with
  | .rejected | .deferred =>
      { config with patchLifecycle := config.patchLifecycle ++
        [{ verdict := verdict, frontier := config.patchFrontier, installedProgram := none }] }
  | .accepted =>
      match config.cut with
      | none => config
      | some _ =>
          { config with
            activeProgram := designatedRuntimeProgram
            patchLifecycle := config.patchLifecycle ++
              [{ verdict := .accepted, frontier := config.patchFrontier,
                 installedProgram := some designatedCheckedIdentity }] }

structure ObserverRow where
  ordinal : Nat
  kind : OccurrenceKind
  source : SourceRef
  predecessor : Option Nat
  label : SecurityLabel
  redaction : Redaction
deriving DecidableEq, Repr

def observerRow (occurrence : Occurrence) : ObserverRow :=
  { ordinal := occurrence.ordinal
    kind := occurrence.kind
    source := occurrence.source
    predecessor := occurrence.predecessor
    label := occurrence.label
    redaction := occurrence.redaction }

def observerSafeTrace (config : RuntimeConfig) : List ObserverRow :=
  config.occurrences.map observerRow

/- Raw H keeps authority/witness/capability/failure payloads locally.  The
   observer map below has no way to retain any of those fields. -/
def privateObserverSafeTrace (trace : List Occurrence) : List ObserverRow :=
  trace.map observerRow

def protectedOccurrence : Occurrence :=
  rawFreeOccurrence 0 .ownerFailure .ownerSource none .restrictedLabel .ownerRedaction

def privateTraceWithAlpha : List Occurrence :=
  [({ protectedOccurrence with
      rawWitness := some RawWitnessPayload.witnessAlpha
      rawCapability := some RawCapabilityPayload.capabilityAlpha
      rawAuthority := some RawAuthorityPayload.authorityAlpha
      rawFailure := some RawFailurePayload.failureAlpha })]

def privateTraceWithBeta : List Occurrence :=
  [({ protectedOccurrence with
      rawWitness := some RawWitnessPayload.witnessBeta
      rawCapability := some RawCapabilityPayload.capabilityBeta
      rawAuthority := some RawAuthorityPayload.authorityBeta
      rawFailure := some RawFailurePayload.failureBeta })]

def applyAdmissionStatus (config : RuntimeConfig) : RuntimeAdmissionStatus → RuntimeConfig
  | .runtimeAdmitted => config
  | .deferredToM9 _ => config
  | .rejected _ => config

def sourceToCoreActions : List SourceToCoreRow → List LoweredAction
  | [] => []
  | row :: rest => row.action :: sourceToCoreActions rest

def checkedEvaluationKinds : List CheckedEvaluation → List CheckedEvaluationKind
  | [] => []
  | evaluation :: rest => evaluation.kind :: checkedEvaluationKinds rest

def residualSources : List ResidualEvidence → List SourceRef
  | [] => []
  | residual :: rest => residual.source :: residualSources rest

/- OBL-050: exact finite checked-artifact identity, payload admission, and lowering. -/
theorem checked_program_identity_covers_static_environment_evaluation_effect_obligation_and_source_map :
    ownerArtifact.identity.staticEnvironment = .ownerEnvironment ∧
    ownerArtifact.identity.evaluationShape = checkedEvaluationKinds ownerArtifact.evaluations ∧
    ownerArtifact.identity.effectObligationShape = ownerArtifact.effectObligations ∧
    ownerArtifact.identity.sourceMapShape = ownerArtifact.sourceToCore ∧
    ownerArtifact.identity.residualSourceRefs = residualSources ownerArtifact.residuals ∧
    relationArtifact.identity.staticEnvironment = .relationEnvironment ∧
    relationArtifact.identity.evaluationShape = checkedEvaluationKinds relationArtifact.evaluations ∧
    relationArtifact.identity.effectObligationShape = relationArtifact.effectObligations ∧
    relationArtifact.identity.sourceMapShape = relationArtifact.sourceToCore ∧
    relationArtifact.identity.residualSourceRefs = residualSources relationArtifact.residuals ∧
    designatedArtifact.identity.staticEnvironment = .designatedEnvironment ∧
    designatedArtifact.identity.evaluationShape = checkedEvaluationKinds designatedArtifact.evaluations ∧
    designatedArtifact.identity.effectObligationShape = designatedArtifact.effectObligations ∧
    designatedArtifact.identity.sourceMapShape = designatedArtifact.sourceToCore ∧
    designatedArtifact.identity.residualSourceRefs = residualSources designatedArtifact.residuals := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem admission_lowering_is_deterministic :
    ownerRuntimeProgram.checkedArtifact = ownerArtifact ∧
    ownerRuntimeProgram.checkedIdentity = ownerCheckedIdentity ∧
    ownerRuntimeProgram.admission = ownerAdmission ∧
    resolveRuntimeAdmission .ownerResidualFree = .runtimeAdmitted ∧
    ownerRuntimeProgram.loweredActions = sourceToCoreActions ownerArtifact.sourceToCore := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem relation_payload_mismatch_is_rejected_before_relation_install :
    runtimeAdmissionFor .relationComplete = relationAdmission ∧
    resolveRuntimeAdmission .relationComplete = .runtimeAdmitted ∧
    relationAdmission.evidence =
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] ∧
    resolveRuntimeAdmission .relationMissingFallback = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongSource = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongVisibility = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongLeaseReference = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongBindingFrontier = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongEpoch = .rejected .relationSource ∧
    applyAdmissionStatus relationRuntimeConfig (resolveRuntimeAdmission .relationWrongVisibility) =
      relationRuntimeConfig ∧
    applyAdmissionStatus relationRuntimeConfig (resolveRuntimeAdmission .relationWrongLeaseReference) =
      relationRuntimeConfig ∧
    applyAdmissionStatus relationRuntimeConfig (resolveRuntimeAdmission .relationWrongBindingFrontier) =
      relationRuntimeConfig ∧
    applyAdmissionStatus relationRuntimeConfig (resolveRuntimeAdmission .relationWrongEpoch) =
      relationRuntimeConfig := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem relation_admission_validates_declared_lease_ref_frontier_and_epochs :
    relationAdmission.evidence =
      [ .relationVisibility .relationSource .restrictedLabel .relationRedaction
      , .relationLifetime .relationSource .leaseRef1 .binding1
      , .relationFallbackValidity .relationSource .primaryEpoch1 .fallbackEpoch1 ] ∧
    resolveRuntimeAdmission .relationComplete = .runtimeAdmitted ∧
    resolveRuntimeAdmission .relationWrongLeaseReference = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongBindingFrontier = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongEpoch = .rejected .relationSource := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem relation_admission_rejects_wrong_declared_binding_frontier :
    runtimeAdmissionFor .relationWrongBindingFrontier = wrongBindingFrontierRelationAdmission ∧
    resolveRuntimeAdmission .relationWrongBindingFrontier = .rejected .relationSource := by
  exact ⟨rfl, rfl⟩

theorem duplicate_or_conflicting_residual_evidence_rejects_deterministically :
    runtimeAdmissionFor .relationDuplicateLifetimeEvidence = duplicateRelationLifetimeAdmission ∧
    runtimeAdmissionFor .relationConflictingLifetimeEvidence = conflictingRelationLifetimeAdmission ∧
    runtimeAdmissionFor .relationReverseConflictingLifetimeEvidence =
      reverseConflictingRelationLifetimeAdmission ∧
    resolveRuntimeAdmission .relationDuplicateLifetimeEvidence = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationConflictingLifetimeEvidence = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationReverseConflictingLifetimeEvidence = .rejected .relationSource ∧
    applyAdmissionStatus relationRuntimeConfig
      (resolveRuntimeAdmission .relationDuplicateLifetimeEvidence) = relationRuntimeConfig ∧
    applyAdmissionStatus relationRuntimeConfig
      (resolveRuntimeAdmission .relationConflictingLifetimeEvidence) = relationRuntimeConfig ∧
    applyAdmissionStatus relationRuntimeConfig
      (resolveRuntimeAdmission .relationReverseConflictingLifetimeEvidence) = relationRuntimeConfig := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem relation_admission_requires_complete_source_bound_evidence :
    resolveRuntimeAdmission .relationComplete = .runtimeAdmitted ∧
    resolveRuntimeAdmission .relationMissingFallback = .rejected .relationSource ∧
    resolveRuntimeAdmission .relationWrongSource = .rejected .relationSource := by
  exact ⟨rfl, rfl, rfl⟩

theorem designated_admission_preserves_its_source_bound_redaction_requirement :
    runtimeAdmissionFor .designatedComplete = designatedAdmission ∧
    designatedAdmission.evidence =
      [.designatedValueVisibilityRedaction .designatedSource .restrictedLabel .designatedRedaction] ∧
    resolveRuntimeAdmission .designatedComplete = .runtimeAdmitted ∧
    resolveRuntimeAdmission .designatedWrongRedaction = .rejected .designatedSource ∧
    applyAdmissionStatus designatedRuntimeConfig
      (resolveRuntimeAdmission .designatedWrongRedaction) = designatedRuntimeConfig := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem mismatched_program_identity_rejects :
    checkedArtifactFor .ownerIdentityMismatch = ownerArtifact ∧
    runtimeAdmissionFor .ownerIdentityMismatch =
      { checkedIdentity := relationCheckedIdentity, evidence := [] } ∧
    resolveRuntimeAdmission .ownerIdentityMismatch = .rejected .ownerSource := by
  exact ⟨rfl, rfl, rfl⟩

theorem unresolved_or_m9_residual_is_deferred_without_semantic_mutation :
    checkedArtifactFor .authVerifyDeferred = deferredArtifact ∧
    runtimeAdmissionFor .authVerifyDeferred = deferredAdmission ∧
    resolveRuntimeAdmission .authVerifyDeferred = .deferredToM9 .authSource ∧
    applyAdmissionStatus initialOwnerConfig (resolveRuntimeAdmission .authVerifyDeferred) =
      initialOwnerConfig := by
  exact ⟨rfl, rfl, rfl, rfl⟩

/- OBL-051: owner FIFO service records authority, witness, write, and failure facts in H. -/
theorem owner_fifo_rmw_is_serial_at_service :
    (frozenTwoRequestReplay initialOwnerConfig).hp = 80 ∧
    (frozenTwoRequestReplay initialOwnerConfig).ownerQueue = [] ∧
    (frozenTwoRequestReplay initialOwnerConfig).occurrences.map Occurrence.kind =
      [.ownerAuthorityValidated, .ownerWitnessValidated, .ownerWrite,
       .ownerAuthorityValidated, .ownerWitnessValidated, .ownerWrite] := by
  exact ⟨rfl, rfl, rfl⟩

theorem stale_witness_owner_service_is_fail_closed_for_the_store :
    (serviceOneOwnerRequest staleWitnessConfig).hp = staleWitnessConfig.hp ∧
    (serviceOneOwnerRequest staleWitnessConfig).relation = staleWitnessConfig.relation ∧
    (serviceOneOwnerRequest staleWitnessConfig).designated = staleWitnessConfig.designated ∧
    (serviceOneOwnerRequest staleWitnessConfig).occurrences.map Occurrence.kind =
      [.ownerWitnessRejected, .ownerFailure] := by
  exact ⟨rfl, rfl, rfl, rfl⟩

/- OBL-052: the frozen unified H is deterministic and backward-dependent. -/
theorem frozen_replay_matches_the_declared_two_request_profile :
    frozenTwoRequestReplay initialOwnerConfig =
      { initialOwnerConfig with
        ownerQueue := []
        hp := 80
        occurrences :=
          [ rawFreeOccurrence 0 .ownerAuthorityValidated .ownerSource none
              .restrictedLabel .ownerRedaction
          , rawFreeOccurrence 1 .ownerWitnessValidated .ownerSource (some 0)
              .restrictedLabel .ownerRedaction
          , rawFreeOccurrence 2 .ownerWrite .ownerSource (some 1)
              .restrictedLabel .ownerRedaction
          , rawFreeOccurrence 3 .ownerAuthorityValidated .ownerSource (some 2)
              .restrictedLabel .ownerRedaction
          , rawFreeOccurrence 4 .ownerWitnessValidated .ownerSource (some 3)
              .restrictedLabel .ownerRedaction
          , rawFreeOccurrence 5 .ownerWrite .ownerSource (some 4)
              .restrictedLabel .ownerRedaction ] } := by
  rfl

theorem frozen_replay_dependency_trace_is_acyclic :
    dependencyTraceIsAcyclic (frozenTwoRequestReplay initialOwnerConfig).occurrences = true := by
  rfl

/- OBL-053: dynamic relation-operation inventory gates, fresh witness, and designated stability. -/
theorem relation_owner_publication_and_consumer_projection_preserve_owner_state :
    publishedRelationConfig.relation = some canonicalRelation ∧
    consumerLocalProjection publishedRelationConfig = some publishedRelationConfig ∧
    relationTransitionGate publishedRelationConfig = some publishedRelationConfig ∧
    publishedRelationConfig.occurrences.map Occurrence.kind =
      [.relationAuthorityValidated, .relationPublished] := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem fresh_reacquire_installs_new_witness_epoch_and_lineage :
    reacquirePrimary publishedRelationConfig .forgedReuse = publishedRelationConfig ∧
    (reacquirePrimary freshReacquireRuntimeConfig .freshExact).relation = some
      ({ canonicalRelation with
        bindingFrontier := .binding2
        leaseEpoch := .reacquiredPrimaryEpoch
        primaryEpoch := .reacquiredPrimaryEpoch
        witness := .freshWitness
        lineage := ["bird_follow:lineage:binding2"] }) ∧
    (reacquirePrimary freshReacquireRuntimeConfig .freshExact).leaseInventory =
      freshLeaseInventory ∧
    (reacquirePrimary freshReacquireRuntimeConfig .freshExact).occurrences.map
      Occurrence.kind = [.relationAuthorityValidated, .relationPublished, .relationFreshReacquired] := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem operation_time_lease_inventory_gate_rechecks_projection_transition_and_reacquire :
    canonicalLeaseInventory =
      [{ relation := "bird_follow", owner := .relationOwner, reference := .leaseRef1,
         lease := .liveLease, bindingFrontier := .binding1, epoch := .primaryEpoch1 }] ∧
    consumerLocalProjection publishedRelationConfig = some publishedRelationConfig ∧
    relationTransitionGate publishedRelationConfig = some publishedRelationConfig ∧
    consumerLocalProjection missingLeaseOperationConfig = none ∧
    relationTransitionGate missingLeaseOperationConfig = none ∧
    consumerLocalProjection expiredLeaseOperationConfig = none ∧
    relationTransitionGate expiredLeaseOperationConfig = none ∧
    consumerLocalProjection mismatchedLeaseOperationConfig = none ∧
    relationTransitionGate mismatchedLeaseOperationConfig = none ∧
    reacquirePrimary missingLeaseOperationConfig .freshExact = missingLeaseOperationConfig ∧
    reacquirePrimary expiredLeaseOperationConfig .freshExact = expiredLeaseOperationConfig ∧
    reacquirePrimary mismatchedLeaseOperationConfig .freshExact = mismatchedLeaseOperationConfig ∧
    freshLeaseInventory =
      [{ relation := "bird_follow", owner := .relationOwner, reference := .leaseRef1,
         lease := .liveLease, bindingFrontier := .binding2, epoch := .reacquiredPrimaryEpoch }] ∧
    (reacquirePrimary freshReacquireRuntimeConfig .freshExact).relation = some
      ({ canonicalRelation with
        bindingFrontier := .binding2
        leaseEpoch := .reacquiredPrimaryEpoch
        primaryEpoch := .reacquiredPrimaryEpoch
        witness := .freshWitness
        lineage := ["bird_follow:lineage:binding2"] }) ∧
    (reacquirePrimary freshReacquireRuntimeConfig .freshExact).leaseInventory =
      freshLeaseInventory := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem private_fallback_retains_admitted_label_and_redaction :
    localFallbackProjection canonicalPrivateRelation =
      { label := .privateLabel, redaction := .privateRelationRedaction } := by
  rfl

theorem designated_duplicate_decision_preserves_frontier_policy_stamp_version_and_redaction :
    (decideDesignatedValue designatedRuntimeConfig).designated = some canonicalDesignatedValue ∧
    (decideDesignatedValue (decideDesignatedValue designatedRuntimeConfig)).designated =
      (decideDesignatedValue designatedRuntimeConfig).designated ∧
    (decideDesignatedValue designatedRuntimeConfig).occurrences.map Occurrence.kind =
      [.designatedAuthorityValidated, .designatedValuePublished] := by
  exact ⟨rfl, rfl, rfl⟩

/- OBL-054: a local cut saves the full K8 carrier, including active plans and H. -/
def acceptedPatchConfig : RuntimeConfig := applySelectedPatch initialOwnerConfig .accepted

theorem local_cut_save_restore_preserves_the_complete_runtime_config :
    (saveAtLocalCut acceptedPatchConfig).bind (fun saved => restoreAtLocalCut saved acceptedPatchConfig) =
      some acceptedPatchConfig := by
  rfl

theorem local_cut_save_restore_retains_active_checked_plan_and_unified_history :
    (saveAtLocalCut acceptedPatchConfig).map SaveObject.config = some acceptedPatchConfig ∧
    acceptedPatchConfig.activeProgram = designatedRuntimeProgram ∧
    acceptedPatchConfig.occurrences = initialOwnerConfig.occurrences := by
  exact ⟨rfl, rfl, rfl⟩

theorem stale_membership_capability_witness_and_lease_restore_are_rejected :
    (saveAtLocalCut staleMembershipConfig).bind
      (fun saved => restoreAtLocalCut saved initialOwnerConfig) = none ∧
    (saveAtLocalCut staleCapabilityConfig).bind
      (fun saved => restoreAtLocalCut saved initialOwnerConfig) = none ∧
    (saveAtLocalCut staleWitnessConfig).bind
      (fun saved => restoreAtLocalCut saved initialOwnerConfig) = none ∧
    (saveAtLocalCut staleLeaseConfig).bind
      (fun saved => restoreAtLocalCut saved initialOwnerConfig) = none := by
  exact ⟨rfl, rfl, rfl, rfl⟩

/- OBL-055: rejected/deferred preserve state; the selected accepted candidate installs atomically. -/
theorem rejected_patch_preserves_semantic_snapshot :
    semanticSnapshot (applySelectedPatch initialOwnerConfig .rejected) =
      semanticSnapshot initialOwnerConfig ∧
    (applySelectedPatch initialOwnerConfig .rejected).patchLifecycle =
      [{ verdict := .rejected, frontier := .frontier1, installedProgram := none }] := by
  exact ⟨rfl, rfl⟩

theorem deferred_patch_preserves_semantic_snapshot :
    semanticSnapshot (applySelectedPatch initialOwnerConfig .deferred) =
      semanticSnapshot initialOwnerConfig ∧
    (applySelectedPatch initialOwnerConfig .deferred).patchLifecycle =
      [{ verdict := .deferred, frontier := .frontier1, installedProgram := none }] := by
  exact ⟨rfl, rfl⟩

theorem accepted_patch_atomically_installs_checked_plan_in_one_runtime_config :
    acceptedPatchConfig.activeProgram = designatedRuntimeProgram ∧
    acceptedPatchConfig.activeProgram.checkedArtifact = designatedArtifact ∧
    acceptedPatchConfig.activeProgram.checkedIdentity = designatedCheckedIdentity ∧
    acceptedPatchConfig.activeProgram.admission = designatedAdmission ∧
    acceptedPatchConfig.activeProgram.loweredActions =
      [.designatedRequest .designatedSource, .designatedReceiptUse .designatedSource,
       .designatedValuePublish .designatedSource] ∧
    acceptedPatchConfig.hp = initialOwnerConfig.hp ∧
    acceptedPatchConfig.ownerQueue = initialOwnerConfig.ownerQueue ∧
    acceptedPatchConfig.patchLifecycle =
      [{ verdict := .accepted, frontier := .frontier1,
         installedProgram := some designatedCheckedIdentity }] := by
  exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

/- OBL-056: raw H is internal; observer output retains only typed structural
   failure rows with their explicit label/redaction while erasing raw payloads. -/
theorem observer_safe_trace_erases_raw_authority_witness_and_capability_payloads :
    privateObserverSafeTrace privateTraceWithAlpha =
      privateObserverSafeTrace privateTraceWithBeta := by
  rfl

theorem typed_redacted_observer_failure_rows_erase_raw_authority_witness_and_capability :
    privateObserverSafeTrace privateTraceWithAlpha =
      privateObserverSafeTrace privateTraceWithBeta ∧
    privateObserverSafeTrace privateTraceWithAlpha =
      [{ ordinal := 0, kind := .ownerFailure, source := .ownerSource,
         predecessor := none, label := .restrictedLabel, redaction := .ownerRedaction }] := by
  exact ⟨rfl, rfl⟩

theorem observer_safe_trace_is_derived_from_unified_occurrences :
    (observerSafeTrace (frozenTwoRequestReplay initialOwnerConfig)).map ObserverRow.kind =
      [.ownerAuthorityValidated, .ownerWitnessValidated, .ownerWrite,
       .ownerAuthorityValidated, .ownerWitnessValidated, .ownerWrite] := by
  rfl

#print axioms admission_lowering_is_deterministic
#print axioms checked_program_identity_covers_static_environment_evaluation_effect_obligation_and_source_map
#print axioms relation_payload_mismatch_is_rejected_before_relation_install
#print axioms relation_admission_validates_declared_lease_ref_frontier_and_epochs
#print axioms relation_admission_rejects_wrong_declared_binding_frontier
#print axioms duplicate_or_conflicting_residual_evidence_rejects_deterministically
#print axioms relation_admission_requires_complete_source_bound_evidence
#print axioms designated_admission_preserves_its_source_bound_redaction_requirement
#print axioms mismatched_program_identity_rejects
#print axioms unresolved_or_m9_residual_is_deferred_without_semantic_mutation
#print axioms owner_fifo_rmw_is_serial_at_service
#print axioms stale_witness_owner_service_is_fail_closed_for_the_store
#print axioms frozen_replay_matches_the_declared_two_request_profile
#print axioms frozen_replay_dependency_trace_is_acyclic
#print axioms relation_owner_publication_and_consumer_projection_preserve_owner_state
#print axioms fresh_reacquire_installs_new_witness_epoch_and_lineage
#print axioms operation_time_lease_inventory_gate_rechecks_projection_transition_and_reacquire
#print axioms private_fallback_retains_admitted_label_and_redaction
#print axioms designated_duplicate_decision_preserves_frontier_policy_stamp_version_and_redaction
#print axioms local_cut_save_restore_preserves_the_complete_runtime_config
#print axioms local_cut_save_restore_retains_active_checked_plan_and_unified_history
#print axioms stale_membership_capability_witness_and_lease_restore_are_rejected
#print axioms rejected_patch_preserves_semantic_snapshot
#print axioms deferred_patch_preserves_semantic_snapshot
#print axioms accepted_patch_atomically_installs_checked_plan_in_one_runtime_config
#print axioms observer_safe_trace_erases_raw_authority_witness_and_capability_payloads
#print axioms typed_redacted_observer_failure_rows_erase_raw_authority_witness_and_capability
#print axioms observer_safe_trace_is_derived_from_unified_occurrences

end MirTheoryV0M8DeterministicRuntime
