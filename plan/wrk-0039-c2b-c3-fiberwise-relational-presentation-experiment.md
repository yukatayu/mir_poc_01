# WRK-0039 - C2-B/C3 fiberwise relational presentation evidence

This is **LAB** evidence for `working/WRK-0039` after its registration cut.
It retains the exact WRK-0037 finite lookup table as a marked byte-identical
baseline, then compares it only with independently enumerated relation fibers.
The comparison is defined at each supplied `(Frontier, Request)` key; it does
not claim a global inverse for a bare `DirectView`. WRK-0038 remains unexecuted
and is not evidence for this artifact.

The fenced source is a disposable, artifact-local Lean check. Its labels,
constructors, functions, and theorem names do not denote Mir request or
occurrence identity, Core state, Config, history, SaveObject, authority,
source syntax, runtime behavior, or a public interface.

## Outcome Lean source

```lean
-- BEGIN PINNED WRK-0037 BASELINE
namespace BPrimaryOpaqueLab

inductive Request where
  | q0
  | q1

inductive Frontier where
  | awaiting
  | replied
  | received
  | consumed
  | failed

inductive Phase where
  | awaiting
  | replied
  | received
  | consumed
  | failed

inductive CheckedFact where
  | m1Checked

inductive ValidationResult where
  | success
  | ownerFailure

structure ValidationRecord where
  checked : CheckedFact
  result : ValidationResult

inductive ReplyRole where
  | reply0

inductive ReceiptRole where
  | matching
  | wrongLocus

inductive FailureRole where
  | ownerFailure

inductive FailureReason where
  | validationFailed

inductive HeldDisposition where
  | holdingGammaDelta
  | deltaConsumed
  | deltaReleased

inductive ConsumedValue where
  | value0

inductive Provenance where
  | reply0

inductive Redaction where
  | visible

inductive Dependency where
  | child0

structure ReplyRecord where
  role : ReplyRole
  value : ConsumedValue
  provenance : Provenance
  redaction : Redaction

structure ReceiptRecord where
  reply : ReplyRole
  receipt : ReceiptRole

structure FailureRecord where
  role : FailureRole
  reason : FailureReason

structure Ground where
  value : ConsumedValue
  provenance : Provenance

structure ResumeRecord where
  receipt : ReceiptRecord
  ground : Ground

structure GroundedDependency where
  child : Dependency
  ground : Ground

structure Incidental where
  payload : Nat
  m1Claims : Nat
  principal : Nat
  sourceLocus : Nat
  ownerLocus : Nat
  sourceSpan : Nat
  failureRow : Nat
  transportSession : Nat
  queueContext : Nat
  predecessorShape : Nat

def incidental : Request -> Incidental := fun _ =>
  { payload := 7
    m1Claims := 11
    principal := 13
    sourceLocus := 17
    ownerLocus := 19
    sourceSpan := 23
    failureRow := 29
    transportSession := 31
    queueContext := 37
    predecessorShape := 41 }

def phaseAt : Frontier -> Request -> Phase
  | .awaiting, .q0 => .awaiting
  | .awaiting, .q1 => .awaiting
  | .replied, .q0 => .replied
  | .replied, .q1 => .failed
  | .received, .q0 => .received
  | .received, .q1 => .failed
  | .consumed, .q0 => .consumed
  | .consumed, .q1 => .failed
  | .failed, .q0 => .consumed
  | .failed, .q1 => .failed

def validationAt (frontier : Frontier) (request : Request) : Option ValidationRecord :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => some { checked := .m1Checked, result := .success }
  | .received => some { checked := .m1Checked, result := .success }
  | .consumed => some { checked := .m1Checked, result := .success }
  | .failed => some { checked := .m1Checked, result := .ownerFailure }

def replyAt (frontier : Frontier) (request : Request) : Option ReplyRecord :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => some { role := .reply0, value := .value0, provenance := .reply0, redaction := .visible }
  | .received => some { role := .reply0, value := .value0, provenance := .reply0, redaction := .visible }
  | .consumed => some { role := .reply0, value := .value0, provenance := .reply0, redaction := .visible }
  | .failed => none

def receiptAt (frontier : Frontier) (request : Request) : Option ReceiptRecord :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => some { reply := .reply0, receipt := .matching }
  | .received => some { reply := .reply0, receipt := .matching }
  | .consumed => some { reply := .reply0, receipt := .matching }
  | .failed => none

def failureAt (frontier : Frontier) (request : Request) : Option FailureRecord :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => none
  | .received => none
  | .consumed => none
  | .failed => some { role := .ownerFailure, reason := .validationFailed }

def heldAt (frontier : Frontier) (request : Request) : HeldDisposition :=
  match phaseAt frontier request with
  | .awaiting => .holdingGammaDelta
  | .replied => .holdingGammaDelta
  | .received => .holdingGammaDelta
  | .consumed => .deltaConsumed
  | .failed => .deltaReleased

def resultAt (frontier : Frontier) (request : Request) : Option ConsumedValue :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => none
  | .received => none
  | .consumed => some .value0
  | .failed => none

def provenanceAt (frontier : Frontier) (request : Request) : Option Provenance :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => some .reply0
  | .received => some .reply0
  | .consumed => some .reply0
  | .failed => none

def ground0 : Ground := { value := .value0, provenance := .reply0 }

def resumeAt (frontier : Frontier) (request : Request) : Option ResumeRecord :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => none
  | .received => none
  | .consumed => some { receipt := { reply := .reply0, receipt := .matching }, ground := ground0 }
  | .failed => none

def dependencyAt (frontier : Frontier) (request : Request) : Option GroundedDependency :=
  match phaseAt frontier request with
  | .awaiting => none
  | .replied => none
  | .received => none
  | .consumed => some { child := .child0, ground := ground0 }
  | .failed => none

def mutationAt (frontier : Frontier) (request : Request) : Bool :=
  match phaseAt frontier request with
  | .awaiting => false
  | .replied => false
  | .received => false
  | .consumed => true
  | .failed => false

structure DirectView where
  phase : Phase
  validation : Option ValidationRecord
  reply : Option ReplyRecord
  receipt : Option ReceiptRecord
  failure : Option FailureRecord
  held : HeldDisposition
  result : Option ConsumedValue
  provenance : Option Provenance
  resume : Option ResumeRecord
  dependency : Option GroundedDependency
  mutation : Bool

def directView (frontier : Frontier) (request : Request) : DirectView :=
  { phase := phaseAt frontier request
    validation := validationAt frontier request
    reply := replyAt frontier request
    receipt := receiptAt frontier request
    failure := failureAt frontier request
    held := heldAt frontier request
    result := resultAt frontier request
    provenance := provenanceAt frontier request
    resume := resumeAt frontier request
    dependency := dependencyAt frontier request
    mutation := mutationAt frontier request }

def restore : Request -> Request
  | .q0 => .q1
  | .q1 => .q0

def loadedView (frontier : Frontier) (savedRequest : Request) : DirectView :=
  directView frontier (restore savedRequest)

def receiptExtension : Frontier -> Request -> ReceiptRole -> Option Frontier
  | .awaiting, .q0, .matching => none
  | .replied, .q0, .matching => some .received
  | .received, .q0, .matching => none
  | .consumed, .q0, .matching => none
  | .failed, .q0, .matching => none
  | .awaiting, .q1, .matching => none
  | .replied, .q1, .matching => none
  | .received, .q1, .matching => none
  | .consumed, .q1, .matching => none
  | .failed, .q1, .matching => none
  | .awaiting, .q0, .wrongLocus => none
  | .replied, .q0, .wrongLocus => none
  | .received, .q0, .wrongLocus => none
  | .consumed, .q0, .wrongLocus => none
  | .failed, .q0, .wrongLocus => none
  | .awaiting, .q1, .wrongLocus => none
  | .replied, .q1, .wrongLocus => none
  | .received, .q1, .wrongLocus => none
  | .consumed, .q1, .wrongLocus => none
  | .failed, .q1, .wrongLocus => none

def resumeExtension : Frontier -> Request -> Option Frontier
  | .awaiting, .q0 => none
  | .replied, .q0 => none
  | .received, .q0 => some .consumed
  | .consumed, .q0 => none
  | .failed, .q0 => none
  | .awaiting, .q1 => none
  | .replied, .q1 => none
  | .received, .q1 => none
  | .consumed, .q1 => none
  | .failed, .q1 => none

def receiptThenResume (frontier : Frontier) (request : Request) (receipt : ReceiptRole) :
    Option Frontier :=
  match receiptExtension frontier request receipt with
  | none => none
  | some next => resumeExtension next request

def authorizationFromRequest : Request -> Bool := fun _ => false

def authorizationFromIncidental : Incidental -> Bool := fun _ => false

def authorizationFromReceipt : ReceiptRole -> Bool := fun _ => false

def authorizationFromProvenance : Option Provenance -> Bool := fun _ => false

theorem requestsDistinct : Request.q0 ≠ Request.q1 := by
  intro sameRequest
  cases sameRequest

theorem noIncidentalLeftInverse :
    ¬ Exists (fun recover : Incidental -> Request =>
      forall request, recover (incidental request) = request) := by
  intro hasRecovery
  rcases hasRecovery with ⟨recover, recovers⟩
  have sameRecovered : recover (incidental .q0) = recover (incidental .q1) := by
    rfl
  have q0IsRecovered : recover (incidental .q0) = .q0 := recovers .q0
  have q1IsRecovered : recover (incidental .q1) = .q1 := recovers .q1
  exact requestsDistinct (q0IsRecovered.symm.trans (sameRecovered.trans q1IsRecovered))

theorem equalIncidentalDifferentPhase :
    incidental .q0 = incidental .q1 /\
      phaseAt .replied .q0 ≠ phaseAt .replied .q1 := by
  constructor
  · rfl
  · intro samePhase
    cases samePhase

theorem restoreIsInjective : Function.Injective restore := by
  intro left right sameRestored
  cases left <;> cases right <;> cases sameRestored <;> rfl

theorem restoreIsInvolutive (request : Request) : restore (restore request) = request := by
  cases request <;> rfl

theorem restoredViewMatchesSaved (frontier : Frontier) (request : Request) :
    loadedView frontier (restore request) = directView frontier request := by
  cases frontier <;> cases request <;> rfl

theorem distinctDirectViewsAtReplyFrontier :
    directView .replied .q0 ≠ directView .replied .q1 := by
  intro sameView
  have samePhase : phaseAt .replied .q0 = phaseAt .replied .q1 :=
    congrArg DirectView.phase sameView
  cases samePhase

theorem matchingReceiptExtension :
    receiptExtension .replied .q0 .matching = some .received := by
  rfl

theorem duplicateReceiptExtensionIsRejected :
    receiptExtension .received .q0 .matching = none := by
  rfl

theorem lateReceiptExtensionIsRejected :
    receiptExtension .consumed .q0 .matching = none := by
  rfl

theorem wrongLocusReceiptExtensionIsRejected :
    receiptExtension .replied .q0 .wrongLocus = none := by
  rfl

theorem q1ReceiptExtensionIsRejected :
    receiptExtension .replied .q1 .matching = none := by
  rfl

theorem receiptExtensionIsUnique (frontier : Frontier) (request : Request)
    (receipt : ReceiptRole) (next : Frontier) :
    receiptExtension frontier request receipt = some next ->
      frontier = .replied /\ request = .q0 /\ receipt = .matching /\ next = .received := by
  cases frontier <;> cases request <;> cases receipt <;> cases next <;> intro accepted
  all_goals first | exact ⟨rfl, rfl, rfl, rfl⟩ | cases accepted

theorem resumeExtensionIsUnique (frontier : Frontier) (request : Request) (next : Frontier) :
    resumeExtension frontier request = some next ->
      frontier = .received /\ request = .q0 /\ next = .consumed := by
  cases frontier <;> cases request <;> cases next <;> intro resumed
  all_goals first | exact ⟨rfl, rfl, rfl⟩ | cases resumed

theorem matchingReceiptHasOneScopedResume :
    receiptThenResume .replied .q0 .matching = some .consumed := by
  rfl

theorem rejectedReceiptHasNoCombinedResume (frontier : Frontier) (request : Request)
    (receipt : ReceiptRole) (rejected : receiptExtension frontier request receipt = none) :
    receiptThenResume frontier request receipt = none := by
  unfold receiptThenResume
  rw [rejected]

theorem terminalFailureExcludesSuccessContinuation (frontier : Frontier) :
    failureAt frontier .q1 = some { role := .ownerFailure, reason := .validationFailed } ->
      replyAt frontier .q1 = none /\
        receiptAt frontier .q1 = none /\
          resultAt frontier .q1 = none /\
            provenanceAt frontier .q1 = none /\
              resumeAt frontier .q1 = none /\
                dependencyAt frontier .q1 = none /\
                  mutationAt frontier .q1 = false := by
  cases frontier <;> intro hasFailure
  · cases hasFailure
  · exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩
  · exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩
  · exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩
  · exact ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

theorem opaqueInputsAreNotAuthority (request : Request) :
    authorizationFromRequest request = false /\
      authorizationFromIncidental (incidental request) = false /\
        authorizationFromReceipt .matching = false /\
          authorizationFromProvenance (provenanceAt .consumed request) = false := by
  cases request <;> exact ⟨rfl, rfl, rfl, rfl⟩

#print axioms noIncidentalLeftInverse
#print axioms equalIncidentalDifferentPhase
#print axioms restoreIsInjective
#print axioms restoreIsInvolutive
#print axioms restoredViewMatchesSaved
#print axioms distinctDirectViewsAtReplyFrontier
#print axioms receiptExtensionIsUnique
#print axioms resumeExtensionIsUnique
#print axioms matchingReceiptHasOneScopedResume
#print axioms rejectedReceiptHasNoCombinedResume
#print axioms terminalFailureExcludesSuccessContinuation
#print axioms opaqueInputsAreNotAuthority

end BPrimaryOpaqueLab
-- END PINNED WRK-0037 BASELINE

namespace BPrimaryOpaqueLab

-- BEGIN RELATION DEFINITIONS
def relSuccessValidation : ValidationRecord := { checked := .m1Checked, result := .success }
def relFailureValidation : ValidationRecord := { checked := .m1Checked, result := .ownerFailure }
def relReply : ReplyRecord := { role := .reply0, value := .value0, provenance := .reply0, redaction := .visible }
def relReceipt : ReceiptRecord := { reply := .reply0, receipt := .matching }
def relFailure : FailureRecord := { role := .ownerFailure, reason := .validationFailed }
def relGround : Ground := { value := .value0, provenance := .reply0 }
def relResume : ResumeRecord := { receipt := relReceipt, ground := relGround }
def relDependency : GroundedDependency := { child := .child0, ground := relGround }

inductive CellR : Frontier -> Request -> Phase -> Option ValidationRecord -> Option ReplyRecord -> Option ReceiptRecord -> Option FailureRecord -> HeldDisposition -> Option ConsumedValue -> Option Provenance -> Option ResumeRecord -> Option GroundedDependency -> Bool -> Prop where
  | awaitingQ0 : CellR .awaiting .q0 .awaiting none none none none .holdingGammaDelta none none none none false
  | awaitingQ1 : CellR .awaiting .q1 .awaiting none none none none .holdingGammaDelta none none none none false
  | repliedQ0 : CellR .replied .q0 .replied (some relSuccessValidation) (some relReply) (some relReceipt) none .holdingGammaDelta none (some .reply0) none none false
  | repliedQ1 : CellR .replied .q1 .failed (some relFailureValidation) none none (some relFailure) .deltaReleased none none none none false
  | receivedQ0 : CellR .received .q0 .received (some relSuccessValidation) (some relReply) (some relReceipt) none .holdingGammaDelta none (some .reply0) none none false
  | receivedQ1 : CellR .received .q1 .failed (some relFailureValidation) none none (some relFailure) .deltaReleased none none none none false
  | consumedQ0 : CellR .consumed .q0 .consumed (some relSuccessValidation) (some relReply) (some relReceipt) none .deltaConsumed (some .value0) (some .reply0) (some relResume) (some relDependency) true
  | consumedQ1 : CellR .consumed .q1 .failed (some relFailureValidation) none none (some relFailure) .deltaReleased none none none none false
  | failedQ0 : CellR .failed .q0 .consumed (some relSuccessValidation) (some relReply) (some relReceipt) none .deltaConsumed (some .value0) (some .reply0) (some relResume) (some relDependency) true
  | failedQ1 : CellR .failed .q1 .failed (some relFailureValidation) none none (some relFailure) .deltaReleased none none none none false

inductive IncidentalR : Request -> Incidental -> Prop where
  | q0 : IncidentalR .q0 { payload := 7, m1Claims := 11, principal := 13, sourceLocus := 17, ownerLocus := 19, sourceSpan := 23, failureRow := 29, transportSession := 31, queueContext := 37, predecessorShape := 41 }
  | q1 : IncidentalR .q1 { payload := 7, m1Claims := 11, principal := 13, sourceLocus := 17, ownerLocus := 19, sourceSpan := 23, failureRow := 29, transportSession := 31, queueContext := 37, predecessorShape := 41 }

inductive ReceiptResultR : Frontier -> Request -> ReceiptRole -> Option Frontier -> Prop where
  | awaitingQ0Matching : ReceiptResultR .awaiting .q0 .matching none
  | awaitingQ0Wrong : ReceiptResultR .awaiting .q0 .wrongLocus none
  | awaitingQ1Matching : ReceiptResultR .awaiting .q1 .matching none
  | awaitingQ1Wrong : ReceiptResultR .awaiting .q1 .wrongLocus none
  | repliedQ0Matching : ReceiptResultR .replied .q0 .matching (some .received)
  | repliedQ0Wrong : ReceiptResultR .replied .q0 .wrongLocus none
  | repliedQ1Matching : ReceiptResultR .replied .q1 .matching none
  | repliedQ1Wrong : ReceiptResultR .replied .q1 .wrongLocus none
  | receivedQ0Matching : ReceiptResultR .received .q0 .matching none
  | receivedQ0Wrong : ReceiptResultR .received .q0 .wrongLocus none
  | receivedQ1Matching : ReceiptResultR .received .q1 .matching none
  | receivedQ1Wrong : ReceiptResultR .received .q1 .wrongLocus none
  | consumedQ0Matching : ReceiptResultR .consumed .q0 .matching none
  | consumedQ0Wrong : ReceiptResultR .consumed .q0 .wrongLocus none
  | consumedQ1Matching : ReceiptResultR .consumed .q1 .matching none
  | consumedQ1Wrong : ReceiptResultR .consumed .q1 .wrongLocus none
  | failedQ0Matching : ReceiptResultR .failed .q0 .matching none
  | failedQ0Wrong : ReceiptResultR .failed .q0 .wrongLocus none
  | failedQ1Matching : ReceiptResultR .failed .q1 .matching none
  | failedQ1Wrong : ReceiptResultR .failed .q1 .wrongLocus none

inductive ResumeResultR : Frontier -> Request -> Option Frontier -> Prop where
  | awaitingQ0 : ResumeResultR .awaiting .q0 none
  | awaitingQ1 : ResumeResultR .awaiting .q1 none
  | repliedQ0 : ResumeResultR .replied .q0 none
  | repliedQ1 : ResumeResultR .replied .q1 none
  | receivedQ0 : ResumeResultR .received .q0 (some .consumed)
  | receivedQ1 : ResumeResultR .received .q1 none
  | consumedQ0 : ResumeResultR .consumed .q0 none
  | consumedQ1 : ResumeResultR .consumed .q1 none
  | failedQ0 : ResumeResultR .failed .q0 none
  | failedQ1 : ResumeResultR .failed .q1 none

inductive RestoreR : Request -> Request -> Prop where
  | q0 : RestoreR .q0 .q1
  | q1 : RestoreR .q1 .q0

def ReceiptThenResumeR (frontier : Frontier) (request : Request) (role : ReceiptRole) (outcome : Option Frontier) : Prop :=
  (ReceiptResultR frontier request role none /\ outcome = none) \/
    Exists fun middle => ReceiptResultR frontier request role (some middle) /\ ResumeResultR middle request outcome
-- END RELATION DEFINITIONS

abbrev BundledFiber (frontier : Frontier) (request : Request) :=
  { view : DirectView // view = directView frontier request }

-- This is existential witness packaging for CellR, not a second lookup or profile table.
set_option linter.unusedVariables false in
abbrev CellWitnessTuple :=
  Sigma fun phase : Phase => Sigma fun validation : Option ValidationRecord => Sigma fun reply : Option ReplyRecord => Sigma fun receipt : Option ReceiptRecord => Sigma fun failure : Option FailureRecord => Sigma fun held : HeldDisposition => Sigma fun result : Option ConsumedValue => Sigma fun provenance : Option Provenance => Sigma fun resume : Option ResumeRecord => Sigma fun dependency : Option GroundedDependency => Sigma fun mutation : Bool => PUnit

abbrev RelFiber (frontier : Frontier) (request : Request) :=
  { row : CellWitnessTuple // CellR frontier request row.1 row.2.1 row.2.2.1 row.2.2.2.1 row.2.2.2.2.1 row.2.2.2.2.2.1 row.2.2.2.2.2.2.1 row.2.2.2.2.2.2.2.1 row.2.2.2.2.2.2.2.2.1 row.2.2.2.2.2.2.2.2.2.1 row.2.2.2.2.2.2.2.2.2.2.1 }

def toRel (frontier : Frontier) (request : Request) (bundle : BundledFiber frontier request) : RelFiber frontier request := by
  rcases bundle with ⟨view, viewEq⟩
  subst view
  cases frontier <;> cases request
  · exact ⟨⟨.awaiting, none, none, none, none, .holdingGammaDelta, none, none, none, none, false, PUnit.unit⟩, CellR.awaitingQ0⟩
  · exact ⟨⟨.awaiting, none, none, none, none, .holdingGammaDelta, none, none, none, none, false, PUnit.unit⟩, CellR.awaitingQ1⟩
  · exact ⟨⟨.replied, some relSuccessValidation, some relReply, some relReceipt, none, .holdingGammaDelta, none, some .reply0, none, none, false, PUnit.unit⟩, CellR.repliedQ0⟩
  · exact ⟨⟨.failed, some relFailureValidation, none, none, some relFailure, .deltaReleased, none, none, none, none, false, PUnit.unit⟩, CellR.repliedQ1⟩
  · exact ⟨⟨.received, some relSuccessValidation, some relReply, some relReceipt, none, .holdingGammaDelta, none, some .reply0, none, none, false, PUnit.unit⟩, CellR.receivedQ0⟩
  · exact ⟨⟨.failed, some relFailureValidation, none, none, some relFailure, .deltaReleased, none, none, none, none, false, PUnit.unit⟩, CellR.receivedQ1⟩
  · exact ⟨⟨.consumed, some relSuccessValidation, some relReply, some relReceipt, none, .deltaConsumed, some .value0, some .reply0, some relResume, some relDependency, true, PUnit.unit⟩, CellR.consumedQ0⟩
  · exact ⟨⟨.failed, some relFailureValidation, none, none, some relFailure, .deltaReleased, none, none, none, none, false, PUnit.unit⟩, CellR.consumedQ1⟩
  · exact ⟨⟨.consumed, some relSuccessValidation, some relReply, some relReceipt, none, .deltaConsumed, some .value0, some .reply0, some relResume, some relDependency, true, PUnit.unit⟩, CellR.failedQ0⟩
  · exact ⟨⟨.failed, some relFailureValidation, none, none, some relFailure, .deltaReleased, none, none, none, none, false, PUnit.unit⟩, CellR.failedQ1⟩

def toBundle {frontier : Frontier} {request : Request} (relation : RelFiber frontier request) : BundledFiber frontier request := by
  rcases relation with ⟨⟨phase, validation, reply, receipt, failure, held, result, provenance, resume, dependency, mutation, _⟩, cell⟩
  refine ⟨{ phase := phase, validation := validation, reply := reply, receipt := receipt, failure := failure, held := held, result := result, provenance := provenance, resume := resume, dependency := dependency, mutation := mutation }, ?_⟩
  cases cell <;> rfl

theorem toBundleToRel (frontier : Frontier) (request : Request) (bundle : BundledFiber frontier request) :
    toBundle (toRel frontier request bundle) = bundle := by
  rcases bundle with ⟨view, viewEq⟩
  subst view
  apply Subtype.ext
  cases frontier <;> cases request <;> rfl

theorem toRelToBundle {frontier : Frontier} {request : Request} (relation : RelFiber frontier request) :
    toRel frontier request (toBundle relation) = relation := by
  rcases relation with ⟨⟨phase, validation, reply, receipt, failure, held, result, provenance, resume, dependency, mutation, _⟩, cell⟩
  cases cell <;> rfl

theorem incidentalRComplete (request : Request) : IncidentalR request (incidental request) := by
  cases request
  · exact IncidentalR.q0
  · exact IncidentalR.q1

theorem incidentalRSound (request : Request) (item : Incidental) : IncidentalR request item -> item = incidental request := by
  intro relation
  cases relation <;> rfl

theorem receiptResultRSound (frontier : Frontier) (request : Request) (role : ReceiptRole) (outcome : Option Frontier) :
    ReceiptResultR frontier request role outcome -> receiptExtension frontier request role = outcome := by
  intro relation
  cases relation <;> rfl

theorem receiptResultRComplete (frontier : Frontier) (request : Request) (role : ReceiptRole) :
    ReceiptResultR frontier request role (receiptExtension frontier request role) := by
  cases frontier <;> cases request <;> cases role
  all_goals first
  | exact ReceiptResultR.awaitingQ0Matching
  | exact ReceiptResultR.awaitingQ0Wrong
  | exact ReceiptResultR.awaitingQ1Matching
  | exact ReceiptResultR.awaitingQ1Wrong
  | exact ReceiptResultR.repliedQ0Matching
  | exact ReceiptResultR.repliedQ0Wrong
  | exact ReceiptResultR.repliedQ1Matching
  | exact ReceiptResultR.repliedQ1Wrong
  | exact ReceiptResultR.receivedQ0Matching
  | exact ReceiptResultR.receivedQ0Wrong
  | exact ReceiptResultR.receivedQ1Matching
  | exact ReceiptResultR.receivedQ1Wrong
  | exact ReceiptResultR.consumedQ0Matching
  | exact ReceiptResultR.consumedQ0Wrong
  | exact ReceiptResultR.consumedQ1Matching
  | exact ReceiptResultR.consumedQ1Wrong
  | exact ReceiptResultR.failedQ0Matching
  | exact ReceiptResultR.failedQ0Wrong
  | exact ReceiptResultR.failedQ1Matching
  | exact ReceiptResultR.failedQ1Wrong

theorem resumeResultRSound (frontier : Frontier) (request : Request) (outcome : Option Frontier) :
    ResumeResultR frontier request outcome -> resumeExtension frontier request = outcome := by
  intro relation
  cases relation <;> rfl

theorem resumeResultRComplete (frontier : Frontier) (request : Request) :
    ResumeResultR frontier request (resumeExtension frontier request) := by
  cases frontier <;> cases request
  all_goals first
  | exact ResumeResultR.awaitingQ0
  | exact ResumeResultR.awaitingQ1
  | exact ResumeResultR.repliedQ0
  | exact ResumeResultR.repliedQ1
  | exact ResumeResultR.receivedQ0
  | exact ResumeResultR.receivedQ1
  | exact ResumeResultR.consumedQ0
  | exact ResumeResultR.consumedQ1
  | exact ResumeResultR.failedQ0
  | exact ResumeResultR.failedQ1

theorem restoreRSound (saved live : Request) : RestoreR saved live -> restore saved = live := by
  intro relation
  cases relation <;> rfl

theorem restoreRComplete (saved : Request) : RestoreR saved (restore saved) := by
  cases saved
  · exact RestoreR.q0
  · exact RestoreR.q1

theorem receiptThenResumeRSound (frontier : Frontier) (request : Request) (role : ReceiptRole) (outcome : Option Frontier) :
    ReceiptThenResumeR frontier request role outcome -> receiptThenResume frontier request role = outcome := by
  intro relation
  rcases relation with ⟨receiptNone, outcomeNone⟩ | ⟨middle, receiptSome, resumed⟩
  · have checked := receiptResultRSound frontier request role none receiptNone
    unfold receiptThenResume
    rw [checked]
    exact outcomeNone.symm
  · have checkedReceipt := receiptResultRSound frontier request role (some middle) receiptSome
    have checkedResume := resumeResultRSound middle request outcome resumed
    unfold receiptThenResume
    rw [checkedReceipt]
    exact checkedResume

theorem receiptThenResumeRComplete (frontier : Frontier) (request : Request) (role : ReceiptRole) :
    ReceiptThenResumeR frontier request role (receiptThenResume frontier request role) := by
  unfold receiptThenResume
  cases receiptStep : receiptExtension frontier request role with
  | none =>
      apply Or.inl
      constructor
      · rw [← receiptStep]
        exact receiptResultRComplete frontier request role
      · rfl
  | some middle =>
      apply Or.inr
      exact ⟨middle, by rw [← receiptStep]; exact receiptResultRComplete frontier request role, resumeResultRComplete middle request⟩

theorem fiberRoundTripPreservesEveryView (frontier : Frontier) (request : Request) (bundle : BundledFiber frontier request) :
    (toBundle (toRel frontier request bundle)).1 = bundle.1 := by
  exact congrArg Subtype.val (toBundleToRel frontier request bundle)

theorem localRestoreCommutesWithFiber (frontier : Frontier) (request : Request) :
    loadedView frontier (restore request) =
      (toBundle (toRel frontier request ⟨directView frontier request, rfl⟩)).1 := by
  cases frontier <;> cases request <;> rfl

theorem restoreRCommutesWithFiber (frontier : Frontier) {saved live : Request}
    (relation : RestoreR saved live) :
    loadedView frontier saved =
      (toBundle (toRel frontier live ⟨directView frontier live, rfl⟩)).1 := by
  cases relation <;> cases frontier <;> rfl

theorem noIncidentalRRecovery :
    ¬ Exists (fun recover : Incidental -> Request =>
      forall request item, IncidentalR request item -> recover item = request) := by
  intro hasRecovery
  rcases hasRecovery with ⟨recover, recovers⟩
  have q0Recovered : recover (incidental .q0) = .q0 := recovers .q0 (incidental .q0) IncidentalR.q0
  have q1Recovered : recover (incidental .q1) = .q1 := recovers .q1 (incidental .q1) IncidentalR.q1
  have sameRecovered : recover (incidental .q0) = recover (incidental .q1) := by rfl
  exact requestsDistinct (q0Recovered.symm.trans (sameRecovered.trans q1Recovered))

theorem noBareViewRecovery :
    ¬ Exists (fun recover : DirectView -> Request =>
      forall frontier request, recover (directView frontier request) = request) := by
  intro hasRecovery
  rcases hasRecovery with ⟨recover, recovers⟩
  have q0Recovered := recovers .awaiting .q0
  have q1Recovered := recovers .awaiting .q1
  have sameView : directView .awaiting .q0 = directView .awaiting .q1 := by rfl
  exact requestsDistinct (q0Recovered.symm.trans (sameView ▸ q1Recovered))

#print axioms toBundleToRel
#print axioms toRelToBundle
#print axioms requestsDistinct
#print axioms matchingReceiptExtension
#print axioms duplicateReceiptExtensionIsRejected
#print axioms lateReceiptExtensionIsRejected
#print axioms wrongLocusReceiptExtensionIsRejected
#print axioms q1ReceiptExtensionIsRejected
#print axioms incidentalRComplete
#print axioms incidentalRSound
#print axioms receiptResultRComplete
#print axioms receiptResultRSound
#print axioms resumeResultRComplete
#print axioms resumeResultRSound
#print axioms restoreRComplete
#print axioms restoreRSound
#print axioms receiptThenResumeRComplete
#print axioms receiptThenResumeRSound
#print axioms fiberRoundTripPreservesEveryView
#print axioms localRestoreCommutesWithFiber
#print axioms restoreRCommutesWithFiber
#print axioms noIncidentalRRecovery
#print axioms noBareViewRecovery

end BPrimaryOpaqueLab
```

## Observation matrix

| Registered condition | Explicit local check | What the check does not infer |
| --- | --- | --- |
| exact baseline | the source between the pinned markers is byte-identical to the registered WRK-0037 Lean source | that the baseline selects a Canon carrier or semantics |
| all supplied cell fibers | `CellR` has one explicit constructor for each of ten `(Frontier, Request)` cells; `toRel` and `toBundle` prove both pointwise round trips | a global relation over unspecified or reachable states |
| separate graph definitions | `CellR`, `IncidentalR`, `ReceiptResultR`, `ResumeResultR`, and `RestoreR` are enumerated without a `DirectView` or lookup reference | a general relation-first implementation or data model |
| view observations | `toBundleToRel`, `toRelToBundle`, and `fiberRoundTripPreservesEveryView` preserve every represented view column at its supplied key | reconstruction of a key from the bare view value |
| incidental observation | `incidentalRComplete`, `incidentalRSound`, and `noIncidentalRRecovery` cover both equal-incidental rows and reject recovery from that observation | an authorization, correlation, or source-elaboration rule |
| receipt and resume tables | completeness and soundness theorems cover all twenty receipt outcomes, all ten resume outcomes including `none`, and the derived combined result | delivery, retry, exactly-once, fairness, or scheduling semantics |
| local restore | `restoreRComplete`, `restoreRSound`, and `restoreRCommutesWithFiber` cover the two finite swap rows at every supplied cell | persistence, recovery, or identity across independent loads |
| deliberate non-invertibility | `noBareViewRecovery` rejects one total `DirectView -> Request` recovery function for all supplied keys, because some keys collide as values | that every individual view is non-discriminating, or that no partial/additional-premise recovery exists |

## Reproduction and results

The outcome procedure extracts the sole Lean block to a disposable temporary
file, extracts the old WRK-0037 block to a second temporary file, and compares
the marked baseline bytes. It separately scans the relation-definition region
for the prohibited view and lookup names, compiles the successor with
`lean --trust=0`, prints the retained theorem axioms, and scans the extracted
source for placeholder, unsafe, classical-choice, quotient, native-decision,
and axiom tokens.

The observed finite result is restricted to the registered table: for every
supplied key, the cell/view relation fiber and bundled lookup fiber translate in
both directions and preserve every `DirectView` column. Separate soundness,
completeness, and commutation theorems establish agreement for incidental,
receipt, resume, derived combined, and restore observations, including every
rejected `none` result. The two incidental rows admit no total `Incidental -> Request` left
inverse. The direct-view table admits no single total `DirectView -> Request`
function that recovers the request for all ten supplied keys, because some
supplied keys have identical views; this does not assert that every individual
view is non-discriminating or rule out recovery with further premises. This
demonstrates neither a project carrier choice nor a source-level
omission/inference rule. Source-level ergonomics remain outside this L3 scope.

## Non-claims

This finite check does not select Family A, B, or C; define Mir occurrence or
request equality/correlation; introduce a Core constructor, typed effect,
Config/history/SaveObject/queue/wire field, source rule, syntax, elaboration,
Canon authority relation, Canon restore law, contract, Canon theorem/OBL, Gate, Phase, runtime,
implementation, API, sample, or public behavior. It does not prove delivery,
retry, timeout, cancellation, fairness, transport exactly-once, persistence,
distributed behavior, global semantic equivalence, or a general graph model.
It does not authorize inference from implicit context. The temporary Lean file
is not a module, schema, checker input, or stable downstream artifact.
