import MirroreaProofFirstReferenceExecution

namespace MirroreaProofFirst.ReferenceCancellationTrace
open ReferenceExecution

theorem transition_used (m next : Machine p a) (step : Transition m next) :
    ∀ id ∈ m.store.core.system.used, id ∈ next.store.core.system.used := by
  cases step with
  | management run =>
      have core := manage_core _ _ _ _ _ _ _ run
      obtain ⟨_,_,_,_,_,_,equal⟩ := CompositionMachine.manage_parts _ _ _ _ _ _ _ core
      have exactCore := congrArg (fun pair : CompositionMachine.Machine p a × Option Nat => pair.1.system.used) equal
      dsimp only at exactCore
      rw [exactCore]
      intro id member; exact List.mem_cons_of_mem _ member
  | authority => exact fun _ h => h
  | startPlain run =>
      have core := startPlain_core _ _ _ _ _ _ _ _ run
      have equal := (CompositionMachine.start_parts _ _ _ _ _ _ _ _ core).2.2
      change next.store.core = CompositionMachine.enqueue m.store.core _ at equal
      rw [equal]
      exact fun _ h => h
  | startReference run =>
      obtain ⟨_,core⟩ := startReference_core _ _ _ _ _ _ _ _ run
      have equal := (CompositionMachine.start_parts _ _ _ _ _ _ _ _ core).2.2
      change next.store.core = CompositionMachine.enqueue m.store.core _ at equal
      rw [equal]
      exact fun _ h => h
  | finish run =>
      have core := finish_core_run _ _ _ _ run
      obtain ⟨_,_,_,equal⟩ := CompositionMachine.finish_parts _ _ _ _ core
      rw [equal]
      intro id member; exact List.mem_cons_of_mem _ member
  | cancellation run =>
      obtain ⟨_,core⟩ := cancel_core _ _ _ _ _ _ _ run
      obtain ⟨_,_,_,_,equal⟩ := CompositionMachine.cancel_parts _ _ _ _ _ _ _ _ core
      rw [equal]
      intro id member; exact List.mem_cons_of_mem _ (List.mem_cons_of_mem _ member)
  | acquire run =>
      rw [acquire_core _ _ _ _ _ _ _ run]
      intro id member; exact List.mem_cons_of_mem _ member
  | reacquire run =>
      rw [reacquire_core _ _ _ _ _ _ _ run]
      intro id member; exact List.mem_cons_of_mem _ member
  | release run =>
      rw [release_core _ _ _ _ _ _ _ run]
      intro id member; exact List.mem_cons_of_mem _ member
  | @normalize member place principal requestId key =>
      cases consumed : (normalize m member place principal requestId key).consumed with
      | false => rw [normalize_unconsumed _ _ _ _ _ _ consumed]; exact fun _ h => h
      | true =>
          rw [normalize_consumed_core _ _ _ _ _ _ consumed]
          intro id atUsed; exact List.mem_cons_of_mem _ atUsed

theorem reached_used (m next : Machine p a) (path : Reached m next) :
    ∀ id ∈ m.store.core.system.used, id ∈ next.store.core.system.used := by
  induction path with
  | refl => exact fun _ h => h
  | step _ step ih => exact fun id mem => transition_used _ _ step id (ih id mem)

theorem used_not_pending (m : Machine p a) (entry : Pending) (valid : Invariant m)
    (used : CompositionMachine.ticketId entry.ticket ∈ m.store.core.system.used) : entry ∉ m.pending := by
  intro present
  have coreMem : entry.ticket ∈ m.store.core.pending := by
    rw [← valid.2]
    exact List.mem_map.mpr ⟨entry,present,rfl⟩
  exact valid.1.1.1.2.2 entry.ticket coreMem used

theorem terminal_future (m next : Machine p a) (entry : Pending) (valid : Invariant m)
    (used : CompositionMachine.ticketId entry.ticket ∈ m.store.core.system.used) (path : Reached m next) :
    entry ∉ next.pending ∧ (∀ value, finish next entry value = none) ∧
      ∀ member place principal id, cancel next member place principal id entry = none := by
  have absent := used_not_pending next entry (reached_preserves _ _ valid path)
    (reached_used _ _ path _ used)
  exact ⟨absent,by intro value; simp [finish,absent],by intro member place principal id; simp [cancel,absent]⟩

-- This quantifies every admitted future extension, rather than comparing two
-- unrelated immutable branches. A result and a cancellation cannot both commit
-- for the same UseId on one actual history; neither terminal permits id reuse.
theorem after_cancel_terminal (m cancelled next : Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (entry other : Pending) (valid : Invariant m)
    (accepted : cancel m member place principal id entry = some cancelled)
    (same : CompositionMachine.ticketId other.ticket = CompositionMachine.ticketId entry.ticket)
    (path : Reached cancelled next) :
    other ∉ next.pending ∧ (∀ value, finish next other value = none) ∧
      ∀ member place principal id, cancel next member place principal id other = none := by
  have initialValid := cancel_preserves _ _ _ _ _ _ _ valid accepted
  have used : CompositionMachine.ticketId other.ticket ∈ cancelled.store.core.system.used := by
    obtain ⟨_,core⟩ := cancel_core _ _ _ _ _ _ _ accepted
    obtain ⟨_,_,_,_,equal⟩ := CompositionMachine.cancel_parts _ _ _ _ _ _ _ _ core
    rw [equal,same]
    exact List.mem_cons_self
  exact terminal_future _ _ _ initialValid used path

theorem after_result_terminal (m completed next : Machine p a) (entry other : Pending) (value : Int)
    (valid : Invariant m) (accepted : finish m entry value = some completed)
    (same : CompositionMachine.ticketId other.ticket = CompositionMachine.ticketId entry.ticket)
    (path : Reached completed next) :
    other ∉ next.pending ∧ (∀ value, finish next other value = none) ∧
      ∀ member place principal id, cancel next member place principal id other = none := by
  have initialValid := finish_preserves _ _ _ _ valid accepted
  have used : CompositionMachine.ticketId other.ticket ∈ completed.store.core.system.used := by
    have core := finish_core_run _ _ _ _ accepted
    obtain ⟨_,_,_,equal⟩ := CompositionMachine.finish_parts _ _ _ _ core
    rw [equal,same]
    exact List.mem_cons_self
  exact terminal_future _ _ _ initialValid used path

#print axioms transition_used
#print axioms reached_used
#print axioms terminal_future
#print axioms after_cancel_terminal
#print axioms after_result_terminal
end MirroreaProofFirst.ReferenceCancellationTrace
