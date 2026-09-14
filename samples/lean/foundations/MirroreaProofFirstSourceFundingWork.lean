import MirroreaProofFirstSourceFundingPreservation
import MirroreaProofFirstOwnerWorkInterval

namespace MirroreaProofFirst.SourceFundingWork
open PublicationOwnerBudget

-- The carrier for the exact final budget of a single owner's actual work
-- interval. No other owner transition may interleave with this frame update.
-- Physical exclusion and correspondence with retained native monitors remain
-- obligations of the sole private-pipe supervisor.
def paidVector (before : Vector (Fin 513) p) (target : Fin p) : Vector (Fin 513) p :=
  Vector.ofFn fun i =>
    if i = target then ⟨(before[i.val]).val-3,by have := (before[i.val]).isLt; omega⟩
    else before[i.val]

theorem paid_vector_exact {p a : Nat} (before : Vector (Fin 513) p) (target : Fin p) :
    ∀ owner, SourceFundingInput.credits ((paidVector before target,PublicationInput.Input.step (.finish target)) : SourceFundingInput.Request p a) owner =
      (before[owner.val]).val-commandCost (.finish target : PublicationInput.Command p a) owner := by
  intro owner
  simp [SourceFundingInput.credits,paidVector,commandCost]
  split <;> simp_all

-- This is a model-level enabledness and preservation theorem. Its premises
-- describe the initial owner and independently certified source state, rather
-- than assuming that a reservation, production or source finish succeeded.
-- In particular three unrelated debits cannot serve as its work witness.
theorem work_then_finish {p a : Nat} {assigned : SourceInput.Assignment p a}
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
    (room : owner.owner.reserved.length < owner.owner.core.capacity) :
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
      OwnerResponseProfile.WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
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
    OwnerWorkInterval.enabled_interval (assigned:=owner.owner.core.assigned) (scopeId:=scopeId)
      (capacity:=capacity) (revision:=revision) path small profile current scope admitted idle fresh room funded
  obtain ⟨next,notified,tail,preserved,retained⟩ :=
    SourceFundingPreservation.paid_head_admitted valid present pending covered (paid_vector_exact (a:=a) before target)
  refine ⟨reserved,probed,computed,record,next,first,second,third,unchanged,?_,notified,tail,preserved,retained,readable,arrival⟩
  simpa [paidVector] using spent

#print axioms paid_vector_exact
#print axioms work_then_finish

-- Source-certified initial image enables the actual native model's first
-- initialization; a successful reply is not assumed. Fresh source revision
-- zero and a fresh owner are explicit. This is neither restart nor import.
theorem initialize_from_source {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId capacity : Nat} {seed : SourceInput.Bootstrap a}
    {old : PublicationCapacityDriver.State p a} {source : PublicationInput.State p a}
    {pending : Fin p → Bool} {credits : Fin p → Nat} {target : Fin p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source) (zero : source.publication.barrier.published = 0)
    (covered : OwnerFundingCursor.InitCovered old.suffix pending credits)
    (owed : pending target = true) (small : capacity ≤ 64) :
    ∃ owner : OwnerEndpoint.State p a,
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId capacity
        (OwnerEndpointBudget.initial (credits target))
        (.owner (.initialize (CustodyPublication.image source.publication.current))) =
        (⟨some owner,credits target-1⟩,.inl 10) ∧
      owner.owner.core.revision = source.publication.barrier.published ∧
      owner.fence = source.publication.barrier.published ∧
      OwnerFundingCursor.InitCovered old.suffix (fun i => if i=target then false else pending i)
        (fun i => if i=target then credits i-1 else credits i) := by
  obtain ⟨_,final,planned,_⟩ := valid.2 source present
  have imageFits := PublicationCapacity.assigned_endpoint_fits
    (PublicationCapacity.boundSequence_first planned) target
  have funded : 1 ≤ credits target := by
    have enough := covered target
    simp only [owed,ite_true] at enough
    omega
  let initialized : OwnerEndpoint.State p a :=
    ⟨OwnerReservation.initial ⟨assigned.realm,target⟩ scopeId
      (CustodyPublication.image source.publication.current) capacity,0⟩
  refine ⟨initialized,?_,?_,?_,OwnerFundingCursor.initialization_paid covered owed⟩
  · simp [OwnerEndpointBudget.transition,OwnerEndpointBudget.initial,
      OwnerEndpointBudget.cost,OwnerEndpointBudget.needsFinish,OwnerEndpointBudget.held,
      funded,OwnerEndpointProfile.transition,OwnerEndpointProfile.check,small,
      OwnerEndpoint.transition,OwnerReservationWorker.transition,imageFits,initialized]
  · simp [initialized,OwnerReservation.initial,zero]
  · simp [initialized,zero]

#print axioms initialize_from_source

-- Stronger independent pre-result profile for the actual two-Sum query
-- carrier. Plain funded-request readability cannot supply this headroom.
def CheckedCarrierResponseFits (p scopeId revision : Nat) (ticket : InvocationBoundary.Ticket) : Prop :=
  OwnerResponseProfile.ResponseFits scopeId revision ticket ∧ p ≤ 64 ∧
    (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372+12*p+25 ≤ 65536 ∧
    max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)+7 ≤ 256

def checkedResponseCheck (p scopeId revision : Nat) (ticket : InvocationBoundary.Ticket) : Bool :=
  OwnerResponseProfile.responseCheck scopeId revision ticket && decide (p ≤ 64 ∧
    (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372+12*p+25 ≤ 65536 ∧
    max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)+7 ≤ 256)

theorem checked_response_exact : checkedResponseCheck p scopeId revision ticket = true ↔
    CheckedCarrierResponseFits p scopeId revision ticket := by
  simp [checkedResponseCheck,CheckedCarrierResponseFits,OwnerResponseProfile.responseCheck_exact]

theorem checked_receipt_arrival_fits (profile : CheckedCarrierResponseFits p scopeId revision ticket)
    (ordinalSmall : ordinal < 64)
    (range : InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi)
    (vectorValue : Vector (Fin 513) p) :
    OwnerResponseProfile.WireFits (SourceFundingQuery.checkedInput p a) 256
      (.inr (.inr (vectorValue,.step (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩)))) := by
  open OwnerResponseProfile OwnerCodecTree in
  have envelope := fits_envelope profile.1.1 profile.1.2.1 ordinalSmall (response_ticket_fits profile.1) range
  open OwnerResponseProfile OwnerCodecTree in
  have commandWrapped : Fits (PublicationInput.command p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+358+4+5)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+18+2+2)
      (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩) :=
    fits_inr (fits_inl envelope _) (SourceInput.command p a)
  open OwnerResponseProfile OwnerCodecTree in
  have base : Fits (PublicationInput.input p a) 256
      ((OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372)
      (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)
      (.step (.arrival ⟨scopeId,ordinal,revision,ticket,.value value⟩)) :=
    fits_inr commandWrapped (SourceCodec.compact p)
  exact OwnerResponseProfile.fits_wire
    (SourceFundingFrame.checked_execution_fits profile.2.1 vectorValue base) profile.2.2.1 profile.2.2.2

theorem rooted_checked_computation_arrival
    (path : OwnerReservation.Steps (OwnerReservation.initial assigned scope image capacity) state)
    (small : capacity ≤ 64)
    (profile : CheckedCarrierResponseFits p state.core.scopeId state.core.revision ticket)
    (reserved : OwnerReservation.reserve state requestedScope ticket = (next,.reserved))
    (vectorValue : Vector (Fin 513) p) :
    ∃ later record, OwnerReservation.compute next = some (later,.produced record) ∧
      OwnerResponseProfile.WireFits (SourceFundingQuery.checkedInput p a) 256
        (.inr (.inr (vectorValue,.step (.arrival (OwnerReceipt.project record))))) := by
  obtain ⟨later,record,computed⟩ := OwnerReservation.rooted_reserved_computes path reserved
  obtain ⟨value,result,range⟩ := OwnerResponseProfile.computed_result_range computed
  have nextPath := OwnerReservation.Steps.next path (.reservation reserved)
  have ordinal := OwnerResponseProfile.rooted_computed_ordinal nextPath small computed
  obtain ⟨activeTicket,core,active,submitted,_⟩ := OwnerReservation.compute_parts computed
  have activeAt : next.active = some ticket := by rw [(OwnerReservation.reserve_parts reserved).2.2.2.2.2]
  have same : activeTicket = ticket := Option.some.inj (active.symm.trans activeAt)
  subst activeTicket
  have recordAt := (OwnerOccurrence.produced_parts submitted).2.2.2.2.1
  have coreAt := OwnerReservation.reserve_core reserved
  have scopeAt : record.scopeId = state.core.scopeId := by rw [recordAt,coreAt]
  have revisionAt : record.revision = state.core.revision := by rw [recordAt,coreAt]
  have ticketAt : record.ticket = ticket := by rw [recordAt]
  have projected : OwnerReceipt.project record =
      ⟨state.core.scopeId,record.ordinal,state.core.revision,ticket,.value value⟩ := by
    simp only [OwnerReceipt.project,scopeAt,revisionAt,ticketAt,result]
  refine ⟨later,record,computed,?_⟩
  rw [projected]
  exact checked_receipt_arrival_fits profile ordinal range vectorValue

#print axioms checked_response_exact
#print axioms checked_receipt_arrival_fits
#print axioms rooted_checked_computation_arrival

-- Known typed source refusals preserve the entire native semantic state,
-- including its private suffix and remaining quota. Equality of a projected
-- snapshot alone is not the premise of this law. Unknown IO is separate.
theorem known_refusal_frames {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    {old : PublicationCapacityDriver.State p a} {request : SourceFundingInput.Request p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (refused : (SourceFundingInput.execute assigned scopeId seed old request).2 ≠ .accepted) :
    (SourceFundingInput.execute assigned scopeId seed old request).1 = old := by
  cases ran : PublicationLifecycle.transitionFast assigned scopeId seed old request.2 with
  | mk candidate status =>
    by_cases accepted : status = .accepted
    · subst status
      by_cases enough : OwnerFundingCursor.chargeCheck candidate.suffix 0
          (SourceFundingInput.credits request) (SourceFundingInput.initialDebit old) = true
      · simp [SourceFundingInput.execute,ran,enough] at refused
      · simp [SourceFundingInput.execute,ran,enough]
    · have baseRefused : (PublicationLifecycle.transition assigned scopeId seed old request.2).2 ≠ .accepted := by
        rw [← PublicationLifecycle.transitionFast_exact valid,ran]
        exact accepted
      have framed := PublicationLifecycle.refusal_frames_state assigned scopeId seed old request.2 baseRefused
      simpa [SourceFundingInput.execute,ran,accepted,← PublicationLifecycle.transitionFast_exact valid] using framed

#print axioms known_refusal_frames
end MirroreaProofFirst.SourceFundingWork
