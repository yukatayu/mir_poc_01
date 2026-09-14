import MirroreaProofFirstSourceCreditEntry

namespace MirroreaProofFirst.OwnerWorkInterval

-- Private generated interval, including the retained active-freeze diagnostic.
-- Its physical consumer must exclude all competing owner/source commands from
-- submission through terminal notification; a per-call writer lock is weaker.
-- Unknown IO outcomes retire that consumer; they do not prove non-admission.
theorem enabled_interval {s : OwnerEndpoint.State p a}
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) s.owner)
    (small : s.owner.core.capacity ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits s.owner.core.scopeId s.owner.core.revision ticket)
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
      OwnerResponseProfile.WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
  obtain ⟨owner,reserve⟩ := OwnerReservation.admissible_reservation_progress scope admitted idle fresh room
  obtain ⟨later,record,compute⟩ := OwnerReservation.rooted_reserved_computes path reserve
  have before : OwnerEndpointProfile.ActiveFits s.owner := ⟨small,by simp [idle]⟩
  have after := OwnerEndpointProfile.reserve_preserves before profile reserve
  have core := OwnerReservation.reserve_core reserve
  have active : owner.active = some ticket := by rw [(OwnerReservation.reserve_parts reserve).2.2.2.2.2]
  let held : OwnerEndpoint.State p a := ⟨owner,s.fence⟩
  let done : OwnerEndpoint.State p a := ⟨later,s.fence⟩
  have first : OwnerEndpointProfile.transition assigned scopeId capacity (some s) (.owner (.reserve ticket)) = (some held,.inl 6) := by
    rw [OwnerEndpointProfile.admitted_exact (show OwnerEndpointProfile.Accepts capacity (some s) (.owner (.reserve ticket)) from ⟨small,profile⟩)]
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
    (OwnerEndpointProfile.computed_readable after compute).1,(OwnerEndpointProfile.computed_readable after compute).2⟩
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

#print axioms enabled_interval

-- Direct generated-source consumer: its checked reply profile, retained actual
-- dispatch and exact stored owner context provide the diagnostic work interval.
-- Physical origin and exclusion are still implementation obligations.
theorem entered_interval {source next : PublicationInput.State p a}
    {writer active : OwnerCreditCustody.Local} {owner : OwnerEndpoint.State p a}
    (entry : SourceCreditEntry.enter scopeId source writer endpoint = some (next,active))
    (fit : PublicationCapacity.StateFits scopeId source)
    (dispatched : next.dispatch = some dispatch)
    (bound : RoutedOwner.ReplyContextMatches dispatch owner)
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) owner.owner)
    (small : owner.owner.core.capacity ≤ 64)
    (current : owner.owner.core.revision = owner.fence)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image dispatch.ticket)
    (idle : owner.owner.active = none) (fresh : OwnerReservation.hasKey owner.owner dispatch.ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity) (funded : 3 ≤ writer.credits) :
    ∃ (reserved probed computed : OwnerEndpointBudget.State p a) (record : OwnerOccurrence.Record p a),
      OwnerEndpointBudget.transition assigned dispatch.context.scopeId capacity ⟨some owner,writer.credits⟩ (.owner (.reserve dispatch.ticket)) = (reserved,.inl 6) ∧
      OwnerEndpointBudget.transition assigned dispatch.context.scopeId capacity reserved (.freeze revision) = (probed,.inl 2) ∧
      OwnerEndpointBudget.transition assigned dispatch.context.scopeId capacity probed (.owner .compute) = (computed,.inr (OwnerReceipt.project record)) ∧
      probed.owner = reserved.owner ∧ computed.remaining = writer.credits-3 ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
  have ran := (SourceCreditEntry.enter_exact.mp entry).1.2.2
  exact enabled_interval path small (RoutedOwner.generated_profile fit ran dispatched bound)
    current bound.1.symm admitted idle fresh room funded

#print axioms entered_interval
end MirroreaProofFirst.OwnerWorkInterval
