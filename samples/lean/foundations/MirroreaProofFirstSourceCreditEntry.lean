import MirroreaProofFirstOwnerCreditCustody

namespace MirroreaProofFirst.SourceCreditEntry
open OwnerCreditCustody

-- Nonproduction reference composition. Source initiation and writer custody
-- share one serial coordinator entry. Neither ticket nor a successful-enter
-- notification is supplied by its caller. This definition is not a claim of
-- atomic IPC: a failed/unknown IO outcome must retire the physical coordinator.
def enter (scope : Nat) (source : PublicationInput.State p a) (writer : Local)
    (endpoint : Fin p) : Option (PublicationInput.State p a × Local) := do
  let waiting ← source.publication.current.privateState.session.state.source.waiting
  let leased ← claim writer waiting.entry.ticket
  let next ← PublicationInput.execute scope source (.enter endpoint)
  return (next,{leased with entered := true})

-- Independent declarative admission includes the original pending ticket,
-- available local writer/credits and the source's own semantic step. There is
-- no assumption that this new composed checker already accepted.
def Admitted (scope : Nat) (source next : PublicationInput.State p a)
    (writer : Local) (endpoint : Fin p) : Prop :=
  writer.lease = none ∧ 2 ≤ writer.credits ∧
  PublicationInput.execute scope source (.enter endpoint) = some next

theorem enter_exact : enter scope source writer endpoint = some (next,active) ↔
    Admitted scope source next writer endpoint ∧
    ∃ saved, source.publication.current.privateState.session.state.source.waiting = some saved ∧
      active = {writer with lease := some saved.entry.ticket,entered := true} := by
  constructor
  · intro ran
    cases pending : source.publication.current.privateState.session.state.source.waiting with
    | none => simp [enter,pending] at ran
    | some saved =>
      cases claimed : claim writer saved.entry.ticket with
      | none => simp [enter,pending,claimed] at ran
      | some leased =>
        obtain ⟨idle,funded,rfl⟩ := claim_exact.mp claimed
        cases executed : PublicationInput.execute scope source (.enter endpoint) with
        | none => simp [enter,pending,claimed,executed] at ran
        | some actual =>
          have eqs : actual = next ∧ {writer with lease := some saved.entry.ticket,entered := true} = active := by
            simpa [enter,pending,claimed,executed] using ran
          obtain ⟨rfl,rfl⟩ := eqs
          exact ⟨⟨idle,funded,executed⟩,saved,rfl,rfl⟩
  · rintro ⟨⟨idle,funded,executed⟩,saved,pending,rfl⟩
    have claimed : claim writer saved.entry.ticket = some {writer with lease := some saved.entry.ticket,entered := false} :=
      claim_exact.mpr ⟨idle,funded,rfl⟩
    simp [enter,pending,claimed,executed]

theorem admitted_progress (ready : Admitted scope source next writer endpoint) :
    ∃ active, enter scope source writer endpoint = some (next,active) := by
  obtain ⟨_,saved,pending,_,_⟩ := PublicationInput.dispatch_created ready.2.2
  exact ⟨_,enter_exact.mpr ⟨ready,saved,pending,rfl⟩⟩

-- The matching lease is derived from the actual accepted source command.
-- Possessing an unrelated ticket cannot turn into a source-enter notification.
theorem accepted_correlated (ran : enter scope source writer endpoint = some (next,active)) :
    ∃ dispatch, next.dispatch = some dispatch ∧ dispatch.endpoint = endpoint ∧
      active.lease = some dispatch.ticket ∧ active.entered = true ∧
      active.credits = writer.credits ∧ 2 ≤ active.credits := by
  obtain ⟨ready,saved,pending,rfl⟩ := enter_exact.mp ran
  obtain ⟨_,original,was,place,dispatch⟩ := PublicationInput.dispatch_created ready.2.2
  have same : original = saved := Option.some.inj (was.symm.trans pending)
  subst original
  exact ⟨_,dispatch,rfl,rfl,rfl,rfl,ready.2.1⟩

theorem accepted_no_cancel (ran : enter scope source writer endpoint = some (next,active)) :
    cancel active = none := by
  obtain ⟨_,_,_,_,entered,_,_⟩ := accepted_correlated ran
  exact entered_no_cancel entered

theorem accepted_preserves_local (ran : enter scope source writer endpoint = some (next,active)) :
    LocalInvariant active := by
  obtain ⟨ready,saved,_,rfl⟩ := enter_exact.mp ran
  simp [LocalInvariant,ready.2.1]

theorem accepted_reserve_funded {owner : OwnerEndpointBudget.State p a}
    (ran : enter scope source writer endpoint = some (next,active))
    (binding : writer.credits = owner.remaining) :
    ∃ dispatch, next.dispatch = some dispatch ∧
      OwnerEndpointBudget.cost owner.owner (.owner (.reserve dispatch.ticket)) ≤ owner.remaining := by
  obtain ⟨dispatch,present,_,_,_,credits,funded⟩ := accepted_correlated ran
  refine ⟨dispatch,present,?_⟩
  change 2 ≤ owner.remaining
  omega

#print axioms enter_exact
#print axioms admitted_progress
#print axioms accepted_correlated
#print axioms accepted_no_cancel
#print axioms accepted_preserves_local
#print axioms accepted_reserve_funded
end MirroreaProofFirst.SourceCreditEntry
