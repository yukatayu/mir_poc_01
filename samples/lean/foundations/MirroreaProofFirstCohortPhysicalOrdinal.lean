import MirroreaProofFirstCohortPhase
import MirroreaProofFirstJointDriver
namespace MirroreaProofFirst.CohortPhysicalOrdinal
open CohortPhase

-- Bootstrap has one physical submission and no funding transition. Translate
-- BOTH the live source counter and any retained pending-payment occurrence.
-- This translation does not identify host commit with physical reply.
def pendingAt (offset : Nat) (pending : PaidHeadPhase.Pending p) : PaidHeadPhase.Pending p :=
  {pending with sourceOrdinal := pending.sourceOrdinal+offset}

def modeAt (offset : Nat) : Mode p → Mode p
  | .paid pending => .paid (pendingAt offset pending)
  | other => other

def liveAt (offset : Nat) (s : Live p a) : Live p a :=
  {s with ordinal := s.ordinal+offset,mode := modeAt offset s.mode}

def stateAt (offset : Nat) (s : State p a) : State p a := s.map (liveAt offset)

@[simp] theorem command_at : PaidHeadPhase.command (a:=a) (pendingAt offset pending) = PaidHeadPhase.command pending := rfl
@[simp] theorem request_at : paymentRequest (pendingAt offset pending) source = paymentRequest pending source := rfl
@[simp] theorem reply_at : paymentReply (pendingAt offset pending) = paymentReply pending := rfl
@[simp] theorem vector_at : VectorAt (liveAt offset s) vector ↔ VectorAt s vector := Iff.rfl
@[simp] theorem credits_at : credits (liveAt offset s) = credits s := rfl
@[simp] theorem work_at : WorkCovered (liveAt offset s) dispatch count ↔ WorkCovered s dispatch count := Iff.rfl
@[simp] theorem query_at : QueryAllowed (modeAt offset mode) ↔ QueryAllowed mode := by
  cases mode <;> rfl

@[simp] theorem permits_at :
    PaidHeadPhase.permits (pendingAt offset pending) (ordinal+offset) command = true ↔
    PaidHeadPhase.permits pending ordinal command = true := by
  rw [PaidHeadPhase.permits_exact,PaidHeadPhase.permits_exact]
  simp [pendingAt,PaidHeadPhase.command]

theorem funding_at : Funding (liveAt offset s) ↔ Funding s := by
  cases phase : s.mode <;> simp [Funding,liveAt,modeAt,phase,WorkCovered,pendingAt,PaidHeadPhase.command] <;> (intros; rfl)

theorem invariant_at : Invariant assigned scope bootstrap (stateAt offset s) ↔ Invariant assigned scope bootstrap s := by
  cases s with
  | none => simp [Invariant,stateAt]
  | some s =>
    constructor
    · intro h actual equal
      cases equal
      have valid := h (liveAt offset s) rfl
      exact ⟨valid.1,funding_at.mp valid.2⟩
    · intro h actual equal
      cases equal
      have valid := h s rfl
      exact ⟨valid.1,funding_at.mpr valid.2⟩

@[simp] theorem source_at :
    liveAt offset (sourceResult s next mode) = sourceResult (liveAt offset s) next (modeAt offset mode) := by
  simp [liveAt,sourceResult,Nat.add_assoc,Nat.add_comm,Nat.add_left_comm]
@[simp] theorem owner_at :
    liveAt offset (ownerResult s target next mode) = ownerResult (liveAt offset s) target next (modeAt offset mode) := rfl

-- All ordinary, paid and work-interior rules retain the same native requests,
-- owners and full driver. The payment's transport occurrence is translated too.
theorem step_at
    (step : Step assigned scope bootstrap capacity old event next) :
    Step assigned scope bootstrap capacity (stateAt offset old) event (stateAt offset next) := by
  cases step with
  | frame => exact .frame
  | retire => exact .retire
  | query allowed =>
    simpa [stateAt,liveAt,Nat.add_assoc,Nat.add_comm,Nat.add_left_comm] using
      (Step.query (s:=liveAt offset _) (query_at.mpr allowed))
  | launch mode fresh bound ran =>
    simpa [stateAt,modeAt] using Step.launch (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) fresh bound ran
  | «initialize» mode due present ran =>
    simpa [stateAt,modeAt] using Step.initialize (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) due present ran
  | extraInitialize mode done present slack ran =>
    simpa [stateAt,modeAt] using Step.extraInitialize (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) done present slack ran
  | preludeDone mode done =>
    simpa [stateAt,liveAt,modeAt] using Step.preludeDone (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) done
  | source mode allowedSource bound ran =>
    simpa [stateAt,modeAt] using Step.source (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) allowedSource bound ran
  | @refused s vector input phase bound refused =>
    have translated : (liveAt offset s).mode = .ordinary ∨ ∃ owed, (liveAt offset s).mode = .prelude owed := by
      rcases phase with h | ⟨owed,h⟩
      · exact Or.inl (by simp [liveAt,modeAt,h])
      · exact Or.inr ⟨owed,by simp [liveAt,modeAt,h]⟩
    simpa [stateAt,liveAt,Nat.add_assoc,Nat.add_comm,Nat.add_left_comm] using Step.refused translated bound refused
  | administration mode admin offhead slack ran =>
    simpa [stateAt,modeAt] using Step.administration (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) admin offhead slack ran
  | pay mode present head ordinal ran =>
    simpa [stateAt,modeAt] using Step.pay (s:=liveAt offset _) (pending:=pendingAt offset _)
      (congrArg (modeAt offset) mode) present (by simpa using head)
      (by simpa [pendingAt,liveAt] using congrArg (fun n => n+offset) ordinal) (by simpa using ran)
  | notify mode allowed bound ran =>
    simpa [stateAt,modeAt] using Step.notify (s:=liveAt offset _) (pending:=pendingAt offset _)
      (congrArg (modeAt offset) mode) (permits_at.mpr allowed) bound (by simpa using ran)
  | enter mode bound ran present dispatched head =>
    simpa [stateAt,modeAt] using Step.enter (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) bound ran present dispatched head
  | reserve mode ran =>
    simpa [stateAt,modeAt] using Step.reserve (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) ran
  | probe mode ran =>
    simpa [stateAt,modeAt] using Step.probe (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) ran
  | compute mode ran ticket scopeAt revision =>
    simpa [stateAt,modeAt] using Step.compute (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) ran ticket scopeAt revision
  | finish mode bound ran =>
    simpa [stateAt,modeAt] using Step.finish (s:=liveAt offset _)
      (congrArg (modeAt offset) mode) bound ran

-- A funded lifetime starts immediately AFTER the observed no-reply bootstrap.
-- The actual bootstrap bytes/custody remain the native startup obligation.
def postBootstrap (sourceBudget ownerBudget : Nat) : State p a :=
  stateAt 1 (CohortPhase.initial sourceBudget ownerBudget)

inductive Runs (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) : List (Event p a) → State p a → Prop where
  | postBootstrap : Runs assigned scope bootstrap capacity sourceBudget ownerBudget [] (postBootstrap sourceBudget ownerBudget)
  | step : Runs assigned scope bootstrap capacity sourceBudget ownerBudget events old →
      Step assigned scope bootstrap capacity old event next →
      Runs assigned scope bootstrap capacity sourceBudget ownerBudget (events ++ [event]) next

theorem runs_at (path : CohortPhase.Runs assigned scope bootstrap capacity sourceBudget ownerBudget events state) :
    Runs assigned scope bootstrap capacity sourceBudget ownerBudget events (stateAt 1 state) := by
  induction path with
  | initial => exact .postBootstrap
  | step prior step ih => exact .step ih (step_at step)

theorem runs_invariant (path : Runs assigned scope bootstrap capacity sourceBudget ownerBudget events state) :
    Invariant assigned scope bootstrap state := by
  induction path with
  | postBootstrap => exact invariant_at.mpr CohortPhase.initial_invariant
  | step prior step ih => exact CohortPhase.step_preserves ih step

theorem first_source_ordinal
    (step : Step assigned scope bootstrap capacity (postBootstrap sourceBudget ownerBudget) (.source request) (some next)) :
    next.ordinal = 2 := by
  cases step <;> simp_all [postBootstrap,stateAt,CohortPhase.initial,liveAt,modeAt,sourceResult]

theorem joint_launch_ordinal
    (launched : SourceInput.launch assigned bootstrap program = some seed)
    (ran : SourceFundingInput.execute assigned scope bootstrap (PublicationCapacityDriver.initial sourceBudget)
      (vector,.launch program) = (next,.accepted)) :
    (JointDriver.fromLaunch (capacity:=capacity) (budget:=budget) launched ran).actual.ordinal = 2 := rfl

-- Translating just the live counter leaves the pending occurrence stale.
-- It must be rejected even when the command itself still exactly matches.
theorem stale_pending_rejected (pending : PaidHeadPhase.Pending p) :
    PaidHeadPhase.permits pending (pending.sourceOrdinal+1) (PaidHeadPhase.command (a:=a) pending) = false := by
  apply PaidHeadPhase.moved_refused
  omega

#print axioms runs_at
#print axioms runs_invariant
#print axioms first_source_ordinal
#print axioms joint_launch_ordinal
#print axioms stale_pending_rejected

#print axioms funding_at
#print axioms invariant_at
#print axioms step_at
end MirroreaProofFirst.CohortPhysicalOrdinal
