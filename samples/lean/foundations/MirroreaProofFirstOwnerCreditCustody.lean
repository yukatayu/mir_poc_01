import MirroreaProofFirstRoutedOwner

namespace MirroreaProofFirst.OwnerCreditCustody

-- This monitor belongs to the sole writer of one fresh owner endpoint. It is
-- neither an authorization grant nor a remote query. Every complete reply must
-- pass the actual codec/transition boundary; IO failure retires the writer.
def budgetRefused : Sum Nat OwnerReceipt.Envelope → Bool
  | .inl code => decide (code = 16)
  | .inr _ => false

def observe (credits : Nat) (reply : Sum Nat OwnerReceipt.Envelope) : Nat :=
  if budgetRefused reply then credits else credits-1

theorem worker_not_budget
    (ran : OwnerReservationWorker.transition assigned scopeId capacity state command = (next,reply)) :
    budgetRefused reply = false := by
  cases state with
  | none =>
    cases command <;> simp only [OwnerReservationWorker.transition] at ran
    case «initialize» image => split at ran <;> cases ran <;> rfl
    all_goals cases ran; rfl
  | some s =>
    cases command with
    | «initialize» image => cases ran; rfl
    | reserve ticket =>
      cases reserved : OwnerReservation.reserve s scopeId ticket with
      | mk value response =>
        simp only [OwnerReservationWorker.transition,reserved] at ran
        cases response <;> cases ran <;> rfl
    | compute =>
      cases computed : OwnerReservation.compute s with
      | none => simp only [OwnerReservationWorker.transition,computed] at ran; cases ran; rfl
      | some pair =>
        obtain ⟨value,response⟩ := pair
        simp only [OwnerReservationWorker.transition,computed] at ran
        cases response <;> cases ran <;> rfl
    | install revision image =>
      simp only [OwnerReservationWorker.transition] at ran
      split at ran
      · cases installed : OwnerReservation.install s revision image with
        | none => rw [installed] at ran; cases ran; rfl
        | some value => rw [installed] at ran; cases ran; rfl
      · cases ran; rfl
    | abandon => cases ran; rfl

theorem endpoint_not_budget
    (ran : OwnerEndpoint.transition assigned scopeId capacity state command = (next,reply)) :
    budgetRefused reply = false := by
  cases state with
  | none =>
    cases command with
    | freeze revision => cases ran; rfl
    | owner command =>
      cases step : OwnerReservationWorker.transition assigned scopeId capacity none command with
      | mk value response =>
        simp only [OwnerEndpoint.transition,step] at ran
        cases ran; exact worker_not_budget step
  | some s =>
    cases command with
    | freeze revision => simp only [OwnerEndpoint.transition] at ran; split at ran <;> cases ran <;> rfl
    | owner command =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran
      · cases ran; rfl
      · split at ran
        · cases step : OwnerReservationWorker.transition assigned scopeId capacity (some s.owner) command with
          | mk value response => rw [step] at ran; cases ran; exact worker_not_budget step
        · cases ran; rfl

theorem profile_not_budget
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply)) :
    budgetRefused reply = false := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · exact endpoint_not_budget ran
  · cases ran; rfl

-- The monitor follows all commands, including initialize/freeze/install,
-- semantic refusals and the budget refusal which spends no credit.
theorem observe_exact
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    observe state.remaining reply = next.remaining := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases step : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk value response =>
      rw [step] at ran
      cases ran
      simp [observe,profile_not_budget step]
  · cases ran; rfl

inductive Transcript (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat) :
    OwnerEndpointBudget.State p a → List (Sum Nat OwnerReceipt.Envelope) → OwnerEndpointBudget.State p a → Prop where
  | nil : Transcript assigned scopeId capacity state [] state
  | cons : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      Transcript assigned scopeId capacity next replies final →
      Transcript assigned scopeId capacity state (reply::replies) final

theorem transcript_exact (path : Transcript assigned scopeId capacity state replies final) :
    replies.foldl observe state.remaining = final.remaining := by
  induction path with
  | nil => rfl
  | cons step _ ih => simpa [List.foldl,observe_exact step] using ih

-- Pure local scheduling custody for one endpoint's sole writer. The lease
-- reserves access to the writer, not semantic owner work. The coordinator must
-- release it if source enter refuses/fails; no owner reservation is synthesized.
structure Local where
  credits : Nat
  lease : Option InvocationBoundary.Ticket
  entered : Bool := false

def claim (state : Local) (ticket : InvocationBoundary.Ticket) : Option Local :=
  if state.lease.isNone && decide (2 ≤ state.credits) then
    some {state with lease := some ticket,entered := false}
  else none

def Claimed (state next : Local) (ticket : InvocationBoundary.Ticket) : Prop :=
  state.lease = none ∧ 2 ≤ state.credits ∧ next = {state with lease := some ticket,entered := false}

theorem claim_exact : claim state ticket = some next ↔ Claimed state next ticket := by
  simp only [claim,Claimed]
  split
  · rename_i allowed
    have parts : state.lease = none ∧ 2 ≤ state.credits := by simpa using allowed
    simp [parts,eq_comm]
  · rename_i denied
    have no : ¬ (state.lease = none ∧ 2 ≤ state.credits) := by simpa using denied
    constructor
    · intro impossible; cases impossible
    · rintro ⟨idle,credits,_⟩
      exact False.elim (no ⟨idle,credits⟩)

-- While leased, ordinary mutators are deferred locally. Only the exact reserve
-- for that lease can be sent, after the actual source enter. This is a required
-- serialized writer protocol, not a claim that arbitrary raw pipe users obey it.
def maySend (state : Local) (command : OwnerEndpoint.Command p a) : Bool :=
  match state.lease with
  | none => true
  | some ticket => state.entered && (match command with
      | .owner (.reserve requested) => decide (requested = ticket)
      | _ => false)

def completed (state : Local) (reply : Sum Nat OwnerReceipt.Envelope) : Local :=
  ⟨observe state.credits reply,none,false⟩

-- Record the actual successful source-enter boundary. Physical callers must
-- bind this local notification to the real matching source reply; its truth
-- does not follow merely from possessing a ticket or this function.
def entered (state : Local) (ticket : InvocationBoundary.Ticket) : Option Local :=
  if state.lease == some ticket && !state.entered then some {state with entered := true} else none

def cancel (state : Local) : Option Local :=
  if state.entered then none else some {state with lease := none}

theorem entered_exact : entered state ticket = some next ↔
    state.lease = some ticket ∧ state.entered = false ∧ next = {state with entered := true} := by
  unfold entered
  split
  · rename_i guard
    have parts : state.lease = some ticket ∧ state.entered = false := by simpa using guard
    simp [parts,eq_comm]
  · rename_i guard
    have no : ¬(state.lease = some ticket ∧ state.entered = false) := by simpa using guard
    constructor
    · intro impossible; cases impossible
    · rintro ⟨held,staged,_⟩; exact False.elim (no ⟨held,staged⟩)

theorem entered_no_cancel (active : state.entered = true) : cancel state = none := by simp [cancel,active]
theorem staged_no_send (held : state.lease = some ticket) (staged : state.entered = false) :
    maySend (p:=p) (a:=a) state command = false := by simp [maySend,held,staged]

theorem claimed_no_second (held : state.lease = some ticket) : claim state other = none := by
  simp [claim,held]

theorem leased_send_exact (held : state.lease = some ticket) (active : state.entered = true) :
    maySend (p:=p) (a:=a) state command = true ↔ command = .owner (.reserve ticket) := by
  cases command with
  | freeze revision => simp [maySend,held,active]
  | owner command => cases command <;> simp [maySend,held,active]

theorem claimed_funded {owner : OwnerEndpointBudget.State p a} (made : claim before ticket = some lease)
    (binding : before.credits = owner.remaining) :
    OwnerEndpointBudget.cost owner.owner (.owner (.reserve ticket)) ≤ owner.remaining := by
  have credits := (claim_exact.mp made).2.1
  change 2 ≤ owner.remaining
  omega

theorem completed_binding {proxy : Local} {owner : OwnerEndpointBudget.State p a}
    (binding : proxy.credits = owner.remaining)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity owner command = (next,reply)) :
    (completed proxy reply).credits = next.remaining := by
  simp only [completed,binding]
  exact observe_exact ran

theorem cancel_frames (ran : cancel proxy = some next) : next.credits = proxy.credits := by
  unfold cancel at ran
  split at ran
  · cases ran
  · cases ran; rfl

def LocalInvariant (state : Local) : Prop :=
  (state.lease.isSome = true → 2 ≤ state.credits) ∧
  (state.entered = true → state.lease.isSome = true)

theorem empty_valid : LocalInvariant ⟨credits,none,false⟩ := by simp [LocalInvariant]
theorem claim_preserves (ran : claim state ticket = some next) : LocalInvariant next := by
  obtain ⟨_,funded,rfl⟩ := claim_exact.mp ran
  simp [LocalInvariant,funded]
theorem entered_preserves (valid : LocalInvariant state) (ran : entered state ticket = some next) :
    LocalInvariant next := by
  obtain ⟨held,_,rfl⟩ := entered_exact.mp ran
  simpa [LocalInvariant,held] using valid.1 (by simp [held])
theorem completed_valid : LocalInvariant (completed state reply) := by simp [LocalInvariant,completed]
theorem cancel_preserves (ran : cancel state = some next) : LocalInvariant next := by
  unfold cancel at ran
  split at ran
  · cases ran
  · rename_i inactive
    cases ran
    simp [LocalInvariant,inactive]

#print axioms entered_exact
#print axioms entered_no_cancel
#print axioms staged_no_send
#print axioms empty_valid
#print axioms claim_preserves
#print axioms entered_preserves
#print axioms completed_valid
#print axioms cancel_preserves

#print axioms worker_not_budget
#print axioms endpoint_not_budget
#print axioms profile_not_budget
#print axioms observe_exact
#print axioms transcript_exact
#print axioms claim_exact
#print axioms claimed_no_second
#print axioms leased_send_exact
#print axioms claimed_funded
#print axioms completed_binding
#print axioms cancel_frames
end MirroreaProofFirst.OwnerCreditCustody
