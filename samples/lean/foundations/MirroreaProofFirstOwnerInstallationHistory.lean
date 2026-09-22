import MirroreaProofFirstOwnerRevisionHistory

namespace MirroreaProofFirst.OwnerInstallationHistory

-- This history contains observations of actual successful initialize/install
-- replies, including confirmations. It is evidence, not a consumable payment
-- credit or authority. Other owner operations cannot delete those old facts.
def observe (records : List Nat) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : List Nat :=
  match OwnerImageMonitor.observe none command reply with
  | none => records
  | some current => current.1 :: records

def Authentic (state : OwnerEndpointBudget.State p a) (records : List Nat) : Prop :=
  ∀ revision, revision ∈ records → ∃ owner,
    state.owner = some owner ∧ revision ≤ owner.owner.core.revision

theorem observation_overwrites
    (recorded : OwnerImageMonitor.observe none command reply = some current) :
    OwnerImageMonitor.observe known command reply = some current := by
  unfold OwnerImageMonitor.observe at recorded ⊢
  split at recorded <;> simp_all

theorem successful_observation
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply))
    (recorded : OwnerImageMonitor.observe none command reply = some current) :
    ∃ owner, next.owner = some owner ∧ owner.owner.core.revision = current.1 := by
  have exact := (OwnerImageMonitor.budget_exact ran).trans (observation_overwrites recorded)
  cases present : next.owner with
  | none => simp [OwnerImageMonitor.project,present] at exact
  | some owner =>
    refine ⟨owner,rfl,?_⟩
    have fields : (owner.owner.core.revision,owner.owner.core.image) = current := by
      simpa [OwnerImageMonitor.project,OwnerImageMonitor.view,present] using exact
    exact congrArg Prod.fst fields

theorem old_records_preserved
    (old : Authentic state records)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    Authentic next records := by
  intro revision member
  obtain ⟨owner,present,bound⟩ := old revision member
  obtain ⟨after,retained,monotone⟩ := OwnerRevisionHistory.budget_mono present ran
  exact ⟨after,retained,Nat.le_trans bound monotone⟩

theorem observe_preserves
    (old : Authentic state records)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    Authentic next (observe records command reply) := by
  have retained := old_records_preserved old ran
  cases recorded : OwnerImageMonitor.observe none command reply with
  | none => simpa [observe,recorded] using retained
  | some current =>
    intro revision member
    have either : revision = current.1 ∨ revision ∈ records := by
      simpa [observe,recorded] using member
    rcases either with equal | prior
    · obtain ⟨owner,present,actual⟩ := successful_observation ran recorded
      exact ⟨owner,present,Nat.le_of_eq (equal.trans actual.symm)⟩
    · exact retained revision prior

inductive Runs (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) :
    OwnerEndpointBudget.State p a → List Nat → Prop where
  | fresh : Runs assigned scopeId capacity budget (OwnerEndpointBudget.initial budget) []
  | step : Runs assigned scopeId capacity budget state records →
      OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      Runs assigned scopeId capacity budget next (observe records command reply)

theorem reached_authentic (path : Runs assigned scopeId capacity budget state records) :
    Authentic state records := by
  induction path with
  | fresh => intro revision impossible; cases impossible
  | step prior ran ih => exact observe_preserves ih ran

#print axioms observation_overwrites
#print axioms successful_observation
#print axioms old_records_preserved
#print axioms observe_preserves
#print axioms reached_authentic
end MirroreaProofFirst.OwnerInstallationHistory
