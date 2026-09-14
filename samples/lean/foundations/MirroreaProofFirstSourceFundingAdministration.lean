import MirroreaProofFirstSourceFundingWork
import MirroreaProofFirstOwnerPublicationBudget

namespace MirroreaProofFirst.SourceFundingAdministration
open PublicationOwnerBudget

-- A single actual administrative debit. The supervisor must supply the same
-- physical owner vector and exclude interleavings until source notification.
def paidVector (before : Vector (Fin 513) p) (target : Fin p) : Vector (Fin 513) p :=
  Vector.ofFn fun i =>
    if i = target then ⟨(before[i.val]).val-1,by have := (before[i.val]).isLt; omega⟩
    else before[i.val]

theorem paid_vector_exact (before : Vector (Fin 513) p) (target : Fin p)
    (kind : Bool) (revision : Nat) :
    ∀ owner, SourceFundingInput.credits
      ((paidVector before target,.step (if kind then .freeze target revision else .install target revision)) : SourceFundingInput.Request p a) owner =
      (before[owner.val]).val-commandCost (if kind then .freeze target revision else .install target revision : PublicationInput.Command p a) owner := by
  intro owner
  cases kind <;> simp [SourceFundingInput.credits,paidVector,commandCost]
  all_goals split <;> simp_all

-- Repeating the current freeze, including an earlier permitted off-head
-- freeze, has the same successful endpoint meaning. A higher unseen fence is
-- deliberately excluded; a source head query alone cannot establish this.
theorem enabled_freeze {owner : OwnerEndpoint.State p a}
    (idle : owner.owner.active = none) (bounded : owner.fence ≤ revision)
    (funded : 1 ≤ credits) :
    OwnerEndpointBudget.transition assigned scopeId capacity ⟨some owner,credits⟩ (.freeze revision) =
      (⟨some {owner with fence := revision},credits-1⟩,.inl 12) := by
  simp [OwnerEndpointBudget.transition,OwnerEndpointBudget.cost,OwnerEndpointBudget.needsFinish,
    OwnerEndpointBudget.held,idle,funded,OwnerEndpointProfile.transition,OwnerEndpointProfile.check,
    OwnerEndpoint.transition,Nat.max_eq_right bounded]

-- The complete current image may either be installed now or already installed
-- exactly. The latter uses the actual byte-injective confirmation rule, not a
-- fresh install event or a caller-created acknowledgement.
theorem enabled_install {owner : OwnerEndpoint.State p a}
    (idle : owner.owner.active = none) (fence : owner.fence = revision)
    (ready : owner.owner.core.revision < revision ∨
      (owner.owner.core.revision = revision ∧ owner.owner.core.image = image))
    (valid : OwnerReservationWorker.imageCheck assigned image = true)
    (funded : 1 ≤ credits) :
    ∃ last : OwnerEndpoint.State p a,
      OwnerEndpointBudget.transition assigned scopeId capacity ⟨some owner,credits⟩
        (.owner (.install revision image)) = (⟨some last,credits-1⟩,.inl 7) ∧
      last.fence = revision ∧ last.owner.core.revision = revision ∧
      last.owner.core.image = image ∧ last.owner.active = none ∧
      last.owner.reserved = owner.owner.reserved ∧
      last.owner.core.records = owner.owner.core.records := by
  have cost : OwnerEndpointBudget.cost (some owner) (.owner (.install revision image)) = 1 := by
    simp [OwnerEndpointBudget.cost,OwnerEndpointBudget.needsFinish,OwnerEndpointBudget.held,idle]
  have enough : OwnerEndpointBudget.cost (some owner) (.owner (.install revision image)) ≤ credits := by
    rw [cost]; exact funded
  rcases ready with newer | ⟨same,imageAt⟩
  · let last : OwnerEndpoint.State p a :=
      ⟨{owner.owner with core := OwnerOccurrence.install owner.owner.core revision image},revision⟩
    have different : revision ≠ owner.owner.core.revision := by omega
    have step : OwnerEndpointProfile.transition assigned scopeId capacity (some owner)
        (.owner (.install revision image)) = (some last,.inl 7) := by
      simp [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,OwnerEndpoint.transition,
        OwnerEndpoint.confirms,OwnerEndpoint.permitted,different,fence,
        OwnerReservationWorker.transition,OwnerReservation.install,valid,newer,idle,last]
    refine ⟨last,?_,rfl,rfl,rfl,?_,rfl,rfl⟩
    · rw [OwnerEndpointBudget.funded_exact enough]; simp [step]
    · exact idle
  · refine ⟨owner,?_,fence,same,imageAt,idle,rfl,rfl⟩
    have confirmed := OwnerEndpoint.exact_installation_reconfirms (assigned:=assigned) (scopeId:=scopeId)
      (capacity:=capacity) same.symm fence.symm imageAt.symm
    rw [OwnerEndpointBudget.funded_exact enough]
    simpa [OwnerEndpointProfile.transition,OwnerEndpointProfile.check] using
      congrArg (fun result => ((⟨result.1,credits-1⟩ : OwnerEndpointBudget.State p a),result.2)) confirmed

-- Construct the real model freeze and its accepted source notification from
-- the old certified suffix and owner conditions. No successful reply/payment
-- is assumed. Actual native state/namespace correspondence is still separate.
theorem freeze_then_notify {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId capacity : Nat} {seed : SourceInput.Bootstrap a}
    {old : PublicationCapacityDriver.State p a} {source : PublicationInput.State p a}
    {rest : List (PublicationInput.Command p a)} {target : Fin p} {revision : Nat}
    {owner : OwnerEndpoint.State p a} {before : Vector (Fin 513) p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source) (pending : old.suffix = .freeze target revision :: rest)
    (covered : Covered old.suffix (fun i => (before[i.val]).val))
    (idle : owner.owner.active = none) (bounded : owner.fence ≤ revision) :
    ∃ next : PublicationCapacityDriver.State p a,
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId capacity
        ⟨some owner,(before[target.val]).val⟩ (.freeze revision) =
        (⟨some {owner with fence := revision},((paidVector before target)[target.val]).val⟩,.inl 12) ∧
      SourceFundingInput.execute assigned scopeId seed old (paidVector before target,.step (.freeze target revision)) =
        (next,.accepted) ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId seed next ∧
      Covered next.suffix (fun i => ((paidVector before target)[i.val]).val) := by
  have funded : 1 ≤ (before[target.val]).val := by
    have enough := covered target
    rw [pending,demand_cons] at enough
    simp only [commandCost,ite_true] at enough
    omega
  obtain ⟨next,accepted,tail,preserved,retained⟩ := SourceFundingPreservation.paid_head_admitted
    valid present pending covered (paid_vector_exact (a:=a) before target true revision)
  refine ⟨next,?_,accepted,tail,preserved,retained⟩
  simpa [paidVector] using enabled_freeze (assigned:=⟨assigned.realm,target⟩)
    (scopeId:=scopeId) (capacity:=capacity) idle bounded funded

-- Installation consumes the certified source's actual complete current image.
-- Its validity is derived from the old source path, not supplied as a successful
-- install premise. Owner phase correspondence remains an explicit initial fact.
theorem install_then_notify {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId capacity : Nat} {seed : SourceInput.Bootstrap a}
    {old : PublicationCapacityDriver.State p a} {source : PublicationInput.State p a}
    {rest : List (PublicationInput.Command p a)} {target : Fin p} {revision : Nat}
    {owner : OwnerEndpoint.State p a} {before : Vector (Fin 513) p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source) (pending : old.suffix = .install target revision :: rest)
    (covered : Covered old.suffix (fun i => (before[i.val]).val))
    (idle : owner.owner.active = none) (fence : owner.fence = revision)
    (ready : owner.owner.core.revision < revision ∨
      (owner.owner.core.revision = revision ∧
        owner.owner.core.image = CustodyPublication.image source.publication.current)) :
    ∃ (next : PublicationCapacityDriver.State p a) (last : OwnerEndpoint.State p a),
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId capacity
        ⟨some owner,(before[target.val]).val⟩
        (.owner (.install revision (CustodyPublication.image source.publication.current))) =
        (⟨some last,((paidVector before target)[target.val]).val⟩,.inl 7) ∧
      last.fence = revision ∧ last.owner.core.revision = revision ∧
      last.owner.core.image = CustodyPublication.image source.publication.current ∧
      last.owner.active = none ∧ last.owner.reserved = owner.owner.reserved ∧
      last.owner.core.records = owner.owner.core.records ∧
      SourceFundingInput.execute assigned scopeId seed old (paidVector before target,.step (.install target revision)) =
        (next,.accepted) ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId seed next ∧
      Covered next.suffix (fun i => ((paidVector before target)[i.val]).val) := by
  obtain ⟨_,final,planned,_⟩ := valid.2 source present
  have imageFits := PublicationCapacity.assigned_endpoint_fits
    (PublicationCapacity.boundSequence_first planned) target
  have funded : 1 ≤ (before[target.val]).val := by
    have enough := covered target
    rw [pending,demand_cons] at enough
    simp only [commandCost,ite_true] at enough
    omega
  obtain ⟨last,installed,fenceAt,revisionAt,imageAt,stillIdle,keys,records⟩ :=
    enabled_install (scopeId:=scopeId) (capacity:=capacity) idle fence ready imageFits funded
  obtain ⟨next,accepted,tail,preserved,retained⟩ := SourceFundingPreservation.paid_head_admitted
    valid present pending covered (paid_vector_exact (a:=a) before target false revision)
  refine ⟨next,last,?_,fenceAt,revisionAt,imageAt,stillIdle,keys,records,accepted,tail,preserved,retained⟩
  simpa [paidVector] using installed

#print axioms paid_vector_exact
#print axioms enabled_freeze
#print axioms enabled_install
#print axioms freeze_then_notify
#print axioms install_then_notify
end MirroreaProofFirst.SourceFundingAdministration
