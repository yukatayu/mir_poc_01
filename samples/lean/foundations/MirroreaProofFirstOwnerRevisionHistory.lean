import MirroreaProofFirstSourceOwnerImage

namespace MirroreaProofFirst.OwnerRevisionHistory

-- Revision monotonicity belongs to the actual guarded worker. The broader
-- OwnerReservation.Step allows arbitrary install revisions and is insufficient.
theorem worker_mono
    {p a : Nat} {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat}
    {state : OwnerReservation.State p a} {command : OwnerReservationWorker.Command p a}
    {next : Option (OwnerReservation.State p a)} {reply : Sum Nat OwnerReceipt.Envelope}
    (ran : OwnerReservationWorker.transition assigned scopeId capacity (some state) command = (next,reply)) :
    ∃ after, next = some after ∧ state.core.revision ≤ after.core.revision := by
  cases command with
  | «initialize» image => cases ran; exact ⟨state,rfl,Nat.le_refl _⟩
  | reserve ticket =>
    cases reserved : OwnerReservation.reserve state scopeId ticket with
    | mk owner status =>
      simp only [OwnerReservationWorker.transition,reserved] at ran
      cases ran
      exact ⟨owner,rfl,by rw [OwnerReservation.reserve_core reserved]; exact Nat.le_refl _⟩
  | compute =>
    cases computed : OwnerReservation.compute state with
    | none => simp only [OwnerReservationWorker.transition,computed] at ran; cases ran; exact ⟨state,rfl,Nat.le_refl _⟩
    | some pair =>
      obtain ⟨owner,status⟩ := pair
      have same := congrArg Prod.fst (OwnerImageMonitor.compute_view computed)
      simp only [OwnerReservationWorker.transition,computed] at ran
      cases ran
      exact ⟨owner,rfl,by simpa [OwnerImageMonitor.view] using Nat.le_of_eq same.symm⟩
  | install revision image =>
    simp only [OwnerReservationWorker.transition] at ran
    split at ran
    · rename_i checked
      have checks : OwnerReservationWorker.imageCheck assigned image = true ∧ state.core.revision < revision := by simpa using checked
      have newer := checks.2
      cases installed : OwnerReservation.install state revision image with
      | none => simp only [installed] at ran; cases ran; exact ⟨state,rfl,Nat.le_refl _⟩
      | some owner =>
        simp only [installed] at ran
        cases ran
        refine ⟨owner,rfl,?_⟩
        rw [(OwnerReservation.install_idle installed).2]
        exact Nat.le_of_lt newer
    · cases ran; exact ⟨state,rfl,Nat.le_refl _⟩
  | abandon => cases ran; exact ⟨_,rfl,Nat.le_refl _⟩

theorem endpoint_mono
    {p a : Nat} {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat}
    {state : OwnerEndpoint.State p a} {command : OwnerEndpoint.Command p a}
    {next : Option (OwnerEndpoint.State p a)} {reply : Sum Nat OwnerReceipt.Envelope}
    (ran : OwnerEndpoint.transition assigned scopeId capacity (some state) command = (next,reply)) :
    ∃ after, next = some after ∧ state.owner.core.revision ≤ after.owner.core.revision := by
  cases command with
  | freeze revision =>
    simp only [OwnerEndpoint.transition] at ran
    split at ran <;> cases ran <;> exact ⟨_,rfl,Nat.le_refl _⟩
  | owner command =>
    simp only [OwnerEndpoint.transition] at ran
    split at ran
    · cases ran; exact ⟨state,rfl,Nat.le_refl _⟩
    · split at ran
      · cases worker : OwnerReservationWorker.transition assigned scopeId capacity (some state.owner) command with
        | mk after response =>
          obtain ⟨owner,equal,monotone⟩ := worker_mono worker
          rw [worker,equal] at ran
          cases ran
          exact ⟨_,rfl,monotone⟩
      · cases ran; exact ⟨state,rfl,Nat.le_refl _⟩

theorem profile_mono
    {p a : Nat} {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat}
    {state : OwnerEndpoint.State p a} {command : OwnerEndpoint.Command p a}
    {next : Option (OwnerEndpoint.State p a)} {reply : Sum Nat OwnerReceipt.Envelope}
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity (some state) command = (next,reply)) :
    ∃ after, next = some after ∧ state.owner.core.revision ≤ after.owner.core.revision := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · exact endpoint_mono ran
  · cases ran; exact ⟨state,rfl,Nat.le_refl _⟩

theorem budget_mono
    {p a : Nat} {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat}
    {state next : OwnerEndpointBudget.State p a} {owner : OwnerEndpoint.State p a}
    {command : OwnerEndpoint.Command p a} {reply : Sum Nat OwnerReceipt.Envelope}
    (present : state.owner = some owner)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    ∃ after, next.owner = some after ∧ owner.owner.core.revision ≤ after.owner.core.revision := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases native : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk after response =>
      rw [present] at native
      obtain ⟨value,equal,monotone⟩ := profile_mono native
      rw [present,native] at ran
      cases ran
      exact ⟨value,equal,monotone⟩
  · cases ran; exact ⟨owner,present,Nat.le_refl _⟩

#print axioms worker_mono
#print axioms endpoint_mono
#print axioms profile_mono
#print axioms budget_mono
end MirroreaProofFirst.OwnerRevisionHistory
