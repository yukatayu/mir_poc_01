# WRK-0037 C2-B/C3 B-primary opaque-anchor evidence

This is **LAB** evidence for `working/WRK-0037` after its registration cut.
The fenced source is a disposable, artifact-local Lean check. Its labels,
constructors, functions, and theorem names do not denote Mir request or
occurrence identity, Core state, Config, history, SaveObject, authority,
source syntax, runtime behavior, or a public interface.

## Retained question

Can one fixed finite presentation keep two distinct request atoms even when all
listed incidental observations are equal, while making every staged result
explicitly keyed by that atom and preserving it through a stated injective
restore? The model gives `q0` one success branch and `q1` one terminal-failure
branch. `DirectView` records phase, validation outcome, reply, receipt,
failure, held-context disposition, result, provenance, resume, and dependency
for each frontier/request pair.

In this two-request table, `noIncidentalLeftInverse` proves that the listed
incidental record has no total left inverse that recovers both request atoms.
Together with `equalIncidentalDifferentPhase`, it records only that equal
incidental records coexist with different reply-frontier phases in this model.
It makes no claim about recovery using other context, a different carrier, or
any future ergonomic inference rule.

`ReceiptRole.wrongLocus` is only an adversarial local label. It models neither
a real locus nor a transport protocol. `receiptExtension` is a finite table, not
delivery, retry, exactly-once, or scheduling semantics.

## Registered Lean source

```lean
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
```

## Observation matrix

| Registered condition | Explicit local check | What the check does not infer |
| --- | --- | --- |
| distinct opaque requests | `requestsDistinct`, `equalIncidentalDifferentPhase`, and `noIncidentalLeftInverse` | a Mir occurrence equality rule, recovery rule, or request carrier |
| direct staged state | `DirectView` contains every registered q-indexed field | a Config/history/SaveObject or wire representation |
| restore | `restoreIsInvolutive` and `restoredViewMatchesSaved` quantify every finite frontier/request pair | arbitrary injective restore, identity across independent loads, or a persistence contract |
| one scoped receipt/resume | `receiptExtensionIsUnique`, `resumeExtensionIsUnique`, and `rejectedReceiptHasNoCombinedResume` define one finite combined path | transport exactly-once, retry, fairness, or a scheduler law |
| failure branch | `terminalFailureExcludesSuccessContinuation` removes success-only fields and mutation for `q1` | a general failure semantics or mutation theorem |
| authority separation | every local request/incidental/receipt/provenance authorization table is false | a Canon authorization relation |

## Reproduction and results

The exact WRK-0037 registered command extracts the sole Lean block above to
`${TMPDIR:-/tmp}/mir-wrk0037-c2b-c3-b-primary-opaque-anchor.lean` and invokes
`lean --trust=0` on it. Before the final source, the test-first draft omitted
the restore renaming from `loadedView`; Lean rejected the assertion that the
loaded image of `q0` had `q0`'s reply-frontier phase. The final source makes
the rename explicit and retains the corresponding all-frontier proof. A second
test-first draft accepted the same matching receipt at `received`; Lean rejected
the required duplicate-rejection assertion before the final extension table was
written.

The observed finite result is limited to this local table: two equal-incidental
records have no total left inverse recovering both
request atoms, and this explicitly defined involutive swap preserves every
represented lookup field when read through the inverse saved key. It does not
show that this is the right project carrier, a persistence model, a recovery
rule, or a user-facing notation. The experiment makes no claim about inference
or carriers outside this artifact.

## Non-claims

This finite check does not select Family A, B, or C; define Mir occurrence or
request equality/correlation; introduce a Core constructor, typed effect,
Config/history/SaveObject/queue/wire field, source rule, syntax, elaboration,
authority relation, restore law, contract, theorem/OBL, Gate, Phase, runtime,
implementation, API, sample, or public behavior. It does not prove delivery,
retry, timeout, cancellation, fairness, transport exactly-once, persistence,
or distributed behavior. The temporary Lean file is not a module, schema,
checker input, or stable downstream artifact.
