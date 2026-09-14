import MirroreaProofFirstOwnerTableEvaluator

namespace MirroreaProofFirst.OwnerOccurrence
abbrev Ticket := InvocationBoundary.Ticket

-- One independently assigned live owner namespace. Install never changes this
-- namespace or erases records. Fresh launch/namespace authenticity and image
-- publication custody are separate physical obligations, not invented by Nat.
structure Record (p a : Nat) where
  scopeId : Nat
  ordinal : Nat
  revision : Nat
  image : OwnerImage.Image p a
  ticket : Ticket
  result : OwnerEvaluator.Result

structure State (p a : Nat) where
  assigned : OwnerEvaluator.Assignment p
  scopeId : Nat
  revision : Nat
  image : OwnerImage.Image p a
  records : List (Record p a)
  capacity : Nat

inductive Reply (p a : Nat) where
  | denied | duplicate | conflict | exhausted
  | produced (record : Record p a)

-- Principal/id is only local to this retained owner namespace. The full ticket
-- is checked separately, so key reuse with changed code/context is a conflict.
def key (ticket : Ticket) : Nat × Nat := (ticket.principal,ticket.id)
def seen (s : State p a) (ticket : Ticket) : Bool :=
  s.records.any (fun record => decide (key record.ticket = key ticket))

def currentCheck (s : State p a) (ticket : Ticket) : Bool :=
  decide (s.image.state.realm = s.assigned.realm ∧ s.image.view.realm = s.assigned.realm) &&
  OwnerTableEvaluator.validity s.image && OwnerTableEvaluator.check (OwnerImage.restore s.image) s.assigned.place ticket

theorem current_exact : currentCheck s ticket = true ↔ OwnerEvaluator.Admitted s.assigned s.image ticket := by
  simp only [currentCheck,Bool.and_eq_true,decide_eq_true_eq,OwnerTableEvaluator.check_exact,
    OwnerTableEvaluator.validity_same,
    OwnerValidity.check_exact,OwnerProjection.check_exact,OwnerEvaluator.Admitted]
  simp only [and_assoc]

-- There is no stored-result replay path. Current admission precedes even the
-- duplicate classification, matching the accepted I3 distinction. Computation
-- occurs only on the fresh branch; the immutable decision is retained before
-- returning the response for later IO. Native failure during this transition
-- still requires a retained reservation / failed-namespace mechanism.
def submit (s : State p a) (scopeId : Nat) (ticket : Ticket) : State p a × Reply p a :=
  if scopeId ≠ s.scopeId || !currentCheck s ticket then (s,.denied)
  else if seen s ticket then
    if s.records.any (fun record => decide (record.ticket = ticket)) then (s,.duplicate)
    else (s,.conflict)
  else if s.capacity ≤ s.records.length then (s,.exhausted)
  else
    let record : Record p a := ⟨s.scopeId,s.records.length,s.revision,s.image,ticket,
      OwnerTableEvaluator.run s.assigned s.image ticket⟩
    ({s with records := record :: s.records},.produced record)

def install (s : State p a) (revision : Nat) (image : OwnerImage.Image p a) : State p a :=
  {s with revision := revision,image := image}

theorem install_records : (install s revision image).records = s.records := rfl
theorem install_namespace : (install s revision image).scopeId = s.scopeId := rfl

theorem seen_exact : seen s ticket = true ↔ ∃ record ∈ s.records, key record.ticket = key ticket := by
  simp [seen,List.any_eq_true]

theorem produced_parts (produced : submit s scopeId ticket = (next,.produced record)) :
    scopeId = s.scopeId ∧ currentCheck s ticket = true ∧ seen s ticket = false ∧
    s.records.length < s.capacity ∧
    record = ⟨s.scopeId,s.records.length,s.revision,s.image,ticket,
      OwnerTableEvaluator.run s.assigned s.image ticket⟩ ∧
    next = {s with records := record :: s.records} := by
  unfold submit at produced
  split at produced
  · cases produced
  · rename_i admitted
    split at produced
    · split at produced <;> cases produced
    · rename_i fresh
      split at produced
      · cases produced
      · rename_i room
        simp only [Prod.mk.injEq,Reply.produced.injEq] at produced
        obtain ⟨state,recordEq⟩ := produced
        have allowed : scopeId = s.scopeId ∧ currentCheck s ticket = true := by
          simpa using admitted
        refine ⟨allowed.1,allowed.2,?_,Nat.lt_of_not_ge room,recordEq.symm,?_⟩
        · simpa using fresh
        · simpa only [recordEq] using state.symm

theorem produced_meaning (produced : submit s scopeId ticket = (next,.produced record)) :
    ∃ value, record.result = .value value ∧
      OwnerEvaluator.Admitted s.assigned record.image record.ticket ∧
      InstancePrograms.Machine.Executes record.ticket.definition.code record.ticket.argument value := by
  obtain ⟨_,admitted,_,_,rfl,_⟩ := produced_parts produced
  have valid := current_exact.mp admitted
  obtain ⟨value,ran,_⟩ := OwnerEvaluator.admitted_completes s.assigned s.image ticket valid
  refine ⟨value,?_,valid,(OwnerEvaluator.result_exact _ _ _ _).mp ran |>.2⟩
  simpa only [OwnerTableEvaluator.run_exact] using ran

theorem old_records_retained (step : submit s scopeId ticket = (next,reply))
    (old : record ∈ s.records) : record ∈ next.records := by
  unfold submit at step
  split at step
  · cases step; exact old
  · split at step
    · split at step <;> cases step <;> exact old
    · split at step
      · cases step; exact old
      · cases step; exact List.mem_cons_of_mem _ old

theorem produced_fresh (produced : submit s scopeId ticket = (next,.produced record))
    (old : previous ∈ s.records) : key previous.ticket ≠ key record.ticket := by
  obtain ⟨_,_,fresh,_,rfl,_⟩ := produced_parts produced
  intro same
  have yes := seen_exact.mpr ⟨previous,old,same⟩
  rw [fresh] at yes
  cases yes

theorem admitted_fresh_progress (scope : scopeId = s.scopeId)
    (admitted : OwnerEvaluator.Admitted s.assigned s.image ticket)
    (fresh : seen s ticket = false) (room : s.records.length < s.capacity) :
    ∃ next record, submit s scopeId ticket = (next,.produced record) := by
  simp [submit,scope,current_exact.mpr admitted,fresh,Nat.not_le.mpr room]

inductive Step : State p a → State p a → Prop where
  | submission : submit s scopeId ticket = (next,reply) → Step s next
  | installation : Step s (install s revision image)

inductive Steps : State p a → State p a → Prop where
  | refl : Steps s s
  | next : Steps start s → Step s next → Steps start next

theorem step_records_retained (step : Step s next) (old : record ∈ s.records) :
    record ∈ next.records := by
  cases step with
  | submission submitted => exact old_records_retained submitted old
  | installation => exact old

theorem path_records_retained (path : Steps s next) (old : record ∈ s.records) :
    record ∈ next.records := by
  induction path with
  | refl => exact old
  | next _ step ih => exact step_records_retained step ih

theorem no_second_production
    (first : submit s scopeId ticket = (next,.produced record))
    (path : Steps next later)
    (second : submit later otherScope otherTicket = (last,.produced other)) :
    key record.ticket ≠ key other.ticket := by
  have member : record ∈ next.records := by
    obtain ⟨_,_,_,_,_,state⟩ := produced_parts first
    rw [state]; exact List.mem_cons_self
  exact produced_fresh second (path_records_retained path member)

theorem no_replay_result (seenBefore : seen s ticket = true)
    (response : submit s scopeId ticket = (next,reply)) :
    next = s ∧ (reply = .denied ∨ reply = .duplicate ∨ reply = .conflict) := by
  unfold submit at response
  split at response
  · cases response; exact ⟨rfl,Or.inl rfl⟩
  · split at response
    · cases response; exact ⟨rfl,Or.inr (Or.inl rfl)⟩
    · cases response; exact ⟨rfl,Or.inr (Or.inr rfl)⟩

-- Key non-repetition did not validate arbitrary initial record ordinals.
-- Empty fresh history establishes this separate retained-order invariant.
def Ordinals (s : State p a) : Prop :=
  s.records.map Record.ordinal = (List.range s.records.length).reverse

theorem submit_ordinals (valid : Ordinals s) (step : submit s scopeId ticket = (next,reply)) :
    Ordinals next := by
  unfold submit at step
  split at step
  · cases step; exact valid
  · split at step
    · split at step <;> cases step <;> exact valid
    · split at step
      · cases step; exact valid
      · cases step
        simp only [Ordinals,List.map_cons,List.length_cons]
        rw [List.range_succ,List.reverse_append]
        simpa only [List.reverse_singleton,List.singleton_append] using
          congrArg (List.cons s.records.length) valid

theorem path_ordinals (valid : Ordinals s) (path : Steps s next) : Ordinals next := by
  induction path with
  | refl => exact valid
  | next previous step ih =>
      cases step with
      | submission submitted => exact submit_ordinals ih submitted
      | installation => exact ih

theorem ordinals_unique (valid : Ordinals s) : (s.records.map Record.ordinal).Nodup := by
  rw [valid]
  rw [List.nodup_iff_pairwise_ne,List.pairwise_reverse]
  exact List.nodup_range.imp (fun different => Ne.symm different)

#print axioms submit_ordinals
#print axioms path_ordinals
#print axioms ordinals_unique

#print axioms step_records_retained
#print axioms path_records_retained
#print axioms no_second_production
#print axioms no_replay_result

#print axioms current_exact
#print axioms seen_exact
#print axioms produced_parts
#print axioms produced_meaning
#print axioms old_records_retained
#print axioms produced_fresh
#print axioms admitted_fresh_progress
end MirroreaProofFirst.OwnerOccurrence
