import MirroreaProofFirstOwnerOccurrence

namespace MirroreaProofFirst.OwnerReservation
open OwnerOccurrence

-- A sole owner driver retains reservation before dispatching computation.
-- Abandonment never makes a key fresh. Installation is excluded while active.
-- This is a nonproduction transition model; native custody, process death and
-- authenticated allocation of scopeId remain explicit implementation duties.
structure State (p a : Nat) where
  core : OwnerOccurrence.State p a
  reserved : List Ticket
  active : Option Ticket

inductive Response where
  | denied | busy | duplicate | conflict | exhausted | reserved
  deriving DecidableEq, Repr

-- Fresh namespace only, not same-instance recovery. No populated state import.
def initial (assigned : OwnerEvaluator.Assignment p) (scopeId : Nat)
    (image : OwnerImage.Image p a) (capacity : Nat) : State p a :=
  ⟨⟨assigned,scopeId,0,image,[],capacity⟩,[],none⟩

def hasKey (s : State p a) (ticket : Ticket) : Bool :=
  s.reserved.any (fun old => decide (key old = key ticket))

def reserve (s : State p a) (scopeId : Nat) (ticket : Ticket) : State p a × Response :=
  if scopeId ≠ s.core.scopeId || !currentCheck s.core ticket then (s,.denied)
  else if s.active.isSome then (s,.busy)
  else if hasKey s ticket then
    if s.reserved.contains ticket then (s,.duplicate) else (s,.conflict)
  else if s.core.capacity ≤ s.reserved.length then (s,.exhausted)
  else ({s with reserved := ticket :: s.reserved,active := some ticket},.reserved)

def compute (s : State p a) : Option (State p a × Reply p a) := do
  let ticket ← s.active
  let (core,reply) := submit s.core s.core.scopeId ticket
  return ({s with core := core,active := none},reply)

def abandon (s : State p a) : State p a := {s with active := none}

def install (s : State p a) (revision : Nat) (image : OwnerImage.Image p a) : Option (State p a) :=
  if s.active.isSome then none else
    some {s with core := OwnerOccurrence.install s.core revision image}

theorem reserve_parts (accepted : reserve s scopeId ticket = (next,.reserved)) :
    scopeId = s.core.scopeId ∧ currentCheck s.core ticket = true ∧ s.active = none ∧
    hasKey s ticket = false ∧ s.reserved.length < s.core.capacity ∧
    next = {s with reserved := ticket :: s.reserved,active := some ticket} := by
  unfold reserve at accepted
  split at accepted
  · cases accepted
  · rename_i allowed
    split at accepted
    · cases accepted
    · rename_i idle
      split at accepted
      · split at accepted <;> cases accepted
      · rename_i fresh
        split at accepted
        · cases accepted
        · rename_i room
          have admitted : scopeId = s.core.scopeId ∧ currentCheck s.core ticket = true := by simpa using allowed
          refine ⟨admitted.1,admitted.2,?_,?_,Nat.lt_of_not_ge room,?_⟩
          · simpa using idle
          · simpa using fresh
          · exact (Prod.mk.inj accepted).1.symm

theorem reserve_core (transition : reserve s scopeId ticket = (next,response)) : next.core = s.core := by
  unfold reserve at transition
  split at transition
  · cases transition; rfl
  · split at transition
    · cases transition; rfl
    · split at transition
      · split at transition <;> cases transition <;> rfl
      · split at transition <;> cases transition <;> rfl

theorem reserve_retains (transition : reserve s scopeId ticket = (next,response))
    (old : prior ∈ s.reserved) : prior ∈ next.reserved := by
  unfold reserve at transition
  split at transition
  · cases transition; exact old
  · split at transition
    · cases transition; exact old
    · split at transition
      · split at transition <;> cases transition <;> exact old
      · split at transition
        · cases transition; exact old
        · cases transition; exact List.mem_cons_of_mem _ old

theorem compute_parts (transition : compute s = some (next,reply)) :
    ∃ ticket core, s.active = some ticket ∧ submit s.core s.core.scopeId ticket = (core,reply) ∧
      next = {s with core := core,active := none} := by
  cases active : s.active with
  | none => simp [compute,active] at transition
  | some ticket =>
      cases result : submit s.core s.core.scopeId ticket with
      | mk core reply =>
          simp only [compute,active,Option.bind_eq_bind,Option.bind_some,result,
            Option.pure_def,Option.some.injEq,Prod.mk.injEq] at transition
          obtain ⟨state,equal⟩ := transition
          exact ⟨ticket,core,rfl,equal ▸ result,state.symm⟩

theorem computation_clears (transition : compute s = some (next,reply)) : compute next = none := by
  obtain ⟨_,_,_,_,rfl⟩ := compute_parts transition
  rfl

theorem computation_meaning (transition : compute s = some (next,.produced record)) :
    ∃ value, record.result = .value value ∧ OwnerEvaluator.Admitted s.core.assigned record.image record.ticket ∧
      InstancePrograms.Machine.Executes record.ticket.definition.code record.ticket.argument value := by
  obtain ⟨_,_,_,produced,_⟩ := compute_parts transition
  exact produced_meaning produced

theorem install_idle (installed : install s revision image = some next) :
    s.active = none ∧ next = {s with core := OwnerOccurrence.install s.core revision image} := by
  unfold install at installed
  split at installed
  · cases installed
  · rename_i idle
    exact ⟨by simpa using idle,(Option.some.inj installed).symm⟩

inductive Step : State p a → State p a → Prop where
  | reservation : reserve s scopeId ticket = (next,response) → Step s next
  | computation : compute s = some (next,reply) → Step s next
  | installation : install s revision image = some next → Step s next
  | abandonment : Step s (abandon s)

inductive Steps : State p a → State p a → Prop where
  | refl : Steps s s
  | next : Steps origin s → Step s next → Steps origin next

theorem step_retains (step : Step s next) (old : ticket ∈ s.reserved) : ticket ∈ next.reserved := by
  cases step with
  | reservation transition => exact reserve_retains transition old
  | computation transition => obtain ⟨_,_,_,_,rfl⟩ := compute_parts transition; exact old
  | installation transition => rw [(install_idle transition).2]; exact old
  | abandonment => exact old

theorem path_retains (path : Steps s next) (old : ticket ∈ s.reserved) : ticket ∈ next.reserved := by
  induction path with
  | refl => exact old
  | next _ step ih => exact step_retains step ih

theorem fresh_key (accepted : reserve s scopeId ticket = (next,.reserved)) (old : prior ∈ s.reserved) :
    key prior ≠ key ticket := by
  have fresh := (reserve_parts accepted).2.2.2.1
  intro same
  have yes : hasKey s ticket = true := by simp only [hasKey,List.any_eq_true]; exact ⟨prior,old,by simpa using same⟩
  rw [fresh] at yes
  cases yes

theorem no_second_reservation
    (first : reserve s scopeId ticket = (next,.reserved)) (path : Steps next later)
    (second : reserve later otherScope other = (last,.reserved)) : key ticket ≠ key other := by
  have present : ticket ∈ next.reserved := by rw [(reserve_parts first).2.2.2.2.2]; exact List.mem_cons_self
  exact fresh_key second (path_retains path present)

theorem admissible_reservation_progress (scope : scopeId = s.core.scopeId)
    (admitted : OwnerEvaluator.Admitted s.core.assigned s.core.image ticket)
    (idle : s.active = none) (fresh : hasKey s ticket = false)
    (room : s.reserved.length < s.core.capacity) :
    ∃ next, reserve s scopeId ticket = (next,.reserved) := by
  simp [reserve,scope,current_exact.mpr admitted,idle,fresh,Nat.not_le.mpr room]

def ActiveReserved (s : State p a) : Prop := ∀ ticket, s.active = some ticket → ticket ∈ s.reserved
def InactiveKey (ticket : Ticket) (s : State p a) : Prop :=
  ∀ active, s.active = some active → key ticket ≠ key active

theorem reservation_owns (owned : ActiveReserved s)
    (transition : reserve s scopeId ticket = (next,response)) : ActiveReserved next := by
  unfold reserve at transition
  split at transition
  · cases transition; exact owned
  · split at transition
    · cases transition; exact owned
    · split at transition
      · split at transition <;> cases transition <;> exact owned
      · split at transition
        · cases transition; exact owned
        · cases transition
          intro active same
          cases same
          exact List.mem_cons_self

theorem step_owns (owned : ActiveReserved s) (step : Step s next) : ActiveReserved next := by
  cases step with
  | reservation transition => exact reservation_owns owned transition
  | computation transition => obtain ⟨_,_,_,_,rfl⟩ := compute_parts transition; simp [ActiveReserved]
  | installation transition => rw [(install_idle transition).2]; exact owned
  | abandonment => simp [ActiveReserved,abandon]

theorem rooted_owns (path : Steps (initial assigned scopeId image capacity) s) : ActiveReserved s := by
  induction path with
  | refl => simp [ActiveReserved,initial]
  | next _ step ih => exact step_owns ih step

theorem reservation_inactive (present : prior ∈ s.reserved) (inactive : InactiveKey prior s)
    (transition : reserve s scopeId ticket = (next,response)) : InactiveKey prior next := by
  have sameTransition := transition
  unfold reserve at transition
  split at transition
  · cases transition; exact inactive
  · split at transition
    · cases transition; exact inactive
    · split at transition
      · split at transition <;> cases transition <;> exact inactive
      · split at transition
        · cases transition; exact inactive
        · cases transition
          intro active same
          cases same
          exact fresh_key sameTransition present

theorem step_inactive (present : ticket ∈ s.reserved) (inactive : InactiveKey ticket s)
    (step : Step s next) : InactiveKey ticket next := by
  cases step with
  | reservation transition => exact reservation_inactive present inactive transition
  | computation transition => obtain ⟨_,_,_,_,rfl⟩ := compute_parts transition; simp [InactiveKey]
  | installation transition => rw [(install_idle transition).2]; exact inactive
  | abandonment => simp [InactiveKey,abandon]

theorem path_inactive (path : Steps s next) (present : ticket ∈ s.reserved)
    (inactive : InactiveKey ticket s) : InactiveKey ticket next := by
  induction path with
  | refl => exact inactive
  | next previous step ih => exact step_inactive (path_retains previous present) ih step

theorem no_second_computation
    (owned : ActiveReserved s) (active : s.active = some ticket)
    (first : compute s = some (next,reply)) (path : Steps next later)
    (other : later.active = some otherTicket) : key ticket ≠ key otherTicket := by
  have retained := owned ticket active
  obtain ⟨_,_,_,_,atNext⟩ := compute_parts first
  have present : ticket ∈ next.reserved := by rw [atNext]; exact retained
  have inactive : InactiveKey ticket next := by rw [atNext]; simp [InactiveKey]
  exact path_inactive path present inactive otherTicket other

-- Independent finite-resource/freshness premises, not an assumed compute
-- success. Relating reserved and produced ledgers on every reachable native
-- state remains a distinct accounting invariant; neither list is erased.
theorem reserved_computation_progress
    (reserved : reserve s scopeId ticket = (next,.reserved))
    (fresh : OwnerOccurrence.seen s.core ticket = false)
    (room : s.core.records.length < s.core.capacity) :
    ∃ later record, compute next = some (later,.produced record) := by
  have parts := reserve_parts reserved
  have admitted := OwnerOccurrence.current_exact.mp parts.2.1
  obtain ⟨core,record,produced⟩ := OwnerOccurrence.admitted_fresh_progress
    (s:=s.core) (scopeId:=s.core.scopeId) rfl admitted fresh room
  rw [parts.2.2.2.2.2]
  exact ⟨⟨core,ticket :: s.reserved,none⟩,record,by simp [compute,produced]⟩

def Inventory (s : State p a) : Prop :=
  (∀ record ∈ s.core.records, ∃ prior ∈ s.reserved, key record.ticket = key prior) ∧
  s.core.records.length + s.active.toList.length ≤ s.reserved.length

theorem submitted_records (ran : submit core scopeId ticket = (next,reply)) :
    next.records = core.records ∨ ∃ record, next.records = record :: core.records ∧ record.ticket = ticket := by
  unfold submit at ran
  split at ran
  · cases ran; exact Or.inl rfl
  · split at ran
    · split at ran <;> cases ran <;> exact Or.inl rfl
    · split at ran
      · cases ran; exact Or.inl rfl
      · cases ran; exact Or.inr ⟨_,rfl,rfl⟩

theorem reservation_inventory (inventory : Inventory s)
    (ran : reserve s scopeId ticket = (next,response)) : Inventory next := by
  have original := ran
  unfold reserve at ran
  split at ran
  · cases ran; exact inventory
  · split at ran
    · cases ran; exact inventory
    · rename_i idle
      split at ran
      · split at ran <;> cases ran <;> exact inventory
      · split at ran
        · cases ran; exact inventory
        · cases ran
          refine ⟨?_,?_⟩
          · intro record present
            obtain ⟨prior,old,same⟩ := inventory.1 record present
            exact ⟨prior,List.mem_cons_of_mem _ old,same⟩
          · have empty : s.active = none := by simpa using idle
            have bound := inventory.2
            simp only [empty,Option.toList_none,List.length_nil,Nat.add_zero] at bound
            simpa using Nat.add_le_add_right bound 1

theorem computation_inventory (owned : ActiveReserved s) (inventory : Inventory s)
    (ran : compute s = some (next,reply)) : Inventory next := by
  obtain ⟨ticket,core,active,submitted,rfl⟩ := compute_parts ran
  have present := owned ticket active
  have bound := inventory.2
  simp only [active,Option.toList_some,List.length_singleton] at bound
  rcases submitted_records submitted with unchanged | ⟨record,records,ticketAt⟩
  · refine ⟨?_,?_⟩
    · intro row member
      apply inventory.1 row
      simpa only [unchanged] using member
    · simpa only [unchanged,Option.toList_none,List.length_nil,Nat.add_zero] using
        (Nat.le_trans (Nat.le_add_right s.core.records.length 1) bound)
  · refine ⟨?_,?_⟩
    · intro row member
      change row ∈ core.records at member
      rw [records] at member
      rcases List.mem_cons.mp member with rfl | old
      · exact ⟨ticket,present,congrArg key ticketAt⟩
      · exact inventory.1 row old
    · simpa only [records,List.length_cons,Option.toList_none,List.length_nil,Nat.add_zero] using bound

theorem step_inventory (owned : ActiveReserved s) (inventory : Inventory s)
    (step : Step s next) : Inventory next := by
  cases step with
  | reservation ran => exact reservation_inventory inventory ran
  | computation ran => exact computation_inventory owned inventory ran
  | installation ran => rw [(install_idle ran).2]; exact inventory
  | abandonment =>
      refine ⟨inventory.1,?_⟩
      have bound := Nat.le_trans (Nat.le_add_right s.core.records.length s.active.toList.length) inventory.2
      simpa [abandon] using bound

theorem rooted_inventory (path : Steps (initial assigned scopeId image capacity) s) : Inventory s := by
  induction path with
  | refl => simp [Inventory,initial]
  | next previous step ih => exact step_inventory (rooted_owns previous) ih step

theorem rooted_reserved_computes
    (path : Steps (initial assigned scopeId image capacity) s)
    (reserved : reserve s requestedScope ticket = (next,.reserved)) :
    ∃ later record, compute next = some (later,.produced record) := by
  have inventory := rooted_inventory path
  have parts := reserve_parts reserved
  apply reserved_computation_progress reserved
  · have no : ¬ OwnerOccurrence.seen s.core ticket = true := by
      intro yes
      obtain ⟨record,present,same⟩ := OwnerOccurrence.seen_exact.mp yes
      obtain ⟨prior,old,bound⟩ := inventory.1 record present
      exact fresh_key reserved old (bound.symm.trans same)
    simpa using no
  · have bound := inventory.2
    have room := parts.2.2.2.2.1
    omega

theorem step_ordinals (valid : OwnerOccurrence.Ordinals s.core) (step : Step s next) :
    OwnerOccurrence.Ordinals next.core := by
  cases step with
  | reservation ran => rw [reserve_core ran]; exact valid
  | computation ran =>
      obtain ⟨_,_,_,produced,rfl⟩ := compute_parts ran
      exact OwnerOccurrence.submit_ordinals valid produced
  | installation ran => rw [(install_idle ran).2]; exact valid
  | abandonment => exact valid

theorem rooted_ordinals (path : Steps (initial assigned scopeId image capacity) s) :
    OwnerOccurrence.Ordinals s.core := by
  induction path with
  | refl => rfl
  | next _ step ih => exact step_ordinals ih step

#print axioms rooted_ordinals

#print axioms rooted_inventory
#print axioms rooted_reserved_computes

#print axioms rooted_owns
#print axioms path_inactive
#print axioms no_second_computation
#print axioms reserved_computation_progress

#print axioms reserve_parts
#print axioms compute_parts
#print axioms computation_clears
#print axioms computation_meaning
#print axioms install_idle
#print axioms path_retains
#print axioms no_second_reservation
#print axioms admissible_reservation_progress
end MirroreaProofFirst.OwnerReservation
