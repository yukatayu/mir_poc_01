import MirroreaProofFirstReferenceSourceTrace

namespace MirroreaProofFirst.ReferenceChronology
open ReferenceStore

def Recorded (before after : ReferenceStore.Machine p a) : Prop :=
  after.history = before.history ++ [frame after.core] ∧ ∃ event, after.events = event :: before.events

theorem transition_recorded (m next : ReferenceStore.Machine p a) (step : ReferenceTrace.Transition m next) :
    next = m ∨ Recorded m next := by
  cases step with
  | management run =>
      obtain ⟨core,created,_,_,equal⟩ := manage_parts _ _ _ _ _ _ _ run
      obtain ⟨rfl,rfl⟩ := Prod.mk.inj equal
      exact Or.inr ⟨rfl,_,rfl⟩
  | authority => exact Or.inr ⟨rfl,_,rfl⟩
  | start run =>
      simp only [start,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i pair started
        obtain ⟨core,ticket⟩ := pair
        simp only [Option.some.injEq,Prod.mk.injEq] at run
        obtain ⟨rfl,rfl⟩ := run
        exact Or.inr ⟨rfl,_,rfl⟩
  | cancellation run =>
      obtain ⟨_,core,_,rfl⟩ := cancel_parts _ _ _ _ _ _ _ _ run
      exact Or.inr ⟨rfl,_,rfl⟩
  | finishPlain run =>
      simp only [finishPlain,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · cases run
        exact Or.inr ⟨rfl,_,rfl⟩
  | acquire run => exact Or.inr (ReferenceMutation.acquire_recorded _ _ _ _ _ _ _ run)
  | reacquire run => exact Or.inr (ReferenceMutation.reacquire_recorded _ _ _ _ _ _ _ run)
  | release run => exact Or.inr (ReferenceMutation.release_recorded _ _ _ _ _ _ _ run)
  | normalize => exact ReferenceMutation.normalize_recorded _ _ _ _ _ _

theorem changed_recorded (m next : ReferenceStore.Machine p a) (step : ReferenceTrace.Transition m next)
    (changed : next ≠ m) : Recorded m next := by
  rcases transition_recorded _ _ step with same | recorded
  · exact False.elim (changed same)
  · exact recorded

-- This relation records the actual resulting state at every non-stuttering
-- admitted transition. Its constructors do not assume anything about a stored
-- history list, a stored event list, or equality of serial and event count.
inductive Chronological : ReferenceStore.Machine p a → ReferenceStore.Machine p a →
    List (ReferenceAccessHistory.Frame p a) → Prop where
  | refl : Chronological m m []
  | idle : Chronological first m frames → ReferenceTrace.Transition m m → Chronological first m frames
  | step : Chronological first m frames → ReferenceTrace.Transition m next → next ≠ m →
      Chronological first next (frames ++ [frame next.core])

theorem reached_chronological (first last : ReferenceStore.Machine p a) (path : ReferenceTrace.Reached first last) :
    ∃ frames, Chronological first last frames := by
  induction path with
  | refl => exact ⟨[],.refl⟩
  | @step m next previous step ih =>
      obtain ⟨frames,record⟩ := ih
      by_cases same : next = m
      · subst next; exact ⟨frames,.idle record step⟩
      · exact ⟨_,.step record step same⟩

theorem chronological_history (first last : ReferenceStore.Machine p a)
    (frames : List (ReferenceAccessHistory.Frame p a)) (record : Chronological first last frames) :
    last.history = first.history ++ frames ∧ last.events.length = first.events.length + frames.length := by
  induction record with
  | refl => simp
  | idle previous step ih => exact ih
  | @step m next previousFrames previous step changed ih =>
      obtain ⟨historyAt,event,eventAt⟩ := changed_recorded _ _ step changed
      refine ⟨?_,?_⟩
      · rw [historyAt,ih.1,List.append_assoc]
      · simp only [eventAt,List.length_cons,ih.2,List.length_append,List.length_nil]
        omega

theorem reached_history (first last : ReferenceStore.Machine p a) (path : ReferenceTrace.Reached first last) :
    ∃ frames, Chronological first last frames ∧ last.history = first.history ++ frames ∧
      last.events.length = first.events.length + frames.length := by
  obtain ⟨frames,record⟩ := reached_chronological _ _ path
  exact ⟨frames,record,chronological_history _ _ _ record⟩

theorem source_step_store (s next : ReferenceSource.State p a) (step : ReferenceSourceTrace.Step s next) :
    ReferenceTrace.Reached s.machine.store next.machine.store :=
  ReferenceExecution.reached_projects _ _ (ReferenceSourceTrace.step_machine _ _ step)

theorem store_trans (first middle last : ReferenceStore.Machine p a)
    (left : ReferenceTrace.Reached first middle) (right : ReferenceTrace.Reached middle last) :
    ReferenceTrace.Reached first last := by
  induction right with
  | refl => exact left
  | step previous step ih => exact .step ih step

theorem source_reached_store (s next : ReferenceSource.State p a) (path : ReferenceSourceTrace.Reached s next) :
    ReferenceTrace.Reached s.machine.store next.machine.store := by
  induction path with
  | refl => exact .refl
  | step previous step ih => exact store_trans _ _ _ ih (source_step_store _ _ step)

-- Even a failed source statement can contain actual committed microsteps.
-- They appear in this chronological history; a failed non-mutating check does
-- not append an invented successful occurrence. Full admitted authority input
-- is carried by the transition, not reconstructed from its generation label.
theorem rooted_source_history (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : ReferenceSource.State p a)
    (path : ReferenceSourceTrace.Rooted realm view policy s) :
    ∃ frames, Chronological (ReferenceStore.initial realm view policy) s.machine.store frames ∧
      s.machine.store.history = [frame (ReferenceStore.initial (p:=p) realm view policy).core] ++ frames ∧
      s.machine.store.events.length = frames.length := by
  obtain ⟨frames,record,historyAt,countAt⟩ := reached_history _ _ (source_reached_store _ _ path)
  exact ⟨frames,record,historyAt,by simpa [ReferenceSource.initial,ReferenceExecution.initial,ReferenceStore.initial] using countAt⟩

theorem rooted_history_count (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : ReferenceSource.State p a)
    (path : ReferenceSourceTrace.Rooted realm view policy s) :
    s.machine.store.history.length = s.machine.store.events.length + 1 := by
  obtain ⟨frames,_,historyAt,countAt⟩ := rooted_source_history _ _ _ _ path
  simp [historyAt,countAt,Nat.add_comm]

#print axioms transition_recorded
#print axioms chronological_history
#print axioms rooted_source_history
#print axioms rooted_history_count
end MirroreaProofFirst.ReferenceChronology
