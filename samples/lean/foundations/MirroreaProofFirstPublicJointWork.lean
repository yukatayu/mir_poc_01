import MirroreaProofFirstPublicJointReplay
import MirroreaProofFirstSourceFundingCheckedWork

namespace MirroreaProofFirst.PublicJointWork
open PublicationOwnerBudget SourceFundingWork

-- External candidate: same-witness computation plus correlated record. This
-- strengthens the existing interval proof; it does not assume successful work.
theorem correlated_interval {s : OwnerEndpoint.State p a}
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
      record.ticket = ticket ∧ record.scopeId = s.owner.core.scopeId ∧ record.revision = s.owner.core.revision ∧
      probed.owner = reserved.owner ∧ computed.remaining = credits-3 ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (SourceFundingQuery.checkedInput p a) 256
        (.inr (.inr (actual,.step (.arrival (OwnerReceipt.project record))))) := by
  obtain ⟨owner,reserve⟩ := OwnerReservation.admissible_reservation_progress scope admitted idle fresh room
  obtain ⟨later,record,compute,checkedArrival⟩ :=
    SourceFundingWork.rooted_checked_computation_arrival path
      (by simpa [OwnerResponseProfile.rooted_capacity path] using small) profile reserve actual
  obtain ⟨activeTicket,core,activeAt,submitted,_⟩ := OwnerReservation.compute_parts compute
  have ticketAt : owner.active = some ticket := by rw [(OwnerReservation.reserve_parts reserve).2.2.2.2.2]
  have equalTicket : activeTicket = ticket := Option.some.inj (activeAt.symm.trans ticketAt)
  subst activeTicket
  have recordAt := (OwnerOccurrence.produced_parts submitted).2.2.2.2.1
  have oldCore := OwnerReservation.reserve_core reserve
  have correlated : record.ticket = ticket ∧ record.scopeId = s.owner.core.scopeId ∧ record.revision = s.owner.core.revision := by
    rw [recordAt,oldCore]
    exact ⟨rfl,rfl,rfl⟩
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
  refine ⟨⟨some held,credits-1⟩,⟨some held,credits-2⟩,⟨some done,credits-3⟩,record,?_,?_,?_,correlated.1,correlated.2.1,correlated.2.2,rfl,rfl,
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

#print axioms correlated_interval


-- Construct the whole closed operation using the SAME joint source/owner
-- history and an independently certified post-entry source driver. No owner
-- success, computation result or successful source finish is a premise.
-- The vector is tied to every owner, not only the work target. Admission,
-- fresh lifetime key, free slot, profile and driver validity remain explicit.
theorem funded_entered_work {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId budget revision : Nat} {capacity : Fin p → Nat}
    {seed : QualifiedCustody.State p a} {bootstrap : SourceInput.Bootstrap a}
    {s : SourceRegistration.State p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
    {entered : PublicationInput.State p a} {old : PublicationCapacityDriver.State p a}
    {rest : List (PublicationInput.Command p a)} {owner : OwnerEndpoint.State p a}
    {before : Vector (Fin 513) p}
    (history : PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (enter : PublicationInput.execute scopeId s.actual.source (.enter target) = some entered)
    (dispatch : entered.dispatch = some ⟨target,⟨scopeId,s.actual.source.publication.barrier.installed target⟩,ticket⟩)
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (sourceAt : old.source = some entered) (head : old.suffix = .finish target :: rest)
    (vectorAt : ∀ i, (s.actual.owners i).remaining = (before[i.val]).val)
    (present : (s.actual.owners target).owner = some owner)
    (covered : Covered old.suffix (fun i => (before[i.val]).val))
    (small : capacity target ≤ 64)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image ticket)
    (fresh : OwnerReservation.hasKey owner.owner ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity)
    (carrier : p ≤ 64 ∧
      (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372+12*p+25 ≤ 65536 ∧
      max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)+7 ≤ 256) :
    ∃ (reserved probed computed : OwnerEndpointBudget.State p a) (record : OwnerOccurrence.Record p a)
      (after : PublicationInput.State p a) (next : PublicationCapacityDriver.State p a),
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) (s.actual.owners target)
        (.owner (.reserve ticket)) = (reserved,.inl 6) ∧
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) reserved
        (.freeze revision) = (probed,.inl 2) ∧
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scopeId (capacity target) probed
        (.owner .compute) = (computed,.inr (OwnerReceipt.project record)) ∧
      record.ticket = ticket ∧ record.scopeId = scopeId ∧
      record.revision = s.actual.source.publication.barrier.installed target ∧
      PublicationInput.execute scopeId entered (.finish target) = some after ∧
      SourceFundingInput.execute assigned scopeId bootstrap old
        (paidVector before target,.step (.finish target)) = (next,.accepted) ∧
      next.source = some after ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId bootstrap next ∧
      Covered next.suffix (fun i => ((paidVector before target)[i.val]).val) ∧
      (∀ i, ((PublicJointHistory.workResult s target ticket revision reserved probed computed
        (OwnerReceipt.project record) after).actual.owners i).remaining = ((paidVector before target)[i.val]).val) ∧
      PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed
        (PublicJointHistory.workResult s target ticket revision reserved probed computed (OwnerReceipt.project record) after) ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (SourceFundingQuery.checkedInput p a) 256
        (.inr (.inr (paidVector before target,.step (.arrival (OwnerReceipt.project record))))) := by
  have registration := PublicJointHistory.registration_path history
  obtain ⟨actual,atOwner,atRevision,atImage,atFence⟩ := SourceRegistration.entered_current registration clear present enter
  have sameOwner : actual = owner := Option.some.inj (atOwner.symm.trans present)
  subst actual
  obtain ⟨initialImage,root⟩ := OwnerActualRoot.public_root (PublicJointHistory.owner_path history) present
  have names := OwnerActualRoot.rooted_namespace root
  have capacityAt : owner.owner.core.capacity = capacity target := congrArg (fun x => x.2.2) names
  have scopeAt : owner.owner.core.scopeId = scopeId := congrArg (fun x => x.2.1) names
  have floor := SourceOwnerFloor.continuing_floors (SourceOwnerImage.floor_path (SourceRegistration.image_path registration)) clear target
  have fenceAt : owner.fence = s.actual.source.publication.barrier.fence target := by
    simpa [OwnerFenceMonitor.floor,present] using floor
  have revisionAt : owner.owner.core.revision = s.actual.source.publication.barrier.installed target := by
    rw [SourceOwnerGate.entered_logical_gate enter]
    exact atRevision.trans (atFence.symm.trans fenceAt)
  obtain ⟨_,final,planned,_⟩ := valid.2 entered sourceAt
  have profile : OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision ticket := by
    have known := PublicationCapacity.state_dispatch_fits (PublicationCapacity.boundSequence_first planned).1
      ⟨target,⟨scopeId,s.actual.source.publication.barrier.installed target⟩,ticket⟩ dispatch
    simpa [scopeAt,revisionAt] using known
  have funded : 3 ≤ (s.actual.owners target).remaining := by
    have enough := covered target
    rw [head,demand_cons] at enough
    simp only [commandCost,ite_true] at enough
    rw [vectorAt target]
    omega
  obtain ⟨reserved,probed,computed,record,reserve,probe,compute,ticketAt,recordScope,recordRevision,unchanged,spent,readable,arrival⟩ :=
    correlated_interval (assigned:=⟨assigned.realm,target⟩) (scopeId:=scopeId) (capacity:=capacity target)
      (revision:=revision) root (by omega) ⟨profile,carrier⟩ (paidVector before target)
      (atRevision.trans atFence.symm) scopeAt.symm admitted
      (PublicOwnerBoundary.actual_present_idle (PublicJointHistory.owner_path history) present) fresh room funded
  have budgetAt : (⟨some owner,(s.actual.owners target).remaining⟩ : OwnerEndpointBudget.State p a) = s.actual.owners target := by
    cases actualState : s.actual.owners target with
    | mk value remaining => simp only [actualState] at present ⊢; cases present; rfl
  rw [budgetAt] at reserve
  have correlated : record.ticket = ticket ∧ record.scopeId = scopeId ∧
      record.revision = s.actual.source.publication.barrier.installed target :=
    ⟨ticketAt,recordScope.trans scopeAt,recordRevision.trans revisionAt⟩
  obtain ⟨next,notified,tail,preserved,retained⟩ :=
    SourceFundingPreservation.paid_head_admitted valid sourceAt head covered (paid_vector_exact (a:=a) before target)
  obtain ⟨after,atAfter,finished⟩ := SourceOwnerFloor.funded_step_actual valid sourceAt notified
  refine ⟨reserved,probed,computed,record,after,next,reserve,probe,compute,correlated.1,correlated.2.1,correlated.2.2,
    finished,notified,atAfter,tail,preserved,retained,?_,?_,readable,arrival⟩
  · intro i
    rw [PublicJointHistory.work_owners]
    by_cases equal : i = target
    · subst i
      simpa [PublicOwnerBoundary.put,paidVector,vectorAt target] using spent
    · simp [PublicOwnerBoundary.put,paidVector,equal,vectorAt]
  · exact .step history (.work clear enter dispatch reserve probe compute
      correlated.1 correlated.2.1 correlated.2.2 finished)

#print axioms funded_entered_work


-- Start BEFORE entry. The certified source suffix supplies entry and finish
-- enabledness, while the waiting ticket is read from this exact source. The
-- resulting checker acceptance is derived from the constructed operation.
theorem funded_scheduled_work {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId budget revision : Nat} {capacity : Fin p → Nat}
    {seed : QualifiedCustody.State p a} {bootstrap : SourceInput.Bootstrap a}
    {s : SourceRegistration.State p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
    {old : PublicationCapacityDriver.State p a} {rest : List (PublicationInput.Command p a)}
    {owner : OwnerEndpoint.State p a} {before : Vector (Fin 513) p}
    (history : PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (valid : PublicationLifecycle.Invariant assigned scopeId bootstrap old)
    (sourceAt : old.source = some s.actual.source)
    (head : old.suffix = .enter target :: .finish target :: rest)
    (waiting : s.actual.source.publication.current.privateState.session.state.source.waiting.map
      (fun saved => saved.entry.ticket) = some ticket)
    (vectorAt : ∀ i, (s.actual.owners i).remaining = (before[i.val]).val)
    (present : (s.actual.owners target).owner = some owner)
    (covered : Covered old.suffix (fun i => (before[i.val]).val))
    (small : capacity target ≤ 64)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image ticket)
    (fresh : OwnerReservation.hasKey owner.owner ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity)
    (carrier : p ≤ 64 ∧
      (OwnerPacketCodec.encode OwnerFullCodec.ticket ticket).length+372+12*p+25 ≤ 65536 ∧
      max (p+2) (OwnerTreeBytes.treeCost (OwnerFullCodec.ticket.encode ticket)+24)+7 ≤ 256) :
    ∃ (intermediate next : PublicationCapacityDriver.State p a) (after : SourceRegistration.State p a),
      SourceFundingInput.execute assigned scopeId bootstrap old (before,.step (.enter target)) = (intermediate,.accepted) ∧
      SourceFundingInput.execute assigned scopeId bootstrap intermediate
        (paidVector before target,.step (.finish target)) = (next,.accepted) ∧
      next.source = some after.actual.source ∧ next.suffix = rest ∧
      PublicationLifecycle.Invariant assigned scopeId bootstrap next ∧
      Covered next.suffix (fun i => ((paidVector before target)[i.val]).val) ∧
      (∀ i, (after.actual.owners i).remaining = ((paidVector before target)[i.val]).val) ∧
      PublicJointHistory.Step (fun i => ⟨assigned.realm,i⟩) scopeId capacity s after ∧
      PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scopeId capacity budget seed after ∧
      (PublicJointReplay.advance ⟨s,history⟩ (.work target revision)).map Subtype.val = some after := by
  have paid : ∀ i, SourceFundingInput.credits ((before,PublicationInput.Input.step (.enter target)) : SourceFundingInput.Request p a) i =
      (before[i.val]).val-commandCost (.enter target : PublicationInput.Command p a) i := by
    intro i; simp [SourceFundingInput.credits,commandCost]
  obtain ⟨intermediate,enteredSource,headFinish,validEntered,coveredEntered⟩ :=
    SourceFundingPreservation.paid_head_admitted valid sourceAt head covered paid
  obtain ⟨entered,atEntered,enter⟩ := SourceOwnerFloor.funded_step_actual valid sourceAt enteredSource
  obtain ⟨_,saved,atWaiting,_,dispatch⟩ := PublicationInput.dispatch_created enter
  have ticketAt : saved.entry.ticket = ticket := by simpa [atWaiting] using waiting
  rw [ticketAt] at dispatch
  obtain ⟨reserved,probed,computed,record,finalSource,next,reserve,probe,compute,ticketCorrelated,scopeCorrelated,
    revisionCorrelated,finish,notified,atNext,tail,preserved,retained,vectorNext,jointNext,readable,arrival⟩ :=
    funded_entered_work (revision:=revision) history clear enter dispatch validEntered atEntered headFinish
      vectorAt present coveredEntered small admitted fresh room carrier
  let after := PublicJointHistory.workResult s target ticket revision reserved probed computed (OwnerReceipt.project record) finalSource
  refine ⟨intermediate,next,after,enteredSource,notified,atNext,tail,preserved,retained,vectorNext,?_,jointNext,?_⟩
  · exact .work clear enter dispatch reserve probe compute ticketCorrelated scopeCorrelated revisionCorrelated finish
  · exact PublicJointReplay.work_complete ⟨s,history⟩ clear enter dispatch reserve probe compute
      ⟨ticketCorrelated,scopeCorrelated,revisionCorrelated⟩ finish

#print axioms funded_scheduled_work
end MirroreaProofFirst.PublicJointWork
