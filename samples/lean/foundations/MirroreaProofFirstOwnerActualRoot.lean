import MirroreaProofFirstPublicOwnerBoundary

namespace MirroreaProofFirst.OwnerActualRoot

-- Derive the reservation-model root used by computation progress from the
-- same actual budget command history. Initial assignment/scope/capacity are
-- never replaced with a new existential namespace after initialization.
def Original (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat)
    (state : Option (OwnerEndpoint.State p a)) : Prop :=
  ∀ owner, state = some owner → ∃ image,
    OwnerReservation.Steps (OwnerReservation.initial assigned scopeId image capacity) owner.owner

theorem endpoint_created {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity : Nat} {command : OwnerEndpoint.Command p a}
    {owner : OwnerEndpoint.State p a} {reply : Sum Nat OwnerReceipt.Envelope}
    (ran : OwnerEndpoint.transition assigned scopeId capacity none command = (some owner,reply)) :
    ∃ image, owner.owner = OwnerReservation.initial assigned scopeId image capacity := by
  cases command with
  | freeze revision => cases ran
  | owner command =>
    cases command <;> simp only [OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
    case «initialize» image =>
      split at ran
      · cases ran; exact ⟨image,rfl⟩
      · cases ran
    all_goals cases ran

theorem endpoint_preserves {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity : Nat} {command : OwnerEndpoint.Command p a}
    {old next : Option (OwnerEndpoint.State p a)} {reply : Sum Nat OwnerReceipt.Envelope}
    (root : Original assigned scopeId capacity old)
    (ran : OwnerEndpoint.transition assigned scopeId capacity old command = (next,reply)) :
    Original assigned scopeId capacity next := by
  intro after present
  cases old with
  | none =>
    rw [present] at ran
    obtain ⟨image,equal⟩ := endpoint_created ran
    exact ⟨image,by rw [equal]; exact .refl⟩
  | some before =>
    obtain ⟨image,path⟩ := root before rfl
    obtain ⟨value,equal,step,_⟩ := OwnerEndpoint.owner_step ran
    have same : value = after := Option.some.inj (equal.symm.trans present)
    subst value
    refine ⟨image,?_⟩
    rcases step with unchanged | stepped
    · rw [unchanged]; exact path
    · exact .next path stepped

theorem profile_preserves {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity : Nat} {command : OwnerEndpoint.Command p a}
    {old next : Option (OwnerEndpoint.State p a)} {reply : Sum Nat OwnerReceipt.Envelope}
    (root : Original assigned scopeId capacity old)
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity old command = (next,reply)) :
    Original assigned scopeId capacity next := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · exact endpoint_preserves root ran
  · cases ran; exact root

theorem budget_preserves {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity : Nat} {command : OwnerEndpoint.Command p a}
    {old next : OwnerEndpointBudget.State p a} {reply : Sum Nat OwnerReceipt.Envelope}
    (root : Original assigned scopeId capacity old.owner)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity old command = (next,reply)) :
    Original assigned scopeId capacity next.owner := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases underlying : OwnerEndpointProfile.transition assigned scopeId capacity old.owner command with
    | mk owner result =>
      rw [underlying] at ran
      cases ran
      exact profile_preserves root underlying
  · cases ran; exact root

theorem budget_root {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity budget : Nat} {state : OwnerEndpointBudget.State p a}
    (path : OwnerEndpointBudget.Runs assigned scopeId capacity budget state) :
    Original assigned scopeId capacity state.owner := by
  induction path with
  | fresh => intro owner impossible; cases impossible
  | next prior ran ih => exact budget_preserves ih ran

-- Public runs expand actual work, rather than assuming a rooted owner in every
-- Work constructor. Retired handles have no continuing-state obligation.
def ActualHistories (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) (state : PublicOwnerBoundary.State p a) : Prop :=
  ∀ owners, state = some owners → ∀ i,
    OwnerEndpointBudget.Runs (assigned i) scopeId (capacity i) budget (owners i)

theorem public_step {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {old next : PublicOwnerBoundary.State p a}
    (actual : ActualHistories assigned scopeId capacity budget old)
    (step : PublicOwnerBoundary.Step assigned scopeId capacity old next) :
    ActualHistories assigned scopeId capacity budget next := by
  cases step with
  | frame => exact actual
  | retire => intro owners impossible; cases impossible
  | @admin owners target command after reply allowed ran =>
    intro next equal i
    cases equal
    by_cases same : i = target
    · subst i; simpa [PublicOwnerBoundary.put] using OwnerEndpointBudget.Runs.next (actual owners rfl target) ran
    · simpa [PublicOwnerBoundary.put,same] using actual owners rfl i
  | @work owners target ticket revision after work =>
    intro next equal i
    cases equal
    by_cases same : i = target
    · subst i
      cases work with
      | run reserved probed computed =>
        simpa [PublicOwnerBoundary.put] using
          OwnerEndpointBudget.Runs.next (.next (.next (actual owners rfl target) reserved) probed) computed
    · simpa [PublicOwnerBoundary.put,same] using actual owners rfl i

theorem public_histories {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {state : PublicOwnerBoundary.State p a}
    (path : PublicOwnerBoundary.Runs assigned scopeId capacity budget state) :
    ActualHistories assigned scopeId capacity budget state := by
  induction path with
  | fresh => intro owners equal i; cases equal; exact .fresh
  | step prior step ih => exact public_step ih step

theorem public_root {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat} {owners : PublicOwnerBoundary.Owners p a}
    {i : Fin p} {owner : OwnerEndpoint.State p a}
    (path : PublicOwnerBoundary.Runs assigned scopeId capacity budget (some owners))
    (present : (owners i).owner = some owner) :
    ∃ image, OwnerReservation.Steps (OwnerReservation.initial (assigned i) scopeId image (capacity i)) owner.owner :=
  budget_root (public_histories path owners rfl i) owner present

def namespaceData (owner : OwnerOccurrence.State p a) :=
  (owner.assigned,owner.scopeId,owner.capacity)

theorem submitted_namespace {p a : Nat} {old next : OwnerOccurrence.State p a}
    {scopeId : Nat} {ticket : OwnerOccurrence.Ticket} {reply : OwnerOccurrence.Reply p a}
    (ran : OwnerOccurrence.submit old scopeId ticket = (next,reply)) : namespaceData next = namespaceData old := by
  unfold OwnerOccurrence.submit at ran
  split at ran
  · cases ran; rfl
  · split at ran
    · split at ran <;> cases ran <;> rfl
    · split at ran <;> cases ran <;> rfl

theorem reservation_namespace {p a : Nat} {old next : OwnerReservation.State p a}
    (step : OwnerReservation.Step old next) : namespaceData next.core = namespaceData old.core := by
  cases step with
  | reservation ran => rw [OwnerReservation.reserve_core ran]
  | computation ran =>
    obtain ⟨ticket,core,active,submitted,rfl⟩ := OwnerReservation.compute_parts ran
    exact submitted_namespace submitted
  | installation ran => rw [(OwnerReservation.install_idle ran).2]; rfl
  | abandonment => rfl

theorem rooted_namespace {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity : Nat} {image : OwnerImage.Image p a} {owner : OwnerReservation.State p a}
    (path : OwnerReservation.Steps (OwnerReservation.initial assigned scopeId image capacity) owner) :
    namespaceData owner.core = (assigned,scopeId,capacity) := by
  induction path with
  | refl => rfl
  | next prior step ih => exact (reservation_namespace step).trans ih

-- Root, namespace, small capacity and idle now follow from one actual public
-- history. Fresh reservation keys and remaining slots are still INDEPENDENT
-- premises, as are admission/currentness/profile and current debit coverage.
theorem public_enabled_work {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget revision : Nat} {capacity : Fin p → Nat} {owners : PublicOwnerBoundary.Owners p a}
    {i : Fin p} {owner : OwnerEndpoint.State p a} {ticket : OwnerOccurrence.Ticket}
    (history : PublicOwnerBoundary.Runs assigned scopeId capacity budget (some owners))
    (present : (owners i).owner = some owner) (small : capacity i ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision ticket)
    (current : owner.owner.core.revision = owner.fence)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image ticket)
    (fresh : OwnerReservation.hasKey owner.owner ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity)
    (funded : 3 ≤ (owners i).remaining) :
    ∃ next, PublicOwnerBoundary.Work (assigned i) scopeId (capacity i) ticket revision (owners i) next ∧
      OwnerEndpointBudget.held next.owner = false := by
  obtain ⟨image,path⟩ := public_root history present
  have names := rooted_namespace path
  have actualCapacity : owner.owner.core.capacity = capacity i := congrArg (fun x => x.2.2) names
  have actualScope : owner.owner.core.scopeId = scopeId := congrArg (fun x => x.2.1) names
  obtain ⟨next,work,idle⟩ := PublicOwnerBoundary.enabled_work
    (assigned:=assigned i) (capacity:=capacity i) (revision:=revision)
    path (by omega) profile current actualScope.symm admitted
    (PublicOwnerBoundary.actual_present_idle history present) fresh room funded
  refine ⟨next,?_,idle⟩
  have same : (⟨some owner,(owners i).remaining⟩ : OwnerEndpointBudget.State p a) = owners i := by
    cases actual : owners i with
    | mk value remaining => simp only [actual] at present ⊢; cases present; rfl
  rw [same] at work
  exact work

-- Decisive general negative: idle/current/admitted/fresh and enough credits
-- still do not create a reservation slot. Refusal5 is an actual debit, not an
-- inferred semantic success. This includes capacity0 with an empty history.
theorem exhausted_reservation {p a : Nat} {assigned : OwnerEvaluator.Assignment p}
    {scopeId capacity credits : Nat} {owner : OwnerEndpoint.State p a}
    {ticket : OwnerOccurrence.Ticket}
    (small : owner.owner.core.capacity ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision ticket)
    (current : owner.owner.core.revision = owner.fence)
    (scope : scopeId = owner.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image ticket)
    (idle : owner.owner.active = none) (fresh : OwnerReservation.hasKey owner.owner ticket = false)
    (full : owner.owner.core.capacity ≤ owner.owner.reserved.length) (funded : 2 ≤ credits) :
    OwnerEndpointBudget.transition assigned scopeId capacity ⟨some owner,credits⟩ (.owner (.reserve ticket)) =
      (⟨some owner,credits-1⟩,.inl 5) := by
  have checked : OwnerOccurrence.currentCheck owner.owner.core ticket = true := OwnerOccurrence.current_exact.mpr admitted
  have reserved : OwnerReservation.reserve owner.owner scopeId ticket = (owner.owner,.exhausted) := by
    simp [OwnerReservation.reserve,scope,checked,idle,fresh,full]
  have profiled : OwnerEndpointProfile.transition assigned scopeId capacity (some owner) (.owner (.reserve ticket)) =
      (some owner,.inl 5) := by
    rw [OwnerEndpointProfile.admitted_exact (show OwnerEndpointProfile.Accepts capacity (some owner) (.owner (.reserve ticket)) from ⟨small,profile⟩)]
    simp [OwnerEndpoint.transition,OwnerEndpoint.confirms,OwnerEndpoint.permitted,current,
      OwnerReservationWorker.transition,reserved,OwnerReservationWorker.responseCode]
  rw [OwnerEndpointBudget.funded_exact (show OwnerEndpointBudget.cost (some owner) (.owner (.reserve ticket)) ≤ credits from funded)]
  simp [profiled]

#print axioms endpoint_created
#print axioms endpoint_preserves
#print axioms profile_preserves
#print axioms budget_preserves
#print axioms budget_root
#print axioms public_step
#print axioms public_histories
#print axioms public_root
#print axioms submitted_namespace
#print axioms reservation_namespace
#print axioms rooted_namespace
#print axioms public_enabled_work
#print axioms exhausted_reservation

end MirroreaProofFirst.OwnerActualRoot
