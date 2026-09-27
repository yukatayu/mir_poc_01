import MirroreaProofFirstOwnerActualRoot

namespace MirroreaProofFirst.OwnerReservationMonitor

abbrev Known := Nat × List (Nat × Nat)

def view (owner : OwnerReservation.State p a) : Known :=
  (owner.core.capacity,owner.reserved.map OwnerOccurrence.key)

def project (state : Option (OwnerEndpoint.State p a)) : Option Known :=
  state.map (fun owner => view owner.owner)

-- Initialized capacity is the frozen native startup argument. Actual reserve6
-- alone adds a key. Refusal, computation, cancellation, installation and credit
-- exhaustion never refund a reservation slot or erase a used key.
def observe (capacity : Nat) (known : Option Known) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : Option Known :=
  match command,reply with
  | .owner (.initialize _),.inl 10 => some (capacity,[])
  | .owner (.reserve ticket),.inl 6 => known.map (fun (cap,keys) => (cap,OwnerOccurrence.key ticket :: keys))
  | _,_ => known

theorem reserve_view {p a : Nat} {old next : OwnerReservation.State p a}
    {scopeId : Nat} {ticket : OwnerOccurrence.Ticket} {response : OwnerReservation.Response}
    (ran : OwnerReservation.reserve old scopeId ticket = (next,response)) :
    view next = if response = .reserved then (old.core.capacity,OwnerOccurrence.key ticket :: (view old).2) else view old := by
  unfold OwnerReservation.reserve at ran
  split at ran
  · cases ran; rfl
  · split at ran
    · cases ran; rfl
    · split at ran
      · split at ran <;> cases ran <;> rfl
      · split at ran <;> cases ran <;> rfl

theorem compute_view {p a : Nat} {old next : OwnerReservation.State p a}
    {reply : OwnerOccurrence.Reply p a}
    (ran : OwnerReservation.compute old = some (next,reply)) : view next = view old := by
  obtain ⟨ticket,core,active,submitted,rfl⟩ := OwnerReservation.compute_parts ran
  have capacity := congrArg (fun x => x.2.2) (OwnerActualRoot.submitted_namespace submitted)
  simp only [OwnerActualRoot.namespaceData] at capacity
  simp [view,capacity]

theorem worker_exact
    (ran : OwnerReservationWorker.transition assigned scopeId capacity state command = (next,reply)) :
    next.map view = observe capacity (state.map view) (.owner command) reply := by
  cases state with
  | none =>
    cases command <;> simp only [OwnerReservationWorker.transition] at ran
    case «initialize» image => split at ran <;> cases ran <;> rfl
    all_goals cases ran; rfl
  | some state =>
    cases command with
    | «initialize» image => cases ran; rfl
    | reserve ticket =>
      cases reserved : OwnerReservation.reserve state scopeId ticket with
      | mk owner status =>
        have known := reserve_view reserved
        simp only [OwnerReservationWorker.transition,reserved] at ran
        cases ran
        cases status <;> simpa [observe,OwnerReservationWorker.responseCode,view] using congrArg some known
    | compute =>
      cases computed : OwnerReservation.compute state with
      | none => simp only [OwnerReservationWorker.transition,computed] at ran; cases ran; rfl
      | some pair =>
        obtain ⟨owner,status⟩ := pair
        have same := compute_view computed
        simp only [OwnerReservationWorker.transition,computed] at ran
        cases ran
        cases status <;> simpa only [observe,Option.map_some] using congrArg some same
    | install revision image =>
      simp only [OwnerReservationWorker.transition] at ran
      split at ran
      · cases installed : OwnerReservation.install state revision image with
        | none => simp only [installed] at ran; cases ran; rfl
        | some owner =>
          have same := (OwnerReservation.install_idle installed).2
          simp only [installed] at ran
          cases ran
          simp [observe,view,same,OwnerOccurrence.install]
      · cases ran; rfl
    | abandon => cases ran; rfl

theorem endpoint_exact
    (ran : OwnerEndpoint.transition assigned scopeId capacity state command = (next,reply)) :
    project next = observe capacity (project state) command reply := by
  cases state with
  | none =>
    cases command with
    | freeze revision => cases ran; rfl
    | owner command =>
      cases worker : OwnerReservationWorker.transition assigned scopeId capacity none command with
      | mk owner response =>
        have exact := worker_exact worker
        simp only [OwnerEndpoint.transition,worker] at ran
        cases ran
        simpa only [project,Option.map_map,Function.comp_def] using exact
  | some state =>
    cases command with
    | freeze revision =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran <;> cases ran <;> rfl
    | owner command =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran
      · rename_i confirmed
        cases command with
        | install revision image => cases ran; rfl
        | «initialize» image => cases confirmed
        | reserve ticket => cases confirmed
        | compute => cases confirmed
        | abandon => cases confirmed
      · split at ran
        · cases worker : OwnerReservationWorker.transition assigned scopeId capacity (some state.owner) command with
          | mk owner response =>
            have exact := worker_exact worker
            rw [worker] at ran
            cases ran
            simpa only [project,Option.map_map,Function.comp_def,Option.map_some] using exact
        · cases ran
          cases command <;> rfl

theorem profile_exact
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply)) :
    project next = observe capacity (project state) command reply := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · exact endpoint_exact ran
  · cases ran
    cases command with
    | freeze revision => rfl
    | owner command => cases command <;> rfl

theorem budget_exact
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    project next.owner = observe capacity (project state.owner) command reply := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases native : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk owner response =>
      rw [native] at ran
      cases ran
      exact profile_exact native
  · cases ran
    cases command with
    | freeze revision => rfl
    | owner command => cases command <;> rfl

inductive Runs (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) :
    OwnerEndpointBudget.State p a → Option Known → Prop where
  | fresh : Runs assigned scopeId capacity budget (OwnerEndpointBudget.initial budget) none
  | step : Runs assigned scopeId capacity budget state known →
      OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      Runs assigned scopeId capacity budget next (observe capacity known command reply)

theorem reached_exact (path : Runs assigned scopeId capacity budget state known) :
    project state.owner = known := by
  induction path with
  | fresh => rfl
  | step prior ran ih => rw [budget_exact ran,ih]

def slotCheck (known : Option Known) (ticket : OwnerOccurrence.Ticket) : Bool :=
  match known with
  | none => false
  | some (capacity,keys) =>
      !keys.any (fun key => decide (key = OwnerOccurrence.key ticket)) && decide (keys.length < capacity)

def Slot (state : OwnerEndpointBudget.State p a) (ticket : OwnerOccurrence.Ticket) : Prop :=
  ∃ owner, state.owner = some owner ∧ OwnerReservation.hasKey owner.owner ticket = false ∧
    owner.owner.reserved.length < owner.owner.core.capacity

theorem slot_check_exact (path : Runs assigned scopeId capacity budget state known) :
    slotCheck known ticket = true ↔ Slot state ticket := by
  rw [←reached_exact path]
  cases present : state.owner with
  | none => simp [slotCheck,project,present,Slot]
  | some owner =>
    simp [slotCheck,project,present,view,Slot,OwnerReservation.hasKey,List.any_map,Function.comp_def]

-- Both histories concern the very same budget-owner value and frozen startup
-- assignment. The public history derives root/idle; the actual observation
-- history derives fresh key/room. No fresh/room/idle predicate is supplied by
-- the caller. Physical trace custody remains a separate implementation duty.
theorem checked_public_work {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget revision : Nat} {capacity : Fin p → Nat} {owners : PublicOwnerBoundary.Owners p a}
    {i : Fin p} {known : Option Known} {owner : OwnerEndpoint.State p a} {ticket : OwnerOccurrence.Ticket}
    (publicHistory : PublicOwnerBoundary.Runs assigned scopeId capacity budget (some owners))
    (observations : Runs (assigned i) scopeId (capacity i) budget (owners i) known)
    (present : (owners i).owner = some owner) (small : capacity i ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision ticket)
    (current : owner.owner.core.revision = owner.fence)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image ticket)
    (checked : slotCheck known ticket = true) (funded : 3 ≤ (owners i).remaining) :
    ∃ next, PublicOwnerBoundary.Work (assigned i) scopeId (capacity i) ticket revision (owners i) next ∧
      OwnerEndpointBudget.held next.owner = false := by
  obtain ⟨actual,atOwner,fresh,room⟩ := (slot_check_exact observations).mp checked
  have same : actual = owner := Option.some.inj (atOwner.symm.trans present)
  subst actual
  exact OwnerActualRoot.public_enabled_work publicHistory present small profile current admitted fresh room funded

#print axioms checked_public_work

#print axioms reserve_view
#print axioms compute_view
#print axioms worker_exact
#print axioms endpoint_exact
#print axioms profile_exact
#print axioms budget_exact
#print axioms reached_exact
#print axioms slot_check_exact

end MirroreaProofFirst.OwnerReservationMonitor
