import MirroreaProofFirstOwnerEndpointBudget

namespace MirroreaProofFirst.OwnerFenceMonitor

-- This monitor consumes the retained request and its ACTUAL typed reply. It
-- does not trust a source-side acknowledgement or grant authority to use work.
def floor (state : Option (OwnerEndpoint.State p a)) : Nat :=
  (state.map OwnerEndpoint.State.fence).getD 0

def advance (known : Nat) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : Nat :=
  match command,reply with
  | .freeze revision,.inl 12 => max known revision
  | _,_ => known

theorem endpoint_exact
    (ran : OwnerEndpoint.transition assigned scopeId capacity state command = (next,reply)) :
    floor next = advance (floor state) command reply := by
  cases state with
  | none =>
    cases command with
    | freeze revision => cases ran; rfl
    | owner command =>
      cases command <;> simp only [OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
      case «initialize» image => split at ran <;> cases ran <;> rfl
      all_goals cases ran; rfl
  | some state =>
    cases command with
    | freeze revision =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran <;> cases ran <;> rfl
    | owner command =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran
      · cases ran; rfl
      · split at ran
        · cases worker : OwnerReservationWorker.transition assigned scopeId capacity (some state.owner) command with
          | mk owner response =>
            rw [worker] at ran
            rcases OwnerReservationWorker.transition_step worker with same | ⟨value,same,_⟩
            · rw [same] at ran; cases ran; rfl
            · rw [same] at ran; cases ran; rfl
        · cases ran; rfl

theorem profile_exact
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply)) :
    floor next = advance (floor state) command reply := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · exact endpoint_exact ran
  · cases ran
    cases command <;> rfl

theorem budget_exact
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    floor next.owner = advance (floor state.owner) command reply := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases native : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk owner response =>
      rw [native] at ran
      cases ran
      exact profile_exact native
  · cases ran
    cases command <;> rfl

-- The path records ALL actual budget transitions, including initialization,
-- extra administration, rejected input, abandon, reserve and compute. There is
-- no invariant premise on a transition and no omitted reset/restore constructor.
inductive Runs (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) :
    OwnerEndpointBudget.State p a → Nat → Prop where
  | fresh : Runs assigned scopeId capacity budget (OwnerEndpointBudget.initial budget) 0
  | step : Runs assigned scopeId capacity budget state known →
      OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      Runs assigned scopeId capacity budget next (advance known command reply)

theorem reached_exact (path : Runs assigned scopeId capacity budget state known) :
    floor state.owner = known := by
  induction path with
  | fresh => rfl
  | step prior ran ih => rw [budget_exact ran,ih]

-- Relate a retained set/list of successful freezes to the numeric monitor.
-- Max starts at0, the actual fresh endpoint floor, without a caller-supplied seed.
def maximum : List Nat → Nat
  | [] => 0
  | revision::rest => max (maximum rest) revision

theorem maximum_bounded : maximum records ≤ published ↔ ∀ r ∈ records, r ≤ published := by
  induction records with
  | nil => simp [maximum]
  | cons revision rest ih =>
    simp only [maximum,List.mem_cons,forall_eq_or_imp]
    constructor
    · intro bound
      have bounds : maximum rest ≤ published ∧ revision ≤ published := by omega
      exact ⟨bounds.2,ih.mp bounds.1⟩
    · rintro ⟨first,tail⟩
      have bound := ih.mpr tail
      omega

theorem observed_current_floor
    (path : Runs assigned scopeId capacity budget state (maximum records))
    (present : state.owner = some owner)
    (guard : ∀ r ∈ records, r ≤ published) : owner.fence ≤ published := by
  have exactFloor := reached_exact path
  simp only [floor,present,Option.map_some,Option.getD_some] at exactFloor
  rw [exactFloor]
  exact maximum_bounded.mpr guard

-- Useful consumer requires independent core/source correspondence. It does
-- not pretend to establish that separate all-entry invariant here.
theorem current_from_bounds
    (path : Runs assigned scopeId capacity budget state (maximum records))
    (present : state.owner = some owner)
    (guard : ∀ r ∈ records, r ≤ published)
    (core : owner.owner.core.revision = published)
    (lower : published ≤ owner.fence) : owner.owner.core.revision = owner.fence := by
  have upper := observed_current_floor path present guard
  omega

-- Literal successful replies determine the retained evidence; nothing is
-- appended on diagnostic/refusal or unobserved attempts.
def observe (records : List Nat) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : List Nat :=
  match command,reply with
  | .freeze revision,.inl 12 => revision::records
  | _,_ => records

theorem observed_maximum : maximum (observe records command reply) =
    advance (maximum records) command reply := by
  cases command with
  | owner command => rfl
  | freeze revision =>
    cases reply with
    | inr envelope => rfl
    | inl code =>
      by_cases same : code = 12
      · subst code; rfl
      · simp [observe,advance,same]

inductive ObservedRuns (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) :
    OwnerEndpointBudget.State p a → List Nat → Prop where
  | fresh : ObservedRuns assigned scopeId capacity budget (OwnerEndpointBudget.initial budget) []
  | step : ObservedRuns assigned scopeId capacity budget state records →
      OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      ObservedRuns assigned scopeId capacity budget next (observe records command reply)

theorem observed_path (path : ObservedRuns assigned scopeId capacity budget state records) :
    Runs assigned scopeId capacity budget state (maximum records) := by
  induction path with
  | fresh => exact .fresh
  | step prior ran ih => rw [observed_maximum]; exact .step ih ran

def useCheck (records : List Nat) (published : Nat) : Bool :=
  records.all (fun revision => decide (revision ≤ published))

theorem use_check_exact : useCheck records published = true ↔ ∀ r ∈ records, r ≤ published := by
  simp [useCheck,List.all_eq_true]

theorem guard_bounds_actual
    (path : ObservedRuns assigned scopeId capacity budget state records)
    (present : state.owner = some owner) (guard : useCheck records published = true) :
    owner.fence ≤ published :=
  observed_current_floor (observed_path path) present (use_check_exact.mp guard)

#print axioms observed_maximum
#print axioms observed_path
#print axioms use_check_exact
#print axioms guard_bounds_actual
#print axioms endpoint_exact
#print axioms profile_exact
#print axioms budget_exact
#print axioms reached_exact
#print axioms maximum_bounded
#print axioms observed_current_floor
#print axioms current_from_bounds
end MirroreaProofFirst.OwnerFenceMonitor
