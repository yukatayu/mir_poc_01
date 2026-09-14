import MirroreaProofFirstSourceFundingWork

namespace MirroreaProofFirst.SourceFundingCheckedWork
open PublicationOwnerBudget SourceFundingWork

-- Strengthened constructive consumer. The computation equation and checked
-- carrier readability share the same witness, including the actual diagnostic
-- debit and paid finish. No equality of independently chosen records is assumed.
-- Native state binding, custody and physical namespace remain separate gates.
theorem checked_interval {s : OwnerEndpoint.State p a}
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) s.owner)
    (small : s.owner.core.capacity ≤ 64)
    (profile : SourceFundingWork.CheckedCarrierResponseFits p s.owner.core.scopeId s.owner.core.revision ticket)
    (actual : Vector (Fin 513) p)
    (current : s.owner.core.revision = s.fence)
    (scope : scopeId = s.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted s.owner.core.assigned s.owner.core.image ticket)
    (idle : s.owner.active = none) (fresh : OwnerReservation.hasKey s.owner ticket = false)
    (room : s.owner.reserved.length < s.owner.core.capacity) (funded : 3 ≤ credits) :
    ∃ (reserved probed computed : OwnerEndpointBudget.State p a) (record : OwnerOccurrence.Record p a),
      OwnerEndpointBudget.transition assigned scopeId capacity ⟨some s,credits⟩ (.owner (.reserve ticket)) = (reserved,.inl 6) ∧
      OwnerEndpointBudget.transition assigned scopeId capacity reserved (.freeze revision) = (probed,.inl 2) ∧
      OwnerEndpointBudget.transition assigned scopeId capacity probed (.owner .compute) = (computed,.inr (OwnerReceipt.project record)) ∧
      probed.owner = reserved.owner ∧ computed.remaining = credits-3 ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (SourceFundingQuery.checkedInput p a) 256
        (.inr (.inr (actual,.step (.arrival (OwnerReceipt.project record))))) := by
  obtain ⟨owner,reserve⟩ := OwnerReservation.admissible_reservation_progress scope admitted idle fresh room
  obtain ⟨later,record,compute,checkedArrival⟩ :=
    SourceFundingWork.rooted_checked_computation_arrival path
      (by simpa [OwnerResponseProfile.rooted_capacity path] using small) profile reserve actual
  have before : OwnerEndpointProfile.ActiveFits s.owner := ⟨small,by simp [idle]⟩
  have after := OwnerEndpointProfile.reserve_preserves before profile.1 reserve
  have core := OwnerReservation.reserve_core reserve
  have active : owner.active = some ticket := by rw [(OwnerReservation.reserve_parts reserve).2.2.2.2.2]
  let held : OwnerEndpoint.State p a := ⟨owner,s.fence⟩
  let done : OwnerEndpoint.State p a := ⟨later,s.fence⟩
  have first : OwnerEndpointProfile.transition assigned scopeId capacity (some s) (.owner (.reserve ticket)) = (some held,.inl 6) := by
    rw [OwnerEndpointProfile.admitted_exact (show OwnerEndpointProfile.Accepts capacity (some s) (.owner (.reserve ticket)) from ⟨small,profile.1⟩)]
    simp [OwnerEndpoint.transition,OwnerEndpoint.confirms,OwnerEndpoint.permitted,current,
      OwnerReservationWorker.transition,reserve,OwnerReservationWorker.responseCode,held]
  have second : OwnerEndpointProfile.transition assigned scopeId capacity (some held) (.freeze revision) = (some held,.inl 2) := by
    simp [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,OwnerEndpoint.transition,held,active]
  have third : OwnerEndpointProfile.transition assigned scopeId capacity (some held) (.owner .compute) = (some done,.inr (OwnerReceipt.project record)) := by
    rw [OwnerEndpointProfile.admitted_exact (show OwnerEndpointProfile.Accepts capacity (some held) (.owner .compute) from after)]
    have same : owner.core.revision = s.fence := by rw [core,current]
    simp [OwnerEndpoint.transition,OwnerEndpoint.confirms,OwnerEndpoint.permitted,same,
      OwnerReservationWorker.transition,compute,held,done]
  have computeCost := OwnerEndpointBudget.produced_compute_cost third
  refine ⟨⟨some held,credits-1⟩,⟨some held,credits-2⟩,⟨some done,credits-3⟩,record,?_,?_,?_,rfl,rfl,
    (OwnerEndpointProfile.computed_readable after compute).1,checkedArrival⟩
  · rw [OwnerEndpointBudget.funded_exact (show OwnerEndpointBudget.cost (some s) (.owner (.reserve ticket)) ≤ credits by change 2 ≤ credits; omega)]
    simp [first]
  · have enough : OwnerEndpointBudget.cost (some held) (.freeze revision) ≤ credits-1 := by
      have bounds := OwnerEndpointBudget.cost_bounds (state:=some held) (command:=OwnerEndpoint.Command.freeze revision)
      omega
    rw [OwnerEndpointBudget.funded_exact enough]
    simp [second,Nat.sub_sub]
  · have enough : OwnerEndpointBudget.cost (some held) (.owner .compute) ≤ credits-2 := by rw [computeCost]; omega
    rw [OwnerEndpointBudget.funded_exact enough]
    simp [third,Nat.sub_sub]


theorem checked_work_then_finish {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId capacity : Nat} {seed : SourceInput.Bootstrap a}
    {old : PublicationCapacityDriver.State p a} {source : PublicationInput.State p a}
    {rest : List (PublicationInput.Command p a)} {target : Fin p}
    {owner : OwnerEndpoint.State p a} {dispatch : PublicationInput.Dispatch p}
    {initialAssigned : OwnerEvaluator.Assignment p} {initialScope initialCapacity : Nat}
    {initialImage : OwnerImage.Image p a}
    {before : Vector (Fin 513) p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source) (pending : old.suffix = .finish target :: rest)
    (covered : Covered old.suffix (fun i => (before[i.val]).val))
    (dispatched : source.dispatch = some dispatch) (targetAt : dispatch.endpoint = target)
    (bound : RoutedOwner.ReplyContextMatches dispatch owner)
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) owner.owner)
    (small : owner.owner.core.capacity ≤ 64)
    (current : owner.owner.core.revision = owner.fence)
    (scope : scopeId = owner.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image dispatch.ticket)
    (idle : owner.owner.active = none) (fresh : OwnerReservation.hasKey owner.owner dispatch.ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity)
    (carrier : p ≤ 64 ∧
      (OwnerPacketCodec.encode OwnerFullCodec.ticket dispatch.ticket).length+372+12*p+25 ≤ 65536 ∧
      max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode dispatch.ticket)+24)+7 ≤ 256) :
    ∃ (reserved probed computed : OwnerEndpointBudget.State p a)
      (record : OwnerOccurrence.Record p a) (next : PublicationCapacityDriver.State p a),
      OwnerEndpointBudget.transition owner.owner.core.assigned scopeId capacity ⟨some owner,(before[target.val]).val⟩
        (.owner (.reserve dispatch.ticket)) = (reserved,.inl 6) ∧
      OwnerEndpointBudget.transition owner.owner.core.assigned scopeId capacity reserved (.freeze revision) = (probed,.inl 2) ∧
      OwnerEndpointBudget.transition owner.owner.core.assigned scopeId capacity probed (.owner .compute) =
        (computed,.inr (OwnerReceipt.project record)) ∧
      probed.owner = reserved.owner ∧
      computed.remaining = ((paidVector before target)[target.val]).val ∧
      SourceFundingInput.execute assigned scopeId seed old (paidVector before target,.step (.finish target)) =
        (next,.accepted) ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId seed next ∧
      Covered next.suffix (fun i => ((paidVector before target)[i.val]).val) ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (SourceFundingQuery.checkedInput p a) 256
        (.inr (.inr (paidVector before target,.step (.arrival (OwnerReceipt.project record))))) := by
  obtain ⟨_,final,planned,_⟩ := valid.2 source present
  have profile : OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision dispatch.ticket := by
    have known := PublicationCapacity.state_dispatch_fits
      (PublicationCapacity.boundSequence_first planned).1 dispatch dispatched
    simpa [bound.1,bound.2] using known
  have funded : 3 ≤ (before[target.val]).val := by
    have enough := covered target
    rw [pending,demand_cons] at enough
    simp only [commandCost,ite_true] at enough
    omega
  obtain ⟨reserved,probed,computed,record,first,second,third,unchanged,spent,readable,arrival⟩ :=
    checked_interval (assigned:=owner.owner.core.assigned) (scopeId:=scopeId)
      (capacity:=capacity) (revision:=revision) path small ⟨profile,carrier⟩ (paidVector before target) current scope admitted idle fresh room funded
  obtain ⟨next,notified,tail,preserved,retained⟩ :=
    SourceFundingPreservation.paid_head_admitted valid present pending covered (paid_vector_exact (a:=a) before target)
  refine ⟨reserved,probed,computed,record,next,first,second,third,unchanged,?_,notified,tail,preserved,retained,readable,arrival⟩
  simpa [paidVector] using spent


#print axioms checked_interval
#print axioms checked_work_then_finish
end MirroreaProofFirst.SourceFundingCheckedWork
