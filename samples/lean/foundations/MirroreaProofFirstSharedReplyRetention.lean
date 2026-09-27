import MirroreaProofFirstSharedWireLifetime
namespace MirroreaProofFirst.SharedReplyRetention

-- One already armed, occurrence-bound request. Raw receipt, semantic validation,
-- raw handoff, proof-side validation/promotion, and retirement are distinct. This is NOT the outer host journal:
-- raw handoff neither validates a reply nor commits host mirrors/releases a gate.
-- Validation is the actual pure certificate checker, not a claim that Python
-- executes that complete checker before returning bytes to CreditWriter.
-- Actual owned endpoint/byte custody and deterministic native code remain the
-- same explicit physical premises as SharedWireLifetime.
open SharedWireLifetime
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

inductive Phase where
  | waiting
  | raw (bytes : List UInt8)
  | handed (bytes : List UInt8)
  | validated (bytes : List UInt8)
  | promoted (bytes : List UInt8)
  deriving DecidableEq, Repr

structure Slot (p a : Nat) where
  phase : Phase
  stopped : Bool
  applied : Bool
  physical : SharedNativeStep.State p a

inductive Action where
  | deliver
  | retain (bytes : List UInt8)
  | handoff
  | validate
  | promote
  | stop
  deriving DecidableEq, Repr

variable (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)

def armed : Slot p a := ⟨.waiting,false,false,native t.before⟩

def invariant (s : Slot p a) : Prop :=
  s.physical = pendingPhysical t s.applied ∧
  match s.phase with
  | .waiting => True
  | .raw _ | .handed _ => s.applied = true
  | .validated bytes | .promoted bytes => s.applied = true ∧ bytes = reply t.before t.request

-- These rules prescribe independent assignments, not a correspondence guard.
-- Delivery may occur after retirement, just as for the underlying wire model.
inductive Step : Slot p a → Action → Slot p a → Prop where
  | deliver : Step ⟨.waiting,stopped,false,physical⟩ .deliver
      ⟨.waiting,stopped,true,execute t physical⟩
  | retain : Step ⟨.waiting,false,true,physical⟩ (.retain bytes) ⟨.raw bytes,false,true,physical⟩
  | handoff : Step ⟨.raw bytes,false,true,physical⟩ .handoff ⟨.handed bytes,false,true,physical⟩
  | validate : Step ⟨.handed (reply t.before t.request),false,true,physical⟩ .validate
      ⟨.validated (reply t.before t.request),false,true,physical⟩
  | reject (wrong : bytes ≠ reply t.before t.request) :
      Step ⟨.handed bytes,false,true,physical⟩ .validate ⟨.handed bytes,true,true,physical⟩
  | promote : Step ⟨.validated bytes,false,true,physical⟩ .promote
      ⟨.promoted bytes,false,true,physical⟩
  | stop : Step ⟨phase,false,applied,physical⟩ .stop ⟨phase,true,applied,physical⟩

def advance (s : Slot p a) (action : Action) : Option (Slot p a) :=
  match s,action with
  | ⟨.waiting,stopped,false,physical⟩,.deliver => some ⟨.waiting,stopped,true,execute t physical⟩
  | ⟨.waiting,false,true,physical⟩,.retain bytes => some ⟨.raw bytes,false,true,physical⟩
  | ⟨.raw bytes,false,true,physical⟩,.handoff => some ⟨.handed bytes,false,true,physical⟩
  | ⟨.handed bytes,false,true,physical⟩,.validate =>
      if bytes = reply t.before t.request then some ⟨.validated bytes,false,true,physical⟩
      else some ⟨.handed bytes,true,true,physical⟩
  | ⟨.validated bytes,false,true,physical⟩,.promote => some ⟨.promoted bytes,false,true,physical⟩
  | ⟨phase,false,applied,physical⟩,.stop => some ⟨phase,true,applied,physical⟩
  | _,_ => none

theorem armed_valid : invariant t (armed t) := ⟨rfl,True.intro⟩

theorem preserves (valid : invariant t s) (step : Step t s action after) : invariant t after := by
  cases step with
  | deliver => exact ⟨congrArg (execute t) valid.1,True.intro⟩
  | retain => exact ⟨valid.1,rfl⟩
  | handoff => exact valid
  | validate => exact ⟨valid.1,rfl,rfl⟩
  | reject wrong => exact ⟨valid.1,rfl⟩
  | promote => exact valid
  | stop => exact valid

theorem advance_complete (step : Step t s action next) : advance t s action = some next := by
  cases step <;> simp_all [advance]

theorem advance_sound (checked : advance t s action = some next) : Step t s action next := by
  rcases s with ⟨phase,stopped,applied,physical⟩
  cases phase <;> cases stopped <;> cases applied <;> cases action <;>
    simp only [advance] at checked <;> try contradiction
  all_goals
    first
    | (split at checked <;> simp_all only [Option.some.injEq] <;> subst next
       · exact .validate
       · exact .reject (by assumption))
    | (cases Option.some.inj checked; constructor)

theorem advance_exact : advance t s action = some next ↔ Step t s action next :=
  ⟨advance_sound t,advance_complete t⟩

-- Forget raw buffering while retaining the precise wire certificate and native
-- state. Stopping AFTER validation retains the validated successor evidence;
-- it is not mapped back to an unknown-before state.
def wire (s : Slot p a) : SharedWireLifetime.State assigned scope bootstrap capacity sourceBudget ownerBudget seed :=
  match s.phase with
  | .waiting | .raw _ | .handed _ => ⟨if s.stopped then .retired t else .awaiting t,s.applied,s.physical⟩
  | .validated _ => ⟨.received t,s.applied,s.physical⟩
  | .promoted _ => ⟨.settled t.next.val,false,s.physical⟩

theorem wire_valid (valid : invariant t s) : Invariant (wire t s) := by
  rcases s with ⟨phase,stopped,applied,physical⟩
  cases phase with
  | waiting => cases stopped <;> exact valid.1
  | raw bytes => cases stopped <;> exact valid.1
  | handed bytes => cases stopped <;> exact valid.1
  | validated bytes =>
      refine ⟨valid.2.1,?_⟩
      have state := valid.1
      have appliedTrue : applied = true := valid.2.1
      change physical = pendingPhysical t applied at state
      simp only [pendingPhysical,appliedTrue,↓reduceIte] at state
      exact state.trans (next_physics t).symm
  | promoted bytes =>
      refine ⟨rfl,?_⟩
      have state := valid.1
      have appliedTrue : applied = true := valid.2.1
      change physical = pendingPhysical t applied at state
      simp only [pendingPhysical,appliedTrue,↓reduceIte] at state
      exact state.trans (next_physics t).symm

theorem refines (step : Step t s action after) :
    ∃ actions, Runs (wire t s) actions (wire t after) := by
  cases step with
  | deliver =>
      rename_i stopped physical
      cases stopped with
      | false => exact ⟨[.deliver],.step .nil .deliver⟩
      | true => exact ⟨[.deliver],.step .nil .lateDeliver⟩
  | retain => exact ⟨[],.nil⟩
  | handoff => exact ⟨[],.nil⟩
  | validate => exact ⟨[.receive (reply t.before t.request)],.step .nil .receive⟩
  | reject wrong => exact ⟨[.receive _],.step .nil (.mismatch wrong)⟩
  | promote => exact ⟨[.promote],.step .nil .promote⟩
  | stop =>
      rename_i phase applied physical
      cases phase with
      | waiting => exact ⟨[.loseReply],.step .nil .lose⟩
      | raw bytes => exact ⟨[.loseReply],.step .nil .lose⟩
      | handed bytes => exact ⟨[.loseReply],.step .nil .lose⟩
      | validated bytes => exact ⟨[],.nil⟩
      | promoted bytes => exact ⟨[],.nil⟩

-- General positive path and the actual capture-write failure cut. Neither
-- requires a particular program/example nor fabricates an endpoint response.
theorem normal_receipt (physical : SharedNativeStep.State p a) :
    Step t ⟨.waiting,false,false,physical⟩ .deliver ⟨.waiting,false,true,execute t physical⟩ ∧
    Step t ⟨.waiting,false,true,execute t physical⟩ (.retain (reply t.before t.request))
      ⟨.raw (reply t.before t.request),false,true,execute t physical⟩ ∧
    Step t ⟨.raw (reply t.before t.request),false,true,execute t physical⟩ .handoff
      ⟨.handed (reply t.before t.request),false,true,execute t physical⟩ ∧
    Step t ⟨.handed (reply t.before t.request),false,true,execute t physical⟩ .validate
      ⟨.validated (reply t.before t.request),false,true,execute t physical⟩ ∧
    Step t ⟨.validated (reply t.before t.request),false,true,execute t physical⟩ .promote
      ⟨.promoted (reply t.before t.request),false,true,execute t physical⟩ :=
  ⟨.deliver,.retain,.handoff,.validate,.promote⟩

theorem raw_capture_failure :
    Step t ⟨.raw bytes,false,true,physical⟩ .stop ⟨.raw bytes,true,true,physical⟩ := .stop

theorem stopped_raw_retained :
    advance t ⟨.raw bytes,true,true,physical⟩ .validate = none ∧
    advance t ⟨.raw bytes,true,true,physical⟩ .promote = none := ⟨rfl,rfl⟩

theorem stopped_known_retained :
    advance t ⟨.validated bytes,true,true,physical⟩ .promote = none := rfl

theorem raw_is_not_promotion :
    advance t ⟨.raw bytes,false,true,physical⟩ .promote = none := rfl

theorem bad_raw_cannot_validate (wrong : bytes ≠ reply t.before t.request) :
    advance t ⟨.handed bytes,false,true,physical⟩ .validate = some ⟨.handed bytes,true,true,physical⟩ := by
  simp [advance,wrong]

theorem stop_frames (step : Step t s .stop after) :
    after.phase = s.phase ∧ after.applied = s.applied ∧ after.physical = s.physical := by
  cases step
  exact ⟨rfl,rfl,rfl⟩

theorem stopped_monotone (step : Step t s action after) (stopped : s.stopped = true) :
    after.stopped = true := by cases step <;> simp_all

def observedBytes : Phase → Option (List UInt8)
  | .waiting => none
  | .raw bytes | .handed bytes | .validated bytes | .promoted bytes => some bytes

theorem retained_bytes_frame (step : Step t s action after)
    (retained : observedBytes s.phase = some bytes) : observedBytes after.phase = some bytes := by
  cases step <;> simp_all [observedBytes]

theorem raw_is_not_validation :
    advance t ⟨.raw bytes,false,true,physical⟩ .validate = none := rfl

-- Arming must carry the actual prepare equation, including off-wire metadata.
-- A bare Attempt does not itself supply this equation. The endpoint is the
-- applied/raw/stopped projection; the no-delivery alternative cannot replace it.
theorem raw_stop_path (prepared : prepare t.before t.request = some t.next) :
    Runs (initial t.before) [.send t.request,.deliver,.loseReply]
      (wire t ⟨.raw bytes,true,true,execute t (native t.before)⟩) := by
  have applied := (unknown_paths t.before t.request t.next prepared).2.1
  have physical : execute t (native t.before) = native t.next.val := (next_physics t).symm
  simpa only [wire,↓reduceIte,physical] using applied

#print axioms stopped_monotone
#print axioms retained_bytes_frame
#print axioms raw_is_not_validation
#print axioms raw_stop_path
#print axioms preserves
#print axioms advance_exact
#print axioms wire_valid
#print axioms refines
#print axioms normal_receipt
#print axioms raw_capture_failure
#print axioms stopped_raw_retained
#print axioms stopped_known_retained
#print axioms raw_is_not_promotion
#print axioms bad_raw_cannot_validate
#print axioms stop_frames
end MirroreaProofFirst.SharedReplyRetention
