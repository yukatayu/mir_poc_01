import MirroreaProofFirstOwnerWorkInterval

namespace MirroreaProofFirst.OwnerPublicationBudget

-- Two further owner commands are needed after a produced result: quiescent
-- freeze and actual current-image installation. This is not funded by the
-- preceding three-credit work interval. The image premise is the actual
-- checker for the independent assigned owner, not a caller's success flag.
theorem enabled_publication {s : OwnerEndpoint.State p a}
    (idle : s.owner.active = none)
    (current : s.owner.core.revision = s.fence)
    (newer : s.owner.core.revision < revision)
    (valid : OwnerReservationWorker.imageCheck assigned image = true)
    (funded : 2 ≤ credits) :
    ∃ (frozen installed : OwnerEndpointBudget.State p a) (last : OwnerEndpoint.State p a),
      OwnerEndpointBudget.transition assigned scopeId capacity ⟨some s,credits⟩ (.freeze revision) = (frozen,.inl 12) ∧
      OwnerEndpointBudget.transition assigned scopeId capacity frozen (.owner (.install revision image)) = (installed,.inl 7) ∧
      installed.owner = some last ∧ installed.remaining = credits-2 ∧
      last.fence = revision ∧ last.owner.core.revision = revision ∧ last.owner.core.image = image ∧
      last.owner.active = none ∧ last.owner.reserved = s.owner.reserved ∧
      last.owner.core.records = s.owner.core.records ∧ last.owner.core.scopeId = s.owner.core.scopeId := by
  let frozenOwner : OwnerEndpoint.State p a := {s with fence := revision}
  let installedOwner : OwnerEndpoint.State p a :=
    ⟨{s.owner with core := OwnerOccurrence.install s.owner.core revision image},revision⟩
  have fenceBound : s.fence ≤ revision := by omega
  have oldDifferent : revision ≠ s.owner.core.revision := by omega
  have freeze : OwnerEndpointProfile.transition assigned scopeId capacity (some s) (.freeze revision) = (some frozenOwner,.inl 12) := by
    simp [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,OwnerEndpoint.transition,idle,
      Nat.max_eq_right fenceBound,frozenOwner]
  have install : OwnerEndpointProfile.transition assigned scopeId capacity (some frozenOwner) (.owner (.install revision image)) = (some installedOwner,.inl 7) := by
    simp [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,OwnerEndpoint.transition,
      OwnerEndpoint.confirms,OwnerEndpoint.permitted,oldDifferent,frozenOwner,
      OwnerReservationWorker.transition,valid,newer,OwnerReservation.install,idle,installedOwner]
  have freezeCost : OwnerEndpointBudget.cost (some s) (.freeze revision) = 1 := by
    simp [OwnerEndpointBudget.cost,OwnerEndpointBudget.needsFinish,OwnerEndpointBudget.held,idle]
  have installCost : OwnerEndpointBudget.cost (some frozenOwner) (.owner (.install revision image)) = 1 := by
    simp [OwnerEndpointBudget.cost,OwnerEndpointBudget.needsFinish,OwnerEndpointBudget.held,frozenOwner,idle]
  refine ⟨⟨some frozenOwner,credits-1⟩,⟨some installedOwner,credits-2⟩,installedOwner,?_,?_,rfl,rfl,rfl,rfl,rfl,?_,rfl,rfl,rfl⟩
  · rw [OwnerEndpointBudget.funded_exact (show OwnerEndpointBudget.cost (some s) (.freeze revision) ≤ credits by rw [freezeCost]; omega)]
    simp [freeze]
  · rw [OwnerEndpointBudget.funded_exact (show OwnerEndpointBudget.cost (some frozenOwner) (.owner (.install revision image)) ≤ credits-1 by rw [installCost]; omega)]
    simp [install,Nat.sub_sub]
  · exact idle

#print axioms enabled_publication
end MirroreaProofFirst.OwnerPublicationBudget
